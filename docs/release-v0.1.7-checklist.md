# SortSmith v0.1.7 Release Checklist

This checklist describes publication of the 0.1.7 maintenance tag from `release/0.1.7`.

## Source and version

- [ ] Confirm `release/0.1.7` is based on `release/0.1.6`.
- [ ] Confirm cached-preview safety and collision fixes are present.
- [ ] Confirm `Cargo.toml`, `apps/desktop/package.json`, and `apps/desktop/src-tauri/tauri.conf.json` report `0.1.7` on the release branch.
- [ ] Run `node scripts/verify-release-version.mjs v0.1.7`.

## Core validation

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `git diff --check`
- [ ] Unix cached-preview external-symlink regression passes.
- [ ] Cached-preview duplicate-destination regression passes.

## Desktop validation

- [ ] `npm install --no-audit --no-fund`
- [ ] `npm run typecheck`
- [ ] `npm test`
- [ ] `npm run build`
- [ ] `npm run tauri build`

## Cross-platform validation

- [ ] Linux CI/build/package is green.
- [ ] Windows CI/build/package is green.
- [ ] macOS CI/build/package is green.
- [ ] Generated installers show version `0.1.7`.

## GitHub publication

- [ ] Create/push `v0.1.7` from `release/0.1.7` only.
- [ ] Confirm the tag-triggered release workflow runs.
- [ ] Review the generated draft release and its artifacts.
- [ ] Title: `SortSmith v0.1.7 — Patch Release`.
- [ ] Body: `RELEASE_NOTES_v0.1.7.md`.
- [ ] Pre-release: disabled.
- [ ] Latest: disabled.
- [ ] Publish only after all gates are green.

## Post-release

- [ ] Confirm the GitHub release is published and not a draft.
- [ ] Confirm the tag points to the intended `release/0.1.7` maintenance commit.
- [ ] Confirm release assets are version `0.1.7`.
- [ ] Keep the 0.1.x maintenance history separate from the later feature line.
- [ ] Update `what_changed.md` with the final publication status.
