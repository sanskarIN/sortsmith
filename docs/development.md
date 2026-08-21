# Development

## Repository layout

- `crates/sortsmith-core` — platform-neutral Rust business logic.
- `apps/desktop/src-tauri` — Tauri adapter and persistence.
- `apps/desktop/src` — React/TypeScript UI.
- `docs` — architecture, testing, release, accessibility, and operations notes.

## Recommended loop

```bash
cargo test -p sortsmith-core
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cd apps/desktop
npm run typecheck
npm test
npm run tauri dev
```

Use fictional fixtures only. Never point development automation at an irreplaceable folder; create a temporary test directory instead.

## Dependency policy

Frontend direct dependencies and dev dependencies are pinned to exact versions in `package.json`. Dependabot proposes updates weekly. Rust dependencies are constrained through the workspace manifest and will produce a shared `Cargo.lock` on the first Rust-capable build. The first networked release-preparation environment should also generate and commit `package-lock.json` and `Cargo.lock` before a release tag.
