---
document_type: epic
epic_id: "E-13"
version: "1.0"
title: "Artifact Integrity — Path Discipline Single Source of Truth"
status: draft
prd_capabilities: [CAP-008, CAP-016, CAP-027]
prd_frs: []
anchor_strategy: greenfield-discipline-gap-codification
priority: P1
target_release: "v1.0-feature-engine-discipline-pass-1"
story_count: 1
subsystems_affected: [SS-04, SS-06]
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 2
traces_to: .factory/specs/architecture/decisions/ADR-016-artifact-path-registry-single-source-of-truth.md
depends_on: []
last_amended: "2026-05-06 (v1.0 — initial authoring for cycle v1.0-feature-engine-discipline-pass-1)"
inputs:
  - .factory/specs/architecture/decisions/ADR-016-artifact-path-registry-single-source-of-truth.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
input-hash: "TBD"
---
<!-- [process-gap] Frontmatter fields anchor_strategy, depends_on extend the canonical epic-template baseline (same as E-9 v1.0 / E-11 v1.0). Template update tracked as follow-up. -->

# Epic E-13: Artifact Integrity — Path Discipline Single Source of Truth

## Description

Establishes a single-source-of-truth registry (`plugins/vsdd-factory/config/artifact-path-registry.yaml`)
that governs canonical `.factory/` artifact locations and enforces them across three
layers: a WASM hook (mechanical block), 9 creation skills (structured resolution via
registry lookup), and writing-agent prompt preambles (cultural reinforcement). A
`relocate-artifact` skill provides corrective scanning and `git mv` repair for
artifacts already written to wrong paths.

The core problem: without a registry, different agents write artifacts to different
paths based on their individual context (e.g., `specs/` vs `.factory/specs/`). The
registry is the single authority; the WASM hook enforces it at write time; the skills
query it at creation time; the prompt preambles prevent well-meaning but incorrect path
assumptions from taking root. ADR-016 defines this three-layer model as the canonical
artifact path discipline for vsdd-factory.

> **Delivery sequencing note (hard constraint):** S-13.01 ships FIRST in the
> v1.0-feature-engine-discipline-pass-1 cycle's delivery order (C → A → B, where
> C = E-13, A = E-12, B = remaining cycle work). Reasoning: the `validate-artifact-path`
> WASM hook activates immediately in `block` mode per OQ5 resolution; registering it
> BEFORE `.factory/` is clean would brick subsequent agents. S-13.01's `relocate-artifact`
> skill runs detect-then-apply preflight as a hard gate before WASM hook registration.
> No other story in this cycle may be dispatched until S-13.01 is CONVERGED.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-008 | Gate tool calls with pre-execution behavioral checks (PreToolUse hooks) | P0 |
| CAP-016 | Self-referential process codification via adversarial review | P1 |
| CAP-027 | Enforce artifact path discipline via registry-backed single source of truth | P1 |

## Capability Anchor Justification

**Primary anchor:** CAP-008 ("Gate tool calls with pre-execution behavioral checks
(PreToolUse hooks)") per `domain-spec/capabilities.md` §CAP-008. The
`validate-artifact-path` WASM hook (S-13.01) is a PreToolUse gate that blocks Write
and Edit tool calls targeting incorrect `.factory/` subpaths. This is the same
CAP-008 surface as the existing protection hooks in SS-04.

**CAP-027 anchor:** The registry YAML + three enforcement layers (hook, skills,
preambles) constitute the artifact path discipline system. CAP-027 is newly authored
for this cycle; ADR-016 is its backing decision.

**SS-04 (Plugin Ecosystem) anchor:** S-13.01 delivers the `validate-artifact-path`
native WASM crate at `crates/hook-plugins/validate-artifact-path/` — an SS-04 artifact.
The crate loads `plugins/vsdd-factory/config/artifact-path-registry.yaml` at startup
and enforces canonical paths at write time.

**SS-06 (Skill Catalog) anchor:** S-13.01 updates 9 creation skills (state-manager,
story-writer, product-owner, architect, test-writer, implementer, adversary,
demo-recorder, formal-verifier) to query the registry before writing artifacts. These
skills live in `plugins/vsdd-factory/skills/` — SS-06's domain. The `relocate-artifact`
skill is a new SS-06 artifact.

## Subsystem Anchors

- **SS-04 (Plugin Ecosystem):** owns `crates/hook-plugins/`. S-13.01 adds the
  `validate-artifact-path` native WASM crate, registers it in
  `plugins/vsdd-factory/hooks-registry.toml`, and adds bats tests + cargo unit tests.
  The crate must load the registry YAML via a pure function (VP-069 proptest target) and
  perform path-matching via a pure predicate (VP-070 kani target).
- **SS-06 (Skill Catalog):** owns `plugins/vsdd-factory/skills/`. S-13.01 updates
  9 creation skills to add registry-lookup preambles, and delivers the new
  `relocate-artifact` skill. The relocate skill runs detect-then-apply preflight
  (scan `.factory/` for misplaced artifacts, compute correct canonical path from
  registry, run `git mv`) before WASM hook registration.

## Stories Planned

| Story | Description | Size | Subsystem | Depends On |
|-------|-------------|------|-----------|------------|
| S-13.01 | Path governance bundle (registry YAML + `validate-artifact-path` WASM hook + 9 creation-skill updates + writing-agent prompt preambles + `relocate-artifact` skill) | L | SS-04, SS-06 | — |

S-13.01 is deliberately a single story to preserve cohesion: the registry YAML is
the single source of truth that all other parts (hook, skills, preambles) reference.
Splitting these into separate stories would create a dependency gap where the hook
could activate before the registry exists, or skills could ship without registry
awareness.

## Dependency Topology (Intra-epic)

```
S-13.01 (Path governance bundle — all parts in one story)
```

Single story; no intra-epic dependencies. Cross-epic: E-13/S-13.01 must complete
before any other story in the v1.0-feature-engine-discipline-pass-1 cycle (C → A → B
delivery order).

## Anchored BCs

| BC ID | Description | Subsystem | Priority |
|-------|-------------|-----------|----------|
| BC-4.11.001 | validate-artifact-path WASM hook blocks writes to non-canonical `.factory/` paths; registry is the single source of truth | SS-04 | P0 |
| BC-6.22.001 | relocate-artifact skill runs detect-then-apply preflight and completes before WASM hook registration (sequencing prerequisite) | SS-06 | P0 |

## Anchored ADRs

| ADR ID | Title |
|--------|-------|
| ADR-016 | Artifact path registry as single source of truth |

## Anchored VPs

| VP ID | Type | Description |
|-------|------|-------------|
| VP-069 | proptest | Registry-load purity: `load_registry()` is a pure function with no side effects |
| VP-070 | kani | Path-matching purity: `is_canonical_path()` predicate is a pure total function |
| VP-072 | integration/bats | Single-source-of-truth invariant: all 9 updated skills resolve paths from registry, not hardcoded strings |

## Delivery Sequencing Constraint

S-13.01 activates `validate-artifact-path` in `block` mode (per ADR-016, OQ5
resolution). This means:

1. The `relocate-artifact` skill MUST run detect-then-apply before hook activation.
2. Hook activation is the FINAL task in S-13.01, not the first.
3. S-13.01 must be fully CONVERGED before any other story in the cycle dispatches.

Task ordering within S-13.01:

```
T1: Author artifact-path-registry.yaml (registry SoT)
T2: Author validate-artifact-path WASM crate (does not activate yet)
T3: Update 9 creation skills + writing-agent prompt preambles
T4: Implement relocate-artifact skill
T5: Run relocate-artifact detect-then-apply preflight on current .factory/ tree
T6: Register validate-artifact-path in hooks-registry.toml (hook activates)
T7: Add bats integration tests + cargo unit tests
```

T6 (hook activation) cannot precede T5 (preflight repair). Implementer MUST enforce
this ordering. Story-writer MUST surface this constraint as an explicit task dependency
in the S-13.01 story file.

## Open Questions

| OQ ID | Scope | Description |
|-------|-------|-------------|
| OQ-E13-01 | SS-04 | Registry YAML schema: flat key→path map vs hierarchical artifact-type tree. Decision needed before S-13.01 WASM crate implementation. |
| OQ-E13-02 | SS-06 | Scope of creation-skill updates: 9 named skills confirmed; are there additional skills (e.g., codebase-analyzer, holdout-evaluator) that write `.factory/` artifacts and require registry awareness? |
| OQ-E13-03 | SS-04 | Block mode activation trigger: hook activates at registry registration time (T6 above) or at `validate-artifact-path` first invocation? Semantics differ for cold-start scenarios. |

## CHANGELOG

| Version | Date | Change |
|---------|------|--------|
| v1.0 | 2026-05-06 | Initial authoring for cycle v1.0-feature-engine-discipline-pass-1 as F3 prerequisite. One story: S-13.01 (path governance bundle, SS-04 + SS-06). Anchored BCs: BC-4.11.001 (SS-04), BC-6.22.001 (SS-06). ADR: ADR-016. VPs: VP-069, VP-070, VP-072. Delivery sequencing note (C → A → B order, S-13.01 first) documented. |
