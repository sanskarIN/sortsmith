# SortSmith — Work Handoff

## Current active workstream: v0.1.5 stable release

- Release branch: `release/0.1.5`
- Base: `release/0.1.4`
- Target version: `0.1.5`
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.5 implementation

### Version synchronization

Release metadata is synchronized to `0.1.5` in:

- root `Cargo.toml` workspace package;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

The release tag `v0.1.5` is intended to be checked with `scripts/verify-release-version.mjs` before tagging.

### Safer symlink-directory traversal

`crates/sortsmith-core/src/engine.rs` applies the selected-root boundary during WalkDir entry filtering when link following is enabled.

Symbolic-link entries are resolved before traversal descends into them. A symlink resolving outside the selected root is pruned before WalkDir can traverse its target directory.

This extends v0.1.4: file symlinks were already rejected before planning; v0.1.5 also prevents an external symlink directory from exposing its nested tree to recursive preview traversal.

### Defense in depth

The v0.1.4 file-level containment check remains in the planning loop, and execution continues to validate canonical source and destination-parent containment before filesystem mutation.

### Regression coverage

The Unix-specific v0.1.5 regression creates an external directory containing a matching nested file, links to it from inside the selected root, enables recursive link following, and verifies that the external tree is not traversed or counted as scanned and no organization operation is planned.

The maintenance line also retains the v0.1.1-v0.1.4 coverage for Unicode rule limits, journal replacement, journal-root containment, traversal rejection, portable filename validation, collision reservation, and external file-symlink preview rejection.

## Stable-release documentation finalized

- `CHANGELOG.md` now identifies v0.1.5 as a stable maintenance patch dated 2026-09-04.
- `RELEASE_NOTES_v0.1.5.md` is finalized for a stable release rather than a pre-release.
- `docs/release-v0.1.5-checklist.md` now contains the stable release gate and post-publication verification.
- This `what_changed.md` records the stable-release continuation.

## v0.1.5 commits

The v0.1.5 branch contains the following meaningful work commits:

1. `c8b09fee3d394b4d8224cec09f0189198a920145` — `fix(core): prune external symlink directories during preview`
2. `347741aca0fe61e7327c1f41d1d9022190951ad5` — `release: bump workspace version to 0.1.5`
3. `60bf7902c4e378b16cd28a415cf4ead0cbbd72da` — `release(frontend): bump desktop version to 0.1.5`
4. `4462ad6c3182154432b573c3a2b412eb6754444c` — `release(tauri): bump application version to 0.1.5`
5. `e57ad3c0db594faa88fb527b9b5e5b3b54045bc5` — `docs(changelog): prepare v0.1.5 symlink traversal patch`
6. `daa21b041ef25cf43072b56426c1c1cf0795696e` — `docs(release): add v0.1.5 release notes`
7. `cda55c4db6419ee37f32737d48e7f390d8b07308` — `docs(release): add v0.1.5 publication checklist`
8. `67e3d8ba3f8d623bbd15400d46994bd14195e58d` — `docs(release): finalize v0.1.5 stable release notes`
9. `f5a962fc5e06e6f726284568d54ea07f296bf9d2` — `docs(release): finalize v0.1.5 stable release checklist`
10. `967dbfc813f48e5077e43c105fc56e964bd2c719` — `docs(changelog): finalize v0.1.5 stable release entry`

No empty commits were added merely to inflate history.

## Historical v0.1.x context

The `0.1.x` maintenance line is intentionally separate from the later feature line. `release/0.1.1` originated from the recovered final `0.1.0` source boundary immediately before the later `0.2.0` version bump.

The existing `v0.1.0` tag points at newer repository history and must not be rewritten casually. Maintenance branches preserve the historical source lineage without merging backward into modern `main`.

Maintenance progression:

- v0.1.1: Unicode-character-based rule-value and filename-regex limits.
- v0.1.2: safer existing-journal replacement and core undo-path containment.
- v0.1.3: preview-wide destination reservation for collision-safe planning.
- v0.1.4: preview-time rejection of files resolved outside the selected root when link following is enabled.
- v0.1.5: traversal-time pruning of external symlink directories before recursive descent.

## Release automation

The release workflow is tag driven. It triggers for `v*` tags, verifies release metadata, builds on its configured Linux, Windows, and macOS runners, and uses the Tauri action to prepare a draft GitHub release.

The workflow currently installs frontend dependencies with `npm install`. `npm ci` should only replace this after a real npm lockfile has been generated and committed from a trusted networked environment.

## Verification status

The repository-side stable release preparation is complete. However, this connected environment does not have the project checkout/toolchains required to truthfully report local Rust, Node.js, Tauri, installer, or cross-platform validation as passed.

A direct network clone attempt was unavailable because the execution environment could not resolve `github.com`; therefore no local build/test result is being fabricated.

Before calling the stable release fully verified, run:

```bash
node scripts/verify-release-version.mjs v0.1.5
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

Then review Linux, Windows, and macOS artifacts and perform clean-machine installation/launch smoke tests where available.

## Stable GitHub release procedure

After every applicable validation gate passes:

```bash
git checkout release/0.1.5
git pull origin release/0.1.5
node scripts/verify-release-version.mjs v0.1.5
git diff --check
git status --short
git tag -a v0.1.5 -m "SortSmith v0.1.5"
git push origin v0.1.5
```

The tag should trigger `.github/workflows/release.yml`. Review all generated artifacts and the draft release before publishing.

Recommended GitHub release settings:

- Tag: `v0.1.5`
- Target: `release/0.1.5`
- Title: `SortSmith v0.1.5 — Patch Release`
- Pre-release: disabled
- Latest release: disabled
- Body: `RELEASE_NOTES_v0.1.5.md`

Because modern `main` is already on a later `0.3.x` feature line, v0.1.5 should remain a maintenance release and should not be selected as the repository's latest release.

## Release-publication limitation

The available GitHub integration can update repository files and branches and inspect GitHub state, but it does not expose a tag-creation or release-publication action. Consequently, the final tag push and GitHub release publication must be performed by the repository owner after local validation.

Do not claim v0.1.5 has been published until the tag and published GitHub release can be verified.

## After v0.1.5

Do not merge the `0.1.x` maintenance branch into modern `main` merely to advance the patch version. Once v0.1.5 is verified and published, return to the modern feature-development line for the next milestone.

The next feature release should be planned from the current `main`/development state rather than backporting unrelated `0.2.x` or `0.3.x` functionality into `0.1.x`.
