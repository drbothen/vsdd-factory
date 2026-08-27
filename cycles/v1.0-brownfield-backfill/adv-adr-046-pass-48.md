# ADR-046 Adversarial Spec-Convergence Review — Pass 48

**Reviewed artifact set (frozen):** ADR-046 v1.18 + BC-4.17.001 v1.22 + BC-5.40.001 v1.18 + BC-7.07.001 v1.34
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED + 1 LOW observation)
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **STAYS 0/3** (already 0/3 from pass-46's reset; a finding keeps it there)
**D-chain:** D-1105

## Part A — Finding Set (frozen set: ADR-046 v1.18 + BC-4.17.001 v1.22 + BC-5.40.001 v1.18 + BC-7.07.001 v1.34)

**HIGH (0):** none this pass.

**MEDIUM (1):**

- **F-P48-001 (MED, POLICY 4, false-fabrication provenance claim).** ADR-046's own v1.18
  disposition prose — at THREE loci: the frontmatter `last_amended` field's nested v1.18 entry,
  and the `## Changelog` v1.18 row (the same claim restated at both, plus the field's own summary
  clause — 3 loci total) — claimed that the quote "MUST NOT be overridden via environment or
  arguments" (part of the pass-46 fix, F-P46-002's companion "fabricated verbatim quote"
  correction) was FABRICATED, stating it was "not verbatim-present anywhere in this repository"
  on the strength of a `grep -rn` sweep. **That sweep was mis-scoped**: it searched `.factory/`
  only and never searched `plugins/` or `crates/` — the pass-46 architect's own audit trail
  confirms the grep command used was rooted at `.factory/`, not the repository root.
  **VERIFIED** (a TRUE repo-wide `grep -rn` for the phrase, across the whole repository, not
  `.factory/`-scoped, executed this pass): the phrase IS verbatim-present in
  `plugins/vsdd-factory/bin/factory-lock-write.sh`'s `TTL_SECONDS` header comment — reading
  `TTL_SECONDS=2700 (45 minutes). MUST NOT be overridden via environment or arguments.` — an
  ADR-046 `inputs:`-listed file, and one of the three drifting-TTL-literal sources §Rationale's
  "Why the TTL must be sourced, not reinvented" (F-006) paragraph is explicitly about. The only
  other hits for the phrase repo-wide are this ADR's own two disposition loci (restating the
  false claim) plus decision-log.md/adv-adr-046-pass-46.md narrative (correctly quoting the
  false claim as a historical record of what pass-46 asserted, not asserting it themselves). **So
  the phrase was INHERITED from `factory-lock-write.sh`, not fabricated.** The v1.18
  disposition's LIVE-BODY correction itself — re-attributing AC-007 to S-17.01 and quoting
  BC-5.40.001 Invariant 2 verbatim in §Rationale/§Source-Origin — was independently re-verified
  accurate this pass and remains UNCHANGED; only the "fabricated" PROVENANCE claim attached to
  that correction's own disposition narrative was wrong.
  **Mandatory sweep for other mis-scoped absence claims performed** (in-scope, this pass): grepped
  the full document body for `grep -rn`/`grep-complete`/"not present anywhere"/"verbatim-absent"/
  "repo-wide" patterns — no other mis-scoped absence claim found; the document's other "repo-wide"
  hit is the File-Change Plan's unrelated citation, already accurate.
  Fixed by architect: both v1.18 loci (the `last_amended` field's nested v1.18 entry, and the
  Changelog v1.18 row) corrected to state the accurate provenance — inherited from
  `factory-lock-write.sh`, not fabricated — and the root cause (the pass-46 grep was mis-scoped
  to `.factory/`, never searched `plugins/`). Live-body §Rationale left unchanged (already
  accurate). ADR-046 v1.18→v1.19.

**LOW (1 observation, non-blocking, FIXED):**

- **O-P48-001 (LOW, POLICY 4, under-inclusive exhaustive-enumeration claim).** BC-7.07.001's
  Description summary sentence enumerated exit-0 conditions using the word "only" — "exits 0
  only on success/no-op/STATE.md-unreadable" — but Precondition 4's worktree-discovery-failure
  and canonicalize split-tree-mismatch paths, and Postcondition 9's hook-crash-under-
  `on_error=continue` path, ALSO exit 0 (fail-open), and were omitted from the enumeration —
  making "only" an under-inclusive exhaustive claim that contradicts the normative body.
  **Mandatory sweep for sibling under-inclusive exhaustive-enumeration claims performed**
  (in-scope, this pass): grepped this BC's Description plus every other summary/overview
  sentence for further "only"/"exclusively"/"exhaustively" exhaustive-enumeration claims and
  verified each against the normative Preconditions/Postconditions/Invariants sections — found
  ONE additional straggler of the identical defect class: Postcondition 8's own closing sentence
  ("The ONLY exit codes this hook ever produces are 0 (success, no-op, or STATE.md-unreadable
  fail-open) and 2 (...)") restates the same under-inclusive exit-0 condition list inside the
  NORMATIVE Postconditions section itself — a more serious instance than the Description summary
  since it purports to be the authoritative enumeration. All other "only"/"exclusively"
  occurrences in the body (hermetic-read scope, SkipReason variant count, malformed-condition
  scoping, etc.) were verified accurate against their governing sections — no further stragglers
  found.
  Fixed by product-owner: Description expanded to enumerate on success, no-op (clean state),
  STATE.md-unreadable, worktree-discovery-failure / split-tree-mismatch (Precondition 4), or
  hook-crash (Postcondition 9) conditions — "only" dropped. Postcondition 8's closing sentence
  expanded to the identical corrected enumeration. No PC/Invariant/EC renumbered (append-only
  numbering preserved — POLICY 1). BC-7.07.001 v1.34→v1.35.

**One MEDIUM finding and one LOW observation, both fixed same-burst. VERDICT: FINDINGS. Streak
STAYS 0/3** per BC-5.39.001's literal-3-CLEAN discipline — the streak was already at floor from
pass-46's reset; this pass's MEDIUM finding alone is sufficient to keep it there (there is no
lower floor than 0/3), and the LOW observation's fix-vs-accept disposition (D-1101 convergence-
governance rule: fix a LOW observation when the streak is already at 0/3, zero incremental cost,
fresh/in-session/sibling-confirmed-correctable) applies identically to O-P44-001's precedent.

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set
on its own merits only). Every dimension this gate's 47-pass history has previously found a
defect in was independently re-checked against the current frozen set and confirmed holding, with
zero regression beyond the two items above:

- **AC-attribution cluster-wide drain (D-1100, eighth discipline, extended D-1103/D-1104,
  eleventh discipline):** re-derived every `AC-[0-9]+` live-body citation across all four
  frozen-set artifacts — zero mis-anchors found, class remains genuinely DRAINED, no regression.
- **Byte-range/body-confinement arm-scope reconciliation (D-1096/D-1097 classes, sixth+seventh
  disciplines):** re-derived across every `extract_frontmatter`-guarantee locus — all confirmed
  correctly arm-split, no regression.
- **Record-why-not-just-checked discipline (D-1103, tenth discipline):** this pass's own audit
  records, for every locus enumerated in both sweeps (absence-claims sweep, exhaustive-
  enumeration sweep), its specific classification and reasoning — applying the discipline.
- **Grep-complete cluster-wide `inputs:` completeness (D-1090/D-1100):** re-audited all four
  artifacts' own `inputs:` arrays — zero omissions found.
- **4-leg `modified:`-array head==version parity (D-1089):** all four artifacts' `modified:`
  array heads confirmed matching their own frontmatter `version:` field, prior to this pass's
  edit.
- **Illustrative-quote verbatim-source-accuracy + sibling-parity check (D-1101, ninth
  discipline):** all three companion BCs' illustrative CAP-031/CAP-032 quotes re-derived and
  confirmed verbatim-correct against `capabilities.md` — no regression, FOURTH consecutive
  confirmation.
- **ADR §Decision/§N.M anchor correctness (D-1092/F-P35-001 class):** every citation
  independently re-derived from the cited ADR's own section content — all CORRECT.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):** recounted —
  confirmed 6 numbered decisions, correctly cited "1–6" throughout.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts — stable
  since pass-27 (22 consecutive passes now, counting this one). **Neither of this pass's two
  items touches the behavioral core — both are confined to disposition-prose/summary-prose
  accuracy.**
- **Every load-bearing code claim (function names, file paths, constant names) independently
  re-verified against the actual source files:** all accurate, including the newly-verified
  `factory-lock-write.sh` TTL_SECONDS header comment provenance (F-P48-001).
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) prior to this burst's edit; two artifacts edited this pass (ADR-046, BC-7.07.001),
  expected to re-trigger the cyclic tangle — settled per state-manager's usual disposition, not
  force-converged.

**Novelty assessment:** both items this pass belong to defect classes NOT previously identified
at this gate — neither is a recurrence of an existing codified discipline. **F-P48-001** is the
first instance at this gate of a fix's own disposition prose containing a false ABSENCE
("fabricated"/"not present anywhere") claim caused by an under-scoped grep (`.factory/`-only
instead of repository-wide) — a distinct failure mode from the AC-attribution or verbatim-quote-
misattribution classes, though it shares their "the remediation's own prose is itself attack
surface" character (see the META pattern below). **O-P48-001** is the first instance of an
under-inclusive SUMMARY/OVERVIEW exhaustive-enumeration claim ("only"/"exclusively") that omits
normatively-documented exit paths — distinct from the AC-attribution and byte-range classes.
Both are, however, the FOURTH and FIFTH instances of a recurring META-pattern first observed at
pass-37 (F-P37-001 decision-count) and pass-44 (O-P44-001 misattributed quote): **the fix's OWN
disposition/changelog/summary prose is itself attack surface** — every factual claim a
remediation's own narrative makes must be independently verified, not merely assumed correct
because it "sounds right" or restates a prior pass's assertion. Per BC-5.39.001, the MEDIUM
finding (F-P48-001) keeps the streak at 0/3 regardless of the LOW observation's independent
disposition.

## Part C — State at Close of Review

ADR-046 **v1.18→v1.19** (F-P48-001 fix, architect). BC-4.17.001 **v1.22 UNCHANGED** (not
implicated this pass). BC-5.40.001 **v1.18 UNCHANGED** (not implicated this pass). BC-7.07.001
**v1.34→v1.35** (O-P48-001 fix, product-owner). BC-5.39.001 3-CLEAN streak: **0/3 → STAYS 0/3**
(the MEDIUM finding keeps it there; already at floor from pass-46's reset). Gate history to date:
48 passes run against evolving/frozen sets; 38 genuine findings found and fixed (adding
F-P48-001), plus 4 audit-extra stragglers (pass-31, pass-33) and 1 latent-bracket drain (pass-37,
not counted as genuine), 1 ACCEPTED non-blocking observation (O-P42-001, does not count against
the streak), and 2 FIXED non-blocking observations (O-P44-001, governance-elected fix at zero
streak cost; O-P48-001, this pass, same disposition rule applied); 6 clean passes to date (34,
36, 38, 41, 42, 45).

**Index reconciliation (state-manager, this burst):** ARCH-INDEX v3.88→v3.89 (ADR-046 row
version-chain cell appended, pass-47-unchanged note + pass-48 fix note). BC-INDEX v5.12→v5.13
(BC-7.07.001 row version-chain cell appended, v1.35 entry).

**NEXT: fresh pass-49** against the newly-frozen set (ADR-046 v1.19 + BC-4.17.001 v1.22 +
BC-5.40.001 v1.18 + BC-7.07.001 v1.35), starting a new streak at 0/3, applying all eleven
now-codified convergence-technique disciplines proactively from the start, plus the two new
disciplines this pass produces: (12) VERBATIM-ABSENCE claims must be backed by a TRUE
repository-wide grep, stating scope explicitly; (13) SUMMARY-ENUMERATION claims ("only"/
"exclusively"/exhaustive) must be swept within-artifact against the normative body. The human
decision this session remains to CONTINUE looping toward literal 3-CLEAN (not accept D-386
Option C asymptotic acceptance). S-17.05 TDD implementation remains gated on convergence.
