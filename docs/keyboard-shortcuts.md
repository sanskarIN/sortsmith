# Keyboard shortcuts

SortSmith keeps high-frequency organizer actions available from the keyboard while preserving normal text-editing behavior inside inputs, textareas, selects, and editable fields.

| Shortcut | Action |
|---|---|
| `Alt+1` | Open Organize |
| `Alt+2` | Open Rules |
| `Alt+3` | Open Duplicates |
| `Alt+4` | Open Automation |
| `Alt+5` | Open History |
| `Alt+6` | Open Settings |
| `Alt+7` | Open About |
| `Ctrl+O` / `Cmd+O` | Choose the working folder |
| `Ctrl+Enter` / `Cmd+Enter` | Run a dry-run preview |
| `Ctrl+Shift+Enter` / `Cmd+Shift+Enter` | Apply the reviewed preview when one is available |
| `Ctrl+Z` / `Cmd+Z` | Undo the latest recorded SortSmith operation when available |
| `Shift+?` | Open the in-app shortcut reference |
| `Escape` | Close the shortcut reference |

## Safety behavior

Apply and undo shortcuts honor the same availability and confirmation rules as their visible buttons. A shortcut cannot bypass the configured confirmation-before-apply setting, and unavailable actions remain unavailable while SortSmith is busy or when no preview/journal exists.

Shortcut handling intentionally does not intercept the application-level combinations while focus is inside a form control or content-editable element. This preserves ordinary editing commands such as undo inside a text field.

The shortcut reference is also available in the running app by pressing `Shift+?` outside an editor.
