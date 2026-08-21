# Setup

Install current Rust stable, Node.js 22 or newer, npm, Git, and Tauri 2 operating-system prerequisites.

## Windows
Install Microsoft C++ Build Tools and WebView2 (normally present on supported Windows versions).

## macOS
Install Xcode Command Line Tools with `xcode-select --install`.

## Linux
Install the WebKitGTK, GTK, SSL, appindicator, librsvg, and build-essential packages required by Tauri for your distribution. Package names differ by distribution; follow current Tauri 2 prerequisites.

Then:

```bash
git clone https://github.com/sanskarIN/sortsmith.git
cd sortsmith/apps/desktop
npm install
npm run tauri dev
```
