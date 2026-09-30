# SortSmith — Work Handoff

## Current active workstream: v0.1.9 maintenance release

- Release branch: `release/0.1.9`
- Base: `release/0.1.8`
- Target version: `0.1.9`
- Default branch: `main`
- Main version line: `0.3.x`
- Repository: `https://github.com/sanskarIN/sortsmith`
- License: Apache-2.0
- Commit identity: `Sanskar <sanskarin@outlook.in>`

## v0.1.9 scope

v0.1.9 is intentionally a focused 0.1.x maintenance release. It is based on `release/0.1.8` and does not backport newer 0.3.x feature-development work from `main`.

The release synchronizes the application version across:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

The release also adds a dedicated release note and changelog entry.

## Safety baseline retained

The v0.1.8 baseline remains intact, including:

- External symbolic-link containment during recursive scans.
- Collision-safe no-overwrite moves.
- Durable journal checkpoints.
- Safe undo path validation.
- Portable filename validation.
- Deterministic duplicate-detection output.

No intentional breaking public API change is introduced by the v0.1.9 maintenance work.

## v0.1.9 commit sequence

The release branch currently contains focused commits for:

1. `chore(release): bump workspace version to 0.1.9`
2. `chore(release): synchronize desktop package version to 0.1.9`
3. `chore(release): synchronize tauri version to 0.1.9`
4. `docs(release): add v0.1.9 release notes`
5. `docs(changelog): restore complete changelog with v0.1.9 entry`

All project commits use `Sanskar <sanskarin@outlook.in>`.

## Verification gate

The v0.1.9 tag must not be published until the dedicated branch passes:

```bash
node scripts/verify-release-version.mjs v0.1.9
File moves and undo moves use a no-overwrite hard-link path with a `create_new` streamed-copy fallback instead of relying on an overwriting `rename` boundary. Execution retries collision-safe destinations when a destination becomes occupied after preview.

### Duplicate-scan root containment

Duplicate scanning prunes symbolic-link entries whose resolved targets are outside the selected root when link following is enabled.

### Desktop reliability

Watched-folder background execution prevents overlapping timer-triggered scans. Automation preset selection is resynchronized after asynchronous state loading/deletion, and persistence success/failure is propagated to rule, preset, history, and settings-backup UI flows.

## Main-branch status

The v0.1.6 maintenance line was previously integrated into `main` without downgrading the main version metadata. The v0.1.7 duplicate-result determinism fix is intentionally kept on the dedicated maintenance branch and should also be backported to `main` only as a compatible source change if required by the 0.3.0 line.

The cached-preview hardening created during this work is already present directly on `main`, where it is relevant to the 0.3.0 cache implementation. It is not part of the v0.1.7 release branch.

`main` remains the source for the later feature-development line and must not be tagged `v0.1.7`.
## Current repository state

- Default branch: `main`
- Main version line: `0.3.0`
- Current development focus: stabilize the 0.3.0 main line and keep filesystem safety regressions covered by public integration tests.
- License: Apache-2.0
- Repository: `https://github.com/sanskarIN/sortsmith`
- Commit identity: `Sanskar <sanskarin@outlook.in>`

## Main branch work completed on 2026-09-05

### Public filesystem safety regression coverage

Added `crates/sortsmith-core/tests/main_branch_safety_regressions.rs`.

The integration coverage exercises the public `preview_organization` API and verifies that, when symbolic-link following is enabled:

- recursive scans do not traverse an external symbolic-link directory;
- external symbolic-link files are ignored instead of being planned for organization;
- no external file is turned into a move operation;
- the public preview reports the external-file symlink as ignored and records a privacy-safe recoverable warning.

This complements the existing unit coverage and keeps the security boundary tested from outside the core module implementation.

### Main CI/build stabilization

Fixed the 0.3.0 main-line Rust build blockers found by GitHub Actions:

- corrected symlink pruning predicate precedence in `engine.rs`;
- corrected the same predicate in `duplicates.rs`;
- fixed journal path normalization so fallible absolute-path conversion is propagated from the iterator closure;
- restored scan-cache metadata construction locally in `scan_cache.rs`, removing the stale dependency on a missing engine helper;
- updated frontend CI to npm `11.6.0` with `--legacy-peer-deps`, matching the dependency-install path that avoids the observed npm resolver failure.

The failed CI run had exposed real compile errors in `scan_cache.rs`, `duplicates.rs`, `engine.rs`, and `journal.rs`, plus an npm `edgesOut` resolver failure. These were treated as implementation issues rather than ignored CI noise.

### Release-line cleanup

The stale `release/0.1.8` pull request was closed because it had diverged substantially from the active `main` development line and was not mergeable. The 0.1.8 work is not being force-applied over the newer 0.3.0 main history.

Main remains the source of truth for the active 0.3.0 development line.

## Main version integrity

The following main-branch metadata remains synchronized at `0.3.0`:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

The repository must not be tagged as `v0.1.8` from main while main is still on the 0.3.0 development line.

GitHub-side implementation and documentation changes have been completed. Local Rust, Node.js, Tauri, installer, and cross-platform builds have **not** been claimed as passed because this environment does not provide a trustworthy local project checkout and complete toolchain execution path.

The available GitHub connector does not expose a complete check-run listing for arbitrary push workflow executions, so no CI result is fabricated here.

Before publication, run the complete release validation from `release/0.1.7`:

```bash
git checkout release/0.1.7
git pull --ff-only origin release/0.1.7
node scripts/verify-release-version.mjs v0.1.7
## Verification policy

GitHub-side changes are committed, but no local build or test result is claimed merely from source inspection. CI is the authoritative verification gate for the branch.

For the main line, the expected quality suite is:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cd apps/desktop
npm install --legacy-peer-deps --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

The supported desktop installers should additionally be smoke-tested on Windows, macOS, and Linux before production publication.

## Publication procedure

After every automated validation gate is green and installer smoke tests are complete:

```bash
git checkout release/0.1.9
git pull --ff-only origin release/0.1.9
node scripts/verify-release-version.mjs v0.1.9
git diff --check
git status --short
git tag -a v0.1.9 -m "SortSmith v0.1.9"
git push origin v0.1.9
```

Recommended metadata:

- Tag: `v0.1.9`
- Target: `release/0.1.9`
- Title: `SortSmith v0.1.9 — Maintenance Release`
- Pre-release: disabled
- Latest: disabled
- Body: `RELEASE_NOTES_v0.1.9.md`

## Current status

The v0.1.9 release branch is prepared, but it is not yet declared production-ready. Automated CI validation is still the release gate. No green result is being fabricated, and no tag should be published until the complete validation path succeeds.
As of this handoff, `v0.1.7` has **not** been published. The release branch and release materials are prepared, but the tag and GitHub release must be created only after the validation gates pass.
Filesystem behavior should additionally be reviewed for preview-only planning, reversible journals, collision-safe moves, and root containment when symlink following is enabled.

## Next main-line priorities

1. Verify the latest Rust fixes and npm changes through GitHub Actions.
2. Fix any remaining compile, lint, format, test, typecheck, or build failures on the actual 0.3.0 main implementation.
3. Continue the 0.3 development line with small, independently verifiable commits.
4. Keep `what_changed.md` synchronized after each substantive project milestone.
5. Only create a release tag after version metadata, tests, packaging, and release artifacts have all passed their required gates.
