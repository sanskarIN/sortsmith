# Troubleshooting

- **Folder rejected:** ensure the path exists and is a directory. The desktop host canonicalizes selected roots before filesystem work.
- **Permission denied:** adjust operating-system permissions for that folder, then preview again. SortSmith does not attempt privilege escalation.
- **A file was skipped:** inspect the preview's recoverable issues; unreadable entries are skipped instead of aborting the whole scan.
- **Hidden files were not scanned:** enable **Include hidden files** in Settings. Hidden directories are pruned during normal and duplicate traversal when this setting is off.
- **A rename rule is rejected:** remove path separators, control/invalid filename characters, trailing spaces/periods, Windows-reserved device names, or an overlong rendered filename. Rename templates may use `{name}` and `{ext}` only.
- **Undo skipped an item:** the original path may now be occupied or the moved file may be missing. SortSmith will not overwrite another file during undo.
- **Undo history entry is unavailable:** journal availability is derived from the current filesystem. A journal can remain visible even after some or all files were already restored or changed outside SortSmith.
- **Settings backup is rejected:** choose a regular `.json` file created by SortSmith. Imports are schema-validated, symlinks are refused, and files larger than 16 MiB are rejected.
- **Saved settings cannot load:** the app-data `state.json` must be a regular local file with supported schema version `1`. Do not replace it with a symlink.
- **Operation log stopped updating after manual tampering:** SortSmith refuses symlink/non-file log targets. Restore the app-data log path to a normal file or remove the tampered entry so SortSmith can recreate it.
- **Linux build errors:** confirm all current Tauri 2 WebKitGTK system packages are installed.
- **Release tag rejected:** run `node scripts/verify-release-version.mjs vX.Y.Z` and make the Cargo workspace, frontend package, and Tauri versions match before tagging.
- **Dependency install unavailable:** use a trusted networked environment, run `npm install` and `cargo generate-lockfile`, then commit the resulting `apps/desktop/package-lock.json` and `Cargo.lock` before release.
