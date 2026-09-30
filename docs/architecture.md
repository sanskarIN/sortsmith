# Architecture

SortSmith is a modular monolith with three runtime boundaries plus repository-level verification tooling.

1. **Core domain (`crates/sortsmith-core`)** — platform-neutral Rust models, bundled default presets, rule validation/evaluation, directory scans, incremental preview caching, duplicate hashing, execution, collision handling, and undo journals. It does not depend on Tauri.
2. **Desktop adapter (`apps/desktop/src-tauri`)** — validates UI/import inputs, resolves the app-data directory, persists bounded state atomically, protects filesystem command boundaries, owns the process-local interactive preview cache, maintains privacy-safe rotating operation logs, and exposes a narrow Tauri command API.
3. **Frontend (`apps/desktop/src`)** — React presentation and user interaction. It never performs direct filesystem operations; all privileged work crosses typed Tauri commands. It also owns backward-compatible presentation-layer normalization for the legacy random bundled-preset identifier so existing user state can be upgraded without a breaking schema change.
4. **Repository verification (`scripts` + `.github/workflows`)** — release metadata synchronization, release lockfile validation, CI quality gates, CodeQL, dependency updates, and cross-platform draft packaging.

## Filesystem mutation boundary

Filesystem mutations occur only after a preview has been returned to the user. The desktop adapter re-checks that preview paths stay inside the canonical selected root before execution and rejects redirection through existing symlinks outside that root. Normal scans do not follow links. Collision-safe destination selection is repeated at execution time so a newly occupied target is not overwritten.

Undo journals are saved separately from preferences so an interrupted settings write cannot erase undo history. Journal JSON is streamed through buffered I/O and committed by rename. Undo replays entries in reverse order and refuses to overwrite a path that has become occupied.

## Incremental preview-cache boundary

The 0.3 preparation branch adds an in-memory cache for repeated interactive previews. The cache is an optimization only; it is not a source of truth and it is not persisted to disk.

A cache scope is exact across the selected root, complete ordered rule set, and scan options. Changing any of those inputs clears cached entries before planning continues. Each file entry is keyed by path and is reusable only while its size and modification timestamp still match the current filesystem metadata. New, moved, renamed, or changed files are rebuilt through the normal file-description path, and entries for files no longer encountered are pruned after each successful scan.

Cached decisions do not bypass filesystem safety:

- destination construction still runs from the current file entry;
- collision-safe destination resolution runs on every preview, including cache hits;
- `ModifiedOlderThanDays` rules are re-evaluated on every reused file because their result can change as wall-clock time advances;
- execute and undo clear the desktop cache before filesystem mutation so partial failures cannot leave an apparently valid pre-mutation cache behind;
- watched-folder execution also clears the interactive cache before applying changes;
- if the cache mutex is unavailable/poisoned, interactive preview falls back to the existing uncached planner rather than weakening validation or blocking the feature.

The current cache still walks the selected directory and reads lightweight filesystem metadata for each file. It primarily avoids rebuilding unchanged file descriptions and repeating static rule matching. No performance claim is valid until the paired uncached/warm-cache Criterion groups are measured on the same machine.

## Local state boundary

State schema version `1` remains compatible in 0.2 and contains preferences, custom rules, presets, watched folders, and recent journal identifiers. Rust validates limits, unique identifiers, rule syntax, preset references, themes, intervals, and path fields before accepting persisted/imported state. State/import/export JSON is bounded to 16 MiB. Local state refuses symlink-backed `state.json` files, and temporary state writes are flushed and synced before replacement.

The 0.2 bundled-preset migration deliberately does **not** require schema version `2`: stable bundled preset UUIDs are normal preset identifiers within the existing schema. The frontend recognizes the legacy `Everyday tidy` preset only when the new stable ID is absent, preserves its rules, remaps watched-folder references, and adds missing bundled packs while respecting the 50-preset state limit. The same normalization runs before state writes, so imported legacy backups are upgraded immediately.

Portable settings exports omit recent journal identifiers at the UI boundary because journal files are machine-local and are not part of a portable settings backup.

The 0.3 incremental preview cache does not change the persisted state schema because it is process-local and disposable.

## Logging boundary

Structured operation logs contain only timestamp, event type, journal identifier, completed count, and error count. They deliberately exclude file paths and contents. The active JSONL log rotates at 5 MiB, retains one previous segment, and refuses symlink/non-file targets.

## Automation boundary

Watched-folder automation runs only while SortSmith is open in version 0.2. A watch resolves an existing folder, obtains rules from its referenced preset, previews with link following disabled, executes reversibly, records a journal, and updates its last-run time. Native background scheduling is intentionally deferred until platform-specific startup, permission, and consent behavior can be implemented and tested.

The first 0.3 scale work optimizes repeated interactive previews only. Native background scheduling remains a separate platform-consent feature and is not implied by the preview cache.

## Release boundary

Source version `0.2.0` on `main` is a prepared candidate, not proof of release readiness. Ordinary CI checks that Cargo, frontend, and Tauri versions agree. Tag-driven packaging additionally requires package-manager-generated `Cargo.lock` and `apps/desktop/package-lock.json`, validates Cargo resolution with `--locked`, and installs frontend dependencies with `npm ci`. Draft artifacts still require real platform smoke tests, accessibility checks, screenshots, and signing/notarization decisions before publication.

The `develop/0.3.0` branch is intentionally isolated from that candidate until 0.2 release validation is resolved. Its code must pass the same Rust/desktop/frontend gates before it can be considered mergeable.
