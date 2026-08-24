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
- [ ] Complete first green cross-platform CI run and clean installer smoke tests before tagging the release.

## 0.2 — Desktop polish
- [x] Native folder picker and richer all-criteria rule builder.
- [x] Journal history UI with selective operation undo.
- [x] Native import/export controls in Settings.
- [x] Keyboard-first quick actions for high-frequency organizer workflows, with an in-app shortcut reference and editing-safe key handling.
- [ ] Signed/notarized installers and real release screenshots.
- [ ] Expanded integration and accessibility tests on all supported operating systems.

## 0.3 — Scale and automation
- [ ] Native background scheduling where each platform can provide safe, explicit user consent.
- [ ] Incremental scan cache for very large folders, backed by benchmarks and explicit invalidation rules.
- [x] Saved user-defined preset management with load, metadata editing, guarded deletion, and watched-folder reference protection.
- [ ] Additional curated built-in preset packs after rule behavior is validated against representative user workflows.
- [x] Property coverage for rule serialization, rename templates, portable filenames, and path traversal edge cases.
- [x] Repeatable Criterion benchmarks for organization planning and duplicate hashing on representative synthetic directory trees.
