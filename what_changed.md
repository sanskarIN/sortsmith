# SortSmith — Work Handoff

## Current active workstream: v0.1.3

- Release branch: `release/0.1.3`
- Base: completed `release/0.1.2` maintenance line
- Target patch version: `0.1.3`
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.3 implementation completed

### Version synchronization

The following release metadata now reports `0.1.3` on `release/0.1.3`:

- root `Cargo.toml` workspace package;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

The existing `scripts/verify-release-version.mjs` verifies that a release tag such as `v0.1.3` matches all three metadata sources.

### Collision-safe preview planning

`crates/sortsmith-core/src/engine.rs` now keeps an in-memory set of destinations already reserved by earlier operations in the same preview.

Preview planning passes that set to `collision_safe_path_with_reserved`, so two source files that converge on the same destination filename receive different planned destinations before execution begins.

This is particularly important for recursive organization. For example, `first/note.txt` and `second/note.txt` can both match a rule that moves files into one `Text` folder. The preview now plans `Text/note.txt` and `Text/note (1).txt` rather than assigning the same destination to both operations.

### Regression coverage

Added a core regression test that:

- creates identical filenames in separate source directories;
- enables recursive scanning;
- generates an organization preview;
- verifies two operations are planned;
- verifies the destinations are unique;
- verifies the expected collision suffix is used.

The earlier v0.1.1 and v0.1.2 regression suites remain in the maintenance line, covering Unicode rule boundaries, journal replacement, traversal rejection, portable filenames, and forged-journal protection.

## Release documentation completed

- `CHANGELOG.md` now has a dedicated `0.1.3` entry dated 2026-09-03.
- `RELEASE_NOTES_v0.1.3.md` contains publication-ready release notes and verification commands.
- `docs/release-v0.1.3-checklist.md` contains the release gate, tag instructions, GitHub release fields, platform checks, and post-release verification.
- This `what_changed.md` records the complete v0.1.3 continuation.

## v0.1.3 commits

1. `df8129bc81f350277cbca7830abaf950cc0caa95` — `release: bump workspace version to 0.1.3`
2. `39aa5b216b9a92494cef7cc1e679ff318d2e5ccc` — `release(frontend): bump desktop version to 0.1.3`
3. `dcb883f3e01b4eee8db8fbfed3287588841faab9` — `release(tauri): bump application version to 0.1.3`
4. `d1a290d1d187635039e02d06703c26dc1c004551` — `fix(core): reserve planned destinations during preview`
5. `e443f92bcaca39035e3fd3a4a22d80e170025ed1` — `test(core): cover duplicate preview destinations`
6. `488763054e258468ebe167f703596b87456bb888` — `docs(changelog): prepare v0.1.3 collision safety patch`
7. `c6f3401c009a3096a1634c26948e48da33592d3d` — `docs(release): add v0.1.3 release notes`
8. `0f5451d1a2f7f43ee9ae287d4840a6e9c2368373` — `docs(release): add v0.1.3 publication checklist`
9. Current handoff update — records this continuation and release state.

The commits are separated by responsibility and contain real release or code/documentation changes; no empty commits were added merely to inflate history.

## Historical v0.1.x context

The `0.1.x` maintenance line is intentionally separate from the later feature line. `release/0.1.1` was created from the recovered final `0.1.0` source boundary immediately before the later `0.2.0` version bump.

The existing `v0.1.0` tag points at newer repository history and must not be rewritten casually. The maintenance branches preserve the historical source lineage without merging backward into modern `main`.

The preceding releases added the following maintenance fixes:

- v0.1.1: Unicode-character-based rule-value and filename-regex limits.
- v0.1.2: safer existing-journal replacement and core undo-path containment.
- v0.1.3: preview-wide destination reservation for collision-safe planning.

## Release automation reviewed

The release workflow is tag driven. It triggers for `v*` tags, checks release metadata, builds on Ubuntu, Windows, and macOS, and uses the Tauri action to prepare a draft GitHub release.

The workflow currently installs frontend dependencies with `npm install`. `npm ci` should only replace this after a real npm lockfile has been generated and committed from a trusted networked environment.

The release workflow also verifies the tag version using `scripts/verify-release-version.mjs`.

## Verification status

Repository preparation is complete, but this connected environment has not executed the Rust or frontend toolchains locally. Therefore `v0.1.3` must not yet be described as fully tested or published.

Required local verification:

```bash
node scripts/verify-release-version.mjs v0.1.3
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

## Release procedure

After all local checks pass:

```bash
git checkout release/0.1.3
git pull origin release/0.1.3
node scripts/verify-release-version.mjs v0.1.3
git diff --check
git tag -a v0.1.3 -m "SortSmith v0.1.3"
git push origin v0.1.3
```

The tag push should trigger `.github/workflows/release.yml`. Review the generated platform artifacts before publishing the draft release.

Recommended GitHub release title:

`SortSmith v0.1.3 — Patch Release`

Recommended target:

`release/0.1.3`

Use `RELEASE_NOTES_v0.1.3.md` as the release-body source.

## Known release blockers

- Local Rust format/check/test/Clippy results have not been executed in this environment.
- Local frontend typecheck/test/build results have not been executed in this environment.
- Cross-platform Tauri artifacts for the `v0.1.3` tag have not yet been observed.
- Clean-machine installer smoke tests remain required.
- Signing/notarization remains unverified.
- Dependency lockfiles remain a separate reproducibility task unless generated and committed in the maintenance line.

## After v0.1.3

Do not merge the `0.1.x` maintenance branch into modern `main` merely to advance the patch version. After the patch release is verified, return to the modern feature-development line for the next milestone.

The next feature release should be planned from the current `main`/development state rather than backporting unrelated `0.2.x` or `0.3.x` functionality into `0.1.x`.

## Continuation rule

Before publishing `v0.1.3`, execute the complete local release gate, push the exact tag, inspect all platform workflow results, review artifacts, and perform clean-machine smoke tests. Do not claim publication until those gates have actually passed.
