---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-19T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "ba15b68"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 3
cascade: S-21.11-v2
previous_review: null
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 3)

> **Persistence note (state-manager, this burst):** passes 1 and 2 of the S-21.11 v2 cascade
> (findings F-S2111V2-P1-001 and F-S2111V2-P2-001..005) were recorded directly in
> `decision-log.md` D-1041/D-1042 and in STATE.md, without a standalone verbatim review file —
> the S-21.11 v2 cascade had no dedicated `INDEX.md` section or `adv-*.md` file convention until
> this burst. This file establishes that convention going forward, matching the sibling pattern
> used by the S-21.07 and S-21.09 LOCAL cascades (`adv-s21.NN-local-pass-N.md`). Content below is
> transcribed verbatim from the orchestrator-relayed adversary dispatch per POLICY 22
> (orchestrator-transcribed; the live adversary session ran in a separate context this burst
> resumes from).

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.3 (input-hash `3f97013`); `BC-1.03.017.md` v1.15; `BC-1.03.018.md` v1.1; `ADR-039` v1.12
(§AMD-003 RATIFIED); factory-artifacts bundle HEAD `4308b6a5`. Rubric: full
`.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN
1 HIGH finding (streak-resetting). 2 grounding confirmations. 2 non-resetting observations
(1 carry-forward, 1 LOW).

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>` (e.g.
`F-S2111V2-P3-001`), established at pass-1 (F-S2111V2-P1-001) and continued at pass-2
(F-S2111V2-P2-001..005). This is a project-specific finding-ID convention distinct from the
generic `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` scheme, matching the convention already in use across
the S-21.11 v2 cascade's decision-log.md entries (D-1041, D-1042).

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-S2111V2-P2-001 | HIGH | RESOLVED | S-21.11 v2.3 Task #19b predicate narrowed (story-writer, D-1042) — re-verified this pass, no residual. |
| F-S2111V2-P2-002 | HIGH | RESOLVED | ADR-039 v1.12 §AMD-003 §Status swept RATIFIED (architect, D-1042) — re-verified this pass, no residual. |
| F-S2111V2-P2-003 | HIGH | RESOLVED | BC-INDEX title cells for BC-1.03.017/BC-1.03.018 swept to verbatim H1 subsets (state-manager, D-1042) — re-verified this pass, no residual. |
| F-S2111V2-P2-004 | MEDIUM | RESOLVED | BC-1.03.017 v1.15 PC13 full-18-entry on_error=Block coverage (product-owner, D-1042) — re-verified this pass, no residual. |
| F-S2111V2-P2-005 | LOW | RESOLVED | S-21.11 v2.3 Token Budget prose/table reconciled to ~60,000/30.0% (story-writer, D-1042) — re-verified this pass, no residual. |

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S2111V2-P3-001 (HIGH, streak-resetting)

**Location:** `BC-1.03.017.md` §Architecture Anchors (PC13-extension clause) + §Traceability
(ADR row citation).

**Defect:** ADR-039 §AMD-003's "Precise rule (normative)" paragraph, as ratified at v1.12,
carried an overbroad negation predicate — `on_error == Block AND result is NOT
PluginResult::Ok { exit_code: 0, .. }`. Story-writer's own F-001 remediation at S-21.11 v2.3
narrowed the operative predicate INSIDE THE STORY BODY (Task #19b) to the correct
`on_error == Block AND result is Ok { exit_code != 0, .. }` form, but the narrowing was not
propagated upstream: BC-1.03.017 v1.15's own §Architecture Anchors PC13-extension clause and its
§Traceability ADR-039 citation both still described the broad `NOT Ok{exit_code:0}` form as the
governing predicate.

The broad form, if implemented literally, forces `Timeout { cause: Fuel | Epoch } +
on_error = Block + failure_policy = FailOpen → block`, directly contradicting PC5, PC10(a),
EC-009, and Invariant 1's axes-independence guarantee ("the axes are orthogonal") — and
reintroduces exactly the CWE-636 fail-open-vs-fail-closed self-lock class S-21.11 exists to
close. It would also turn TC-12 arm (a) — a Phase-5 green-gate prerequisite asserting
`Timeout{Epoch}+on_error=Block+failure_policy=FailOpen→exit 0` — RED.

**Routed:** architect (ADR-039 authoritative-rule correction) + product-owner (BC-1.03.017
sibling-sweep of the two BC-body sites citing the broad form).

**RESOLVED this burst:** ADR-039 v1.12→v1.13 (architect; new §Erratum E-005, "Precise rule
(normative)" paragraph rewritten to the explicit two-condition narrow form; `status: ratified`
preserved — POLICY 22 re-ratification NOT required, erratum-exempt category per §Erratum E-005's
own "Ratification note": the decision itself (option (b), narrow Ok-only extension) was already
ratified at v1.11/D-1041 exactly as stated; this is a wording/formalization correction confined
to one paragraph's own formalization, not a change to decision content). BC-1.03.017 v1.15→v1.16
(product-owner; §Architecture Anchors PC13-extension clause + §Traceability ADR-039 citation both
swept to the narrow form, with explicit MUST-NOT-be-a-negation guidance and a restatement that
the base `Crashed | Timeout` rule remains governed solely by `on_error`, unaffected). PC13's own
body prose already asserted the correct narrow form and required no edit. Sibling-swept
BC-1.03.018 (TD-VSDD-060): no occurrence of the broad-negation pattern found; no edit needed. No
AC in BC-1.03.017 restates the broad predicate (this BC carries no Acceptance Criteria section).

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 0 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — iterate (streak resets 0/3; pass-4 required)
**Readiness:** requires revision (routed architect + product-owner; both RESOLVED this same burst)

## Grounding confirmations (non-findings, independently re-derived)

- **18-entry `on_error="block"` registry set exact.** `grep -c` against the live
  `hooks-registry.toml` returns 18, matching AC-024..AC-041 one-for-one and matching PC13's
  Coverage Set table row-for-row. No drift.
- **`timeout_ms=10_000` four-of-five accurate; agent-gate priorities 130/120 confirmed.** The
  five legacy-bash-adapter.wasm-hosted plugins' calibrated `timeout_ms` figures were spot-checked
  against the registry and found accurate for four of the five (the fifth carries a distinct,
  correctly-cited value — not a defect, a genuine per-plugin calibration difference).
  `validate-wave-gate-prerequisite` (priority 130) and `validate-pr-merge-prerequisites`
  (priority 120) confirmed as DIFFERENT `execute_tiers` tiers per `routing.rs::group_by_priority`
  — the EC-005 mechanism correction from F-006/pass-1 holds.

POLICY 8 parity holds (BC-1.03.017's frontmatter `behavioral_contracts` cross-reference and body
BC-table/Token-Budget stayed internally consistent through the v1.16 edit). POLICY 7 confirmed:
BC-1.03.017's BC-INDEX title cell remains a verbatim subset of the BC's own current H1. F-005's
prior token-budget reconciliation (60,000 / 200,000 = 30.0%) re-verified consistent, no new
drift.

## Observations (non-resetting)

- **[carry-forward, = known F-007]** BC-1.03.017/BC-1.03.018 both still carry `VP-TBD` under
  POLICY 9 — pre-existing, tracked as `[F-007]` in STATE.md Blocking Issues/Drift Items, anchored
  to a future dedicated VP-authoring pass. Not a new finding; re-observed unchanged.
- **[LOW]** `STORY-INDEX.md`'s S-21.11 catalog-row narrative attributes "§AMD-003 v1.12 RATIFIED"
  as the version at which §AMD-003 became substantively ratified. §AMD-003's substantive
  ratification actually landed at ADR-039 **v1.11** (D-1041, POLICY 22, human sign-off); v1.12
  was a pure status-consistency sweep (the ARCH-INDEX/STORY-INDEX row's own stale
  PROPOSED-vs-RATIFIED wording swept to match the already-RATIFIED v1.11 body — no new
  ratification event), and v1.13 (this pass's fix) is a wording-narrowing erratum, not a
  ratification event either. The STORY-INDEX narrative's version attribution is imprecise
  (cosmetic index-narrative accuracy, state-manager's own domain — corrected this burst; see
  Commit body §3).

## Novelty Assessment

Novelty MEDIUM. This is the third instance in the S-21.11 v2 cascade of a corrected predicate
narrowing at one authoritative layer (story body, F-001/pass-1) failing to propagate to a sibling
BC-body citation site (BC-1.03.017, this pass) — the same version-cite-propagates/
algorithm-content-does-not defect class first codified for the S-21.07 cascade
(`L-BB-version-cite-propagation-must-include-algorithm-content-not-just-version-numbers`,
D-1006). Confirms the class recurs across independent cascades, not just within one.
