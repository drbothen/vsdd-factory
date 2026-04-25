---
document_type: epic
epic_id: "E-5"
version: "1.0"
prd_capabilities: []
status: draft
story_count: 7
---

# Epic E-5: New Hook Events and 1.0.0 Release

## Description

Seven stories (Tier G parallel + Tier H gate) that wire four new Claude Code hook
event types (SessionStart, SessionEnd, WorktreeCreate/Remove, PostToolUseFailure),
write the migration guide and semver commitment documentation, and cut the final
`1.0.0` release tag. S-5.07 is the gate story. Milestone: `1.0.0`. Subsystems:
SS-01, SS-04, SS-08, SS-10.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| (pre-CAP) | SessionStart: session.started event + factory-health brief | P1 |
| (pre-CAP) | SessionEnd: session.ended with duration + tool-call count | P1 |
| (pre-CAP) | WorktreeCreate/Remove: auto-register worktrees with observability | P1 |
| (pre-CAP) | PostToolUseFailure: tool.error events for ROI error-rate tracking | P1 |
| (pre-CAP) | Complete migration guide (0.79.x → 1.0) | P0 |
| (pre-CAP) | Semver stability commitment documentation | P0 |

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|---------------|
| EAC-001 | session.started emitted on SessionStart; session.ended emitted on SessionEnd | Integration test | session lifecycle events in file sink |
| EAC-002 | worktree.created/removed events emitted; sink config auto-generated | Integration test | WorktreeCreate/Remove payloads |
| EAC-003 | tool.error event emitted on PostToolUseFailure | Integration test | Failure payload → event in sink |
| EAC-004 | Migration guide has no TODO(S-5.05) markers; all sections complete | grep check | grep -r "TODO(S-5.05)" docs/guide/ returns empty |
| EAC-005 | Semver commitment doc published and cross-linked from index | Manual check | docs/guide/semver-commitment.md exists |
| EAC-006 | 1.0.0 tag produces stable GH Release (prerelease:false) with SDK published | Manual verification | cargo publish + GH Release |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-5.01 | SessionStart hook wiring | 3 | S-4.08 | draft |
| S-5.02 | SessionEnd hook wiring | 3 | S-4.08 | draft |
| S-5.03 | WorktreeCreate / WorktreeRemove hook wiring | 5 | S-4.08 | draft |
| S-5.04 | PostToolUseFailure hook wiring | 3 | S-4.08 | draft |
| S-5.05 | Migration guide (0.79.x → 1.0) | 5 | S-4.08 | partial |
| S-5.06 | Semver commitment documentation | 2 | S-4.08 | draft |
| S-5.07 | 1.0.0 release gate | 3 | S-5.01..S-5.06 + 1-week shakedown | draft |

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| Claude Code | SessionStart, SessionEnd, WorktreeCreate/Remove, PostToolUseFailure event types | Available in CC |
| rc.1 shakedown | 1+ week of rc.1 stability | Pending rc.1 |
| crates.io | vsdd-hook-sdk 0.1.0 publish | Dry-run clean; real publish pending |
