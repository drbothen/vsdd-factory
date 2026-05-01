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
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.083.md
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/hook-sdk/src/host.rs
input-hash: "c988344"
traces_to: prd.md
pass: p4
previous_review: adv-s8.04-p3.md
target: story
target_file: .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 2
findings_medium: 1
findings_low: 3
findings_nit: 0
---

# Adversarial Review: S-8.04 v1.2 (Pass 4)

## Finding ID Convention

Finding IDs use the format: `F-S804-P4-<SEQ>`

- `F`: Fixed prefix
- `S804`: Story identifier
- `P4`: Pass number
- `<SEQ>`: Three-digit sequence

## Part A — Fix Verification (Pass-3 carryover)

No v1.3 fix burst was applied to S-8.04 between pass-3 and pass-4. The D-6 external blocker (host::write_file absent from SDK) remains in force; S-8.10 is the SDK extension story that resolves it.

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S804-P3-001 D-6 blocker | External | UNCHANGED | S-8.10 v1.1 authored; D-6 Option A resolution in progress |
| F-S804-P3-002 fixture key drift | LOW | UNRESOLVED | Line 279 still uses `gate: ~` instead of `gate_status: ~` |
| F-S804-P3-003 vsdd-hook-sdk version assertion | LOW | UNRESOLVED | Library table version assertion still premature |
| F-S804-P3-004 T-1.5 wording drift | LOW | UNRESOLVED | "before deprecation" wording unchanged |

## Part B — New Findings (6)

### HIGH

#### F-S804-P4-001: SS-04 fabricated subsystem name — POLICY 6 (NEVER SKIP-FIX)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.04 lines 56, 115-118, 122
- **Description:** Lines 56, 115-118, and 122 reference "SS-04 Wave State" as the subsystem anchor, with an adjacent "canonical name confirmed" claim. This name is entirely fabricated — it does not appear anywhere in ARCH-INDEX.md or the subsystem registry. ARCH-INDEX:77 specifies SS-04 canonical name as "Plugin Ecosystem." The fabrication is more severe than S-8.02's "Hook Plugins" drift (which was a real-word contraction of the correct name); "Wave State" is a different concept entirely. Blast radius is potentially >= 5 stories that may have copied this anchor. POLICY 6: NEVER SKIP-FIX.
- **Evidence:** ARCH-INDEX:77 = "Plugin Ecosystem". SS-04-plugin-ecosystem.md H1 = "Plugin Ecosystem". S-8.04 lines 56/115-118/122 = "Wave State". No registry entry named "Wave State" exists.
- **Proposed Fix:** Replace all "SS-04 Wave State" occurrences with "SS-04 Plugin Ecosystem". Conduct a story-body sweep for any "Wave State" text used as an SS-04 anchor.

#### F-S804-P4-002: write_file API drift — S-8.10 v1.1 max_bytes parameter missing

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.04 lines 92, 232, 421, 488 (all write_file call sites)
- **Description:** S-8.10 v1.1 (authored 2026-05-01) added a mandatory `max_bytes: u32` parameter to the write_file signature, making the canonical signature `write_file(path, data, max_bytes)`. S-8.04 v1.2 was authored on 2026-04-30 against S-8.10 v1.0's 3-parameter signature. All 4 write_file call sites in S-8.04 use the old 3-parameter form. Pass-3 verification F-S804-P2-004 was correct at time of writing (S-8.10 v1.0) but has been invalidated by the sibling change. This is a partial-fix regression triggered by sibling drift, not an oversight by the S-8.04 author.
- **Evidence:** S-8.10 v1.1 AC-1 signature = `write_file(path: &str, data: &[u8], max_bytes: u32)`. S-8.04 lines 92/232/421/488 = `write_file(path, data)` (2-parameter form).
- **Proposed Fix:** Update all 4 write_file call sites to 3-parameter form: `write_file(path, data, max_bytes)`. Add a `max_bytes` constant definition in the Library table (e.g., `const MAX_WAVE_STATE_BYTES: u32 = 65536`).

### MEDIUM

#### F-S804-P4-003: BC-7.03.083 verbatim quote fails line-range check

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.04 BC trace table row for BC-7.03.083
- **Description:** BC-7.03.083 verbatim quote in the story BC table cites registry line 877-894. Empirical check of the live hooks-registry.toml shows the relevant entry at lines 942-948 (the registry was expanded after BC-7.03.083 was authored). The story acknowledges the drift via parenthetical but does not update the BC source file itself. The BC file (ss-07/BC-7.03.083.md) remains the source-of-truth with stale line numbers. This is a sibling-drift class defect (BC L3 source-of-truth stale vs registry).
- **Proposed Fix:** Update BC-7.03.083.md line 36 to cite the current registry range (942-948). This is a BC file update, not a story file update.

### LOW

#### F-S804-P4-004: Library table version assertion premature (= F-S804-P3-003, not applied)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.04 Library table
- **Description:** Library table version assertion for vsdd-hook-sdk is premature — version will not be known until S-8.10 SDK PR is merged and Cargo.toml is updated. Unchanged from pass-3.
- **Proposed Fix:** Replace version number with "TBD (pending S-8.10 merge)."

#### F-S804-P4-005: AC-006 fixture key `gate: ~` vs `gate_status: ~` (= F-S804-P3-002, not applied)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.04 AC-006, line 279
- **Description:** AC-006 bats fixture uses `gate: ~` but the actual wave-state.yaml schema uses `gate_status` as the key name. The fixture key drift means the bats test would not exercise the correct field. Unchanged from pass-3.
- **Proposed Fix:** Replace `gate: ~` with `gate_status: ~` in the AC-006 fixture block.

#### F-S804-P4-006: T-1.5 "before deprecation" wording drift (= F-S804-P3-004, not applied)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.04 T-1.5
- **Description:** T-1.5 task wording uses "before deprecation" which implies the hook will be deprecated. The correct framing is "pending S-8.09 Tier 1 gate closure." Unchanged from pass-3.
- **Proposed Fix:** Replace "before deprecation" with "pending S-8.09 regression-gate closure."

## Process-gap Observations

- Pass-3 claimed "SS-04 SKIP-FIX correctly justified" — this was inaccurate. Pass-3 verified frontmatter only and did not check body label against ARCH-INDEX. "Wave State" fabrication survived pass-3.
- S-8.10 v1.1 changed the write_file signature after S-8.04 was authored. No downstream-consumer alert mechanism exists. Recommend adding a "downstream_consumers" frontmatter field to SDK-extension stories so dependents can be identified when the API changes.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 1 |
| LOW | 3 |
| NIT | 0 |

**Overall Assessment:** block — 2 HIGH including fabricated subsystem name and API drift
**Convergence:** regression (+2 net; trajectory inflected upward)
**Readiness:** requires revision (v1.3 fix burst required; NOT eligible for SKIP-FIX-ADVANCE)

## Verdict

**SUBSTANTIVE** — NOT eligible for SKIP-FIX-ADVANCE. 2 NEW HIGH + 1 MED are content defects unrelated to the external D-6 blocker. Clock HELD at 0_of_3.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 7 | 6 | 3 | 1 | 17 |
| p2 | 3 | 5 | 2 | 1 | 11 |
| p3 | 0 | 1 | 3 | 0 | 4 |
| p4 | 2 | 1 | 3 | 0 | 6 |

Regression: +2 net finding count; severity profile worsened (0H at p3 → 2H at p4).

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 6 |
| **Closures** | 0 (no fix burst between p3 and p4) |
| **Novelty score** | 0.5 (3 genuinely new; 3 carryover from p3) |
| **Median severity** | MED |
| **Trajectory** | 17→11→4→6 |
| **Verdict** | FINDINGS_REMAIN |
