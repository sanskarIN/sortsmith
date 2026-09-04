# SortSmith v0.1.7 Release Checklist

## Source

- [ ] Confirm `release/0.1.7` contains the intended maintenance fixes.
- [ ] Confirm the release branch is based on `release/0.1.6`.
- [ ] Confirm no unrelated feature work was added to the maintenance branch.

## Version integrity

- [ ] `Cargo.toml` reports `0.1.7`.
- [ ] `apps/desktop/package.json` reports `0.1.7`.
- [ ] `apps/desktop/src-tauri/tauri.conf.json` reports `0.1.7`.
- [ ] Run `node scripts/verify-release-version.mjs v0.1.7`.

## Core validation

- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo check --workspace`.
- [ ] Run `cargo test --workspace`.
- [ ] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- [ ] Run `git diff --check`.
- [ ] Verify the cached-preview external-symlink regression passes on Unix.
- [ ] Verify the cached-preview duplicate-destination regression passes.
- [ ] Verify existing cache invalidation and time-sensitive rule tests still pass.

## Desktop validation

- [ ] Run `npm install --no-audit --no-fund` from `apps/desktop`.
- [ ] Run `npm run typecheck`.
- [ ] Run `npm test`.
- [ ] Run `npm run build`.
- [ ] Run `npm run tauri build`.

## Cross-platform release gate

- [ ] Validate Linux build/package.
- [ ] Validate Windows build/package.
- [ ] Validate macOS build/package.
- [ ] Confirm the release workflow's generated artifacts are present and usable.
- [ ] Review installer/application metadata for version `0.1.7`.

## GitHub release

- [ ] Create/push tag `v0.1.7` from `release/0.1.7` only.
- [ ] Confirm the tag-triggered release workflow starts.
- [ ] Review the generated draft release.
- [ ] Set title to `SortSmith v0.1.7 — Patch Release`.
- [ ] Use `RELEASE_NOTES_v0.1.7.md` as the release body.
- [ ] Keep **Pre-release** disabled.
- [ ] Keep **Latest** disabled because this is a maintenance release on the 0.1.x line.
- [ ] Publish only after all validation and artifact checks are green.

## Post-release

- [ ] Confirm the GitHub release is published and not a draft.
- [ ] Confirm `v0.1.7` points to the intended maintenance-line commit/tag.
- [ ] Confirm the release assets correspond to version `0.1.7`.
- [ ] Keep the 0.1.x maintenance history separate from the later feature-development line.
- [ ] Update `what_changed.md` with the final publication status.
