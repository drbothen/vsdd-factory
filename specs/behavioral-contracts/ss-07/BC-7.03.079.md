---
document_type: behavioral-contract
level: L3
version: "v1.4"
status: draft
producer: "PHASE_1_4_B_BCS_AGENT_9"
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [pass-3-deep-hooks.md, pass-3-behavioral-contracts.md, pass-3-behavioral-contracts-deep-r1.md, bc-id-mapping.md]
input-hash: "118ab49"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "pass-3-deep-hooks.md:884"
subsystem: "SS-07"
capability: "TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified:
  - "2026-07-13 (v1.4)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-7.03.079: track-agent-start: identity & registry binding

## Description

track-agent-start: identity & registry binding. Telemetry; never blocks; on_error=continue.

**Source category:** Routing hooks (PreToolUse, PostToolUse, lifecycle).
**Audit ID:** `BC-AUDIT-1078` (extracted from `pass-3-deep-hooks.md` line 884).
**Hook script:** ``plugins/vsdd-factory/hooks/track-agent-start.sh``.
**Registry entry:** `hooks-registry.toml::track-agent-start` (PreToolUse, tool=`^Agent$` (anchored; implemented by S-19.04, W2 — pending merge), priority=110, timeout_ms=5000, on_error=continue)..

## Preconditions

1. Hook event/tool match: PreToolUse `^Agent$` (anchored form; implemented by S-19.04, W2 — pending merge)..
2. Trigger: Every Agent dispatch.

## Postconditions

1. Behavior: Telemetry; never blocks; on_error=continue.
2. Exit codes: Always 0.
3. Error policy: continue.

## Invariants

1. The hook's identity and dispatch characteristics — name, event, plugin, priority, on_error, timeout_ms — remain stable across the contract lifetime. This constraint does NOT freeze the `tool` pattern field against corrections where the prior value was an incorrect narrowing or incidental substring over-match of the guard's semantic intent. S-19.04's re-anchoring of `tool = "Agent"` to the anchored regex `tool = "^Agent$"` (per the S-19.04 D-a table) is a semantic-intent correction: anchoring prevents false-fires on hypothetical future tool names with `Agent` as a substring (e.g., `AgentAsync`) while preserving full routing to all PreToolUse Agent events. This is NOT a binding-tuple migration change. Post-S-19.04 binding tuple (implemented by S-19.04, W2 — pending merge): event=PreToolUse, tool=`^Agent$`, priority=110, on_error=continue.
2. Exit-code semantics conform to the dispatcher contract: 0 = allow / advisory, 2 = block, 1 = jq-missing-fail-closed (where applicable).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs. Used for regression testing and agent validation.

| Input | Expected Output | Category |
|-------|-----------------|----------|
| TBD | TBD | happy-path |
| TBD edge-case | TBD | edge-case |
| TBD error-case | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|--------------|
| VP-TBD | TBD — to be assigned during VP synthesis | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-07 (Hook Bash Layer) |
| Stories | S-8.08 |

## Related BCs (Recommended)

- TBD — to be cross-linked during BC graph synthesis.

## Architecture Anchors (Recommended)

- `architecture/ss-07-hook-bash.md` — anchor TBD.

## Story Anchor (Recommended)

TBD — story will be assigned during story-writer phase.

## VP Anchors (Recommended)

- TBD — VP linkage to be added during VP synthesis.

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | ``plugins/vsdd-factory/hooks/track-agent-start.sh`` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Document** | `pass-3-deep-hooks.md` line 884 |
| **Audit ID** | `BC-AUDIT-1078` |
| **Source Line(s) (within hook)** | Header 1-13. |

#### Evidence Types Used

- **guard clause**: explicit validation check in the hook script body (regex / substring / glob match).
- **documentation**: `hooks-registry.toml` declares the binding tuple (event, tool, priority, on_error).
- **assertion**: explicit `exit 2` / `emit hook.block` path in the hook body.

#### Purity Classification

| Property | Assessment |
|----------|------------|
| **I/O operations** | reads + writes (stdin JSON, stderr diagnostics, optional event emission via `${CLAUDE_PLUGIN_ROOT}/bin/emit-event`) |
| **Global state access** | reads global (env vars: `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`, optionally `VSDD_*`) |
| **Deterministic** | yes — bash hooks are deterministic given identical stdin envelope and filesystem state |
| **Thread safety** | not applicable (subprocess-isolated invocation per hook fire) |
| **Overall classification** | effectful shell |

#### Refactoring Notes

Bash hook scripts are inherently effectful (stdin/stderr, optional event emit, optional state-file reads). Native (Rust) replacement would extract pure parse/decision logic from the I/O shell, exposing a `fn(payload) -> HookResult` contract per BC-7.02.009. Until that port lands, the contract is preserved by the script body verbatim and the registry binding tuple.


## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.4 | 2026-07-13 | product-owner | S-19.04 adversary pass-1 F-1 (architect ruling 2026-07-13): Invariant 1 amended — "binding tuple unchanged" constraint clarified to cover identity/dispatch characteristics (name, event, plugin, priority, on_error, timeout_ms) and explicitly NOT freeze the tool-pattern field against semantic-intent corrections. S-19.04's re-anchoring of `tool = "Agent"` → `tool = "^Agent$"` (D-a table) is a semantic-intent correction, not a binding-tuple migration change. Description registry entry and Precondition 1 updated to post-S-19.04 anchored form `^Agent$` (implemented by S-19.04, W2 — pending merge). Changelog reordered newest-first (pre-existing ascending-order corrected in same burst; v1.2 row volatile line cite removed — symbol anchor form retained, line number omitted per scanner requirements; historical record in git). |
| v1.3 | 2026-05-09 | state-manager | F-P47-001 fix-burst-43: Traceability Stories TBD→S-8.08 (S-8.08 behavioral_contracts frontmatter cites this BC; bidirectional L-P28-001 propagation). |
| v1.2 | 2026-05-08 | implementer | TD-VSDD-091 Chunk 5 — migrated `hooks-registry.toml` entry `track-agent-start` from script-path form to symbol-anchor form `hooks-registry.toml::track-agent-start`. |
| v1.1 | 2026-04-25 | PHASE_1_4_B_BCS_AGENT_9 | Initial authoring. |
