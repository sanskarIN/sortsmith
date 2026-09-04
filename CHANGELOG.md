# Changelog

All notable changes follow Keep a Changelog principles and Semantic Versioning.

## [Unreleased]

Development line: `0.3.0` on `develop/0.3.0`. This work is intentionally isolated from the `0.2.0` release candidate on `main` until the 0.2 release gate is resolved.

### Added
- Incremental in-memory preview scan cache for repeated interactive organization previews.
- Exact cache scoping across selected root, ordered rules, and scan options.
- File-level cache invalidation using path, file size, and modification timestamp.
- Cache pruning for deleted/moved files and explicit clearing before filesystem mutations.
- Mandatory re-evaluation of time-sensitive `ModifiedOlderThanDays` rules on reused file descriptions.
- Fail-safe desktop fallback to the uncached planner if the process-local cache mutex is unavailable.
- Warm-cache Criterion benchmark group paired with the existing uncached organization-planning benchmark.
- Core regression tests for unchanged-file reuse, rule-scope invalidation, changed-file rescanning, deletion pruning, explicit clearing, and time-sensitive revalidation.

### Changed
- Interactive desktop previews now use the incremental cache while preserving the existing `PreviewResult` contract.
- Destination construction and collision-safe destination resolution still run on every preview, including cache hits.
- Apply, undo, and in-app watched-folder execution clear the interactive preview cache before filesystem mutation.
- Workspace, frontend package, and Tauri application development versions are aligned at `0.3.0` on the development branch.

### Verification pending
- Rust formatting, Clippy, unit tests, desktop-host checks, and CodeQL must be green for the 0.3 branch.
- Same-machine uncached versus warm-cache Criterion measurements must be recorded before claiming a performance improvement or setting a performance budget.
- The cache remains in-memory only; persistent cache format/invalidation is intentionally not introduced in this phase.

## [0.2.0] - Unreleased release candidate

The source candidate remains on `main`. This section is intentionally undated until the release gate is complete and the tag is actually published.

### Added
- Production-oriented Rust/Tauri/React SortSmith baseline.
- Rules for extension, MIME prefix, age, size, and filename regex, including a multi-criterion rule builder and reusable presets.
- Saved user-defined preset management: snapshot active rules, load a preset into the active rule set, edit custom preset metadata, and safely delete presets that are not assigned to watched folders.
- Four bundled preset packs with stable identities: Everyday tidy, Media library, Developer workspace, and Downloads cleanup.
- Backward-compatible frontend migration from the legacy random Everyday tidy preset identifier to its stable bundled identifier, including watched-folder preset-reference remapping.
- Dry-run organization, collision-safe moves/renames, reversible journals, latest undo, and selectable operation-history undo.
- Duplicate detection with BLAKE3 and no automatic deletion.
- Watched-folder automation with user-controlled presets and intervals while the desktop app is open.
- Native folder selection plus native settings backup/import dialogs.
- Keyboard-first quick actions for page navigation, folder selection, preview, apply, and undo, plus an in-app shortcut reference. Editing fields retain their normal keyboard behavior.
- Schema-versioned local settings, bounded JSON persistence, and privacy-safe rotating operation logs.
- Property-based core coverage for serialization, path traversal rejection, portable filenames, and rename-template extension preservation.
- Criterion benchmark targets for organization planning and duplicate hashing using isolated synthetic directory fixtures.
- Responsive light/dark/system UI with accessibility baseline and About/support/funding information.
- Full project documentation, ADRs, contribution/security/privacy policies, CI, Dependabot, CodeQL for TypeScript and Rust, issue/PR templates, and cross-platform release automation.
- Release metadata verification that can run before tagging and rejects inconsistent Cargo, frontend, or Tauri versions.
- Release lockfile verification that blocks packaging until `Cargo.lock` and `apps/desktop/package-lock.json` are committed and aligned with the application version.

### Changed
- Release candidate version metadata is aligned at `0.2.0` across the Rust workspace, frontend package, and Tauri application configuration.
- The About screen reads its version from frontend package metadata instead of maintaining a separate hard-coded version string.
- Undo journal persistence and loading stream JSON instead of allocating an additional whole-document byte buffer.
- Execution verifies that undo-journal storage can be initialized before the first file mutation.
- Frontend tooling declares the supported Node.js 22 and npm 10 runtime range.
- Rejected settings/rule state is no longer activated optimistically in the running UI when persistence fails.
- CI runs desktop-host unit tests in addition to Rust checks, Clippy, core tests, frontend tests, type checking, builds, and release-version synchronization checks.
- The Rules page separates active-rule editing from durable user preset management instead of treating the first preset as the only reusable rule source.
- Bundled preset protection now uses stable preset IDs instead of relying on list position.
- Release packaging uses `npm ci`, verifies both lockfiles, and validates the Rust lockfile with `cargo fetch --locked` before Tauri packaging.

### Accessibility
- The active sidebar destination exposes `aria-current="page"` to assistive technologies.
- Keyboard quick actions provide direct navigation and high-frequency organizer operations without stealing keys from form controls.
- The shortcut reference is exposed as a labelled modal dialog with keyboard dismissal.
- The shortcut dialog moves focus to its close control when opened and restores the previously focused control when closed.
- About exposes both business contacts, support, funding, license, version, and project credit.

### Security
- Root canonicalization and planned-operation containment checks exist in both the desktop boundary and core execution path.
- Forged execution previews that point sources or destinations outside the selected root are rejected before mutation.
- Parent traversal rejection and destination symlink escape protection.
- No link-following during normal scans and no file-content/path data in structured operation logs.
- Hidden directories are pruned during duplicate scans unless the user explicitly enables hidden-file scanning.
- Rendered rename destinations reject non-portable filenames, Windows-reserved device names, unsafe characters, and overlong names.
- Collision fallback never returns an already occupied destination after the normal numeric suffix range is exhausted.
- Saved settings reject symlink-backed state files, enforce a 16 MiB storage/import contract, and sync temporary files before replacement.
- Operation logs refuse symlink/non-file targets and rotate at 5 MiB to bound local growth.
- Custom preset deletion is blocked while a watched-folder configuration still references that preset.
- Bundled presets cannot be renamed or deleted, preserving a recoverable built-in rule library.

## [0.1.7] - 2026-09-04

Stable maintenance release focused on keeping cached preview planning aligned with the filesystem safety guarantees of the primary preview engine.

### Fixed
- Cached organization previews now prune symbolic-link files and directories whose resolved targets are outside the selected root when `follow_links` is enabled.
- Cached previews now apply selected-root containment checks before reusing metadata for followed links.
- Cached preview planning now reserves destinations already assigned earlier in the same preview, preventing duplicate source filenames from converging on one physical destination.

### Security
- The performance-oriented cached preview path now enforces the same external-symlink traversal boundary as the primary preview path.
- Preview-time collision reservation remains deterministic and collision-safe even when file descriptions are reused from the in-memory cache.

### Tests
- Added a Unix regression covering an external symbolic-link directory with a nested matching file while cached preview uses `follow_links`.
- Added cached-preview regression coverage for duplicate source filenames targeting the same destination folder.
- Retained cache-hit collision recomputation and cache invalidation coverage.

### Release Engineering
- Synchronized the Rust workspace, desktop package, and Tauri application versions at `0.1.7` on the dedicated maintenance branch.
- Prepared stable v0.1.7 release notes and publication checklist.

## [0.1.6] - 2026-09-04

Stable maintenance release focused on recursive symbolic-link traversal safety, portable Windows filename validation, collision planning robustness, reliable undo recovery, and desktop persistence behavior.

### Fixed
- Filename validation now rejects Unicode superscript aliases for numbered Windows `COM` and `LPT` device names.
- Reserved destination comparison is case-insensitive on Windows and collision suffixes are bounded to portable filename limits.
- Journal snapshots normalize relative paths to absolute paths and checkpoint after successful moves.
- File moves and undo moves use no-overwrite primitives with collision retries instead of an overwriting `rename` boundary.
- Duplicate scans prune external followed symlink targets.
- Watched-folder execution prevents overlapping background scans and persistence failures are propagated to affected UI flows.

## [0.1.5] - 2026-09-04

Stable maintenance patch focused on preventing recursive traversal from escaping the selected folder through symbolic-link directories.

### Fixed
- Preview traversal prunes symbolic-link entries whose resolved targets are outside the selected root before recursive descent.

## [0.1.4] - 2026-09-03

Patch release focused on safe recursive preview behavior when symbolic links are followed.

### Fixed
- Preview mode resolves followed symbolic links and skips targets outside the selected root.

## [0.1.3] - 2026-09-03

Patch release focused on deterministic preview planning and collision safety.

### Fixed
- Preview planning reserves destinations already assigned to earlier operations in the same preview.

## [0.1.2] - 2026-09-03

Patch release focused on journal durability and defense-in-depth for undo operations.

### Fixed
- Journal snapshots can replace existing journal files on platforms where `rename` does not overwrite the target.
- Core undo validates recorded paths against the journal root before mutation.

## [0.1.1] - 2026-09-03

Patch release focused on Unicode-safe rule validation and release metadata consistency.

### Fixed
- Rule value and filename-regex limits count Unicode characters rather than UTF-8 bytes.

## [0.1.0] - 2026-08-21

Initial development baseline. The release tag must only be published after cross-platform CI and installer verification are green.
