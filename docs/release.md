# Release

1. Update `CHANGELOG.md`, version fields, and `what_changed.md`.
2. Generate and commit lockfiles from a networked Rust/Node environment if they are not yet present.
3. Run the full local quality suite and build from a clean checkout.
4. Push a signed or reviewed tag `vX.Y.Z`.
5. The release workflow builds desktop bundles on Windows, macOS, and Linux and uploads generated artifacts to a GitHub Release.
6. Verify generated artifacts on clean target machines before promoting the release broadly.

Code signing/notarization requires platform-specific credentials and is intentionally not stored in this public repository.
