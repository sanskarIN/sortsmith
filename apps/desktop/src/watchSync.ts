import type { AppStateData } from "./types";

export function mergeWatchedRunState(current: AppStateData, persisted: AppStateData): AppStateData {
  return {
    ...current,
    watchedFolders: persisted.watchedFolders,
    recentJournalIds: persisted.recentJournalIds,
  };
}

export function summarizeWatchedRunMessages(messages: string[]): string | null {
  const meaningful = messages.map(message => message.trim()).filter(Boolean);
  if (meaningful.length === 0) return null;
  if (meaningful.length === 1) return meaningful[0];
  return meaningful.join(" ");
}
