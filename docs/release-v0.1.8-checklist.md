# SortSmith v0.1.8 Release Checklist

## Source integrity

- [x] Dedicated `release/0.1.8` branch created from `release/0.1.7`.
- [x] Rust workspace version is `0.1.8`.
- [x] Desktop package version is `0.1.8`.
- [x] Tauri configuration version is `0.1.8`.
- [x] v0.1.8 changelog entry added.
- [x] v0.1.8 release notes added.
- [x] README release-status wording corrected.
- [x] Work handoff updated.

## Automated validation

- [ ] `node scripts/verify-release-version.mjs v0.1.8`
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `git diff --check`
- [ ] `git status --short`
- [ ] `npm install --no-audit --no-fund` from `apps/desktop`
- [ ] `npm run typecheck`
- [ ] `npm test`
- [ ] `npm run build`
- [ ] `npm run tauri build`

## Cross-platform release smoke tests

- [ ] Windows installer launches successfully.
- [ ] Windows preview/apply/undo smoke test passes.
- [ ] macOS installer launches successfully.
- [ ] macOS preview/apply/undo smoke test passes.
- [ ] Linux package launches successfully.
- [ ] Linux preview/apply/undo smoke test passes.
- [ ] No unintended filesystem mutation occurs during preview.
- [ ] Duplicate detection remains non-destructive.
- [ ] Existing collision and symlink safety behavior remains intact.

## Publication gate

Do not publish `v0.1.8` until every automated validation and required installer smoke test above is green.

After validation:

```bash
git checkout release/0.1.8
git pull --ff-only origin release/0.1.8
node scripts/verify-release-version.mjs v0.1.8
git diff --check
git status --short
git tag -a v0.1.8 -m "SortSmith v0.1.8"
git push origin v0.1.8
```

The tag-triggered GitHub Actions release workflow should then prepare the release artifacts. Review the resulting draft and artifacts before publishing.
