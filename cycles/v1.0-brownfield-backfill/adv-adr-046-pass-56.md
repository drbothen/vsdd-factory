# ADR-046 Adversarial Spec-Convergence Review — Pass 56

**Reviewed artifact set (frozen):** ADR-046 v1.22 + BC-4.17.001 v1.24 + BC-5.40.001 v1.20 + BC-7.07.001 v1.37
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED) — F-P56-001, FIXED (whole class)
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 1/3 → **RESETS to 0/3** — the SEVENTH streak reset this session, but the most substantive finding of the entire convergence effort (a genuine spec-vs-code behavioral divergence, not a citation/provenance-layer defect)
**D-chain:** D-1113

## Part A — Finding Set (frozen set: ADR-046 v1.22 + BC-4.17.001 v1.24 + BC-5.40.001 v1.20 + BC-7.07.001 v1.37)

**HIGH (0):** none this pass.

**MEDIUM (1):**

- **F-P56-001 (MED, POLICY 15 spec-vs-code + POLICY 4 internal-consistency).** ADR-046 and both
  companion BCs (BC-4.17.001, BC-7.07.001) mischaracterized an empty-string or absent `holder`
  sub-field as equivalent to the pre-existing, unnumbered **0th case** (`factory_lock:` fully absent
  or fully null — "no lock held," `Ok((RenewOutcome::NoOp, None))`, "treated identically to absent
  block," described as "inherited from `renew_lock`'s existing presence-precheck."

  **Ground truth**, confirmed by direct inspection of `crates/factory-lock-parse/src/lib.rs`'s
  `parse_factory_lock` (architect):
  - `Ok(None)` is returned ONLY when the `factory_lock:` block is fully absent from the frontmatter
    (line 206-208), OR present but fully null/empty with **no sub-fields at all** — no `holder`, no
    `locked_at`, no `expires_at` (line 211-213).
  - Once ANY sub-field is present, the function proceeds to validate `holder` (lines 216-226): an
    empty-string `holder` (`Some(h) if h.is_empty()`) returns
    `Err(MalformedLockBlock("factory_lock.holder is empty string"))`; an absent `holder` with other
    sub-fields present (`None` arm) returns
    `Err(MalformedLockBlock("factory_lock.holder field is absent"))`. Neither degenerate-holder case
    ever reaches `Ok(None)`.
  - `crates/factory-lock/src/lib.rs`'s `renew_lock_with_now` (the actual call site cited by both
    BCs) confirms the mapping: Step 1's `has_factory_lock_key` presence pre-check tests ONLY the
    literal `factory_lock:` key line — it does NOT inspect `holder`'s value at all, so the
    "inherited from `renew_lock`'s existing presence-precheck" grounding in ADR-046's own text was
    FALSE. Step 2 maps the resulting `Err(MalformedLockBlock)` to `Err(LockError::Malformed)` —
    case 1, never `NoOp`.
  - A dedicated existing unit test, `test_BC_5_40_001_parse_factory_lock_errors_on_empty_holder`
    (`crates/factory-lock-parse/src/lib.rs` lines 567-579), independently corroborates: it asserts
    `Err(MalformedLockBlock(_))` for an empty-string holder and explicitly panics if the parser were
    to return `Ok(None)` instead.

  **Loci corrected (whole class, across ADR + 2 companion BCs):**
  - **ADR-046 (architect, v1.22→v1.23):** §Decision 1(b)'s "Holder-present check" bullet and its
    canonical five-case table's 0th-case parenthetical — both narrowed the `Ok(None)` 0th case to
    cover ONLY a fully-absent-or-fully-null block (no sub-fields at all); the false
    "inherited from `renew_lock`'s existing presence-precheck" / "unchanged from today's
    `renew_lock`" grounding struck; empty-string or absent-with-siblings-present `holder` now
    explicitly routed to case 1 (`Err(LockError::Malformed)`).
  - **BC-4.17.001 (product-owner, v1.24→v1.25):** PC2's 0th-case bullet and case-1 bullet in the
    five-case return-value table, EC-011, the Canonical Test Vector for `holder: ""`, and PC3b's
    non-goal event-suppression list — all previously mischaracterized empty/absent-holder as
    equivalent to a true absent block (a stale F-015 disposition); corrected to route to case 1.
  - **BC-7.07.001 (product-owner, 2 rounds, v1.37→v1.38→v1.39):**
    - Round 1 (v1.38): PC3, Invariant 3 (execution-order branch), and the new Invariant 3b —
      corrected the same 0th-case/case-1 boundary in this BC's own canonical five-case enumeration
      and its `renew_lock_if_holder` decision-tree narrative.
    - Round 2 (v1.39, cluster re-check straggler): EC-009's condition cell had NOT been swept in
      round 1 — it still read "or `factory_lock.holder` is null (no lock held)," a THIRD distinct
      degenerate-holder sub-case (an explicit `holder: null` YAML token) that the round-1 sweep
      missed. Ground truth (architect-confirmed): `extract_yaml_string_value` has NO special-case
      for the bare `null` token, so `holder: null` yields the literal 4-char string `"null"`, NOT
      absence/empty — never the 0th case. EC-009 corrected to the same phrasing already adopted at
      v1.38 for PC3/Invariant 3/Invariant 3b/EC-004. **New EC-011 added** (append-only, no
      renumbering) documenting the `holder: null` quirk: resolves to case 1 (`Malformed`, EC-004) if
      `locked_at`/`expires_at` are absent, or to a genuinely-held lock with `holder` literally
      `"null"` (renewal-eligible, cases 1-5) if `locked_at`/`expires_at` are present and valid —
      never the EC-009 0th case. A cluster re-check of this BC's live body for remaining
      holder-null/empty/absent → no-lock-held/0th/no-op mischaracterizations found no further
      stragglers; EC-009 was the last one.
  - **BC-5.40.001 (v1.20, UNCHANGED):** cluster-checked CLEAN — its "malformed→unlocked" language
    describes `verify-factory-lock`'s own distinct admit-time behavior at a different call site, not
    an instance of this class.

  **Disposition:** still behaviorally benign in all three degenerate-holder sub-cases (no renewal,
  no `exec_subprocess`/identity check, no write) — but the specs previously claimed this reached the
  0th case's silent `NoOp` path, when the actual code path is case 1's advisory `log_warn`. A
  hypothetical future reader trusting the spec's "identical to absent block" claim could reasonably
  conclude no diagnostic signal is ever emitted for a malformed `holder`; the code in fact emits one.
  No PC/Invariant/EC renumbered anywhere (append-only numbering preserved — POLICY 1); EC-011 is a
  new ID, not a reuse.

**Zero LOW/informational observations this pass** beyond the single MED finding above (fixed across
its whole class, including the round-2 EC-009/EC-011 straggler).

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings beyond F-P56-001)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set on
its own merits only). F-P56-001 was found via an edge-case lens no prior pass (1 through 55) had
applied: independently re-deriving the parser's ACTUAL `Ok(None)` vs `Err(Malformed)` vs `Ok(Some)`
partition for every degenerate `holder` sub-value, rather than accepting the spec's own narrative
characterization of "no lock held" at face value. Every dimension this gate's 55-pass history had
previously found a defect in was independently re-checked against the current frozen set and
confirmed holding, with zero regression beyond F-P56-001:

- **STEP-NUMBER CITATION (sixteenth discipline, D-1111, re-confirmed a second independent time):**
  all four `verify-state-timestamp-refresh` step-number loci in ADR-046 re-verified correct ("Step
  8" lock-expiry arm, "Steps 4-7" timestamp arm) — no regression.
- **Illustrative "analogous to T-NNN" enumeration accuracy (ninth discipline, D-1101, extended
  D-1108):** re-verified — still reads "analogous to T-001/T-004/T-005/T-007." Holds.
- **Behavioral core (write-composition table, identity-gating logic, event-sourcing struct-variant
  text) OUTSIDE the 0th-case/case-1 boundary itself:** re-verified CLEAN — stable since pass-27.
- **AC-attribution cluster-wide drain (eighth/eleventh disciplines):** re-derived every `AC-[0-9]+`
  live-body citation — zero mis-anchors, class remains DRAINED.
- **CITATION→INPUT PARITY (fourteenth discipline) / catalog-membership-verification (fifteenth
  discipline):** no new citation-without-input or catalog-membership gaps found; F-P56-001's fix
  cited no new source requiring an `inputs:` entry.
- **BC-7.07.001 §Description "no-op" phrasing (O-P53-DESC-NOOP):** re-examined — remains DEFENSIBLE,
  ACCEPTED-tracked, untouched by this pass's fix (a different locus — the Description's plain-English
  summary, not the PC2/EC-009/EC-011 normative tables).
- **BC-5.40.001 `modified:` array v1.1-v1.4 disposition-prose gap (O-P42-001):** re-examined —
  remains ACCEPTED-tracked, untouched.

**Novelty assessment:** F-P56-001 is a NEW distinct finding class — the FIRST genuine spec-vs-code
BEHAVIORAL divergence this gate has caught since the behavioral core itself stabilized at pass-27 (29
consecutive clean re-verifications of the core, now broken by this one narrow-but-real edge-case
defect). Every prior blocking finding since pass-27 was confined to the provenance/citation/
traceability/metadata perimeter (cross-reference accuracy, `inputs:` completeness, catalog
membership, step-number citation, illustrative-enumeration accuracy). F-P56-001 is different in
kind: it is a claim about WHAT THE CODE ACTUALLY DOES in a specific input state (empty/absent/null
`holder`), and the claim was FALSE — verified false by direct inspection of the parser's match arms
and corroborated by an existing unit test. This defect survived 55 prior passes because no prior
adversary had independently re-derived the parser's degenerate-holder-value partition from its
actual source match arms; all prior passes accepted the spec's own "0th case, no lock held" framing
of the empty/absent-holder condition without cross-checking it against `parse_factory_lock`'s literal
match logic. CODIFIED as a process discipline (`[codified][process-gap]`) in `lessons.md`: any "0th
case" / "no lock held" / `NoOp` claim about a degenerate or missing field value MUST be verified
against the actual parser's `Ok`/`Err` partition, with every degenerate sub-case (empty-string,
absent-with-siblings-present, explicit `null` token, and whitespace) traced individually — not
inferred from the field's ABSENCE alone. The class required 2 sweep rounds this burst (empty/absent
first at BC-7.07.001 v1.38, then the EC-009/`holder: null` straggler at v1.39) — reinforcing that
exhaustive per-sub-case enumeration, not a single representative case, is required for this
discipline.

## Part C — State at Close of Review

ADR-046 **v1.22→v1.23** (F-P56-001 fix, architect). BC-4.17.001 **v1.24→v1.25** (F-P56-001 fix,
product-owner). BC-7.07.001 **v1.37→v1.39** (F-P56-001 fix, product-owner, 2 rounds — v1.38 for the
primary empty/absent-holder correction, v1.39 for the EC-009 null-holder straggler + new EC-011).
BC-5.40.001 **v1.20, UNCHANGED** (cluster-checked CLEAN — no instance of this class). BC-5.39.001
3-CLEAN streak: **1/3 → RESETS to 0/3** — the SEVENTH reset this session, but qualitatively distinct
from every prior reset: this is the first BLOCKING finding since pass-27 that touches genuine
behavioral substance (what the code does in a specific input state) rather than the
provenance/citation/metadata perimeter. Gate history to date: 56 passes run against evolving/frozen
sets; 43 genuine BLOCKING findings found and fixed (42 through pass-55, plus F-P56-001), plus 10
audit-extra stragglers (pass-31, pass-33, pass-49 x6), 1 latent-bracket drain (pass-37, not counted
as genuine), 2 ACCEPTED non-blocking observations (O-P42-001, O-P53-DESC-NOOP, neither counts against
the streak), and 3 FIXED non-blocking observations (O-P44-001, O-P48-001, O-P51-001). The behavioral
core (write-composition, five-outcome table, identity-gating, event-sourcing) OUTSIDE the
0th-case/case-1 boundary remains independently re-verified CLEAN; this is the first genuine crack in
what had otherwise been a 29-consecutive-pass-stable core.

**Index reconciliation (state-manager, this burst):** ARCH-INDEX **v3.92→v3.93** (ADR-046 row bumped
v1.22→v1.23; version-stable read-through convention preserved). BC-INDEX **v5.15→v5.16**
(BC-4.17.001 row v1.24→v1.25; BC-7.07.001 row v1.37→v1.39). STORY-INDEX v4.392, VP-INDEX v2.79 both
**UNCHANGED** (no story/VP edit this pass).

**Input-hash recompute (cyclic-hash TD [D-1082] — settled + cross-referenced, NOT reopened):**
`compute-input-hash --update` run for ADR-046, BC-4.17.001, BC-7.07.001 in edit order (ADR-046 first,
per this burst's actual edit sequence, then BC-4.17.001, then BC-7.07.001 last):
- ADR-046: `cb428ff` → `3335ad4` (1-hop residual accepted — later-edited BC-4.17.001/BC-7.07.001
  changes feed back into ADR-046's own computed hash per the established [D-1082] cyclic tangle; not
  re-chased).
- BC-4.17.001: `0edc756` → `b7f7213` (1-hop residual accepted — BC-7.07.001, edited after it, feeds
  back; not re-chased).
- BC-7.07.001: `673078a` → `e73bc01` (**SETTLED** — last-edited artifact this burst's edit order;
  confirmed via `compute-input-hash --check` exit 0).

**EC-011 consistency check (this burst's task item 3):** `grep -n "EC-011"` against BC-7.07.001's
body confirms exactly ONE definition locus (the new §Edge Cases table row) — no collision with any
pre-existing EC-011 ID. This BC has no `## Token Budget` section and no explicit "EC count" field
anywhere in its frontmatter or body to reconcile against the EC-011 addition — nothing to flag for
product-owner follow-up; consistent by omission.

**NEXT: fresh pass-57** against the newly-frozen set (ADR-046 v1.23 + BC-4.17.001 v1.25 +
BC-5.40.001 v1.20 + BC-7.07.001 v1.39), starting a new streak at 0/3, applying all sixteen
now-codified convergence-technique disciplines proactively from the start, PLUS the newly-codified
seventeenth discipline (0TH-CASE/NO-OP CLAIM VERIFICATION): any "0th case" / "no lock held" / `NoOp`
claim about a degenerate or missing field value MUST be verified against the actual parser's
`Ok`/`Err` partition, with every degenerate sub-case traced individually. The human decision this
session remains to CONTINUE looping toward literal 3-CLEAN (not accept D-386 Option C asymptotic
acceptance) — accept-provisional under D-386 Option C remains available at any time. S-17.05 TDD
implementation remains gated on convergence.
