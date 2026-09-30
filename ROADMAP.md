# Roadmap

## 0.1 — Safe organizer baseline
- [x] Rule engine and dry-run preview.
- [x] Reversible execution journal.
- [x] Duplicate candidate detection.
- [x] Presets, full metadata rule editor, watched-folder scheduling while app is open.
- [x] Native folder picker, native settings backup/restore, and selectable undo history.
- [x] Theme/accessibility baseline and documentation.
- [x] CI, TypeScript/Rust CodeQL, dependency update configuration, and cross-platform release workflow.
- [x] Portable rename validation, bounded durable state writes, streamed journals, and bounded privacy-safe operation logs.
- [ ] Complete first green cross-platform CI run and clean installer smoke tests before tagging a public release.

## 0.2 — Desktop polish
- [x] Native folder picker and richer all-criteria rule builder.
- [x] Journal history UI with selective operation undo.
- [x] Native import/export controls in Settings.
- [x] Keyboard-first quick actions for high-frequency organizer workflows, with an in-app shortcut reference and editing-safe key handling.
- [x] Shortcut-dialog focus handoff/restoration and semantic dialog labelling.
- [x] Stable bundled preset identities plus backward-compatible migration of the legacy Everyday tidy identifier and watched-folder references.
- [x] Curated built-in preset packs for everyday files, media, developer staging folders, and downloads.
- [x] Release-candidate version metadata aligned at `0.2.0` with continuous metadata-sync verification.
- [x] Fail-closed release workflow requiring committed/aligned npm and Cargo lockfiles before packaging.
- [ ] Generate and commit `Cargo.lock` and `apps/desktop/package-lock.json` from a trusted networked toolchain environment.
- [ ] Signed/notarized installers and real release screenshots.
- [ ] Expanded integration and accessibility tests on all supported operating systems.
- [ ] Complete clean installer smoke tests on Windows, macOS, and Linux before publishing `v0.2.0`.

## 0.3 — Scale and automation
- [ ] Native background scheduling where each platform can provide safe, explicit user consent.
- [x] Implement process-local incremental preview caching with exact root/rule/options scoping, file metadata invalidation, deletion pruning, and time-sensitive rule revalidation.
- [x] Integrate cached interactive previews into the Tauri desktop host with uncached fallback and mutation-time invalidation.
- [x] Add repeatable warm-cache Criterion coverage alongside the existing uncached planning benchmark.
- [ ] Establish same-machine uncached versus warm-cache measurements and define a performance budget before expanding cache behavior.
- [ ] Decide whether a persistent cache is justified; if so, design explicit versioning, bounded storage, corruption recovery, and invalidation rules before implementation.
- [x] Saved user-defined preset management with load, metadata editing, guarded deletion, and watched-folder reference protection.
- [x] Additional curated built-in preset packs with stable identifiers and compatibility migration.
- [x] Property coverage for rule serialization, rename templates, portable filenames, and path traversal edge cases.
- [x] Repeatable Criterion benchmarks for organization planning and duplicate hashing on representative synthetic directory trees.
- [ ] Complete 0.3 branch CI/CodeQL validation and add platform integration coverage for repeated preview/apply/undo cycles.

## 0.4 — Candidate follow-on work
- [ ] Evaluate per-root cache partitioning if multiple watched folders need independent warm caches.
- [ ] Evaluate native filesystem-change notifications only after cross-platform correctness and permission behavior are documented.
- [ ] Revisit persistent indexing only if measured 0.3 performance shows the in-memory cache is insufficient for target folder sizes.
