# Changelog

All notable changes follow Keep a Changelog principles and Semantic Versioning.

## [Unreleased]

### Added
- Production-oriented Rust/Tauri/React SortSmith baseline.
- Rules for extension, MIME prefix, age, size, and filename regex.
- Dry-run organization, collision-safe moves/renames, reversible journals, and undo.
- Duplicate detection with BLAKE3 and no automatic deletion.
- Presets, custom rules, watched folders, schema-versioned local settings, import/export backend commands, and operation logging.
- Responsive light/dark/system UI with accessibility baseline and About/support/funding information.
- Full project documentation, ADRs, contribution/security/privacy policies, CI, CodeQL, Dependabot, issue/PR templates, and release automation.

### Security
- Root canonicalization and planned-operation containment checks.
- Parent traversal rejection and destination symlink escape protection.
- No link-following during normal scans and no file-content/path data in structured operation logs.

## [0.1.0] - 2026-08-21

Initial development baseline. The release tag must only be published after cross-platform CI and installer verification are green.
