# ADR 0002: Versioned JSON state and separate undo journals

**Status:** Accepted — 2026-08-21

## Context
SortSmith needs lightweight local persistence but does not yet require relational queries or multi-user concurrency.

## Decision
Store one schema-versioned JSON state document plus one JSON file per execution journal using atomic temp-file replacement.

## Consequences
Backups and inspection remain simple. Migrations must be explicit when `schema_version` changes. A database can be introduced later only if query complexity justifies it.
