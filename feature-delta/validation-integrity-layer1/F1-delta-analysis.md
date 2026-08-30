---
document_type: f1-delta-analysis
feature_id: validation-integrity-layer1
feature_name: "Dispatcher INDETERMINATE Outcome — Validation Integrity Layer 1"
intent: feature
feature_type: infrastructure
trivial_scope: false
severity: null
level: L4
version: "1.0"
status: draft
producer: architect
timestamp: 2026-08-30T00:00:00Z
traces_to: .factory/specs/architecture/ARCH-INDEX.md
last_amended: "2026-08-30 (v1.0) — initial F1 delta-analysis (architect; Feature Mode Phase F1)"
---

# F1 Delta Analysis: Dispatcher INDETERMINATE Outcome — Validation Integrity Layer 1

## 1. Feature Summary

**Problem (operational forensics):** PostToolUse WASM validators run in fuel-bounded + epoch(wall-clock)-bounded
sandboxes. On large `.factory/` artifacts (decision-log ~6.4K lines, burst-log ~7.7K lines, lessons >3K lines)
they fail to complete: ~11,262 fuel-exhaustion timeouts, ~480 epoch timeouts, 167 `OutputTooLarge` host-fn
failures, and ~455 events where the entire validator suite wiped out together on a single edit. Because these
hooks are PostToolUse + advisory, writes already happened and a non-completing validator is treated as PASS —
**state mutates UNVALIDATED, silently.** The `regression-gate` failed to persist its own state file 22×
(OutputTooLarge). Current mitigations (fuel cap 10M→20M; CLAUDE.md prose size budgets; agent-side compaction
awareness) are symptom-treatment and LLM-compensation. The human has directed a MECHANISTIC fix where the
runtime and data structures enforce integrity; never the agent.

**Three-Layer Architecture (all three documented here; only Layer 1 delivered this cycle):**

- **Layer 1 (DELIVER NOW — S-25.01):** Make "couldn't validate" a first-class, fail-LOUD dispatcher outcome.
  Add an `INDETERMINATE` outcome class — named outcome distinct from PASS/FAIL — emitted when a plugin cannot
  complete due to fuel exhaustion, epoch timeout, or OutputTooLarge from a host function. For plugins with
  `failure_policy = "fail-closed"` (existing ADR-039 registry field, S-21.10 merged), INDETERMINATE causes: a
  hard visible signal in the event log; a durable unvalidated-mutation marker file written to `.factory/`; and
  blocking of the next state-advancing Agent dispatch until the artifact is re-validated. Existing
  `failure_policy = "fail-open"` (default for all current plugins) preserves today's advisory behavior — no
  regression. Pure dispatcher (`crates/factory-dispatcher/`) + event emission (`crates/sink-core/`) + new
  PreToolUse marker-check plugin + `hooks-registry.toml` per-plugin `failure_policy` assignments.

- **Layer 2 (BACKLOG — S-25.02 — deliberate feature-ordering per CLAUDE.md Canonical Principle §2):**
  Continuous, size-triggered sharding of append-only cycle artifacts (decision-log, burst-log, lessons) into
  capped shards plus an index; rotation triggered by a deterministic size rule (PreToolUse hook or append
  helper), with the shard cap DERIVED from the fuel budget (provable "no shard exceeds the validator-completion
  envelope" invariant). Removes the dark zone BY CONSTRUCTION.

- **Layer 3 (BACKLOG — S-25.03 — deliberate feature-ordering per CLAUDE.md Canonical Principle §2):**
  Validators read BOUNDED WINDOWS from shards rather than whole files (changelog-monotonicity → head rows;
  trajectory-tail → banner; closes-completeness → current burst; whole-corpus validators iterate the shard
  index). Fix regression-gate's own state-file write to bounded/rotated + fail-loud.

Layers 2 and 3 are explicitly REGISTERED BACKLOG under E-25 — they are features deliberately ordered after
Layer 1, NOT tech-debt-register items. They must not be silently deferred; they have specific story IDs.

---

## 2. Intent and Scope Classification

| Dimension | Value |
|-----------|-------|
| Intent | `feature` — new first-class outcome + durable marker mechanism |
| Feature type | `infrastructure` — dispatcher runtime + event emission + hooks-registry |
| Trivial scope | `false` — new ADR, new BCs, new VP catalog entries, new WASM plugin crate |
| Severity | n/a (not a bug-fix) |
| Pipeline route | Full F1 → F7 |

---

## 3. Foundation Alignment: ADR-039 and Existing `failure_policy` Infrastructure

**Critical context for F2/F3/F4:** The following groundwork is ALREADY IN PLACE and Layer 1 BUILDS ON IT:

| Component | Status | Reference |
|-----------|--------|-----------|
| `failure_policy` TOML field + `FailurePolicy` enum (FailClosed/FailOpen) | MERGED (S-21.10, PR #781) | ADR-039 §Decision 1+2; BC-1.01.016 |
| ADR-039: validator failure policy for resource exhaustion | RATIFIED v1.16 | decisions/ADR-039 |
| `PluginResult::Timeout { cause: TimeoutCause::Fuel }` and `TimeoutCause::Epoch` | In production | `crates/factory-dispatcher/src/invoke.rs` |
| Executor enforcement seams (failure_policy→current-dispatch block) | DRAFT S-21.19..S-21.24 | E-21 Wave 6–8 |

**Terminology reconciliation (must be maintained in F2 ADR-047):**

The feature request uses the terms "INDETERMINATE" and "on_indeterminate". The existing codebase uses:
- Registry field: `failure_policy = "fail-closed"` (TOML) / `FailurePolicy::FailClosed` (Rust)
- Existing outcomes: `PluginResult::Timeout { cause: Fuel | Epoch }` and `PluginResult::Crashed`

Layer 1 introduces **INDETERMINATE as a SEMANTIC LABEL** for the OUTCOME CLASS, not a new registry field.
The registry field remains `failure_policy`. F2 must codify in ADR-047:
- `failure_policy = "fail-closed"` is the registry declaration (existing ADR-039 field)
- INDETERMINATE is the named outcome emitted in the event log when a plugin cannot complete
- There is NO new `on_indeterminate` field — the existing `failure_policy` covers the routing semantics

**Dependency chain for full operational effect:**

```
S-21.10 (MERGED) — failure_policy schema
    ↓
S-21.19..S-21.24 (DRAFT) — enforcement: failure_policy=fail-closed blocks current dispatch
    ↓
S-25.01 (NEW, this cycle) — INDETERMINATE outcome label + durable marker + next-advance gate
```

S-25.01 DEPENDS ON S-21.10 (merged — dependency satisfied). S-25.01 is COMPLEMENTARY to S-21.19-S-21.24
(enforcement seams) but does NOT strictly depend on them — the marker mechanism and next-advance gate work
independently of whether the current-dispatch block is wired. Full operational effect (INDETERMINATE both
blocks the current dispatch AND writes the durable marker) requires BOTH S-25.01 AND S-21.24 to be merged.
F2 must document this ordering recommendation explicitly in ADR-047.

---

## 4. Subsystem and Capability Anchor

### Primary Subsystems

| Subsystem | Role in Layer 1 |
|-----------|----------------|
| **SS-01 Hook Dispatcher Core** | Primary — executor detects cannot-complete outcomes, emits INDETERMINATE event, writes marker trigger, routes per failure_policy |
| **SS-03 Event Emission (OTel-Aligned)** | New `plugin.indeterminate` event type in OTel-aligned schema |
| **SS-07 Hook Bash Layer** | hooks-registry.toml: add `failure_policy = "fail-closed"` to ~10 critical validators; new marker-check plugin entry |
| **SS-04 Plugin Ecosystem** | New native-WASM plugin: `validate-unvalidated-mutation-marker` (PreToolUse ^Agent$ gate) |

### New Capability

No existing CAP covers "INDETERMINATE outcome classification, durable mutation marker, and next-advance gate."

**Proposed: CAP-041 — Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate**

Narrative: "The dispatcher classifies plugin non-completion (fuel exhaustion, epoch timeout, host
OutputTooLarge) as a named INDETERMINATE outcome. For `failure_policy = "fail-closed"` plugins, INDETERMINATE
causes: (1) a `plugin.indeterminate` event in the OTel-aligned event log; (2) a durable marker file written
at `.factory/unvalidated-mutation.marker`; (3) the next state-advancing PreToolUse `^Agent$` dispatch is
blocked with a hard visible message until the marker is cleared by successful re-validation."

CAP-041 lives in SS-01 (outcome classification + marker write) with dependencies on SS-03 (event type) and
SS-04 (marker-check plugin). The D-NNN decision-log entry for E-25 registration must cite CAP-041.

---

## 5. Proposed ADR

**ADR-047**: "INDETERMINATE Outcome Model — First-Class Cannot-Complete Outcome, Durable Mutation Marker,
and Next-Advance Gate (Three-Layer Validation Integrity Architecture)"

**Status at F1**: Proposed / ratification pending (POLICY 22 applies — human ratification required before F4).

### Decisions ADR-047 Must Ratify

| Decision | Content |
|----------|---------|
| 1 | **Outcome trichotomy:** PASS / FAIL / INDETERMINATE. PASS = plugin completed and found nothing blocking. FAIL = plugin completed and found a blocking condition. INDETERMINATE = plugin could NOT complete (fuel exhaustion `TimeoutCause::Fuel`, epoch interrupt `TimeoutCause::Epoch`, host fn returned OutputTooLarge). These are mutually exclusive and exhaustive for the purposes of failure-policy routing. |
| 2 | **Registry field is `failure_policy` (existing, no new field).** `failure_policy = "fail-closed"` → INDETERMINATE causes hard block + marker + next-advance gate. `failure_policy = "fail-open"` (default) → INDETERMINATE is advisory only (no marker, no gate). There is NO new `on_indeterminate` field; ADR-039 §Decision 1+2 already covers the routing semantics. ADR-047 extends them with the marker + gate mechanism. |
| 3 | **Durable unvalidated-mutation marker:** Path `.factory/unvalidated-mutation.marker`. Written by the dispatcher (via existing write_file host capability or the new direct-write path noted below) when INDETERMINATE+fail-closed fires on a PostToolUse hook. Contains: timestamp, plugin name, artifact path, INDETERMINATE cause (fuel/epoch/output-too-large), and the trace_id for correlation. |
| 4 | **Next-advance gate:** A new PreToolUse `^Agent$` WASM plugin (`validate-unvalidated-mutation-marker`), `on_error = "block"`, `async = false`. Reads `.factory/unvalidated-mutation.marker`; if it exists, blocks the Agent dispatch with a human-readable message naming the plugin that failed, the artifact, and the re-validation command. The gate does NOT apply to read-only tool calls or Bash/Edit/Write. |
| 5 | **Marker clear protocol:** The marker is cleared (deleted) by the same dispatcher run that successfully completes the previously-indeterminate validator. Specifically: if a validator that previously produced INDETERMINATE now produces PASS, the dispatcher deletes the marker file for that validator entry. Alternatively: the marker can be manually cleared by an operator after manual verification (escape hatch). |
| 6 | **OutputTooLarge detection complexity.** `OutputTooLarge` (-3) is returned from HOST FUNCTIONS to the plugin. The plugin then decides its own exit code. The dispatcher cannot currently distinguish "plugin saw OutputTooLarge and correctly handled it" from "plugin saw OutputTooLarge and silently returned Ok {exit_code:0}." Layer 1 resolves this via a per-invocation flag in the Store data: when any host function returns OutputTooLarge to the plugin, the Store records `host_output_too_large_seen: true`; after the plugin completes, the executor checks this flag. If `host_output_too_large_seen` AND `plugin exit_code == 0` AND `failure_policy == fail-closed`, the outcome is INDETERMINATE (not PASS). This is the hardest technical piece of Layer 1; ADR-047 Decision 6 must specify the Store data extension and the detection algorithm. |
| 7 | **Backward-compat contract:** Existing PASS/FAIL semantics are UNCHANGED. Existing `on_error = "block"` / `on_error = "continue"` semantics are UNCHANGED. `failure_policy = "fail-open"` (current default for ALL ~52 plugins in production) is UNCHANGED in behavior — no marker, no gate, no regression. Only plugins that receive an explicit `failure_policy = "fail-closed"` assignment (done in the same PR as Layer 1) are affected. |
| 8 | **Layer 2 and Layer 3 ratified as future phases.** ADR-047 §Ratified Future Phases documents Layer 2 (shard rotation) and Layer 3 (bounded validator windows) as the intended completion of the validation-integrity architecture, with explicit story IDs S-25.02 and S-25.03 in E-25. These are REGISTERED BACKLOG, not deferred tech-debt. |

---

## 6. BC Proposals for Layer 1

All BCs are in subsystem SS-01 (Hook Dispatcher Core), proposed section BC-1.18 (new capability section for
INDETERMINATE outcome model). BC-1.17 is the existing `read_prefix` section.

| BC ID | Title | Key Postconditions | Status |
|-------|-------|-------------------|--------|
| **BC-1.18.001** | "When a `failure_policy = fail-closed` Validator Cannot Complete, Dispatcher Classifies Outcome as INDETERMINATE, Emits `plugin.indeterminate` Event, and Writes Unvalidated-Mutation Marker" | PC1: `TimeoutCause::Fuel`, `TimeoutCause::Epoch`, and host-fn OutputTooLarge-then-Ok all yield outcome class INDETERMINATE for fail-closed plugin. PC2: INDETERMINATE is distinct from PASS (exit_code:0, no block) and FAIL (exit_code:0, block requested by plugin). PC3: A `plugin.indeterminate` event with cause field is emitted to the event log. PC4: `.factory/unvalidated-mutation.marker` is written atomically with required fields. PC5: Existing PASS and FAIL outcome semantics for all other PluginResult variants are unchanged. | draft |
| **BC-1.18.002** | "Next State-Advancing `^Agent$` Dispatch Is Blocked While Unvalidated-Mutation Marker Exists" | PC1: When `.factory/unvalidated-mutation.marker` exists, the `validate-unvalidated-mutation-marker` PreToolUse hook fires on the next `^Agent$` tool call and returns `outcome=block`. PC2: Block message names the plugin that produced INDETERMINATE, the artifact path, and the re-validation command. PC3: Bash, Read, Edit, Write, and non-Agent tool calls are NOT blocked by the marker. | draft |
| **BC-1.18.003** | "Successful Re-Validation Clears Unvalidated-Mutation Marker" | PC1: When the previously-indeterminate validator completes successfully (PASS) on the same artifact, dispatcher deletes the marker. PC2: If marker is absent, the clear operation is a no-op (idempotent). PC3: Manual operator deletion via `rm .factory/unvalidated-mutation.marker` is the supported escape hatch; no special command needed. | draft |
| **BC-1.18.004** | "`failure_policy = fail-open` Plugins Yield Advisory-Only INDETERMINATE — No Marker, No Gate" | PC1: INDETERMINATE for a fail-open plugin emits only a `plugin.indeterminate` advisory event. PC2: No marker file is written. PC3: No next-advance gate is triggered. PC4: This is the behavior for ALL existing plugins until `failure_policy = "fail-closed"` is explicitly assigned. PC5: The default (`failure_policy` field absent) is `fail-open` (existing S-21.10 default). | draft |

For SS-03 (Event Emission), a new event type `plugin.indeterminate` must be added to the BC-3.08.001 event
catalog (ADR-039 and ADR-042 govern the event schema). This is a BC-3.08.001 amendment, not a new BC; it
belongs in S-25.01's scope and must be specified in F2.

---

## 7. VP Proposals for Layer 1

Current VP total: 101 (VP-INDEX v2.79). New VPs start at VP-102.

| VP ID | Title | Proof Method | Source BC | Target Module |
|-------|-------|-------------|-----------|---------------|
| **VP-102** | "Fuel-Exhaustion and Epoch-Timeout Yield INDETERMINATE Outcome for fail-closed Plugin" | Rust unit test (mock Store with fuel trap + epoch interrupt) | BC-1.18.001 PC1 | SS-01 `crates/factory-dispatcher/src/executor.rs` |
| **VP-103** | "Host OutputTooLarge Then Plugin Ok(exit:0) Yields INDETERMINATE for fail-closed Plugin" | Rust unit test (mock host fn returning -3 + Store flag inspection) | BC-1.18.001 PC1, Decision 6 | SS-01 `crates/factory-dispatcher/src/executor.rs` + `invoke.rs` |
| **VP-104** | "INDETERMINATE for fail-closed Plugin Writes Unvalidated-Mutation Marker with Required Fields" | Rust unit test (marker file write to tempdir, field verification) | BC-1.18.001 PC4 | SS-01 `crates/factory-dispatcher/src/executor.rs` |
| **VP-105** | "Next-Advance Gate Blocks Agent Dispatch While Marker Exists, Passes When Absent" | Rust unit test + bats integration (marker present/absent scenarios) | BC-1.18.002 PC1, PC3 | SS-04 `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs` |
| **VP-106** | "Successful Re-Validation Deletes Marker; fail-open INDETERMINATE Writes No Marker" | Rust unit test (clear on PASS + fail-open no-write) | BC-1.18.003 PC1, BC-1.18.004 PC2 | SS-01 `crates/factory-dispatcher/src/executor.rs` |

All VPs are initial status `draft` and will be authored during F2 (product-owner proposes postconditions;
architect finalizes VP harness skeletons).

---

## 8. Epic and Story Decomposition

### New Epic: E-25 "Validation Integrity and Large-Artifact Resilience"

**Status:** HOLDING EPIC — registered at F2/F3 spec burst. Three stories across three layers.

### Story Decomposition

| Story ID | Title | Status | Layer | Deps | Points (est.) |
|----------|-------|--------|-------|------|---------------|
| **S-25.01** | "Dispatcher INDETERMINATE Outcome Layer 1: Fail-Loud on Cannot-Complete — durable marker + next-advance gate" | ACTIVE (deliver this cycle) | Layer 1 | [S-21.10] | ~12 |
| **S-25.02** | "Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Artifacts" | **REGISTERED BACKLOG** (deliberate feature-ordering) | Layer 2 | [S-25.01] | ~15 (est.) |
| **S-25.03** | "Bounded Validator Windows Layer 3: Validators Read from Shards via Bounded Lookups" | **REGISTERED BACKLOG** (deliberate feature-ordering) | Layer 3 | [S-25.02] | ~12 (est.) |

**S-25.02 and S-25.03 are DELIBERATE BACKLOG per CLAUDE.md Canonical Principle §2 (feature ordering is the
only acceptable speed lever). They MUST appear in the STORY-INDEX as registered stories in E-25 at F3 time,
not as tech-debt-register entries.**

---

## 9. Test Approach (Red Gate Definition)

**Test strategy: REAL — no facade, no paper-test.**

### Rust Unit Tests (`crates/factory-dispatcher/` and `crates/hook-plugins/validate-unvalidated-mutation-marker/`)

Red Gate criterion: ALL of the following test function stubs must be FAILING (compile but panic with `todo!()`)
before any implementation. After implementation, ALL must pass.

```
// crates/factory-dispatcher/src/executor.rs (test module)
fn test_BC_1_18_001_fuel_exhaustion_yields_indeterminate_for_fail_closed_plugin()
fn test_BC_1_18_001_epoch_timeout_yields_indeterminate_for_fail_closed_plugin()
fn test_BC_1_18_001_output_too_large_then_ok_yields_indeterminate_for_fail_closed_plugin()
fn test_BC_1_18_001_indeterminate_is_distinct_from_pass_and_fail()
fn test_BC_1_18_001_indeterminate_writes_marker_to_factory_path()
fn test_BC_1_18_001_marker_contains_required_fields()
fn test_BC_1_18_003_successful_revalidation_deletes_marker()
fn test_BC_1_18_004_fail_open_indeterminate_writes_no_marker()
fn test_BC_1_18_004_fail_open_default_preserves_advisory_behavior()
fn test_backward_compat_pass_fail_on_error_semantics_unchanged()

// crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs
fn test_BC_1_18_002_blocks_agent_dispatch_when_marker_exists()
fn test_BC_1_18_002_passes_agent_dispatch_when_no_marker()
fn test_BC_1_18_002_block_message_names_plugin_and_artifact()
```

### Bats Integration Tests (`plugins/vsdd-factory/tests/`)

New bats test file: `validate_indeterminate_marker.bats`

Tests:
- `hooks-registry.toml`: validate-unvalidated-mutation-marker entry parses with `on_error = "block"`
- End-to-end with simulated INDETERMINATE: marker file appears, next Agent dispatch blocked
- Escape hatch: manual marker deletion unblocks the next dispatch
- Regression: existing bats suite (all ~52 plugins) passes unchanged

### Regression Gate

The full bats suite (`plugins/vsdd-factory/tests/run-all.sh`) and `cargo test --workspace --all-targets`
MUST stay green. Any existing test failure introduced by Layer 1 is a P0 blocker.

---

## 10. Impact Boundary

### NEW Files

| File | Type | Subsystem |
|------|------|-----------|
| `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs` | New WASM plugin crate | SS-04 |
| `crates/hook-plugins/validate-unvalidated-mutation-marker/Cargo.toml` | New crate manifest | SS-04 |
| `crates/factory-dispatcher/src/indeterminate_marker.rs` | New module: marker write/clear/check logic | SS-01 |
| `crates/factory-dispatcher/tests/indeterminate_tests.rs` (or inline test module) | Unit tests for VP-102–VP-106 | SS-01 |
| `plugins/vsdd-factory/tests/validate_indeterminate_marker.bats` | Bats integration tests | SS-07 |

### MODIFIED Files

| File | Change | Subsystem |
|------|--------|-----------|
| `crates/factory-dispatcher/src/executor.rs` | Detect INDETERMINATE outcome class; route per failure_policy; trigger marker write | SS-01 |
| `crates/factory-dispatcher/src/invoke.rs` | Add `host_output_too_large_seen` flag to Store data; set flag from host fn wrappers | SS-01 |
| `crates/factory-dispatcher/src/registry.rs` | No change to FailurePolicy enum (already present); possibly add accessor for marker-write path | SS-01 |
| `crates/factory-dispatcher/Cargo.toml` | Add new module `indeterminate_marker` | SS-01 |
| `crates/hook-plugins/Cargo.toml` (workspace manifest) | Add `validate-unvalidated-mutation-marker` workspace member | SS-04 |
| `Cargo.toml` (root workspace) | Add new crate to workspace members | SS-01 |
| `plugins/vsdd-factory/hooks-registry.toml` | Add `failure_policy = "fail-closed"` to ~10 critical validators (see §11); add new `validate-unvalidated-mutation-marker` PreToolUse `^Agent$` entry | SS-07 |

### DEPENDENT Files (Unchanged, Read Modified Modules)

| File | Dependency on Modified Module |
|------|------------------------------|
| `crates/factory-dispatcher/src/engine.rs` | Calls `execute_tier` in executor.rs |
| `crates/factory-dispatcher/src/main.rs` | Imports executor types |
| `crates/factory-dispatcher/src/aggregator.rs` | Aggregates PluginOutcome from executor |
| `crates/factory-dispatcher/src/routing.rs` | Routes based on TierExecutionSummary |

---

## 11. `failure_policy = "fail-closed"` Assignments for Critical Validators

The following plugins should receive `failure_policy = "fail-closed"` in hooks-registry.toml as part of S-25.01.
This is an operator-permission decision: F2 must confirm this list with the human.

| Plugin | Event | Rationale |
|--------|-------|-----------|
| `validate-burst-log-structure` | PostToolUse `^(Edit\|Write\|MultiEdit)$` | Cannot-complete on burst-log allows structurally malformed commits |
| `validate-wave-gate-prerequisite` | PreToolUse `^Agent$` | Cannot-complete allows state-advancing dispatch without wave-gate check |
| `regression-gate` | PostToolUse `^(Edit\|Write)$` | Cannot-complete allows regression regressions to go undetected; also failed 22× writing its own state |
| `convergence-tracker` | PostToolUse `^(Edit\|Write)$` | Cannot-complete allows convergence drift to go untracked |
| `validate-pr-merge-prerequisites` | PreToolUse `^Agent$` | Cannot-complete allows non-conforming PR merges |
| `validate-factory-path-staging` | PreToolUse `^(Edit\|Write\|MultiEdit)$` | Cannot-complete allows path-staging violations |
| `validate-cross-site-correspondence` | PostToolUse `^(Edit\|Write\|MultiEdit)$` | Cannot-complete allows cross-site value drift (ADR-042 §Decision 5 motivation) |

Advisory plugins (KEEP fail-open — these are informational, not integrity gates):
`capture-commit-activity`, `capture-pr-activity`, `session-start-telemetry`, `session-end-telemetry`,
`block-ai-attribution`, `check-state-health`.

**F2 must confirm this partitioning with the human before F3 story authoring.**

---

## 12. Backward-Compatibility and Regression Risk

### Backward-Compat Contract

| Existing Behavior | Change Under Layer 1 |
|------------------|---------------------|
| `on_error = "block"` semantics | UNCHANGED — crash+on_error=block still blocks current dispatch exactly as before |
| `on_error = "continue"` semantics | UNCHANGED |
| `failure_policy = "fail-open"` (default for all ~52 current plugins) | UNCHANGED — INDETERMINATE is advisory only, no marker, no gate |
| PASS/FAIL outcome semantics | UNCHANGED — INDETERMINATE is a THIRD class, not a reinterpretation |
| `PluginResult::Timeout { .. }` and `PluginResult::Crashed { .. }` | Still emitted as existing events; INDETERMINATE is a CLASSIFICATION LAYER on top, not a replacement variant |

The backward-compat invariant is: if no plugin has `failure_policy = "fail-closed"` assigned, Layer 1 adds
no observable behavior change beyond additional advisory `plugin.indeterminate` events in the event log (for
existing fuel/epoch timeouts, which are currently logged as `plugin.timeout` events anyway).

### Regression Risk Assessment

| Module | Risk Level | Rationale |
|--------|-----------|-----------|
| `executor.rs` | HIGH | Hot path for every dispatch; INDETERMINATE detection touches the outcome evaluation branch |
| `invoke.rs` | MEDIUM | Store data extension adds a flag; risk scoped to the flag-setting path in host fn wrappers |
| `validate-unvalidated-mutation-marker` | LOW | New plugin; no existing code modified |
| `hooks-registry.toml` `failure_policy` assignments | MEDIUM | Registry parsing must stay green; existing entries unchanged |
| Event emission (SS-03) | MEDIUM | New event type `plugin.indeterminate` must not break existing OTel schema consumers |

### Regression Test Strategy

1. `cargo test --workspace --all-targets` — full cargo suite must stay green (regression gate)
2. `plugins/vsdd-factory/tests/run-all.sh` — full bats suite must stay green
3. Per-VP unit tests (VP-102..VP-106) — all must pass after implementation
4. Specific regression guard: `test_BC_1_18_004_fail_open_default_preserves_advisory_behavior` explicitly
   verifies that the S-21.10 default (`FailurePolicy::FailOpen`) produces ZERO behavioral change

---

## 13. Existing Artifacts in Regression Risk Zone

The following existing tests cover modules being modified and must remain green:

| Existing Test | Covers | Risk |
|--------------|--------|------|
| `mod s21_10_bc_1_01_016_failure_policy` in `registry.rs` | `FailurePolicy` schema parsing | MEDIUM — executor changes must not break registry parsing tests |
| `test_BC_1_01_016_phase1_failure_policy_does_not_affect_on_error_accessor` | Phase 1 axes-independence | HIGH — Layer 1 must preserve axes-independence (ADR-039 §Decision 1 orthogonality) |
| `plugin_fail_closed` unit tests in `executor.rs` | Crash/timeout + on_error routing | HIGH — existing block semantics unchanged |
| All bats tests exercising `validate-cross-site-correspondence`, `regression-gate`, `convergence-tracker` | Critical validators | MEDIUM — behavior unchanged for fail-open config |

---

## 14. Release Path

**Layer 1 reaches operator level only when a new rc releases the updated dispatcher binary.**

The dispatcher binary at `~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/` is the operator-level
consumed copy. Develop-branch changes do NOT affect the cached plugin. Layer 1 changes become effective for
all users only after:

1. Layer 1 merged to develop (this cycle)
2. A new release candidate (e.g., v1.0.0-rc.25) is cut with the updated binary bundle
3. The marketplace cache at `~/.claude/plugins/cache/` is populated with the new rc

The CLAUDE.md release note for rc.25 must call out: "Layer 1 validation integrity: INDETERMINATE outcome
model active; critical validators now emit unvalidated-mutation marker on cannot-complete; next state-advancing
dispatch blocked until re-validated."

---

## 15. Affected Files Summary (`affected-files.txt` content)

```
NEW   crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs
NEW   crates/hook-plugins/validate-unvalidated-mutation-marker/Cargo.toml
NEW   crates/factory-dispatcher/src/indeterminate_marker.rs
NEW   plugins/vsdd-factory/tests/validate_indeterminate_marker.bats
MODIFIED   crates/factory-dispatcher/src/executor.rs
MODIFIED   crates/factory-dispatcher/src/invoke.rs
MODIFIED   crates/factory-dispatcher/Cargo.toml
MODIFIED   Cargo.toml
MODIFIED   plugins/vsdd-factory/hooks-registry.toml
DEPENDENT  crates/factory-dispatcher/src/engine.rs
DEPENDENT  crates/factory-dispatcher/src/main.rs
DEPENDENT  crates/factory-dispatcher/src/aggregator.rs
DEPENDENT  crates/factory-dispatcher/src/routing.rs
```

---

## 16. Impact Assessment Table

| Dimension | Assessment |
|-----------|-----------|
| PRD / BCs | New BCs BC-1.18.001–004 (SS-01); amendment to BC-3.08.001 (SS-03, new event type) |
| Architecture | ADR-047 (new); CAP-041 (new capability); no subsystem boundary changes |
| UX | None — infrastructure only |
| Stories | S-25.01 ACTIVE; S-25.02, S-25.03 REGISTERED BACKLOG |
| Tests | 13+ new Rust unit tests (VP-102–VP-106); 1 new bats test file; full regression suite |
| Verification | 5 new VPs (VP-102–VP-106); existing VP-INDEX total 101 → 106 |
| Existing BCs | No existing BC postconditions change; BC-1.01.016 phase1 no-enforcement boundary advances to enforcement (this is the intended Phase 2 completion, not a spec violation) |
| Regression stories in risk zone | S-21.19–S-21.24 (in-flight enforcement seams) must stay compatible with Layer 1 INDETERMINATE classification; F4 must verify no double-block on the same INDETERMINATE event |

---

## 17. Files NOT Changed (Regression Baseline)

The following are in scope for regression testing but must NOT be modified by Layer 1:

- `crates/hook-sdk/src/` (no SDK ABI change for Layer 1)
- `crates/sink-core/`, `crates/sink-file/` (event emission structure unchanged; new event type is additive)
- `crates/factory-lock/`, `crates/factory-lock-parse/` (unaffected)
- All existing `crates/hook-plugins/` other than new plugin (unaffected)
- `plugins/vsdd-factory/agents/`, `plugins/vsdd-factory/skills/` (unaffected)
- `.factory/specs/` (specs authored in F2 only; no F1 file mutations)

---

## 18. Open Questions for Human Resolution (F2 Gate)

The following questions MUST be resolved by the human before F3 story authoring. Present at F1 approval gate.

1. **Failure_policy=fail-closed assignment list (§11):** Is the proposed list of ~7 critical validators
   correct? Are there additional validators that should be fail-closed? Any that should stay fail-open?

2. **Marker file path:** Is `.factory/unvalidated-mutation.marker` the right path? Should it be per-plugin
   (e.g., `.factory/unvalidated-mutation-<plugin-name>.marker`) to allow partial re-validation?

3. **OutputTooLarge detection (ADR-047 Decision 6):** The Store-data flag approach adds a field to the
   wasmtime Store. Is this acceptable complexity for Layer 1, or should OutputTooLarge detection be deferred
   to Layer 3 (which reforms the validator read patterns anyway)? Deferring would mean Layer 1 only covers
   Fuel/Epoch INDETERMINATE, not OutputTooLarge.

4. **Dependency on S-21.19–S-21.24:** S-25.01 can be delivered before the enforcement seams are merged.
   Is that acceptable (delivering the marker mechanism before the current-dispatch block is wired), or should
   S-25.01 explicitly depend on S-21.24 to ensure full operational effect on delivery?

5. **ADR-039 amendment vs. ADR-047:** Given the deep dependency on ADR-039's framework, should Layer 1 be
   an amendment to ADR-039 (AMD-005) rather than a new ADR-047? This avoids splitting the validator-failure
   architecture across two ADRs. Recommendation (architect): new ADR-047 to keep ADR-039 focused on
   resource-exhaustion calibration; ADR-047 extends it with the outcome model. Human confirmation required.

---

## 19. Self-Audit Checklist

- [x] No "MVP/for now/good enough" rationalization — all three layers are registered, Layer 1 is complete
- [x] No tech-debt-register entries for Layers 2 and 3 — they are REGISTERED BACKLOG stories S-25.02/S-25.03
- [x] No "pending architect review" placeholders — all questions are surfaced to human as Open Questions
- [x] Feature type: infrastructure (correct route — full F1-F7, no quick-dev shortcut)
- [x] Backward-compat contract is explicit and complete
- [x] Regression test strategy names the specific failing tests that are the Red Gate
- [x] Release path calls out the rc requirement
- [x] Subsystem/capability/ADR/BC/VP IDs all proposed and justified
- [x] All eight F1-required determinations answered (§4 subsystem+CAP, §5 ADR, §6 BCs, §7 VPs, §8 epic+stories, §9 tests+Red Gate, §10 impact boundary, §12 backward-compat, §14 release note)
