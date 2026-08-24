# Development

## Repository layout

- `crates/sortsmith-core` — platform-neutral Rust business logic and default bundled presets.
- `apps/desktop/src-tauri` — Tauri adapter, persistence, filesystem boundary validation, local logging, and command API.
- `apps/desktop/src` — React/TypeScript UI, including isolated helpers for saved presets, bundled-preset compatibility, and keyboard quick actions.
- `scripts` — repository/release verification helpers.
- `docs` — architecture, testing, release, accessibility, performance, presets, shortcuts, and operations notes.

## Recommended loop

From the repository root:

```bash
cargo test -p sortsmith-core
cargo test -p sortsmith
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
node scripts/verify-release-version.mjs
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

When changing keyboard behavior, update `src/shortcuts.ts` first so shortcut resolution stays independently testable, extend `src/shortcuts.test.ts`, and keep `KeyboardController.tsx` focused on browser event wiring and presentation. Application shortcuts must continue to ignore editable controls. Modal changes must preserve labelled dialog semantics, Escape dismissal, focus entry, and focus restoration.

When changing saved preset behavior, keep cloning/validation helpers in `src/presets.ts`, preserve Rust-side `AppStateData` validation as the authority for persisted data, and verify that deleting a preset cannot leave a watched-folder reference dangling.

When changing bundled presets, keep stable preset IDs synchronized between `crates/sortsmith-core/src/models.rs` and `apps/desktop/src/bundledPresets.ts`. Extend `bundledPresets.test.ts` for compatibility behavior. Existing user rules and watched-folder assignments must be preserved during migrations; never delete user presets to make room for a bundled pack.

## Release metadata check

The prepared candidate is `0.2.0`. Ordinary development/CI can verify that all authoritative metadata agrees without passing a tag:

```bash
node scripts/verify-release-version.mjs
```

Before the release tag, verify the exact intended tag and lockfiles:

```bash
node scripts/verify-release-version.mjs v0.2.0
node scripts/verify-release-lockfiles.mjs
cargo fetch --locked
```

The version script verifies the workspace Cargo version, frontend package version, and Tauri version. The About page reads the frontend package version rather than maintaining a fourth version string.

## Dependency policy

Frontend direct dependencies and dev dependencies are pinned to exact versions in `package.json`. `package.json` also declares the supported Node.js 22/npm 10 runtime range. Dependabot proposes updates weekly. Rust dependencies are constrained through the workspace manifest and share one `Cargo.lock` once generated.

The first trusted networked release-preparation environment must generate and commit `apps/desktop/package-lock.json` and `Cargo.lock` before `v0.2.0`. Do not hand-author these files. The release workflow is intentionally fail-closed until both exist and are aligned with version `0.2.0`.

After lockfiles exist, release-sensitive frontend installation uses `npm ci`; release Rust dependency resolution is preflighted with `cargo fetch --locked`. Development CI may continue resolving dependencies normally until the reviewed lockfiles are committed, after which CI should also migrate to lockfile-enforcing commands.

## Change discipline

- Keep domain rules and filesystem safety in `sortsmith-core` rather than duplicating execution behavior in the UI.
- Keep bundled-preset compatibility behavior explicit and tested when cross-language catalog metadata must be mirrored.
- Validate untrusted UI/import data again in Rust.
- Add a regression or property test for every fixed filesystem/path rule when practical.
- Benchmark before introducing scan caches or parallelism changes.
- Do not log file contents, filenames, or paths in structured operation telemetry.
- Update `CHANGELOG.md`, relevant docs, and `what_changed.md` after meaningful work.
- Prefer small Conventional Commits with one reviewable purpose.
