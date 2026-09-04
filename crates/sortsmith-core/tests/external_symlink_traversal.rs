#![cfg(unix)]

use sortsmith_core::{preview_organization, Rule, RuleAction, RuleCriterion, ScanOptions};
use std::os::unix::fs::symlink;
use tempfile::tempdir;
use uuid::Uuid;

fn text_rule() -> Rule {
    Rule {
        id: Uuid::new_v4(),
        name: "Text files".into(),
        enabled: true,
        match_all: true,
        criteria: vec![RuleCriterion::Extension { values: vec!["txt".into()] }],
        action: RuleAction::MoveTo { subdirectory: "Text".into() },
    }
}

#[test]
fn recursive_preview_does_not_traverse_external_symlink_directory() {
    let root = tempdir().unwrap();
    let external = tempdir().unwrap();
    let nested = external.path().join("nested");
    std::fs::create_dir_all(&nested).unwrap();
    let external_file = nested.join("outside.txt");
    std::fs::write(&external_file, b"outside").unwrap();

    let linked_directory = root.path().join("linked");
    symlink(external.path(), &linked_directory).unwrap();

    let options = ScanOptions {
        recursive: true,
        include_hidden: false,
        follow_links: true,
        max_depth: Some(32),
    };

    let preview = preview_organization(root.path(), &[text_rule()], &options).unwrap();

    assert!(preview.operations.is_empty());
    assert_eq!(preview.scanned_files, 0);
    assert!(preview.recoverable_errors.is_empty());
    assert!(external_file.exists());
}
