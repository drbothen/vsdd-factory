---
document_type: behavioral-contract
level: L3
version: "1.5"
status: draft
producer: "PHASE_1_4_B_BCS_AGENT_9"
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [pass-3-deep-hooks.md, pass-3-behavioral-contracts.md, pass-3-behavioral-contracts-deep-r1.md, bc-id-mapping.md]
input-hash: "[live-state]"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "pass-3-deep-hooks.md:1016"
subsystem: "SS-07"
capability: "CAP-008"
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

# Behavioral Contract BC-7.03.091: warn-pending-wave-gate: identity & registry binding

## Description

warn-pending-wave-gate: identity & registry binding. Session-end safety net for forgotten wave gates.

**Source category:** Lifecycle hooks (Stop event).
**Audit ID:** `BC-AUDIT-1090` (extracted from `pass-3-deep-hooks.md` line 1016).
**Hook implementation:** `crates/hook-plugins/warn-pending-wave-gate/src/lib.rs` (native WASM port; S-8.07).
**Registry entry:** `hooks-registry.toml::warn-pending-wave-gate` (Stop, no tool filter, priority=920, timeout_ms=5000, on_error=continue).

## Preconditions

1. Hook event/tool match: Stop.

## Postconditions

1. Behavior: Session-end safety net for forgotten wave gates. Reads `.factory/wave-state.yaml` and checks all waves for `gate_status: pending`.
2. Exit codes: Always 0 (advisory — never blocks session end).
3. Error policy: on_error=continue (graceful degradation on parse failure or absent file).

## Invariants

1. Hook implementation identity (`crates/hook-plugins/warn-pending-wave-gate/src/lib.rs`) and registry binding remain stable across the contract lifetime.
2. Exit-code semantics conform to the dispatcher contract: 0 = allow / advisory. This hook NEVER exits 2 (block).
3. `wave-state.yaml.waves` MUST be parsed as a YAML sequence (list) of `WaveEntry` objects — NOT as a YAML mapping. The canonical schema is the SEQUENCE form (`waves: [{wave: ..., gate_status: ...}, ...]`). The prior MAPPING form was removed in F-P3-001 fix-burst (TD-073 root cause). Any schema change that reverts to MAPPING form violates this invariant.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `wave-state.yaml` absent or unreadable | Exit 0 with no output (graceful — file absence is not an error at session end) |
| EC-002 | `wave-state.yaml` present but malformed YAML | Exit 0 with no output (parse failure → treat as no pending waves) |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs. Used for regression testing and agent validation.

| Input | Expected Output | Category |
|-------|-----------------|----------|
| Stop event; no `.factory/wave-state.yaml` | Exit 0, no stderr output | happy-path |
| Stop event; wave-state.yaml with `gate_status: not_started` | Exit 0, no WAVE GATE REMINDER | happy-path |
| Stop event; wave-state.yaml YAML sequence form with one pending wave | Exit 0, stderr WAVE GATE REMINDER mentioning pending wave name | happy-path |
| Stop event; wave-state.yaml YAML mapping form | Exit 0, no output (parse fails gracefully; EC-002) | edge-case |
| Stop event; wave-state.yaml absent | Exit 0, no output (EC-001) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|--------------|
| (no formal VP — covered by bats test warn-pending-wave-gate.bats AC-001: verifies hook fires on Stop, hook name resolves, registry binding confirmed via hooks-registry.toml inspection) | Identity and registry binding — hook name, event, plugin path, priority, on_error in hooks-registry.toml match BC-7.03.091 specification | manual/bats |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-008 (Gate tool calls with pre-execution behavioral checks / lifecycle enforcement) |
| L2 Domain Invariants | wave-state.yaml SEQUENCE schema invariant (INV-3 above) |
| Architecture Module | SS-07 (Hook Bash Layer / native WASM) |
| Stories | S-8.07 |

## Related BCs (Recommended)

- BC-7.03.092 — warn-pending-wave-gate: stderr warning behavior (companion contract).

## Architecture Anchors (Recommended)

- `architecture/SS-07-hook-bash.md` — SS-07 module definition.
- `hooks-registry.toml::warn-pending-wave-gate` — authoritative runtime binding.

## Story Anchor (Recommended)

S-8.07 — native-port-warn-pending-wave-gate.

## VP Anchors (Recommended)

- VP assignment pending VP synthesis for S-8.07 acceptance criteria.

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/warn-pending-wave-gate/src/lib.rs` (native WASM; S-8.07 port from `plugins/vsdd-factory/hooks/warn-pending-wave-gate.sh`) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Document** | `pass-3-deep-hooks.md` line 1016 |
| **Audit ID** | `BC-AUDIT-1090` |
| **Source Line(s) (within hook)** | Header. |

#### Evidence Types Used

- **guard clause**: explicit validation check in the hook logic (YAML parse + gate_status scan).
- **documentation**: `hooks-registry.toml` declares the binding tuple (event, tool, priority, on_error).

#### Purity Classification

| Property | Assessment |
|----------|------------|
| **I/O operations** | reads (wave-state.yaml via WASI file I/O, stdin JSON envelope); writes (stderr diagnostics via WASI stdio) |
| **Global state access** | reads `CLAUDE_PROJECT_DIR` (via WASI env) to locate `.factory/wave-state.yaml` |
| **Deterministic** | yes — given identical stdin envelope and filesystem state |
| **Thread safety** | not applicable (subprocess-isolated invocation per hook fire) |
| **Overall classification** | effectful WASM plugin |

#### Refactoring Notes

Native WASM port (S-8.07) replaced the original bash hook. Pure parse/decision logic is in `hook_logic()` (`crates/hook-plugins/warn-pending-wave-gate/src/lib.rs`). The `#[hook]` macro wires stdin/stdout framing. WASI file I/O is used for wave-state.yaml access.


## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.5 | 2026-05-10 | implementer | F-P6-005: VP-TBD entry replaced with explicit no-formal-VP declaration citing covering bats test (warn-pending-wave-gate.bats AC-001). F-P6-006: architecture anchor path corrected ss-07-hook-bash.md → SS-07-hook-bash.md (uppercase prefix matches actual file). |
| 1.4 | 2026-05-10 | implementer | F-P5-002: TBD fields resolved — capability TBD→CAP-008, hook path updated to native WASM lib.rs (S-8.07 port), EC-001/EC-002 populated (absent/malformed YAML), INV-3 added (SEQUENCE schema invariant, closes TD-073 root cause governance gap), traceability L2 Capability populated, Source Evidence path updated. Changelog reordered newest-first. |
| 1.3 | 2026-05-09 | state-manager | F-P47-001 fix-burst-43: Traceability Stories TBD→S-8.07 (S-8.07 behavioral_contracts frontmatter cites this BC; bidirectional L-P28-001 propagation). |
| 1.2 | 2026-05-08 | implementer | TD-VSDD-091 Chunk 5 — migrated `hooks-registry.toml:799-816` → `hooks-registry.toml::warn-pending-wave-gate`. |
| 1.1 | 2026-04-25 | PHASE_1_4_B_BCS_AGENT_9 | Initial authoring. |
