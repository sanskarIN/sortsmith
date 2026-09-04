# SortSmith v0.1.1 — Patch Release

SortSmith v0.1.1 is a focused maintenance release following the initial `0.1.0` baseline.

## Highlights

- Unicode-safe validation for rule text limits.
- Regression tests covering the Unicode boundary conditions.
- Rust, desktop, and Tauri version metadata synchronized to `0.1.1`.
- Dedicated `release/0.1.1` maintenance branch based on the final `0.1.0` source line.

## Fixed

### Unicode rule validation

Rule value validation previously measured text length using UTF-8 byte length. That can reject legitimate non-ASCII values before they reach the intended character limit. Version `0.1.1` measures rule values and filename-regex patterns by Unicode character count instead.

The release adds boundary tests for exactly 128 Unicode characters and for 129 characters.

## Compatibility

- Rust workspace version: `0.1.1`
- Desktop package version: `0.1.1`
- Tauri application version: `0.1.1`
- License: Apache-2.0
- Supported desktop targets remain Windows, macOS, and Linux.

## Safety

This patch does not weaken filesystem safety boundaries. Existing protections for traversal, unsafe filename characters, reserved Windows names, destination containment, collision handling, and reversible operations remain in place.

## Verification before publication

Run the complete release gate locally and in CI:

```text
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
npm ci --ignore-scripts --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

Also verify that all three application metadata sources report `0.1.1` before creating the tag.

## Release status

This file is the release-note source for the `v0.1.1` GitHub release. The Git tag and GitHub release should only be published after the release checklist is green.

## Contributors

SortSmith is maintained by Sanskar and welcomes bug reports, testing feedback, and contributions through GitHub.
