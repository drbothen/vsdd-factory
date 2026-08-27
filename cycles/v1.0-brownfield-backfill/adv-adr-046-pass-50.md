# ADR-046 Adversarial Spec-Convergence Review — Pass 50

**Reviewed artifact set (frozen):** ADR-046 v1.20 + BC-4.17.001 v1.23 + BC-5.40.001 v1.19 + BC-7.07.001 v1.36
**Review date:** 2026-08-27
**Verdict:** FINDINGS (2 MED)
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **STAYS 0/3** (already 0/3 from pass-46's reset; a finding keeps it there)
**D-chain:** D-1107

## Part A — Finding Set (frozen set: ADR-046 v1.20 + BC-4.17.001 v1.23 + BC-5.40.001 v1.19 + BC-7.07.001 v1.36)

**HIGH (0):** none this pass.

**MEDIUM (2):**

- **F-P50-001 (MED, POLICY 4, false-'verified present' traceability defect).** BC-4.17.001's own
  §Story Anchor and ADR-046's own §File-Change Plan both assert that S-17.05 — the ADR-046
  implementing story — is "verified present in STORY-INDEX.md" / "is its catalog entry." **This
  claim was FALSE.** STORY-INDEX's E-17 roster ended at S-17.04 with the epic marked "COMPLETE";
  S-17.05 existed as a drafted story FILE but was never REGISTERED as a STORY-INDEX catalog row.
  **VERIFIED**: `grep -c "^| S-17.05 |" .factory/stories/STORY-INDEX.md` against the pre-fix file
  content confirmed zero hits — no catalog row existed. The false "verified present" language
  traces to the pass-25 F-P25-002 remediation (2026-08-26), which resolved the Traceability
  `[pending]`→S-17.05 anchor in the BC prose but never performed — nor claimed to perform — the
  actual STORY-INDEX membership check the "verified present" wording asserts. The gap survived
  ~24 further passes (pass-26 through pass-49) because no pass's grep-complete inputs/citation
  audit was scoped to check STORY-INDEX row-membership specifically — every prior audit checked
  `inputs:`-completeness or AC-attribution, never catalog-row existence.
  **Fixed by state-manager**: S-17.05 REGISTERED in STORY-INDEX.md (v4.391→v4.392) — E-17 roster
  reconciled: story_count 4→5, points 26→34, waves 1-4→1-5 (waves 1-4 remain MERGED/COMPLETE per
  issue #170; wave 5 draft, pending BC-5.39.001 3-CLEAN spec gate, NOT started). This makes the
  BC-4.17.001/ADR-046 "verified present in STORY-INDEX" claim TRUE without editing either of the 4
  frozen-set specs — the defect was in the catalog, not in the citing prose.

- **F-P50-002 (MED, POLICY 18, inputs:-completeness).** S-17.05 is cited by exact file path and by
  content claims (§Story Anchor, E-17 Wave 5, 8pts, `tdd_mode: strict`) in all three companion BCs'
  live bodies, but S-17.05 was absent from all three BCs' `inputs:` arrays. **VERIFIED**:
  `grep -c "S-17.05-stamp-state-timestamp-hook" <each BC's inputs: array>` confirmed zero hits
  prior to this pass's fix, against a confirmed nonzero exact-path/content citation count in each
  BC's live body (§Story Anchor section). This is an instance of the CITATION→INPUT PARITY
  discipline (fourteenth, D-1106) extended: exact-path story citations require the story in
  `inputs:`, same as file/content citations of BCs, ADRs, or code.
  **Fixed by product-owner**: S-17.05 added to `inputs:` in BC-4.17.001 (v1.23→v1.24), BC-5.40.001
  (v1.19→v1.20), BC-7.07.001 (v1.36→v1.37). BC-5.40.001's own cross-check additionally found and
  fixed a sibling gap in the same sweep: S-17.01 (cited with content claims in §Traceability
  Stories row and §Story Anchor since PR #181/D-544, 2026-06-11) was ALSO missing from
  BC-5.40.001's `inputs:` — added same-burst.
  ADR-046 v1.20 **UNCHANGED** — it already listed S-17.05 in its own `inputs:` array; only the
  three companion BCs carried this straggler.

**LOW (0):** none this pass.

**Two MEDIUM findings on the reviewed frozen set, both fixed same-burst. VERDICT: FINDINGS. Streak
STAYS 0/3** per BC-5.39.001's literal-3-CLEAN discipline — the streak was already at floor from
pass-46's reset; these findings alone are sufficient to keep it there (there is no lower floor
than 0/3).

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set on
its own merits only). Every dimension this gate's 49-pass history has previously found a defect in
was independently re-checked against the current frozen set and confirmed holding, with zero
regression beyond the two items above:

- **AC-attribution cluster-wide drain (D-1100, eighth discipline, extended D-1103/D-1104,
  eleventh discipline):** re-derived every `AC-[0-9]+` live-body citation across all four
  frozen-set artifacts — zero mis-anchors found, class remains genuinely DRAINED, no regression.
- **Byte-range/body-confinement arm-scope reconciliation (D-1096/D-1097 classes):** re-derived
  across every `extract_frontmatter`-guarantee locus — all confirmed correctly arm-split, no
  regression.
- **CITATION→INPUT PARITY (D-1106, fourteenth discipline), applied to its own class:** F-P50-002
  is a genuine recurrence of the class D-1106 codified, extended from BC/ADR/code citations to
  exact-path STORY citations — not previously enumerated as in-scope for the fourteenth
  discipline's audit perimeter until this pass. No other `inputs:` gaps found beyond S-17.05 (and,
  cross-check-triggered, S-17.01 on BC-5.40.001) across the full frozen set.
  **Grep-complete cluster-wide `inputs:` completeness (D-1090/D-1100), UP TO THE POINT OF THE
  LAST BODY-EVOLVING EDIT:** the pass-49 audit was correct as of the content it audited — this
  pass's F-P50-002 is not a failure of that audit's execution, but confirmation that the
  fourteenth discipline's own perimeter (verbatim citations of FILES) had not yet been explicitly
  extended to STORY-ID exact-path citations before this pass.
- **4-leg `modified:`-array head==version parity (D-1089):** all four artifacts' `modified:`
  array heads confirmed matching their own frontmatter `version:` field, prior to this pass's edit.
- **Illustrative-quote verbatim-source-accuracy + sibling-parity check (D-1101, ninth
  discipline):** all three companion BCs' illustrative CAP-031/CAP-032 quotes re-derived and
  confirmed verbatim-correct against `capabilities.md` — no regression, SIXTH consecutive
  confirmation.
- **ADR §Decision/§N.M anchor correctness (D-1092):** every citation independently re-derived
  from the cited ADR's own section content — all CORRECT.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):** recounted —
  confirmed 6 numbered decisions, correctly cited "1–6" throughout.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts — stable
  since pass-27 (24 consecutive passes now, counting this one). **Neither of this pass's findings
  touches the behavioral core — both are confined to the traceability/catalog-membership and
  inputs-completeness perimeters.**
- **Every load-bearing code claim (function names, file paths, constant names) independently
  re-verified against the actual source files:** all accurate.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) prior to this burst's edit; three of four artifacts edited this pass (BC-4.17.001,
  BC-5.40.001, BC-7.07.001 — ADR-046 UNCHANGED), producing multiple residuals — settled per
  state-manager's usual disposition, not force-converged.

**Novelty assessment:** F-P50-001 belongs to a defect class NOT previously identified at this gate
as a distinct discipline — it is the FIRST finding to catch a "verified present in an index"
assertion that was never backed by an actual mechanical membership check, as opposed to a citation
that is present-but-not-in-`inputs:` (the fourteenth discipline's own class) or a citation that is
factually mis-scoped (the AC-attribution class). F-P50-002 is a direct EXTENSION of the fourteenth
discipline (CITATION→INPUT PARITY, D-1106) to a new citation TYPE (exact-path story citations)
that discipline's initial codification did not explicitly enumerate. Together these findings
close a ~48-pass-old traceability gap the gate's own inputs/AC-focused audits could not have
caught, because neither audit type checks catalog-row EXISTENCE.

## Part C — State at Close of Review

ADR-046 v1.20 **UNCHANGED** (already listed S-17.05 in `inputs:`; no edit this pass). BC-4.17.001
**v1.23→v1.24** (F-P50-002 fix, product-owner). BC-5.40.001 **v1.19→v1.20** (F-P50-002 fix +
S-17.01 cross-check sibling gap, product-owner). BC-7.07.001 **v1.36→v1.37** (F-P50-002 fix,
product-owner). STORY-INDEX **v4.391→v4.392** (F-P50-001 fix — S-17.05 registered, E-17 roster
reconciled, story-writer). BC-5.39.001 3-CLEAN streak: **0/3 → STAYS 0/3** (both MEDIUM findings
keep it there; already at floor from pass-46's reset). Gate history to date: 50 passes run against
evolving/frozen sets; 41 genuine findings found and fixed (adding F-P50-001 + F-P50-002), plus 10
audit-extra stragglers from earlier passes (pass-31, pass-33, pass-49 ×6), 1 latent-bracket drain
(pass-37, not counted as genuine), 1 ACCEPTED non-blocking observation (O-P42-001, does not count
against the streak), and 2 FIXED non-blocking observations (O-P44-001, O-P48-001); 6 clean passes
to date (34, 36, 38, 41, 42, 45).

**Index reconciliation (state-manager, this burst):** BC-INDEX v5.14→v5.15 (BC-4.17.001/
BC-5.40.001/BC-7.07.001 row version-chain cells appended, pass-50 fix notes). STORY-INDEX v4.392
already bumped by story-writer this burst (F-P50-001 registration). ARCH-INDEX v3.90 UNCHANGED
(ADR-046 not edited this pass). VP-INDEX v2.79 UNCHANGED.

**STORY-INDEX pre-existing stale-aggregate drift (accepted non-blocking, NOT fixed this
burst):** story-writer's own F-P50-001 registration burst flagged that STORY-INDEX's headline
"131 stories across 20 epics" text and its §Status Summary counts are PRE-EXISTING stale drift
that predates E-18/E-19/E-21 growth — out of F-P50-001's own perimeter (which is scoped to E-17's
S-17.05 registration only). Recorded as a NEW tracked Drift Item this burst: anchor is the next
maintenance sweep OR a full STORY-INDEX headline/Status-Summary reconciliation pass, whichever
comes first — explicitly NOT remediated this burst.

**NEXT: fresh pass-51** against the newly-frozen set (ADR-046 v1.20 [UNCHANGED] + BC-4.17.001
v1.24 + BC-5.40.001 v1.20 + BC-7.07.001 v1.37), starting a new streak at 0/3, applying all
fourteen now-codified convergence-technique disciplines proactively from the start, plus the
extension F-P50-002 demonstrates: CITATION→INPUT PARITY (fourteenth discipline) covers exact-path
STORY citations as well as file/BC/ADR citations — any body edit that ADDS a verbatim or
exact-path citation of a source file/story MUST add that source to `inputs:` in the SAME burst.
Additionally, a "verified present in <index>" assertion in spec prose is NOT self-certifying —
any such claim MUST be backed by an actual mechanical index-membership check at the time the
claim is made (F-P50-001's own new discipline, mandate: register implementing stories in
STORY-INDEX at draft time; any story-anchor "present-in-index" assertion requires the membership
check). The human decision this session remains to CONTINUE looping toward literal 3-CLEAN (not
accept D-386 Option C asymptotic acceptance). S-17.05 TDD implementation remains gated on
convergence.
