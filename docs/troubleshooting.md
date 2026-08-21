# Troubleshooting

- **Folder rejected:** ensure the path exists and is a directory. The desktop host canonicalizes selected roots before filesystem work.
- **Permission denied:** adjust operating-system permissions for that folder, then preview again. SortSmith does not attempt privilege escalation.
- **A file was skipped:** inspect the preview's recoverable issues; unreadable entries are skipped instead of aborting the whole scan.
- **Undo skipped an item:** the original path may now be occupied or the moved file may be missing. SortSmith will not overwrite another file during undo.
- **Linux build errors:** confirm all current Tauri 2 WebKitGTK system packages are installed.
- **Dependency install unavailable:** use a networked environment, run `npm install` and `cargo generate-lockfile`, then commit the resulting lockfiles before release.
