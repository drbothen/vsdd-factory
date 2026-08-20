---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "de77a25"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 9
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-8.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 9)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.8 (input-hash `97029a5`); `BC-1.03.017.md` v1.17; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `f0d95b79` (D-1048 commit).
Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN

1 MEDIUM finding (streak-resetting). 1 non-resetting observation. Multiple grounding
confirmations.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued at pass-2 (F-S2111V2-P2-001..005), pass-3
(F-S2111V2-P3-001), pass-4 (F-S2111V2-P4-001), pass-5 (F-S2111V2-P5-001), pass-6
(F-S2111V2-P6-001), pass-7 (F-S2111V2-P7-001), and pass-8 (F-S2111V2-P8-001).

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P8-001 | MEDIUM | RESOLVED | story-writer's defensive numeric-magnitude scan re-verified this pass — Task #32's directive reads `timeout_ms >= 30_000`; no `timeout_ms` site carries a stray `M` suffix and no `fuel_cap` site is mis-scaled outside the documented AC-002 low-fuel fixture. |

## Part B — New Findings (or all findings for pass 1)

### MEDIUM

#### F-S2111V2-P9-001 (MEDIUM, streak-resetting)

**Location:** `BC-1.03.017.md` v1.17, live `## Traceability` table, ADR row, §Decision 3
sub-clause.

**Defect:** the §Decision 3 sub-clause cites the break-glass amendment's delivery vehicle two
contradictory ways **in the same table cell**: it opens with "...v1.9 amendment: mandatory
authenticated break-glass companion, S-21.17" — citing **S-21.17**, a retired, never-authored
story ID absent from STORY-INDEX (retired per BC-1.03.018's own Stories row, which already
carries the correct "the prior follow-up name S-21.17 is retired" annotation) — and then, later
in that same cell, states "...delivered WITHIN S-21.11...governed by sibling BC-1.03.018...". The
cell asserts two different delivery vehicles for the identical mechanism within one citation. This
BC's own v1.11 changelog entry (architect, sibling-sweep citation update, same burst as ADR-039
v1.9→v1.10) claimed the S-21.17→S-21.11 citation redirect was complete across BC-1.03.017, but
this one occurrence in the live Traceability row was missed by that sweep — a partial-sweep
residual, four bursts stale (v1.11 through v1.17 all retained it unnoticed because none of the
intervening passes 3/4/5/6/7/8 touched this specific sub-clause).

**Routed:** product-owner (rewrite the live cite to remove the retired S-21.17 reference and match
the same cell's "delivered WITHIN S-21.11" clause and BC-1.03.018's retirement-annotation
convention; run a literal grep sweep of both BC-1.03.017 and BC-1.03.018 for `S-21\.17`,
classifying every occurrence by captured evidence — TD-VSDD-060).

**RESOLVED this burst:**
- **product-owner** — BC-1.03.017 v1.17→v1.18: rewrote the live §Decision 3 cite to "...mandatory
  authenticated break-glass companion, delivered within S-21.11 (prior follow-up name S-21.17
  retired))" — matching the same cell's "delivered WITHIN S-21.11" clause and BC-1.03.018's
  Stories-row retirement-annotation convention verbatim ("the prior follow-up name S-21.17 is
  retired"). Ran a literal grep sweep (`grep -no "S-21\.17"`) of both BC-1.03.017 and BC-1.03.018
  and classified every occurrence by captured evidence, not assertion: BC-1.03.017 had 4 line-hits
  — line 99 (frontmatter `last_amended`, describing v1.11's own historical S-21.17→S-21.11
  redirect: HISTORICAL, left as-is), line 1016 (the live `## Traceability` ADR row, the
  F-S2111V2-P9-001 site: LIVE, fixed), line 1031 (v1.11 changelog row narrating the S-21.17
  amendment as it stood at v1.10: HISTORICAL, left as-is), line 1060 (v1.11 changelog row
  repeating the same historical narration: HISTORICAL, left as-is). BC-1.03.018 had 1 line-hit —
  its Stories row ("the prior follow-up name S-21.17 is retired"): already correctly annotated,
  no edit needed. 1 live site fixed; 3 historical sites correctly left unchanged (POLICY 5
  historical-narrative exemption). POLICY 8 parity preserved (no PC/Precondition/Invariant/AC
  content altered — this is a citation-only fix within the Traceability row).
- **story-writer** — S-21.11 v2.8→v2.9: swept 60 live `BC-1.03.017 v1.17`→`v1.18` cites via the
  D-1047-codified whitespace-normalized/multiline detector (`tr '\n' ' ' < file |
  grep -oE 'BC-1\.03\.017 +v1\.[0-9]+' | sort | uniq -c`) — zero non-v1.18 live residue confirmed
  by captured stdout. Defensively grepped the entire story for `S-21\.17` — zero occurrences found
  anywhere in the file, live or historical; the story correctly self-identifies as S-21.11
  throughout. `input-hash` (`97029a5`) intentionally left UNCHANGED — the story's declared
  `inputs:` (ADR-039, hooks-registry.toml, `wasm-fuel-exhaustion-detection.md`) do NOT include
  BC-1.03.017 and none of them changed this burst.
- **state-manager (this burst)** — pass-9 report persisted verbatim; INDEX.md pass-9 row +
  Convergence Status advance; BC-INDEX v4.81→v4.82 (BC-1.03.017 row +v1.18; title cell
  UNCHANGED — verbatim H1 subset confirmed; `total_bcs` UNCHANGED 1986); STORY-INDEX
  v4.368→v4.369 (S-21.11 catalog row: story cite v2.8→v2.9; BC-array cite v1.17→v1.18); BC-1.03.018
  input-hash reconciled `43d1e13`→`ff9c2d5` via the operator-authoritative rc.23 binary (its own
  declared `inputs:` include BC-1.03.017, which changed content this burst); S-21.11 story
  input-hash `97029a5` content-currency re-confirmed via `compute-input-hash --check` (exit 0 —
  see Part (b) of the OBSERVATION below); ARCH-INDEX v3.73 / VP-INDEX v2.76 UNCHANGED (no ADR/VP
  content changed this burst); TD-VSDD-060 sibling-sweep confirmed clean (no other live artifact
  carries a stale BC-1.03.017 v1.17-or-earlier cite or a live retired S-21.17 cite); this D-1049
  decision-log.md entry; STATE.md advance; single atomic commit to `factory-artifacts` per
  TD-VSDD-053.

### Confirmations (converged)

- Version-cite parity CLEAN including line-wrapped sites (normalized detector): all live
  `BC-1.03.017 v1.18` / `BC-1.03.018 v1.1` / `BC-1.01.016 v1.3` citations agree post-remediation.
- Numeric-magnitude parity CLEAN — no regression from pass-8's fix (`timeout_ms` uniformly
  `_000`-scale; `fuel_cap` uniformly M-scale except the documented AC-002 low-fuel fixture),
  consistent with ADR-039 §Decision 4's `fuel_cap ≥ max(p99×1.5, 50M)` /
  `timeout_ms ≥ max(p99_ms×2.0, 30_000)` formulas.
- Predicate-coherence / axes-independence remain CLEAN across BC and story (no regression from
  pass-6/pass-7/pass-8 remediations).
- 18-entry `on_error="block"` `hooks-registry.toml` plugin enum remains EXACT against AC-024–
  AC-041 and PC13's Coverage Set table; `agent-gate` priorities 120/130 and `timeout_ms=10000`
  confirmed consistent.
- POLICY 7 (BC H1 ↔ BC-INDEX title cell) and POLICY 8 (bc-array/body-table/AC propagation) parity
  both hold.
- Erratum E-005's re-ratification-not-required disposition remains SOUND.
- `VP-TBD` on BC-1.03.017/BC-1.03.018 (`[F-007]`) remains a sanctioned, previously-disclosed
  deferral — re-observed unchanged, not a new finding.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 0 |
| ADVISORY | 0 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate (streak resets 0/3; pass-10 required)
**Readiness:** requires revision (routed product-owner + story-writer; RESOLVED this same burst)

## Grounding confirmations (non-findings, independently re-derived)

- **Story input-hash content-currency confirmed.** `compute-input-hash --check` (operator-
  authoritative marketplace rc.23 binary, per-file, per L-EDP1-073) against the S-21.11 story
  exits 0 — `97029a5` remains content-current against the story's declared `inputs:` (ADR-039,
  `wasm-fuel-exhaustion-detection.md`, `hooks-registry.toml`), none of which changed this burst.
- **BC-1.03.018 input-hash drift confirmed and reconciled.** `compute-input-hash --check` against
  BC-1.03.018 returned DRIFT (`43d1e13` ≠ computed `ff9c2d5`) prior to this burst's fix, because
  BC-1.03.017 (a declared input) changed content at v1.18. `--update` reconciled it to `ff9c2d5`;
  a subsequent `--check` returns exit 0.

## Observations (non-resetting)

- **`[POLICY 18]` [ADDRESSED this burst]** the adversary (read-only) could not itself recompute the
  story's input-hash to confirm `97029a5` remains content-current against ADR-039 v1.13/
  hooks-registry.toml/wasm-fuel-exhaustion-detection.md byte content — POLICY 18
  content-currency-vs-cite-parity distinction. Closed this burst: state-manager ran
  `compute-input-hash --check` (operator-authoritative rc.23 binary) against the story, exit 0,
  confirming `97029a5` is content-current, not merely cite-consistent.
- **`[carry-forward, = known F-007]`** BC-1.03.017/BC-1.03.018 both still carry `VP-TBD` under
  POLICY 9 — pre-existing, tracked in STATE.md Blocking Issues/Drift Items, anchored to a future
  dedicated VP-authoring pass. Not a new finding; re-observed unchanged.

## Novelty Assessment

Novelty MEDIUM — this is NOT a new defect class for the cascade; it is a recurrence of the
D-1006 version-cite-propagates/algorithm-content-does-not family (passes 3/4/5/6), but with a
new failure mode: a **partial-sweep residual surviving four intervening bursts** (v1.11 through
v1.17, i.e. D-1043 through D-1048, none of which touched this specific §Decision 3 sub-clause)
before finally surfacing. The v1.11 sibling-sweep citation-update burst (architect,
2026-08-19) asserted the S-21.17→S-21.11 redirect was complete across BC-1.03.017, but a targeted
citation-sweep — like a targeted content-sweep — can miss an occurrence stated in different
surrounding prose (here: a single cell containing BOTH the stale and the corrected framing, which
a naive single-pattern-match sweep can satisfy by matching only the corrected half). This
reinforces D-1045(h)/D-1046(h)'s standing lesson (semantic, not literal-string, sweeps) one layer
further: even a sweep that DOES find and fix an occurrence in a cell can still leave a
contradictory OTHER occurrence in the SAME cell unaddressed, because the fix satisfies a
literal-pattern re-check without a re-read of the full cell's semantic content. No new codified
lesson beyond the existing D-1045(h)/D-1046(h)/D-1047(h) family — this is treated as a further
confirmatory instance of the "sweeps must be semantic, and must re-read the FULL scope of the
edited unit (here: the whole table cell, not just the matched substring)" discipline already
standing.
