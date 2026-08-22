# Performance

Performance-sensitive work is kept in the Rust core. WalkDir streams directory entries rather than materializing every path first. Hidden directories are pruned during traversal when hidden files are disabled, avoiding unnecessary work. Duplicate detection groups by size before hashing and hashes candidate files in parallel using Rayon with 1 MiB buffered reads.

Undo journals are serialized and parsed through buffered streams instead of allocating an additional whole-document byte buffer. Desktop settings use buffered JSON I/O with a 16 MiB storage contract, and structured operation logs rotate at 5 MiB so long-running installations do not grow one unbounded log file.

Initial budgets for a release candidate on a typical modern laptop:
- UI remains responsive during ordinary filesystem commands.
- Preview should avoid holding file contents in memory.
- Duplicate hashing memory should scale with worker count, not total folder size.
- Journal and settings persistence should scale with serialized record size without duplicate full-file buffers.
- Operation-log storage remains bounded to the active log plus one previous segment.

Measure before optimizing. Profiling should add Criterion-style benchmarks for rule matching, duplicate hashing, and representative large-folder scans before introducing caches; any cache must define explicit invalidation behavior.
