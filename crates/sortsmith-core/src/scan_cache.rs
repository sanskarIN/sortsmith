use crate::engine::describe_file_from_metadata;
use crate::models::{FileEntry, PlannedOperation, PreviewResult, Rule, RuleCriterion, ScanOptions};
use crate::rules::{destination_for, PreparedRule};
use crate::safety::collision_safe_path;
use crate::Result;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScanCacheStats {
    pub reused_files: usize,
    pub rescanned_files: usize,
    pub revalidated_time_sensitive_files: usize,
    pub cached_entries: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScanCacheScope {
    root: PathBuf,
    rules: Vec<Rule>,
    options: ScanOptions,
}

#[derive(Clone, Debug)]
struct CachedFile {
    size: u64,
    modified_at: Option<DateTime<Utc>>,
    file: FileEntry,
    matched_rule_index: Option<usize>,
}

#[derive(Debug, Default)]
pub struct ScanCache {
    scope: Option<ScanCacheScope>,
    entries: HashMap<PathBuf, CachedFile>,
    stats: ScanCacheStats,
}

impl ScanCache {
    pub fn clear(&mut self) {
        self.scope = None;
        self.entries.clear();
        self.stats = ScanCacheStats::default();
    }

    pub fn stats(&self) -> ScanCacheStats { self.stats }

    fn prepare_scope(&mut self, root: &Path, rules: &[Rule], options: &ScanOptions) {
        let next = ScanCacheScope { root: root.to_path_buf(), rules: rules.to_vec(), options: options.clone() };
        if self.scope.as_ref() != Some(&next) {
            self.entries.clear();
            self.scope = Some(next);
        }
        self.stats = ScanCacheStats::default();
    }
}

pub fn preview_organization_cached(
    root: &Path,
    rules: &[Rule],
    options: &ScanOptions,
    cache: &mut ScanCache,
) -> Result<PreviewResult> {
    let prepared_rules = rules.iter().filter(|rule| rule.enabled).map(PreparedRule::new).collect::<Result<Vec<_>>>()?;
    let has_time_sensitive_rules = prepared_rules.iter().any(|prepared| {
        prepared.rule().criteria.iter().any(|criterion| matches!(criterion, RuleCriterion::ModifiedOlderThanDays { .. }))
    });

    cache.prepare_scope(root, rules, options);
    let mut result = PreviewResult::default();
    let mut seen = HashSet::new();
    let depth = if options.recursive { options.max_depth.unwrap_or(32) } else { 1 };
    let walker = WalkDir::new(root).follow_links(options.follow_links).max_depth(depth);

    for item in walker.into_iter().filter_entry(|entry| options.include_hidden || !is_hidden(entry, root)) {
        let entry = match item {
            Ok(value) => value,
            Err(error) => {
                result.recoverable_errors.push(redact_walk_error(&error));
                continue;
            }
        };
        if !entry.file_type().is_file() { continue; }

        result.scanned_files += 1;
        let path = entry.path().to_path_buf();
        let metadata = match entry.metadata() {
            Ok(value) => value,
            Err(error) => {
                result.recoverable_errors.push(redact_walk_error(&error));
                continue;
            }
        };
        let modified_at = metadata.modified().ok().map(DateTime::<Utc>::from);
        let size = metadata.len();
        seen.insert(path.clone());

        let reused = cache.entries.get(&path).filter(|cached| cached.size == size && cached.modified_at == modified_at).cloned();
        let (file, matched_rule_index) = if let Some(cached) = reused {
            cache.stats.reused_files += 1;
            let matched = if has_time_sensitive_rules {
                cache.stats.revalidated_time_sensitive_files += 1;
                first_matching_rule(&prepared_rules, &cached.file)
            } else {
                cached.matched_rule_index
            };
            (cached.file, matched)
        } else {
            cache.stats.rescanned_files += 1;
            let file = describe_file_from_metadata(root, &path, &metadata);
            let matched = first_matching_rule(&prepared_rules, &file);
            cache.entries.insert(path.clone(), CachedFile {
                size,
                modified_at,
                file: file.clone(),
                matched_rule_index: matched,
            });
            (file, matched)
        };

        let Some(rule_index) = matched_rule_index else {
            result.ignored_files += 1;
            continue;
        };
        let rule = prepared_rules[rule_index].rule();
        let destination = collision_safe_path(&destination_for(root, &file, rule)?);
        if destination != file.path {
            result.operations.push(PlannedOperation {
                id: Uuid::new_v4(),
                source: file.path.clone(),
                destination,
                rule_id: rule.id,
                rule_name: rule.name.clone(),
                size: file.size,
            });
        } else {
            result.ignored_files += 1;
        }
    }

    cache.entries.retain(|path, _| seen.contains(path));
    cache.stats.cached_entries = cache.entries.len();
    Ok(result)
}

fn first_matching_rule(prepared_rules: &[PreparedRule<'_>], file: &FileEntry) -> Option<usize> {
    prepared_rules.iter().position(|prepared| prepared.matches(file))
}

fn is_hidden(entry: &DirEntry, root: &Path) -> bool {
    if entry.path() == root { return false; }
    entry.file_name().to_str().is_some_and(|name| name.starts_with('.'))
}

fn redact_walk_error(error: &walkdir::Error) -> String {
    match error.io_error() {
        Some(io) => format!("A folder entry could not be read: {}", io.kind()),
        None => "A folder entry could not be read.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::preview_organization;
    use crate::models::RuleAction;
    use tempfile::tempdir;

    fn extension_rule(name: &str, destination: &str) -> Rule {
        Rule {
            id: Uuid::new_v4(),
            name: name.into(),
            enabled: true,
            match_all: true,
            criteria: vec![RuleCriterion::Extension { values: vec!["txt".into()] }],
            action: RuleAction::MoveTo { subdirectory: destination.into() },
        }
    }

    #[test]
    fn cached_and_uncached_plans_make_the_same_decisions() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        std::fs::write(root.path().join("ignored.bin"), b"binary").unwrap();
        let rules = vec![extension_rule("Text", "Text")];
        let options = ScanOptions::default();
        let uncached = preview_organization(root.path(), &rules, &options).unwrap();
        let mut cache = ScanCache::default();
        let cached = preview_organization_cached(root.path(), &rules, &options, &mut cache).unwrap();

        assert_eq!(uncached.scanned_files, cached.scanned_files);
        assert_eq!(uncached.ignored_files, cached.ignored_files);
        assert_eq!(uncached.recoverable_errors, cached.recoverable_errors);
        assert_eq!(uncached.operations.len(), cached.operations.len());
        for (left, right) in uncached.operations.iter().zip(&cached.operations) {
            assert_eq!(left.source, right.source);
            assert_eq!(left.destination, right.destination);
            assert_eq!(left.rule_id, right.rule_id);
            assert_eq!(left.rule_name, right.rule_name);
            assert_eq!(left.size, right.size);
        }
    }

    #[test]
    fn reuses_unchanged_file_descriptions() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let rule = extension_rule("Text", "Text");
        let mut cache = ScanCache::default();

        let first = preview_organization_cached(root.path(), &[rule.clone()], &ScanOptions::default(), &mut cache).unwrap();
        assert_eq!(first.operations.len(), 1);
        assert_eq!(cache.stats(), ScanCacheStats { reused_files: 0, rescanned_files: 1, revalidated_time_sensitive_files: 0, cached_entries: 1 });

        let second = preview_organization_cached(root.path(), &[rule], &ScanOptions::default(), &mut cache).unwrap();
        assert_eq!(second.operations.len(), 1);
        assert_eq!(first.operations[0].source, second.operations[0].source);
        assert_eq!(first.operations[0].destination, second.operations[0].destination);
        assert_eq!(cache.stats(), ScanCacheStats { reused_files: 1, rescanned_files: 0, revalidated_time_sensitive_files: 0, cached_entries: 1 });
    }

    #[test]
    fn collision_resolution_is_recomputed_on_cache_hit() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let rule = extension_rule("Text", "Text");
        let mut cache = ScanCache::default();

        let first = preview_organization_cached(root.path(), &[rule.clone()], &ScanOptions::default(), &mut cache).unwrap();
        let occupied = first.operations[0].destination.clone();
        std::fs::create_dir_all(occupied.parent().unwrap()).unwrap();
        std::fs::write(&occupied, b"existing destination").unwrap();

        let second = preview_organization_cached(root.path(), &[rule], &ScanOptions::default(), &mut cache).unwrap();
        assert_eq!(cache.stats().reused_files, 1);
        assert_ne!(second.operations[0].destination, occupied);
        assert!(!second.operations[0].destination.exists());
    }

    #[test]
    fn rule_changes_reset_cache_scope() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let mut cache = ScanCache::default();

        preview_organization_cached(root.path(), &[extension_rule("Text", "Text")], &ScanOptions::default(), &mut cache).unwrap();
        let second = preview_organization_cached(root.path(), &[extension_rule("Docs", "Documents")], &ScanOptions::default(), &mut cache).unwrap();

        assert_eq!(cache.stats().reused_files, 0);
        assert_eq!(cache.stats().rescanned_files, 1);
        assert!(second.operations[0].destination.ends_with(Path::new("Documents").join("note.txt")));
    }

    #[test]
    fn changed_files_are_rescanned() {
        let root = tempdir().unwrap();
        let path = root.path().join("note.txt");
        std::fs::write(&path, b"hello").unwrap();
        let rule = extension_rule("Text", "Text");
        let mut cache = ScanCache::default();

        preview_organization_cached(root.path(), &[rule.clone()], &ScanOptions::default(), &mut cache).unwrap();
        std::fs::write(&path, b"hello world with a different size").unwrap();
        preview_organization_cached(root.path(), &[rule], &ScanOptions::default(), &mut cache).unwrap();

        assert_eq!(cache.stats().reused_files, 0);
        assert_eq!(cache.stats().rescanned_files, 1);
    }

    #[test]
    fn deleted_files_are_pruned() {
        let root = tempdir().unwrap();
        let first = root.path().join("first.txt");
        let second = root.path().join("second.txt");
        std::fs::write(&first, b"one").unwrap();
        std::fs::write(&second, b"two").unwrap();
        let rule = extension_rule("Text", "Text");
        let mut cache = ScanCache::default();

        preview_organization_cached(root.path(), &[rule.clone()], &ScanOptions::default(), &mut cache).unwrap();
        assert_eq!(cache.stats().cached_entries, 2);
        std::fs::remove_file(second).unwrap();
        preview_organization_cached(root.path(), &[rule], &ScanOptions::default(), &mut cache).unwrap();

        assert_eq!(cache.stats().cached_entries, 1);
        assert_eq!(cache.stats().reused_files, 1);
    }

    #[test]
    fn clear_forces_full_rescan() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let rule = extension_rule("Text", "Text");
        let mut cache = ScanCache::default();

        preview_organization_cached(root.path(), &[rule.clone()], &ScanOptions::default(), &mut cache).unwrap();
        cache.clear();
        preview_organization_cached(root.path(), &[rule], &ScanOptions::default(), &mut cache).unwrap();

        assert_eq!(cache.stats().reused_files, 0);
        assert_eq!(cache.stats().rescanned_files, 1);
    }

    #[test]
    fn time_sensitive_rules_revalidate_reused_files() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let rule = Rule {
            id: Uuid::new_v4(),
            name: "Older files".into(),
            enabled: true,
            match_all: true,
            criteria: vec![RuleCriterion::ModifiedOlderThanDays { days: 0 }],
            action: RuleAction::MoveTo { subdirectory: "Older".into() },
        };
        let mut cache = ScanCache::default();

        preview_organization_cached(root.path(), &[rule.clone()], &ScanOptions::default(), &mut cache).unwrap();
        preview_organization_cached(root.path(), &[rule], &ScanOptions::default(), &mut cache).unwrap();

        assert_eq!(cache.stats().reused_files, 1);
        assert_eq!(cache.stats().revalidated_time_sensitive_files, 1);
    }
}
