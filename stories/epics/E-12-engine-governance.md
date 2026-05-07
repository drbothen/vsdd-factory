---
document_type: epic
epic_id: "E-12"
version: "1.0"
title: "Engine Governance — Per-Story Adversarial Convergence Discipline"
status: draft
prd_capabilities: [CAP-008, CAP-016, CAP-026]
prd_frs: []
anchor_strategy: greenfield-discipline-gap-codification
priority: P1
target_release: "v1.0-feature-engine-discipline-pass-1"
story_count: 2
subsystems_affected: [SS-04, SS-05]
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 2
traces_to: .factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md
depends_on: ["E-7"]
last_amended: "2026-05-06 (v1.0 — initial authoring for cycle v1.0-feature-engine-discipline-pass-1)"
inputs:
  - .factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/stories/epics/E-7-process-codification.md
input-hash: "TBD"
---
<!-- [process-gap] Frontmatter fields anchor_strategy, depends_on extend the canonical epic-template baseline (same as E-9 v1.0 / E-11 v1.0). Template update tracked as follow-up. -->

# Epic E-12: Engine Governance — Per-Story Adversarial Convergence Discipline

## Description

Closes the documented-vs-implemented gap on per-story adversarial convergence.
The current pipeline runs a single end-of-cycle adversarial pass; individual stories
ship without enforced convergence at story boundary. ADR-017 defines a three-perimeter
scope contract (in-story body, story boundary, wave close) for the adversary agent and
mandates per-story convergence as a gate condition before PR merge. E-12 codifies that
discipline into two layers: (1) workflow + agent documentation updates that define the
process (Step 4.5 in `per-story-delivery.md`, adversary scope contract, orchestrator
MANDATORY STEPS reconciliation, wave-gate Gate 3 narrowing), and (2) a native WASM
enforcement hook (`validate-per-story-adversary-convergence`) that mechanically blocks
PR merge when the per-story convergence gate has not been satisfied.

This is the spiritual successor to E-7 (Process Codification): E-7 codified
spec-first/TDD-Iron-Law discipline gaps surfaced in the S-6.01 sub-cycle; E-12
codifies the per-story adversarial convergence gap surfaced in the
v1.0-feature-engine-discipline-pass-1 F2 spec evolution. Pattern is identical —
process gap identified, documented in ADR, codified into enforcement hook.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-008 | Gate tool calls with pre-execution behavioral checks (PreToolUse hooks) | P0 |
| CAP-016 | Self-referential process codification via adversarial review | P1 |
| CAP-026 | Enforce per-story adversarial convergence as a pipeline gate | P1 |

## Capability Anchor Justification

**Primary anchor:** CAP-008 ("Gate tool calls with pre-execution behavioral checks
(PreToolUse hooks)") per `domain-spec/capabilities.md` §CAP-008. The
`validate-per-story-adversary-convergence` WASM hook (S-12.02) is a PreToolUse gate
that blocks the `git push` / PR-merge tool call when the per-story convergence
condition is unmet. This is the same CAP-008 surface as the existing protection hooks
in SS-04.

**CAP-016 anchor:** ADR-017's three-perimeter model and Step 4.5 workflow update
(S-12.01) are process codification artifacts — the same capability class as E-7's
lessons-codification work.

**SS-05 (Pipeline Orchestration) anchor:** S-12.01 touches `per-story-delivery.md`,
`adversary.md` scope contract, and orchestrator AGENT.md MANDATORY STEPS — all
SS-05 artifacts (`plugins/vsdd-factory/agents/`, `plugins/vsdd-factory/workflows/phases/`).

**SS-04 (Plugin Ecosystem) anchor:** S-12.02 delivers a new native WASM crate at
`crates/hook-plugins/validate-per-story-adversary-convergence/` — an SS-04 artifact.

## Subsystem Anchors

- **SS-05 (Pipeline Orchestration):** owns `plugins/vsdd-factory/workflows/phases/per-story-delivery.md`,
  `plugins/vsdd-factory/agents/adversary.md`, `plugins/vsdd-factory/agents/orchestrator/AGENT.md`,
  `plugins/vsdd-factory/workflows/wave-gate.md`. S-12.01 modifies these files to
  insert Step 4.5, add the three-perimeter scope contract to adversary.md, reconcile
  MANDATORY STEPS, and narrow Gate 3 language.
- **SS-04 (Plugin Ecosystem):** owns `crates/hook-plugins/`. S-12.02 adds the
  `validate-per-story-adversary-convergence` native WASM crate, registers it in
  `plugins/vsdd-factory/hooks-registry.toml`, and adds bats tests + cargo unit tests.

## Stories Planned

| Story | Description | Size | Subsystem | Depends On |
|-------|-------------|------|-----------|------------|
| S-12.01 | Workflow + agent doc updates (per-story-delivery.md Step 4.5, adversary.md scope contract, wave-gate Gate 3 narrowing, orchestrator MANDATORY STEPS reconciliation) | M | SS-05 | — |
| S-12.02 | `validate-per-story-adversary-convergence` WASM hook (Rust crate, hooks-registry.toml registration, bats tests + cargo tests) | M | SS-04 | S-12.01 |

## Dependency Topology (Intra-epic)

```
S-12.01 (Workflow + agent docs) ──→ S-12.02 (WASM hook implementation)
```

S-12.01 must ship first because the WASM hook (S-12.02) enforces the contract
defined by the Step 4.5 workflow update. Implementing the hook before the workflow
is defined would produce a gate with no documented recovery path.

## Anchored BCs

| BC ID | Description | Subsystem | Priority |
|-------|-------------|-----------|----------|
| BC-5.39.001 | Per-story adversary step 4.5 is present and sequenced correctly in per-story-delivery.md | SS-05 | P0 |
| BC-5.39.002 | Adversary.md scope contract covers all three perimeters (in-story body, story boundary, wave close) | SS-05 | P0 |
| BC-4.10.001 | validate-per-story-adversary-convergence WASM hook blocks PR merge when convergence gate unsatisfied | SS-04 | P0 |
| BC-4.10.002 | validate-per-story-adversary-convergence degrades gracefully when convergence state file absent | SS-04 | P1 |

## Anchored ADRs

| ADR ID | Title |
|--------|-------|
| ADR-017 | Per-story adversary three-perimeter model + phasing |

## Anchored VPs

| VP ID | Type | Description |
|-------|------|-------------|
| VP-071 | kani | Convergence block invariant: hook always returns deny when convergence flag absent |

## Relationship to E-7

E-7 (Process Codification, status: open) codified five S-6.01 sub-cycle lessons into
agent prompts, lint hooks, and rule files. E-12 is NOT a reopening of E-7 — E-7 is
converged and its stories are complete. E-12 is the next process codification wave,
targeting a different gap (per-story adversarial convergence) surfaced after E-7 closed.
The pattern is identical; the discipline gap is distinct.

## Open Questions

| OQ ID | Scope | Description |
|-------|-------|-------------|
| OQ-E12-01 | SS-04 | Convergence state file format: TOML vs JSON vs plain flag file. Decision needed before S-12.02 implementation. |
| OQ-E12-02 | SS-05 | Gate 3 narrowing scope: does narrowing apply to wave-gate only or also to per-story-delivery phased gate? |

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-06 | Initial authoring for cycle v1.0-feature-engine-discipline-pass-1 as F3 prerequisite. Two stories: S-12.01 (workflow + agent docs, SS-05) + S-12.02 (WASM hook, SS-04). Anchored BCs: BC-5.39.001/002 (SS-05), BC-4.10.001/002 (SS-04). ADR: ADR-017. VP: VP-071. |
