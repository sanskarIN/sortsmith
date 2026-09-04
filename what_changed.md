# SortSmith — Work Handoff

## Current repository state

- Default branch: `main`
- Main version line: `0.3.0`
- Current maintenance release workstream: `v0.1.7`
- Maintenance branch: `release/0.1.7`
- Maintenance base: `release/0.1.6`
- License: Apache-2.0
- Repository: `https://github.com/sanskarIN/sortsmith`
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.7 implementation completed

### Deterministic duplicate results

`crates/sortsmith-core/src/duplicates.rs` now sorts the paths inside each duplicate group before returning the result. Duplicate groups already have stable size/hash ordering; this change also makes their member ordering stable.

This prevents filesystem traversal order and Rayon hashing completion order from leaking into the returned API result. Repeated scans over the same unchanged directory therefore present duplicate members in the same path order.

### Regression coverage

Added `duplicate_files_are_sorted_by_path`, which creates two equal-content files in reverse lexical creation order and verifies that the returned duplicate group is ordered lexically by path.

Existing coverage remains for equal-content detection without deletion, hidden-directory pruning, and external symbolic-link directories when link following is enabled.

### Maintenance-line scope

The in-memory `scan_cache.rs` work belongs to the later 0.3.0 feature-development line and was deliberately removed from `release/0.1.7`. This keeps the 0.1.x patch release focused on a small compatibility-safe maintenance fix instead of backporting an entire later feature.

The corresponding cached-preview hardening remains on `main`, where the 0.3.0 scan-cache feature lives.

## v0.1.7 release engineering

The dedicated `release/0.1.7` branch was created from `release/0.1.6` and contains the implementation fix, version synchronization, release documentation, checklist, and handoff updates.

Version `0.1.7` is synchronized across:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

Release support files are present:

- `CHANGELOG.md`
- `RELEASE_NOTES_v0.1.7.md`
- `docs/release-v0.1.7-checklist.md`
- `what_changed.md`

## v0.1.6 implementation and bug-fix audit

### Public API-level symlink traversal coverage

The v0.1.5 implementation prevents recursive traversal into symbolic-link directories whose resolved targets are outside the selected root when `follow_links` is enabled. v0.1.6 added a public integration test at `crates/sortsmith-core/tests/external_symlink_traversal.rs` covering this behavior through `preview_organization`.

### Windows filename portability hardening

`crates/sortsmith-core/src/safety.rs` rejects Unicode superscript aliases for numbered Windows device names such as `COM¹`, `COM²`, `COM³`, `LPT¹`, `LPT²`, and `LPT³`.

Reserved destination comparison is case-insensitive on Windows. Generated collision names are bounded to the portable 255-byte and 255-UTF-16-unit filename limits.

### Journal durability and path normalization

Journal snapshots normalize relative root and entry paths to absolute paths and synchronize the journal directory after atomic replacement on Unix-like systems.

### Crash recovery journal checkpoints

The execution engine saves the journal after each successfully completed move, reducing the amount of completed work that can be lost if a multi-file operation is interrupted.

### No-overwrite move safety

File moves and undo moves use a no-overwrite hard-link path with a `create_new` streamed-copy fallback instead of relying on an overwriting `rename` boundary. Execution retries collision-safe destinations when a destination becomes occupied after preview.

### Duplicate-scan root containment

Duplicate scanning prunes symbolic-link entries whose resolved targets are outside the selected root when link following is enabled.

### Desktop reliability

Watched-folder background execution prevents overlapping timer-triggered scans. Automation preset selection is resynchronized after asynchronous state loading/deletion, and persistence success/failure is propagated to rule, preset, history, and settings-backup UI flows.

### Release-branch CI coverage

`.github/workflows/ci.yml` runs on `release/**` pushes as well as `main` and pull requests.

## Main-branch integration status

The historical `release/0.1.6` maintenance line was integrated into `main` with a real two-parent merge while preserving main's `0.3.0` version metadata.

The later v0.1.7 duplicate-result determinism fix is now also present directly on `main` as a compatible source change. The cached-preview safety/collision hardening remains on `main` as part of the 0.3.0 scan-cache feature.

`main` therefore contains the maintenance-line safety hardening plus the newer 0.3.0 feature-development line. The dedicated `release/0.1.7` branch remains the correct source for the v0.1.7 tag because its version metadata is 0.1.7.

## Version integrity

- `main` remains `0.3.0`.
- `release/0.1.7` is synchronized at `0.1.7`.
- Do not tag `main` as `v0.1.7`.
- The `v0.1.7` tag must be created from `release/0.1.7` after validation.

## Verification status

GitHub-side source and documentation changes have been completed. Local Rust, Node.js, Tauri, installer, and cross-platform builds have **not** been claimed as passed because this environment does not provide a trustworthy local project checkout and complete toolchain execution path.

The available GitHub connector does not expose a complete check-run listing for arbitrary push workflow executions, so no CI result is fabricated here.

Before publication, run the complete release validation from `release/0.1.7`:

```bash
git checkout release/0.1.7
git pull --ff-only origin release/0.1.7
node scripts/verify-release-version.mjs v0.1.7
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
git diff --check
git status --short

cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

The deterministic duplicate-ordering regression must pass as part of the core test suite. Full application packaging should be validated on Linux, Windows, and macOS.

## v0.1.7 publication procedure

After all validation gates pass:

```bash
git checkout release/0.1.7
git pull --ff-only origin release/0.1.7
node scripts/verify-release-version.mjs v0.1.7
git diff --check
git status --short
git tag -a v0.1.7 -m "SortSmith v0.1.7"
git push origin v0.1.7
```

The tag-triggered release workflow is configured to create a draft release. Review the generated Linux/Windows/macOS artifacts and draft release before publishing.

Recommended release metadata:

- Tag: `v0.1.7`
- Target: `release/0.1.7`
- Title: `SortSmith v0.1.7 — Patch Release`
- Pre-release: disabled
- Latest: disabled
- Body: `RELEASE_NOTES_v0.1.7.md`

## Release status

As of this handoff, `v0.1.7` has **not** been published. The release branch and release materials are prepared, but the tag and GitHub release must be created only after the validation gates pass.
