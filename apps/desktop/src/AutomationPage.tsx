import { useState } from "react";
import { chooseFolder } from "./dialogs";
import type { AppStateData, WatchedFolder } from "./types";

interface AutomationPageProps {
  state: AppStateData;
  persist: (state: AppStateData) => Promise<void>;
}

export function AutomationPage({ state, persist }: AutomationPageProps) {
  const [path, setPath] = useState("");
  const [interval, setIntervalValue] = useState(60);
  const [presetId, setPresetId] = useState(state.presets[0]?.id ?? "");
  const [message, setMessage] = useState("Automation runs only while SortSmith is open and every run remains reversible.");

  async function choose() {
    try {
      const selected = await chooseFolder(path);
      if (selected) setPath(selected);
    } catch (error) {
      setMessage(`Folder picker failed: ${String(error)}`);
    }
  }

  async function add() {
    const normalizedPath = path.trim();
    if (!normalizedPath) { setMessage("Choose a folder before adding a watch."); return; }
    const safeInterval = normalizeInterval(interval);
    if (!presetId) { setMessage("Choose a preset for the watched folder."); return; }
    const watch: WatchedFolder = {
      id: crypto.randomUUID(),
      path: normalizedPath,
      presetId,
      intervalMinutes: safeInterval,
      enabled: true,
      lastRunAt: null,
    };
    await persist({ ...state, watchedFolders: [...state.watchedFolders, watch] });
    setPath("");
    setIntervalValue(60);
    setMessage("Watched folder added. It will run when its interval becomes due while SortSmith is open.");
  }

  async function updateWatch(id: string, patch: Partial<WatchedFolder>) {
    await persist({ ...state, watchedFolders: state.watchedFolders.map(watch => watch.id === id ? { ...watch, ...patch } : watch) });
  }

  async function removeWatch(id: string) {
    await persist({ ...state, watchedFolders: state.watchedFolders.filter(watch => watch.id !== id) });
  }

  return <section className="stack">
    <div className="panel">
      <h3>Add watched folder</h3>
      <div className="form-grid">
        <label>Folder<input value={path} onChange={event => setPath(event.target.value)} placeholder="Choose a folder or paste its path"/></label>
        <label>Preset<select value={presetId} onChange={event => setPresetId(event.target.value)}>{state.presets.map(preset => <option key={preset.id} value={preset.id}>{preset.name}</option>)}</select></label>
        <label>Interval minutes<input type="number" min={5} max={10080} value={interval} onChange={event => setIntervalValue(Number(event.target.value))}/></label>
      </div>
      <div className="actions"><button onClick={() => void choose()}>Choose folder</button><button className="primary" onClick={() => void add()}>Add watch</button></div>
      <p className="hint" role="status" aria-live="polite">{message}</p>
    </div>

    <div className="panel">
      <div className="panel-head"><div><h3>Watched folders</h3><p>Each folder has its own preset, interval, and enable switch.</p></div></div>
      {state.watchedFolders.length === 0 ? <div className="empty"><div className="empty-icon">⏱</div><h4>No watched folders</h4><p>Add a folder and choose how often SortSmith should organize it while the app is open.</p></div> : <div className="cards">
        {state.watchedFolders.map(watch => <article className="rule-card" key={watch.id}>
          <div style={{ flex: 1, minWidth: 0 }}>
            <span className="pill">{watch.enabled ? "Enabled" : "Paused"}</span>
            <h4 title={watch.path}>{watch.path}</h4>
            <p>Last run: {watch.lastRunAt ? new Date(watch.lastRunAt).toLocaleString() : "Never"}</p>
            <div className="input-row">
              <label>Preset<select value={watch.presetId ?? ""} onChange={event => void updateWatch(watch.id, { presetId: event.target.value || null })}><option value="">No preset</option>{state.presets.map(preset => <option key={preset.id} value={preset.id}>{preset.name}</option>)}</select></label>
              <label>Minutes<input className="small-input" type="number" min={5} max={10080} value={watch.intervalMinutes} onChange={event => void updateWatch(watch.id, { intervalMinutes: normalizeInterval(Number(event.target.value)) })}/></label>
            </div>
          </div>
          <div className="actions"><button onClick={() => void updateWatch(watch.id, { enabled: !watch.enabled })}>{watch.enabled ? "Pause" : "Enable"}</button><button onClick={() => void removeWatch(watch.id)}>Remove</button></div>
        </article>)}
      </div>}
    </div>
  </section>;
}

function normalizeInterval(value: number) {
  if (!Number.isFinite(value)) return 60;
  return Math.min(10080, Math.max(5, Math.round(value)));
}
