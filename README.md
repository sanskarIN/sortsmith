<p align="center"><img src="assets/logo.svg" width="112" alt="SortSmith logo"></p>
<h1 align="center">SortSmith</h1>
<p align="center"><strong>Private, reversible file organization for Windows, macOS, and Linux.</strong></p>
<p align="center"><strong>Made by the Sanskar</strong></p>

[![CI](https://github.com/sanskarIN/sortsmith/actions/workflows/ci.yml/badge.svg)](https://github.com/sanskarIN/sortsmith/actions/workflows/ci.yml)
[![CodeQL](https://github.com/sanskarIN/sortsmith/actions/workflows/codeql.yml/badge.svg)](https://github.com/sanskarIN/sortsmith/actions/workflows/codeql.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-sanskarIN-FFDD00?logo=buy-me-a-coffee&logoColor=000000)](https://buymeacoffee.com/sanskarIN)

SortSmith is an offline-first desktop file organizer built with **Rust + Tauri + React**. It previews changes before touching the filesystem, records reversible operation journals, detects duplicate candidates by content hash without deleting them, and can run user-controlled watched-folder rules while the app is open.

> **Development status:** the `0.1.x` maintenance line has reached v0.1.8 preparation. The dedicated `release/0.1.8` branch is kept separate from the `main` branch's 0.3.x feature-development line. A release tag must only be published after the complete validation and installer checks are green.

## Screenshots

Real release screenshots will be captured from verified release builds. Capture requirements live in [`docs/screenshots/README.md`](docs/screenshots/README.md). Until then, the UI is reproducible from source with the development commands below.

## Features

- Rules by extension, MIME prefix, modified age, size range, and filename regex.
- Multi-criterion rule builder with match-all/match-any behavior and reusable presets.
- Saved user presets: snapshot active rules, load them later, edit custom preset metadata, and safely delete presets that are not used by watched folders.
- Native folder picker plus typed-path fallback.
- Dry-run previews with source/destination visibility before any change.
- Collision-safe moving and renaming with cross-platform filename validation.
- Reversible JSON journals, latest undo, and selectable operation-history undo.
- BLAKE3 duplicate-candidate detection with size pre-filtering, parallel hashing, hidden-folder controls, and no auto-delete.
- Watched folders with user-controlled presets and intervals while SortSmith is running.
- Native JSON settings backup/restore with schema validation and local undo-history preservation.
- Keyboard-first quick actions for navigation, folder selection, preview, apply, and undo; press `Shift+?` in the app for the reference. See [`docs/keyboard-shortcuts.md`](docs/keyboard-shortcuts.md).
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

See [`docs/setup.md`](docs/setup.md) for platform prerequisites and [`docs/development.md`](docs/development.md) for the complete workflow.

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
