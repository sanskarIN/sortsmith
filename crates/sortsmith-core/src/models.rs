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
    Extension {
        values: Vec<String>,
    },
    MimePrefix {
        values: Vec<String>,
    },
    ModifiedOlderThanDays {
        days: u32,
    },
    SizeRange {
        #[serde(rename = "minBytes", alias = "min_bytes")]
        min_bytes: Option<u64>,
        #[serde(rename = "maxBytes", alias = "max_bytes")]
        max_bytes: Option<u64>,
    },
    NameRegex {
        pattern: String,
    },
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
        Self {
            recursive: false,
            include_hidden: false,
            follow_links: false,
            max_depth: Some(32),
        }
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
        Self {
            theme: "system".into(),
            reduced_motion: false,
            confirm_before_apply: true,
            include_hidden: false,
            recursive_scan: false,
        }
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
        Self {
            schema_version: 1,
            settings: AppSettings::default(),
            rules: Vec::new(),
            presets: default_presets(),
            watched_folders: Vec::new(),
            recent_journal_ids: Vec::new(),
        }
    }
}

const EVERYDAY_PRESET_ID: &str = "11111111-1111-4111-8111-111111111101";
const MEDIA_PRESET_ID: &str = "11111111-1111-4111-8111-111111111102";
const DEVELOPER_PRESET_ID: &str = "11111111-1111-4111-8111-111111111103";
const DOWNLOADS_PRESET_ID: &str = "11111111-1111-4111-8111-111111111104";

pub fn default_presets() -> Vec<Preset> {
    vec![
        preset(
            EVERYDAY_PRESET_ID,
            "Everyday tidy",
            "Sort common documents, images, archives, audio, and video into clear folders.",
            vec![
                extension_rule(
                    "Images",
                    &["jpg", "jpeg", "png", "gif", "webp", "svg", "heic"],
                    "Images",
                ),
                extension_rule(
                    "Documents",
                    &[
                        "pdf", "doc", "docx", "txt", "md", "rtf", "odt", "xls", "xlsx", "ppt",
                        "pptx",
                    ],
                    "Documents",
                ),
                extension_rule(
                    "Archives",
                    &["zip", "7z", "rar", "tar", "gz", "bz2", "xz"],
                    "Archives",
                ),
                extension_rule(
                    "Audio",
                    &["mp3", "wav", "flac", "m4a", "aac", "ogg"],
                    "Audio",
                ),
                extension_rule(
                    "Video",
                    &["mp4", "mkv", "mov", "webm", "avi", "m4v"],
                    "Video",
                ),
            ],
        ),
        preset(
            MEDIA_PRESET_ID,
            "Media library",
            "Group image, audio, and video files below a single Media folder.",
            vec![
                extension_rule(
                    "Media images",
                    &["jpg", "jpeg", "png", "gif", "webp", "svg", "heic", "avif"],
                    "Media/Images",
                ),
                extension_rule(
                    "Media audio",
                    &["mp3", "wav", "flac", "m4a", "aac", "ogg", "opus"],
                    "Media/Audio",
                ),
                extension_rule(
                    "Media video",
                    &["mp4", "mkv", "mov", "webm", "avi", "m4v"],
                    "Media/Video",
                ),
            ],
        ),
        preset(
            DEVELOPER_PRESET_ID,
            "Developer workspace",
            "Separate common source, data/configuration, and package files for project staging folders.",
            vec![
                extension_rule(
                    "Source code",
                    &[
                        "rs", "ts", "tsx", "js", "jsx", "py", "java", "kt", "swift", "go", "php",
                        "cs", "cpp", "c", "h", "hpp",
                    ],
                    "Development/Source",
                ),
                extension_rule(
                    "Data and configuration",
                    &["json", "yaml", "yml", "toml", "xml", "csv", "ini", "env"],
                    "Development/Data",
                ),
                extension_rule(
                    "Packages and archives",
                    &["zip", "7z", "tar", "gz", "tgz", "bz2", "xz"],
                    "Development/Packages",
                ),
            ],
        ),
        preset(
            DOWNLOADS_PRESET_ID,
            "Downloads cleanup",
            "Tidy common downloads into installers, archives, documents, and images.",
            vec![
                extension_rule(
                    "Installers",
                    &["exe", "msi", "msix", "dmg", "pkg", "deb", "rpm", "appimage"],
                    "Installers",
                ),
                extension_rule(
                    "Downloaded archives",
                    &["zip", "7z", "rar", "tar", "gz", "bz2", "xz"],
                    "Archives",
                ),
                extension_rule(
                    "Downloaded documents",
                    &[
                        "pdf", "doc", "docx", "txt", "md", "rtf", "odt", "xls", "xlsx", "ppt",
                        "pptx",
                    ],
                    "Documents",
                ),
                extension_rule(
                    "Downloaded images",
                    &["jpg", "jpeg", "png", "gif", "webp", "svg", "heic", "avif"],
                    "Images",
                ),
            ],
        ),
    ]
}

fn preset(id: &str, name: &str, description: &str, rules: Vec<Rule>) -> Preset {
    Preset {
        id: Uuid::parse_str(id).expect("bundled preset UUID must be valid"),
        name: name.into(),
        description: description.into(),
        rules,
    }
}

fn extension_rule(name: &str, values: &[&str], subdirectory: &str) -> Rule {
    Rule {
        id: Uuid::new_v4(),
        name: name.into(),
        enabled: true,
        match_all: true,
        criteria: vec![RuleCriterion::Extension {
            values: values.iter().map(|v| (*v).to_string()).collect(),
        }],
        action: RuleAction::MoveTo {
            subdirectory: subdirectory.into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn size_range_uses_frontend_camel_case_keys() {
        let criterion = RuleCriterion::SizeRange {
            min_bytes: Some(1_024),
            max_bytes: Some(2_048),
        };
        let json = serde_json::to_value(&criterion).unwrap();
        assert_eq!(json["kind"], "sizeRange");
        assert_eq!(json["minBytes"], 1_024);
        assert_eq!(json["maxBytes"], 2_048);
        assert!(json.get("min_bytes").is_none());
    }

    #[test]
    fn size_range_reads_legacy_snake_case_keys() {
        let criterion: RuleCriterion = serde_json::from_value(serde_json::json!({
            "kind": "sizeRange",
            "min_bytes": 10,
            "max_bytes": 20
        }))
        .unwrap();
        assert_eq!(
            criterion,
            RuleCriterion::SizeRange {
                min_bytes: Some(10),
                max_bytes: Some(20)
            }
        );
    }

    #[test]
    fn bundled_preset_ids_are_stable_and_unique() {
        let first = default_presets();
        let second = default_presets();
        let first_ids = first.iter().map(|preset| preset.id).collect::<Vec<_>>();
        let second_ids = second.iter().map(|preset| preset.id).collect::<Vec<_>>();
        assert_eq!(first_ids, second_ids);
        assert_eq!(
            first_ids.iter().copied().collect::<HashSet<_>>().len(),
            first_ids.len()
        );
    }

    #[test]
    fn bundled_preset_rules_are_valid() {
        for preset in default_presets() {
            assert!(
                !preset.rules.is_empty(),
                "{} should contain rules",
                preset.name
            );
            for rule in preset.rules {
                crate::rules::validate_rule(&rule).unwrap();
            }
        }
    }
}
