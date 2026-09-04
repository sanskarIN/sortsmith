import { useMemo, useState } from "react";
import { PresetManager } from "./PresetManager";
import type { Action, AppStateData, Criterion, Rule } from "./types";

type CriterionKind = Criterion["kind"];
type ActionKind = Action["kind"];
type CriterionDraft = { id: string; kind: CriterionKind; primary: string; secondary: string };

interface RulesPageProps {
  state: AppStateData;
  persist: (state: AppStateData) => Promise<boolean>;
}

const criterionLabels: Record<CriterionKind, string> = {
  extension: "Extension",
  mimePrefix: "MIME type prefix",
  modifiedOlderThanDays: "Modified age",
  sizeRange: "Size range",
  nameRegex: "Filename pattern",
};

const actionLabels: Record<ActionKind, string> = {
  moveTo: "Move into folder",
  renamePrefix: "Add filename prefix",
  renameTemplate: "Rename with template",
};

function freshCriterion(kind: CriterionKind = "extension"): CriterionDraft {
  return { id: crypto.randomUUID(), kind, primary: "", secondary: "" };
}

export function RulesPage({ state, persist }: RulesPageProps) {
  const [name, setName] = useState("");
  const [matchAll, setMatchAll] = useState(true);
  const [criteria, setCriteria] = useState<CriterionDraft[]>([freshCriterion()]);
  const [actionKind, setActionKind] = useState<ActionKind>("moveTo");
  const [actionValue, setActionValue] = useState("");
  const [error, setError] = useState("");

  const presetRules = state.presets[0]?.rules ?? [];
  const effectiveRules = state.rules.length ? state.rules : presetRules;
  const sourceLabel = state.rules.length ? "Your custom rule set" : `Using ${state.presets[0]?.name ?? "the built-in"} preset`;

  const criterionHelp = useMemo(() => ({
    extension: "Comma-separated extensions, for example: png, jpg, webp",
    mimePrefix: "Comma-separated MIME prefixes, for example: image/, application/pdf",
    modifiedOlderThanDays: "Number of days old, for example: 30",
    sizeRange: "Minimum and maximum size in MiB; either side may be blank",
    nameRegex: "Regular expression matched against the filename, for example: ^Screenshot",
  } satisfies Record<CriterionKind, string>), []);

  function updateCriterion(id: string, patch: Partial<CriterionDraft>) {
    setCriteria(current => current.map(item => item.id === id ? { ...item, ...patch } : item));
  }

  function compileCriterion(draft: CriterionDraft): Criterion {
    const primary = draft.primary.trim();
    const secondary = draft.secondary.trim();
    switch (draft.kind) {
      case "extension": {
        const values = splitValues(primary).map(value => value.replace(/^\./, ""));
        if (!values.length) throw new Error("Extension criteria need at least one extension.");
        return { kind: "extension", values };
      }
      case "mimePrefix": {
        const values = splitValues(primary);
        if (!values.length) throw new Error("MIME criteria need at least one prefix.");
        return { kind: "mimePrefix", values };
      }
      case "modifiedOlderThanDays":
        return { kind: "modifiedOlderThanDays", days: parseWholeNumber(primary, "Modified age") };
      case "sizeRange": {
        if (!primary && !secondary) throw new Error("Size range needs a minimum, maximum, or both.");
        const minBytes = primary ? mibToBytes(primary, "Minimum size") : undefined;
        const maxBytes = secondary ? mibToBytes(secondary, "Maximum size") : undefined;
        if (minBytes !== undefined && maxBytes !== undefined && minBytes > maxBytes) throw new Error("Minimum size cannot be larger than maximum size.");
        return { kind: "sizeRange", minBytes, maxBytes };
      }
      case "nameRegex":
        if (!primary) throw new Error("Filename pattern cannot be empty.");
        try { new RegExp(primary); } catch { throw new Error("Filename pattern is not a valid regular expression."); }
        return { kind: "nameRegex", pattern: primary };
    }
  }

  function compileAction(): Action {
    const value = actionValue.trim();
    if (!value) throw new Error("Choose a value for the rule action.");
    if (actionKind === "moveTo") return { kind: "moveTo", subdirectory: value };
    if (actionKind === "renamePrefix") return { kind: "renamePrefix", prefix: value };
    if (!value.includes("{name}") && !value.includes("{ext}")) throw new Error("Rename templates should include {name} or {ext} so filenames remain meaningful.");
    return { kind: "renameTemplate", template: value };
  }

  async function addRule() {
    setError("");
    try {
      if (!name.trim()) throw new Error("Rule name cannot be empty.");
      if (!criteria.length) throw new Error("Add at least one criterion.");
      const rule: Rule = {
        id: crypto.randomUUID(),
        name: name.trim(),
        enabled: true,
        matchAll,
        criteria: criteria.map(compileCriterion),
        action: compileAction(),
      };
      const customBase = state.rules.length ? state.rules : presetRules;
      if (customBase.length >= 500) throw new Error("This build supports up to 500 active rules.");
      if (!await persist({ ...state, rules: [...customBase, rule] })) return;
      setName("");
      setMatchAll(true);
      setCriteria([freshCriterion()]);
      setActionKind("moveTo");
      setActionValue("");
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    }
  }

  async function toggleRule(ruleId: string) {
    const customBase = state.rules.length ? state.rules : presetRules;
    await persist({ ...state, rules: customBase.map(rule => rule.id === ruleId ? { ...rule, enabled: !rule.enabled } : rule) });
  }

  async function removeRule(ruleId: string) {
    const customBase = state.rules.length ? state.rules : presetRules;
    await persist({ ...state, rules: customBase.filter(rule => rule.id !== ruleId) });
  }

  async function duplicateRule(rule: Rule) {
    const customBase = state.rules.length ? state.rules : presetRules;
    if (customBase.length >= 500) { setError("This build supports up to 500 active rules."); return; }
    const copy: Rule = {
      ...rule,
      id: crypto.randomUUID(),
      name: `${rule.name} copy`,
      criteria: rule.criteria.map(criterion => criterion.kind === "extension" || criterion.kind === "mimePrefix" ? { ...criterion, values: [...criterion.values] } : { ...criterion }) as Criterion[],
      action: { ...rule.action },
    };
    await persist({ ...state, rules: [...customBase, copy] });
  }

  return <section className="stack">
    <div className="panel">
      <div className="panel-head"><div><h3>Build a rule</h3><p>Combine metadata criteria and choose whether every criterion or any criterion must match.</p></div></div>
      <div className="form-grid">
        <label>Rule name<input value={name} onChange={event => setName(event.target.value)} placeholder="Large old archives"/></label>
        <label>Matching<select value={matchAll ? "all" : "any"} onChange={event => setMatchAll(event.target.value === "all")}><option value="all">Match all criteria</option><option value="any">Match any criterion</option></select></label>
        <label>Action<select value={actionKind} onChange={event => setActionKind(event.target.value as ActionKind)}>{(Object.keys(actionLabels) as ActionKind[]).map(kind => <option key={kind} value={kind}>{actionLabels[kind]}</option>)}</select></label>
      </div>

      <div className="cards" aria-label="Rule criteria">
        {criteria.map((criterion, index) => <div className="rule-card" key={criterion.id}>
          <div style={{ flex: 1 }}>
            <div className="form-grid">
              <label>Criterion {index + 1}<select value={criterion.kind} onChange={event => updateCriterion(criterion.id, { kind: event.target.value as CriterionKind, primary: "", secondary: "" })}>{(Object.keys(criterionLabels) as CriterionKind[]).map(kind => <option key={kind} value={kind}>{criterionLabels[kind]}</option>)}</select></label>
              <label>{criterion.kind === "sizeRange" ? "Minimum MiB" : criterion.kind === "modifiedOlderThanDays" ? "Days" : "Value"}<input value={criterion.primary} onChange={event => updateCriterion(criterion.id, { primary: event.target.value })} placeholder={criterionPlaceholder(criterion.kind)}/></label>
              {criterion.kind === "sizeRange" ? <label>Maximum MiB<input value={criterion.secondary} onChange={event => updateCriterion(criterion.id, { secondary: event.target.value })} placeholder="Optional maximum"/></label> : <label>Guidance<input value={criterionHelp[criterion.kind]} readOnly aria-label={`${criterionLabels[criterion.kind]} guidance`}/></label>}
            </div>
          </div>
          <button onClick={() => setCriteria(current => current.filter(item => item.id !== criterion.id))} disabled={criteria.length === 1} aria-label={`Remove criterion ${index + 1}`}>Remove</button>
        </div>)}
      </div>

      <div className="input-row">
        <button onClick={() => setCriteria(current => current.length >= 16 ? current : [...current, freshCriterion()])} disabled={criteria.length >= 16}>Add criterion</button>
        <input value={actionValue} onChange={event => setActionValue(event.target.value)} aria-label="Action value" placeholder={actionPlaceholder(actionKind)}/>
        <button className="primary" onClick={() => void addRule()}>Add rule</button>
      </div>
      {error && <p className="hint" role="alert">{error}</p>}
    </div>

    <PresetManager state={state} effectiveRules={effectiveRules} persist={persist}/>

    <div className="panel">
      <div className="panel-head"><div><h3>Active rules</h3><p>{sourceLabel}</p></div>{state.rules.length > 0 && <button onClick={() => void persist({ ...state, rules: [] })}>Reset to default preset</button>}</div>
      <div className="cards">
        {effectiveRules.map(rule => <article className="rule-card" key={rule.id}>
          <div>
            <span className="pill">{rule.enabled ? "Enabled" : "Disabled"} · {rule.matchAll ? "all" : "any"}</span>
            <h4>{rule.name}</h4>
            <p>{describeRule(rule)}</p>
          </div>
          <div className="actions">
            <button onClick={() => void toggleRule(rule.id)}>{rule.enabled ? "Disable" : "Enable"}</button>
            <button onClick={() => void duplicateRule(rule)}>Duplicate</button>
            <button onClick={() => void removeRule(rule.id)} aria-label={`Remove ${rule.name}`}>Remove</button>
          </div>
        </article>)}
      </div>
    </div>
  </section>;
}

function splitValues(value: string) {
  return [...new Set(value.split(",").map(item => item.trim()).filter(Boolean))];
}

function parseWholeNumber(value: string, label: string) {
  const number = Number(value);
  if (!Number.isInteger(number) || number < 0 || number > 365_000) throw new Error(`${label} must be a whole number in the supported range.`);
  return number;
}

function mibToBytes(value: string, label: string) {
  const number = Number(value);
  if (!Number.isFinite(number) || number < 0) throw new Error(`${label} must be a non-negative number.`);
  const bytes = Math.round(number * 1024 * 1024);
  if (!Number.isSafeInteger(bytes)) throw new Error(`${label} is too large.`);
  return bytes;
}

function criterionPlaceholder(kind: CriterionKind) {
  if (kind === "extension") return "pdf, docx, txt";
  if (kind === "mimePrefix") return "image/, application/pdf";
  if (kind === "modifiedOlderThanDays") return "30";
  if (kind === "sizeRange") return "10";
  return "^Screenshot";
}

function actionPlaceholder(kind: ActionKind) {
  if (kind === "moveTo") return "Archives/Old";
  if (kind === "renamePrefix") return "sorted-";
  return "{name}-organized.{ext}";
}

function describeRule(rule: Rule) {
  const criteria = rule.criteria.map(describeCriterion).join(rule.matchAll ? " AND " : " OR ");
  return `${criteria} → ${describeAction(rule.action)}`;
}

function describeCriterion(criterion: Criterion) {
  if (criterion.kind === "extension") return `extension ${criterion.values.join(", ")}`;
  if (criterion.kind === "mimePrefix") return `MIME ${criterion.values.join(", ")}`;
  if (criterion.kind === "modifiedOlderThanDays") return `older than ${criterion.days} day(s)`;
  if (criterion.kind === "sizeRange") {
    const min = criterion.minBytes === undefined ? "any" : `${(criterion.minBytes / 1024 / 1024).toFixed(1)} MiB`;
    const max = criterion.maxBytes === undefined ? "any" : `${(criterion.maxBytes / 1024 / 1024).toFixed(1)} MiB`;
    return `size ${min}–${max}`;
  }
  return `name /${criterion.pattern}/`;
}

function describeAction(action: Action) {
  if (action.kind === "moveTo") return `move to ${action.subdirectory}`;
  if (action.kind === "renamePrefix") return `prefix with ${action.prefix}`;
  return `rename as ${action.template}`;
}
