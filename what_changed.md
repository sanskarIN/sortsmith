# SortSmith — Work Handoff

## Current active workstream: v0.1.8 maintenance release

- Release branch: `release/0.1.8`
- Base: `release/0.1.7`
- Target version: `0.1.8`
- Default branch: `main`
- Main version line: `0.3.0`
- Repository: `https://github.com/sanskarIN/sortsmith`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.8 implementation and release-engineering audit

### Version synchronization

The v0.1.8 maintenance branch synchronizes the release version across:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

This keeps the Rust workspace, desktop frontend, and Tauri bundle aligned for the release-version verification gate and packaging metadata.

### Documentation accuracy

Updated `README.md` so the project no longer describes the 0.1.x line as merely a 0.1.0 implementation baseline. It now identifies v0.1.8 preparation and explicitly separates the maintenance branch from the 0.3.x `main` development line.

Added `RELEASE_NOTES_v0.1.8.md` with the release scope, compatibility statement, validation commands, metadata, and publication gate.

Updated `CHANGELOG.md` with the v0.1.8 maintenance entry.

### Scope control

The v0.1.8 release intentionally remains a small 0.1.x maintenance release. No 0.3.x scan-cache, automation, or other feature-development work is backported into this branch.

## v0.1.7 baseline preserved

The preceding v0.1.7 branch introduced deterministic ordering for files inside duplicate groups. That implementation remains unchanged in v0.1.8.

Existing 0.1.6 filesystem safety hardening remains unchanged as well, including external symbolic-link containment, collision-safe no-overwrite moves, durable journal checkpoints, and safer undo behavior.

## v0.1.8 commit sequence

The release branch currently contains focused commits for:

1. `chore(release): bump workspace version to 0.1.8`
2. `chore(desktop): sync package version to 0.1.8`
3. `chore(tauri): sync application version to 0.1.8`
4. `docs(changelog): add v0.1.8 maintenance release entry`
5. `docs(release): add v0.1.8 release notes`
6. `docs(readme): clarify v0.1.8 maintenance release status`
7. This handoff update.

All project commits use the requested identity `Sanskar <sanskarin@outlook.in>` through the connected GitHub integration.

## Verification status

Repository-side preparation is complete for the current v0.1.8 scope. The GitHub Actions CI run triggered by the latest branch commit is currently the authoritative automated validation path.

Local Rust, Node.js, Tauri, installer, and cross-platform execution has not been claimed as passed in this environment. No successful test result is fabricated.

Before publication, the following must pass from `release/0.1.8`:

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

cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

The production installers should additionally be smoke-tested on supported Windows, macOS, and Linux environments before publication.

## v0.1.8 publication procedure

After every validation gate is green:

```bash
git checkout release/0.1.8
git pull --ff-only origin release/0.1.8
node scripts/verify-release-version.mjs v0.1.8
git diff --check
git status --short
git tag -a v0.1.8 -m "SortSmith v0.1.8"
git push origin v0.1.8
```

The repository's tag-triggered workflow should then prepare the release artifacts according to the configured release workflow. Review the generated draft and artifacts before publishing.

Recommended release metadata:

- Tag: `v0.1.8`
- Target: `release/0.1.8`
- Title: `SortSmith v0.1.8 — Maintenance Release`
- Pre-release: disabled
- Latest: disabled
- Body: `RELEASE_NOTES_v0.1.8.md`

## Release status

As of this handoff, the `release/0.1.8` branch is prepared but the `v0.1.8` Git tag has not been published. Publication remains gated on green CI and the required installer smoke tests.
