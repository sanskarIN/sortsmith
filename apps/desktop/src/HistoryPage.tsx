import { useCallback, useEffect, useState } from "react";
import { backend } from "./api";
import type { AppStateData, JournalSummary } from "./types";
import { shortPath } from "./utils";

interface HistoryPageProps {
  state: AppStateData;
  persist: (state: AppStateData) => Promise<boolean>;
}

export function HistoryPage({ state, persist }: HistoryPageProps) {
  const [journals, setJournals] = useState<JournalSummary[]>([]);
  const [busyId, setBusyId] = useState<string | null>(null);
  const [message, setMessage] = useState("Undo history is stored locally and never uploaded.");

  const refresh = useCallback(async () => {
    try {
      setJournals(await backend.listJournals());
    } catch (error) {
      setMessage(`Could not read undo history: ${String(error)}`);
    }
  }, []);

  useEffect(() => { void refresh(); }, [refresh]);

  async function undoSelected(journal: JournalSummary) {
    if (journal.availableToUndo === 0) return;
    if (!window.confirm(`Undo up to ${journal.availableToUndo} file change(s) from this operation?`)) return;
    setBusyId(journal.id);
    try {
      const report = await backend.undo(journal.id);
      const saved = await persist({ ...state, recentJournalIds: state.recentJournalIds.filter(id => id !== journal.id) });
      setMessage(`Restored ${report.completed} file(s).${report.errors.length ? ` ${report.errors.length} item(s) could not be restored automatically.` : ""}${saved ? "" : " The files were restored, but the recent-history state could not be saved."}`);
      await refresh();
    } catch (error) {
      setMessage(`Undo could not complete: ${String(error)}`);
    } finally {
      setBusyId(null);
    }
  }

  return <section className="stack">
    <div className="panel">
      <div className="panel-head">
        <div><h3>Operation history</h3><p>Choose a reversible operation instead of being limited to the latest one.</p></div>
        <button onClick={() => void refresh()} disabled={busyId !== null}>Refresh</button>
      </div>
      <p className="hint" role="status" aria-live="polite">{message}</p>
    </div>

    <div className="panel">
      {journals.length === 0 ? <div className="empty"><div className="empty-icon">↺</div><h4>No local journals yet</h4><p>Apply a SortSmith preview to create a reversible operation journal.</p></div> : <div className="cards">
        {journals.map(journal => <article className="rule-card" key={journal.id}>
          <div>
            <span className="pill">{journal.availableToUndo} of {journal.entryCount} available</span>
            <h4>{new Date(journal.createdAt).toLocaleString()}</h4>
            <p title={journal.root}>{shortPath(journal.root, 68)}</p>
          </div>
          <button className={journal.availableToUndo > 0 ? "primary" : ""} disabled={journal.availableToUndo === 0 || busyId !== null} onClick={() => void undoSelected(journal)}>
            {busyId === journal.id ? "Undoing…" : journal.availableToUndo > 0 ? "Undo operation" : "Already restored"}
          </button>
        </article>)}
      </div>}
    </div>
  </section>;
}
