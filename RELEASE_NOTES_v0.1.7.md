# SortSmith v0.1.7 — Patch Release

SortSmith v0.1.7 is a stable maintenance release for the 0.1.x line. It makes duplicate-detection output deterministic without changing duplicate matching or deletion behavior.

## Highlights

### Deterministic duplicate results

Duplicate groups are now normalized by sorting the file paths inside each duplicate group before the result is returned. The group ordering already follows stable size/hash ordering; v0.1.7 completes the deterministic ordering by making the member list stable as well.

This prevents filesystem traversal and parallel hashing order from leaking into the UI or downstream consumers. Repeated scans over the same unchanged directory now return duplicate members in the same path order.

### Regression coverage

The release adds a focused regression that creates two equal-content files in reverse lexical creation order and verifies that the duplicate group is returned in lexical path order.

Existing coverage for content equality, hidden-directory handling, and external symbolic-link traversal remains in place.

## Compatibility

No intentional breaking public API change is introduced. Duplicate detection still uses BLAKE3, still requires equal file size and content before grouping, and still never deletes files automatically.

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

The full workspace and desktop suite should be exercised on Linux, Windows, and macOS before publishing production installers.

## Release metadata

- **Version:** `0.1.7`
- **Tag:** `v0.1.7`
- **Target branch:** `release/0.1.7`
- **Release title:** `SortSmith v0.1.7 — Patch Release`
- **Pre-release:** No
- **Latest:** No
- **License:** Apache-2.0

## Release status

Repository-side preparation includes the deterministic duplicate-ordering fix, regression coverage, synchronized version metadata, changelog, release notes, checklist, and handoff updates. Local Rust/Node/Tauri execution has not been claimed as passed in this environment. Publish the tag only after the validation gates pass in CI or on the release machine.
