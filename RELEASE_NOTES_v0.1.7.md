# SortSmith v0.1.7 — Patch Release

SortSmith v0.1.7 is a stable maintenance release for the 0.1.x line. It hardens the in-memory cached preview path so cached previews preserve the same filesystem safety and collision-planning guarantees as the non-cached preview engine.

## Highlights

### Cached preview symlink containment

The cached organization preview now applies the selected-root boundary while traversing with `follow_links` enabled. Symbolic-link files and directories whose resolved targets are outside the selected root are pruned before cached metadata is reused or new metadata is collected.

This closes a consistency gap where the normal preview path already rejected external symlink traversal, while the performance-oriented cached path could otherwise inspect entries outside the selected root.

### Cached preview collision reservation

Cached preview planning now reserves destinations already assigned earlier in the same preview and uses the same collision-safe reservation primitive as the normal planner.

Two files with the same name discovered from different source directories therefore cannot receive the same destination merely because the preview was served through the cache.

### Regression coverage

The release adds focused cache regressions for:

- external symbolic-link directory traversal with `follow_links` enabled;
- duplicate source filenames converging on one destination folder;
- continued collision recomputation when a cached file is reused.

Existing cache invalidation, rule-scope, deletion-pruning, time-sensitive-rule, and full-rescan coverage remains in place.

## Security impact

The cached preview path now follows the same selected-root traversal boundary as the primary preview implementation. No automatic deletion behavior is introduced, and execution/undo safety remains enforced independently of preview planning.

## Compatibility

No intentional breaking public API change is introduced by this patch release. The scan cache remains an in-memory optimization and does not change the persisted journal format.

## Version synchronization

The following release metadata is synchronized to `0.1.7`:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

## Validation

Before publishing the tag, run:

```bash
node scripts/verify-release-version.mjs v0.1.7
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

For the desktop application, also run:

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

The Unix external-symlink regression should execute on Unix-like CI. The full workspace suite should be run on Linux, Windows, and macOS before publishing production installers.

## Release metadata

- **Version:** `0.1.7`
- **Tag:** `v0.1.7`
- **Target branch:** `release/0.1.7`
- **Release title:** `SortSmith v0.1.7 — Patch Release`
- **Pre-release:** No
- **Latest:** No
- **License:** Apache-2.0

## Release status

Repository-side preparation includes the cached-preview safety fix, collision regression coverage, synchronized version metadata, changelog, release notes, checklist, and handoff updates. Local Rust/Node/Tauri execution has not been claimed as passed in this environment. Publish the tag only after the validation gates pass in CI or on the release machine.
