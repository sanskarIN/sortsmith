# SortSmith — Work Handoff

## Current workstream: v0.1.1

- Release branch: `release/0.1.1`
- Base source commit: `fd02142b414c7571e3eb3fdf223abd9b61538947`
- Base commit meaning: final `0.1.0` source line immediately before the `0.2.0` version bump.
- Target patch version: `0.1.1`
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- License: Apache-2.0
- Commit identity requested for project work: `Sanskar <sanskarin@outlook.in>`

## Important release-history correction

The existing Git tag `v0.1.0` currently points at the modern `main` tip rather than the historical `0.1.0` source commit. The historical `0.1.0` source boundary was recovered from the parent of commit `4269ae046659d8768f43f6c468fc0bd37e0ecd9e`, whose patch changes the workspace version from `0.1.0` to `0.2.0`.

For that reason, this continuation deliberately created `release/0.1.1` from `fd02142b414c7571e3eb3fdf223abd9b61538947`, not from the current `main` branch. This keeps the `0.1.x` maintenance line separate from the later `0.2.x` and `0.3.x` feature lines.

The existing `v0.1.0` tag must not be rewritten casually. It should be treated as an existing repository-history artifact while the new `v0.1.1` tag is created from the dedicated maintenance branch after verification.

## v0.1.1 implementation completed

### Version synchronization

The following release metadata now reports `0.1.1` on `release/0.1.1`:

- root `Cargo.toml` workspace package;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

The repository already contains `scripts/verify-release-version.mjs`, which accepts a tag such as `v0.1.1` and verifies the Cargo, frontend, and Tauri versions are identical.

### Core patch

`crates/sortsmith-core/src/rules.rs` now measures bounded textual rule values by Unicode character count instead of UTF-8 byte length.

This matters because a limit described as a character limit should not reject a valid non-ASCII value merely because the same characters occupy multiple UTF-8 bytes. The patch changes:

- extension/MIME rule value length checks to character counting;
- filename-regex pattern length checking to character counting.

### Regression coverage

Added core tests for the Unicode boundary:

- exactly 128 Unicode characters are accepted;
- 129 Unicode characters are rejected.

Existing safety and rule tests remain in the same module, including:

- unsafe rename-prefix rejection;
- invalid size-range rejection;
- unknown rename-template placeholder rejection;
- safe template rendering and extension preservation;
- reserved rendered filename rejection;
- extensionless trailing-period rejection;
- repeated prepared-regex matching.

## Release documentation completed

- `CHANGELOG.md` now has a dedicated `0.1.1` entry dated 2026-09-03.
- `RELEASE_NOTES_v0.1.1.md` contains the publication-ready release notes and verification commands.
- `docs/release-v0.1.1-checklist.md` contains the complete publication gate, tag instructions, GitHub release fields, and post-release checks.

## Release automation reviewed

The repository's release workflow is tag driven. It runs for `v*` tags, verifies release metadata, builds on Ubuntu, Windows, and macOS, and uses the Tauri action to prepare a draft GitHub release.

The release workflow currently installs frontend dependencies with `npm install`. Because the `0.1.0` historical source line does not contain a committed npm lockfile, `npm ci` is not appropriate until a lockfile is generated and committed on the relevant maintenance line.

The release workflow also calls `scripts/verify-release-version.mjs`, so the synchronized `0.1.1` metadata is ready for the tag validation stage.

## Commits made for v0.1.1

1. `543d3fbec7a8f52ab31602301597f9d5b92333b9` — `release: bump workspace version to 0.1.1`
2. `28744cc6cbd5768403a3a612a0ceedb00b1e9a59` — `release(frontend): bump desktop version to 0.1.1`
3. `56b3a8ef89a707ffdbaa01399701141b2c3a9509` — `release(tauri): bump application version to 0.1.1`
4. `73cd2ddfad5c0695e7a7c91fcb3bf1628b0c077b` — `fix(core): validate rule text limits by characters`
5. `4f1c391f0f147999b67f33ca6f2a1bd5fc0c55ad` — `test(core): clean Unicode rule limit regression coverage`
6. `0a7b8fb31139fc8c07c4511dd777e639ea607205` — `docs(changelog): prepare v0.1.1 patch release notes`
7. `d9076074e52ad2fb1e676a1ef6d9fee29d4087f8` — `docs(release): add v0.1.1 release notes`
8. `bb2608205ec9c0f272db002a1d4e1adb88212d93` — `docs(release): add v0.1.1 publication checklist`

The commits are intentionally separated by responsibility rather than manufactured as empty commits. This preserves a reviewable release history while still providing the requested maximum meaningful granularity.

## Verification status

Repository-level preparation is complete, but this environment has not executed the Rust or frontend toolchains locally.

Required verification before tagging:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings

cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

Also run:

```bash
node scripts/verify-release-version.mjs v0.1.1
git diff --check
```

The release workflow should then be allowed to validate the exact tag on all supported build platforms.

## GitHub Actions limitation

The current CI workflow triggers on pushes to `main` and on pull requests; it does not run for a plain push to `release/0.1.1`. Therefore the absence of a CI run on the maintenance branch must not be interpreted as a successful test result.

The tag-driven release workflow is the important final cross-platform gate because it verifies the tag version and invokes Tauri builds on Ubuntu, Windows, and macOS.

## v0.1.1 release procedure

After the local release commands above pass:

```bash
git checkout release/0.1.1
git pull origin release/0.1.1
git tag -a v0.1.1 -m "SortSmith v0.1.1"
git push origin v0.1.1
```

The tag push should start `.github/workflows/release.yml`. Let the three platform builds finish before considering the release published.

Recommended GitHub release title:

`SortSmith v0.1.1 — Patch Release`

Use `RELEASE_NOTES_v0.1.1.md` as the release-body source. The automated Tauri workflow is configured to create a draft release, so the final publication should be reviewed manually after the artifacts are available.

## Current main-line context

The modern `main` branch has continued beyond the historical `0.1.x` line and currently contains later `0.2.x` and `0.3.x` work, including the incremental preview-cache development. Do not merge the `release/0.1.1` branch into `main`; it is a maintenance-release line intended to produce the patch tag from the historical baseline.

After `v0.1.1` is successfully verified and published, future feature development should continue from the modern `main`/development line. The next feature release remains a separate milestone rather than a continuation of the `0.1.x` maintenance branch.

## Known release blockers

- Local Rust verification has not been executed in this environment.
- Local frontend verification has not been executed in this environment.
- Cross-platform Tauri builds have not yet been observed for the `v0.1.1` tag.
- Installer smoke tests have not yet been performed on clean Windows, macOS, and Linux environments.
- Signing/notarization has not been configured or verified.
- The historical `v0.1.0` tag points to a newer commit than the recovered `0.1.0` source boundary; do not rewrite it without an explicit history-migration decision.

## Continuation rule

For the next release task, first verify the final `release/0.1.1` commit tree and run the complete release gate. Do not claim `v0.1.1` is published until the tag exists and the tag-driven workflow has produced successful platform builds. Once the patch release is complete, return to the modern development line for the next feature milestone.
