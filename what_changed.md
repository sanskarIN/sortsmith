# SortSmith — Work Handoff

## Current repository state

- Default branch: `main`
- Main version line: `0.3.0`
- Current development focus: stabilize the 0.3.0 main line and keep filesystem safety regressions covered by public integration tests.
- License: Apache-2.0
- Repository: `https://github.com/sanskarIN/sortsmith`
- Commit identity: `Sanskar <sanskarin@outlook.in>`

## Main branch work completed on 2026-09-05

### Public filesystem safety regression coverage

Added `crates/sortsmith-core/tests/main_branch_safety_regressions.rs`.

The integration coverage exercises the public `preview_organization` API and verifies that, when symbolic-link following is enabled:

- recursive scans do not traverse an external symbolic-link directory;
- external symbolic-link files are ignored instead of being planned for organization;
- no external file is turned into a move operation;
- the public preview reports the external-file symlink as ignored and records a privacy-safe recoverable warning.

This complements the existing unit coverage and keeps the security boundary tested from outside the core module implementation.

### Main CI/build stabilization

Fixed the 0.3.0 main-line Rust build blockers found by GitHub Actions:

- corrected symlink pruning predicate precedence in `engine.rs`;
- corrected the same predicate in `duplicates.rs`;
- fixed journal path normalization so fallible absolute-path conversion is propagated from the iterator closure;
- restored scan-cache metadata construction locally in `scan_cache.rs`, removing the stale dependency on a missing engine helper;
- updated frontend CI to npm `11.6.0` with `--legacy-peer-deps`, matching the dependency-install path that avoids the observed npm resolver failure.

The failed CI run had exposed real compile errors in `scan_cache.rs`, `duplicates.rs`, `engine.rs`, and `journal.rs`, plus an npm `edgesOut` resolver failure. These were treated as implementation issues rather than ignored CI noise.

### Release-line cleanup

The stale `release/0.1.8` pull request was closed because it had diverged substantially from the active `main` development line and was not mergeable. The 0.1.8 work is not being force-applied over the newer 0.3.0 main history.

Main remains the source of truth for the active 0.3.0 development line.

## Main version integrity

The following main-branch metadata remains synchronized at `0.3.0`:

- `Cargo.toml`
- `apps/desktop/package.json`
- `apps/desktop/src-tauri/tauri.conf.json`

The repository must not be tagged as `v0.1.8` from main while main is still on the 0.3.0 development line.

## Verification policy

GitHub-side changes are committed, but no local build or test result is claimed merely from source inspection. CI is the authoritative verification gate for the branch.

For the main line, the expected quality suite is:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cd apps/desktop
npm install --legacy-peer-deps --no-audit --no-fund
npm run typecheck
npm test
npm run build
```

Filesystem behavior should additionally be reviewed for preview-only planning, reversible journals, collision-safe moves, and root containment when symlink following is enabled.

## Next main-line priorities

1. Verify the latest Rust fixes and npm changes through GitHub Actions.
2. Fix any remaining compile, lint, format, test, typecheck, or build failures on the actual 0.3.0 main implementation.
3. Continue the 0.3 development line with small, independently verifiable commits.
4. Keep `what_changed.md` synchronized after each substantive project milestone.
5. Only create a release tag after version metadata, tests, packaging, and release artifacts have all passed their required gates.
