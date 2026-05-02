---
document_type: wave-gate-compensating-controls
wave: W-15
cycle: v1.0-brownfield-backfill
status: active
producer: state-manager
created: 2026-05-02
---

# W-15 Wave Gate Compensating Controls

This file tracks compensating controls that MUST be executed at the W-15 wave gate
before the wave can be declared complete. Each entry was registered during the
per-story-delivery cycle when a standard gate criterion was deferred under Option B
(compensating-control registration in lieu of immediate remediation).

## Open Controls

### CC-W15-001: S-8.30 Mutation Testing

| Field | Value |
|-------|-------|
| **Story** | S-8.30 — SDK extension: HookPayload SubagentStop top-level fields |
| **PR** | #49, merged at `394d991` (2026-05-02) |
| **Control type** | mutation_testing_required: true |
| **Trigger** | RED_RATIO = 0.0 from stub-architect overshoot — stub-architect pre-created struct fields before test-writer wrote tests, causing tests to pass against the stub rather than against a true failing baseline |
| **Required action** | Run `cargo mutants -p vsdd-hook-sdk` at wave gate and verify: (1) mutation score ≥ 80% for the 4 new HookPayload fields (agent_type, subagent_name, last_assistant_message, result); (2) all surviving mutants are documented as acceptable gaps or killed by additional targeted tests |
| **Registered in** | `.factory/cycles/v1.0-brownfield-backfill/S-8.30/implementation/red-gate-log.md` |
| **Option B source** | per-story-delivery.md §RED_RATIO_GATE Option B: "register mutation_testing_required: true; defer to wave gate" |
| **Source decision** | D-193 |
| **Status** | OPEN — must resolve before W-15 wave gate PASS |

## Closed Controls

_None yet._

## Wave Gate Checklist Addendum

At the W-15 wave gate, the standard checklist must be extended with:

- [ ] CC-W15-001: `cargo mutants -p vsdd-hook-sdk` run; mutation score ≥ 80% for S-8.30 fields; results documented
