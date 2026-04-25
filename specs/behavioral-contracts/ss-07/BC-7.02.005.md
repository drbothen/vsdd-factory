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
extracted_from: "pass-3-behavioral-contracts-deep-r1.md:220"
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

# Behavioral Contract BC-7.02.005: Hook latency budget is sub-100ms; deterministic; LLM-free

## Description

Hook latency budget is sub-100ms; deterministic; LLM-free. Each script's docstring explicitly asserts "Deterministic, <100ms, no LLM" or similar. Body contains no network calls, no LLM SDK invocations, only `grep`/`sed`/`jq`/`awk` against bounded file content.

**Source category:** Validator hook class contracts.
**Audit ID:** `BC-AUDIT-103` (extracted from `pass-3-behavioral-contracts-deep-r1.md` line 220).

## Preconditions

1. Any validator hook.

## Postconditions

1. Each script's docstring explicitly asserts "Deterministic, <100ms, no LLM" or similar. Body contains no network calls, no LLM SDK invocations, only `grep`/`sed`/`jq`/`awk` against bounded file content.

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
| **Path** | TBD |
| **Confidence** | high (declared explicitly across 5 of 5 sampled validators). |
| **Extraction Date** | 2026-04-25 |
| **Source Document** | `pass-3-behavioral-contracts-deep-r1.md` line 220 |
| **Audit ID** | `BC-AUDIT-103` |
| **Source Line(s) (within hook)** | TBD |

#### Evidence Types Used

- **guard clause**: explicit validation check in the hook script body (regex / substring / glob match).

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

#### Original Source Evidence Quote

> `validate-novelty-assessment.sh:11-12` ("Deterministic, <100ms, no LLM"); `validate-bc-title.sh:11-12`; `validate-state-size.sh:10-11`; `pr-manager-completion-guard.sh:14-15`; `protect-secrets.sh:19-20` ("Deterministic, <50ms, no LLM").

