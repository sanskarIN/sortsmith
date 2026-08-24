import { describe, expect, it } from "vitest";
import { cloneRules, createPreset, renamePreset } from "./presets";
import type { Rule } from "./types";

const rule: Rule = {
  id: "11111111-1111-4111-8111-111111111111",
  name: "Images",
  enabled: true,
  matchAll: true,
  criteria: [{ kind: "extension", values: ["png", "jpg"] }],
  action: { kind: "moveTo", subdirectory: "Images" },
};

describe("preset helpers", () => {
  it("deep-clones rule criteria collections", () => {
    const cloned = cloneRules([rule]);
    const criterion = cloned[0].criteria[0];
    expect(criterion.kind).toBe("extension");
    if (criterion.kind !== "extension") throw new Error("unexpected criterion kind");
    criterion.values.push("webp");
    expect(rule.criteria[0]).toEqual({ kind: "extension", values: ["png", "jpg"] });
  });

  it("normalizes preset metadata and clones rules", () => {
    const preset = createPreset("  Photos  ", "  Camera files  ", [rule]);
    expect(preset.name).toBe("Photos");
    expect(preset.description).toBe("Camera files");
    expect(preset.rules).toEqual([rule]);
    expect(preset.rules).not.toBe([rule]);
  });

  it("rejects empty presets", () => {
    expect(() => createPreset("Empty", "", [])).toThrow(/at least one rule/i);
  });

  it("renames a preset without changing its identifier or rules", () => {
    const preset = createPreset("Photos", "Old", [rule]);
    const renamed = renamePreset(preset, "Media", "Updated");
    expect(renamed.id).toBe(preset.id);
    expect(renamed.name).toBe("Media");
    expect(renamed.description).toBe("Updated");
    expect(renamed.rules).toEqual(preset.rules);
  });
});
