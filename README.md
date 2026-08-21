<p align="center"><img src="assets/logo.svg" width="112" alt="SortSmith logo"></p>
<h1 align="center">SortSmith</h1>
<p align="center"><strong>Private, reversible file organization for Windows, macOS, and Linux.</strong></p>
<p align="center"><strong>Made by the Sanskar</strong></p>

[![CI](https://github.com/sanskarIN/sortsmith/actions/workflows/ci.yml/badge.svg)](https://github.com/sanskarIN/sortsmith/actions/workflows/ci.yml)
[![CodeQL](https://github.com/sanskarIN/sortsmith/actions/workflows/codeql.yml/badge.svg)](https://github.com/sanskarIN/sortsmith/actions/workflows/codeql.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-sanskarIN-FFDD00?logo=buy-me-a-coffee&logoColor=000000)](https://buymeacoffee.com/sanskarIN)

SortSmith is an offline-first desktop file organizer built with **Rust + Tauri + React**. It previews changes before touching the filesystem, records reversible operation journals, detects duplicate candidates by content hash without deleting them, and can run user-controlled watched-folder rules while the app is open.

> **Development status:** the `0.1.0` implementation baseline is in place. Do not treat it as a published release until the repository's cross-platform CI, clean installer smoke tests, and release checklist are green.

## Screenshots

Real release screenshots will be captured from verified release builds. Placeholder policy and capture requirements live in [`docs/screenshots/README.md`](docs/screenshots/README.md). Until then, the UI is reproducible from source with the development commands below.

## Features

- Rules by extension, MIME prefix, modified age, size range, and filename regex.
- Dry-run previews with source/destination visibility before any change.
- Collision-safe renaming and moving.
- Reversible JSON journals and one-click latest undo.
- BLAKE3 duplicate-candidate detection with size pre-filtering and no auto-delete.
- Reusable presets and custom rules.
- Watched folders with user-controlled intervals while SortSmith is running.
- Local settings, export/import backend commands, and schema-versioned persistence.
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

## Quick start

Prerequisites: Rust stable, Node.js 22+, npm, and the platform packages required by Tauri.

```bash
git clone https://github.com/sanskarIN/sortsmith.git
cd sortsmith/apps/desktop
npm install
npm run tauri dev
```

See [`docs/setup.md`](docs/setup.md) for platform prerequisites and [`docs/development.md`](docs/development.md) for the complete workflow.

## Testing

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cd apps/desktop
npm install --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

## Build and release

```bash
cd apps/desktop
npm install --no-audit --no-fund
npm run tauri build
```

Packaging is automated by `.github/workflows/release.yml` for version tags. Release guidance is in [`docs/release.md`](docs/release.md).

## Architecture

The repository is a modular monolith: deterministic filesystem domain logic lives in `crates/sortsmith-core`; Tauri commands adapt that logic to the desktop runtime; the React frontend owns presentation and interaction. See [`docs/architecture.md`](docs/architecture.md) and [`docs/adr/`](docs/adr/).

## Security and privacy

SortSmith deliberately avoids network features in its core workflows. It does not upload file contents or filenames. Undo journals contain local file paths because reversal requires them. See [`SECURITY.md`](SECURITY.md) and [`PRIVACY.md`](PRIVACY.md).

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
