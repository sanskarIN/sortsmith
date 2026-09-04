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

Release metadata is synchronized to `0.1.6` in the Rust workspace, desktop package, and Tauri application configuration.

### Public API-level symlink traversal coverage

The v0.1.5 implementation prevents recursive traversal into symbolic-link directories whose resolved targets are outside the selected root when `follow_links` is enabled. v0.1.6 adds a public integration test at `crates/sortsmith-core/tests/external_symlink_traversal.rs` that exercises `preview_organization` and verifies an external linked directory produces no planned operation and no scanned nested external file.

### Windows filename portability hardening

`crates/sortsmith-core/src/safety.rs` now rejects Unicode superscript aliases for numbered Windows device names, including `COM¹`, `COM²`, `COM³`, `LPT¹`, `LPT²`, and `LPT³`. Regression coverage verifies these aliases are rejected while `COM0` remains valid.

This closes a cross-platform filename-validation edge case without changing valid ordinary filenames.

## v0.1.6 documentation

- `CHANGELOG.md` records the stable v0.1.6 maintenance release.
- `RELEASE_NOTES_v0.1.6.md` contains the stable release body.
- `docs/release-v0.1.6-checklist.md` contains release validation and publication gates.
- This handoff records the actual implementation and documentation work.

## Commits created in this continuation

1. `3d0e2f0a81c3da2713c26c96dd14fa5539e78323` — `release: bump workspace version to 0.1.6`
2. `2be3b2697d8857e10d3a5c7c0e7c384732e775b6` — `release(frontend): bump desktop version to 0.1.6`
3. `3e1a7ebe2cc3fc8860fd81094b49dfc1886916df` — `release(tauri): bump application version to 0.1.6`
4. `5e433d4b0df184c50488fdf7132a0a5736b65505` — `test(core): add public symlink traversal safety regression`
5. `a50d4009fb917e757419882b858fb29a61ad7dcd` — `docs(changelog): prepare v0.1.6 integration safety maintenance release`
6. `41943976645f080f1a1e5b835a8aa19e03e2587b` — `docs(release): add v0.1.6 stable release notes`
7. `185f0c0d0861c1cb99f428782ace80ab7b509b6e` — `docs(release): add v0.1.6 stable release checklist`
8. `b8eda6f46312bbb9062170633890efefe53d3169` — `test(core): cover Windows UTF-16 filename boundary`
9. `81b8731db92b8c30a1b5a202b6070742a1f0150a` — `refactor(core): normalize rule validation test fixture formatting`
10. `e8c1962711d7769d9b6c09f5c2688ee3cc0705e8` — `fix(core): reject Unicode Windows device-name aliases`
11. `de324a0eb86179b30b3f236ece34d8741a9bfe36` — `docs(release): refine v0.1.6 release notes with portability fix`
12. `a476b2bf0b28741417458e800f9f500aaffd7f92` — `docs(changelog): record v0.1.6 portability hardening`

The branch now contains 20 commits ahead of `release/0.1.5` including the earlier v0.1.6 preparation commits and this continuation. No empty commits were intentionally created merely to inflate history.

## Verification status

Repository-side implementation and release documentation are prepared. Local Rust, Node.js, Tauri, installer, and cross-platform builds have not been claimed as passed because this connected environment does not provide the project checkout/toolchains needed to execute them truthfully.

Run the following before publication:

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

The Unix public symlink regression and filename portability tests should execute as part of the workspace test suite on Unix-capable runners.

## Stable release procedure

After validation passes:

```bash
git checkout release/0.1.6
git pull origin release/0.1.6
node scripts/verify-release-version.mjs v0.1.6
git diff --check
git status --short
git tag -a v0.1.6 -m "SortSmith v0.1.6"
git push origin v0.1.6
```

Then review the tag-triggered release workflow, generated Linux/Windows/macOS artifacts, and draft GitHub release before publishing.

Recommended release metadata:

- Tag: `v0.1.6`
- Target: `release/0.1.6`
- Title: `SortSmith v0.1.6 — Patch Release`
- Pre-release: disabled
- Latest: disabled
- Body: `RELEASE_NOTES_v0.1.6.md`

The available GitHub integration can modify repository files and branches but does not expose tag creation or GitHub release publication. Therefore the final tag push and publication remain an owner-side step after validation.

Do not claim v0.1.6 is published until the tag and GitHub release can be independently verified.

## Maintenance-line boundary

Keep the 0.1.x maintenance branch separate from modern `main`, which is already on a later feature-development line. Do not merge the maintenance branch into modern main merely to advance the patch version.
