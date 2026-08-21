# Performance

Performance-sensitive work is kept in the Rust core. WalkDir streams directory entries rather than materializing every path first. Duplicate detection groups by size before hashing and hashes candidate files in parallel using Rayon with 1 MiB buffered reads.

Initial budgets for a release candidate on a typical modern laptop:
- UI remains responsive during ordinary filesystem commands.
- Preview should avoid holding file contents in memory.
- Duplicate hashing memory should scale with worker count, not total folder size.

Measure before optimizing. Profiling should add Criterion-style benchmarks for rule matching and representative large-folder scans before introducing caches; any cache must define explicit invalidation behavior.
