import { useEffect, useMemo, useState } from "react";
import packageMetadata from "../package.json";
import { backend } from "./api";
import { AutomationPage } from "./AutomationPage";
import { BUNDLED_PRESET_IDS, upgradeBundledPresets } from "./bundledPresets";
import { chooseFolder } from "./dialogs";
import { HistoryPage } from "./HistoryPage";
import { strings } from "./i18n";
import { KeyboardController } from "./KeyboardController";
import { RulesPage } from "./RulesPage";
import { SettingsDataTools } from "./SettingsDataTools";
import type { AppStateData, DuplicateGroup, PreviewResult } from "./types";
import { formatBytes, shortPath } from "./utils";

type Page = "organize" | "rules" | "duplicates" | "automation" | "history" | "settings" | "about";

const fallbackState: AppStateData = {
  schemaVersion: 1,
  settings: { theme: "system", reducedMotion: false, confirmBeforeApply: true, includeHidden: false, recursiveScan: false },
  rules: [], presets: [], watchedFolders: [], recentJournalIds: [],
};

function App() {
  const [page, setPage] = useState<Page>("organize");
  const [state, setState] = useState<AppStateData>(fallbackState);
  const [rootPath, setRootPath] = useState("");
  const [preview, setPreview] = useState<PreviewResult | null>(null);
  const [duplicates, setDuplicates] = useState<DuplicateGroup[]>([]);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState("Ready. No files are changed until you apply a preview.");
  const activeRules = useMemo(() => {
    if (state.rules.length) return state.rules;
    return state.presets.find(preset => preset.id === BUNDLED_PRESET_IDS.everyday)?.rules ?? state.presets[0]?.rules ?? [];
  }, [state]);

  useEffect(() => {
    let cancelled = false;
    backend.loadState().then(async loaded => {
      const upgrade = upgradeBundledPresets(loaded);
      if (upgrade.changed) {
        try {
          await backend.saveState(upgrade.state);
        } catch {
          if (!cancelled) {
            setState(loaded);
            setMessage("Saved settings loaded, but the bundled preset catalog could not be upgraded. Existing settings remain unchanged.");
          }
          return;
        }
      }
      if (cancelled) return;
      setState(upgrade.state);
      if (upgrade.missingPresetCount > 0) {
        setMessage(`${upgrade.missingPresetCount} bundled preset pack(s) could not be added because the saved preset limit is already full.`);
      }
    }).catch(() => {
      if (!cancelled) setMessage("Could not load saved settings. Safe defaults are active.");
    });
    return () => { cancelled = true; };
  }, []);

  useEffect(() => {
    const theme = state.settings.theme;
    document.documentElement.dataset.theme = theme;
    document.documentElement.dataset.reducedMotion = state.settings.reducedMotion ? "true" : "false";
  }, [state.settings]);

  useEffect(() => {
    const timer = window.setInterval(() => { backend.runDueWatches().catch(() => undefined); }, 60_000);
    return () => window.clearInterval(timer);
  }, []);

  async function persist(next: AppStateData) {
    const normalized = upgradeBundledPresets(next);
    try {
      await backend.saveState(normalized.state);
      setState(normalized.state);
      if (normalized.missingPresetCount > 0) {
        setMessage(`${normalized.missingPresetCount} bundled preset pack(s) could not be added because the saved preset limit is already full.`);
      }
    } catch (error) {
      setMessage(`The change could not be saved, so the previous settings remain active. ${String(error)}`);
    }
  }

  async function pickRootFolder() {
    try {
      const selected = await chooseFolder(rootPath);
      if (!selected) return;
      setRootPath(selected);
      setPreview(null);
      setDuplicates([]);
      setMessage("Folder selected. Run a preview before applying any changes.");
    } catch (error) {
      setMessage(`Folder picker failed: ${String(error)}`);
    }
  }

  async function runPreview() {
    if (!rootPath.trim()) { setMessage("Choose a folder first."); return; }
    setBusy(true); setMessage("Scanning metadata and evaluating rules…");
    try {
      const result = await backend.preview(rootPath.trim(), activeRules, state.settings.recursiveScan, state.settings.includeHidden);
      setPreview(result); setMessage(`Preview ready: ${result.operations.length} change${result.operations.length === 1 ? "" : "s"}.`);
    } catch (error) { setMessage(`Preview failed: ${String(error)}`); }
    finally { setBusy(false); }
  }

  async function applyPreview() {
    if (!preview || preview.operations.length === 0) return;
    if (state.settings.confirmBeforeApply && !window.confirm(`Apply ${preview.operations.length} reversible file changes?`)) return;
    setBusy(true);
    try {
      const report = await backend.execute(rootPath.trim(), preview);
      const next = { ...state, recentJournalIds: [report.journal.id, ...state.recentJournalIds].slice(0, 20) };
      await persist(next); setPreview(null);
      setMessage(`Completed ${report.completed} changes. ${report.errors.length ? `${report.errors.length} item(s) need attention.` : "Undo is available."}`);
    } catch (error) { setMessage(`Apply failed safely: ${String(error)}`); }
    finally { setBusy(false); }
  }

  async function undoLatest() {
    const journalId = state.recentJournalIds[0];
    if (!journalId) { setMessage("There is no recent SortSmith operation to undo."); return; }
    setBusy(true);
    try {
      const report = await backend.undo(journalId);
      await persist({ ...state, recentJournalIds: state.recentJournalIds.slice(1) });
      setMessage(`Restored ${report.completed} file(s). ${report.errors.length ? `${report.errors.length} item(s) could not be restored automatically.` : ""}`);
    } catch (error) { setMessage(`Undo could not complete: ${String(error)}`); }
    finally { setBusy(false); }
  }

  async function scanDuplicates() {
    if (!rootPath.trim()) { setMessage("Choose a folder first."); return; }
    setBusy(true);
    try { const groups = await backend.duplicates(rootPath.trim(), true, state.settings.includeHidden); setDuplicates(groups); setMessage(`Found ${groups.length} duplicate group(s). Nothing was deleted.`); }
    catch (error) { setMessage(`Duplicate scan failed: ${String(error)}`); }
    finally { setBusy(false); }
  }

  return <div className="app-shell">
    <KeyboardController
      busy={busy}
      canApply={Boolean(preview?.operations.length)}
      canUndo={state.recentJournalIds.length > 0}
      onNavigate={nextPage => setPage(nextPage)}
      onChooseFolder={() => void pickRootFolder()}
      onPreview={() => void runPreview()}
      onApply={() => void applyPreview()}
      onUndo={() => void undoLatest()}
    />
    <aside className="sidebar" aria-label="Main navigation">
      <div className="brand"><div className="brand-mark" aria-hidden="true">S</div><div><strong>SortSmith</strong><span>File Organizer</span></div></div>
      <nav>{(["organize","rules","duplicates","automation","history","settings","about"] as Page[]).map(item => <button key={item} className={page===item?"active":""} aria-current={page===item?"page":undefined} onClick={()=>setPage(item)}>{item[0].toUpperCase()+item.slice(1)}</button>)}</nav>
      <div className="sidebar-footer"><span className="privacy-dot"/>Offline-first & private · Shift+? shortcuts</div>
    </aside>

    <main className="content">
      <header className="topbar"><div><p className="eyebrow">{strings.appName}</p><h1>{titleFor(page)}</h1></div><div className="status" role="status" aria-live="polite">{busy ? "Working…" : message}</div></header>
      {page === "organize" && <Organize rootPath={rootPath} setRootPath={setRootPath} preview={preview} busy={busy} onChooseFolder={pickRootFolder} onPreview={runPreview} onApply={applyPreview} onUndo={undoLatest} canUndo={state.recentJournalIds.length>0} />}
      {page === "rules" && <RulesPage state={state} persist={persist} />}
      {page === "duplicates" && <DuplicatesPage rootPath={rootPath} setRootPath={setRootPath} groups={duplicates} onChooseFolder={pickRootFolder} onScan={scanDuplicates} busy={busy}/>} 
      {page === "automation" && <AutomationPage state={state} persist={persist} />}
      {page === "history" && <HistoryPage state={state} persist={persist} />}
      {page === "settings" && <SettingsPage state={state} persist={persist} />}
      {page === "about" && <AboutPage />}
      <footer>{strings.madeBy}</footer>
    </main>
  </div>;
}

function titleFor(page: Page) { return ({ organize:"Organize safely", rules:"Rules & presets", duplicates:"Duplicate candidates", automation:"Watched folders", history:"Undo history", settings:"Settings", about:"About SortSmith" } as const)[page]; }

function Organize({rootPath,setRootPath,preview,busy,onChooseFolder,onPreview,onApply,onUndo,canUndo}:{rootPath:string;setRootPath:(v:string)=>void;preview:PreviewResult|null;busy:boolean;onChooseFolder:()=>void;onPreview:()=>void;onApply:()=>void;onUndo:()=>void;canUndo:boolean}) {
  return <section className="stack">
    <div className="hero-card"><div><p className="eyebrow">Dry run first. Always.</p><h2>{strings.tagline}</h2><p>SortSmith evaluates file metadata locally, shows every planned move, and records a reversible journal when you apply.</p></div><div className="shield">↺</div></div>
    <div className="panel"><label htmlFor="folder">Folder to organize</label><div className="input-row"><input id="folder" value={rootPath} onChange={e=>setRootPath(e.target.value)} placeholder="Choose a folder or paste its path"/><button onClick={onChooseFolder} disabled={busy}>Choose folder</button><button className="primary" onClick={onPreview} disabled={busy}>{strings.dryRun}</button></div><p className="hint">The native picker grants only the folder you choose. SortSmith never uploads file contents.</p></div>
    <div className="stats-grid"><Stat label="Scanned" value={preview?.scannedFiles ?? 0}/><Stat label="Planned changes" value={preview?.operations.length ?? 0}/><Stat label="Untouched" value={preview?.ignoredFiles ?? 0}/><Stat label="Recoverable issues" value={preview?.recoverableErrors.length ?? 0}/></div>
    <div className="panel"><div className="panel-head"><div><h3>Change preview</h3><p>Review source → destination before applying.</p></div><div className="actions"><button onClick={onUndo} disabled={!canUndo||busy}>Undo latest</button><button className="primary" onClick={onApply} disabled={!preview?.operations.length||busy}>{strings.apply}</button></div></div>
      {!preview?.operations.length ? <Empty icon="✓" title="Nothing queued" text="Run a preview to see exactly what SortSmith would change."/> : <div className="table-wrap"><table><thead><tr><th>Rule</th><th>Source</th><th>Destination</th><th>Size</th></tr></thead><tbody>{preview.operations.map(op=><tr key={op.id}><td><span className="pill">{op.ruleName}</span></td><td title={op.source}>{shortPath(op.source)}</td><td title={op.destination}>{shortPath(op.destination)}</td><td>{formatBytes(op.size)}</td></tr>)}</tbody></table></div>}
    </div>
  </section>;
}

function DuplicatesPage({rootPath,setRootPath,groups,onChooseFolder,onScan,busy}:{rootPath:string;setRootPath:(v:string)=>void;groups:DuplicateGroup[];onChooseFolder:()=>void;onScan:()=>void;busy:boolean}) { return <section className="stack"><div className="panel"><label htmlFor="dup-folder">Folder to inspect</label><div className="input-row"><input id="dup-folder" value={rootPath} onChange={e=>setRootPath(e.target.value)} placeholder="Choose a folder or paste its path"/><button onClick={onChooseFolder} disabled={busy}>Choose folder</button><button className="primary" onClick={onScan} disabled={busy}>Find candidates</button></div><p className="hint">Uses BLAKE3 hashes after a size pre-filter. SortSmith reports duplicates but never auto-deletes them.</p></div><div className="panel">{groups.length===0?<Empty icon="≈" title="No duplicate groups shown" text="Start a scan to compare file content safely."/>:<div className="cards">{groups.map(g=><article className="dup-card" key={g.hash}><div className="panel-head"><strong>{g.files.length} identical files</strong><span>{formatBytes(g.size)} each</span></div>{g.files.map(f=><code key={f.path}>{f.path}</code>)}</article>)}</div>}</div></section>; }

function SettingsPage({state,persist}:{state:AppStateData;persist:(s:AppStateData)=>Promise<void>}) { const s=state.settings; const set=(patch:Partial<typeof s>)=>persist({...state,settings:{...s,...patch}}); return <section className="stack"><div className="panel settings"><h3>Appearance</h3><label>Theme<select value={s.theme} onChange={e=>set({theme:e.target.value as typeof s.theme})}><option value="system">System</option><option value="light">Light</option><option value="dark">Dark</option></select></label><Switch label="Reduce motion" checked={s.reducedMotion} onChange={v=>set({reducedMotion:v})}/></div><div className="panel settings"><h3>Organization</h3><Switch label="Confirm before applying changes" checked={s.confirmBeforeApply} onChange={v=>set({confirmBeforeApply:v})}/><Switch label="Scan subfolders" checked={s.recursiveScan} onChange={v=>set({recursiveScan:v})}/><Switch label="Include hidden files" checked={s.includeHidden} onChange={v=>set({includeHidden:v})}/></div><SettingsDataTools state={state} persist={persist}/><div className="panel"><h3>Privacy & data</h3><p>SortSmith works locally. It does not send filenames or file contents to a server. Operation journals contain paths needed for undo and stay in the app data directory.</p></div></section>; }

function AboutPage(){ return <section className="stack"><div className="about-card"><div className="logo-large">S</div><p className="eyebrow">Version {packageMetadata.version}</p><h2>SortSmith</h2><p>A private, reversible file organizer for Windows, macOS, and Linux.</p><div className="link-grid"><a href="https://github.com/sanskarIN" target="_blank" rel="noreferrer">GitHub</a><a href="https://buymeacoffee.com/sanskarIN" target="_blank" rel="noreferrer">Buy Me a Coffee</a><a href="mailto:sanskarin@outlook.in">Business email</a><a href="mailto:sanskarin.business@gmail.com">Business Gmail</a><a href="mailto:supportramsandesh@gmail.com">Support</a></div><p>Licensed under Apache-2.0.</p><strong>{strings.madeBy}</strong></div></section>; }

function Stat({label,value}:{label:string;value:number}){return <div className="stat"><span>{label}</span><strong>{value}</strong></div>}
function Empty({icon,title,text}:{icon:string;title:string;text:string}){return <div className="empty"><div className="empty-icon">{icon}</div><h4>{title}</h4><p>{text}</p></div>}
function Switch({label,checked,onChange}:{label:string;checked:boolean;onChange:(v:boolean)=>void}){return <label className="switch-row"><span>{label}</span><input type="checkbox" checked={checked} onChange={e=>onChange(e.target.checked)}/></label>}

export default App;
