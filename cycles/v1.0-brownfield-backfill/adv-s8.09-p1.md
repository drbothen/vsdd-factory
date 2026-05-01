---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.09-native-port-regression-gate-adapter-retirement-prep.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.09 v1.0
target_file: .factory/stories/S-8.09-native-port-regression-gate-adapter-retirement-prep.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 16
findings_high: 6
findings_med: 7
findings_low: 2
findings_nit: 1
---

# Adversarial Review: S-8.09 v1.0 (Pass 1)

## Finding ID Convention

`F-S809-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S809-P1-001: WASM target wasm32-wasi everywhere — actual SDK target is wasm32-wasip1

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** T-2 / Library & Framework Requirements (universal systematic)
- **Description:** Universal systematic finding across all Tier 1 stories. wasm32-wasi is deprecated; wasm32-wasip1 is the correct target. Empirically verified against SDK source and plugin Cargo.toml files.
- **Proposed Fix:** Replace wasm32-wasi with wasm32-wasip1 throughout.

#### F-S809-P1-002: Library & Framework Requirements lists host::write_file as "existing" — SDK does NOT expose it

- **Severity:** HIGH
- **Category:** interface-gaps
- **Location:** Library & Framework Requirements — host::write_file entry
- **Description:** The Library table lists `host::write_file` as an "existing SDK function." Empirically verified against `crates/hook-sdk/src/ffi.rs` and `crates/hook-sdk/src/host.rs`: this function does not exist. S-8.09 (regression-gate) requires file I/O for state tracking; the missing host function blocks implementation. This is the same D-6 Option A trigger as S-8.04. Both S-8.04 and S-8.09 are BLOCKED pending SDK extension PR.
- **Evidence:** `crates/hook-sdk/src/host.rs`: no `write_file` symbol. `crates/hook-sdk/src/ffi.rs`: no `__host_write_file` binding.
- **Proposed Fix:** Remove host::write_file from "existing" column; add it to "required SDK extension" column. Mark S-8.09 BLOCKED with explicit prerequisite on SDK extension PR (same as S-8.04).

#### F-S809-P1-003: OQ-6 gate condition is unverifiable — "satisfying profile" criteria not defined; story pre-anchors security-reviewer to specific profile

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** OQ-6 gate / AC-010
- **Description:** AC-010 requires OQ-6 gate clearance ("security-reviewer must produce E-8-oq6-capability-profile.md before implementation"). However, the acceptance criteria for what constitutes a "satisfying profile" are not defined in the story or the E-8 epic. The security-reviewer cannot know what to produce, and the orchestrator cannot evaluate gate passage. Additionally, AC-010 pre-anchors the reviewer to a specific profile structure, constraining the OQ-6 evaluation before it occurs.
- **Proposed Fix:** Add acceptance criteria for OQ-6 gate: specify what fields/sections E-8-oq6-capability-profile.md must contain to satisfy the gate. Do not pre-anchor the profile structure in the story.

#### F-S809-P1-004: AC-009 is internally contradictory and risks shipping a falsely-passing test

- **Severity:** HIGH
- **Category:** contradictions
- **Location:** AC-009
- **Description:** AC-009 references BC-7.03.073 as its trace anchor AND includes the caveat "may be revised post-implementation." An AC that is acknowledged as potentially incorrect at authoring time cannot be a valid acceptance criterion. If the test passes against an incorrect AC, the implementation may be wrong but appear correct.
- **Evidence:** AC-009 text: "[may be revised post-implementation]" qualifier present at story authoring time.
- **Proposed Fix:** Resolve the AC-009 uncertainty before story authoring is complete. Either pin the correct BC trace or replace the AC with a TO-DO placeholder that blocks implementation until resolved.

#### F-S809-P1-005: AC-007 mixes 9 scenarios but test count, scenario letters, and bats fixture coverage are inconsistent

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** AC-007 scenarios (a)-(i)
- **Description:** AC-007 enumerates scenarios (a) through (i) (9 scenarios). However: (1) only `cargo test` and `git commit` scenarios are confirmed to have bats fixtures; (2) scenarios (c)-(h) lack fixture coverage specification; (3) the scenario letter assignments do not match the bash source's case ordering. The WASM port has no verified test coverage for 7 of 9 scenarios.
- **Evidence:** AC-007 text: scenarios (a)=(cargo test) and (b)=(git commit) have explicit fixture paths; (c)-(i) have no fixture paths.
- **Proposed Fix:** For each scenario (a)-(i): (1) specify the bats fixture file and test name, or (2) explicitly mark as "behavior-verified via bash parity" with the specific bash branch reference.

#### F-S809-P1-006: AC-002 is destructive deletion AC with NO blast-radius safeguard for hooks.json downstream consumers

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** AC-002
- **Description:** AC-002 prescribes deletion of hooks.json entries with no safeguard for downstream consumers that may still reference the hooks.json format (e.g., CI scripts, dev tooling, documentation). As the W-15-closing story, S-8.09's deletions mark the completion of Tier 1 migration; any hooks.json consumers that survive the deletion are undetected breaking changes. No impact analysis is included.
- **Proposed Fix:** Add a pre-deletion impact analysis task: "grep codebase for remaining hooks.json references to regression-gate entries; confirm no downstream consumers before deletion."

### MEDIUM

#### F-S809-P1-007: Adapter pre-retirement audit (AC-011) cannot pass at S-8.09 close per story's own dependency graph

- **Severity:** MEDIUM
- **Category:** contradictions
- **Location:** AC-011
- **Description:** AC-011 requires all Tier 1 adapters to be audited as part of S-8.09 completion. However, S-8.09's dependency graph includes S-8.01..S-8.08 as prerequisites. If S-8.09 closes out the Tier 1 batch, then S-8.01..S-8.08 must already be complete — but their adapters are audited by S-8.09's AC-011. This creates a self-referential gate paradox: S-8.09 cannot audit what it depends on without those dependencies already being closed.
- **Proposed Fix:** Move AC-011 adapter audit to a separate post-S-8.09 story (e.g., S-8.10 or a dedicated audit story), or reframe AC-011 as a verification-only step that confirms already-completed adapter retirements.

#### F-S809-P1-008: AC-012 `find . -name emit-event -type f | grep bin/` is unbounded and environment-dependent

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** AC-012
- **Description:** AC-012 specifies a `find . -name emit-event -type f | grep bin/` command to verify adapter retirement. This command is unbounded (searches entire workspace), environment-dependent (depends on `.gitignore`, `.dockerignore`, mounted volumes), and may produce false positives from build artifacts or vendored dependencies. A bounded, deterministic check is needed.
- **Proposed Fix:** Replace with a bounded path: `find ./src ./scripts ./hooks -name emit-event -type f` or an equivalent git-aware command that excludes build artifacts.

#### F-S809-P1-009: AC-008 BC trace broken — BC-7.03.071 invariant 2 referenced but BC invariants not enumerated in spec

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-008 → BC-7.03.071 invariant 2
- **Description:** AC-008 traces to BC-7.03.071 invariant 2. BC-7.03.071's invariant 2 is not enumerated in the story's body BC table, making the trace unverifiable against the story's own documentation.
- **Proposed Fix:** Enumerate BC-7.03.071's invariants in the body BC table, or confirm the invariant 2 content and add it.

#### F-S809-P1-010: EC-006 silently swallows state-file write failures — violates AGENT-SOUL silent-failure principle

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** EC-006
- **Description:** EC-006 specifies that state-file write failures are silently swallowed (exit 0, no log). This violates the AGENT-SOUL principle against silent failures. For a regression-gate hook, a silent write failure would leave the state file in an inconsistent state without any observability signal, potentially causing false-pass on subsequent runs.
- **Evidence:** AGENT-SOUL.md: "Never silently swallow errors that affect observable state."
- **Proposed Fix:** Update EC-006: on state-file write failure, emit an error event or log to stderr before exiting 0. Escalate to a non-zero exit if the failure is unrecoverable.

#### F-S809-P1-011: EC-003 malformed-prior-state semantics differ from bash source (bash suppresses stderr; spec doesn't mandate observability)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** EC-003
- **Description:** EC-003 covers malformed prior-state handling. The bash source uses `2>/dev/null` to suppress parse errors, making malformed state silently ignored. The WASM port's EC-003 does not mandate equivalent observability suppression. An implementer who adds stderr logging for malformed state would diverge from bash behavior.
- **Proposed Fix:** Specify EC-003 explicitly: malformed prior state → silently ignore (matching bash `2>/dev/null` semantics), OR intentionally add observability (stderr log) and document the behavioral deviation.

#### F-S809-P1-012: AC-007(i) perf comparison vs S-8.00 baseline — S-8.00 baseline is bash, not WASM; regression-gate not in measured set

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-007(i)
- **Description:** Same class as F-S807-P1-011. AC-007(i) compares WASM performance to S-8.00 baseline. The S-8.00 baseline measures bash hooks; regression-gate is not in its measured set. The comparison is undefined.
- **Proposed Fix:** Remove S-8.00 baseline reference or define a regression-gate-specific baseline measurement.

#### F-S809-P1-013: regression-gate has no tool filter in registry but WASM port must body-filter; capability declarations don't reflect this

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** Registry capability declarations / AC-003
- **Description:** The regression-gate hook filters events by tool type (cargo test, git commit, etc.) in the hook body. The registry entry does not include tool-type filters (no `tool_filter` or equivalent). This means the hook fires on ALL SubagentStop events and must self-filter. The capability declarations do not reflect the self-filtering pattern, nor is there an AC that tests that the hook correctly ignores non-matching tool types.
- **Proposed Fix:** Add an EC for non-matching tool types: "on SubagentStop event with tool not in regression-gate target set, exit 0 without state update or emit."

#### F-S809-P1-014: T-2 wait-for-audit-doc is not orchestrator-actionable — no signal mechanism

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** T-2
- **Description:** T-2 includes "wait for E-8-oq6-capability-profile.md to be produced by security-reviewer" as a task step. This is not orchestrator-actionable — there is no signal mechanism (file watcher, CI gate, manual flag) defined for when the document is available. The implementer has no way to know when T-2's wait condition is satisfied.
- **Proposed Fix:** Define the signal mechanism: either (a) file existence check in CI (bats test: `test -f .factory/E-8-oq6-capability-profile.md`), or (b) explicit orchestrator gate step in the wave schedule.

### LOW

#### F-S809-P1-015: Token Budget arithmetic excludes OQ-6 audit doc on first read but includes ~400 tokens for it

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Token Budget
- **Description:** The Token Budget's arithmetic excludes the OQ-6 audit doc from the "first read" phase total, but then adds ~400 tokens for it in a separate line. The arithmetic is inconsistent — the total may double-count or under-count depending on whether the OQ-6 doc is read.
- **Proposed Fix:** Clarify Token Budget arithmetic: exclude OQ-6 audit doc from total until it exists (conditional cost), and annotate accordingly.

#### F-S809-P1-016: Token Budget architecture excludes OQ-6 audit doc — total count inconsistent across table rows

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Token Budget total row
- **Description:** Related to F-S809-P1-015. The Token Budget total row is inconsistent across passes due to the conditional OQ-6 audit doc inclusion.
- **Proposed Fix:** Consolidate into F-S809-P1-015 fix.

### NIT

#### F-S809-P1-016-NIT: Story heading inconsistency — "W-15 closer" in body vs "adapter retirement prep" in filename

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** Story heading vs filename
- **Description:** The story body uses "W-15 closer" as the characterization while the filename uses "adapter retirement prep." The two descriptions emphasize different aspects of the story. Minor inconsistency; no behavioral impact.
- **Proposed Fix:** Align body heading with filename or vice versa. Suggest: "Native port: regression-gate + adapter retirement prep (W-15 closer)" as the canonical title.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 6 |
| MEDIUM | 7 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** BLOCKED — host::write_file absent in SDK (D-6 Option A trigger, same as S-8.04); OQ-6 gate criteria undefined; fix burst required before pass-2

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 16 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 16/16 = 1.00 |
| **Median severity** | HIGH/MED |
| **Trajectory** | 16 |
| **Verdict** | FINDINGS_REMAIN |
