# ADR-046 Adversarial Spec-Convergence Review — Pass 47

**Reviewed artifact set (frozen):** ADR-046 v1.18 + BC-4.17.001 v1.21 + BC-5.40.001 v1.18 + BC-7.07.001 v1.34
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED)
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **STAYS 0/3** (already 0/3 from pass-46; a finding keeps it there)
**D-chain:** D-1104

## Part A — Finding Set (frozen set: ADR-046 v1.18 + BC-4.17.001 v1.21 + BC-5.40.001 v1.18 + BC-7.07.001 v1.34)

**HIGH (0):** none this pass.

**MEDIUM (1):**

- **F-P47-001 (MED, POLICY 4, cross-reference integrity).** BC-4.17.001 Invariant 3's own
  parenthetical — "This BC does not change the TTL value itself (BC-5.40.001 Invariant 2/AC-007
  — 2700 seconds, non-configurable — is UNCHANGED)" — carried the identical mis-scoping pattern
  the pass-46 fix (F-P46-002) already corrected on ADR-046's own AC-007 citation: it presented
  AC-007 as though it were an acceptance criterion belonging to BC-5.40.001 itself, when
  BC-5.40.001 has no Acceptance Criteria section and no AC-NNN numbering scheme at all (this BC
  has no `## Acceptance Criteria` heading; confirmed by a full section-heading sweep of
  BC-5.40.001). AC-007 is in fact a STORY-level acceptance criterion of
  `.factory/stories/S-17.01-factory-lock-schema-cas-push.md`, tracing to BC-5.40.001 Invariant 2.
  The pass-46 fix corrected ADR-046's own two AC-007 loci but did not trigger a cluster-wide
  sweep of the OTHER two companion BCs for the same pattern — this BC's own live-body AC-007
  citation survived unaudited into this pass.
  **Mandatory cluster-wide exhaustive live-body AC-reference audit performed** (in-scope, this
  pass, extending the pass-43/pass-46 single-artifact-scoped audits to all three cluster BCs at
  once): every `AC-[0-9]+` token in the live body (Preconditions/Postconditions/Invariants/Edge
  Cases/Verification Properties/Traceability/Architecture Anchors/Description — excluding dated
  historical `modified:`/`last_amended:`/`## Changelog` narrative, POLICY 1 append-only, out of
  scope) of BC-4.17.001, BC-5.40.001, and BC-7.07.001 was enumerated and verdicted. BC-4.17.001:
  ONE live-body hit — Invariant 3's AC-007, the finding above. BC-5.40.001: SIX AC-NNN hits
  found, all either dated historical narrative (its own `last_amended`, Changelog v1.4 row) or
  live-body `§Verification Properties`/`§VP Anchors` rows already correctly scoped to their
  owning story S-19.08 (AC-001..AC-005, cross-checked against S-19.08's own AC table) — no
  mis-anchor found, no edit made. BC-7.07.001: FOUR AC-018 hits found, all dated historical
  narrative (`last_amended`, `modified:`, Changelog v1.34/v1.19 rows) already correctly resolved
  to S-18.04a at pass-43 (F-P43-002) — zero live-body AC-018 occurrences exist outside historical
  narrative, no edit made. **BC-4.17.001's Invariant 3 was the ONLY remaining live-body
  mis-anchor across all three cluster BCs — the AC-attribution class is now DRAINED
  cluster-wide** (ADR-046 drained at pass-46; all three companion BCs drained this pass). Fixed
  same-burst by product-owner: Invariant 3's parenthetical corrected from "(BC-5.40.001
  Invariant 2/AC-007 — 2700 seconds, non-configurable — is UNCHANGED)" to "(BC-5.40.001
  §Invariant 2 — 2700 seconds, non-configurable; also S-17.01's AC-007 — is UNCHANGED)",
  mirroring the pass-46 ADR-046 remedy exactly: AC-007 attributed to its owning story S-17.01,
  BC-5.40.001 §Invariant 2 retained as the BC anchor. No PC/Invariant/EC renumbered (append-only
  numbering preserved — POLICY 1). BC-4.17.001 v1.21→v1.22.

**LOW (0):** none this pass — no observations of any kind, blocking or non-blocking.

**One MEDIUM finding (fixed same-burst). VERDICT: FINDINGS. Streak STAYS 0/3 per BC-5.39.001's
literal-3-CLEAN discipline — the streak was already 0/3 from pass-46's reset; this pass's
finding keeps it there rather than a further reset (there is no lower floor than 0/3).**

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set
on its own merits only). Every dimension this gate's 46-pass history has previously found a
defect in was independently re-checked against the current frozen set and confirmed holding,
with zero regression beyond the one finding above:

- **Byte-range/body-confinement arm-scope reconciliation (D-1096/D-1097 classes, sixth+seventh
  disciplines, drained at pass-46):** re-derived across every `extract_frontmatter`-guarantee
  locus in all four artifacts — all confirmed correctly arm-split, no regression.
- **Record-why-not-just-checked discipline (D-1103, tenth discipline, NEW at pass-46):** this
  pass's own audit records, for every locus enumerated in the AC-reference sweep, its specific
  classification (mis-scoped-fix-needed vs. correctly-scoped-historical vs.
  correctly-scoped-live-body) and the reasoning — applying the discipline the pass-46 finding
  itself produced.
- **Illustrative-quote verbatim-source-accuracy + sibling-parity check (D-1101, ninth
  discipline):** all three companion BCs' illustrative CAP-031/CAP-032 quotes re-derived and
  confirmed verbatim-correct against `capabilities.md` — no regression, THIRD consecutive
  confirmation (following pass-45, pass-46).
- **ADR §Decision/§N.M anchor correctness (D-1092/F-P35-001 class):** every citation
  independently re-derived from the cited ADR's own section content — all CORRECT.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):** recounted —
  confirmed 6 numbered decisions, correctly cited "1–6" throughout.
- **Grep-complete cluster-wide `inputs:` completeness (D-1090/D-1100):** re-audited all four
  artifacts' own `inputs:` arrays — zero omissions found.
- **4-leg `modified:`-array head==version parity (D-1089):** all four artifacts' `modified:`
  array heads confirmed matching their own frontmatter `version:` field, prior to this pass's
  edit.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts — stable
  since pass-27 (21 consecutive passes now, counting this one). **This pass's one finding is
  confined entirely to the provenance/cross-reference perimeter — it does not touch the
  behavioral core.**
- **Every load-bearing code claim (function names, file paths, constant names) independently
  re-verified against the actual source files:** all accurate.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) prior to this burst's edit; only BC-4.17.001 edited this pass, so no cyclic
  re-triggering expected.

**Novelty assessment:** the finding this pass is a genuinely NEW instance of the already-codified
AC-owning-artifact cross-reference discipline (D-1100, eighth discipline, extended at D-1103) —
not an eleventh discipline. It is the direct CLUSTER-SIBLING of the pass-46 fix (F-P46-002):
both pass-43 (BC-7.07.001/AC-018) and pass-46 (ADR-046/AC-007) audits were SINGLE-ARTIFACT-
scoped, each fixing only the one artifact its own finding named rather than sweeping every
cluster artifact in the same burst — so BC-4.17.001's own live-body AC-007 mis-anchor (this
pass's F-P47-001) survived unaudited through both prior passes. This pass's mandatory
CLUSTER-WIDE audit (all three companion BCs swept in one pass) is the discipline this recurring
pattern demands: any class-draining grep audit must sweep every cluster artifact in the SAME
burst, not just the artifact where the finding originally surfaced. Per BC-5.39.001, any
BLOCKING finding — regardless of severity or class — keeps the streak at (or resets it to) 0/3;
since the streak was already 0/3 entering this pass, this finding does not lower it further —
it simply STAYS at 0/3. This pattern (unswept-sibling class stragglers surfacing at the
convergence pass itself) is now confirmed across THREE instances (pass-43, pass-46, pass-47) —
continued substantive progress (the AC-attribution class is now genuinely cluster-wide DRAINED),
not gaming.

## Part C — State at Close of Review

ADR-046 **v1.18 UNCHANGED** (no finding routed to it this pass). BC-4.17.001 **v1.21→v1.22**
(F-P47-001 fix, product-owner). BC-5.40.001 **v1.18 UNCHANGED** (audited, no mis-anchor found).
BC-7.07.001 **v1.34 UNCHANGED** (audited, no mis-anchor found). BC-5.39.001 3-CLEAN streak: **0/3
→ STAYS 0/3** (a finding keeps it there; already at floor from pass-46's reset). Gate history to
date: 47 passes run against evolving/frozen sets; 37 genuine findings found and fixed, plus 4
audit-extra stragglers (pass-31, pass-33) and 1 latent-bracket drain (pass-37, not counted as
genuine), 1 ACCEPTED non-blocking observation (O-P42-001, does not count against the streak), and
1 FIXED non-blocking observation (O-P44-001, governance-elected fix at zero streak cost); 6 clean
passes to date (34, 36, 38, 41, 42, 45).

**Index reconciliation (state-manager, this burst):** BC-INDEX v5.11→v5.12 (BC-4.17.001 row
version-chain cell appended). ARCH-INDEX v3.88 UNCHANGED (ADR-046 not touched this pass).

**NEXT: fresh pass-48** against the newly-frozen set (ADR-046 v1.18 + BC-4.17.001 v1.22 +
BC-5.40.001 v1.18 + BC-7.07.001 v1.34), starting a new streak at 0/3, applying all ten
now-codified convergence-technique disciplines proactively from the start, plus the NEW eleventh
discipline this pass produces (cluster-wide-not-single-artifact audit scope) — with the
AC-attribution class now confirmed genuinely DRAINED cluster-wide across all four frozen-set
artifacts. The human decision this session remains to CONTINUE looping toward literal 3-CLEAN
(not accept D-386 Option C asymptotic acceptance). S-17.05 TDD implementation remains gated on
convergence.
