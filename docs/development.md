# Development

## Repository layout

- `crates/sortsmith-core` — platform-neutral Rust business logic.
- `apps/desktop/src-tauri` — Tauri adapter, persistence, filesystem boundary validation, local logging, and command API.
- `apps/desktop/src` — React/TypeScript UI, including isolated helpers for presets and keyboard quick actions.
- `scripts` — repository/release verification helpers.
- `docs` — architecture, testing, release, accessibility, performance, shortcuts, and operations notes.

## Recommended loop

From the repository root:

```bash
cargo test -p sortsmith-core
cargo test -p sortsmith
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

When changing scan, planning, or duplicate-detection performance, also establish a same-machine before/after result:

```bash
cargo bench -p sortsmith-core --bench planning
```

Then:

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
npm run tauri dev
```

Use fictional fixtures only. Never point development automation at an irreplaceable folder; create a temporary test directory instead. Keep symlink and permission-edge tests inside disposable fixtures.

When changing keyboard behavior, update `src/shortcuts.ts` first so shortcut resolution stays independently testable, extend `src/shortcuts.test.ts`, and keep `KeyboardController.tsx` focused on browser event wiring and presentation. Application shortcuts must continue to ignore editable controls.

When changing preset behavior, keep cloning/validation helpers in `src/presets.ts`, preserve Rust-side `AppStateData` validation as the authority for persisted data, and verify that deleting a preset cannot leave a watched-folder reference dangling.

## Release metadata check

Before a version tag, run from the repository root:

```bash
node scripts/verify-release-version.mjs v0.1.0
```

The script verifies that the workspace Cargo version, frontend package version, and Tauri version match the requested release tag. CI repeats this check in the release workflow.

## Dependency policy

Frontend direct dependencies and dev dependencies are pinned to exact versions in `package.json`. `package.json` also declares the supported Node.js 22/npm 10 runtime range. Dependabot proposes updates weekly. Rust dependencies are constrained through the workspace manifest and will produce a shared `Cargo.lock` on the first Rust-capable networked build.

The first trusted networked release-preparation environment must generate and commit `apps/desktop/package-lock.json` and `Cargo.lock` before a release tag. After lockfiles exist, release/CI installation should migrate to lockfile-enforcing commands (`npm ci` and Cargo `--locked`) rather than silently resolving new dependency graphs.

## Change discipline

- Keep domain rules in `sortsmith-core` rather than duplicating them in the UI.
- Validate untrusted UI/import data again in Rust.
- Add a regression or property test for every fixed filesystem/path rule when practical.
- Benchmark before introducing scan caches or parallelism changes.
- Do not log file contents, filenames, or paths in structured operation telemetry.
- Update `CHANGELOG.md`, relevant docs, and `what_changed.md` after meaningful work.
- Prefer small Conventional Commits with one reviewable purpose.
