---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T00:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-0.03-activation-skill-platform-detection.md
  - .factory/stories/S-2.06-activation-skill-integration.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
input-hash: "c75e21b"
traces_to: ".factory/specs/prd.md#FR-037"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-5-ss-06-re-anchor
pass: 1
previous_review: null
po_commit_reviewed: c75e21b
verdict: FINDINGS_REMAIN
finding_count: 11
convergence_step: 0_of_3
---

# Adversarial Review — Wave 5 SS-06 Re-anchor — Pass 1

## Finding ID Convention

Finding IDs use the format: `ADV-W5SS06-P1-<SEV>-NNN`.

## Part A — Fix Verification (pass >= 2 only)

_Pass 1 baseline — no previous findings to verify._

## Part B — New Findings (or all findings for pass 1)

### CRITICAL

#### ADV-W5SS06-P1-CRIT-001 — PRD §FR-037 BC title catalogue desynchronized from 4-of-5 BC-9.01 file H1s

**Severity:** CRITICAL (POLICY 7 violation; spec-fidelity contradiction)
**Files:** `.factory/specs/prd.md:716-720` vs `.factory/specs/behavioral-contracts/ss-09/BC-9.01.001-005.md:27`

**Evidence:** PRD §FR-037 asserts BC titles (e.g., BC-9.01.001 "Per-project activation required (DI-015)") that DO NOT match actual BC file H1s (BC-9.01.001 actual H1 = "bump-version.sh accepts semver prerelease format"). 4 of 5 BC-9.01.* are release-tooling BCs (bump-version, chore commits, release-bot atomic), not activation-gate BCs that FR-037 describes.

**Fix options:**
(a) Update PRD §FR-037 BC titles to match actual H1s verbatim (accept FR-037 covers release+activation tooling)
(b) Author NEW BC-9.01.NNN files for activation-gate scope and retire current ones

**Recommended (a):** match PRD to BC-file source-of-truth.

### ADV-W5SS06-P1-CRIT-002 — BC-9.01.002 mis-anchored to S-0.03 (chore-commit BC ≠ platform-detection story)

**Severity:** CRITICAL (POLICY 4)
**Files:** S-0.03:20 frontmatter; BC-9.01.002 H1 = "chore commit (operator-staged) modifies only CHANGELOG.md"

**Evidence:** BC-9.01.002 contracts release-tooling chore commits. S-0.03 scope is uname parsing + platform normalization. Anchor justification cited "Tier A release phase" — circular hand-wave.

**Fix:** Remove BC-9.01.002 from S-0.03 frontmatter, body BC table, ACs. Decrement Token Budget BC count 4→3.

### HIGH

#### ADV-W5SS06-P1-HIGH-001 — BC-9.01.001 + BC-9.01.003 mis-anchored to S-2.06 (release-tooling BCs ≠ activation-integration story)

**Severity:** HIGH (POLICY 4)
**Files:** S-2.06:20 frontmatter; BC-9.01.001 H1 (bump-version.sh); BC-9.01.003 H1 (release-bot atomic commit)

**Evidence:** S-2.06 scope: hooks.json copy + dispatcher verify. BC-9.01.001 = version-bump tooling. BC-9.01.003 = release CI bot. Neither exercised by activate skill. Justifications cited "release cycle" — anti-pattern.

**Fix:** Remove from S-2.06 frontmatter, body, ACs. Decrement Token Budget 11→9.

#### ADV-W5SS06-P1-HIGH-002 — CAP-007 declares SS-01 but no SS-01 BC anchored

**Severity:** HIGH (POLICY 4 + POLICY 5)
**Files:** capabilities.md:44; PRD §8:1093

**Evidence:** Wave 5 expanded CAP-007 to `SS-01, SS-06, SS-09`. Stories declare `subsystems: [SS-06, SS-09]` (no SS-01). No BC-1.* anchored. capabilities.md:46 process-gap comment justifies only SS-06 expansion, not SS-01.

**Fix:** Drop SS-01 from CAP-007 Subsystems (revert to `SS-06, SS-09`); same-burst PRD §8 propagation. Note: SS-01 was in original CAP-007 baseline; if dropping, document rationale.

#### ADV-W5SS06-P1-HIGH-003 — CAP-028 + DI-015 propagation gap

**Severity:** HIGH (POLICY 2 + POLICY 5)
**Files:** PRD:723; story frontmatters; all 5 BC-9.01.*.md L2 Domain Invariants cells

**Evidence:** PRD §FR-037 says "Enforces: DI-015, CAP-007, CAP-028". Stories list only CAP-007. All 5 BC-9.01.*.md Traceability `L2 Domain Invariants` = TBD; DI-015 currently orphaned per POLICY 2.

**Fix:** Add CAP-028 to story capabilities arrays OR drop from PRD §FR-037. Populate BC-9.01.NNN Traceability with DI-015 citation.

#### ADV-W5SS06-P1-HIGH-004 — VP-015 (DI-015 / SS-09) uncited despite anchor relevance

**Severity:** HIGH (POLICY 9 + POLICY 5)
**Files:** VP-INDEX:68; story frontmatters

**Evidence:** VP-015 "Per-Project Activation Required Before Dispatcher Can Run" (SS-09, manual, DI-015 anchor) exists in catalog. Both stories have `verification_properties: []`. PO baseline noted "no VP citations added".

**Fix:** Add VP-015 to S-2.06 verification_properties (S-2.06 exercises activation gate).

### MEDIUM

#### ADV-W5SS06-P1-MED-001 — wave field divergence (S-0.03=1, S-2.06=9)

**Severity:** MEDIUM (pending intent verification)
**Files:** S-0.03:25; S-2.06:25

**Evidence:** wave field semantics unclear. May reference delivery wave (pre-existing), not re-anchor wave (Wave 5).

**Fix:** Document convention; `(pending intent verification)`.

#### ADV-W5SS06-P1-MED-002 — BC-9.01.005 PRD title mismatch (anchor stretch already filed CRIT-001)

**Severity:** MEDIUM (POLICY 7)
**Files:** S-2.06:61; BC-9.01.005:27; PRD:720

**Evidence:** S-2.06 body uses BC H1 verbatim (correct); PRD diverges. Filed under CRIT-001.

**Fix:** Confirm during CRIT-001 resolution — no S-2.06 body change required.

#### ADV-W5SS06-P1-MED-003 — Pre-existing dependency asymmetry (S-2.04.blocks↛S-2.06)

**Severity:** MEDIUM (out-of-scope; task #111)

**Evidence:** Wave 5 didn't introduce new asymmetry; pre-existing gap stays.

**Fix:** Defer to task #111.

#### ADV-W5SS06-P1-MED-004 — CAP-007 narrative does not justify SS-01 inclusion

**Severity:** MEDIUM (POLICY 5)
**Files:** capabilities.md:42-46

**Evidence:** Narrative is activate-skill-centric (SS-06) and platform-variant-centric (SS-09). Process-gap comment justifies only SS-06 expansion.

**Fix:** Compounds HIGH-002. Either drop SS-01 OR extend narrative.

### LOW

#### ADV-W5SS06-P1-LOW-001 — BC-6.03.001/002 SKILL.md line ranges may be stale

**Severity:** LOW (pending intent verification)
**Files:** BC-6.03.001:109; BC-6.03.002:109

**Fix:** Verify line ranges resolve in actual SKILL.md. Out of Wave 5 scope.

#### ADV-W5SS06-P1-LOW-002 — All 10 SS-06 BC files have TBD Architecture Module anchors

**Severity:** LOW (pending intent verification)

**Fix:** May be deferred to Phase 1.6c. Document deferral policy explicitly.

#### ADV-W5SS06-P1-LOW-003 — Missing F-104 stretch-anchor disclosure for BC-9.01.001/003 in S-2.06

**Severity:** LOW (depends on HIGH-001 resolution)

**Fix:** Resolve HIGH-001 first; if BCs stay anchored, add disclosure section.

## Sweep Results — Per-Axis

| Policy | Status |
|--------|--------|
| POLICY 1 (append-only) | CLEAN |
| POLICY 4 (semantic anchoring) | VIOLATION (CRIT-002, HIGH-001, HIGH-002) |
| POLICY 5 (creators_justify_anchors) | VIOLATION (HIGH-002/003/004, MED-004) |
| POLICY 6 (subsystem-registry verbatim) | CLEAN |
| POLICY 7 (BC H1 source-of-truth) | VIOLATION (CRIT-001 PRD diverges) |
| POLICY 8 (frontmatter↔body↔ACs) | partial — Token Budget BC counts match; AC-trace style varies |
| POLICY 9 (VP-INDEX coherence) | partial — VP-015 missing (HIGH-004) |
| POLICY 2 (lift_invariants_to_bcs) | VIOLATION (HIGH-003 DI-015 orphan) |

### POLICY 7 BC H1 verbatim sweep (story bodies)

15/15 story-body↔BC-H1 matches CLEAN. PRD §FR-037 fails (4/5 drift) — see CRIT-001.

### Input-hash format consistency

PO modification of BC-9.01.001-005 (`[pending-recompute]` → 7-char hex) consistent with rest of catalog. Benign.

### CAP→PRD §8 propagation

capabilities.md CAP-007 ↔ PRD §8 line 1093 verbatim match. Same-burst propagation satisfied (but content itself flagged HIGH-002).

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 2 |
| HIGH | 4 |
| MEDIUM | 4 |
| LOW | 3 |

**Overall Assessment:** SUBSTANTIVE — Wave 5 SS-06 baseline has POLICY 4 + POLICY 7 violations blocking convergence.

## Convergence

**Convergence step: 0_of_3.** Pass-1 baseline. Verdict: FINDINGS_REMAIN.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings count** | 11 |
| **Duplicate count** | 0 (baseline) |
| **Novelty score** | 1.0 |
| **Median severity** | HIGH |
| **Severity distribution** | 2 CRIT, 4 HIGH, 4 MED, 3 LOW |
| **Trajectory** | pass-1 = 11 (within Wave-1 baseline 7-12 band) |
| **Verdict** | FINDINGS_REMAIN |

## Trajectory Baseline

| Pass | Total | CRIT | HIGH | MED | LOW |
|------|-------|------|------|-----|-----|
| 1 | 11 | 2 | 4 | 4 | 3 |

## Verdict

**FINDINGS_REMAIN.** 2 CRITICAL findings block convergence:
- CRIT-001 PRD §FR-037 ↔ BC-9.01 H1 desync (catastrophic spec contradiction)
- CRIT-002 BC-9.01.002 wrong-story anchor (clean POLICY 4 violation)

Plus 4 HIGH findings (anchor stretches, CAP-007 SS-01 unjustified, CAP-028+DI-015 propagation, VP-015 missing) and 4 MED + 3 LOW.
