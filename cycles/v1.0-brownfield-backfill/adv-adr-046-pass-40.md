# ADR-046 Adversarial Spec-Convergence Review — Pass 40

**Reviewed artifact set (frozen):** ADR-046 v1.16 + BC-4.17.001 v1.18 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED), 0 HIGH, 0 LOW
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **STAYS 0/3** (already 0/3 from the pass-39 reset; a finding on this pass keeps it at 0/3 rather than advancing it)
**D-chain:** D-1097

## Part A — Finding Set (frozen set: ADR-046 v1.16 + BC-4.17.001 v1.18 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33)

**HIGH (0):** none this pass.

**MEDIUM (1):**

- **F-P40-001 (MED, POLICY 4, sibling-locus-sweep-completeness)** — BC-4.17.001 v1.18's pass-39/
  D-1096 remediation corrected Precondition 4 and Invariant 7 to arm-scope the
  `extract_frontmatter`-slice byte-range restriction to the `timestamp:` arm only, restating the
  `expires_at` arm's frontmatter confinement as a semantic-region guarantee fed the full
  `content_after_pc1`. That fix, however, did NOT sweep to VP-TBD-8 — the BC's own
  §Verification Properties table row covering the identical `extract_frontmatter`-use guarantee.
  VP-TBD-8 still read (pre-this-pass) as a single joint clause applying the frontmatter-slice
  confinement to BOTH arms indiscriminately — the exact pre-F-P39-001 framing the Precondition/
  Invariant pair was corrected away from at v1.18, left un-swept at this sibling locus. A literal
  reading of VP-TBD-8 (the artifact a verifier would consult to write the corresponding unit test)
  would re-encode the same data-destructive hazard F-P39-001 closed: testing/asserting that the
  `expires_at` arm's renewal is confined to the `extract_frontmatter` slice, when Precondition 2/
  Invariant 9 require it be fed the FULL `content_after_pc1`. This is a genuine re-encoding of a
  just-fixed defect at a sibling locus — not a new class of contradiction, but direct empirical
  validation that the D-1096 arm-parity sibling-sweep codification (a what-vs-how reconciliation
  applied to one arm/case MUST sweep to every analogous sibling arm/case IN THE SAME BURST) was
  itself under-applied at v1.18: the sweep covered Precondition 4 and Invariant 7 but not the VP
  table, ADR anchors, or SDK-grounding blocks carrying the same guarantee.

  **Disposition: FIXED.** Product-owner corrected VP-TBD-8 to state the arm split explicitly:
  (a) PC1's `timestamp:` scan remains byte-range-confined to the `extract_frontmatter` slice; (b)
  PC2's `expires_at` renewal is fed the FULL `content_after_pc1`, its frontmatter confinement
  restated as a semantic-region guarantee delegated to `renew_lock_if_holder`/
  `flp::parse_factory_lock`/`rewrite_expires_at`, verified by asserting post-write STATE.md BODY
  byte-preservation rather than by asserting slice-consumption. VP-TBD-8's stale internal pointer
  (`corrected 2026-08-26, F-P15-001`, referencing the v1.9 fence-not-located sweep) was also
  corrected to cite the v1.18/F-P39-001 arm-scope split and this v1.19 sweep. **Comprehensive
  sweep performed same-burst:** every other locus in BC-4.17.001 mentioning `extract_frontmatter`,
  frontmatter slice/region, byte-range, or joint PC1/PC2 scoping was checked against the v1.18
  arm-split — Precondition 4 (confirmed correct, arm-split since v1.18, not re-broken), Invariant 7
  including its fence-not-located tail (confirmed correct, arm-split since v1.18, not re-broken),
  PC1's rewrite-mechanism paragraph, PC3a, PC4, Invariant 5, Edge Cases, Canonical Test Vectors,
  Architecture Anchors, and Description — VP-TBD-8 was the ONLY locus still carrying the joint-arm
  framing; every other locus already stated the v1.18 arm-split or described a genuinely unaffected
  clause (fully-structural fail-open, semantic-scope write claims, emit-only diagnostics). No
  PC/Invariant/EC renumbered (append-only numbering preserved per POLICY 1). BC-4.17.001
  **v1.18 → v1.19**. Bracket-balance in the resulting `last_amended` field re-verified balanced
  (24 `[Prior:` opens vs. 24 closing `]`s, literal count) — the v1.19 entry's own nested history
  wrapping introduced no imbalance.

**LOW (0):** none this pass.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set on
its own merits only). Every dimension this gate's 39-pass history has ever found a defect in was
independently re-checked against the current frozen set:

- **Arm-parity what-vs-how reconciliation (D-1096/F-P39-001 class), Precondition/Invariant legs
  specifically:** Precondition 4 and Invariant 7 both independently re-verified correct and
  arm-split since v1.18 — no regression on the two loci the pass-39 fix directly touched.
- **ADR §Decision/§N.M anchor correctness (D-1092/F-P35-001 class):** every `ADR-NNN §Decision N`/
  `§N.M` citation across the frozen set independently re-derived — all CORRECT, no regression.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):** independently
  recounted ADR-046's `## Decision` section — confirmed 6 numbered decisions — no regression.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts EXCEPT the one
  sibling-locus straggler found above — no other regression of any prior-pass fix.
- **Every load-bearing code claim (function names, file paths, constant names):** independently
  re-verified against the actual source files — `crates/hook-sdk/src/result.rs`'s `HookResult`
  enum, `crates/factory-lock/src/lib.rs`'s `renew_lock_if_holder`/`rewrite_expires_at`/
  `TTL_SECONDS`, `crates/factory-lock-parse/src/lib.rs`'s `extract_frontmatter`/
  `STATE_MD_MAX_BYTES` — all accurate, no fresh mis-attribution found.
- **`inputs:` completeness on all four frontmatter arrays:** re-audited via the GREP-COMPLETE
  mechanical method (D-1090) — zero omissions found on any of the four artifacts.
- **`modified:`-array-head-parity (4-leg head==version self-check, D-1089):** all four artifacts
  confirmed consistent on the entering v1.18 state (checked pre-fix; the v1.19 fix itself is
  product-owner's/state-manager's responsibility to re-verify, out of this pass's own
  re-derivation scope once the finding was identified and routed).
- **Self-attested completeness-claim discipline (D-1094's mitigation):** this pass's own finding
  narrative makes no uncounted cardinality claim requiring a mechanical backing check beyond the
  explicit 8-locus enumeration performed and stated above.
- **Cross-anchor citation accuracy, type-provenance (`LockState` vs `FactoryLock`), POLICY 19
  anti-volatile-pin, §Story Anchor/Traceability parity, subsystem labels, status/lifecycle pairs:**
  all re-verified clean across the frozen set — no regression on any previously-codified dimension.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) — current settled state internally consistent with each artifact's own `inputs:`
  array; BC-4.17.001's own input-hash confirmed UNCHANGED at `4970575` (none of its cited
  `inputs:` files changed content this burst).

**No other spec-vs-code or spec-vs-spec contradiction found this pass on ANY previously-codified
dimension. F-P40-001 is confined to the single VP-TBD-8 sibling-locus straggler described above.**

**Novelty assessment:** this finding is NOT a new dimension — it is the FIRST direct empirical
validation of the D-1096 arm-parity sibling-sweep codification's own scope: the codification states
a what-vs-how reconciliation applied to one arm/case MUST sweep to every analogous sibling
arm/case in the same burst, and this pass demonstrates that "sibling arm/case" must be read to
include not just Preconditions/Invariants but also §Verification Properties rows, Architecture
Anchors, and SDK-grounding blocks carrying the identical guarantee — the v1.18 fix swept the
Precondition/Invariant pair correctly but left the VP-table locus unswept. This is process-gap
evidence about the codification's own completeness, not a fresh spec-vs-code defect independent of
pass-39. Per BC-5.39.001, a finding on any pass keeps or resets the streak to 0/3; since the streak
was already 0/3 entering this pass (from pass-39's reset), it STAYS at 0/3 rather than resetting
again.

## Part C — State at Close of Review

BC-4.17.001 **v1.18 → v1.19** (F-P40-001, VP-TBD-8 sibling-locus-straggler sweep). ADR-046 **v1.16
UNCHANGED** (not touched — the straggler lives entirely inside BC-4.17.001's own VP table, not in
ADR-046 itself). BC-5.40.001 **v1.16 UNCHANGED**; BC-7.07.001 **v1.33 UNCHANGED** (neither carries
the defective directive — confirmed clean, no edit).

BC-5.39.001 3-CLEAN streak: **0/3 → STAYS 0/3.** Gate history to date: 40 passes run against
evolving/frozen sets; 32 genuine findings found and fixed (31 prior + this pass's 1), plus 4
audit-extra stragglers (pass-31, pass-33) and 1 latent-bracket drain (pass-37, not counted as
genuine); 3 clean passes (34, 36, 38), each followed by a reset (35, 37, 39 respectively).

**NEXT: fresh pass-41** against the newly-frozen set (ADR-046 v1.16 + BC-4.17.001 v1.19 +
BC-5.40.001 v1.16 + BC-7.07.001 v1.33); needs 3 consecutive clean passes (41, 42, 43) for literal
3-CLEAN convergence, applying all convergence-technique disciplines proactively — now including the
EXTENDED arm-parity/sibling-sweep check as a discrete item covering VPs, Architecture Anchors, and
SDK-grounding blocks, not just Preconditions/Invariants. The human decision this session remains to
CONTINUE looping toward literal 3-CLEAN convergence (not accept D-386 Option C asymptotic
acceptance). S-17.05 TDD implementation remains gated on convergence.
