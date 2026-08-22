---
document_type: behavioral-contract
level: L3
version: "v1.3"
status: draft
producer: product-owner
timestamp: 2026-08-19T00:00:00Z
last_amended: "2026-08-22 (v1.3) — Wave-7 adversary pass-2 remediation, cycle
v1.0-brownfield-backfill, applying the architect's adjudicated language: (Q-A,
F-S2123-P2-002) PC9 rewritten to a DUAL-TRIGGER ordering gate — the prior wording keyed
exclusively on `failure_policy=\"fail-closed\"`, missing that both named Agent gates ALREADY
carry `on_error=\"block\"` in the LIVE registry today and therefore ALREADY self-lock on
crash/timeout under ADR-039 §Decision 1's axes-independence, independent of any
`failure_policy` flip; PC9 now asserts the ordering rule fires when EITHER
`failure_policy=\"fail-closed\"` OR `on_error=\"block\"` is present (two independently-sufficient
triggers, not a conjunction). Extended the control set from four to six: added a
`failure_policy`-trigger POSITIVE-CONTROL relabel, a NEW `on_error`-only-trigger
POSITIVE-CONTROL (matching the live registry's actual state), a NEW combined-both-triggers
POSITIVE-CONTROL, a NEGATIVE-CONTROL covering all three, a corrected VACUITY-CONTROL (now
requiring BOTH triggers absent — the pre-dual-trigger vacuity fixture, keying only on
`failure_policy` absent, was false-vacuous against the live `on_error=\"block\"` state), and a
LIVE-TREE-CONTROL whose correct outcome today is that the ordering rule is ALREADY TRIGGERED.
Invariant 6 rewritten to match. Six Canonical Test Vector rows added/rewritten for the
dual-trigger controls. (Q-B, F-S2123-P2-006) New Postcondition PC12 — audit-loss
observability, independent of suppression outcome: PC11's suppression-denial is correctly a
no-op on a counterfactually-clean-pass matched invocation (no block to deny), which left a
failed/skipped audit write on such an invocation SILENT — no observable trace distinguishes it
from a normal clean pass with a successful write. PC12 mandates an unconditional stderr
diagnostic (gate name, trace UUID, failure variant) whenever `InternalLog::write_checked`
returns other than `Written`, firing identically in PC11's suppression-denial case AND the
counterfactual-clean-pass case; the diagnostic is itself best-effort (regress terminates
there). New Invariant 8 codifies this as PC11's audit-durability guarantee's observability
complement. New Edge Case EC-008 (matched clean-pass invocation + failed/skipped audit write →
block_intent unchanged, PC12 diagnostic MUST fire). One new Canonical Test Vector row (PC12
POSITIVE control, EC-008). Traceability ADR row extended with ADR-039 §Decision 1 (Q-A
citation) and F-S2123-P2-006 (Q-B citation); Security, Stories, and Story Anchor rows updated
to attribute PC9's correction and PC12 to S-21.23 alongside PC11. VP-TBD (Verification
Properties + VP Anchors) extended for PC9's six-control dual-trigger gate and PC12's
observability property. H1 enriched per POLICY 7 with the dual-trigger and audit-loss-
observability clauses. No PC renumbering; PC12 is new (additive); PC1–PC8, PC10, PC11
UNCHANGED in substance (PC9's rewrite is corrective, not renumbering). Does NOT touch
BC-1.03.017 (separate product-owner burst, same task), the S-21.x story bodies/ACs/frontmatter
(story-writer's domain, dispatched separately this burst per POLICY 8 propagation), or any
INDEX/STATE.md (state-manager's domain). This content change means this BC's own `input-hash`
will go stale as a result and is flagged for state-manager reconciliation same-burst.
BC-1.03.018 v1.3. [Prior: 2026-08-22 (v1.2) — Wave-7 adversary pass-1 remediation, cycle
v1.0-brownfield-backfill (Q3, F-S2123-P1-001; ADR-039 §AMD-004, RATIFIED 2026-08-22, architect
adjudication via orchestrator relay): break-glass suppression MUST be fail-closed-on-audit-
failure. The mandated audit sink (`InternalLog::write`) is, by its own documented contract,
best-effort — it swallows I/O errors and its mount-gate skip path also returns `Ok(())` — so a
caller cannot distinguish 'durably written' from 'silently dropped'/'silently skipped' from that
`Result` alone; left as specified, a break-glass activation whose audit write is dropped
(precisely during the live self-lock scenario break-glass exists to recover from) still
suppresses the underlying block, reproducing the CWE-636 silent-approval state PC5/PC6's own
audit-mandatory requirement exists to prevent, one layer down. Added new Postcondition PC11:
suppression (PC2/PC3) is granted ONLY IF the `break_glass.activated` audit write is CONFIRMED
DURABLY WRITTEN via the new, additive `InternalLog::write_checked` tri-state method (`Written`/
`SkippedMountGate`/`Failed(io::Error)`, added ALONGSIDE the existing best-effort `write()`,
which remains unchanged for its ~6 other hot-path telemetry callers); if the write FAILS or is
SKIPPED, the override MUST NOT be applied and the gate's block decision stands exactly as if
unset. New Invariant 7 codifies the audit-durability-gates-suppression principle as the
human-readable complement to PC11, cross-referenced from Invariant 3 and PC5. New Edge Case
EC-007 (matched override + unwritable/mount-gated audit sink → suppression denied, block
stands). Three new Canonical Test Vector rows (PC11 POSITIVE controls for `Failed` and
`SkippedMountGate`; NEGATIVE control for `Written`). Traceability ADR row extended with the
ADR-039 §AMD-004 citation; Architecture Module and Stories rows updated to name
`InternalLog::write_checked` and S-21.23 (the Wave-7 split-topology sub-story of CONVERGED
S-21.11 owning this leg's Phase-3 TDD implementation) respectively. Architecture Anchors
extended with an `InternalLog::write_checked` bullet; Story Anchor section extended with
S-21.23. VP-TBD (Verification Properties row + VP Anchors) extended with PC11's property and a
Phase-6 formal-verifier VP-assignment anchor note. H1 enriched per POLICY 7 with the
audit-durability clause. No PC renumbering; PC11 is new (additive); PC1-PC10 UNCHANGED. Does
NOT touch BC-1.03.017 (Q1/Q2's amendment — separate product-owner burst, same task), the S-21.x
story bodies/ACs/frontmatter (story-writer's domain, dispatched separately this burst per
POLICY 8 propagation), or any INDEX/STATE.md (state-manager's domain). This content change
means this BC's own `input-hash` will go stale as a result and is flagged for state-manager
reconciliation same-burst. BC-1.03.018 v1.2. [Prior: 2026-08-19 (v1.1) — S-21.11 v2.0 adversarial pass-1 remediation (F-S2111V2-P1-001-mechanism-adjudication memo): (1) F-004 fix — EC-005's Description column corrected from the false 'both named gates dispatched in the SAME tier' premise (live `hooks-registry.toml` has `validate-pr-merge-prerequisites` at priority 120 and `validate-wave-gate-prerequisite` at priority 130 — distinct priorities place them in two DIFFERENT `routing.rs::group_by_priority` tiers) to the accurate mechanism: both gates are dispatched in separate tiers, evaluated unconditionally within the same dispatch via `execute_tiers`'s no-early-return tier loop; EC-005's stated conclusion (two separate `break_glass.activated` events, per-gate audit granularity preserved) is unchanged — only the reason given was wrong. No PC/Precondition/Invariant content altered by this leg. (2) F-006 fix — new PC10 adds a testable assertion plus explicit threat-boundary note: an `Agent`-tool-invoked path (Bash-tool subprocess, MCP tool invocation, or hook-wrapper subprocess) CANNOT arm `VSDD_BREAK_GLASS_GATE_BYPASS` for the dispatcher's own process environment — only a human operator's own process environment can, because child-process environment mutations never propagate to a parent process (OS process-model property); the dispatcher inherits its environment from whatever launched it (ultimately the operator's shell), not from any descendant subprocess an `Agent`-invoked tool call spawns. Invariant 1 cross-referenced to PC10. New Canonical Test Vector row + Architecture Anchors bullet. New Changelog section added (this BC previously had none). BC-1.03.018 v1.1.]]"
phase: brownfield-backfill
inputs:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "a8749f9"
traces_to: .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-039"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-08-19 (v1.1)"
  - "2026-08-22 (v1.2)"
  - "2026-08-22 (v1.3)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.03.018: factory-dispatcher `execute_tiers` break-glass override — `VSDD_BREAK_GLASS_GATE_BYPASS` environment-variable escape hatch for the two self-locking PreToolUse `^Agent$` gates (`validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`), with mandatory `break_glass.activated` JSONL audit, intra-S-21.11 dual-trigger (failure_policy OR on_error) pre-flip-ordering gate, human-operator-only Agent-tool-cannot-arm threat-boundary control, audit-write-durability-gates-suppression fail-closed-on-audit-failure control, and audit-loss stderr observability independent of suppression outcome (ADR-039 §Decision 1 axes-independence + §Decision 3 v1.10 minimum-viable definition + §AMD-004 audit-durability amendment; PC9 dual-trigger ordering gate; PC10 F-006 threat-boundary gate; PC11 §AMD-004 fail-closed-on-audit-failure gate; PC12 audit-loss observability gate)

## Description

The factory-dispatcher's `execute_tiers` block-decision loop (`crates/factory-dispatcher/src/executor.rs`) consults a process-environment override, `VSDD_BREAK_GLASS_GATE_BYPASS`, before folding either `validate-wave-gate-prerequisite`'s or `validate-pr-merge-prerequisites`'s outcome into `block_intent`. When the override names a gate (or the literal `all`), that gate's outcome is treated as allow for that invocation only, regardless of which decision path (`plugin_requests_block` or `plugin_fail_closed`) would otherwise have produced the block, and regardless of the plugin's own `PluginResult` variant. This closes the self-lock hazard ADR-039 §Decision 3 identifies for these two `legacy-bash-adapter.wasm`-hosted, `PreToolUse`/`^Agent$`-registered validators: unlike a PostToolUse block (which cannot retroactively undo a completed write), a PreToolUse block on the `Agent` tool prevents ANY subsequent agent dispatch — including the dispatch needed to fix a miscalibration. The mechanism is authenticated by possession of shell/process-environment access (this factory has no separate credential layer for hook bypass) and is human-operator-only by construction: setting the variable does not route through the dispatcher's own `Agent`-tool gate, so no `Agent`-invoked tool can set or exploit it to self-approve. Every activation is mandatorily audited via a structured `break_glass.activated` event to the dispatcher-internal JSONL log.

## Preconditions

1. ADR-039 §Decision 3 v1.10 amendment's break-glass minimum-viable definition is RATIFIED (POLICY 22, 2026-08-19) — no further design pass is required to derive this BC's postconditions.
2. The two named PreToolUse `^Agent$` gate entries exist in `hooks-registry.toml` with `event = "PreToolUse"`, `tool = "^Agent$"`, `plugin = "hook-plugins/legacy-bash-adapter.wasm"` (confirmed live: `validate-wave-gate-prerequisite` at priority 130, `validate-pr-merge-prerequisites` at priority 120).
3. `execute_tiers`'s block-decision loop (`crates/factory-dispatcher/src/executor.rs`) is the aggregation point where, per tier outcome, `block_intent` is set to `true` when `plugin_requests_block(&outcome.result) || plugin_fail_closed(&outcome.result, outcome.on_error)`. The break-glass check MUST be consulted per-`RegistryEntry.name` at or before this point, scoped to the two named entries only, before either outcome's block determination is folded into `block_intent`.
4. Per ADR-039 §Decision 3 v1.10, this mechanism's delivery is intra-S-21.11: its landing (this BC's PC1–PC8) MUST precede, or be atomic with (same commit), the commit that sets `failure_policy = "fail-closed"` for `validate-wave-gate-prerequisite` or `validate-pr-merge-prerequisites` in `hooks-registry.toml` (BC-1.03.017 PC9's scope for these two plugins). This is a sequencing precondition on the story, mechanically enforced by this BC's PC9.

## Postconditions

1. **PC1 — Override unset/absent → no behavioral change:** When `VSDD_BREAK_GLASS_GATE_BYPASS` is unset (or set to the empty string), both named gates' block decisions are computed exactly as if the break-glass mechanism did not exist — BC-1.03.017's `failure_policy`/`on_error` semantics apply unmodified. This is the baseline regression control: introducing the break-glass check MUST NOT alter any existing BC-1.03.017 postcondition's observed outcome when the override is inactive.

2. **PC2 — Override set to an exact gate name → that gate's block suppressed for that invocation, regardless of block cause:** When `VSDD_BREAK_GLASS_GATE_BYPASS` contains the plugin `name` of `validate-wave-gate-prerequisite` or `validate-pr-merge-prerequisites` (exact match; comma-separated list; each token trimmed of leading/trailing whitespace before comparison), that specific gate's contribution to `block_intent` for that invocation is forced to `false` — REGARDLESS of which decision path would otherwise have produced the block (`plugin_requests_block`'s `HookResult::Block` path, OR `plugin_fail_closed`'s exhaustion path under `failure_policy = FailClosed`), and regardless of the plugin's own `PluginResult` variant (`Crashed`, `Timeout{Fuel}`, `Timeout{Epoch}`). This matches ADR-039's MVD text verbatim: "the dispatcher treats that gate's outcome as allow regardless of the plugin's own result for that invocation only."

3. **PC3 — Override set to the literal `all` → both named gates bypassed:** `VSDD_BREAK_GLASS_GATE_BYPASS=all` suppresses both `validate-wave-gate-prerequisite`'s and `validate-pr-merge-prerequisites`' block contributions for that invocation, equivalent to naming both explicitly.

4. **PC4 — Override set to a non-matching value → no suppression (negative control):** When `VSDD_BREAK_GLASS_GATE_BYPASS` is set to a value that does not exact-match either named gate's `name` and is not `all` (e.g., a typo, or the name of an unrelated plugin such as `validate-factory-path-root`), neither named gate's normal block behavior is suppressed. This proves the matcher is name-scoped and not a blanket kill-switch that fires on any non-empty value.

5. **PC5 — Mandatory audit on every activation, no silent bypass:** Every time PC2 or PC3 suppresses a gate's block contribution for a given invocation, the dispatcher MUST emit a structured `break_glass.activated` event to `.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl` (the same event-sink family documented for `VSDD_SINK_FILE` diagnostics) carrying at minimum: the bypassed gate name(s), an ISO-8601 timestamp, and the dispatch's trace UUID (`dispatcher_trace_id`). A test asserting PC2/PC3's suppression behavior fires with NO corresponding `break_glass.activated` record in the captured log output MUST fail — silent bypass is a defect, not an acceptable implementation (ADR-039 MVD, verbatim: "Silent bypass (no audit event) is not an acceptable implementation"). **This postcondition governs EMISSION of the audit event; it does NOT by itself guarantee the emission attempt durably lands — see PC11 (Q3, ADR-039 §AMD-004) for the fail-closed-on-audit-failure gating that makes suppression itself conditional on a CONFIRMED durable write, not merely an attempted one.**

6. **PC6 — Audit fires on every matched invocation, independent of whether the plugin's own outcome would have blocked:** The `break_glass.activated` event MUST be emitted whenever the override names a gate that is dispatched in that invocation's tier set, REGARDLESS of whether that gate's own `PluginResult` would have produced a block absent the override (i.e., the audit is not conditioned on a counterfactual block having actually occurred). This closes an ambiguity in the MVD's "observes the override active for a gated invocation" language: "active" is defined as "the override names this gate AND this gate is invoked," not "the override suppressed an outcome that would otherwise have blocked." Rationale: an operator who leaves `VSDD_BREAK_GLASS_GATE_BYPASS` set after the emergency has passed is an auditable fact regardless of whether any individual invocation would have blocked; conditioning the audit on a counterfactual block would silently under-report forgotten-armed break-glass state.

7. **PC7 — Non-persistent, validation-preserving override (no registry mutation, plugin still runs):** The override does NOT write to, or otherwise mutate, `hooks-registry.toml` — it is a per-invocation runtime decision only, never a persistent configuration change. The named plugin still executes and its `PluginResult` is still recorded in `per_plugin_results` / logged via the normal `emit_invoked`/lifecycle event path; only that outcome's contribution to the aggregate `block_intent` is suppressed. `TierExecutionSummary.per_plugin_results` for a break-glass-active invocation MUST still contain an entry for the bypassed plugin showing its actual (would-be-blocking) result, not a synthesized "skipped" or "not-run" result.

8. **PC8 — Scope restriction: override has no effect on any plugin other than the two named gates:** Setting `VSDD_BREAK_GLASS_GATE_BYPASS` to the exact `name` of any registered plugin OTHER than `validate-wave-gate-prerequisite` or `validate-pr-merge-prerequisites` (e.g., `validate-factory-path-root`, or any `on_error="continue"` advisory plugin) has NO effect on that other plugin's block decision — the mechanism is hard-scoped to the two named gates at the implementation level, not a generic "bypass any plugin by name" facility. This closes an over-broad-implementation risk: a naive implementation that treats ANY matching plugin name as bypassable would silently defeat fail-closed enforcement for plugins ADR-039 never intended to be bypassable (e.g., `validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`).

9. **PC9 — Mechanical intra-story ordering gate, DUAL-TRIGGER (ADR-039 §Decision 1
    axes-independence + §Decision 3 v1.10 sequencing rule, made CI-checkable, mirroring
    BC-1.03.017 PC11's pattern; corrected Q-A/F-S2123-P2-002, Wave-7 pass-2 remediation):**
    A Cargo integration test (`test_no_agent_gate_fail_closed_flip_without_break_glass`) MUST
    assert: if `hooks-registry.toml` carries, for `validate-wave-gate-prerequisite` OR
    `validate-pr-merge-prerequisites`, EITHER (i) `failure_policy = "fail-closed"` OR (ii)
    `on_error = "block"` — these are TWO INDEPENDENTLY-SUFFICIENT triggers, not a
    conjunction; EITHER ONE ALONE is sufficient to require the break-glass consultation
    code — then that SAME commit tree's `crates/factory-dispatcher/src/executor.rs` MUST
    already contain the break-glass consultation code (a `VSDD_BREAK_GLASS_GATE_BYPASS`-reading
    check in the `execute_tiers` block-decision path, name-independent detection analogous to
    BC-1.03.017 PC11's `.failure_policy`-reference detection). Absence of the break-glass code
    while EITHER trigger is present on EITHER gate MUST cause the test to FAIL (CI blocks
    merge).

    **Why the dual trigger, not `failure_policy` alone (Q-A/F-S2123-P2-002):** the prior PC9
    wording keyed exclusively on `failure_policy = "fail-closed"`, treating it as the sole
    event that makes break-glass necessary. This missed that BOTH named gates ALREADY carry
    `on_error = "block"` in the LIVE `hooks-registry.toml` TODAY, independent of any
    `failure_policy` annotation. Per ADR-039 §Decision 1's axes-independence,
    `on_error = "block"` on its own already blocks on `PluginResult::Crashed` (PC4-class,
    BC-1.03.017) and, under the pre-migration 2-arg decision path, on `Timeout` as well —
    meaning these two Agent gates ALREADY self-lock on a crash or timeout TODAY, with or
    without any `failure_policy` flip. A `failure_policy`-only trigger left this CURRENT,
    ALREADY-LIVE self-lock exposure entirely outside PC9's ordering gate: the break-glass
    consultation code could be completely absent from `executor.rs` today, with both Agent
    gates already capable of a crash/timeout self-lock, and PC9 would report GREEN because no
    `failure_policy` flip had yet occurred. The dual trigger closes this gap — either axis
    being sufficient to self-lock is sufficient to require break-glass — and makes ADR-039
    §Decision 3 v1.10's ordering rule mechanically un-violatable against BOTH self-lock
    exposures, not only the `failure_policy`-flip one.

    The gate MUST include:
    (a) **POSITIVE-CONTROL (`failure_policy` trigger):** synthetic registry with
        `validate-wave-gate-prerequisite` at `failure_policy="fail-closed"` (`on_error`
        irrelevant/absent for this control) + synthetic executor-source snippet WITHOUT the
        break-glass check → assert the gate fires RED.
    (b) **POSITIVE-CONTROL (`on_error` trigger, NEW — Q-A/F-S2123-P2-002):** synthetic
        registry with `validate-pr-merge-prerequisites` at `on_error="block"` and
        `failure_policy` absent/fail-open + synthetic executor-source snippet WITHOUT the
        break-glass check → assert the gate fires RED — proving the `on_error`-only trigger
        is independently sufficient to require break-glass, matching the LIVE registry's
        current state for both named gates.
    (c) **POSITIVE-CONTROL (combined-both-triggers, NEW — Q-A/F-S2123-P2-002):** synthetic
        registry with a named gate carrying BOTH `failure_policy="fail-closed"` AND
        `on_error="block"` simultaneously + synthetic executor-source snippet WITHOUT the
        break-glass check → assert the gate fires RED — proving the disjunction correctly
        fires when both triggers are present together, not merely each tested in isolation.
    (d) **NEGATIVE-CONTROL:** the same three synthetic registry states from (a)/(b)/(c), each
        paired with a synthetic executor-source snippet WITH the break-glass check present →
        assert the gate PASSES for each.
    (e) **VACUITY-CONTROL (corrected — Q-A/F-S2123-P2-002):** synthetic registry with BOTH
        named gates at `failure_policy` absent/fail-open AND `on_error != "block"` (NEITHER
        trigger present) + any executor-source state → assert the gate returns GREEN
        vacuously (neither trigger is present, so the ordering rule is not triggered). This
        is now necessarily a SYNTHETIC-ONLY state — the LIVE registry already carries
        `on_error="block"` for both named gates, so a vacuity fixture keying only on
        `failure_policy` absent (the pre-dual-trigger wording) would be FALSE-VACUOUS against
        the live `on_error="block"` state and must not be used to represent "no trigger
        present."
    (f) **LIVE-TREE-CONTROL:** the detector MUST be run against the actual
        `crates/factory-dispatcher/src/executor.rs` and the actual `hooks-registry.toml`, and
        MUST report the correct ordering state given BOTH triggers' live values. Because both
        named gates ALREADY carry `on_error="block"` live, this control's correct outcome
        TODAY — independent of any `failure_policy` flip — is that the gate reports the
        ordering rule AS ALREADY TRIGGERED, requiring the break-glass consultation code to
        already be present in the live tree. This differs from the pre-dual-trigger reading,
        under which the live-tree gate could report GREEN-vacuously until a `failure_policy`
        flip occurred, silently ignoring the already-live `on_error="block"` self-lock
        exposure.

10. **PC10 — Break-glass arm requires human-operator process-environment access; `Agent`-tool-invoked paths CANNOT arm the override (threat-boundary negative control, F-006, S-21.11 v2.0 adversarial pass-1):**
    `VSDD_BREAK_GLASS_GATE_BYPASS` is armed if and only if it is present in the OS process
    environment of the process that ultimately launches the `factory-dispatcher` binary for a
    given dispatch — i.e., the human operator's shell session (or an ancestor process the
    operator directly controls) that started the Claude Code session. This is a structural
    property of the OS process model, not an application-level access check: a child process's
    environment is a COPY taken at `fork`/`exec` time, and a child mutating its own environment
    (via `export`, `os.environ[...] = ...`, or any other in-process env-setting call) can NEVER
    write back into its parent's environment. Consequently:
    (a) A Bash-tool subprocess spawned by an `Agent`-tool-invoked Claude session is a
        DESCENDANT of the dispatcher's own process tree (or of a sibling session's process
        tree), never an ancestor — it cannot arm the override for that dispatcher invocation or
        any subsequent one, because environment mutations flow parent-to-child only, never
        child-to-parent.
    (b) An MCP tool invocation runs as its own subprocess or distinct server process entirely
        outside the Claude Code harness's own environment-inheritance chain — it has no write
        access to the harness's or the dispatcher's process environment by any mechanism this
        codebase provides.
    (c) A hook-wrapper subprocess (e.g. `legacy-bash-adapter`'s own `exec_subprocess` calls, or
        any other WASM-hosted plugin's host-call subprocess) is itself a CHILD of the dispatcher
        process for that single invocation — the same parent-to-child-only environment-flow
        argument applies; it cannot arm the override for its own dispatch or any future one.

    **Testable assertion:** an integration test
    (`test_break_glass_env_not_settable_by_child_process`) MUST spawn a child process that
    attempts to set `VSDD_BREAK_GLASS_GATE_BYPASS` in its own environment (via
    `std::env::set_var`, or an equivalent shell `export`, executed inside the spawned child)
    and then assert that the PARENT test process's own
    `std::env::var("VSDD_BREAK_GLASS_GATE_BYPASS")` remains unset/unchanged after the child
    process exits — proving the OS-level parent-environment isolation this PC relies on, in the
    actual runtime the dispatcher executes in, not merely asserted as documentation.

    **Explicit threat-boundary note (out of this BC's own enforcement scope, documented per
    F-006):** this PC does NOT claim the dispatcher validates WHO set the variable in its
    inherited environment. If the human operator's own shell session is itself compromised
    (e.g., a malicious `.bashrc`, a compromised SSH session, or a supply-chain-compromised CLI
    tool running with the operator's own shell privileges), that compromised process COULD set
    the variable before launching Claude Code, and the dispatcher would inherit it exactly as it
    would inherit a legitimate operator's setting — Invariant 1's
    "authentication-by-possession, not identity" scope already documents this as a deliberate,
    accepted design boundary, and PC10 does not change that scope. What PC10 closes is
    narrower and precise: the `Agent` tool dispatch path ITSELF — the mechanism this factory's
    own AI agents use to take action — provides no route to arm the override, by construction
    of the OS process model, regardless of what any `Agent`-invoked tool call attempts.

11. **PC11 — Audit fail-closed-on-failure: suppression is granted ONLY IF the
    `break_glass.activated` audit write DURABLY completes (Q3, F-S2123-P1-001; ADR-039
    §AMD-004, v1.16, RATIFIED 2026-08-22, architect adjudication Q3 via orchestrator relay):**
    When PC2 or PC3 would otherwise suppress a gate's block contribution for a given invocation,
    that suppression is granted ONLY IF the corresponding `break_glass.activated` audit write
    (PC5/PC6) is CONFIRMED DURABLY WRITTEN — i.e., `InternalLog::write_checked` (the new,
    additive, tri-state method ADR-039 §AMD-004 mandates, added ALONGSIDE the existing
    best-effort `InternalLog::write`, which remains UNCHANGED for its ~6 other hot-path
    telemetry callers such as `dispatcher.started`/`plugin.invoked`/`plugin.completed`) returns
    `Written`. If the audit write FAILS (`write_checked` returns `Failed(io::Error)` — an I/O
    error) OR is SKIPPED (`write_checked` returns `SkippedMountGate` — the `.factory/` mount-gate
    skip path, which the pre-existing best-effort `write()`/`write_inner` conflates with a
    successful write, since both return `Ok(())`), the override MUST NOT be applied for that
    gate on that invocation — the underlying gate's block decision (from `plugin_requests_block`
    or the exhaustion/error-exit decision functions BC-1.03.017 governs) stands, computed
    EXACTLY as if the override were unset (PC1 baseline semantics), even though the override
    was set and named this gate.
    This is the fail-closed-on-audit-failure posture: an audit write that cannot be confirmed
    durable MUST NOT silently grant a bypass, because a break-glass activation whose audit
    trail is dropped — precisely during a live self-lock, the scenario this mechanism exists to
    recover from — reproduces exactly the CWE-636 silent-approval failure mode PC5/PC6's own
    mandatory-audit requirement exists to prevent, one layer down. This is the same
    deny-on-uncertainty architectural bias already governing every other decision in ADR-039.
    **Testable assertion:** an integration test (`test_break_glass_denied_on_unwritable_audit_sink`)
    MUST construct a scenario where the audit sink is unwritable (simulated I/O error) or
    mount-gated (simulated pre-mount `.factory/` state), set `VSDD_BREAK_GLASS_GATE_BYPASS` to
    name a blocking gate, and assert `block_intent` for that gate is UNCHANGED from the
    no-override baseline (i.e., suppression denied) — proving the gating is enforced against
    the ACTUAL `write_checked` result at the break-glass call site, not merely documented as a
    design intent.
    **Call-site scope, not a change to `InternalLog::write`'s general contract:**
    `InternalLog::write`'s existing best-effort, error-swallowing contract is CORRECT and
    UNCHANGED for its other callers — making the general writer blocking or
    failure-propagating would risk hanging/crashing the dispatcher on ordinary telemetry, a
    strictly worse outcome than the narrow gap this postcondition closes. This postcondition's
    gating applies ONLY at the break-glass call site, which alone consults `write_checked`'s
    tri-state result to decide whether to honor the override (ADR-039 §AMD-004's "wiring not
    redesign" carve-out — `InternalLog::write_checked` itself is implemented in S-21.23's
    Phase-3 TDD scope, governed by this postcondition as its behavioral-contract leg).

12. **PC12 — Audit-loss observability, independent of suppression outcome (Q-B,
    F-S2123-P2-006, Wave-7 pass-2 remediation):** Whenever `InternalLog::write_checked` is
    invoked for a `break_glass.activated` audit attempt and returns other than `Written` (i.e.
    `Failed(io::Error)` or `SkippedMountGate`), the dispatcher MUST emit a distinct immediate
    diagnostic to the same fallback channel `InternalLog::write` already uses on its own
    internal errors (`eprintln!`/stderr), identifying: the matched gate name, the dispatch's
    trace UUID (`dispatcher_trace_id`), and the `write_checked` failure variant
    (`Failed(io::Error)` vs. `SkippedMountGate`).
    **This emission is UNCONDITIONAL on whether the invocation was blocking** — it fires
    identically in both of the following cases, which PC11 alone does not distinguish:
    (a) **PC11's suppression-denial case:** the matched gate WOULD have blocked, the audit
        write fails/is skipped, and PC11 denies suppression — here the diagnostic is a
        secondary signal alongside the already-observable non-suppressed block.
    (b) **EC-004's counterfactual-clean-pass case:** the matched gate would NOT have blocked
        (a clean pass), the audit write fails/is skipped, and PC11's suppression-denial logic
        is a no-op because there was no block to deny. In this case `block_intent` is
        unaffected either way (exit 0 with or without a successful write), so WITHOUT this
        diagnostic, a failed/skipped audit write on a clean-pass matched invocation is
        SILENT — it drops the PC6-mandated audit record with no other observable signal
        distinguishing it from a normal clean pass with a successful write. PC12 closes this
        residual observability gap: the diagnostic is the ONLY observable artifact of the
        dropped record in case (b).
    **The diagnostic is itself best-effort — the regress terminates here.** This
    `eprintln!`/stderr emission is NOT itself gated on a further confirmed-write check (that
    would recurse indefinitely); it is a best-effort, immediate, synchronous fallback signal,
    matching the same best-effort posture `InternalLog::write` already uses for its own
    internal I/O errors. PC12 does not alter PC11's suppression-decision semantics in any way
    — it adds an independent, non-gating observability signal alongside it.
    **Testable assertion:** an integration test MUST assert that when
    `InternalLog::write_checked` is simulated to return `Failed`/`SkippedMountGate` for a
    matched-but-counterfactually-clean-pass invocation (EC-008), captured stderr output
    contains the diagnostic (gate name, trace UUID, failure variant) even though
    `block_intent` is unchanged (exit 0) — proving the diagnostic fires independent of the
    suppression-decision outcome, not only in PC11's suppression-denial case.

## Invariants

1. **Authentication-by-possession, human-operator-only:** The override is read directly from process environment by the dispatcher's own gate-evaluation code path. No `Agent`-tool-invoked call, no WASM plugin payload field, and no `hooks-registry.toml` content can set or read this variable to self-approve a bypass — only a human operator with direct shell/process-environment access to the machine running the session can activate it. This factory has no separate credential/token layer for hook bypass; requiring one would be new infrastructure out of proportion to a break-glass escape hatch (ADR-039 §Decision 3 v1.10 MVD). PC10 makes the "no `Agent`-tool-invoked path can arm it" half of this invariant a testable assertion plus an explicit threat-boundary note (F-006): possession-based means possession of the OPERATOR's own process environment, not possession of any Agent-tool capability, which structurally cannot reach it (child-to-parent environment mutation is impossible under the OS process model).

2. **Non-agent-mediated (self-lock-prevention) invariant:** Setting the override does NOT require a working `Agent` tool dispatch — the shell-environment-set path never routes through the dispatcher's `Agent`-tool PreToolUse gate chain. This is the structural property that makes the mechanism usable exactly when the self-lock it is designed to escape has already occurred.

3. **Audit-mandatory, no-silent-bypass invariant (CWE-636 lineage closure at the override layer):** Every activation (PC2/PC3) produces a `break_glass.activated` event (PC5/PC6). An unaudited bypass would reintroduce, at the override layer, the exact "silent approval" failure mode (CWE-636) that BC-1.03.017's fail-closed enforcement exists to close at the exhaustion-decision layer.

4. **Non-persistent, per-invocation-only invariant:** The override never mutates `hooks-registry.toml` and never disables a plugin's own validation logic (PC7). It is strictly a per-invocation decision-outcome override.

5. **Scope-restriction invariant:** The override affects exactly the two named gates and no others (PC8). It is not a generic plugin-bypass facility.

6. **Intra-story ordering invariant, DUAL-TRIGGER (parallel to BC-1.03.017 Invariant 7;
   corrected Q-A/F-S2123-P2-002, Wave-7 pass-2):** The break-glass mechanism (this BC's
   PC1–PC8) MUST be present in any commit tree where either named gate carries EITHER
   `failure_policy = "fail-closed"` OR `on_error = "block"` (PC9's two independently-sufficient
   triggers) — not `failure_policy` alone. Since both named gates already carry
   `on_error = "block"` in the live registry, this invariant is ALREADY triggered today,
   independent of any `failure_policy` flip. This is the human-readable statement of ADR-039
   §Decision 1's axes-independence combined with §Decision 3 v1.10's sequencing rule; PC9 is
   its mechanically-enforced complement.

7. **Audit-durability-gates-suppression invariant (CWE-636 lineage closure at the audit layer;
   Q3, F-S2123-P1-001; ADR-039 §AMD-004):** Invariant 3's audit-mandatory guarantee ("every
   activation produces a `break_glass.activated` event") is necessary but not, by itself,
   sufficient — it describes the EMISSION obligation (PC5/PC6), not whether the emitted write
   durably lands. This invariant (PC11's human-readable statement) closes that residual gap: in
   the ABSENCE of a durably-confirmed audit write (`InternalLog::write_checked` returning
   `Written`), the override has NO effect on `block_intent` — deny-on-uncertainty, not
   approve-on-attempt. An audit-emission ATTEMPT that fails or is silently mount-gate-skipped
   MUST NOT be treated as equivalent to a successful, durable audit write for the purpose of
   deciding whether to honor a break-glass suppression. This is the same fail-closed
   architectural bias governing every other decision in ADR-039, applied one layer down from
   Invariant 3's emission-only guarantee.

8. **Audit-loss observability invariant, independent of suppression outcome (Q-B,
   F-S2123-P2-006):** PC11's fail-closed-on-audit-failure posture correctly denies
   suppression when the audit write fails/is skipped for a WOULD-HAVE-BLOCKED matched
   invocation, but is a no-op — and therefore silent — for a counterfactually-clean-pass
   matched invocation (EC-004/EC-008), where there is no block to deny. This invariant
   (PC12's human-readable statement) closes that residual silence: a failed/skipped audit
   write MUST always produce SOME observable diagnostic, regardless of whether the matched
   invocation was blocking, so that a dropped `break_glass.activated` record is never purely
   silent in either case. The diagnostic itself remains best-effort (no further gating), same
   as `InternalLog::write`'s existing internal-error posture.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Override set but malformed (e.g., only whitespace, or a stray trailing comma producing an empty token) | Malformed/empty tokens are treated as non-matching (no bypass for that token); a fully-empty-after-trim value is equivalent to unset (PC1 baseline); dispatcher MUST NOT crash or panic on malformed input |
| EC-002 | Override contains both named gates explicitly, comma-separated, without using `all` (e.g., `validate-wave-gate-prerequisite,validate-pr-merge-prerequisites`) | Both gates bypassed — behaviorally equivalent to `VSDD_BREAK_GLASS_GATE_BYPASS=all`, proving the list-form and the `all`-form converge on identical coverage for the two-gate case |
| EC-003 | Override value has case variation (e.g., `All`, `VALIDATE-WAVE-GATE-PREREQUISITE`) | Matching is exact-case (case-sensitive), matching the plugin `name` field's own case sensitivity in `hooks-registry.toml`; a case-mismatched value does NOT bypass (falls under PC4's negative-control behavior) — this is a deliberate, testable design decision, not left ambiguous |
| EC-004 | Override active; the named gate's plugin would have produced a CLEAN PASS (no block) even without the override | `break_glass.activated` is still emitted per PC6 (audit fires on override-active-for-matched-invocation, not conditioned on a counterfactual block); dispatcher's observable exit code/block_intent is unchanged from the no-override case (both are exit 0) — PC6's audit-independent-of-counterfactual behavior is the only observable difference |
| EC-005 | Both named gates dispatched (in separate `execute_tiers` tiers — priority 130 vs 120 — evaluated unconditionally within the same dispatch via `execute_tiers`'s no-early-return tier loop; corrected F-004, S-21.11 v2.0 adversarial pass-1 — the prior "SAME tier" premise was false per live `hooks-registry.toml`: `validate-wave-gate-prerequisite` is priority 130, `validate-pr-merge-prerequisites` is priority 120, and `routing.rs::group_by_priority` starts a new tier on every distinct priority value), override set to `all`, both would have blocked | Both gates' contributions suppressed independently; TWO separate `break_glass.activated` events emitted (one per gate name), each carrying its own gate name and the shared dispatch trace UUID — not a single combined event, so per-gate audit granularity is preserved. This conclusion does not depend on, and is not evidenced by, the two gates sharing a tier — it follows from `execute_tiers` looping over every tier unconditionally (no early return on a block-producing tier), so both gates are always invoked and both outcomes always folded into `block_intent` within one dispatch, regardless of tier membership |
| EC-006 | Override set; the underlying plugin CRASHES (`PluginResult::Crashed`, `on_error = Block`) rather than exhausting a resource budget | Crash-caused block is ALSO suppressed by the override (PC2's "regardless of which decision path... and regardless of the plugin's own `PluginResult` variant" — the override is outcome-level, not exhaustion-specific; it is not limited to `plugin_fail_closed`'s exhaustion path) |
| EC-007 | Override matches a named gate that would otherwise block; the `break_glass.activated` audit write to `.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl` FAILS (simulated I/O error) OR is SKIPPED (simulated `.factory/` mount-gate-not-ready state) — i.e., `InternalLog::write_checked` returns `Failed(io::Error)` or `SkippedMountGate`, not `Written` (Q3, F-S2123-P1-001, ADR-039 §AMD-004) | Suppression is DENIED — the gate's block decision stands EXACTLY as if the override were unset (PC1 baseline); `block_intent=true`/exit 2 (or whatever the underlying gate's decision would otherwise produce) is UNCHANGED despite the override naming this gate. This is the fail-closed-on-audit-failure posture (PC11) — an unconfirmed audit write must not silently grant a bypass, precisely because this is most likely to occur during the live self-lock scenario break-glass exists to recover from |
| EC-008 | Override matches a gate that would NOT have blocked (clean pass, EC-004) AND the `break_glass.activated` audit write FAILS or is SKIPPED (Q-B, F-S2123-P2-006) | `block_intent` UNCHANGED (`false`, exit 0) — PC11's suppression-denial logic is a no-op here (there was no block to deny), so this case is observably IDENTICAL to a normal clean pass at the `block_intent`/exit-code level. BUT the PC12 diagnostic MUST fire (gate name, trace UUID, failure variant to stderr) — this is the ONLY observable artifact distinguishing this case from a normal clean pass with a successful audit write; without it, the dropped `break_glass.activated` record would be silently lost with no observable trace at all |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `VSDD_BREAK_GLASS_GATE_BYPASS` unset; `validate-wave-gate-prerequisite` returns `PluginResult::Timeout{Epoch}` + `failure_policy=FailClosed` | `block_intent=true`; exit 2 (BC-1.03.017 PC6 semantics unmodified) | happy-path (PC1 baseline regression control) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=validate-wave-gate-prerequisite`; same plugin returns `PluginResult::Timeout{Epoch}` + `failure_policy=FailClosed` | `block_intent` contribution from this gate suppressed → exit 0 for this gate's contribution; `break_glass.activated` event emitted with gate name, timestamp, trace UUID | happy-path (PC2 positive control) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=all`; both `validate-wave-gate-prerequisite` and `validate-pr-merge-prerequisites` return blocking outcomes | Both gates' contributions suppressed; TWO `break_glass.activated` events emitted (one per gate) | happy-path (PC3 / EC-005) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=validate-factory-path-root` (non-matching name); `validate-wave-gate-prerequisite` returns a blocking outcome | `validate-wave-gate-prerequisite`'s block is NOT suppressed; `block_intent=true`; exit 2; NO `break_glass.activated` event for `validate-wave-gate-prerequisite` | negative-control (PC4) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=validate-wave-gate-prerequisite`; `validate-pr-merge-prerequisites` (a different gate, not named) returns a blocking outcome | `validate-pr-merge-prerequisites`'s block is NOT suppressed (scope restriction — override is per-name, not global) | negative-control (PC8) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=validate-wave-gate-prerequisite`; plugin returns a CLEAN PASS (no block) | `break_glass.activated` event STILL emitted (audit fires on matched invocation, independent of counterfactual block); observable exit code unchanged (exit 0 either way) | edge-case (PC6 / EC-004) |
| `hooks-registry.toml` sets `failure_policy="fail-closed"` for `validate-pr-merge-prerequisites` (`on_error` irrelevant/absent for this control); synthetic executor-source snippet WITHOUT break-glass consultation code | `test_no_agent_gate_fail_closed_flip_without_break_glass` FAILS (RED) | migration-window-gate (PC9 POSITIVE-CONTROL, `failure_policy` trigger) |
| `hooks-registry.toml` sets `on_error="block"` for `validate-wave-gate-prerequisite` (`failure_policy` absent/fail-open — matches the LIVE registry's current state); synthetic executor-source snippet WITHOUT break-glass consultation code | `test_no_agent_gate_fail_closed_flip_without_break_glass` FAILS (RED) — proves the `on_error`-only trigger is independently sufficient | migration-window-gate (PC9 POSITIVE-CONTROL, `on_error` trigger, NEW — Q-A/F-S2123-P2-002) |
| `hooks-registry.toml` sets BOTH `failure_policy="fail-closed"` AND `on_error="block"` simultaneously for `validate-pr-merge-prerequisites`; synthetic executor-source snippet WITHOUT break-glass consultation code | `test_no_agent_gate_fail_closed_flip_without_break_glass` FAILS (RED) — proves the disjunction fires when both triggers co-occur | migration-window-gate (PC9 POSITIVE-CONTROL, combined-both-triggers, NEW — Q-A/F-S2123-P2-002) |
| Any of the three registry states above; synthetic executor-source snippet WITH break-glass consultation code present | `test_no_agent_gate_fail_closed_flip_without_break_glass` PASSES for each | migration-window-pass (PC9 NEGATIVE-CONTROL) |
| Both named gates at `failure_policy` absent/fail-open AND `on_error != "block"` (NEITHER trigger present — a synthetic-only state, since the live registry already carries `on_error="block"` for both gates); any executor-source state | Gate returns GREEN vacuously (neither trigger is present, so the ordering rule is not triggered) | migration-window-vacuity (PC9 VACUITY-CONTROL, corrected Q-A/F-S2123-P2-002 — dual-trigger-neither-present, not merely `failure_policy`-absent) |
| Live tree: actual `crates/factory-dispatcher/src/executor.rs` and actual `hooks-registry.toml` (both named gates carry `on_error="block"` today, independent of any `failure_policy` flip) | Detector reports the ordering rule AS ALREADY TRIGGERED (dual-trigger, Q-A/F-S2123-P2-002) — the break-glass consultation code MUST already be present in the live tree today, not only once a `failure_policy` flip lands | migration-window-live-tree (PC9 LIVE-TREE-CONTROL) |
| Child process attempts `std::env::set_var("VSDD_BREAK_GLASS_GATE_BYPASS", "all")` (or shell `export`) inside its own environment, then exits | Parent test process's `std::env::var("VSDD_BREAK_GLASS_GATE_BYPASS")` remains unset/unchanged — `test_break_glass_env_not_settable_by_child_process` PASSES, proving no Agent-tool-invoked path (Bash-tool subprocess, MCP tool invocation, hook-wrapper subprocess) can arm the override for the dispatcher's own process environment | threat-boundary (PC10, F-006, ADR-039 §Decision 3 human-operator-only scope) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=validate-wave-gate-prerequisite`; that gate returns a blocking outcome; `InternalLog::write_checked` simulated to return `Failed(io::Error)` (unwritable audit sink) | Suppression DENIED — `block_intent=true`; exit 2 (gate's block decision stands, unchanged from the no-override baseline); NO durable `break_glass.activated` record confirmed; `test_break_glass_denied_on_unwritable_audit_sink` PASSES | fail-closed-on-audit-failure (PC11 POSITIVE control, Q3/F-S2123-P1-001, ADR-039 §AMD-004, EC-007) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=validate-pr-merge-prerequisites`; that gate returns a blocking outcome; `InternalLog::write_checked` simulated to return `SkippedMountGate` (`.factory/` mount not yet ready) | Suppression DENIED — `block_intent=true`; exit 2 (identical outcome to the `Failed` case — a skip is NOT treated as equivalent to a durable write); `test_break_glass_denied_on_unwritable_audit_sink` PASSES | fail-closed-on-audit-failure (PC11 POSITIVE control, mount-gate-skip sub-case, EC-007) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=validate-wave-gate-prerequisite`; that gate returns a blocking outcome; `InternalLog::write_checked` returns `Written` (audit sink healthy) | Suppression GRANTED — `block_intent` contribution suppressed → exit 0 for this gate's contribution (PC2 baseline behavior restored once the audit write is confirmed durable) | negative-control (PC11 NEGATIVE-CONTROL — durable-write case, confirms PC11 does not over-block a healthy audit path) |
| `VSDD_BREAK_GLASS_GATE_BYPASS=validate-wave-gate-prerequisite`; that gate returns a CLEAN PASS (would NOT have blocked, EC-004); `InternalLog::write_checked` simulated to return `Failed(io::Error)` | `block_intent` UNCHANGED (exit 0 — identical to a normal clean pass at the block_intent/exit-code level); captured stderr output MUST contain the PC12 diagnostic (gate name, trace UUID, `Failed` variant) — the only observable trace of the dropped audit record | audit-loss-observability (PC12 POSITIVE control, EC-008, Q-B/F-S2123-P2-006) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | For the two named PreToolUse `^Agent$` gates: `VSDD_BREAK_GLASS_GATE_BYPASS` unset/non-matching → no behavioral change from BC-1.03.017 baseline (PC1/PC4); exact-name or `all` match → that gate's block contribution suppressed for that invocation regardless of block-decision path or `PluginResult` variant (PC2/PC3/EC-006); every matched-and-invoked case emits `break_glass.activated` to dispatcher-internal JSONL with gate name(s)+timestamp+trace UUID, independent of counterfactual block (PC5/PC6); override never mutates `hooks-registry.toml` and never disables plugin execution (PC7); override has zero effect on any plugin other than the two named gates (PC8); no commit tree may carry, for either named gate, EITHER `failure_policy="fail-closed"` OR `on_error="block"` without the break-glass consultation code already present (PC9, DUAL-TRIGGER six-control gate parallel to BC-1.03.017 PC11, corrected Q-A/F-S2123-P2-002 — since both named gates already carry `on_error="block"` live, this rule is already triggered today, independent of any `failure_policy` flip). **PC10 (threat-boundary, F-006):** no `Agent`-tool-invoked path (Bash-tool subprocess, MCP tool invocation, hook-wrapper subprocess) can arm `VSDD_BREAK_GLASS_GATE_BYPASS` for the dispatcher's own process environment — child-to-parent environment mutation is impossible under the OS process model; explicit threat-boundary note documents the accepted out-of-scope case (a compromised operator shell session inherits exactly as a legitimate one would). **PC11 (audit fail-closed-on-failure, Q3/F-S2123-P1-001, ADR-039 §AMD-004):** suppression (PC2/PC3) is granted ONLY IF the corresponding `break_glass.activated` audit write is CONFIRMED DURABLY WRITTEN via `InternalLog::write_checked` returning `Written`; if the write FAILS (`Failed(io::Error)`) or is SKIPPED (`SkippedMountGate`), the override MUST NOT be applied and the underlying gate's block decision stands exactly as if the override were unset — this closes the residual CWE-636 gap where an audit-emission ATTEMPT (PC5/PC6) silently fails to durably land. **PC12 (audit-loss observability, Q-B/F-S2123-P2-006):** whenever `InternalLog::write_checked` returns other than `Written` for a `break_glass.activated` audit attempt, the dispatcher MUST emit a distinct immediate stderr diagnostic (gate name, trace UUID, failure variant) — UNCONDITIONAL on whether the invocation was blocking, so it fires identically in PC11's suppression-denial case AND EC-004/EC-008's counterfactual-clean-pass case, closing the residual silence where a failed/skipped audit write on a clean-pass matched invocation would otherwise leave no observable trace at all. | unit tests (executor break-glass-check path coverage per PC1-PC4, PC7, PC8) + integration test asserting captured dispatcher-internal JSONL output contains `break_glass.activated` with required fields (PC5/PC6, Envoy #38801 discipline — behavioral audit assertion, not configuration assertion) + Cargo gate test with SIX controls, dual-trigger (PC9, Q-A/F-S2123-P2-002 — `failure_policy` trigger, `on_error` trigger, combined-both-triggers, negative, vacuity, live-tree) + child-process-spawning integration test asserting parent-environment isolation (`test_break_glass_env_not_settable_by_child_process`; PC10) + integration test asserting suppression is denied when `InternalLog::write_checked` returns `Failed`/`SkippedMountGate`, and granted when it returns `Written` (`test_break_glass_denied_on_unwritable_audit_sink`; PC11) + integration test asserting the PC12 stderr diagnostic fires on a matched-clean-pass invocation with a failed/skipped audit write, independent of `block_intent` (PC12, EC-008) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-039 |
| Capability Anchor Justification | CAP-039 ("Break-glass operator override for self-locking PreToolUse `^Agent$` gates") per capabilities.md §CAP-039 — CAP-039 was authored in this same burst specifically to anchor this BC; its outcome text ("an operator whose `Agent` dispatch is wedged by a fail-closed `validate-wave-gate-prerequisite` or `validate-pr-merge-prerequisites` block can set `VSDD_BREAK_GLASS_GATE_BYPASS`... with the bypass event durably recorded in the audit log") is verbatim the behavior this BC's PC1-PC9 specify. No pre-existing capability fit: CAP-002 (normal hook block/allow routing) does not cover an override of that decision; CAP-008 (Bash-tool PreToolUse gating) is a distinct tool/check class; CAP-011 (fuel/epoch budget enforcement) is the mechanism this BC provides an escape hatch FROM, not a restatement of it; CAP-031's "break-glass" `/factory-unlock --force` term is a same-name, distinct-concern precedent (factory-lock/lease exclusivity, not validator-gate self-lock escape). |
| L2 Domain Invariants | TBD (no CAP-039-adjacent DI-NNN currently exists in `domain-spec/invariants.md`; this BC's audit-mandatory postconditions (PC5/PC6, and PC11's audit-DURABILITY gating) are structurally analogous to DI-004's "capability denial always produces both a return code AND an audit event — one without the other is a bug" pattern, though DI-004 itself governs a different subsystem surface (capability-gated host functions) and is not cited as enforcing this BC) |
| Architecture Module | SS-01 (Hook Dispatcher Core) — `crates/factory-dispatcher/src/executor.rs::execute_tiers` block-decision loop; no schema change to `plugins/vsdd-factory/hooks-registry.toml` is required (the override is env-var-driven, not registry-driven); PC11 additionally anchors `crates/factory-dispatcher/src/internal_log.rs::InternalLog::write_checked` (new, additive, tri-state method — `Written`/`SkippedMountGate`/`Failed(io::Error)` — added alongside the existing best-effort `InternalLog::write`, which is unchanged for its other callers) |
| ADR | ADR-039 §Decision 3 v1.9 amendment (break-glass requirement first introduced for the two PreToolUse `^Agent$` gates); ADR-039 §Decision 3 v1.10 amendment (RATIFIED, POLICY 22, 2026-08-19 — concrete minimum-viable definition: environment-variable override, human-operator-only, audited via JSONL; intra-story ordering constraint: break-glass commit precedes or is atomic with the fail-closed-flip commit for these two gates within S-21.11) — this BC IS that minimum-viable definition's behavioral contract. **ADR-039 §AMD-004 (v1.16, RATIFIED 2026-08-22 — architect adjudication Q3, via orchestrator relay, POLICY 22 ratification-channel; S-21.23 F-001, HIGH, pre-TDD adversarial pass-1, Wave-7 split of CONVERGED S-21.11): break-glass suppression MUST be fail-closed-on-audit-failure — an audit write that fails or is skipped (mount-gate) MUST NOT permit the override; the mandated audit sink (`InternalLog::write`) is, by contract, best-effort and swallows I/O errors while also returning `Ok(())` on a mount-gate skip, so a caller cannot distinguish "durably written" from "silently dropped"/"silently skipped" from that `Result` alone; this amendment mandates the additive `InternalLog::write_checked` tri-state method (added ALONGSIDE, not replacing, the existing best-effort `write()`) and requires the break-glass call site alone to gate suppression on its result — **PC11 (v1.2) is this BC's behavioral-test leg for §AMD-004's ruling; the `InternalLog::write_checked` method itself is S-21.23 Phase-3 TDD implementer scope**.** **ADR-039 §Decision 1 (axes-independence — `on_error` governs crash/host-error outcomes independently of `failure_policy`; Q-A/F-S2123-P2-002, Wave-7 pass-2, RATIFIED 2026-08-22): both named Agent gates ALREADY carry `on_error="block"` in the LIVE registry today, independent of any `failure_policy` annotation, and therefore ALREADY self-lock on crash/timeout under §Decision 1's axes-independence — **PC9 (v1.3) is this BC's DUAL-TRIGGER correction: the intra-story ordering gate now keys on EITHER `failure_policy="fail-closed"` OR `on_error="block"`, independently sufficient, closing the gap where a `failure_policy`-only trigger left the already-live `on_error="block"` self-lock exposure outside the gate's coverage**.** **F-S2123-P2-006 (Q-B, Wave-7 pass-2, audit-loss observability): PC11's suppression-denial is a no-op on a counterfactual clean-pass matched invocation, silently dropping the PC6-mandated audit record when the write additionally fails/is skipped — **PC12 (v1.3) is this BC's behavioral-test leg closing that residual silence with an unconditional stderr diagnostic**.** Sibling: BC-1.03.017 (governs the `failure_policy`/`on_error` exhaustion-decision axes this BC's override suppresses the OUTCOME of, without altering the decision function itself) |
| Security | CWE-636 lineage (Not Failing Securely) closure-at-the-override-layer: PC5/PC6's mandatory-audit requirement prevents the override itself from becoming a new silent-approval vector; PC9's DUAL-TRIGGER correction (Q-A/F-S2123-P2-002) closes a self-lock exposure gap that was ALREADY LIVE (both Agent gates already carry `on_error="block"`, independent of any `failure_policy` flip); PC11 (§AMD-004) closes the residual gap where the audit EMISSION attempt itself silently fails or is skipped, one layer down; PC12 (Q-B/F-S2123-P2-006) closes the remaining observability gap where PC11's suppression-denial is a silent no-op on a counterfactual clean-pass invocation. Kubernetes/GKE/OPA Gatekeeper admission-webhook self-deadlock precedent (named hazard class; narrow authenticated exemption over blanket fail-open) per ADR-039 §Decision 3 v1.9 |
| Stories | S-21.11 (original unified delivery vehicle) → PC1–PC8 delivered within S-21.11's absorbed scope (break-glass, AMD-002 wiring fix, per-plugin `timeout_ms` calibration, gated fail-closed flip; the prior follow-up name S-21.17 is retired); PC10 delivered as part of S-21.11 v2.0 adversarial pass-1 remediation (F-006); **PC9's DUAL-TRIGGER correction (Q-A/F-S2123-P2-002), PC11 (Q3/F-S2123-P1-001, ADR-039 §AMD-004), and PC12 (Q-B/F-S2123-P2-006) are S-21.23's Phase-3 TDD implementer scope — S-21.23 is the Wave-7 split-topology sub-story of CONVERGED S-21.11 governing this BC's break-glass ordering/audit/durability/observability leg** |
| Cycle | v1.0-brownfield-backfill |

## Related BCs

- **BC-1.03.017** — sibling: governs the `failure_policy`/`on_error` exhaustion-enforcement decision (block-vs-advisory dispatch) for the same two `legacy-bash-adapter.wasm`-hosted PreToolUse `^Agent$` gates among its six targeted plugins; BC-1.03.018 overrides the OUTCOME that decision produces for these two gates specifically, without altering BC-1.03.017's decision function. PC9's ordering gate is intra-story-coupled to BC-1.03.017 PC9's fail-closed-flip completion for these two plugins.
- **BC-1.03.009** — sibling block-intent: governs `block_intent` for the `HookResult::Block` path (`plugin_requests_block`); BC-1.03.018's override suppresses this path's contribution for the two named gates in addition to `plugin_fail_closed`'s exhaustion path (PC2).

## Architecture Anchors

- `crates/factory-dispatcher/src/executor.rs::execute_tiers` — the block-decision aggregation loop (`for outcome in &tier_outcomes { if plugin_requests_block(...) || plugin_fail_closed(...) { block_intent = true; } }`); the break-glass check MUST be consulted per-`RegistryEntry.name`, scoped to `validate-wave-gate-prerequisite` and `validate-pr-merge-prerequisites` only, before either disjunct's result is folded into `block_intent` for these two entries
- `plugins/vsdd-factory/hooks-registry.toml` — the two named entries (`validate-pr-merge-prerequisites` priority 120, `validate-wave-gate-prerequisite` priority 130; both `event = "PreToolUse"`, `tool = "^Agent$"`, `plugin = "hook-plugins/legacy-bash-adapter.wasm"`); this BC requires NO schema change to this file — the override is env-var-driven, not registry-driven
- `crates/factory-dispatcher/src/vsdd_sink.rs`, `crates/factory-dispatcher/src/log_dir.rs`, `crates/factory-dispatcher/src/main.rs` — existing `std::env::var` read precedent (`VSDD_SINK_FILE`, `VSDD_LOG_DIR`, `FACTORY_ROOT`) that this BC's `VSDD_BREAK_GLASS_GATE_BYPASS` read follows as an established codebase convention for environment-variable-driven dispatcher behavior
- `.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl` — `break_glass.activated` audit event sink (same event-sink family already documented under "VSDD_SINK_FILE diagnostic capture" in CLAUDE.md; internal-log lifecycle events analogous to existing `plugin.log`/`plugin.crashed`/`plugin.timeout` record types)
- BC-1.03.017 Architecture Anchors (`executor.rs`, `hooks-registry.toml`) — shared anchors; this BC's changes are additive to the same block-decision loop, not a replacement of BC-1.03.017's decision function
- OS process model (`fork`/`exec` environment-copy semantics; no codebase file — a platform-level guarantee, not an in-repo mechanism) — PC10's threat-boundary basis: a child process's environment mutation can never propagate back to its parent; this is what makes an `Agent`-tool-invoked Bash-tool subprocess, MCP tool invocation, or hook-wrapper subprocess structurally unable to arm `VSDD_BREAK_GLASS_GATE_BYPASS` for the dispatcher's own process
- ADR-039 §Decision 3 v1.10 amendment — source of PC1-PC9's minimum-viable definition; F-S2111V2-P1-001-mechanism-adjudication memo (`.factory/cycles/v1.0-brownfield-backfill/F-S2111V2-P1-001-mechanism-adjudication.md`) — source of PC10's F-006 threat-boundary directive and EC-005's F-004 correction
- `crates/factory-dispatcher/src/internal_log.rs::InternalLog::write_checked` — PC11's wiring
  target (ADR-039 §AMD-004, RATIFIED v1.16): a new, additive method returning a tri-state
  outcome (`Written` / `SkippedMountGate` / `Failed(io::Error)`), added ALONGSIDE the existing
  best-effort `InternalLog::write`/`write_inner` (which is UNCHANGED and remains the general
  hot-path telemetry sink for its ~6 other callers — `dispatcher.started`, `plugin.invoked`,
  `plugin.completed`, and others). The break-glass call site ALONE consults `write_checked`'s
  result to gate suppression (PC11); every other `InternalLog::write` caller is unaffected by
  this BC

## Story Anchor

S-21.11 — validator exhaustion fail-closed calibration and enforcement (expanded, unified scope: AMD-002 wiring fix + per-plugin `timeout_ms` calibration + break-glass mechanism + gated fail-closed flip); **S-21.23 — Wave-7 split-topology sub-story of CONVERGED S-21.11, owning PC9's DUAL-TRIGGER ordering-gate correction (Q-A, ADR-039 §Decision 1), PC11's `InternalLog::write_checked` implementation and the break-glass call-site audit-durability gating (Q3, ADR-039 §AMD-004), and PC12's audit-loss stderr diagnostic (Q-B) — all Phase-3 TDD scope**

## VP Anchors

- VP-TBD — break-glass override dispatch: all twelve postconditions exercised by behavioral tests driving the actual `execute_tiers` path with real env-var state; mandatory audit-event assertion against captured JSONL output; DUAL-TRIGGER six-control intra-story ordering gate (PC9, corrected Q-A/F-S2123-P2-002 — `failure_policy` OR `on_error` independently sufficient); child-process parent-environment-isolation threat-boundary test (PC10); audit-write-durability-gates-suppression test against simulated `Failed`/`SkippedMountGate`/`Written` `InternalLog::write_checked` outcomes (PC11, Q3/ADR-039 §AMD-004); audit-loss-observability stderr-diagnostic test, unconditional on suppression outcome (PC12, Q-B/F-S2123-P2-006) — the real VP is anchored to Phase-6 formal-verifier for VP-NNN assignment and propagation to VP-INDEX/verification-architecture.md/verification-coverage-matrix.md per `vp_index_is_vp_catalog_source_of_truth`

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.3 | 2026-08-22 | product-owner | Wave-7 adversary pass-2 remediation, cycle v1.0-brownfield-backfill, applying the architect's adjudicated language: (Q-A, F-S2123-P2-002) PC9 rewritten to a DUAL-TRIGGER ordering gate — the prior wording keyed exclusively on `failure_policy="fail-closed"`, missing that both named Agent gates ALREADY carry `on_error="block"` in the LIVE registry today and therefore ALREADY self-lock on crash/timeout under ADR-039 §Decision 1's axes-independence, independent of any `failure_policy` flip; PC9 now asserts the ordering rule fires when EITHER `failure_policy="fail-closed"` OR `on_error="block"` is present (two independently-sufficient triggers). Extended the control set from four to six controls (added `on_error`-only-trigger POSITIVE-CONTROL matching the live registry's actual state, combined-both-triggers POSITIVE-CONTROL, corrected VACUITY-CONTROL requiring BOTH triggers absent, and a LIVE-TREE-CONTROL whose correct outcome today is that the ordering rule is ALREADY TRIGGERED). Invariant 6 rewritten to match. Six Canonical Test Vector rows added/rewritten. (Q-B, F-S2123-P2-006) New Postcondition PC12 — audit-loss observability, independent of suppression outcome: PC11's suppression-denial is correctly a no-op on a counterfactually-clean-pass matched invocation, which left a failed/skipped audit write on such an invocation SILENT; PC12 mandates an unconditional stderr diagnostic (gate name, trace UUID, failure variant) whenever `InternalLog::write_checked` returns other than `Written`, firing identically in PC11's suppression-denial case AND the counterfactual-clean-pass case; the diagnostic is itself best-effort. New Invariant 8 codifies this as PC11's observability complement. New Edge Case EC-008. One new Canonical Test Vector row (PC12 POSITIVE control). Traceability ADR row extended with ADR-039 §Decision 1 (Q-A) and F-S2123-P2-006 (Q-B); Security, Stories, and Story Anchor rows updated to attribute PC9's correction and PC12 to S-21.23 alongside PC11. VP-TBD (Verification Properties + VP Anchors) extended for PC9's six-control dual-trigger gate and PC12's observability property. H1 enriched per POLICY 7. No PC renumbering; PC12 is new (additive); PC1-PC8, PC10, PC11 UNCHANGED in substance. Does NOT touch BC-1.03.017 (separate product-owner burst, same task), the S-21.x story bodies/ACs/frontmatter (story-writer's domain, dispatched separately this burst per POLICY 8 propagation), or any INDEX/STATE.md (state-manager's domain). This content change means this BC's own `input-hash` will go stale as a result and is flagged for state-manager reconciliation same-burst. BC-1.03.018 v1.3. |
| v1.2 | 2026-08-22 | product-owner | Wave-7 adversary pass-1 remediation, cycle v1.0-brownfield-backfill (Q3, F-S2123-P1-001; ADR-039 §AMD-004, RATIFIED 2026-08-22, architect adjudication via orchestrator relay): break-glass suppression MUST be fail-closed-on-audit-failure. The mandated audit sink (`InternalLog::write`) is best-effort by contract — it swallows I/O errors and its mount-gate skip path also returns `Ok(())` — so a caller cannot distinguish "durably written" from "silently dropped"/"silently skipped"; left as specified, a break-glass activation whose audit write is dropped (precisely during the live self-lock scenario break-glass exists to recover from) still suppresses the underlying block, reproducing CWE-636's silent-approval state one layer down from PC5/PC6's audit-mandatory requirement. Added new Postcondition PC11: suppression (PC2/PC3) is granted ONLY IF the `break_glass.activated` audit write is CONFIRMED DURABLY WRITTEN via the new, additive `InternalLog::write_checked` tri-state method (`Written`/`SkippedMountGate`/`Failed(io::Error)`, added ALONGSIDE the existing best-effort `write()`, unchanged for its other callers); if the write FAILS or is SKIPPED, the override MUST NOT be applied — the gate's block decision stands exactly as if unset. New Invariant 7 codifies the audit-durability-gates-suppression principle, cross-referenced from Invariant 3 and PC5. New Edge Case EC-007 (matched override + unwritable/mount-gated audit sink → suppression denied, block stands). Three new Canonical Test Vector rows (PC11 POSITIVE controls for `Failed`/`SkippedMountGate`; NEGATIVE control for `Written`). Traceability ADR row extended with the ADR-039 §AMD-004 citation; Architecture Module and Stories rows updated to name `InternalLog::write_checked` and S-21.23 (Wave-7 split-topology sub-story of CONVERGED S-21.11, owning this leg's Phase-3 TDD implementation). Architecture Anchors extended with an `InternalLog::write_checked` bullet; Story Anchor extended with S-21.23. VP-TBD (Verification Properties + VP Anchors) extended with PC11's property and a Phase-6 formal-verifier VP-assignment anchor note. H1 enriched per POLICY 7 with the audit-durability clause. No PC renumbering; PC11 is new (additive); PC1-PC10 UNCHANGED. Does NOT touch BC-1.03.017 (Q1/Q2's amendment — separate product-owner burst, same task), the S-21.x story bodies/ACs/frontmatter (story-writer's domain, dispatched separately this burst per POLICY 8 propagation), or any INDEX/STATE.md (state-manager's domain). This content change means this BC's own `input-hash` will go stale as a result and is flagged for state-manager reconciliation same-burst. BC-1.03.018 v1.2. |
| v1.1 | 2026-08-19 | product-owner | S-21.11 v2.0 adversarial pass-1 remediation (F-S2111V2-P1-001-mechanism-adjudication memo): (1) F-004 — EC-005's Description column corrected from the false "both named gates dispatched in the SAME tier" premise (live `hooks-registry.toml` places `validate-wave-gate-prerequisite` at priority 130 and `validate-pr-merge-prerequisites` at priority 120 — distinct priorities place them in two DIFFERENT `routing.rs::group_by_priority` tiers) to the accurate mechanism: both gates dispatched in separate tiers, evaluated unconditionally within the same dispatch via `execute_tiers`'s no-early-return tier loop; EC-005's stated conclusion (two separate `break_glass.activated` events, per-gate audit granularity preserved) is unchanged — only the reason given was wrong; no PC/Precondition/Invariant content altered by this leg. (2) F-006 — new PC10 adds a testable assertion (`test_break_glass_env_not_settable_by_child_process`) plus an explicit threat-boundary note: an `Agent`-tool-invoked path (Bash-tool subprocess, MCP tool invocation, or hook-wrapper subprocess) CANNOT arm `VSDD_BREAK_GLASS_GATE_BYPASS` for the dispatcher's own process environment — only a human operator's own process environment can, because child-process environment mutations never propagate to a parent process (OS process-model property: `fork`/`exec` copies the environment at spawn time). Invariant 1 amended with a cross-reference to PC10. New Canonical Test Vector row (PC10). New Architecture Anchors bullets (OS process model basis; adjudication memo citation). H1 enriched with the PC10/F-006 clause per POLICY 7. This is the first Changelog entry for this BC — none existed at v1.0. PC count extended PC1-PC9 → PC1-PC10 (additive-only; no renumbering). BC-1.03.018 v1.1. |
| v1.0 | 2026-08-19 | product-owner | Initial creation (S-21.11 expanded-scope BC coverage burst, orchestrator-directed): authors the break-glass operator-override behavioral contract mandated by ADR-039 §Decision 3 v1.10 amendment's minimum-viable definition (RATIFIED, POLICY 22, 2026-08-19). Filed as a NEW sibling BC to BC-1.03.017. New capability CAP-039 authored in `domain-spec/capabilities.md` v1.11 to anchor this BC. BC-1.03.018 v1.0. |
