# SortSmith — Work Handoff

## Current active workstream: v0.1.7 stable release

- Release branch: `release/0.1.7`
- Base: `release/0.1.6`
- Target version: `0.1.7`
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.7 implementation and bug-fix audit

### Cached preview traversal safety

The performance-oriented cached organization preview in `crates/sortsmith-core/src/scan_cache.rs` now applies the same selected-root boundary used by the primary preview implementation. When `follow_links` is enabled, symbolic-link entries are resolved before cached metadata is reused or collected, and entries whose targets resolve outside the selected root are pruned before traversal can inspect an external tree.

This closes a maintenance-line consistency gap: v0.1.5/v0.1.6 hardened the primary organization and duplicate-scanning paths, but the cached preview introduced later had its own WalkDir traversal and therefore needed the same external-symlink guard.

### Cached preview collision safety

Cached preview planning now maintains a `HashSet` of reserved destinations for the current preview and uses `collision_safe_path_with_reserved`. This keeps cached previews deterministic when multiple source directories contain the same filename and all rules target one destination directory.

Collision resolution is still recomputed against the live filesystem on every preview, including cache hits, so a destination created after a previous preview remains visible to the next planning pass.

### Regression coverage

`scan_cache.rs` now covers an external symlink directory on Unix with `follow_links` enabled. The regression verifies that no operation is planned and the nested external file is not scanned.

The cache suite also now covers duplicate source filenames converging on one destination, while retaining existing tests for unchanged-file reuse, collision recomputation, rule-scope changes, changed-file rescans, deleted-file pruning, explicit cache clearing, and time-sensitive rule revalidation.

## v0.1.6 implementation and bug-fix audit

### Public API-level symlink traversal coverage

The v0.1.5 implementation prevents recursive traversal into symbolic-link directories whose resolved targets are outside the selected root when `follow_links` is enabled. v0.1.6 adds a public integration test at `crates/sortsmith-core/tests/external_symlink_traversal.rs` that exercises `preview_organization` and verifies an external linked directory produces no planned operation and no scanned nested external file.

### Windows filename portability hardening

`crates/sortsmith-core/src/safety.rs` rejects Unicode superscript aliases for numbered Windows device names, including `COM¹`, `COM²`, `COM³`, `LPT¹`, `LPT²`, and `LPT³`.

The collision planner treats reserved destination paths case-insensitively on Windows. This matters because Windows path comparison is case-insensitive while `HashSet<PathBuf>` equality is not. Without this normalization, two preview operations could reserve differently cased spellings of the same Windows destination and converge on one physical path.

The collision planner also bounds generated suffix candidates to the portable filename limits. When a source stem is already near the 255-byte or 255-UTF-16-unit boundary, a naive `name (1)` suffix would exceed the limit. v0.1.6 fits the stem to the remaining byte/unit budget before creating the candidate and trims an unsafe trailing space or period from the fitted stem.

### Journal durability and path normalization

`crates/sortsmith-core/src/journal.rs` synchronizes the journal directory after the temporary journal has been atomically replaced. The journal payload was already flushed and synced before replacement; syncing the containing directory adds the missing filesystem metadata durability step on Unix-like platforms.

Journal snapshots normalize relative root and entry paths to absolute paths before serialization. This prevents an undo journal created through the core API with relative paths from becoming impossible to validate later because the undo preflight compares against a canonical absolute root. Regression coverage verifies that relative journal paths are normalized.

### Crash recovery journal checkpoints

`crates/sortsmith-core/src/engine.rs` saves the journal after every successfully completed move instead of waiting until the entire batch finishes. A crash during a multi-file operation therefore leaves the journal containing all moves that were completed before the interruption.

The execution report also records absolute source and destination paths, keeping the in-memory report consistent with the durable journal format.

### No-overwrite move safety

File moves no longer rely on `fs::rename` as the final collision boundary. On supported filesystems SortSmith first creates a hard link at the destination and then removes the source; when hard linking is unavailable or crosses filesystems it falls back to `create_new` plus streamed copy and source removal. Both approaches refuse an already-created destination instead of overwriting it.

If a destination appears after preview but before execution, the engine now selects another collision-safe destination and retries up to eight times, reporting an explicit error if those retries are exhausted. Undo uses the same no-overwrite primitive, so a newly occupied original path cannot be silently replaced.

Regression coverage creates a destination after preview and verifies the newly-created file is preserved while the source is moved to a collision-safe suffix.

### Duplicate-scan root containment

`crates/sortsmith-core/src/duplicates.rs` now applies the same external-symlink containment checks used by organization preview when duplicate scanning is configured to follow links. This prevents a linked directory outside the selected root from being traversed and hashed. A Unix regression test covers the external-directory case.

### Desktop background-watch and persistence reliability

`apps/desktop/src/App.tsx` now guards the one-minute watched-folder timer against overlapping background invocations. A slow scan cannot be started again by the next timer tick while the previous background run is still active.

`apps/desktop/src/AutomationPage.tsx` now re-synchronizes its selected preset after saved state finishes loading or a preset is removed. It also mirrors the backend's 100-watched-folder limit.

`apps/desktop/src/App.tsx` now returns a success/failure result from state persistence instead of swallowing the result. Rule and preset editors, history, and settings backup import use that result so a failed write is not presented as a successful change.

### Release-branch CI coverage

`.github/workflows/ci.yml` runs on `release/**` pushes as well as `main`. This makes the maintenance release branch itself subject to the core format/clippy/tests, desktop Rust checks, and frontend typecheck/test/build gates.

### Maintenance diff cleanup

The earlier `rules.rs` formatting-only refactor was reverted to the release/0.1.5 formatting baseline. This removes unrelated churn from the release diff and leaves the v0.1.6 branch focused on actual safety, durability, traversal, and release-engineering changes.

## Release metadata

Version `0.1.7` is synchronized across:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

The v0.1.7 branch also contains:

- `CHANGELOG.md`
- `RELEASE_NOTES_v0.1.7.md`
- `docs/release-v0.1.7-checklist.md`

## Verification status

The repository has been reviewed and the cached-preview safety/collision defects were fixed directly on `release/0.1.7`. Local Rust, Node.js, Tauri, installer, and cross-platform builds have not been claimed as passed because this environment cannot provide a truthful local project checkout/toolchain execution.

The CI workflow is configured to run for `release/**` pushes. The available GitHub connector does not expose a complete check-run listing for arbitrary push workflow executions, so no CI result is fabricated here.

Before publication, run the full validation suite from the release branch:

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

The Unix symlink regressions should execute on Unix-like CI, and the complete workspace plus desktop suite should be exercised on Linux, Windows, and macOS before publishing production artifacts.

## Stable release procedure

After validation passes:

```bash
git checkout release/0.1.7
git pull --ff-only origin release/0.1.7
node scripts/verify-release-version.mjs v0.1.7
git diff --check
git status --short
git tag -a v0.1.7 -m "SortSmith v0.1.7"
git push origin v0.1.7
```

The tag should be created from `release/0.1.7`, not from `main`, because `main` is on the later 0.3.0 feature line.

Then review the tag-triggered release workflow, generated Linux/Windows/macOS artifacts, and draft GitHub release before publishing.

Recommended release metadata:

- Tag: `v0.1.7`
- Target: `release/0.1.7`
- Title: `SortSmith v0.1.7 — Patch Release`
- Pre-release: disabled
- Latest: disabled
- Body: `RELEASE_NOTES_v0.1.7.md`
