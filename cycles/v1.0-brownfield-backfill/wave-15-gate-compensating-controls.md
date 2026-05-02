---
document_type: wave-gate-compensating-controls
wave: W-15
cycle: v1.0-brownfield-backfill
status: retired
producer: state-manager
created: 2026-05-02
closed: 2026-05-02
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
| **Status** | RETIRED — 2026-05-02. W-15 wave gate CONVERGED (D-208). Mutation testing deferred per wave gate consensus; residual tracked as TD item. |

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
| **Status** | RETIRED — 2026-05-02. W-15 wave gate CONVERGED (D-208). Mutation testing deferred per wave gate consensus; residual tracked as TD item. |

### CC-W15-003: Handoff-Validator Advisory Block-Mode Pattern Review

| Field | Value |
|-------|-------|
| **Story** | S-8.01 — Native port: handoff-validator |
| **PR** | #50, merged at `60be88e` (2026-05-02) |
| **Control type** | architectural-decision-required |
| **Trigger** | S-8.01 implemented block-mode as advisory-only (dispatcher proceeds regardless of `HookResult::Block` signal) rather than as a true gate. The rationale is documented in `docs/demo-evidence/S-8.01/advisory-block-mode-rationale.md`. No SDK `HookResult::Block` variant currently exists. |
| **Required action** | At W-15 wave gate, review advisory-block-mode-rationale.md and decide: (a) add `HookResult::Block` SDK variant in W-16 as a first-class gating mechanism, OR (b) accept advisory-only pattern as canonical for W-15 and document as a v1.1 enhancement candidate. Decision must be recorded as a Decisions Log entry (D-NNN). |
| **Registered in** | D-195 |
| **Status** | RETIRED — 2026-05-02. Advisory block-mode pattern corrected in PR #60 (executor.rs:90 drops on_error==Block precondition; stdout block emission added). Canonical pattern documented in HOST_ABI.md via PR #61. D-206. |

### CC-W15-004: WASI preopened_dir vs host::write_file Canonical FS-Write Pattern

| Field | Value |
|-------|-------|
| **Stories** | S-8.06 (WASI preopened_dir approach) vs S-8.04 (host::write_file capability-gated approach) |
| **PRs** | S-8.06 PR #51 9873f78; S-8.04 PR #54 5622aa6 |
| **Control type** | architectural-decision-required |
| **Trigger** | Two different approaches to filesystem writes shipped in the same batch: S-8.06 added `WASI preopened_dir` for `CLAUDE_PROJECT_DIR` in invoke.rs (direct WASI filesystem access), while S-8.04 uses the capability-gated `host::write_file` host function (BC-2.02.011 bounded-call mandate). Both work, but having two canonical patterns for filesystem access in hook plugins creates ambiguity for future story authors (S-8.11..S-8.29 Tier 2/3). |
| **Required action** | At W-15 wave gate, review both patterns and decide: (a) which is canonical for non-trivial filesystem writes in hook plugins (preopened_dir for read+write, host::write_file for bounded writes); (b) document the decision as an ADR or E-8 D-NNN addendum; (c) update SS-02 spec and/or E-8 epic decision D-6 with the canonical guidance. Decision must be recorded before dispatching Tier 2/3 stories. |
| **Registered in** | D-194 |
| **Status** | RETIRED — 2026-05-02. Architectural decision deferred to v1.1: preopened_dir for broader read+write access, host::write_file for bounded capability-gated writes. WASI preopened_dir surface documented in HOST_ABI.md (Filesystem Access Model). D-208. |

### CC-W15-005: regression-gate Mutation Testing

| Field | Value |
|-------|-------|
| **Story** | S-8.09 — Native port: regression-gate + adapter retirement prep |
| **PR** | #58, merged at `3adfe0b` (2026-05-02) |
| **Control type** | mutation_testing_required: true |
| **Trigger** | 36 unit tests in regression-gate crate cover the 9-pattern test-runner matching logic and pass→fail transition detection. RED_RATIO not verified (stub-architect pattern used); mutation testing needed to confirm tests are non-vacuous and achieve adequate branch coverage of the pattern-matching cascade. |
| **Required action** | Run `cargo mutants -p regression-gate` at wave gate and verify: (1) mutation score ≥ 80% for the 9-pattern list matching logic and pass→fail transition warning path; (2) surviving mutants documented as acceptable gaps or killed by additional targeted tests |
| **Source decision** | D-202 |
| **Status** | RETIRED — 2026-05-02. W-15 wave gate CONVERGED (D-208). Mutation testing deferred per wave gate consensus; residual tracked as TD item. |

### CC-W15-006: macOS symlink fix CI verification

| Field | Value |
|-------|-------|
| **Story** | S-8.09 — regression-gate (bonus dispatcher fix in invoke.rs) |
| **PR** | #58, merged at `3adfe0b` (2026-05-02) |
| **Control type** | environment-verification-required |
| **Trigger** | The macOS `/var/folders/` symlink issue in `write_file` path_allow ancestor fallback was fixed by applying `canonicalize()` before the ancestor walk. The fix was validated on dev machine but NOT on the actual macOS CI runner (darwin-arm64 / darwin-x64 matrix). The CI matrix uses hosted runners where `/var/folders/` symlink behavior may differ. |
| **Required action** | At W-15 wave gate, confirm that CI passes on darwin-arm64 and darwin-x64 runner targets for any test that exercises `host::write_file` with temp-dir paths. Specifically verify that `path_allowed()` returns true for canonical paths under `/private/var/folders/` when the preopened path_allow list uses the `/var/folders/` symlink form. |
| **Source decision** | D-202 |
| **Status** | RETIRED — 2026-05-02. CI passed on darwin-arm64 and darwin-x64 per wave gate adversary re-run #2 (CONVERGED). macOS symlink fix verified functional. D-208. |

### CC-W15-007: regression-gate 9-pattern list completeness

| Field | Value |
|-------|-------|
| **Story** | S-8.09 — Native port: regression-gate + adapter retirement prep |
| **PR** | #58, merged at `3adfe0b` (2026-05-02) |
| **Control type** | completeness-review-required |
| **Trigger** | The regression-gate crate encodes a hardcoded list of 9 test-runner output patterns used to detect pass/fail transitions. This list was derived from the existing bats test suite output at time of implementation. New test frameworks or output format changes could produce patterns not in the list, causing silent test misses (false PASS verdicts). |
| **Required action** | At W-15 wave gate, team reviews the 9-pattern list in `regression_gate_logic` and confirms: (1) all test output patterns currently produced by the workspace test suite are covered; (2) any patterns not covered are either acceptable gaps or trigger a new story for pattern extension; decision recorded as a D-NNN entry or task. |
| **Source decision** | D-202 |
| **Status** | RETIRED — 2026-05-02. 9-pattern list accepted as complete for W-15 scope; pattern extension story tracked for future waves. D-208. |

### CC-W15-008: Release Pipeline Must Build All 16 Native WASM Plugins (BLOCKING)

| Field | Value |
|-------|-------|
| **Source** | CRIT-W15-001 from wave-15-gate-adversary.md |
| **Control type** | release-pipeline-fix-required |
| **Severity** | BLOCKING |
| **Trigger** | `.github/workflows/release.yml` builds only 3 of 16 native WASM plugins. The 9 W-15 plugins are absent from the build matrix. Every release produces activation-time `LoadFailed` for all 9 missing plugins. |
| **Required action** | Fix release.yml and ci.yml to build all 16 native WASM plugins. Use `--workspace` build + stage all `.wasm` outputs into `plugins/vsdd-factory/hook-plugins/`. Add a count-check step (must equal 16). Fix is item 1 in wave-15-gate-fix-burst-plan.md. |
| **Registered by** | W-15 wave gate adversary review (2026-05-02) |
| **Status** | RETIRED — 2026-05-02. CLOSED in PR #59: release.yml + ci.yml updated to build all 16 native WASM plugins; count-verification step added. CRIT-W15-001 CLOSED. D-205. |

### CC-W15-009: update-wave-state-on-merge `default = ["standalone"]` Must Be Inverted (BLOCKING)

| Field | Value |
|-------|-------|
| **Source** | HIGH-W15-004 from wave-15-gate-adversary.md |
| **Control type** | fail-safe-default-required |
| **Severity** | BLOCKING |
| **Trigger** | `update-wave-state-on-merge/Cargo.toml` declares `default = ["standalone"]`. The standalone feature enables direct file I/O bypassing the host::write_file capability-gated path. Fail-safe default should be `default = []` (capability-gated path). With the current default, any build without `--no-default-features` silently uses the less-sandboxed path. |
| **Required action** | Invert to `default = []` in `crates/hook-plugins/update-wave-state-on-merge/Cargo.toml`. The release.yml fix-burst step already uses `--no-default-features` (redundant but harmless after inversion). Fix is item 6 in wave-15-gate-fix-burst-plan.md. |
| **Registered by** | W-15 wave gate adversary review (2026-05-02) |
| **Status** | RETIRED — 2026-05-02. CLOSED in PR #59: `default = []` in Cargo.toml. HIGH-W15-004 CLOSED. D-205. |

### CC-W15-010: update-wave-state-on-merge Regex False-Positive Class (HIGH)

| Field | Value |
|-------|-------|
| **Source** | CRIT-W15-004 from wave-15-gate-adversary.md |
| **Control type** | correctness-fix-required |
| **Severity** | HIGH |
| **Trigger** | `update-wave-state-on-merge/src/lib.rs:98` regex `merge\|squash` matches "merge conflict", "squash strategy" — false-positive wave state updates on unrelated PRs. Doc comments and bash semantics specify `merged\|squash.*merge`. |
| **Required action** | Fix regex in lib.rs:98 to `merged\|squash.*merge`. Add regression tests for false-positive strings ("fix merge conflict", "squash redundant commits"). Fix is item 5 in wave-15-gate-fix-burst-plan.md. |
| **Registered by** | W-15 wave gate adversary review (2026-05-02) |
| **Status** | RETIRED — 2026-05-02. CLOSED in PR #59: regex fixed to `merged|squash.*merge`; false-positive regression tests added. CRIT-W15-004 CLOSED. D-205. |

### CC-W15-011: Whitespace Counting Alignment (chars vs bytes) (HIGH)

| Field | Value |
|-------|-------|
| **Source** | HIGH-W15-002 from wave-15-gate-adversary.md |
| **Control type** | consistency-fix-required |
| **Severity** | HIGH |
| **Trigger** | `handoff-validator` uses `.chars().filter(|c| c.is_whitespace())` (Unicode-aware). `track-agent-stop` uses `.bytes().filter(|b| *b == b' ' \|\| ...)` (ASCII-only). Two sibling plugins that both count whitespace will disagree on Unicode inputs. |
| **Required action** | Align both plugins to `.chars().filter(|c| c.is_whitespace())` (Unicode-aware canonical). Add doc comment in both noting the canonical approach. Add Unicode whitespace test in both. Fix is item 9 in wave-15-gate-fix-burst-plan.md. |
| **Registered by** | W-15 wave gate adversary review (2026-05-02) |
| **Status** | RETIRED — 2026-05-02. CLOSED in PR #59: both plugins aligned to `.chars().filter(|c| c.is_whitespace())`; Unicode whitespace tests added. HIGH-W15-002 CLOSED. D-205. |

## Closed Controls

All 11 controls RETIRED 2026-05-02 (W-15 wave gate CONVERGED — D-208).

## Wave Gate Checklist Addendum

At the W-15 wave gate, the standard checklist must be extended with:

- [x] CC-W15-001: RETIRED — mutation testing deferred; tracked as TD item. D-208.
- [x] CC-W15-002: RETIRED — mutation testing deferred; tracked as TD item. D-208.
- [x] CC-W15-003: RETIRED — advisory-only pattern corrected in PR #60; canonical pattern documented in HOST_ABI.md via PR #61. D-206/D-207.
- [x] CC-W15-004: RETIRED — preopened_dir vs host::write_file documented in HOST_ABI.md. D-208.
- [x] CC-W15-005: RETIRED — mutation testing deferred; tracked as TD item. D-208.
- [x] CC-W15-006: RETIRED — CI CONVERGED on darwin-arm64/x64 per wave gate re-run #2. D-208.
- [x] CC-W15-007: RETIRED — 9-pattern list accepted as complete for W-15 scope. D-208.
- [x] CC-W15-008: RETIRED — PR #59 fixed release.yml + ci.yml to build all 16 plugins; count check added. D-205.
- [x] CC-W15-009: RETIRED — PR #59 inverted to `default = []`. D-205.
- [x] CC-W15-010: RETIRED — PR #59 fixed regex to `merged|squash.*merge` + regression tests. D-205.
- [x] CC-W15-011: RETIRED — PR #59 aligned both plugins to chars-based filter + Unicode tests. D-205.
