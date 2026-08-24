import { describe, expect, it } from "vitest";
import { BUNDLED_PRESET_IDS, bundledPresets, isBundledPresetId, upgradeBundledPresets } from "./bundledPresets";
import type { AppStateData, Preset } from "./types";

function stateWith(presets: Preset[]): AppStateData {
  return {
    schemaVersion: 1,
    settings: { theme: "system", reducedMotion: false, confirmBeforeApply: true, includeHidden: false, recursiveScan: false },
    rules: [],
    presets,
    watchedFolders: [],
    recentJournalIds: [],
  };
}

describe("bundled preset catalog", () => {
  it("uses stable unique preset and rule identifiers", () => {
    const presets = bundledPresets();
    expect(new Set(presets.map(preset => preset.id)).size).toBe(presets.length);
    for (const preset of presets) {
      expect(isBundledPresetId(preset.id)).toBe(true);
      expect(new Set(preset.rules.map(rule => rule.id)).size).toBe(preset.rules.length);
    }
  });

  it("adds missing bundled presets without replacing custom presets", () => {
    const custom: Preset = { id: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa", name: "Custom", description: "Keep me", rules: [] };
    const result = upgradeBundledPresets(stateWith([custom]));
    expect(result.changed).toBe(true);
    expect(result.missingPresetCount).toBe(0);
    expect(result.state.presets.some(preset => preset.id === custom.id)).toBe(true);
    expect(result.state.presets).toHaveLength(5);
  });

  it("migrates the legacy random Everyday tidy id and watched-folder reference", () => {
    const legacyId = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaa01";
    const legacy: Preset = {
      id: legacyId,
      name: "Everyday tidy",
      description: "legacy",
      rules: [{
        id: "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb",
        name: "Legacy custom-safe snapshot",
        enabled: true,
        matchAll: true,
        criteria: [{ kind: "extension", values: ["txt"] }],
        action: { kind: "moveTo", subdirectory: "Text" },
      }],
    };
    const input = stateWith([legacy]);
    input.watchedFolders = [{ id: "cccccccc-cccc-4ccc-8ccc-cccccccccccc", path: "/tmp", presetId: legacyId, intervalMinutes: 15, enabled: true }];

    const result = upgradeBundledPresets(input);
    const migrated = result.state.presets.find(preset => preset.id === BUNDLED_PRESET_IDS.everyday);
    expect(migrated?.rules[0].name).toBe("Legacy custom-safe snapshot");
    expect(result.state.watchedFolders[0].presetId).toBe(BUNDLED_PRESET_IDS.everyday);
  });

  it("does not mutate an already complete catalog", () => {
    const input = stateWith(bundledPresets());
    const result = upgradeBundledPresets(input);
    expect(result.changed).toBe(false);
    expect(result.state).toBe(input);
    expect(result.missingPresetCount).toBe(0);
  });

  it("never exceeds the backend fifty-preset limit", () => {
    const custom = Array.from({ length: 50 }, (_, index): Preset => ({
      id: `aaaaaaaa-aaaa-4aaa-8aaa-${String(index).padStart(12, "0")}`,
      name: `Custom ${index}`,
      description: "",
      rules: [],
    }));
    const result = upgradeBundledPresets(stateWith(custom));
    expect(result.state.presets).toHaveLength(50);
    expect(result.missingPresetCount).toBe(bundledPresets().length);
  });
});
