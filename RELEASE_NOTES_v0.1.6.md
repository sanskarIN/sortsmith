# SortSmith v0.1.6 — Patch Release

SortSmith v0.1.6 is a stable maintenance release that strengthens filesystem portability and regression coverage on the 0.1.x maintenance line.

## Highlights

### Public API-level symlink safety regression

v0.1.5 introduced traversal-time pruning for symbolic-link directories whose resolved targets are outside the selected organization root. v0.1.6 adds an integration test through the public `sortsmith-core` API so this security boundary is protected by externally observable behavior rather than only an internal unit test.

The regression scenario creates an external directory containing a nested matching file, links to that directory from inside the selected root, enables recursive scanning with link following, and verifies that no organization operation is planned and the external file is not scanned or modified.

### Unicode Windows device-name protection

Filename validation now rejects the Unicode superscript aliases recognized by Windows for numbered `COM` and `LPT` device names, such as `COM¹` and `LPT²`.

This closes a portability edge case where a rendered rename could pass ordinary ASCII reserved-name checks while still mapping to a Windows device name.

## Why this matters

Filesystem safety and cross-platform filename validation are security-sensitive boundaries. v0.1.6 makes the v0.1.5 traversal guarantee observable through the public API and closes a Windows filename portability edge case.

## Compatibility

No intentional breaking API change is introduced by this patch release. Existing valid filenames and organization rules continue to work normally.

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
```

For the desktop application, also run the normal typecheck, frontend tests, build, and Tauri build gates documented in `docs/release-v0.1.6-checklist.md`.

## Release metadata

- **Version:** `0.1.6`
- **Tag:** `v0.1.6`
- **Target branch:** `release/0.1.6`
- **Release title:** `SortSmith v0.1.6 — Patch Release`
- **Pre-release:** No
- **Latest:** No
- **License:** Apache-2.0

## Upgrade guidance

Users on the 0.1.x maintenance line can upgrade to v0.1.6 after the published release artifacts have been reviewed. This release is particularly useful for workflows using recursive symbolic links and for users who need portable filename validation across Windows and Unix-like systems.

## Release status

Repository-side preparation is complete after the implementation, regression coverage, synchronized metadata, changelog, release notes, checklist, and handoff updates are committed. Local builds and cross-platform artifact validation must still pass before the `v0.1.6` tag is published.
