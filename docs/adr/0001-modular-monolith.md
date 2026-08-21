# ADR 0001: Modular monolith with a pure Rust core

**Status:** Accepted — 2026-08-21

## Context
SortSmith needs cross-platform filesystem logic that can be tested independently of UI/webview dependencies.

## Decision
Keep domain logic in `sortsmith-core`, adapt it through Tauri commands, and keep React focused on presentation.

## Consequences
Core tests remain fast and portable. Tauri permissions and UI changes do not infect domain code. Cross-platform packaging still requires platform runners.
