# Testing

The core crate carries unit/integration-style coverage for path safety, collision handling, dry-run/apply/undo, and duplicate detection. The frontend tests deterministic formatting and timing utilities with Vitest.

CI executes formatting, Clippy, Rust tests, TypeScript checks, Vitest, and the frontend production build. Desktop packaging is exercised in the release workflow across Windows, macOS, and Linux.

When fixing a bug, first capture the failure in a regression test when feasible. Filesystem tests must use temporary directories and must not access personal folders.

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
