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
pass: 3
previous_review: ADV-S5.03-P02.md
verdict: CLOCK_RESET
convergence_step: 0_of_3
findings_count:
  CRIT: 0
  HIGH: 0
  MED: 2
  OBS: 3
  total: 5
---

# ADV-S5.03-P03 — Pass-3 Adversarial Review for S-5.03 (WorktreeCreate/WorktreeRemove)

## Finding ID Convention

Pass-3 findings use severity-prefixed IDs: `MED-P03-NNN`, `OBS-P03-NNN`.

## Part A — Pass-2 Fix Verification (10 of 10 VERIFIED FIXED)

| Finding | Description | Status |
|---------|-------------|--------|
| CRIT-P02-001 | 4+4 revert — BC-4.07.001/.002 use 4+4 grouping consistently | VERIFIED FIXED |
| CRIT-P02-002 | once-key-absent EC-001 — both BCs say "once key ABSENT" | VERIFIED FIXED |
| CRIT-P02-003 | Sibling changelog narratives — all 5 sibling BCs enumerate applied/not-applicable | VERIFIED FIXED |
| HIGH-P02-001 | BC-INDEX line 252 — title matches BC-4.05.001 H1 | VERIFIED FIXED |
| HIGH-P02-002 | BC-INDEX line 259 — "once key ABSENT" present | VERIFIED FIXED |
| HIGH-P02-003 | Story BC table descriptions — 4+4 grouping restored | VERIFIED FIXED |
| HIGH-P02-004 | Input-hash regen — 5 fresh hashes (including VP-067 ea362a9) | VERIFIED FIXED |
| HIGH-P02-006 | Token Budget rows — total 6600 with hooks-registry.toml + hooks.json.template | VERIFIED FIXED |
| HIGH-P02-007 | Story BC table titles — match BC H1s verbatim | VERIFIED FIXED |
| OBS-P02-001 | ARCH-INDEX SS-04 count updated to 27 | VERIFIED FIXED |

## Part B — New Findings (5 total: 0 CRIT, 0 HIGH, 2 MED, 3 OBS)

### MEDIUM

#### MED-P03-001: VP-067 retains concrete InternalEvent::now() attribution — POLICY 9 propagation gap
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** VP-067 §1 (Property Statement, approx. line 60)
- **Description:** VP-067 §1 retained the concrete phrase "set by InternalEvent::now()" attributing the construction-time RESERVED_FIELDS bucket to a specific implementation detail. After the CRIT-P02-001 fix reverted HIGH-003 and restored the opaque 4+4 bucket framing in BC-4.07.001 v1.2, this implementation-level attribution in VP-067 is now inconsistent with the abstract framing in its source BC. POLICY 9 requires VP-067 to stay aligned with the BC it traces to (BC-4.07.001). The attribution is an emit_event.rs source-code detail that must not be surfaced at the verification-property layer.
- **Evidence:** BC-4.07.001 v1.2 (post-pass-2): "4 construction-time (opaque bucket; implementation details not surfaced at spec layer)". VP-067 §1 (pre-fix): "set by InternalEvent::now()" — names a specific struct not in HOST_ABI.md.
- **Proposed Fix:** Replace concrete "set by InternalEvent::now()" attribution with abstract construction-time framing consistent with BC-4.07.001 v1.2: "4 construction-time fields populated at event construction (implementation opaque at spec layer)."

#### MED-P03-002: BC-INDEX lines 257+258 carry enrichment not present in BC H1s — POLICY 7 violation
- **Severity:** MEDIUM
- **Category:** policy
- **Location:** .factory/specs/behavioral-contracts/BC-INDEX.md lines 257+258
- **Description:** The BC-INDEX title column entries for BC-4.07.001 and BC-4.07.002 contained "Option A zero-capability; 10-field wire payload (2+4+4)" enrichment language that does not appear in the BC H1s. POLICY 7 requires the BC-INDEX title column to match BC H1s verbatim. The HIGH-P02-007 fix in pass-2 synced the story BC table titles to H1s but missed the parallel BC-INDEX title column — a partial-fix gap. This is a direct sibling of HIGH-P02-007.
- **Evidence:** BC-4.07.001 H1: "WorktreeCreate hook plugin emits worktree.created event with {worktree_path, worktree_name} on WorktreeCreate event." BC-INDEX line 257 (pre-fix): enriched with "Option A zero-capability; 10-field wire payload (2+4+4)" commentary. POLICY 7: "BC-INDEX title column MUST match BC H1 verbatim."
- **Proposed Fix:** Strip enrichment from BC-INDEX lines 257+258. Set each title to the verbatim BC H1 text. No BC body changes required.

### Observations

#### OBS-P03-001: Story v2.2 Changelog row — cosmetic trailing cell artifact
- **Severity:** OBS
- **Category:** formatting
- **Location:** S-5.03 story Changelog table, v2.2 row (approx. line 241)
- **Description:** The v2.2 Changelog row contained a trailing "story-writer |" string that reads as a 4th column entry. The Changelog table header defines 3 columns (version, date, description). The trailing string is cosmetic and does not affect meaning, but it breaks table alignment and implies a 4th column that does not exist.
- **Proposed Fix:** Remove trailing "story-writer |" from the v2.2 Changelog cell. Cosmetic only; no content change.

#### OBS-P03-002: BC-INDEX total_bcs frontmatter + prose out of sync with body Total row
- **Severity:** OBS
- **Category:** bookkeeping
- **Location:** .factory/specs/behavioral-contracts/BC-INDEX.md (frontmatter total_bcs, prose line, body Total row)
- **Description:** BC-INDEX frontmatter `total_bcs: 1905` and prose "1,905" were not synchronized to the body Total row which correctly showed 1909. This is a pre-existing arithmetic drift from a prior cycle's count update that was not propagated to the frontmatter and prose fields. Out of S-5.03 scope but surfaced during pass-3 review; folded into the fix burst for correctness.
- **Proposed Fix:** Update BC-INDEX frontmatter `total_bcs: 1905 → 1909` and prose "1,905 → 1,909". Body Total row is already correct at 1909.

#### OBS-P03-003: Sibling-sweep changelog discipline templatable for recurring patterns
- **Severity:** OBS
- **Category:** process
- **Description:** The MED-P03-002 finding (BC-INDEX title column enrichment not stripped in parallel with story BC table fix) follows the same class as CRIT-P02-003 (partial-fix gap from sibling sweep). If this finding class recurs across 3+ cycles, the sibling-sweep checklist should be templated with explicit "BC-INDEX title column" as a mandatory verification row whenever story BC table titles are touched.
- **Proposed Fix:** No fix required this burst. Learning recorded; already captured in sidecar-learning.md from pass-2 (OBS-P02-002). If recurrence confirmed at pass-4 or later cycle, promote to formal checklist template item.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 2 |
| LOW | 0 |
| OBS | 3 |

**Overall Assessment:** block (2 MED findings present; CLOCK_RESET threshold requires zero MED or above for convergence step increment)
**Convergence:** CLOCK_RESET — 2 MED findings block convergence step increment. Counter resets to 0 of 3.
**Readiness:** requires revision

## Fix Burst Outcome

PO fix burst: 3 files modified.
- VP-067 v1.1 → v1.2: abstract construction-time framing (MED-P03-001 closed)
- BC-INDEX lines 257+258 enrichment stripped; total_bcs frontmatter + prose 1905 → 1909 (MED-P03-002 + OBS-P03-002 closed)
- S-5.03 story line 241 cosmetic Changelog trailing cell removed (OBS-P03-001 closed)

NO BC content changes. NO story content changes (cosmetic only). OBS-P03-003 is a process learning with no fix artifact; already covered by sidecar-learning.md.
Convergence step: 0 of 3 (reset; 2 MED were present at pass-3). Pass-4 risk: LOW (story-writer + PO concur; no architectural issues remain).

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 5 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | MED |
| **Severity distribution** | 0 CRIT, 0 HIGH, 2 MED, 0 LOW, 3 OBS |
| **Trajectory** | 14 → 15 → 5 (CLOCK_RESET; 2 MED present; all CRIT+HIGH resolved) |
| **Verdict** | CLOCK_RESET |
