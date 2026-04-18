---
document_type: cycle-manifest
cycle_id: vX.Y.Z-[type]-[name]
cycle_type: greenfield | feature | bugfix | deprecation | refactor
version: vX.Y.Z
status: in-progress | complete
started: YYYY-MM-DDTHH:MM:SS
completed: YYYY-MM-DDTHH:MM:SS
producer: orchestrator
---

# Cycle Manifest: vX.Y.Z ([Cycle Type])

## Delivered

| Metric | Value |
|--------|-------|
| Stories delivered | STORY-NNN through STORY-NNN |
| BCs created | N new, N modified, N deprecated |
| VPs created | N new, N verified, N withdrawn |
| Holdout scenarios | N new, N retired |
| Total cost | $NNN.NN |
| Adversarial passes | N |
| Final holdout satisfaction | 0.NN |
| Release version | vX.Y.Z |

## Spec Changes

| Artifact | Change | Before | After |
|----------|--------|--------|-------|
| prd.md | [description of change] | [previous state] | [new state] |
| architecture/ | [description of change] | [previous state] | [new state] |

## Living Spec Snapshot

Captured at: git tag vX.Y.Z on factory-artifacts branch
Retrieve: git show vX.Y.Z:specs/prd.md

## Deprecations (if any)

| Artifact | Deprecated By | Replacement | Sunset Date |
|----------|--------------|-------------|-------------|
| [artifact ID] | this cycle | [replacement ID] | vX.Y.Z |

## Tech Debt Created

| ID | Description | Priority | Source |
|----|-------------|----------|--------|
| TD-NNN | [description] | P0/P1/P2 | [source] |

## Governance Policies Adopted

| Policy | Adopted In | Incident Reference | Generalization |
|--------|-----------|-------------------|----------------|
| [policy_flag_name] | Burst/Pass N | [finding IDs] | [one-line generalization] |

## Notes

[Any cycle-specific observations, lessons learned, or follow-up items.]
