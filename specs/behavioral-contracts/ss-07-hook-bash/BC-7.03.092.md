---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: "PHASE_1_4_B_BCS_AGENT_9"
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [pass-3-deep-hooks.md, pass-3-behavioral-contracts.md, pass-3-behavioral-contracts-deep-r1.md, bc-id-mapping.md]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "pass-3-deep-hooks.md:1028"
subsystem: "SS-07"
capability: "TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-7.03.092: warn-pending-wave-gate: stderr warning when any wave has gate_status: pending

## Description

warn-pending-wave-gate: stderr warning when any wave has gate_status: pending. Emits hook.block severity=warn (`pending_wave_gate_at_session_end`) with comma-joined `pending_waves`. Stderr WAVE GATE REMINDER lists each pending wave + invocation hints (`/vsdd-factory:wave-gate` or manual gate_status update).

**Source category:** Routing hooks (PreToolUse, PostToolUse, lifecycle).
**Audit ID:** `BC-AUDIT-1091` (extracted from `pass-3-deep-hooks.md` line 1028).
**Hook script:** ``plugins/vsdd-factory/hooks/warn-pending-wave-gate.sh``.

## Preconditions

1. Trigger: `.factory/wave-state.yaml` exists; python3 available; any wave has `gate_status: pending`.

## Postconditions

1. Behavior: Emits hook.block severity=warn (`pending_wave_gate_at_session_end`) with comma-joined `pending_waves`. Stderr WAVE GATE REMINDER lists each pending wave + invocation hints (`/vsdd-factory:wave-gate` or manual gate_status update).
2. Exit codes: 0.

## Invariants

1. Hook script identity (script path) and registry binding remain stable across the contract lifetime.
2. Exit-code semantics conform to the dispatcher contract: 0 = allow / advisory, 2 = block, 1 = jq-missing-fail-closed (where applicable).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs. Used for regression testing and agent validation.

| Input | Expected Output | Category |
|-------|-----------------|----------|
| Acceptance criterion (see below) | Stop with `waves.W3.gate_status: pending` produces stderr "W3 gate is pending". | happy-path |
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
| Stories | TBD (filled by story-writer) |

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
| **Path** | ``plugins/vsdd-factory/hooks/warn-pending-wave-gate.sh`` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Document** | `pass-3-deep-hooks.md` line 1028 |
| **Audit ID** | `BC-AUDIT-1091` |
| **Source Line(s) (within hook)** | 32-63. |

#### Evidence Types Used

- **guard clause**: explicit validation check in the hook script body (regex / substring / glob match).
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

