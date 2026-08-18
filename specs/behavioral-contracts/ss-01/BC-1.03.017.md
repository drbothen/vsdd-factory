---
document_type: behavioral-contract
level: L3
version: "v1.3"
status: draft
producer: product-owner
timestamp: 2026-08-06T00:00:00Z
last_amended: "2026-08-17 (v1.3) — adversary pass-3 remediation (product-owner): two F-S2111-P3 findings remediated. F-S2111-P3-001: reconciled 50M boundary to inclusive floor (>= 50_000_000 ACCEPT, < 50_000_000 REJECT) — sibling sweep across PC8, PC9, Invariant 2, Invariant 7, Architecture Anchors, VP-TBD, and Canonical Test Vectors; POSITIVE-CONTROL fixture updated from fuel_cap=10_000_000 to fuel_cap=20_000_000 (factory default per ADR-042 §Decision 2, clearly below floor and realistic); added boundary-pass test vector asserting fuel_cap=50_000_000 PASSES. F-S2111-P3-005: PC11 added — hard migration-window completeness CI gate (test_no_on_error_block_without_fail_closed_when_3arg_executor) enforcing that if the extended 3-arg plugin_fail_closed signature is present in executor.rs, every on_error=block targeted plugin MUST carry failure_policy=fail-closed (CWE-636 static gate, checkable at any single commit, closes gap left by Invariant 7 ordering rule). PC11 test vector added. BC-1.03.017 v1.3."
phase: brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/research/wasm-fuel-exhaustion-detection.md
input-hash: "d51127e"
traces_to: .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill-E21-W6
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.03.017: factory-dispatcher::executor::failure_policy enforcement — exhaustion-outcome dispatch (fail-closed→block; fail-open→advisory), on_error axes independence, crash-versus-exhaustion distinct paths, and Phase-3-before-Phase-4 structural half-state gate (ADR-039 §Decision 3+6 Phase 4 enforcement leg)

## Description

The factory-dispatcher executor MUST extend the `plugin_fail_closed` function (or introduce a
replacement) in `crates/factory-dispatcher/src/executor.rs` to consult the `failure_policy`
field (introduced by BC-1.01.016 / S-21.10) when handling resource-exhaustion outcomes
(`PluginResult::Timeout { cause: TimeoutCause::Fuel }` and `TimeoutCause::Epoch`).

**The enforcement defect being closed:** In the current implementation, `plugin_fail_closed`
returns `false` when `on_error == OnError::Continue`, regardless of `TimeoutCause`. For the
approximately 38 validator plugins registered with `on_error = "continue"`, fuel exhaustion
therefore produces the same allow-decision as a clean pass. This is **CWE-636 "Not Failing
Securely (Failing Open)"** (primary) and **CWE-390 "Detection of Error Condition Without
Action"** (secondary), as classified by `.factory/research/wasm-fuel-exhaustion-detection.md`.
The production-host precedent (Envoy `FailurePolicy` default `FAIL_CLOSED`; Istio `failStrategy`
default `FAIL_CLOSE`) confirms that authorization-class plugins should block on exhaustion.

**The self-lock hazard is already live today.** During ADR-039 authoring (2026-08-06), writes
to `ARCH-INDEX.md` triggered `fail-closed: plugin timed out` blocks from
`validate-factory-path-root`, `validate-input-hash`, and `validate-template-compliance`. Any
premature enforcement flip without calibrated per-plugin fuel caps would hard-block all
`.factory/` writes. The Phase-3-before-Phase-4 ordering constraint in this BC (PC8, PC9) is
therefore a correctness requirement, not a best-practice recommendation.

**Enforcement semantics:** When `failure_policy = FailClosed`, a fuel- or epoch-exhausted
plugin MUST produce a block signal (exit code 2) regardless of the value of `on_error`. When
`failure_policy = FailOpen`, a fuel- or epoch-exhausted plugin MUST produce an advisory event
(exit code 0). The `on_error` axis remains governing for crash outcomes only — it does NOT
override `failure_policy` for exhaustion outcomes.

**The Envoy #38801 lesson is binding:** The test suite MUST assert observed outcomes (block or
advisory at the dispatcher level), NOT merely that the `failure_policy` field is configured.
Envoy documented `FAIL_CLOSED` diverged from observed behavior because the test suite asserted
configuration intent rather than behavioral outcomes.

## Preconditions

1. S-21.10 has shipped: `FailurePolicy` enum and `RegistryEntry.failure_policy` field are present
   in `crates/factory-dispatcher/src/registry.rs` (BC-1.01.016 postconditions hold).
2. Per-plugin fuel-cap calibration (devops-engineer role) has been executed for each of the
   six targeted validator-class plugins: `validate-factory-path-root`, `validate-input-hash`,
   `validate-template-compliance`, `validate-wave-gate-prerequisite`,
   `validate-pr-merge-prerequisites`, `validate-cross-site-correspondence`. Calibration corpus
   MUST include: `lessons.md` at ≥3000 lines; `STATE.md` at current live size;
   `decision-log.md` at current live size; and the 576,396-byte production-scale fixture at
   `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md`.
3. For each targeted plugin, `fuel_cap` has been set to
   `max(measured_p99 × 1.5, 50_000_000)` from precondition 2 measurements.
4. The calibration results (plugin name, p99 measured, chosen `fuel_cap`) are recorded in the
   PR description or a calibration log artifact before Phase 4 annotations land.

## Postconditions

1. **PC1 — Exhaustion + fail-closed → BLOCK (exit 2):**
   `PluginResult::Timeout { cause: TimeoutCause::Fuel }` with
   `failure_policy = FailurePolicy::FailClosed` causes the executor decision function to return
   `true` (block intent); the dispatcher exit code is 2. A real dispatch with a plugin
   configured `failure_policy = "fail-closed"` and `fuel_cap = 100` (deliberately too small)
   on a payload that exhausts the budget MUST produce exit code 2 on the observed dispatcher
   output.

2. **PC2 — Exhaustion + fail-open → advisory (exit 0):**
   `PluginResult::Timeout { cause: TimeoutCause::Fuel }` with
   `failure_policy = FailurePolicy::FailOpen` causes the executor decision function to return
   `false`; the dispatcher exit code is 0; an advisory event is emitted (not a block). This
   verifies the `fail-open` path is preserved for plugins that legitimately require it.

3. **PC3 — `on_error` independence: exhaustion + fail-closed blocks regardless of `on_error`:**
   A plugin with `on_error = OnError::Continue` AND `failure_policy = FailurePolicy::FailClosed`
   that exhausts its fuel budget MUST produce a block (exit 2). `on_error` governs crash
   outcomes only; it does NOT override `failure_policy` for exhaustion outcomes. This directly
   validates the axes-independence design and supersedes the prior
   `fail_closed_timeout_with_on_error_continue_is_open` assertion for the `FailClosed`
   configuration case.

4. **PC4 — Crash governed exclusively by `on_error` (crash ≠ exhaustion, path A):**
   `PluginResult::Crashed` (crash) with `on_error = OnError::Block` and
   `failure_policy = FailurePolicy::FailOpen` MUST produce a block (exit 2) via the `on_error`
   path. The block is caused by the crash, not by exhaustion policy. `failure_policy` is not
   consulted for crash outcomes.

5. **PC5 — `on_error = block` does NOT gate exhaustion when `failure_policy = fail-open`
   (crash ≠ exhaustion, path B):**
   `PluginResult::Timeout { cause: TimeoutCause::Fuel }` with `on_error = OnError::Block` and
   `failure_policy = FailurePolicy::FailOpen` MUST produce exit 0. Exhaustion is a
   resource-policy outcome; `on_error = block` does not apply to exhaustion when
   `failure_policy = FailOpen`.

6. **PC6 — Epoch exhaustion treated identically to fuel exhaustion:**
   `PluginResult::Timeout { cause: TimeoutCause::Epoch }` with
   `failure_policy = FailurePolicy::FailClosed` MUST produce a block (exit 2). Both
   `TimeoutCause::Fuel` and `TimeoutCause::Epoch` are resource-exhaustion outcomes per
   ADR-039 §Decision 1; both trigger the `failure_policy` enforcement path.

7. **PC7 — `fail_closed_timeout_with_on_error_continue_is_open` revised, not deleted
   (TD-VSDD-059 compliance):**
   The existing test `fail_closed_timeout_with_on_error_continue_is_open` in the executor
   module MUST be revised (not deleted) to assert the new invariant for the `fail-open`
   configuration case: `Timeout { cause: Fuel } + on_error=Continue + failure_policy=FailOpen
   → NOT block`. The function name MUST be retained or a close derivative used. The revised
   test MUST appear in the PR diff. Deletion without an equivalent replacement is a TD-VSDD-059
   paper-fix violation.

8. **PC8 — Structural half-state gate: no `failure_policy = "fail-closed"` with uncalibrated
   `fuel_cap`, no `on_error = "block"` targeted plugin left at `failure_policy = fail-open`
   post-executor-flip, and both positive and negative gate controls present (standing
   regression/invariant gate):**
   A Cargo integration test (`test_no_fail_closed_plugin_with_uncalibrated_cap`) MUST assert
   that no `[[hook]]` entry in `hooks-registry.toml` carries both
   `failure_policy = "fail-closed"` AND `fuel_cap < 50_000_000` (the calibration floor per
   ADR-039 §Decision 4 is 50_000_000 inclusive — `fuel_cap >= 50_000_000` is VALID; the
   factory default of 20_000_000 per ADR-042 §Decision 2 is below this floor and therefore
   insufficient for fail-closed annotation). This gate
   is a **standing regression/invariant gate**: it is GREEN when the registry contains zero
   fail-closed entries (vacuously satisfied with the empty set), GREEN at final state (all
   targeted plugins annotated with calibrated caps), and RED only when a bad half-state edit
   introduces a fail-closed entry without a sufficient cap. The gate MUST include both:
   (a) **POSITIVE-CONTROL fixture** (a hard-coded fail-closed entry with `fuel_cap = 20_000_000`
   — the factory default per ADR-042 §Decision 2, strictly below the 50_000_000 inclusive floor
   and therefore a realistic failing case — injected directly in the test body) that asserts the
   gate fires RED on that fixture — proving the gate is non-vacuous and not susceptible to
   false-green behavior when the live registry contains zero fail-closed entries; and
   (b) **NEGATIVE-CONTROL fixture** (a hard-coded fail-closed entry with `fuel_cap = 75_000_000`,
   i.e., above the 50_000_000 floor, injected directly in the test body) that asserts the gate
   does NOT fire on that fixture (result: PASS / no error) — proving the gate correctly
   distinguishes valid calibrated fail-closed entries from bad half-state entries (POLICY 15:
   every gate outcome requires a control; the positive-control-only version leaves the
   "gate accepts valid entry" path unverified).
   The genuine red-first TDD gate is PC9
   (AC-009: `test_all_six_validator_class_plugins_are_fail_closed`), which is RED before Phase
   4 annotations land and GREEN only after all targeted plugins carry fail-closed with calibrated
   caps.
   **Symmetric half-state prohibition (F-S2111-P2-001):** Once the extended 3-arg
   `plugin_fail_closed` function is present in any CI-passing commit, no targeted plugin
   currently carrying `on_error = "block"` (`validate-factory-path-root`,
   `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
   `validate-wave-gate-prerequisite`) MAY remain at `failure_policy = fail-open`. Under the
   2-arg function these five plugins block on exhaustion via the `on_error = "block"` path;
   under the extended function exhaustion is governed exclusively by `failure_policy`, so
   failure_policy=fail-open causes them to FAIL OPEN — a CWE-636 regression. The
   decision-function change and the fail-closed annotations for these five plugins MUST be
   co-committed (same commit) or ordered annotate-first-then-flip. A plugin entry MUST NOT
   carry `failure_policy = "fail-closed"` without simultaneously carrying
   `fuel_cap >= 50_000_000` (exactly 50_000_000 is the inclusive floor and a VALID calibrated
   value per ADR-039 §Decision 4).

9. **PC9 — All targeted validator-class plugins carry `failure_policy = "fail-closed"` with
   calibrated `fuel_cap >= 50_000_000` in final state:**
   After all Phase 4 calibration-and-annotation commits land, `hooks-registry.toml` MUST
   contain `failure_policy = "fail-closed"` AND `fuel_cap >= 50_000_000` for all plugins in
   the **post-amendment targeted set**. The default targeted set is all six of:
   `validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
   `validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`,
   `validate-cross-site-correspondence`. If EC-004 fires for any plugin in this set
   (calibration reveals an impractical cap requirement), S-21.11 is descoped to the flippable
   subset via orchestrator-approved spec amendment; PC9's asserted set is reduced to the
   flippable plugins only; the deferred plugin routes to named follow-up story S-21.13. PC9
   asserts the post-amendment set, not necessarily all six — a partial-set completion is valid
   if EC-004 applied.
   **Critical caveat for `on_error = "block"` plugins (F-S2111-P2-003):** When EC-004 fires
   for any of the five `on_error = "block"` plugins (`validate-factory-path-root`,
   `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
   `validate-wave-gate-prerequisite`), descoping that plugin from PC9 is NOT
   behavior-neutral — under the extended 3-arg function the plugin will regress from
   block-on-exhaustion to fail-open on exhaustion. Before the Phase 4 commit merges, one of
   the two EC-004 remediation paths MUST be followed for every deferred `on_error="block"`
   plugin: (a) retain a fallback or gate that preserves block-on-exhaustion for that plugin
   until S-21.13 lands, or (b) record the transient fail-open window explicitly and gate
   S-21.13 as a HARD BLOCKER. See EC-004 for the full remediation specification.
   Advisory-only and observability plugins MUST NOT receive
   `failure_policy = "fail-closed"`.

10. **PC10 — `fail_closed_timeout_with_on_error_block` revised, not deleted (TD-VSDD-059
    complement to PC7):**
    The existing test `fail_closed_timeout_with_on_error_block` in the executor module
    (which under the 2-arg `plugin_fail_closed` currently asserts
    `Timeout { cause: Fuel|Epoch } + on_error=Block → block==true`) MUST be DELIBERATELY
    REVISED (not deleted) to assert the new axes-independent semantics. The revised test
    MUST cover both sub-cases:
    (a) `Timeout { cause: Fuel } + on_error=Block + failure_policy=FailOpen → NOT block`
    (exit 0): exhaustion is governed by `failure_policy`; `on_error=Block` does NOT apply
    to exhaustion outcomes when `failure_policy=FailOpen` (PC5 / EC-009).
    (b) `Timeout { cause: Fuel } + on_error=Block + failure_policy=FailClosed → block`
    (exit 2): exhaustion governed by `failure_policy=FailClosed`; both axes agree on block,
    but the block is caused by `failure_policy`, not `on_error` (PC1 / PC6 class).
    The function name MUST be retained or a close derivative used (e.g.,
    `fail_closed_timeout_with_on_error_block_axes_independent`). The revised test MUST
    appear in the PR diff. Deletion without an equivalent replacement is a TD-VSDD-059
    paper-fix violation. This PC is parallel to PC7's treatment of
    `fail_closed_timeout_with_on_error_continue_is_open` — both sibling tests require
    revision to accurately reflect the extended decision function's axes-independent
    semantics.

11. **PC11 — Hard migration-window completeness gate: if the extended 3-arg `plugin_fail_closed`
    is in effect, every `on_error="block"` targeted plugin MUST carry `failure_policy="fail-closed"`
    (CWE-636 static gate, checkable at any single commit):**
    A Cargo integration test (`test_no_on_error_block_without_fail_closed_when_3arg_executor`)
    MUST assert: if `crates/factory-dispatcher/src/executor.rs` contains the extended 3-arg
    `plugin_fail_closed` signature (detected by pattern — e.g., `fn plugin_fail_closed(` with
    a `failure_policy` parameter), then every `[[hook]]` entry in `hooks-registry.toml` whose
    `name` is one of the five targeted `on_error = "block"` plugins (`validate-factory-path-root`,
    `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
    `validate-wave-gate-prerequisite`) MUST carry an explicit `failure_policy = "fail-closed"`.
    Any absence of this annotation while the 3-arg function is present MUST cause the test to
    FAIL (CI blocks merge). This gate makes the CWE-636 migration-window regression that
    Invariant 7 prohibits by ordering rule **structurally impossible to merge**: the gate
    evaluates both conditions (executor signature AND registry annotation) on the same commit
    tree, with no ordering dependency. The gate is GREEN when the 3-arg function is absent
    (Phase 1–2 state), and GREEN only when both the 3-arg function is present AND all five
    plugins carry `failure_policy = "fail-closed"` (Phase 4 complete state). It fires RED on
    the bad intermediate state (3-arg present + any on_error=block plugin at fail-open), which
    is precisely the CWE-636 regression window that no prior PC gated mechanically.
    **Relationship to Invariant 7:** This PC makes Invariant 7's ordering rule machine-checkable.
    Invariant 7 remains in force as the human-readable policy statement; PC11 is its
    mechanically-enforced complement. PC11 does NOT replace PC8 or PC9 — those gates address
    different failure modes (uncalibrated caps and final-state completeness respectively).

## Invariants

1. **Axes-independence invariant (ADR-039 §Decision 1):** `failure_policy` governs
   resource-exhaustion outcomes (`TimeoutCause::Fuel`, `TimeoutCause::Epoch`); `on_error`
   governs crash/host-error outcomes. Neither axis overrides the other. A plugin may
   simultaneously carry `on_error = "continue"` (crash = advisory) and
   `failure_policy = "fail-closed"` (exhaustion = block) — this is the intended steady-state
   for most validator-class plugins.

2. **No-half-state invariant (ADR-039 §Decision 3):** No `failure_policy = "fail-closed"`
   annotation MAY coexist with `fuel_cap < 50_000_000` at any CI-passing commit. The
   50_000_000 value is the calibration floor per ADR-039 §Decision 4 (`max(p99×1.5, 50M)`);
   exactly 50_000_000 is the inclusive minimum VALID value (`fuel_cap >= 50_000_000` is
   required; `fuel_cap < 50_000_000` is prohibited). The factory default of 20_000_000
   (ADR-042 §Decision 2) is below this floor and insufficient for fail-closed annotation.
   The Phase-3-before-Phase-4 ordering constraint is structurally enforced by the
   `test_no_fail_closed_plugin_with_uncalibrated_cap` CI gate test (PC8).
   Fail-closed without a sufficient budget is equivalent to blocking unconditionally — the
   intended function is to block writes that fail validation, not to block all writes.

3. **CWE-636 closure invariant:** Once Phase 4 is complete, no authorization-class WASM
   validator plugin silently approves a write when it exhausts its fuel budget. The fail-open
   enforcement defect documented in F-S2107-P7-010 (HIGH) is closed for the six targeted
   plugins.

4. **Advisory-only plugins remain fail-open:** Observability hooks, telemetry collectors, and
   convergence-tracking plugins MUST NOT receive `failure_policy = "fail-closed"`.
   Classification as validator-class vs advisory-class is per plugin; only the six explicitly
   named plugins receive the flip.

5. **D-442(e) line-count workaround remains in force until calibration confirms sufficiency:**
   The ≤3500 soft / ≤4000 hard `lessons.md` line-count workaround from D-442(e) MUST remain
   in force until per-plugin calibration (PC2) confirms that all validators reading `lessons.md`
   have a `fuel_cap` sufficient for the D-442(e) hard limit (4000 lines). If calibration shows
   insufficiency, the workaround remains and the finding surfaces to the orchestrator rather
   than silently relaxing D-442(e).

6. **Behavioral tests, not configuration tests (Envoy #38801 lesson):** Every enforcement
   postcondition (PC1 through PC6) MUST be verified by tests that drive the actual dispatch
   path with a budget-exhausting input and assert the observed outcome (block or advisory),
   NOT merely that the `failure_policy` field is configured to the expected value.

7. **Symmetric half-state prohibition — `on_error="block"` targeted plugins must not regress
   to fail-open on exhaustion (migration-ordering atomicity, F-S2111-P2-001):** The five
   targeted plugins currently carrying `on_error = "block"` in `hooks-registry.toml`
   (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
   `validate-pr-merge-prerequisites`, `validate-wave-gate-prerequisite`) presently block on
   exhaustion via the `on_error` path of the 2-arg `plugin_fail_closed`. Once the extended
   3-arg function is in effect, exhaustion is governed exclusively by `failure_policy`. Any
   CI-passing commit that contains the extended function while any of these five plugins
   remains at `failure_policy = fail-open` (absent-field default) is a **CWE-636 regression**
   — those plugins will fail OPEN on exhaustion rather than block. This invariant prohibits
   that half-state. The decision-function change and the `failure_policy = "fail-closed"`
   annotations for the five `on_error="block"` plugins MUST be co-committed (same commit) or
   ordered annotate-before-flip (annotations committed first, executor flip committed second).
   The `validate-cross-site-correspondence` plugin (`on_error = "continue"`) does NOT
   contribute this regression risk — it already failed open on exhaustion under the 2-arg
   function and continues to do so until explicitly annotated. The PC8 gate test enforces
   the calibration constraint (no fail-closed without `fuel_cap >= 50M`). PC11 makes this
   ordering constraint a static CI gate (checkable at any single commit; the bad intermediate
   state — 3-arg executor present AND any on_error=block targeted plugin at fail-open — causes
   PC11's test to FAIL, making it mechanically impossible to merge). This Invariant 7 remains
   the human-readable policy statement governing the atomicity/ordering constraint.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin with `failure_policy = "fail-closed"` completes successfully (no exhaustion) | Exit 0; both `on_error` and `failure_policy` are irrelevant for clean-pass outcomes |
| EC-002 | Plugin with `failure_policy = "fail-closed"` + `on_error = "continue"` crashes (not exhaustion) | Crash governed via `on_error = continue`; exit 0 (crash is advisory); `failure_policy` is not consulted for crash outcomes |
| EC-003 | Plugin with `failure_policy = "fail-closed"` + `on_error = "block"` crashes (not exhaustion) | Crash governed via `on_error = block`; exit 2; `failure_policy` not consulted for crash outcomes |
| EC-004 | Calibration reveals a targeted plugin needs `fuel_cap > 500M` for p99×1.5 | MUST surface to orchestrator; do not annotate with an insufficient cap; plugin deferred from Phase 4 flip. Resolution to prevent PC9 deadlock: S-21.11 is descoped to the flippable subset via orchestrator-approved spec amendment; PC9's enumerated set is reduced to the flippable plugins only; the deferred plugin routes to named follow-up story **S-21.13** (validate-cross-site-correspondence targeted-row lookup eliminating the O(n) fuel ceiling; depends_on [S-21.10, S-21.11]; ordering tension if validate-cross-site-correspondence is the deferred plugin — annotate-before-flip constraint per Invariant 7 still applies to the on_error=block subset). A hard-assertion of all six plugins in PC9 when EC-004 fires for any of them would cause convergence deadlock — the conditional descoping is the only resolution path. **Deferral is NOT behavior-neutral for `on_error="block"` plugins (F-S2111-P2-003):** if any of the five `on_error="block"` targeted plugins is deferred (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`, `validate-wave-gate-prerequisite`), that plugin will transition from block-on-exhaustion (current behavior via the `on_error` path under the 2-arg function) to fail-open on exhaustion under the extended 3-arg function — a CWE-636 regression. A deferred `on_error="block"` plugin MUST NOT be left at failure_policy=fail-open post-merge of the executor flip; one of these two remediation paths MUST be followed before that commit lands: **(a)** retain a fallback or gate that keeps exhaustion-blocking behavior for that plugin active until S-21.13 (or the named follow-up) lands; or **(b)** record the transient fail-open window explicitly in the PR description and gate S-21.13 as a HARD BLOCKER (no merge of any commit that depends on fail-closed enforcement for that plugin until the annotation lands). The `validate-cross-site-correspondence` plugin (`on_error="continue"`) does NOT carry this regression risk — it already failed open on exhaustion under the 2-arg function and continues to do so under the 3-arg function until annotated. |
| EC-005 | `lessons.md` validator exhausts on a >4000-line `lessons.md` after Phase 4 flip | Signals calibration was insufficient (PC2+PC9 not met); surface to orchestrator; D-442(e) remains in force |
| EC-006 | `TimeoutCause::Epoch` with `failure_policy = "fail-closed"` | BLOCK (exit 2); epoch deadline is a resource-exhaustion outcome; same enforcement path as `TimeoutCause::Fuel` |
| EC-007 | New validator-class plugin added after S-21.11 merges (without `failure_policy = "fail-closed"`) | Defaults to `fail-open` per BC-1.01.016 backward-compat; PC8 gate only fires for annotated-but-uncalibrated entries; classification of new plugins is a future-story concern |
| EC-008 | Plugin with `on_error = "block"` + `failure_policy = "fail-closed"` exhausts fuel | BLOCK via `failure_policy` path; `on_error = block` is redundant for exhaustion when `failure_policy = "fail-closed"` but both agree on the block outcome |
| EC-009 | Plugin with `on_error = "block"` + `failure_policy = "fail-open"` exhausts fuel | Exit 0; exhaustion governed by `failure_policy = fail-open`; `on_error = block` does not apply to exhaustion outcomes (PC5) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `Timeout { cause: Fuel }` + `FailClosed` | Decision function returns `true`; exit 2 | happy-path (fail-closed enforcement, PC1) |
| `Timeout { cause: Fuel }` + `FailOpen` | Decision function returns `false`; exit 0 | happy-path (fail-open pass-through, PC2) |
| `Timeout { cause: Fuel }` + `on_error=Continue` + `FailClosed` | Decision returns `true`; exit 2 — `on_error` does not override `failure_policy` for exhaustion | axes-independence (PC3) |
| `Crashed` (crash) + `on_error=Block` + `FailOpen` | Decision returns `true`; exit 2 — crash governed by `on_error` | crash-path (PC4) |
| `Timeout { cause: Fuel }` + `on_error=Block` + `FailOpen` | Decision returns `false`; exit 0 — exhaustion not governed by `on_error=block` when `failure_policy=FailOpen` | exhaustion-is-not-crash (PC5) |
| `Timeout { cause: Epoch }` + `FailClosed` | Decision returns `true`; exit 2 — epoch exhaustion = fuel exhaustion for enforcement | epoch-exhaustion (PC6) |
| Real dispatch: plugin with `fuel_cap=100` + `failure_policy="fail-closed"` on budget-exhausting payload | Observed dispatcher exit code 2 | integration (PC1 behavioral) |
| `hooks-registry.toml` entry with `failure_policy="fail-closed"` + `fuel_cap=20_000_000` (factory default per ADR-042 §Decision 2, strictly below floor) | `test_no_fail_closed_plugin_with_uncalibrated_cap` FAILS (CI blocks the half-state) | half-state-rejected (PC8 POSITIVE-CONTROL, F-S2111-P3-001) |
| All six targeted plugins with `failure_policy="fail-closed"` + `fuel_cap=75_000_000` (example calibrated) | `test_all_six_validator_class_plugins_are_fail_closed` passes; PC8 gate passes | final-state (PC9) |
| `hooks-registry.toml` entry with `failure_policy="fail-closed"` + `fuel_cap=75_000_000` (>= 50M floor, calibrated) | `test_no_fail_closed_plugin_with_uncalibrated_cap` PASSES / does not fire (gate accepts valid calibrated entry) | negative-control (PC8 NEGATIVE-CONTROL fixture, F-S2111-P2-004) |
| `hooks-registry.toml` entry with `failure_policy="fail-closed"` + `fuel_cap=50_000_000` (exactly at inclusive floor) | `test_no_fail_closed_plugin_with_uncalibrated_cap` PASSES / does not fire (inclusive floor: exactly 50_000_000 is a valid calibrated value per ADR-039 §Decision 4) | boundary-pass (PC8, F-S2111-P3-001 inclusive-floor) |
| `Timeout { cause: Fuel }` + `on_error=Block` + `FailOpen` (revision of `fail_closed_timeout_with_on_error_block` sub-case a) | Decision returns `false`; exit 0 — exhaustion governed by `failure_policy=FailOpen`; `on_error=Block` does not apply to exhaustion | axes-independence-on_error_block-fail-open (PC10a) |
| `Timeout { cause: Fuel }` + `on_error=Block` + `FailClosed` (revision of `fail_closed_timeout_with_on_error_block` sub-case b) | Decision returns `true`; exit 2 — exhaustion governed by `failure_policy=FailClosed`; block caused by failure_policy, not on_error | axes-independence-on_error_block-fail-closed (PC10b) |
| executor.rs contains 3-arg `plugin_fail_closed` + one `on_error="block"` targeted plugin lacks `failure_policy="fail-closed"` | `test_no_on_error_block_without_fail_closed_when_3arg_executor` FAILS (CI blocks merge of the bad intermediate CWE-636 state) | migration-window-gate (PC11) |
| executor.rs contains 3-arg `plugin_fail_closed` + all five `on_error="block"` targeted plugins carry `failure_policy="fail-closed"` | `test_no_on_error_block_without_fail_closed_when_3arg_executor` PASSES (Phase 4 complete state) | migration-window-pass (PC11 NEGATIVE-CONTROL) |

## Related BCs

- **BC-1.01.016** — prerequisite schema: provides `FailurePolicy` enum and
  `RegistryEntry.failure_policy` field; S-21.10 (BC-1.01.016) MUST merge before S-21.11 (this BC)
- **BC-1.03.002** — sibling detection layer: governs `invoke_plugin` returning
  `PluginResult::Timeout { cause: TimeoutCause::Fuel }` when fuel is exhausted; BC-1.03.017
  governs the enforcement decision the executor makes with that result based on `failure_policy`.
  The two BCs are complementary: BC-1.03.002 establishes the detection precondition;
  BC-1.03.017 establishes the enforcement postcondition.
- **BC-1.03.009** — sibling block-intent: governs `block_intent` for the `HookResult::Block`
  path; BC-1.03.017 adds a parallel block-intent path for exhaustion outcomes under
  `failure_policy = "fail-closed"`

## Architecture Anchors

- `crates/factory-dispatcher/src/executor.rs` — `plugin_fail_closed` function (extended to
  accept `failure_policy: FailurePolicy`); for
  `Timeout { cause: TimeoutCause::Fuel | TimeoutCause::Epoch }`, returns `true` when
  `failure_policy == FailurePolicy::FailClosed` regardless of `on_error`;
  `fail_closed_timeout_with_on_error_continue_is_open` test MUST be revised (not deleted) to
  assert `Timeout + Continue + FailOpen → NOT block` (PC7);
  `fail_closed_timeout_with_on_error_block` test MUST ALSO be revised (not deleted) — per
  PC10, the revised test asserts both sub-cases: (a) `Timeout + on_error=Block + FailOpen →
  NOT block` (exit 0) and (b) `Timeout + on_error=Block + FailClosed → block` (exit 2);
  both sibling tests (`on_error_continue` and `on_error_block`) require the same axes-
  independence treatment; TD-VSDD-059 applies to both
- `plugins/vsdd-factory/hooks-registry.toml` — six targeted plugin entries receive calibrated
  `fuel_cap >= 50M` AND `failure_policy = "fail-closed"` atomically per-plugin (50_000_000 is
  the inclusive floor; values below 50M are rejected by PC8); Phase 4 annotations land ONLY
  after Phase 3 calibration completes; PC8 gate test enforces no half-state; PC11 gate test
  enforces no on_error=block targeted plugin at fail-open while 3-arg executor is present
- `crates/factory-dispatcher/src/registry.rs` — `FailurePolicy` enum and
  `RegistryEntry.failure_policy` field (delivered by S-21.10 / BC-1.01.016); executor reads
  `failure_policy` from the dispatched `RegistryEntry`
- ADR-039 §Decision 3 — safe migration ordering: Phase 1 (schema) → Phase 2 (mitigations) →
  Phase 3 (calibration) → Phase 4 (enforcement flip); no half-state at any CI-passing commit
- ADR-039 §Decision 6 — four required behavioral test scenarios: Timeout+FailClosed→block
  (PC1); Timeout+FailOpen→advisory (PC2); on_error independence (PC3); crash≠exhaustion
  distinct paths (PC4+PC5)

## Story Anchors

- S-21.10 (prerequisite: Phase 1 schema extension; BC-1.01.016)
- S-21.11 (Phase 3 calibration + Phase 4 enforcement flip)
- S-21.13 (EC-004 follow-up: validate-cross-site-correspondence targeted-row lookup eliminating O(n) fuel ceiling; depends_on [S-21.10, S-21.11]; HARD BLOCKER if any on_error=block plugin is deferred under EC-004 path (b))

## VP Anchors

- VP-TBD — failure_policy enforcement dispatch: all six postconditions exercised by behavioral
  tests driving the actual dispatch path; half-state structural gate; all six targeted
  validators carry `failure_policy="fail-closed"` with calibrated `fuel_cap >= 50M`
  (inclusive floor); migration-window completeness gate (PC11)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | For resource-exhaustion outcomes (`TimeoutCause::Fuel`, `TimeoutCause::Epoch`): `failure_policy=FailClosed` → block (exit 2); `failure_policy=FailOpen` → advisory (exit 0); `on_error` does not override `failure_policy` for exhaustion; crash (`PluginResult::Crashed`) is governed by `on_error` only; no `failure_policy="fail-closed"` entry in `hooks-registry.toml` without `fuel_cap >= 50M` (inclusive calibration floor per ADR-039 §Decision 4; `fuel_cap < 50M` is prohibited; `fuel_cap = 50M` is VALID); all targeted validators carry `failure_policy="fail-closed"` with `fuel_cap >= 50M`; `fail_closed_timeout_with_on_error_block` test revised (not deleted) to assert both `on_error=Block + FailOpen → NOT block` and `on_error=Block + FailClosed → block` (PC10; TD-VSDD-059); PC8 gate test includes both POSITIVE-CONTROL (fail-closed + fuel_cap=20M < 50M floor → RED) and NEGATIVE-CONTROL (fail-closed + fuel=75M → PASS; and fuel=50M → PASS) fixtures (POLICY 15); PC11 gate test asserts that if 3-arg executor is present, all five on_error=block targeted plugins carry failure_policy=fail-closed (CWE-636 static gate). | unit tests (executor path coverage per PC1–PC6, PC10) + integration/bats test (real dispatch at `fuel_cap=100` → exit 2; PC1 behavioral) + Cargo gate tests (hooks-registry.toml parse; PC8 with both controls; PC11 migration-window gate) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 (Hook Dispatcher Core) — `crates/factory-dispatcher/src/executor.rs`; enforcement dispatch for resource-exhaustion outcomes |
| ADR | ADR-039 §Decision 1 (axes separation: exhaustion vs crash); ADR-039 §Decision 2 (validator-class plugins use `fail-closed` after calibration); ADR-039 §Decision 3 (safe migration ordering; Phase-3-before-Phase-4 atomicity; half-state forbidden); ADR-039 §Decision 4 (p99×1.5 fuel-cap calibration; Option A minimum requirement; 50M floor); ADR-039 §Decision 6 (four behavioral test scenarios; Envoy #38801 lesson — behavioral tests not configuration tests) |
| Security | CWE-636 (Not Failing Securely — closed for six validator-class WASM plugins after Phase 4); CWE-390 (Detection of Error Condition Without Action — closed for enforcement path). Research basis: `.factory/research/wasm-fuel-exhaustion-detection.md` |
| Stories | S-21.10 (prerequisite), S-21.11, S-21.13 (EC-004 follow-up; HARD BLOCKER if on_error=block plugin deferred) |
| Cycle | v1.0-brownfield-backfill (E-21 Wave 6) |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-08-17 | product-owner | Adversary pass-3 remediation (two F-S2111-P3 findings): (1) F-S2111-P3-001 — reconciled 50M boundary to inclusive floor (>= 50_000_000 ACCEPT, < 50_000_000 REJECT) — atomic sibling sweep across PC8, PC9, Invariant 2, Invariant 7, Architecture Anchors, VP-TBD, and Canonical Test Vectors; POSITIVE-CONTROL fixture updated from fuel_cap=10_000_000 to fuel_cap=20_000_000 (factory default per ADR-042 §Decision 2, clearly below floor and realistic); added boundary-pass test vector asserting fuel_cap=50_000_000 PASSES (the calibration-formula minimum is now an inclusive ACCEPT). (2) F-S2111-P3-005 — PC11 added: hard migration-window completeness CI gate (test_no_on_error_block_without_fail_closed_when_3arg_executor) asserting that if the extended 3-arg plugin_fail_closed signature is present in executor.rs, every on_error="block" targeted plugin MUST carry failure_policy="fail-closed"; closes the CWE-636 static-gap left by Invariant 7's ordering rule (which was ordering-based, not commit-checkable); PC11 test vector added. |
| v1.2 | 2026-08-17 | product-owner | Adversary pass-2 remediation (five F-S2111-P2 findings): (1) F-S2111-P2-001 — PC8 extended with symmetric half-state prohibition: no on_error=block targeted plugin may remain at failure_policy=fail-open once the extended 3-arg plugin_fail_closed is in effect; Invariant 7 added codifying migration-ordering atomicity and naming the five at-risk plugins. (2) F-S2111-P2-002 — PC10 added: fail_closed_timeout_with_on_error_block MUST be deliberately revised (TD-VSDD-059) to assert axes-independent sub-cases (FailOpen→NOT block, FailClosed→block); Canonical Test Vectors and Architecture Anchors updated. (3) F-S2111-P2-003 — EC-004 extended: deferral NOT behavior-neutral for on_error=block plugins (CWE-636 regression if left at fail-open); two remediation paths enumerated (fallback gate OR hard-blocker on follow-up); PC9 annotated with cross-reference to EC-004 on_error=block consequence. (4) F-S2111-P2-004 — PC8 extended with NEGATIVE-CONTROL fixture (fuel_cap=75_000_000, >50M floor → gate must PASS/not fire), closing POLICY 15 single-outcome-control gap; Canonical Test Vectors updated. (5) F-S2111-P2-006 — EC-004 names S-21.13 as concrete follow-up story anchor (Canonical Principle Rule 3); Story Anchors updated. |
| v1.1 | 2026-08-17 | product-owner | Spec-review remediation (F-S2111 adversary + SR findings): (1) F-S2111-P1-001 — HookEntry→RegistryEntry in Precondition 1, Related BCs BC-1.01.016 bullet, and Architecture Anchors registry.rs bullet (×2); phantom struct — actual is `pub struct RegistryEntry` in registry.rs. (2) F-S2111-P1-003/SR-001 — PC8 reclassified as standing regression/invariant gate (green-when-empty, green-at-final-state, RED on bad half-state); POSITIVE-CONTROL fixture requirement added; red-first framing removed; PC9/AC-009 is the genuine red-first gate. (3) F-S2111-P1-004/SR-008 — PC8 + Invariant-2 + VP gate threshold raised 20M→50M (calibration floor ADR-039 §Decision 4; factory default 20M per ADR-042 §Decision 2 is below the floor). (4) SR-002 — EC-004 vs PC9 deadlock resolved: explicit descoping-to-flippable-subset via orchestrator-approved spec amendment added to EC-004 and PC9; PC9 now conditional on post-amendment set. (5) SR-004 — PluginResult::Error→PluginResult::Crashed in PC4 and Canonical Test Vectors; no Error variant in invoke.rs enum (variants: Ok, Timeout, Crashed). (6) F-S2111-P1-008 — Precondition 2 fixture citation corrected: phantom S-21.07 task #33 replaced with committed path (BC-INDEX.md at 576,396 bytes). |
| v1.0 | 2026-08-06 | product-owner | Initial creation (S-21.10/S-21.11 BC authoring burst; ADR-039 §Decision 3+6 Phase 4 enforcement leg; four behavioral test scenarios from Decision 6 as PC1–PC6; structural half-state gate PC8; six targeted validators PC9; CWE-636+CWE-390 closure). |
