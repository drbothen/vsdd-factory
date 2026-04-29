---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-28T00:00:00Z
phase: 5
inputs:
  - .factory/stories/S-5.03-worktree-hooks.md
  - .factory/specs/verification-properties/VP-065.md
  - .factory/specs/verification-properties/VP-066.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.04.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.05.001.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/stories/S-5.02-session-end-hook.md
input-hash: "d7a5acd"
traces_to: ".factory/specs/prd.md"
pass: 5
previous_review: ADV-S5.03-P04.md
verdict: CLOCK_RESET
convergence_step: 0_of_3
findings_count:
  CRIT: 0
  HIGH: 0
  MED: 2
  OBS: 2
  total: 4
---

# ADV-S5.03-P05 — Pass-5 Adversarial Review for S-5.03 (WorktreeCreate/WorktreeRemove)

## Finding ID Convention

Pass-5 findings use severity-prefixed IDs: `MED-P05-NNN`, `OBS-P05-NNN`.

## Part A — Pass-4 Fix Verification (7 of 7 VERIFIED FIXED)

| Finding | Description | Status |
|---------|-------------|--------|
| MED-P04-001 | VP-067 modified[] append v1.2-adv-s5.03-p03 | VERIFIED FIXED |
| MED-P04-002 | BC-INDEX line 260 BC-4.07.004 comma restore | VERIFIED FIXED |
| MED-P04-003 | BC-INDEX line 256 BC-4.05.005 enrichment strip | VERIFIED FIXED |
| MED-P04-004 | ARCH-INDEX BC total 1,905 → 1,909 | VERIFIED FIXED |
| MED-P04-005 | S-5.03 input-hash regen 0fe87b6 → d7a5acd; v2.3 | VERIFIED FIXED |
| MED-P04-006 | VP-065 + VP-066 + BC-4.04.001 + BC-4.05.001 abstract framing sibling sweep | PARTIALLY VERIFIED (VP-065 line 61 yes; line 63 no — see MED-P05-001) |
| Version coherence | 4 sibling-swept files frontmatter ↔ Changelog ↔ hash coherent | VERIFIED FIXED |

## Part B — New Findings (2 MED + 2 OBS; 0 CRIT, 0 HIGH)

### MEDIUM

#### MED-P05-001: VP-065 line 63 Wire format note retains concrete `InternalEvent::now()` — partial-fix slip from MED-P04-006
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** VP-065 §1 Wire format note (line 63)
- **Description:** MED-P04-006 applied the abstract construction-time framing sweep to VP-065 §1 (Property Statement). Line 61 was updated to abstract framing ("set by the dispatcher … implementation provenance is opaque from the spec layer"). However, line 63 — a Wire format note within the same Property Statement §1, two paragraphs down — still carries the concrete `InternalEvent::now()` attribution. A single VP section now contains contradictory framing: abstract at line 61 and concrete at line 63. This is a partial-fix slip: the sibling-sweep addressed the primary paragraph but did not grep the same file for residual occurrences of the replaced term.
- **Evidence:** VP-065 line 61: abstract framing (fixed). VP-065 line 63: "set by `InternalEvent::now()`" (concrete; unfixed). Same Property Statement §1, two paragraphs apart.
- **Proposed Fix:** Replace concrete `InternalEvent::now()` attribution on line 63 with abstract framing matching line 61: "set by the dispatcher (implementation provenance opaque from spec layer) and may arrive as numbers on the wire." VP-065 v1.1 → v1.2. PO scope.

#### MED-P05-002 → adjudicated REJECTED: S-5.02 line 94 AC2 concrete `InternalEvent::now()` citation
- **Severity:** MEDIUM (adjudicated REJECTED by orchestrator)
- **Category:** spec-fidelity
- **Location:** S-5.02 line 94 AC2
- **Description:** S-5.02 AC2 cited `InternalEvent::now()` as a concrete construction-time attribution consistent with the pre-MED-P03-001 framing. Adversary read frontmatter status: ready (stale), treating S-5.02 as an active draft subject to abstract-framing sweep. Orchestrator adjudication: S-5.02 is effectively merged (PR #36 / D-138 / STORY-INDEX). Merged-stories-untouched convention applies — AC bodies must not be changed post-merge. The stale `status: ready` frontmatter field was the root cause of the adversary misclassification.
- **Evidence:** S-5.02 frontmatter status: ready (stale at time of pass-5 review; correct value is merged per STORY-INDEX D-138 / PR #36 / develop edef7da). Orchestrator adjudication on file: close via frontmatter housekeeping only; body untouched.
- **Proposed Fix:** S-5.02 frontmatter status: ready → merged; v2.7 → v2.8 housekeeping Changelog row. Body (AC2 text) UNCHANGED per merged-stories-untouched convention. Closes MED-P05-002 without spec change.

### Observations

#### OBS-P05-001: Sibling-sweep scope checklist missing active-anchor-stories axis
- **Severity:** OBS
- **Category:** process
- **Description:** The sibling-sweep scope checklist used during MED-P04-006 covered BC and VP files but did not include active anchor stories (S-5.01, S-5.02, S-5.03) as a sweep axis. Active stories may carry concrete attribution in ACs and implementation notes. Including anchor stories in the sweep scope (with merged-untouched gate) would have surfaced S-5.02 AC2 during the pass-4 burst, preventing the MED-P05-002 false positive.
- **Proposed Fix:** No fix required this burst. Learning recorded in sidecar-learning.md (OBS-P05-001 append). Sibling-sweep scope checklist to add active-anchor-stories axis with merged-untouched gate.

#### OBS-P05-002: Intra-document consistency check — grep same file for residual occurrences
- **Severity:** OBS
- **Category:** process
- **Description:** MED-P05-001 (VP-065 line 63 partial-fix slip) arose because the pass-4 fix applied abstract framing at the first occurrence (line 61) but did not grep the same document for additional occurrences of the concrete term being replaced. A two-step discipline — (1) replace the primary paragraph, (2) grep the same file for any remaining instances of the replaced term — would have caught line 63 in the same burst.
- **Proposed Fix:** No fix required this burst. Learning recorded in sidecar-learning.md (OBS-P05-002 append). Recommend adding intra-document residual-occurrence grep to PO fix burst checklist.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 2 (MED-P05-001 open; MED-P05-002 adjudicated REJECTED) |
| LOW | 0 |
| OBS | 2 |

**Overall Assessment:** block (MED-P05-001 present; CLOCK_RESET threshold requires zero MED or above for convergence step increment)
**Convergence:** CLOCK_RESET — MED-P05-001 (VP-065 line 63 partial-fix slip) blocks convergence step increment. Counter resets to 0 of 3.
**Readiness:** requires revision

## Fix Burst Outcome

PO scope (3 files):
- VP-065 v1.1 → v1.2: line 63 abstract framing (MED-P05-001 closed); Changelog row; modified[] append v1.2-adv-s5.03-p05
- S-5.02 frontmatter status: ready → merged; v2.7 → v2.8 housekeeping Changelog row (MED-P05-002 adjudicated REJECTED; body untouched per convention)
- sidecar-learning.md OBS-P05-001 + OBS-P05-002 process-gap append

Convergence step: 0_of_3 (reset; 2 MED present this pass, 1 open + 1 adjudicated-rejected).
Pass-6 expectation: NITPICK_ONLY = 1_of_3 (housekeeping fixes only; no contract changes).

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 4 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | MED |
| **Severity distribution** | 0 CRIT, 0 HIGH, 2 MED (1 open + 1 adjudicated REJECTED), 2 OBS |
| **Trajectory** | 14 → 15 → 5 → 8 → 4 (CLOCK_RESET; 1 open MED; all CRIT+HIGH resolved) |
| **Verdict** | CLOCK_RESET |
