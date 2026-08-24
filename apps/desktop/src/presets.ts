import type { Action, Criterion, Preset, Rule } from "./types";

export function cloneCriterion(criterion: Criterion): Criterion {
  if (criterion.kind === "extension" || criterion.kind === "mimePrefix") {
    return { ...criterion, values: [...criterion.values] };
  }
  return { ...criterion };
}

export function cloneAction(action: Action): Action {
  return { ...action };
}

export function cloneRules(rules: Rule[]): Rule[] {
  return rules.map(rule => ({
    ...rule,
    criteria: rule.criteria.map(cloneCriterion),
    action: cloneAction(rule.action),
  }));
}

export function createPreset(name: string, description: string, rules: Rule[]): Preset {
  const normalizedName = name.trim();
  const normalizedDescription = description.trim();

  if (!normalizedName || normalizedName.length > 128) {
    throw new Error("Preset name must contain 1 to 128 characters.");
  }
  if (normalizedDescription.length > 512) {
    throw new Error("Preset description must be 512 characters or fewer.");
  }
  if (!rules.length) {
    throw new Error("Add or load at least one rule before saving a preset.");
  }

  return {
    id: crypto.randomUUID(),
    name: normalizedName,
    description: normalizedDescription,
    rules: cloneRules(rules),
  };
}

export function renamePreset(preset: Preset, name: string, description: string): Preset {
  const replacement = createPreset(name, description, preset.rules);
  return { ...replacement, id: preset.id };
}
