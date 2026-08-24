# SortSmith — Work Handoff

## Current version / milestone

- Version: `0.1.0`
- Date: 2026-08-24
- Repository: `https://github.com/sanskarIN/sortsmith`
- Default branch: `main`
- Visibility/source model: public, open source
- License: Apache-2.0
- Current milestone: the safe desktop implementation baseline and the planned 0.2 keyboard-polish work are implemented in source. The 0.3 saved-preset, property-test, and benchmark foundations are also implemented. Release publication is still intentionally blocked on real green CI evidence, committed dependency lockfiles, clean installer smoke tests, real verified screenshots, and signing/notarization where required.

## Completed implementation baseline

### Rust core

- Platform-neutral `sortsmith-core` crate.
- Rule criteria for extension, MIME prefix, modified age, size range, and filename regex.
- Move-to-folder, filename-prefix, and rename-template actions.
- Rule validation with portable filename rules, reserved Windows device-name rejection, path-separator rejection, traversal rejection, and bounded rule shapes.
- Dry-run organization planning with collision-safe destinations.
- Reserved planned destinations are considered while planning, preventing two operations in one preview from selecting the same destination.
- Root containment checks for planned execution paths.
- Filesystem execution with cross-device fallback.
- Reversible JSON undo journals and reverse-order undo.
- Journal preflight before the first mutation so execution does not begin if undo storage cannot be initialized safely.
- Duplicate candidate discovery with size pre-filtering, BLAKE3 hashing, parallel candidate hashing, hidden-directory pruning, and no automatic deletion.
- Streamed journal I/O and bounded local persistence behavior.

### Desktop/Tauri boundary

- Schema-versioned local application state.
- Built-in preset plus custom rules, saved presets, watched folders, settings, and recent journal identifiers.
- Native folder picker.
- Native JSON settings export/import dialogs.
- Journal-history listing and selective undo UI support.
- Watched-folder scheduling while the application is open.
- Root canonicalization and operation containment validation repeated at the Tauri boundary.
- Destination-parent symlink escape checks.
- State validation limits for rules, presets, watched folders, and recent journals.
- State file size limit, regular-file checks, symlink rejection, and atomic JSON replacement.
- Privacy-oriented structured operation logging with counts/identifiers only, rotating at a bounded size.

### React/TypeScript frontend

- Responsive desktop shell with Organize, Rules, Duplicates, Automation, History, Settings, and About sections.
- Light, dark, and system themes.
- Reduced-motion preference and visible focus states.
- Live operation status messages.
- Native folder selection, dry-run preview, apply, latest undo, duplicate scan, selective history undo, settings backup/import, automation management, and support/about links.
- Rejected persisted state changes remain inactive if the backend save fails.
- Rule builder with match-all/match-any behavior and all supported criterion/action kinds.
- Saved user-defined preset management:
  - snapshot the active rule set into a custom preset;
  - load any saved preset into the active rule set;
  - edit custom preset name/description;
  - delete custom presets only when no watched folder references them;
  - preserve independent rule snapshots rather than retaining shared mutable arrays.
- Keyboard-first quick actions:
  - `Alt+1` through `Alt+7` for primary navigation;
  - `Ctrl/Cmd+O` to choose a folder;
  - `Ctrl/Cmd+Enter` to run a dry-run preview;
  - `Ctrl/Cmd+Shift+Enter` to apply an available preview;
  - `Ctrl/Cmd+Z` to undo the latest available operation;
  - `Shift+?` to open the shortcut reference;
  - `Escape` to close the shortcut reference.
- Shortcut handling does not intercept keys while focus is in input, textarea, select, or content-editable controls.
- Shortcut apply/undo paths call the same application functions as visible controls and therefore retain normal busy-state, availability, and confirmation rules.

## Verification and quality work added in the latest continuation

### Frontend tests

- Preset snapshots are tested for value equality and reference independence.
- Nested criterion arrays are verified to be cloned rather than shared.
- Empty preset creation is rejected.
- Preset metadata rename preserves the preset identifier while cloning its rule snapshot.
- Keyboard shortcut resolution is tested for Windows/Linux-style Control and macOS-style Command modifiers.
- Alt-number navigation is tested.
- Editing targets are verified not to receive application shortcut interception.
- Shortcut-help resolution is tested.

### Rust property tests

`crates/sortsmith-core/tests/properties.rs` now uses Proptest to exercise generated cases for:

- size-range criterion JSON serialization/deserialization round trips;
- safe relative subdirectories remaining below the supplied root;
- parent traversal inputs remaining rejected;
- generated portable ASCII filenames passing portable filename validation;
- generated rename templates preserving `.txt` extension behavior while staying under the root.

These property tests run through the normal core test suite.

### Performance benchmarks

`crates/sortsmith-core/benches/planning.rs` now provides Criterion targets for:

- organization planning across synthetic fixture sets of 100, 1,000, and 5,000 files;
- duplicate hashing across 100 and 1,000 candidates grouped into repeated-content buckets.

The benchmark fixtures are temporary and isolated. They do not read personal directories. The intended comparison command is:

```bash
cargo bench -p sortsmith-core --bench planning
```

Timing results must be compared on the same machine/toolchain; shared CI timing is not treated as a stable performance guarantee. CI Clippy uses `--all-targets`, so the benchmark source is still compiled as part of the Rust quality path.

## Documentation updated in this continuation

- `README.md` now documents saved presets, keyboard quick actions, property coverage, Criterion benchmarks, and links the keyboard shortcut reference.
- `CHANGELOG.md` records saved presets, keyboard actions, property tests, benchmarks, accessibility changes, and watched-folder preset deletion protection.
- `ROADMAP.md` now marks keyboard quick actions, saved user preset management, property coverage, and benchmark foundations as complete without falsely marking additional curated built-in preset packs complete.
- `docs/performance.md` documents the repeatable benchmark targets and measurement rules.
- `docs/testing.md` documents property testing, shortcut/preset tests, and the benchmark verification command.
- `docs/development.md` documents benchmark workflow and the design boundaries for shortcut/preset changes.
- `docs/accessibility.md` documents keyboard-first interaction and remaining manual platform accessibility checks.
- `docs/keyboard-shortcuts.md` is the full shortcut/safety reference.
- This `what_changed.md` has been rewritten to reflect the actual current repository instead of older limitations that had already been completed by later commits.

## Current repository quality/release automation

- GitHub Actions CI separates core Rust, desktop Rust/Tauri, and frontend jobs.
- Rust CI checks formatting, Clippy, core tests, desktop host checks/Clippy/tests, and now compiles property/benchmark targets through the configured test/all-target paths.
- Frontend CI installs dependencies, typechecks, runs Vitest, and produces a Vite build.
- CodeQL covers TypeScript and Rust.
- Dependabot is configured.
- Tag-driven release automation targets Windows, macOS, and Linux.
- Release metadata verification prevents a version tag from silently disagreeing with Cargo, frontend package metadata, or Tauri configuration.

## Dependency reproducibility status

The repository still does **not** contain:

- `Cargo.lock`
- `apps/desktop/package-lock.json`

Both paths were explicitly checked on 2026-08-24 and GitHub returned them as absent. This remains a release blocker. They must be generated in a trusted networked Rust/Node environment, reviewed, committed, and then CI/release installation should move to lockfile-enforcing commands such as `cargo ... --locked` and `npm ci`.

Do not hand-author either lockfile. They must be produced by the corresponding package manager from the committed manifests.

## Verification limitations of this continuation environment

- Repository reads/writes and commits were performed through the connected GitHub integration with push/admin access to `sanskarIN/sortsmith`.
- This continuation did not obtain a local checkout with working dependency resolution, so it does not claim a locally executed Rust compile, Clippy pass, Vitest pass, frontend build, Tauri build, installer build, or Criterion timing result.
- The source and workflow definitions were reviewed directly in the repository, but release readiness must be based on actual GitHub Actions/platform build results rather than inferred success.
- Signing/notarization cannot be completed safely without platform credentials stored outside Git.
- Real release screenshots and clean installer smoke tests require verified platform builds.

## Remaining work / next exact tasks

1. Inspect the newest GitHub Actions run for the latest `main` commit and fix every real compile, lint, test, or build failure reported by the runners. Do not mark this complete until the required checks are actually green.
2. From a trusted networked environment with the supported Rust/Node toolchains, generate and commit `Cargo.lock` and `apps/desktop/package-lock.json`.
3. Change frontend CI/release installation from `npm install` to `npm ci` after the npm lockfile exists, and use Cargo `--locked` in release-sensitive commands after `Cargo.lock` exists.
4. Run `cargo bench -p sortsmith-core --bench planning` on a consistent development machine and record a baseline before implementing the incremental scan cache.
5. Design and implement the incremental scan cache with explicit invalidation rules; verify its correctness against the uncached planner and demonstrate a benchmark improvement before enabling it by default.
6. Add additional curated built-in preset packs only after their behavior is validated. Saved user-defined preset management is already implemented.
7. Expand integration/accessibility testing on Windows, macOS, and Linux, including keyboard shortcut conflicts, modal focus behavior, native dialogs, permissions, symlinks, Unicode paths, long paths where supported, and installer launch/uninstall behavior.
8. Implement native background scheduling only with explicit platform consent, clear startup behavior, and least-privilege filesystem access. The current watched-folder scheduler intentionally runs only while SortSmith is open.
9. Capture real release screenshots from verified builds and replace the placeholder screenshot state.
10. Configure signing/notarization credentials outside Git, run clean-checkout installer smoke tests on every distributed platform, and only then create the first release tag.

## Known limitations that are intentionally still open

- Watched folders are app-lifetime automation, not native OS background scheduling.
- Incremental scan caching is not implemented yet.
- Additional curated built-in preset packs are not implemented yet; user-created saved presets are implemented.
- Cross-platform accessibility/integration coverage still needs real platform execution.
- Real release screenshots are not yet captured.
- Signing/notarization and clean installer smoke tests are not yet completed.
- Dependency lockfiles are not yet committed.
- The first public release tag must not be treated as ready until the required CI/platform evidence is green.

## Obsolete limitations removed from the old handoff

The previous handoff still claimed several items were missing even though subsequent repository work had already implemented them. Those stale notes have now been removed. The current repository already contains:

- native folder picker support;
- native settings import/export dialogs;
- journal-history/selective undo UI;
- hardened execution/journal preflight behavior;
- current persistence/security documentation.

## Latest continuation commits (2026-08-24)

1. `aacc4699d45ea6068dcc3105a52b4031ceaf55eb` — `feat(frontend): add preset management helpers`
2. `d64123991d0bbe6768c3751c2ca03be9cc00d27a` — `test(frontend): cover preset helper behavior`
3. `d4f1c1b7b5c9d6784aa35d8748fe9af8829dbc10` — `feat(frontend): add saved preset manager`
4. `70278792e04911289bff4f2dac0ea04f41da9b94` — `feat(frontend): integrate saved preset workflows`
5. `5dd38d3210dceebd05502b46ef3a8a22d1dd55e7` — `build(core): add property and benchmark dependencies`
6. `957d6ccc210038733185299ceef3ed099298e3c4` — `build(core): register property tests and benchmarks`
7. `14253da376420d7d74c3092e6969201536d51b77` — `test(core): add serialization and path property coverage`
8. `e883ebce2270229f7d5aedc5d47f9ac0f4e52a85` — `test(core): keep generated filenames cross-platform safe`
9. `0e7b2721d7ea6b5ee1feaf72ed1245dd0c55f7e7` — `perf(core): benchmark planning and duplicate hashing`
10. `69ef05c48ab7d89d1720cd971d9bcc0cdfa66f66` — `feat(frontend): define keyboard quick actions`
11. `12ce3a9137a45fed84c259909439309cb6098210` — `test(frontend): cover keyboard quick actions`
12. `3a03fa72df648b50fd5f682a15784017e64a60d7` — `feat(frontend): add keyboard shortcut controller`
13. `aa28b421a401b8568e391bd5085e43c9d0e799b0` — `style(frontend): add shortcut help dialog`
14. `654de0e62e25b26751a7f6b686712e322803251d` — `style(frontend): load shortcut dialog styles`
15. `40e99645ea0d80d4bfc608217a4e534505cbccca` — `feat(frontend): wire keyboard-first organizer actions`
16. `93597e2c49a1d42ce495d37206b30534438840b7` — `docs(performance): document repeatable benchmark targets`
17. `22bd22f10bcc4d99723e83b6a119668ca7ed5daf` — `docs(testing): add property and benchmark verification`
18. `41aa7a9f80571bd6dc6263a38538aa9754e0f835` — `test(frontend): assert preset snapshots are independent`
19. `f15e5cdde8583a05799d82bbaa4d05fb2357af95` — `docs(roadmap): mark delivered polish and verification work`
20. `d3ed45ddd67461437591ece04a60c3c934842940` — `docs(changelog): record presets shortcuts properties and benchmarks`
21. `fb9299417cc7aaac773bfd4342af93c6aa6bc632` — `docs(readme): expose presets shortcuts and benchmarks`
22. `7b85fdafaa67f34b41b8a0474c5d9f0f53597018` — `docs(frontend): document keyboard quick actions`
23. `922d6e4d4bd1678f84a6b7d5d496e9f8a7ddface` — `docs(accessibility): cover keyboard-first interaction`
24. `44193eec6f755285a9f86da1608141c798693017` — `docs(development): add benchmark and shortcut workflow`
25. `d58d6ec9020226b35a4a0603d15f0f9145613955` — `docs(readme): link keyboard shortcut reference`

## Continuation rule

Before the next code change, read this file, `ROADMAP.md`, the latest `main` commits, and the latest GitHub Actions results. Prefer fixing demonstrated CI/platform failures before adding another release feature. Keep commits small and reviewable, keep filesystem behavior local-first/reversible, and update this file again after the next meaningful batch.
