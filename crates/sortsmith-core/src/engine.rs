use crate::error::io;
use crate::journal::{load_journal, save_journal};
use crate::models::*;
use crate::rules::{destination_for, PreparedRule};
use crate::safety::collision_safe_path;
use crate::Result;
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

pub fn preview_organization(root: &Path, rules: &[Rule], options: &ScanOptions) -> Result<PreviewResult> {
    let prepared_rules = rules.iter().filter(|rule| rule.enabled).map(PreparedRule::new).collect::<Result<Vec<_>>>()?;
    let mut result = PreviewResult::default();
    let depth = if options.recursive { options.max_depth.unwrap_or(32) } else { 1 };
    let walker = WalkDir::new(root).follow_links(options.follow_links).max_depth(depth);
    for item in walker.into_iter().filter_entry(|e| options.include_hidden || !is_hidden(e, root)) {
        let entry = match item {
            Ok(v) => v,
            Err(err) => { result.recoverable_errors.push(redact_walk_error(&err)); continue; }
        };
        if !entry.file_type().is_file() { continue; }
        result.scanned_files += 1;
        let file = match describe_file(root, entry.path()) {
            Ok(f) => f,
            Err(err) => { result.recoverable_errors.push(err.to_string()); continue; }
        };
        let mut planned = false;
        for prepared in &prepared_rules {
            if prepared.matches(&file) {
                let rule = prepared.rule();
                let destination = collision_safe_path(&destination_for(root, &file, rule)?);
                if destination != file.path {
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

pub fn execute_preview(root: &Path, preview: &PreviewResult, journal_dir: &Path) -> Result<ExecutionReport> {
    fs::create_dir_all(journal_dir).map_err(|e| io(journal_dir, e))?;
    let mut journal = OperationJournal { id: Uuid::new_v4(), created_at: Utc::now(), root: root.to_path_buf(), entries: Vec::new() };
    let mut errors = Vec::new();
    for op in &preview.operations {
        let destination = collision_safe_path(&op.destination);
        if let Some(parent) = destination.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                errors.push(format!("Could not create destination folder: {err}"));
                continue;
            }
        }
        match fs::rename(&op.source, &destination) {
            Ok(()) => journal.entries.push(JournalEntry { operation_id: op.id, from: op.source.clone(), to: destination }),
            Err(rename_error) if rename_error.kind() == std::io::ErrorKind::CrossesDevices => {
                match copy_then_remove(&op.source, &destination) {
                    Ok(()) => journal.entries.push(JournalEntry { operation_id: op.id, from: op.source.clone(), to: destination }),
                    Err(err) => errors.push(format!("Could not move a file across devices: {err}")),
                }
            }
            Err(err) => errors.push(format!("Could not move a file: {err}")),
        }
    }
    save_journal(journal_dir, &journal)?;
    Ok(ExecutionReport { completed: journal.entries.len(), journal, errors })
}

pub fn undo_journal(journal_path: &Path) -> Result<ExecutionReport> {
    let journal = load_journal(journal_path)?;
    let mut completed = 0usize;
    let mut errors = Vec::new();
    for entry in journal.entries.iter().rev() {
        if !entry.to.exists() { errors.push("A moved file is missing; skipped during undo.".into()); continue; }
        if entry.from.exists() { errors.push("Original path is occupied; skipped during undo.".into()); continue; }
        if let Some(parent) = entry.from.parent() { let _ = fs::create_dir_all(parent); }
        match fs::rename(&entry.to, &entry.from) {
            Ok(()) => completed += 1,
            Err(err) if err.kind() == std::io::ErrorKind::CrossesDevices => match copy_then_remove(&entry.to, &entry.from) { Ok(()) => completed += 1, Err(e) => errors.push(format!("Undo failed across devices: {e}")) },
            Err(err) => errors.push(format!("Undo failed: {err}")),
        }
    }
    Ok(ExecutionReport { journal, completed, errors })
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

fn copy_then_remove(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::copy(source, destination)?;
    if let Err(err) = fs::remove_file(source) {
        let _ = fs::remove_file(destination);
        return Err(err);
    }
    Ok(())
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
    fn rejects_invalid_rules_before_scanning() {
        let root = tempdir().unwrap();
        std::fs::write(root.path().join("note.txt"), b"hello").unwrap();
        let mut invalid = txt_rule();
        invalid.action = RuleAction::MoveTo { subdirectory: "../outside".into() };
        assert!(preview_organization(root.path(), &[invalid], &ScanOptions::default()).is_err());
    }
}
