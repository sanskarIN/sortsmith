#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Duration, Utc};
use sortsmith_core::{find_duplicates, execute_preview, preview_organization, undo_journal, AppStateData, DuplicateGroup, ExecutionReport, OperationJournal, PreviewResult, Rule, ScanOptions};
use std::fs;
use std::path::{Component, Path, PathBuf};
use tauri::{AppHandle, Manager};

const MAX_IMPORT_BYTES: u64 = 2 * 1024 * 1024;

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
    let bytes = fs::read(&path).map_err(|e| format!("Could not read settings: {e}"))?;
    let state: AppStateData = serde_json::from_slice(&bytes).map_err(|e| format!("Saved settings are invalid: {e}"))?;
    if state.schema_version != 1 { return Err("This SortSmith data version is not supported by this build.".into()); }
    Ok(state)
}

#[tauri::command]
fn save_state(app: AppHandle, state: AppStateData) -> Result<(), String> {
    if state.schema_version != 1 { return Err("Unsupported settings schema version.".into()); }
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
fn find_duplicate_candidates(root: String, recursive: bool, include_hidden: bool) -> Result<Vec<DuplicateGroup>, String> {
    let root = validated_root(&root)?;
    find_duplicates(&root, &ScanOptions { recursive, include_hidden, follow_links: false, max_depth: Some(64) }).map_err(|e| e.to_string())
}

#[tauri::command]
fn export_state(path: String, state: AppStateData) -> Result<(), String> {
    if state.schema_version != 1 { return Err("Unsupported settings schema version.".into()); }
    let target = validated_json_export_path(&path)?;
    atomic_json_write(&target, &state)
}

#[tauri::command]
fn import_state(path: String) -> Result<AppStateData, String> {
    let source = validated_json_import_path(&path)?;
    let metadata = fs::metadata(&source).map_err(|_| "Could not inspect the selected import file.".to_string())?;
    if metadata.len() > MAX_IMPORT_BYTES { return Err("The selected settings file is too large to be a SortSmith export.".into()); }
    let bytes = fs::read(&source).map_err(|_| "Could not read the selected import file.".to_string())?;
    let state: AppStateData = serde_json::from_slice(&bytes).map_err(|_| "The selected file is not a valid SortSmith settings export.".to_string())?;
    if state.schema_version != 1 { return Err("Unsupported import schema version.".into()); }
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

fn validate_journal_paths(journal: &OperationJournal) -> Result<(), String> {
    let root = validated_root(&journal.root.to_string_lossy()).map_err(|_| "The journal root is unavailable or invalid.".to_string())?;
    for entry in &journal.entries {
        if !safe_operation_path(&root, &entry.to, true) || !safe_operation_path(&root, &entry.from, false) {
            return Err("The undo journal contains a path outside its recorded root and was blocked.".into());
        }
    }
    Ok(())
}

fn safe_operation_path(root: &Path, path: &Path, must_exist: bool) -> bool {
    if must_exist {
        return path.canonicalize().is_ok_and(|candidate| candidate.starts_with(root));
    }

    let Ok(relative) = path.strip_prefix(root) else { return false; };
    if relative.components().any(|component| matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_))) {
        return false;
    }

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
    let record = serde_json::json!({
        "timestamp": Utc::now(),
        "event": event,
        "journalId": journal_id,
        "completed": completed,
        "errorCount": error_count
    });
    use std::io::Write;
    if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(dir.join("operations.jsonl")) {
        let _ = writeln!(file, "{}", record);
    }
}

fn validated_root(raw: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(raw.trim());
    if raw.trim().is_empty() || !path.exists() || !path.is_dir() { return Err("Choose an existing folder.".into()); }
    path.canonicalize().map_err(|e| format!("Could not resolve the selected folder: {e}"))
}

fn has_json_extension(path: &Path) -> bool {
    path.extension().and_then(|value| value.to_str()).is_some_and(|value| value.eq_ignore_ascii_case("json"))
}

fn validated_json_import_path(raw: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(raw.trim());
    if raw.trim().is_empty() || !path.is_absolute() || !has_json_extension(&path) {
        return Err("Choose an absolute .json settings file.".into());
    }
    let link_metadata = fs::symlink_metadata(&path).map_err(|_| "The selected import file is unavailable.".to_string())?;
    if link_metadata.file_type().is_symlink() || !link_metadata.is_file() {
        return Err("The selected import must be a regular JSON file, not a link or directory.".into());
    }
    path.canonicalize().map_err(|_| "Could not resolve the selected import file.".to_string())
}

fn validated_json_export_path(raw: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(raw.trim());
    if raw.trim().is_empty() || !path.is_absolute() || !has_json_extension(&path) {
        return Err("Choose an absolute .json export path.".into());
    }
    if path.exists() {
        let metadata = fs::symlink_metadata(&path).map_err(|_| "Could not inspect the selected export path.".to_string())?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err("The export target must be a regular JSON file.".into());
        }
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
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    fs::write(&temp, bytes).map_err(|e| format!("Could not write data: {e}"))?;

    if !path.exists() {
        return fs::rename(&temp, path).map_err(|e| format!("Could not finalize data file: {e}"));
    }

    fs::rename(path, &backup).map_err(|e| {
        let _ = fs::remove_file(&temp);
        format!("Could not prepare the existing data file for replacement: {e}")
    })?;

    match fs::rename(&temp, path) {
        Ok(()) => {
            let _ = fs::remove_file(&backup);
            Ok(())
        }
        Err(error) => {
            let _ = fs::rename(&backup, path);
            let _ = fs::remove_file(&temp);
            Err(format!("Could not finalize data file: {error}"))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![load_state, save_state, preview, execute, undo, find_duplicate_candidates, export_state, import_state, run_due_watches])
        .run(tauri::generate_context!())
        .expect("error while running SortSmith");
}
