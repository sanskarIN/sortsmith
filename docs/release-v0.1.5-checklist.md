# SortSmith v0.1.5 Release Checklist

## Scope

Stable patch release for the `release/0.1.x` maintenance line. The primary change is earlier pruning of symbolic-link directories that resolve outside the selected root during recursive preview.

## Repository gate

- [ ] Release branch is `release/0.1.5`.
- [ ] Root workspace version is `0.1.5`.
- [ ] Desktop package version is `0.1.5`.
- [ ] Tauri application version is `0.1.5`.
- [ ] `node scripts/verify-release-version.mjs v0.1.5` passes.
- [ ] `git diff --check` passes.
- [ ] Working tree is clean before tagging.

## Core validation

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace`
- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Desktop validation

From `apps/desktop`:

- [ ] `npm install --no-audit --no-fund`
- [ ] `npm run typecheck`
- [ ] `npm test`
- [ ] `npm run build`
- [ ] `npm run tauri build`

## Symlink safety gate

- [ ] Unix file-symlink regression passes.
- [ ] Unix external symlink-directory regression passes.
- [ ] Recursive preview with `follow_links` disabled remains unchanged.
- [ ] Recursive preview with an internal symlink target remains usable.
- [ ] External symlink directories are pruned before recursive descent.
- [ ] External nested files are not counted as scanned through a pruned external symlink directory.
- [ ] Execution-time canonical path containment remains enabled.
- [ ] Forged preview and forged journal containment regressions remain green.

## Cross-platform release gate

- [ ] Linux artifact builds successfully.
- [ ] Windows artifact builds successfully.
- [ ] macOS artifact builds successfully.
- [ ] Clean-machine installation/launch smoke test completed for each available platform.
- [ ] Existing configuration behavior remains compatible.
- [ ] Journal creation, execution, collision handling, and undo behavior remain compatible.

## Tagging

Only create the stable tag after every available validation gate above is green:

```bash
git checkout release/0.1.5
git pull origin release/0.1.5
node scripts/verify-release-version.mjs v0.1.5
git diff --check
git status --short
git tag -a v0.1.5 -m "SortSmith v0.1.5"
git push origin v0.1.5
```

## GitHub release metadata

- **Tag:** `v0.1.5`
- **Target:** `release/0.1.5`
- **Title:** `SortSmith v0.1.5 — Patch Release`
- **Pre-release:** disabled
- **Latest release:** disabled
- **Body:** `RELEASE_NOTES_v0.1.5.md`

The repository is already on a later `0.3.x` feature line, so this historical `0.1.x` maintenance release should not be selected as the repository's latest release.

The tag-triggered workflow is expected to create a draft release. Review all generated artifacts before publishing that draft as the stable v0.1.5 release.

## Publication verification

After publication:

- [ ] GitHub release shows `v0.1.5`.
- [ ] Release is not marked as a pre-release.
- [ ] Release is not marked as the latest release.
- [ ] Published assets correspond to the `v0.1.5` tag.
- [ ] Release target is the `release/0.1.5` maintenance line.
- [ ] Release notes match `RELEASE_NOTES_v0.1.5.md`.

## Known limitation

The repository integration available for this project can update files and branches and inspect GitHub state, but it does not expose a release-publication action. The release owner must therefore create/push the tag and publish the generated GitHub draft after the local and cross-platform validation gates pass.