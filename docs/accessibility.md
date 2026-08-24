# Accessibility

SortSmith targets WCAG-oriented desktop accessibility: semantic buttons/labels, visible keyboard focus, non-color-only status text, scalable system fonts, high-contrast light/dark palettes, responsive layouts, and a reduced-motion preference.

Keyboard-first interaction includes direct `Alt+1` through `Alt+7` page navigation, modifier-based folder/preview/apply/undo actions, and a `Shift+?` shortcut reference. Application shortcuts are ignored while focus is inside text inputs, textareas, selects, or editable content so normal editing behavior remains available.

The shortcut reference uses a labelled modal dialog with descriptive text. When opened, focus moves to its Close button. It supports `Escape` and outside-click dismissal, and closing the dialog restores focus to the control that was active before the dialog opened. This keeps keyboard users anchored in the workflow instead of dropping focus back to the document body.

## 0.2.0 manual release checks

Automated shortcut-resolution tests do not prove platform accessibility. Before publishing `v0.2.0`, perform keyboard-only checks on Windows, macOS, and Linux covering:

- `Alt+1` through `Alt+7` navigation and platform shortcut conflicts;
- `Ctrl/Cmd+O`, preview, apply, and undo shortcuts;
- unchanged editing behavior inside inputs, textareas, selects, and editable content;
- shortcut dialog focus entry, visible focus, Escape dismissal, outside-click dismissal, and focus restoration;
- logical focus order through sidebar navigation, forms, tables, preset controls, history, and settings;
- 200% zoom and responsive reflow without hidden controls or required horizontal page scrolling;
- light, dark, and system-theme contrast;
- reduced-motion behavior;
- form labels, button accessible names, `aria-current`, status announcements, and dialog naming;
- screen-reader naming on at least Windows and macOS using platform-appropriate assistive technology;
- native folder/import/export dialog usability from the keyboard.

Record the tested operating system, app artifact, assistive technology where applicable, and any exceptions in the release evidence. The full shortcut reference is in [`keyboard-shortcuts.md`](keyboard-shortcuts.md).
