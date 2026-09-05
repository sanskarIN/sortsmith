# SortSmith v0.1.8 — Maintenance Release

SortSmith v0.1.8 is a focused 0.1.x maintenance release following v0.1.7. It tightens release-boundary integrity, synchronizes application metadata, and corrects stale release documentation without backporting unrelated 0.3.x feature work.

## Highlights

### Version metadata integrity

The release version `0.1.8` is synchronized across:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

This keeps the Rust workspace, desktop frontend, and Tauri bundle aligned for packaging and release verification.

### Documentation accuracy

The maintenance handoff and README status wording have been updated so the 0.1.x release branch is clearly separated from the `main` branch's 0.3.x development line.

### Scope control

No 0.3.x feature-development work is backported into this maintenance release. The v0.1.8 boundary remains intentionally small and compatibility-focused.

## Compatibility

- No intentional breaking public API changes.
- Duplicate detection behavior from v0.1.7 is preserved.
- Filesystem safety and no-overwrite protections from v0.1.6 remain unchanged.
- The release is intended as a drop-in maintenance update for the 0.1.x line.

## Validation

The release must not be published until the branch passes the complete validation gates:

```bash
git checkout release/0.1.8
git pull --ff-only origin release/0.1.8

node scripts/verify-release-version.mjs v0.1.8

cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings

git diff --check
git status --short
```

Then validate the desktop application:

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

Production installers should be smoke-tested on supported Windows, macOS, and Linux environments before publication.

## Release Metadata

- **Version:** `0.1.8`
- **Tag:** `v0.1.8`
- **Target branch:** `release/0.1.8`
- **Release title:** `SortSmith v0.1.8 — Maintenance Release`
- **Release type:** Stable maintenance release
- **Pre-release:** No
- **Latest release:** No
- **License:** Apache-2.0

## Contributor Identity

Project commit identity:

`Sanskar <sanskarin@outlook.in>`

## Release Status

The `release/0.1.8` branch and release documentation are prepared. Repository-side changes have not been represented as locally tested here. GitHub Actions must complete successfully, and the required cross-platform installer checks must pass before the `v0.1.8` tag is published.
