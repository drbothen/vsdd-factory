---
document_type: epic
epic_id: "E-0"
version: "1.0"
prd_capabilities: []
status: closed
story_count: 5
---

# Epic E-0: Infrastructure Prep

## Description

Five parallel foundation stories that establish the tooling, CI, docs skeleton,
and platform-detection logic required before any dispatcher code can be released.
All five run in parallel (Tier A) with no inter-dependencies. Milestone:
`1.0.0-beta.1`. Subsystems: SS-06 (activate skill), SS-07 (hooks template),
SS-08 (docs), SS-09 (config/activation), SS-10 (CLI tools).

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| (pre-CAP) | Prerelease semver support in bump-version.sh | P0 |
| (pre-CAP) | Platform-aware activation for 5 target platforms | P0 |
| (pre-CAP) | CI-generated hooks.json platform variants | P0 |
| (pre-CAP) | Documentation skeleton for v1.0 guides | P1 |

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|---------------|
| EAC-001 | `bump-version.sh` accepts prerelease tags (1.0.0-beta.1) | bats tests | stable + prerelease + malformed inputs |
| EAC-002 | Release workflow marks GH Release as prerelease for tags with `-` | Manual CI run | beta tag → prerelease:true; stable tag → prerelease:false |
| EAC-003 | Activation skill detects OS+arch and writes platform to settings.local.json | bats tests | 5 platforms + 2 unsupported |
| EAC-004 | 5 hooks.json.<platform> variants generated from single template | bats tests | drift check; each platform has correct binary path |
| EAC-005 | 4 documentation skeletons exist with section stubs and TODO markers | Manual check | grep docs/guide/ for skeletons |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-0.01 | bump-version.sh prerelease support | 2 | — | merged |
| S-0.02 | Release workflow prerelease handling | 2 | S-0.01 | merged |
| S-0.03 | Activation skill platform detection | 3 | — | merged |
| S-0.04 | hooks.json.template + CI generation | 3 | — | merged |
| S-0.05 | Documentation scaffolding | 2 | — | merged |

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| GitHub Actions | Workflow prerelease flag support | Available (softprops/action-gh-release@v2) |
| CI runners | All 5 platform build targets | Available via cross-compilation |
