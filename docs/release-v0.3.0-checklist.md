# SortSmith v0.3.0 Release Checklist

This checklist is for the first public GitHub release from the current `main` history.

## Version identity

- [x] Workspace version is `0.3.0`.
- [x] Desktop frontend version is `0.3.0`.
- [x] Tauri application version is `0.3.0`.
- [ ] Verify the release-version checker reports `0.3.0` everywhere it covers.
- [ ] Confirm no unintended version references remain inconsistent with the release.

## Source validation

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Frontend validation

From `apps/desktop`:

- [ ] `npm ci`
- [ ] `npm run typecheck`
- [ ] `npm test -- --run`
- [ ] `npm run build`

## Desktop validation

- [ ] Run the desktop host test suite.
- [ ] Verify application startup.
- [ ] Verify folder selection.
- [ ] Verify rule editing and preset loading.
- [ ] Verify dry-run preview.
- [ ] Verify collision-safe destination planning.
- [ ] Verify apply and undo behavior on disposable test data.
- [ ] Verify watched-folder behavior on disposable test data.
- [ ] Verify settings backup/import.
- [ ] Verify keyboard shortcuts and shortcut dialog focus behavior.

## Security validation

- [ ] Verify root containment and traversal protections.
- [ ] Verify destination symlink escape protection.
- [ ] Verify no-link-following scan behavior.
- [ ] Verify portable filename and Windows-reserved-name validation.
- [ ] Verify operation-log privacy behavior.
- [ ] Verify settings/log symlink protections.
- [ ] Verify bundled/custom preset deletion protections.

## Dependency and lockfile validation

- [ ] Confirm `Cargo.lock` is committed and consistent with the release tree.
- [ ] Confirm `apps/desktop/package-lock.json` is committed and consistent with the release tree.
- [ ] Run `cargo fetch --locked`.
- [ ] Run clean frontend `npm ci` from the committed lockfile.

## CI and security automation

- [ ] Required CI checks are green on the exact release commit.
- [ ] CodeQL is green.
- [ ] Dependabot configuration is healthy.
- [ ] Release-version verification is green.
- [ ] Release-lockfile verification is green.
- [ ] Packaging workflow is green for the supported targets.

## Release artifact review

- [ ] Review the final `CHANGELOG.md` section.
- [ ] Review `RELEASE_NOTES.md`.
- [ ] Confirm the release is marked as a pre-release/public preview.
- [ ] Confirm no draft-only or temporary release-generation workflow is accidentally included.
- [ ] Confirm the release commit is on `main`.

## Git tag

Create the tag only after every required check above is complete:

```bash
git checkout main
git pull origin main
git tag -a v0.3.0 -m "SortSmith v0.3.0"
git push origin v0.3.0
```

## GitHub release

Recommended release metadata:

- Tag: `v0.3.0`
- Title: `SortSmith v0.3.0 — First Public Preview`
- Target: `main`
- Pre-release: enabled
- Latest release: do not present it as a stable `1.0.0`-level release
- Notes: use `RELEASE_NOTES.md`

## After publication

- [ ] Verify the release page and tag.
- [ ] Verify the source archive references the expected commit.
- [ ] Verify packaged artifacts if published.
- [ ] Update the roadmap/status documentation if needed.
- [ ] Record the final release commit and tag in `what_changed.md`.
- [ ] Open the next development milestone/issue set.
