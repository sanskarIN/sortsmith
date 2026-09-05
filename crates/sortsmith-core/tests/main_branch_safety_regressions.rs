use sortsmith_core::{preview_organization, Rule, RuleAction, RuleCriterion, ScanOptions};
use tempfile::tempdir;
use uuid::Uuid;

fn text_rule() -> Rule {
    Rule {
        id: Uuid::new_v4(),
        name: "Text files".into(),
        enabled: true,
        match_all: true,
        criteria: vec![RuleCriterion::Extension {
            values: vec!["txt".into()],
        }],
        action: RuleAction::MoveTo {
            subdirectory: "Text".into(),
        },
    }
}

#[cfg(unix)]
#[test]
fn main_branch_does_not_traverse_external_symlink_directories() {
    use std::os::unix::fs::symlink;

    let root = tempdir().unwrap();
    let outside = tempdir().unwrap();
    std::fs::create_dir_all(outside.path().join("nested")).unwrap();
    std::fs::write(outside.path().join("nested").join("secret.txt"), b"secret").unwrap();
    symlink(outside.path(), root.path().join("external")).unwrap();

    let options = ScanOptions {
        recursive: true,
        include_hidden: false,
        follow_links: true,
        max_depth: Some(32),
    };
    let preview = preview_organization(root.path(), &[text_rule()], &options).unwrap();

    assert!(preview.operations.is_empty());
    assert_eq!(preview.scanned_files, 0);
}

#[cfg(unix)]
#[test]
fn main_branch_ignores_external_file_symlinks() {
    use std::os::unix::fs::symlink;

    let root = tempdir().unwrap();
    let outside = tempdir().unwrap();
    std::fs::write(outside.path().join("secret.txt"), b"secret").unwrap();
    symlink(outside.path().join("secret.txt"), root.path().join("linked.txt")).unwrap();

    let options = ScanOptions {
        recursive: false,
        include_hidden: false,
        follow_links: true,
        max_depth: Some(8),
    };
    let preview = preview_organization(root.path(), &[text_rule()], &options).unwrap();

    assert!(preview.operations.is_empty());
    assert_eq!(preview.ignored_files, 1);
    assert!(preview
        .recoverable_errors
        .iter()
        .any(|error| error.contains("symbolic link")));
}
