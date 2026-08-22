# Testing

The core crate carries unit/integration-style coverage for rule validation, path safety, collision handling, portable rename output, dry-run/apply/undo, journal persistence, hidden-folder behavior, and duplicate detection. The Tauri desktop host has focused tests for bounded operation-log rotation, and the frontend tests deterministic formatting/timing utilities with Vitest.

CI executes Rust formatting, core and desktop-host Clippy, core and desktop-host tests, TypeScript checks, Vitest, and the frontend production build. CodeQL scans both JavaScript/TypeScript and Rust. Desktop packaging is exercised in the release workflow across Windows, macOS, and Linux.

When fixing a bug, first capture the failure in a regression test when feasible. Filesystem tests must use isolated temporary locations and must never access personal folders. Tests must not depend on production credentials, personal data, or network services.

## Required release checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri build
```

Before a release tag, also run the repository metadata guard from the repository root:

```bash
node scripts/verify-release-version.mjs v0.1.0
```

A release is not considered verified until required GitHub checks are green and clean-machine installer smoke tests have passed for each distributed platform.
