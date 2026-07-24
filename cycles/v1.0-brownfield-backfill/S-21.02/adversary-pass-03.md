---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-24T00:00:00Z
phase: 5
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/STORY-S-21.02-post-rebase-diff-integrity-gate.md"
pass: 3
previous_review: "adversary-pass-02.md"
story: S-21.02
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: f5ea156a
reviewed_branch: feature/S-21.02-post-rebase-diff-integrity-gate
base_commit: 7bb0e797
date: 2026-07-24
---

# S-21.02 LOCAL Adversary Pass-3 — NOT-CLEAN

**Date:** 2026-07-24
**Story:** S-21.02 — Post-rebase diff-integrity gate
**Pass:** 3 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H1 / M1 / L1 / NITPICK0 / OBS2
**Total findings:** 3 findings + 2 observations
**Reviewed diff:** HEAD f5ea156a on feature/S-21.02-post-rebase-diff-integrity-gate vs base 7bb0e797
**Fix-burst commits:** cae0e7ee (implementer — gate-host doc error-variant line) + 8abf24e2 (test-writer — parity assertion + three-dot alignment + force_stat_fail fidelity + stale-comment sweep)
**Next reviewed HEAD:** 8abf24e2
**Finding-count trajectory:** P1 6 / P2 4 / P3 3 (descending — monotone convergence, zero paper-fixes across all passes)

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2102-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2102`: Story identifier (S-21.02 compact form)
- `P<PASS>`: Pass number (e.g., `P1`, `P2`, `P3`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Observations use `OBS-<SEQ>` (no severity component; informational only).

Examples: `F-S2102-P3-001` (HIGH finding, pass 3, first finding), `OBS-1` (first observation).

---

## Part A — Pass-2 Finding Closure Review

All 4 pass-2 findings are CONFIRMED-FIXED. No paper-fixes detected. Every closure is load-bearing.

| Finding | Pass-2 Severity | Status | Closure Evidence |
|---------|----------------|--------|------------------|
| F-S2102-P2-003 | HIGH | CONFIRMED-FIXED | 7a6111f1: BC-5.44.001 v1.3→v1.4 H1 title + Description actor corrected from "pr-manager and orchestrator" to "devops-engineer (inter-wave) and implementer (per-story)"; c73152fb: story S-21.02 v1.4→v1.5 propagation cite sweep. Full POLICY 14/17 5-leg parity verification confirmed across BC v1.4 / story v1.5 / BC-INDEX v4.21 / STORY-INDEX v4.237 — all version rows and changelog entries consistent; no residual "pr-manager" actor in any artifact. |
| F-S2102-P2-001 | MEDIUM | CONFIRMED-FIXED | f5ea156a: real-rebase fixtures added (git init + cherry-pick + conflict resolved via `git rebase -X theirs`); PRE_REBASE_TIP captured before rebase; range-diff two-range form invoked; `!`-commit pre-assertion added; gate invoked against rebased fixture repo with exit-code and stdout verified. `-X theirs` fixture caveat (classical ORT adjacent-hunk silent-drop not exercised) dispositioned ACCEPTED-WITH-RECORD — caveat documented in fixture comments; not a paper-fix. |
| F-S2102-P2-002 | MEDIUM | CONFIRMED-FIXED | f5ea156a: undocumented condition (c) removed; merge-base failure now routes through PC4 escalation handler (exit 2 with full diagnostic); T-005 sub-case C added for merge-base failure path; BC-5.44.001 §Algorithm two-condition model now matches implementation exactly. |
| F-S2102-P2-004 | LOW | CONFIRMED-FIXED | f5ea156a: step-f doc-parity markers added; AC-002 JSON report assertion added to test matrix; bats now asserts existence of log file and presence of `"gate_result"` key in JSON output. |

**OBS-1 (merge-base failure fail-open, pass-2):** CONFIRMED-FIXED — absorbed into F-S2102-P2-002 closure above.
**OBS-2 (intentionality keyword-grep proxy, pass-2):** ACCEPTED-WITH-RECORD — `!`-grep is correct and deterministic for the documented threat class; full range-diff structured parsing is out of scope per design intent; simulation-proxy comment present in harness and fixture.

---

## Part B — New Findings

### HIGH

#### F-S2102-P3-001 — UnverifiedNetNegativeDelta error-variant token absent from gate-host doc; AC-001 grep gate unsatisfiable

- **Severity:** HIGH
- **Category:** spec-fidelity / coverage-gap
- **Location:** gate-host documentation (doc error-variant section); AC-001 grep gate; bats test matrix
- **Description:** The gate implementation emitted `UnverifiedNetNegativeDelta` as a structured error-variant token in its STOP payload (BC-5.44.001 PC2 contract). The gate-host documentation's error-variant table did not include this token, making the AC-001 grep gate (`grep -E 'UnverifiedNetNegativeDelta'`) unsatisfiable against the documented artifact. The omission was masked by a doc-parity gap: the harness and the doc were generated in separate fix bursts without a cross-check, and the AC-001 gate column entry did not specify which artifact it was asserting against. A CI-green false-positive was possible: the test would pass only if the bats harness directly invoked the gate and asserted stdout, not if the gate-host doc was the tested artifact.
- **Evidence:** Gate-host doc error-variant table: token list did not include `UnverifiedNetNegativeDelta`. AC-001 gate column: `bats: grep UnverifiedNetNegativeDelta` — no artifact path specified. Harness stdout: `UnverifiedNetNegativeDelta` present in gate STOP output. Doc-parity gap: the two were never cross-checked.
- **Routing:** implementer (add error-variant line to gate-host doc) + test-writer (add parity assertion confirming doc token matches harness stdout token)
- **FIXED at:** cae0e7ee (implementer — gate-host doc error-variant table updated with `UnverifiedNetNegativeDelta` row and canonical description); 8abf24e2 (test-writer — AC-001 parity assertion added: harness stdout `UnverifiedNetNegativeDelta` cross-checked against doc error-variant table; artifact path now explicit in gate column)
- **Status:** FIXED

---

### MEDIUM

#### F-S2102-P3-002 — Harness range-diff two-range form mismatches three-dot form spec'd uniformly in BC/story/doc

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** gate harness (range-diff invocation form); BC-5.44.001 §Algorithm; story S-21.02 AC gate column; gate-host doc §Algorithm
- **Description:** BC-5.44.001 §Algorithm, the story S-21.02 AC table, and the gate-host documentation all uniformly specified the range-diff invocation using three-dot form (`git range-diff A...B C...D`). The harness implementation used two-range form (`git range-diff A B C D`), which differs in how symmetric difference is computed and can produce different output for asymmetric branch histories. The spec-wins adjudication rule (CLAUDE.md §Architectural Authority rule 12) requires the harness to be brought into alignment with the spec, not the reverse. The T-001 pre-check did not assert the invocation form; the `!`-detection was not re-verified under three-dot form.
- **Evidence:** Gate harness: `git range-diff $PRE_REBASE_TIP $MERGE_BASE $POST_REBASE_TIP $ORIGIN_DEVELOP` (two-range). BC-5.44.001 §Algorithm: `git range-diff $PRE_REBASE_TIP...$MERGE_BASE $POST_REBASE_TIP...$ORIGIN_DEVELOP` (three-dot). Story S-21.02: three-dot form in AC-001 description. Gate-host doc: three-dot form in §Algorithm.
- **Routing:** spec-wins adjudication confirmed by orchestrator — harness must align to spec
- **FIXED at:** 8abf24e2 (test-writer — harness range-diff invocation updated to three-dot form; T-001 pre-check assertion added for invocation-form compliance; `!`-detection re-verified under three-dot form using existing `-X theirs` fixture; fixture still triggers `!` under three-dot as expected)
- **Status:** FIXED

---

### LOW

#### F-S2102-P3-003 — Stale fixture comments describing superseded -X ours design

- **Severity:** LOW
- **Category:** documentation-drift
- **Location:** bats test fixture setup comments
- **Description:** The fixture setup comments in the bats test file contained references to `-X ours` as the conflict resolution strategy ("originally used -X ours to exercise feature-wins path"). The actual implementation uses `-X theirs` (the current committed design). These stale comments described a superseded design path and would mislead a future reader about why `-X theirs` was chosen and what the fixture exercises.
- **Evidence:** Fixture comment: `# originally used -X ours to exercise feature-wins path; switched to -X theirs for conflict resolution realism`. The phrase "originally used -X ours" is a narrative artifact of the pass-1→pass-2 evolution; not accurate as a standing fixture description.
- **Routing:** test-writer (fixture comment cleanup)
- **FIXED at:** 8abf24e2 (test-writer — stale `-X ours` reference removed; fixture comment updated to describe the current `-X theirs` design rationale and what the fixture exercises under that strategy)
- **Status:** FIXED

---

### Observations (non-finding, informational)

#### OBS-1 — Story ADR-031 version-cite provenance inconsistency

The story S-21.02 cites ADR-031 at version v1.3 in the §References block while the BC-5.44.001 cite in the same story uses v1.4. On inspection, this is deliberate: the story was authored using ADR-031 v1.3 as its source-of-truth; the v1.4 bump post-dates the story's original specification. The E-21 spec convergence established a consistent-form O-4 policy for provenance cites: version numbers in story §References blocks are pinned to the version that was authoritative at story-authorship time, not updated retroactively. ADR-031 v1.4 changelog documents the §Consequences numbering freeze to preserve provenance cites; the anchors cited in the story remain stable across v1.3→v1.4. No change required.

**Disposition:** ACCEPTED-WITH-RECORD — deliberate O-4 consistent-form from E-21 spec convergence; anchors stable; no regression.

#### OBS-2 — force_stat_fail partially inert; no classical ORT adjacent-hunk silent-drop fixture

As noted in pass-2, the range-diff `!`-commit detection uses a `!`-grep proxy rather than full structured range-diff parsing. Additionally, the `force_stat_fail` test flag (T-005) was partially inert: the flag was checked but the full stat path was not fully disabled under the flag, leaving T-005 exercising a mixed detection path rather than isolating the range-diff-primary-alone scenario.

`force_stat_fail` fidelity has been FIXED at 8abf24e2: the flag now disables the full stat path; T-005 sub-case D added for STOP via range-diff primary alone (confirming the gate correctly blocks on `!`-detection even when --stat is unavailable). The ORT adjacent-hunk silent-drop fixture limitation remains: constructing a deterministic classical ORT adjacent-hunk silent-drop is hard and the BC itself notes empirical rarity of this class. This is documented in fixture comments.

**Disposition (force_stat_fail fidelity):** FIXED at 8abf24e2.
**Disposition (ORT-silent-drop fixture limitation):** ACCEPTED-WITH-RECORD — BC notes empirical rarity; fixture comments document the limitation; not a coverage gap that changes the gate's threat model.

---

### Clean Axes Verified

- **All 4 pass-2 findings closed load-bearing:** confirmed (see Part A table above).
- **POLICY 14/17 5-leg parity on F-P2-003 closure:** BC v1.4 / story v1.5 / BC-INDEX v4.21 / STORY-INDEX v4.237 / gate-host doc all consistent on actor; no residual "pr-manager" in any artifact.
- **Scope discipline:** Fix burst touched only gate-host doc, bats test file. No lateral expansion.
- **POLICY 21 compliance:** No new `.sh` files introduced.
- **TD-VSDD-091:** Spec narrative cites behavioral anchors (BC-5.44.001 clause identifiers) not line numbers.
- **Three-dot form alignment propagated:** BC-5.44.001 §Algorithm / story AC column / gate-host doc / harness all now use consistent three-dot form.
- **Paper-fix check (passes 1–3):** Zero paper-fixes detected across all three passes. Every closure is load-bearing (executable assertions, structural gate changes, or spec corrections with verifiable propagation).

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 1 |
| MEDIUM   | 1 |
| LOW      | 1 |
| NITPICK  | 0 |
| OBS      | 2 |

**Overall Assessment:** block
**Convergence:** findings remain (all fixed in-burst) — streak 0/3, iterate to pass-4
**Readiness:** all pass-3 findings FIXED in-burst (cae0e7ee, 8abf24e2); pass-4 adversary cascade required

**All pass-3 findings were fixed during the fix burst. Pass-4 adversary review required for streak progression.**

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 3 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (3 / (3 + 0)) |
| **Median severity** | 2.0 (MEDIUM) |
| **Trajectory** | 6 → 4 → 3 |
| **Verdict** | FINDINGS_REMAIN |
