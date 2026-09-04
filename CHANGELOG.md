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

Patch release focused on deterministic collision handling during preview planning.

### Fixed
- Preview now reserves destinations selected for earlier operations, preventing duplicate source filenames from converging on the same destination when multiple files are organized in one run.

### Tests
- Added regression coverage for duplicate filenames discovered from different source directories during recursive preview.

### Security
- Retained the journal-root containment checks introduced in v0.1.2.

## [0.1.2] - 2026-09-03

Patch release focused on journal durability and undo-path integrity.

### Fixed
- Journal replacement works on platforms where renaming onto an existing target does not overwrite it directly.
- Undo rejects forged journal entries whose paths escape the journal's recorded root.

### Tests
- Added regression coverage for journal replacement and forged out-of-root journal entries.

## [0.1.1] - 2026-09-03

Patch release focused on Unicode-aware rule validation.

### Fixed
- Rule value and filename-regex length validation counts Unicode characters instead of UTF-8 bytes.

### Tests
- Added regression coverage for Unicode rule limits.
