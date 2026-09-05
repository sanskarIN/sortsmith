use proptest::prelude::*;
use sortsmith_core::models::{FileEntry, Rule, RuleAction, RuleCriterion};
use sortsmith_core::rules::{destination_for, validate_rule};
use sortsmith_core::safety::{safe_subdirectory, validate_filename};
use std::path::{Path, PathBuf};
use uuid::Uuid;

fn extension_rule(action: RuleAction) -> Rule {
    Rule {
        id: Uuid::new_v4(),
        name: "property rule".into(),
        enabled: true,
        match_all: true,
        criteria: vec![RuleCriterion::Extension {
            values: vec!["txt".into()],
        }],
        action,
    }
}

proptest! {
    #[test]
    fn size_ranges_round_trip_through_json(minimum in any::<u64>(), delta in 0u64..=1_000_000) {
        let maximum = minimum.saturating_add(delta);
        let original = RuleCriterion::SizeRange { min_bytes: Some(minimum), max_bytes: Some(maximum) };
        let encoded = serde_json::to_string(&original).unwrap();
        let decoded: RuleCriterion = serde_json::from_str(&encoded).unwrap();
        prop_assert_eq!(decoded, original);
    }

    #[test]
    fn safe_relative_subdirectories_remain_below_root(segments in prop::collection::vec("[A-Za-z0-9_-]{1,16}", 1..6)) {
        let root = Path::new("sortsmith-property-root");
        let relative = segments.join("/");
        let destination = safe_subdirectory(root, &relative).unwrap();
        prop_assert!(destination.starts_with(root));
        prop_assert_ne!(destination, root);
    }

    #[test]
    fn parent_traversal_is_always_rejected(suffix in "[A-Za-z0-9_-]{0,32}") {
        let candidate = if suffix.is_empty() { "..".to_string() } else { format!("../{suffix}") };
        prop_assert!(safe_subdirectory(Path::new("root"), &candidate).is_err());
    }

    #[test]
    fn portable_ascii_filenames_validate(stem in "[A-Za-z0-9_-]{1,120}", extension in "[A-Za-z0-9]{1,8}") {
        let filename = format!("file_{stem}.{extension}");
        prop_assert!(validate_filename(&filename, "generated filename").is_ok());
    }

    #[test]
    fn rename_templates_preserve_txt_extension(stem in "[A-Za-z0-9_-]{1,64}", suffix in "[A-Za-z0-9_-]{1,24}") {
        let root = Path::new("root");
        let file = FileEntry {
            path: PathBuf::from("root").join(format!("{stem}.txt")),
            relative_path: PathBuf::from(format!("{stem}.txt")),
            size: 1,
            modified_at: None,
            mime: Some("text/plain".into()),
            extension: Some("txt".into()),
        };
        let rule = extension_rule(RuleAction::RenameTemplate { template: format!("{{name}}-{suffix}.{{ext}}") });
        validate_rule(&rule).unwrap();
        let destination = destination_for(root, &file, &rule).unwrap();
        prop_assert_eq!(destination.extension().and_then(|value| value.to_str()), Some("txt"));
        prop_assert!(destination.starts_with(root));
    }
}
