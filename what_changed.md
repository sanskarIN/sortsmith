# SortSmith — Work Handoff

## Repository state

- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- Public/open-source license: Apache-2.0
- `main` candidate version: `0.2.0`
- Next-version development branch: `develop/0.3.0`
- `develop/0.3.0` source version: `0.3.0`
- 0.3 draft PR: `#7` — `feat: prepare 0.3 incremental preview cache`
- 0.2 reproducibility draft PR: `#8` — `build: generate reproducible 0.2 release lockfiles`
- Date of this handoff: 2026-08-25

`main` remains intentionally isolated as the `0.2.0` release candidate. The 0.3 implementation is **not merged into main**. This preserves the ability to validate and tag 0.2 independently while next-version work continues safely on a separate branch.

Neither `v0.2.0` nor `v0.3.0` should be published from the repository state described here. 0.2 still has release-evidence/platform blockers, and 0.3 is a development branch whose latest CI/CodeQL run was queued at the time this handoff was written.

## Work completed in this continuation

### 1. Created an isolated 0.3 development line

Created branch:

- `develop/0.3.0`

Base commit:

- `e1cdcfe4578d41af0afad8b6aefd8ab332d57ca5` — `docs(handoff): record 0.2.0 candidate preparation`

This branch separation is deliberate. It prevents unverified 0.3 code from making the prepared 0.2 candidate harder to reproduce or release.

A draft pull request was opened:

- PR `#7`: `https://github.com/sanskarIN/sortsmith/pull/7`
- Base: `main`
- Head: `develop/0.3.0`
- State at this handoff: open, draft, mergeable according to the GitHub connector

### 2. Implemented the first 0.3 Scale & Automation feature: incremental interactive preview caching

New core module:

- `crates/sortsmith-core/src/scan_cache.rs`

The cache is deliberately process-local and in-memory. It is an optimization, not a filesystem authority and not a persisted index.

The cache scope is exact across:

- selected root;
- complete ordered rule set;
- scan options.

If any scope input changes, cached entries are discarded before the next plan is reused.

A cached file description is reusable only while all of these remain true:

- the same file path is still encountered;
- current file size equals cached size;
- current modification timestamp equals cached modification timestamp.

Files that disappear from the walk are pruned from the cache. New, renamed, moved, or changed files use the normal metadata-description path.

### 3. Preserved time-sensitive rule correctness

`ModifiedOlderThanDays` cannot safely reuse a previous true/false match forever because wall-clock time can advance while size/path/mtime stay unchanged.

The cache therefore detects whether the active prepared rule set contains a time-sensitive modified-age criterion. Reused file descriptions are allowed, but rule matching is re-evaluated for those files on every preview.

`ScanCacheStats` tracks:

- reused files;
- rescanned files;
- revalidated time-sensitive files;
- currently retained cache entries.

These counters are intended for testing/measurement; no user-facing performance claim is made from them.

### 4. Kept destination collision safety outside the cache

The cache does **not** persist final planned destination paths.

For every preview, including a cache hit, SortSmith still runs:

- `destination_for(...)` from the current rule/file description;
- `collision_safe_path(...)` against the current filesystem.

This matters because a destination can become occupied even when the source file has not changed.

A dedicated regression test now warms the cache, creates the previously selected destination as a new conflicting file, previews again, and verifies that the second plan selects a different non-existing destination.

### 5. Added cached-versus-uncached decision equivalence coverage

A core test now runs the existing uncached planner and the new cached planner on the same disposable fixture and verifies equality of the meaningful planning decisions:

- scanned-file count;
- ignored-file count;
- recoverable errors;
- operation count;
- source;
- destination;
- rule ID;
- rule name;
- file size.

Generated operation UUIDs are intentionally not compared because each preview is a new plan.

### 6. Added cache invalidation/regression coverage

`scan_cache.rs` now tests:

- unchanged-file description reuse;
- exact rule-scope invalidation;
- changed-file rescanning;
- deleted-file pruning;
- explicit cache clearing;
- modified-age/time-sensitive revalidation;
- cached/uncached planning decision equivalence;
- collision recomputation on cache hits.

All filesystem tests use temporary directories and disposable files.

### 7. Refactored file description creation for safe reuse

`crates/sortsmith-core/src/engine.rs` now has a crate-visible helper that builds a `FileEntry` from already-fetched `std::fs::Metadata`.

The existing uncached path delegates to this helper, so cached and uncached previews share the same file-description construction logic instead of maintaining divergent metadata parsing.

### 8. Exported the cache as a core API

`crates/sortsmith-core/src/lib.rs` now exports:

- `preview_organization_cached`
- `ScanCache`
- `ScanCacheStats`

The old `preview_organization` API remains available and unchanged.

### 9. Integrated cached previews into the Tauri desktop host

`apps/desktop/src-tauri/src/lib.rs` now manages one process-local `Mutex<ScanCache>` through Tauri managed state.

The interactive `preview` command:

1. canonicalizes/validates the selected root exactly as before;
2. builds the same safe `ScanOptions` (`follow_links: false`);
3. attempts to lock the cache;
4. uses `preview_organization_cached(...)` when the cache is available;
5. falls back to the existing uncached `preview_organization(...)` if the mutex is poisoned/unavailable.

A cache failure therefore cannot disable preview or cause validation to be skipped.

### 10. Invalidated the desktop cache before filesystem mutation

The Tauri host clears the interactive preview cache before:

- apply/execute;
- undo;
- in-app watched-folder execution.

The cache is cleared before the mutation rather than after a reported success. This matters because a partially successful filesystem operation followed by a later error must not leave a pre-mutation cache that appears reusable.

Watched-folder execution still uses the existing uncached planner in this phase. The interactive cache is only an interactive-preview optimization, not a hidden change to watched-folder scheduling behavior.

### 11. Added paired warm-cache benchmarking

`crates/sortsmith-core/benches/planning.rs` retains the existing `organization_planning` benchmark and adds:

- `organization_planning_warm_cache`

Both benchmark the same synthetic 100, 1,000, and 5,000-file sizes.

The warm-cache fixture is populated before measurement. No performance percentage, latency target, or budget is claimed yet. Correct use is to measure both groups on the same machine, same filesystem/storage, same Rust toolchain, and same build profile.

### 12. Added 0.3 cache design documentation

Updated:

- `docs/testing.md`
- `docs/architecture.md`
- `README.md`
- `ROADMAP.md`
- `CHANGELOG.md`

Added:

- `docs/adr/0004-process-local-preview-cache.md`

ADR 0004 records why the first cache is process-local instead of persistent, why destination choices remain uncached, why time-sensitive decisions must be re-evaluated, and why native watcher-only invalidation is deferred.

### 13. Prepared the branch as a real 0.3.0 development line

The three release/version metadata sources on `develop/0.3.0` are aligned at `0.3.0`:

- workspace `Cargo.toml`;
- `apps/desktop/package.json`;
- `apps/desktop/src-tauri/tauri.conf.json`.

This is development metadata only. It is **not** authorization to create `v0.3.0`.

### 14. Found and addressed a real frontend CI dependency-install failure

The first PR #7 CI run reached frontend dependency installation and failed before TypeScript/Vitest/build with:

```text
npm error Cannot read properties of null (reading 'edgesOut')
```

That run used:

- Node.js `22.23.2`
- npm `10.9.8`

The repository explicitly declares `npm@10.9.2` in `apps/desktop/package.json`.

The 0.3 CI workflow now installs the repository-pinned npm version before dependency resolution:

```bash
npm install --global npm@10.9.2
```

The final 0.3 CI run must confirm whether this resolves the hosted-runner Arborist/install failure. Do not mark frontend CI green until that run completes.

## 0.2 release blocker work started in parallel

### 15. Created a separate 0.2 lockfile-generation branch

Created:

- `release/0.2-lockfiles`

Base:

- the unchanged 0.2 candidate head on `main` (`e1cdcfe4578d41af0afad8b6aefd8ab332d57ca5`).

Draft PR opened:

- PR `#8`: `https://github.com/sanskarIN/sortsmith/pull/8`
- Base: `main`
- Head: `release/0.2-lockfiles`

This work exists because the prior handoff correctly refused to hand-author `Cargo.lock` or `apps/desktop/package-lock.json`.

### 16. Added a temporary trusted GitHub Actions lockfile generator on the release branch

Temporary workflow on the release branch only:

- `.github/workflows/generate-release-lockfiles.yml`

Its job uses:

- stable Rust/Cargo;
- Node.js 22;
- repository-pinned npm `10.9.2`.

It generates:

- `Cargo.lock` with `cargo generate-lockfile`;
- `apps/desktop/package-lock.json` with npm package-lock generation.

Before it is allowed to commit the generated files, it runs:

```bash
node scripts/verify-release-version.mjs v0.2.0
node scripts/verify-release-lockfiles.mjs
cargo fetch --locked
cd apps/desktop
npm ci --ignore-scripts --no-audit --no-fund
```

It also runs `git diff --check`.

If successful, it commits only the generated lockfiles with:

```text
build(lockfiles): generate 0.2 release dependency locks
```

The workflow configures the commit identity as:

- name: `sanskarIN`
- email: `sanskarin@outlook.in`

At the time this handoff entry was written, PR #8's `Generate 0.2 release lockfiles` job was queued. `Cargo.lock` had not yet appeared on the release branch, so this file does **not** claim that lockfile generation has succeeded.

The temporary generator workflow must be deleted from the release branch after generation/review. The final durable 0.2 PR should not merge a one-off lockfile-generation workflow into `main` unless there is a separate deliberate reason to keep it.

## GitHub validation state at this handoff

### PR #7 — 0.3 incremental cache

Latest code head immediately before this handoff update:

- `ce3f74df9f2316507da0b98768ea42fe46ce28ff`

GitHub had queued:

- CI run `32842573251`
- CodeQL run `32842573276`

The previous PR #7 frontend run provided a real failure signal at dependency installation (`edgesOut`) and the npm pin fix was committed afterward.

Because current Rust jobs had not started at handoff time, this continuation does **not** claim:

- `cargo fmt --check` success for the cache code;
- Clippy success;
- cache test compilation/success;
- Tauri host compilation/success;
- final frontend typecheck/test/build success;
- CodeQL success.

When CI starts, inspect actual failing job logs and commit fixes instead of guessing.

### PR #8 — 0.2 reproducibility

GitHub had queued:

- `Generate 0.2 release lockfiles` run `32842676221`
- CI run `32842676303`
- CodeQL run `32842676246`

Do not claim the 0.2 dependency blocker cleared until real generated lockfiles exist, pass the verification scripts, and survive clean locked installation/resolution.

## Commits made on `develop/0.3.0` in this continuation

Starting from 0.2 candidate head `e1cdcfe4578d41af0afad8b6aefd8ab332d57ca5`:

1. `86605a4517340bcf6f25aa1d0ff3746b67c1c9dd` — `refactor(core): share file metadata description helper`
2. `b58b717fd921b85568396573d374c959794bade0` — `feat(core): add incremental preview scan cache`
3. `95d2057b1cd7a4beb0f795c135fa136fbd56b5f5` — `feat(core): export cached preview API`
4. `012f9782e135e8714bbd452b9cfd1583f6f0c65e` — `test(core): cover scan cache invalidation and reuse`
5. `af761f7c692163150cf09a5d5fbe31de58bdc0c6` — `feat(desktop): enable cached interactive previews`
6. `2ca9f14f547a39fb113a0fd4228961e9df658f9d` — `bench(core): add warm incremental preview benchmark`
7. `ca229e75d9ab446d8c6574e72f652a0334ab6c72` — `docs(testing): define incremental cache verification`
8. `9311176bf659ce3664c9e1c7ed6d4b4f5c1e45e6` — `docs(architecture): document preview cache safety boundary`
9. `f8253d684cb5a42caf5eef16dcaf25480028a86f` — `release: bump workspace development version to 0.3.0`
10. `d5269ba4ec40c205ad3786c5064ff959a2e85c7d` — `release(frontend): bump desktop development version to 0.3.0`
11. `e0aa7b7e7530f05cbf139727cabdfb3d964d909a` — `release(tauri): bump application development version to 0.3.0`
12. `46b9c3a71a7c3c78a08e59935b30b8a6417b9159` — `docs(changelog): open 0.3.0 development section`
13. `28730b426c87bbe19edf559c8fb478d18ac67280` — `docs(roadmap): record 0.3 cache implementation status`
14. `2ee74461b01f2a57f0f3cc70a5ec7a1a1e3f6445` — `docs(readme): describe 0.3 development line`
15. `1d3d70c877d7cf0984994066839776a0347e1cd1` — `ci(frontend): use repository-pinned npm version`
16. `2027f94178c23dc4306597289c94bc2f0c131cf6` — `docs(adr): record process-local preview cache decision`
17. `ce3f74df9f2316507da0b98768ea42fe46ce28ff` — `test(core): prove cached preview decision equivalence`

This `what_changed.md` update is the next focused commit after those entries.

## Commits made on `release/0.2-lockfiles` so far

1. `62b235d555d7ede336a14b9a8af6454cc76cc295` — `ci(release): generate 0.2 lockfiles on release branch`
2. `a995b7efa2f8e96f0aad042ed5bcfda698cd86d7` — `ci(release): allow lockfile generation from draft PR`

Expected next generated commit, only if the workflow succeeds:

- `build(lockfiles): generate 0.2 release dependency locks`

Do not invent this commit or its contents if the job fails.

## Remaining work before `v0.2.0`

1. Let the package-manager generator create real `Cargo.lock` and `apps/desktop/package-lock.json`.
2. Inspect both generated files and verify expected workspace package versions.
3. Confirm `verify-release-lockfiles.mjs`, `cargo fetch --locked`, and `npm ci` succeed on the generated dependency graph.
4. Remove the temporary generator workflow from the release branch.
5. Change ordinary frontend CI to deterministic lockfile installation once the lockfile is committed; pin npm to the declared repository version if hosted runners otherwise drift.
6. Run/fix full Rust format, Clippy, tests, desktop host checks, TypeScript, Vitest, Vite build, and CodeQL against the exact candidate.
7. Run same-machine Criterion baselines and record toolchain/hardware/storage.
8. Build Windows/macOS/Linux candidate packages.
9. Complete clean-machine install/first-launch/preview/apply/undo/import/export/accessibility/uninstall smoke tests.
10. Resolve signing/notarization requirements for intended distribution.
11. Fill `docs/release-evidence-0.2.0.md` with real evidence.
12. Capture verified screenshots from the tested artifacts.
13. Only then approve/publish `v0.2.0`.

## Remaining work for 0.3

1. Obtain a fully green final PR #7 CI/CodeQL run and fix every real compiler/formatter/lint/test failure.
2. Run uncached and warm-cache Criterion groups on one controlled machine and record actual measurements.
3. Define a performance budget only from those measurements.
4. Exercise repeated preview → filesystem change → preview → apply → undo cycles on Windows, macOS, and Linux.
5. Decide whether a single in-memory scope is sufficient or whether per-root partitioning is justified by measured workflows.
6. Do not implement persistent caching unless a new design defines bounded storage, versioning, corruption recovery, and invalidation behavior.
7. Design native background scheduling separately, including explicit per-platform consent, startup/disable behavior, permissions, uninstall cleanup, error visibility, and reversible execution guarantees.
8. Only after 0.2 is resolved and 0.3 is independently verified should the 0.3 development PR be considered for merge/release preparation.

## State/migration compatibility

- Persisted application state schema remains `1`.
- The 0.3 preview cache is not stored in `state.json`.
- The cache is not included in settings export/import.
- Undo journal format is unchanged.
- Bundled-preset migration remains unchanged.
- No new user data migration is required for the current in-memory cache implementation.
- Restarting SortSmith always starts with an empty preview cache.

## Important safety/release rules for the next continuation

- Do not merge PR #7 into `main` merely because the source version says 0.3.0.
- Do not tag `v0.2.0` until release evidence is complete.
- Do not hand-author dependency lockfiles.
- Do not claim cache performance without real same-machine measurements.
- Do not cache final destination collision choices.
- Do not persist the cache without a new explicit format/invalidation decision.
- Do not claim GitHub checks are green unless the exact latest commit has completed green checks.
