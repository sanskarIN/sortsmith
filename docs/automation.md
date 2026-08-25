# Watched-folder automation

SortSmith's current watched-folder automation is deliberately in-app. It runs only while the desktop application is open. Native operating-system background scheduling remains a separate 0.3 design task because each platform needs explicit startup, permission, disable, and uninstall behavior.

## Current execution model

The frontend asks the Tauri host to evaluate due watches once per minute. The backend remains authoritative for due-time checks, filesystem validation, rule evaluation, execution, journaling, and persisted `lastRunAt`/recent-journal state.

A watch is eligible only when:

- it is enabled;
- its interval is at least five minutes;
- its configured interval has elapsed since the last successful/no-change run;
- its folder still resolves to an existing directory;
- its referenced preset still resolves to usable rules.

Normal scans do not follow links. Every mutation still goes through the same preview and reversible execution paths used by interactive organization.

## State reconciliation

A successful due-watch check can update two persisted fields that the React UI also displays:

- `watchedFolders[*].lastRunAt`;
- `recentJournalIds`.

After the backend reports one or more watch results, the frontend reloads persisted state and merges only those automation-owned fields into the current in-memory state. It deliberately preserves the currently active settings, rules, and preset objects so a timer refresh cannot replace unrelated interactive state.

The one-minute timer also prevents overlapping watch checks inside the same frontend instance. A new tick is ignored while the previous tick is still in flight.

## No-change runs

A due watch whose preview contains no planned operations does **not** call the execution path and therefore does not create an empty undo journal.

The backend records `lastRunAt` for the completed check and reports either:

- that the folder was already organized; or
- that no changes were needed but some entries could not be inspected.

This keeps automation history useful instead of filling it with zero-entry journals.

## Idle behavior

If no watched folder is due, the backend returns without rewriting `state.json`. This avoids an unnecessary state-file write every minute while SortSmith is open.

Unavailable folders, missing presets, and safe scan/execution failures produce generic status messages and do not claim a successful run.

## Reversibility

When a due watch has planned operations, SortSmith creates the normal operation journal before mutation. Successful journal IDs are added to recent history and the frontend refreshes that history state after the watch tick.

No automated path deletes duplicate candidates or bypasses collision/path-containment checks.

## Native background scheduling is still deferred

The current implementation is not a login/startup service and does not pretend to run while SortSmith is closed. A future native scheduler must define and test, per platform:

- explicit opt-in and opt-out;
- startup/launch behavior;
- permission prompts and revocation;
- missed-run semantics;
- concurrency with an already-open SortSmith process;
- state/journal locking;
- update behavior;
- uninstall cleanup;
- error visibility;
- accessibility of scheduler controls.

That work must not reuse the current in-app timer as evidence that native background execution is already safe or complete.
