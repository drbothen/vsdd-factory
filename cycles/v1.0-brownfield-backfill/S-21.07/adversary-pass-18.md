---
pass: 18
verdict: NOT-CLEAN
reviewed_head: 96b4be19158ae27131ae330f684af414821e7c5f
factory_artifacts_head: 445f33b855ae638b9720425a683df577e3e112a0
novelty: MEDIUM
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-17.md"
---

## Summary

VERDICT: NOT-CLEAN. Counts: BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 0 / NIT 0 = 1 finding. Streak RESETS 1/3 → 0/3 (BC-5.39.001; pass-17's fresh CLEAN broken; 3 fresh consecutive CLEAN required from pass-19). Trajectory pass-11=1,12=1,13=2,14=0,15=1,16=1,17=0,18=1 (tail →1→1→0→1).

---

## Part A — Findings

### F-S2107-P18-001 (MEDIUM — POLICY 5 category-(i) + POLICY 4)

BC-5.39.010 §VP Anchors states "VP-102 through VP-118 (17 VPs) are planned for this story per D-945" while the BC's own §Verification Properties table enumerates 19 rows (Class A: 8; Class B: 4; Class D: 3 DEFERRED; Class E: 4). The count grew 17→19 at BC v1.12, when two advisory-path property rows ("A Arm1 Primary-Newer-than-Index Advisory (PC2a)" and "B Arm1 STORY-INDEX-Consistent Advisory (PC13a)") were added per F-P6-001 Option 1, but §VP Anchors was never swept to match — a TD-VSDD-060 sibling-site gap surviving 6 BC versions (v1.12→v1.18) undetected.

Cross-checked against S-21.07's own Token Budget row, which already correctly cites "19 VPs" — 19 is the canonical count; §VP Anchors was the sole stale site in either document. In-perimeter: BC-5.39.010 is the governing BC for this story's cascade; the stale reserved-range statement creates a 2-ID shortfall risk at post-merge VP-INDEX allocation time (an allocator reading "17 VPs... per D-945" as the authoritative range would under-reserve IDs for the two v1.12 advisory rows). Routed to product-owner (BC content owner) for the §VP Anchors reconciliation.

### Prior-pass closure independently re-verified

D-1000/D-1001/D-1002 STORY-INDEX aggregation-hygiene and frontmatter-version-parity corrections independently re-derived CLEAN: E-21 authoritative total from catalog rows L726-739 (S-21.01=11,.02=3,.03=3,.04=5,.05=5,.06=8,.07=11,.09=16,.10=5,.11=16,.12=8,.13=13,.14=8,.15=5) = 117 pts / 14 stories / 8 distinct waves — all five live aggregation cells (provenance blockquote L721 tail, DAG wave-schedule header L722, delivery blockquote L741, master-total per-epic term L763, per-epic footnote L776) agree at 14/117/8. STORY-INDEX frontmatter version:/last_amended: now reads v4.322 matching every downstream cite (D-1002 frontmatter-parity fix independently confirmed applied — zero remaining v4.321/D-998 residuals in the frontmatter block).

### Independent CLEAN axes

BC/story/STORY-INDEX version parity (BC-5.39.010 v1.18, story v1.11, STORY-INDEX catalog row v1.18/v1.11 — all three consistent pre-burst); STORY-INDEX frontmatter D-1002 parity at v4.322; E-21 aggregation 14/117/8 across all five cells (see above); three-way input-hash equality (story frontmatter 7bc1850 = STORY-INDEX catalog row 7bc1850 = delivery blockquote 7bc1850, pre-burst); AC/EC/VP counts 24/34-36/19 internally consistent between BC and story (the VP-count mismatch is confined to §VP Anchors prose, not the counted table or the story's Token Budget); retracted-claim class zero live members (fuel_cap/calibration retraction language all correctly historical-attributed); D-449 literal-shell attestation of D-1001/D-1002 genuine (captured-stdout grep/awk evidence present in burst-log Dim-2 blocks, not pseudocode).

### Observations (non-blocking, NOT findings)

**O-P17-01** (tracked carve-out, re-observed unchanged): master-total leading aggregate "Total story points: 533+ across 136 stories" (STORY-INDEX L763) remains a stale floor against the per-epic terms' current sum of 630; cross-epic master-total drift is explicitly outside the S-21.07-anchored cascade's perimeter (E-21's own 117 within the line is correct). Not reopened as an S-21.07 finding.

**O-P17-02** (likely-intentional convention, re-observed unchanged): BC-INDEX row for BC-5.39.010 shows E-12 in the epic column while anchoring story S-21.07 is E-21 — matches the entire sibling BC-5.39.003-008 validate-hook cohort convention (E-12 is the engine-governance epic owning the validate-* hook family). BC-INDEX untouched this cycle; convention stable across 17 prior passes. Reported for intent adjudication only; not a blocking mis-anchor.

### Coverage

§VP Anchors vs §Verification Properties count-parity re-derivation (whole-BC scan), D-1000/D-1001/D-1002 STORY-INDEX aggregation and frontmatter-parity re-verification (all five E-21 aggregation cells + frontmatter block), BC/story/STORY-INDEX three-way version and input-hash parity, retracted-claim class whole-story sweep, D-449 literal-shell attestation of the two most recent state-manager bursts. No findings suppressed; the one genuine drift (§VP Anchors count) is reported as MEDIUM in-perimeter.

---

## Part B — Streak / Trajectory

- Streak: **0/3** (BC-5.39.001 — RESET; pass-17's fresh CLEAN broken by this pass's in-perimeter MEDIUM; 3 fresh consecutive CLEAN passes required from pass-19).
- Trajectory: `47→18→25→25→24→20→16→8→10→1→1→2→0→1→1→0→1` (tail: `→1→1→0→1`, D-433(e)+D-439(c) LENGTH=4).
- 17 true adversary reviews; 2 CLEAN verdicts (pass-14, pass-17).
- Next gate: **pass-19 adversary** (fresh-context, reads `adversary-pass-18.md` Part A only per the Iron Law). CLOSED same-burst via product-owner (BC-5.39.010 v1.19 §VP Anchors reconciliation) + story-writer (S-21.07 v1.12 BC-version-cite propagation); pass-19 must independently re-verify the fix before any streak advance.
