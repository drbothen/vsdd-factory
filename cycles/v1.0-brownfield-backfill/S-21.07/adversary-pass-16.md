---
pass: 16
verdict: NOT-CLEAN
reviewed_head: 96b4be19158ae27131ae330f684af414821e7c5f
factory_artifacts_head: 1750bd56004274ed64fb85a19e97978ee33f3a62
novelty: null
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-15.md"
---

## Summary

VERDICT: NOT-CLEAN. Counts: BLOCKER 0, HIGH 0, MEDIUM 1, LOW 0, NIT 0 = 1. Streak HOLDS 0/3 (already reset at pass-15; a second consecutive NOT-CLEAN does not further decrement — it simply does not advance). Trajectory pass-11=1, 12=1, 13=2, 14=0, 15=1, 16=1 (tail →1→2→0→1).

## Part A — Findings

F-S2107-P16-001 (MEDIUM) — POLICY 5 category-(i) same-file aggregation-cell sibling-sweep + TD-VSDD-060: the D-998 aggregation sweep that closed F-S2107-P15-001 was **NOT class-complete**. Its predicate (`grep -n "stories total\."`) was FORM-SPECIFIC — it located only cells using the literal phrase "N stories total. M pts." and missed E-21 aggregation cells using other phrasings for the identical points/story/wave total. Three stale E-21 cells confirmed at `factory-artifacts` `1750bd56`, `STORY-INDEX.md`:

1. L722 (E-21 DAG wave-schedule blockquote header): read `(7 waves; W3 sequential: S-21.06; ...)` while the same blockquote's own body enumerates W1 through W8 (W8 parallel: S-21.14, S-21.15) — the header undercounts its own enumeration by one wave. Must read `8 waves`.
2. L776 (E-21 per-epic footnote under the master "Total story points" line): the bracketed live-tense tail read `[Historical v1.0 tally; current: 6 stories/35 pts/3 waves — see row summary]` — stale by the SAME magnitude as the just-fixed L721 defect (differs from the authoritative L741 delivery-blockquote total of 14 stories/117 pts/8 waves). Must read `14 stories/117 pts/8 waves`.
3. L763 (master "Total story points" line, footnote-linked asterisk form): the E-21 term read `35 E-21*************`. Must read `117 E-21*************`, matching L741 and the catalog-row sum.

E-21 catalog rows (L726-739, 14 rows, anchored via literal-shell `awk -F'|' '/^\| S-21\./'` restricted to table rows beginning with `| S-21.` — excludes narrative/last_amended lines that also contain the substring "S-21.") sum to 117 pts / 14 stories / 8 distinct waves (W1{01,02,03} W2{04,05} W3{06} W4{07,09,12} W5{10} W6{11} W7{13} W8{14,15}) — this is the authoritative E-21 total, already matching L721 (fixed at D-998) and L741 (delivery blockquote, always correct). D-998's sweep predicate was scoped to one phrasing ("stories total.") and therefore never examined the DAG-header, the master-line asterisk-footnote system, or the per-epic bracketed "current:" tally — three structurally distinct cell shapes carrying the identical aggregation obligation. This is the fourth generation of the fix-scoped-to-named-cell-not-every-blockquote defect class in this cascade (D-994/D-995/D-996/D-998 lineage), now recurring one level up: the *sweep gate itself* was scoped to a named phrasing, not the semantic class of "any cell asserting a points/story/wave/EC/AC total."

### File-wide semantic-role reconciliation performed this pass (human-directed, exceeds this finding's own perimeter)

A TRUE file-wide sweep was executed — every epic (E-0..E-22), every aggregation-bearing cell (authored-provenance blockquote, delivery blockquote, coverage blockquote, DAG wave-schedule blockquote, per-epic footnote, and the master line), reconciled by semantic role rather than by a single grep phrasing:

- **E-21 (in-perimeter):** catalog-row sum (literal-shell `awk`, anchored to `| S-21.` table rows) = 14 stories / 117 pts / 8 waves. L721 (already 117, D-998), L722 header (7→8 waves), L741 (already 117), L763 master (35→117), L776 footnote (6/35/3→14/117/8) — all four live cells now agree; zero remaining disagreement.
- **E-19 (out-of-perimeter, COMPLETE/MERGED epic):** canonical total is its own delivery blockquote, L716 "9 stories total. 55 pts." (catalog-verified correct — 9 rows sum to 55). L763 master term corrected 50→55. A SECOND stale "current:" footnote bracket of the identical structural shape was discovered at L777 (`current: 8 stories/50 pts`) and corrected to `current: 9 stories/55 pts` — this is a genuinely new finding beyond the human brief's enumerated E-21 sites, surfaced only by the semantic-role (not phrasing-specific) method mandated this pass.
- **E-18 (out-of-perimeter, COMPLETE/MERGED epic):** canonical total is its own delivery blockquote, L690 "17 stories total. 107 pts." L763 master term corrected 99→107. E-18's DAG-header (L667, "9 waves + prereq" / 15 stories) and footnote (L775, "104 pts total ... 15 stories ... 9-wave DAG") carry NO live "current:"-tense marker — both are dated authoring-time snapshots ("all 15 stories registered" as of 2026-06-16, before S-18.13/S-18.14 were added) and are preserved as genuinely-frozen historical narrative per the D-996(d)/D-998 precedent, consistent with this pass's own rule (only cells making a CURRENT-tense total claim are fixed). E-18's deeper catalog-vs-delivery-blockquote disagreement (18-row current catalog sum = 125 pts vs the blockquote's 107) remains OUT-OF-PERIMETER and untouched, per the standing D-996(d)/D-998 precedent — this pass does not reopen that question, only the master-line/footnote arithmetic that referenced the epic's OWN already-canonical 107 figure.
- **E-6, E-7, E-8, E-10, E-15, E-16, E-17 (out-of-perimeter):** E-10 carries an explicit self-checking total ("Total E-10 points: 45 (2+5+8+5+8+5+2+5+5)") which independently sums to 45, matching L763 — no action. E-6 is a single-row table (3 pts, matching L763's "3 E-6") — no separate blockquote to disagree. E-7/E-8/E-15/E-16/E-17 carry no live "current:"-tagged or delivery-blockquote-style terminal total distinct from L763's own figure (whole-file `grep -n "current:"` returns exactly two matches in the entire document — L776 and L777, both now fixed); their L763 terms (21/44*/14+*/8*/26*) are therefore either already consistent or have no independent canonical cell to check against, and are left untouched as no live disagreement was found.
- **E-0..E-5 (190, combined figure), E-9/E-11/E-12/E-13/E-14/E-12-F3-amendment (TBD, non-numeric):** no single canonical blockquote exists to reconcile a combined 6-epic figure or a TBD placeholder against; explicitly out of the "CURRENT-tense numeric claim with a canonical comparator" scope this pass targets — left untouched, not a live disagreement (no comparator exists).

**Fix (state-manager, this burst):** `STORY-INDEX.md` v4.321→**v4.322** — L722 header "7 waves"→"8 waves" (+W8 clause appended to header enumeration); L763 master line E-18 "99"→"107", E-19 "50"→"55", E-21 "35"→"117"; L776 E-21 footnote "current: 6 stories/35 pts/3 waves"→"current: 14 stories/117 pts/8 waves"; L777 E-19 footnote "current: 8 stories/50 pts"→"current: 9 stories/55 pts". Zero remaining live disagreements confirmed by literal-shell re-sweep (see burst-log.md D-1000 Dim-2 for captured stdout).

### Independent axes re-derived CLEAN (no finding)

Retracted-claim class (fuel_cap/10M/20M/calibrat/on_error/BC-version) ZERO live members whole-story; POLICY 7 H1 parity BC-5.39.010 H1=BC-INDEX title=story BC-table=STORY-INDEX catalog verbatim; POLICY 14 leg-5 BC-INDEX chain synced at v1.18; POLICY 18 three-way input-hash `7bc1850` (story=catalog=blockquote) unchanged; count parity 24 ACs / 34 story-ECs matches STORY-INDEX; S-21.07's own points figure (11) correct at every site in the file (S-21.07 is not itself an aggregation cell); the story spec file `S-21.07-validate-cross-site-correspondence.md` was NOT touched this pass (fix is STORY-INDEX-only, consistent with the D-998 precedent of keeping the twice-CLEAN story content stable).

### Observations

O-P16-01 [process-gap] — the existing codified lesson `L-BB-epic-total-aggregation-sweep-on-any-epic-blockquote-edit` (D-998(e)) enumerated only provenance/delivery/coverage blockquotes and operationalized the sweep via `grep "stories total\."` — a single phrasing. It would not have caught the DAG-header wave-count, the master-total line, or the per-epic bracketed footnote, all three of which are demonstrated-live aggregation surfaces this pass. STRENGTHENED this burst (see decision-log D-1000(f) and lessons.md) — the sweep obligation is now stated by SEMANTIC ROLE (any cell asserting a points/story/wave/EC/AC total for an epic, regardless of surface phrasing) rather than by a single grep pattern, and requires reconciliation against the epic's canonical total (its own delivery blockquote for out-of-perimeter/COMPLETE epics; catalog-row literal-shell sum for in-perimeter epics).

O-P16-02 (out-of-perimeter, non-finding, transparency) — this pass's file-wide method additionally re-confirmed the standing E-18 catalog-vs-blockquote disagreement (107 vs 125) already tracked as a STATE.md Blocking Issue and §8 pending-decision item (D-998). Not re-litigated or reopened this pass; the human directive this session specifically bounded the E-18 §8 item to "now fixed to canonical 107" — i.e. the MASTER-LINE arithmetic (which is now fixed and matches E-18's own canonical delivery-blockquote figure of 107), not the deeper catalog-count question, which remains open and out of this pass's perimeter.

O-P16-03 (carried) — the pass-15 O-P15-02 disposition (AC-020 Notes L596 illustrative `grep -c` missing `-E`) remains ACCEPTED-OBSERVATION-WITH-RATIONALE, unchanged; not re-examined this pass since the story file was not touched.

## Part B — Streak / Trajectory

- Streak: **0/3** (BC-5.39.001 — HOLDS at 0/3; a second consecutive NOT-CLEAN following the pass-15 reset does not advance the streak; the streak requires 3 FRESH CONSECUTIVE CLEAN verdicts, none yet earned since the reset).
- Trajectory: `47→18→25→25→24→20→16→8→10→1→1→2→0→1→1` (tail: `→2→0→1→1`, D-433(e)+D-439(c) LENGTH=4).
- 15 true adversary reviews; 1 CLEAN verdict (pass-14, unchanged).
- Next gate: **pass-17 adversary** (fresh-context, reads `adversary-pass-16.md` Part A only per the Iron Law). 3 fresh CONSECUTIVE CLEAN passes are still required from pass-16 onward (pass-16 itself was NOT-CLEAN — the count restarts at pass-17) to converge (BC-5.39.001).
