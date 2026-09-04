# SortSmith v0.1.1 Release Checklist

## 1. Source and version gate

- [x] Create `release/0.1.1` from the final `0.1.0` source commit.
- [x] Set workspace version to `0.1.1`.
- [x] Set desktop package version to `0.1.1`.
- [x] Set Tauri application version to `0.1.1`.
- [x] Record the patch release in `CHANGELOG.md`.
- [x] Add release notes.

## 2. Patch validation

- [x] Validate rule text limits using Unicode character counts.
- [x] Add a 128-character Unicode acceptance regression test.
- [x] Add a 129-character Unicode rejection regression test.
- [ ] Run the full Rust workspace test suite.
- [ ] Run formatting verification.
- [ ] Run Clippy with warnings denied.
- [ ] Run frontend type checking and tests.
- [ ] Run the production frontend build.

## 3. Release integrity

- [ ] Confirm the release branch contains no unrelated `0.2.x` or `0.3.x` feature work.
- [ ] Confirm the generated release tree is clean with `git diff --check`.
- [ ] Confirm dependency lockfiles are present if the release workflow requires them.
- [ ] Confirm CI is green for the release branch.
- [ ] Review the final diff against the `0.1.0` source commit.

## 4. Tag

Create the annotated tag only after all verification gates pass:

```bash
git checkout release/0.1.1
git pull origin release/0.1.1
git tag -a v0.1.1 -m "SortSmith v0.1.1"
git push origin v0.1.1
```

## 5. GitHub release

Use:

- Tag: `v0.1.1`
- Target: `release/0.1.1`
- Title: `SortSmith v0.1.1 — Patch Release`
- Pre-release: enabled unless cross-platform packaging has already been verified for this exact build
- Release notes: copy `RELEASE_NOTES_v0.1.1.md`

## 6. Post-release

- [ ] Confirm the tag points to the verified release commit.
- [ ] Confirm the GitHub release displays the expected source archive.
- [ ] Verify the release page links to the changelog and project documentation.
- [ ] Record the final release commit SHA in `what_changed.md`.
- [ ] Start the next development line only after the patch release is published or explicitly deferred.
