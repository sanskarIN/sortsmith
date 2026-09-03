# Changelog

All notable changes follow Keep a Changelog principles and Semantic Versioning.

## [Unreleased]

Development continues on the next feature line.

## [0.1.1] - 2026-09-03

Patch release focused on release metadata consistency and Unicode-safe rule validation.

### Fixed
- Rule value and filename-regex limits now count Unicode characters rather than UTF-8 bytes, so valid non-ASCII rules are not rejected prematurely.
- Added regression coverage for 128-character and 129-character Unicode rule values.

### Release Engineering
- Synchronized the Rust workspace, desktop package, and Tauri application versions at `0.1.1`.
- Prepared a dedicated `release/0.1.1` maintenance line from the final `0.1.0` source commit.
- Kept the patch release scoped to maintenance changes so the later `0.2.x` and `0.3.x` feature lines remain distinct.

## [0.1.0] - 2026-08-21

Initial development baseline. The release tag must only be published after cross-platform CI and installer verification are green.

### Included in the baseline
- Production-oriented Rust/Tauri/React SortSmith baseline.
- Rules for extension, MIME prefix, age, size, and filename regex, including a multi-criterion rule builder and reusable presets.
- Saved user-defined preset management: snapshot active rules, load a preset into the active rule set, edit custom preset metadata, and safely delete presets that are not assigned to watched folders.
- Dry-run organization, collision-safe moves/renames, reversible journals, latest undo, and selectable operation-history undo.
- Duplicate detection with BLAKE3 and no automatic deletion.
- Watched-folder automation with user-controlled presets and intervals while the desktop app is open.
- Native folder selection plus native settings backup/import dialogs.
- Keyboard-first quick actions for page navigation, folder selection, preview, apply, and undo, plus an in-app shortcut reference.
- Schema-versioned local settings, bounded JSON persistence, and privacy-safe rotating operation logs.
- Property-based core coverage for serialization, path traversal rejection, portable filenames, and rename-template extension preservation.
- Criterion benchmark targets for organization planning and duplicate hashing using isolated synthetic directory fixtures.
- Responsive light/dark/system UI with accessibility baseline and About/support/funding information.
- Full project documentation, ADRs, contribution/security/privacy policies, CI, Dependabot, CodeQL, issue/PR templates, and cross-platform release automation.
