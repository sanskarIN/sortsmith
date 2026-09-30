# SortSmith v0.1.9 — Maintenance Release

SortSmith v0.1.9 is a focused 0.1.x maintenance release that prepares the next patch boundary from the completed v0.1.8 maintenance line.

## Highlights

### Version integrity

The Rust workspace, desktop frontend package, and Tauri application metadata are synchronized at `0.1.9`.

This keeps release-version verification deterministic across the application layers.

### Release-scope discipline

v0.1.9 remains based on the dedicated `release/0.1.8` maintenance line. Newer 0.3.x development work from `main` is not backported into this patch release.

### Existing safety guarantees retained

The v0.1.8 baseline retains the established 0.1.x filesystem safety model, including:

- External symbolic-link containment during recursive scanning.
- Collision-safe no-overwrite file moves.
- Durable journal checkpoints.
- Safe undo boundaries.
- Portable filename validation.
- Deterministic duplicate-detection output.

No intentional breaking public API change is introduced by the v0.1.9 release metadata update.

## Validation gate

The release is publishable only after the dedicated `release/0.1.9` branch passes the repository CI checks and the application packaging smoke tests.

Required repository checks:

```bash
node scripts/verify-release-version.mjs v0.1.9
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

Required desktop checks:

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

Supported-platform installer smoke tests should be completed on Windows, macOS, and Linux before a production publication.

## Release metadata

- **Version:** `0.1.9`
- **Tag:** `v0.1.9`
- **Target branch:** `release/0.1.9`
- **Release title:** `SortSmith v0.1.9 — Maintenance Release`
- **Release type:** Stable patch release
- **Pre-release:** No
- **License:** Apache-2.0
- **Commit identity:** `Sanskar <sanskarin@outlook.in>`

## Publication gate

Do not publish or mark the release as stable until all automated validation gates are green and the supported desktop installers have been smoke-tested.
