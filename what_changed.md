# SortSmith — Work Handoff

## Current version / milestone

- Version prepared in source: `0.2.0`
- Intended next tag: `v0.2.0`
- Date: 2026-08-24
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- Visibility/source model: public, open source
- License: Apache-2.0
- Current milestone: 0.2 Desktop Polish source work is substantially implemented and release metadata is prepared. The repository is **not yet approved for public release** because dependency lockfiles, actual green platform CI evidence, clean installer smoke tests, verified screenshots, and signing/notarization decisions remain outstanding.

## Work completed in this continuation

### 1. Prepared version 0.2.0 consistently

The three authoritative version sources now all use `0.2.0`:

- workspace `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

The About page no longer contains its own hard-coded release number. It imports `package.json` and renders `packageMetadata.version`, removing a fourth version value that could drift from the release metadata checked by CI.

The package JSON import shape was independently checked with the available local TypeScript compiler using the project's JSON-module-compatible compiler settings.

### 2. Added a stable bundled preset catalog

The previous build had one built-in `Everyday tidy` preset whose UUID was generated randomly. That made it impossible to identify bundled presets robustly across installations and made the previous frontend assumption of “preset at index 0 is built-in” fragile.

Version 0.2 now defines stable bundled preset UUIDs and ships four built-in packs:

1. `Everyday tidy`
   - Images
   - Documents
   - Archives
   - Audio
   - Video
2. `Media library`
   - `Media/Images`
   - `Media/Audio`
   - `Media/Video`
3. `Developer workspace`
   - `Development/Source`
   - `Development/Data`
   - `Development/Packages`
4. `Downloads cleanup`
   - Installers
   - Archives
   - Documents
   - Images

Fresh Rust default state now contains those four presets with stable preset IDs. Bundled rule definitions continue to use normal validated SortSmith rule models, so they go through the same path/subdirectory validation as user-created rules.

### 3. Added backward-compatible bundled-preset migration

`apps/desktop/src/bundledPresets.ts` contains the frontend compatibility layer for existing 0.1 state.

The migration behavior is deliberately non-destructive:

- if the stable `Everyday tidy` ID already exists, no legacy-ID migration is needed;
- otherwise, an existing legacy preset named `Everyday tidy` is preserved and assigned the new stable bundled ID;
- its existing rule snapshot is preserved instead of being replaced silently;
- watched-folder entries referencing the old random preset ID are remapped to the stable ID;
- other missing bundled packs are appended while capacity remains;
- the application never removes user presets to make room for bundled packs;
- the backend 50-preset limit is respected;
- if the catalog cannot be completed because all preset slots are used, the UI reports the number of missing bundled packs.

The normalization runs both during initial state loading and before every persisted state write. This means an imported legacy settings backup is upgraded immediately through the existing `persist(...)` path instead of waiting for an application restart.

The persisted state schema remains version `1`; no incompatible schema bump was needed because stable bundled UUIDs are valid normal preset identifiers in the existing schema.

### 4. Hardened preset management semantics

`PresetManager.tsx` no longer treats `state.presets[0]` as the only protected built-in preset.

Bundled preset protection is now based on stable bundled IDs:

- all four bundled presets can be loaded;
- bundled presets cannot be renamed;
- bundled presets cannot be deleted;
- custom presets remain editable;
- custom presets remain undeletable while a watched folder references them;
- users can customize a bundled pack by loading it, editing active rules, and saving the active rule set as a new user preset.

This preserves a recoverable built-in library without preventing user customization.

### 5. Expanded bundled-preset tests

`apps/desktop/src/bundledPresets.test.ts` covers:

- unique stable bundled preset identifiers;
- rule-ID uniqueness within each bundled preset;
- adding missing bundled packs while preserving custom presets;
- migration of the legacy random `Everyday tidy` ID;
- watched-folder reference remapping during migration;
- preservation of the legacy preset rule snapshot;
- no-op behavior when the catalog is already complete;
- preservation of the backend 50-preset capacity limit without deleting user data.

The bundled-preset helper was also checked independently with strict local TypeScript type checking using the TypeScript compiler available in this environment.

### 6. Expanded Rust default-preset verification

`crates/sortsmith-core/src/models.rs` now provides the four bundled presets in new/default application state.

Core tests now verify:

- bundled preset IDs are stable across repeated `default_presets()` calls;
- bundled preset IDs are unique;
- every bundled rule passes the existing Rust rule validator.

The Rust execution environment available in this chat still does not contain `rustc` or `cargo`, so these tests have not been claimed as locally executed. They must run in GitHub Actions or another trusted Rust environment.

### 7. Improved keyboard dialog accessibility

The shortcut dialog already had labelled modal semantics and Escape dismissal. Version 0.2 now also manages focus explicitly:

- when the shortcut reference opens, focus moves to its Close button;
- the dialog exposes descriptive text through `aria-describedby`;
- when the dialog closes, focus returns to the element that was active before the dialog opened;
- previous focus state is cleared after restoration.

This avoids leaving keyboard users at the document body after dismissing shortcut help.

Real platform accessibility checks are still required before release.

### 8. Strengthened continuous release-version verification

`scripts/verify-release-version.mjs` now supports two modes:

- **CI/development mode** with no tag argument: it uses the workspace Cargo version as the expected version and verifies that frontend and Tauri metadata agree;
- **release-tag mode** with `vX.Y.Z`: it additionally verifies the exact requested tag.

The frontend CI job now runs the no-tag synchronization check before dependency installation/typecheck/tests/build.

The release version script was behavior-checked locally with isolated metadata fixtures:

- aligned `0.2.0` metadata passed in no-tag mode;
- aligned `v0.2.0` passed in tag mode;
- stale `v0.1.0` correctly failed against 0.2.0 metadata.

### 9. Added fail-closed release lockfile verification

New script: `scripts/verify-release-lockfiles.mjs`.

The script requires:

- `Cargo.lock`
- `apps/desktop/package-lock.json`

It also checks:

- npm lockfile format is modern enough for the supported npm line;
- the npm root package version matches `package.json`;
- the Cargo lockfile contains both workspace packages (`sortsmith` and `sortsmith-core`);
- both workspace package versions in `Cargo.lock` match the frontend release version.

The script was syntax-checked with Node.js 22.16.0 and behavior-checked locally with isolated fixtures:

- it correctly failed when `Cargo.lock` was absent;
- it correctly passed with structurally valid test lockfiles aligned at `0.2.0`.

The test lockfiles used for script verification were temporary local fixtures only. They were **not** committed and are not substitutes for package-manager-generated project lockfiles.

### 10. Hardened the tag-driven release workflow

`.github/workflows/release.yml` now performs these gates before Tauri packaging:

1. exact tag/version verification;
2. committed lockfile verification;
3. `cargo fetch --locked`;
4. Linux Tauri dependency installation when applicable;
5. frontend dependency installation with `npm ci --no-audit --no-fund`;
6. Tauri cross-platform draft packaging.

A tag created before real lockfiles are committed will fail closed instead of silently resolving an unreviewed dependency graph.

The release continues to create draft artifacts rather than automatically publishing unverified installers.

### 11. Added explicit 0.2.0 release evidence documentation

New file: `docs/release-evidence-0.2.0.md`.

It contains an evidence template for:

- candidate SHA/tag identity;
- CI and CodeQL status;
- lockfile reproducibility;
- Rust/frontend quality commands;
- benchmark environment;
- Windows installer verification;
- macOS installer/signing/notarization verification;
- Linux package verification;
- keyboard/accessibility checks;
- preset migration and bundled-preset checks;
- real screenshot provenance;
- final BLOCKED/APPROVED decision.

The file is intentionally a blank verification template and does not claim that platform tests have passed.

### 12. Updated release-facing documentation

Updated documents include:

- `README.md`
- `CHANGELOG.md`
- `ROADMAP.md`
- `PRIVACY.md`
- `SECURITY.md`
- `docs/architecture.md`
- `docs/development.md`
- `docs/testing.md`
- `docs/accessibility.md`
- `docs/github.md`
- `docs/keyboard-shortcuts.md`
- `docs/release.md`
- `docs/screenshots/README.md`

New documents include:

- `docs/presets.md`
- `docs/release-evidence-0.2.0.md`

The documentation now distinguishes **prepared source version** from **verified public release** and no longer treats changing the version number as proof that 0.2.0 is shippable.

## Current verification status

### Checks actually performed in this continuation environment

Available local tools detected:

- Node.js `v22.16.0`
- npm `10.9.2`
- global TypeScript compiler `5.8.3`

Unavailable local tools:

- `rustc`
- `cargo`

Performed locally with isolated fixtures:

- Node syntax check of `verify-release-version.mjs` logic;
- Node syntax check of `verify-release-lockfiles.mjs`;
- release-version guard success for aligned 0.2.0 metadata;
- release-version guard failure for stale `v0.1.0`;
- release-lockfile guard failure when lockfiles are absent;
- release-lockfile guard success with structurally aligned temporary fixture lockfiles;
- strict TypeScript check of bundled-preset migration/remapping helper structure;
- TypeScript JSON-module import check matching the About-page package-version pattern.

### Checks not claimed

This continuation does **not** claim:

- Rust formatting success;
- Rust Clippy success;
- Rust unit/property test success;
- Tauri Rust compilation success;
- full project TypeScript/Vitest success with the repository's pinned dependency graph;
- Vite production build success;
- native Tauri build success;
- real Criterion timing results;
- Windows/macOS/Linux installer success;
- signing/notarization completion;
- real screenshot completion.

Those require the actual project dependencies/toolchains or target platforms.

## GitHub status visibility

The connected GitHub integration confirmed the repository remains public and this account retains push/admin permission.

For the pre-handoff 0.2 candidate head, GitHub's combined-status endpoint returned no surfaced status contexts. A direct public Actions page fetch also did not provide usable run data in this environment. Therefore this file does **not** mark CI green. The next continuation must inspect actual workflow results when GitHub exposes them and fix any real failures before release.

## Dependency reproducibility blocker

The repository still does **not** contain package-manager-generated release lockfiles:

- `Cargo.lock`
- `apps/desktop/package-lock.json`

This is now enforced by the release workflow itself.

Do not hand-author either lockfile. Generate them from the committed manifests using the supported toolchains in a trusted networked environment, review them, and commit them before `v0.2.0`.

After those files exist, ordinary CI should also be migrated from resolution-based installs to lockfile-enforcing commands where appropriate.

## Remaining work before v0.2.0 can be published

1. Inspect the latest GitHub Actions runs for the exact 0.2.0 candidate commit and fix every real format, Clippy, test, typecheck, build, or CodeQL failure.
2. Generate `Cargo.lock` with Cargo from the current committed Rust manifests.
3. Generate `apps/desktop/package-lock.json` with supported npm from the current committed frontend manifest.
4. Review and commit both generated lockfiles.
5. Run:

   ```bash
   node scripts/verify-release-version.mjs v0.2.0
   node scripts/verify-release-lockfiles.mjs
   cargo fetch --locked
   ```

6. Run the full Rust/frontend/Tauri quality suite from a clean checkout.
7. Run `cargo bench -p sortsmith-core --bench planning` on a consistent machine and record the first real baseline with hardware/toolchain information.
8. Complete Windows, macOS, and Linux accessibility/integration checks from `docs/testing.md` and `docs/accessibility.md`.
9. Build candidate installers/packages on every distributed platform.
10. Fill `docs/release-evidence-0.2.0.md` with actual artifact/check evidence.
11. Smoke-test install, first launch, disposable-fixture preview/apply/undo, preset migration, native dialogs, keyboard focus behavior, and uninstall/removal on clean target systems.
12. Configure signing/notarization through protected credentials where required by the intended distribution path.
13. Capture real screenshots only from the verified artifacts and record their provenance.
14. Only after all release blockers are cleared, create/publish `v0.2.0` and finalize the dated changelog entry.

## Work intentionally deferred beyond 0.2.0

### Native background scheduling

Watched folders still run only while SortSmith is open. Native operating-system scheduling remains deferred until platform-specific startup, consent, permission, and disable/uninstall behavior can be designed and tested safely.

### Incremental scan cache

The cache is not implemented yet. Criterion targets exist, but a real baseline still needs to be measured. Cache design must define explicit invalidation rules and must be verified against the uncached planner before it can be enabled.

### Performance budgets

The repository has repeatable benchmark targets but no claimed release performance numbers yet. Establish same-machine baselines before setting budgets or claiming cache improvements.

These are the primary 0.3 Scale & Automation tasks after the 0.2 release gate is satisfied.

## Migration notes

- Persisted state schema remains `1`.
- Existing 0.1 `Everyday tidy` random IDs are migrated at the frontend state-normalization boundary rather than through a schema-version bump.
- Watched-folder references to the migrated legacy preset are updated in the same normalized state write.
- User rules and custom presets are preserved.
- No user preset is deleted to make room for a bundled preset.
- Undo journal format remains unchanged.
- Settings backup format remains compatible with schema `1`.

## Commits in this 0.2.0 continuation batch

Starting from previous handoff head `237d84991b41334be10a7f61e46bc134b589988d`, the pre-handoff candidate reached `3b4448d82fa8cf43341fea1838e69a123d4bace1`, **30 commits ahead** with no commits behind.

1. `1ff7b4e37cf839b7628c0ac9355af51178b67897` — `feat(frontend): define stable bundled preset catalog`
2. `23cd25c3635d6632538aba48a1ad0b10c911dcdf` — `test(frontend): cover bundled preset migration`
3. `0220dc06fc7709545a816b1d172f0822d0acb3a0` — `fix(frontend): protect all bundled presets by stable id`
4. `5ca1b5a5b9ca9b4e0e3783cef4ec9eed3e74b226` — `feat(core): expand stable bundled preset library`
5. `b7991904a58d85160720d35b2f28c5da3cd142b3` — `feat(frontend): migrate bundled presets on startup`
6. `f24517f1bf40d1d635a3f9d43fe6214bfe3793ad` — `a11y(frontend): restore focus around shortcut dialog`
7. `fd02142b414c7571e3eb3fdf223abd9b61538947` — `refactor(frontend): source about version from package metadata`
8. `4269ae046659d8768f43f6c468fc0bd37e0ecd9e` — `release: bump workspace version to 0.2.0`
9. `4ecbcffff4ecb5566c3a76304559f74aa344e810` — `release(frontend): bump desktop version to 0.2.0`
10. `cec8d2a12d90fe8d0d324ba4af133f53e445092c` — `release(tauri): bump application version to 0.2.0`
11. `17ab9e28fa4e79d5cb293335eeb02dbf3f5b79a8` — `release: require dependency lockfiles before tagging`
12. `0cd6cf3a13a65bace32fc242bc4eeeea773b2f9a` — `ci: allow release metadata sync checks before tagging`
13. `3578d13a3b9c72f3910b8ea24bda598011094350` — `ci: verify release metadata stays synchronized`
14. `0c480e1b870d8c378b0c2658cae7be7f740243b3` — `release: enforce lockfiles before packaging`
15. `b445bebafa5bcf3862f153b938690e196c642a51` — `docs(changelog): prepare 0.2.0 release candidate notes`
16. `b461ea0d7256ca0112dc3bf2c83943048ef0d132` — `docs(roadmap): align 0.2 candidate and delivered preset packs`
17. `24662d6e844a73f6ad2d2642b6584809c340f571` — `fix(frontend): normalize bundled presets on every save`
18. `1391db192859043c9b1cf6f84602456c1d40f1c0` — `docs(presets): document bundled and saved preset behavior`
19. `8b5b20ef85372663811fee4fe6fbbde492f96359` — `docs(readme): prepare 0.2.0 candidate guidance`
20. `61e28fb73415f8ab0f4c4493e3f836bbb2f051d5` — `docs(release): enforce 0.2 lockfile and smoke-test gate`
21. `fd143d64b5fa13699a8b1ddf988d50c095602b70` — `docs(development): align 0.2 preset and release workflow`
22. `a04dae103ffe5cecfa7f3206b959fb93bc8f817d` — `docs(testing): cover 0.2 preset migration and release guards`
23. `f42bfc62712d379b382d9f5014fc70056a216d46` — `docs(accessibility): document 0.2 focus restoration behavior`
24. `b69ea0ccc9067b5c47a6ad4b45da91c13bd78c51` — `docs(shortcuts): document modal focus lifecycle`
25. `7efae67a4f97e4f15dc565aad462268ab787a1fd` — `docs(github): document 0.2 release safeguards`
26. `401decfc92cbf1862ab4f85346e5d7317b05cbc6` — `docs(release): add 0.2.0 verification evidence template`
27. `f66f2e8bcbd2059e10ad0b5336b8e88d7b439ab2` — `docs(screenshots): align capture plan with 0.2.0 evidence`
28. `6b0f787e19a9acd504c047093a4ba7abb46b777b` — `docs(privacy): align no-telemetry statement with 0.2.0`
29. `49573ed93c2852f6914bf2165deb0b10035ef70c` — `docs(security): align threat model with 0.2.0`
30. `3b4448d82fa8cf43341fea1838e69a123d4bace1` — `docs(architecture): align state and automation notes with 0.2`

This handoff update itself is the next commit after that 30-commit range.

## Continuation rule

Before the next code change:

1. read this file;
2. read `ROADMAP.md`;
3. inspect the newest `main` commits;
4. inspect actual GitHub Actions/CodeQL results if available;
5. prioritize demonstrated CI/build failures and release lockfiles before adding another release feature.

Do not mark `v0.2.0` ready based only on source review. Keep filesystem changes local-first, previewed, reversible, and root-contained. Keep commits small and reviewable. Update this file again after the next meaningful batch.
