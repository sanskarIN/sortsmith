# SortSmith v0.1.6 Stable Release Checklist

## Release identity

- [x] Version: `0.1.6`
- [x] Tag: `v0.1.6`
- [x] Target branch: `release/0.1.6`
- [x] Title: `SortSmith v0.1.6 — Patch Release`
- [x] Stable release; pre-release disabled
- [x] Do not mark as latest because the main feature line is newer

## Repository gate

- [x] Dedicated maintenance branch created from `release/0.1.5`
- [x] Workspace version synchronized
- [x] Desktop package version synchronized
- [x] Tauri application version synchronized
- [x] Changelog entry added
- [x] Release notes added
- [x] Public API-level symlink traversal regression test added
- [x] Duplicate-scanner external-symlink regression added
- [x] Relative journal-path regression added
- [x] Post-preview collision regression added
- [x] No-overwrite move primitive added for execution and undo
- [x] Background watched-folder overlap guard added
- [x] Automation preset-selection synchronization added
- [x] `what_changed.md` updated

## Core validation

Run from the repository root:

```bash
node scripts/verify-release-version.mjs v0.1.6
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

The Unix integration regression is:

```text
crates/sortsmith-core/tests/external_symlink_traversal.rs
```

It verifies that recursive preview with `follow_links` enabled does not traverse an external symlink directory or plan operations for files beneath it.

Additional core cases that must be covered by the workspace tests include:

- Relative-root execution produces an absolute, undoable journal.
- A destination created after preview is preserved and the source is moved to a collision-safe suffix.
- Generated collision filenames remain within portable filename limits.
- Windows reserved-name and case-insensitive collision behavior passes on Windows CI.
- Duplicate scanning does not traverse an external symlink directory.
- Journal replacement and directory durability tests pass on their supported platforms.

## Desktop validation

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

Also manually verify that:

- [ ] A slow watched-folder run cannot overlap with the next timer tick.
- [ ] Preset selection becomes available after saved state finishes loading.
- [ ] Adding a watched folder at the 100-folder limit is rejected before persistence.
- [ ] A failed settings save does not get described to the user as a successful change.

## Cross-platform gate

- [ ] Linux build completes successfully.
- [ ] Windows build completes successfully.
- [ ] macOS build completes successfully.
- [ ] Generated installers/bundles are inspected.
- [ ] Clean-machine installation and launch smoke tests pass where available.
- [ ] No unexpected security or filesystem permission prompts are introduced.

## Tagging

Only after validation passes:

```bash
git checkout release/0.1.6
git pull origin release/0.1.6
node scripts/verify-release-version.mjs v0.1.6
git diff --check
git status --short
git tag -a v0.1.6 -m "SortSmith v0.1.6"
git push origin v0.1.6
```

## GitHub release configuration

- Tag: `v0.1.6`
- Target: `release/0.1.6`
- Title: `SortSmith v0.1.6 — Patch Release`
- Pre-release: unchecked
- Latest release: unchecked
- Body: use `RELEASE_NOTES_v0.1.6.md`

## Publication verification

After the tag is pushed:

1. Confirm the tag resolves to the final `release/0.1.6` commit.
2. Confirm the release workflow starts.
3. Review Linux, Windows, and macOS artifacts.
4. Verify artifact version metadata reports `0.1.6`.
5. Confirm the GitHub release is published as stable.
6. Keep it non-latest because the repository's modern main line is newer.

## Tooling limitation

The available GitHub integration can create branches and update repository files, but it does not expose tag creation or GitHub release publication. Therefore the final tag push and publication must be performed by the repository owner after local and cross-platform validation.

Do not report v0.1.6 as published until the tag and published GitHub release are independently verified.
