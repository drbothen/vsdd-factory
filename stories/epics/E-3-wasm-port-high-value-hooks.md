---
document_type: epic
epic_id: "E-3"
version: "1.0"
prd_capabilities: []
status: draft
story_count: 4
---

# Epic E-3: WASM Port — High-Value Hooks

## Description

Four stories that port the three highest-value bash hooks to native WASM and
complete the `emit_event` host function pipeline. All four are parallel (Tier E),
gated on beta.1 stability. S-3.04 (`emit_event` refactor) blocks the three port
stories. Milestone: `1.0.0-rc.1`. Subsystems: SS-01, SS-03, SS-04.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| (pre-CAP) | capture-commit-activity as native WASM | P1 |
| (pre-CAP) | capture-pr-activity as native WASM | P1 |
| (pre-CAP) | block-ai-attribution as native WASM PreToolUse gate | P1 |
| (pre-CAP) | emit_event host fn routing through sink pipeline | P0 |

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|---------------|
| EAC-001 | capture-commit-activity.wasm emits commit.made with sha/branch/message | Integration test | git commit → event in file sink |
| EAC-002 | capture-pr-activity.wasm emits pr.created/merged/closed per subcommand | Integration test | gh pr create/merge/close → typed events |
| EAC-003 | block-ai-attribution.wasm blocks PreToolUse on AI attribution patterns | Unit test | commit with Co-Authored-By → Block result |
| EAC-004 | emit_event host fn routes to configured sinks; events enriched with trace_id | Integration test | Plugin emits event → verify in file sink |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-3.04 | emit_event as host function refactor | 3 | S-1.04 | partial |
| S-3.01 | Port capture-commit-activity to WASM | 5 | S-2.08, S-3.04 | draft |
| S-3.02 | Port capture-pr-activity to WASM | 5 | S-2.08, S-3.04 | draft |
| S-3.03 | Port block-ai-attribution to WASM | 3 | S-2.08, S-3.04 | draft |

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| beta.1 release | Stable dispatcher + SDK | Shipped (v1.0.0-beta.4) |
| emit_event host fn | Sink pipeline integration | Partial (host fn impl exists; bash tool not retired) |
