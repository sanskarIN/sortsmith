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

## [0.1.0] - 2026-08-21

Initial development baseline. The release tag must only be published after cross-platform CI and installer verification are green.
