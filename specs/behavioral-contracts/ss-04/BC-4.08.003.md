---
document_type: behavioral-contract
level: L3
version: "v1.0"
status: active
producer: product-owner
timestamp: 2026-04-28T00:00:00
phase: 1a
inputs:
  - .factory/stories/S-5.04-post-tool-use-failure.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "80444f0"
traces_to: .factory/specs/prd.md#FR-046
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-013"
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

# BC-4.08.003: hooks-registry.toml registers PostToolUseFailure with name="tool-failure-hooks", event="PostToolUseFailure", plugin="hook-plugins/tool-failure-hooks.wasm", timeout_ms=5000; ZERO capability tables; NO once field

## Description

The dispatcher reads `plugins/vsdd-factory/hooks-registry.toml` to map event names to WASM plugin paths. The `PostToolUseFailure` event routes to `hook-plugins/tool-failure-hooks.wasm` via a single `[[hooks]]` entry. The entry requires: `name = "tool-failure-hooks"` (stable identifier per production `RegistryEntry` schema), `event = "PostToolUseFailure"` (exact event-name string match), `plugin = "hook-plugins/tool-failure-hooks.wasm"` (with `hook-plugins/` directory prefix — bare filename causes dispatcher binary-location failure), `timeout_ms = 5000` (field name per production schema — NOT `epoch_budget_ms`, NOT `timeout`). ZERO capability tables are declared: no `[hooks.capabilities.read_file]`, no `[hooks.capabilities.exec_subprocess]`. The entry carries NO `once` field — `RegistryEntry` has none, and `deny_unknown_fields` rejects it. This is Layer 2 of the dual-routing-tables pattern per ADR-011.

## Preconditions

1. `plugins/vsdd-factory/hooks-registry.toml` is the dispatcher-side routing source of truth (per ADR-011 dual-hook-routing-tables, SS-07 ownership).
2. The `hooks-registry.toml` file is syntactically valid TOML and passes dispatcher schema validation (`schema_version = 1`).
3. The `tool-failure-hooks.wasm` binary is compiled and present in the plugin directory at dispatch time.

## Postconditions

1. `hooks-registry.toml` contains exactly one `[[hooks]]` entry with `event = "PostToolUseFailure"` and `name = "tool-failure-hooks"`. The `name` field is required by `RegistryEntry` (no default); omitting it causes a deserialization error at registry-load time.
2. The entry specifies `plugin = "hook-plugins/tool-failure-hooks.wasm"` (with `hook-plugins/` directory prefix). A path without the prefix causes the dispatcher to fail to locate the binary.
3. The entry has `timeout_ms = 5000`. Field name `timeout_ms` per production schema (`RegistryEntry` in `registry.rs` declares `timeout_ms: Option<u32>` with `deny_unknown_fields`). NOT `epoch_budget_ms` (S-5.01 pass-1 field-name correction), NOT `timeout` (hooks.json.template field — different config layer).
4. The entry carries no `[hooks.capabilities.read_file]` table — the plugin reads no files (all data comes from the incoming envelope).
5. The entry carries no `[hooks.capabilities.exec_subprocess]` table — the plugin invokes no subprocesses (Option A zero-capability scoping per BC-4.08.001 Scoping Decision). The deny-by-default capability sandbox leaves the plugin with zero declared capabilities, enforced by BC-1.05.001 (exec_subprocess denied when no exec_subprocess capability declared) and BC-1.05.021 (read_file denied when no Capabilities.read_file block).
6. The entry carries no `once` field — `RegistryEntry` has no such field and `deny_unknown_fields` would reject it. PostToolUseFailure once-semantics are handled (as absence of `once: true`) at Layer 1 in `hooks.json.template` (BC-4.08.002).
7. Dispatcher successfully loads the entry without error at startup.

## Invariants

1. The `PostToolUseFailure` entry in `hooks-registry.toml` must remain registered through all v1.0 releases — removal requires a deprecation pass.
2. NO `once` field SHALL appear on the entry — `RegistryEntry` has no such field; `deny_unknown_fields` rejects it.
3. The `plugin` field MUST include the `hook-plugins/` directory prefix: `hook-plugins/tool-failure-hooks.wasm`. A bare `tool-failure-hooks.wasm` path (without prefix) causes dispatcher binary-location failure.
4. NO capability tables SHALL be declared while the plugin reads no files and invokes no subprocesses. Adding capability tables unnecessarily expands the sandbox surface — removal requires a BC update.
5. The entry MUST use `timeout_ms = 5000` (not `epoch_budget_ms`, not `timeout` — per production schema lesson from S-5.01 pass-1 field name correction).
6. Exactly one `PostToolUseFailure` entry must exist. Duplicate entries are a misconfiguration; VP-068 asserts `post_tool_use_failure_count == 1` and fails if duplicates are present.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `timeout_ms` missing from the entry | Registry load succeeds — `RegistryEntry.timeout_ms` is `Option<u32>` with `#[serde(default)]`; missing values default to `RegistryDefaults.timeout_ms = 5000`. However, VP-068 test assertion expects explicit `timeout_ms` field and fails if absent. Explicit declaration is therefore required. |
| EC-002 | `plugin` path missing `hook-plugins/` prefix (e.g., `tool-failure-hooks.wasm` without directory) | Dispatcher cannot find the binary at the resolved path; plugin load fails at dispatch time; `PostToolUseFailure` event is not processed; `tool.error` event never emitted. |
| EC-003 | Duplicate `PostToolUseFailure` entries in `hooks-registry.toml` | Both entries loaded into `Vec<RegistryEntry>`; routing follows first-entry-wins under Vec iteration order. VP-068 asserts `post_tool_use_failure_count == 1`; test fails if duplicates present. |
| EC-004 | A `[hooks.capabilities.exec_subprocess]` table is accidentally added to the entry | `RegistryEntry` deserializes with the capability declared; plugin gains exec_subprocess access it does not use. Misconfiguration, not a crash. VP-068 asserts NO capability tables present; test fails at test time. |
| EC-005 | `once` field added to the entry in `hooks-registry.toml` | `RegistryEntry` has `deny_unknown_fields`; registry load fails with deserialization error at startup. Dispatcher reports load failure; all routing halts until the TOML is corrected. |
| EC-006 | Field name `epoch_budget_ms` used instead of `timeout_ms` | `RegistryEntry` has `deny_unknown_fields`; registry load fails with deserialization error. Use `timeout_ms` — per production `RegistryEntry` schema in `registry.rs`. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Parse `hooks-registry.toml`; find `PostToolUseFailure` entry | Entry present: `event = "PostToolUseFailure"`, `name = "tool-failure-hooks"`, `plugin = "hook-plugins/tool-failure-hooks.wasm"`, `timeout_ms = 5000`; NO `capabilities` sub-table | happy-path |
| `hooks-registry.toml` entry with `plugin` missing `hook-plugins/` prefix | Dispatcher fails to locate binary; PostToolUseFailure events produce no emissions | error (bad plugin path) |
| `hooks-registry.toml` entry with a `[hooks.capabilities.exec_subprocess]` table present | Entry deserializes; plugin gains exec_subprocess capability it should not have; VP-068 NO-capability assertion fails | error (misconfiguration) |
| `hooks-registry.toml` entry with `once = true` added | `deny_unknown_fields` causes deserialization error at registry-load time; dispatcher startup fails | error (unknown field) |
| `hooks-registry.toml` entry with `epoch_budget_ms = 5000` instead of `timeout_ms` | `deny_unknown_fields` causes deserialization error at registry-load time | error (wrong field name) |
| Two `PostToolUseFailure` entries in `hooks-registry.toml` | VP-068 `post_tool_use_failure_count == 1` assertion fails; duplicate detection catches it | error (duplicate entry) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-068 | Tool-Failure Hook Plugin Surface Invariant — All BC-4.08.* Postconditions Hold in Integration Test | integration |

## Related BCs

- **BC-4.08.002** — counterpart (`hooks.json.template` is the Claude Code-side registration; this is the dispatcher-side registration; both must exist for full routing of PostToolUseFailure)
- **BC-4.08.001** — enables (this routing entry causes the dispatcher to invoke the plugin that emits `tool.error`)
- **BC-4.07.004** — structural analog (hooks-registry.toml WorktreeCreate/WorktreeRemove entries with same zero-capability profile; BC-4.08.003 differs in having one entry for one event instead of two entries for two events)
- **BC-4.05.005** — structural analog (SessionEnd Layer 2 registration with zero capability tables; same zero-capability profile)
- **BC-1.05.001** — enforces (exec_subprocess denied when no exec_subprocess capability declared — deny-by-default sandbox for zero-capability profile)
- **BC-1.05.021** — enforces (read_file denied when no Capabilities.read_file block — deny-by-default sandbox for zero-capability profile)

## Architecture Anchors

- SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (PostToolUseFailure routing entry; routing semantics for the tool-failure-hooks WASM plugin are SS-04-owned per F-8 ruling applied to PostToolUseFailure)
- SS-07 — Hook Bash Layer owns `hooks-registry.toml` as the committed, operator-editable dispatcher routing table (per ARCH-INDEX.md and ADR-011).

**Architectural Notes:**

**F-8 Ruling (subsystem ownership, applied to PostToolUseFailure):** This BC retains `subsystem: SS-04` in its frontmatter. Same rationale as BC-4.07.004 and BC-4.05.005: BC-4.08.003 contracts the routing semantics of the PostToolUseFailure WASM plugin entry — which plugin is invoked, with which (zero) capabilities. SS-04 owns these routing semantics even though `hooks-registry.toml` lives in SS-07 space.

**No capability tables (same as SessionEnd and Worktree):** This BC requires ZERO capability tables. The tool-failure plugin reads all data from the envelope and has no external dependencies. Same deny-by-default sandbox profile as BC-4.05.005 and BC-4.07.004.

**F-17 Generator Workflow (inherited):** `hooks-registry.toml` is human-edited source of truth as of v1.0.0. The PostToolUseFailure entry is added by direct edit. The legacy generator script is retired and must not be used.

**Single entry for single event:** Unlike BC-4.07.004 (two entries for two complementary events), PostToolUseFailure requires exactly one `[[hooks]]` entry pointing to `hook-plugins/tool-failure-hooks.wasm`.

## Story Anchor

S-5.04

## VP Anchors

VP-068

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-013 |
| Capability Anchor Justification | CAP-013 ("Capture post-execution activity (PostToolUse hooks)") per capabilities.md §CAP-013. The hooks-registry.toml entry is the dispatcher-side routing that connects PostToolUseFailure events to the WASM plugin that captures "tool errors for audit and observability purposes". |
| L2 Domain Invariants | DI-004 (capability denial — by declaring NO capabilities, deny-by-default ensures exec_subprocess and read_file are both denied; plugin never attempts to call them); DI-015 (per-project activation required — this entry is directly added by human edit per the file header; must be present before dispatcher can route PostToolUseFailure) |
| Architecture Module | SS-04 — `plugins/vsdd-factory/hooks-registry.toml` (PostToolUseFailure entry added by direct edit) |
| Stories | S-5.04 |
| Functional Requirement | FR-046 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-04-28 | product-owner | Initial creation (S-5.04 foundation burst). Promoted from v1.1 BC candidate "BC-4.08.003-post-tool-use-failure-dispatcher-wiring" in legacy story. Production schema field names verified: `name` (NOT plugin_name), `event` (NOT event_name), `plugin` with `hook-plugins/` PREFIX, `timeout_ms` (NOT epoch_budget_ms). ZERO capability tables (Option A). NO `once` field (`deny_unknown_fields` rejects it). Single entry for single event. |
