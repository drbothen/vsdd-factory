---
document_type: epic
epic_id: "E-2"
version: "1.0"
prd_capabilities: []
status: closed
story_count: 8
---

# Epic E-2: Legacy Adapter and Beta Release

## Description

Eight stories (Tier C parallel + Tier D gate) that wire the legacy bash adapter,
generate the registry from hooks.json, establish the cross-platform CI matrix,
automate binary commits, integrate the activation skill, validate regression tests,
and cut the `1.0.0-beta.1` release. S-2.08 is the gate story. Milestone:
`1.0.0-beta.1`. Subsystems: SS-01, SS-02, SS-04, SS-07, SS-09, SS-10.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| (pre-CAP) | All 45 legacy bash hooks running through WASM adapter | P0 |
| (pre-CAP) | hooks-registry.toml auto-generated from hooks.json | P0 |
| (pre-CAP) | 5-platform CI build matrix | P0 |
| (pre-CAP) | Release workflow commits platform binaries to repo | P0 |
| (pre-CAP) | hook-sdk crates.io publish (dry-run) | P1 |
| (pre-CAP) | Activation skill wires correct hooks.json variant per platform | P0 |
| (pre-CAP) | Full bats regression suite green under new dispatcher | P0 |

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|---------------|
| EAC-001 | All 45 legacy bash hooks fire via legacy-bash-adapter without regression | Full bats suite (1177+ tests) | regression-v1.0.bats + full suite |
| EAC-002 | `hooks-registry.toml` generated from `hooks.json`; idempotent | 6 bats tests | normal gen, re-run, missing input |
| EAC-003 | `factory-dispatcher` builds on all 5 platforms in CI | CI matrix run | darwin-arm64/x64, linux-x64/arm64, windows-x64 |
| EAC-004 | Release workflow commits platform binaries on tag push | Manual release run | Per-platform binary at expected path |
| EAC-005 | `/vsdd-factory:activate` detects platform and copies correct hooks.json | End-to-end test | darwin-arm64 activation |
| EAC-006 | 1.0.0-beta.1 tag produces GH pre-release with binaries attached | Manual verification | GH Release marked prerelease:true |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-2.01 | legacy-bash-adapter WASM plugin | 5 | S-1.03, S-1.04, S-1.05, S-1.06 | merged |
| S-2.02 | hooks-registry.toml auto-generation | 2 | S-2.01 | merged |
| S-2.03 | Cross-platform CI matrix build targets | 5 | S-1.01, S-1.02 | merged |
| S-2.04 | Binary commit automation in Release workflow | 5 | S-2.03 | merged |
| S-2.05 | hook-sdk publish to crates.io (0.1.0) | 2 | S-1.03 | partial |
| S-2.06 | Activation skill integrates with real hooks.json variants | 3 | S-0.03, S-0.04, S-2.04 | merged |
| S-2.07 | Regression test suite validation | 5 | S-1.02, S-1.04, S-1.05, S-1.06, S-1.07, S-1.08, S-2.01, S-2.02 | merged |
| S-2.08 | 1.0.0-beta.1 release gate | 3 | all prior | merged |

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| crates.io | SDK publish token | Available (CI secret) |
| GitHub Actions | 5-platform matrix build | Available |
| GitHub Releases | prerelease flag | Available |
