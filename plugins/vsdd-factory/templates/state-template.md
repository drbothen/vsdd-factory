---
document_type: pipeline-state
level: ops
version: "1.0"
status: draft
producer: "[agent-id]"
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1a
inputs: []
input-hash: "[md5]"
traces_to: ""
---

# Pipeline State: [Product Name]

## Project Metadata

| Field | Value |
|-------|-------|
| **Product** | [product name] |
| **Repository** | [repo URL or path] |
| **Mode** | greenfield / brownfield / feature |
| **Language** | |
| **Target Workspace** | |
| **Started** | YYYY-MM-DDTHH:MM:SS |
| **Last Updated** | YYYY-MM-DDTHH:MM:SS |
| **Current Phase** | [1a/1b/2/3/3.5/4/5/6] |
| **Current Step** | |

## Phase Progress

| Phase | Status | Started | Completed | Gate | Finding Progression |
|-------|--------|---------|-----------|------|---------------------|
| 1: Spec Crystallization | not-started / in-progress / passed / blocked | | | | [22→6→0 converged] |
| 2: Story Decomposition | | | | | |
| 3: Test-First Implementation | | | | | |
| 3.5: Holdout Evaluation | | | | | |
| 4: Adversarial Refinement | | | | | |
| 5: Formal Hardening | | | | | |
| 6: Convergence | | | | | |

## Current Phase Steps

| Step | Agent | Status | Output | File Size |
|------|-------|--------|--------|-----------|
| | | | | |

> File Size column tracks artifact bloat. PRDs over 100KB may need splitting.

## Decisions Log

| ID | Decision | Rationale | Phase | Date | Made By |
|----|----------|-----------|-------|------|---------|
| D-001 | [what was decided] | [why] | [phase] | YYYY-MM-DD | [agent/human] |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes/no | [e.g., "CLI-only product, no UI surfaces"] |

## Blocking Issues

| ID | Issue | Severity | Blocking Phase | Owner | Resolution |
|----|-------|----------|---------------|-------|------------|
| BLK-001 | [issue description] | [critical/high/medium] | [phase] | [owner] | [status/plan] |
