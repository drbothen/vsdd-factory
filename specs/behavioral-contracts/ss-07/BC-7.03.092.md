---
document_type: behavioral-contract
level: L3
version: "1.3"
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

# Behavioral Contract BC-7.03.092: warn-pending-wave-gate: stderr warning when any wave has gate_status: pending

## Description

warn-pending-wave-gate: stderr warning when any wave has gate_status: pending. Reads `.factory/wave-state.yaml` (SEQUENCE form), finds all waves where `gate_status == "pending"`, and if any exist emits a WAVE GATE REMINDER to stderr listing each pending wave name with invocation hints (`/vsdd-factory:wave-gate` or manual gate_status update).

**Source category:** Lifecycle hooks (Stop event).
**Audit ID:** `BC-AUDIT-1091` (extracted from `pass-3-deep-hooks.md` line 1028).
**Hook implementation:** `crates/hook-plugins/warn-pending-wave-gate/src/lib.rs` (native WASM port; S-8.07).

## Preconditions

1. Trigger: `.factory/wave-state.yaml` exists and is parseable as YAML SEQUENCE form; at least one wave has `gate_status: pending`.

## Postconditions

1. Behavior: Emits stderr WAVE GATE REMINDER listing each pending wave name with invocation hints (`/vsdd-factory:wave-gate` or manual gate_status update).
2. Exit codes: 0 (advisory — never blocks session end regardless of pending wave count).
3. Error handling: If `wave-state.yaml` is absent, unreadable, or malformed, exit 0 with no output (graceful degradation per BC-7.03.091 EC-001/EC-002).

## Invariants

1. Hook implementation identity (`crates/hook-plugins/warn-pending-wave-gate/src/lib.rs`) and registry binding remain stable across the contract lifetime.
2. Exit-code semantics: always 0. This hook NEVER exits 2 (block).
3. `wave-state.yaml.waves` MUST be parsed as a YAML sequence (list) of `WaveEntry` objects. The canonical schema is `waves: [{wave: <name>, gate_status: <status>}, ...]`. See BC-7.03.091 INV-3.
4. The WAVE GATE REMINDER MUST include each pending wave name and at least one actionable invocation hint. The reminder MUST appear on stderr (not stdout) so it does not interfere with the dispatcher's JSON stdout protocol.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No waves have `gate_status: pending` | Exit 0, no output |
| EC-002 | Multiple waves have `gate_status: pending` | Exit 0, WAVE GATE REMINDER lists all pending wave names |
| EC-003 | `wave-state.yaml` present but malformed YAML | Exit 0, no output (graceful parse failure) |
| EC-004 | `gate_status` field absent for a wave entry | Wave entry treated as non-pending (serde default); no REMINDER for that wave |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs. Used for regression testing and agent validation.

| Input | Expected Output | Category |
|-------|-----------------|----------|
| Stop event; wave-state.yaml with `W3.gate_status: pending` | Exit 0, stderr contains "W3" and WAVE GATE REMINDER | happy-path |
| Stop event; wave-state.yaml with two pending waves W1, W2 | Exit 0, stderr contains both "W1" and "W2" | happy-path (EC-002) |
| Stop event; all waves have `gate_status: passed` | Exit 0, no output | happy-path (EC-001) |
| Stop event; wave-state.yaml absent | Exit 0, no output | edge-case |
| Stop event; wave-state.yaml malformed | Exit 0, no output (EC-003) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|--------------|
| VP-TBD | Pending wave detection — any wave with gate_status=pending triggers WAVE GATE REMINDER on stderr | manual/bats |
| VP-TBD | Advisory-only — exit code is always 0 regardless of pending wave count | manual/bats |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-008 (Gate tool calls with pre-execution behavioral checks / lifecycle enforcement) |
| L2 Domain Invariants | wave-state.yaml SEQUENCE schema invariant (INV-3); advisory-only exit code (INV-2) |
| Architecture Module | SS-07 (Hook Bash Layer / native WASM) |
| Stories | S-8.07 |

## Related BCs (Recommended)

- BC-7.03.091 — warn-pending-wave-gate: identity & registry binding (companion contract).

## Architecture Anchors (Recommended)

- `architecture/ss-07-hook-bash.md` — SS-07 module definition.
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
| **Source Document** | `pass-3-deep-hooks.md` line 1028 |
| **Audit ID** | `BC-AUDIT-1091` |
| **Source Line(s) (within hook)** | Lines 32-63. |

#### Evidence Types Used

- **guard clause**: explicit validation check in the hook logic (YAML parse + gate_status scan).
- **assertion**: explicit stderr output path in hook body when pending waves found.

#### Purity Classification

| Property | Assessment |
|----------|------------|
| **I/O operations** | reads (wave-state.yaml via WASI file I/O, stdin JSON envelope); writes (stderr WAVE GATE REMINDER via WASI stdio) |
| **Global state access** | reads `CLAUDE_PROJECT_DIR` (via WASI env) to locate `.factory/wave-state.yaml` |
| **Deterministic** | yes — given identical stdin envelope and filesystem state |
| **Thread safety** | not applicable (subprocess-isolated invocation per hook fire) |
| **Overall classification** | effectful WASM plugin |

#### Refactoring Notes

Native WASM port (S-8.07) replaced the original bash hook. The hook reads wave-state.yaml via WASI file I/O, scans for pending gates, and writes the WAVE GATE REMINDER to stderr. Pure logic is testable via unit tests in `crates/hook-plugins/warn-pending-wave-gate/tests/integration_test.rs`.


## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.3 | 2026-05-10 | implementer | F-P5-002: TBD fields resolved — capability TBD→CAP-008, hook path updated to native WASM lib.rs (S-8.07 port), EC-001 through EC-004 populated, INV-3/INV-4 added (SEQUENCE schema + advisory-only + stderr requirement), traceability L2 Capability populated, VP table expanded, Source Evidence path updated. Changelog reordered newest-first. |
| 1.2 | 2026-05-09 | state-manager | F-P47-001 fix-burst-43: Traceability Stories TBD→S-8.07 (S-8.07 behavioral_contracts frontmatter cites this BC; bidirectional L-P28-001 propagation). |
| 1.1 | 2026-04-25 | PHASE_1_4_B_BCS_AGENT_9 | Initial authoring. |
