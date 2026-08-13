---
pass: 19
verdict: NOT-CLEAN
reviewed_head: 96b4be19158ae27131ae330f684af414821e7c5f
factory_artifacts_head: 9a40f9b5fec7163f401ea1d762c911f2fbb51a85
novelty: MEDIUM
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-18.md"
---

## Summary

VERDICT: NOT-CLEAN. Counts: BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 0 / NIT 0 = 1 finding. Streak HOLDS 0/3 (BC-5.39.001; pass-18 already reset the streak — this pass is a SECOND consecutive NOT-CLEAN, the streak does not go negative, it simply does not advance; 3 fresh consecutive CLEAN required from pass-20). Trajectory pass-11=1,12=1,13=2,14=0,15=1,16=1,17=0,18=1,19=1 (tail →1→0→1→1).

---

## Part A — Findings

### F-S2107-P19-001 (MEDIUM — POLICY 5 sibling-sweep/TD-VSDD-060 + POLICY 4; aggravated by TD-VSDD-059/POLICY 15 false-attestation)

D-1003's BC-5.39.010 v1.18→v1.19 propagation burst (story-writer, S-21.07 v1.11→v1.12) claimed "all 16 live sites advanced v1.18→v1.19" and "zero live v1.18 residuals confirmed by story-wide grep sweep." This claim is FALSE. The sweep's own verification predicate — `grep -niE "BC-5\.39\.010 v1\.18"`, a single literal space, prefix-required match — structurally cannot match a whitespace-tolerant or line-wrapped variant of the same live cite. Two live residuals survived:

1. **AC-019 §Build constraints** (line-wrapped cite): the token `BC-5.39.010` appeared at the end of one line and `v1.18 §Gate Spec, F-S2107-P10-004` at the start of the next — a single logical cite split across a Markdown line-wrap boundary. The prefix-required single-space grep cannot match a cite whose "BC-5.39.010" and "v1.18" tokens are separated by a newline rather than a space. This is the un-swept third member of a triplet whose two siblings (in AC-020 Notes, same "current cap figure" / "source-HEAD-vs-operator-effective" content) were already correctly advanced in the D-1003 burst — the fix was applied to two of three occurrences of the identical clause, not the class.

2. **AC-018 §Fixture rationale**: the provenance upper-bound phrase "PC2b/E1 content unchanged through v1.18" did not carry forward. This is a bare `v1.18` token with no `BC-5.39.010` prefix on the same line at all — the grep predicate could never have matched it regardless of spacing, because the predicate's premise (requiring the two-token phrase `BC-5.39.010 v1.18`) does not cover single-token provenance-boundary citations that rely on document-level context to identify which BC's version they describe.

This is a POLICY 5 category-(i) same-file sibling-sweep gap: the fix was scoped to the finding's own named sites (the two AC-020 Notes occurrences it started from) and one further explicit consequence, but the completeness ATTESTATION ("zero live residuals confirmed") was written against a grep predicate too narrow to prove that claim — a form of the same "fix-scoped-to-named-site-not-the-class" pattern already codified at D-996/D-998/D-1000 for this cascade, but this time the process gap is one layer further in: the SWEEP EXECUTED was narrower than the SWEEP CLAIMED. Per TD-VSDD-059 (paper-fix detection) and POLICY 15 (attestation must be backed by the actual predicate run, not asserted), self-disclosed "confirmed zero residuals" language in a fix-burst changelog entry is NOT authoritative without independent re-derivation using a predicate that actually covers whitespace-tolerant and line-wrapped forms — this pass supplies that independent re-derivation and finds it false.

In-perimeter: both residual sites are live body prose inside S-21.07 itself (not append-only Changelog/`[Prior:]` historical material, which would be correctly exempt under POLICY 1). A reader landing on either AC would see a stale `v1.18` cite for the story's own governing BC. Routed to story-writer for the fix, using a whitespace-tolerant + bare-token verification predicate this time (`grep -niE "v1\.1[0-8]"` to catch every `v1.10`–`v1.18` token regardless of prefix, plus a line-wrap-specific check `grep -nA1 "BC-5\.39\.010$"` for cites split across a wrap boundary).

### Prior-pass closure independently re-verified

D-1003's F-S2107-P18-001 closure (the §VP Anchors count-parity fix) independently re-confirmed CLEAN: BC-5.39.010 §VP Anchors now reads "VP-102 through VP-120 (19 VPs)", matching the BC's own 19-row §Verification Properties table exactly (Class A: 8; Class B: 4; Class D: 3 DEFERRED; Class E: 4 = 19). No regression on this axis — the count-parity fix itself is sound; the finding in this pass is a SEPARATE, narrower version-propagation-completeness defect in the same burst's companion fix.

### Independent CLEAN axes

BC-5.39.010→S-21.07 catalog propagation: version cell, title/H1, narrative "authors this gate" clause, BC Status governing-BC statement, AC-001/009/020(×3)/022/023/024 body "Under BC..." anchors, BC table version cell, Token Budget row, Task 1/Task 10 all independently re-derived at v1.19 (correct, pre-existing this pass). Three-way input-hash parity HOLDS at `93c4a89` (story frontmatter = STORY-INDEX catalog row = STORY-INDEX delivery blockquote, re-confirmed via `compute-input-hash --check` exit 0). BC/story/STORY-INDEX version parity (BC-5.39.010 v1.19, story v1.12, STORY-INDEX catalog row v1.19/v1.12 — all consistent pre-burst). E-21 aggregation 14/117/8 across all five live cells (provenance blockquote, DAG header, delivery blockquote, master-line, footnote) re-confirmed unchanged and correct. Retracted-claim class (fuel_cap/calibration retraction language) re-swept whole-story: zero live members, all correctly historical-attributed. D-449 literal-shell attestation of the D-1003 burst's own Dim-2 evidence re-confirmed genuine (captured-stdout `grep -o` count=0 claim present verbatim in burst-log, not pseudocode) — though the underlying claim that attestation supported ("zero v1.18 residuals") is itself the defect this pass surfaces: the D-449 gate confirms the SHELL COMMAND WAS RUN LITERALLY, not that the shell command's PREDICATE was adequate to the claim it was used to support. This distinction — mechanical-execution-evidence vs. predicate-adequacy — is itself worth carrying forward as a standing caution.

### Observations (non-blocking, NOT findings)

**O-P19-01**: STORY-INDEX catalog row cites "34 ECs" for S-21.07 while the BC-5.39.010 body cites "36 ECs" (EC-035/EC-036 both trace to the BC but are not independently counted in the story's summary cell). This is the pre-existing, already-tracked O-P15-03 carve-out (POLICY 8 EC-mirror is story→BC directional, not bidirectional-count-parity) — re-observed unchanged, not reopened as a new finding, not fixed this pass.

**O-P17-01** (tracked carve-out, re-observed unchanged): STORY-INDEX master-total leading aggregate "Total story points: 533+ across 136 stories" remains a stale floor against the per-epic terms' current sum (~630); cross-epic master-total drift is outside the S-21.07-anchored cascade's perimeter. Not reopened.

**O-P17-02** (likely-intentional convention, re-observed unchanged): BC-INDEX row for BC-5.39.010 shows E-12 in the epic column while anchoring story S-21.07 is E-21 — matches the entire sibling BC-5.39.003-008 validate-hook cohort convention. Stable across 18 prior passes. Reported for intent adjudication only; not a blocking mis-anchor.

**O-P14-03** (out-of-perimeter, re-observed unchanged): cross-artifact fuel/byte model reconciliation (the relationship between `max_bytes` read caps and measured fuel consumption across sibling hook specs) remains a documentation-consistency question spanning multiple BCs, not scoped to S-21.07 alone. Not reopened.

**O-P15-03** (dispositioned, re-observed unchanged): POLICY 8's EC-mirror obligation is story→BC directional (the story must not invent ECs absent from its governing BC) — it does not require the story's own summary EC-count cell to equal the BC's total EC count when the story only exercises a subset. Non-violation, not reopened. (Directly relevant this pass — see O-P19-01 above, which is the concrete instance of this already-dispositioned question.)

### Coverage

Whole-story `grep -niE "v1\.1[0-8]"` bare-token sweep (catches every v1.10–v1.18 occurrence regardless of prefix/spacing) plus line-wrap-specific `grep -nA1 "BC-5\.39\.010$"` and `grep -niE "BC-5\.39\.010[[:space:]]"` checks; every hit individually triaged against four legitimate categories (append-only `[Prior:]` chain; body Changelog historical rows; legitimately-historical §BC Status provenance annotations; fixture/example strings for the unrelated example BC BC-6.26.001 in AC-005/AC-006/EC-002/EC-026) versus live-cite residuals. D-1003 §VP Anchors fix re-derivation. Three-way input-hash and version-parity re-derivation. Retracted-claim class whole-story sweep. D-449 literal-shell attestation review of the D-1003 burst's own evidence. No findings suppressed; the one genuine drift (two live v1.18 residuals surviving an inadequate sweep predicate, aggravated by a false completeness attestation) is reported as MEDIUM in-perimeter.

---

## Part B — Streak / Trajectory

- Streak: **0/3** (BC-5.39.001 — HOLDS; second consecutive NOT-CLEAN; the streak was already at 0/3 after pass-18's reset and a further NOT-CLEAN pass does not decrement below 0 — it simply does not advance; 3 fresh consecutive CLEAN passes required from pass-20).
- Trajectory: `47→18→25→25→24→20→16→8→10→1→1→2→0→1→1→0→1→1` (tail: `→1→0→1→1`, D-433(e)+D-439(c) LENGTH=4).
- 18 true adversary reviews; 2 CLEAN verdicts (pass-14, pass-17).
- Next gate: **pass-20 adversary** (fresh-context, reads `adversary-pass-19.md` Part A only per the Iron Law). CLOSED same-burst via story-writer (S-21.07 v1.13 — 2 residual sites corrected v1.18→v1.19, plus corrected last_amended attestation using a whitespace-tolerant + bare-token verification predicate); pass-20 must independently re-verify the fix before any streak advance.
