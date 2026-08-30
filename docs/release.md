# Release

The current prepared candidate is `0.2.0`. Do not publish `v0.2.0` until every gate below is satisfied.

1. Update `CHANGELOG.md`, version fields, and `what_changed.md`.
2. Generate and commit `Cargo.lock` and `apps/desktop/package-lock.json` from a trusted networked Rust/Node environment if they are not yet present. Do not hand-author either lockfile.
3. From the repository root, verify metadata and committed lockfiles:

   ```bash
   node scripts/verify-release-version.mjs v0.2.0
   node scripts/verify-release-lockfiles.mjs
   cargo fetch --locked
   ```

4. Run the full quality suite and build from a clean checkout. Once lockfiles exist, use lockfile-enforcing commands for release verification:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace
   cargo bench -p sortsmith-core --bench planning
   cd apps/desktop
   npm ci --no-audit --no-fund
   npm run typecheck
   npm test
   npm run build
   npm run tauri build
   ```

5. Confirm required CI and CodeQL checks are green on the exact `main` commit intended for the tag.
6. Perform keyboard-only and native-dialog accessibility checks on Windows, macOS, and Linux. Include shortcut conflicts, modal focus handoff/restoration, 200% zoom, theme contrast, and status announcements.
7. Push the reviewed tag `v0.2.0`. The release workflow repeats the version and lockfile guards before packaging.
8. The release workflow builds desktop bundles on Windows, macOS, and Linux and uploads generated artifacts to a **draft** GitHub Release.
9. Smoke-test every generated installer/package on a clean target machine before publishing the draft release broadly. Verify install, first launch, native folder selection, dry-run, apply/undo on disposable fixtures, settings backup/restore, and uninstall behavior.
10. Capture real screenshots only from verified builds and replace placeholder screenshot documentation.
11. Record artifact names, tested operating-system versions, signing/notarization status, benchmark environment, and known limitations in the final release notes and `what_changed.md`.

## Automated safeguards

Ordinary CI runs `scripts/verify-release-version.mjs` without a tag so Cargo, frontend, and Tauri versions cannot silently drift.

The tag-driven release workflow additionally runs:

```bash
node scripts/verify-release-version.mjs "$TAG"
node scripts/verify-release-lockfiles.mjs
cargo fetch --locked
```

Frontend release dependency installation uses `npm ci`. A missing, stale, or version-mismatched lockfile blocks packaging instead of silently resolving a different dependency graph.

## Signing and notarization

Code signing/notarization requires platform-specific credentials and is intentionally not stored in this public repository. Configure credentials only through protected CI secrets or trusted platform key stores. Never commit certificates, private keys, passwords, tokens, or notarization credentials.

The repository can prepare unsigned draft artifacts, but public distribution should follow the signing/notarization expectations of each target platform.

## Release gate

Do not publish merely because a tag built successfully. The release gate requires:

- green quality and security checks on the tagged commit;
- verified `0.2.0` metadata consistency;
- reviewed package-manager-generated lockfiles;
- clean-machine installer smoke tests on all distributed platforms;
- current real screenshots from verified builds;
- completed signing/notarization where required for the chosen distribution path;
- no known blocker or critical defect.
