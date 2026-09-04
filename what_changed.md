# SortSmith — Work Handoff

## Current active workstream: v0.1.5

- Release branch: `release/0.1.5`
- Base: `release/0.1.4`
- Target patch version: `0.1.5`
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.5 implementation completed

### Version synchronization

The following release metadata now reports `0.1.5` on `release/0.1.5`:

- root `Cargo.toml` workspace package;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

The existing `scripts/verify-release-version.mjs` is intended to verify that a release tag such as `v0.1.5` matches all three metadata sources.

### Safer symlink-directory traversal

`crates/sortsmith-core/src/engine.rs` now applies the selected-root boundary during WalkDir entry filtering when link following is enabled.

Symbolic-link entries are resolved before traversal descends into them. If a symlink resolves outside the selected root, the entry is pruned before WalkDir can traverse its target directory.

This strengthens v0.1.4: file symlinks were already rejected before planning; v0.1.5 also prevents an external symlink directory from exposing its nested tree to recursive preview traversal.

### Defense in depth retained

The v0.1.4 file-level containment check remains in the planning loop, and execution still validates canonical source and destination-parent containment before filesystem mutation. The new traversal filter is an earlier boundary, not a replacement for later safety checks.

### Regression coverage

Added a Unix-specific core regression test that:

- creates a selected root and an external directory;
- creates a nested matching file in the external directory;
- creates a symlink directory inside the selected root pointing to the external directory;
- enables recursive link following;
- verifies no organization operation is planned;
- verifies the external nested file is not counted as scanned.

The existing v0.1.1–v0.1.4 coverage remains in the maintenance line, including Unicode rule limits, journal replacement, journal-root containment, traversal rejection, portable filenames, collision reservation, and external file-symlink preview rejection.

## Release documentation completed

- `CHANGELOG.md` now has a dedicated `0.1.5` entry dated 2026-09-04.
- `RELEASE_NOTES_v0.1.5.md` contains publication-ready pre-release notes and validation guidance.
- `docs/release-v0.1.5-checklist.md` contains metadata, Rust, desktop, symlink-security, cross-platform, and GitHub release gates.
- This `what_changed.md` records the v0.1.5 continuation.

## v0.1.5 commits

1. `c8b09fee3d394b4d8224cec09f0189198a920145` — `fix(core): prune external symlink directories during preview`
2. `347741aca0fe61e7327c1f41d1d9022190951ad5` — `release: bump workspace version to 0.1.5`
3. `60bf7902c4e378b16cd28a415cf4ead0cbbd72da` — `release(frontend): bump desktop version to 0.1.5`
4. `4462ad6c3182154432b573c3a2b412eb6754444c` — `release(tauri): bump application version to 0.1.5`
5. `e57ad3c0db594faa88fb527b9b5e5b3b54045bc5` — `docs(changelog): prepare v0.1.5 symlink traversal patch`
6. `daa21b041ef25cf43072b56426c1c1cf0795696e` — `docs(release): add v0.1.5 release notes`
7. `cda55c4db6419ee37f32737d48e7f390d8b07308` — `docs(release): add v0.1.5 publication checklist`
8. Current handoff update — records this continuation and release state.

The commits are separated by responsibility and contain real code, release metadata, tests, changelog, release-note, checklist, or handoff changes. No empty commits were added merely to inflate history.

## Historical v0.1.x context

The `0.1.x` maintenance line is intentionally separate from the later feature line. `release/0.1.1` was created from the recovered final `0.1.0` source boundary immediately before the later `0.2.0` version bump.

The existing `v0.1.0` tag points at newer repository history and must not be rewritten casually. The maintenance branches preserve the historical source lineage without merging backward into modern `main`.

The preceding maintenance releases added:

- v0.1.1: Unicode-character-based rule-value and filename-regex limits.
- v0.1.2: safer existing-journal replacement and core undo-path containment.
- v0.1.3: preview-wide destination reservation for collision-safe planning.
- v0.1.4: preview-time rejection of files resolved outside the selected root when link following is enabled.
- v0.1.5: traversal-time pruning of external symlink directories before recursive descent.

## Release automation reviewed

The release workflow is tag driven. It triggers for `v*` tags, verifies release metadata, builds on its configured Linux, Windows, and macOS runners, and uses the Tauri action to prepare a draft GitHub release.

The workflow currently installs frontend dependencies with `npm install`. `npm ci` should only replace this after a real npm lockfile has been generated and committed from a trusted networked environment.

## Verification status

Repository preparation is complete, but this connected environment has not executed the Rust or frontend toolchains locally. Therefore `v0.1.5` must not yet be described as fully tested, built, or published.

Required local verification:

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

## Release procedure

After all local checks pass:

```bash
git checkout release/0.1.5
git pull origin release/0.1.5
node scripts/verify-release-version.mjs v0.1.5
git diff --check
git tag -a v0.1.5 -m "SortSmith v0.1.5"
git push origin v0.1.5
```

The tag push should trigger `.github/workflows/release.yml`. Review generated platform artifacts before publishing the draft release.

Recommended GitHub release settings:

- Tag: `v0.1.5`
- Target: `release/0.1.5`
- Title: `SortSmith v0.1.5 — Patch Pre-release`
- Pre-release: enabled
- Latest release: disabled
- Body: `RELEASE_NOTES_v0.1.5.md`

## Known release blockers

- Local Rust format/check/test/Clippy results have not been executed in this environment.
- Local frontend typecheck/test/build results have not been executed in this environment.
- Cross-platform Tauri artifacts for `v0.1.5` have not yet been observed.
- Clean-machine installer smoke tests remain required.
- Signing/notarization remains unverified.
- Dependency lockfiles remain a separate reproducibility task unless generated and committed in the maintenance line.
- The available repository integration can prepare and push branch/file changes but does not provide a release-publication action; tag creation and GitHub release publication remain release-owner steps.

## After v0.1.5

Do not merge the `0.1.x` maintenance branch into modern `main` merely to advance the patch version. After the maintenance pre-release is verified, return to the modern feature-development line for the next milestone.

The next feature release should be planned from the current `main`/development state rather than backporting unrelated `0.2.x` or `0.3.x` functionality into `0.1.x`.

## Continuation rule

Before publishing `v0.1.5`, execute the complete local release gate, push the exact tag, inspect all platform workflow results, review artifacts, and perform clean-machine smoke tests. Do not claim publication until those gates have actually passed.
