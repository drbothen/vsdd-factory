---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.03-worktree-hooks.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "0b97a0a"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.07.004: hooks-registry.toml registers WorktreeCreate and WorktreeRemove routing to hook-plugins/worktree-hooks.wasm; single crate, two entries; ZERO capability tables; timeout_ms:5000

## Description

The dispatcher reads `plugins/vsdd-factory/hooks-registry.toml` to map event names to WASM plugin paths. Both `WorktreeCreate` and `WorktreeRemove` events route to the **same WASM binary**: `hook-plugins/worktree-hooks.wasm`. This requires two separate `[[hooks]]` entries in `hooks-registry.toml` — one with `event = "WorktreeCreate"` and one with `event = "WorktreeRemove"` — both pointing to the same plugin path. The single-crate / two-entry design reduces binary count while maintaining clean dispatcher-side routing separation. Critically, ZERO capability tables are declared for either entry: neither `[hooks.capabilities.read_file]` nor `[hooks.capabilities.exec_subprocess]` is needed (Option A zero-capability scoping, per BC-4.07.001 Scoping Decision). This is Layer 2 of the dual-routing-tables pattern per ADR-011. The `hooks-registry.toml` entries do NOT carry a `once` field — `RegistryEntry` has none, and `deny_unknown_fields` rejects it. Once-per-session discipline is NOT applicable to worktree events; they are stateless per BC-4.07.001 Invariant 7 and BC-4.07.002 Invariant 7.

## Single-Crate / Two-Entry Design Rationale

**Decision:** One `worktree-hooks` crate handling both events, registered as two separate `[[hooks]]` entries in `hooks-registry.toml`.

**Rationale for single crate:** S-5.02 establishes the pattern of a dedicated crate per plugin. However, S-5.02 was one event / one plugin. For two complementary events (Create + Remove), a single crate with internal dispatch on `event_type` is simpler (one .wasm binary, shared types, smaller total binary footprint) without meaningful coupling risk — the two dispatch paths are independent code branches within `on_hook`.

**Rationale for two registry entries:** The dispatcher matches on `event` field exactly. A single `[[hooks]]` entry cannot match two different event names. Two separate entries (one for `WorktreeCreate`, one for `WorktreeRemove`) both pointing to `hook-plugins/worktree-hooks.wasm` is the correct structural choice. The dispatcher loads the plugin once and invokes it once per matching event — there is no double-loading.

**Rejected alternative:** Two separate crates (`worktree-create-telemetry.wasm` + `worktree-remove-telemetry.wasm`). Doubles binary count, adds Cargo workspace complexity, duplicates shared types. Not justified for two closely related events that share the same capability profile (zero capabilities).

## Preconditions

1. `plugins/vsdd-factory/hooks-registry.toml` is the dispatcher-side routing source of truth (per ADR-011 dual-hook-routing-tables, SS-07 ownership).
2. The `hooks-registry.toml` file is syntactically valid TOML and passes dispatcher schema validation (`schema_version = 1`).
3. The `worktree-hooks.wasm` binary is compiled and present in the plugin directory at dispatch time.

## Postconditions

1. `hooks-registry.toml` contains a `[[hooks]]` entry with `event = "WorktreeCreate"` and `name = "worktree-hooks"`. The `name` field is required by `RegistryEntry` (no default); omitting it causes a deserialization error at registry-load time.
2. `hooks-registry.toml` contains a `[[hooks]]` entry with `event = "WorktreeRemove"` and `name = "worktree-hooks"`. Same `name` value as the WorktreeCreate entry — both route to the same plugin.
3. Both entries specify `plugin = "hook-plugins/worktree-hooks.wasm"` (with `hook-plugins/` directory prefix). A path without the prefix causes the dispatcher to fail to locate the binary.
4. Both entries have `timeout_ms = 5000`. Justification: the worktree plugin has no subprocess wait and no file reads; 5000ms is the `RegistryDefaults` value and is adequate for the stateless emit-only path. Explicit declaration is required by this BC to maintain visible intent. Field name `timeout_ms` per production schema (`RegistryEntry` in `registry.rs` declares `timeout_ms: Option<u32>` with `deny_unknown_fields`).
5. Neither `WorktreeCreate` entry nor `WorktreeRemove` entry carries `[hooks.capabilities.read_file]` — the plugin reads no files (all data comes from the incoming envelope).
6. Neither entry carries `[hooks.capabilities.exec_subprocess]` — the plugin invokes no subprocesses (Option A zero-capability scoping per BC-4.07.001). The deny-by-default capability sandbox (BC-1.05.022) leaves the plugin with zero declared capabilities for both events.
7. Neither entry carries a `once` field — `RegistryEntry` has no such field and `deny_unknown_fields` would reject it. Worktree event once-semantics are handled (as absence of `once: true`) at Layer 1 in `hooks.json.template` (BC-4.07.003).
8. Dispatcher successfully loads both entries without error at startup.

## Invariants

1. Both `WorktreeCreate` and `WorktreeRemove` entries in `hooks-registry.toml` must remain registered through all v1.0 releases — removal requires a deprecation pass.
2. NO `once` field SHALL appear on either entry — `RegistryEntry` has no such field; `deny_unknown_fields` rejects it.
3. The `plugin` field MUST include the `hook-plugins/` directory prefix: `hook-plugins/worktree-hooks.wasm`. A bare `worktree-hooks.wasm` path (without prefix) causes dispatcher binary-location failure.
4. NO capability tables SHALL be declared for either entry while the plugin reads no files and invokes no subprocesses. Adding capability tables unnecessarily expands the sandbox surface — removal requires a BC update.
5. `name` must be a unique, stable identifier per entry. Duplicate `name` values within the same event are not validated at load time; operator must avoid them manually.
6. Both entries MUST use `timeout_ms = 5000` (not `epoch_budget_ms`, not `timeout` — per production schema lesson from S-5.01 pass-1 field name correction).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `timeout_ms` missing from either entry | Registry load succeeds — `RegistryEntry.timeout_ms` is `Option<u32>` with `#[serde(default)]`; missing values default to `RegistryDefaults.timeout_ms = 5000`. However, VP-067 test assertion expects explicit `timeout_ms` field and fails if absent. Explicit declaration is therefore required. |
| EC-002 | `plugin` path missing `hook-plugins/` prefix (e.g., `worktree-hooks.wasm` without directory) | Dispatcher cannot find the binary at the resolved path; plugin load fails at dispatch time; corresponding worktree event is not processed; `worktree.*` event never emitted. |
| EC-003 | Duplicate `WorktreeCreate` entries in `hooks-registry.toml` | Both entries loaded into `Vec<RegistryEntry>`; routing follows first-entry-wins under Vec iteration order (toml-rs table-array append semantics). VP-067 asserts `worktree_create_count == 1`; test fails if duplicates present. |
| EC-004 | Duplicate `WorktreeRemove` entries in `hooks-registry.toml` | Same as EC-003. VP-067 asserts `worktree_remove_count == 1`; test fails if duplicates present. |
| EC-005 | A `[hooks.capabilities.exec_subprocess]` table is accidentally added to either entry | `RegistryEntry` deserializes with the capability declared; plugin gains exec_subprocess access it does not use. Misconfiguration, not a crash. VP-067 asserts NO capability tables present for both entries; test fails at test time. |
| EC-006 | `once` field added to either entry in `hooks-registry.toml` | `RegistryEntry` has `deny_unknown_fields`; registry load fails with deserialization error at startup. Dispatcher reports load failure; all routing halts until the TOML is corrected. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks-registry.toml`; find `WorktreeCreate` entry | Entry present: `event = "WorktreeCreate"`, `name = "worktree-hooks"`, `plugin = "hook-plugins/worktree-hooks.wasm"`, `timeout_ms = 5000`; NO `capabilities` sub-table | happy-path (WorktreeCreate entry) |
| Parse `hooks-registry.toml`; find `WorktreeRemove` entry | Entry present: `event = "WorktreeRemove"`, `name = "worktree-hooks"`, `plugin = "hook-plugins/worktree-hooks.wasm"`, `timeout_ms = 5000`; NO `capabilities` sub-table | happy-path (WorktreeRemove entry) |
| `hooks-registry.toml` entry with `plugin` missing `hook-plugins/` prefix | Dispatcher fails to locate binary; worktree events produce no emissions | error (bad plugin path) |
| `hooks-registry.toml` entry with a `[hooks.capabilities.exec_subprocess]` table present | Entry deserializes; plugin gains exec_subprocess capability it should not have; VP-067 NO-capability assertion fails | error (misconfiguration) |
| `hooks-registry.toml` entry with `once = true` added | `deny_unknown_fields` causes deserialization error at registry-load time; dispatcher startup fails | error (unknown field) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-067 | Worktree Hook Plugin Surface Invariant — All BC-4.07.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.07.003** — counterpart (`hooks.json.template` is the Claude Code-side registration; this is the dispatcher-side registration; both must exist for full routing of both worktree events)
- **BC-4.07.001** — enables (WorktreeCreate routing entry causes the dispatcher to invoke the plugin that emits `worktree.created`)
- **BC-4.07.002** — enables (WorktreeRemove routing entry causes the dispatcher to invoke the plugin that emits `worktree.removed`)
- **BC-4.05.005** — structural analog (SessionEnd Layer 2 registration with zero capability tables; BC-4.07.004 has same zero-capability profile but covers two events instead of one)
- **BC-4.04.005** — structural analog (SessionStart Layer 2 registration; BC-4.07.004 differs in having no capability tables and covering two events)

## Architecture Anchors

- SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (WorktreeCreate + WorktreeRemove routing entries; routing semantics for the worktree-hooks WASM plugin are SS-04-owned per F-8 ruling applied to worktree events)
- SS-07 — Hook Bash Layer owns `hooks-registry.toml` as the committed, operator-editable dispatcher routing table (per ARCH-INDEX.md and ADR-011).

**Architectural Notes:**

**F-8 Ruling (subsystem ownership, applied to worktree events):** This BC retains `subsystem: SS-04` in its frontmatter. Same rationale as BC-4.05.005: BC-4.07.004 contracts the routing semantics of the WorktreeCreate/WorktreeRemove WASM plugin entries — which plugin is invoked, with which (zero) capabilities, and under which deduplication policy. SS-04 owns these routing semantics even though `hooks-registry.toml` lives in SS-07 space.

**No capability tables (worktree-specific — same as SessionEnd):** Like BC-4.05.005, this BC requires ZERO capability tables. The worktree plugin reads all data from the envelope and has no external dependencies. This is the cleanest possible sandbox profile: deny-by-default with zero declared capabilities.

**F-17 Generator Workflow (inherited):** `hooks-registry.toml` is human-edited source of truth as of v1.0.0. Both worktree entries are added by direct edit. The legacy generator script is retired and must not be used. (See BC-4.04.005 §"F-17 Generator Workflow" for the full ruling and historical context.)

**Two entries, one plugin:** Having two `[[hooks]]` entries that both reference `hook-plugins/worktree-hooks.wasm` is intentional and correct. The dispatcher loads the plugin binary once per dispatch call — there is no double-loading. The internal `on_hook` function dispatches to the appropriate code path based on the `event_type` field in the incoming envelope.

## Story Anchor

S-5.03

## VP Anchors

VP-067

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| L2 Domain Invariants | DI-004 (capability denial — by declaring NO capabilities, deny-by-default ensures exec_subprocess and read_file are both denied for this plugin; no audit event emitted because the plugin never attempts to call them); DI-007 (always-on telemetry — both `worktree.created` and `worktree.removed` are emitted unconditionally via these routing entries); DI-015 (per-project activation required — these entries are directly added by human edit per the file header) |
| Architecture Module | SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (WorktreeCreate + WorktreeRemove entries added by direct edit) |
| Stories | S-5.03 |
| Functional Requirement | FR-046 |
