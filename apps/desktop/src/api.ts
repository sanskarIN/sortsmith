import { invoke } from "@tauri-apps/api/core";
import type { AppStateData, DuplicateGroup, ExecutionReport, PreviewResult, Rule } from "./types";

export const backend = {
  loadState: () => invoke<AppStateData>("load_state"),
  saveState: (state: AppStateData) => invoke<void>("save_state", { state }),
  preview: (root: string, rules: Rule[], recursive: boolean, includeHidden: boolean) => invoke<PreviewResult>("preview", { root, rules, recursive, includeHidden }),
  execute: (root: string, preview: PreviewResult) => invoke<ExecutionReport>("execute", { root, preview }),
  undo: (journalId: string) => invoke<ExecutionReport>("undo", { journalId }),
  duplicates: (root: string, recursive: boolean, includeHidden: boolean) => invoke<DuplicateGroup[]>("find_duplicate_candidates", { root, recursive, includeHidden }),
  exportState: (path: string, state: AppStateData) => invoke<void>("export_state", { path, state }),
  importState: (path: string) => invoke<AppStateData>("import_state", { path }),
  runDueWatches: () => invoke<string[]>("run_due_watches"),
};
