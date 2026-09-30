<p align="center"><img src="assets/logo.svg" width="112" alt="SortSmith logo"></p>
<h1 align="center">SortSmith</h1>
<p align="center"><strong>Private, reversible file organization for Windows, macOS, and Linux.</strong></p>
<p align="center"><strong>Made by the Sanskar</strong></p>

[![CI](https://github.com/sanskarIN/sortsmith/actions/workflows/ci.yml/badge.svg)](https://github.com/sanskarIN/sortsmith/actions/workflows/ci.yml)
[![CodeQL](https://github.com/sanskarIN/sortsmith/actions/workflows/codeql.yml/badge.svg)](https://github.com/sanskarIN/sortsmith/actions/workflows/codeql.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-sanskarIN-FFDD00?logo=buy-me-a-coffee&logoColor=000000)](https://buymeacoffee.com/sanskarIN)

SortSmith is an offline-first desktop file organizer built with **Rust + Tauri + React**. It previews changes before touching the filesystem, records reversible operation journals, detects duplicate candidates by content hash without deleting them, and can run user-controlled watched-folder rules while the app is open.

> **Development status:** `main` remains the prepared `0.2.0` release candidate. The `develop/0.3.0` branch is the next-version line and currently adds incremental in-memory preview caching. Do not treat either version as publishable until its required CI/release gates are complete; `v0.2.0` still requires committed lockfiles, clean cross-platform installer smoke tests, verified screenshots, and required signing/notarization.

## Screenshots

Real release screenshots will be captured from verified release builds. Capture requirements live in [`docs/screenshots/README.md`](docs/screenshots/README.md). Until then, the UI is reproducible from source with the development commands below.

## Features

- Rules by extension, MIME prefix, modified age, size range, and filename regex.
- Multi-criterion rule builder with match-all/match-any behavior and reusable presets.
- Four protected bundled preset packs: Everyday tidy, Media library, Developer workspace, and Downloads cleanup.
- Backward-compatible migration of the legacy Everyday tidy preset ID, including watched-folder reference remapping.
- Saved user presets: snapshot active rules, load them later, edit custom preset metadata, and safely delete presets that are not used by watched folders. See [`docs/presets.md`](docs/presets.md).
- Native folder picker plus typed-path fallback.
- Dry-run previews with source/destination visibility before any change.
- Incremental process-local caching for repeated interactive previews on the 0.3 development line, with exact scope invalidation and time-sensitive rule revalidation.
- Collision-safe moving and renaming with cross-platform filename validation.
- Reversible JSON journals, latest undo, and selectable operation-history undo.
- BLAKE3 duplicate-candidate detection with size pre-filtering, parallel hashing, hidden-folder controls, and no auto-delete.
- Watched folders with user-controlled presets and intervals while SortSmith is running.
- Native JSON settings backup/restore with schema validation and local undo-history preservation.
- Keyboard-first quick actions for navigation, folder selection, preview, apply, and undo; press `Shift+?` in the app for the reference. See [`docs/keyboard-shortcuts.md`](docs/keyboard-shortcuts.md).
- Shortcut-help focus moves into the dialog and returns to the previously focused control when the dialog closes.
- Bounded, durable local settings persistence and rotating privacy-safe operation logs.
- Light, dark, and system themes; keyboard focus states; reduced-motion preference.
- Permission failures are isolated as recoverable errors where possible.
- No cloud account, telemetry, or file-content collection.

## Supported platforms

| Platform | Target | Notes |
|---|---|---|
| Windows | Windows 10/11 | Tauri WebView2 runtime |
| macOS | Current supported macOS releases | WebKit |
| Linux | Mainstream desktop distributions | WebKitGTK system dependencies |

## Tech stack

- Rust 2024 workspace with a UI-independent `sortsmith-core` crate.
- Tauri 2 desktop host.
- React 19 + TypeScript + Vite frontend.
- BLAKE3, WalkDir, Rayon, Regex, Serde, Chrono.
- Proptest property coverage and Criterion performance benchmarks for core filesystem behavior.
- GitHub Actions CI and CodeQL scanning for TypeScript and Rust.

## Quick start

Prerequisites: Rust stable with Rust 2024 edition support, **Node.js 22**, **npm 10**, and the platform packages required by Tauri.

```bash
git clone https://github.com/sanskarIN/sortsmith.git
cd sortsmith/apps/desktop
npm install
npm run tauri dev
```

For 0.3 development, check out `develop/0.3.0` before installing dependencies. See [`docs/setup.md`](docs/setup.md) for platform prerequisites and [`docs/development.md`](docs/development.md) for the complete workflow.

## Testing

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo bench -p sortsmith-core --bench planning
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

CI separately exercises the core crate, Tauri desktop host, and frontend so failures remain easy to diagnose. Timing-sensitive Criterion measurements should be compared on the same local machine/toolchain rather than treated as stable shared-runner numbers. On the 0.3 branch, compare the `organization_planning` and `organization_planning_warm_cache` benchmark groups before making any cache-performance claim.

## Build and release

Development builds may continue using normal package-manager resolution until the lockfiles are generated. A release tag is fail-closed and requires committed lockfiles.

The `v0.2.0` release must be created only from the validated `main` candidate. Before tagging from the repository root:

```bash
node scripts/verify-release-version.mjs v0.2.0
node scripts/verify-release-lockfiles.mjs
```

After `Cargo.lock` and `apps/desktop/package-lock.json` exist and have been reviewed:

```bash
cargo fetch --locked
cd apps/desktop
npm ci --no-audit --no-fund
npm run tauri build
```

The 0.3 development branch uses synchronized `0.3.0` source metadata but is not a release candidate. Its ordinary consistency check is:

```bash
node scripts/verify-release-version.mjs
```

Packaging is automated by `.github/workflows/release.yml` for `v*` tags. The workflow rechecks release metadata and lockfiles before building on Windows, macOS, and Linux, and creates a draft GitHub Release rather than publishing unverified artifacts automatically. Release guidance is in [`docs/release.md`](docs/release.md).

## Architecture

The repository is a modular monolith: deterministic filesystem domain logic lives in `crates/sortsmith-core`; Tauri commands adapt that logic to the desktop runtime; the React frontend owns presentation and interaction. The 0.3 cache remains a disposable optimization inside the core/desktop boundary and does not change persisted settings. See [`docs/architecture.md`](docs/architecture.md) and [`docs/adr/`](docs/adr/).

## Security and privacy

SortSmith deliberately avoids network features in its core workflows. It does not upload file contents or filenames. Undo journals contain local file paths because reversal requires them; portable settings backups omit undo-history identifiers. Saved settings and imported backups are bounded and validated before use, and privacy-safe operation logs contain counts/identifiers rather than file paths or content. See [`SECURITY.md`](SECURITY.md) and [`PRIVACY.md`](PRIVACY.md).

## Contributing

Contributions are welcome. Read [`CONTRIBUTING.md`](CONTRIBUTING.md), run the quality suite, and use focused Conventional Commit messages.

## License

Apache License 2.0. See [`LICENSE`](LICENSE).

## Contact and support

- Business: `sanskarin@outlook.in`
- Business: `sanskarin.business@gmail.com`
- Support: `supportramsandesh@gmail.com`
- GitHub: https://github.com/sanskarIN
- Funding: https://buymeacoffee.com/sanskarIN

[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-sanskarIN-FFDD00?logo=buy-me-a-coffee&logoColor=000000)](https://buymeacoffee.com/sanskarIN)

**Made by the Sanskar**