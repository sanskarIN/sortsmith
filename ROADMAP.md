# Roadmap

## 0.1 — Safe organizer baseline
- [x] Rule engine and dry-run preview.
- [x] Reversible execution journal.
- [x] Duplicate candidate detection.
- [x] Presets, basic custom rule editor, watched-folder scheduling while app is open.
- [x] Theme/accessibility baseline and documentation.
- [x] CI, CodeQL, dependency update configuration, and cross-platform release workflow.
- [ ] Complete first green cross-platform CI run and clean installer smoke tests before tagging the release.

## 0.2 — Desktop polish
- [ ] Native folder picker and richer all-criteria rule builder.
- [ ] Journal history UI with selective undo.
- [ ] Native import/export controls in Settings.
- [ ] Signed/notarized installers and real release screenshots.
- [ ] Expanded integration and accessibility tests on all supported operating systems.

## 0.3 — Scale and automation
- [ ] Native background scheduling where each platform can provide safe, explicit user consent.
- [ ] Incremental scan cache for very large folders, backed by benchmarks and explicit invalidation rules.
- [ ] More preset packs and multi-criterion rule composition UX.
- [ ] Additional fuzz/property coverage for rule serialization, templates, and path edge cases.
