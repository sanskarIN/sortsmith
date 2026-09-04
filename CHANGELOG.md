# Changelog

All notable changes follow Keep a Changelog principles and Semantic Versioning.

## [Unreleased]

Development continues on the next feature line.

## [0.1.7] - 2026-09-04

Stable maintenance release focused on deterministic duplicate-detection results without changing duplicate matching or deletion behavior.

### Fixed
- Duplicate groups now sort their member file paths before being returned, so filesystem traversal and parallel hashing order cannot leak into result ordering.

### Tests
- Added regression coverage that verifies equal-content duplicate members are returned in lexical path order.
- Existing content-equality, hidden-directory, and external-symlink traversal coverage remains in place.

### Release Engineering
- Synchronized the Rust workspace, desktop package, and Tauri application versions at `0.1.7`.
- Prepared the dedicated `release/0.1.7` maintenance branch from `release/0.1.6`.
- Added stable v0.1.7 release notes and publication checklist.

## [0.1.6] - 2026-09-04

Stable maintenance release focused on recursive symbolic-link traversal safety, portable Windows filename validation, collision planning robustness, reliable undo recovery, and desktop persistence behavior.

### Added
- Added a Unix integration test through the public `sortsmith-core` API covering a recursive scan with `follow_links` enabled and an external symbolic-link directory.
- Added duplicate-scanner regression coverage proving an external symlink directory is not traversed or hashed.
- Added regression coverage for portable collision-name limits, relative-root journal execution, and destinations that appear after preview.

### Fixed
- Filename validation rejects Unicode superscript aliases for numbered Windows `COM` and `LPT` device names.
- Reserved destination comparison is case-insensitive on Windows.
- Collision suffix generation fits the source stem to portable filename limits.
- Journal snapshots normalize relative paths to absolute paths and synchronize the containing directory on Unix-like systems.
- The undo journal is checkpointed after each successfully completed move.
- File moves and undo moves use a no-overwrite hard-link path with a `create_new` streamed-copy fallback and bounded collision retries.
- Duplicate scans prune symbolic-link entries whose targets resolve outside the selected root.
- Watched-folder execution prevents overlapping background scans.
- Automation state synchronizes its selected preset and enforces the backend watched-folder limit.
- Rule, preset, history, and settings-backup persistence flows now receive explicit success/failure results.

### Security
- Recursive organization and duplicate scanning keep followed symbolic links inside the selected root.
- Execution and undo refuse to overwrite destinations that appear after preview or while undo is prepared.

## [0.1.5] - 2026-09-04

Stable maintenance patch focused on preventing recursive traversal from escaping the selected folder through symbolic-link directories.

### Fixed
- Preview traversal prunes symbolic-link entries whose resolved targets are outside the selected root before recursive descent.
- Existing file-level external-symlink rejection remains defense-in-depth.

## [0.1.4] - 2026-09-03

Patch release focused on safe recursive preview behavior when symbolic links are followed.

### Fixed
- Preview mode resolves followed symbolic links and skips targets outside the selected root.
- External symlink targets are reported as recoverable preview errors.

## [0.1.3] - 2026-09-03

Patch release focused on deterministic preview planning and collision safety.

### Fixed
- Preview planning reserves destinations already assigned to earlier operations in the same preview.
- Multiple files with the same filename in different source directories receive distinct planned destinations.

## [0.1.2] - 2026-09-03

Patch release focused on journal durability and defense-in-depth for undo operations.

### Fixed
- Journal snapshots can replace existing journal files on platforms where `rename` does not overwrite the target.
- Core undo validates recorded journal paths against the journal root before mutation.

## [0.1.1] - 2026-09-03

Patch release focused on Unicode-safe rule validation and release metadata consistency.

### Fixed
- Rule value and filename-regex limits count Unicode characters rather than UTF-8 bytes.

## [0.1.0] - 2026-08-21

Initial development baseline. The release tag must only be published after cross-platform CI and installer verification are green.
