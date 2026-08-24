import { describe, expect, it } from "vitest";
import { shortcutFor } from "./shortcuts";

const base = { ctrlKey: false, metaKey: false, shiftKey: false, altKey: false, editing: false };

describe("keyboard shortcuts", () => {
  it("maps platform modifier actions", () => {
    expect(shortcutFor({ ...base, key: "o", ctrlKey: true })).toEqual({ kind: "chooseFolder" });
    expect(shortcutFor({ ...base, key: "Enter", metaKey: true })).toEqual({ kind: "preview" });
    expect(shortcutFor({ ...base, key: "Enter", metaKey: true, shiftKey: true })).toEqual({ kind: "apply" });
    expect(shortcutFor({ ...base, key: "z", ctrlKey: true })).toEqual({ kind: "undo" });
  });

  it("maps Alt plus number to primary pages", () => {
    expect(shortcutFor({ ...base, key: "1", altKey: true })).toEqual({ kind: "navigate", page: "organize" });
    expect(shortcutFor({ ...base, key: "7", altKey: true })).toEqual({ kind: "navigate", page: "about" });
  });

  it("does not steal editing shortcuts from form controls", () => {
    expect(shortcutFor({ ...base, key: "z", ctrlKey: true, editing: true })).toBeNull();
    expect(shortcutFor({ ...base, key: "Enter", metaKey: true, editing: true })).toBeNull();
  });

  it("opens help with question mark outside editors", () => {
    expect(shortcutFor({ ...base, key: "?", shiftKey: true })).toEqual({ kind: "showHelp" });
  });
});
