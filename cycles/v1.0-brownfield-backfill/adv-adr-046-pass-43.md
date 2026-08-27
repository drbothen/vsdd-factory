# ADR-046 Adversarial Spec-Convergence Review — Pass 43 (the CONVERGENCE pass)

**Reviewed artifact set (frozen):** ADR-046 v1.16 + BC-4.17.001 v1.19 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33
**Review date:** 2026-08-27
**Verdict:** FINDINGS (2 MED) + 2 observations (O-P43-001 LOW, fixed; O-P43-002 informational, no action)
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 2/3 → **RESETS to 0/3** (4th reset this session)
**D-chain:** D-1100

## Part A — Finding Set (frozen set: ADR-046 v1.16 + BC-4.17.001 v1.19 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33)

**HIGH (0):** none this pass.

**MEDIUM (2):**

- **F-P43-001 (MED, POLICY 18, inputs: completeness).** All three companion BCs
  (BC-4.17.001, BC-5.40.001, BC-7.07.001) quote their respective capability's description
  verbatim in their Capability Anchor Justification / Traceability section against
  `.factory/specs/domain-spec/capabilities.md` (CAP-031 for BC-4.17.001/BC-5.40.001,
  CAP-032 for BC-7.07.001), but none of the three BCs' `inputs:` frontmatter arrays listed
  that file — a load-bearing current-state citation with no `inputs:` entry, the same
  parity-gap character this gate has repeatedly found on ADR-046's own `inputs:` array
  (F-P28-001, F-P30-002, F-P33-001) but never before checked on the BCs' own arrays. A
  MANDATORY grep-complete inputs audit of all three BCs (not a read-through — enumerating
  every `.factory/specs/*.md`, `crates/*.rs`, `plugins/*.{sh,toml}` citation in each BC's
  body and cross-checking against its own `inputs:` array) found `capabilities.md` missing
  from all three, plus — on BC-5.40.001 specifically — three FURTHER genuinely-cited-and-
  missing files that had never been swept: `plugins/vsdd-factory/bin/factory-lock-write.sh`
  (Postcondition 4's break-glass-fallback paragraph makes a load-bearing current-behavior
  claim about `factory-lock-write.sh renew`), `plugins/vsdd-factory/hooks/verify-git-push.sh`
  (Precondition 5/Invariant 5's `--force-with-lease`-permitted claim), and
  `crates/hook-plugins/verify-state-timestamp-refresh/tests/integration_t006_no_output_too_large.rs`
  (the §VP Anchors literal grep-evidence block quotes this file's test function names
  verbatim, distinct from the already-listed `src/lib.rs`). BC-4.17.001 and BC-7.07.001's
  own audits found no other missing citations beyond `capabilities.md`.

- **F-P43-002 (MED, POLICY 4, cross-reference integrity).** ADR-046's Companion Amendment 3
  closing sentence referred to "The BC's existing AC-018 ('If `Renewed`: call
  `host::write_file`')" as though AC-018 were a normative acceptance-criterion section of
  BC-7.07.001 itself. A full section-heading sweep of BC-7.07.001 (Description /
  Preconditions / Postconditions / Invariants / Edge Cases / Canonical Test Vectors /
  Related BCs / Architecture Anchors / Story Anchor / VP Anchors / Verification
  Properties / Traceability / Changelog) confirms BC-7.07.001 has no Acceptance Criteria
  section and no AC-NNN numbering scheme at all. AC-018 is in fact a STORY-level
  acceptance criterion of `.factory/stories/S-18.04a-precompact-flush-sh-core.md`
  (`### AC-018 (traces to ADR-028 §Decision 2 + §Decision 9 + BC-7.07.001 PC3 — native
  lock renewal...)`), itself tracing to BC-7.07.001's own Postcondition 3 case 5
  (`Ok((RenewOutcome::Renewed(new_content), None))`) and Invariant 3 step 4 (`if
  RenewOutcome::Renewed: write updated STATE.md via host write_file`). The "AC-018 ...
  UNCHANGED" phrasing originates verbatim from BC-7.07.001's own v1.19 Changelog row, so it
  was not fabricated by ADR-046 — only mis-scoped as a BC-internal reference where none
  resolves, and BC-7.07.001's own v1.19 narrative independently carries the identical
  mis-scoping (the mirror was two-way, not a one-directional ADR error).

**LOW (0 blocking; 1 non-blocking observation fixed, 1 informational):**

- **O-P43-001 (LOW, stale volatile pin, fixed).** BC-4.17.001's Invariant 6 carried a
  stale BC-to-BC version pin, `(see BC-5.40.001 v1.5 Invariant 1)` — BC-5.40.001 had long
  since outgrown v1.5 (at v1.16 at the time of this review), a POLICY 19-class anti-pattern
  the sibling-sweep discipline (D-1096/D-1097) had not previously checked for cross-BC
  version pins specifically. Stripped to the stable anchor `(see BC-5.40.001 §Invariant
  1)`. A sweep of BC-4.17.001 for other live-body BC-to-BC version pins found none — the
  two other `BC-X.YY.NNN vN.N`-shaped matches in the file are both confined to dated
  historical `modified:`/Changelog narrative (POLICY 1 append-only, not live-body).

- **O-P43-002 (informational, no action).** No ADR-046 content is implicated by either
  finding above — both F-P43-001 and O-P43-001 are BC-only, and F-P43-002, while requiring
  an ADR-046 edit, is a cross-reference-attribution correction rather than a design-content
  change. Recorded for completeness of the observation ledger only; no artifact edit
  attaches to this entry beyond what F-P43-001/F-P43-002/O-P43-001 already require.

**Zero HIGH findings. 2 MEDIUM findings (both fixed same-burst). VERDICT: FINDINGS. Streak
RESETS 2/3 → 0/3 per BC-5.39.001's literal-3-CLEAN discipline — any BLOCKING finding resets
the streak regardless of severity class.**

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact
set on its own merits only). Every dimension this gate's 42-pass history has previously
found a defect in was independently re-checked against the current frozen set and confirmed
holding, with zero regression:

- **Arm-parity what-vs-how reconciliation + locus-class-extension (D-1096/D-1097 classes,
  sixth+seventh disciplines):** all eleven `extract_frontmatter`-guarantee loci across
  BC-4.17.001 re-derived and confirmed consistently arm-split — no regression, THIRD
  consecutive confirmation (following pass-41, pass-42).
- **ADR §Decision/§N.M anchor correctness (D-1092/F-P35-001 class):** every citation
  independently re-derived from the cited ADR's own section content — all CORRECT.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):**
  recounted — confirmed 6 numbered decisions, correctly cited "1–6" throughout.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts —
  stable since pass-27 (17 consecutive passes now, counting this one). **This pass's two
  findings are confined entirely to the provenance/cross-reference perimeter — neither
  touches the behavioral core.**
- **Every load-bearing code claim (function names, file paths, constant names)
  independently re-verified against the actual source files:** all accurate.
- **`modified:`-array-head-parity (4-leg head==version self-check, D-1089):** all four
  artifacts confirmed prior to this pass's edits.
- **`last_amended` bracket-balance, cross-anchor citation accuracy, type-provenance,
  POLICY 19 anti-volatile-pin (beyond the O-P43-001 instance), §Story Anchor/Traceability
  parity, subsystem labels, status/lifecycle pairs:** all re-verified clean — no regression
  on any previously-codified dimension beyond the two findings and one observation above.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) prior to this burst's edits.

**Novelty assessment:** this pass's findings are a NEW dimension for this gate — the first
time the mandatory grep-complete inputs audit (D-1090) has been applied to the three BCs'
own `inputs:` arrays rather than only ADR-046's. That the BCs' own capabilities.md citation
had survived 42 prior passes unaudited is itself evidence the audit-scope needed explicit
extension — codified this burst (see lessons.md). F-P43-002's AC-018 mis-scoping is a
genuine cross-reference defect but of the same citation-accuracy character as the pass-35
reset (not a behavioral-core, data-destructive defect like pass-39). **Per BC-5.39.001, any
BLOCKING finding — regardless of severity or class — resets the streak. This is the 4th
reset this session** (after pass-35, pass-37, pass-39), and the 2nd at a
provenance/traceability perimeter rather than the core design (pass-35 was also
citation-accuracy class; pass-37 was bookkeeping; pass-39 was the sole substantive
data-destructive reset).

## Part C — State at Close of Review

ADR-046 **v1.16→v1.17** (F-P43-002 fix, architect). BC-4.17.001 **v1.19→v1.20** (F-P43-001 +
O-P43-001, product-owner). BC-5.40.001 **v1.16→v1.17** (F-P43-001 + 3 audit-extra inputs,
product-owner). BC-7.07.001 **v1.33→v1.34** (F-P43-001 + F-P43-002 mirror, product-owner).
BC-5.39.001 3-CLEAN streak: **2/3 → RESETS to 0/3** (4th reset this session). Gate history to
date: 43 passes run against evolving/frozen sets; 34 genuine findings found and fixed, plus
4 audit-extra stragglers (pass-31, pass-33) and 1 latent-bracket drain (pass-37, not counted
as genuine), 1 ACCEPTED non-blocking observation (O-P42-001, does not count against the
streak), and 1 fixed non-blocking observation this pass (O-P43-001); 5 clean passes (34, 36,
38, 41, 42).

**Index reconciliation (state-manager, this burst):** ARCH-INDEX v3.86→v3.87 (ADR-046 row
version cell + narrative). BC-INDEX v5.08→v5.09 (three BC row version-chain cells
appended). Input-hash recomputed for all four frozen-set artifacts (final stored values:
ADR-046 8f11d0e, BC-4.17.001 39fa054, BC-5.40.001 b711178, BC-7.07.001 d4b0881 — BC-7.07.001
settled exactly since it was the last file edited this burst; the other three carry an
accepted 1-hop residual drift per the cyclic-hash TD [D-1082] convention, NOT chased
further).

**NEXT: fresh pass-44** against the newly-frozen set (ADR-046 v1.17 + BC-4.17.001 v1.20 +
BC-5.40.001 v1.17 + BC-7.07.001 v1.34), starting a new streak at 0/3, applying all seven
now-codified convergence-technique disciplines proactively PLUS the newly-extended
grep-complete-BC-inputs-audit discipline (this pass's own codification) from the start. The
human decision this session remains to CONTINUE looping toward literal 3-CLEAN (not accept
D-386 Option C asymptotic acceptance). S-17.05 TDD implementation remains gated on
convergence.
