export type PageShortcut = "organize" | "rules" | "duplicates" | "automation" | "history" | "settings" | "about";

export type QuickAction =
  | { kind: "navigate"; page: PageShortcut }
  | { kind: "chooseFolder" }
  | { kind: "preview" }
  | { kind: "apply" }
  | { kind: "undo" }
  | { kind: "showHelp" };

export interface ShortcutInput {
  key: string;
  ctrlKey: boolean;
  metaKey: boolean;
  shiftKey: boolean;
  altKey: boolean;
  editing: boolean;
}

const pageByNumber: Record<string, PageShortcut> = {
  "1": "organize",
  "2": "rules",
  "3": "duplicates",
  "4": "automation",
  "5": "history",
  "6": "settings",
  "7": "about",
};

export function shortcutFor(input: ShortcutInput): QuickAction | null {
  const key = input.key.toLowerCase();
  const modifier = input.ctrlKey || input.metaKey;

  if (!input.editing && input.shiftKey && !modifier && !input.altKey && input.key === "?") {
    return { kind: "showHelp" };
  }
  if (input.editing) return null;

  if (input.altKey && !modifier && !input.shiftKey && pageByNumber[key]) {
    return { kind: "navigate", page: pageByNumber[key] };
  }
  if (!modifier || input.altKey) return null;

  if (key === "o" && !input.shiftKey) return { kind: "chooseFolder" };
  if (key === "enter" && input.shiftKey) return { kind: "apply" };
  if (key === "enter") return { kind: "preview" };
  if (key === "z" && !input.shiftKey) return { kind: "undo" };
  return null;
}

export function isEditingTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tagName = target.tagName.toLowerCase();
  return target.isContentEditable || tagName === "input" || tagName === "textarea" || tagName === "select";
}
