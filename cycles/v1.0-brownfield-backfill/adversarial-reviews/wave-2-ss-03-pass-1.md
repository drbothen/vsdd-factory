---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T22:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-1.08-sink-file-driver.md
  - .factory/stories/S-1.09-sink-otel-grpc-driver.md
  - .factory/stories/S-4.01-sink-http-driver.md
  - .factory/stories/S-4.02-sink-datadog-driver.md
  - .factory/stories/S-4.03-sink-honeycomb-driver.md
  - .factory/stories/S-4.04-retry-circuit-breaker.md
  - .factory/stories/S-4.05-dead-letter-queue.md
  - .factory/stories/S-4.06-routing-tag-enrichment.md
  - .factory/stories/S-4.07-observability-integration-tests.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/behavioral-contracts/ss-03/
input-hash: "9e36a73"
traces_to: ""
cycle: v1.0-brownfield-backfill
sub_cycle: wave-2-ss-03-re-anchor
pass: 1
verdict: FINDINGS_REMAIN
finding_count: 11
convergence_step: 0_of_3
po_commit_reviewed: 73bbf7d
previous_review: null
---

# Adversarial Review — Wave 2 SS-03 Re-anchor — Pass 1

## Finding ID Convention

Pass-1 findings use `F-0NN` numbering.

## Part B — New Findings

### CRITICAL (2)

#### F-001 [CRIT] — All 7 stories trace to FR-003 which is SS-01 plugin lifecycle, NOT SS-03 sinks

**Affected:** S-1.08, S-1.09, S-4.01, S-4.02, S-4.03, S-4.06, S-4.07

PRD §FR-003 (line 164): "Plugin execution lifecycle (epoch, fuel, crash, block-intent)" — governs SS-01.

Correct SS-03 FRs (PRD line 291+):
- FR-010 — Sink registry, routing filter, config loading (BC-3.01.001-005, BC-3.06.001-006)
- FR-011 — File sink (BC-3.01.006-008, BC-3.02.001-016)
- FR-012 — OTel gRPC sink (BC-3.01.009, BC-3.03.001-013, BC-3.04.001-002, BC-3.05.001-003)

PO inferred FR-003 from CAP-003 numbering — incorrect because FR-NNN and CAP-NNN are independent ID spaces.

**Remediation per story:**
- S-1.08 (file sink) → FR-011
- S-1.09 (otel-grpc sink) → FR-012
- S-4.01/S-4.02/S-4.03 (HTTP/Datadog/Honeycomb new drivers) → no existing FR; propose FR-044-advanced-sink-drivers OR extend FR-010/012
- S-4.06 (routing/tag enrichment) → FR-010
- S-4.07 (integration tests) → FR-010 (or composite)

#### F-002 [CRIT] — S-4.04, S-4.05 trace to FR-024 which is SS-06 spec crystallization, NOT SS-03 resilience

**Affected:** S-4.04, S-4.05

PRD §FR-024 (line 530): "Spec crystallization skills (create-brief, create-domain-spec, create-prd, create-architecture)" — governs SS-06.

NO FR exists for SS-03 retry/CB/DLQ. PRD §FR-012 line 327 mentions "Retry/circuit-breaker: pending (S-4.04)" but never broke out as independent FR.

**Remediation:** Add FR-044 — Per-sink resilience (retry, circuit breaker, DLQ) to PRD via PO edit. Re-anchor S-4.04/S-4.05 to FR-044.

### HIGH (4)

#### F-003 [HIGH] — BC-3.01.007/008 are FILE-sink-specific BCs stretch-anchored to non-file sinks

**Affected:** S-4.01, S-4.02, S-4.03, S-4.04, S-4.05

BC-3.01.007 H1: "**file sink** mpsc bounded at default 1000". BC-3.01.008 H1: "**file sink** failures recorded into Mutex<Vec<SinkFailure>>".

Stories anchor with rationale "pattern applies to all sinks" — same stretch-anchor pattern Wave 1 F-001 rejected.

**Remediation:** Remove BC-3.01.007/008 from S-4.01/02/03/04/05. Either generalize the BCs (rename "file sink" → "any sink") OR add new cross-sink BCs OR rely on v1.1 vendor-specific candidates.

#### F-004 [HIGH] — S-4.06 AC#1 cites BC-3.04.001 with semantically inverted postcondition

**Affected:** S-4.06 line 80-81

BC-3.04.001 postcondition 1 contracts the CURRENT pre-wired state ("Router has no extra logic today; no call sites exist"). The AC describes the FUTURE wired state. Semantic inversion.

**Remediation:** Re-trace AC#1 to BC-3.01.004 (RoutingFilter semantics) and the v1.1 candidate `BC-3.NN.NNN-router-filter-wired-in-dispatch`.

#### F-005 [HIGH] — S-4.07 AC#5 mis-cites BC-3.01.002 with synthesized postcondition text

**Affected:** S-4.07 line 103-104

BC-3.01.002 actual postcondition: "Unknown sink type warns to stderr but does not fail config load." AC trace synthesizes "failing sink type does not prevent other sinks from working" — not in BC.

**Remediation:** Re-trace AC#5 to VP-012 + v1.1 candidate `BC-3.NN.NNN-circuit-breaker-integration`.

#### F-006 [HIGH] — S-4.06 anchors BC-3.02.010/011 (file-sink tag enrichment) to Router-layer wiring story

**Affected:** S-4.06 (lines 24-26 frontmatter; lines 100-102 body)

BC-3.02.010 H1: "**sink-file**::tag_enrichment_writes_tags_onto_every_event" — file-sink-scoped. S-4.06's scope is Router-layer (sink-agnostic).

**Remediation:** Demote BC-3.02.010/011 to body "related contracts" only, NOT frontmatter array. Rely on v1.1 candidate `BC-3.NN.NNN-tag-enrichment-wired-in-dispatch`.

### MEDIUM (4)

#### F-007 [MED] — S-4.04 has 2 BCs neither describing new RetryPolicy/CircuitBreaker behavior

**Affected:** S-4.04

Both anchored BCs describe pre-existing patterns. New behavior (RetryPolicy + CB state machine) entirely uncontracted. 4 v1.1 candidates correctly capture gap.

**Adjudication:** Adversary recommends defer all 4 v1.1 candidates to backlog (Wave 2 doesn't block on new BC creation).

**Remediation:** Acceptable as-is with explicit note that Wave 2 closeout deliberately leaves S-4.04 thin pending v1.1 BC creation.

#### F-008 [MED] — CAP-023, CAP-024 Subsystems field omits SS-01 [process-gap recurrence]

**Affected:** capabilities.md lines 150 (CAP-023), 156 (CAP-024)

Same drift pattern Wave 1 F-007 fixed for CAP-010 + Wave 2 found extending to CAP-003. Now CAP-023, CAP-024 also missing SS-01 despite stories anchoring SS-01 base machinery (BC-3.01.001 SinkRegistry + BC-3.06.005 SinkConfigCommon both in `crates/factory-dispatcher/src/sinks.rs`).

**Remediation:** Update CAP-023 Subsystems → ["SS-01", "SS-03"]; CAP-024 → ["SS-01", "SS-03"] (and possibly SS-10 for DLQ events).

#### F-009 [MED] — PO summary "12 unanchored BCs" should be 4

**Affected:** PO commit message arithmetic

Actual unanchored: BC-3.01.005, BC-3.06.002, BC-3.06.003, BC-3.06.004 = 4 (not 12).

**Remediation:** PO recount in pass-2 burst.

#### F-010 [MED] — BC-3.01.005 (SinkEvent flat serialization) should be anchored to S-1.08

**Affected:** BC-3.01.005, S-1.08

PO flagged as concern but didn't anchor. BC-3.01.005 governs SinkEvent serde representation; S-1.08's core AC ("Events appended as newline-delimited JSON") cannot hold without flat-serialization invariant.

**Remediation:** Add BC-3.01.005 to S-1.08 frontmatter + body table.

### LOW (1)

#### F-011 [LOW] — S-4.06 partial status note in prose form, not Partial Status table

**Affected:** S-4.06 lines 56-58

Status note exists but as prose. Wave 1 calibration prefers structured Partial Status table.

**Remediation:** Convert prose note to Partial Status table format.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 2 |
| HIGH | 4 |
| MEDIUM | 4 |
| LOW | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 1 |
| **New findings count** | 11 |
| **Duplicate count** | 0 (first pass) |
| **Novelty score** | 1.0 |
| **Median severity** | MEDIUM |
| **Severity distribution** | 2 CRIT, 4 HIGH, 4 MED, 1 LOW |
| **Trajectory** | starting baseline |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3** consecutive NITPICK-only passes per ADR-013.

2 CRITICAL findings block immediately. 4 HIGH semantic anchor mis-attributions block per BC-5.04.003.

## Trajectory Comparison vs Wave 1

| Pass | Wave 1 | Wave 2 |
|---|---|---|
| Pass-1 baseline | 10 findings (3H/4M/3L) | 11 findings (2C/4H/4M/1L) |
| Severity ceiling | HIGH | CRITICAL |

Wave 2 starts heavier due to PO's CAP→FR inference antipattern surfaced. Expected trajectory: 11 → 6 → 3 → 1 → 0 → 0.

## Findings by Axis

| Axis | Findings |
|---|---|
| A — BC Existence | (none) ✓ |
| B — Semantic Anchoring | F-003, F-004, F-005, F-006 |
| C — Coverage Completeness | F-009, F-010 |
| D — AC↔BC Bidirectional | F-004, F-005 |
| E — Capability Justification | (none) ✓ |
| F — Subsystem/FR Hygiene | F-001, F-002 (CRIT FR drift) |
| G — VP Soundness | (none) ✓ |
| H — CAP Choice | F-008 (CAP subsystem drift) |
| I — Spec-First Gate | F-011 |
| J — POLICY 1 Reuse | (none) ✓ |
| K — Edge Cases | (none) ✓ |
| L — Bookkeeping | F-009 |
| M — Vendor Specificity | (defer to v1.1 — adjudicated clean) |

## Observations

- **[process-gap]** PO's pattern of inferring FR-NNN from CAP-NNN numbering (F-001, F-002) is a recurring failure mode. Add FR-anchor gate to product-owner agent prompt parallel to BC-5.36.001-002 spec-first gate.
- **[process-gap]** CAP `Subsystems:` drift now confirmed across CAP-003 (Wave 1), CAP-010 (Wave 1), CAP-023 (Wave 2), CAP-024 (Wave 2). Systemic — capabilities.md author/business-analyst should run CAP-wide audit before Wave 3.
