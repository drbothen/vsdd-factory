---
document_type: behavioral-contract
level: L3
version: "1.3"
last_amended: 2026-05-08
status: draft
producer: phase-1-4b-agent-5
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-workflows.md]
input-hash: "99bbe9c"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/discovery.lobster"
subsystem: "SS-05"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-5.32.016: discovery:feature-scoring-value

## Description

Step `feature-scoring-value` (line 167; lobster carve-out: stable anchor is step name `feature-scoring-value`, not line number). Type: agent. Agent: product-owner. Depends: `[intelligence-synthesis]`. Condition: `config.feature_discovery.enabled == true`. Source 167-178. Delphi Step 1 (independent scoring, no other scores visible). Scores Value, Alignment, Time Criticality, evidence_strength. <!-- F-P22-001: lobster-line-cite deferred per pass-21/22 carve-out; line range preserved as source evidence; lobster files have stable section structure (step `feature-scoring-value` is the stable anchor); migrate to step-name anchor in future sweep -->

## Preconditions

1. intelligence-synthesis completed.
2. Feature discovery is enabled.
3. No other Delphi scorer's outputs are visible to this scorer.

## Postconditions

1. Value, Alignment, Time Criticality, and evidence_strength scores produced for each feature opportunity.

## Invariants

1. Scorer cannot read feasibility or novelty scores at this stage.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Wall breach attempt | Denied |
| EC-002 | No opportunities to score | No-op output |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Synthesis with opportunities | Scores produced | happy-path |
| Empty synthesis | Empty scoring | edge-case |
| Wall breach | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Other Delphi scorers' outputs not in scorer context | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.017 — feature-scoring-feasibility
- BC-5.32.018 — feature-scoring-novelty
- BC-5.32.019 — feature-debate

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#delphi-scoring`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 167-178; lobster path carve-out: line range is unstable as lobster files evolve) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: Delphi step semantics

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (synthesis) |
| **Global state access** | reads filtered context |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P22-001: §Description lobster line-cite annotated with carve-out deferral)

**Driver:** F-P22-001 pass-22 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 semantic-pattern-class discipline) — §Description cited lobster step by line number (`(line 167)`) and source range (`Source 167-178.`). These are lobster-file references and fall under the lobster-line-cite carve-out exception; the stable anchor is the step name `feature-scoring-value`, not the line number.

**Change made:**
- §Description: inline annotation added noting lobster carve-out (stable anchor = step name `feature-scoring-value`); HTML carve-out comment added citing F-P22-001 deferral.
- Frontmatter `version:` bumped `"1.1"` → `"1.2"`.
- Changelog entry added: F-P22-001 corpus-wide sweep, L-P19-001 + L-P20-001 applied with FULL semantic scope.


---

## Amendment 2026-05-08 (v→ F-P23-001: lobster-line-cite annotated with carve-out)

**Driver:** F-P23-001 pass-23 retroactive corpus-wide sweep (per L-P19-001 / L-P20-001 / L-P22-001) — lobster path lines 167-178

**Changes made:**
- Inline lobster carve-out annotation added to all active-body line cites.
- Frontmatter `version:` incremented. Changelog entry added.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-05-08 | state-manager | F-P23-001 corpus-wide sweep: lobster-line-cite annotated with carve-out per L-P19-001 + L-P20-001 + L-P22-001. |
