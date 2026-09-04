# SortSmith v0.1.2 Release Checklist

## 1. Source integrity

- [ ] Confirm `release/0.1.2` starts from the verified `0.1.1` maintenance line.
- [ ] Confirm no feature-line changes from `0.2.x` or `0.3.x` were backported unintentionally.
- [ ] Review the final commit diff for only intended patch changes.

## 2. Version consistency

Verify all release metadata:

```bash
node scripts/verify-release-version.mjs v0.1.2
```

Expected versions:

- `Cargo.toml`: `0.1.2`
- `apps/desktop/package.json`: `0.1.2`
- `apps/desktop/src-tauri/tauri.conf.json`: `0.1.2`

## 3. Rust verification

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
git diff --check
```

Pay particular attention to:

- journal replacement tests;
- forged-journal containment tests;
- existing path traversal and filename safety tests;
- execution and undo round trips.

## 4. Frontend verification

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

## 5. Packaging verification

```bash
npm run tauri build
```

On a trusted clean environment, verify the resulting package launches and the application reports version `0.1.2`.

## 6. GitHub Actions

After the `v0.1.2` tag is pushed:

- [ ] Confirm the release workflow starts.
- [ ] Confirm version metadata verification passes.
- [ ] Confirm Ubuntu build succeeds.
- [ ] Confirm Windows build succeeds.
- [ ] Confirm macOS build succeeds.
- [ ] Review generated artifacts.
- [ ] Confirm no unexpected build warnings/errors affect distribution.

## 7. Installer smoke tests

For each supported platform:

- [ ] Install from the generated artifact on a clean machine/VM.
- [ ] Launch SortSmith.
- [ ] Verify the application version is `0.1.2`.
- [ ] Create a dry-run preview.
- [ ] Apply a safe test operation.
- [ ] Undo the operation.
- [ ] Verify journal history remains readable.
- [ ] Uninstall and confirm normal cleanup behavior.

## 8. Security checks

- [ ] Confirm no external-path journal can be undone.
- [ ] Confirm symlink-aware desktop journal validation remains active.
- [ ] Confirm no credentials, tokens, signing keys, or secrets entered the repository.
- [ ] Confirm operation logs remain privacy-safe.

## 9. GitHub release publication

Use:

- Tag: `v0.1.2`
- Title: `SortSmith v0.1.2 — Patch Release`
- Target: `release/0.1.2`
- Release body: `RELEASE_NOTES_v0.1.2.md`

The repository's tag-driven release workflow is configured to create a draft release. Review its artifacts before publishing the draft.

## 10. Post-release

- [ ] Confirm the `v0.1.2` tag points to the intended maintenance commit.
- [ ] Confirm release artifacts are downloadable.
- [ ] Confirm release notes and changelog agree.
- [ ] Update `what_changed.md` with final CI and packaging evidence.
- [ ] Keep `release/0.1.x` separate from the modern feature-development line.
- [ ] Resume feature work from the modern `main` branch after the maintenance release is complete.
