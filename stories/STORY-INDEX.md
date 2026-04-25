---
document_type: story-index
level: ops
version: "1.0"
status: current
producer: story-writer
timestamp: 2026-04-25T00:00:00
phase: 1.8
inputs:
  - .factory/stories/v1.0/EPIC.md
  - .factory/phase-0-ingestion/pass-6-synthesis.md
  - .factory/specs/architecture/ARCH-INDEX.md
traces_to: .factory/specs/domain-spec/capabilities.md
---

# Story Index — vsdd-factory v1.0.0-greenfield

> Auto-generated during Phase 1.8 migration from legacy S-N.M format to canonical
> S-N.MM format. This index is the authoritative source for story count and status.
> 41 stories across 6 epics (E-0 through E-5).

## Status Summary

| Status | Count |
|--------|-------|
| merged | 22 |
| partial | 4 |
| draft | 15 |
| **Total** | **41** |

## Epic E-0 — Infrastructure Prep (Tier A — all merged)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-0.01 | bump-version.sh prerelease support | E-0 | 2 | P0 | -- | merged |
| S-0.02 | Release workflow prerelease handling | E-0 | 2 | P0 | S-0.01 | merged |
| S-0.03 | Activation skill platform detection | E-0 | 3 | P0 | -- | merged |
| S-0.04 | hooks.json.template + CI generation | E-0 | 3 | P0 | -- | merged |
| S-0.05 | Documentation scaffolding | E-0 | 2 | P1 | -- | merged |

## Epic E-1 — Dispatcher Foundation (Tier B.0 + B.x — all merged)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-1.01 | Cargo workspace + CI scaffolding | E-1 | 5 | P0 | -- | merged |
| S-1.02 | factory-dispatcher core (stdin, TOML load, routing) | E-1 | 8 | P0 | S-1.01 | merged |
| S-1.03 | hook-sdk crate (macro, types, bindings) | E-1 | 8 | P0 | S-1.01 | merged |
| S-1.04 | Host function surface implementation | E-1 | 8 | P0 | S-1.01, S-1.02, S-1.03 | merged |
| S-1.05 | wasmtime integration + epoch/fuel enforcement | E-1 | 8 | P0 | S-1.01, S-1.02, S-1.04 | merged |
| S-1.06 | tokio + parallel-within-tier execution | E-1 | 5 | P0 | S-1.01, S-1.02, S-1.04, S-1.05 | merged |
| S-1.07 | dispatcher-internal.jsonl writer | E-1 | 3 | P0 | S-1.01, S-1.02 | merged |
| S-1.08 | sink-file driver | E-1 | 5 | P0 | S-1.01, S-1.07 | merged |
| S-1.09 | sink-otel-grpc driver | E-1 | 5 | P0 | S-1.01, S-1.08 | merged |

## Epic E-2 — Legacy Adapter and Beta Release (Tier C + D — mostly merged)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-2.01 | legacy-bash-adapter WASM plugin | E-2 | 5 | P0 | S-1.03, S-1.04, S-1.05, S-1.06 | merged |
| S-2.02 | hooks-registry.toml auto-generation | E-2 | 2 | P0 | S-2.01 | merged |
| S-2.03 | Cross-platform CI matrix build targets | E-2 | 5 | P0 | S-1.01, S-1.02 | merged |
| S-2.04 | Binary commit automation in Release workflow | E-2 | 5 | P0 | S-2.03 | merged |
| S-2.05 | hook-sdk publish to crates.io (0.1.0) | E-2 | 2 | P1 | S-1.03 | partial |
| S-2.06 | Activation skill integrates with real hooks.json variants | E-2 | 3 | P0 | S-0.03, S-0.04, S-2.04 | merged |
| S-2.07 | Regression test suite validation | E-2 | 5 | P0 | S-1.02, S-1.04, S-1.05, S-1.06, S-1.07, S-1.08, S-2.01, S-2.02 | merged |
| S-2.08 | 1.0.0-beta.1 release gate | E-2 | 3 | P0 | all S-0.x + S-1.x + S-2.01..S-2.07 | merged |

## Epic E-3 — WASM Port: High-Value Hooks (Tier E — draft/partial)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-3.04 | emit_event as host function refactor | E-3 | 3 | P0 | S-1.04 | partial |
| S-3.01 | Port capture-commit-activity to WASM | E-3 | 5 | P1 | S-2.08, S-3.04 | draft |
| S-3.02 | Port capture-pr-activity to WASM | E-3 | 5 | P1 | S-2.08, S-3.04 | draft |
| S-3.03 | Port block-ai-attribution to WASM | E-3 | 3 | P1 | S-2.08, S-3.04 | draft |

## Epic E-4 — Observability Sinks and RC Release (Tier E + F — draft/partial)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-4.01 | sink-http driver | E-4 | 5 | P1 | S-1.08 | draft |
| S-4.02 | sink-datadog driver | E-4 | 5 | P1 | S-1.08, S-4.01 | draft |
| S-4.03 | sink-honeycomb driver | E-4 | 3 | P1 | S-1.08, S-4.01 | draft |
| S-4.04 | Per-sink retry + circuit breaker | E-4 | 8 | P1 | S-1.08, S-4.01 | draft |
| S-4.05 | Dead letter queue implementation | E-4 | 3 | P1 | S-4.04 | draft |
| S-4.06 | Per-sink routing filters + tag enrichment | E-4 | 3 | P1 | S-1.08 | partial |
| S-4.07 | End-to-end observability integration tests | E-4 | 8 | P1 | S-3.01..S-3.04, S-4.01..S-4.06 | draft |
| S-4.08 | 1.0.0-rc.1 release gate | E-4 | 3 | P0 | S-4.07 + 2-week shakedown | draft |

## Epic E-5 — New Hook Events and 1.0.0 Release (Tier G + H — draft/partial)

| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-5.01 | SessionStart hook wiring | E-5 | 3 | P1 | S-4.08 | draft |
| S-5.02 | SessionEnd hook wiring | E-5 | 3 | P1 | S-4.08 | draft |
| S-5.03 | WorktreeCreate / WorktreeRemove hook wiring | E-5 | 5 | P1 | S-4.08 | draft |
| S-5.04 | PostToolUseFailure hook wiring | E-5 | 3 | P1 | S-4.08 | draft |
| S-5.05 | Migration guide (0.79.x → 1.0) | E-5 | 5 | P1 | S-4.08 | partial |
| S-5.06 | Semver commitment documentation | E-5 | 2 | P1 | S-4.08 | draft |
| S-5.07 | 1.0.0 release gate | E-5 | 3 | P0 | S-5.01..S-5.06 + 1-week shakedown | draft |

---

**Status values:** draft, ready, in-progress, merged, partial, blocked

**Total story points:** 183 across 41 stories

**Rules:**
- Every story has a unique sequential ID (zero-padded: S-N.MM)
- Points are 1-13 (no story exceeds 13 points)
- Dependencies are acyclic (topological sort valid)
- P0 stories do not depend on P1/P2 stories
