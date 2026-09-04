# SortSmith v0.1.3 — Patch Release

SortSmith v0.1.3 is a focused maintenance release on the 0.1.x line. It improves preview planning so collision-safe destinations are reserved across the entire preview, preventing multiple source files from being assigned the same destination when they converge on one folder.

## Highlights

- Preview planning now reserves destinations selected by earlier operations in the same preview.
- Recursive organization correctly handles duplicate filenames from different source directories.
- Collision-safe suffixes are now visible in the preview before the user applies the operation plan.
- Rust, desktop, and Tauri version metadata synchronized to 0.1.3.

## Fixed

### Duplicate destination planning

Previously, collision detection considered the filesystem but did not reserve destinations already selected by earlier operations in the same preview. When two files from different directories had the same filename and a rule moved both into the same destination folder, the preview could assign the same destination to both operations.

The preview planner now maintains an in-memory set of reserved destinations and uses the collision-safe allocator with that set. The resulting preview contains unique destinations before execution begins.

This keeps the dry-run preview faithful to the operation plan that will actually be applied and reduces ambiguity for users reviewing a large organization before execution.

## Tests

Added regression coverage that:

- creates identical filenames in separate source directories;
- runs recursive organization;
- verifies both files receive distinct planned destinations;
- verifies the second destination receives the expected collision suffix.

Existing coverage for journal replacement, undo-path containment, Unicode rule limits, traversal rejection, portable filenames, and reversible operations remains in place.

## Compatibility

- Rust workspace version: `0.1.3`
- Desktop package version: `0.1.3`
- Tauri application version: `0.1.3`
- License: Apache-2.0
- Supported desktop targets: Windows, macOS, and Linux.

## Verification

Run from a clean checkout:

```bash
node scripts/verify-release-version.mjs v0.1.3
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings

git diff --check

cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

For packaging, also run:

```bash
npm run tauri build
```

## Release

**Tag:** `v0.1.3`

**Release title:** `SortSmith v0.1.3 — Patch Release`

**Target:** `release/0.1.3`

The release should only be published after the local quality suite, GitHub Actions builds, generated artifacts, and clean-machine installer tests have been reviewed.

## Scope

This release remains intentionally patch-level. Larger feature work continues on the modern development line and is not backported into the 0.1.x maintenance branch.
