---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
  - .factory/stories/S-8.10-sdk-extension-write-file.md
  - .factory/stories/STORY-INDEX.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.04-p2.md
  - crates/hook-sdk/src/host.rs
  - crates/hook-plugins/capture-commit-activity/Cargo.toml
  - Cargo.toml
  - plugins/vsdd-factory/hooks-registry.toml
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "c988344"
traces_to: prd.md
pass: p3
previous_review: adv-s8.04-p2.md
target: story
target_file: .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
verdict: MINOR
clock: 0_of_3
findings_critical: 0
findings_high: 1
findings_medium: 0
findings_low: 3
findings_nit: 0
---

# Adversarial Review: S-8.04 v1.2 (Pass 3)

## Finding ID Convention

Finding IDs use the format: `F-S804-P3-<SEQ>` per project short-form convention.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S804-P2-001 | HIGH | RESOLVED | D-6 blocker disclosure confirmed |
| F-S804-P2-002 | HIGH | RESOLVED | S-8.10 dependency added |
| F-S804-P2-003 | MED | RESOLVED | jq removal tied to host::write_file |
| F-S804-P2-004 | MED | RESOLVED | API quote matches S-8.10 AC-1 |
| F-S804-P2-005 | MED | RESOLVED | STORY-INDEX row confirmed |
| F-S804-P2-006 | MED | RESOLVED | depends_on updated |
| F-S804-P2-007 | MED | RESOLVED | Resolved |
| F-S804-P2-008 | LOW | RESOLVED | Resolved |
| F-S804-P2-009 | LOW | RESOLVED | Resolved |
| F-S804-P2-010 | LOW | RESOLVED | Resolved |
| F-S804-P2-011 | LOW | RESOLVED | Resolved |

## Part B — New Findings (or all findings for pass 1)

Pass-3 review of S-8.04 v1.2 (526 lines, hash c988344). All 11 pass-2 findings closed. All universal patches verified. D-6 unblocker path complete. 4 findings: 1H (D-6 process-gap, no spec fix), 3L. Verdict MINOR. Clock 0/3 held (D-6 blocker external). Trajectory 17 -> 11 -> 4 (64% decay).

### HIGH

#### F-S804-P3-001: D-6 process-gap — external blocker persists
- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** S-8.04 BLOCKER section
- **Description:** D-6 Option A blocker still active — host::write_file absent in vsdd-hook-sdk; S-8.10 v1.0 status=draft. Story remains correctly BLOCKED. No spec fix required; persistent blocker disclosure across passes 1/2/3.
- **Evidence:** host.rs does not contain write_file. S-8.10 status=draft.
- **Proposed Fix:** No spec fix — external blocker. Document persistent status; resolved when S-8.10 merges.

### MEDIUM

_None._

### LOW

#### F-S804-P3-002: AC-006 fixture field name mismatch
- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.04 AC-006 line 279
- **Description:** AC-006 narrative refers to fixture as `gate: ~` while actual fixture content uses `gate_status: ~` (line 279 vs line 270-278 YAML).
- **Evidence:** Fixture YAML at lines 270-278 uses `gate_status: ~`; AC-006 narrative at line 279 says `gate: ~`.
- **Proposed Fix:** Replace `gate: ~` with `gate_status: ~` on L279.

#### F-S804-P3-003: Library Table version assertion premature
- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.04 Library Table line 488
- **Description:** Library Table claims `vsdd-hook-sdk = 0.2.0` but post-S-8.10 bump not yet realized; current Cargo.toml is 0.1.0.
- **Evidence:** Current Cargo.toml vsdd-hook-sdk = 0.1.0. S-8.10 has not merged.
- **Proposed Fix:** Add T-2 verification step "Confirm 0.2.0 before continuing — if 0.1.x, S-8.10 incomplete; STOP", OR reword L488 to "0.2.0 (asserted post-S-8.10 merge — gate at T-0)".

#### F-S804-P3-004: T-1.5 "before deprecation" wording drift
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** S-8.04 T-1.5
- **Description:** T-1.5 says "0.9.34 (last release before deprecation)" — minor wording drift; "before deprecation" implies a 0.9.35+ exists.
- **Evidence:** dtolnay deprecated the crate at 0.9.34; no 0.9.35 exists.
- **Proposed Fix:** Replace with "(last release; dtolnay deprecated the crate at this version)".

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 3 |
| NIT | 0 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate (D-6 external)
**Readiness:** requires external dependency resolution (S-8.10)

## D-6 Unblocker Verification

- depends_on: ["S-8.00", "S-8.10"] — VERIFIED
- BLOCKER references S-8.10 (no TBD) — VERIFIED
- API quote matches S-8.10 AC-1 — VERIFIED
- jq removal tied to host::write_file (T-9) — VERIFIED
- S-8.10 row in STORY-INDEX confirmed; blocks: S-8.04, S-8.09 — VERIFIED

## Universal Patches: ALL VERIFIED (incl. SS-04 SKIP-FIX correctly justified)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 4 |
| **Duplicate/variant findings** | 1 (D-6 recurring process-gap) |
| **Novelty score** | 0.8 (4/5) |
| **Median severity** | 1.5 |
| **Trajectory** | 17→11→4 |
| **Verdict** | FINDINGS_REMAIN |
