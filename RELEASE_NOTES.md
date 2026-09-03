# SortSmith v0.3.0 — First Public Preview

**Release type:** Pre-release / Public Preview  
**Version:** `0.3.0`  
**License:** Apache-2.0

## Overview

SortSmith v0.3.0 is the first public preview release of SortSmith, an open-source Rust/Tauri file-organization application focused on rule-based organization, safe previews, reversible operations, duplicate detection, and privacy-conscious local workflows.

This release establishes the first version that should be presented publicly from the current `main` branch. Earlier `0.1.0` and `0.2.0` entries represent development history and release-candidate work rather than published GitHub releases.

## Highlights

### Rule-Based Organization

- Match files by extension, MIME prefix, age, size, and filename regular expressions.
- Build multi-criterion rules and reusable rule presets.
- Use bundled preset packs for common organization workflows.

### Safe Planning and File Operations

- Dry-run organization previews before mutation.
- Collision-safe moves and renames.
- Reversible operation journals.
- Latest-undo and selectable operation-history undo.
- Root containment and traversal protections around planned operations.

### Duplicate Detection

- BLAKE3-based duplicate detection.
- No automatic duplicate deletion.
- Hidden-directory handling that avoids scanning hidden directories unless explicitly enabled.

### Watched-Folder Automation

- User-controlled watched folders.
- Configurable presets and execution intervals while the desktop application is open.
- Preview-cache invalidation before filesystem mutation so stale interactive planning data is not reused after changes.

### Incremental Interactive Preview Cache

- Process-local, in-memory caching for repeated interactive previews.
- Exact cache scoping across selected root, ordered rules, and scan options.
- File-level validation using path, size, and modification timestamp.
- Cache pruning for deleted or moved files.
- Mandatory re-evaluation of time-sensitive modified-age rules.
- Safe fallback to the uncached planner when the cache mutex is unavailable.
- Warm-cache Criterion benchmark coverage.

The cache does not store final destination choices. Destination construction and collision resolution are recomputed for every preview so newly occupied destinations remain safe.

### Desktop Application

- Native folder selection.
- Native settings backup/import dialogs.
- Keyboard-first navigation and quick actions.
- Shortcut reference dialog with focus management.
- Responsive light/dark/system UI.
- About, support, funding, license, and project information.

### Persistence and Diagnostics

- Schema-versioned local settings.
- Bounded JSON persistence/import behavior.
- Privacy-safe rotating operation logs.
- Protection against symlink-backed settings/log targets.

### Testing and Quality

- Unit and regression coverage across the Rust core and desktop host.
- Property-based tests for important serialization and path-safety behavior.
- Criterion benchmarks for organization planning, duplicate hashing, and warm-cache previews.
- CI, CodeQL, Dependabot, release-version checks, and release-lockfile verification.

## Security and Safety

SortSmith is designed to reduce accidental filesystem damage, but this preview release should not be treated as a guarantee of safety for critical data.

Before using automated organization against important files, maintain appropriate backups and inspect dry-run previews carefully.

Important protections include:

- Root canonicalization and containment checks.
- Rejection of parent traversal and destination symlink escapes.
- No link-following during normal scans.
- Collision-safe destination resolution.
- Validation of portable filenames and Windows-reserved device names.
- Prevention of deleting bundled presets.
- Prevention of deleting custom presets still referenced by watched folders.

## Known Limitations

As a `0.x` public preview, SortSmith remains under active development.

Known areas still being expanded include:

- Broader cross-platform installer validation.
- More extensive end-to-end desktop testing.
- Additional performance measurements on representative real-world workloads.
- More advanced watcher/invalidation integration.
- Further UI polish and accessibility coverage.
- More comprehensive user documentation.

Internal APIs and behavior may change in future `0.x` releases.

## Upgrade and Compatibility Notes

This is the first published GitHub release, so there is no earlier published release to upgrade from.

The project uses Semantic Versioning conventions during `0.x` development, but breaking changes may still occur before `1.0.0`.

## Verification Before Publishing

The release should only be tagged after the repository's release checks are green, including formatting, compilation, tests, Clippy, frontend type checking/tests/build, desktop-host checks, CodeQL, and release-version/lockfile validation.

Recommended local Rust checks:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Recommended frontend checks:

```bash
cd apps/desktop
npm ci
npm run typecheck
npm test -- --run
npm run build
```

## What's Next

Development after `v0.3.0` will focus on reliability, scale, automation, and a stronger cross-platform desktop experience.

Planned areas include:

- More advanced watcher integration.
- Broader cache invalidation strategies.
- Additional performance and benchmark evidence.
- Stronger end-to-end test coverage.
- Improved diagnostics and recovery workflows.
- Cross-platform packaging and installer validation.
- Continued accessibility and UI improvements.

## Feedback

Bug reports, feature requests, documentation improvements, and code contributions are welcome through the SortSmith GitHub repository.

When reporting a problem, include the SortSmith version, operating system, reproduction steps, expected behavior, actual behavior, and relevant non-sensitive logs.

## Release Status

**Pre-release / Public Preview**

`v0.3.0` is the first public preview release from the current project history. It is not a `1.0.0` stability promise.

---

**SortSmith v0.3.0**  
*Organize files. Define rules. Build with confidence.*
