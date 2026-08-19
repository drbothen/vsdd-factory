---
document_type: behavioral-contract
level: L3
version: "v1.13"
status: draft
producer: product-owner
timestamp: 2026-08-06T00:00:00Z
last_amended: "2026-08-19 (v1.13) — S-21.11 v2.0 adversarial pass-1 remediation (product-owner; F-S2111V2-P1-001-mechanism-adjudication memo, ADR-039 §AMD-003 RATIFIED v1.11): new PC13 asserts the §AMD-003 rule — a fail-closed-eligible plugin (`on_error = OnError::Block`) whose outcome is `PluginResult::Ok { exit_code != 0, .. }` MUST be treated as a block (`block_intent = true`, exit 2), regardless of `failure_policy`; covers both a `legacy-bash-adapter.wasm` host-wall-clock timeout surfacing as `HookResult::Error` -> exit 1 (F-001) and any other generic `HookResult::Error` exit path (F-005, ruled in-scope). Includes POSITIVE control (`on_error=Block` + `Ok{exit!=0}` -> block) and two NEGATIVE controls (`on_error=Block` + `Ok{exit==0}` -> no block; `on_error=Continue` + `Ok{exit!=0}` -> unaffected). New Invariant 10 codifies PC13 as a strict superset of the pre-existing `Crashed | Timeout` rule. Traceability ADR row extended to cite ADR-039 §AMD-003 alongside §AMD-001/§AMD-002. EC-011 corrected (F-002): the prior 'silent false clean-pass at 45s' pre-fix characterization was wrong — the pre-fix outcome is nondeterministic (`PluginResult::Ok{exit_code:1}` via `HookResult::Error`, OR a guest-epoch `Timeout` race on control-return), not a deterministic clean pass; post-fix, PC12 (kill timing) + PC13 (`Ok{exit!=0}` -> block) together close every sub-case. Three new PC13 Canonical Test Vector rows added. Architecture Anchors extended to cite `crates/hook-sdk/src/result.rs::HookResult::exit_code` and the PC13 decision-site extension in `executor.rs`. H1 enriched with PC13's clause per POLICY 7. PC count extended PC1-PC12 -> PC1-PC13 (additive-only; no renumbering). Scope note: this burst does NOT touch S-21.11's story body/ACs (story-writer's domain) and does NOT alter PC1-PC12's existing semantics. BC-1.03.017 v1.13. [Prior: 2026-08-19 (v1.12) — S-21.11 expanded-scope BC coverage burst (product-owner; scoped to the AMD-002 runtime-wiring gap only, orchestrator directive): new PC12 asserts the RUNTIME behavior AMD-002 (RATIFIED v1.10) identified as unwired — for `legacy-bash-adapter.wasm`-hosted plugins, the actual bash-subprocess kill deadline (`exec_subprocess.rs::run()`) MUST equal the registry's calibrated `timeout_ms`, not the hardcoded `BASH_TIMEOUT_MS=60_000` constant in `run_bash_via_host`; includes a POSITIVE control (short `timeout_ms` kills early) and a NEGATIVE reference documenting the current pre-fix 60s-regardless-of-config defect state, plus a highest-risk EC-011 (script duration between calibrated `timeout_ms` and the hardcoded 60s produces a silent false clean-pass under the current implementation). New Precondition 5 states the config-vs-runtime assumption gap explicitly. PC9 amended (additive) with a PC12-dependency clause: registry-config completeness (fuel_cap/timeout_ms set) is necessary but, per §AMD-002, not alone sufficient for the five bash-adapter plugins to be treated as fully protective. New Invariant 9 codifies the config-vs-runtime wiring bifurcation. Two new Canonical Test Vector rows (PC12 POSITIVE/NEGATIVE) plus one EC-011 vector added. Architecture Anchors extended to cite `legacy-bash-adapter/src/lib.rs::run_bash_via_host` and `exec_subprocess.rs::run()`'s 5ms poll loop (explicitly distinguished from the unrelated wasmtime `EPOCH_TICK_MS`=10ms guest-epoch ticker per ADR-039 §Decision 4 v1.9 mechanism-precision correction). Traceability `L2 Capability` resolved from placeholder `CAP-TBD` to `CAP-011` (\"Enforce fuel and epoch budgets on plugin execution\") with a new S-7.01 Capability Anchor Justification row added (capabilities.md §CAP-011 verbatim cite) — this BC's enforcement-dispatch scope, extended by PC12 to the bash-adapter wiring's runtime correctness, is squarely CAP-011's 'a runaway plugin is killed within timeout_ms...never hung processes' outcome. PC count extended PC1-PC11 -> PC1-PC12 (additive-only; no renumbering). Scope note: this burst does NOT touch S-21.11's story body/ACs (story-writer's domain, dispatched separately) and does NOT alter PC1-PC11's existing semantics. BC-1.03.017 v1.12. [Prior: 2026-08-19 (v1.11) — Sibling-sweep citation update (architect; TD-VSDD-060; parallel to ADR-039 v1.9->v1.10, same burst): Traceability row's ADR citation updated — §AMD-002 now cites RATIFIED (2026-08-19, v1.10, POLICY 22) instead of PROPOSED/NOT RATIFIED, with the corrected corroboration basis (ADR-039's own v1.8 §AMD-001 -> v1.9 §Decision 4 mechanism-precision self-correction, not the retracted ADR-025 §Decision 18 citation); §Decision 3's break-glass citation redirected from named follow-up S-21.17 to S-21.11 (absorbed, no-split human decision); AMD-002's named follow-up S-21.18 likewise redirected to S-21.11. Stories row unaffected (already cites S-21.11). Citation-only sweep: no PC/Precondition/Invariant content altered; PC count unchanged at PC1-PC11. BC-1.03.017 v1.11. [Prior: 2026-08-18 (v1.10) — F-S2111-P13-001 research-corrections fold-in (architect; parallel terminology sweep + AMD-002 cite, ADR-039 v1.9): swept 'epoch axis'/'epoch-axis floor'/'epoch mechanism' prose terminology to 'host-wall-clock-timeout axis' throughout Preconditions 2/3, PC8, PC9, Canonical Test Vectors, Architecture Anchors, VP-TBD, and Traceability — `timeout_ms` field name and `TimeoutCause::Epoch`/`Timeout{Epoch}` Rust code identifiers unchanged (literal code, not prose). Architecture Anchors + VP-TBD + Traceability updated to additionally cite ADR-039 §AMD-002 (PROPOSED/NOT RATIFIED architect self-verification finding: `legacy-bash-adapter`'s bash-subprocess kill deadline is a fixed 60,000 ms constant independent of the registry's calibrated `timeout_ms`; named follow-up S-21.18, new, not yet authored) alongside the now-RATIFIED §AMD-001. No PC/Precondition/Invariant semantics altered; PC count unchanged at PC1-PC11. BC-1.03.017 v1.10. [Prior: 2026-08-18 (v1.9) — F-S2111-P13-001 remediation (architect; scoped architectural precondition/PC correction; ADR-039 §Decision 1/2/3/4 v1.8 amendment): Precondition 2/3 bifurcated by plugin adapter class — the native-WASM plugin (validate-cross-site-correspondence) calibrates fuel_cap per the original formula; the five legacy-bash-adapter.wasm-hosted plugins (validate-factory-path-root, validate-input-hash, validate-template-compliance, validate-wave-gate-prerequisite, validate-pr-merge-prerequisites) additionally calibrate timeout_ms per the new epoch-axis formula (timeout_ms >= max(measured_p99_ms x 2.0, 30_000)) because their bash subprocess execution is invisible to the WASM fuel counter (ADR-042 §Decision 3 class (b)). PC8 extended with a parallel timeout_ms structural half-state assertion (POSITIVE/NEGATIVE controls added) for legacy-bash-adapter.wasm entries — fuel_cap sufficiency alone is no longer treated as complete calibration evidence for these five plugins. PC9 final-state criterion updated to require both axes per plugin's adapter class. New Invariant 8 codifies the axis-bifurcation principle. Two new Canonical Test Vector rows added (PC8 timeout_ms POSITIVE/NEGATIVE controls). Architecture Anchors + VP-TBD + Traceability updated to cite ADR-039 v1.8 §AMD-001. PC count unchanged at PC1..PC11 (no renumbering); this is additive-only within existing PCs plus one new Invariant. Residual product-owner BC-body edit noted: this burst does NOT touch AC-to-PC narrative mapping in the S-21.11 story body (deferred to post-ratification resume burst per orchestrator scoping) and does NOT alter PC1-PC7/PC10/PC11's axes-independence or migration-window substance, which remain product-owner's domain if further narrative refinement is needed. BC-1.03.017 v1.9. [Prior: 2026-08-18 (v1.8) — F-S2111-P11-001 remediation (product-owner): extended PC10 to require deliberate revision (TD-VSDD-059) of BOTH the unit test fail_closed_timeout_with_on_error_block AND its integration-level mirror test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block (TC-12, full_stack_plugin_invocation.rs); TC-12 currently asserts exit_code==2 for on_error=Block+failure_policy=FailOpen+Timeout{Epoch} — false under axes-independent semantics (PC5/EC-009); must be revised to assert exit 0, with a SHOULD arm for failure_policy=FailClosed→exit 2 (Invariant 6 / Envoy #38801 symmetric coverage); two TC-12 Canonical Test Vector rows added; Architecture Anchors updated to cite TC-12; VP-TBD updated. BC-1.03.017 v1.8.]]"
phase: brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/specs/architecture/decisions/ADR-042-validate-cross-site-correspondence-fuel-budget-raise-and-loud-exhaustion-signaling.md
  - .factory/research/wasm-fuel-exhaustion-detection.md
input-hash: "4ba4e32"
traces_to: .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-011"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill-E21-W6
modified:
  - "2026-08-17 (v1.1)"
  - "2026-08-17 (v1.2)"
  - "2026-08-17 (v1.3)"
  - "2026-08-17 (v1.4)"
  - "2026-08-17 (v1.5)"
  - "2026-08-17 (v1.6)"
  - "2026-08-18 (v1.7)"
  - "2026-08-18 (v1.8)"
  - "2026-08-18 (v1.9)"
  - "2026-08-18 (v1.10)"
  - "2026-08-19 (v1.11)"
  - "2026-08-19 (v1.12)"
  - "2026-08-19 (v1.13)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.03.017: factory-dispatcher::executor::failure_policy enforcement — exhaustion-outcome dispatch (fail-closed→block; fail-open→advisory), on_error axes independence, crash-versus-exhaustion distinct paths, Phase-3-before-Phase-4 structural half-state gate, migration-window on_error=block completeness gate, and legacy-bash-adapter runtime-timeout-wiring verification (ADR-039 §Decision 3+6 Phase 4 enforcement leg + §AMD-002 wiring leg + §AMD-003 on_error=Block plugin-error fail-closed leg; PC11 CWE-636 static gate; PC12 AMD-002 runtime gate; PC13 AMD-003 plugin-error-exit fail-closed gate)

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
2. Per-plugin calibration (devops-engineer role) has been executed for each of the six
   targeted validator-class plugins, bifurcated by plugin adapter class per ADR-039 §Decision
   1/3/4 (v1.8 amendment; §AMD-001):
   - **Native-WASM plugin — fuel-axis calibration:** `validate-cross-site-correspondence`
     (hosted by its own `hook-plugins/validate-cross-site-correspondence.wasm` binary).
     `fuel_consumed` is measured against the calibration corpus below.
   - **`legacy-bash-adapter.wasm`-hosted plugins — host-wall-clock-timeout-axis calibration
     ADDITIONALLY required (fuel-axis calibration alone is insufficient):** `validate-factory-path-root`,
     `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`,
     `validate-pr-merge-prerequisites`. Their bash subprocess execution is invisible to the
     WASM fuel counter (fuel exhaustion, if any, occurs before the WASI `exec_subprocess`
     call per ADR-042 §Decision 3 class (b)); their actual resource-exhaustion axis is the
     host-enforced wall-clock deadline. `time_consumed_ms` (bash subprocess wall-clock
     duration) is measured against the same calibration corpus, in ADDITION to (not instead
     of) `fuel_consumed` for the adapter's own marshaling step.

   Calibration corpus MUST include: `lessons.md` at ≥4000 lines; `STATE.md` at current live
   size; `decision-log.md` at current live size; and the 576,396-byte production-scale
   fixture at
   `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md`.
   The same corpus backs both the fuel-axis and host-wall-clock-timeout-axis measurements — only the metric
   collected differs by adapter class.
3. For each targeted plugin, the calibrated value has been set per its adapter class from
   precondition 2 measurements:
   - Native-WASM plugin: `fuel_cap` set to `max(measured_p99 × 1.5, 50_000_000)`.
   - `legacy-bash-adapter.wasm`-hosted plugins: `fuel_cap` set to
     `max(measured_p99 × 1.5, 50_000_000)` for the adapter's marshaling step AND `timeout_ms`
     set to `max(measured_p99_ms × 2.0, 30_000)` for the bash subprocess wall-clock budget
     (ADR-039 §Decision 4 host-wall-clock-timeout-axis formula, v1.8). Both fields MUST be set; neither
     substitutes for the other.
4. The calibration results (plugin name, p99 measured, chosen `fuel_cap`) are recorded in the
   PR description or a calibration log artifact before Phase 4 annotations land.
5. **AMD-002 runtime-wiring precondition (assumption underlying PC1/PC6/PC9 for
   `legacy-bash-adapter.wasm`-hosted plugins; ADR-039 §AMD-002, RATIFIED v1.10):**
   Preconditions 2/3's calibrated `timeout_ms` value is a REGISTRY-CONFIG assumption only.
   PC1/PC6's enforcement decision (`failure_policy=FailClosed` → block on
   `Timeout{cause: Epoch}`) implicitly assumes the `Timeout{cause: Epoch}` outcome itself
   fires at the calibrated `timeout_ms`, not at some unrelated value. For
   `legacy-bash-adapter.wasm`-hosted plugins, this assumption does NOT hold until the
   AMD-002 wiring fix lands (PC12): prior to the fix, the adapter's bash-subprocess kill
   deadline is a fixed `BASH_TIMEOUT_MS = 60_000` constant
   (`crates/hook-plugins/legacy-bash-adapter/src/lib.rs::run_bash_via_host`), independent of
   the registry's calibrated `timeout_ms` fed to `exec_subprocess.rs::run()`. PC9's
   final-state completeness assertion for the five `legacy-bash-adapter.wasm`-hosted plugins
   is registry-complete but NOT runtime-complete until PC12 additionally holds.

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
   `fuel_cap`; both positive and negative gate controls present (standing regression/invariant
   gate; migration-window on_error=block ordering constraint is mechanically enforced by PC11,
   not by this gate):**
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

   **Parallel host-wall-clock-timeout-axis assertion for `legacy-bash-adapter.wasm`-hosted entries (F-S2111-P13-001;
   ADR-039 §Decision 1/4 v1.8 amendment — fuel-axis calibration is necessary but NOT sufficient
   for these entries):** The same test MUST ALSO assert that no `[[hook]]` entry whose
   `plugin = "hook-plugins/legacy-bash-adapter.wasm"` carries both `failure_policy = "fail-closed"`
   AND `timeout_ms < 30_000` (the host-wall-clock-timeout-axis calibration floor per ADR-039 §Decision 4 v1.8
   formula: `max(measured_p99_ms × 2.0, 30_000)`; exactly `30_000` is the inclusive minimum).
   This assertion is IN ADDITION to the `fuel_cap` assertion above, not a replacement — a
   `legacy-bash-adapter.wasm`-hosted entry satisfying `fuel_cap ≥ 50_000_000` alone remains
   half-state and MUST still fail this gate if `timeout_ms < 30_000`, because `fuel_cap` gives
   no protection against that adapter class's actual exhaustion axis (the bash subprocess is
   invisible to the WASM fuel counter). The host-wall-clock-timeout-axis assertion likewise requires both
   controls:
   (c) **TIMEOUT-POSITIVE-CONTROL fixture** (a hard-coded `legacy-bash-adapter.wasm` entry with
   `failure_policy = "fail-closed"` and `timeout_ms = 10_000`, i.e., the current live default
   for four of the five targeted bash-adapter plugins and strictly below the 30_000 floor,
   injected directly in the test body) that asserts the gate fires RED on that fixture.
   (d) **TIMEOUT-NEGATIVE-CONTROL fixture** (a hard-coded `legacy-bash-adapter.wasm` entry with
   `failure_policy = "fail-closed"` and `timeout_ms = 45_000`, i.e., above the 30_000 floor,
   injected directly in the test body) that asserts the gate does NOT fire on that fixture
   (result: PASS / no error).
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
   co-committed (same commit) or ordered annotate-first-then-flip. The mechanical CI gate
   enforcing this ordering constraint is PC11
   (`test_no_on_error_block_without_fail_closed_when_3arg_executor`), not this gate; PC8's
   test asserts only the calibration constraint (no fail-closed without `fuel_cap >= 50M`).
   A plugin entry MUST NOT carry `failure_policy = "fail-closed"` without simultaneously
   carrying `fuel_cap >= 50_000_000` (exactly 50_000_000 is the inclusive floor and a VALID
   calibrated value per ADR-039 §Decision 4).

9. **PC9 — All targeted validator-class plugins carry `failure_policy = "fail-closed"` with
   calibration sufficient for their adapter class's actual exhaustion axis in final state
   (bifurcated per ADR-039 §Decision 1/4 v1.8 amendment; F-S2111-P13-001):**
   After all Phase 4 calibration-and-annotation commits land, `hooks-registry.toml` MUST
   contain `failure_policy = "fail-closed"` for all plugins in the **post-amendment targeted
   set**, AND each plugin's calibrated field(s) MUST satisfy its adapter class's requirement:
   the native-WASM plugin (`validate-cross-site-correspondence`) MUST carry
   `fuel_cap >= 50_000_000`; each `legacy-bash-adapter.wasm`-hosted plugin
   (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
   `validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`) MUST carry BOTH
   `fuel_cap >= 50_000_000` AND `timeout_ms >= 30_000` — `fuel_cap` sufficiency alone does
   NOT satisfy PC9 for these five (their real exhaustion axis is the host wall-clock timeout, `timeout_ms`, per
   Invariant 8).
   **PC12 dependency for full protection (ADR-039 §AMD-002, RATIFIED v1.10):** satisfying
   this postcondition's registry-config criteria (`fuel_cap >= 50_000_000` and, for
   `legacy-bash-adapter.wasm`-hosted plugins, `timeout_ms >= 30_000`) is necessary but, per
   §AMD-002, NOT alone sufficient for the five `legacy-bash-adapter.wasm`-hosted plugins to
   be treated as fully protective — PC12's runtime-wiring assertion (the calibrated
   `timeout_ms` value must actually reach the bash-subprocess kill deadline, not the
   hardcoded `BASH_TIMEOUT_MS` constant) MUST additionally hold before S-21.11's Phase 4
   fail-closed flip for these five plugins is considered complete. A commit that satisfies
   PC9's registry-config criteria while PC12 remains unmet (wiring fix not yet landed) leaves
   the residual AMD-002 gap open and MUST be flagged to the orchestrator as a known
   limitation, not silently treated as PC9-complete. The default targeted set is all six of:
   `validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
   `validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`,
   `validate-cross-site-correspondence`. If EC-004 fires for any plugin in this set
   (calibration reveals an impractical cap requirement), S-21.11 is descoped to the flippable
   subset via orchestrator-approved spec amendment; PC9's asserted set is reduced to the
   flippable plugins only; the deferred plugin routes to named follow-up story S-21.13. PC9
   asserts the post-amendment set, not necessarily all six — a partial-set completion is valid
   if EC-004 applied.
   **Critical caveat for `on_error = "block"` plugins — EC-004 is NOT a valid descope path
   (F-S2111-P2-003, amended v1.4):** When calibration reveals a required `fuel_cap > 500M` for
   any of the five `on_error = "block"` plugins (`validate-factory-path-root`,
   `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
   `validate-wave-gate-prerequisite`), EC-004 deferral is NOT a permitted resolution — it is
   **annotate-or-block-the-flip only**: either (a) annotate the plugin
   `failure_policy="fail-closed"` within S-21.11 (even if the cap requirement is high; surface
   to orchestrator and raise the cap), or (b) block the entire Phase-4 executor flip (do not
   ship the enforcement-active decision path in S-21.11) until the plugin can be annotated in a
   follow-up. There is NO path that permits the enforcement-active executor to merge while any
   `on_error="block"` plugin remains at `failure_policy=fail-open` — PC11's CI gate makes that
   state mechanically un-mergeable. Routing an `on_error="block"` plugin to S-21.13 is a
   mis-route: S-21.13 is scoped exclusively to `validate-cross-site-correspondence`'s O(n)
   fuel-ceiling algorithmic fix and has no mandate to annotate `on_error="block"` plugins.
   The `validate-cross-site-correspondence` plugin (`on_error="continue"`) does NOT carry this
   regression risk — it already failed open on exhaustion under the 2-arg function; its deferral
   routes to S-21.13 per EC-004's valid on_error=continue descope path.
   **Annotation-landing obligation for EC-004 Case A (F-S2111-P5-005):** When
   `validate-cross-site-correspondence` is deferred to S-21.13, that story (or its named
   successor) MUST include an explicit mandate to annotate
   `validate-cross-site-correspondence` with `failure_policy="fail-closed"` and a calibrated
   `fuel_cap >= 50_000_000` once its O(n) fuel-ceiling algorithmic fix removes the excessive
   cap requirement. The fail-closed annotation MUST NOT fall through the EC-004 descope; the
   descope is a timing deferral only, not a permanent exemption from fail-closed enforcement.
   Advisory-only and observability plugins MUST NOT receive
   `failure_policy = "fail-closed"`.

10. **PC10 — `fail_closed_timeout_with_on_error_block` revised, not deleted (TD-VSDD-059
    complement to PC7):**
    The existing test `fail_closed_timeout_with_on_error_block` in the executor module
    (which under the 2-arg `plugin_fail_closed` currently asserts
    `Timeout { cause: Fuel|Epoch } + on_error=Block → block==true`) MUST be DELIBERATELY
    REVISED (not deleted) to assert the new axes-independent semantics. The revised test
    MUST cover both sub-cases:
    (a) `Timeout { cause: Fuel|Epoch } + on_error=Block + failure_policy=FailOpen → NOT block`
    (exit 0): exhaustion is governed by `failure_policy`; `on_error=Block` does NOT apply
    to exhaustion outcomes when `failure_policy=FailOpen` (PC5 / EC-009).
    (b) `Timeout { cause: Fuel|Epoch } + on_error=Block + failure_policy=FailClosed → block`
    (exit 2): exhaustion governed by `failure_policy=FailClosed`; both axes agree on block,
    but the block is caused by `failure_policy`, not `on_error` (PC1 / PC6 class).
    The function name MUST be retained or a close derivative used (e.g.,
    `fail_closed_timeout_with_on_error_block_axes_independent`). The revised test MUST
    appear in the PR diff. Deletion without an equivalent replacement is a TD-VSDD-059
    paper-fix violation. This PC is parallel to PC7's treatment of
    `fail_closed_timeout_with_on_error_continue_is_open` — both sibling tests require
    revision to accurately reflect the extended decision function's axes-independent
    semantics.
    **Integration-level mirror (TC-12) requires the same deliberate revision
    (F-S2111-P11-001):**
    The integration-level test `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block`
    (TC-12) in `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs` — labeled
    by the codebase as the integration-level mirror of `fail_closed_timeout_with_on_error_block`
    — constructs `on_error=Block + failure_policy=FailOpen (registry default) + Timeout{Epoch}`
    and currently asserts `exit_code==2`. Under the axes-independent semantics mandated by
    this BC (PC5/EC-009: `Timeout{Fuel|Epoch} + on_error=Block + failure_policy=FailOpen →
    NOT block → exit 0`), TC-12's `exit_code==2` assertion becomes FALSE. TC-12 MUST be
    DELIBERATELY REVISED (not deleted, per TD-VSDD-059) to assert the new semantics:
    (a) `on_error=Block + failure_policy=FailOpen + Timeout{Epoch} → exit 0`.
    TC-12 SHOULD also carry a corresponding `failure_policy=FailClosed` arm asserting:
    (b) `on_error=Block + failure_policy=FailClosed + Timeout{Epoch} → exit 2`
    for symmetric behavioral coverage (Invariant 6 / Envoy #38801 discipline: tests MUST
    assert observed outcomes, not merely configuration intent; the integration layer carrying
    only the FailOpen arm would leave the integration-level FailClosed path unverified at the
    dispatch level). Both the unit test revision and the TC-12 integration revision MUST appear
    in the PR diff. Deletion of either without an equivalent replacement is a TD-VSDD-059
    paper-fix violation.

11. **PC11 — Hard migration-window completeness gate: if the executor is in enforcement-active
    state, every `on_error="block"` targeted plugin MUST carry `failure_policy="fail-closed"`
    (CWE-636 static gate, checkable at any single commit; name-independent detection):**
    A Cargo integration test (`test_no_on_error_block_without_fail_closed_when_3arg_executor`)
    MUST assert: if `crates/factory-dispatcher/src/executor.rs` is in enforcement-active state —
    detected by the presence of any block-decision site in the executor block-decision chain
    (`execute_tier`, `execute_tiers`, or their helpers) that references a `.failure_policy` value
    when deciding to block on a `Timeout` outcome, however the data reaches that site (via
    `PluginOutcome`, direct field access on `RegistryEntry`, or any intermediate path); this
    signal is name-independent and data-flow-independent: fires for both extend-in-place and
    introduce-a-replacement implementer designs regardless of intermediate data-flow path — then every `[[hook]]` entry
    in `hooks-registry.toml` whose
    `name` is one of the five targeted `on_error = "block"` plugins (`validate-factory-path-root`,
    `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`,
    `validate-wave-gate-prerequisite`) MUST carry an explicit `failure_policy = "fail-closed"`.
    Any absence of this annotation while the executor is enforcement-active MUST cause the test to
    FAIL (CI blocks merge). This gate detects a failure_policy-dependent exhaustion block
    decision anywhere in the executor block-decision chain (design-flow-independent, not bound
    to any specific intermediate data-flow path); the gate evaluates both conditions (executor
    enforcement-active state AND registry annotation) on the same commit tree, with no ordering
    dependency. The bad intermediate state (enforcement-active executor + any on_error=block
    plugin at fail-open) causes the test to FAIL, making the CWE-636 migration-window
    regression mechanically detectable at any single commit. The gate is GREEN when the
    executor is NOT enforcement-active (Phase 1–2 state), and GREEN only when the executor is
    enforcement-active AND all five plugins carry `failure_policy = "fail-closed"` (Phase 4
    complete state). It fires RED on the bad intermediate state, which is precisely the CWE-636
    regression window that no prior PC gated mechanically.
    The gate MUST include four controls — three structured as PURE FUNCTIONS over INJECTABLE
    inputs (a synthetic executor-source snippet string + a synthetic registry, NOT a scan
    bound to the live tree) and one live-tree assertion at Phase-4-complete:
    (a) **POSITIVE-CONTROL:** enforcement-active executor-source snippet (any block-decision
        site references `.failure_policy` for `Timeout` outcome) + a synthetic registry MISSING
        one of the five on_error=block `failure_policy="fail-closed"` annotations → assert the
        gate fires RED. Proves non-vacuity: the detector fires on the bad intermediate CWE-636
        state; a source-text detector that silently matches zero cannot produce this RED.
    (b) **NEGATIVE-CONTROL:** enforcement-active executor-source snippet + a synthetic registry
        with ALL five on_error=block plugins annotated `failure_policy="fail-closed"` → assert
        the gate does NOT fire (result: PASS). Proves the gate correctly accepts the
        Phase-4-complete state and does not false-positive on valid fully-annotated
        configurations.
    (c) **VACUITY-CONTROL:** enforcement-ABSENT executor-source snippet (no `.failure_policy`
        reference in the block-decision chain for `Timeout` outcomes) + any synthetic registry
        state → assert the gate returns GREEN, AND assert the detector's enforcement-detection
        logic ran and returned `EnforcementAbsent` (via an explicit `detection_ran` / tri-state
        diagnostic), and that RED-emission was skipped as a consequence. This distinguishes a
        genuine Phase-1/2 GREEN (executor not yet enforcement-active; enforcement-detection
        logic ran, correctly classified the executor as enforcement-absent, and skipped
        RED-emission) from a vacuous GREEN caused by a detector failure that never ran the
        enforcement-detection logic at all.
    (d) **LIVE-TREE-CONTROL:** at Phase-4-complete, the detector MUST be run against the
        ACTUAL `crates/factory-dispatcher/src/executor.rs` (not a synthetic snippet) and MUST
        return `enforcement_active = true`. This proves the detector's enforcement-detection
        logic recognizes the real enforcement code as shipped — a syntactically-wrong live-tree
        detector whose `.failure_policy` scan matches zero against the actual `execute_tiers`
        form would pass controls (a), (b), and (c) yet be inert against real code (silent
        CWE-636 false-green). The live-tree assertion closes this gap by asserting the detector
        fires in the enforcement-ACTIVE direction against actual source. Acceptable
        implementation: the POSITIVE-CONTROL synthetic snippet MUST be a verbatim excerpt of
        the real `execute_tiers` block-decision site, AND the same detector run on the live
        tree MUST return `enforcement_active = true`.
    **Relationship to Invariant 7:** This PC makes Invariant 7's ordering rule machine-checkable.
    Invariant 7 remains in force as the human-readable policy statement; PC11 is its
    mechanically-enforced complement. PC11 does NOT replace PC8 or PC9 — those gates address
    different failure modes (uncalibrated caps and final-state completeness respectively).
    **EC-004 descope does NOT reduce the PC11 assertion set:** EC-004's reduced-set deferral
    (via orchestrator-approved spec amendment) applies ONLY to `validate-cross-site-correspondence`
    (`on_error = "continue"`), which is NOT among the five `on_error = "block"` plugins asserted
    by this gate. For the five `on_error = "block"` plugins, EC-004 is not a valid descope path
    — they must be annotated `failure_policy="fail-closed"` within S-21.11 or the Phase-4 flip
    must be blocked entirely (see EC-004 amendment v1.4). PC11's five-plugin assertion has no
    reduced-set escape.

12. **PC12 — AMD-002 runtime wiring: the effective bash-subprocess wall-clock kill deadline
    for `legacy-bash-adapter.wasm`-hosted plugins MUST equal the registry's calibrated
    `timeout_ms`, not the hardcoded `BASH_TIMEOUT_MS` constant (ADR-039 §AMD-002, RATIFIED
    v1.10; closes the runtime-wiring gap left open by PC8/PC9's registry-config-only
    assertions):**
    For any `[[hook]]` entry with `plugin = "hook-plugins/legacy-bash-adapter.wasm"` and
    registry field `timeout_ms = X`, a real dispatch that invokes that plugin against a bash
    subprocess whose runtime exceeds `X` MUST have its subprocess killed at approximately
    `X` — observed kill time within `X` plus the `exec_subprocess.rs::run()` poll interval
    (~5 ms; `std::thread::sleep(Duration::from_millis(5))`) — NOT at the current hardcoded
    60,000 ms (`legacy-bash-adapter::BASH_TIMEOUT_MS`). This is a distinct enforcement point
    from wasmtime's `EPOCH_TICK_MS` (10 ms, `crates/factory-dispatcher/src/engine.rs`) — that
    ticker governs guest-WASM epoch interruption and is unrelated to this host-level
    subprocess-kill deadline (ADR-039 §Decision 4 v1.9 mechanism-precision correction; do not
    conflate the two).

    **POSITIVE control (the wiring fix's target behavior):** a `hooks-registry.toml` entry
    hosted by `legacy-bash-adapter.wasm` with `timeout_ms = 2_000` (deliberately short),
    invoked against a bash script that sleeps 10 s. The subprocess MUST be observed killed at
    ≈2 s (well under the hardcoded 60 s), and the dispatcher MUST report
    `Timeout{cause: Epoch}` for that invocation at that observed time — proving the wiring
    fix reads and applies the registry's calibrated value rather than the hardcoded constant.

    **NEGATIVE reference (the AMD-002 defect state — documented as the current/pre-fix
    baseline, not a standing test that must continue passing):** under the CURRENT
    (pre-wiring-fix) implementation, the same `timeout_ms = 2_000` entry invoked against the
    same 10 s-sleeping script is NOT killed until ≈60 s, because `run_bash_via_host`
    (`crates/hook-plugins/legacy-bash-adapter/src/lib.rs`) passes its own hardcoded
    `BASH_TIMEOUT_MS` constant to the host call, never the registry's `timeout_ms`. The
    wiring fix's Cargo integration test
    (`test_legacy_bash_adapter_honors_registry_timeout_ms`) MUST assert the POSITIVE
    behavior, and MUST fail against the pre-fix code path (red-first against the current
    implementation, green only after AMD-002's wiring fix lands) — this is the Envoy #38801
    discipline (Invariant 6) applied to the wiring fix itself: the test drives the actual
    dispatch/subprocess path with a real short `timeout_ms` and a real long-running script,
    not merely a unit-level assertion that adapter code reads a config field.

    **Blast-radius scope note (non-restrictive on this PC's assertion; restrictive on
    S-21.11's `failure_policy` flip scope):** the wiring defect this PC closes is global to
    `legacy-bash-adapter.wasm` — ADR-039 §AMD-002 (v1.10) confirms it affects all ~37
    `legacy-bash-adapter.wasm`-routed `hooks-registry.toml` entries via live grep, not only
    the five §Decision 2 plugins. `test_legacy_bash_adapter_honors_registry_timeout_ms` MUST
    therefore be written generically against the adapter's wiring behavior (any
    `legacy-bash-adapter.wasm`-hosted entry), not hardcoded to only the five targeted plugins
    — the fix is adapter-level, not per-plugin. S-21.11's `failure_policy = "fail-closed"`
    annotation scope (PC9) remains the five/six named plugins only; PC12 does not expand
    PC9's targeted set.

13. **PC13 — `on_error = Block` fails closed on ANY plugin-reported error exit, not only a
    crash/timeout outcome (ADR-039 §AMD-003, RATIFIED v1.11; closes S-21.11 v2.0 adversarial
    pass-1 BLOCKER F-S2111V2-P1-001 — a bash-adapter host-wall-clock timeout, and any other
    `HookResult::Error` path, surfaces as `PluginResult::Ok { exit_code: 1, .. }`, which PC1-PC12's
    `Crashed | Timeout` matching does not catch):**
    For any plugin dispatched with `on_error = OnError::Block`, if the plugin's outcome is
    `PluginResult::Ok { exit_code, .. }` where `exit_code != 0`, the executor's block-decision
    function (`plugin_fail_closed` or its replacement) MUST return `true`
    (`block_intent = true`, dispatcher exit code 2), REGARDLESS of `failure_policy`. This is a
    THIRD axis alongside PC1-PC9's `failure_policy` (resource-exhaustion) coverage and
    PC4/PC5/PC10's `on_error`-vs-`Crashed` coverage — it is the missing `on_error`-vs-clean-
    nonzero-exit case: a plugin that ran to completion without crashing or timing out at the
    WASM-trap/epoch layer, but returned a nonzero exit via its own reported `HookResult::Error`
    (exit code 1 per `crates/hook-sdk/src/result.rs::HookResult::exit_code`), or via any other
    non-`outcome:block` nonzero exit path.

    **This rule closes two concrete instances of the same class, unified per ADR-039 §AMD-003's
    F-005 in-scope ruling:**
    (a) **F-001 — bash-adapter host-wall-clock timeout:** a `legacy-bash-adapter.wasm`-hosted
        plugin's bash-subprocess wall-clock timeout (`exec_subprocess.rs::run()`'s poll-loop
        kill) does NOT produce `PluginResult::Timeout { .. }` — that variant is constructed
        ONLY by `classify_trap` on a genuine `Trap::Interrupt`, which cannot fire while the
        guest is blocked inside the synchronous `exec_subprocess` host call. Instead it
        propagates as `host::exec_subprocess`'s `Err(codes::TIMEOUT)` -> `run_bash_via_host`'s
        string-erasing `Err` map -> `adapter_logic`'s `HookResult::error(...)` -> `exit_code = 1`
        -> `classify_trap`'s `Err(I32Exit(1))` arm -> `PluginResult::Ok { exit_code: 1, .. }`.
    (b) **F-005 — any other `HookResult::Error` path (generic, not timeout-specific):** ANY
        other error path inside `adapter_logic` that returns `HookResult::Error` (a missing
        `script_path`, a bash exit code other than 0/2, or `exec_subprocess` itself returning a
        non-timeout error) produces the identical `PluginResult::Ok { exit_code: 1, .. }` shape,
        and the identical fail-open gap applies. F-005 is ruled IN SCOPE for this BC (not
        deferred): both instances are closed by the identical one-line predicate change, so
        splitting them would require touching the same decision function twice for the
        identical root cause.

    **POSITIVE control:** a synthetic `PluginOutcome` with `on_error = OnError::Block` and
    `result = PluginResult::Ok { exit_code: 1, .. }` MUST cause the decision function to return
    `true` (`block_intent = true`; dispatcher exit code 2).

    **NEGATIVE control 1 (clean exit is not blocked):** a synthetic `PluginOutcome` with
    `on_error = OnError::Block` and `result = PluginResult::Ok { exit_code: 0, .. }` MUST NOT
    cause a block via this rule (exit 0) — a genuinely clean pass remains unaffected.

    **NEGATIVE control 2 (`on_error = Continue` is unaffected by this rule):** a synthetic
    `PluginOutcome` with `on_error = OnError::Continue` and
    `result = PluginResult::Ok { exit_code: 1, .. }` MUST NOT cause a block via this rule — the
    pre-existing fail-open-on-crash-when-`on_error=Continue` semantics are preserved unchanged;
    this rule is additive to the `on_error = Block` case only.

    **Relationship to `exit_code == 2`:** `exit_code == 2` (the `HookResult::Block` mapping)
    remains additionally, independently caught by `plugin_requests_block`'s stdout-substring
    check regardless of `on_error` (unconditional per the existing CRIT-PR59-001 fix); PC13's
    rule is redundant-but-harmless for that case and newly protective for `exit_code == 1`
    (`HookResult::Error`) and any other nonzero exit a compliant or non-compliant plugin may
    produce.

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

8. **Fuel-axis calibration is necessary but not sufficient for `legacy-bash-adapter.wasm`-hosted
   plugins (ADR-039 §Decision 1/4 v1.8 amendment; F-S2111-P13-001, architect-CONFIRMED HIGH):**
   Because a `legacy-bash-adapter.wasm`-hosted plugin's bash subprocess execution occurs after
   — and is invisible to — the adapter's own WASM fuel-metered marshaling step (ADR-042
   §Decision 3 class (b): fuel exhaustion, if any, occurs before the WASI `exec_subprocess`
   call), `fuel_cap` sufficiency provides no protection against a bash subprocess wall-clock
   hang. `validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
   `validate-wave-gate-prerequisite`, and `validate-pr-merge-prerequisites` are all hosted by
   `hook-plugins/legacy-bash-adapter.wasm` and are therefore all subject to this invariant.
   These five plugins additionally require calibrated `timeout_ms` sufficiency
   (`timeout_ms >= max(measured_p99_ms × 2.0, 30_000)`, ADR-039 §Decision 4 host-wall-clock-timeout-axis formula)
   before receiving `failure_policy = "fail-closed"`. `validate-cross-site-correspondence`
   (native `hook-plugins/validate-cross-site-correspondence.wasm`) is NOT subject to this
   invariant — its validation logic executes directly as WASM instructions, so `fuel_cap`
   genuinely bounds its execution end-to-end. **Self-lock consequence for the PreToolUse
   `^Agent$` gates:** `validate-wave-gate-prerequisite` and `validate-pr-merge-prerequisites`
   are two of the five `legacy-bash-adapter.wasm`-hosted plugins AND are registered on
   `event = "PreToolUse"`, `tool = "^Agent$"`. Flipping either to `failure_policy = "fail-closed"`
   on `fuel_cap` sufficiency alone, without demonstrated `timeout_ms` sufficiency, risks a
   hard, unconditional block on every future `Agent` tool dispatch — including the dispatches
   needed to fix the miscalibration (ADR-039 §Decision 3 v1.8 amendment).

9. **Config-vs-runtime wiring bifurcation for `legacy-bash-adapter.wasm`-hosted plugins
   (ADR-039 §AMD-002, RATIFIED v1.10; PC12):** A calibrated `timeout_ms >= 30_000` declared
   in `hooks-registry.toml` (Invariant 8 / PC8 / PC9) governs the DECLARED config value only.
   It does not, by itself, guarantee that value is the one enforced at the bash-subprocess
   kill deadline — `legacy-bash-adapter`'s `run_bash_via_host` currently feeds a hardcoded
   `BASH_TIMEOUT_MS = 60_000` constant to `exec_subprocess.rs::run()`, independent of the
   registry's `timeout_ms`. PC12 closes this wiring gap; until PC12's wiring fix lands,
   PC9's final-state assertion for the five `legacy-bash-adapter.wasm`-hosted plugins is
   registry-complete but NOT runtime-complete, and the residual gap MUST be surfaced to the
   orchestrator rather than silently treated as full protection.

10. **PC13 strict-superset invariant (ADR-039 §AMD-003, RATIFIED v1.11):** The
    `on_error = Block` + `PluginResult::Ok { exit_code != 0 }` -> block rule (PC13) is a STRICT
    SUPERSET of the pre-existing `Crashed | Timeout` fail-closed rule (PC4/PC10's
    `on_error`-governs-crash path). It does not remove, narrow, or reinterpret any existing
    block path — `Crashed` and `Timeout { .. }` outcomes continue to block under
    `on_error = Block` exactly as before; PC13 only ADDS the previously-uncaught
    `Ok { exit_code != 0 }` case. `exit_code == 2` (`HookResult::Block`'s mapping) remains
    additionally, independently caught by `plugin_requests_block`'s stdout-substring check
    regardless of `on_error` — PC13's rule is redundant-but-harmless for that case and newly
    protective for `exit_code == 1` (`HookResult::Error`) and any other nonzero exit a
    compliant or non-compliant plugin may produce. This mirrors Invariant 2's treatment of PC8
    as a strict superset of the pre-existing 2-arg calibration gate.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin with `failure_policy = "fail-closed"` completes successfully (no exhaustion) | Exit 0; both `on_error` and `failure_policy` are irrelevant for clean-pass outcomes |
| EC-002 | Plugin with `failure_policy = "fail-closed"` + `on_error = "continue"` crashes (not exhaustion) | Crash governed via `on_error = continue`; exit 0 (crash is advisory); `failure_policy` is not consulted for crash outcomes |
| EC-003 | Plugin with `failure_policy = "fail-closed"` + `on_error = "block"` crashes (not exhaustion) | Crash governed via `on_error = block`; exit 2; `failure_policy` not consulted for crash outcomes |
| EC-004 | Calibration reveals a targeted plugin needs `fuel_cap > 500M` for p99×1.5 | MUST surface to orchestrator; do not annotate with an insufficient cap. **Path diverges by `on_error` value — the two cases are NOT symmetric:** **Case A — `on_error="continue"` plugin (`validate-cross-site-correspondence`) — VALID descope path:** S-21.11 is descoped to the flippable subset via orchestrator-approved spec amendment; PC9's enumerated set is reduced to the flippable plugins only; the deferred plugin routes to named follow-up story **S-21.13** (validate-cross-site-correspondence targeted-row lookup eliminating the O(n) fuel ceiling; depends_on [S-21.10, S-21.11]). This is behavior-neutral: `validate-cross-site-correspondence` already fails open on exhaustion under the 2-arg function and continues to do so until annotated. PC11's five-plugin assertion is unaffected because `validate-cross-site-correspondence` is not among the five `on_error="block"` plugins asserted by PC11. **Annotation-landing obligation (F-S2111-P5-005):** S-21.13 (or its named successor) MUST include an explicit mandate to annotate `validate-cross-site-correspondence` with `failure_policy="fail-closed"` and a calibrated `fuel_cap >= 50_000_000` once its O(n) fuel-ceiling algorithmic fix removes the excessive cap requirement. The EC-004 descope is a timing deferral only, not a permanent exemption from fail-closed enforcement — the fail-closed annotation MUST NOT fall through the descope. **Case B — `on_error="block"` plugins (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`, `validate-pr-merge-prerequisites`, `validate-wave-gate-prerequisite`) — EC-004 is NOT a valid descope path (F-S2111-P4-002):** For these five plugins, deferral via EC-004 is **forbidden**. The only permitted resolutions are: **(a) annotate-within-S-21.11:** annotate the plugin `failure_policy="fail-closed"` in S-21.11 (even if the cap requirement is high; surface to orchestrator and raise it); OR **(b) block-the-flip:** do not ship the enforcement-active decision path in S-21.11 until the plugin is annotated in a follow-up story. There is NO path that permits the enforcement-active executor to merge while any `on_error="block"` plugin remains at `failure_policy=fail-open` — PC11's CI gate (test_no_on_error_block_without_fail_closed_when_3arg_executor) makes that state mechanically un-mergeable. Routing an `on_error="block"` plugin to S-21.13 is a mis-route: S-21.13 is scoped exclusively to `validate-cross-site-correspondence`'s O(n) fuel-ceiling algorithmic fix and has no mandate to annotate `on_error="block"` plugins. |
| EC-005 | `lessons.md` validator exhausts on a >4000-line `lessons.md` after Phase 4 flip | Signals calibration was insufficient (PC2+PC9 not met); surface to orchestrator; D-442(e) remains in force |
| EC-006 | `TimeoutCause::Epoch` with `failure_policy = "fail-closed"` | BLOCK (exit 2); epoch deadline is a resource-exhaustion outcome; same enforcement path as `TimeoutCause::Fuel` |
| EC-007 | New validator-class plugin added after S-21.11 merges (without `failure_policy = "fail-closed"`) | Defaults to `fail-open` per BC-1.01.016 backward-compat; PC8 gate only fires for annotated-but-uncalibrated entries; classification of new plugins is a future-story concern |
| EC-008 | Plugin with `on_error = "block"` + `failure_policy = "fail-closed"` exhausts fuel | BLOCK via `failure_policy` path; `on_error = block` is redundant for exhaustion when `failure_policy = "fail-closed"` but both agree on the block outcome |
| EC-009 | Plugin with `on_error = "block"` + `failure_policy = "fail-open"` exhausts fuel | Exit 0; exhaustion governed by `failure_policy = fail-open`; `on_error = block` does not apply to exhaustion outcomes (PC5) |
| EC-010 | `legacy-bash-adapter.wasm`-hosted plugin's script completes within the calibrated `timeout_ms` (no exhaustion) | Clean pass; PC12 not exercised (no kill event) — same as EC-001 baseline |
| EC-011 | `legacy-bash-adapter.wasm`-hosted plugin's script runs LONGER than the calibrated `timeout_ms` but SHORTER than the hardcoded 60,000 ms constant (e.g., `timeout_ms=30_000`, script runs 45 s) — the highest-risk AMD-002/AMD-003 defect window | **Pre-wiring-fix (current implementation) — CORRECTED (F-002; prior "silent false clean-pass at 45s" characterization was WRONG):** the outcome is NONDETERMINISTIC, not a deterministic clean pass. Because `run_bash_via_host` passes the hardcoded `BASH_TIMEOUT_MS=60_000` constant to `exec_subprocess.rs::run()` regardless of the registry's calibrated `timeout_ms=30_000`, the subprocess is not killed at 30 s — but the dispatch surfaces as EITHER (a) `PluginResult::Ok { exit_code: 1, .. }` via `adapter_logic`'s `HookResult::Error` mapping (if the bash script, `exec_subprocess`, or the adapter's own marshaling encounters any error condition), OR (b) a guest-epoch `PluginResult::Timeout { cause: TimeoutCause::Epoch }` race on control-return (if wasmtime's independent `EPOCH_TICK_MS` guest-interruption ticker fires against the adapter's own WASM execution once control returns from the blocking host call). In BOTH sub-cases the calibrated 30 s budget is silently violated with no fail-closed enforcement reaching `on_error=Block` under PC1-PC12's pre-§AMD-003 rules. **Post-wiring-fix + PC12 + PC13 (§AMD-002 + §AMD-003 combined):** PC12 fixes the kill timing (killed at ≈30 s); whichever outcome results — `PluginResult::Ok { exit_code: 1, .. }` (via `HookResult::Error`) or `PluginResult::Timeout { .. }` — MUST produce a block under `on_error=Block`: PC13 closes the `Ok{exit_code != 0}` case, PC4/PC10 already close the `Timeout`/`Crashed` case. The nondeterminism in WHICH outcome surfaces no longer has a safety impact — every sub-case now blocks. |

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
| `hooks-registry.toml` entry with `plugin="hook-plugins/legacy-bash-adapter.wasm"` + `failure_policy="fail-closed"` + `timeout_ms=10_000` (current live default for four of the five targeted bash-adapter plugins, strictly below the 30_000 host-wall-clock-timeout-axis floor) | `test_no_fail_closed_plugin_with_uncalibrated_cap` FAILS (CI blocks the half-state — fuel_cap alone does not calibrate the bash subprocess's actual exhaustion axis) | half-state-rejected (PC8 TIMEOUT-POSITIVE-CONTROL, F-S2111-P13-001, ADR-039 §Decision 4 v1.8) |
| `hooks-registry.toml` entry with `plugin="hook-plugins/legacy-bash-adapter.wasm"` + `failure_policy="fail-closed"` + `timeout_ms=45_000` (>= 30_000 host-wall-clock-timeout-axis floor, calibrated) | `test_no_fail_closed_plugin_with_uncalibrated_cap` PASSES / does not fire (gate accepts valid calibrated entry) | negative-control (PC8 TIMEOUT-NEGATIVE-CONTROL, F-S2111-P13-001, ADR-039 §Decision 4 v1.8) |
| `Timeout { cause: Fuel\|Epoch }` + `on_error=Block` + `FailOpen` (revision of `fail_closed_timeout_with_on_error_block` sub-case a) | Decision returns `false`; exit 0 — exhaustion governed by `failure_policy=FailOpen`; `on_error=Block` does not apply to exhaustion | axes-independence-on_error_block-fail-open (PC10a) |
| `Timeout { cause: Fuel\|Epoch }` + `on_error=Block` + `FailClosed` (revision of `fail_closed_timeout_with_on_error_block` sub-case b) | Decision returns `true`; exit 2 — exhaustion governed by `failure_policy=FailClosed`; block caused by failure_policy, not on_error | axes-independence-on_error_block-fail-closed (PC10b) |
| TC-12 revised: `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` (`full_stack_plugin_invocation.rs`) — `on_error=Block` + `failure_policy=FailOpen` (registry default) + `Timeout{Epoch}` (integration dispatch) | Observed dispatcher exit code 0 — axes-independent semantics: exhaustion governed by `failure_policy=FailOpen`; `on_error=Block` does not apply to exhaustion outcomes (revision of TC-12 per PC10 / F-S2111-P11-001; TD-VSDD-059 deliberate revision) | integration-mirror-fail-open (PC10 TC-12 revision) |
| TC-12 symmetric arm (SHOULD): `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` — `on_error=Block` + `failure_policy=FailClosed` + `Timeout{Epoch}` (integration dispatch) | Observed dispatcher exit code 2 — exhaustion governed by `failure_policy=FailClosed`; symmetric behavioral coverage per Invariant 6 / Envoy #38801 discipline (integration-layer fail-closed path verified at observed-outcome level, not merely configuration assertion) | integration-mirror-fail-closed-symmetric (PC10 TC-12 symmetric arm) |
| Synthetic enforcement-active executor-source snippet (any block-decision site in `execute_tier`/`execute_tiers`/helpers references `.failure_policy` for `Timeout` outcome, however the data reaches it) + synthetic registry MISSING one of the five on_error=block `failure_policy="fail-closed"` annotations | `test_no_on_error_block_without_fail_closed_when_3arg_executor` FAILS (gate fires RED; POSITIVE-CONTROL: proves non-vacuity — detector fires on bad intermediate CWE-636 state; data-flow-independent detection) | migration-window-gate (PC11 POSITIVE-CONTROL) |
| Synthetic enforcement-active executor-source snippet (any block-decision site references `.failure_policy` for `Timeout` outcome) + synthetic registry with all five on_error=block plugins annotated `failure_policy="fail-closed"` | `test_no_on_error_block_without_fail_closed_when_3arg_executor` PASSES (Phase 4 complete state; NEGATIVE-CONTROL: gate does not false-positive on valid fully-annotated registry) | migration-window-pass (PC11 NEGATIVE-CONTROL) |
| Synthetic enforcement-ABSENT executor-source snippet (no `.failure_policy` reference in block-decision chain for `Timeout` outcome) + any synthetic registry state | Gate returns GREEN AND detector's enforcement-detection logic ran and returned `EnforcementAbsent` (tri-state diagnostic; RED-emission skipped as consequence; VACUITY-CONTROL: distinguishes genuine Phase-1/2 GREEN from vacuous GREEN caused by detector that never ran enforcement-detection logic) | migration-window-vacuity (PC11 VACUITY-CONTROL) |
| Live tree: actual `crates/factory-dispatcher/src/executor.rs` at Phase-4-complete (enforcement-active code present) | Detector returns `enforcement_active = true` (LIVE-TREE-CONTROL: proves detector fires against real enforcement code, not only synthetic snippets; closes CWE-636 false-green gap where a wrong detector passes synthetic controls yet is inert on real code per F-S2111-P6-002) | migration-window-live-tree (PC11 LIVE-TREE-CONTROL) |
| `legacy-bash-adapter.wasm` entry `timeout_ms=2_000`, bash script sleeps 10 s (CURRENT pre-fix code path — `run_bash_via_host` passes hardcoded `BASH_TIMEOUT_MS`) | Subprocess killed at ≈60_000 ms; registry `timeout_ms` value ignored | AMD-002-defect-baseline (PC12 NEGATIVE reference, F-S2111-P13-001 wiring gap) |
| `legacy-bash-adapter.wasm` entry `timeout_ms=2_000`, bash script sleeps 10 s (POST-wiring-fix: `run_bash_via_host` passes registry `timeout_ms`) | Subprocess killed at ≈2_000 ms + ~5 ms poll tolerance; `Timeout{cause: Epoch}` observed at that time; `test_legacy_bash_adapter_honors_registry_timeout_ms` PASSES | AMD-002-wiring-fixed (PC12 POSITIVE control) |
| `legacy-bash-adapter.wasm` entry `timeout_ms=30_000`, bash script runs 45 s (CURRENT pre-fix code path) | NONDETERMINISTIC pre-fix outcome: EITHER `PluginResult::Ok{exit_code:1}` via `HookResult::Error` OR a guest-epoch `Timeout{cause:Epoch}` race on control-return — NOT a deterministic clean pass (EC-011 corrected, F-002); calibrated 30 s budget silently violated in both sub-cases under PC1-PC12's pre-§AMD-003 rules | AMD-002/AMD-003-nondeterministic-defect (EC-011, PC12+PC13) |
| `on_error=Block` + `PluginResult::Ok { exit_code: 1, .. }` (synthetic `PluginOutcome`) | Decision function returns `true`; `block_intent=true`; exit 2 — closes F-001 (bash-adapter timeout surfacing as `Ok{exit:1}`) and F-005 (any other `HookResult::Error` exit path) uniformly | plugin-error-fail-closed (PC13 POSITIVE control, ADR-039 §AMD-003) |
| `on_error=Block` + `PluginResult::Ok { exit_code: 0, .. }` (synthetic `PluginOutcome`) | Decision function returns `false` via this rule; exit 0 — a genuinely clean pass is unaffected by PC13 | plugin-error-fail-closed-negative (PC13 NEGATIVE control 1) |
| `on_error=Continue` + `PluginResult::Ok { exit_code: 1, .. }` (synthetic `PluginOutcome`) | Decision function returns `false` via this rule; exit 0 — PC13 is additive to `on_error=Block` only; pre-existing fail-open-on-crash-when-`on_error=Continue` semantics preserved unchanged | plugin-error-fail-closed-negative (PC13 NEGATIVE control 2) |

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

- `crates/factory-dispatcher/src/executor.rs` — enforcement-active decision path: `plugin_fail_closed`
  extended to accept `failure_policy: FailurePolicy`, OR a replacement function (e.g.
  `plugin_exhaustion_fail_closed`) — either way, a `PluginOutcome` type carries a
  `failure_policy: FailurePolicy` field and `execute_tiers` consults it for block decisions; for
  `Timeout { cause: TimeoutCause::Fuel | TimeoutCause::Epoch }`, returns `true` when
  `failure_policy == FailurePolicy::FailClosed` regardless of `on_error`;
  `fail_closed_timeout_with_on_error_continue_is_open` test MUST be revised (not deleted) to
  assert `Timeout + Continue + FailOpen → NOT block` (PC7);
  `fail_closed_timeout_with_on_error_block` test MUST ALSO be revised (not deleted) — per
  PC10, the revised test asserts both sub-cases: (a) `Timeout + on_error=Block + FailOpen →
  NOT block` (exit 0) and (b) `Timeout + on_error=Block + FailClosed → block` (exit 2);
  both sibling tests (`on_error_continue` and `on_error_block`) require the same axes-
  independence treatment; TD-VSDD-059 applies to both;
  `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` (TC-12) in
  `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs` is the integration-level
  mirror of `fail_closed_timeout_with_on_error_block` and MUST ALSO be revised (not deleted,
  per TD-VSDD-059) to assert exit 0 for the `on_error=Block + failure_policy=FailOpen + Timeout{Epoch}`
  case (F-S2111-P11-001); SHOULD additionally carry a `failure_policy=FailClosed` arm asserting
  exit 2 for symmetric behavioral coverage (Invariant 6 / Envoy #38801); both revisions MUST
  appear in the PR diff;
  **PC13 extension (ADR-039 §AMD-003, RATIFIED v1.11):** the block-decision function's
  predicate MUST additionally return `true` when `on_error == OnError::Block` AND `result` is
  NOT `PluginResult::Ok { exit_code: 0, .. }` — this closes the `PluginResult::Ok { exit_code: 1 }`
  gap left open by the `Crashed | Timeout` matching above; the fix is a single predicate change
  at the same decision-site this Architecture Anchor already governs
- `crates/hook-sdk/src/result.rs::HookResult::exit_code` — PC13's cited mapping:
  `HookResult::Error { .. } => 1`; this is the exit code an `adapter_logic` error path (F-001's
  bash-adapter timeout via `Err(codes::TIMEOUT)`, or F-005's any-other-error path) produces,
  which surfaces at the executor as `PluginResult::Ok { exit_code: 1, .. }` via
  `classify_trap`'s `Err(I32Exit(1))` arm — the shape PC13 closes
- `plugins/vsdd-factory/hooks-registry.toml` — six targeted plugin entries receive calibrated
  `failure_policy = "fail-closed"` atomically per-plugin, with the calibrated field(s)
  determined by adapter class (ADR-039 §Decision 1/4 v1.8 amendment; F-S2111-P13-001): the
  native-WASM plugin (`validate-cross-site-correspondence`) needs `fuel_cap >= 50M`; the five
  `legacy-bash-adapter.wasm`-hosted plugins (`validate-factory-path-root`,
  `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`,
  `validate-pr-merge-prerequisites`) need BOTH `fuel_cap >= 50M` AND `timeout_ms >= 30_000`
  (the host-wall-clock-timeout-axis floor — `fuel_cap` alone does not calibrate their actual exhaustion axis,
  since the bash subprocess is invisible to the WASM fuel counter per ADR-042 §Decision 3
  class (b)). 50_000_000 / 30_000 are the inclusive floors; values below are rejected by PC8;
  Phase 4 annotations land ONLY after Phase 3 calibration completes for every axis a plugin's
  adapter class is subject to; PC8 gate test enforces no half-state on EITHER axis
  (fuel_cap-only calibration is standing regression/invariant gate; timeout_ms calibration for
  `legacy-bash-adapter.wasm` entries added F-S2111-P13-001; migration-window ordering enforced
  by PC11); PC11 gate test enforces no on_error=block targeted plugin at fail-open while
  executor is enforcement-active (detected via any block-decision site in
  `execute_tier`/`execute_tiers`/helpers referencing `.failure_policy` for `Timeout` outcome,
  however the data reaches it — data-flow-independent per F-S2111-P5-002; gate includes
  POSITIVE/NEGATIVE/VACUITY/LIVE-TREE controls per F-S2111-P5-001 and F-S2111-P6-002)
- `crates/factory-dispatcher/src/registry.rs` — `FailurePolicy` enum and
  `RegistryEntry.failure_policy` field (delivered by S-21.10 / BC-1.01.016); executor reads
  `failure_policy` from the dispatched `RegistryEntry`
- ADR-039 §Decision 3 — safe migration ordering: Phase 1 (schema) → Phase 2 (mitigations) →
  Phase 3 (calibration, bifurcated by adapter class per v1.8 amendment) → Phase 4 (enforcement
  flip); no half-state at any CI-passing commit on EITHER calibration axis
- ADR-039 §Decision 6 — four required behavioral test scenarios: Timeout+FailClosed→block
  (PC1); Timeout+FailOpen→advisory (PC2); on_error independence (PC3); crash≠exhaustion
  distinct paths (PC4+PC5)
- ADR-039 §Decision 1/2/4 v1.8 amendment (§AMD-001; F-S2111-P13-001) — fuel-vs-host-wall-clock-timeout axis
  bifurcation: `fuel_cap` calibration is genuinely sufficient only for the native-WASM plugin
  (`validate-cross-site-correspondence`); the five `legacy-bash-adapter.wasm`-hosted plugins
  additionally require `timeout_ms` calibration per the new host-wall-clock-timeout-axis formula
  (`timeout_ms >= max(measured_p99_ms × 2.0, 30_000)`), because their bash subprocess is
  invisible to the WASM fuel counter
- ADR-042 §Decision 3 class (b) — evidentiary basis for the v1.8 amendment: "fuel exhaustion
  occurs before the WASI `exec_subprocess` call, the bash script body never executes when the
  adapter is fuel-starved"; confirms `fuel_cap` cannot meter bash subprocess execution time
  for `legacy-bash-adapter.wasm`-hosted plugins
- `crates/hook-plugins/legacy-bash-adapter/src/lib.rs::run_bash_via_host` — PC12's wiring-fix
  target: this call site currently passes its own hardcoded `BASH_TIMEOUT_MS = 60_000`
  constant to the host `exec_subprocess` call; the fix requires it to instead pass the
  invoking `RegistryEntry`'s calibrated `timeout_ms` value (ADR-039 §AMD-002, RATIFIED v1.10)
- `crates/factory-dispatcher/src/host/exec_subprocess.rs::run()` — the actual host
  enforcement point for the bash subprocess wall-clock bound: an `Instant`-based deadline
  loop (`std::thread::sleep(Duration::from_millis(5))` poll interval) that calls
  `child.kill()` on overrun. PC12 asserts against THIS enforcement point, not wasmtime's
  `EPOCH_TICK_MS` (10 ms, `crates/factory-dispatcher/src/engine.rs`) — the two are distinct
  and unrelated mechanisms (ADR-039 §Decision 4 v1.9 mechanism-precision correction; do not
  conflate the adapter's poll interval with the epoch ticker)

## Story Anchors

- S-21.10 (prerequisite: Phase 1 schema extension; BC-1.01.016)
- S-21.11 (Phase 3 calibration + Phase 4 enforcement flip)
- S-21.13 (EC-004 follow-up for `validate-cross-site-correspondence` only: targeted-row lookup eliminating O(n) fuel ceiling; depends_on [S-21.10, S-21.11]; scoped exclusively to the on_error=continue plugin; on_error=block plugins are NOT routed here per EC-004 amendment v1.4; MUST include explicit mandate to annotate `validate-cross-site-correspondence` `failure_policy="fail-closed"` once O(n) fuel ceiling is removed — annotation-landing obligation per F-S2111-P5-005)

## VP Anchors

- VP-TBD — failure_policy enforcement dispatch: all six postconditions exercised by behavioral
  tests driving the actual dispatch path; half-state structural gate; all six targeted
  validators carry `failure_policy="fail-closed"` with calibrated `fuel_cap >= 50M`
  (inclusive floor); migration-window completeness gate (PC11)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | For resource-exhaustion outcomes (`TimeoutCause::Fuel`, `TimeoutCause::Epoch`): `failure_policy=FailClosed` → block (exit 2); `failure_policy=FailOpen` → advisory (exit 0); `on_error` does not override `failure_policy` for exhaustion; crash (`PluginResult::Crashed`) is governed by `on_error` only; no `failure_policy="fail-closed"` entry in `hooks-registry.toml` without `fuel_cap >= 50M` (inclusive calibration floor per ADR-039 §Decision 4; `fuel_cap < 50M` is prohibited; `fuel_cap = 50M` is VALID); no `legacy-bash-adapter.wasm`-hosted entry with `failure_policy="fail-closed"` without ALSO `timeout_ms >= 30_000` (inclusive host-wall-clock-timeout-axis calibration floor per ADR-039 §Decision 4 v1.8 amendment / §AMD-001; F-S2111-P13-001 — `fuel_cap` sufficiency alone does not calibrate this adapter class's actual exhaustion axis, since the bash subprocess is invisible to the WASM fuel counter); all targeted validators carry `failure_policy="fail-closed"` with calibration sufficient for their adapter class (native-WASM: `fuel_cap >= 50M`; `legacy-bash-adapter.wasm`-hosted: `fuel_cap >= 50M` AND `timeout_ms >= 30_000`); `fail_closed_timeout_with_on_error_block` test revised (not deleted) to assert both `on_error=Block + FailOpen → NOT block` and `on_error=Block + FailClosed → block` (PC10; TD-VSDD-059); integration-level mirror `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` (TC-12, `full_stack_plugin_invocation.rs`) ALSO revised (not deleted) to assert `on_error=Block + failure_policy=FailOpen + Timeout{Epoch} → exit 0`, with SHOULD arm for `failure_policy=FailClosed → exit 2` (symmetric behavioral coverage per Invariant 6 / Envoy #38801; both revisions MUST appear in PR diff; F-S2111-P11-001); PC8 gate test includes both POSITIVE-CONTROL (fail-closed + fuel_cap=20M < 50M floor → RED) and NEGATIVE-CONTROL (fail-closed + fuel=75M → PASS; and fuel=50M → PASS) fixtures (POLICY 15); PC11 gate test asserts that if the executor is enforcement-active (detected via any block-decision site in `execute_tier`/`execute_tiers`/helpers referencing `.failure_policy` for `Timeout` outcome, however the data reaches it — data-flow-independent per F-S2111-P5-002; fires for both extend-in-place and introduce-a-replacement designs), all five on_error=block targeted plugins carry failure_policy=fail-closed (CWE-636 static gate; EC-004 descope does not reduce this assertion set); PC11 gate includes POSITIVE-CONTROL (enforcement-active snippet + registry missing one annotation → RED), NEGATIVE-CONTROL (enforcement-active snippet + all five annotated → PASS), VACUITY-CONTROL (enforcement-absent snippet → GREEN AND enforcement-detection logic ran and returned `EnforcementAbsent` with RED-emission skipped as consequence), and LIVE-TREE-CONTROL (detector run against actual `crates/factory-dispatcher/src/executor.rs` at Phase-4-complete → `enforcement_active = true`; closes CWE-636 false-green gap per F-S2111-P6-002) per F-S2111-P5-001 and F-S2111-P6-002. **PC12 (ADR-039 §AMD-002, RATIFIED v1.10):** for any `legacy-bash-adapter.wasm`-hosted registry entry with `timeout_ms = X`, a real dispatch against a bash subprocess running longer than `X` MUST be killed at ≈`X` (within the ~5 ms `exec_subprocess.rs::run()` poll tolerance), not at the hardcoded `BASH_TIMEOUT_MS = 60_000` constant; includes a POSITIVE control (`timeout_ms=2_000`, script sleeps 10 s → killed ≈2 s) and documents the NEGATIVE pre-fix reference (killed ≈60 s regardless of registry `timeout_ms`) as the AMD-002 defect baseline. **PC13 (ADR-039 §AMD-003, RATIFIED v1.11):** for any plugin dispatched with `on_error = OnError::Block`, `result = PluginResult::Ok { exit_code != 0, .. }` MUST cause the decision function to return `true` (block, exit 2), regardless of `failure_policy`; includes a POSITIVE control (`on_error=Block` + `Ok{exit_code:1}` → block) and two NEGATIVE controls (`on_error=Block` + `Ok{exit_code:0}` → no block; `on_error=Continue` + `Ok{exit_code:1}` → unaffected); closes both F-001 (bash-adapter timeout surfacing as `Ok{exit:1}`) and F-005 (any other `HookResult::Error` exit path), ruled in-scope together per §AMD-003. | unit tests (executor path coverage per PC1–PC6, PC10, PC13) + integration/bats test (real dispatch at `fuel_cap=100` → exit 2; PC1 behavioral) + Cargo gate tests (hooks-registry.toml parse; PC8 with both controls; PC11 migration-window gate with four controls) + Cargo integration test driving the real `legacy-bash-adapter` subprocess path with a short calibrated `timeout_ms` and a long-running script (`test_legacy_bash_adapter_honors_registry_timeout_ms`; PC12, Envoy #38801 discipline — behavioral, not configuration, assertion) + unit test asserting `on_error=Block` + `PluginResult::Ok{exit_code!=0}` → `block_intent=true` (`test_on_error_block_fails_closed_on_plugin_error_exit_code`; PC13) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-011 |
| Capability Anchor Justification | CAP-011 ("Enforce fuel and epoch budgets on plugin execution") per capabilities.md §CAP-011 — CAP-011's stated outcome is "a runaway plugin is killed within `timeout_ms` + `EPOCH_TICK_MS` (10ms)... never hung processes." This BC's `failure_policy` enforcement dispatch (block-vs-advisory on `TimeoutCause::Fuel`/`TimeoutCause::Epoch`) is the ENFORCEMENT half of CAP-011 (detection is CAP-011/BC-1.03.002; PC1–PC11 govern what the dispatcher DOES once a budget is exceeded). PC12 (v1.12) extends this same capability anchor one step further: it asserts CAP-011's "killed within `timeout_ms`" guarantee is genuinely met at the runtime level for `legacy-bash-adapter.wasm`-hosted plugins — i.e., that the value CAP-011 promises to enforce is the registry's calibrated `timeout_ms`, not an unwired hardcoded constant (ADR-039 §AMD-002). No new/different capability is needed for PC12: it is the same "runaway plugin killed within budget" outcome, verified at the wiring layer rather than the decision layer. |
| L2 Domain Invariants | TBD (no CAP-011-adjacent DI-NNN currently exists in `domain-spec/invariants.md`; DI-002/DI-003 govern adjacent executor-tier semantics — plugin crash/timeout isolation and `block_intent` aggregation — but do not themselves assert the fuel/epoch-budget enforcement decision this BC governs) |
| Architecture Module | SS-01 (Hook Dispatcher Core) — `crates/factory-dispatcher/src/executor.rs`; enforcement dispatch for resource-exhaustion outcomes; PC12 additionally anchors `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` and `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| ADR | ADR-039 §Decision 1 (axes separation: exhaustion vs crash; v1.8 amendment: fuel-vs-host-wall-clock-timeout signal bifurcation by adapter class); ADR-039 §Decision 2 (validator-class plugins use `fail-closed` after calibration; v1.8 amendment: native-WASM vs `legacy-bash-adapter.wasm` scope split); ADR-039 §Decision 3 (safe migration ordering; Phase-3-before-Phase-4 atomicity; half-state forbidden on both axes; v1.8 amendment: explicit self-lock statement for the two PreToolUse `^Agent$` gates; v1.9 amendment: mandatory authenticated break-glass companion, S-21.17); ADR-039 §Decision 4 (p99×1.5 fuel-cap calibration, Option A minimum requirement, 50M floor for native-WASM plugins; v1.8 amendment: parallel p99_ms×2.0 `timeout_ms` host-wall-clock-timeout-axis formula, 30_000ms floor, for `legacy-bash-adapter.wasm`-hosted plugins; v1.9: reframed as local calibration policy, not an SRE-standard formula); ADR-039 §Decision 6 (four behavioral test scenarios; Envoy #38801 lesson — behavioral tests not configuration tests); ADR-039 §AMD-001 (v1.8; F-S2111-P13-001 amendment record; RATIFIED 2026-08-18 v1.9 under POLICY 22); ADR-039 §AMD-002 (v1.9; architect self-verification finding — `legacy-bash-adapter`'s bash-subprocess kill deadline is a fixed `BASH_TIMEOUT_MS=60_000` constant independent of the registry's calibrated `timeout_ms`; blast radius: all ~37 `legacy-bash-adapter.wasm`-routed registry entries, not only the five §Decision 2 plugins; RATIFIED 2026-08-19 v1.10 under POLICY 22, corroborated by independent code-review verification against live source — the prior "ADR-025 §Decision 18" corroboration citation was wrong and is retracted; genuine corroboration is ADR-039's own v1.8 §AMD-001 → v1.9 §Decision 4 mechanism-precision self-correction history; wiring-fix remediation delivered WITHIN S-21.11, not a separate follow-up story — **PC12 (v1.12) is this BC's behavioral-test leg for §AMD-002's wiring-fix obligation**); §Decision 3 v1.9 break-glass amendment (mandatory authenticated bypass for the two PreToolUse `^Agent$` gates; minimum-viable definition — environment-variable override, human-operator-only, audited via JSONL — specified in §Decision 3 v1.10 amendment; delivered WITHIN S-21.11, intra-story ordering: break-glass commit precedes or is atomic with the fail-closed-flip commit for the two named gates; **governed by sibling BC-1.03.018, not this BC** — see BC-1.03.018 for the break-glass behavioral contract); ADR-039 §AMD-003 (v1.11, RATIFIED — S-21.11 v2.0 adversarial pass-1 BLOCKER F-S2111V2-P1-001: `on_error = "block"` does not fail-closed on a plugin's own reported `HookResult::Error` (`PluginResult::Ok { exit_code: 1 }`); `plugin_fail_closed` extended per the precise rule `on_error == OnError::Block AND result is NOT PluginResult::Ok { exit_code: 0, .. } => block`; F-005 generic-error path ruled in-scope alongside F-001's timeout-specific leg — **PC13 (v1.13) is this BC's behavioral-test leg for §AMD-003's enforcement rule**); ADR-042 §Decision 3 class (b) (evidentiary basis: bash subprocess execution is invisible to the WASM fuel counter) |
| Security | CWE-636 (Not Failing Securely — closed for six validator-class WASM plugins after Phase 4); CWE-390 (Detection of Error Condition Without Action — closed for enforcement path). Research basis: `.factory/research/wasm-fuel-exhaustion-detection.md` |
| Stories | S-21.10 (prerequisite), S-21.11, S-21.13 (EC-004 follow-up for `validate-cross-site-correspondence` only; on_error=block plugins are NOT routed here per EC-004 amendment v1.4; MUST annotate `validate-cross-site-correspondence` fail-closed once O(n) fuel ceiling removed per F-S2111-P5-005) |
| Cycle | v1.0-brownfield-backfill (E-21 Wave 6) |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.13 | 2026-08-19 | product-owner | S-21.11 v2.0 adversarial pass-1 remediation (F-S2111V2-P1-001-mechanism-adjudication memo; ADR-039 §AMD-003 RATIFIED v1.11): new PC13 asserts the §AMD-003 rule — a plugin dispatched with `on_error = OnError::Block` whose outcome is `PluginResult::Ok { exit_code != 0, .. }` MUST be treated as a block (`block_intent=true`, exit 2), regardless of `failure_policy`; unifies F-001 (bash-adapter host-wall-clock timeout surfacing as `HookResult::Error` → exit 1, via `Err(codes::TIMEOUT)` → `run_bash_via_host`'s error map → `adapter_logic`'s `HookResult::error(...)` → `classify_trap`'s `Err(I32Exit(1))` arm → `PluginResult::Ok{exit_code:1}`) and F-005 (any other generic `HookResult::Error` exit path), both ruled in-scope per §AMD-003's F-005 ruling. POSITIVE control (`on_error=Block` + `Ok{exit!=0}` → block) + two NEGATIVE controls (`on_error=Block` + `Ok{exit==0}` → no block; `on_error=Continue` + `Ok{exit!=0}` → unaffected). New Invariant 10 codifies PC13 as a strict superset of the pre-existing `Crashed \| Timeout` rule (does not remove any existing block path). Three new PC13 Canonical Test Vector rows. EC-011 corrected (F-002): replaced the wrong "silent false clean-pass at 45s" pre-fix characterization with the accurate nondeterministic pre-fix behavior (`PluginResult::Ok{exit_code:1}` via `HookResult::Error` OR a guest-epoch `Timeout` race on control-return) and the post-fix guarantee (PC12 kill timing + PC13 `Ok{exit!=0}`→block together close every sub-case). Architecture Anchors extended: `executor.rs` bullet gets a PC13 extension clause; new bullet cites `crates/hook-sdk/src/result.rs::HookResult::exit_code`. Traceability ADR row extended with a new §AMD-003 citation alongside §AMD-001/§AMD-002. VP-TBD property extended with the PC13 rule and its unit test (`test_on_error_block_fails_closed_on_plugin_error_exit_code`). H1 enriched with the §AMD-003 leg and PC13 clause per POLICY 7. PC count extended PC1-PC12 → PC1-PC13 (additive-only; no renumbering). Does NOT touch S-21.11's story body/ACs (story-writer's domain, dispatched separately) and does NOT alter PC1-PC12's existing semantics. BC-1.03.017 v1.13. |
| v1.12 | 2026-08-19 | product-owner | S-21.11 expanded-scope BC coverage burst (orchestrator-directed, scoped to the AMD-002 runtime-wiring gap only): new PC12 asserts the RUNTIME behavior AMD-002 (RATIFIED v1.10) identified as unwired — for `legacy-bash-adapter.wasm`-hosted plugins, the actual bash-subprocess kill deadline (`exec_subprocess.rs::run()`) MUST equal the registry's calibrated `timeout_ms`, not the hardcoded `BASH_TIMEOUT_MS=60_000` constant in `run_bash_via_host`; POSITIVE control (short `timeout_ms` kills early) + NEGATIVE reference (current pre-fix 60s-regardless-of-config defect state) + new EC-011 (highest-risk silent-false-pass window: script duration between calibrated `timeout_ms` and the hardcoded 60s). New Precondition 5 states the config-vs-runtime assumption gap. PC9 amended (additive) with a PC12-dependency clause. New Invariant 9 codifies the config-vs-runtime wiring bifurcation. Two new PC12 Canonical Test Vector rows + one EC-011 vector. Architecture Anchors extended to cite `legacy-bash-adapter/src/lib.rs::run_bash_via_host` and `exec_subprocess.rs::run()`'s 5ms poll loop, explicitly distinguished from the unrelated wasmtime `EPOCH_TICK_MS`=10ms guest-epoch ticker (ADR-039 §Decision 4 v1.9 mechanism-precision correction). Traceability `L2 Capability` resolved `CAP-TBD` → `CAP-011` ("Enforce fuel and epoch budgets on plugin execution") with new S-7.01 Capability Anchor Justification row (capabilities.md §CAP-011 verbatim cite). ADR row cross-references BC-1.03.018 (new sibling BC) for the break-glass mechanism, which is NOT governed by this BC. PC count extended PC1-PC11 → PC1-PC12 (additive-only; no renumbering). Does NOT touch S-21.11's story body/ACs (story-writer's domain) or alter PC1-PC11's existing semantics. BC-1.03.017 v1.12. |
| v1.11 | 2026-08-19 | architect | Sibling-sweep citation update (TD-VSDD-060; parallel to ADR-039 v1.9→v1.10, same burst; two human decisions this session, POLICY 22 ratification-channel): Traceability ADR row updated — §AMD-002 now cites RATIFIED (2026-08-19, v1.10) instead of PROPOSED/NOT RATIFIED, with the corrected corroboration basis (ADR-039's own v1.8 §AMD-001 → v1.9 §Decision 4 mechanism-precision self-correction; the prior "ADR-025 §Decision 18" citation was wrong — that ADR concerns the unrelated factory-artifacts lock/lease decision — and is retracted) and the reframed blast radius (~37 legacy-bash-adapter.wasm-routed registry entries affected structurally, not only the five §Decision 2 plugins targeted by S-21.11's fail-closed flip); §Decision 3 break-glass citation redirected from named follow-up S-21.17 to S-21.11 (human decided S-21.11 is NOT split — it absorbs break-glass, per-plugin timeout_ms calibration, the AMD-002 wiring fix, and the gated fail-closed flip as one unified story); AMD-002's own named follow-up S-21.18 likewise redirected to S-21.11; intra-story ordering constraint noted (break-glass commit precedes or is atomic with the fail-closed-flip commit for validate-wave-gate-prerequisite and validate-pr-merge-prerequisites). Citation-only sweep: no PC/Precondition/Invariant/AC content altered; PC count unchanged at PC1-PC11; Stories row unaffected (already cited S-21.11, not S-21.17/S-21.18). BC-1.03.017 v1.11. |
| v1.10 | 2026-08-18 | architect | F-S2111-P13-001 research-corrections fold-in (parallel to ADR-039 v1.9): swept prose terminology "epoch axis"/"epoch-axis floor"/"epoch mechanism" → "host wall-clock timeout axis" throughout Preconditions 2/3, PC8, PC9, Canonical Test Vectors, Architecture Anchors, VP-TBD, and Traceability (ADR-039 §Decision 1's technical premise, independently research-validated, is that wasmtime's `epoch_interruption` feature — like fuel — cannot bound a host-blocking subprocess call; the correct label is a dispatcher/host-enforced wall-clock timeout). `timeout_ms` field name and `TimeoutCause::Epoch`/`Timeout{Epoch}` Rust code identifiers left unchanged (literal code, not prose). Traceability ADR row updated: §AMD-001 now cites RATIFIED (2026-08-18, v1.9) instead of PENDING; new §AMD-002 cite added (architect self-verification finding, PROPOSED / NOT RATIFIED — `legacy-bash-adapter`'s bash-subprocess kill deadline is a fixed `BASH_TIMEOUT_MS=60_000` constant independent of the registry's calibrated `timeout_ms`; named follow-up S-21.18, new, not yet authored); §Decision 3 v1.9 break-glass amendment cited (S-21.17, new, not yet authored). No PC/Precondition/Invariant semantics altered; PC count unchanged at PC1-PC11. BC-1.03.017 v1.10. |
| v1.9 | 2026-08-18 | architect | F-S2111-P13-001 remediation (scoped architectural precondition/PC correction; ADR-039 §Decision 1/2/3/4 v1.8 amendment): Precondition 2/3 bifurcated by plugin adapter class — native-WASM plugin (`validate-cross-site-correspondence`) calibrates `fuel_cap` per the original formula; the five `legacy-bash-adapter.wasm`-hosted plugins (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`) additionally calibrate `timeout_ms` per the new epoch-axis formula (`timeout_ms >= max(measured_p99_ms × 2.0, 30_000)`) because their bash subprocess execution is invisible to the WASM fuel counter (ADR-042 §Decision 3 class (b)). PC8 extended with a parallel `timeout_ms` structural half-state assertion (TIMEOUT-POSITIVE-CONTROL / TIMEOUT-NEGATIVE-CONTROL added) for `legacy-bash-adapter.wasm` entries — `fuel_cap` sufficiency alone is no longer treated as complete calibration evidence for these five plugins. PC9 final-state criterion updated to require both axes per plugin's adapter class. New Invariant 8 codifies the axis-bifurcation principle and the self-lock consequence for the two PreToolUse `^Agent$` gates. Two new Canonical Test Vector rows added (PC8 `timeout_ms` POSITIVE/NEGATIVE controls). Architecture Anchors + VP-TBD + Traceability updated to cite ADR-039 v1.8 §AMD-001. PC count unchanged at PC1–PC11 (additive-only; no renumbering). **Residual product-owner BC-body edit noted:** this burst is scoped to the architectural precondition/PC correction only; it does NOT touch AC-to-PC narrative mapping in the S-21.11 story body (deferred to a post-ratification resume burst) and does NOT alter PC1–PC7/PC10/PC11's axes-independence or migration-window substance, which remain product-owner's domain for any further narrative refinement. BC-1.03.017 v1.9. |
| v1.8 | 2026-08-18 | product-owner | F-S2111-P11-001 remediation: extended PC10 to require deliberate revision (TD-VSDD-059) of BOTH the unit test `fail_closed_timeout_with_on_error_block` AND its integration-level mirror `test_e2e_BC_7_06_001_sync_hook_timeout_fail_closed_on_error_block` (TC-12, `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs`). TC-12 currently constructs `on_error=Block + failure_policy=FailOpen (registry default) + Timeout{Epoch}` and asserts `exit_code==2`; under the axes-independent semantics mandated by this BC (PC5/EC-009), that assertion is FALSE and must be revised to assert exit 0. TC-12 SHOULD additionally carry a `failure_policy=FailClosed → exit 2` arm for symmetric behavioral coverage (Invariant 6 / Envoy #38801 discipline). Both revisions MUST appear in the PR diff; deletion of either without an equivalent replacement is a TD-VSDD-059 paper-fix violation. Two TC-12 Canonical Test Vector rows added (`integration-mirror-fail-open`, `integration-mirror-fail-closed-symmetric`). Architecture Anchors updated to cite TC-12 and `full_stack_plugin_invocation.rs`. VP-TBD property updated to reference TC-12 integration mirror obligation and F-S2111-P11-001. PC count unchanged: PC1..PC11. PC10↔AC-011 mapping unchanged. |
| v1.7 | 2026-08-18 | product-owner | Adversary pass-8 remediation (two F-S2111-P8 findings): (1) F-S2111-P8-002 — raised Precondition 2 calibration corpus floor for `lessons.md` from ≥3000 to ≥4000 lines; aligns with D-442(e) hard limit (4000 lines) so Invariant 5's exit condition (calibration confirms `fuel_cap` sufficient for the hard limit) is structurally achievable; a 3000-line corpus structurally cannot confirm 4000-line sufficiency; framing at 3000 was numerically wrong given D-442(e) soft=3500/hard=4000. (2) F-S2111-P8-003 — updated PC10 sub-cases (a) and (b) from `cause: Fuel` to `cause: Fuel\|Epoch` for self-consistency with the PC10 header (which already states `Fuel\|Epoch`) and with epoch-parity requirement in PC6/AC-010; Canonical Test Vectors PC10a and PC10b rows updated to match. BC-1.03.017 v1.7. |
| v1.6 | 2026-08-17 | product-owner | Adversary pass-6 remediation (three F-S2111-P6 findings): (1) F-S2111-P6-002 — added LIVE-TREE-CONTROL (fourth control) to PC11: at Phase-4-complete the detector MUST run against actual `crates/factory-dispatcher/src/executor.rs` and return `enforcement_active = true`; closes CWE-636 false-green gap where a syntactically-wrong detector could pass all three synthetic controls yet be inert against real enforcement code; acceptable implementation mandates the POSITIVE-CONTROL snippet be a verbatim excerpt of the real `execute_tiers` block-decision site AND the detector returns `enforcement_active = true` on the live tree; PC11 controls preamble updated from "three controls" to "four controls"; Canonical Test Vectors LIVE-TREE-CONTROL row added; Architecture Anchors and VP-TBD updated to reference LIVE-TREE control and F-S2111-P6-002. (2) F-S2111-P6-003 — fixed PC11(c) VACUITY-CONTROL self-contradiction: removed "evaluated the annotation-check branch"/"correctly skipped" contradictory phrasing; rewritten to assert the detector's enforcement-detection logic ran and returned `EnforcementAbsent` (via explicit `detection_ran` / tri-state diagnostic), and that RED-emission was skipped as a consequence; Canonical Test Vectors VACUITY-CONTROL row updated to match. (3) F-S2111-P6-004 — corrected PC8 title to remove migration-window on_error=block ordering claim (ordering constraint is mechanically enforced by PC11, not PC8); added clarifying sentence in Symmetric half-state prohibition text cross-referencing PC11 as the authoritative mechanical gate for the ordering constraint; PC8's scope is now unambiguous: calibration gate only (no fail-closed without fuel_cap >= 50M). BC-1.03.017 v1.6. |
| v1.5 | 2026-08-17 | product-owner | Adversary pass-5 remediation (three F-S2111-P5 findings): (1) F-S2111-P5-001 — added POSITIVE/NEGATIVE/VACUITY non-vacuity controls to PC11 (parallel to PC8); controls structured as pure functions over injectable inputs (synthetic executor-source snippet + synthetic registry, NOT bound to live tree); POSITIVE-CONTROL: enforcement-active snippet + registry missing one of five on_error=block annotations → assert RED; NEGATIVE-CONTROL: enforcement-active snippet + all five annotated → assert PASS; VACUITY-CONTROL: enforcement-absent snippet → assert GREEN AND detector reached annotation-check branch (vacuous-GREEN distinguishable from real-GREEN); Canonical Test Vectors PC11 rows updated from 2 to 3 rows. (2) F-S2111-P5-002 — broadened PC11 enforcement-active detection signal from data-flow-coupled ("PluginOutcome carries failure_policy field AND execute_tiers consults it") to data-flow-independent ("any block-decision site in execute_tier/execute_tiers/helpers references .failure_policy value when deciding to block on Timeout outcome, however the data reaches it"); softened over-claimed "structurally impossible to merge" to "mechanically detectable at any single commit"; Architecture Anchors and VP-TBD updated to match. (3) F-S2111-P5-005 — EC-004 Case A now mandates S-21.13 (or named successor) MUST annotate validate-cross-site-correspondence failure_policy="fail-closed" once its O(n) fuel-ceiling algorithmic fix removes the excessive cap requirement (descope is timing deferral only, not permanent exemption); PC9 annotation-landing obligation clause added. BC-1.03.017 v1.5. |
| v1.4 | 2026-08-17 | product-owner | Adversary pass-4 remediation (three F-S2111-P4 findings — holistic PC11/AC-012 migration-window-gate axis closure): (1) F-S2111-P4-001 — decoupled PC11 enforcement-active detection from function name; replaced name-based `fn plugin_fail_closed(` pattern with data-anchored signal (`PluginOutcome` carrying `failure_policy: FailurePolicy` field + `execute_tiers` consulting it for block decisions); detection is name-independent and holds for both extend-in-place and introduce-a-replacement implementer paths; Canonical Test Vectors PC11 rows updated to match. (2) F-S2111-P4-002 — resolved EC-004/PC11 deadlock + mis-route: EC-004 now explicitly bifurcates on `on_error` value; for `on_error="block"` plugins EC-004 is NOT a valid descope path (annotate-within-S-21.11 OR block-the-flip are the only options); path (b) "record transient fail-open window" removed for on_error=block case; S-21.13 mis-route for on_error=block removed (S-21.13 is scoped to validate-cross-site-correspondence on_error=continue only); PC9 critical caveat updated to match; PC11 extended with EC-004 non-applicability clause for the five on_error=block plugins; Story Anchors S-21.13 annotation updated. (3) F-S2111-P4-003 — H1 enriched to include migration-window on_error=block completeness gate clause (POLICY 7: enrichment must go into H1, not live only downstream in story BC-table); BC-INDEX title cell must be swept to match (state-manager same-burst per POLICY 14 leg-5). |
| v1.3 | 2026-08-17 | product-owner | Adversary pass-3 remediation (two F-S2111-P3 findings): (1) F-S2111-P3-001 — reconciled 50M boundary to inclusive floor (>= 50_000_000 ACCEPT, < 50_000_000 REJECT) — atomic sibling sweep across PC8, PC9, Invariant 2, Invariant 7, Architecture Anchors, VP-TBD, and Canonical Test Vectors; POSITIVE-CONTROL fixture updated from fuel_cap=10_000_000 to fuel_cap=20_000_000 (factory default per ADR-042 §Decision 2, clearly below floor and realistic); added boundary-pass test vector asserting fuel_cap=50_000_000 PASSES (the calibration-formula minimum is now an inclusive ACCEPT). (2) F-S2111-P3-005 — PC11 added: hard migration-window completeness CI gate (test_no_on_error_block_without_fail_closed_when_3arg_executor) asserting that if the extended 3-arg plugin_fail_closed signature is present in executor.rs, every on_error="block" targeted plugin MUST carry failure_policy="fail-closed"; closes the CWE-636 static-gap left by Invariant 7's ordering rule (which was ordering-based, not commit-checkable); PC11 test vector added. |
| v1.2 | 2026-08-17 | product-owner | Adversary pass-2 remediation (five F-S2111-P2 findings): (1) F-S2111-P2-001 — PC8 extended with symmetric half-state prohibition: no on_error=block targeted plugin may remain at failure_policy=fail-open once the extended 3-arg plugin_fail_closed is in effect; Invariant 7 added codifying migration-ordering atomicity and naming the five at-risk plugins. (2) F-S2111-P2-002 — PC10 added: fail_closed_timeout_with_on_error_block MUST be deliberately revised (TD-VSDD-059) to assert axes-independent sub-cases (FailOpen→NOT block, FailClosed→block); Canonical Test Vectors and Architecture Anchors updated. (3) F-S2111-P2-003 — EC-004 extended: deferral NOT behavior-neutral for on_error=block plugins (CWE-636 regression if left at fail-open); two remediation paths enumerated (fallback gate OR hard-blocker on follow-up); PC9 annotated with cross-reference to EC-004 on_error=block consequence. (4) F-S2111-P2-004 — PC8 extended with NEGATIVE-CONTROL fixture (fuel_cap=75_000_000, >50M floor → gate must PASS/not fire), closing POLICY 15 single-outcome-control gap; Canonical Test Vectors updated. (5) F-S2111-P2-006 — EC-004 names S-21.13 as concrete follow-up story anchor (Canonical Principle Rule 3); Story Anchors updated. |
| v1.1 | 2026-08-17 | product-owner | Spec-review remediation (F-S2111 adversary + SR findings): (1) F-S2111-P1-001 — HookEntry→RegistryEntry in Precondition 1, Related BCs BC-1.01.016 bullet, and Architecture Anchors registry.rs bullet (×2); phantom struct — actual is `pub struct RegistryEntry` in registry.rs. (2) F-S2111-P1-003/SR-001 — PC8 reclassified as standing regression/invariant gate (green-when-empty, green-at-final-state, RED on bad half-state); POSITIVE-CONTROL fixture requirement added; red-first framing removed; PC9/AC-009 is the genuine red-first gate. (3) F-S2111-P1-004/SR-008 — PC8 + Invariant-2 + VP gate threshold raised 20M→50M (calibration floor ADR-039 §Decision 4; factory default 20M per ADR-042 §Decision 2 is below the floor). (4) SR-002 — EC-004 vs PC9 deadlock resolved: explicit descoping-to-flippable-subset via orchestrator-approved spec amendment added to EC-004 and PC9; PC9 now conditional on post-amendment set. (5) SR-004 — PluginResult::Error→PluginResult::Crashed in PC4 and Canonical Test Vectors; no Error variant in invoke.rs enum (variants: Ok, Timeout, Crashed). (6) F-S2111-P1-008 — Precondition 2 fixture citation corrected: phantom S-21.07 task #33 replaced with committed path (BC-INDEX.md at 576,396 bytes). |
| v1.0 | 2026-08-06 | product-owner | Initial creation (S-21.10/S-21.11 BC authoring burst; ADR-039 §Decision 3+6 Phase 4 enforcement leg; four behavioral test scenarios from Decision 6 as PC1–PC6; structural half-state gate PC8; six targeted validators PC9; CWE-636+CWE-390 closure). |
