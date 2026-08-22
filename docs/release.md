# Release

1. Update `CHANGELOG.md`, version fields, and `what_changed.md`.
2. Generate and commit `Cargo.lock` and `apps/desktop/package-lock.json` from a trusted networked Rust/Node environment if they are not yet present.
3. From the repository root, verify that the intended tag matches every version source:

   ```bash
   node scripts/verify-release-version.mjs vX.Y.Z
   ```

4. Run the full local quality suite and build from a clean checkout:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace
   cd apps/desktop
   npm install --no-audit --no-fund
   npm run typecheck
   npm test
   npm run build
   npm run tauri build
   ```

5. Confirm required CI and CodeQL checks are green on `main`.
6. Push a reviewed tag `vX.Y.Z`. The release workflow repeats the version-metadata guard before packaging.
7. The release workflow builds desktop bundles on Windows, macOS, and Linux and uploads generated artifacts to a draft GitHub Release.
8. Smoke-test every generated installer/package on a clean target machine before publishing the draft release broadly.
9. Record artifact names, tested operating-system versions, signing/notarization status, and any known limitations in the final release notes and `what_changed.md`.

## Signing and notarization

Code signing/notarization requires platform-specific credentials and is intentionally not stored in this public repository. Configure those credentials only through protected CI secrets or trusted platform key stores. Never commit certificates, private keys, passwords, tokens, or notarization credentials.

## Release gate

Do not publish merely because a tag built successfully. The release gate requires green quality/security checks, verified version consistency, clean-machine installer smoke tests, current documentation, and no known blocker/critical defects.
