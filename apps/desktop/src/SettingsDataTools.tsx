import { useState } from "react";
import { backend } from "./api";
import { chooseSettingsExport, chooseSettingsImport } from "./dialogs";
import type { AppStateData } from "./types";

interface SettingsDataToolsProps {
  state: AppStateData;
  persist: (state: AppStateData) => Promise<void>;
}

export function SettingsDataTools({ state, persist }: SettingsDataToolsProps) {
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState("Backups contain preferences, rules, presets, and watched-folder configuration. File contents are never included.");

  async function exportSettings() {
    setBusy(true);
    try {
      const path = await chooseSettingsExport();
      if (!path) return;
      const portableState = { ...state, recentJournalIds: [] };
      await backend.exportState(path, portableState);
      setMessage("Settings backup exported successfully.");
    } catch (error) {
      setMessage(`Export failed: ${String(error)}`);
    } finally {
      setBusy(false);
    }
  }

  async function importSettings() {
    setBusy(true);
    try {
      const path = await chooseSettingsImport();
      if (!path) return;
      const imported = await backend.importState(path);
      if (!window.confirm("Replace your current SortSmith preferences, rules, presets, and watched folders with this backup?")) return;
      await persist({ ...imported, recentJournalIds: state.recentJournalIds });
      setMessage("Settings backup imported. Local undo history was preserved.");
    } catch (error) {
      setMessage(`Import failed: ${String(error)}`);
    } finally {
      setBusy(false);
    }
  }

  return <div className="panel settings-data-tools">
    <div className="panel-head">
      <div><h3>Backup & restore</h3><p>Portable JSON settings backups stay on your device.</p></div>
      <div className="actions">
        <button onClick={importSettings} disabled={busy}>Import</button>
        <button className="primary" onClick={exportSettings} disabled={busy}>Export</button>
      </div>
    </div>
    <p className="hint" role="status" aria-live="polite">{busy ? "Working with the selected backup…" : message}</p>
  </div>;
}
