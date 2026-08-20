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
pass: 10
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-9.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 10)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.9 (input-hash `97029a5`); `BC-1.03.017.md` v1.18; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `4e084433` (D-1049 commit).
Rubric: full `.factory/policies.yaml` (POLICY 1-22).

## Verdict: CLEAN

Zero BLOCKER/HIGH/MEDIUM streak-resetting findings. 3 LOW/ADVISORY non-resetting observations. This
is the FIRST clean pass of the S-21.11 v2 cascade — BC-5.39.001 streak ADVANCES **0/3 → 1/3**.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued at pass-2 (F-S2111V2-P2-001..005), pass-3
(F-S2111V2-P3-001), pass-4 (F-S2111V2-P4-001), pass-5 (F-S2111V2-P5-001), pass-6
(F-S2111V2-P6-001), pass-7 (F-S2111V2-P7-001), pass-8 (F-S2111V2-P8-001), and pass-9
(F-S2111V2-P9-001). This pass's 3 LOW/ADVISORY observations use `F-S2111V2-P10-001..003`.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P9-001 | MEDIUM | RESOLVED | Re-verified BC-1.03.017's live `## Traceability` ADR row §Decision 3 sub-clause now reads a single, non-contradictory delivery-vehicle citation ("...delivered within S-21.11 (prior follow-up name S-21.17 retired)"). A fresh `grep -no "S-21\.17"` of BC-1.03.017 confirms the 3 remaining hits are all in the frontmatter `last_amended` string and the v1.11 changelog rows (HISTORICAL, POLICY 5 exempt) — no live `S-21.17` reference remains. BC-1.03.018's Stories row remains correctly annotated. |

## Part B — New Findings (none; CLEAN pass)

No BLOCKER, HIGH, or MEDIUM findings. No streak-resetting findings.

### Independently re-derived converged axes (all CLEAN, fresh-context)

- AC↔PC↔EC↔Task↔Invariant cross-reference integrity: BC-1.03.017 PC1..PC13 all trace to their
  governing ACs; BC-1.03.018 PC1..PC10 all trace to AC-014..AC-023; all cross-references
  bidirectional; no dangling or off-by-one references found.
- Counts: BC-1.03.017 13 PCs / 11 Invariants / 11 ECs / 5 Preconditions confirmed by direct count;
  BC-1.03.018 10 PCs / 6 Invariants / 6 ECs confirmed; PC13's Coverage Set table = 18 rows,
  matching the story's AC-024..AC-041 range (18 ACs) exactly.
- The 18-entry `on_error="block"` `hooks-registry.toml` plugin-name enum matches PC13's Coverage
  Set table and AC-024–AC-041 EXACTLY, byte-for-byte, no additions or omissions.
- Subsystems `[SS-01, SS-02, SS-04, SS-07]` remain justified against each BC's actual scope and
  agree with ARCH-INDEX.md's subsystem registry.
- Index parity CLEAN under POLICY 3/7/8/18: BC H1 titles match BC-INDEX title cells verbatim;
  bc-array/body-table/AC propagation intact; frontmatter `input-hash` fields match a fresh
  three-way check (STORY-INDEX cite, story frontmatter, story body content).
- Token Budget cell: 60,000 tokens = 30.0% of the story's stated ceiling, arithmetic re-verified.
- Erratum E-005's re-ratification-not-required disposition remains legitimate — the narrowed
  two-leg predicate form is unchanged since D-1043 and does not require a fresh ratification cycle.
- Predicate-coherence / axes-independence hold across every site restating PC13's additive-only
  concept in both BC-1.03.017 and the story (no regression from the pass-5/pass-6/pass-9
  remediations).
- Numeric-magnitude parity CLEAN — `timeout_ms` uniformly `_000`-scale, `fuel_cap` uniformly
  M-scale except the documented AC-002 low-fuel fixture (`fuel_cap = 100`) — consistent with
  ADR-039 §Decision 4's `fuel_cap >= max(p99*1.5, 50M)` / `timeout_ms >= max(p99_ms*2.0, 30_000)`
  formulas (no regression from pass-8).
- No live reference to the retired `S-21.17` story ID survives anywhere in the story or either BC
  (no regression from pass-9's fix).
- Semantic anchoring: every module/file/function anchor cited in the story and both BCs resolves
  against the actual codebase layout under `.factory/specs/architecture/`.
- `VP-TBD` on BC-1.03.017/BC-1.03.018 (`[F-007]`) remains a sanctioned, previously-disclosed
  deferral — re-observed unchanged, not a new finding.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 2 |
| ADVISORY | 1 |

**Overall Assessment:** pass-clean
**Convergence:** streak ADVANCES 0/3 → **1/3**. Two further FRESH CONSECUTIVE CLEAN passes
(pass-11, pass-12) required for BC-5.39.001 3-CLEAN convergence.
**Readiness:** no action required this pass; bundle should remain STABLE for pass-11.

## Observations (non-resetting)

- **F-S2111V2-P10-001 (LOW):** the story's §"Scope Elements and Task Ordering (DAG)" node-(D) box
  enumerates only "AC-002..AC-006, AC-010, AC-011" in its illustrative caption, but Node (D) also
  authoritatively owns Task #19b (AC-013b) and Task #19c (AC-024..AC-041) per the authoritative
  Tasks section. The Tasks section itself is complete and correct — this is a caption-staleness
  nit in the illustrative DAG diagram only, not a coverage gap. **Disposition: DOCUMENTED, deferred
  to a single consolidated cosmetic sweep at convergence-close** (not remediated this burst, to
  preserve bundle stability for passes 11-12).
- **F-S2111V2-P10-002 (LOW):** the DAG/AC-011 reference label "Executor Extension" is not verbatim
  against the Tasks heading "Node (D) — Executor decision-function extension." The reference
  resolves unambiguously (no reader could confuse the two), so this is a readability nit, not a
  correctness defect. **Disposition: DOCUMENTED, deferred to the same convergence-close cosmetic
  sweep.**
- **F-S2111V2-P10-003 (ADVISORY):** the break-glass ACs (AC-015/AC-016/AC-019) cite EC-004/
  EC-005/EC-006 using BC-1.03.018's own internal EC numbering, while the story's own renumbered
  edge-case list uses EC-012/EC-013/EC-014 for the identical concepts. The "BC-1.03.018 PCn"-style
  prefix used at each citation site disambiguates the two numbering schemes unambiguously — this
  is a readability nit, not a broken reference. **Disposition: DOCUMENTED, deferred to the same
  convergence-close cosmetic sweep.**
- **`[carry-forward, = known F-007]`** BC-1.03.017/BC-1.03.018 both still carry `VP-TBD` under
  POLICY 9 — pre-existing, tracked in STATE.md Blocking Issues/Drift Items, anchored to a future
  dedicated VP-authoring pass. Not a new finding; re-observed unchanged.

## Novelty Assessment

Novelty LOW — a CLEAN pass with 3 cosmetic LOW/ADVISORY observations (2 caption/label staleness
nits in the story's illustrative DAG diagram, 1 cross-BC EC-numbering readability nit), none of
which touch predicate content, index parity, numeric magnitudes, or cite propagation. No new
defect class. All 3 observations are deliberately left unremediated this burst — per explicit
dispatch scoping — to keep the reviewed bundle byte-identical for passes 11 and 12, consistent with
the BC-5.39.001 3-CLEAN protocol's requirement that consecutive clean passes review a STABLE
artifact. They are routed to a single consolidated cosmetic sweep at convergence-close (after
pass-12, before TDD handoff), rather than three separate one-line touches that would each restart
the streak.
