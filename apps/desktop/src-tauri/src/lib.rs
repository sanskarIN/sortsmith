#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Duration, Utc};
use sortsmith_core::{find_duplicates, execute_preview, preview_organization, undo_journal, AppStateData, DuplicateGroup, ExecutionReport, OperationJournal, PreviewResult, Rule, ScanOptions};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Component, Path, PathBuf};
use tauri::{AppHandle, Manager};

const MAX_STATE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_OPERATION_LOG_BYTES: u64 = 5 * 1024 * 1024;
const MAX_CUSTOM_RULES: usize = 500;
const MAX_PRESETS: usize = 50;
const MAX_RULES_PER_PRESET: usize = 500;
const MAX_WATCHED_FOLDERS: usize = 100;
const MAX_RECENT_JOURNALS: usize = 100;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JournalSummary {
    id: uuid::Uuid,
    created_at: DateTime<Utc>,
    root: PathBuf,
    entry_count: usize,
    available_to_undo: usize,
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path().app_data_dir().map_err(|e| format!("Could not resolve app data directory: {e}"))
}

fn state_path(app: &AppHandle) -> Result<PathBuf, String> { Ok(app_data_dir(app)?.join("state.json")) }
fn journals_dir(app: &AppHandle) -> Result<PathBuf, String> { Ok(app_data_dir(app)?.join("journals")) }

#[tauri::command]
fn load_state(app: AppHandle) -> Result<AppStateData, String> {
    let path = state_path(&app)?;
    if !path.exists() {
        let state = AppStateData::default();
        save_state(app, state.clone())?;
        return Ok(state);
    }
    let metadata = fs::symlink_metadata(&path).map_err(|e| format!("Could not inspect saved settings: {e}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err("Saved settings must be a regular local file.".into());
    }
    if metadata.len() > MAX_STATE_BYTES {
        return Err("Saved settings exceed the supported local state size.".into());
    }
    let file = File::open(&path).map_err(|e| format!("Could not read settings: {e}"))?;
    let state: AppStateData = serde_json::from_reader(BufReader::new(file)).map_err(|e| format!("Saved settings are invalid: {e}"))?;
    validate_state_data(&state)?;
    Ok(state)
}

#[tauri::command]
fn save_state(app: AppHandle, state: AppStateData) -> Result<(), String> {
    validate_state_data(&state)?;
    let path = state_path(&app)?;
    atomic_json_write(&path, &state)
}

#[tauri::command]
fn preview(root: String, rules: Vec<Rule>, recursive: bool, include_hidden: bool) -> Result<PreviewResult, String> {
    let root = validated_root(&root)?;
    let options = ScanOptions { recursive, include_hidden, follow_links: false, max_depth: Some(32) };
    preview_organization(&root, &rules, &options).map_err(|e| e.to_string())
}

#[tauri::command]
fn execute(app: AppHandle, root: String, preview: PreviewResult) -> Result<ExecutionReport, String> {
    let root = validated_root(&root)?;
    for op in &preview.operations {
        if !safe_operation_path(&root, &op.source, true) || !safe_operation_path(&root, &op.destination, false) {
            return Err("A planned operation escaped the selected root and was blocked.".into());
        }
    }
    let report = execute_preview(&root, &preview, &journals_dir(&app)?).map_err(|e| e.to_string())?;
    append_operation_log(&app, "execute", report.journal.id, report.completed, report.errors.len());
    Ok(report)
}

#[tauri::command]
fn undo(app: AppHandle, journal_id: String) -> Result<ExecutionReport, String> {
    let id = uuid::Uuid::parse_str(&journal_id).map_err(|_| "Invalid journal identifier.".to_string())?;
    let path = journals_dir(&app)?.join(format!("{id}.journal.json"));
    let journal = sortsmith_core::journal::load_journal(&path).map_err(|e| e.to_string())?;
    validate_journal_paths(&journal)?;
    let report = undo_journal(&path).map_err(|e| e.to_string())?;
    append_operation_log(&app, "undo", report.journal.id, report.completed, report.errors.len());
    Ok(report)
}

#[tauri::command]
fn list_journals(app: AppHandle) -> Result<Vec<JournalSummary>, String> {
    let dir = journals_dir(&app)?;
    if !dir.exists() { return Ok(Vec::new()); }
    let entries = fs::read_dir(&dir).map_err(|_| "Could not read local undo history.".to_string())?;
    let mut summaries = Vec::new();

    for item in entries {
        let Ok(item) = item else { continue; };
        let path = item.path();
        let is_journal = path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.ends_with(".journal.json"));
        if !is_journal { continue; }
        let Ok(metadata) = fs::symlink_metadata(&path) else { continue; };
        if metadata.file_type().is_symlink() || !metadata.is_file() { continue; }
        let Ok(journal) = sortsmith_core::journal::load_journal(&path) else { continue; };
        let available_to_undo = journal.entries.iter().filter(|entry| entry.to.exists() && !entry.from.exists()).count();
        summaries.push(JournalSummary { id: journal.id, created_at: journal.created_at, root: journal.root, entry_count: journal.entries.len(), available_to_undo });
    }

    summaries.sort_by(|left, right| right.created_at.cmp(&left.created_at));
    summaries.truncate(100);
    Ok(summaries)
}

#[tauri::command]
fn find_duplicate_candidates(root: String, recursive: bool, include_hidden: bool) -> Result<Vec<DuplicateGroup>, String> {
    let root = validated_root(&root)?;
    find_duplicates(&root, &ScanOptions { recursive, include_hidden, follow_links: false, max_depth: Some(64) }).map_err(|e| e.to_string())
}

#[tauri::command]
fn export_state(path: String, state: AppStateData) -> Result<(), String> {
    validate_state_data(&state)?;
    let target = validated_json_export_path(&path)?;
    atomic_json_write(&target, &state)
}

#[tauri::command]
fn import_state(path: String) -> Result<AppStateData, String> {
    let source = validated_json_import_path(&path)?;
    let metadata = fs::metadata(&source).map_err(|_| "Could not inspect the selected import file.".to_string())?;
    if metadata.len() > MAX_STATE_BYTES { return Err("The selected settings file is too large to be a SortSmith export.".into()); }
    let file = File::open(&source).map_err(|_| "Could not read the selected import file.".to_string())?;
    let state: AppStateData = serde_json::from_reader(BufReader::new(file)).map_err(|_| "The selected file is not a valid SortSmith settings export.".to_string())?;
    validate_state_data(&state)?;
    Ok(state)
}

#[tauri::command]
fn run_due_watches(app: AppHandle) -> Result<Vec<String>, String> {
    let mut state = load_state(app.clone())?;
    let mut messages = Vec::new();
    let preset_map = state.presets.iter().map(|p| (p.id, p.rules.clone())).collect::<std::collections::HashMap<_,_>>();
    let journals = journals_dir(&app)?;
    for watch in &mut state.watched_folders {
        if !watch.enabled || watch.interval_minutes < 5 { continue; }
        let due = watch.last_run_at.is_none_or(|last| Utc::now() >= last + Duration::minutes(i64::from(watch.interval_minutes)));
        if !due { continue; }
        let Ok(root) = validated_root(&watch.path.to_string_lossy()) else { messages.push("A watched folder is unavailable.".into()); continue; };
        let rules = watch.preset_id.and_then(|id| preset_map.get(&id).cloned()).unwrap_or_default();
        if rules.is_empty() { messages.push("A watched folder has no usable preset.".into()); continue; }
        let options = ScanOptions { recursive: state.settings.recursive_scan, include_hidden: state.settings.include_hidden, follow_links: false, max_depth: Some(32) };
        match preview_organization(&root, &rules, &options).and_then(|p| execute_preview(&root, &p, &journals)) {
            Ok(report) => { watch.last_run_at = Some(Utc::now()); state.recent_journal_ids.insert(0, report.journal.id); append_operation_log(&app, "watch", report.journal.id, report.completed, report.errors.len()); messages.push(format!("A watched folder completed {} change(s).", report.completed)); }
            Err(_) => messages.push("A watched folder run could not complete safely.".into()),
        }
    }
    state.recent_journal_ids.truncate(20);
    save_state(app, state)?;
    Ok(messages)
}

fn validate_state_data(state: &AppStateData) -> Result<(), String> {
    if state.schema_version != 1 { return Err("Unsupported settings schema version.".into()); }
    if !matches!(state.settings.theme.as_str(), "system" | "light" | "dark") { return Err("Settings contain an unsupported theme value.".into()); }
    if state.rules.len() > MAX_CUSTOM_RULES || state.presets.len() > MAX_PRESETS || state.watched_folders.len() > MAX_WATCHED_FOLDERS || state.recent_journal_ids.len() > MAX_RECENT_JOURNALS {
        return Err("Settings contain more items than this SortSmith build supports.".into());
    }

    validate_unique_rule_ids(&state.rules)?;
    for rule in &state.rules { sortsmith_core::rules::validate_rule(rule).map_err(|e| e.to_string())?; }

    let mut preset_ids = HashSet::new();
    for preset in &state.presets {
        if !preset_ids.insert(preset.id) { return Err("Settings contain duplicate preset identifiers.".into()); }
        if preset.name.trim().is_empty() || preset.name.chars().count() > 128 || preset.description.chars().count() > 512 || preset.rules.len() > MAX_RULES_PER_PRESET {
            return Err("Settings contain an invalid preset definition.".into());
        }
        validate_unique_rule_ids(&preset.rules)?;
        for rule in &preset.rules { sortsmith_core::rules::validate_rule(rule).map_err(|e| e.to_string())?; }
    }

    let mut watch_ids = HashSet::new();
    for watch in &state.watched_folders {
        if !watch_ids.insert(watch.id) { return Err("Settings contain duplicate watched-folder identifiers.".into()); }
        let path_text = watch.path.to_string_lossy();
        if path_text.trim().is_empty() || path_text.chars().count() > 4_096 || !(5..=10_080).contains(&watch.interval_minutes) {
            return Err("Settings contain an invalid watched-folder entry.".into());
        }
        if let Some(preset_id) = watch.preset_id {
            if !preset_ids.contains(&preset_id) { return Err("A watched folder references a preset that is not present in this backup.".into()); }
        }
    }

    let mut journal_ids = HashSet::new();
    if state.recent_journal_ids.iter().any(|id| !journal_ids.insert(*id)) { return Err("Settings contain duplicate undo-history identifiers.".into()); }
    Ok(())
}

fn validate_unique_rule_ids(rules: &[Rule]) -> Result<(), String> {
    let mut ids = HashSet::new();
    if rules.iter().any(|rule| !ids.insert(rule.id)) { return Err("Settings contain duplicate rule identifiers.".into()); }
    Ok(())
}

fn validate_journal_paths(journal: &OperationJournal) -> Result<(), String> {
    let root = validated_root(&journal.root.to_string_lossy()).map_err(|_| "The journal root is unavailable or invalid.".to_string())?;
    for entry in &journal.entries {
        if !safe_operation_path(&root, &entry.to, false) || !safe_operation_path(&root, &entry.from, false) {
            return Err("The undo journal contains a path outside its recorded root and was blocked.".into());
        }
    }
    Ok(())
}

fn safe_operation_path(root: &Path, path: &Path, must_exist: bool) -> bool {
    if must_exist { return path.canonicalize().is_ok_and(|candidate| candidate.starts_with(root)); }
    let Ok(relative) = path.strip_prefix(root) else { return false; };
    if relative.components().any(|component| matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_))) { return false; }
    let mut current = root.to_path_buf();
    if let Some(parent) = relative.parent() {
        for component in parent.components() {
            current.push(component.as_os_str());
            if current.exists() {
                let Ok(resolved) = current.canonicalize() else { return false; };
                if !resolved.starts_with(root) { return false; }
                current = resolved;
            }
        }
    }
    true
}

fn append_operation_log(app: &AppHandle, event: &str, journal_id: uuid::Uuid, completed: usize, error_count: usize) {
    let Ok(dir) = app_data_dir(app) else { return; };
    if fs::create_dir_all(&dir).is_err() { return; }
    let path = dir.join("operations.jsonl");
    if !prepare_operation_log(&path, MAX_OPERATION_LOG_BYTES) { return; }
    let record = serde_json::json!({ "timestamp": Utc::now(), "event": event, "journalId": journal_id, "completed": completed, "errorCount": error_count });
    if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(&path) { let _ = writeln!(file, "{}", record); }
}

fn prepare_operation_log(path: &Path, max_bytes: u64) -> bool {
    if !path.exists() { return true; }
    let Ok(metadata) = fs::symlink_metadata(path) else { return false; };
    if metadata.file_type().is_symlink() || !metadata.is_file() { return false; }
    if metadata.len() < max_bytes { return true; }

    let previous = path.with_file_name("operations.previous.jsonl");
    if previous.exists() {
        let Ok(previous_metadata) = fs::symlink_metadata(&previous) else { return false; };
        if previous_metadata.file_type().is_symlink() || !previous_metadata.is_file() { return false; }
        if fs::remove_file(&previous).is_err() { return false; }
    }
    fs::rename(path, previous).is_ok()
}

fn validated_root(raw: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(raw.trim());
    if raw.trim().is_empty() || !path.exists() || !path.is_dir() { return Err("Choose an existing folder.".into()); }
    path.canonicalize().map_err(|e| format!("Could not resolve the selected folder: {e}"))
}

fn has_json_extension(path: &Path) -> bool { path.extension().and_then(|value| value.to_str()).is_some_and(|value| value.eq_ignore_ascii_case("json")) }

fn validated_json_import_path(raw: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(raw.trim());
    if raw.trim().is_empty() || !path.is_absolute() || !has_json_extension(&path) { return Err("Choose an absolute .json settings file.".into()); }
    let link_metadata = fs::symlink_metadata(&path).map_err(|_| "The selected import file is unavailable.".to_string())?;
    if link_metadata.file_type().is_symlink() || !link_metadata.is_file() { return Err("The selected import must be a regular JSON file, not a link or directory.".into()); }
    path.canonicalize().map_err(|_| "Could not resolve the selected import file.".to_string())
}

fn validated_json_export_path(raw: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(raw.trim());
    if raw.trim().is_empty() || !path.is_absolute() || !has_json_extension(&path) { return Err("Choose an absolute .json export path.".into()); }
    if path.exists() {
        let metadata = fs::symlink_metadata(&path).map_err(|_| "Could not inspect the selected export path.".to_string())?;
        if metadata.file_type().is_symlink() || !metadata.is_file() { return Err("The export target must be a regular JSON file.".into()); }
    }
    let parent = path.parent().ok_or_else(|| "The selected export path has no parent directory.".to_string())?;
    let parent = parent.canonicalize().map_err(|_| "The selected export directory is unavailable.".to_string())?;
    if !parent.is_dir() { return Err("The selected export directory is invalid.".into()); }
    let filename = path.file_name().ok_or_else(|| "The selected export filename is invalid.".to_string())?;
    Ok(parent.join(filename))
}

fn atomic_json_write(path: &Path, value: &impl serde::Serialize) -> Result<(), String> {
    let parent = path.parent().ok_or_else(|| "The data path has no parent directory.".to_string())?;
    fs::create_dir_all(parent).map_err(|e| format!("Could not create data directory: {e}"))?;
    let filename = path.file_name().and_then(|value| value.to_str()).unwrap_or("sortsmith.json");
    let nonce = uuid::Uuid::new_v4();
    let temp = parent.join(format!(".{filename}.{nonce}.tmp"));
    let backup = parent.join(format!(".{filename}.{nonce}.bak"));

    let file = File::create(&temp).map_err(|e| format!("Could not create temporary data file: {e}"))?;
    let mut writer = BufWriter::new(file);
    if let Err(error) = serde_json::to_writer_pretty(&mut writer, value) {
        drop(writer);
        let _ = fs::remove_file(&temp);
        return Err(format!("Could not serialize data: {error}"));
    }
    if let Err(error) = writer.flush() {
        drop(writer);
        let _ = fs::remove_file(&temp);
        return Err(format!("Could not flush data: {error}"));
    }
    if let Err(error) = writer.get_ref().sync_all() {
        drop(writer);
        let _ = fs::remove_file(&temp);
        return Err(format!("Could not sync data: {error}"));
    }
    let size = writer.get_ref().metadata().map_err(|e| format!("Could not inspect temporary data: {e}"))?.len();
    drop(writer);
    if size > MAX_STATE_BYTES {
        let _ = fs::remove_file(&temp);
        return Err("Settings exceed the supported local state size.".into());
    }

    if !path.exists() { return fs::rename(&temp, path).map_err(|e| { let _ = fs::remove_file(&temp); format!("Could not finalize data file: {e}") }); }
    fs::rename(path, &backup).map_err(|e| { let _ = fs::remove_file(&temp); format!("Could not prepare the existing data file for replacement: {e}") })?;
    match fs::rename(&temp, path) {
        Ok(()) => { let _ = fs::remove_file(&backup); Ok(()) }
        Err(error) => { let _ = fs::rename(&backup, path); let _ = fs::remove_file(&temp); Err(format!("Could not finalize data file: {error}")) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_operation_log_at_limit() {
        let dir = std::env::temp_dir().join(format!("sortsmith-log-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("operations.jsonl");
        fs::write(&path, b"12345").unwrap();

        assert!(prepare_operation_log(&path, 5));
        assert!(!path.exists());
        assert_eq!(fs::read(dir.join("operations.previous.jsonl")).unwrap(), b"12345");

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn leaves_small_operation_log_in_place() {
        let dir = std::env::temp_dir().join(format!("sortsmith-log-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("operations.jsonl");
        fs::write(&path, b"1234").unwrap();

        assert!(prepare_operation_log(&path, 5));
        assert!(path.exists());
        assert!(!dir.join("operations.previous.jsonl").exists());

        fs::remove_dir_all(dir).unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![load_state, save_state, preview, execute, undo, list_journals, find_duplicate_candidates, export_state, import_state, run_due_watches])
        .run(tauri::generate_context!())
        .expect("error while running SortSmith");
}
