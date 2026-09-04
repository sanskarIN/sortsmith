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

### Public API-level symlink traversal coverage

The v0.1.5 implementation prevents recursive traversal into symbolic-link directories whose resolved targets are outside the selected root when `follow_links` is enabled. v0.1.6 adds a public integration test at `crates/sortsmith-core/tests/external_symlink_traversal.rs` that exercises `preview_organization` and verifies an external linked directory produces no planned operation and no scanned nested external file.

### Windows filename portability hardening

`crates/sortsmith-core/src/safety.rs` rejects Unicode superscript aliases for numbered Windows device names, including `COM¹`, `COM²`, `COM³`, `LPT¹`, `LPT²`, and `LPT³`.

The same collision planner now treats reserved destination paths case-insensitively on Windows. This matters because Windows path comparison is case-insensitive while `HashSet<PathBuf>` equality is not. Without this normalization, two preview operations could reserve differently cased spellings of the same Windows destination and converge on one physical path.

A Windows-specific regression test covers differently cased reserved paths and verifies that the next collision suffix is selected.

### Release metadata

Version `0.1.6` is synchronized across the Rust workspace, desktop package, and Tauri application configuration.

## v0.1.6 documentation

- `CHANGELOG.md` records the stable v0.1.6 maintenance release.
- `RELEASE_NOTES_v0.1.6.md` contains the stable release body.
- `docs/release-v0.1.6-checklist.md` contains release validation and publication gates.
- This handoff records the actual implementation and documentation work.

## Important history note

Not every earlier v0.1.6 preparation commit represents product functionality. In particular, the earlier rule-test formatting refactor is documentation/maintenance noise rather than a feature claim. The substantive code work for this continuation is the symlink integration regression coverage, Unicode Windows device-name validation, and Windows case-insensitive reserved-destination collision handling.

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

The Unix public symlink regression and Windows-specific collision test should execute on their respective platforms as part of the workspace test suite.

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
