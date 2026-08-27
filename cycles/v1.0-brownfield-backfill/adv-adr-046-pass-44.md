# ADR-046 Adversarial Spec-Convergence Review — Pass 44

**Reviewed artifact set (frozen):** ADR-046 v1.17 + BC-4.17.001 v1.20 + BC-5.40.001 v1.17 + BC-7.07.001 v1.34
**Review date:** 2026-08-27
**Verdict:** NO BLOCKER/HIGH/MED findings — ONE non-blocking LOW observation (O-P44-001), FIXED this burst
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 0/3 → **STAYS 0/3** (fix supersedes the reviewed set — see Part C disposition; fresh 3-clean count begins at pass-45 against the corrected set)
**D-chain:** D-1101

## Part A — Finding Set (frozen set: ADR-046 v1.17 + BC-4.17.001 v1.20 + BC-5.40.001 v1.17 + BC-7.07.001 v1.34)

**HIGH (0):** none this pass.

**MEDIUM (0):** none this pass.

**LOW (0 blocking; 1 non-blocking observation, FIXED this burst):**

- **O-P44-001 (LOW, POLICY 4/5, illustrative-quote misattribution, fixed).** BC-5.40.001's
  v1.17 `last_amended` disposition prose (the entry recording the pass-43 F-P43-001
  `capabilities.md` `inputs:` fix) illustrated the newly-added citation with a parenthetical
  purporting to quote CAP-031's verbatim description text — `('this BC defines the
  authoritative lock state data structure...')` — but that phrase is in fact this BC's OWN
  Capability Anchor Justification prose (the BC's own argument for why CAP-031 is its correct
  capability anchor), not any text appearing in CAP-031's entry in
  `.factory/specs/domain-spec/capabilities.md`. Ground truth, verified by opening
  `capabilities.md` §CAP-031 directly: the capability's actual verbatim description opens
  "Enforce single-writer cross-session exclusivity on factory-artifacts state" (with a
  separate mechanism sub-clause, "TTL is 45 minutes with mid-burst renewal"). The v1.17 prose
  did not fabricate a false capability relationship — CAP-031 genuinely is this BC's correct
  anchor, and the underlying `inputs:` fix (F-P43-001) is itself correct and unaffected — the
  defect is narrowly that the illustrative parenthetical quoted the wrong source text (this
  BC's own justification sentence) while presenting it as CAP-031's description.

  **Sibling-parity check (in-scope, this pass).** BC-4.17.001 v1.20's own analogous
  illustrative quote for its identical F-P43-001 `capabilities.md` fix — `"TTL is 45 minutes
  with mid-burst renewal"` — and BC-7.07.001 v1.34's own analogous illustrative quote —
  CAP-032's title, `"Guarantee lossless context-window transitions via wave-boundary
  checkpoint and PreCompact flush"` — were each independently re-verified against their cited
  capability's actual description/title text in `capabilities.md`. Both siblings CONFIRMED
  CORRECT — the quoted text is genuinely present, verbatim, in each capability's own entry.
  Neither sibling BC required an edit; the misattribution is confined to BC-5.40.001 alone.

**Zero BLOCKING findings at any severity. VERDICT: the reviewed set is substantively
CONVERGED.** The behavioral core (write-composition, five-outcome table, identity-gating,
event-sourcing) remains independently re-verified CLEAN for the 18th consecutive pass (since
pass-27). O-P44-001 is a cosmetic citation-accuracy defect confined to one dated historical
disposition paragraph — it does NOT touch `inputs:` completeness itself (`capabilities.md`
remains correctly listed and remains load-bearing), design substance, or any PC/Invariant/EC.
Per BC-5.39.001, a non-blocking observation does not itself reset a streak — but because
product-owner elected to FIX O-P44-001 rather than accept-and-track it (see Part C
disposition), the frozen set's bytes changed during this pass, so pass-44 cannot be counted as
a continuing clean-streak advance against the SAME unchanged set the discipline requires.

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set
on its own merits only). Every dimension this gate's 43-pass history has ever found a defect in
was independently re-checked against the current frozen set and confirmed holding, with zero
regression:

- **Arm-parity what-vs-how reconciliation + locus-class-extension (D-1096/D-1097 classes,
  sixth+seventh disciplines):** all eleven `extract_frontmatter`-guarantee loci across
  BC-4.17.001 re-derived and confirmed consistently arm-split — no regression, FOURTH
  consecutive confirmation (following pass-41, pass-42, pass-43).
- **ADR §Decision/§N.M anchor correctness (D-1092/F-P35-001 class):** every citation
  independently re-derived from the cited ADR's own section content — all CORRECT.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):** recounted —
  confirmed 6 numbered decisions, correctly cited "1–6" throughout.
- **Grep-complete cluster-wide `inputs:` completeness (D-1090/D-1100, eighth discipline):**
  re-audited all four artifacts' own `inputs:` arrays — zero omissions found beyond the
  citation-quality issue captured as O-P44-001 above (which is a quote-accuracy defect, not an
  `inputs:`-completeness gap — `capabilities.md` IS present and correctly cited on all three
  BCs).
- **AC-owning-artifact cross-reference discipline (D-1100):** re-checked every AC-NNN-shaped
  citation across the frozen set for owning-artifact accuracy — none found; the F-P43-002
  AC-018 fix holds with no regression.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts — stable
  since pass-27 (18 consecutive passes now, counting this one). **This pass's sole observation
  is confined entirely to the provenance/citation-illustration perimeter — it does not touch
  the behavioral core.**
- **Every load-bearing code claim (function names, file paths, constant names) independently
  re-verified against the actual source files:** all accurate.
- **`modified:`-array-head-parity (4-leg head==version self-check, D-1089):** all four
  artifacts confirmed prior to this pass's edit.
- **`last_amended` bracket-balance, cross-anchor citation accuracy, type-provenance, POLICY 19
  anti-volatile-pin, §Story Anchor/Traceability parity, subsystem labels, status/lifecycle
  pairs:** all re-verified clean — no regression on any previously-codified dimension beyond
  the one observation above.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) prior to this burst's edit; this burst edits only ONE of the four cluster
  artifacts (BC-5.40.001), so the cyclic-recompute cascade this tangle otherwise triggers does
  NOT re-activate this burst.

**Novelty assessment:** O-P44-001 is a genuinely NEW class of citation defect for this gate —
not a repeat of the `inputs:`-completeness gap this same disposition paragraph itself just
fixed (F-P43-001), but a fresh accuracy defect in the ILLUSTRATIVE QUOTE that describes that
fix, introduced in OUR OWN pass-43 remediation prose this session (not inherited, pre-existing
history — contrast O-P42-001). The two sibling BCs' equivalent prose got this right,
demonstrating the correct pattern was known and simply not applied consistently across all
three BCs in the same burst. **Per BC-5.39.001, only a BLOCKING finding resets the streak; this
observation was non-blocking.** However, because it was FIXED (not accepted), the reviewed
set's bytes no longer match what pass-44 evaluated, so this pass is recorded as a
governance-fix pass rather than a counted clean-streak advance — see Part C.

## Part C — State at Close of Review

BC-5.40.001 **v1.17→v1.18** (O-P44-001 fix, product-owner). ADR-046 **v1.17 UNCHANGED**;
BC-4.17.001 **v1.20 UNCHANGED**; BC-7.07.001 **v1.34 UNCHANGED** (all three re-audited,
confirmed clean, no edit). BC-5.39.001 3-CLEAN streak: **0/3 → STAYS 0/3.**

**Fix-vs-accept governance disposition (distinct from the O-P42-001 precedent at D-1099).**
Unlike O-P42-001 — a PRE-EXISTING, PRE-ADR-046, dated-historical `modified:`-array cosmetic
asymmetry, correctly ACCEPTED-and-tracked at D-1099 because fixing it mid-2/3-streak would have
broken the byte-unchanged invariant that streak then depended on, forfeiting 2 already-banked
clean passes for a pre-existing item unrelated to the current burst — O-P44-001 is: (a) a FRESH
misattribution introduced in this session's own pass-43 remediation prose (not inherited
history), (b) a defect class the two sibling BCs' equivalent prose already got right in the
SAME burst, evidencing the correct pattern was available and simply unapplied to BC-5.40.001
specifically, and (c) found while the streak was ALREADY at 0/3 (pass-43's own FINDINGS verdict
had already reset the streak before this pass ran) — so fixing it here costs ZERO additional
streak whatsoever, unlike a fix mid-2/3-streak which would have cost 2 banked clean passes for
no operative gain. The correct governance call at 0/3, for a fresh in-session correctable
inaccuracy the siblings got right, is FIX — not accept-and-track. Fixed same-burst by
product-owner. Gate history to date: 44 passes run against evolving/frozen sets; 34 genuine
BLOCKING findings found and fixed, plus 4 audit-extra stragglers (pass-31, pass-33) and 1
latent-bracket drain (pass-37, not counted as genuine); 1 ACCEPTED non-blocking observation
(O-P42-001, does not count against the streak) and 1 FIXED non-blocking observation this pass
(O-P44-001); 5 clean passes (34, 36, 38, 41, 42) plus this pass-44 (zero-blocking, but not
counted toward the streak since the set was edited mid-pass).

**Streak-stays rationale (mechanical).** BC-5.39.001's literal-3-CLEAN discipline credits a
clean-streak advance only for a pass that returns CLEAN against a set that remains
BYTE-UNCHANGED through the review (the same discipline that made O-P42-001 accept-and-track the
correct call at 2/3). Because product-owner's fix changes BC-5.40.001's bytes, the exact set
pass-44 reviewed no longer exists — there is nothing left to "advance the streak against."
Pass-44 is therefore recorded as a governance-fix pass: zero BLOCKING findings (so it does not
count as a RESET either), but zero streak credit (so it does not count as an ADVANCE). The
streak numerically STAYS at 0/3, and a fresh 3-clean count begins at pass-45 against the
newly-corrected set.

**Index reconciliation (state-manager, this burst):** BC-INDEX v5.09→**v5.10** (BC-5.40.001 row
version-chain cell +v1.18 appended). ARCH-INDEX v3.87, STORY-INDEX v4.391, VP-INDEX v2.79
UNCHANGED (only BC-5.40.001 touched this burst — no ADR/story/VP content changed).

**Input-hash recompute:** BC-5.40.001 only (the sole artifact edited this burst):
`b711178`→`e5499da`, confirmed matching via `compute-input-hash --check` (exit 0 post-update).
ADR-046/BC-4.17.001/BC-7.07.001 input-hashes UNCHANGED — this burst does not re-enter the
[D-1082] 4-artifact cyclic tangle, since only one of the four cluster artifacts was edited (no
cross-citation shift propagates to the other three, which do not cite BC-5.40.001's changed
`last_amended` prose in any load-bearing way their own `inputs:` hashing depends on).

**NEXT: fresh pass-45** against the newly-corrected set (ADR-046 v1.17 + BC-4.17.001 v1.20 +
BC-5.40.001 **v1.18** + BC-7.07.001 v1.34), starting a FRESH 3-clean count at 0/3, applying all
eight now-codified convergence-technique disciplines proactively from the start, plus treating
O-P44-001's fix as already applied (not a fresh finding to re-litigate) and its underlying
lesson — illustrative quotes must cite the ACTUAL verbatim source text, cross-checked against
sibling disposition narratives in the same burst — as a ninth discipline to apply proactively.
The human decision this session remains to CONTINUE looping toward literal 3-CLEAN convergence
(not accept D-386 Option C asymptotic acceptance). S-17.05 TDD implementation remains gated on
convergence.
