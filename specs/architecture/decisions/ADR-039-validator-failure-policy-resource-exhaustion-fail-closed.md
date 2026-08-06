---
document_type: architecture-decision-record
level: L3
adr_id: ADR-039
version: "1.1"
title: "ADR-039: Validator failure policy for resource exhaustion — per-plugin failure_policy field, fail-closed default for authorization-class validators, and safe migration ordering"
status: proposed
date: 2026-08-06
producer: architect
timestamp: 2026-08-06T00:00:00Z
deciders:
  - architect
subsystems_affected: [SS-01, SS-05]
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
research_basis: .factory/research/wasm-fuel-exhaustion-detection.md
extends: ADR-035 §Decision 5
last_amended: |-
  2026-08-06 (v1.1) — Context + Decision 3 amended (architect; orchestrator observation):
  PostToolUse blocks on ARCH-INDEX.md during ADR authoring (2026-08-06) confirm the
  self-lock hazard is already live for today's fail-closed validators
  (validate-factory-path-root, validate-input-hash, validate-template-compliance —
  all on_error=block). Context "self-lock hazard" paragraph expanded with observational
  evidence; Decision 3 "observed evidence" paragraph added. Phase 3-before-Phase 4
  ordering constraint strengthened from precautionary to response-to-active-failure.
  [Prior: 2026-08-06 (v1.0) — Initial ruling (architect; S-21.07 pass-7 validator-failure-policy
  adjudication routed by orchestrator): CWE-636 fail-open confirmed for Timeout+on_error=Continue
  path. Six decisions: (1) failure_policy and on_error are separate axes; (2) per-plugin
  failure_policy field in hooks-registry.toml; (3) self-lock hazard — fuel-cap calibration
  MUST precede fail-closed flip; (4) fuel budgeting via p99×1.5-2.0 not fixed constant;
  (5) near-term mitigations — headroom warning + ≥574 KB fixture; (6) verification requirement
  — behavioral test must exercise observed outcome, not documented intent.
  fail_closed_timeout_with_on_error_continue_is_open test encodes current policy and MUST be
  revised deliberately. Adjudicates F-S2107-P7-010/011/015 (design legs). PROPOSED 2026-08-06.]
modified:
  - "2026-08-06 (v1.0)"
  - "2026-08-06 (v1.1)"
---

# ADR-039: Validator failure policy for resource exhaustion — per-plugin `failure_policy` field, fail-closed default for authorization-class validators, and safe migration ordering

## Context

Adversarial review pass-7 of story S-21.07 raised a cluster of findings (F-S2107-P7-010,
F-S2107-P7-011, F-S2107-P7-015) related to WASM plugin resource exhaustion and its effect
on validator enforcement. Research commissioned in `.factory/research/wasm-fuel-exhaustion-detection.md`
established the following ground truth:

**The detection mechanism is already correct.** `Trap::OutOfFuel` (Wasmtime 44.0.1 variant)
is structurally distinguishable from clean completion. The dispatcher already classifies it as
`PluginResult::Timeout { cause: TimeoutCause::Fuel, ... }` and emits a `PLUGIN_TIMEOUT` event
with `cause="fuel"`. The adversarial finding that fuel exhaustion is "indistinguishable" from
a clean pass is directionally correct as an *enforcement* claim, not a detection claim.

**The enforcement defect is real.** `plugin_fail_closed` returns `false` when
`on_error == OnError::Continue`, regardless of `TimeoutCause`. For approximately 38 validator
plugins registered with `on_error = "continue"`, fuel exhaustion therefore produces the same
allow-decision as a clean pass. This is **CWE-636 "Not Failing Securely (Failing Open)"**
(primary) and **CWE-390 "Detection of Error Condition Without Action"** (secondary).

**The current behavior is deliberate policy, not an oversight.** The test
`fail_closed_timeout_with_on_error_continue_is_open` (in the `executor` module) explicitly
asserts that `Timeout { cause: TimeoutCause::Fuel } + on_error=Continue` MUST NOT trigger
fail-closed. This test was written with intent. Changing the policy requires revising this
test deliberately, not deleting it quietly.

**The self-lock hazard is already live today — not a hypothetical future risk introduced by
the migration.** During the authoring of this ADR (2026-08-06), writes to ARCH-INDEX.md
triggered `fail-closed: plugin timed out` PostToolUse blocks from `validate-factory-path-root`,
`validate-input-hash`, and `validate-template-compliance` — three plugins with
`on_error = "block"`. When those plugins exhaust their fuel budget processing a large file,
`plugin_fail_closed` returns `true` (`on_error=Block` + `Timeout`) and the dispatcher fires a
block signal. The writes succeeded in that session only because PostToolUse fires after the
write completes; the block was asserted but could not retroactively prevent the write.
`lessons.md` exhibits the same pattern at its current size (D-442(e); ≤3500 soft / ≤4000 hard
line-count workaround). Any naive flip of `on_error = "continue"` to `"block"` — or of
`failure_policy` to `"fail-closed"` without a recalibrated cap — for validators that read
these artifacts would extend this failure mode to PreToolUse gates, where the block would be
hard and unconditional. The Phase 3-before-Phase 4 ordering constraint in Decision 3 is
therefore not precautionary: it is a response to a failure mode already manifesting in
production today.

**ADR-035 §Decision 5 precedent.** ADR-035 established that `TimeoutCause::Fuel` is a
resource-policy error (not a validation result), that an advisory message MUST be emitted on
exhaustion, and that a per-plugin `fuel_cap` registry field is required. This ADR extends that
ruling to decide the *enforcement* question ADR-035 left open: what should the dispatcher's
block/allow decision be when a validator exhausts its fuel budget?

**Authorization-class classification.** The factory's validator plugins are authorization-class
in the Istio sense: they decide whether a pipeline write is semantically permitted given the
current state of the spec corpus. Istio's `failStrategy` defaults to `FAIL_CLOSE` for this
class and explicitly warns against `FAIL_OPEN`. Envoy's `FailurePolicy` enum defaults to
`FAIL_CLOSED` for the same reason. A fuel-exhausted validator that cannot complete its check
should block, not silently approve.

**The Envoy #38801 lesson.** Envoy issue #38801 (2025-03, v1.33) documents a case where
`failure_policy: FAIL_CLOSED` caused a request hang instead of a 503 response — documented
intent diverged from observed behavior. This is directly relevant: whichever policy this ADR
adopts must have a test that *asserts the observed outcome*, not merely a test that verifies
the policy is *configured*.

---

## Decision

This ADR makes six rulings across three concern areas: (1) axis separation and field scope,
(2) safe migration ordering, and (3) operational verification requirements. Together they
constitute the complete design for replacing the current fail-open enforcement path with
fail-closed enforcement for authorization-class WASM validator plugins, without triggering
the live self-lock hazard.

---

## Decisions

### Decision 1 — `on_error` and `failure_policy` are separate, non-unified axes

The `on_error` field and a new `failure_policy` field govern **different failure modes** and
MUST NOT be collapsed into a single field.

| Field | Failure class it governs | Examples |
|-------|--------------------------|---------|
| `on_error` | Plugin crashes and host-side invocation errors (abrupt non-resource traps; WASM parse failure; host ABI mismatch) | `Trap::UnreachableCodeReached`; `Trap::MemoryOutOfBounds`; deserialization failure |
| `failure_policy` (NEW) | Resource exhaustion outcomes (`TimeoutCause::Fuel`, `TimeoutCause::Epoch`) | `Trap::OutOfFuel`; epoch deadline exceeded |

A plugin may legitimately be crash-advisory (`on_error = "continue"`) — crash means the plugin
encountered unexpected input it cannot classify; a non-block advisory is appropriate. The same
plugin may simultaneously require fail-closed on fuel exhaustion — exhaustion means it received
input the current budget cannot handle; silently approving that write is a policy violation.
The axes are orthogonal.

The registry `on_error` field MUST retain its current semantics:
`"continue"` = fail-open on crash; `"block"` = fail-closed on crash.

A new `failure_policy` TOML field is introduced per plugin entry:
`"fail-closed"` = block on resource exhaustion; `"fail-open"` = allow on resource exhaustion.

If the `failure_policy` field is absent from an entry, the dispatcher MUST treat it as
`"fail-open"` for backward compatibility during the migration window.

### Decision 2 — Scope: per-plugin `failure_policy` field; validator-class plugins use `fail-closed` after migration

**Per-plugin scope is the correct granularity** (not a global default). Advisory-only plugins
(observability hooks, telemetry collectors, convergence-tracking) SHOULD remain `fail-open` on
exhaustion — their job is to record state, not to gate writes. Validator-class plugins (those
whose purpose is write-gating) MUST be `fail-closed` after calibration per Decision 3+4.

Examples of validator-class plugins that MUST receive `failure_policy = "fail-closed"` after
their calibrated `fuel_cap` is verified: `validate-factory-path-root`, `validate-input-hash`,
`validate-template-compliance`, `validate-wave-gate-prerequisite`,
`validate-pr-merge-prerequisites`, `validate-cross-site-correspondence`.

**Registry TOML schema extension.** Each `[[hook]]` entry in `hooks-registry.toml` MAY include:

```toml
failure_policy = "fail-closed"   # resource exhaustion (fuel/epoch) blocks the write
# OR
failure_policy = "fail-open"     # resource exhaustion is advisory only (default if absent)
```

The field is orthogonal to `on_error`. A validator plugin MAY have
`on_error = "continue"` (crash = advisory) AND `failure_policy = "fail-closed"` (exhaustion =
block) simultaneously. This is the intended steady-state for most validators.

The classification of each existing plugin as validator-class or advisory-class is determined
per plugin during the migration story, not by this ADR. Absent annotation = `fail-open`.

### Decision 3 — Self-lock hazard: fuel-cap calibration MUST precede (or be atomic with) the fail-closed flip

**The ordering constraint is hard.** A plugin entry MUST NOT receive
`failure_policy = "fail-closed"` unless its `fuel_cap` has been measured against the live
production corpus AND verified sufficient for the largest live input.

**Ordered migration phases:**

**Phase 1 — Schema extension (no behavior change).** Add `failure_policy` field parsing to the
dispatcher registry loader. If present and equals `"fail-closed"`, the value is stored but
`plugin_fail_closed` enforcement is NOT yet changed. Safe to ship independently.

**Phase 2 — Near-term mitigations (see Decision 5).** Add fuel-headroom warning and
production-scale fixtures. This makes exhaustion observable before any policy change and
provides measurement data for Phase 3.

**Phase 3 — Per-plugin fuel-cap calibration.** For each validator targeted for the
fail-closed flip: measure `fuel_consumed` over a corpus that includes all live production-scale
artifacts. Set `fuel_cap` at ≥ p99 × 1.5. Verify sufficiency for the largest known live input.

**Phase 4 — The enforcement flip.** Extend `plugin_fail_closed` (or introduce a replacement
function) to accept `failure_policy`. For
`Timeout { cause: TimeoutCause::Fuel | TimeoutCause::Epoch }` with
`failure_policy = FailClosed`, return `true` regardless of `on_error`. For each validator: set
`failure_policy = "fail-closed"` in `hooks-registry.toml` **in the same commit** that verifies
its calibrated `fuel_cap`. No half-state: a plugin MUST NOT carry `failure_policy = "fail-closed"`
with the factory default `fuel_cap = 10_000_000`.

**Specific self-lock constraint for lessons.md validators:** No validator that reads
`lessons.md` may receive `failure_policy = "fail-closed"` until its `fuel_cap` is calibrated
against a lessons.md at or above the D-442(e) hard limit (4000 lines). The D-442(e)
line-count workaround is a symptom, not a solution; the root fix is a calibrated per-plugin
budget that accommodates production-size lessons.md without exhausting.

**Observed evidence that the constraint is non-hypothetical (2026-08-06):** During the session
that authored this ADR, writes to ARCH-INDEX.md triggered `fail-closed: plugin timed out`
PostToolUse blocks from `validate-factory-path-root`, `validate-input-hash`, and
`validate-template-compliance`. These three plugins have `on_error = "block"` today and
already exhaust their fuel budget on large files. The writes succeeded only because PostToolUse
cannot retroactively block a completed write. Any `on_error = "continue"` validator that
receives `failure_policy = "fail-closed"` without a recalibrated cap will exhibit the same
failure mode — and for PreToolUse gates, that would produce a hard unconditional block on every
large-artifact write. Phase 3 calibration is not a best-practice recommendation; it is a
prerequisite for preventing the specific failure mode currently observed in production.

### Decision 4 — Fuel budgeting: p99-derived per-plugin caps; fixed `10_000_000` constant is the wrong shape

**The problem with the fixed constant.** The `RegistryDefaults` constant
`fuel_cap = 10_000_000` is a single value applied to all plugins regardless of input size. For
validators that are linear in input size, this provides no meaningful budget guarantee across
the 100× input size variation between a synthetic test fixture and a live STATE.md. F-P1-003
demonstrates this: two `String` allocations per line exhausted the 10M budget on a 426-line
file — well below the largest live artifacts.

**Correct budgeting — minimum requirement for migration (Option A):**
Measure `fuel_consumed` on the calibration corpus. Set the per-plugin `fuel_cap` at
`max(measured_p99 × 1.5, 50_000_000)` where 50M is a recommended floor. Simpler; required
minimum for Phase 3.

**Preferred long-term approach (Option B — size-proportional budget):**
Expose a registry field `fuel_per_kb: u64` (optional). The dispatcher computes
`fuel_cap = base_fuel + fuel_per_kb × ceil(input_size / 1024)` at invocation time, calibrated
from corpus measurements. This approach adapts automatically to input growth and eliminates the
D-442(e) size constraint as a root fix rather than a workaround.

Option B is architecturally preferable but may be deferred to the wave immediately following
Phase 4 migration, provided D-442(e) remains in force until Option B ships.

**Calibration corpus requirements.** The following are mandatory for any validator that reads
whole `.factory/` artifacts:

- `lessons.md` at ≥3000 lines (current soft limit; calibration must cover above it)
- `STATE.md` at current live size
- `decision-log.md` at current live size
- The ≥574 KB synthetic fixture from Decision 5

A corpus of only small test fixtures provides no budget signal for multi-hundred-kilobyte inputs.

### Decision 5 — Near-term mitigations: fuel-headroom warning and production-scale fixture

These two mitigations are independent of the fail-closed policy change and are approved for
immediate delivery within S-21.07 (Task #33):

**Mitigation 1 — Fuel-headroom warning:**
On `PluginResult::Ok`, if `fuel_consumed > 0.9 × cap`, the dispatcher MUST emit a WARN-level
structured event: `"fuel-headroom-warning: plugin consumed ≥90% of budget; next larger input
may trap — recalibrate fuel_cap"`. The event MUST include `plugin_name`, `fuel_consumed`,
`fuel_cap`, and `headroom_ratio` fields. The check belongs in the `Ok` path of the invocation
result handler, after `fuel_consumed_from_store` is computed in the invoke module.

**Mitigation 2 — Production-scale fixture (≥574 KB):**
At least one fixture ≥574 KB MUST be added to the test suite for validators that read whole
artifacts. This fixture is required for calibration corpus measurement (Decision 4) and for
the enforcement test required by Decision 6.

**Residual risk after near-term mitigations only:** The headroom warning improves observability
but does not change the enforcement decision. After Mitigations 1+2, a validator exhausting
fuel on a production write still fails open; the warning is logged but the write is approved.
The structural defect (CWE-636) persists until the Phase 4 enforcement flip.

### Decision 6 — Verification requirement: the test must assert observed behavior, not documented intent

The Envoy #38801 lesson is binding: `FAIL_CLOSED` in production caused a hang instead of a
503 because the test suite asserted configuration, not behavior. Any `failure_policy`
implementation MUST include tests asserting observed outcomes:

1. **Exhaustion + fail-closed → block:** A test invokes a plugin with a fuel cap too small to
   complete on the supplied input (e.g., `fuel_cap = 100`), verifies the result is
   `PluginResult::Timeout { cause: TimeoutCause::Fuel }`, and asserts the dispatcher's final
   decision is BLOCK (exit code 2) when `failure_policy = "fail-closed"`.

2. **Exhaustion + fail-open → advisory:** The same exhaustion scenario with
   `failure_policy = "fail-open"` MUST produce an advisory event, not a block. Exit code 0.

3. **`on_error` independence:** A plugin with `on_error = "continue"` AND
   `failure_policy = "fail-closed"` that exhausts fuel MUST block. This verifies the axes are
   independent — `on_error` does not override `failure_policy` for exhaustion outcomes.

4. **Crash vs exhaustion distinct paths:** A crashed plugin (`on_error = "block"`) MUST block
   via the `on_error` path. A fuel-exhausted plugin with `failure_policy = "fail-open"` and
   `on_error = "block"` MUST NOT block — exhaustion is not a crash.

**Handling the existing `fail_closed_timeout_with_on_error_continue_is_open` test:**
This test currently asserts: `Timeout + on_error=Continue → NOT fail-closed`. This is the
current deliberate policy. When Phase 4 enforcement ships, this test MUST be revised — not
deleted — to assert the new invariant for the `fail-open` configuration case. Deleting the
test without an equivalent replacement is forbidden under TD-VSDD-059 (paper-fix detection):
the deletion would allow future regressions to be introduced silently.

---

## Rationale

### Why `on_error` and `failure_policy` must be separate axes

The two failure modes have fundamentally different causes and different remediation paths. A
crash indicates the plugin encountered unexpected input or an internal error; raising an
advisory and continuing is often correct because the write may be semantically valid. An
exhaustion outcome indicates the plugin ran out of budget before producing any verdict; the
correct semantic is "unknown — deny" because no verdict was produced, not "no violation found."
Collapsing the two into a single field (e.g., re-semanticizing `on_error = "block"` to cover
exhaustion) would require re-auditing all 52 existing plugin entries, introduce ambiguity, and
obscure the distinction for future reviewers.

### Why per-plugin scope is necessary

Activating fail-closed globally on day one triggers the live self-lock: `lessons.md` already
exhausts the 10M budget, so any validator reading it would immediately hard-block all
`.factory/` writes. Per-plugin scope with safe migration ordering is the only path that
(a) achieves the correct steady-state policy, (b) does not cause a P0 self-lock, and
(c) can be progressively verified through testing.

### Why fuel-cap calibration must precede the fail-closed flip

Fail-closed without a sufficient budget is equivalent to blocking unconditionally — the plugin
will always exhaust before completing on production-size inputs, making every gated write fail.
This is a correctness inversion: the intended function of the fail-closed policy is to block
writes that *fail validation*, not to block all writes. The ordering constraint is therefore a
correctness requirement, not a convenience.

### Why Option A (static per-plugin cap) satisfies the migration requirement

Option B (size-proportional budgeting) is the structurally correct long-term solution because
it adapts to input growth without requiring re-calibration. However, it requires new
dispatcher infrastructure (`fuel_per_kb` field, runtime budget computation). Option A, while
requiring periodic re-calibration as artifacts grow, is sufficient to eliminate the CWE-636
defect for the initial migration story. The D-442(e) line-count constraint remains as a
transitional guard until Option B ships.

### Why the Envoy #38801 lesson requires behavioral tests

A policy codified only in documentation or registry configuration provides no protection
against implementation drift. Envoy's documented `FAIL_CLOSED` diverged from observed behavior
in a widely-tested, heavily-used proxy. A smaller codebase with fewer integration tests is
even more vulnerable to this divergence. The only reliable guard is a test that drives the
actual dispatch path with a budget-exhausting input and asserts the block outcome directly.

---

## Consequences

### Positive

- Eliminates CWE-636 for the validator class once Phase 4 migration completes: authorization-
  class plugins no longer silently approve writes when budget-exhausted.
- The `on_error` / `failure_policy` separation allows fine-grained policy without forcing all
  52 plugins into a simultaneous migration.
- Fuel-headroom warning provides early signal for budget drift, reducing the probability of
  silent exhaustion accumulating undetected.
- Option B size-proportional budgeting (future story) removes the D-442(e) line-count
  constraint as a root fix.
- ADR-035 §Decision 5's advisory emission requirement is PRESERVED and extended: a fail-closed
  exhaustion produces both an advisory event (telemetry) and a block outcome (enforcement).

### Negative / Trade-offs

- Migration adds operational surface: plugin entries carry two policy fields (`on_error` +
  `failure_policy`). Reviewers and future implementers must understand both axes.
- Calibration requires production-scale corpus inputs; synthetic test fixtures cannot substitute
  for calibration. This is a one-time investment per plugin.
- Phase 4 enforcement for any given plugin is gated on Phase 3 calibration; there is a window
  during which `failure_policy` is parsed but not yet enforced (Phase 1 through Phase 3). This
  window must be tracked in the migration story.
- If a validator's production-scale calibration reveals an impractically large required budget
  (e.g., it would need 500M fuel for STATE.md), that validator CANNOT be flipped until a
  structural remedy exists. The migration may be partial at first.

### Status as of v1.0

PROPOSED 2026-08-06. Not yet implemented. Adjudicates F-S2107-P7-010, F-S2107-P7-011,
F-S2107-P7-015 design legs from adversarial pass-7 of S-21.07. Decision 5 near-term
mitigations (fuel-headroom warning + ≥574 KB fixture) are in-scope for S-21.07 immediate
delivery. Phase 1-4 migration requires a new story.

---

## Alternatives Considered

**Alternative A — Flip `on_error = "block"` to cover exhaustion.** Rejected. `on_error` is
crash behavior; changing its semantics to also cover exhaustion redefines an existing field
with different meaning. Existing `on_error = "block"` plugins were annotated for crash
behavior, not exhaustion; changing the field meaning retroactively would require re-auditing
all 52 entries.

**Alternative B — Global `failure_policy = "fail-closed"` default for all plugins.** Rejected.
Activating fail-closed globally on day one triggers the live self-lock on `lessons.md` and
any other validator that currently exhausts its budget. The self-lock hazard is documented and
active; a global flip without prior per-plugin calibration is not safe.

**Alternative C — Single unified `FailurePolicy` enum covering crash + exhaustion (Envoy
model).** Rejected. See Decision 1 rationale. The existing `on_error` field has a clear
semantics contract that should not be retroactively broadened.

**Alternative D — Size limits instead of fuel calibration (keep D-442(e) as the permanent
fix).** Rejected under the canonical production-grade principle. D-442(e) is a symptom-level
workaround: limiting artifact size to work around an insufficient budget inverts the correct
dependency. D-442(e) remains as a transitional constraint during the migration window only.

---

## Source / Origin

- **F-S2107-P7-010 (HIGH):** Adversarial pass-7 of S-21.07 — validator WASM plugins have no
  guard teeth for fuel exhaustion; exhaustion yields the same allow-decision as a clean pass.
- **F-S2107-P7-011 (HIGH):** Adversarial pass-7 of S-21.07 — no production-scale fixture
  (≥574 KB) in the test suite; fuel calibration performed against small synthetic inputs only.
- **F-S2107-P7-015 (MEDIUM):** Adversarial pass-7 of S-21.07 — no near-miss warning when fuel
  consumption approaches the cap on a successful pass.
- **Research basis:** `.factory/research/wasm-fuel-exhaustion-detection.md` (2026-08-06) —
  CWE-636/CWE-390 classification; Wasmtime 44.0.1 `Trap::OutOfFuel` detection semantics;
  Envoy `FailurePolicy` FAIL_CLOSED default; Istio `failStrategy` FAIL_CLOSE advisory; Envoy
  #38801 behavioral-divergence lesson; p99×1.5-2.0 calibration recommendation; F-P1-003
  10M-budget exhaustion on 426-line file (documented in `validate-state-structure`).
- **ADR-035 §Decision 5:** Established `TimeoutCause::Fuel` as resource-policy error, mandated
  advisory emission on exhaustion, required per-plugin `fuel_cap` registry field. This ADR
  extends the enforcement question ADR-035 left open.
- **D-442(e):** `lessons.md` size budget ≤3500 soft / ≤4000 hard — the live manifestation of
  the constant-budget shape defect; its existence motivated the migration ordering constraint
  in Decision 3.
- **`fail_closed_timeout_with_on_error_continue_is_open` test (executor module):** Explicitly
  codifies current fail-open policy; cited as the test requiring deliberate revision (not
  deletion) when Phase 4 enforcement ships.

---

## Status

PROPOSED 2026-08-06; ADR-039 v1.0.

Adjudicates F-S2107-P7-010 (HIGH), F-S2107-P7-011 (HIGH), F-S2107-P7-015 (MEDIUM) design
legs from adversarial pass-7 of S-21.07. Extends ADR-035 §Decision 5 to the enforcement
question. Does NOT supersede ADR-035.

Implementation routing:
- **implementer:** Decision 1+2 (registry schema extension + `plugin_fail_closed` extension
  for `failure_policy`) + Decision 6 behavioral tests — Phase 1 and Phase 4 stories
- **implementer:** Decision 5 Mitigation 1 (fuel-headroom warning in invoke module) +
  Mitigation 2 (≥574 KB fixture) — in-scope for S-21.07 near-term delivery
- **devops-engineer:** Decision 4 calibration corpus construction + per-plugin `fuel_cap`
  measurement — prerequisite for Phase 3
- **story-writer:** New story for Phase 3+4 migration (per-plugin calibration + fail-closed
  annotation + Option B size-proportional budget)
- **product-owner:** Decision 4 Option B `fuel_per_kb` field — new registry schema field
  requires BC update when adopted
