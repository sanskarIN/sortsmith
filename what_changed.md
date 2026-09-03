# SortSmith — Work Handoff

## Current active workstream: v0.1.2

- Release branch: `release/0.1.2`
- Base: completed `release/0.1.1` maintenance line
- Target patch version: `0.1.2`
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## v0.1.2 implementation completed

### Version synchronization

The following release metadata now reports `0.1.2` on `release/0.1.2`:

- root `Cargo.toml` workspace package;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

The existing `scripts/verify-release-version.mjs` verifies that a release tag such as `v0.1.2` matches all three metadata sources.

### Journal durability fix

`crates/sortsmith-core/src/journal.rs` now handles an existing target journal explicitly when replacing the flushed temporary snapshot.

The normal path still writes to a temporary file, flushes it, calls `sync_all`, and then renames it into place. If the destination already exists on a platform where rename refuses to overwrite it, the implementation removes the old target and retries the rename.

A regression test now saves the same journal twice and confirms that the newest complete snapshot is loaded successfully.

### Core undo security hardening

`crates/sortsmith-core/src/engine.rs` now validates the recorded `from` and `to` paths in an undo journal against the journal's recorded root before beginning any undo mutation.

This is defense in depth. The desktop/Tauri boundary already performs journal-path validation, while the core API now protects direct Rust callers as well.

The validation rejects:

- absolute paths outside the recorded root;
- parent traversal components;
- existing files whose canonical paths escape the root;
- existing parent directories that resolve through symlinks outside the root.

A regression test creates a forged journal pointing at an external file and verifies that `undo_journal` rejects it without moving the external file.

## Release documentation completed

- `CHANGELOG.md` now has a dedicated `0.1.2` entry dated 2026-09-03.
- `RELEASE_NOTES_v0.1.2.md` contains publication-ready release notes and verification commands.
- `docs/release-v0.1.2-checklist.md` contains the release gate, version checks, tag fields, platform smoke tests, and post-release tasks.
- This `what_changed.md` records the complete v0.1.2 continuation.

## v0.1.2 commits

1. `0842a5a984460a3bc34ef86210e9730a636b127b` — `release: bump workspace version to 0.1.2`
2. `ad4ebae1fdd46034796070140aa5c4b7a4a3178b` — `release(frontend): bump desktop version to 0.1.2`
3. `a951663da1ea8a518802ffec815896a257039322` — `release(tauri): bump application version to 0.1.2`
4. `085e47b0567eac04ab006a36bdc9cd15aa401cab` — `fix(core): replace existing journal snapshots safely`
5. `5bea81dbaf2e786ac8d799c853d75e97ba9e24c8` — `fix(core): validate undo journal paths before mutation`
6. `43d7494b41de020d8a57ebf33e93b9884693363a` — `docs(changelog): prepare v0.1.2 patch release`
7. `cc1380a7842cb68f11df54ca0b31f7be6995b556` — `docs(release): add v0.1.2 release notes`
8. `57613b54f2a99f9a0ad0a03b0b3ec3e0d421f79f` — `docs(release): add v0.1.2 publication checklist`

The commits are intentionally separated by responsibility and contain real changes; no empty or artificial commits were added solely to inflate the history.

## Historical v0.1.x context

The `0.1.x` maintenance line is intentionally separate from the later feature line. `release/0.1.1` was created from the recovered final `0.1.0` source boundary immediately before the later `0.2.0` version bump.

The existing `v0.1.0` tag points at newer repository history and must not be rewritten casually. The maintenance branches therefore preserve the historical source lineage without merging backward into modern `main`.

The preceding `v0.1.1` patch corrected Unicode rule-value and filename-regex limits so the documented limits count Unicode characters rather than UTF-8 bytes.

## Release automation reviewed

The release workflow is tag driven. It triggers for `v*` tags, checks release metadata, builds on Ubuntu, Windows, and macOS, and uses the Tauri action to prepare a draft GitHub release.

The release workflow currently installs frontend dependencies with `npm install`. `npm ci` should only replace this after a real npm lockfile has been generated and committed from a trusted networked environment.

The repository also does not claim signing/notarization until platform credentials and actual platform builds have been configured and verified.

## Verification status

Repository preparation is complete, but this connected environment has not executed the Rust or frontend toolchains locally. Therefore `v0.1.2` must not yet be described as fully tested or published.

Required local verification:

```bash
node scripts/verify-release-version.mjs v0.1.2
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
```

For packaging, also run:

```bash
npm run tauri build
```

The exact `v0.1.2` tag should then be allowed to exercise the tag-driven cross-platform release workflow.

## Release procedure

After all local checks pass:

```bash
git checkout release/0.1.2
git pull origin release/0.1.2
node scripts/verify-release-version.mjs v0.1.2
git diff --check
git tag -a v0.1.2 -m "SortSmith v0.1.2"
git push origin v0.1.2
```

Recommended GitHub release title:

`SortSmith v0.1.2 — Patch Release`

Recommended target:

`release/0.1.2`

Use `RELEASE_NOTES_v0.1.2.md` as the release-body source. The automated workflow is configured to create a draft release; review generated artifacts before publishing.

## Known release blockers

- Local Rust format/check/test/Clippy results have not been executed in this environment.
- Local frontend typecheck/test/build results have not been executed in this environment.
- Cross-platform Tauri artifacts for the `v0.1.2` tag have not yet been observed.
- Clean-machine installer smoke tests remain required.
- Signing/notarization remains unverified.
- Dependency lockfiles remain a separate reproducibility task unless generated and committed in the maintenance line.

## After v0.1.2

Do not merge the `0.1.x` maintenance branch into modern `main` merely to advance the patch version. After the patch release is verified, return to the modern feature-development line for the next milestone.

The next feature release should be planned from the current `main`/development state rather than backporting unrelated `0.2.x` or `0.3.x` functionality into `0.1.x`.

## Previous v0.1.1 work record

The v0.1.1 maintenance line included:

- workspace/frontend/Tauri version synchronization to `0.1.1`;
- Unicode-character-based rule value and filename-regex limits;
- boundary tests for 128 and 129 Unicode characters;
- `CHANGELOG.md` release entry;
- `RELEASE_NOTES_v0.1.1.md`;
- `docs/release-v0.1.1-checklist.md`;
- release/handoff documentation.

The `release/0.1.2` branch starts from that completed maintenance line and adds only the patch changes documented above.

## Continuation rule

Before publishing `v0.1.2`, verify the final branch tree, execute the complete local release gate, push the tag, inspect all platform workflow results, review artifacts, and perform clean-machine smoke tests. Do not claim publication until those gates have actually passed.
