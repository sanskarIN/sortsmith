# SortSmith v0.1.2 — Patch Release

SortSmith v0.1.2 is a focused maintenance release on the `0.1.x` line. It improves journal replacement behavior and adds defense-in-depth validation to core undo operations.

## Highlights

- Safer replacement of existing operation-journal snapshots across platforms.
- Core undo path validation before any filesystem mutation.
- Regression coverage for both durability and security boundaries.
- Rust, desktop, and Tauri version metadata synchronized to `0.1.2`.

## Fixed

### Existing journal replacement

Saving a journal more than once can target the same journal filename. Some platforms do not allow a rename operation to overwrite an existing file. SortSmith now handles the existing-target case explicitly after the temporary journal has been flushed and synchronized.

This keeps the normal temporary-file workflow while making repeated journal snapshots work consistently across supported platforms.

### Undo path containment

The core `undo_journal` API now validates every recorded `from` and `to` path against the journal's recorded root before beginning the undo loop.

This is defense in depth: the desktop/Tauri boundary already validates journal paths, and the core API now protects callers that invoke the Rust library directly as well.

A forged journal containing an external path is rejected before the external file can be moved.

## Tests

Added regression tests for:

- replacing an existing journal snapshot and loading the newest version;
- rejecting a forged journal whose recorded paths are outside its root;
- preserving the existing preview, execute, undo, and filesystem-safety behavior.

## Compatibility

- Rust workspace version: `0.1.2`
- Desktop package version: `0.1.2`
- Tauri application version: `0.1.2`
- License: Apache-2.0
- Supported desktop targets remain Windows, macOS, and Linux.

## Verification

Run from a clean checkout of the `release/0.1.2` branch:

```bash
node scripts/verify-release-version.mjs v0.1.2
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings

cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

Then run:

```bash
cd ../..
git diff --check
git status --short
```

## Release gate

Do not publish the tag until the local quality suite passes. After tagging `v0.1.2`, allow the tag-driven release workflow to build Windows, macOS, and Linux artifacts. Review the resulting draft release and smoke-test the generated packages on clean target systems before publishing.

## Recommended GitHub Release fields

- Tag: `v0.1.2`
- Release title: `SortSmith v0.1.2 — Patch Release`
- Target: `release/0.1.2`
- Pre-release: enabled if this maintenance line is still being distributed as an early preview
- Release notes: use this file as the release body

## Notes

This release remains intentionally scoped to patch-level maintenance. Larger organizer features remain on the later development line and are not backported into `0.1.x` merely to increase the patch release size.
