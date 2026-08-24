# Changelog

All notable changes follow Keep a Changelog principles and Semantic Versioning.

## [Unreleased]

### Added
- Production-oriented Rust/Tauri/React SortSmith baseline.
- Rules for extension, MIME prefix, age, size, and filename regex, including a multi-criterion rule builder and reusable presets.
- Saved user-defined preset management: snapshot active rules, load a preset into the active rule set, edit custom preset metadata, and safely delete presets that are not assigned to watched folders.
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
- Release metadata verification that rejects version tags inconsistent with Cargo, frontend, or Tauri versions.

### Changed
- Undo journal persistence and loading now stream JSON instead of allocating an additional whole-document byte buffer.
- Execution verifies that undo-journal storage can be initialized before the first file mutation.
- Frontend tooling declares the supported Node.js 22 and npm 10 runtime range.
- Rejected settings/rule state is no longer activated optimistically in the running UI when persistence fails.
- CI now runs desktop-host unit tests in addition to Rust checks, Clippy, core tests, frontend tests, type checking, and builds.
- The Rules page now separates active-rule editing from durable user preset management instead of treating the first built-in preset as the only reusable rule source.

### Accessibility
- The active sidebar destination exposes `aria-current="page"` to assistive technologies.
- Keyboard quick actions provide direct navigation and high-frequency organizer operations without stealing keys from form controls.
- The shortcut reference is exposed as a modal dialog with a labelled heading and keyboard dismissal.
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

## [0.1.0] - 2026-08-21

Initial development baseline. The release tag must only be published after cross-platform CI and installer verification are green.
