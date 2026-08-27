# ADR-046 Adversarial Spec-Convergence Review — Pass 39

**Reviewed artifact set (frozen):** ADR-046 v1.16 + BC-4.17.001 v1.17 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33
**Review date:** 2026-08-27
**Verdict:** FINDINGS (1 MED), 0 HIGH, 0 LOW
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 1/3 → **RESETS to 0/3** (the THIRD reset this session — first at pass-35 on a genuinely new audit dimension, second at pass-37 on that dimension's own remediation-prose bookkeeping; this one is a GENUINE data-destructive internal contradiction in the frozen set's own operative spec content, not a metadata/prose defect like the two prior resets)
**D-chain:** D-1096

## Part A — Finding Set (frozen set: ADR-046 v1.16 + BC-4.17.001 v1.17 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33)

**HIGH (0):** none this pass.

**MEDIUM (1):**

- **F-P39-001 (MED, POLICY 4, semantic-anchoring-integrity)** — BC-4.17.001 v1.17's Precondition 4
  and Invariant 7 mandated operating ONLY on the `extract_frontmatter` frontmatter-only slice for
  BOTH the `timestamp:` re-stamp arm AND the `expires_at` renewal arm. This directly contradicts
  Precondition 2's and Invariant 9's own requirement that `renew_lock_if_holder` be fed the FULL
  `content_after_pc1` (the whole file, not a frontmatter slice) and write back the whole
  reconstructed file. A literal reading of Precondition 4/Invariant 7's slice-exclusivity directive
  applied to the `expires_at` arm would feed `renew_lock_if_holder` a frontmatter-only slice,
  truncating its `RenewOutcome::Renewed(new_content)` return value to the frontmatter region alone —
  and since Invariant 9 composes both arms into a SINGLE `host::write_file` call, that truncated
  region would become the entire written file, DESTROYING STATE.md's body content on write. This is
  not a cosmetic or narrative defect: following the BC's own literal text on the `expires_at` arm is
  data-destructive. Independently re-derived by opening PC1, PC2, PC4, Invariant 7, and Invariant 9
  together and tracing the actual data flow each arm feeds into the single composed write — PC2 and
  Invariant 9 are internally consistent with each other (full-content in, full-content out) and were
  NOT the source of the contradiction; PC4/Invariant 7's slice-exclusivity mandate is the locus that
  needed to be scoped, because it was written as if it applied uniformly to both arms when only the
  `timestamp:` arm (PC1's read-only scan) is actually frontmatter-slice-safe.

  **Disposition: FIXED.** Product-owner corrected Precondition 4 and Invariant 7 to scope the
  `extract_frontmatter`-slice byte-range restriction to PC1's `timestamp:` scan only. For the
  `expires_at` arm, the frontmatter-confinement guarantee is restated as a semantic-region guarantee
  delegated internally to `renew_lock_if_holder`/`flp::parse_factory_lock`/`rewrite_expires_at` —
  those functions are themselves responsible for locating and rewriting only the `factory_lock`
  frontmatter block — while the arm is still fed the full `content_after_pc1` per PC2, exactly
  mirroring PC4's own pre-existing "'Targeted' is a semantic-scope guarantee, not a write-mechanism
  constraint" framing (the same framing PC4 already used to reconcile an analogous what-vs-how
  tension for the `timestamp:` arm at Pass-16/O-P16-001). PC1, PC2, and Invariant 9 themselves are
  UNCHANGED — independently re-verified already correct; the fix is confined to PC4/Invariant 7's
  scoping language. No PC/Invariant/EC renumbered (append-only numbering preserved per POLICY 1).
  BC-4.17.001 **v1.17 → v1.18**. Bracket-balance in the resulting `last_amended` field re-verified
  balanced (18 `[Prior:` opens vs. 18 closing `]`s, literal count) — the v1.18 entry's own nested
  history wrapping introduced no imbalance.

**LOW (0):** none this pass.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set on
its own merits only). Every dimension this gate's 38-pass history has ever found a defect in was
independently re-checked against the current frozen set, and the contradiction above was found via
a first-principles trace of the write-composition data flow rather than any previously-codified
checklist item:

- **ADR §Decision/§N.M anchor correctness (D-1092/F-P35-001 class):** every `ADR-NNN §Decision N`/
  `§N.M` citation across the frozen set independently re-derived — all CORRECT, no regression.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):** independently
  recounted ADR-046's `## Decision` section — confirmed 6 numbered decisions, matching both BCs'
  amendment prose — no regression.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts EXCEPT the one
  contradiction found above — no other regression of any prior-pass fix.
- **Every load-bearing code claim (function names, file paths, constant names):** independently
  re-verified against the actual source files — `crates/hook-sdk/src/result.rs`'s `HookResult` enum,
  `crates/factory-lock/src/lib.rs`'s `renew_lock_if_holder`/`rewrite_expires_at`/`TTL_SECONDS`,
  `crates/factory-lock-parse/src/lib.rs`'s `extract_frontmatter`/`STATE_MD_MAX_BYTES` — all accurate,
  no fresh mis-attribution found.
- **`inputs:` completeness on all four frontmatter arrays:** re-audited via the GREP-COMPLETE
  mechanical method (D-1090) — zero omissions found on any of the four artifacts.
- **`modified:`-array-head-parity (4-leg head==version self-check, D-1089):** all four artifacts
  confirmed — `version:` == `modified:`-array-head == `## Changelog`-table-head ==
  `last_amended`-prefix, no gaps (checked pre-fix on the entering v1.17 state; the v1.18 fix itself
  is product-owner's responsibility to re-verify, out of this pass's own re-derivation scope once the
  finding was identified and routed for fix).
- **Self-attested completeness-claim discipline (D-1094's mitigation):** this pass's own finding
  narrative makes no uncounted cardinality claim requiring a mechanical backing check — the
  contradiction was demonstrated by direct clause-to-clause data-flow tracing, not by a count
  assertion.
- **Cross-anchor citation accuracy, type-provenance (`LockState` vs `FactoryLock`), POLICY 19
  anti-volatile-pin, §Story Anchor/Traceability parity, subsystem labels, status/lifecycle pairs:**
  all re-verified clean across the frozen set — no regression on any previously-codified dimension.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not re-chased)
  — current settled state internally consistent with each artifact's own `inputs:` array.

**No other spec-vs-code or spec-vs-spec contradiction found this pass on ANY previously-codified
dimension. F-P39-001 is confined to the single PC4/Invariant-7-vs-PC2/Invariant-9 contradiction
described above.**

**Novelty assessment:** this is the THIRD reset this session, but qualitatively DIFFERENT from the
prior two. Pass-35's reset (D-1092) was a newly-revealed audit-dimension gap (ADR §Decision anchor
correctness) — a citation-accuracy defect. Pass-37's reset (D-1094) was a bookkeeping miscount
INSIDE a prior remediation's own narrative — a metadata/prose defect with no data-destructive
consequence. **This pass's finding is neither** — it is a genuine unreconciled internal contradiction
in the BC's own OPERATIVE spec content (Precondition/Invariant text governing actual write behavior)
that, if followed literally, would truncate and destroy STATE.md's body on a live write. It survived
37 prior passes (including the pass-16 fix that reconciled the analogous what-vs-how tension for
PC4's OTHER case, EC-014/EC-015, and every subsequent comprehensive audit) because no prior pass's
audit checklist included "trace the full write-composition data flow for each PC4/Invariant-7 arm
independently and check it against PC2/Invariant 9's own full-content requirement" as a discrete
check — every prior audit validated citation accuracy, array ordering, inputs completeness, and
cardinality claims, but not this specific what-vs-how arm-parity gap. Per BC-5.39.001, this is 1 of 3
required CONSECUTIVE clean passes from zero; the streak resets to 0/3.

## Part C — State at Close of Review

BC-4.17.001 **v1.17 → v1.18** (F-P39-001, Precondition 4 + Invariant 7 arm-scoped reconciliation).
ADR-046 **v1.16 UNCHANGED** (not touched — the contradiction lives entirely inside BC-4.17.001's own
PC4/Invariant 7 text, not in ADR-046 itself). BC-5.40.001 **v1.16 UNCHANGED**; BC-7.07.001 **v1.33
UNCHANGED** (neither carries the defective directive — confirmed clean, no edit).

BC-5.39.001 3-CLEAN streak: **1/3 → RESETS to 0/3.** Gate history to date: 39 passes run against
evolving/frozen sets; 31 genuine findings found and fixed (30 prior + this pass's 1), plus 4
audit-extra stragglers (pass-31, pass-33) and 1 latent-bracket drain (pass-37, not counted as
genuine); 3 clean passes (34, 36, 38), each followed by a reset.

**NEXT: fresh pass-40** against the newly-frozen set (ADR-046 v1.16 + BC-4.17.001 v1.18 +
BC-5.40.001 v1.16 + BC-7.07.001 v1.33); needs 3 consecutive clean passes (40, 41, 42) for literal
3-CLEAN convergence, applying all convergence-technique disciplines proactively — now including the
arm-parity what-vs-how reconciliation check as a discrete, explicit item. The human decision this
session remains to CONTINUE looping toward literal 3-CLEAN convergence (not accept D-386 Option C
asymptotic acceptance). S-17.05 TDD implementation remains gated on convergence.
