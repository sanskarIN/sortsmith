# SortSmith — Work Handoff

## Current active workstream: v0.1.9 maintenance release

- Release branch: `release/0.1.9`
- Base: `release/0.1.8`
- Target version: `0.1.9`
- Default branch: `main`
- Main version line: `0.3.x`
- Repository: `https://github.com/sanskarIN/sortsmith`
- License: Apache-2.0
- Commit identity: `Sanskar <sanskarin@outlook.in>`

## v0.1.9 scope

v0.1.9 is intentionally a focused 0.1.x maintenance release. It is based on `release/0.1.8` and does not backport newer 0.3.x feature-development work from `main`.

The release synchronizes the application version across:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

The release also adds a dedicated release note and changelog entry.

## Safety baseline retained

The v0.1.8 baseline remains intact, including:

- External symbolic-link containment during recursive scans.
- Collision-safe no-overwrite moves.
- Durable journal checkpoints.
- Safe undo path validation.
- Portable filename validation.
- Deterministic duplicate-detection output.

No intentional breaking public API change is introduced by the v0.1.9 maintenance work.

## v0.1.9 commit sequence

The release branch currently contains focused commits for:

1. `chore(release): bump workspace version to 0.1.9`
2. `chore(release): synchronize desktop package version to 0.1.9`
3. `chore(release): synchronize tauri version to 0.1.9`
4. `docs(release): add v0.1.9 release notes`
5. `docs(changelog): restore complete changelog with v0.1.9 entry`

All project commits use `Sanskar <sanskarin@outlook.in>`.

## Verification gate

The v0.1.9 tag must not be published until the dedicated branch passes:

```bash
node scripts/verify-release-version.mjs v0.1.9
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

The supported desktop installers should additionally be smoke-tested on Windows, macOS, and Linux before production publication.

## Publication procedure

After every automated validation gate is green and installer smoke tests are complete:

```bash
git checkout release/0.1.9
git pull --ff-only origin release/0.1.9
node scripts/verify-release-version.mjs v0.1.9
git diff --check
git status --short
git tag -a v0.1.9 -m "SortSmith v0.1.9"
git push origin v0.1.9
```

Recommended metadata:

- Tag: `v0.1.9`
- Target: `release/0.1.9`
- Title: `SortSmith v0.1.9 — Maintenance Release`
- Pre-release: disabled
- Latest: disabled
- Body: `RELEASE_NOTES_v0.1.9.md`

## Current status

The v0.1.9 release branch is prepared, but it is not yet declared production-ready. Automated CI validation is still the release gate. No green result is being fabricated, and no tag should be published until the complete validation path succeeds.
