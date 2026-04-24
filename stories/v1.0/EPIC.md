# v1.0 Factory Plugin Kit — Epic

**Design doc:** [`../../specs/2026-04-24-v1.0-factory-plugin-kit-design.md`](../../specs/2026-04-24-v1.0-factory-plugin-kit-design.md)
**Milestone ladder:** `1.0.0-beta.1` → `1.0.0-rc.1` → `1.0.0`

This epic decomposes the v1.0 design into stories, with explicit dependency
tiers so that independent work can progress in parallel. Each story has a
contract defining what it exposes to other stories; cross-tier work proceeds
only after the prior tier's contracts are frozen.

## Dependency tiers

Visually, the graph is wide then narrows: Tier A can start today; Tier B
fans out behind a single blocker; Tiers C–H are gated on the tier before.

```
Tier A (start immediately, all parallel)
    ├── S-0.1 — bump-version.sh prerelease support
    ├── S-0.2 — Release workflow prerelease handling
    ├── S-0.3 — Activation skill platform detection
    ├── S-0.4 — hooks.json.template + CI generation
    └── S-0.5 — Docs scaffolding

Tier B.0 (single blocker, gate for Tier B.x)
    └── S-1.1 — Cargo workspace + CI scaffolding ⚠️ BLOCKING

Tier B.x (after S-1.1, all parallel)
    ├── S-1.2 — factory-dispatcher core (stdin, TOML load, routing)
    ├── S-1.3 — hook-sdk crate (macro, types, bindings)
    ├── S-1.4 — Host function surface implementation
    ├── S-1.5 — wasmtime integration + epoch/fuel enforcement
    ├── S-1.6 — tokio + parallel-within-tier execution
    ├── S-1.7 — dispatcher-internal.jsonl writer
    ├── S-1.8 — sink-file driver (default file-sink)
    └── S-1.9 — sink-otel-grpc driver (beta.1 scope)

Tier C (after Tier B.x, parallel)
    ├── S-2.1 — legacy-bash-adapter WASM plugin
    ├── S-2.2 — hooks-registry.toml auto-generation from existing hooks.json
    ├── S-2.3 — Cross-platform CI matrix build targets
    ├── S-2.4 — Binary commit automation in Release workflow
    ├── S-2.5 — hook-sdk publish to crates.io (0.1.0)
    ├── S-2.6 — Activation skill integrates with real hooks.json variants
    └── S-2.7 — Regression test suite validation

Tier D (blocks on Tier C)
    └── S-2.8 — 1.0.0-beta.1 release gate + tag 🏁

Tier E (after beta.1 stable, parallel)
    ├── S-3.1 — Port capture-commit-activity to WASM
    ├── S-3.2 — Port capture-pr-activity to WASM
    ├── S-3.3 — Port block-ai-attribution to WASM
    ├── S-3.4 — emit_event as host function refactor
    ├── S-4.1 — sink-http driver
    ├── S-4.2 — sink-datadog driver
    ├── S-4.3 — sink-honeycomb driver
    ├── S-4.4 — Per-sink retry + circuit breaker
    ├── S-4.5 — Dead letter queue implementation
    ├── S-4.6 — Per-sink routing filters + tag enrichment
    └── S-4.7 — End-to-end observability integration tests

Tier F (blocks on Tier E)
    └── S-4.8 — 1.0.0-rc.1 release gate + tag 🏁

Tier G (after rc.1 stable, parallel)
    ├── S-5.1 — SessionStart hook wiring
    ├── S-5.2 — SessionEnd hook wiring
    ├── S-5.3 — WorktreeCreate / WorktreeRemove hook wiring
    ├── S-5.4 — PostToolUseFailure hook wiring
    ├── S-5.5 — Migration guide (0.79.x → 1.0)
    └── S-5.6 — Semver commitment documentation

Tier H (blocks on Tier G)
    └── S-5.7 — 1.0.0 release gate + tag 🏁
```

## Milestone → stories

| Milestone | Stories included |
|---|---|
| `1.0.0-beta.1` | S-0.1 through S-0.5 (Phase 0); S-1.1 through S-1.9 (Phase 1); S-2.1 through S-2.7 (Phase 2); S-2.8 (release gate) |
| `1.0.0-rc.1` | S-3.1 through S-3.4 (Phase 3); S-4.1 through S-4.7 (Phase 4); S-4.8 (release gate) |
| `1.0.0` | S-5.1 through S-5.6 (Phase 5); S-5.7 (release gate) |

## Parallelism principles

1. **Contracts are the unit of coordination.** Every story's "Contracts" section
   declares what it exposes to consumers. Downstream stories depend on
   contracts, not on implementation details.
2. **A story is "done" when its acceptance criteria pass, not when it's
   committed.** Merged to main but blocking on a dep = still done; the dep is
   a separate concern.
3. **Inside a tier, stories are genuinely independent.** If two stories in the
   same tier touch the same file, at least one needs to split or a
   serialization story inserted.
4. **Cross-tier work waits on contracts freezing, not on code landing.** A
   downstream story can start designing against the contract as soon as the
   upstream story's contract is approved, even if upstream implementation is
   in flight.

## Naming

`S-<tier-phase>.<sequence>` where tier-phase aligns with the design-doc phase
(0 for infra, 1 for dispatcher, 2 for adapter, etc.) and sequence is story
order within that phase. The grouping in the tier graph above is an
execution-order view; the ID is a phase view.

## What a story file contains

See `templates/STORY-TEMPLATE.md` (in this directory) for the exact shape.
Every story in this epic follows it.

## Current state (2026-04-24)

- **Spec:** [`v1.0-factory-plugin-kit-design.md`](../../specs/2026-04-24-v1.0-factory-plugin-kit-design.md) — frozen, all Q1–Q7 resolved
- **Epic:** complete
- **Stories drafted:** all 36 stories (S-0.1 through S-5.7)
- **Stories pending draft:** none
- **Work in progress:** none yet — ready to begin implementation of Tier A
  + S-1.1 in parallel
