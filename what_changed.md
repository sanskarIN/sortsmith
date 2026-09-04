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

### Cached preview traversal safety

The performance-oriented cached organization preview in `crates/sortsmith-core/src/scan_cache.rs` now applies the same selected-root boundary used by the primary preview implementation. When `follow_links` is enabled, symbolic-link entries are resolved before cached metadata is reused or collected, and entries whose targets resolve outside the selected root are pruned before traversal can inspect an external tree.

This closes a consistency gap: the primary organization and duplicate-scanning paths had external-symlink protection, while the later in-memory cached planner had its own WalkDir traversal and therefore needed the same security boundary.

### Cached preview collision safety

Cached preview planning now maintains a `HashSet` of reserved destinations for the current preview and uses `collision_safe_path_with_reserved`. Multiple source directories containing the same filename can therefore target one destination folder without receiving the same planned destination.

Collision resolution continues to be recomputed against the live filesystem on every preview, including cache hits, so destinations created after an earlier preview are still detected.

### Regression coverage

Added cache regressions for:

- external symbolic-link directory traversal with `follow_links` enabled on Unix;
- duplicate source filenames converging on one destination folder;
- existing collision recomputation after a cache hit.

Existing cache coverage for unchanged-file reuse, changed-file rescans, deletion pruning, rule-scope resets, explicit clearing, and time-sensitive rule revalidation remains in place.

## v0.1.7 release engineering

The dedicated `release/0.1.7` branch was created from `release/0.1.6` and now contains:

- cached-preview safety/collision implementation;
- synchronized `0.1.7` versions in `Cargo.toml`, `apps/desktop/package.json`, and `apps/desktop/src-tauri/tauri.conf.json`;
- `CHANGELOG.md` entry;
- `RELEASE_NOTES_v0.1.7.md`;
- `docs/release-v0.1.7-checklist.md`.

The same v0.1.7 source hardening and release documentation have also been mirrored onto `main`, while `main` remains correctly on its later `0.3.0` version line.

## v0.1.6 implementation and bug-fix audit

### Public API-level symlink traversal coverage

The v0.1.5 implementation prevents recursive traversal into symbolic-link directories whose resolved targets are outside the selected root when `follow_links` is enabled. v0.1.6 adds a public integration test at `crates/sortsmith-core/tests/external_symlink_traversal.rs` that exercises `preview_organization` and verifies an external linked directory produces no planned operation and no scanned nested external file.

### Windows filename portability hardening

`crates/sortsmith-core/src/safety.rs` rejects Unicode superscript aliases for numbered Windows device names, including `COM¹`, `COM²`, `COM³`, `LPT¹`, `LPT²`, and `LPT³`.

The collision planner treats reserved destination paths case-insensitively on Windows and bounds generated suffix candidates to portable filename limits.

### Journal durability and path normalization

`crates/sortsmith-core/src/journal.rs` synchronizes the journal directory after atomic replacement on Unix-like systems. Journal snapshots normalize relative root and entry paths to absolute paths before serialization.

### Crash recovery journal checkpoints

`crates/sortsmith-core/src/engine.rs` saves the journal after every successfully completed move so completed operations remain recoverable if a multi-file batch is interrupted.

### No-overwrite move safety

File moves and undo moves use a no-overwrite hard-link path with a `create_new` streamed-copy fallback rather than relying on overwriting `rename` behavior. Execution retries collision-safe destinations when a destination becomes occupied after preview.

### Duplicate-scan root containment

`crates/sortsmith-core/src/duplicates.rs` applies the selected-root external-symlink boundary when duplicate scanning follows links.

### Desktop reliability

The watched-folder timer prevents overlapping background invocations. Automation preset state is resynchronized after asynchronous loads/deletions, and persistence success/failure is propagated to rule, preset, history, and settings-backup UI flows.

### Release-branch CI coverage

`.github/workflows/ci.yml` runs on `release/**` pushes as well as `main` and pull requests.

## Main-branch integration status

The historical `release/0.1.6` maintenance line was integrated into `main` with a real two-parent merge while preserving main's `0.3.0` version metadata. The later v0.1.7 cached-preview hardening is now also present directly on `main`.

`main` therefore contains the maintenance-line filesystem hardening plus the newer 0.3.0 feature-development line. The dedicated `release/0.1.7` branch remains the correct source for the v0.1.7 tag because its version metadata is 0.1.7.

## Version integrity

- `main` remains `0.3.0`.
- `release/0.1.7` is synchronized at `0.1.7`.
- Do not tag `main` as `v0.1.7`.
- The `v0.1.7` tag must be created from `release/0.1.7` after validation.

## Validation status

The GitHub-side source and documentation changes have been made. Local Rust, Node.js, Tauri, installer, and cross-platform builds have **not** been claimed as passed because this environment does not provide a trustworthy local project checkout and full toolchain execution path.

The available GitHub connector does not expose a complete check-run listing for arbitrary push workflow executions, so no CI result is fabricated here.

Before publication, run:

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

The Unix cached-preview symlink regression should run on Unix-like CI. Full application packaging should be validated on Linux, Windows, and macOS.

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

The tag-triggered release workflow is configured to create a draft release. Review the generated artifacts and release body before publishing.

Recommended metadata:

- Tag: `v0.1.7`
- Target: `release/0.1.7`
- Title: `SortSmith v0.1.7 — Patch Release`
- Pre-release: disabled
- Latest: disabled
- Body: `RELEASE_NOTES_v0.1.7.md`

## Release status

As of this handoff, `v0.1.7` has **not** been published. The release branch and release materials are prepared, but the tag and GitHub release should only be created after the validation gates pass.
