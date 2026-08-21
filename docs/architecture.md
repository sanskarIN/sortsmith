# Architecture

SortSmith is a modular monolith with three boundaries.

1. **Core domain (`crates/sortsmith-core`)** — pure Rust models, rule evaluation, scans, duplicate hashing, execution, collision handling, and undo journals. It does not depend on Tauri.
2. **Desktop adapter (`apps/desktop/src-tauri`)** — validates UI inputs, resolves the app-data directory, persists state atomically, invokes core operations, and exposes a narrow Tauri command API.
3. **Frontend (`apps/desktop/src`)** — React presentation and user interaction. It never performs direct filesystem operations.

Filesystem mutations occur only after a preview has been returned to the user. The desktop adapter re-checks that preview paths stay inside the canonical selected root before execution and rejects redirection through existing symlinks outside that root. Journals are saved separately from preferences so an interrupted settings write cannot erase undo history.
