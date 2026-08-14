---
pass: 21
verdict: NOT-CLEAN
reviewed_head: 96b4be19158ae27131ae330f684af414821e7c5f
factory_artifacts_head: e9fd7607d4ed79cff77c25d784af19408bcf6201
novelty: HIGH
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-20.md"
---

## Summary

VERDICT: NOT-CLEAN. Counts: BLOCKER 0 / HIGH 1 / MEDIUM 0 / LOW 0 / NIT 0 = 1 finding. Streak RESETS 1/3 → 0/3 (BC-5.39.001; IN-PERIMETER HIGH — spec-implementation contradiction inside S-21.07's own build directive). Trajectory pass-18=1,19=1,20=0,21=1 (tail →1→1→0→1).

---

## Part A — Findings

### F-S2107-P21-001 (HIGH — POLICY 4 semantic-anchoring/spec-implementation contradiction + TD-VSDD-060)

Story Task 10 (§Tasks, lines 970-974 prior to this pass) directed the implementer to build `extract_story_bc_version_citations`'s Arm A2 extractor using `\bv?([0-9]+\.[0-9]+)\b` — the single-pass bare-optional-v regex — annotated "per BC-5.39.010 v1.19 PC13 (unchanged since v1.14)". This is the EXACT form that BC-5.39.010 v1.19 PC13 itself declares NON-CONFORMING (BC lines 411-437 mandate a two-phase algorithm; line 439 names the bare-optional-v form explicitly as the non-conforming predecessor). Task 10's directive therefore contradicted BOTH:

1. **The governing BC** — BC-5.39.010 v1.19 PC13's own two-phase mandate (Phase 1 pure-version-field `^v?([0-9]+\.[0-9]+)$` for isolated Behavioral-Contracts-table Version cells; Phase 2 BC-ID-anchored mandatory-v `\bv([0-9]+\.[0-9]+)\b` for Token Budget rows), and
2. **The story's own §BC Status section** (lines ~226-228), which has correctly documented the two-phase algorithm since story v1.5 — Task 10 was the sole stale outlier inside the same file.

**Root cause:** the regex was correct when originally set at story v1.2 against BC v1.3 PC13 (which then genuinely used the bare form). BC v1.8 replaced PC13 with the two-phase algorithm and declared the bare form NON-CONFORMING. Every subsequent BC-version-cite propagation sweep (v1.4 through v1.13, including this cascade's own POLICY 8 discipline) advanced Task 10's cited VERSION NUMBER on each pass without re-deriving the REGEX CONTENT from the BC text each time — a **version-cite-propagates / algorithm-content-does-not** gap. The false "(unchanged since v1.14)" annotation attached to the stale bare form is itself evidence of the defect: the annotation was literally true of the CITED-VERSION NUMBER (PC13's clause identity is unchanged since v1.14) while being false of the REGEX CONTENT it was attached to (which had in fact silently diverged from the BC seven story-versions earlier, at BC v1.8).

**Blast radius / severity rationale:** an implementer coding Task 10 verbatim as it read prior to this pass would reintroduce exactly the three collision classes BC-5.39.010 v1.19 PC13's own "Why the prior bare form is NON-CONFORMING" clause documents — a story-ID collision (a Trace-column ID like `S-1.03` reads as version `1.03`; 29 rows/6 stories), a BC-section-number collision (`5.39` from `BC-5.39.010` matching before the real Token-Budget version), and an ACs-column collision on **this very story's own governing citation** (the BC-table ACs cell's literal text `DEFERRED v1.6` would match before the real Version-column cell, producing a false BLOCK on S-21.07's own governing-BC row). HIGH because the defect would mislead the implementer into building the wrong extractor from a story that otherwise reads as fully v1.19-conformant — a spec-implementation contradiction with a concrete self-inflicted false-BLOCK consequence, not a cosmetic staleness.

**In-perimeter:** the contradiction is live body prose inside S-21.07's own §Tasks section (Task 10), not append-only Changelog/`[Prior:]` historical material (which would be correctly exempt under POLICY 1). A reader (or an implementer dispatched against this Task) landing on Task 10 prior to this pass would build the wrong algorithm from a story whose every other BC-version cite reads v1.19-correct. Routed to story-writer; CLOSED THIS BURST — Task 10 rewritten verbatim to the two-phase algorithm, collision rationale restated inline so the implementer cannot silently regress to the bare form, and a class-complete story-wide sweep performed for any other bare-regex-literal or algorithm-content site.

### Prior-pass closure independently re-verified

Pass-20 was CLEAN (0 findings) — there was nothing to independently re-verify a closure of. This pass's finding is a fresh first-observation, not a regression on any prior-pass axis.

### Independent CLEAN axes

Independently re-derived and confirmed CLEAN, unaffected by F-S2107-P21-001: version parity chain (story v1.13 = STORY-INDEX catalog + blockquote; BC cite v1.19 consistent across story title/H1/BC-table/narrative/AC anchors/Token Budget/§BC Status/Task 1 cites — Task 10's OWN version-number annotation was itself correctly "v1.19", only its REGEX CONTENT was stale, which is precisely the novel axis this finding surfaces). Three-way input-hash parity HOLDS at `93c4a89` (story frontmatter = STORY-INDEX catalog row = STORY-INDEX delivery blockquote; `compute-input-hash --check` exit 0 — correct, since this is a body-only algorithm-content fix, `inputs:` list untouched). POLICY 7 H1 SoT (BC-table title cell = BC-5.39.010 H1 verbatim = BC-INDEX Title cell). F-S2107-P18-001 closure (§VP Anchors count-parity, 19 VPs) and F-S2107-P19-001 closure (line-wrap/bare-token version residuals) both re-confirmed still holding — orthogonal axes, unaffected by this pass's finding. E-21 aggregation 14/117/8 consistent across all five cells. Retracted-claim class (fuel_cap/calibration language) zero live members.

### Observations (non-blocking, NOT findings)

**REC-P21-A** [test-coverage recommendation, non-blocking]: the story carries no corpus-style negative regression test analogous to BC-5.39.010's own `corpus_arm_a1` requirement that would catch a future regression from the two-phase algorithm back to the bare form via the story-ID/Token-Budget-BC-section/ACs-column collision hazards — S-21.07's own BC-table ACs cell "DEFERRED v1.6" is a live instance of exactly the ACs-column hazard class. A negative test (e.g. `test_BC_5_39_010_arm_a2_no_false_block_on_acs_column`) is recommended, anchored to S-21.07's own implementation phase (test-writer, when the story is implemented) or a future pass. Not actioned this burst — tracked only.

**OBS-P21-B** [cosmetic, non-blocking, tracked]: a pre-existing bracket-count mismatch was observed in the append-only `[Prior:...]` nesting chains of both this story's `last_amended:` field and BC-5.39.010's `last_amended:` field (opener count does not exactly equal trailing `]` count). This is harmless — the field is an opaque YAML block-scalar string, not machine-parsed nesting, so there is no parse impact — and it predates this fix. Not a finding; not corrected (append-only history is not edited per POLICY 1).

**O-P19-01/O-P15-03, O-P17-02, O-P17-01, O-P14-03** (tracked carve-outs, re-observed unchanged, NOT findings): all four re-observed in the same disposition as pass-20 — EC-count carve-out (dispositioned), BC-INDEX E-12 cohort convention (stable 20 passes now, intent-adjudication only), master-total stale-floor hygiene (out-of-perimeter), cross-artifact fuel/byte model (out-of-perimeter).

The STORY-INDEX E-21 blockquote's historical authoring annotation citing a "seven-arm Classes A/B/D/E" shape is historical-by-construction narrative describing an earlier authoring state, not a live finding — re-confirmed unchanged, not reopened.

### Coverage

Full-file read of Task 10 against BC-5.39.010 v1.19 PC13 §Postconditions (lines 411-439) and against the story's own §BC Status section; whole-story `grep -nE "bv\?\(|v\?\(\[0-9\]"` bare-form-literal sweep (4 hits — all narrative/rationale prose describing the rejected form, zero live directives beyond the one Task-10 site this finding closes); input-hash re-derivation; version-parity re-derivation across all live BC-5.39.010 cite sites. Novelty HIGH — first observation of the version-cite-propagates/algorithm-content-does-not gap class in this cascade (a companion to, but structurally distinct from, the D-996/D-998/D-1000/D-1004 fix-scoped-to-named-site families, which were all about sibling-site SCOPE or predicate ADEQUACY, not about a spec directive's own CONTENT silently diverging from its cited BC version while the citation itself stayed accurate).

---

## Part B — Streak / Trajectory

- Streak: **0/3** (BC-5.39.001 — RESETS from 1/3; IN-PERIMETER HIGH finding; 3 fresh CONSECUTIVE CLEAN passes required from pass-22).
- Trajectory: `47→18→25→25→24→20→16→8→10→1→1→2→0→1→1→0→1→1→0→1` (tail: `→1→1→0→1`, D-433(e)+D-439(c) LENGTH=4).
- 20 true adversary reviews; 3 CLEAN verdicts (pass-14, pass-17, pass-20).
- Next gate: **pass-22 adversary** (fresh-context, reads `adversary-pass-21.md` Part A only per the Iron Law). CLOSED same-burst via story-writer (S-21.07 v1.13→v1.14 — Task 10 rewritten to the two-phase algorithm verbatim, class-complete bare-regex-literal sweep performed, input-hash UNCHANGED `93c4a89`); pass-22 must independently re-verify the fix before any streak advance.
