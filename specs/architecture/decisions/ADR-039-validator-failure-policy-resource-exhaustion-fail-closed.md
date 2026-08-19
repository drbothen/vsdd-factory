---
document_type: architecture-decision-record
level: L3
adr_id: ADR-039
version: "1.9"
title: "ADR-039: Validator failure policy for resource exhaustion — per-plugin failure_policy field, fail-closed default for authorization-class validators, and safe migration ordering"
status: ratified
date: 2026-08-06
producer: architect
timestamp: 2026-08-06T00:00:00Z
deciders:
  - architect
subsystems_affected: [SS-01, SS-07]
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
research_basis: .factory/research/wasm-fuel-exhaustion-detection.md
extends: ADR-035 §Decision 5
last_amended: |-
  2026-08-18 (v1.9-ratification+corrections) — F-S2111-P13-001 research validation folded in;
  AMD-001 RATIFIED (architect; human sign-off this session; independent Perplexity
  sonar-deep-research + docs.rs/wasmtime/46.0.2 version-pinned verification CONFIRMED the v1.8
  fuel-vs-epoch technical premise verbatim, with four advisory corrections): (1) terminology
  sweep — "epoch interruption"/"epoch axis" renamed to "host wall-clock timeout" throughout
  §Decision 1-4 and §AMD-001 (wasmtime's epoch_interruption feature ALSO cannot bound a
  host-blocking subprocess call per the same doc sentence already cited; `timeout_ms` field
  name and `TimeoutCause::Epoch` code identifier unchanged); mechanism precision paragraph
  added to §Decision 4 citing the actual enforcement point
  (`exec_subprocess.rs::run()`'s Instant-based poll+kill, not wasmtime epoch interruption);
  (2) `timeout_ms >= max(measured_p99_ms x 2.0, 30_000)` reframed as a local calibration
  policy validated by observed false-timeout rate, not an SRE-standard formula — closest
  published analogue is AWS Agentic AI Lens 2-3x p95; (3) 30_000 ms floor documented as a
  cold/loaded-CI cushion, not a latency-derived or Kubernetes-precedented value; (4) new
  break-glass requirement added to §Decision 3 for the two PreToolUse ^Agent$ gates
  (validate-wave-gate-prerequisite, validate-pr-merge-prerequisites) — authenticated
  out-of-band bypass MUST pair the fail-closed posture, named follow-up S-21.17 (new, not yet
  authored). New §AMD-002 filed PROPOSED / NOT RATIFIED (architect self-verification, not one
  of the four authorized corrections): legacy-bash-adapter's exec_subprocess call uses a fixed
  BASH_TIMEOUT_MS=60_000 constant, independent of the registry's calibrated timeout_ms — Phase
  3 calibration currently has no live enforcement target for these five plugins until wired;
  named follow-up S-21.18 (new, not yet authored). BC-1.03.017 v1.9->v1.10 (parallel
  terminology sweep + AMD-002 cite, same burst). ARCH-INDEX row + version co-updated
  (architect-applied). ADR-039 v1.9. [Prior: 2026-08-18 (v1.8-amendment) — Fuel-vs-epoch axis bifurcation for legacy-bash-adapter.wasm
  plugins (architect; F-S2111-P13-001, architect-CONFIRMED HIGH, S-21.11 pass-13): §Decision
  1/2/3/4 amended. Prior text treated calibrated `fuel_cap` (§Decision 3 Phase 3, §Decision 4
  formula) as the uniform calibration mechanism for all six §Decision 2 named plugins. This
  mislabels the resource-exhaustion axis for five of the six: `validate-factory-path-root`,
  `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`, and
  `validate-pr-merge-prerequisites` are all hosted by `hook-plugins/legacy-bash-adapter.wasm`
  (confirmed via `plugin =` field, live `plugins/vsdd-factory/hooks-registry.toml`). Per
  ADR-042 §Decision 3 class (b): "fuel exhaustion occurs before the WASI `exec_subprocess`
  call, the bash script body never executes when the adapter is fuel-starved" — the WASM
  `fuel_cap` meters ONLY the adapter's own marshaling logic, never the bash subprocess it
  shells out to. The subprocess's wall-clock execution is bounded exclusively by the
  already-live per-plugin `timeout_ms` field (no schema change required), enforced via the
  epoch mechanism (`TimeoutCause::Epoch`). Only `validate-cross-site-correspondence` (native
  `hook-plugins/validate-cross-site-correspondence.wasm`, confirmed via the same registry
  grep) executes its validation logic directly as WASM instructions and is genuinely
  fuel-metered end-to-end. §Decision 2's per-plugin scope list, §Decision 3's Phase 3
  calibration procedure, and §Decision 4's calibration formula are amended to bifurcate by
  adapter class — see new §AMD-001. Self-lock hazard restated for the two PreToolUse
  `^Agent$` gates (`validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`): a
  fail-closed flip calibrated only against `fuel_cap` gives no protection against an
  epoch/`timeout_ms` hang on these two gates and risks a hard, unconditional block on every
  future `Agent` tool dispatch — including the dispatches needed to fix the miscalibration.
  **This is a substantive amendment to Decision 2/3/4 normative content, not a
  citation/numeric/label erratum — it therefore REQUIRES human ratification under POLICY 22
  before Phase 3/4 work resumes for the five `legacy-bash-adapter.wasm`-hosted plugins.**
  Status: v1.7 base content remains RATIFIED; the v1.8 delta (§AMD-001, §Decision 2/3/4
  bifurcation) is PROPOSED / RATIFICATION-PENDING pending human review. See §AMD-001. Does
  NOT rescope S-21.11's story body or create the bash-adapter follow-up story — those are
  deferred to a separate burst after human sign-off per orchestrator scoping. ADR-039 v1.8.
  [Prior: 2026-08-18 (v1.7-erratum) — Corpus-floor numeric consistency erratum (architect; F-S2111-P12-001):
  §Decision 4 "Calibration corpus requirements" first bullet read "lessons.md at ≥3000 lines (current
  soft limit; calibration must cover above it)" — defective on three counts: (1) contradicts §Decision 3
  (which mandates calibration at/above the D-442(e) HARD limit = 4000 lines); (2) "current soft limit"
  label is factually wrong — D-442(e) soft limit is 3500, not 3000; (3) downstream-inconsistent with
  BC-1.03.017 Precondition 2 v1.7 and S-21.11 AC-007 v1.10, which both specify ≥4000 lines. Bullet
  updated to "lessons.md at ≥4000 lines (D-442(e) hard limit; soft limit is 3500)". No decision
  semantics, threshold-policy, or normative prescription altered (§Decision 3 already mandated 4000;
  this aligns §Decision 4 to it). Does not require human re-ratification under POLICY 22 (POLICY 22
  governs decision changes; this corrects a numeric sub-clause to match its own parent clause). Status
  remains RATIFIED. ADR-039 v1.7. See §Erratum E-004.
  [Prior: 2026-08-17 (v1.6-erratum) — Non-normative narrative count erratum (architect; F-S2111-P3):
  §Rationale, §Alternatives A, and §Consequences cited "52 existing plugin entries" / "52 plugins"
  / "52 entries" — stale count from the initial v1.0 draft. Live hooks-registry.toml now contains
  76 [[hooks]] entries. Three occurrences reconciled to "existing plugin entries (currently 76)" to
  avoid future staleness. §Context "approximately 38 on_error=continue validators" unchanged (that
  figure refers to a subset class, not the total count). No decision, threshold, or policy content
  altered. Does not require human re-ratification under POLICY 22 (POLICY 22 governs decision
  changes; this corrects a stale narrative count). Status remains RATIFIED. ADR-039 v1.6.
  See §Erratum E-003.
  [Prior: 2026-08-17 (v1.5-erratum) — Non-load-bearing anchor correction (architect; F-S2111-P2-005
  sibling-sweep from S-21.11 story anchor correction): frontmatter `subsystems_affected`
  corrected SS-05→SS-07. ADR-039 governs `hooks-registry.toml` (owned by SS-07 "Hook Bash
  Layer" per ARCH-INDEX §Subsystem Registry) and `executor.rs`/`registry.rs` (SS-01). SS-05
  "Pipeline Orchestration" owns agents + workflows; ADR-039 touches none of it. The S-21.11
  story already corrected its own `subsystems: SS-05→SS-07` at v1.3; this ADR frontmatter was
  the un-swept sibling. No decision, threshold, or normative content altered. Does not require
  human re-ratification under POLICY 22 (POLICY 22 governs decision changes; this corrects a
  frontmatter label). Status remains RATIFIED. ADR-039 v1.5. See §Erratum E-002.
  [Prior: 2026-08-17 (v1.4-erratum) — Non-load-bearing citation erratum (architect; F-S2111-P1-008
  sibling-sweep from S-21.11 spec review): §Decision 5 opening sentence corrected — "S-21.07
  (Task #33)" was a factual citation error; S-21.07 has only 22 tasks (task 12 deferred) and
  no task #33 exists. The ≥574 KB Mitigation 2 fixture was delivered in S-21.07 as the
  `a1-production-scale` bats scenario fixture (commit e94767bc; path:
  `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md`,
  576,396 bytes). §Decision 5 opening sentence updated to reference the actual fixture. §Implementation
  Status Phase 2 corrected: prior text stated "S-21.07 shipped without these mitigations" for
  Mitigation 2 — incorrect; Mitigation 2 fixture IS committed; updated to reflect delivered status.
  No decision, threshold, or normative content altered. Does not require human re-ratification under
  POLICY 22 (POLICY 22 governs decision changes; this corrects a factual citation). Status remains
  RATIFIED. ADR-039 v1.4. See §Erratum at end of document.
  [Prior: 2026-08-16 (v1.3-ratified) — Human ratification via orchestrator (POLICY 22 ratification-channel;
  D-1022 2026-08-16): status proposed→ratified. ADR-039 v1.3 is the ratified version.
  [Prior: 2026-08-16 (v1.3) — Pre-ratification research reconciliation (architect): (1) stale
  fuel-default constant corrected — Decision 3 + Decision 4 + Rationale §per-plugin-scope
  updated 10_000_000 → 20_000_000 with ADR-042 §Decision 2 cite (ADR-042 is SoT for
  current fuel-cap value; SHAPE argument fully preserved); (2) Decision 4 Option A
  reworded — load-bearing guarantee is largest-known-live-input sufficiency + production-
  scale corpus; p99×1.5 reframed as headroom floor heuristic (calibrated risk parameter,
  not SRE standard); hard-block boundary note added (p99.9/observed-max); (3) CWE
  taxonomy enriched — CWE-755 (Improper Handling of Exceptional Conditions) lineage added
  (CWE-755 → CWE-703 → CWE-636) in Context and Source sections; (4) §Consequences
  (Negative) — absent-annotation = fail-open footgun documented as residual risk; follow-up
  hardening story S-21.16 cited. Status PROPOSED / ratification-pending; v1.3 incorporates
  research reconciliations. ADR-039 v1.3.
  [Prior: 2026-08-16 (v1.2) — Ratification prep (architect): internal version inconsistency
  resolved — §Consequences "Status as of v1.0" heading and §Status "ADR-039 v1.0"
  reference corrected to current v1.1/v1.2 (body had never been updated from initial
  v1.0 draft wording after v1.1 frontmatter bump); implementation status updated —
  Phase 1 (S-21.10) delivered; Phase 2 (fuel-headroom warning, Mitigation 1) in progress
  on fix/fuel-exhaustion-fail-loud; Decision 5 S-21.07 aspiration note corrected
  (S-21.07 merged PR #776 without Decision 5 mitigations); Phase 3+4 (S-21.11) authored
  and queued. Status section updated to record FINALIZED FOR RATIFICATION without
  self-ratifying. ADR-039 v1.2.
  [Prior: 2026-08-06 (v1.1) — Context + Decision 3 amended (architect; orchestrator observation):
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
  revised deliberately. Adjudicates F-S2107-P7-010/011/015 (design legs). PROPOSED 2026-08-06.]]]]
modified:
  - "2026-08-06 (v1.0)"
  - "2026-08-06 (v1.1)"
  - "2026-08-16 (v1.2)"
  - "2026-08-16 (v1.3)"
  - "2026-08-17 (v1.4-erratum)"
  - "2026-08-16 (v1.3-ratified)"
  - "2026-08-17 (v1.5-erratum)"
  - "2026-08-17 (v1.6-erratum)"
  - "2026-08-18 (v1.7-erratum)"
  - "2026-08-18 (v1.8-amendment)"
  - "2026-08-18 (v1.9-ratification+corrections)"
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
allow-decision as a clean pass. This is **CWE-636 "Not Failing Securely (Failing Open)"** (primary; lineage:
**CWE-755** (Improper Handling of Exceptional Conditions) → **CWE-703** (Improper Check or
Handling of Exceptional Conditions) → **CWE-636**) and **CWE-390 "Detection of Error
Condition Without Action"** (secondary). CWE-755 is the root ancestor class; CWE-703 is
CWE-636's direct parent in the CWE hierarchy.

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
| `failure_policy` (NEW) | Resource exhaustion outcomes (`TimeoutCause::Fuel`, `TimeoutCause::Epoch`) | `Trap::OutOfFuel`; wasmtime epoch deadline exceeded (guest-WASM-instruction bound; see Decision 1 amendment — this is NOT the same thing as the bash subprocess's host wall-clock timeout) |

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

**Amendment (v1.8, §AMD-001): `failure_policy` is uniform at the decision level (block vs.
allow); the resource-exhaustion SIGNAL it consumes is NOT uniform across plugin adapter
classes.** `TimeoutCause::Fuel` fires when the Wasmtime fuel counter for a plugin's OWN
executing WASM code reaches zero. `TimeoutCause::Epoch` fires when the host-enforced
wall-clock deadline (`timeout_ms`) elapses. For a plugin whose validation logic executes
directly as WASM instructions (a native-WASM plugin), both signals reflect that plugin's own
work and both are meaningful calibration targets. For a plugin hosted by
`hook-plugins/legacy-bash-adapter.wasm` — a shim that marshals input and shells out to a bash
script via WASI `exec_subprocess` — the fuel counter meters ONLY the adapter's own marshaling
code; it never ticks during the bash subprocess's execution (ADR-042 §Decision 3 class (b):
"fuel exhaustion occurs before the WASI `exec_subprocess` call, the bash script body never
executes when the adapter is fuel-starved"). For these plugins, `TimeoutCause::Epoch`/
`timeout_ms` is the ONLY signal that reflects the bash script's actual running time;
`fuel_cap` calibration provides zero protection against a slow or hanging bash script.
§Decision 2, §Decision 3, and §Decision 4 are amended accordingly — see §AMD-001.

### Decision 2 — Scope: per-plugin `failure_policy` field; validator-class plugins use `fail-closed` after migration

**Per-plugin scope is the correct granularity** (not a global default). Advisory-only plugins
(observability hooks, telemetry collectors, convergence-tracking) SHOULD remain `fail-open` on
exhaustion — their job is to record state, not to gate writes. Validator-class plugins (those
whose purpose is write-gating) MUST be `fail-closed` after calibration per Decision 3+4.

Examples of validator-class plugins that MUST receive `failure_policy = "fail-closed"` after
calibration (§Decision 3/4; amended v1.8 — see §AMD-001 for the adapter-class bifurcation):

- **Native-WASM (fuel-axis calibration applies and is sufficient):**
  `validate-cross-site-correspondence` — the only one of the six hosted by its own dedicated
  WASM binary (`hook-plugins/validate-cross-site-correspondence.wasm`); its `fuel_cap`
  genuinely bounds its execution end-to-end.
- **`legacy-bash-adapter.wasm`-hosted (fuel-axis calibration is necessary but NOT sufficient;
  host-wall-clock-timeout-axis (`timeout_ms`) calibration is additionally required):** `validate-factory-path-root`,
  `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`,
  `validate-pr-merge-prerequisites`. These five share one adapter binary; their `fuel_cap`
  bounds only the adapter's marshaling step, never the bash script it invokes. Calibration for
  these plugins MUST additionally verify `timeout_ms` sufficiency per the host-wall-clock-
  timeout-axis formula in §Decision 4 before either receives `failure_policy = "fail-closed"`.

**Registry TOML schema extension.** Each `[[hook]]` entry in `hooks-registry.toml` MAY include:

```toml
failure_policy = "fail-closed"   # resource exhaustion (fuel / host wall-clock timeout) blocks the write
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

**Phase 3 — Per-plugin calibration, bifurcated by adapter class (v1.8 amendment; see
§AMD-001).** For each validator targeted for the fail-closed flip, the calibrated resource
axis depends on which WASM binary hosts it:

- **Native-WASM plugins** (`validate-cross-site-correspondence`): measure `fuel_consumed`
  over a corpus that includes all live production-scale artifacts. **The load-bearing
  requirement is: verify sufficiency for the largest known live input.** Use
  `fuel_cap ≥ p99 × 1.5` as a headroom floor heuristic (see Decision 4).
- **`legacy-bash-adapter.wasm`-hosted plugins** (`validate-factory-path-root`,
  `validate-input-hash`, `validate-template-compliance`, `validate-wave-gate-prerequisite`,
  `validate-pr-merge-prerequisites`): fuel-cap calibration alone is NOT sufficient — the bash
  subprocess is invisible to the fuel counter (§Decision 1 amendment). These plugins
  additionally require measuring wall-clock `time_consumed_ms` over the same production-scale
  corpus and calibrating `timeout_ms ≥ p99_ms × 2.0` (headroom floor: see Decision 4
  host-wall-clock-timeout-axis formula). `fuel_cap ≥ 50_000_000` is NOT evidence of calibration sufficiency for
  these five plugins — it must be treated as an independent, additional requirement, not a
  substitute.

**Phase 4 — The enforcement flip.** Extend `plugin_fail_closed` (or introduce a replacement
function) to accept `failure_policy`. For
`Timeout { cause: TimeoutCause::Fuel | TimeoutCause::Epoch }` with
`failure_policy = FailClosed`, return `true` regardless of `on_error`. For each validator: set
`failure_policy = "fail-closed"` in `hooks-registry.toml` **in the same commit** that verifies
its calibrated `fuel_cap`. No half-state: a plugin MUST NOT carry `failure_policy = "fail-closed"`
with the factory default `fuel_cap = 20_000_000` (per ADR-042 §Decision 2; raised from
the prior 10M constant).

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

**Self-lock hazard specific to the two PreToolUse `^Agent$` gates (v1.8 amendment;
F-S2111-P13-001).** Two of the five `legacy-bash-adapter.wasm`-hosted plugins —
`validate-wave-gate-prerequisite` and `validate-pr-merge-prerequisites` — are registered on
`event = "PreToolUse"`, `tool = "^Agent$"` (confirmed: live
`plugins/vsdd-factory/hooks-registry.toml`). A PreToolUse block on the `Agent` tool prevents
the dispatch from happening at all — unlike the already-documented PostToolUse self-lock
(which cannot retroactively undo a completed write), a PreToolUse `^Agent$` block prevents ANY
subsequent agent dispatch, including the dispatches that would be needed to remediate the
miscalibration. If either of these two plugins receives `failure_policy = "fail-closed"` on
the strength of a `fuel_cap ≥ 50_000_000` predicate alone — which, per §Decision 1's
amendment, provides no protection against the plugin's actual exhaustion axis (`timeout_ms`)
— a bash script that runs long on a production-scale `.factory/` corpus (large `STATE.md`,
`lessons.md`, `decision-log.md`) can exceed its host wall-clock timeout (`timeout_ms`) and
hard-block every future `Agent` dispatch. **Safe posture: `failure_policy = "fail-closed"` MUST NOT be applied to any
`legacy-bash-adapter.wasm`-hosted plugin — and MUST NOT be applied to either PreToolUse
`^Agent$` gate in particular — on the basis of `fuel_cap` sufficiency alone. It requires
demonstrated `timeout_ms` sufficiency per the host-wall-clock-timeout-axis calibration formula
(§Decision 4). This requirement is now paired with a mandatory break-glass companion — see
the new paragraph immediately below.**

**Break-glass requirement for the two PreToolUse `^Agent$` gates (v1.9 amendment;
F-S2111-P13-001 research correction #4).** A calibrated `timeout_ms` reduces the probability
of a self-lock but cannot eliminate it — a bash script can still exceed even a well-calibrated
deadline under an unmodeled failure mode (disk stall, fork bomb, adversarial input). Per the
Kubernetes/GKE/OPA Gatekeeper precedent for admission-webhook self-deadlock (a fail-closed gate
that guards its own repair path is a known, named hazard class, mitigated by narrow authenticated
exemption rather than blanket fail-open), `validate-wave-gate-prerequisite` and
`validate-pr-merge-prerequisites` — the two `legacy-bash-adapter.wasm`-hosted plugins
registered on `event = "PreToolUse"`, `tool = "^Agent$"` — MUST pair their fail-closed,
not-fail-closed-on-fuel-alone posture (§Decision 1/3 above) with a documented, authenticated
break-glass mechanism that lets an operator bypass or disable either gate out-of-band if it
ever wedges `Agent` dispatch, without requiring a working `Agent` dispatch to perform the
bypass (e.g., a `hooks-registry.toml` edit path that does not itself route through the
dispatcher's `Agent`-tool gate, or an explicit environment-variable/CLI override checked
before the gate's block decision is evaluated). This is a NEW architectural requirement, not
yet implemented: it MUST be designed and delivered as part of the Phase 4 enforcement work for
these two plugins, and MUST NOT be treated as satisfied by the calibrated `timeout_ms` alone.
**Named follow-up: S-21.17** (new; not yet authored) — "Authenticated break-glass bypass for
the two PreToolUse `^Agent$` dispatch-guarding gates." S-21.11's Phase 4 fail-closed flip for
`validate-wave-gate-prerequisite` and `validate-pr-merge-prerequisites` specifically MUST NOT
ship without either (a) S-21.17 landing first, or (b) an explicit orchestrator-approved
decision to accept the self-lock residual risk pending S-21.17, recorded at the time of that
Phase 4 commit. This requirement does not apply to the other three `legacy-bash-adapter.wasm`-
hosted plugins (`validate-factory-path-root`, `validate-input-hash`,
`validate-template-compliance`) or to the native-WASM plugin — their `on_error`/`event`
registration (PostToolUse, or non-`^Agent$` tool) does not carry the dispatch-prevention
hazard: a PostToolUse block cannot retroactively undo a completed write (§Decision 3 above).

### Decision 4 — Fuel budgeting: p99-derived per-plugin caps; fixed `20_000_000` constant (per ADR-042) is the wrong shape

**The problem with the fixed constant.** The `RegistryDefaults` constant
`fuel_cap = 20_000_000` (raised 10M→20M by ADR-042 §Decision 2) is a single value applied to
all plugins regardless of input size. For validators that are linear in input size, this
provides no meaningful budget guarantee across the 100× input size variation between a
synthetic test fixture and a live STATE.md. F-P1-003 demonstrates this: two `String`
allocations per line exhausted the pre-ADR-042 10M budget on a 426-line file — well below
the largest live artifacts. The same shape defect persists at 20M: ADR-042 §Decision 1 Erratum
(v1.3) confirms production cycle artifacts reach 198–297% of the 20M cap under the
adapter-class fuel model.

**Correct budgeting — minimum requirement for migration (Option A):**
Measure `fuel_consumed` on the calibration corpus. **The load-bearing guarantee is:
sufficiency for the largest known live input at the time of calibration.** A `fuel_cap` of
`max(measured_p99 × 1.5, 50_000_000)` is the headroom floor heuristic beneath that
guarantee — a calibrated risk parameter, not an SRE standard. For hard-block (fail-closed)
boundary validators (where exhaustion = unconditional pipeline block), p99 × 1.5 leaves
~1% of production-representative inputs at exhaustion risk; the practical target for those
validators should be p99.9 or the observed maximum of the calibration corpus. Simpler;
required minimum for Phase 3.

**Preferred long-term approach (Option B — size-proportional budget):**
Expose a registry field `fuel_per_kb: u64` (optional). The dispatcher computes
`fuel_cap = base_fuel + fuel_per_kb × ceil(input_size / 1024)` at invocation time, calibrated
from corpus measurements. This approach adapts automatically to input growth and eliminates the
D-442(e) size constraint as a root fix rather than a workaround.

Option B is architecturally preferable but may be deferred to the wave immediately following
Phase 4 migration, provided D-442(e) remains in force until Option B ships.

**Host-wall-clock-timeout (`timeout_ms`) budgeting for `legacy-bash-adapter.wasm`-hosted
plugins (v1.8 amendment; §AMD-001).** The `fuel_cap` formula above bounds ONLY the adapter's marshaling fuel; it
provides no protection for the bash subprocess's wall-clock execution (§Decision 1
amendment). For the five `legacy-bash-adapter.wasm`-hosted plugins named in §Decision 2,
calibration MUST additionally measure `time_consumed_ms` (bash subprocess wall-clock
duration) over the same production-scale corpus below, and set:

`timeout_ms ≥ max(measured_p99_ms × 2.0, 30_000)`

The `2.0` safety multiplier (vs. `1.5` for the fuel formula) and the `30_000` ms (30 s) floor
are deliberately more conservative than the fuel-axis formula: wall-clock duration is NOT
deterministic across CI runners or developer machines — unlike fuel, which counts WASM
instructions and is invariant to machine speed — and is additionally subject to disk I/O and
process-scheduling variance. The `30_000` ms floor is chosen proportionally to the fuel
floor's relationship to its own pre-calibration baseline: `50_000_000` fuel is ~2.5× the
`20_000_000` global fuel default (ADR-042 §Decision 2); `30_000` ms is ~3× the current
per-plugin `timeout_ms` defaults for the five targeted plugins (`5_000` ms for
`validate-factory-path-root`; `10_000` ms for the other four, per live
`hooks-registry.toml`). As with the fuel formula, the calibration corpus requirements below
apply identically to the host-wall-clock-timeout axis — the difference is which metric is measured
(`fuel_consumed` vs. wall-clock `time_consumed_ms`) and which registry field the calibrated
value is written to (`fuel_cap` vs. `timeout_ms`; both fields are already live in the
registry schema — no schema change is required for this amendment).

**Framing correction (v1.9 amendment; F-S2111-P13-001 research corrections #2/#3;
independently research-validated per `.factory/cycles/v1.0-brownfield-backfill/F-S2111-P13-001-research.md`).**
The `max(measured_p99_ms × 2.0, 30_000)` formula above is a **local calibration policy
validated by observed false-timeout rate**, not a citable industry-standard formula. Google
SRE explicitly declines to prescribe a fixed multiplier for deadline selection; the closest
published multiplier precedent is AWS's Agentic AI Lens, which recommends 2–3× measured
**p95** for per-tool-invocation timeouts — narrower in scope (agent tool calls, not general
subprocesses) but in the same range. This ADR's `2.0×` **p99** is more conservative than that
range (p99 ≥ p95, so `2×p99` sits above `2–3×p95` for right-skewed latency distributions),
which is the correct bias for a fail-closed dispatch gate: erring toward fewer false timeouts
at the cost of slower hang detection. The `30_000` ms floor is likewise a **cold/loaded-CI
cushion** — it absorbs process startup, cold caches, and loaded CI workers — not a
latency-derived or externally-standard value; it MUST NOT be cited against Kubernetes'
unrelated 30-second pod termination *grace period*, which governs a different concern
(orderly shutdown, not execution-time budgeting) and is not precedent for this floor.

**Mechanism precision (v1.9 amendment; F-S2111-P13-001 research correction #1).** The
calibrated `timeout_ms` value's enforcement point is the dispatcher's own
`exec_subprocess` host function (`crates/factory-dispatcher/src/host/exec_subprocess.rs`,
function `run`), which polls the child process on a wall-clock `Instant` deadline and calls
`child.kill()` on overrun, returning a host-level `TIMEOUT` code to the guest as an ordinary
function-call return value — this is a genuine host/dispatcher-enforced wall-clock bound on
the bash subprocess, distinct from and independent of wasmtime's `epoch_interruption`
feature. It is NOT enforced via wasmtime epoch interruption of the guest WASM: per the same
wasmtime doc sentence cited in §Decision 1's amendment ("Epochs (and fuel) do not assist in
handling WebAssembly code blocked in a call to the host"), the store's epoch deadline
(`TimeoutCause::Epoch`) cannot preempt a synchronous, blocking host function call in
progress — it can only manifest as a trap the next time the guest resumes executing WASM
bytecode, which is after the host call (and therefore the subprocess) has already returned.
**See §AMD-002 below for an open, NOT-YET-RATIFIED finding on how this calibrated value
currently reaches (or does not yet reach) that enforcement point in the shipped
`legacy-bash-adapter` implementation.**

**Calibration corpus requirements.** The following are mandatory for any validator that reads
whole `.factory/` artifacts — the same corpus is used for BOTH the fuel-axis measurement
(native-WASM plugins) and the host-wall-clock-timeout-axis measurement
(`legacy-bash-adapter.wasm`-hosted plugins); only the measured metric differs by adapter class:

- `lessons.md` at ≥4000 lines (D-442(e) hard limit; soft limit is 3500)
- `STATE.md` at current live size
- `decision-log.md` at current live size
- The ≥574 KB synthetic fixture from Decision 5

A corpus of only small test fixtures provides no budget signal for multi-hundred-kilobyte inputs.

### Decision 5 — Near-term mitigations: fuel-headroom warning and production-scale fixture

These two mitigations are independent of the fail-closed policy change. Both were in-scope for
S-21.07. Mitigation 2 (≥574 KB production-scale fixture) was delivered in S-21.07 as the
`a1-production-scale` bats scenario fixture (commit e94767bc; `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md`,
576,396 bytes). Mitigation 1 (fuel-headroom warning) remained pending at S-21.07 merge (see
§Implementation Status):

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
exhaustion) would require re-auditing all existing plugin entries (currently 76), introduce
ambiguity, and obscure the distinction for future reviewers.

### Why per-plugin scope is necessary

Activating fail-closed globally on day one triggers the live self-lock: `lessons.md` already
exhausts the 20M budget (per ADR-042 §Decision 1 Erratum v1.3), so any validator reading it
would immediately hard-block all `.factory/` writes. Per-plugin scope with safe migration ordering is the only path that
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
  existing plugin entries (currently 76) into a simultaneous migration.
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
- **Residual risk — absent-annotation = fail-open footgun (follow-up: S-21.16).** The
  Phase-1 default (`absent failure_policy` = `fail-open`) is the safe migration choice but
  creates a latent risk: any future validator-class plugin whose author omits `failure_policy`
  silently inherits fail-open behavior and reintroduces CWE-636. Secure-by-default would set
  fail-closed as the global default; this ADR preserves fail-open for migration safety. The
  hardening path is tracked as **S-21.16** — a follow-up CI lint requiring explicit
  `failure_policy` on every validator-class plugin entry and/or a global-default flip to
  fail-closed post-migration. The Phase-1 fail-open default MUST NOT be changed before
  Phase 3+4 calibration (S-21.11) completes per Decision 3's ordering constraint.

### Implementation Status (as of v1.4 — 2026-08-17)

**Phase 1 — Schema extension (ADR-039 §Decision 1+2):** Story S-21.10 delivered;
`FailurePolicy` enum and `RegistryEntry.failure_policy` field implemented with serde
deserialization and backward-compatible `fail-open` default (BC-1.01.016 v1.2). Phase 1
no-enforcement gate confirmed: `plugin_fail_closed` behavior is unchanged. S-21.11 (Phase
3+4) is blocked on S-21.10 merge.

**Phase 2 — Near-term mitigations (ADR-039 §Decision 5):** Fuel-headroom warning
(Mitigation 1) is in progress on branch `fix/fuel-exhaustion-fail-loud`; S-21.07 (PR #776)
shipped without Mitigation 1. ≥574 KB production-scale fixture (Mitigation 2) was delivered
in S-21.07 as the `a1-production-scale` bats scenario fixture (commit e94767bc;
`plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md`,
576,396 bytes) — calibration corpus prerequisite satisfied. CWE-636 structural defect persists
until Phase 4 enforcement flip.

**Phase 3+4 — Calibration + enforcement flip (ADR-039 §Decision 3):** Story S-21.11
authored and queued; blocked on S-21.10 merge. Phase 3 requires production-scale corpus
calibration per §Decision 4 (max(p99×1.5, 50M) per-plugin cap). Phase 4 enforcement flip
is gated behind Phase 3 calibration per §Decision 3 atomicity constraint — no half-state.
S-21.11 covers all six validator-class plugins listed in §Decision 2.

[Prior v1.0/v1.1 status note: PROPOSED 2026-08-06. Not yet implemented. Decision 5
near-term mitigations noted as in-scope for S-21.07 immediate delivery. Phase 1-4 migration
requires a new story.]

---

## Alternatives Considered

**Alternative A — Flip `on_error = "block"` to cover exhaustion.** Rejected. `on_error` is
crash behavior; changing its semantics to also cover exhaustion redefines an existing field
with different meaning. Existing `on_error = "block"` plugins were annotated for crash
behavior, not exhaustion; changing the field meaning retroactively would require re-auditing
all existing entries (currently 76).

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
  CWE-755 (root) → CWE-703 → CWE-636 / CWE-390 classification; Wasmtime 44.0.1
  `Trap::OutOfFuel` detection semantics;
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

PROPOSED 2026-08-06; AMENDED 2026-08-06 (v1.1 — Context + Decision 3 expanded with
observational evidence; self-lock hazard strengthened to response-to-active-failure);
FINALIZED FOR RATIFICATION 2026-08-16 (v1.2 — implementation status updated; internal
v1.0 body reference inconsistency resolved; phase delivery status recorded);
PRE-RATIFICATION RESEARCH RECONCILIATION 2026-08-16 (v1.3 — four corrections from
research-agent validation: (1) fuel-default corrected 10M→20M per ADR-042 §Decision 2
in Decision 3, Decision 4, and Rationale; SHAPE argument preserved; (2) Decision 4 Option A
reworded — load-bearing guarantee is largest-live-input sufficiency + production-scale
corpus; p99×1.5 reframed as headroom floor heuristic; hard-block boundary note added;
(3) CWE-755 lineage added (CWE-755 → CWE-703 → CWE-636) in Context and Source; (4) S-21.16
footgun note — absent-annotation = fail-open residual risk documented in §Consequences
Negative). Status PROPOSED / ratification-pending; v1.3 incorporates pre-ratification
research reconciliations; human ratifies next. ADR-039 v1.3.
RATIFIED 2026-08-16 (v1.3) by human via orchestrator (POLICY 22 ratification-channel; D-1022):
status proposed→ratified. ADR-039 v1.3 is the ratified version.
CITATION ERRATUM 2026-08-17 (v1.4 — architect; F-S2111-P1-008): §Decision 5 "S-21.07 (Task #33)"
corrected — Task #33 does not exist in S-21.07. Mitigation 2 fixture delivery citation updated to
reference the actual `a1-production-scale` bats scenario fixture (e94767bc). §Implementation Status
Phase 2 corrected accordingly. No decision semantics altered; status remains RATIFIED. ADR-039 v1.4.
ANCHOR ERRATUM 2026-08-17 (v1.5 — architect; F-S2111-P2-005): frontmatter `subsystems_affected`
corrected [SS-01, SS-05] → [SS-01, SS-07]. SS-07 ("Hook Bash Layer") owns `hooks-registry.toml`
per ARCH-INDEX §Subsystem Registry; SS-05 ("Pipeline Orchestration") owns agents + workflows and
is not touched by this ADR. Non-load-bearing anchor correction; no decision semantics altered;
status remains RATIFIED. ADR-039 v1.5. See §Erratum E-002.
NARRATIVE COUNT ERRATUM 2026-08-17 (v1.6 — architect; F-S2111-P3): §Rationale, §Alternatives A,
and §Consequences cited stale "52" plugin-entry count from initial v1.0 draft. Live
hooks-registry.toml has 76 [[hooks]] entries. Three occurrences updated to "existing plugin
entries (currently 76)". §Context "~38 on_error=continue validators" unchanged. No decision
semantics altered; status remains RATIFIED. ADR-039 v1.6. See §Erratum E-003.
CORPUS-FLOOR ERRATUM 2026-08-18 (v1.7 — architect; F-S2111-P12-001): §Decision 4 calibration
corpus first bullet corrected — "lessons.md at ≥3000 lines (current soft limit; calibration must
cover above it)" contradicted §Decision 3 (which mandates calibration at/above D-442(e) hard limit
= 4000 lines) and mis-labelled the soft limit as 3000 (it is 3500 per D-442(e)); also inconsistent
with BC-1.03.017 Precondition 2 v1.7 and S-21.11 AC-007 v1.10 (both specify ≥4000). Bullet updated
to "lessons.md at ≥4000 lines (D-442(e) hard limit; soft limit is 3500)". No decision semantics
altered; status remains RATIFIED. ADR-039 v1.7. See §Erratum E-004.
FUEL-VS-EPOCH AXIS BIFURCATION AMENDMENT 2026-08-18 (v1.8 — architect; F-S2111-P13-001,
architect-CONFIRMED HIGH): §Decision 1/2/3/4 amended — fuel-cap calibration (§Decision 3
Phase 3, §Decision 4 formula) is genuinely sufficient for only one of the six §Decision 2
named plugins (`validate-cross-site-correspondence`, native-WASM); the other five are hosted
by `hook-plugins/legacy-bash-adapter.wasm`, whose bash subprocess execution is invisible to
the WASM fuel counter (ADR-042 §Decision 3 class (b)) — their real exhaustion axis is
epoch/`timeout_ms`. New epoch-axis calibration formula added to §Decision 4
(`timeout_ms ≥ max(measured_p99_ms × 2.0, 30_000)`); explicit self-lock statement added to
§Decision 3 for the two PreToolUse `^Agent$` gates (`validate-wave-gate-prerequisite`,
`validate-pr-merge-prerequisites`). See §AMD-001 for the full before/after table. **This IS a
substantive Decision-content amendment and REQUIRES human ratification under POLICY 22 before
Phase 3/4 proceeds for the five `legacy-bash-adapter.wasm`-hosted plugins under the corrected
model.** Status: v1.7 base content remains RATIFIED; v1.8 delta is PROPOSED /
RATIFICATION-PENDING. ADR-039 v1.8. See §AMD-001.
RATIFICATION + FOUR-CORRECTION FOLD-IN 2026-08-18 (v1.9 — architect; F-S2111-P13-001 research
validation, `.factory/cycles/v1.0-brownfield-backfill/F-S2111-P13-001-research.md`): human
ratified the v1.8 AMD-001 delta this session (POLICY 22 ratification-channel). Independent
Perplexity `sonar-deep-research` + version-pinned `docs.rs/wasmtime/46.0.2` verification
CONFIRMED the fuel-vs-epoch technical premise verbatim, with four advisory corrections folded
into ratified content: (1) terminology — "epoch interruption"/"epoch axis" swept to "host
wall-clock timeout" throughout §Decision 1-4 and §AMD-001 (wasmtime epoch_interruption also
cannot bound a host-blocking subprocess call; `timeout_ms` field name and `TimeoutCause::Epoch`
code identifier unchanged); new mechanism-precision paragraph in §Decision 4 cites the actual
enforcement point (`exec_subprocess.rs::run()`'s Instant-based poll+kill); (2) the
`p99_ms × 2.0` multiplier reframed as local calibration policy validated by observed
false-timeout rate, not an SRE-standard formula (closest published analogue: AWS Agentic AI
Lens 2-3×p95); (3) the `30_000` ms floor documented as a cold/loaded-CI cushion, not a
latency-derived value; (4) new break-glass requirement added to §Decision 3 for the two
PreToolUse `^Agent$` gates — named follow-up **S-21.17** (new, not yet authored). A fifth,
UNAUTHORIZED finding surfaced during this burst (architect self-verification of the shipped
implementation, not one of the four research-agent corrections) is filed separately as
**§AMD-002, PROPOSED / NOT RATIFIED**: `legacy-bash-adapter`'s `exec_subprocess` call uses a
fixed `BASH_TIMEOUT_MS = 60_000` constant, independent of the registry's calibrated
`timeout_ms` — named follow-up **S-21.18** (new, not yet authored). AMD-002 is NOT ratified by
this entry and does NOT block Phase 3 calibration measurement work, but MUST be resolved (or
its residual risk explicitly accepted by the orchestrator) before Phase 4's fail-closed flip
for the five `legacy-bash-adapter.wasm`-hosted plugins is treated as fully protective. Status:
v1.8 delta (AMD-001) is now RATIFIED; §AMD-002 is PROPOSED / NOT RATIFIED. ADR-039 v1.9. See
§AMD-001 and §AMD-002.

Adjudicates F-S2107-P7-010 (HIGH), F-S2107-P7-011 (HIGH), F-S2107-P7-015 (MEDIUM) design
legs from adversarial pass-7 of S-21.07. Extends ADR-035 §Decision 5 to the enforcement
question. Does NOT supersede ADR-035.

Implementation routing (current status):
- **Phase 1 — DELIVERED (S-21.10):** implementer delivered Decision 1+2 schema leg —
  `FailurePolicy` enum + `RegistryEntry.failure_policy` field + serde deserialization +
  backward-compat `fail-open` default (BC-1.01.016 v1.2). No enforcement change.
- **Phase 2 — IN PROGRESS (Mitigation 1) / DELIVERED (Mitigation 2):** Decision 5 Mitigation 1
  (fuel-headroom warning, invoke module) on branch `fix/fuel-exhaustion-fail-loud`. Mitigation 2
  (≥574 KB production-scale fixture) delivered in S-21.07 as `a1-production-scale` bats scenario
  fixture (commit e94767bc) — calibration corpus prerequisite satisfied.
- **Phase 3+4 — AMD-001 RATIFIED (v1.9); calibration may resume; Phase 4 flip additionally
  gated on §AMD-002 (F-S2111-P13-001; v1.9 self-verification finding, PROPOSED/NOT RATIFIED):**
  devops-engineer's Decision 4 calibration work is bifurcated by adapter class — `fuel_cap`
  measurement for the native-WASM plugin (`validate-cross-site-correspondence`) per the
  original formula; `timeout_ms` (host wall-clock) measurement for the five
  `legacy-bash-adapter.wasm`-hosted plugins per the §Decision 4 host-wall-clock-timeout-axis
  formula. Blocked on S-21.10 merge (delivered — no longer blocking). AMD-001 ratification
  (this session) lifts the Phase-3-calibration blocker. **However, per §AMD-002, the
  calibrated `timeout_ms` value does not yet reach the live subprocess-kill deadline in
  `legacy-bash-adapter`** (fixed at `BASH_TIMEOUT_MS = 60_000`) — S-21.11's Phase 4 fail-closed
  flip for the five `legacy-bash-adapter.wasm`-hosted plugins MUST NOT be treated as fully
  protective until S-21.18 lands or the orchestrator explicitly accepts the residual gap.
  implementer: `plugin_fail_closed` extension for `failure_policy` + Decision 6 behavioral
  tests (Phase 4) — unaffected by AMD-001/AMD-002 (the extension is calibration-axis-agnostic).
  New: implementer/devops-engineer routing for **S-21.17** (break-glass for the two PreToolUse
  `^Agent$` gates) and **S-21.18** (wire calibrated `timeout_ms` into `legacy-bash-adapter`'s
  `exec_subprocess` deadline) — both new, not yet authored; surfaced to orchestrator for
  scoping.
- **product-owner:** Decision 4 Option B `fuel_per_kb` field — new registry schema field
  requires BC update when adopted.

---

## Amendments

Unlike the `## Erratum` entries below (non-normative corrections that explicitly state "no
decision semantics altered"), an entry in this section changes Decision content and therefore
requires human ratification under POLICY 22 before the affected Decision(s) take effect for
implementation purposes.

### AMD-001 — Fuel-vs-host-wall-clock-timeout axis bifurcation for `legacy-bash-adapter.wasm`-hosted plugins (v1.8, 2026-08-18)

**Finding:** F-S2111-P13-001 (HIGH; adversarial pass-13 of story S-21.11; architect-CONFIRMED).

**Error:** §Decision 2 named six plugins as receiving `failure_policy = "fail-closed"` "after
their calibrated `fuel_cap` is verified." §Decision 3 Phase 3 and §Decision 4's calibration
formula (`fuel_cap ≥ max(p99 × 1.5, 50_000_000)`) were written as the uniform calibration
mechanism for all six. This is correct for exactly one of the six
(`validate-cross-site-correspondence`, a native-WASM plugin) and incorrect for the other five
(`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
`validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`), all of which are hosted
by the shared `hook-plugins/legacy-bash-adapter.wasm` shim. Per ADR-042 §Decision 3 class (b),
fuel exhaustion for these plugins occurs (if at all) BEFORE the WASI `exec_subprocess` call —
the bash script body's execution is invisible to the WASM fuel counter entirely. Their actual
resource-exhaustion axis is the host-enforced wall-clock deadline
(`timeout_ms` → `TimeoutCause::Epoch`), a mechanism `fuel_cap` calibration does not touch. A
plugin satisfying `fuel_cap ≥ 50_000_000` (trivial for these five, since adapter-marshaling
fuel consumption is small and roughly proportional to input bytes — ADR-042 §Decision 1
measured model: `fuel ≈ 29,452 + 27.514 × payload_bytes`) provides no evidence that its bash
subprocess will not hang or run long on a production-scale artifact.

**Downstream consequence:** BC-1.03.017's PC8/PC9/PC11 gates, as originally drafted, treat
`fuel_cap ≥ 50_000_000` as the sole structural calibration proof required before a plugin may
carry `failure_policy = "fail-closed"`. For the five `legacy-bash-adapter.wasm`-hosted
plugins, satisfying that predicate is calibration theater — it certifies a resource axis that
was never at risk while leaving the axis that IS at risk (`timeout_ms`) completely
uncalibrated. Two of the five (`validate-wave-gate-prerequisite`,
`validate-pr-merge-prerequisites`) additionally gate the `PreToolUse` `^Agent$` event — a
false "calibrated" fail-closed flip on either risks a hard, unconditional block on all future
`Agent` tool dispatch (see §Decision 3 self-lock paragraph above).

**Correction:** §Decision 1 amended with an explanatory paragraph distinguishing the
fuel-metering signal from the host-wall-clock-timeout (`timeout_ms`)-metering signal per
adapter class. §Decision
2's plugin list is bifurcated into a native-WASM group (fuel-axis calibration sufficient) and
a `legacy-bash-adapter.wasm` group (fuel-axis calibration necessary but NOT sufficient;
host-wall-clock-timeout-axis (`timeout_ms`) calibration additionally required). §Decision 3 Phase 3 is bifurcated
by adapter class, and a new explicit self-lock paragraph names the two PreToolUse `^Agent$`
gates and states the safe posture. §Decision 4 gains a parallel host-wall-clock-timeout-axis calibration formula
(`timeout_ms ≥ max(measured_p99_ms × 2.0, 30_000)`) alongside the existing fuel-axis formula.

| Location | Before | After |
|----------|--------|-------|
| §Decision 1 | No distinction between fuel-signal and host-wall-clock-timeout-signal scope per adapter class | New paragraph: fuel meters adapter-owned WASM instructions only; the host-enforced wall-clock timeout (`timeout_ms`) meters the bash subprocess for `legacy-bash-adapter.wasm`-hosted plugins; fuel_cap gives zero protection for the latter |
| §Decision 2 | Single flat list of six plugins, uniform "calibrated `fuel_cap`" gate | Bifurcated: native-WASM (fuel-axis sufficient) vs. `legacy-bash-adapter.wasm`-hosted (fuel-axis necessary, host-wall-clock-timeout-axis additionally required) |
| §Decision 3 Phase 3 | "Per-plugin fuel-cap calibration" uniform across all six | Bifurcated Phase 3 procedure by adapter class; new explicit self-lock paragraph naming the two PreToolUse `^Agent$` gates and stating the safe posture |
| §Decision 4 | Only the fuel-axis formula (`fuel_cap ≥ max(p99 × 1.5, 50_000_000)`) | Fuel-axis formula retained (scoped to native-WASM); new host-wall-clock-timeout-axis formula added (`timeout_ms ≥ max(measured_p99_ms × 2.0, 30_000)`) for `legacy-bash-adapter.wasm`-hosted plugins |

**Scope:** This IS a normative content change — it adds a previously-absent calibration
requirement (host-wall-clock-timeout (`timeout_ms`) sufficiency) as a precondition for five of the six named
plugins to receive `failure_policy = "fail-closed"`, and narrows the claim that
`fuel_cap`-only calibration is sufficient for those five. It does not reverse any of the six
original Decisions' core rulings (axes separation, per-plugin scope, ordering constraint,
p99-derived budgeting, near-term mitigations, behavioral-test requirement) — it corrects which
registry field each Decision's calibration procedure must target for a given plugin's adapter
class.

**Ratification note:** Per POLICY 22 (governs changes to ADR decisions — rulings, thresholds,
normative prescriptions), this amendment DOES require human re-ratification before Phase 3/4
work proceeds for the five `legacy-bash-adapter.wasm`-hosted plugins under the corrected
model. Unlike E-001 through E-004 below, this is not filed as an Erratum precisely because it
changes what Decision 3/4 require devops-engineer to measure and what BC-1.03.017's structural
gates must assert. **RATIFIED 2026-08-18 (v1.9) by human, this session (independent research
validation via `.factory/cycles/v1.0-brownfield-backfill/F-S2111-P13-001-research.md`
confirmed the technical premise verbatim against the pinned wasmtime 46.0.2 docs, with four
advisory corrections folded in — see the v1.9 Status entry below and the terminology,
local-policy-framing, and break-glass additions made throughout §Decision 1/3/4 above).**
Status: v1.7 base RATIFIED; v1.8 delta (AMD-001) RATIFIED 2026-08-18 (v1.9). ADR-039 v1.9.

---

### AMD-002 — `legacy-bash-adapter`'s bash-subprocess kill deadline is a fixed 60,000 ms constant, independent of the registry's calibrated `timeout_ms` (v1.9, 2026-08-18; architect self-verification during F-S2111-P13-001 corrections fold-in — PROPOSED / NOT RATIFIED)

**Finding:** While folding the four F-S2111-P13-001 research corrections into this ADR, direct
verification of the shipped implementation (`crates/hook-plugins/legacy-bash-adapter/src/lib.rs`,
`crates/factory-dispatcher/src/host/exec_subprocess.rs`, `crates/factory-dispatcher/src/invoke.rs`)
surfaced a gap the v1.8 text did not anticipate and that the research memo (scoped to generic
wasmtime documentation, not this codebase's control flow) did not check.

**Evidence:**
1. `invoke.rs` sets the WASM store's epoch deadline from the registry's per-plugin `timeout_ms`
   (`store.set_epoch_deadline(timeout_ms_to_epochs(limits.timeout_ms as u64))`). This can only
   manifest as `Trap::Interrupt` (→ `TimeoutCause::Epoch`) the next time the guest resumes
   executing WASM bytecode — which cannot happen while the guest is blocked inside the
   `exec_subprocess` host call, per the same wasmtime doc sentence this ADR already cites.
2. The actual subprocess kill deadline is enforced independently, inside
   `exec_subprocess.rs`'s `run()` function, via an `Instant`-based polling loop
   (`child.try_wait()` + `Instant::now() >= deadline` + `child.kill()`), returning a host-level
   `TIMEOUT` code to the guest as an ordinary i32 return value — NOT a wasmtime trap, NOT
   `TimeoutCause::Epoch`.
3. The `timeout_ms` VALUE fed into that deadline is NOT the registry's per-plugin `timeout_ms`
   field. `legacy-bash-adapter/src/lib.rs` hardcodes `pub const BASH_TIMEOUT_MS: u32 = 60_000;`
   and passes this fixed constant into its `exec_subprocess` call, independent of whatever value
   Phase 3/4 calibration writes into `hooks-registry.toml`'s `timeout_ms` field for that plugin.
   The adapter's own doc comment states the design assumption — "Picked higher than the
   dispatcher's per-hook `timeout_ms` ceiling so the wasmtime epoch interrupt is the source of
   truth — the bash timeout is a backstop for the rare case where the dispatcher's epoch
   deadline didn't fire" — which does not hold given point 1 above: the epoch deadline cannot
   preempt a blocking host call, so it cannot be "the source of truth" for a hang.

**Corroborating precedent:** This is not a novel claim. ADR-025 §Decision 18 (2026-07-15, an
unrelated context — the `read_prefix` host function's `timeout_ms` field) already established
the identical fact: "epoch interruption cannot preempt blocking `func_wrap` host calls
executing on dispatcher thread; `timeout_ms` is ABI-forward-reserved." That ADR independently
reached the same conclusion this AMD-002 relies on, for a different host function on the same
dispatcher thread model — strengthening confidence that the finding here is a structural
property of the dispatcher's `func_wrap`/epoch architecture, not an isolated misreading.

**Consequence:** As currently coded, calibrating and raising the registry's per-plugin
`timeout_ms` (per this ADR's §Decision 4 host-wall-clock-timeout-axis formula) has NO EFFECT
on the bash subprocess's actual kill deadline — that deadline is fixed at 60,000 ms regardless
of calibration. This does not invalidate §Decision 1's core claim (fuel cannot meter the
subprocess) or §Decision 3's self-lock concern (a hang up to 60 s is still possible and still
dangerous for the two PreToolUse `^Agent$` gates) — if anything it sharpens the self-lock
concern, since today's real backstop is a fixed constant that the calibration procedure cannot
tighten or verify.

**Scope:** This is a material technical finding beyond the four F-S2111-P13-001 corrections
this burst was authorized to fold in as ratified content. It is filed as **PROPOSED / NOT
RATIFIED**. Resolution requires either (a) human ratification of a further Decision-4
amendment re-scoping the calibration target, or (b) routing to devops-engineer/implementer to
thread the registry-calibrated `timeout_ms` into `legacy-bash-adapter`'s `exec_subprocess`
call (replacing or bounding the fixed `BASH_TIMEOUT_MS` constant) before Phase 4's fail-closed
flip can be considered actually protective for the five `legacy-bash-adapter.wasm`-hosted
plugins. **Named follow-up: S-21.18** (new; not yet authored) — "Wire registry-calibrated
`timeout_ms` into `legacy-bash-adapter`'s `exec_subprocess` deadline, replacing the fixed
`BASH_TIMEOUT_MS` constant." S-21.11's Phase 4 enforcement flip for the five
`legacy-bash-adapter.wasm`-hosted plugins MUST NOT be treated as fully protective until S-21.18
lands or an orchestrator-approved decision explicitly accepts the residual gap. Surfaced to
orchestrator for scoping; not implemented in this burst (architect scope is spec-only).

---

## Erratum

### E-002 — `subsystems_affected` label: SS-05 → SS-07 (v1.5, 2026-08-17)

**Finding:** F-S2111-P2-005 (sibling-sweep during S-21.11 story anchor correction).

**Error:** Frontmatter `subsystems_affected: [SS-01, SS-05]` listed SS-05 ("Pipeline
Orchestration"). SS-05 owns `plugins/vsdd-factory/agents/` and `workflows/*.lobster`; ADR-039
touches neither. The config artifact ADR-039 actually governs — `hooks-registry.toml` — is
owned by SS-07 ("Hook Bash Layer") per ARCH-INDEX §Subsystem Registry. The S-21.11 story
already corrected its own `subsystems: SS-05→SS-07` at v1.3; this ADR frontmatter was the
un-swept sibling (TD-VSDD-060 sibling-site sweep obligation).

**Correction:** `subsystems_affected: [SS-01, SS-05]` → `[SS-01, SS-07]`. SS-01 covers
`executor.rs` and `registry.rs` (Hook Dispatcher Core). SS-07 covers `hooks-registry.toml`
(the registry TOML file where `failure_policy` is added per Decision 2).

**Scope:** Frontmatter label correction only. No decision, threshold, rule, or normative
content altered. The six ADR decisions and all rationale are unchanged.

**Ratification note:** This erratum does not require human re-ratification under POLICY 22.
POLICY 22 governs changes to ADR decisions (rulings, thresholds, normative prescriptions). A
subsystem label correction that does not alter any decision semantics is outside POLICY 22's
scope. Status remains RATIFIED. ADR-039 v1.5.

---

### E-001 — §Decision 5 "Task #33" citation (v1.4, 2026-08-17)

**Finding:** F-S2111-P1-008 (sibling-sweep during S-21.11 spec review).

**Error:** §Decision 5 opening sentence stated "approved for immediate delivery within S-21.07
(Task #33)." S-21.07 has only 22 tasks (task 12 deferred); no task #33 exists. The associated
"S-21.07 shipped without these mitigations" note in §Implementation Status Phase 2 was also
incorrect with respect to Mitigation 2.

**Correction:** The ≥574 KB Mitigation 2 fixture was in fact delivered in S-21.07 as the
`a1-production-scale` bats scenario fixture for `validate-cross-site-correspondence` (commit
e94767bc). Committed path:
`plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-production-scale/factory/specs/behavioral-contracts/BC-INDEX.md`
(576,396 bytes). §Decision 5 opening sentence and §Implementation Status Phase 2 updated to
reference the actual delivered fixture. All three Implementation routing lists updated consistently.

**Scope:** Factual citation correction only. No decision, threshold, rule, or normative content
altered.

**Ratification note:** This erratum does not require human re-ratification under POLICY 22.
POLICY 22 governs changes to ADR decisions (rulings, thresholds, normative prescriptions). A
citation correction that does not alter any decision semantics is outside POLICY 22's scope.
Status remains RATIFIED. ADR-039 v1.4.

---

### E-004 — §Decision 4 corpus-floor: ≥3000 lines corrected to ≥4000 lines (v1.7, 2026-08-18)

**Finding:** F-S2111-P12-001 (un-swept sibling of the pass-8 corpus-floor fix).

**Error:** §Decision 4 "Calibration corpus requirements" first bullet read:
`lessons.md at ≥3000 lines (current soft limit; calibration must cover above it)`.
This was defective on three counts:

1. It contradicted §Decision 3, which specifies calibration against `lessons.md` at or above
   the D-442(e) HARD limit (4000 lines) — not the soft limit.
2. The "current soft limit" label was factually wrong — the D-442(e) soft limit is 3500 (not 3000).
3. It was downstream-inconsistent with BC-1.03.017 Precondition 2 v1.7 and story S-21.11 v1.10
   AC-007, which both specify ≥4000 lines.

**Correction:** First bullet updated to: `lessons.md at ≥4000 lines (D-442(e) hard limit; soft limit is 3500)`.

| Location | Before | After |
|----------|--------|-------|
| §Decision 4 — Calibration corpus requirements, first bullet | `lessons.md at ≥3000 lines (current soft limit; calibration must cover above it)` | `lessons.md at ≥4000 lines (D-442(e) hard limit; soft limit is 3500)` |

**Scope:** Numeric consistency correction only. §Decision 3 already mandated calibration at the
D-442(e) hard limit (4000 lines); this erratum aligns §Decision 4's corpus bullet to that same
figure. No decision semantics, threshold-policy, or normative prescription altered.

**Ratification note:** This erratum does not require human re-ratification under POLICY 22.
POLICY 22 governs changes to ADR decisions (rulings, thresholds, normative prescriptions). A
numeric consistency correction that aligns one sub-clause of §Decision 4 to the figure already
mandated by §Decision 3 is outside POLICY 22's scope. Status remains RATIFIED. ADR-039 v1.7.

---

### E-003 — Stale "52" plugin-count in §Rationale, §Alternatives, §Consequences (v1.6, 2026-08-17)

**Finding:** F-S2111-P3 (LOW observation — narrative count staleness).

**Error:** Three occurrences in non-normative narrative cited "all 52 existing plugin entries"
(§Rationale "Why `on_error` and `failure_policy` must be separate axes"), "all 52 plugins"
(§Consequences Positive), and "all 52 entries" (§Alternatives Considered, Alternative A). This
count of 52 reflected the plugin inventory at the time of the initial v1.0 draft (2026-08-06).
The live `plugins/vsdd-factory/hooks-registry.toml` now contains 76 `[[hooks]]` entries. The
ADR-039 §Context paragraph ("approximately 38 validator plugins with `on_error = continue`")
correctly used an approximation for the validator-class subset and required no change.

**Correction:** Three narrative occurrences updated from the hardcoded count "52" to the
de-hardcoded phrasing "existing plugin entries (currently 76)" / "existing entries (currently 76)"
to avoid future staleness as new hook plugins are added.

| Location | Before | After |
|----------|--------|-------|
| §Rationale — "Why `on_error` and `failure_policy` must be separate axes" | "re-auditing all 52 existing plugin entries" | "re-auditing all existing plugin entries (currently 76)" |
| §Consequences — Positive, second bullet | "forcing all 52 plugins into a simultaneous migration" | "forcing all existing plugin entries (currently 76) into a simultaneous migration" |
| §Alternatives — Alternative A | "require re-auditing all 52 entries" | "require re-auditing all existing entries (currently 76)" |

**Scope:** Non-normative narrative count correction only. No decision, threshold, rule,
enforcement policy, migration phase, or normative prescription altered. The six ADR decisions
and all rationale substance are unchanged.

**Ratification note:** This erratum does not require human re-ratification under POLICY 22.
POLICY 22 governs changes to ADR decisions (rulings, thresholds, normative prescriptions). A
narrative count correction that does not alter any decision semantics is outside POLICY 22's
scope. Status remains RATIFIED. ADR-039 v1.6.
