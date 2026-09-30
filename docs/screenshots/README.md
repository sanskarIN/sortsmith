# Release screenshots

Release screenshots for `0.2.0` must be captured from real Windows, macOS, and Linux artifacts only after the corresponding candidate has passed CI and installer smoke tests. Do not add mock screenshots that could misrepresent shipping behavior.

Capture at least:

- Organize with a reviewed dry-run preview using fictional fixture paths.
- Rules with the bundled preset selector visible.
- Duplicate candidates.
- Automation/watched folders.
- History/selective undo.
- Settings and native backup/restore controls.
- Keyboard shortcut reference.
- About showing version `0.2.0`.

Capture light and dark themes where the visual difference is useful. Include one 200% zoom/reflow verification capture if it helps document accessibility evidence.

Never expose personal folders, usernames, filenames, email inboxes, tokens, signing identities, or other private data. Use disposable fictional fixtures and redact anything sensitive before committing an image.

Record screenshot provenance in [`../release-evidence-0.2.0.md`](../release-evidence-0.2.0.md), including the exact commit/artifact and operating system used for capture.
