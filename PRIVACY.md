# Privacy

SortSmith is designed to work locally without an account.

## Data processed

The organizer reads file metadata needed for rules: path/name, size, extension, MIME guess, and modification time. Duplicate detection reads file bytes locally to calculate BLAKE3 hashes. File contents are not transmitted by SortSmith.

## Data stored

The app data directory stores preferences, custom rules, presets, watched-folder configuration, undo journals, and a small structured operation log. Journals include source and destination paths because those paths are required for reversal. Users can remove this local app data using normal operating-system file management.

Saved settings and imported/exported settings files are validated and bounded to 16 MiB. Portable settings exports intentionally omit local undo-history identifiers so a backup does not pretend that journal files will exist on another installation.

The structured operation log records only timestamps, event names, journal identifiers, completed operation counts, and error counts. It does not record file contents or file paths. The active log rotates at 5 MiB and keeps at most one previous log file. SortSmith refuses to append through a symlink or non-file log target.

Bundled and user-defined presets are local rule metadata. Version 0.2 may normalize the legacy `Everyday tidy` preset identifier and watched-folder references to a stable bundled identifier; this migration remains local and does not transmit preset or path data.

## Telemetry and network

Version 0.2.0 includes no telemetry, analytics, advertising, account system, or cloud synchronization. Links in the About page open external websites only when the user activates them.

## Local deletion and backups

SortSmith does not provide a cloud retention layer. Removing the app's local data directory removes its saved preferences, local logs, and undo journals. Settings backup files are ordinary JSON files chosen by the user and remain wherever the user saved them until the user removes them.
