# SortSmith v0.1.6 — Patch Release

SortSmith v0.1.6 is a stable maintenance release that strengthens filesystem safety, crash recovery, duplicate scanning, automation reliability, and cross-platform filename handling on the 0.1.x maintenance line.

## Highlights

### Public API-level symlink safety regression

v0.1.5 introduced traversal-time pruning for symbolic-link directories whose resolved targets are outside the selected organization root. v0.1.6 adds an integration test through the public `sortsmith-core` API so this security boundary is protected by externally observable behavior rather than only an internal unit test.

The regression scenario creates an external directory containing a nested matching file, links to that directory from inside the selected root, enables recursive scanning with link following, and verifies that no organization operation is planned and the external file is not scanned or modified.

### Duplicate scanner containment

Duplicate scanning now applies the same selected-root containment rule when link following is enabled. External symbolic-link directories are pruned before traversal, preventing content outside the selected folder from being hashed as part of a duplicate scan.

### Unicode Windows device-name protection

Filename validation now rejects the Unicode superscript aliases recognized by Windows for numbered `COM` and `LPT` device names, such as `COM¹` and `LPT²`.

This closes a portability edge case where a rendered rename could pass ordinary ASCII reserved-name checks while still mapping to a Windows device name.

### Collision portability and race safety

Collision reservation is case-insensitive on Windows, matching the platform's path semantics. Generated collision names are also fitted to the portable 255-byte and 255-UTF-16-unit filename limits.

Execution no longer uses an overwriting `rename` as its final collision boundary. It uses a no-overwrite hard-link operation where supported, with a `create_new` streamed-copy fallback. If another process or user creates the planned destination after preview, SortSmith selects another collision-safe destination instead of replacing the newly-created file. Undo uses the same no-overwrite primitive.

### Durable undo checkpoints

Journal snapshots now normalize relative paths to absolute paths before serialization and synchronize the containing directory after atomic replacement on Unix-like systems.

During multi-file execution, the journal is checkpointed after each successfully completed move. An interrupted batch therefore retains the moves completed before the interruption instead of waiting for the final batch save.

### Watched-folder reliability

The desktop timer prevents overlapping background watch invocations, and the automation screen resynchronizes its preset selection after saved state loads or preset availability changes. The UI also enforces the backend's 100-watched-folder limit before attempting a save.

## Why this matters

Filesystem safety and cross-platform filename handling are security-sensitive boundaries for a file organizer. v0.1.6 closes several race and portability gaps while extending the existing symlink protections to duplicate scanning and making undo journals more durable during long-running batches.

## Compatibility

No intentional breaking API change is introduced by this patch release. Existing valid filenames and organization rules continue to work normally. The internal journal writer now normalizes relative paths when persisting snapshots; newly written journals therefore use absolute paths for reliable later validation.

## Version synchronization

The following release metadata is synchronized to `0.1.6`:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

## Validation

Before publishing the tag, run:

```bash
node scripts/verify-release-version.mjs v0.1.6
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

The Unix symlink and journal durability tests should run on Unix-like CI, while the Windows-specific reserved-name and case-insensitive collision tests should run on Windows CI.

## Release metadata

- **Version:** `0.1.6`
- **Tag:** `v0.1.6`
- **Target branch:** `release/0.1.6`
- **Release title:** `SortSmith v0.1.6 — Patch Release`
- **Pre-release:** No
- **Latest:** No
- **License:** Apache-2.0

## Upgrade guidance

Users on the 0.1.x maintenance line can upgrade to v0.1.6 after the published release artifacts have been reviewed. This release is particularly useful for workflows using recursive symbolic links, duplicate scanning, collision-heavy organization, long filenames, and reversible multi-file operations.

## Release status

Repository-side preparation includes the implementation fixes, regression coverage, synchronized metadata, changelog, release notes, checklist, and handoff updates. Local builds and cross-platform artifact validation have **not** been claimed as passed because this environment cannot execute the repository's Rust/Node/Tauri toolchains. The `v0.1.6` tag should be published only after the validation commands above pass in CI or on the owner's release machine.
