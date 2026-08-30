# ADR 0004 — Process-local incremental preview cache

- Status: Accepted for the 0.3 development line
- Date: 2026-08-25

## Context

Repeated previews over large folders can spend time rebuilding the same file descriptions and evaluating the same static rule set even when most files have not changed. SortSmith also has strict safety requirements: a performance optimization must not turn cached data into an authority for filesystem mutation, must not hide new destination collisions, and must not make time-based rules stale.

A persistent index would add a new on-disk format, migration/versioning requirements, corruption recovery, privacy/storage considerations, and difficult cross-platform invalidation behavior before the project has measured whether that complexity is necessary.

## Decision

SortSmith 0.3 begins with a process-local, in-memory preview cache owned by the desktop runtime and implemented by the platform-neutral core crate.

The cache scope is exact across:

- canonical selected root;
- complete ordered rule set;
- scan options.

Changing any scope input clears the cache.

Within a stable scope, a cached file description is reusable only when the same path is still present and its current size and modification timestamp match the cached values. Missing entries are pruned after scanning. New, renamed, moved, or changed files are rebuilt normally.

The cache may reuse the index of the first matching static rule, but it must re-evaluate rules containing `ModifiedOlderThanDays` because their result can change as time advances without a metadata change.

Destination rendering and collision-safe destination selection are deliberately not cached. They run during every preview so external destination changes are observed.

The desktop host clears the interactive cache before execute, undo, and watched-folder filesystem mutation. If the cache lock is unavailable, preview falls back to the existing uncached planner.

The cache is not serialized into `state.json`, exported settings, journals, or any other persistent file.

## Consequences

### Positive

- Repeated previews can avoid rebuilding unchanged file descriptions and repeating static rule matching.
- Existing preview/execution data contracts stay unchanged.
- Restarting the application guarantees a clean cache without migration work.
- Cache corruption cannot persist across launches.
- Existing filesystem safety and collision checks remain authoritative.

### Trade-offs

- Directory traversal and lightweight metadata reads still occur on every scan.
- A single cache scope means switching roots/rule sets/options discards the previous warm state.
- Watched folders do not yet receive independent per-root warm caches.
- Performance gains must be measured; they are not assumed.

## Rejected alternatives

### Persistent scan index in 0.3

Rejected for the first cache iteration because it introduces storage format/versioning, corruption recovery, and invalidation complexity before same-machine benchmarks establish need.

### Cache final planned destinations

Rejected because destination occupancy can change independently of source metadata. Reusing a final destination could hide a newly created collision.

### Cache time-sensitive rule decisions

Rejected because a file can cross an age threshold while its path, size, and modification timestamp remain unchanged.

### Filesystem watcher as the only invalidation mechanism

Deferred. Native watcher semantics, event loss/coalescing, permissions, rename behavior, and platform differences require separate design and testing. The current metadata validation remains self-contained and does not depend on watcher reliability.

## Follow-up requirements

Before broadening the cache:

1. Keep cache correctness tests green across Rust CI.
2. Record uncached and warm-cache Criterion results on the same machine/toolchain.
3. Define a performance budget from measured baselines rather than shared CI timing.
4. Evaluate per-root cache partitioning only if multi-folder workflows show measurable benefit.
5. Introduce persistent indexing only through a new ADR that defines format versioning, bounded storage, invalidation, corruption recovery, and privacy behavior.
