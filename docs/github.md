# GitHub project administration

Recommended settings for `main`:
- require pull requests for non-trivial changes;
- require the `core`, `desktop-rust`, and `frontend` CI jobs;
- require the CodeQL analysis workflow/security scanning to complete successfully;
- require conversations to be resolved;
- require branches to be up to date before merge when practical;
- block force pushes and branch deletion;
- enable Dependabot alerts, secret scanning, and push protection where available;
- enable GitHub Discussions for ideas, Q&A, and release feedback.

The `frontend` CI job includes the release-version synchronization guard, so a version drift between Cargo, `package.json`, and Tauri configuration should fail before tagging. The tag-driven release workflow separately requires reviewed package-manager lockfiles before packaging and creates draft release artifacts for later smoke testing.

Do not make a status check required until it has run successfully at least once with its final workflow/job name; otherwise branch protection can deadlock normal maintenance.

Suggested labels: `bug`, `enhancement`, `security`, `accessibility`, `performance`, `core`, `frontend`, `desktop`, `documentation`, `release`, `good first issue`, `help wanted`.

Suggested milestones: `0.1 Safe Baseline`, `0.2 Desktop Polish`, `0.3 Scale & Automation`.

The prepared source candidate is `0.2.0`, but repository administration must not treat `v0.2.0` as approved merely because metadata has been bumped. Require actual green checks, reviewed lockfiles, clean-machine installer evidence, and signing/notarization as appropriate before public release.

For release tags, restrict tag creation to trusted maintainers where repository settings permit it. Signing/notarization secrets belong in protected GitHub Actions environments/secrets, never repository files or issue comments.
