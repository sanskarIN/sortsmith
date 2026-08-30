# ADR 0002: Versioned JSON state and separate undo journals

**Status:** Accepted — 2026-08-21; hardened 2026-08-22

## Context

SortSmith needs lightweight local persistence but does not yet require relational queries or multi-user concurrency. File organization must remain reversible even when preferences are damaged or a settings write is interrupted.

## Decision

Store one schema-versioned JSON state document plus one JSON file per execution journal.

- State and journals are serialized through buffered JSON I/O to avoid duplicate whole-document byte buffers.
- Temporary files are flushed and synced before atomic replacement-style renames.
- State/import/export documents are bounded to 16 MiB and validated before becoming active.
- Saved state and imported files must be regular files rather than symlinks.
- Each execution creates an empty journal successfully before the first filesystem mutation, then replaces it with the completed journal after processing.
- Journals remain separate from preferences so a failed settings write cannot remove undo evidence.

## Consequences

Backups and inspection remain simple. Migrations must be explicit when `schema_version` changes. Execution refuses to begin if its journal storage cannot be initialized. A database can be introduced later only if query complexity, migration needs, or crash-recovery requirements justify it.

The current journal format is a complete JSON document rather than a write-ahead log. A sudden process/OS failure between an individual move and the final completed-journal replacement can still leave the preflight journal without that last operation. A future crash-consistent append/WAL design should be introduced only with a versioned migration and recovery tests rather than silently changing the format.
