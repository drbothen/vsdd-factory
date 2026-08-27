# ADR-046 Adversarial Spec-Convergence Review — Pass 37

**Reviewed artifact set (frozen):** ADR-046 v1.16 + BC-4.17.001 v1.16 + BC-5.40.001 v1.15 + BC-7.07.001 v1.33
**Review date:** 2026-08-26
**Verdict:** FINDINGS (1 MED), 0 HIGH, 1 LOW observation
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 1/3 → **RESETS to 0/3** (a finding after a clean pass resets the streak; 2nd reset of the session, following the pass-35 reset)
**D-chain:** D-1094

## Part A — Finding Set (frozen set: ADR-046 v1.16 + BC-4.17.001 v1.16 + BC-5.40.001 v1.15 + BC-7.07.001 v1.33)

**HIGH (0):** none this pass.

**MEDIUM (1):**

- **F-P37-001 (MED, POLICY 4, semantic-anchoring-integrity)** — BC-4.17.001 v1.16's and BC-5.40.001
  v1.15's own `modified:`/`last_amended`/Changelog amendment prose (the pass-35 remediation's OWN
  audit-narrative text, 3 loci each: `modified:` array entry, `last_amended` field, `## Changelog`
  table row) falsely stated that ADR-046 has "a flat `## Decision` list, 1–5, ... read in full, all
  correct." Opened ADR-046 directly and counted: the `## Decision` section is a flat numbered list
  of **6** items (1–6), not 5. Item 6 (same-release ship + CI-gating registry-invariant XOR check —
  `has_entry(verify-state-timestamp-refresh) XOR has_entry(stamp-state-timestamp)`) exists in
  ADR-046's actual text and was silently omitted from both BCs' self-attested "read in full"
  decision-count claim. This is a citation-count defect in the REMEDIATION'S OWN bookkeeping
  narrative, not in either BC's operative spec content (Preconditions/Postconditions/Invariants/EC)
  and not a further ADR-046 mis-anchor beyond what pass-35 already found and fixed — every actual
  `ADR-046 Decision N` citation in both BCs' bodies (Decision 1/1(a)/1(b), 2, 4, 5) remains correctly
  numbered and unaffected by this finding.
  **Disposition: FIXED.** Product-owner corrected the decision-count assertion from "1–5" to "1–6"
  in all 6 loci (3 in BC-4.17.001 v1.16→v1.17, 3 in BC-5.40.001 v1.15→v1.16), naming item 6
  explicitly by its actual content, with MINIMAL, factual disposition prose — no new completeness
  certification was added in its place (see O-P37-001 below and the lessons.md mitigation this
  finding triggers). No PC/Invariant/EC renumbered; both fixes are in-place corrections to existing
  historical/amendment-narrative text (POLICY 1 append-only preserved).

**LOW (1 observation):**

- **O-P37-001 ([process-gap])** — self-attested "read in full, all correct" audit-narrative claims
  (the exact phrasing both v1.16 entries used to describe the pass-35 ADR §Decision anchor audit)
  have no mechanical backing — nothing greps the cited artifact's actual section-list cardinality
  and compares it against the prose's own count assertion before the assertion is written down. A
  greppable "N distinct `### Decision` / numbered-list items found, prose claims M" check would have
  caught this class at write-time rather than two passes later. Recorded as a process-gap, not a
  fix — see lessons.md `[codified][process-gap]` entry for the mitigation now in force.

**Latent defect additionally drained (proactive, in-scope, PO-surfaced):** BC-5.40.001's own
`last_amended` field carried a pre-existing nested-history bracket-count defect (16 `[Prior:` opens
vs. 13 closing `]`s) predating this pass — unrelated to F-P37-001, surfaced by the product-owner
during this burst's edit and folded into the same v1.16 bump. Corrected to 16/16 balanced. This is
NOT a fresh finding attributable to any prior pass's adversary (the defect predates pass-35's own
edit and was never independently caught); recorded here as drained bookkeeping hygiene, not as a new
F-P37-NNN finding, since it carries no semantic-content risk (frontmatter parse tooling reads
`last_amended` as an opaque string; the bracket imbalance was cosmetic).

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **Substance (Preconditions/Postconditions/Invariants/Edge-Cases/behavioral core across all four
  artifacts):** re-verified clean — **the substance converged; this pass's only finding is a false
  decision-count in the remediation's OWN audit prose**, not a spec-vs-code or spec-vs-spec defect.
  Stable since pass-27 (11 consecutive passes now, counting this one).
- **Every actual `ADR-046 Decision N`/`§N.M` citation in BC-4.17.001 and BC-5.40.001's operative
  body text (Decision 1/1(a)/1(b), 2, 4, 5):** independently re-verified correct against ADR-046's
  actual section content — the F-P37-001 defect is confined to the SUMMARY-COUNT assertion in
  historical amendment prose, not to any live citation.
- **`ADR-025 §Decision N` citations (both BCs' Precondition/Architecture-Anchors sourcing, per the
  pass-35 fix):** re-verified holding, no regression.
- **`modified:`-array-head-parity (4-leg self-check, D-1089):** re-verified holding on both edited
  BCs (BC-4.17.001 v1.17, BC-5.40.001 v1.16) and unaffected BC-7.07.001/ADR-046 — no regression.
- **`inputs:` completeness (GREP-COMPLETE method, D-1090):** no further missing load-bearing
  spec/code citations found on either edited BC, ADR-046, or BC-7.07.001.
- **BC-to-BC cross-anchor citation accuracy (`§Section`/`PCn`/`Invariant-N`):** re-verified across
  all three companion BCs — all resolve correctly, no drift.
- **POLICY 19 (anti-volatile-pin):** no new load-bearing inline version-pin token introduced by this
  burst's fixes.
- **§Story Anchor / Traceability parity:** re-verified clean across all three companion BCs.
- **ADR-046/BC-7.07.001:** UNCHANGED, not touched this pass — neither carries the false
  decision-count assertion (that prose exists only in BC-4.17.001's and BC-5.40.001's own
  self-referential v1.15/v1.16 amendment narratives).

**Clean-axes ledger (adversary's own accounting, this pass):** 9 of 10 standing convergence-technique
axes checked returned CLEAN with zero findings — behavioral core, ADR-046 Decision-citation
correctness (live body text), ADR-025 Decision-citation correctness, 4-leg modified-head parity,
grep-complete inputs completeness, BC-to-BC cross-anchor accuracy, POLICY 19 anti-volatile-pin,
Story-Anchor/Traceability parity, and BC-INDEX/ARCH-INDEX/STORY-INDEX/VP-INDEX row-cell parity
(unaffected, no BC content-shape change). The 10th axis — self-attested audit-narrative accuracy
(the decision-count claim inside the pass-35 fix's OWN disposition prose) — is exactly where
F-P37-001 was found; this is a defect IN a prior remediation's bookkeeping, not a new class of
spec-vs-code drift.

**Novelty assessment:** this is NOT a newly-revealed dimension in the sense pass-35 was (pass-35
discovered ADR §Decision anchor-correctness as an entirely unaudited citation CLASS). Pass-37's
finding is a garden-variety factual error — a wrong small-integer count — inside a narrative
description of a fully-covered, already-codified dimension (the ADR §Decision anchor audit itself,
D-1092). The audit discipline is sound; this particular instance of applying it (at pass-35) mis-
stated its own cardinality. **Meta-significance:** remediation disposition prose that makes a
sweeping self-attested completeness claim ("read in full, all correct") is itself falsifiable
attack surface for a fresh-context adversary — the claim can be independently checked, and here it
was wrong. Per BC-5.39.001, a finding after a clean pass (pass-36) resets the streak: **1/3 → 0/3.**
This is the SECOND reset of the session (first at pass-35, on a genuinely new dimension; this one,
on a bookkeeping error inside that dimension's own remediation narrative).

## Part C — State at Close of Review

BC-4.17.001 **v1.16→v1.17** (F-P37-001, decision-count 1–5→1–6, 3 loci). BC-5.40.001 **v1.15→v1.16**
(F-P37-001 mirror, 3 loci, + latent bracket-balance drain 16/13→16/16). ADR-046 **v1.16 UNCHANGED**
(not touched — the defect is in the BCs' own narrative, not in ADR-046 itself). BC-7.07.001 **v1.33
UNCHANGED** (not touched, does not carry the defective narrative).

BC-5.39.001 3-CLEAN streak: **1/3 → RESETS to 0/3.** Gate history to date: 37 passes run against
evolving/frozen sets; 30 genuine findings found and fixed (29 prior + this pass's 1), plus 4
audit-extra stragglers (pass-31, pass-33)
and 1 latent-bracket drain (this pass, not counted as a genuine finding); 2 passes (34, 36) were
literal-CLEAN, both since superseded by resets (35, 37 respectively).

**NEXT: fresh pass-38** against the newly-frozen set (ADR-046 v1.16 + BC-4.17.001 v1.17 +
BC-5.40.001 v1.16 + BC-7.07.001 v1.33); needs 3 consecutive clean passes (38, 39, 40) for literal
3-CLEAN convergence. The mitigation this pass triggers — fix-burst disposition prose must be MINIMAL
and factual, no sweeping self-attested completeness certifications, and self-attested audits need
mechanical (greppable) backing — is now in force for all future ADR-046 gate remediation bursts (see
lessons.md). The human RE-AFFIRMED "CONTINUE looping toward literal 3-CLEAN" at this decision point
(accept-provisional under D-386 Option C was offered and declined again — the second such
reaffirmation this session, following the same choice at the pass-35 reset). S-17.05 TDD
implementation remains gated on convergence.
