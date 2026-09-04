# SortSmith v0.1.7 Release Checklist

This checklist covers publication of the 0.1.7 maintenance release from `release/0.1.7`.

## Source

- [ ] Confirm `release/0.1.7` is based on `release/0.1.6`.
- [ ] Confirm the deterministic duplicate-result fix is present.
- [ ] Confirm no 0.3.0 feature work has been backported into the maintenance branch.

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
- [ ] Verify duplicate group members are returned in stable path order.
- [ ] Verify existing duplicate content, hidden-directory, and external-symlink tests pass.

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
- [ ] Confirm generated application metadata is version `0.1.7`.

## GitHub release

- [ ] Create/push tag `v0.1.7` from `release/0.1.7` only.
- [ ] Confirm the tag-triggered release workflow starts.
- [ ] Review the generated draft release and all artifacts.
- [ ] Title: `SortSmith v0.1.7 — Patch Release`.
- [ ] Body: `RELEASE_NOTES_v0.1.7.md`.
- [ ] Pre-release: disabled.
- [ ] Latest: disabled.
- [ ] Publish only after validation and artifact checks are green.

## Post-release

- [ ] Confirm the GitHub release is published and not a draft.
- [ ] Confirm `v0.1.7` points to the intended maintenance-line commit/tag.
- [ ] Confirm release assets correspond to version `0.1.7`.
- [ ] Update `what_changed.md` with the final publication status.
