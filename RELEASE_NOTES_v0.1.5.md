# SortSmith v0.1.5 — Patch Release

## Overview

SortSmith v0.1.5 is a maintenance and security-hardening release for the `0.1.x` line. It strengthens recursive preview behavior when symbolic-link following is enabled and prevents external symlink directories from being traversed outside the selected organization root.

## Highlights

### Safer symlink-directory traversal

When `follow_links` is enabled, preview traversal checks symbolic-link entries before WalkDir descends into them. If a symlink resolves outside the selected organization root, the entry is pruned at the traversal boundary.

This extends the v0.1.4 safety work:

- v0.1.4 prevents external file symlink targets from becoming planned operations.
- v0.1.5 prevents external symlink directories from being traversed in the first place.
- Execution-time canonical containment checks remain in place as defense-in-depth.

### Security impact

The change reduces unintended inspection of external directory trees during recursive scans and avoids discovering nested external files only to reject them later during planning.

### Regression coverage

Unix-specific regression coverage verifies an external directory linked from inside the selected root. The test confirms that recursive preview creates no organization operation for the external tree and does not count the external nested file as scanned.

## Version metadata

All release metadata is synchronized to `0.1.5`:

- Rust workspace package
- Desktop `package.json`
- Tauri application configuration

## Compatibility

This is a patch release on the existing `0.1.x` maintenance line. No unrelated `0.2.x` or `0.3.x` feature work is backported.

Existing rule, journal, collision-safety, undo, and desktop behavior remains covered by the preceding maintenance-line regression suite.

## Validation

Repository-side preparation is complete. Final release validation must be performed in an environment with the Rust, Node.js, and Tauri toolchains available.

Recommended release gate:

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

For a complete stable release, also review the Linux, Windows, and macOS artifacts and perform clean-machine install/launch smoke tests where available.

## GitHub release

- **Tag:** `v0.1.5`
- **Target:** `release/0.1.5`
- **Title:** `SortSmith v0.1.5 — Patch Release`
- **Pre-release:** No
- **Latest release:** No
- **Release body:** This document

Because the repository is currently on a later `0.3.x` feature line, v0.1.5 should remain a maintenance-line release rather than being marked as the repository's latest release.

## Upgrade guidance

Users upgrading from v0.1.4 should receive the stronger recursive symlink-directory boundary automatically. If `follow_links` is enabled, symlink directories that resolve outside the selected root will no longer be traversed during preview.

Users should continue reviewing preview results before applying organization operations, particularly when working with symbolic links.

## Acknowledgement

Thank you for testing and improving SortSmith's filesystem safety and reversibility.