use crate::error::io;
use crate::journal::{load_journal, save_journal};
use crate::models::*;
use crate::rules::{destination_for, PreparedRule};
use crate::safety::{collision_safe_path, collision_safe_path_with_reserved};
use crate::{Result, SortSmithError};
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

pub fn preview_organization(root: &Path, rules: &[Rule], options: &ScanOptions) -> Result<PreviewResult> {
    let prepared_rules = rules.iter().filter(|rule| rule.enabled).map(PreparedRule::new).collect::<Result<Vec<_>>>()?;
    let mut result = PreviewResult::default();
    let mut reserved_destinations = HashSet::new();
    let canonical_root = root.canonicalize().map_err(|e| io(root, e))?;
    let depth = if options.recursive { options.max_depth.unwrap_or(32) } else { 1 };
    let walker = WalkDir::new(root).follow_links(options.follow_links).max_depth(depth);
    for item in walker.into_iter().filter_entry(|e| {
        if !options.include_hidden && is_hidden(e, root) {
            return false;
        }
        if options.follow_links {
            return entry_resolves_outside_root(e, &canonical_root) != Some(true);
        }
        true
    }) {
        let entry = match item {
            Ok(v) => v,
            Err(err) => { result.recoverable_errors.push(redact_walk_error(&err)); continue; }
        };
        if !entry.file_type().is_file() { continue; }
        result.scanned_files += 1;
        if options.follow_links && !entry_within_root(entry.path(), &canonical_root) {
            result.recoverable_errors.push("A symbolic link points outside the selected folder; it was skipped.".into());
            result.ignored_files += 1;
            continue;
        }
        let file = match describe_file(root, entry.path()) {
            Ok(f) => f,
            Err(err) => { result.recoverable_errors.push(err.to_string()); continue; }
        };
        let mut planned = false;
        for prepared in &prepared_rules {
            if prepared.matches(&file) {
                let rule = prepared.rule();
                let desired = destination_for(root, &file, rule)?;
                let destination = collision_safe_path_with_reserved(&desired, &reserved_destinations);
                if destination != file.path {
                    reserved_destinations.insert(destination.clone());
                    result.operations.push(PlannedOperation { id: Uuid::new_v4(), source: file.path.clone(), destination, rule_id: rule.id, rule_name: rule.name.clone(), size: file.size });
                    planned = true;
                }
                break;
            }
        }
        if !planned { result.ignored_files += 1; }
    }
    Ok(result)
}

fn entry_resolves_outside_root(entry: &DirEntry, canonical_root: &Path) -> Option<bool> {
    if !entry.file_type().is_symlink() { return Some(false); }
    entry.path().canonicalize().ok().map(|resolved| !resolved.starts_with(canonical_root))
}

fn entry_within_root(path: &Path, canonical_root: &Path) -> bool {
    path.canonicalize().is_ok_and(|resolved| resolved.starts_with(canonical_root))
}

pub fn execute_preview(root: &Path, preview: &PreviewResult, journal_dir: &Path) -> Result<ExecutionReport> {
    validate_preview_paths(root, preview)?;
    let canonical_root = root.canonicalize().map_err(|e| io(root, e))?;
    let mut journal = OperationJournal { id: Uuid::new_v4(), created_at: Utc::now(), root: canonical_root, entries: Vec::new() };
    save_journal(journal_dir, &journal)?;
    let mut errors = Vec::new();
    for op in &preview.operations {
        let mut destination = collision_safe_path(&op.destination);
        if let Some(parent) = destination.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                errors.push(format!("Could not create destination folder: {err}"));
                continue;
            }
        }
        let mut moved = false;
        let mut move_error = false;
        for _ in 0..8 {
            match move_without_overwrite(&op.source, &destination) {
                Ok(()) => { moved = true; break; }
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                    destination = collision_safe_path(&destination);
                    if let Some(parent) = destination.parent() {
                        if let Err(parent_error) = fs::create_dir_all(parent) {
                            errors.push(format!("Could not create destination folder: {parent_error}"));
                            move_error = true;
                            break;
                        }
                    }
                }
                Err(err) => {
                    errors.push(format!("Could not move a file: {err}"));
                    move_error = true;
                    break;
                }
            }
        }
        if !moved && !move_error {
            errors.push("Could not find a collision-free destination after several retries.".into());
        }
        if moved {
            journal.entries.push(JournalEntry {
                operation_id: op.id,
                from: absolute_path(&op.source)?,
                to: absolute_path(&destination)?,
            });
            if let Err(err) = save_journal(journal_dir, &journal) {
                errors.push(format!("A completed move could not be durably recorded in the undo journal: {err}"));
                break;
            }
        }
    }
    Ok(ExecutionReport { completed: journal.entries.len(), journal, errors })
}

fn absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    std::env::current_dir().map(|dir| dir.join(path)).map_err(|e| io(path, e))
}

fn move_without_overwrite(source: &Path, destination: &Path) -> std::io::Result<()> {
    match fs::hard_link(source, destination) {
        Ok(()) => remove_source_after_link(source, destination),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Err(error),
        Err(_) => copy_new_then_remove(source, destination),
    }
}

fn remove_source_after_link(source: &Path, destination: &Path) -> std::io::Result<()> {
    if let Err(error) = fs::remove_file(source) {
        let _ = fs::remove_file(destination);
        return Err(error);
    }
    Ok(())
}

fn copy_new_then_remove(source: &Path, destination: &Path) -> std::io::Result<()> {
    let mut output = fs::OpenOptions::new().write(true).create_new(true).open(destination)?;
    let result = (|| {
        let mut input = fs::File::open(source)?;
        std::io::copy(&mut input, &mut output)?;
        output.flush()?;
        output.sync_all()?;
        fs::remove_file(source)
    })();
    if let Err(error) = result {
        let _ = fs::remove_file(destination);
        return Err(error);
    }
    Ok(())
}

pub fn undo_journal(journal_path: &Path) -> Result<ExecutionReport> {
    let journal = load_journal(journal_path)?;
    validate_journal_paths(&journal)?;
    let mut completed = 0usize;
    let mut errors = Vec::new();
    for entry in journal.entries.iter().rev() {
        if !entry.to.exists() { errors.push("A moved file is missing; skipped during undo.".into()); continue; }
        if entry.from.exists() { errors.push("Original path is occupied; skipped during undo.".into()); continue; }
        if let Some(parent) = entry.from.parent() { let _ = fs::create_dir_all(parent); }
        match move_without_overwrite(&entry.to, &entry.from) {
            Ok(()) => completed += 1,
            Err(err) => errors.push(format!("Undo failed: {err}")),
        }
    }
    Ok(ExecutionReport { journal, completed, errors })
}

fn validate_preview_paths(root: &Path, preview: &PreviewResult) -> Result<()> {
    let canonical_root = root.canonicalize().map_err(|e| io(root, e))?;
    for operation in &preview.operations {
        let canonical_source = operation.source.canonicalize().map_err(|e| io(&operation.source, e))?;
        if !canonical_source.starts_with(&canonical_root) {
            return Err(SortSmithError::UnsafeDestination(operation.source.clone()));
        }

        let relative = operation.destination.strip_prefix(root).map_err(|_| SortSmithError::UnsafeDestination(operation.destination.clone()))?;
        if relative.components().any(|component| matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_))) {
            return Err(SortSmithError::UnsafeDestination(operation.destination.clone()));
        }

        let mut current = canonical_root.clone();
        if let Some(parent) = relative.parent() {
            for component in parent.components() {
                current.push(component.as_os_str());
                if current.exists() {
                    let resolved = current.canonicalize().map_err(|e| io(&current, e))?;
                    if !resolved.starts_with(&canonical_root) {
                        return Err(SortSmithError::UnsafeDestination(operation.destination.clone()));
                    }
                    current = resolved;
                }
            }
        }
    }
    Ok(())
}

fn validate_journal_paths(journal: &OperationJournal) -> Result<()> {
    let canonical_root = journal.root.canonicalize().map_err(|e| io(&journal.root, e))?;
    for entry in &journal.entries {
        if !safe_journal_path(&canonical_root, &entry.from) || !safe_journal_path(&canonical_root, &entry.to) {
            return Err(SortSmithError::UnsafeDestination(entry.to.clone()));
        }
    }
    Ok(())
}

fn safe_journal_path(canonical_root: &Path, path: &Path) -> bool {
    let Ok(relative) = path.strip_prefix(canonical_root) else { return false; };
    if relative.components().any(|component| matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_))) {
        return false;
    }
    if path.exists() {
        return path.canonicalize().is_ok_and(|resolved| resolved.starts_with(canonical_root));
    }

    let mut current = canonical_root.to_path_buf();
    if let Some(parent) = relative.parent() {
        for component in parent.components() {
            current.push(component.as_os_str());
            if current.exists() {
                let Ok(resolved) = current.canonicalize() else { return false; };
                if !resolved.starts_with(canonical_root) { return false; }
                current = resolved;
            }
        }
    }
    true
}

fn describe_file(root: &Path, path: &Path) -> Result<FileEntry> {
    let metadata = fs::metadata(path).map_err(|e| io(path, e))?;
    let modified_at = metadata.modified().ok().map(DateTime::<Utc>::from);
    let extension = path.extension().and_then(|v| v.to_str()).map(|v| v.to_ascii_lowercase());
    let mime = mime_guess::from_path(path).first().map(|v| v.essence_str().to_string());
    Ok(FileEntry { path: path.to_path_buf(), relative_path: path.strip_prefix(root).unwrap_or(path).to_path_buf(), size: metadata.len(), modified_at, mime, extension })
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
    use crate::models::{RuleAction, RuleCriterion};
    use tempfile::tempdir;

    fn txt_rule() -> Rule {
        Rule { id: Uuid::new_v4(), name: "Text".into(), enabled: true, match_all: true, criteria: vec![RuleCriterion::Extension { values: vec!["txt".into()] }], action: RuleAction::MoveTo { subdirectory: "Text".into() } }
    }

    #[test]
    fn previews_executes_and_undoes() {
        let root = tempdir().unwrap();
        let journals = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let preview = preview_organization(root.path(), &[txt_rule()], &ScanOptions::default()).unwrap();
        assert_eq!(preview.operations.len(), 1);
        let report = execute_preview(root.path(), &preview, journals.path()).unwrap();
        assert_eq!(report.completed, 1);
        let journal_path = journals.path().join(format!("{}.journal.json", report.journal.id));
        let undo = undo_journal(&journal_path).unwrap();
        assert_eq!(undo.completed, 1);
        assert!(root.path().join("note.txt").exists());
    }

    #[test]
    fn post_preview_destination_collision_is_never_overwritten() {
        let root = tempdir().unwrap();
        let journals = tempdir().unwrap();
        let source = root.path().join("note.txt");
        let destination = root.path().join("Text").join("note.txt");
        std::fs::write(&source, b"source").unwrap();
        let preview = preview_organization(root.path(), &[txt_rule()], &ScanOptions::default()).unwrap();
        assert_eq!(preview.operations.len(), 1);

        std::fs::create_dir_all(destination.parent().unwrap()).unwrap();
        std::fs::write(&destination, b"newer").unwrap();
        let report = execute_preview(root.path(), &preview, journals.path()).unwrap();

        assert_eq!(report.completed, 1);
        assert_eq!(std::fs::read(&destination).unwrap(), b"newer");
        assert_eq!(std::fs::read(root.path().join("Text").join("note (1).txt")).unwrap(), b"source");
    }

    #[test]
    fn relative_root_execution_produces_an_undoable_absolute_journal() {
        let current = std::env::current_dir().unwrap();
        let relative_name = format!(".sortsmith-relative-root-{}", Uuid::new_v4());
        let relative_root = PathBuf::from(&relative_name);
        let root = current.join(&relative_root);
        let journals = current.join(format!(".sortsmith-relative-journals-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("note.txt"), b"hello").unwrap();

        let preview = preview_organization(&relative_root, &[txt_rule()], &ScanOptions::default()).unwrap();
        let report = execute_preview(&relative_root, &preview, &journals).unwrap();
        assert!(report.journal.root.is_absolute());
        assert!(report.journal.entries[0].from.is_absolute());
        assert!(report.journal.entries[0].to.is_absolute());
        let journal_path = journals.join(format!("{}.journal.json", report.journal.id));
        assert_eq!(undo_journal(&journal_path).unwrap().completed, 1);
        assert!(root.join("note.txt").exists());

        std::fs::remove_dir_all(&root).unwrap();
        std::fs::remove_dir_all(&journals).unwrap();
    }

    #[test]
    fn rejects_invalid_rules_before_scanning() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let mut invalid = txt_rule();
        invalid.action = RuleAction::MoveTo { subdirectory: "../outside".into() };
        assert!(preview_organization(root.path(), &[invalid], &ScanOptions::default()).is_err());
    }

    #[test]
    fn journal_preflight_failure_does_not_move_files() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let preview = preview_organization(root.path(), &[txt_rule()], &ScanOptions::default()).unwrap();
        let blocked_journal_dir = root.path().join("journal-blocker");
        std::fs::write(&blocked_journal_dir, b"not a directory").unwrap();

        assert!(execute_preview(root.path(), &preview, &blocked_journal_dir).is_err());
        assert!(root.path().join("note.txt").exists());
        assert!(!root.path().join("Text").join("note.txt").exists());
    }

    #[test]
    fn forged_preview_cannot_move_outside_root() {
        let root = tempdir().unwrap();
        let journals = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let mut preview = preview_organization(root.path(), &[txt_rule()], &ScanOptions::default()).unwrap();
        let outside = journals.path().join("escaped.txt");
        preview.operations[0].destination = outside.clone();

        assert!(execute_preview(root.path(), &preview, journals.path()).is_err());
        assert!(root.path().join("note.txt").exists());
        assert!(!outside.exists());
    }

    #[test]
    fn preview_reserves_duplicate_destinations() {
        let root = tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("first")).unwrap();
        std::fs::create_dir_all(root.path().join("second")).unwrap();
        std::fs::write(root.path().join("first").join("note.txt"), b"one").unwrap();
        std::fs::write(root.path().join("second").join("note.txt"), b"two").unwrap();
        let options = ScanOptions { recursive: true, include_hidden: false, follow_links: false, max_depth: Some(8) };
        let preview = preview_organization(root.path(), &[txt_rule()], &options).unwrap();
        assert_eq!(preview.operations.len(), 2);
        assert_ne!(preview.operations[0].destination, preview.operations[1].destination);
        let names = preview.operations.iter().map(|operation| operation.destination.file_name().unwrap().to_string_lossy().into_owned()).collect::<HashSet<_>>();
        assert_eq!(names.len(), 2);
        assert!(names.contains("note.txt"));
        assert!(names.contains("note (1).txt"));
    }

    #[test]
    fn forged_journal_cannot_undo_outside_recorded_root() {
        let root = tempdir().unwrap();
        let outside = tempdir().unwrap();
        let journal_dir = tempdir().unwrap();
        let source = outside.path().join("secret.txt");
        let destination = outside.path().join("moved.txt");
        std::fs::write(&destination, b"secret").unwrap();
        let journal = OperationJournal {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            root: root.path().to_path_buf(),
            entries: vec![JournalEntry { operation_id: Uuid::new_v4(), from: source, to: destination.clone() }],
        };
        let path = save_journal(journal_dir.path(), &journal).unwrap();

        assert!(undo_journal(&path).is_err());
        assert!(destination.exists());
    }

    #[cfg(unix)]
    #[test]
    fn preview_skips_file_symlink_that_resolves_outside_root() {
        use std::os::unix::fs::symlink;
        let root = tempdir().unwrap();
        let outside = tempdir().unwrap();
        std::fs::write(outside.path().join("note.txt"), b"secret").unwrap();
        symlink(outside.path().join("note.txt"), root.path().join("linked.txt")).unwrap();

        let options = ScanOptions { recursive: false, include_hidden: false, follow_links: true, max_depth: Some(8) };
        let preview = preview_organization(root.path(), &[txt_rule()], &options).unwrap();
        assert!(preview.operations.is_empty());
        assert_eq!(preview.ignored_files, 1);
        assert!(preview.recoverable_errors.iter().any(|error| error.contains("symbolic link")));
    }

    #[cfg(unix)]
    #[test]
    fn preview_prunes_external_symlink_directories_before_traversal() {
        use std::os::unix::fs::symlink;
        let root = tempdir().unwrap();
        let outside = tempdir().unwrap();
        std::fs::create_dir_all(outside.path().join("nested")).unwrap();
        std::fs::write(outside.path().join("nested").join("note.txt"), b"secret").unwrap();
        symlink(outside.path(), root.path().join("external")).unwrap();

        let options = ScanOptions { recursive: true, include_hidden: false, follow_links: true, max_depth: Some(32) };
        let preview = preview_organization(root.path(), &[txt_rule()], &options).unwrap();
        assert!(preview.operations.is_empty());
        assert_eq!(preview.scanned_files, 0);
        assert_eq!(preview.ignored_files, 0);
    }
}
