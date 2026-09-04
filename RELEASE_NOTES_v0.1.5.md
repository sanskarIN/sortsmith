# SortSmith v0.1.5 — Patch Pre-release

> **Pre-release:** This build is intended for validation before a stable maintenance release.

## Highlights

SortSmith v0.1.5 hardens recursive preview traversal when symbolic-link following is enabled.

### Safer symlink-directory traversal

When `follow_links` is enabled, preview traversal now checks symbolic-link entries before WalkDir descends into them. If a symlink resolves outside the selected organization root, that entry is pruned at the traversal boundary.

This is a stricter continuation of the v0.1.4 safety work:

- v0.1.4 prevented external file symlink targets from becoming planned operations.
- v0.1.5 prevents external symlink directories from being traversed in the first place.
- Execution-time canonical containment checks remain unchanged as defense-in-depth.

### Security impact

The change reduces the chance of unintentionally inspecting an external directory tree when recursive link following is enabled. It also avoids discovering nested external files only to reject them later during planning.

### Regression coverage

Added Unix-specific coverage for an external directory linked from inside the selected root. The test verifies that recursive preview produces no organization operations and does not count the external nested file as scanned.

## Version metadata

All release metadata is synchronized to `0.1.5`:

- Rust workspace package
- Desktop `package.json`
- Tauri application configuration

## Validation status

Repository-side release preparation is complete. Local Rust, frontend, Tauri, and cross-platform installer validation still needs to be run by the release owner before this pre-release is considered ready for broad use.

Recommended checks:

```bash
node scripts/verify-release-version.mjs v0.1.5
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings

git diff --check
git status --short

cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

## GitHub release

- **Tag:** `v0.1.5`
- **Target:** `release/0.1.5`
- **Title:** `SortSmith v0.1.5 — Patch Pre-release`
- **Pre-release:** Yes
- **Latest release:** No

Review all platform artifacts and perform clean-machine smoke tests before treating this build as stable.
