import { describe, expect, it } from "vitest";
import type { AppStateData } from "./types";
import { mergeWatchedRunState, summarizeWatchedRunMessages } from "./watchSync";

function state(overrides: Partial<AppStateData> = {}): AppStateData {
  return {
    schemaVersion: 1,
    settings: {
      theme: "system",
      reducedMotion: false,
      confirmBeforeApply: true,
      includeHidden: false,
      recursiveScan: false,
    },
    rules: [],
    presets: [],
    watchedFolders: [],
    recentJournalIds: [],
    ...overrides,
  };
}

describe("mergeWatchedRunState", () => {
  it("refreshes automation fields without overwriting current interactive settings", () => {
    const current = state({
      settings: {
        theme: "dark",
        reducedMotion: true,
        confirmBeforeApply: false,
        includeHidden: true,
        recursiveScan: true,
      },
      rules: [{
        id: "rule-current",
        name: "Current rule",
        enabled: true,
        matchAll: true,
        criteria: [{ kind: "extension", values: ["txt"] }],
        action: { kind: "moveTo", subdirectory: "Text" },
      }],
    });
    const persisted = state({
      watchedFolders: [{
        id: "watch-1",
        path: "/tmp/inbox",
        presetId: "preset-1",
        intervalMinutes: 60,
        enabled: true,
        lastRunAt: "2026-08-25T11:30:00Z",
      }],
      recentJournalIds: ["journal-1"],
    });

    const merged = mergeWatchedRunState(current, persisted);

    expect(merged.settings).toEqual(current.settings);
    expect(merged.rules).toEqual(current.rules);
    expect(merged.watchedFolders).toEqual(persisted.watchedFolders);
    expect(merged.recentJournalIds).toEqual(["journal-1"]);
  });
});

describe("summarizeWatchedRunMessages", () => {
  it("returns null when there is nothing meaningful to report", () => {
    expect(summarizeWatchedRunMessages([])).toBeNull();
    expect(summarizeWatchedRunMessages([" ", "\n"])).toBeNull();
  });

  it("combines multiple backend messages into one status update", () => {
    expect(summarizeWatchedRunMessages([
      "A watched folder completed 2 change(s).",
      "A watched folder is unavailable.",
    ])).toBe("A watched folder completed 2 change(s). A watched folder is unavailable.");
  });
});
