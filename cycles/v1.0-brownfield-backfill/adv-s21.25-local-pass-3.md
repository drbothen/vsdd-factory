---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-2.md
  - .factory/stories/S-21.25-fuel-headroom-warn-event.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.019.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md
  - .factory/specs/verification-properties/VP-079.md
input-hash: "635b491"
traces_to: S-21.25-fuel-headroom-warn-event.md
pass: 3
cascade: S-21.25-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.25-local-pass-2.md
---

# Adversarial Review — S-21.25 (LOCAL pre-TDD cascade, pass 3) — NOT-CLEAN

Artifacts reviewed: story `S-21.25-fuel-headroom-warn-event.md` v1.2 (input-hash `4af3ec2`);
`BC-1.03.019.md` v1.2; `BC-3.08.001.md` v1.25; `VP-079.md` v1.20. Rubric: full
`.factory/policies.yaml` (POLICY 1-22). This is pass 3 of the S-21.25 LOCAL pre-TDD cascade,
reviewing the D-1060 pass-2 remediation (AC-005 self-match/RED-GREEN-inversion relocation +
`emit_plugin_fuel_headroom_warning` rename) applied against pass 2's findings.

## Verdict: NOT-CLEAN

2 MEDIUM findings. LOCAL streak 0/3 (a NOT-CLEAN pass resets the streak per BC-5.39.001; pass 2's
fixes do not themselves count as a clean pass).

## Part A — Fix Verification (pass 2 findings)

**F-S2125-P2-001** (AC-005 self-match / RED-GREEN inversion): **VERIFIED FIXED, no recurrence**.
The AC-005 regression guard remains relocated to the separate integration-test file
`crates/factory-dispatcher/tests/fuel_headroom_single_emit_site.rs`, which `include_str!`s
`invoke.rs` from outside it and constructs its needle via `concat!("// SINGLE", "-EMIT-SITE")` so
the marker substring never appears written out in the scanning file's own source. Re-derivation of
the RED/GREEN arithmetic (count 0 at RED, count 1 at GREEN) confirms the structural self-match
class of defect genuinely cannot recur under this design — this is not a re-inspection of
unchanged text, it is an independent re-derivation against the current v1.2 Task 7/9/11 wording,
and it holds.

**F-S2125-P2-002** (`emit_fuel_headroom_warning` → `emit_plugin_fuel_headroom_warning` sibling
rename): **VERIFIED FIXED**. The rename is swept consistently across Architecture Mapping, Purity
Classification, Tasks 5/6/7/10, Architecture Compliance Rules, File Structure Requirements, and
AC-009 (body + both test bullets). BC-1.03.019 v1.1→v1.2 re-anchor confirmed consistent across
frontmatter, body BC table, and live narrative cites.

Both prior structural-fix sets — the pure-helper extraction + effectful-shell split (D-1059,
pass 1) and the AC-005 relocation + emitter rename (D-1060, pass 2) — CONFIRMED HELD at this pass.
This pass's findings are new, surfaced by cross-checking the v1.2 bundle's Task 7/11 test-count
narrative against the File Structure Requirements table, and by cross-checking this story's own
VP status note against VP-079's independently-evolving SITE_7 note — not a re-opening of any prior
finding.

## Part B — New Findings

### MEDIUM

#### F-S2125-P3-001: Task 7/11 test-distribution narrative silently dropped 3 `emit_event.rs` tests
- **Severity:** MEDIUM
- **Category:** internal contradiction / narrative-accuracy
- **Location:** S-21.25 v1.2, Task 7 (RED Gate authoring instruction) and Task 11 (GREEN
  verification instruction)
- **Description:** Task 7 and Task 11 both stated "18 test functions total — 17 live in
  `invoke.rs`'s own `#[cfg(test)] mod tests`, and the 18th (AC-005) in the separate
  `tests/fuel_headroom_single_emit_site.rs` file." But the File Structure Requirements table
  (unchanged since v1.2) places the AC-006×1 + AC-008×2 field-shape tests (3 tests) in
  `emit_event.rs`'s own test module, not `invoke.rs` — the "17 + 1" phrasing silently dropped
  those 3 tests from the accounting. `17 + 1 = 18` arithmetically matches the total only by
  coincidence of miscounting two different things as one.
- **Evidence:** File Structure Requirements table lists `emit_event.rs` as carrying the AC-006 and
  AC-008 test functions (mirroring `plugin.timeout`'s existing test suite in that file); grepping
  the actual per-AC test-location assignments across Tasks 7/11 and the File Structure table
  confirms 14 (not 17) tests are `invoke.rs`-resident once AC-006/AC-008's 3 tests are correctly
  attributed to `emit_event.rs`: `14 (invoke.rs) + 3 (emit_event.rs) + 1 (separate file) = 18`.
- **Proposed Fix:** Correct Task 7 and Task 11's distribution narrative to the accurate
  14/3/1 split, explicitly naming which ACs land in which file.
- **Status:** RESOLVED this burst (D-1061) — story-writer corrected Task 7 and Task 11 in
  S-21.25 v1.3 to state the accurate distribution: 14 in `invoke.rs` (AC-001×2, AC-002×1,
  AC-003×1, AC-004×2, AC-005×2-of-3, AC-007×2, AC-009×2, AC-010×2), 3 in `emit_event.rs`
  (AC-006×1, AC-008×2), 1 in the separate `fuel_headroom_single_emit_site.rs` integration file
  (AC-005's marker-scan test) — `14 + 3 + 1 = 18`, now consistent with the File Structure
  Requirements table.

#### F-S2125-P3-002: Story silent on VP-079 v1.20's SITE_7 registration and its own delivery-time obligation
- **Severity:** MEDIUM
- **Category:** cross-document consistency / VP coherence
- **Location:** S-21.25 v1.2 VP status note (`verification_properties: []` justification)
- **Description:** VP-079 v1.20 already catalogs this story's `emit_plugin_fuel_headroom_warning`
  emission by name as SITE_7 (its 7th mutation-counter-proof site) and states "a dedicated fixture
  is required before SITE_7 can be added to the Scenario 6 SITES array + cargo-mutants filter —
  tracked for the test-writer at S-21.25 delivery time." S-21.25's own VP status note justified
  only the BC-1.03.019 VP-TBD semantics-VP deferral (a different, legitimate deferral) and said
  nothing about the VP-079/SITE_7 relationship or the obligation VP-079 v1.20 places on "the
  test-writer at S-21.25 delivery time" — leaving a reader of this story alone unable to tell
  whether SITE_7 is this story's responsibility, forgotten, or intentionally out of scope.
- **Evidence:** VP-079 v1.20 Property 6 SITE_7 scope note text (quoted above); S-21.25 v1.2 VP
  status note text (BC-1.03.019 VP-TBD paragraph only, no SITE_7/VP-079 cross-reference).
- **Proposed Fix:** Either build the SITE_7 fixture in this story, or add an explicit,
  concrete-not-silent acknowledgment of the VP-079/SITE_7 relationship and its deferral target.
  Given this story's `tdd_mode: strict` Task 7/11 RED/GREEN scope is pre-TDD spec/unit-level and
  the SITE_7 fixture is an async-event-schema-conformance mutation-fixture (VP-079's own
  infrastructure, not this story's emission-behavior unit tests), route to orchestrator for
  scope adjudication rather than silently either building or dropping it.
- **Status:** RESOLVED this burst (D-1061), bidirectionally — orchestrator adjudicated the SITE_7
  fixture as a Phase-6 formal-verification concern (not S-21.25's pre-TDD/unit scope, consistent
  with the phase that owns cargo-mutants execution and mutation-coverage gating for this VP
  generally). Story-writer extended S-21.25 v1.3's VP status note with an explicit "VP-079 SITE_7
  acknowledgment" paragraph naming the relationship and the Phase-6 deferral target. Concurrently,
  architect retargeted VP-079's own SITE_7 scope note (v1.20→v1.21) from "tracked for the
  test-writer at S-21.25 delivery time" to Phase-6 formal-verification, so both artifacts now
  agree in both directions.

## Disposition

Both findings routed per the Agent Routing Table: F-S2125-P3-001 is story content (story-writer's
domain — a narrative-accuracy correction, no BC/AC/code-shape change). F-S2125-P3-002 spans two
domains — the VP-079 retarget is architect's domain (VP content), the story's own acknowledgment
paragraph is story-writer's domain — and required an orchestrator scope adjudication first (is the
SITE_7 fixture S-21.25's job or Phase-6's), which is a legitimate orchestrator-only decision, not a
defer-pattern, per the Companion Principle. Story-writer corrected the Task 7/11 distribution and
added the VP-079 acknowledgment in a single S-21.25 v1.2→v1.3 burst; architect retargeted VP-079
v1.20→v1.21 concurrently. Both fixes same-burst (D-1061).

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 0     |
| MEDIUM   | 2     |
| LOW      | 0     |

**Overall Assessment:** block (pre-remediation, v1.2 bundle) — RESOLVED same burst via
story-writer S-21.25 v1.3 + architect VP-079 v1.21 remediation (D-1061).
**Convergence:** findings remain — iterate. LOCAL streak 0/3 (resolving both MEDIUM findings does
not itself advance the streak per BC-5.39.001; pass 4 required against the v1.3 bundle to confirm
CLEAN).
**Readiness:** requires re-review (pass 4, against S-21.25 v1.3 + BC-1.03.019 v1.2 + VP-079 v1.21)
before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 2 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (2/2) |
| **Median severity** | 2.0 (MEDIUM) |
| **Trajectory** | 4 → 3 → 2 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1061); pass 4 required to confirm CLEAN against the remediated bundle. AC-005 recurrence class CONFIRMED GENUINELY FIXED (no recurrence this pass). |
