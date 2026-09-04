# SortSmith v0.1.7 — Patch Release

SortSmith v0.1.7 is a stable 0.1.x maintenance release that makes duplicate-detection output deterministic without changing duplicate matching or deletion behavior.

## Highlights

- Duplicate-group member paths are sorted before results are returned.
- Stable member ordering prevents filesystem traversal and parallel hashing order from leaking into API/UI results.
- Added regression coverage that verifies equal-content duplicate files are returned in lexical path order.
- Existing content-equality, hidden-directory, and external-symlink traversal protections remain unchanged.

## Compatibility

No intentional breaking public API change is introduced. Duplicate detection continues to use BLAKE3, requires equal file size and content for grouping, and never deletes files automatically.

## Release metadata

- Version: `0.1.7`
- Tag: `v0.1.7`
- Maintenance branch: `release/0.1.7`
- Release title: `SortSmith v0.1.7 — Patch Release`
- Pre-release: No
- Latest: No
- License: Apache-2.0

## Validation

Run the complete release suite before publishing:

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

Cross-platform CI should exercise Linux, Windows, and macOS builds before publishing production installers.

## Status

The v0.1.7 maintenance fix and release documentation are present on the dedicated release line and mirrored on `main`. Local toolchain execution has not been claimed as passed in this environment; publish only after CI or equivalent release-machine validation is green.
