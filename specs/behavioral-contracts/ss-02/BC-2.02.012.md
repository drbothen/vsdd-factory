---
document_type: behavioral-contract
level: L3
version: "1.1"
status: ready
producer: product-owner
timestamp: 2026-05-01T00:00:00Z
phase: 1
inputs:
  - crates/hook-sdk/src/payload.rs
  - plugins/vsdd-factory/hooks/handoff-validator.sh
  - plugins/vsdd-factory/hooks/pr-manager-completion-guard.sh
  - plugins/vsdd-factory/hooks/validate-pr-review-posted.sh
  - plugins/vsdd-factory/hooks/track-agent-stop.sh
input-hash: "[pending-recompute]"
traces_to: .factory/specs/domain-spec/capabilities.md
origin: brownfield
extracted_from: "crates/hook-sdk/src/payload.rs"
subsystem: "SS-02"
capability: "CAP-022"
lifecycle_status: active
introduced: v1.1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-2.02.012: HookPayload SubagentStop fields: top-level envelope schema for agent_type, subagent_name, last_assistant_message, result

## Description

SubagentStop events from the Claude Code dispatcher carry agent identity and assistant-message content as **top-level** fields of the JSON envelope — NOT nested inside `tool_input` or `tool_response`. `HookPayload` models these four fields as `Option<String>` with `#[serde(default)]`, providing backward-compatible deserialization (missing fields deserialize to `None`) AND jq-`//`-equivalent null-as-advance semantics (serde_json deserializes JSON `null` to `None` for `Option<T>`). This additive ABI extension maintains HOST_ABI_VERSION = 1 per D-6 Option A and D-183 architectural decision.

## Preconditions

1. The dispatcher deserializes a Claude Code SubagentStop JSON envelope into a `HookPayload` struct.
2. The envelope may carry any combination of the four SubagentStop fields: `agent_type`, `subagent_name`, `last_assistant_message`, `result`. Each may be present (non-null string), present as JSON null, or absent entirely.
3. The `HookPayload` struct declares all four fields as `#[serde(default)] Option<String>`.
4. For non-SubagentStop event_names (PreToolUse, PostToolUse, SessionStart, etc.), these fields are absent from the envelope.

## Postconditions

1. **agent_type field:** `payload.agent_type` is `Some(s)` if the envelope carries a non-null `agent_type` string; `None` if the field is absent or JSON null.
2. **subagent_name field:** `payload.subagent_name` is `Some(s)` if the envelope carries a non-null `subagent_name` string; `None` if absent or JSON null.
3. **last_assistant_message field:** `payload.last_assistant_message` is `Some(s)` if the envelope carries a non-null `last_assistant_message` string; `None` if absent or JSON null.
4. **result field:** `payload.result` is `Some(s)` if the envelope carries a non-null `result` string; `None` if absent or JSON null.
5. **Canonical agent identity fallback chain:** The authoritative Rust expression for resolving agent identity is: `payload.agent_type.as_deref().or(payload.subagent_name.as_deref()).unwrap_or("unknown")`. This mirrors the bash pattern `jq -r '.agent_type // .subagent_name // "unknown"'` used in all four bash hooks.
6. **Canonical assistant-message fallback chain:** The authoritative Rust expression for resolving assistant message content is: `payload.last_assistant_message.as_deref().or(payload.result.as_deref()).unwrap_or("")`. This mirrors the bash pattern `jq -r '.last_assistant_message // .result // empty'` used in `pr-manager-completion-guard.sh`, `validate-pr-review-posted.sh`, and `track-agent-stop.sh`.
7. **Non-SubagentStop events:** For all non-SubagentStop event_names, all four fields default to `None` via `#[serde(default)]`. Plugins SHOULD NOT rely on these fields being populated for non-SubagentStop events.

## Invariants

1. **HOST_ABI_VERSION = 1 unchanged:** These four fields are an additive `HookPayload` extension per D-6 Option A and D-183. No bump to HOST_ABI_VERSION is permitted. Both `crates/hook-sdk/src/lib.rs:58` and `crates/factory-dispatcher/src/lib.rs:43` remain `pub const HOST_ABI_VERSION: u32 = 1;`. See also BC-2.01.003 and BC-2.02.011 Invariant 1.
2. **Backward-compatible deserialization:** All four fields carry `#[serde(default)]`. Envelopes that predate the typed-projection (e.g., PreToolUse payloads) deserialize successfully with all four fields as `None`; no deserialization error occurs.
3. **JSON null → None semantics:** serde_json's documented behavior for `Option<T>` with `#[serde(default)]` maps JSON `null` to `None`. This provides jq-`//`-equivalent null-as-advance fallback semantics without additional transformation logic.
4. **Field names are canonical and immutable:** The field names `agent_type`, `subagent_name`, `last_assistant_message`, `result` are the empirically verified Claude Code SubagentStop envelope field names, confirmed against all four bash hooks at the source paths listed in Evidence below. These names MUST NOT be renamed or aliased without updating all four hook scripts and this BC.
5. **Canonical fallback chains are normative:** Story specs implementing typed-projection (S-8.01, S-8.02, S-8.03, S-8.05, S-8.30) MUST use the canonical fallback chain expressions from Postconditions 5-6. Deviations require an explicit divergence rationale in the story spec.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Field present in envelope with JSON null value (e.g., `"agent_type": null`) | `payload.agent_type = None`; fallback chain advances to `subagent_name` |
| EC-002 | Field absent entirely from envelope | `payload.agent_type = None` (via `#[serde(default)]`); fallback chain advances |
| EC-003 | All four SubagentStop fields absent or null | Agent identity resolves to `"unknown"` (final `unwrap_or`); assistant-message resolves to `""` |
| EC-004 | handoff-validator 3-stage assistant-message chain | `handoff-validator.sh:28` uses `.last_assistant_message \| .result \| .output` as a 3-stage chain (adds `output` as third fallback). The canonical 2-stage chain in Postcondition 6 does NOT include `output`. Story-writer implementing S-8.01 MUST cite this divergence explicitly when adding the third arm: `payload.last_assistant_message.as_deref().or(payload.result.as_deref()).or(payload.output.as_deref()).unwrap_or("")`. Adding `output: Option<String>` to `HookPayload` may be required; if added, it follows the same `#[serde(default)]` pattern and does not bump HOST_ABI_VERSION. |
| EC-005 | Non-SubagentStop event (e.g., PreToolUse) with no SubagentStop fields | All four fields = `None`; plugins that guard on `event_name == "SubagentStop"` before accessing these fields are safe |
| EC-006 | SubagentStop envelope with unexpected additional top-level fields | `HookPayload` does not use `#[serde(deny_unknown_fields)]`; additional fields are silently ignored by serde_json |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"product-owner","last_assistant_message":"Done."}` | `agent_type=Some("product-owner")`, `last_assistant_message=Some("Done.")`, `subagent_name=None`, `result=None` | happy-path |
| `{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","subagent_name":"story-writer","result":"Complete."}` (no agent_type, no last_assistant_message) | `agent_type=None`, `subagent_name=Some("story-writer")`, `last_assistant_message=None`, `result=Some("Complete.")`; identity chain resolves to `"story-writer"`, message chain resolves to `"Complete."` | happy-path (fallback) |
| `{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":null,"subagent_name":null}` | `agent_type=None`, `subagent_name=None`; identity chain resolves to `"unknown"` | edge-case (JSON null) |
| `{"event_name":"PreToolUse","tool_name":"Bash","session_id":"s","dispatcher_trace_id":"t","tool_input":{}}` | All four SubagentStop fields = `None` | edge-case (non-SubagentStop) |
| `{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t"}` (no SubagentStop fields at all) | All four = `None`; identity = `"unknown"`, message = `""` | edge-case (all absent) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | `#[serde(default)]` fields deserialize correctly from absent-field envelopes | Unit test (serde_json roundtrip) |
| (TBD) | JSON null deserializes to `None` for all four fields | Unit test (null fixture) |
| (TBD) | Canonical agent identity fallback chain produces `"unknown"` when all fields absent | Unit test (property-based or table-driven) |
| (TBD) | Canonical assistant-message fallback chain produces `""` when all fields absent | Unit test (property-based or table-driven) |

## Related BCs

- **BC-2.01.001** (composes with) — HookPayload base struct; this BC extends it with four optional SubagentStop fields
- **BC-2.01.003** (depends on) — HOST_ABI_VERSION = 1 invariant; additive field extension must not break it
- **BC-2.02.011** (parallel) — parallel additive ABI extension in the same D-183 burst; both BCs cite HOST_ABI_VERSION = 1 per D-6 Option A

## Architecture Anchors

- `crates/hook-sdk/src/payload.rs` (HookPayload struct — four new `#[serde(default)] pub agent_type: Option<String>` etc. fields added after existing fields)
- `plugins/vsdd-factory/hooks/handoff-validator.sh:25-28` (empirical source for 3-stage message chain including `.output` fallback — EC-004)
- `plugins/vsdd-factory/hooks/pr-manager-completion-guard.sh:25-26` (empirical source for agent identity + 2-stage message chain)
- `plugins/vsdd-factory/hooks/validate-pr-review-posted.sh:21-22` (empirical source for agent identity + 2-stage message chain)
- `plugins/vsdd-factory/hooks/track-agent-stop.sh:22-23` (empirical source for agent identity + 2-stage message chain)

## Story Anchor

S-8.30 — SDK extension story for HookPayload SubagentStop typed projection (authored 2026-05-01, D-183 Phase B). Also anchors: S-8.01 (handoff-validator typed-projection, REOPENED for re-convergence per D-183), S-8.02 (pr-manager-completion-guard typed-projection), S-8.03 (track-agent-stop typed-projection, REOPENED per D-183), S-8.05 (validate-pr-review-posted typed-projection).

## VP Anchors

(TBD — to be assigned in Phase 1.6b verification properties pass)

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-022 |
| Capability Anchor Justification | CAP-022 ("Port hook plugins from bash to native WASM") per capabilities.md §CAP-022. `HookPayload` SubagentStop field projection is a prerequisite for porting the four bash SubagentStop hooks (handoff-validator, pr-manager-completion-guard, track-agent-stop, validate-pr-review-posted) to native WASM: without typed `HookPayload` fields, ported plugins must re-parse raw stdin JSON, defeating the purpose of the typed-projection model. |
| L2 Domain Invariants | TBD (Phase 1.5 invariant lift pass) |
| Architecture Module | SS-02 — `crates/hook-sdk/src/payload.rs` |
| Stories | S-8.30 (implementing story — authored 2026-05-01, D-183 Phase B), S-8.01 (handoff-validator — REOPENED), S-8.02 (pr-manager-completion-guard), S-8.03 (track-agent-stop — REOPENED), S-8.05 (validate-pr-review-posted) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/payload.rs:15-53` (current HookPayload struct); `plugins/vsdd-factory/hooks/handoff-validator.sh:25-28` (3-stage chain: `.agent_type \| .subagent_name \| "unknown"` and `.last_assistant_message \| .result \| .output`); `plugins/vsdd-factory/hooks/pr-manager-completion-guard.sh:25-26` (2-stage chains); `plugins/vsdd-factory/hooks/validate-pr-review-posted.sh:21-22` (2-stage chains); `plugins/vsdd-factory/hooks/track-agent-stop.sh:22-23` (2-stage chains) |
| **Confidence** | HIGH — all four bash hooks confirmed present and read verbatim; field names verified empirically; D-183 decision seals the architectural path |
| **Extraction Date** | 2026-05-01 |
| **Extracted from** | `crates/hook-sdk/src/payload.rs` + 4 bash hook scripts (D-183 burst) |

#### Evidence Types Used

- type constraint (current HookPayload struct in payload.rs)
- runtime observation (bash hook jq expressions — empirical Claude Code envelope schema)
- architectural decision (D-183: Path 1 sealed; replaces ad-hoc per-hook stdin re-parsing)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | No (struct fields; deserialization is pure transformation of already-read bytes) |
| **Global state access** | No |
| **Deterministic** | Yes (serde_json deserialization is deterministic given same input) |
| **Thread safety** | Yes (immutable after construction; `HookPayload` derives Clone but not Sync/Send — TBD Phase 1.6b) |
| **Overall classification** | pure-core (field access and fallback chain evaluation) |

#### Refactoring Notes

The canonical fallback chain expressions (Postconditions 5-6) are short enough to inline at call sites in plugin code. If repeated across many story implementations, a `HookPayload::agent_identity()` and `HookPayload::assistant_message()` helper method pair may be worth extracting — this is left to the implementing story author's discretion and is not required by this BC.

## Change Log

### v1.1 (2026-05-01) — D-183 Phase B canonical story reference update

Replaced provisional "S-8.11" reference with canonical "S-8.30" in Story Anchor section, Invariant 5, and Traceability Stories row. POLICY 1: append-after-Tier-3 chosen over placeholder renumber (Tier 2/3 placeholder IDs S-8.11..S-8.29 remain untouched). S-8.30 was authored as the implementing story for this BC in D-183 Phase B (2026-05-01).
