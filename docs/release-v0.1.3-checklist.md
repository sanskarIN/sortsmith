# SortSmith v0.1.3 Release Checklist

## Release identity

- [ ] Branch: `release/0.1.3`
- [ ] Version: `0.1.3`
- [ ] Tag: `v0.1.3`
- [ ] Release title: `SortSmith v0.1.3 — Patch Release`
- [ ] Target: `release/0.1.3`
- [ ] Release type: normal patch release unless explicitly marked prerelease

## Source and metadata gate

- [ ] `Cargo.toml` workspace version is `0.1.3`
- [ ] `apps/desktop/package.json` version is `0.1.3`
- [ ] `apps/desktop/src-tauri/tauri.conf.json` version is `0.1.3`
- [ ] `node scripts/verify-release-version.mjs v0.1.3` passes
- [ ] No unintended changes are present in the release branch
- [ ] `git diff --check` passes

## Core quality gate

Run:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

- [ ] Formatting passes
- [ ] Workspace check passes
- [ ] All Rust tests pass
- [ ] Clippy passes with warnings denied

## Desktop quality gate

From `apps/desktop`:

```bash
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

- [ ] Dependencies install successfully
- [ ] TypeScript typecheck passes
- [ ] Frontend tests pass
- [ ] Frontend production build passes
- [ ] Tauri packaging succeeds for the locally supported target

## Functional regression gate

Verify manually or with automated tests that:

- [ ] A normal single-folder organization produces the expected preview.
- [ ] Recursive organization with identical filenames in different source directories produces distinct destinations.
- [ ] The preview displays the collision-safe suffix before execution.
- [ ] Applying the preview moves files to the displayed destinations.
- [ ] Undo still restores moved files using the operation journal.
- [ ] Journal replacement still loads the newest complete snapshot.
- [ ] Forged out-of-root journals remain blocked.
- [ ] Unicode rule-length validation remains correct.
- [ ] Filename portability and traversal protections remain intact.

## Cross-platform release gate

Push the tag only after the local gate passes:

```bash
git checkout release/0.1.3
git pull origin release/0.1.3
node scripts/verify-release-version.mjs v0.1.3
git diff --check
git tag -a v0.1.3 -m "SortSmith v0.1.3"
git push origin v0.1.3
```

Then inspect the tag-driven GitHub Actions release workflow.

- [ ] Ubuntu build succeeds
- [ ] Windows build succeeds
- [ ] macOS build succeeds
- [ ] Generated artifacts are present
- [ ] Artifact names and versions are correct
- [ ] No unexpected build warnings/errors remain

## GitHub Release publication

Use:

- **Tag:** `v0.1.3`
- **Target:** `release/0.1.3`
- **Title:** `SortSmith v0.1.3 — Patch Release`
- **Description:** contents of `RELEASE_NOTES_v0.1.3.md`
- **Prerelease:** No for a normal patch release
- **Latest:** choose according to the repository's release chronology; do not mark this historical maintenance release as latest if a newer full release is already intended to be the current latest release

The repository workflow is configured to create a draft release from the tag. Review the generated artifacts before publishing.

## Post-release verification

- [ ] Published release page displays `v0.1.3`
- [ ] Release title is correct
- [ ] Release notes are complete
- [ ] Source archive is associated with the correct tag
- [ ] Platform artifacts are downloadable
- [ ] Windows installer tested on a clean machine
- [ ] macOS artifact tested on a clean machine
- [ ] Linux artifact tested on a clean machine
- [ ] Install, launch, preview, apply, and undo smoke tests pass
- [ ] `what_changed.md` is updated with the final publication state
- [ ] Release branch remains separate from modern `main`

## Known limitations

Signing and notarization must not be described as complete unless platform signing credentials and actual signed artifacts have been configured and verified.

The 0.1.x maintenance branch should not be merged backward into the modern feature-development line solely to advance the patch version.
