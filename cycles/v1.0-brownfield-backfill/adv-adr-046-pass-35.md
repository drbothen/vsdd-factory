# ADR-046 Adversarial Spec-Convergence Review — Pass 35

**Reviewed artifact set (frozen):** ADR-046 v1.16 + BC-4.17.001 v1.15 + BC-7.07.001 v1.33 + BC-5.40.001 v1.14
**Review date:** 2026-08-26
**Verdict:** FINDINGS (2: 1 HIGH + 1 MED), 0 LOW observations
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 1/3 → **RESETS to 0/3** (a finding after a clean pass resets the streak)
**D-chain:** D-1092

## Part A — Finding Set (frozen set: ADR-046 v1.16 + BC-4.17.001 v1.15 + BC-7.07.001 v1.33 + BC-5.40.001 v1.14)

**HIGH (1):**

- **F-P35-001 (HIGH, POLICY 4, semantic-anchoring-integrity)** — 3 loci across 2 companion BCs cited
  `ADR-025 §Decision 12 §12.5` as the decision establishing the 256 KiB `STATE_MD_MAX_BYTES` read
  cap: BC-4.17.001 Precondition 4's cap-sourcing sentence; BC-5.40.001 Precondition 6's cap-sourcing
  sentence; BC-5.40.001's own Architecture Anchors ADR-025 bullet's "cap parity" clause. Verified
  against ADR-025 by opening it directly: §Decision 12 §12.5 is "Shared parse logic — no
  duplication" — the decision that promotes `parse_factory_lock`/`LockState` to the shared
  `factory-lock-parse` crate; it states no byte-cap value anywhere in its text. The decision that
  actually raised the cap from the original 65536 to 262144 is §Decision 14 ("verify-factory-lock
  read-cap 262144 + frontmatter-only parse"), whose own "Normative twin" line names
  `BC-4.13.001 §Precondition 3 (Phase-A)` — the exact BC all 3 loci already cross-cite alongside the
  wrong ADR-025 anchor. This is a mis-anchor, not a wrong value: the 262144 figure itself was
  correct at all 3 loci throughout; only the ADR §Decision NUMBER attributing that figure to its
  source was wrong.
  **Disposition: FIXED.** Product-owner corrected all 3 loci from `ADR-025 §Decision 12 §12.5` to
  `ADR-025 §Decision 14`, quoting the corrected decision's title and Normative-twin citation inline
  for future-proofing (POLICY 19 anti-volatile-pin: stable `§Decision N` anchor form retained, no
  raw version-pin token introduced). BC-5.40.001's separate `§Decision 7 fail-open` clause
  (Architecture Anchors) was independently checked against ADR-025 §Decision 7 ("Crash behavior —
  `on_error = \"continue\"` (fail-open)") and confirmed CORRECT — left unchanged.

**MEDIUM (1):**

- **F-P35-002 (MED, POLICY 18, `inputs:` completeness)** — BC-4.17.001's `inputs:` frontmatter array
  cited ADR-025 nowhere despite Precondition 4's cap-sourcing sentence treating it as a load-bearing
  authority (the very sentence F-P35-001 corrected). Sibling BC-5.40.001 and BC-7.07.001 both
  already list ADR-025 in their own `inputs:` arrays; BC-4.17.001 was the sole companion BC missing
  it.
  **Disposition: FIXED.** Product-owner added
  `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md`
  to BC-4.17.001's `inputs:` array, same path form the siblings already use.

**MANDATORY comprehensive ADR §Decision anchor audit (newly-revealed dimension, in-scope, run on
both BCs and by architect on ADR-046 itself):** every `ADR-NNN §Decision N`/`§N.M` citation across
the frozen set was checked against the cited ADR's actual section content — not merely BC-to-BC
`§Section` cross-references, which every prior comprehensive audit on this gate (F-P25 through
F-P33) confined itself to. This is the first pass to extend the anchor-correctness discipline to
ADR §Decision numbers themselves.

- **BC-4.17.001:** ADR-046 Decision 1/1(a)/1(b) (new plugin + `renew_lock_if_holder`), Decision 2
  (identity model/trim/classifier), Decision 4 (renewal-indeterminate event), Decision 5 (retire
  `verify-state-timestamp-refresh`) — all confirmed CORRECT against ADR-046's flat 1–5 `## Decision`
  list. ADR-025 §Decision 14 (this BC's sole ADR-025 citation, post-fix) — CORRECT.
- **BC-5.40.001:** ADR-025 §Decision 14 (2 loci, both fixed above) and §Decision 7 (fail-open,
  confirmed correct, unchanged) — the only 2 distinct ADR-025 decisions this BC cites.
- **ADR-046 (architect audit):** ADR-046's own only cross-ADR anchor is ADR-025 §Decision 12 §12.2
  ("byte-for-byte comparison" — content-equality semantics for the `expires_at`-arm idempotency
  argument) — independently re-verified against ADR-025 §12.2's actual text and confirmed CORRECT.
  **No edit required; ADR-046 stays v1.16 UNCHANGED.**
- **BC-7.07.001 (product-owner audit):** re-checked for the same ADR §Decision anchor class —
  CLEAN, no mis-anchor found, no edit.

**No other ADR (ADR-025/ADR-046) mis-cited anywhere across the frozen set beyond the 3 F-P35-001
loci.** Product-owner's audit confirms only those 3 loci (across all 3 companion BCs) were
mis-anchored on this dimension.

**LOW (0):** none this pass.

## Part B — Verified-Clean Observations (adversary-confirmed, no findings)

- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts — no regression
  of any prior-pass fix. Stable since pass-27 (9 consecutive passes now, counting this one).
- **`modified:`-array-head-parity (4-leg self-check, D-1089):** re-verified holding on both edited
  BCs (BC-4.17.001 v1.16, BC-5.40.001 v1.15) and unaffected BC-7.07.001/ADR-046 — no regression.
- **`inputs:` completeness (GREP-COMPLETE method, D-1090), beyond the ADR-025 gap fixed above:** no
  further missing load-bearing spec/code citations found on either edited BC, ADR-046, or
  BC-7.07.001.
- **Every load-bearing code claim (function names, file paths, constant names) independently
  re-verified against the actual source files:** all confirmed accurate; no fresh mis-attribution
  found.
- **BC-to-BC cross-anchor citation accuracy (`§Section`/`PCn`/`Invariant-N`):** re-verified across
  all three companion BCs — all resolve correctly, no drift.
- **Type-provenance (`LockState` vs `FactoryLock`):** re-verified clean; no regression.
- **POLICY 19 (anti-volatile-pin):** no new load-bearing inline `ADR-046 vN.N`/`BC-X.YY.NNN vN.N`
  version-pin token introduced by this burst's fixes — the corrected ADR-025 citations use the
  stable `§Decision N` anchor form throughout.
- **§Story Anchor / Traceability parity:** re-verified clean across all three companion BCs; no
  regression.
- **Version-stable ARCH-INDEX directive:** confirmed still holding (out of this burst's scope —
  ARCH-INDEX unaffected, ADR-046 not touched).

**Substance assessment: the substance stayed clean.** Both findings this pass are the same
cap-migration-lineage cluster defect — a citation lineage artifact of the 65536→262144 cap having
moved decision-numbers across ADR-025's own revision history, propagated by copy-paste across
sibling BCs, never independently re-verified against ADR-025's actual section content until this
pass introduced the dimension. Neither finding touches behavioral logic, write composition, or any
code-vs-spec contradiction — both are pure citation-anchor/frontmatter-completeness defects of the
identical class F-P35-001/F-P35-002 describe together.

**Novelty assessment:** this is a **newly-revealed audit dimension** — every prior comprehensive
"cross-anchor" audit on this gate (D-1088's cross-anchor semantic audit, D-1090/D-1091's
grep-complete inputs audits) checked BC-to-BC `§Section` references and `inputs:`-array
completeness, but none independently opened the cited ADR's own `§Decision N` section to verify the
NUMBER is correct — as opposed to verifying the BC's paraphrase of that decision's CONTENT is
accurate (which prior passes did check, and did find clean). This pass is the first to mechanically
re-derive "which ADR-025 decision actually established the 262144 cap" from ADR-025's own text
rather than trusting the inherited citation. Per BC-5.39.001, a finding after a clean pass (pass-34)
resets the streak: **1/3 → 0/3.** This is empirical confirmation of the asymptotic-floor reality
already recorded at D-1091 — each fresh-context adversary pass can reveal a genuinely new dimension
the prior 34 passes' codified disciplines did not cover, even after a literal zero-finding CLEAN
result.

## Part C — State at Close of Review

BC-4.17.001 **v1.15→v1.16** (F-P35-001 loci 1 + F-P35-002). BC-5.40.001 **v1.14→v1.15** (F-P35-001
loci 2+3). ADR-046 **v1.16 UNCHANGED** (architect audit: sole cross-ADR anchor ADR-025 §12.2
verified correct, no edit). BC-7.07.001 **v1.33 UNCHANGED** (audited, confirmed clean, no edit).
BC-5.39.001 3-CLEAN streak: **1/3 → RESETS to 0/3.** Gate history to date: 35 passes run against
evolving/frozen sets; 29 genuine findings found and fixed (27 prior + this pass's 2), plus 4
audit-extra stragglers (2 at pass-31, 2 at pass-33); 1 pass (34) was literal-CLEAN, now superseded
by this reset.

**NEXT: fresh pass-36** against the newly-frozen set (ADR-046 v1.16 + BC-4.17.001 v1.16 +
BC-5.40.001 v1.15 + BC-7.07.001 v1.33); needs 3 consecutive clean passes (36, 37, 38) for literal
3-CLEAN convergence, applying the ADR §Decision anchor audit dimension codified this burst as a
standing discipline alongside the three prior convergence-technique disciplines (version-stable
directive, 4-leg parity, grep-complete inputs audit). S-17.05 TDD implementation remains gated on
convergence. The human decision this session remains to CONTINUE looping toward literal 3-CLEAN
convergence (not accept D-386 Option C asymptotic acceptance).
