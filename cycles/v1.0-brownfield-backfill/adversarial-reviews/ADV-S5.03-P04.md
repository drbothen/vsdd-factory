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
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.002.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.003.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md
  - .factory/specs/verification-properties/VP-067.md
  - .factory/specs/verification-properties/VP-INDEX.md
input-hash: "b4a39f9"
traces_to: ".factory/specs/prd.md"
pass: 4
previous_review: ADV-S5.03-P03.md
verdict: CLOCK_RESET
convergence_step: 0_of_3
findings_count:
  CRIT: 0
  HIGH: 0
  MED: 5
  LOW: 1
  OBS: 2
  total: 8
---

# ADV-S5.03-P04 — Pass-4 Adversarial Review for S-5.03 (WorktreeCreate/WorktreeRemove)

## Finding ID Convention

Pass-4 findings use severity-prefixed IDs: `MED-P04-NNN`, `LOW-P04-NNN`, `OBS-P04-NNN`.

## Part A — Pass-3 Fix Verification (5 of 5 VERIFIED FIXED)

| Finding | Description | Status |
|---------|-------------|--------|
| MED-P03-001 | VP-067 abstract construction-time framing | VERIFIED FIXED |
| MED-P03-002 | BC-INDEX lines 257+258 enrichment stripped | VERIFIED FIXED |
| OBS-P03-001 | Story line 241 cosmetic trailing cell removed | VERIFIED FIXED |
| OBS-P03-002 | BC-INDEX total_bcs 1909 sync (frontmatter + prose) | VERIFIED FIXED |
| VP-067 v1.2 | frontmatter ↔ Changelog ↔ b4a39f9 hash coherent | VERIFIED FIXED |

## Part B — New Findings (8 total: 0 CRIT, 0 HIGH, 5 MED, 1 LOW, 2 OBS)

### MEDIUM

#### MED-P04-001: VP-067 frontmatter modified[] missing v1.2-adv-s5.03-p03 entry — POLICY 1
- **Severity:** MEDIUM
- **Category:** policy
- **Location:** VP-067 frontmatter modified[] field
- **Description:** VP-067 was bumped to v1.2 in the pass-3 fix burst (MED-P03-001 closure) but the modified[] array was not updated to append the v1.2-adv-s5.03-p03 tag. POLICY 1 requires every version bump to be accompanied by a modified[] entry identifying the burst that produced it. The frontmatter-only fix (no content change beyond the abstract framing) is still a versioned edit requiring tracking.
- **Evidence:** VP-067 frontmatter version: "v1.2". modified[]: ends at v1.1-adv-s5.03-p01 with no v1.2 entry. POLICY 1: "modified[] MUST include an entry for every version present in the Changelog."
- **Proposed Fix:** Append `v1.2-adv-s5.03-p03` to VP-067 modified[] array. No version bump required (frontmatter-only fix).

#### MED-P04-002: BC-INDEX line 260 BC-4.07.004 missing comma vs H1 — POLICY 7
- **Severity:** MEDIUM
- **Category:** policy
- **Location:** .factory/specs/behavioral-contracts/BC-INDEX.md line 260
- **Description:** BC-INDEX line 260 title for BC-4.07.004 omits the comma present in the BC H1: "WorktreeCreate hook plugin registers plugin binary in hook registry for both WorktreeCreate and WorktreeRemove events as a single crate[,] two entries". The missing comma makes the title diverge from the verbatim H1. POLICY 7 requires the BC-INDEX title column to match BC H1s verbatim. This is a residual gap from the pass-2 HIGH-P02-007 sibling sweep — the story BC table was fixed but the BC-INDEX column was not fully verified.
- **Evidence:** BC-4.07.004 H1: "…as a single crate, two entries". BC-INDEX line 260: "…as a single crate two entries" (comma absent). POLICY 7: "BC-INDEX title column MUST match BC H1 verbatim."
- **Proposed Fix:** Restore the comma in BC-INDEX line 260.

#### MED-P04-003: BC-INDEX line 256 BC-4.05.005 carries H1-enrichment — POLICY 7 sibling sweep gap
- **Severity:** MEDIUM
- **Category:** policy
- **Location:** .factory/specs/behavioral-contracts/BC-INDEX.md line 256
- **Description:** BC-INDEX line 256 (BC-4.05.005) retained enrichment language not present in the BC H1 — a parallel gap to MED-P03-002 which stripped enrichment from lines 257+258 (BC-4.07.001 and BC-4.07.002) but did not sweep the adjacent BC-4.05.005 entry. POLICY 7 requires verbatim H1 matching throughout the title column. This is a partial-fix gap: the pass-3 sibling sweep stopped two lines short.
- **Evidence:** BC-4.05.005 H1 (verbatim required). BC-INDEX line 256: contains enrichment not in H1. POLICY 7: "BC-INDEX title column MUST match BC H1 verbatim."
- **Proposed Fix:** Strip enrichment from BC-INDEX line 256. Set title to verbatim BC-4.05.005 H1 text.

#### MED-P04-004: ARCH-INDEX line 85 stale BC total "1,905" vs authoritative 1,909 — POLICY 6
- **Severity:** MEDIUM
- **Category:** bookkeeping
- **Location:** .factory/specs/architecture/ARCH-INDEX.md line 85
- **Description:** ARCH-INDEX line 85 retained "1,905" as the BC total count. The BC-INDEX authoritative count was updated to 1,909 in the pass-3 OBS-P03-002 fix burst (BC-INDEX frontmatter + prose corrected). POLICY 6 requires count propagation to all index files. The defensive sweep run in the pass-3 state-manager commit did not cover ARCH-INDEX — a propagation gap.
- **Evidence:** BC-INDEX total_bcs: 1909 (frontmatter, post-pass-3 fix). ARCH-INDEX line 85: "1,905". POLICY 6: "All index files must carry the same authoritative count."
- **Proposed Fix:** Update ARCH-INDEX line 85: 1,905 → 1,909.

#### MED-P04-005: Story input-hash 0fe87b6 stale after VP-067 v1.1→v1.2 in pass-3
- **Severity:** MEDIUM
- **Category:** bookkeeping
- **Location:** S-5.03 story frontmatter input-hash field
- **Description:** The story input-hash was last set to 0fe87b6 in the pass-2 fix burst (HIGH-P02-004 closure). VP-067 was bumped v1.1→v1.2 during the pass-3 fix burst (MED-P03-001 closure), which is a primary input to S-5.03. The input-hash was not regenerated after that VP-067 change, leaving the story hash stale by one version.
- **Evidence:** Story frontmatter input-hash: "0fe87b6". VP-067 version: "v1.2" (bumped in pass-3 fix burst after hash was set). Input-hash must reflect the state of all inputs at the time of last story revision.
- **Proposed Fix:** Regenerate input-hash to reflect VP-067 v1.2 state. Story-writer scope.

### LOW (Pending Orchestrator Intent — adjudicated PROPAGATE → MED-P04-006)

#### LOW-P04-001 → MED-P04-006: Concrete InternalEvent::now() attribution persists in 5 sibling files
- **Severity:** LOW (adjudicated PROPAGATE by orchestrator → promoted MED-P04-006)
- **Category:** spec-fidelity
- **Location:** VP-065 §1 (Property Statement), VP-066 §1 (Property Statement), BC-4.04.001 Postconditions §2, BC-4.05.001 lines 57+63 (Postconditions §2 + RESERVED_FIELDS note)
- **Description:** MED-P03-001 established that concrete InternalEvent::now() attribution is incompatible with the abstract construction-time framing adopted by VP-067 v1.2 and BC-4.07.001 v1.2. The same concrete attribution ("set by InternalEvent::now()") was present in 5 sibling files covering S-5.01 (SessionStart) and S-5.02 (SessionEnd) behaviors. At pass-4, these sibling files were not swept as part of the MED-P03-001 fix scope. Architecturally, the abstract framing is superior; concrete impl attribution at the spec layer is a policy violation across all affected files. Orchestrator adjudicated: PROPAGATE — sibling sweep applies to all 5 locations.
- **Evidence:** VP-065 §1: "set by InternalEvent::now()". VP-066 §1: same. BC-4.04.001 Postconditions §2: same. BC-4.05.001 lines 57+63: same (two occurrences). VP-067 v1.2 (reference): abstract framing "populated at event construction (implementation opaque at spec layer)".
- **Proposed Fix:** Replace concrete attribution with abstract framing in all 5 locations, matching VP-067 v1.2 language. Version bump each affected file (VP-065 v1.0→v1.1, VP-066 v1.0→v1.1, BC-4.04.001 v1.1→v1.2, BC-4.05.001 v1.1→v1.2). PO scope.

### Observations

#### OBS-P04-001: Story v2.2 Changelog no row for pass-3 closure
- **Severity:** OBS
- **Category:** bookkeeping
- **Location:** S-5.03 story Changelog table
- **Description:** The v2.2 Changelog row (pass-2 fix burst) has no sibling v2.x entry for the pass-3 fix burst. The pass-3 fix burst touched VP-067 (PO scope), BC-INDEX (PO scope), and the story cosmetically (OBS-P03-001). The story version was not bumped in pass-3 (cosmetic-only change) — this is acceptable per convention. Bundling the pass-3 closure narrative into the v2.3 Changelog row (with pass-4 fixes) is the correct approach.
- **Proposed Fix:** None required this burst. Bundle pass-3 closure into v2.3 Changelog row alongside pass-4 fixes.

#### OBS-P04-002: [process-gap] modified[] frontmatter discipline — recommend hook gate
- **Severity:** OBS
- **Category:** process
- **Description:** MED-P04-001 (VP-067 modified[] missing v1.2 entry) represents a recurring process gap: version bumps occur during PO fix bursts without a corresponding modified[] update. This is the second occurrence of this gap class (OBS-P02-002 covered the sibling-sweep changelog discipline; the modified[] discipline is a related but distinct gap). A pre-commit hook that validates modified[] coherence against the version field and Changelog table would close this class permanently.
- **Proposed Fix:** No fix required this burst. Learning recorded in sidecar-learning.md (OBS-P04-002 append). If hook gate is implemented, target the validate-template-compliance.sh pipeline.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 5 |
| LOW | 1 (adjudicated → MED-P04-006; sibling sweep applied) |
| OBS | 2 |

**Overall Assessment:** block (5 MED findings present; CLOCK_RESET threshold requires zero MED or above for convergence step increment)
**Convergence:** CLOCK_RESET — 5 MED findings (plus 1 LOW adjudicated MED-P04-006) block convergence step increment. Counter resets to 0 of 3.
**Readiness:** requires revision

## Fix Burst Outcome

PO fix burst: 9 files modified.
- VP-067 modified[] append v1.2-adv-s5.03-p03 (no version bump — MED-P04-001 closed)
- BC-INDEX line 260 BC-4.07.004 comma restored (MED-P04-002 closed)
- BC-INDEX line 256 BC-4.05.005 enrichment stripped (MED-P04-003 closed)
- ARCH-INDEX line 85: 1,905 → 1,909 BC total (MED-P04-004 closed)
- Sibling sweep abstract framing (MED-P04-006 — adjudicated from LOW-P04-001): VP-065 v1.0→v1.1, VP-066 v1.0→v1.1, BC-4.04.001 v1.1→v1.2, BC-4.05.001 v1.1→v1.2 (4 files; abstract construction-time framing propagated from VP-067 v1.2)
- sidecar-learning.md OBS-P04-002 process-gap append

Story-writer burst: 1 file modified.
- S-5.03 v2.2 → v2.3: input-hash regen 0fe87b6 → d7a5acd (MED-P04-005 closed); v2.3 Changelog row consolidating pass-3 closure + pass-4 fix burst (OBS-P04-001 disposition)

Convergence step: 0 of 3 (reset; 5+1 MED present this pass). Pass-5 expectation: NITPICK_ONLY = 1_of_3 if all MEDs landed cleanly and no new sibling-sweep gaps surface.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 8 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | MED |
| **Severity distribution** | 0 CRIT, 0 HIGH, 5 MED, 1 LOW, 2 OBS |
| **Trajectory** | 14 → 15 → 5 → 8 (CLOCK_RESET; 5 MED present; all CRIT+HIGH resolved) |
| **Verdict** | CLOCK_RESET |
