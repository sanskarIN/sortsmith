import { useEffect, useMemo, useState } from "react";
import { isBundledPresetId } from "./bundledPresets";
import { cloneRules, createPreset, renamePreset } from "./presets";
import type { AppStateData, Rule } from "./types";

interface PresetManagerProps {
  state: AppStateData;
  effectiveRules: Rule[];
  persist: (state: AppStateData) => Promise<void>;
}

export function PresetManager({ state, effectiveRules, persist }: PresetManagerProps) {
  const [selectedId, setSelectedId] = useState(state.presets[0]?.id ?? "");
  const [newName, setNewName] = useState("");
  const [newDescription, setNewDescription] = useState("");
  const [editName, setEditName] = useState("");
  const [editDescription, setEditDescription] = useState("");
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");

  const selected = state.presets.find(preset => preset.id === selectedId);
  const selectedIsBuiltIn = isBundledPresetId(selected?.id);
  const selectedIsUsedByWatch = useMemo(
    () => state.watchedFolders.some(watch => watch.presetId === selected?.id),
    [selected?.id, state.watchedFolders],
  );

  useEffect(() => {
    if (!state.presets.length) {
      setSelectedId("");
      return;
    }
    if (!state.presets.some(preset => preset.id === selectedId)) {
      setSelectedId(state.presets[0].id);
    }
  }, [selectedId, state.presets]);

  useEffect(() => {
    setEditName(selected?.name ?? "");
    setEditDescription(selected?.description ?? "");
  }, [selected?.description, selected?.id, selected?.name]);

  async function saveCurrentRules() {
    setError("");
    setMessage("");
    try {
      if (state.presets.length >= 50) throw new Error("This build supports up to 50 saved presets.");
      const preset = createPreset(newName, newDescription, effectiveRules);
      await persist({ ...state, presets: [...state.presets, preset] });
      setSelectedId(preset.id);
      setNewName("");
      setNewDescription("");
      setMessage(`Saved preset “${preset.name}”.`);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    }
  }

  async function loadSelected() {
    if (!selected) return;
    setError("");
    setMessage("");
    try {
      await persist({ ...state, rules: cloneRules(selected.rules) });
      setMessage(`Loaded “${selected.name}” into the active rule set.`);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    }
  }

  async function updateSelected() {
    if (!selected || selectedIsBuiltIn) return;
    setError("");
    setMessage("");
    try {
      const updated = renamePreset(selected, editName, editDescription);
      await persist({
        ...state,
        presets: state.presets.map(preset => preset.id === selected.id ? updated : preset),
      });
      setMessage(`Updated preset “${updated.name}”.`);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    }
  }

  async function deleteSelected() {
    if (!selected || selectedIsBuiltIn || selectedIsUsedByWatch) return;
    if (!window.confirm(`Delete saved preset “${selected.name}”? This does not change the currently active rules.`)) return;
    setError("");
    setMessage("");
    try {
      await persist({ ...state, presets: state.presets.filter(preset => preset.id !== selected.id) });
      setSelectedId(state.presets[0]?.id ?? "");
      setMessage(`Deleted preset “${selected.name}”.`);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    }
  }

  return <div className="panel">
    <div className="panel-head">
      <div>
        <h3>Saved presets</h3>
        <p>Load bundled packs or snapshot the active rule set for later reuse.</p>
      </div>
      <span className="pill">{state.presets.length}/50 presets</span>
    </div>

    <div className="form-grid">
      <label>Preset to manage
        <select value={selectedId} onChange={event => setSelectedId(event.target.value)}>
          {state.presets.map(preset => <option key={preset.id} value={preset.id}>{preset.name}{isBundledPresetId(preset.id) ? " (built in)" : ""}</option>)}
        </select>
      </label>
      <label>Name
        <input value={editName} onChange={event => setEditName(event.target.value)} disabled={!selected || selectedIsBuiltIn}/>
      </label>
      <label>Description
        <input value={editDescription} onChange={event => setEditDescription(event.target.value)} disabled={!selected || selectedIsBuiltIn}/>
      </label>
    </div>
    <div className="actions">
      <button className="primary" onClick={() => void loadSelected()} disabled={!selected}>Load preset</button>
      <button onClick={() => void updateSelected()} disabled={!selected || selectedIsBuiltIn}>Save metadata</button>
      <button
        onClick={() => void deleteSelected()}
        disabled={!selected || selectedIsBuiltIn || selectedIsUsedByWatch}
        title={selectedIsBuiltIn ? "Bundled presets are protected so the built-in library remains recoverable." : selectedIsUsedByWatch ? "Remove this preset from watched folders before deleting it." : undefined}
      >Delete preset</button>
    </div>

    <hr/>
    <div className="form-grid">
      <label>New preset name<input value={newName} onChange={event => setNewName(event.target.value)} placeholder="My downloads cleanup"/></label>
      <label>New preset description<input value={newDescription} onChange={event => setNewDescription(event.target.value)} placeholder="Optional note about when to use it"/></label>
    </div>
    <div className="actions"><button onClick={() => void saveCurrentRules()} disabled={!effectiveRules.length}>Save active rules as preset</button></div>
    {selectedIsBuiltIn && <p className="hint">Bundled presets can be loaded but not renamed or deleted. Save a customized active rule set as a new preset instead.</p>}
    {!selectedIsBuiltIn && selectedIsUsedByWatch && <p className="hint">This preset is assigned to a watched folder, so it cannot be deleted until that assignment is changed.</p>}
    {message && <p className="hint" role="status">{message}</p>}
    {error && <p className="hint" role="alert">{error}</p>}
  </div>;
}
