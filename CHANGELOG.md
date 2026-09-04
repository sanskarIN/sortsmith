# Changelog

All notable changes follow Keep a Changelog principles and Semantic Versioning.

## [Unreleased]

Development continues on the next feature line.

## [0.1.6] - 2026-09-04

Stable maintenance release focused on recursive symbolic-link traversal safety, portable Windows filename validation, and collision planning robustness.

### Added
- Added a Unix integration test through the public `sortsmith-core` API covering a recursive scan with `follow_links` enabled and an external symbolic-link directory.
- The integration test verifies that an external linked directory produces no planned organization operation and that its nested external file is not counted as scanned.
- Added regression coverage proving generated collision names remain within the portable 255-byte and 255-UTF-16-unit filename limits.

### Fixed
- Filename validation now rejects Unicode superscript aliases for numbered Windows `COM` and `LPT` device names, including `COM¹` and `LPT²`.
- Reserved destination comparison is case-insensitive on Windows so differently cased path spellings cannot consume the same physical destination slot.
- Collision suffix generation now fits the original filename stem to the remaining portable filename budget instead of blindly producing an overlong `name (1)` candidate.

### Quality
- Promoted the v0.1.5 symlink traversal protection into public API-level regression coverage, reducing the chance that future refactors bypass the scanner boundary.
- Added explicit regression coverage for the Unicode Windows device-name aliases.
- Added a collision-boundary regression test for long filenames.

### Release Engineering
- Synchronized the Rust workspace, desktop package, and Tauri application versions at `0.1.6`.
- Prepared a dedicated `release/0.1.6` maintenance branch from `release/0.1.5`.
- CI now runs on `release/**` branch pushes, so maintenance release branches receive the same core, desktop, and frontend validation gates as `main`.
- Prepared stable v0.1.6 release notes and publication checklist.

## [0.1.5] - 2026-09-04

Stable maintenance patch focused on preventing recursive traversal from escaping the selected folder through symbolic-link directories.

### Fixed
- Preview traversal now prunes symbolic-link entries whose resolved targets are outside the selected root before WalkDir descends into them.
- External symlink directories are rejected at the traversal boundary instead of allowing an external tree to be visited and filtered file-by-file afterward.
- Existing file-level external-symlink rejection remains in place as defense-in-depth.

### Security
- Recursive scans with `follow_links` enabled now apply the selected-root boundary before descending through a symbolic-link directory.
- This reduces unintended external traversal and the amount of external filesystem metadata that SortSmith needs to inspect.

### Tests
- Added Unix regression coverage for a symlinked directory that points outside the selected root and contains a nested matching file.
- Existing file-symlink, duplicate-destination, journal-integrity, traversal, Unicode-rule, and filename-safety coverage remains in place.

### Release Engineering
- Synchronized the Rust workspace, desktop package, and Tauri application versions at `0.1.5`.
- Finalized the stable v0.1.5 release notes and publication checklist.
- Continued the dedicated `release/0.1.x` maintenance line without merging it into the later feature-development line.

## [0.1.4] - 2026-09-03

Patch release focused on safe recursive preview behavior when symbolic links are followed.

### Fixed
- Preview mode now resolves a followed symbolic link before planning an operation and skips files whose resolved target is outside the selected root.
- External symlink targets are reported as recoverable preview errors instead of producing operations that can only be rejected later during execution.
- The existing execution-time canonical path boundary remains in place as defense-in-depth against forged or changed previews.

### Security
- Recursive preview with `follow_links` enabled no longer plans an operation for a file whose resolved target is outside the selected root.

### Tests
- Added Unix regression coverage for a file symlink inside the selected root that points to a matching file outside the root.
- Existing preview/execute/undo, forged-preview, journal, collision, and rule-validation tests remain in place.

### Release Engineering
- Synchronized the Rust workspace, desktop package, and Tauri application versions at `0.1.4`.
- Prepared the dedicated `release/0.1.4` maintenance branch and stable release documentation.

## [0.1.3] - 2026-09-03

Patch release focused on deterministic preview planning and collision safety.

### Fixed
- Preview planning now reserves destinations already assigned to earlier operations in the same preview.
- Multiple files with the same filename in different source directories no longer receive the same planned destination when recursive organization targets one folder.
- Collision-safe suffixes are now reflected directly in the preview instead of being discovered only while executing the plan.

### Tests
- Added recursive preview regression coverage for duplicate source filenames converging on the same destination folder.
- Existing journal, undo, traversal, Unicode-rule, and filename-safety regression coverage remains in place.

### Release Engineering
- Synchronized the Rust workspace, desktop package, and Tauri application versions at `0.1.3`.
- Continued the dedicated `release/0.1.x` maintenance line without merging it into the later feature-development line.

## [0.1.2] - 2026-09-03

Patch release focused on journal durability and defense-in-depth for undo operations.

### Fixed
- Journal snapshots can now replace an existing journal file on platforms where `rename` does not overwrite an existing destination, including Windows.
- Core undo now validates every recorded journal path against the journal's recorded root before performing any mutation.

### Security
- A forged operation journal that references paths outside its recorded root is rejected before undo begins.
- Existing symlink-aware path containment checks remain in place for desktop execution and undo boundaries.

### Tests
- Added regression coverage for replacing an existing journal snapshot.
- Added regression coverage proving that an out-of-root undo journal cannot mutate an external file.

### Release Engineering
- Synchronized the Rust workspace, desktop package, and Tauri application versions at `0.1.2`.
- Continued the dedicated `release/0.1.x` maintenance line without merging it into the later feature-development line.

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
