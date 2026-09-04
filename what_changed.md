# SortSmith — Work Handoff

## Current repository state

- Default branch: `main`
- Main HEAD: `56853cb16f6917cbbddeebdc2086f2a9f5143e56`
- Main version line: `0.3.0`
- License: Apache-2.0
- Repository: `https://github.com/sanskarIN/sortsmith`

## Main-branch integration completed

The historical `release/0.1.6` maintenance line has now been integrated into `main` without downgrading the main version metadata from `0.3.0`.

The integration is represented by a real two-parent merge commit:

- Main parent: `e56ebd627b52aefa4116730de5459e60bf0b4000`
- Maintenance parent: `ea83959b33b198e73faca5f77ef99ba32bc948e0`
- Merge commit: `56853cb16f6917cbbddeebdc2086f2a9f5143e56`

The `main` tree retains the modern 0.3.0 repository structure while bringing the v0.1.6 maintenance source, regression tests, release notes, and release checklists into the same branch history.

## Code and test changes now present on main

### Filesystem safety

- Recursive preview protects the selected-root boundary when symbolic links are followed.
- External symbolic-link directories are pruned before recursive traversal.
- External file-link targets remain rejected by containment checks.
- Duplicate scanning applies the same external-link containment protection.
- Windows reserved device-name handling includes Unicode superscript aliases for numbered `COM` and `LPT` names.
- Windows collision reservation uses case-insensitive path identity.
- Generated collision names are fitted to portable filename byte and UTF-16 limits.

### Execution and collision safety

- File moves use a no-overwrite destination boundary rather than relying on overwriting `rename` behavior.
- Hard-link creation with source removal is preferred where supported.
- Cross-filesystem moves use `create_new` plus streamed copy and source removal.
- Execution retries collision-safe destinations when a destination becomes occupied after preview.
- Undo uses the same no-overwrite mutation primitive.
- Relative execution paths are normalized to absolute journal paths.

### Journal durability and recovery

- Journal snapshots are flushed and synchronized before replacement.
- Unix journal-directory synchronization is performed after atomic replacement.
- Journal root and entries are normalized to absolute paths before persistence.
- Journals are checkpointed after each successfully completed move, improving crash recovery for multi-file operations.

### Desktop reliability

- Watched-folder background execution prevents overlapping timer-triggered scans.
- Automation preset selection is synchronized after asynchronous state loading and preset deletion.
- The UI mirrors the backend watched-folder limit.
- Rule, preset, history, and settings persistence flows now receive an explicit success/failure result instead of assuming that a write succeeded.

### Regression coverage

`main` now includes the v0.1.6 external-symlink integration regression and the maintenance-line safety coverage for collisions, journals, traversal, filenames, and undo behavior.

## Release/support files integrated

The main branch now also contains the maintenance release notes and publication checklists for v0.1.1 through v0.1.6, alongside the existing v0.3.0 release documentation.

## Version integrity

The merge intentionally preserved the modern main version metadata at `0.3.0`:

- root workspace `Cargo.toml`;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

The historical 0.1.x release-note documents continue to describe their original maintenance releases and are not used as current main version metadata.

## CI

The CI workflow on `main` now also recognizes `release/**` pushes while continuing to run for `main` and pull requests.

## Validation status

The GitHub branch/tree integration has been completed. Local Rust/Node/Tauri builds and tests have not been claimed as passed because this environment does not have a trustworthy local checkout and full toolchain execution path.

The next validation step is to run the complete Rust and desktop quality suite on a real checkout of `main`, inspect any compile/test failures caused by integrating the historical maintenance code with the 0.3.0 line, and fix those directly on `main` before the next release.

## Important branch history note

`release/0.1.6` remains available as a historical maintenance branch. `main` is now the branch containing both the modern main-line history and the v0.1.6 maintenance integration. The merge commit preserves both parents so neither line is silently discarded.
