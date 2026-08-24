import { useEffect, useState } from "react";
import { isEditingTarget, shortcutFor, type PageShortcut } from "./shortcuts";
import "./shortcuts.css";

interface KeyboardControllerProps {
  busy: boolean;
  canApply: boolean;
  canUndo: boolean;
  onNavigate: (page: PageShortcut) => void;
  onChooseFolder: () => void;
  onPreview: () => void;
  onApply: () => void;
  onUndo: () => void;
}

export function KeyboardController({ busy, canApply, canUndo, onNavigate, onChooseFolder, onPreview, onApply, onUndo }: KeyboardControllerProps) {
  const [showHelp, setShowHelp] = useState(false);

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape" && showHelp) {
        event.preventDefault();
        setShowHelp(false);
        return;
      }

      const action = shortcutFor({
        key: event.key,
        ctrlKey: event.ctrlKey,
        metaKey: event.metaKey,
        shiftKey: event.shiftKey,
        altKey: event.altKey,
        editing: isEditingTarget(event.target),
      });
      if (!action) return;

      event.preventDefault();
      if (action.kind === "showHelp") { setShowHelp(true); return; }
      if (action.kind === "navigate") { onNavigate(action.page); return; }
      if (busy) return;
      if (action.kind === "chooseFolder") { onChooseFolder(); return; }
      if (action.kind === "preview") { onPreview(); return; }
      if (action.kind === "apply" && canApply) { onApply(); return; }
      if (action.kind === "undo" && canUndo) onUndo();
    }

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [busy, canApply, canUndo, onApply, onChooseFolder, onNavigate, onPreview, onUndo, showHelp]);

  if (!showHelp) return null;

  return <div className="shortcut-backdrop" role="presentation" onMouseDown={() => setShowHelp(false)}>
    <section className="shortcut-dialog" role="dialog" aria-modal="true" aria-labelledby="shortcut-title" onMouseDown={event => event.stopPropagation()}>
      <div className="panel-head"><div><p className="eyebrow">Keyboard first</p><h2 id="shortcut-title">Quick actions</h2></div><button onClick={() => setShowHelp(false)} aria-label="Close keyboard shortcuts">Close</button></div>
      <dl className="shortcut-list">
        <div><dt>Alt + 1…7</dt><dd>Open Organize, Rules, Duplicates, Automation, History, Settings, or About.</dd></div>
        <div><dt>Ctrl/⌘ + O</dt><dd>Choose the working folder.</dd></div>
        <div><dt>Ctrl/⌘ + Enter</dt><dd>Run a dry-run preview.</dd></div>
        <div><dt>Ctrl/⌘ + Shift + Enter</dt><dd>Apply the reviewed preview when available.</dd></div>
        <div><dt>Ctrl/⌘ + Z</dt><dd>Undo the latest recorded operation when available.</dd></div>
        <div><dt>Shift + ?</dt><dd>Show this shortcut reference.</dd></div>
        <div><dt>Escape</dt><dd>Close this shortcut reference.</dd></div>
      </dl>
      <p className="hint">Shortcuts are disabled while focus is inside an input, textarea, select, or editable field so normal editing keys keep working.</p>
    </section>
  </div>;
}
