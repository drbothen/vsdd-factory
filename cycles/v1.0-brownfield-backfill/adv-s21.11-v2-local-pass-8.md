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
input-hash: "1b3f3f5"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 8
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-7.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 8)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.7 (input-hash `97029a5`); `BC-1.03.017.md` v1.17; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `3e6cbcf5` (D-1047 commit).
Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN

1 MEDIUM finding (streak-resetting). Multiple grounding confirmations.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued at pass-2 (F-S2111V2-P2-001..005), pass-3
(F-S2111V2-P3-001), pass-4 (F-S2111V2-P4-001), pass-5 (F-S2111V2-P5-001), pass-6
(F-S2111V2-P6-001), and pass-7 (F-S2111V2-P7-001).

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P7-001 | MEDIUM | RESOLVED | story-writer's whitespace-normalized/multiline sweep re-verified this pass — the wrapped Task #29 cite reads `BC-1.03.017 v1.17 PC11`; no other line-wrapped stale cite found anywhere in the story body. |

## Part B — New Findings (or all findings for pass 1)

### MEDIUM

#### F-S2111V2-P8-001 (MEDIUM, streak-resetting)

**Location:** S-21.11 story Task #32 (Phase 5), the CHANGELOG-entry directive for the
`[Unreleased] > Security` note.

**Defect:** the directive states the host-wall-clock-timeout floor as `timeout_ms >= 30M`
(30,000,000 ms ≈ 8.3 hours). This is a **1000×-wrong numeric magnitude**: every other live
citation of the timeout floor in the story — AC-001's TIMEOUT-POSITIVE/TIMEOUT-NEGATIVE-CONTROL
fixtures, AC-009, PC8, PC9, and Invariant 8 — states the floor as `timeout_ms >= 30_000`
(30,000 ms = 30 s). The `M` suffix (a millions-scale marker) is a **pattern-copy artifact** from
the immediately adjacent, CORRECT `fuel_cap >= 50M` clause in the same directive sentence:
`fuel_cap` legitimately uses the M/millions scale (50,000,000/20,000,000/75,000,000 across the
story's fuel-cap citations), but `timeout_ms` does not — it is milliseconds-scale throughout
every other site in the story (10_000/30_000/45_000/2_000/120_000). If shipped as written, this
CHANGELOG entry would publish a security-calibration figure to the public record that is wrong by
three orders of magnitude — a validator "fail-closed" timeout floor of 8.3 hours reads as
effectively no timeout at all, the opposite of the calibration this story exists to enforce
(ADR-039 Decisions 2/3/4/6, CWE-636 closure).

**Routed:** story-writer (fix Task #32's directive text; run a defensive numeric-magnitude scan of
every `timeout_ms` and `fuel_cap` citation in the story to rule out further siblings of the same
copy-paste-suffix error).

**RESOLVED this burst:**
- **story-writer** — S-21.11 v2.7→v2.8: fixed `timeout_ms >= 30M` → `timeout_ms >= 30_000` in
  Task #32's directive text (the adjacent `fuel_cap >= 50M` left unchanged — it is correct). Ran a
  **defensive story-wide numeric-magnitude scan** per dispatch instruction
  (`grep -noE 'timeout_ms[^,.;)]{0,40}'` filtered for stray `M`-suffix hits;
  `grep -noE 'fuel_cap[^,.;)]{0,40}'` filtered for non-M/non-`_000_000`-scale hits) — zero further
  sites found. Every other `timeout_ms` citation in the story (AC-001/AC-007/AC-009/AC-013/EC-010/
  EC-011/Tasks #7/#9/#12/#13/#28/#32 and the TIMEOUT-POSITIVE/NEGATIVE-CONTROL fixtures) is already
  in the `_000`-ms scale; every `fuel_cap` citation is already in the M/`_000_000` scale, except the
  deliberately-tiny `fuel_cap = 100` AC-002 bats-integration fixture, which its own surrounding
  prose documents as a low-fuel test case (not a calibration-floor claim) and is correctly scaled
  for that purpose. Cross-checked the "four of the five targeted bash-adapter plugins currently
  default to `timeout_ms = 10_000`" claim (line 270) against the other `timeout_ms = 10_000`
  TIMEOUT-POSITIVE-CONTROL fixture citations (AC-001 body, Task #28) — internally consistent, no
  conflict. `input-hash` (`97029a5`) intentionally left UNCHANGED — no declared `inputs:` file
  changed this burst; state-manager re-verifies below.

### Confirmations (converged)

- PC11's Task #29 wrapped-cite fix from pass-7 re-verified stable — reads `v1.17 PC11`, no
  regression.
- Version-cite parity CLEAN including line-wrapped sites (normalized detector): all live
  `BC-1.03.017 v1.17` / `BC-1.03.018 v1.1` / `BC-1.01.016 v1.3` citations agree.
- Predicate-coherence / axes-independence remain CLEAN across BC and story (no regression from
  pass-6/pass-7 remediations).
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
**Convergence:** findings remain — iterate (streak resets 0/3; pass-9 required)
**Readiness:** requires revision (routed story-writer; RESOLVED this same burst)

## Grounding confirmations (non-findings, independently re-derived)

- **Story input-hash three-way parity confirmed.** `compute-input-hash` (operator-authoritative
  marketplace rc.23 binary, per-file, per L-EDP1-073) against the S-21.11 story returns `97029a5`,
  matching the frontmatter `input-hash` and the STORY-INDEX catalog row. Unchanged by this burst:
  none of the story's declared `inputs:` files (ADR-039, `wasm-fuel-exhaustion-detection.md`,
  `hooks-registry.toml`) changed.
- **Numeric-magnitude sweep found no sibling defects.** The `M`-suffix pattern-copy error was
  confined to Task #32's single citation; no other `timeout_ms` site in the story carries a
  stray `M` suffix, and no `fuel_cap` site is mis-scaled outside the documented AC-002 low-fuel
  test fixture.

## Observations (non-resetting)

- **`[carry-forward, = known F-007]`** BC-1.03.017/BC-1.03.018 both still carry `VP-TBD` under
  POLICY 9 — pre-existing, tracked in STATE.md Blocking Issues/Drift Items, anchored to a future
  dedicated VP-authoring pass. Not a new finding; re-observed unchanged.

## Novelty Assessment

Novelty MEDIUM — a NEW defect class for this cascade, distinct from both the D-1006 version-cite-
propagates/algorithm-content-does-not CONTENT family (passes 3-6) and the pass-7 sweep-METHODOLOGY
gap. This is a **numeric-magnitude pattern-copy error**: a unit-scale suffix (`M`, millions) that
is correct for one adjacent parameter (`fuel_cap`) was copied onto a different parameter
(`timeout_ms`) that uses a different unit scale (milliseconds, not millions), producing a
1000×-wrong floor. The error carried real risk had it shipped: a public CHANGELOG entry
documenting a security-calibration floor is exactly the kind of artifact an operator or downstream
integrator would read and trust without cross-referencing every AC. Closed this burst by a direct
numeric fix plus a defensive magnitude-scan of every `timeout_ms`/`fuel_cap` site in the story
(not just the one flagged site), confirming no sibling instance of the same copy-paste-suffix
error survives elsewhere in the document.
