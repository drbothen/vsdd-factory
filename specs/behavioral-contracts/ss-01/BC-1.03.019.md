---
document_type: behavioral-contract
level: L3
version: "v1.1"
status: draft
producer: product-owner
timestamp: 2026-08-20T00:00:00Z
phase: brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/planning/S-21.11-decomposition-plan.md
input-hash: "7368f5a"
traces_to: .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-011"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill-E21-W6
modified:
  - "2026-08-20 (v1.1)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.03.019: factory-dispatcher::invoke fuel-headroom WARN event — >90% fuel-consumption early-warning signal on `PluginResult::Ok`, independent of `on_error`/`failure_policy` enforcement (ADR-039 §Decision 5 Mitigation 1)

## Description

The factory-dispatcher's `invoke_plugin` (`crates/factory-dispatcher/src/invoke.rs`) MUST emit a
WARN-level structured event whenever a plugin invocation completes as `PluginResult::Ok` having
consumed more than 90% of its configured `fuel_cap`. This is a Phase-2 near-term observability
mitigation from ADR-039 §Decision 5 ("Mitigation 1 — Fuel-headroom warning"), fully independent
of the `failure_policy` fail-closed enforcement machinery sibling BC-1.03.017 governs: it fires
uniformly for every plugin, calibrated or not, fail-open or fail-closed, and never influences the
dispatcher's block decision. It exists to give operators advance notice — "your plugin is close
to trapping; recalibrate `fuel_cap`" — before an actual `Timeout { cause: Fuel }` exhaustion
occurs on a larger future input.

## Preconditions

1. The dispatcher's `invoke_plugin` function has completed a WASM plugin invocation and produced
   a final `PluginResult` value (`Ok`, `Timeout`, or `Crashed`), with `fuel_consumed` already
   computed via `fuel_consumed_from_store(&store, limits.fuel_cap)` (`invoke.rs`, immediately
   after `start_export.call(&mut store, ())` returns or traps).
2. The invoking `RegistryEntry`'s configured `fuel_cap` (either an explicit per-plugin override
   or the `DEFAULT_FUEL_CAP = 20_000_000` fallback, ADR-042 §Decision 1) is available as
   `limits.fuel_cap: u64` and is always `> 0` for any invocation that can reach
   `PluginResult::Ok` — a `fuel_cap = 0` invocation cannot reach `Ok` (see EC-002); the guest
   traps on its very first fuel-metered instruction, producing `Timeout { cause: Fuel }` instead.
3. This BC is independent of BC-1.03.017's `failure_policy`/`on_error` enforcement-dispatch
   machinery: no precondition here depends on `RegistryEntry.failure_policy`,
   `RegistryEntry.on_error`, or ADR-039 §Decision 2/3/4 Phase 3/4 calibration/enforcement state.
   The event fires for every plugin's every invocation, calibrated or not.
4. A `HostContext` carrying `plugin_name: String` is reachable from the invocation-completion
   point (`store.data().host` after `PluginResult` is determined, since `invoke_plugin` moves its
   `host_ctx: HostContext` parameter into `StoreData` at construction time), so the event can be
   emitted through the existing `HostContext::emit_internal` path — the same path
   `emit_plugin_timeout_async`/`emit_plugin_completed_async`
   (`crates/factory-dispatcher/src/host/emit_event.rs`) already use for other plugin-lifecycle
   events.

## Postconditions

1. **PC1 — Threshold predicate fires WARN on `Ok`:**
   When `invoke_plugin` returns `PluginResult::Ok { fuel_consumed, .. }` and
   `fuel_consumed > 0.9 × fuel_cap` (strict inequality), the dispatcher MUST emit exactly one
   WARN-level structured event before `invoke_plugin` returns. To avoid floating-point boundary
   error at exactly 90%, implementations MUST compare using exact integer arithmetic:
   `fuel_consumed.saturating_mul(10) > fuel_cap.saturating_mul(9)`, which is precisely equivalent
   to `fuel_consumed > 0.9 × fuel_cap` for all `u64` inputs realistic for this system (fuel caps
   in the tens-of-millions range; `saturating_mul` is defensive against theoretical overflow, not
   expected to trigger in practice).

2. **PC2 — Exact-90% boundary does NOT fire (negative control):**
   When `fuel_consumed == fuel_cap × 0.9` exactly (e.g., `fuel_consumed = 18_000_000`,
   `fuel_cap = 20_000_000`), the predicate MUST evaluate false and no event MUST be emitted.
   ADR-039 §Decision 5's own wording (`fuel_consumed > 0.9 × cap`) is strict-greater-than; this
   MUST be asserted explicitly by a dedicated boundary test, not left to floating-point
   happenstance (see PC1's integer-comparison requirement).

3. **PC3 — Below-threshold negative control:**
   When `fuel_consumed <= 0.9 × fuel_cap` by any margin (e.g., `fuel_consumed = 10_000_000`,
   `fuel_cap = 20_000_000`, 50%), no event MUST be emitted.

4. **PC4 — Non-`Ok` outcomes never fire this event:**
   `PluginResult::Timeout { .. }` (either `TimeoutCause::Fuel` or `TimeoutCause::Epoch`) and
   `PluginResult::Crashed { .. }` MUST NEVER trigger this event, regardless of the
   `fuel_consumed` value those variants carry. A genuine `Timeout { cause: Fuel }` outcome always
   carries `fuel_consumed == fuel_cap` (100% — per `invoke.rs`'s own doc comment on
   `PluginResult::Timeout::fuel_consumed`), which is trivially `> 0.9 × cap`, but that outcome is
   the actual exhaustion event governed by sibling BC-1.03.017 (PC1/PC2/PC6), not a "near miss."
   This event is exclusively the early-warning signal for the `Ok` (survived) case; firing it on
   a `Timeout` outcome would be a semantic double-count, not a defect closure.

5. **PC5 — Applies uniformly to both `Ok` sub-shapes; single centralized check point:**
   The check MUST fire identically for `PluginResult::Ok { exit_code: 0, .. }` (the WASI
   clean-exit path, `invoke.rs`'s `Ok(())` arm) and `PluginResult::Ok { exit_code != 0, .. }`
   (the `I32Exit` non-zero-exit path inside `classify_trap`) — both are `PluginResult::Ok` per
   the enum definition, and ADR-039 §Decision 5 draws no exit-code distinction. Implementations
   MUST perform this check exactly once, at a single centralized point after `invoke_plugin`'s
   internal `match call_result { .. }` has produced its final `PluginResult` value — NOT
   duplicated inline inside each of the two `Ok`-constructing branches — to prevent both a
   missed-emission defect on the non-zero-exit branch and a double-emission defect if both
   branches independently invoked the check.

6. **PC6 — Required event fields:**
   The emitted event MUST carry the following complete field set, matching ADR-039 §Decision 5
   Mitigation 1 (this enumeration — not the ADR-mandated-payload subset alone — is the complete,
   correct field set; it folds in the `level`/`message` fields PC8 separately mandates and the
   `timestamp` field required for sibling-emitter parity, so no downstream reader should treat
   this list as excluding those PC8/parity fields):
   - `plugin_name: String` — the invoking plugin's registry name (`HostContext.plugin_name`).
   - `fuel_consumed: u64` — the exact value from `PluginResult::Ok::fuel_consumed`.
   - `fuel_cap: u64` — the exact value from `limits.fuel_cap` (the calibrated-or-default cap in
     force for this invocation).
   - `headroom_ratio: f64` — the fraction of budget REMAINING (see PC7 for formula and
     rationale), clamped to `[0.0, 1.0]`.
   - `level: String` — literal `"warn"` (PC8).
   - `message: String` — the ADR-039 §Decision 5 verbatim warning text (PC8).
   - `timestamp: String` — the event's ISO-8601 emission timestamp, carried via
     `.with_field("timestamp", ts.as_str())` — the identical idiom every sibling `plugin.*`
     emitter in `emit_event.rs` already uses (`emit_plugin_timeout_async`, `emit_plugin_abandoned`,
     `emit_plugin_completed_async`). This event MUST NOT omit `timestamp`: S-19.09 T-013/F-WG-003
     was a dedicated fix that added `timestamp` to `emit_plugin_completed_async` precisely because
     it had been missing from that sibling emitter, and this new emitter must not reintroduce that
     gap.
   Plus the standard `InternalEvent` envelope fields every dispatcher event already carries
   (`type`, `trace_id`, `session_id`, `schema_version` — DI-007 / BC-1.12.002 convention).

7. **PC7 — `headroom_ratio` formula (production-grade default; ADR-039 is silent on the exact
   arithmetic):**
   ADR-039 §Decision 5 names the field `headroom_ratio` but does not define its formula. This BC
   adopts `headroom_ratio = 1.0 - (fuel_consumed as f64 / fuel_cap as f64)` — the fraction of the
   budget STILL UNCONSUMED at completion — as the production-grade default, for two reasons: (a)
   "headroom" conventionally denotes the remaining margin before a limit is hit (the standard
   audio/electrical/network-engineering usage), not the amount already used; (b) the ADR's own
   remediation guidance — "recalibrate `fuel_cap`" — is actionable precisely because the operator
   needs to know how much margin is LEFT; the consumed fraction is already fully recoverable from
   the co-emitted `fuel_consumed`/`fuel_cap` fields, so a `headroom_ratio` that merely restated
   `fuel_consumed / fuel_cap` would be redundant with those two fields rather than adding
   information. Worked example: `fuel_consumed = 18_500_000`, `fuel_cap = 20_000_000` →
   `headroom_ratio = 0.075` (7.5% margin remaining). A grep of this repository found no prior
   `headroom`/`headroom_ratio` fuel-adjacent precedent to defer to instead.

8. **PC8 — WARN-level classification and exact message text:**
   The event's `level` field MUST be the literal string `"warn"`. `InternalEvent` (the dispatcher
   self-telemetry envelope) has no dedicated severity field of its own (see Architecture
   Anchors), so `level` MUST be carried as an explicit `fields` entry via
   `.with_field("level", Value::String("warn".into()))` — the identical idiom
   `crates/factory-dispatcher/src/invoke.rs` and `resolver_loader.rs` already use at their own
   `emit_event`/`level_str` call sites for plugin-emitted events. The event MUST additionally
   carry a `message` field equal, verbatim, to ADR-039 v1.15 §Decision 5's mandated text (corrected
   by ADR-039 E-006, 2026-08-20, from an earlier `≥90%` draft wording to strict `>90%` to match the
   strict-greater-than trigger predicate this BC's PC1/PC2 already specify): `"fuel-headroom-warning:
   plugin consumed >90% of budget; next larger input may trap — recalibrate fuel_cap"`.
   Implementations MUST NOT paraphrase, truncate, or interpolate the plugin name into this
   string — `plugin_name` is carried as its own structured field (PC6), not string-interpolated.

9. **PC9 — Independence from `on_error`/`failure_policy`/block-decision (BC-1.03.017
   non-interaction):**
   Emitting this event MUST NOT read `RegistryEntry.on_error`, MUST NOT read
   `RegistryEntry.failure_policy`, MUST NOT set `block_intent = true`, and MUST NOT alter the
   dispatcher's exit code. This event fires identically whether the invoking plugin is
   `on_error = "continue"` or `on_error = "block"`, and whether `failure_policy` is `fail-open`,
   `fail-closed`, or unset (pre-migration state) — it is Phase-2 observability per ADR-039
   §Decision 5 ("independent of the fail-closed policy change"), strictly orthogonal to sibling
   BC-1.03.017's Phase-3/4 enforcement-dispatch decision (its PC1–PC13). The two BCs' triggering
   conditions are mutually exclusive by outcome shape on any single invocation: this BC requires
   `PluginResult::Ok`; BC-1.03.017's exhaustion postconditions (PC1/PC2/PC6) require
   `PluginResult::Timeout`. A single invocation is never both.

10. **PC10 — Exactly-once-per-invocation; no intra-invocation crossing-then-recovery:**
    The check runs exactly once per `invoke_plugin` call, evaluated against the single terminal
    `fuel_consumed` reading captured after the guest's `_start` export returns or traps. wasmtime
    fuel consumption is monotonically non-decreasing during a single invocation — fuel is charged
    per-instruction and never refunded mid-execution — so a single invocation cannot "cross the
    90% threshold and then recover" within itself. There is no live/streaming threshold-crossing
    detector to define dedup semantics for: there is exactly one candidate emission point per
    invocation (post-completion, against the final reading), and this postcondition closes that
    ambiguity explicitly rather than leaving it implicit.

## Invariants

1. Fuel consumption within a single WASM invocation is monotonic non-decreasing (wasmtime charges
   per instruction, never refunds) — checking the terminal `fuel_consumed` value once, after
   invocation completion, is complete and sufficient; no mid-execution sampling is required or
   meaningful.
2. This event's emission is purely additive to `invoke_plugin`'s existing control flow: it MUST
   NOT change the returned `PluginResult` value, MUST NOT change `elapsed_ms`, and MUST NOT
   introduce a new error/panic path — emission failures are swallowed exactly as every other
   `InternalLog`/event-sink write already is (best-effort I/O, per `emit_event.rs`'s documented
   contract).
3. The `0.9` (90%) threshold is ADR-039 §Decision 5's own literal value, not independently
   derived. It MUST be a named constant with a doc comment citing ADR-039 §Decision 5 (e.g.
   `FUEL_HEADROOM_WARNING_NUM: u64 = 9`, `FUEL_HEADROOM_WARNING_DEN: u64 = 10`, or equivalent),
   per this codebase's existing convention of named constants over bare literals (cf.
   `DEFAULT_FUEL_CAP`, `STDERR_CAP_BYTES` in `invoke.rs`).
4. This event is orthogonal to, and does not supersede, sibling BC-1.03.017's exhaustion
   enforcement. A plugin later annotated `failure_policy = "fail-closed"` still receives this WARN
   event on every `Ok` invocation crossing 90%, exactly as it did pre-annotation — annotating
   `failure_policy` changes what happens on actual exhaustion (`Timeout`), not what happens on a
   near-miss `Ok`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Exactly-90% boundary (`fuel_consumed == 0.9 × fuel_cap`) | Does NOT fire (PC2). Integer-exact comparison (`fuel_consumed×10 > fuel_cap×9`) eliminates float-rounding false positives/negatives at the boundary. |
| EC-002 | `fuel_cap = 0` (degenerate/unreachable) | Cannot produce `PluginResult::Ok` — the guest traps before any instruction completes (`store.set_fuel(0)` means the first fuel-metered operation already exceeds budget), so this check's `Ok`-scoped predicate is definitionally unreached. Implementations MUST still guard the arithmetic defensively: PC1's `saturating_mul` comparison is well-defined at `fuel_cap = 0` (`fuel_consumed×10 > 0` is true for any `fuel_consumed > 0` — safe/conservative even on this theoretically-unreachable input) and MUST NOT panic. |
| EC-003 | Missing/unset `fuel_cap` in registry config | Not reachable at this check site — `InvokeLimits.fuel_cap: u64` is mandatory (non-`Option`); registry resolution always falls back to `DEFAULT_FUEL_CAP = 20_000_000` (ADR-042 §Decision 1) when a plugin's entry carries no override. There is no "no cap" state at `invoke_plugin`'s call site. |
| EC-004 | Threshold-crossing-then-recovery within one invocation | Impossible (Invariant 1 / PC10) — fuel consumption is monotonic non-decreasing within a single guest execution; no "recovery" below a previously-crossed threshold can occur mid-invocation. |
| EC-005 | `Ok { exit_code != 0 }` crossing the threshold (non-zero-exit success path) | Fires identically to the `exit_code: 0` path (PC5) — orthogonal to exit-code value and to BC-1.03.017's `on_error`/PC13 classification of that same outcome. |
| EC-006 | `Timeout { cause: Fuel }` genuine exhaustion (`fuel_consumed == fuel_cap`, 100%) | Explicitly OUT of scope — MUST NOT fire this event (PC4); governed instead by sibling BC-1.03.017 PC1/PC2/PC6. |
| EC-007 | Repeated invocations of the same plugin crossing 90% on every dispatch | Each invocation independently evaluates and emits at most one event; there is no cross-invocation state, counter, or suppression (v1.0 runs one dispatcher process per hook invocation, per `internal_log.rs`'s design doc). Repeated per-dispatch warnings are the intended nudge signal, not a defect to be rate-limited — ADR-039 §Decision 5 specifies no rate-limiting. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `PluginResult::Ok{exit_code:0,..}`, `fuel_consumed=18_500_000`, `fuel_cap=20_000_000` (92.5%) | WARN event fires: `plugin_name`, `fuel_consumed=18_500_000`, `fuel_cap=20_000_000`, `headroom_ratio=0.075`, `level="warn"`, `timestamp=<ISO-8601>`, `message="fuel-headroom-warning: plugin consumed >90% of budget; next larger input may trap — recalibrate fuel_cap"` | happy-path |
| `PluginResult::Ok{exit_code:0,..}`, `fuel_consumed=18_000_000`, `fuel_cap=20_000_000` (exactly 90%) | No event emitted (PC2 boundary control) | edge-case |
| `PluginResult::Ok{exit_code:0,..}`, `fuel_consumed=10_000_000`, `fuel_cap=20_000_000` (50%) | No event emitted (PC3 negative control) | edge-case |
| `PluginResult::Timeout{cause:TimeoutCause::Fuel,..}`, `fuel_consumed=100`, `fuel_cap=100` (100%) | No event emitted — governed instead by BC-1.03.017 PC1/PC2 (PC4 non-`Ok`-outcome exclusion) | error |
| `PluginResult::Ok{exit_code:1,..}`, `fuel_consumed=19_000_000`, `fuel_cap=20_000_000` (95%) | WARN event fires identically to the `exit_code:0` case; `headroom_ratio=0.05` (PC5 uniform-sub-shape coverage) | edge-case |

## Related BCs

- **BC-1.03.002** — sibling detection layer: governs `invoke_plugin` returning
  `PluginResult::Timeout { cause: TimeoutCause::Fuel }` on genuine exhaustion. This BC's PC4
  explicitly excludes that outcome from firing the headroom warning — the two BCs partition the
  fuel-budget lifecycle by outcome shape (survived-near-miss vs. actual-exhaustion).
- **BC-1.03.017** — sibling enforcement-dispatch BC (`failure_policy`/`on_error` fail-closed
  decision). This BC's PC9 asserts strict independence: no shared read of
  `failure_policy`/`on_error`, no shared write of `block_intent`, and mutually exclusive
  triggering conditions (`Ok` here vs. `Timeout`/`Crashed`/non-zero-exit-`Ok` there for the
  block-decision path — note EC-005 shows the exit-code!=0-`Ok` shape can independently trigger
  BOTH this BC's warning AND, if `on_error=Block`, BC-1.03.017's PC13 block; the two effects
  compose without conflict since one is observability and the other is enforcement).
- **BC-1.01.016** — provides the `RegistryEntry` schema this BC's Precondition 2 references for
  `fuel_cap` resolution (note: `fuel_cap` itself predates `failure_policy`/BC-1.01.016 per
  ADR-042 — this BC does not depend on BC-1.01.016's `failure_policy` field, only on the
  pre-existing `fuel_cap` field).

## Architecture Anchors

- `crates/factory-dispatcher/src/invoke.rs::invoke_plugin` — the emission point: a single
  centralized check after `fuel_consumed_from_store` is computed and the internal
  `match call_result { .. }` has produced the final `PluginResult`, covering both the WASI
  clean-exit `Ok(())` arm and `classify_trap`'s `I32Exit` `Ok` arm (PC5).
- `crates/factory-dispatcher/src/invoke.rs::PluginResult::Ok` — the enum variant this check
  gates on; `PluginResult::Timeout`/`PluginResult::Crashed` are explicitly excluded (PC4).
- `crates/factory-dispatcher/src/host/emit_event.rs` — new `emit_fuel_headroom_warning(ctx:
  &HostContext, plugin_name: &str, fuel_consumed: u64, fuel_cap: u64)` function, following the
  `emit_plugin_timeout_async`/`emit_plugin_completed_async` convention already established there
  (construct `InternalEvent::now(..)`, chain `.with_trace_id`/`.with_session_id`/
  `.with_plugin_name`/`.with_field(..)`, call `ctx.emit_internal(ev)`).
- `crates/factory-dispatcher/src/internal_log.rs` — new event-type constant
  `PLUGIN_FUEL_HEADROOM_WARNING: &str = "plugin.fuel_headroom_warning"`, added to the existing
  `pub const PLUGIN_*` catalog alongside `PLUGIN_COMPLETED`/`PLUGIN_TIMEOUT`/`PLUGIN_CRASHED`/
  `PLUGIN_ABANDONED`.
- `crates/factory-dispatcher/src/host/mod.rs::HostContext::emit_internal` — the existing
  emission path this BC reuses (writes to the always-on internal log, per DI-007, and pushes to
  the sink-router event queue); no new emission mechanism is required.
- `crates/factory-dispatcher/src/invoke.rs::DEFAULT_FUEL_CAP` (20,000,000; ADR-042 §Decision 1) —
  the fallback value `limits.fuel_cap` resolves to when a plugin's registry entry carries no
  explicit override (EC-003).
- ADR-039 §Decision 5 Mitigation 1 — the governing requirement text (predicate, message,
  fields, emission point).

## Story Anchors

- S-21.25 — Fuel-headroom WARN event (ADR-039 §Decision 5 Mitigation 1); new, Phase-2-independent
  track; no dependency on S-21.19–S-21.24's `failure_policy` seams.

## VP Anchors

- VP-TBD — fuel-headroom warning emission: threshold predicate (PC1–PC3), non-`Ok`-outcome
  exclusion (PC4), uniform sub-shape coverage (PC5), required-fields shape (PC6–PC8),
  independence from `on_error`/`failure_policy`/`block_intent` (PC9), exactly-once-per-invocation
  semantics (PC10).

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | On `PluginResult::Ok`, `fuel_consumed > 0.9 × fuel_cap` (strict, integer-exact comparison) → exactly one WARN-level event emitted carrying `plugin_name`, `fuel_consumed`, `fuel_cap`, `headroom_ratio = 1 - fuel_consumed/fuel_cap`, `level="warn"`, and the ADR-039 §Decision 5 verbatim message; `fuel_consumed <= 0.9 × fuel_cap` (including exactly-90%) → no event; `PluginResult::Timeout`/`PluginResult::Crashed` → no event regardless of `fuel_consumed`, even at 100% fuel exhaustion; the check fires identically for `Ok{exit_code:0}` and `Ok{exit_code!=0}`, evaluated exactly once per invocation against the terminal `fuel_consumed` reading; emission never reads `RegistryEntry.on_error`/`RegistryEntry.failure_policy` and never sets `block_intent`. | unit tests (positive/boundary/negative controls per PC1–PC3; non-`Ok`-outcome exclusion per PC4; both `Ok` sub-shapes per PC5; field-shape assertion per PC6–PC8; independence assertion per PC9 — construct `on_error=Block`+`failure_policy=FailClosed`+`Ok{fuel_consumed>0.9×cap}` and assert `block_intent` unaffected by this event's emission) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-011 |
| Capability Anchor Justification | CAP-011 ("Enforce fuel and epoch budgets on plugin execution") per capabilities.md §CAP-011 — CAP-011's stated outcome clause covers exceeded limits producing `Timeout`/`Fuel` outcomes, "never hung processes." This BC extends CAP-011's OBSERVABILITY dimension: an early-warning signal emitted BEFORE a plugin reaches the `Timeout{Fuel}` outcome CAP-011 already governs, giving operators actionable lead time to recalibrate `fuel_cap`. Three BCs now anchor to CAP-011, each covering a distinct phase of the fuel-budget lifecycle: BC-1.03.002 (detection — the `Timeout{Fuel}` outcome itself), BC-1.03.017 (enforcement — block-vs-advisory dispatch on that outcome), and this BC (early-warning — the near-miss `Ok` signal preceding either). |
| L2 Domain Invariants | DI-007 ("Dispatcher self-telemetry is always-on," as amended by ADR-015 D-15.1 — opt-in via `VSDD_DEBUG_LOG`/`observability-config.toml`; this event rides the same `InternalEvent`/`HostContext::emit_internal` path DI-007 governs, so its always-on-when-opted-in guarantee applies here identically) |
| Architecture Module | SS-01 (Hook Dispatcher Core) — `crates/factory-dispatcher/src/invoke.rs` |
| ADR | ADR-039 v1.15 §Decision 5 Mitigation 1 ("Near-term mitigations: fuel-headroom warning," explicitly independent of the fail-closed policy change in §Decision 1–4/§AMD-001–003). Version cite updated v1.14→v1.15 (this BC's v1.1 fix burst) to track ADR-039 E-006's `≥90%`→`>90%` WARN-message correction, reproduced verbatim in PC8. |
| Security | None directly closed — this is a Phase-2 observability mitigation only. ADR-039 §Decision 5's own "Residual risk after near-term mitigations only" clause is explicit: "the warning is logged but the write is approved... the structural defect (CWE-636) persists until the Phase 4 enforcement flip" (governed by sibling BC-1.03.017, not this BC). |
| Stories | S-21.25 |
| Cycle | v1.0-brownfield-backfill (E-21 Wave 6, parallel/independent track — no dependency edges to the S-21.19–S-21.24 `failure_policy` seams per S-21.11-decomposition-plan.md §2 wave graph) |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.1 | 2026-08-20 | product-owner | S-21.25 adversarial pass-1 (`v1.0-brownfield-backfill`) fix burst — three findings closed: **F-S2125-P1-003** (MEDIUM) — PC6's required-fields enumeration omitted the mandatory `timestamp` common field every sibling `plugin.*` emitter carries (`emit_plugin_timeout_async`/`emit_plugin_abandoned`/`emit_plugin_completed_async` in `emit_event.rs`); S-19.09 T-013/F-WG-003 was a dedicated fix to ADD `timestamp` to `emit_plugin_completed_async` precisely because it had been missing, and this new event must not reintroduce that gap. PC6 rewritten to include `timestamp: String` (same field name as siblings, not a bespoke `ts`) plus a cross-reference note to the S-19.09 precedent. Corresponding BC-3.08.001 registration performed in the same burst (Event 7 `plugin.fuel_headroom_warning` added to the SS-03 event catalog — see BC-3.08.001 v1.25 Changelog); sibling parity + registration was the adversary's preferred resolution over a documented exemption. **F-S2125-P1-005** (LOW) — PC6's "exactly these fields" wording falsely excluded `message`, which PC8 separately mandates. PC6 reworded to drop the false-exclusivity framing and enumerate the complete, correct field set (`plugin_name`, `fuel_consumed`, `fuel_cap`, `headroom_ratio`, `level`, `message`, `timestamp`, plus the standard envelope) so PC6 and PC8 no longer contradict each other. **F-S2125-P1-006 cascade** (from architect's ADR-039 v1.15 fix, E-006) — ADR-039 §Decision 5's WARN message was corrected from an earlier `≥90%` draft wording to strict `>90%` to match the strict-greater-than trigger predicate (PC1/PC2). PC8's mandated message string updated to reproduce the corrected text byte-for-byte: `"fuel-headroom-warning: plugin consumed >90% of budget; next larger input may trap — recalibrate fuel_cap"`. Swept to the Canonical Test Vectors happy-path row (added `timestamp` to the asserted field list; corrected the message string) and to the Traceability ADR row (version cite added, v1.14→v1.15, with the E-006 rationale noted). **F-S2125-P1-004 reconciliation**: the v1.0 changelog row below narrated `input-hash: "PENDING"` while the frontmatter already carried a real computed value (`57262cf`); the row is corrected in place to state the actual value rather than "PENDING" — this BC does not itself recompute or alter the frontmatter `input-hash` field (state-manager's `--check`/`--update` reconcile remains the authoritative source for that value; this is a narrative-text-only correction). **Downstream cascade note (not performed here, story-writer's scope):** this burst's PC6 field-set change (+`timestamp`, +`message` folded into the enumeration) and PC8's message correction cascade to S-21.25 AC-006 (field enumeration) and AC-008 (message string) — story-writer sweeps those in a follow-up dispatch. |
| v1.0 | 2026-08-20 | product-owner | Initial authoring. Governs ADR-039 §Decision 5 Mitigation 1 (fuel-headroom WARN event) — a previously orphaned live MUST-requirement (abandoned branch `fix/fuel-exhaustion-fail-loud` had no landed implementation for this event; verified via repo grep, no `headroom`/`fuel_headroom` occurrence found anywhere in `crates/`) identified in `S-21.11-decomposition-plan.md` §S-21.25, now given a real BC home per operator direction. Ten postconditions (PC1–PC10) cover: the strict `>90%` threshold predicate with an exact-integer comparison to eliminate float-boundary risk (PC1), an exact-90% negative control (PC2), a well-below-threshold negative control (PC3), explicit exclusion of `Timeout`/`Crashed` outcomes including the 100%-fuel-consumed `Timeout{Fuel}` case (PC4), uniform coverage of both `Ok{exit_code:0}` and `Ok{exit_code!=0}` sub-shapes via one centralized check point rather than duplicated per-branch logic (PC5), the four ADR-mandated event fields plus envelope/level fields (PC6), the `headroom_ratio` formula (PC7), WARN-level classification and verbatim message text (PC8), strict independence from `on_error`/`failure_policy`/`block_intent` (PC9, BC-1.03.017 non-interaction), and exactly-once-per-invocation emission grounded in wasmtime's monotonic non-decreasing fuel model (PC10). Production-grade defaults established where ADR-039 §Decision 5 is silent, each with rationale inline (CLAUDE.md canonical principle — no placeholders): (1) `headroom_ratio` defined as remaining-budget fraction (`1 - fuel_consumed/fuel_cap`), not consumed fraction, per standard "headroom" terminology and the ADR's own recalibration-guidance framing, and because a consumed-fraction reading would be redundant with the co-emitted `fuel_consumed`/`fuel_cap` fields (PC7); (2) the `>90%` comparison specified as exact integer arithmetic (`fuel_consumed×10 > fuel_cap×9`) rather than floating-point, eliminating boundary-rounding risk at exactly 90% (PC1/PC2); (3) single-centralized-check-point architecture (after `invoke_plugin`'s `match` produces its final `PluginResult`, not duplicated per `Ok`-constructing branch) to prevent a missed-emission defect on the `classify_trap` `I32Exit` branch (PC5). New event-type constant `plugin.fuel_headroom_warning` proposed for `internal_log.rs`'s existing `PLUGIN_*` catalog; new `emit_fuel_headroom_warning()` helper proposed for `emit_event.rs`, following the `emit_plugin_timeout_async`/`emit_plugin_completed_async` convention (Architecture Anchors). Anchored to CAP-011 ("Enforce fuel and epoch budgets on plugin execution") per capabilities.md §CAP-011 — this BC covers CAP-011's early-warning/observability dispatch, distinct from BC-1.03.002 (detection) and BC-1.03.017 (enforcement), which anchor the same capability's other two lifecycle phases (Capability Anchor Justification). `L2 Domain Invariants` cites DI-007 (dispatcher self-telemetry always-on) since this event rides the same `InternalEvent`/`emit_internal` path. VP-TBD placeholder per POLICY 9's sanctioned-deferral convention — architect assigns a real VP-NNN and propagates to VP-INDEX/verification-architecture.md/verification-coverage-matrix.md in a follow-up dispatch; flagged in this task's report per `vp_index_is_vp_catalog_source_of_truth`. `subsystem: SS-01` set directly from ARCH-INDEX Subsystem Registry (not `SS-TBD`) since the architecture already exists and BC-1.03.017/BC-1.03.018 (siblings governing the same `invoke.rs`/`executor.rs` files) are both confirmed `SS-01`. `input-hash: "57262cf"` (the value the frontmatter actually carried at v1.0 authoring time; corrected in this row by the v1.1 fix burst per F-S2125-P1-004 — the row previously narrated "PENDING" even though the frontmatter already held a real computed value, which was itself the finding) — product-owner has no `exec`/`process` tool access (Tool Access: Denied) and cannot run `compute-input-hash --update`; ongoing/future hash reconciliation (including any drift introduced by this v1.1 fix burst's own edits) remains state-manager's `--check`/`--update` responsibility. Does NOT touch BC-1.03.017, BC-1.03.018, the S-21.11 bundle, the 6 split sub-stories, STORY-INDEX, BC-INDEX, or STATE.md (routing per orchestrator directive: state-manager registers the BC-INDEX row; story-writer creates S-21.25 and anchors its ACs to this BC). Anchor-Back Rule note: S-21.25 does not yet exist as a story file (story-writer's next dispatch creates it citing this BC from inception), so there is no existing story requiring a retroactive BC-table update — the Anchor-Back Rule's "same burst" obligation applies to pre-existing stories whose scope newly touches a BC, which is not this case. |
