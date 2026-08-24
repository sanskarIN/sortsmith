# Accessibility

SortSmith targets WCAG-oriented desktop accessibility: semantic buttons/labels, visible keyboard focus, non-color-only status text, scalable system fonts, high-contrast light/dark palettes, responsive layouts, and a reduced-motion preference.

Keyboard-first interaction includes direct `Alt+1` through `Alt+7` page navigation, modifier-based folder/preview/apply/undo actions, and a `Shift+?` shortcut reference. Application shortcuts are ignored while focus is inside text inputs, textareas, selects, or editable content so normal editing behavior remains available. The shortcut reference uses a labelled modal dialog and supports `Escape` dismissal.

Manual release checks should cover keyboard-only navigation, shortcut conflict checks on Windows/macOS/Linux, 200% zoom, light/dark contrast, focus order, form labels, status announcements, modal focus behavior, and screen-reader naming on Windows and macOS. The full shortcut reference is in [`keyboard-shortcuts.md`](keyboard-shortcuts.md).
