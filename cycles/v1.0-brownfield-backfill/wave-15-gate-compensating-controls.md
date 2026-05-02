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

### CC-W15-002: S-8.08 Mutation Testing

| Field | Value |
|-------|-------|
| **Story** | S-8.08 — Native port: track-agent-start (PreToolUse:Agent) |
| **Worktree branch** | S-8.08 |
| **Control type** | mutation_testing_required: true |
| **Trigger** | In-crate unit test suite (17 tests + 8 edge-case tests) was GREEN against the stub-architect's scaffold, meaning the RED_RATIO = 0.0. Tests pass vacuously against the stub rather than against a true failing baseline. Mutation testing is needed to validate non-vacuous coverage of `track_agent_start_logic` and `extract_story_id`. |
| **Required action** | Run `cargo mutants -p track-agent-start` at wave gate and verify: (1) mutation score ≥ 80% for `extract_story_id` (regex cascade, S-N.NN/STORY-NNN patterns) and `track_agent_start_logic` (tool_name guard, subagent default, story_id field presence); (2) surviving mutants documented as acceptable gaps or killed by additional targeted tests |
| **Registered in** | S-8.08 green commit (feat(s-8.08): green) |
| **Option B source** | per-story-delivery.md §RED_RATIO_GATE Option B: "register mutation_testing_required: true; defer to wave gate" |
| **Status** | OPEN — must resolve before W-15 wave gate PASS |

### CC-W15-003: Handoff-Validator Advisory Block-Mode Pattern Review

| Field | Value |
|-------|-------|
| **Story** | S-8.01 — Native port: handoff-validator |
| **PR** | #50, merged at `60be88e` (2026-05-02) |
| **Control type** | architectural-decision-required |
| **Trigger** | S-8.01 implemented block-mode as advisory-only (dispatcher proceeds regardless of `HookResult::Block` signal) rather than as a true gate. The rationale is documented in `docs/demo-evidence/S-8.01/advisory-block-mode-rationale.md`. No SDK `HookResult::Block` variant currently exists. |
| **Required action** | At W-15 wave gate, review advisory-block-mode-rationale.md and decide: (a) add `HookResult::Block` SDK variant in W-16 as a first-class gating mechanism, OR (b) accept advisory-only pattern as canonical for W-15 and document as a v1.1 enhancement candidate. Decision must be recorded as a Decisions Log entry (D-NNN). |
| **Registered in** | D-195 |
| **Status** | OPEN — must resolve before W-15 wave gate PASS |

### CC-W15-004: WASI preopened_dir vs host::write_file Canonical FS-Write Pattern

| Field | Value |
|-------|-------|
| **Stories** | S-8.06 (WASI preopened_dir approach) vs S-8.04 (host::write_file capability-gated approach) |
| **PRs** | S-8.06 PR #51 9873f78; S-8.04 PR #54 5622aa6 |
| **Control type** | architectural-decision-required |
| **Trigger** | Two different approaches to filesystem writes shipped in the same batch: S-8.06 added `WASI preopened_dir` for `CLAUDE_PROJECT_DIR` in invoke.rs (direct WASI filesystem access), while S-8.04 uses the capability-gated `host::write_file` host function (BC-2.02.011 bounded-call mandate). Both work, but having two canonical patterns for filesystem access in hook plugins creates ambiguity for future story authors (S-8.11..S-8.29 Tier 2/3). |
| **Required action** | At W-15 wave gate, review both patterns and decide: (a) which is canonical for non-trivial filesystem writes in hook plugins (preopened_dir for read+write, host::write_file for bounded writes); (b) document the decision as an ADR or E-8 D-NNN addendum; (c) update SS-02 spec and/or E-8 epic decision D-6 with the canonical guidance. Decision must be recorded before dispatching Tier 2/3 stories. |
| **Registered in** | D-194 |
| **Status** | OPEN — must resolve before W-15 wave gate PASS |

## Closed Controls

_None yet._

## Wave Gate Checklist Addendum

At the W-15 wave gate, the standard checklist must be extended with:

- [ ] CC-W15-001: `cargo mutants -p vsdd-hook-sdk` run; mutation score ≥ 80% for S-8.30 fields; results documented
- [ ] CC-W15-002: `cargo mutants -p track-agent-start` run; mutation score ≥ 80% for `extract_story_id` and `track_agent_start_logic`; results documented
- [ ] CC-W15-003: advisory-block-mode-rationale.md reviewed; decision recorded: `HookResult::Block` SDK variant in W-16 OR accept advisory-only as canonical
- [ ] CC-W15-004: WASI preopened_dir vs host::write_file canonical pattern decided; ADR or E-8 epic D-NNN addendum recorded; SS-02 or E-8 D-6 updated
