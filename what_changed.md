# SortSmith — Work Handoff

## Current active workstream: v0.1.6 stable release

- Release branch: `release/0.1.6`
- Base: `release/0.1.5`
- Target version: `0.1.6`
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.6 implementation

### Version synchronization

Release metadata is synchronized to `0.1.6` in:

- root `Cargo.toml` workspace package;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

The release tag `v0.1.6` is intended to be checked with `scripts/verify-release-version.mjs` before tagging.

### Public API-level symlink traversal coverage

The v0.1.5 implementation already prevents recursive traversal into symbolic-link directories whose resolved targets are outside the selected root when `follow_links` is enabled. v0.1.6 strengthens the maintenance contract by adding a public integration test at:

`crates/sortsmith-core/tests/external_symlink_traversal.rs`

The test uses the exported `preview_organization` API rather than private helper functions. It creates an external directory containing a nested matching file, creates a symbolic link to that external directory from inside the selected root, enables recursive link following, and verifies that:

- no organization operation is planned;
- the nested external file is not counted as scanned;
- the external file remains present and untouched.

The regression is Unix-specific because the test directly exercises the Unix symbolic-link API.

### Why this is a separate patch release

v0.1.6 is intentionally a maintenance/quality patch rather than a feature release. Its purpose is to make the security behavior introduced in v0.1.5 an observable public API contract that future refactors must preserve.

No unrelated `0.2.x` or `0.3.x` features are backported into this maintenance line.

## v0.1.6 documentation finalized

- `CHANGELOG.md` now identifies v0.1.6 as a stable maintenance release dated 2026-09-04.
- `RELEASE_NOTES_v0.1.6.md` contains the stable release body.
- `docs/release-v0.1.6-checklist.md` contains repository, validation, tagging, artifact, and publication gates.
- This `what_changed.md` records the v0.1.6 continuation and release state.

## v0.1.6 commits created in this continuation

1. `3d0e2f0a81c3da2713c26c96dd14fa5539e78323` — `release: bump workspace version to 0.1.6`
2. `2be3b2697d8857e10d3a5c7c0e7c384732e775b6` — `release(frontend): bump desktop version to 0.1.6`
3. `3e1a7ebe2cc3fc8860fd81094b49dfc1886916df` — `release(tauri): bump application version to 0.1.6`
4. `5e433d4b0df184c50488fdf7132a0a5736b65505` — `test(core): add public symlink traversal safety regression`
5. `a50d4009fb917e757419882b858fb29a61ad7dcd` — `docs(changelog): prepare v0.1.6 integration safety maintenance release`
6. `41943976645f080f1a1e5b835a8aa19e03e2587b` — `docs(release): add v0.1.6 stable release notes`
7. `185f0c0d0861c1cb99f428782ace80ab7b509b6e` — `docs(release): add v0.1.6 stable release checklist`
8. This handoff update records the final v0.1.6 continuation state.

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
- v0.1.6: public API-level regression coverage for the v0.1.5 symlink traversal boundary.

## Release automation

The release workflow is tag driven. It triggers for `v*` tags, verifies release metadata, builds on its configured Linux, Windows, and macOS runners, and uses the Tauri action to prepare a draft GitHub release.

The workflow currently installs frontend dependencies with `npm install`. `npm ci` should only replace this after a real npm lockfile has been generated and committed from a trusted networked environment.

## Verification status

Repository-side v0.1.6 release preparation is complete through branch creation, implementation/test coverage, version synchronization, changelog, release notes, checklist, and handoff documentation.

Local Rust, Node.js, Tauri, installer, and cross-platform validation has not been claimed as passed because this connected environment does not provide the project checkout/toolchains required to execute those commands truthfully.

Before calling v0.1.6 fully verified, run:

```bash
git checkout release/0.1.6
git pull origin release/0.1.6
node scripts/verify-release-version.mjs v0.1.6
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

The Unix public regression should be included automatically by `cargo test --workspace` on Unix-capable runners.

Then review Linux, Windows, and macOS artifacts and perform clean-machine installation/launch smoke tests where available.

## Stable GitHub release procedure

After every applicable validation gate passes:

```bash
git checkout release/0.1.6
git pull origin release/0.1.6
node scripts/verify-release-version.mjs v0.1.6
git diff --check
git status --short
git tag -a v0.1.6 -m "SortSmith v0.1.6"
git push origin v0.1.6
```

The tag should trigger `.github/workflows/release.yml`. Review all generated artifacts and the draft release before publishing.

Recommended GitHub release settings:

- Tag: `v0.1.6`
- Target: `release/0.1.6`
- Title: `SortSmith v0.1.6 — Patch Release`
- Pre-release: disabled
- Latest release: disabled
- Body: `RELEASE_NOTES_v0.1.6.md`

Because modern `main` is already on a later `0.3.x` feature line, v0.1.6 should remain a maintenance release and should not be selected as the repository's latest release.

## Release-publication limitation

The available GitHub integration can create branches and update repository files and can inspect repository state, but it does not expose a tag-creation or GitHub release-publication action. Consequently, the final tag push and GitHub release publication must be performed by the repository owner after local validation.

Do not claim v0.1.6 has been published until the tag and published GitHub release can be independently verified.

## After v0.1.6

Do not merge the `0.1.x` maintenance branch into modern `main` merely to advance the patch version. Once v0.1.6 is verified and published, return to the modern feature-development line for the next milestone.

The next feature release should be planned from the current `main`/development state rather than backporting unrelated `0.2.x` or `0.3.x` functionality into `0.1.x`.
