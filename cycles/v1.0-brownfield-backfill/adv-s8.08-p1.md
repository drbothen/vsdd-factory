---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.08-native-port-track-agent-start.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.08 v1.0
target_file: .factory/stories/S-8.08-native-port-track-agent-start.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 12
findings_high: 4
findings_med: 5
findings_low: 2
findings_nit: 1
---

# Adversarial Review: S-8.08 v1.0 (Pass 1)

## Finding ID Convention

`F-S808-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S808-P1-001: hooks.json deletion ACs/Tasks/File-list reference entries that do not exist (DRIFT-004)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-002 / Task list / File Structure
- **Description:** Same class as F-S801-P1-001. The hooks.json command entries for track-agent-start do not exist post-DRIFT-004. The deletion steps in AC-002, the task list, and the File Structure section are phantom operations.
- **Evidence:** DRIFT-004 in E-8 epic: hooks.json entries already removed; hooks-registry.toml is sole dispatch mechanism.
- **Proposed Fix:** Remove all hooks.json deletion references from AC-002, tasks, and File Structure, or rewrite as verification assertions.

#### F-S808-P1-002: Goal/Architecture Compliance Rules contradict registry reality (track-agent-start is registry-only post-port)

- **Severity:** HIGH
- **Category:** contradictions
- **Location:** Goal / Architecture Compliance Rules
- **Description:** The Goal and Architecture Compliance Rules describe track-agent-start as being in hooks.json during the port process. Post-port, it is registry-only. The Goal should describe the WASM crate that replaces the bash hook, not the hooks.json lifecycle, since the hooks.json entries no longer exist.
- **Proposed Fix:** Rewrite Goal to describe the WASM port output: "Create crates/hook-plugins/track-agent-start/ implementing the SubagentStart event handler as a native WASM plugin registered in hooks-registry.toml."

#### F-S808-P1-003: AC-002 traces to BC-7.03.079 invariant 1 which is about identity/registry binding, not deletion semantics

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-002 → BC-7.03.079 invariant 1
- **Description:** AC-002 is a deletion AC. It traces to BC-7.03.079 invariant 1, which governs identity and registry binding stability (the hook is correctly registered and identified). Invariant 1 does not contain deletion semantics; it is about persistent registration, which is actually in tension with a deletion AC.
- **Evidence:** BC-7.03.079 invariant 1 semantics: identity/registration stability. AC-002: deletion of script path.
- **Proposed Fix:** Remove the BC trace from AC-002's deletion step (no BC governs deletion), or find the correct BC if one exists for the WASM migration deletion step.

#### F-S808-P1-004: Capability Anchor Justification claims SS-07 ownership but contract migrates to SS-01 post-port

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** Capability Anchor Justification
- **Description:** The capability justification claims SS-07 (operations/tooling) ownership. Post-port, track-agent-start migrates to SS-01 (hook dispatcher) + SS-04 (plugin ecosystem). SS-07 will not own the WASM plugin. Same class as F-S802-P1-010.
- **Proposed Fix:** Update capability justification to reflect SS-01 + SS-04 post-port ownership.

### MEDIUM

#### F-S808-P1-005: Goal contradicts AC-005 — "lower latency" claim has no AC enforcement

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** Goal vs AC-005
- **Description:** The Goal states "lower latency" as a delivery outcome. AC-005 does not include a latency gate or measurement requirement. The "lower latency" claim is aspirational in the Goal but unenforceable in the ACs.
- **Proposed Fix:** Either remove "lower latency" from the Goal (as unverified) or add a latency measurement AC (non-blocking INFORMATIONAL per E-8 AC-7 Tier 1 exclusion).

#### F-S808-P1-006: T-6 exec_subprocess block removal logic contradicts actual removal for track-agent-start

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** T-6
- **Description:** T-6 prescribes removing the `exec_subprocess` block from the registry entry. However, the removal logic described does not match the actual track-agent-start registry entry structure. The exec_subprocess block may have different fields or nesting than described.
- **Proposed Fix:** Verify T-6's removal instructions against the actual registry entry and correct field references.

#### F-S808-P1-007: input-hash references S-8.00 commit hash vs E-8 epic hash per convention

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** Frontmatter input-hash comment
- **Description:** The input-hash comment references the S-8.00 commit hash. The inputs list references the E-8 epic. Convention (per D-170) is E-8 epic content hash. The two are inconsistent.
- **Proposed Fix:** Update input-hash comment to say "E-8 epic content hash — shared across Tier 1 siblings per convention."

#### F-S808-P1-008: AC-005(f) perf comparison vs S-8.00 baseline impossible — track-agent-start not in S-8.00 measured set

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-005(f)
- **Description:** AC-005(f) compares WASM warm invocation time to S-8.00 baseline. The S-8.00 baseline measurement does not include track-agent-start. Same class as F-S807-P1-011.
- **Proposed Fix:** Remove the S-8.00 baseline reference. Either define a per-story baseline or mark INFORMATIONAL per E-8 AC-7 Tier 1 exclusion.

#### F-S808-P1-009: EC-006 vs T-3 emit_event error handling — "silently swallowed" vs "exit 0 silently" — HOW not specified

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** EC-006 vs T-3
- **Description:** EC-006 says emit_event failures are "silently swallowed." T-3 says "exit 0 silently." These are consistent in outcome (exit 0) but differ in whether the error is logged to stderr. The implementer needs to know whether to log the error before exiting. The bash OR-true pattern logs nothing; the WASM port should specify equivalent behavior.
- **Proposed Fix:** Add to EC-006: "on host::emit_event error, do not log to stderr; exit 0 silently (matching bash `|| true` pattern)."

### LOW

#### F-S808-P1-010: wasm32-wasi target name deprecated; should use wasm32-wasip1

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** T-2 / Library & Framework Requirements
- **Description:** Universal systematic finding. wasm32-wasi deprecated; correct target is wasm32-wasip1.
- **Proposed Fix:** Replace wasm32-wasi with wasm32-wasip1.

#### F-S808-P1-011: Token Budget claims "agent context window 200K for Sonnet" but adversary may run on Opus

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Token Budget
- **Description:** Token Budget assumes 200K Sonnet context window. If adversary runs on Opus (100K window), the budget estimate is wrong.
- **Proposed Fix:** Clarify target model or provide two estimates.

### NIT

#### F-S808-P1-012: Frontmatter behavioral_contracts consistent with body BC table (POLICY 8 PASS — positive acknowledgment)

- **Severity:** NIT
- **Category:** spec-fidelity
- **Location:** Frontmatter behavioral_contracts vs body BC table
- **Description:** POLICY 8 positive acknowledgment: the frontmatter behavioral_contracts and body BC table are consistent for S-8.08. This is noted as a PASS for the POLICY 8 check; no action required.
- **Proposed Fix:** No action required. Positive finding noted for audit trail.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 4 |
| MEDIUM | 5 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst before pass-2)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 12 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 12/12 = 1.00 |
| **Median severity** | HIGH/MED |
| **Trajectory** | 12 |
| **Verdict** | FINDINGS_REMAIN |
