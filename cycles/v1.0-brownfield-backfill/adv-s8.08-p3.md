---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.08-native-port-track-agent-start.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.08-p2.md
  - crates/hook-sdk/src/host.rs
  - Cargo.toml
input-hash: "581dca0"
traces_to: prd.md
pass: p3
previous_review: adv-s8.08-p2.md
target: story
target_file: .factory/stories/S-8.08-native-port-track-agent-start.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 2
findings_medium: 1
findings_low: 1
findings_nit: 0
---

# Adversarial Review: S-8.08 v1.2 (Pass 3)

## Finding ID Convention

Finding IDs use the format: `F-S808-P3-<SEQ>` per project short-form convention.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S808-P2-001 | HIGH | RESOLVED | Resolved |
| F-S808-P2-002 | HIGH | RESOLVED | Resolved |
| F-S808-P2-003 | MED | RESOLVED | Resolved |
| F-S808-P2-004 | MED | RESOLVED | Resolved |
| F-S808-P2-005 | MED | RESOLVED | Resolved |
| F-S808-P2-006 | LOW | RESOLVED | Resolved |
| F-S808-P2-007 | LOW | RESOLVED | Resolved |
| F-S808-P2-008 | LOW | RESOLVED | Resolved |
| F-S808-P2-009 BC Trace AC-006 omission | LOW | PARTIALLY_RESOLVED | Propagation residual surfaces as F-S808-P3-003 |

## Part B — New Findings (or all findings for pass 1)

Pass-3 review of S-8.08 v1.2 (411 lines, hash 581dca0). 8 of 9 pass-2 findings closed; F-S808-P2-009 had propagation residual. Pass-2 fix burst introduced 2 NEW regressions (agent_id contract; T-3 stdin error path). 4 findings: 2H, 1M, 1L. Verdict SUBSTANTIVE. Clock held.

### HIGH

#### F-S808-P3-001: agent_id has no host-fn source
- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** S-8.08 AC-002a; T-3 line 319
- **Description:** AC-002a + T-3 require agent_id, but host SDK provides no host fn for it. host.rs:132-160 enumerates only session_id, dispatcher_trace_id, plugin_root, plugin_version, cwd, env. NO host::agent_id(). Bash source does NOT emit agent_id. T-3:319 includes `("agent_id", &agent_id)` without specifying source. AC-002a is unimplementable as written.
- **Evidence:** host.rs:132-160 — enumerates: session_id, dispatcher_trace_id, plugin_root, plugin_version, cwd, env. No agent_id fn. Bash source grep: no agent_id emission.
- **Proposed Fix:** (preferred) Source agent_id from `host::session_id()` and update Goal+T-3 prose, OR specify stdin envelope field path, OR withdraw AC-002a.

#### F-S808-P3-002: T-3 stdin-error path internally contradictory
- **Severity:** HIGH
- **Category:** contradictions
- **Location:** S-8.08 T-3 lines 304-306; AC-006 lines 222-224
- **Description:** T-3 stdin-error path returns HookResult::Error; contradicts AC-006 + EC-005 best-effort exit-0 contract. T-3:304-306 says "return HookResult::Error ... best-effort; exits 0 — no stderr to dispatcher" — internally contradictory. AC-006:222-224 says "exits 0 with no stderr output. No error is surfaced to the dispatcher."
- **Evidence:** T-3:304-306: "return HookResult::Error ... best-effort; exits 0". These are mutually exclusive: HookResult::Error does not exit 0. AC-006:222-224: "exits 0 with no stderr output."
- **Proposed Fix:** Replace T-3 first sub-bullet with "return Ok/exit 0 silently — no event emitted, no stderr."

### MEDIUM

#### F-S808-P3-003: BC Trace "Covered by ACs" omits AC-006 from BC-7.03.079 row
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.08 BC Trace line 113
- **Description:** BC Trace "Covered by ACs" column omits AC-006 from BC-7.03.079 row despite AC-traces table claiming AC-006 traces BC-7.03.079 invariant 2. Pass-2 F-S808-P2-009 propagation residual.
- **Evidence:** AC-traces table: AC-006 → BC-7.03.079 invariant 2. BC Trace line 113: omits AC-006.
- **Proposed Fix:** Update line 113 cell to `AC-001, AC-002a, AC-005, AC-006`.

### LOW

#### F-S808-P3-004: AC-002a fixture extraction mechanism unspecified
- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** S-8.08 T-5; AC-002a
- **Description:** AC-002a fixture description does not specify how bats extracts agent_id from emitted events. Mechanism unspecified (sink-file? tmpdir mock?).
- **Evidence:** T-5 does not describe event capture mechanism for agent_id extraction.
- **Proposed Fix:** T-5 add sink-file event capture + JSONL parse instructions.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 1 |
| LOW | 1 |
| NIT | 0 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Universal Patches: ALL VERIFIED

(SS-04→SS-02 SKIP-FIX correct: plugin-side; emit_event slice-of-tuples; vsdd-hook-sdk path `../../hook-sdk`; workspace members T-2; S-8.29)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 4 |
| **Duplicate/variant findings** | 1 (P2-009 propagation residual) |
| **Novelty score** | 0.8 (4/5) |
| **Median severity** | 3.5 |
| **Trajectory** | 9→4 (56% decay) |
| **Verdict** | FINDINGS_REMAIN |
