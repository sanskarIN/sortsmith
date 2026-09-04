# SortSmith v0.1.4 — Patch Release

SortSmith v0.1.4 is a focused maintenance release for safer recursive previews when symbolic links are followed.

## Highlights

- **Safer recursive previews:** followed file links are resolved before planning an organization operation.
- **External-target protection:** files whose resolved targets are outside the selected root are skipped and reported as recoverable preview errors.
- **Defense in depth:** execution-time canonical path validation remains active, so a forged or changed preview is still rejected before file mutation.
- **Regression coverage:** added Unix coverage for an external file symlink while retaining the v0.1.1–v0.1.3 safety and durability tests.

## Upgrade notes

No configuration migration is required. Existing rules, presets, settings, and journal formats remain compatible with this patch line.

If `follow_links` is enabled, a symlink that resolves outside the selected folder will now be reported as skipped during preview rather than appearing as an executable operation.

## Validation gate

Before publishing the tag, run the repository's release verification and full Rust/desktop checks described in `docs/release-v0.1.4-checklist.md`.

## Release identity

- Tag: `v0.1.4`
- Branch: `release/0.1.4`
- Version: `0.1.4`
- Release type: patch
- License: Apache-2.0
