# ADR-046 Adversarial Spec-Convergence Review — Pass 26

**Reviewed artifact set (frozen):** ADR-046 v1.11 + BC-4.17.001 v1.12 + BC-7.07.001 v1.28 + BC-5.40.001 v1.10
**Review date:** 2026-08-26
**Verdict:** FINDINGS (1 MED, 2 LOW observations)
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **REMAINS 0/3** (already reset at pass-25; a finding does not reset an already-0/3 streak further)
**D-chain:** D-1083

## Part A — Finding Set (frozen set: ADR-046 v1.11 + BC-4.17.001 v1.12 + BC-7.07.001 v1.28 + BC-5.40.001 v1.10)

**MEDIUM (1):**

- **F-P26-001 (MED, POLICY 14/17/6)** — ADR-046's File-Change Plan carries its own
  self-referential sync instruction row directing the `.factory/specs/architecture/ARCH-INDEX.md`
  ADR-046 row to bump to a specific version. That row still read "ADR-046 row version bump to
  **v1.10**" and led with the pass-21/F-P21-001 disposition — even after the pass-25 architect
  edit (this same session's prior burst) had already advanced the ADR to v1.11 and added the File-
  Change Plan's own new S-17.05 row (F-P25-002). The pass-25 edit swept every OTHER locus stating
  the ADR's type-provenance/traceability content, but did not sweep this SIBLING instruction row —
  which does not describe the ADR's content, but instructs a downstream artifact (ARCH-INDEX) what
  version to cite. Left uncorrected, the next state-manager burst reading this row verbatim would
  have driven ARCH-INDEX to a stale "v1.10" cite, one full version and two passes behind the ADR's
  actual v1.11 state at the time this pass ran.
  **Disposition: FIXED.** Architect rewrote the row to direct the bump to **v1.12** — this
  revision's own resulting version — leading with the pass-26 (F-P26-001) fix summarized here,
  followed by the pass-25 (F-P25-001/F-P25-002) disposition, with the pass-21/F-P21-001 text
  demoted one rung further down the existing Prior chain (content unchanged, only nesting depth).
  A sweep of the rest of the ADR for other load-bearing self-version directives
  (`grep -n "not applied by this document\|state-manager — not applied\|bump to v1\|version bump to"`)
  found no other locus needing correction — the only other "version-bump" mention is the v1.2
  Changelog entry's own historical "flagged for a v1.2 version-bump" clause, which correctly
  describes what that past revision itself did and needs no correction. This is a TD-VSDD-060-class
  sibling-sweep gap generalized one layer further: a self-referential version-bump DIRECTIVE
  inside an ADR's own File-Change Plan is itself a parity leg that must be swept on every revision
  that changes the ADR's own version, not just the ADR's substantive content loci.

**LOW (2, non-blocking observations, no fix this burst):**

- **O-P26-001 (LOW, non-blocking)** — BC-7.07.001 carries `status: active` while its ADR-046
  amendment invariants (Invariant 3a/3b, the identity-gated `renew_lock_if_holder` call at
  `precompact-flush`'s Step 4) are not yet implemented — the implementing story, S-17.05, has not
  started (gated on this very 3-CLEAN convergence). Judged **WORKING-AS-DESIGNED spec-leading-code**
  (anchored S-17.05, consistent with this repo's VSDD "spec wins" standing rule): the BC correctly
  states the target contract ahead of implementation, and S-17.05's Traceability anchor makes the
  gap discoverable. Unlike sibling BC-4.17.001 (which is wholly draft and carries no such
  ambiguity), BC-7.07.001 pre-dates ADR-046 and is being amended in place, so its `active` status
  reflects its pre-ADR-046 baseline behavior (which IS implemented and active today) plus a
  spec-ahead amendment layer (not yet implemented) — both true simultaneously. No inline pending
  marker was added; this is recorded as a non-blocking awareness note only, not a defect.

- **O-P26-002 (LOW, `[process-gap]`, non-blocking)** — ARCH-INDEX's SS-07 subsystem is labeled
  "Hook Bash Layer," a label that predates ADR-046 and is out of this pass's review perimeter (no
  frozen-set artifact introduced or worsened it), but is an increasing misnomer: native-WASM hook
  plugins (including the ones ADR-046 itself proposes — `stamp-state-timestamp`, and the identity
  gate added to `precompact-flush`) continue to accrete under SS-07 alongside the historically bash
  hook scripts the label originally described. Per the S-7.02 cycle-closing checklist, this
  process-gap needs a justified deferral rather than silent omission: recorded as a Drift Item in
  STATE.md anchored to a future ARCH-INDEX subsystem-label review (not this burst's scope — no
  SS-07 label edit made).

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **LockState propagation (F-P25-001 fix, pass-25):** every locus in the frozen set citing the
  parsed lock's type (§Decision 1(b), the File-Change Plan's `crates/factory-lock/src/lib.rs` row,
  BC-7.07.001 Invariant 3b) now consistently states `LockState` via `renew_lock_if_holder`'s own
  independent `flp::parse_factory_lock` call — no residual `FactoryLock` mis-citation found
  anywhere in the frozen set at this pass, including inside nested Changelog/last_amended history
  prose (which correctly narrates the FIX, not the defect, at every point checked).
- **All spec-vs-code claims re-verified accurate:** every `crates/factory-lock*` function name,
  constant, and struct/enum variant citation in the frozen set was independently re-traced against
  `crates/factory-lock/src/lib.rs` and `crates/factory-lock-parse/src/lib.rs` — all resolve to
  real, correctly-named, correctly-typed code; no new spec-vs-code mismatch found this pass beyond
  F-P26-001 (which is a downstream-instruction defect, not a spec-vs-code content defect).
- **Anchors, subsystem names, and registry facts:** every `§Decision N` cross-reference, every
  `SS-04`/`SS-05`/`SS-07` subsystem assignment, and every BC/story ID cited in the frozen set
  resolves to a real, correctly-named target; no dangling or renamed-target citation found.
- **S-17.05 traceability (F-P25-002 fix, pass-25):** the File-Change Plan's S-17.05 row and all
  three companion BCs' Traceability §Stories/§Story Anchor fields remain consistent and resolved
  — no regression of the pass-25 fix found.
- **POLICY 19 (anti-volatile-pin):** no new load-bearing inline `ADR-046 vN.N` / `BC-X.YY.NNN vN.N`
  version-pin token found in any normative body prose (PC/Invariant/EC text) across the frozen set.

This is an unusually large "verified clean / converged cluster" for a FINDINGS-verdict pass: the
single MED finding is a self-referential instruction-row defect (a downstream-sync directive
citing itself stale), not a content defect in the ADR's or BCs' substantive claims — every
substantive claim independently re-traced against source clean.

## Part C — State at Close of Review

ADR-046 **v1.12** (accepted); BC-4.17.001 **v1.12** (unchanged this pass); BC-7.07.001 **v1.28**
(unchanged this pass); BC-5.40.001 **v1.10** (unchanged this pass). BC-5.39.001 3-CLEAN streak:
**0/3** (REMAINS — already reset at pass-25; this pass's finding does not reset an already-0/3
streak further). Gate history to date: 25 passes run against evolving/frozen sets; 8 genuine bugs
found and fixed prior to this pass (F-P10-001/F-P13-001/F-P15-001/F-P18-001 HIGH,
F-P21-001/F-P23-001/F-P25-001/F-P25-002 MED); this pass adds F-P26-001 (MED, fixed) — 9 genuine
findings fixed across 26 passes total. 2 non-blocking LOW observations recorded (O-P26-001, no
fix; O-P26-002, deferred `[process-gap]`).
**NEXT: fresh pass-27** against the newly-frozen set (ADR-046 v1.12 + BC-4.17.001 v1.12 +
BC-7.07.001 v1.28 + BC-5.40.001 v1.10); needs 3 consecutive clean passes (27, 28, 29) for literal
3-CLEAN convergence. S-17.05 TDD implementation remains gated on convergence.
