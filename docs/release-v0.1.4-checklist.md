# SortSmith v0.1.4 Release Checklist

## Release identity

- [x] Maintenance branch: `release/0.1.4`
- [x] Version: `0.1.4`
- [x] Changelog entry added
- [x] Release notes added
- [ ] Annotated tag `v0.1.4` created after validation
- [ ] GitHub release published after the tag workflow completes

## Metadata gate

Run from the repository root:

```bash
node scripts/verify-release-version.mjs v0.1.4
git diff --check
```

Expected versions:

- Root Cargo workspace: `0.1.4`
- Desktop package: `0.1.4`
- Tauri configuration: `0.1.4`

## Rust quality gate

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Desktop quality gate

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

## Security regression gate

Confirm coverage for:

- Preview path containment
- External symlink targets when `follow_links` is enabled
- Recursive duplicate destination reservation
- Journal root containment during undo
- Journal replacement on existing targets
- Unicode-safe rule length validation
- Unsafe destination traversal

## Cross-platform gate

The release workflow is tag-driven and should build the Tauri application on its configured Linux, Windows, and macOS runners. Do not call the release fully validated until those workflow jobs succeed.

## GitHub publication fields

- Tag: `v0.1.4`
- Target: `release/0.1.4`
- Title: `SortSmith v0.1.4 — Patch Release`
- Body: contents of `RELEASE_NOTES_v0.1.4.md`
- Prerelease: No
- Latest: keep disabled if a newer `0.2.x`/`0.3.x` release is already the current latest release

## Tag commands

After all local gates pass:

```bash
git checkout release/0.1.4
git pull origin release/0.1.4
node scripts/verify-release-version.mjs v0.1.4
git diff --check
git tag -a v0.1.4 -m "SortSmith v0.1.4"
git push origin v0.1.4
```

The repository release workflow is configured to create a draft release from `v*` tags. Publish the generated draft only after the cross-platform build is green.
