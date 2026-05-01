---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.06-native-port-session-learning.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.06-p2.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.076.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - crates/hook-sdk/src/host.rs
  - plugins/vsdd-factory/hooks-registry.toml
  - Cargo.toml
input-hash: "e441e99"
traces_to: prd.md
pass: p3
previous_review: adv-s8.06-p2.md
target: story
target_file: .factory/stories/S-8.06-native-port-session-learning.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 2
findings_medium: 3
findings_low: 3
findings_nit: 0
---

# Adversarial Review: S-8.06 v1.2 (Pass 3)

## Finding ID Convention

Finding IDs use the format: `F-S806-P3-<SEQ>` per project short-form convention.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S806-P2-001 "may include" → MUST | HIGH | RESOLVED | MUST applied |
| F-S806-P2-002 AC numbering gap | MED | RESOLVED | Gap closed |
| F-S806-P2-003 BC trace fabrication | MED | PARTIALLY_RESOLVED | Still does not match BC postcondition 1 verbatim — REGRESSION |
| F-S806-P2-004 EC-005 quantified | MED | RESOLVED | Quantified |
| F-S806-P2-005 wave qualifier | LOW | RESOLVED | Qualifier added |
| F-S806-P2-006 BC trace title | LOW | PARTIALLY_RESOLVED | Changed but to wrong canonical — REGRESSION |
| F-S806-P2-007 AC-001 absolute | LOW | RESOLVED | Absolute path applied |
| F-S806-P2-008 T-1 blocking condition | LOW | RESOLVED | Condition added |
| F-S806-P2-009 EC-001 silent rationale | LOW | RESOLVED | Rationale added |

## Part B — New Findings (or all findings for pass 1)

Pass-3 review of S-8.06 v1.2 (hash e441e99). 7 of 9 pass-2 findings closed; 2 partial-fix regressions (F-S806-P2-003 BC trace fabrication, F-S806-P2-006 title fix mis-applied). 8 findings: 2H, 3M, 3L. Verdict SUBSTANTIVE. Clock held. Trajectory 9 -> 8 (~11% decay; pattern of partial-closure regressions).

### HIGH

#### F-S806-P3-001: BC-7.03.076 trace cell does not match BC postcondition wording
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.06 line 133; BC-7.03.076 postcondition 1
- **Description:** Story line 133 says "AC-001: WASM crate implements Stop binding..."; actual BC postcondition 1 is "Behavior: Append-only marker for `/session-review`; never blocks; minimal binary_allow=[bash]." AC-001 mandates empty binary_allow but BC postcondition says binary_allow=[bash]. POLICY 7 violation.
- **Evidence:** BC-7.03.076 postcondition 1 (verbatim): "Behavior: Append-only marker for `/session-review`; never blocks; minimal binary_allow=[bash]." Story line 133 does not quote this.
- **Proposed Fix:** File BC-update PR before coding (T-1 sub-task), OR make trace contingent on S-8.00 BC-anchor table executable diff.

#### F-S806-P3-002: BC table title contradicts BC-INDEX/H1
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.06 BC table; BC-INDEX:1603
- **Description:** Story says "session-learning: append-only marker; never blocks"; BC file H1 + BC-INDEX:1603 say "session-learning: identity & registry binding". POLICY 7 violation. Pass-2 F-S806-P2-006 closure changed title to wrong canonical.
- **Evidence:** BC-INDEX:1603 (verbatim): "session-learning: identity & registry binding". Story BC table uses different title.
- **Proposed Fix:** Restore to "session-learning: identity & registry binding".

### MEDIUM

#### F-S806-P3-003: hooks-registry.toml line range stale
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.06 BC reference
- **Description:** hooks-registry.toml line range stale (BC says 780-797; actual 845-862). Same defect class as S-8.04.
- **Evidence:** hooks-registry.toml current session-learning entry at lines 845-862; BC reference says 780-797.
- **Proposed Fix:** Flag stale in T-1 OR file BC-update PR alongside post-migration update.

#### F-S806-P3-004: T-6 silent on top-level [hooks.capabilities] env_allow block
- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** S-8.06 T-6
- **Description:** T-6 silent on top-level `[hooks.capabilities]` env_allow block (only addresses sub-block exec_subprocess). Sibling S-8.05 pass-2 raised same gap.
- **Evidence:** T-6 addresses exec_subprocess sub-block; top-level env_allow block not addressed.
- **Proposed Fix:** T-6 explicitly states whether top-level block + env_allow is removed/retained empty.

#### F-S806-P3-005: "host::write_file ABSENT" note risks misgeneralization
- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** S-8.06 T-2
- **Description:** "host::write_file ABSENT" repetition risks misgeneralization that read_file is also absent.
- **Evidence:** host::read_file IS available; session-learning is append-only via std::fs, not because read_file is absent.
- **Proposed Fix:** T-2 narrows note: "host::read_file IS available but unused — session-learning is append-only."

### LOW

#### F-S806-P3-006: AC-001 silent on parent [hooks.capabilities] block
- **Severity:** LOW
- **Category:** missing-edge-cases
- **Location:** S-8.06 AC-001
- **Description:** AC-001 silent on parent `[hooks.capabilities]` block (coupled with F-S806-P3-004).
- **Proposed Fix:** Add explicit statement on parent block disposition.

#### F-S806-P3-007: Self-reference loop "must be documented as comment" decorative
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** S-8.06 no-emit_event section
- **Description:** Self-reference loop "must be documented as comment" — comment is decorative since grep test enforces no-emit_event. Drop "must be documented" prescription.
- **Proposed Fix:** Replace "must be documented" with "may be documented; grep test is authoritative enforcement."

#### F-S806-P3-008: [process-gap] Wave 15 section tag has no gap content
- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.06 Wave 15 section
- **Description:** [process-gap] Wave 15 section heading has the tag but body is just routine provisional disclosure (no actual gap content).
- **Proposed Fix:** Either remove tag or add concrete process-gap text describing the actual gap.

## [process-gap]

- Pass-2 closure of F-S806-P2-003 was not validated against actual BC file content. A `validate-bc-trace-match` hook (BC postcondition string substring check) would catch this class. Same pattern likely in S-8.01-S-8.09 siblings.
- BC-INDEX line-number staleness not auto-detected. A periodic `validate-bc-source-anchor-freshness` job would prevent BC source references from drifting silently.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 3 |
| LOW | 3 |
| NIT | 0 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Universal Patches Verification

| Patch | Result |
|-------|--------|
| SS-04→SS-02 SKIP-FIX | CORRECT (plugin-side, not SDK-modifier) |
| emit_event prohibition | VERIFIED |
| read_file justification | VERIFIED (uses std::fs append; read_file unused) |
| vsdd-hook-sdk path | PASS (`../../hook-sdk`) |
| Workspace members | PASS (T-2 line 322) |
| S-8.28→S-8.29 | PASS |

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 8 |
| **Duplicate/variant findings** | 2 (P2-003 and P2-006 regressions) |
| **Novelty score** | 0.8 (8/10) |
| **Median severity** | 2.5 |
| **Trajectory** | 9→8 |
| **Verdict** | FINDINGS_REMAIN |
