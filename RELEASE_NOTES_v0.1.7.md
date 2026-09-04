# SortSmith v0.1.7 — Patch Release

SortSmith v0.1.7 is a stable 0.1.x maintenance release that hardens cached preview planning so the performance path preserves the filesystem safety and collision guarantees of the primary preview engine.

## Highlights

- Cached organization preview now prunes followed symbolic links whose resolved targets are outside the selected root.
- Cached preview performs selected-root containment checks before reusing metadata for followed links.
- Cached preview reserves destinations across the current plan, preventing duplicate source filenames from converging on one destination.
- Added Unix regression coverage for external symlink traversal through the cached preview path.
- Added collision regression coverage for duplicate source filenames and retained cache invalidation/collision-recomputation coverage.

## Security

This release closes a consistency gap between the cached and primary preview paths. Execution and undo remain independently protected by their existing path-containment and no-overwrite safeguards.

## Compatibility

No intentional breaking public API change is introduced. The scan cache remains an in-memory optimization and does not change the persisted journal format.

## Release metadata

- Version: `0.1.7`
- Tag: `v0.1.7`
- Maintenance branch: `release/0.1.7`
- Release title: `SortSmith v0.1.7 — Patch Release`
- Pre-release: No
- Latest: No
- License: Apache-2.0

## Validation

Run the full Rust and desktop validation suite before publishing:

```bash
node scripts/verify-release-version.mjs v0.1.7
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
git diff --check

cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

Cross-platform CI should exercise Linux, Windows, and macOS builds. The Unix symlink regression requires a Unix-like runner.

## Status

The v0.1.7 maintenance fix and release documentation are present on both the dedicated release line and `main`. Local toolchain execution has not been claimed as passed in this environment; publish only after CI or an equivalent release-machine validation is green.
