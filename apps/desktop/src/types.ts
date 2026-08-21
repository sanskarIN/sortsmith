export type Criterion =
  | { kind: "extension"; values: string[] }
  | { kind: "mimePrefix"; values: string[] }
  | { kind: "modifiedOlderThanDays"; days: number }
  | { kind: "sizeRange"; minBytes?: number; maxBytes?: number }
  | { kind: "nameRegex"; pattern: string };

export type Action =
  | { kind: "moveTo"; subdirectory: string }
  | { kind: "renamePrefix"; prefix: string }
  | { kind: "renameTemplate"; template: string };

export interface Rule { id: string; name: string; enabled: boolean; matchAll: boolean; criteria: Criterion[]; action: Action }
export interface Preset { id: string; name: string; description: string; rules: Rule[] }
export interface Settings { theme: "system" | "light" | "dark"; reducedMotion: boolean; confirmBeforeApply: boolean; includeHidden: boolean; recursiveScan: boolean }
export interface WatchedFolder { id: string; path: string; presetId?: string | null; intervalMinutes: number; enabled: boolean; lastRunAt?: string | null }
export interface AppStateData { schemaVersion: number; settings: Settings; rules: Rule[]; presets: Preset[]; watchedFolders: WatchedFolder[]; recentJournalIds: string[] }
export interface PlannedOperation { id: string; source: string; destination: string; ruleId: string; ruleName: string; size: number }
export interface PreviewResult { operations: PlannedOperation[]; scannedFiles: number; ignoredFiles: number; recoverableErrors: string[] }
export interface ExecutionReport { journal: { id: string; createdAt: string; root: string; entries: unknown[] }; completed: number; errors: string[] }
export interface DuplicateGroup { hash: string; size: number; files: { path: string; size: number }[] }
