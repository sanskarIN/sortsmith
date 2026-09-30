# Presets

SortSmith presets are reusable snapshots of rules. Loading a preset copies its rules into the active rule set; it does not immediately change any files. The user still chooses a folder, runs a dry-run preview, reviews destinations, and explicitly applies the preview.

## Bundled preset packs

Version 0.2 ships four protected bundled packs:

- **Everyday tidy** — common images, documents, archives, audio, and video into clear top-level folders.
- **Media library** — images, audio, and video below `Media/Images`, `Media/Audio`, and `Media/Video`.
- **Developer workspace** — common source files, data/configuration files, and package archives below `Development/` staging folders.
- **Downloads cleanup** — installers, archives, documents, and images into common Downloads-oriented folders.

Bundled presets have stable UUIDs. They can be loaded but cannot be renamed or deleted in the UI, which guarantees that the built-in library remains recoverable. To customize a bundled pack, load it, edit the active rules, and save the resulting rule set as a new user preset.

## Saved user presets

User presets can be created from the current active rules. The snapshot clones rule and criterion collections so later edits do not mutate the saved preset accidentally.

Custom presets can be renamed and their description can be edited. A custom preset cannot be deleted while a watched-folder entry references it. Change or remove the watched-folder assignment first.

The current local-state validation contract supports at most 50 total presets and 500 rules per preset.

## Compatibility migration

Older SortSmith state used a randomly generated ID for the original `Everyday tidy` preset. Version 0.2 recognizes that legacy preset by name when the stable ID is absent, preserves its existing rule snapshot, assigns the stable bundled ID, and remaps watched-folder references that pointed at the old ID.

The remaining bundled packs are added when space is available. If all 50 preset slots are already occupied, SortSmith preserves the existing saved presets rather than deleting user data to make room. The UI reports how many bundled packs could not be added.

The same normalization runs when state is loaded and before state writes, so importing an older settings backup upgrades the preset catalog immediately instead of requiring an application restart.

## Safety model

Presets only define rules. Loading or saving a preset does not touch the filesystem. File changes continue to use the same root-containment validation, dry-run preview, collision handling, confirmation setting, execution journal, and undo path as manually created rules.
