# ADR-046 Adversarial Spec-Convergence Review — Pass 49

**Reviewed artifact set (frozen):** ADR-046 v1.19 + BC-4.17.001 v1.22 + BC-5.40.001 v1.18 + BC-7.07.001 v1.35
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED)
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **STAYS 0/3** (already 0/3 from pass-46's reset; a finding keeps it there)
**D-chain:** D-1106

## Part A — Finding Set (frozen set: ADR-046 v1.19 + BC-4.17.001 v1.22 + BC-5.40.001 v1.18 + BC-7.07.001 v1.35)

**HIGH (0):** none this pass.

**MEDIUM (1):**

- **F-P49-001 (MED, POLICY 18, inputs:-completeness).** ADR-046's own v1.19 disposition prose —
  the F-P48-001 fix that re-attributed AC-007 to S-17.01 and quoted BC-5.40.001 Invariant 2
  verbatim in §Rationale/§Source-Origin — cited `.factory/stories/S-17.01-factory-lock-schema-cas-
  push.md` by name and by its AC-007 content, but never added S-17.01 to ADR-046's own `inputs:`
  array. This is a FRESH straggler CREATED by the pass-46/48 AC-007 re-attribution edits
  themselves (the citation did not exist before pass-46; pass-46/47/48's own grep-complete inputs
  audits therefore could not have caught it, since each ran BEFORE the citation that created the
  gap, or was scoped to a different artifact). **VERIFIED**: `grep -c "S-17.01-factory-lock-schema-
  cas-push" ADR-046...md` against the live §Rationale/§Source-Origin prose confirms the citation
  is load-bearing (the AC-007 text is quoted, not merely referenced); `grep -c
  "S-17.01-factory-lock-schema-cas-push.md" <same file's inputs: array>` confirmed zero hits prior
  to this pass's fix.
  **Mandatory grep-complete inputs RE-AUDIT performed** (in-scope, this pass, triggered by
  F-P49-001 and the newly-codified CITATION→INPUT PARITY discipline this finding produces): every
  file-path-shaped and story-ID-shaped token in ADR-046's live body was re-enumerated and checked
  against `inputs:`. One further genuine omission found: §Companion Amendment 3 quotes S-18.04a's
  AC-018 verbatim (`.factory/stories/S-18.04a-precompact-flush-sh-core.md`) — cited since pass-43's
  F-P43-002 fix, never added to `inputs:` at that time or any pass since. Both S-17.01 and S-18.04a
  are genuinely load-bearing (verbatim-quoted content, not incidental mentions).
  Fixed by architect: both `.factory/stories/S-17.01-factory-lock-schema-cas-push.md` and
  `.factory/stories/S-18.04a-precompact-flush-sh-core.md` added to ADR-046's `inputs:` array;
  bracket-balance of the frontmatter `last_amended` field re-verified 27/27 (unchanged by this
  addition — the array edit does not touch the `last_amended` nesting). ADR-046 v1.19→v1.20.

**LOW (0):** none this pass.

**Audit-extra findings (non-blocking to THIS pass's verdict, but genuine defects fixed same-burst
per the CITATION→INPUT PARITY discipline this pass's finding produces):** product-owner ran the
identical grep-complete inputs re-audit across all three companion BCs (cluster-wide, per the
D-1104 eleventh-discipline standing default) and found 5 further citation-without-input
stragglers, all of the same class as F-P49-001 (a body edit added a verbatim citation without a
same-burst `inputs:` addition):

- **BC-4.17.001**: Invariant 3 cites S-17.01's AC-007 verbatim (added at v1.22/F-P47-001) —
  S-17.01 was NOT in `inputs:` until this burst; Invariant 5 cites BC-1.17.001 verbatim — also
  missing. Both added. BC-4.17.001 v1.22→v1.23.
- **BC-5.40.001**: §Verification Properties/§VP Anchors rows attribute AC-001..AC-005 to S-19.08
  by exact citation — S-19.08 was NOT in `inputs:`. Added. BC-5.40.001 v1.18→v1.19.
- **BC-7.07.001**: PC4/Architecture Anchors cite `plugins/vsdd-factory/bin/factory-lock-write.sh`'s
  break-glass-fallback behavior by name — the file was NOT in `inputs:` (a distinct straggler from
  BC-5.40.001's own already-present citation of the same file); Related BCs cites BC-7.07.002 by
  name and cross-reference — also missing. Both added. BC-7.07.001 v1.35→v1.36.

7 total citation-without-input stragglers found and fixed cluster-wide this burst (1 on ADR-046
counted as F-P49-001's own fix scope + 1 audit-extra on ADR-046 itself + 5 on the three companion
BCs).

**One MEDIUM finding on the reviewed frozen set, fixed same-burst, plus 6 audit-extra stragglers
(1 further ADR-046 straggler + 5 BC stragglers) found and fixed via the mandatory cluster-wide
re-audit this finding triggered. VERDICT: FINDINGS. Streak STAYS 0/3** per BC-5.39.001's
literal-3-CLEAN discipline — the streak was already at floor from pass-46's reset; this pass's
MEDIUM finding alone is sufficient to keep it there (there is no lower floor than 0/3).

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set on
its own merits only). Every dimension this gate's 48-pass history has previously found a defect in
was independently re-checked against the current frozen set and confirmed holding, with zero
regression beyond the one item above:

- **AC-attribution cluster-wide drain (D-1100, eighth discipline, extended D-1103/D-1104,
  eleventh discipline):** re-derived every `AC-[0-9]+` live-body citation across all four
  frozen-set artifacts — zero mis-anchors found, class remains genuinely DRAINED, no regression.
- **Byte-range/body-confinement arm-scope reconciliation (D-1096/D-1097 classes):** re-derived
  across every `extract_frontmatter`-guarantee locus — all confirmed correctly arm-split, no
  regression.
- **Verbatim-absence claims (D-1105, twelfth discipline):** this pass made no absence/fabrication
  claims of its own; no regression of the twelfth-discipline class to check against.
- **Summary-enumeration accuracy (D-1105, thirteenth discipline):** re-swept BC-7.07.001's
  Description/Postcondition 8 exit-0 enumeration (the pass-48 fix) — confirmed complete, no
  regression.
- **Grep-complete cluster-wide `inputs:` completeness (D-1090/D-1100), UP TO THE POINT OF THE
  LAST BODY-EVOLVING EDIT:** the pre-existing audits (passes 43, 46, 47) were each individually
  correct AS OF the content they audited; this pass's finding is not a failure of any single
  audit's execution, but a demonstration that a POINT-IN-TIME audit does not stay valid across
  subsequent body-evolving edits that add new citations — the root observation CODIFIED this
  pass as the fourteenth discipline.
- **4-leg `modified:`-array head==version parity (D-1089):** all four artifacts' `modified:`
  array heads confirmed matching their own frontmatter `version:` field, prior to this pass's edit.
- **Illustrative-quote verbatim-source-accuracy + sibling-parity check (D-1101, ninth
  discipline):** all three companion BCs' illustrative CAP-031/CAP-032 quotes re-derived and
  confirmed verbatim-correct against `capabilities.md` — no regression, FIFTH consecutive
  confirmation.
- **ADR §Decision/§N.M anchor correctness (D-1092):** every citation independently re-derived
  from the cited ADR's own section content — all CORRECT.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):** recounted —
  confirmed 6 numbered decisions, correctly cited "1–6" throughout.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts — stable
  since pass-27 (23 consecutive passes now, counting this one). **This pass's finding does not
  touch the behavioral core — it is confined entirely to the `inputs:`-completeness perimeter.**
- **Every load-bearing code claim (function names, file paths, constant names) independently
  re-verified against the actual source files:** all accurate.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) prior to this burst's edit; all four artifacts edited this pass (ADR-046 + all three
  companion BCs), expected to produce multiple 1-hop residuals — settled per state-manager's usual
  disposition, not force-converged.

**Novelty assessment:** F-P49-001 belongs to a defect class NOT previously identified at this gate
as a distinct discipline — it is a SIBLING of the D-1090/D-1100 grep-complete-inputs-audit class,
but distinguished by WHEN the gap opens: prior inputs findings (F-P28-002, F-P29-002, F-P30-002,
F-P31-001, F-P33-001, F-P35-002, F-P43-001) were all PRE-EXISTING omissions that a
not-yet-performed audit would have caught. F-P49-001 is the FIRST instance where a
previously-CLEAN inputs audit (pass-43's grep-complete sweep, extended cluster-wide at pass-44)
was subsequently invalidated by a LATER body edit (pass-46/48's AC-007 re-attribution) that added
a new citation without a same-burst inputs update — the audit was correct when it ran; the ground
shifted under it. This is the sixth instance of the recurring META-pattern first observed at
pass-37 (a remediation's own body/prose changes are themselves attack surface for a DIFFERENT
discipline than the one the remediation itself was fixing), but distinct enough in mechanism
(citation-add without inputs-add, not a fabrication or enumeration-completeness defect) to warrant
its own discipline: CITATION→INPUT PARITY (fourteenth).

## Part C — State at Close of Review

ADR-046 **v1.19→v1.20** (F-P49-001 fix + 1 audit-extra straggler, architect). BC-4.17.001
**v1.22→v1.23** (2 audit-extra stragglers, product-owner). BC-5.40.001 **v1.18→v1.19** (1
audit-extra straggler, product-owner). BC-7.07.001 **v1.35→v1.36** (2 audit-extra stragglers,
product-owner). BC-5.39.001 3-CLEAN streak: **0/3 → STAYS 0/3** (the MEDIUM finding keeps it
there; already at floor from pass-46's reset). Gate history to date: 49 passes run against
evolving/frozen sets; 39 genuine findings found and fixed (adding F-P49-001), plus 4 audit-extra
stragglers from earlier passes (pass-31, pass-33) and 6 audit-extra stragglers this pass (1 on
ADR-046 + 5 across the 3 BCs), 1 latent-bracket drain (pass-37, not counted as genuine), 1
ACCEPTED non-blocking observation (O-P42-001, does not count against the streak), and 2 FIXED
non-blocking observations (O-P44-001, O-P48-001); 6 clean passes to date (34, 36, 38, 41, 42, 45).

**Index reconciliation (state-manager, this burst):** ARCH-INDEX v3.89→v3.90 (ADR-046 row
version-chain cell appended, pass-49 fix note). BC-INDEX v5.13→v5.14 (BC-4.17.001/BC-5.40.001/
BC-7.07.001 row version-chain cells appended).

**NEXT: fresh pass-50** against the newly-frozen set (ADR-046 v1.20 + BC-4.17.001 v1.23 +
BC-5.40.001 v1.19 + BC-7.07.001 v1.36), starting a new streak at 0/3, applying all thirteen
now-codified convergence-technique disciplines proactively from the start, plus the new
fourteenth discipline this pass produces: CITATION→INPUT PARITY — any body edit that ADDS a
verbatim citation/quote of a source file/story MUST add that source to `inputs:` in the SAME
burst; because the grep-complete inputs audit is point-in-time, a run of body-evolving bursts
(such as passes 43-48's AC re-attributions) can re-open the gap even after a prior audit passed
clean, mandating a periodic CLUSTER-WIDE re-audit after any such run. The human decision this
session remains to CONTINUE looping toward literal 3-CLEAN (not accept D-386 Option C asymptotic
acceptance). S-17.05 TDD implementation remains gated on convergence.
