use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub size: u64,
    pub modified_at: Option<DateTime<Utc>>,
    pub mime: Option<String>,
    pub extension: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RuleCriterion {
    Extension { values: Vec<String> },
    MimePrefix { values: Vec<String> },
    ModifiedOlderThanDays { days: u32 },
    SizeRange { min_bytes: Option<u64>, max_bytes: Option<u64> },
    NameRegex { pattern: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RuleAction {
    MoveTo { subdirectory: String },
    RenamePrefix { prefix: String },
    RenameTemplate { template: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub match_all: bool,
    pub criteria: Vec<RuleCriterion>,
    pub action: RuleAction,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScanOptions {
    pub recursive: bool,
    pub include_hidden: bool,
    pub follow_links: bool,
    pub max_depth: Option<usize>,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self { recursive: false, include_hidden: false, follow_links: false, max_depth: Some(32) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlannedOperation {
    pub id: Uuid,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub rule_id: Uuid,
    pub rule_name: String,
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreviewResult {
    pub operations: Vec<PlannedOperation>,
    pub scanned_files: usize,
    pub ignored_files: usize,
    pub recoverable_errors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JournalEntry {
    pub operation_id: Uuid,
    pub from: PathBuf,
    pub to: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OperationJournal {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub root: PathBuf,
    pub entries: Vec<JournalEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionReport {
    pub journal: OperationJournal,
    pub completed: usize,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateFile {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateGroup {
    pub hash: String,
    pub size: u64,
    pub files: Vec<DuplicateFile>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WatchedFolder {
    pub id: Uuid,
    pub path: PathBuf,
    pub preset_id: Option<Uuid>,
    pub interval_minutes: u32,
    pub enabled: bool,
    pub last_run_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: String,
    pub reduced_motion: bool,
    pub confirm_before_apply: bool,
    pub include_hidden: bool,
    pub recursive_scan: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { theme: "system".into(), reduced_motion: false, confirm_before_apply: true, include_hidden: false, recursive_scan: false }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppStateData {
    pub schema_version: u32,
    pub settings: AppSettings,
    pub rules: Vec<Rule>,
    pub presets: Vec<Preset>,
    pub watched_folders: Vec<WatchedFolder>,
    pub recent_journal_ids: Vec<Uuid>,
}

impl Default for AppStateData {
    fn default() -> Self {
        Self { schema_version: 1, settings: AppSettings::default(), rules: Vec::new(), presets: default_presets(), watched_folders: Vec::new(), recent_journal_ids: Vec::new() }
    }
}

pub fn default_presets() -> Vec<Preset> {
    vec![Preset {
        id: Uuid::new_v4(),
        name: "Everyday tidy".into(),
        description: "Sort common documents, images, archives, audio, and video into clear folders.".into(),
        rules: vec![
            extension_rule("Images", &["jpg", "jpeg", "png", "gif", "webp", "svg", "heic"], "Images"),
            extension_rule("Documents", &["pdf", "doc", "docx", "txt", "md", "rtf", "odt", "xls", "xlsx", "ppt", "pptx"], "Documents"),
            extension_rule("Archives", &["zip", "7z", "rar", "tar", "gz", "bz2", "xz"], "Archives"),
            extension_rule("Audio", &["mp3", "wav", "flac", "m4a", "aac", "ogg"], "Audio"),
            extension_rule("Video", &["mp4", "mkv", "mov", "webm", "avi", "m4v"], "Video"),
        ],
    }]
}

fn extension_rule(name: &str, values: &[&str], subdirectory: &str) -> Rule {
    Rule {
        id: Uuid::new_v4(),
        name: name.into(),
        enabled: true,
        match_all: true,
        criteria: vec![RuleCriterion::Extension { values: values.iter().map(|v| (*v).to_string()).collect() }],
        action: RuleAction::MoveTo { subdirectory: subdirectory.into() },
    }
}
