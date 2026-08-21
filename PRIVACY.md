# Privacy

SortSmith is designed to work locally without an account.

## Data processed

The organizer reads file metadata needed for rules: path/name, size, extension, MIME guess, and modification time. Duplicate detection reads file bytes locally to calculate BLAKE3 hashes. File contents are not transmitted by SortSmith.

## Data stored

The app data directory stores preferences, custom rules, watched-folder configuration, and undo journals. Journals include source and destination paths because those paths are required for reversal. Users can remove this local app data using normal operating-system file management.

The structured operation log records only timestamps, event names, journal identifiers, completed operation counts, and error counts. It does not record file contents or file paths.

## Telemetry and network

Version 0.1.0 includes no telemetry, analytics, advertising, account system, or cloud synchronization. Links in the About page open external websites only when the user activates them.
