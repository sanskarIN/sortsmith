# Setup

Install Rust stable with Rust 2024 edition support, **Node.js 22.x**, **npm 10.9.x**, Git, and the Tauri 2 operating-system prerequisites. The frontend manifest intentionally rejects Node 23+ and npm 11+ until those major versions are validated for this release line.

## Windows

Install Microsoft C++ Build Tools and WebView2 (normally present on supported Windows versions). Use the Desktop development with C++ workload so the MSVC linker and Windows SDK are available.

## macOS

Install Xcode Command Line Tools:

```bash
xcode-select --install
```

## Linux

Install the WebKitGTK, GTK, SSL, appindicator, librsvg, and build-essential packages required by Tauri for your distribution. Package names differ by distribution; follow current Tauri 2 prerequisites. The Ubuntu CI runner installs `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, and `patchelf` in addition to the runner's base build tools.

## Clone and run

```bash
git clone https://github.com/sanskarIN/sortsmith.git
cd sortsmith/apps/desktop
npm install
npm run tauri dev
```

## Verify the environment

From the repository root:

```bash
rustc --version
cargo --version
node --version
npm --version
```

From `apps/desktop`:

```bash
npm run typecheck
npm test
npm run build
```

If native Tauri startup fails, see [`troubleshooting.md`](troubleshooting.md). Do not test organizer mutations against irreplaceable personal folders; use a temporary fixture directory until the build is verified on your machine.
