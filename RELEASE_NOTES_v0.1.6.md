# SortSmith v0.1.6 — Patch Release

SortSmith v0.1.6 is a stable maintenance release that strengthens regression coverage around recursive symbolic-link traversal safety and keeps the 0.1.x maintenance line independently verifiable.

## Highlights

### Public API-level symlink safety regression

v0.1.5 introduced traversal-time pruning for symbolic-link directories whose resolved targets are outside the selected organization root. v0.1.6 adds an integration test through the public `sortsmith-core` API so this security boundary is protected by externally observable behavior rather than only an internal unit test.

The regression scenario creates an external directory containing a nested matching file, links to that directory from inside the selected root, enables recursive scanning with link following, and verifies that:

- no organization operation is planned;
- the nested external file is not counted as scanned;
- the external file remains untouched.

The test is Unix-specific because it exercises the platform symbolic-link API directly.

## Why this matters

Filesystem-boundary protections are security-sensitive and can be weakened accidentally during future refactoring. Keeping a public API-level regression test makes the intended contract explicit: recursive preview must not turn an external symbolic-link directory into an in-root organization candidate.

## Compatibility

No intentional breaking API change is introduced by this patch release.

The behavior established in v0.1.5 is preserved: when `follow_links` is enabled, symbolic-link entries resolving outside the selected root are pruned before recursive traversal can descend into their targets.

## Version synchronization

The following release metadata is synchronized to `0.1.6`:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

## Validation

Before publishing the tag, run the project's release validation commands, including:

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

Users on the 0.1.x maintenance line can upgrade to v0.1.6 after the published release artifacts have been reviewed. This release is especially useful for projects that depend on the recursive symlink-boundary guarantees introduced in v0.1.5.

## Release status

The repository-side release preparation is complete when the version metadata, tests, changelog, release notes, checklist, and handoff record are committed. Local builds and cross-platform artifact validation must still be performed in a suitable development environment before the tag is published.
