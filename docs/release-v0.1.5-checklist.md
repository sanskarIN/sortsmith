# SortSmith v0.1.5 Release Checklist

## Scope

Patch pre-release for the `release/0.1.x` maintenance line. The primary change is earlier pruning of symbolic-link directories that resolve outside the selected root during recursive preview.

## Metadata gate

- [ ] Root workspace version is `0.1.5`.
- [ ] Desktop package version is `0.1.5`.
- [ ] Tauri application version is `0.1.5`.
- [ ] `node scripts/verify-release-version.mjs v0.1.5` passes.

## Core validation

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `git diff --check`

## Desktop validation

From `apps/desktop`:

- [ ] `npm install --no-audit --no-fund`
- [ ] `npm run typecheck`
- [ ] `npm test`
- [ ] `npm run build`
- [ ] `npm run tauri build`

## Symlink safety regression gate

- [ ] Unix file-symlink regression passes.
- [ ] Unix external symlink-directory regression passes.
- [ ] Recursive preview with `follow_links` disabled remains unchanged.
- [ ] Recursive preview with an internal symlink target remains usable.
- [ ] External symlink directories are not traversed into an outside tree.
- [ ] Execution-time path containment remains enabled.

## Cross-platform gate

- [ ] Linux artifact built successfully.
- [ ] Windows artifact built successfully.
- [ ] macOS artifact built successfully.
- [ ] Clean-machine install/launch smoke test completed for each available platform.
- [ ] Existing configuration and journal behavior remains compatible.

## GitHub release gate

Create/push the tag only after the validation above is green:

```bash
git checkout release/0.1.5
git pull origin release/0.1.5
node scripts/verify-release-version.mjs v0.1.5
git diff --check
git tag -a v0.1.5 -m "SortSmith v0.1.5"
git push origin v0.1.5
```

Recommended release metadata:

- Tag: `v0.1.5`
- Target: `release/0.1.5`
- Title: `SortSmith v0.1.5 — Patch Pre-release`
- Pre-release: enabled
- Latest release: disabled
- Body: `RELEASE_NOTES_v0.1.5.md`

The tag-triggered workflow should create a draft release. Review artifacts before publishing it.

## Known limitations

The repository integration can update branches and files but does not expose a release-publication action. Tag creation and GitHub release publication remain release-owner steps.
