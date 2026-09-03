# SortSmith — Work Handoff

## Current active workstream: v0.1.4

- Release branch: `release/0.1.4`
- Base: completed `release/0.1.3` maintenance line
- Target patch version: `0.1.4`
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.4 implementation completed

### Version synchronization

The following release metadata now reports `0.1.4` on `release/0.1.4`:

- root `Cargo.toml` workspace package;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

The existing `scripts/verify-release-version.mjs` is intended to verify that a release tag such as `v0.1.4` matches all three metadata sources.

### Safer recursive preview behavior

`crates/sortsmith-core/src/engine.rs` now canonicalizes the selected root once for preview planning.

When `ScanOptions.follow_links` is enabled, each discovered file is resolved before it is planned. If the resolved target is outside the selected root, SortSmith skips that entry, increments the ignored-file count, and adds a recoverable error explaining that an external symbolic-link target was skipped.

This closes an unsafe planning gap: previously, a recursive preview could discover a file through a symbolic link outside the selected root and only the later execution-time validation would reject the operation. The preview now communicates the safety boundary earlier and avoids displaying an operation that cannot safely execute.

Filesystem canonicalization resolves symbolic links and produces an absolute path, making it appropriate for this boundary check.

### Defense in depth retained

Execution still performs its own canonical source and destination-parent containment validation before any file mutation. The v0.1.4 preview change therefore does not replace the execution boundary; it adds an earlier safety layer.

### Regression coverage

Added a Unix-specific core regression test that:

- creates a temporary selected root and an external directory;
- places a text file outside the selected root;
- creates a file symlink inside the root pointing to that external file;
- enables recursive link following;
- verifies no organization operation is planned;
- verifies the file is counted as ignored;
- verifies a recoverable symbolic-link safety message is returned.

The existing v0.1.1–v0.1.3 coverage remains in the maintenance line, including Unicode rule boundaries, journal replacement, journal-root containment, traversal rejection, portable filename validation, and preview-wide collision reservation.

## Release documentation completed

- `CHANGELOG.md` now has a dedicated `0.1.4` entry dated 2026-09-03.
- `RELEASE_NOTES_v0.1.4.md` contains publication-ready release notes and upgrade information.
- `docs/release-v0.1.4-checklist.md` contains the metadata gate, Rust and desktop checks, security regressions, cross-platform release gate, GitHub release fields, and tag commands.
- This `what_changed.md` records the complete v0.1.4 continuation.

## v0.1.4 commits

1. `6091536bd689dd632b18a65c488041c42f2975be` — `fix(core): reject external symlink targets during preview`
2. `7fbdc0f12f4092615dc7eb13c463346bc75851d2` — `release: bump workspace version to 0.1.4`
3. `19aca8ba4c4a654c0ce8213df6d8b91282393395` — `release(frontend): bump desktop version to 0.1.4`
4. `a5e7be7ada029ce4ced6fd9dd2565ab78a0fdc26` — `release(tauri): bump application version to 0.1.4`
5. `b9ad93ea978c56d1196eb45fabda0bdfe1d14260` — `docs(changelog): prepare v0.1.4 symlink safety patch`
6. `fae4d31ad0d4922f6c3101dde2cad28f3f0ddd1a` — `docs(changelog): remove generated citation markup`
7. `aa9853656d52e186fd3fca5d777dd71bdba44e90` — `docs(release): add v0.1.4 release notes`
8. `9290fe0409b6fb7105f0908823ab27addbd13dc5` — `docs(release): add v0.1.4 publication checklist`
9. Current handoff update — records this continuation and release state.

The commits are separated by responsibility and contain real code, release metadata, tests, changelog, release-note, checklist, or handoff changes. No empty commits were added merely to inflate history.

## Historical v0.1.x context

The `0.1.x` maintenance line is intentionally separate from the later feature line. `release/0.1.1` was created from the recovered final `0.1.0` source boundary immediately before the later `0.2.0` version bump.

The existing `v0.1.0` tag points at newer repository history and must not be rewritten casually. The maintenance branches preserve the historical source lineage without merging backward into modern `main`.

The preceding releases added the following maintenance fixes:

- v0.1.1: Unicode-character-based rule-value and filename-regex limits.
- v0.1.2: safer existing-journal replacement and core undo-path containment.
- v0.1.3: preview-wide destination reservation for collision-safe planning.
- v0.1.4: preview-time rejection of files resolved outside the selected root when link following is enabled.

## Release automation reviewed

The release workflow is tag driven. It triggers for `v*` tags, verifies release metadata, builds on its configured Linux, Windows, and macOS runners, and uses the Tauri action to prepare a draft GitHub release.

The workflow currently installs frontend dependencies with `npm install`. `npm ci` should only replace this after a real npm lockfile has been generated and committed from a trusted networked environment.

The release workflow also verifies the tag version using `scripts/verify-release-version.mjs`.

## Verification status

Repository preparation is complete, but this connected environment has not executed the Rust or frontend toolchains locally. Therefore `v0.1.4` must not yet be described as fully tested or published.

Required local verification:

```bash
node scripts/verify-release-version.mjs v0.1.4
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
git checkout release/0.1.4
git pull origin release/0.1.4
node scripts/verify-release-version.mjs v0.1.4
git diff --check
git tag -a v0.1.4 -m "SortSmith v0.1.4"
git push origin v0.1.4
```

The tag push should trigger `.github/workflows/release.yml`. Review the generated platform artifacts before publishing the draft release.

Recommended GitHub release title:

`SortSmith v0.1.4 — Patch Release`

Recommended target:

`release/0.1.4`

Use `RELEASE_NOTES_v0.1.4.md` as the release-body source.

## Known release blockers

- Local Rust format/check/test/Clippy results have not been executed in this environment.
- Local frontend typecheck/test/build results have not been executed in this environment.
- Cross-platform Tauri artifacts for the `v0.1.4` tag have not yet been observed.
- Clean-machine installer smoke tests remain required.
- Signing/notarization remains unverified.
- Dependency lockfiles remain a separate reproducibility task unless generated and committed in the maintenance line.
- The available repository integration can prepare and push branch/file changes but does not provide a release-publication action; the tag and GitHub draft publication therefore remain a release-owner step.

## After v0.1.4

Do not merge the `0.1.x` maintenance branch into modern `main` merely to advance the patch version. After the patch release is verified, return to the modern feature-development line for the next milestone.

The next feature release should be planned from the current `main`/development state rather than backporting unrelated `0.2.x` or `0.3.x` functionality into `0.1.x`.

## Continuation rule

Before publishing `v0.1.4`, execute the complete local release gate, push the exact tag, inspect all platform workflow results, review artifacts, and perform clean-machine smoke tests. Do not claim publication until those gates have actually passed.
