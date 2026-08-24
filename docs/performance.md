# Performance

Performance-sensitive work is kept in the Rust core. WalkDir streams directory entries rather than materializing every path first. Hidden directories are pruned during traversal when hidden files are disabled, avoiding unnecessary work. Duplicate detection groups by size before hashing and hashes candidate files in parallel using Rayon with 1 MiB buffered reads.

Undo journals are serialized and parsed through buffered streams instead of allocating an additional whole-document byte buffer. Desktop settings use buffered JSON I/O with a 16 MiB storage contract, and structured operation logs rotate at 5 MiB so long-running installations do not grow one unbounded log file.

## Repeatable benchmarks

`crates/sortsmith-core/benches/planning.rs` contains Criterion benchmarks for the two filesystem-heavy core paths that need regression visibility:

- organization planning over 100, 1,000, and 5,000 representative files;
- duplicate candidate hashing over 100 and 1,000 same-size candidates grouped into repeated-content buckets.

Run them from the repository root with:

```bash
cargo bench -p sortsmith-core --bench planning
```

The benchmark fixtures live only in isolated temporary directories. They do not read personal folders, require network access, or mutate any path outside their temporary fixture root. Treat measurements as machine-specific: compare changes on the same machine and toolchain rather than publishing one machine's timing as a universal guarantee.

Initial budgets for a release candidate on a typical modern laptop:
- UI remains responsive during ordinary filesystem commands.
- Preview should avoid holding file contents in memory.
- Duplicate hashing memory should scale with worker count, not total folder size.
- Journal and settings persistence should scale with serialized record size without duplicate full-file buffers.
- Operation-log storage remains bounded to the active log plus one previous segment.

Measure before optimizing. The current Criterion targets establish a baseline before an incremental scan cache is introduced; any future cache must define explicit invalidation behavior and demonstrate a measurable win against these uncached benchmarks.
