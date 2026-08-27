# ADR-046 Adversarial Spec-Convergence Review — Pass 54

**Reviewed artifact set (frozen):** ADR-046 v1.21 + BC-4.17.001 v1.24 + BC-5.40.001 v1.20 + BC-7.07.001 v1.37
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED) — F-P54-001, FIXED
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 2/3 → **RESETS to 0/3** — this was the CONVERGENCE pass (streak was 2/3 entering this pass; a BLOCKING-severity finding at the convergence pass itself resets the streak, the second time this has happened this session (parallel to pass-43's 2/3→0/3 reset)
**D-chain:** D-1111

## Part A — Finding Set (frozen set: ADR-046 v1.21 + BC-4.17.001 v1.24 + BC-5.40.001 v1.20 + BC-7.07.001 v1.37)

**HIGH (0):** none this pass.

**MEDIUM (1):**

- **F-P54-001 (MED, POLICY 15 spec-vs-code + POLICY 4 internal-consistency).** ADR-046
  systematically mis-cited `verify-state-timestamp-refresh`'s own module-doc step numbering at
  four loci:
  - §Context item 2 labeled the lock-expiry (`factory_lock.expires_at`) staleness arm "Step 7".
  - §Rationale's "Why the identity gate on `expires_at`" bullet also labeled the lock-expiry arm
    "Step 7".
  - §Decision 5's retirement paragraph labeled the two enforcement arms "Steps 4–6 (timestamp
    staleness block)" and "Step 7 (lock-expiry staleness block)".
  - §Decision 3's "three current mechanisms corrected to four" bullet also labeled the lock-expiry
    arm "Step 7".

  **Ground truth**, confirmed by inspection of
  `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`'s own module-doc "On each
  invocation the guard:" enumeration (whose own Step 3a already reads "If only `factory_lock:` is
  set: skip Steps 4–7; proceed to Step 8"): **Steps 4–7 are the `timestamp:` staleness arm** (Step
  4 extract, Step 5 block-if-absent-in-proposed, Step 6 continue-if-absent-on-disk, Step 7
  block-if-byte-identical); **Step 8 is the separate `factory_lock.expires_at` staleness arm.**
  ADR-046 had the two arms' step-ranges swapped/mislabeled at all four loci above (correctly
  identifying WHICH arm does what functionally, but citing the WRONG step numbers for each).

  There was also an internal §Context/§Decision-5 self-inconsistency compounding the defect:
  §Context's own item 2 (pre-fix) read "Step 7 (module-doc Steps 4–8; the lock-expiry arm)" —
  citing the umbrella range "Steps 4–8" for BOTH arms in the same breath as mislabeling the
  lock-expiry arm's own specific step as "7" instead of "8" — while §Decision 5 (pre-fix) read
  "Steps 4–6 (timestamp) and Step 7 (lock-expiry)", a third, mutually-inconsistent step-range
  split. All three citations (§Context's "Steps 4–8" umbrella, §Context/§Rationale/§Decision-3's
  "Step 7" lock-expiry label, and §Decision-5's "Steps 4–6"/"Step 7" split) could not simultaneously
  be correct, since they assign non-overlapping and non-matching ranges to the same two arms.

  **Fixed by architect**, all four loci corrected in the same burst:
  - §Context item 2 → "Step 8 (module-doc Steps 4–8; the lock-expiry arm)".
  - §Decision 5 → "Steps 4–7 (timestamp staleness block) and Step 8 (lock-expiry staleness
    block)".
  - §Rationale and §Decision 3 → both now cite "Step 8" for the lock-expiry arm.

  A within-artifact grep-complete sweep of every `Step[s] [0-9]` token in the document body
  confirmed these were the ONLY four loci citing `verify-state-timestamp-refresh`'s own step
  numbers — every other "Step N" citation in this ADR (e.g. `precompact-flush`'s own module-doc
  Step 4, the `factory-lock` crate's `renew_lock_with_now` Steps 2/4/5) refers to a DIFFERENT
  module's own step numbering and was already correctly attributed; no sibling recurrence within
  ADR-046 itself. A parallel check of BC-5.40.001, BC-4.17.001, and BC-7.07.001 for the same
  mis-citation pattern found NONE — BC-4.17.001's one "Step 5" occurrence and BC-7.07.001's one
  "Step 2" occurrence both refer to `factory-lock`/`precompact-flush`'s own step numbering, not
  `verify-state-timestamp-refresh`'s, and required no correction.

  This is a genuine, narrow spec-vs-code defect: the SUBSTANCE of §Decision 5 (both enforcement
  arms are retired; the stamper makes both structurally unreachable) was never wrong — only the
  step-number labels attached to each arm were swapped. No Decision content, File-Change Plan, or
  Companion Amendment item was otherwise touched; Decision numbering (1–6) is unchanged; Status
  remains **accepted**.

**Zero LOW/informational observations this pass** beyond the single MED finding above.

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings beyond F-P54-001)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set on
its own merits only). Every dimension this gate's 53-pass history has previously found a defect in
was independently re-checked against the current frozen set and confirmed holding, with zero
regression beyond F-P54-001:

- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified CLEAN across all four artifacts — stable
  since pass-27 (28 consecutive passes now, counting this one).
- **Illustrative-content verbatim-source-accuracy + sibling-parity discipline (D-1101, ninth
  discipline, extended D-1108):** the §Decision 5 "analogous to T-001/T-004/T-005/T-007"
  enumeration re-checked — still correct, no regression.
- **CITATION→INPUT PARITY (D-1106/D-1107, fourteenth discipline):** grep-complete cluster-wide
  `inputs:` sweep found zero new citation-without-input stragglers.
- **Catalog-membership-verification (D-1107, fifteenth discipline):** no new "present in <INDEX>"
  assertion introduced this pass.
- **AC-attribution cluster-wide drain (D-1100/D-1103/D-1104):** re-derived every `AC-[0-9]+`
  live-body citation across all four frozen-set artifacts — zero mis-anchors, class remains
  DRAINED.
- **VERBATIM-ABSENCE / SUMMARY-ENUMERATION disciplines (D-1105):** no new absence or
  enumeration claims found requiring correction, beyond F-P54-001 itself (a distinct sub-class —
  numeric step-citation accuracy, not a verbatim-quote or summary-enumeration defect).
- **4-leg `modified:`-array head==version parity (D-1089):** all four artifacts' `modified:`
  array heads confirmed matching their own frontmatter `version:` field, prior to this pass's
  edit.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) prior to this burst's edit; only ADR-046 edited this pass (the 3 companion BCs
  UNCHANGED) — see Part C for the resulting hash-state disposition.
- **BC-7.07.001 §Description "no-op" phrasing (O-P53-DESC-NOOP):** re-examined — remains
  DEFENSIBLE per pass-53's adjudication, ACCEPTED-tracked, not touched this pass.

**Novelty assessment:** F-P54-001 is a NEW distinct finding class not previously caught by any of
the fifteen codified convergence-technique disciplines — a **STEP-NUMBER CITATION** defect: a
narrative citation of "Step N"/"Steps N-M" referencing a module's OWN internal enumeration must be
cross-checked against that module's actual `//!`/doc-comment step numbering, not merely checked for
functional-arm correctness (i.e. "this text correctly identifies WHICH arm does what" is
insufficient — the exact step NUMBERS attached must also match the source module's own numbering).
This defect survived 53 prior passes because no prior adversary had cross-checked the EXACT
module-doc step numbers against `verify-state-timestamp-refresh`'s own source comment — all prior
passes verified the functional/behavioral correctness of the two-arms description (which was
always accurate) without independently re-deriving the literal step-number labels from the cited
module's own doc-comment enumeration. CODIFIED as the SIXTEENTH convergence-technique discipline:
**STEP-NUMBER CITATION** — see `lessons.md`.

## Part C — State at Close of Review

ADR-046 **v1.21→v1.22** (F-P54-001 fix, architect). BC-4.17.001 v1.24, BC-5.40.001 v1.20,
BC-7.07.001 v1.37 — all three companion BCs **UNCHANGED** this pass (F-P54-001 was confined
entirely to ADR-046's own body text; the parallel cross-BC check found no analogous mis-citation
in any companion BC). BC-5.39.001 3-CLEAN streak: **2/3 → RESETS to 0/3** — this was the
CONVERGENCE pass (1 more consecutive clean pass would have reached literal 3-CLEAN); a genuine
BLOCKING-severity finding at the convergence pass itself resets the streak to 0/3, the SECOND time
this exact pattern (streak 2/3 entering the convergence pass, reset by a fresh-lens finding) has
occurred this session — the first was pass-43 (2/3→0/3, capabilities.md inputs-completeness lens).
Gate history to date: 54 passes run against evolving/frozen sets; 42 genuine BLOCKING findings
found and fixed (41 through pass-53, plus F-P54-001), plus 10 audit-extra stragglers (pass-31,
pass-33, pass-49 ×6), 1 latent-bracket drain (pass-37, not counted as genuine), 2 ACCEPTED
non-blocking observations (O-P42-001, O-P53-DESC-NOOP, neither counts against the streak), and 3
FIXED non-blocking observations (O-P44-001, O-P48-001, O-P51-001). Behavioral core independently
re-verified CLEAN for the 28th consecutive pass (since pass-27) — F-P54-001 does not touch it; the
defect is confined entirely to the citation/provenance layer (exact step-number labels), not the
functional design.

**Index reconciliation (state-manager, this burst):** ARCH-INDEX **v3.91→v3.92** (ADR-046 row
bumped v1.21→v1.22; version-stable read-through convention preserved — no literal target-version
hardcoded in any sync-instruction). BC-INDEX v5.15, STORY-INDEX v4.392, VP-INDEX v2.79 all
**UNCHANGED** (no companion-BC/story/VP edit this pass).

**Input-hash recompute (cyclic-hash TD [D-1082] — settled + cross-referenced, NOT reopened):**
`compute-input-hash --check` run for ADR-046 against its post-edit content: **CONFIRMED SETTLED,
unchanged at `cb428ff`** — no drift. The F-P54-001 fix touched no `inputs:`-listed citation (it
corrected numeric labels attached to an already-cited module, `verify-state-timestamp-refresh`,
which was already present in `inputs:`), so ADR-046's own input-hash — which reflects the content
of its LISTED inputs, not its own body text — is unaffected by this edit. The 3 companion BCs'
own stored hashes (which include ADR-046 as one of their listed inputs) were NOT re-checked for
new drift this burst, consistent with established convention (ADR-046's content changed, so their
hashes are now technically 1-hop-stale relative to ADR-046 v1.22 exactly as they were relative to
v1.21 — no new state introduced; not re-chased, per the established D-1082 disposition).

**NEXT: fresh pass-55** against the newly-frozen set (ADR-046 v1.22 + BC-4.17.001 v1.24 +
BC-5.40.001 v1.20 + BC-7.07.001 v1.37), starting a new streak at 0/3, applying all sixteen
now-codified convergence-technique disciplines proactively from the start, including the newly
codified SIXTEENTH discipline (STEP-NUMBER CITATION): any "Step N"/"Steps N-M" citation of a
module's own enumeration MUST be cross-checked against that module's actual `//!`/doc-comment step
numbering, not merely checked for functional/arm correctness. The human decision this session
remains to CONTINUE looping toward literal 3-CLEAN (not accept D-386 Option C asymptotic
acceptance) — reaffirmed again this burst; accept-provisional under D-386 Option C remains
available at any time. S-17.05 TDD implementation remains gated on convergence.
