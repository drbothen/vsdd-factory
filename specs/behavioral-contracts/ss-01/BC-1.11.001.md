---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: architect
timestamp: 2026-05-04T00:00:00Z
phase: 1.2-rev
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: spec-revision
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.11.001: factory-dispatcher::host::exec_subprocess::injects_vsdd_trace_id_and_parent_span_id — dispatcher-side mandatory injection of VSDD_TRACE_ID and VSDD_PARENT_SPAN_ID into every exec_subprocess invocation

## Description

The dispatcher MUST inject `VSDD_TRACE_ID` and `VSDD_PARENT_SPAN_ID` into
the environment of every subprocess spawned via `host::exec_subprocess` —
unconditionally, regardless of what the invoking plugin's `env_allowlist`
manifest entry declares. These two variables are dispatcher-side invariants,
not per-plugin opt-in variables. No plugin invocation of `exec_subprocess`
may omit them from the subprocess environment.

This BC codifies the implementation contract for ADR-015 D-15.4. The policy
decision (dispatcher-side mandatory injection) is in ADR-015; this BC holds
the implementation shape, edge cases, and verification anchors for SS-01.

The `hooks-registry.toml` `env_allowlist` field semantics are UNCHANGED.
It remains a per-plugin allowlist for OTHER environment variables that plugins
opt into. `VSDD_TRACE_ID` and `VSDD_PARENT_SPAN_ID` are injected before the
env_allowlist filter runs and are NOT subject to that filter.

## Preconditions

1. Plugin calls `vsdd::exec_subprocess` with a valid capability grant.
2. The dispatcher has an active `dispatcher_trace_id` (UUID, generated at
   process startup per ADR-015 D-15.2).
3. The invoking plugin has an active `plugin.invocation_id` (the `span_id`
   for this plugin invocation).

## Postconditions

1. The subprocess environment contains `VSDD_TRACE_ID` set to the current
   invocation's `trace_id` (inherited from `VSDD_TRACE_ID` env at dispatcher
   start, or the per-invocation UUID if no parent trace exists).
2. The subprocess environment contains `VSDD_PARENT_SPAN_ID` set to the
   invoking plugin's `span_id` (i.e., its `plugin.invocation_id`).
3. These two variables are present even when the plugin's `env_allowlist`
   does not list them.
4. A subprocess that itself calls `exec_subprocess` inherits and forwards
   both variables; the `trace_id` is unchanged across the hop; the
   `parent_span_id` at each subsequent hop is the previous level's `span_id`.
5. No other env_allowlist semantics are altered. Other variables are still
   subject to the plugin's declared `env_allowlist`.

## Invariants

1. `VSDD_TRACE_ID` is ALWAYS present in the subprocess environment for any
   `exec_subprocess` call. There is no opt-out mechanism.
2. `VSDD_PARENT_SPAN_ID` is ALWAYS present, set to the invoking plugin's
   `span_id` at the time of the `exec_subprocess` call.
3. The `env_allowlist` gate in `host/env.rs` is NOT applied to
   `VSDD_TRACE_ID` or `VSDD_PARENT_SPAN_ID`. These bypass the allowlist.

## Related BCs

- BC-1.05.001 — exec_subprocess capability check (prerequisite: capability
  must be granted before trace injection occurs)
- BC-1.05.007 — env host fn denies env var not on allow-list (sibling: this
  BC defines the EXCEPTION to that deny-by-default policy for trace vars)
- BC-1.08.006 — dispatcher projects whole process env (context: the ambient
  env from which VSDD_TRACE_ID inherits at startup)

## Architecture Anchors

- `crates/factory-dispatcher/src/host/exec_subprocess.rs` — inject site;
  VSDD_TRACE_ID and VSDD_PARENT_SPAN_ID added to the command env builder
  unconditionally before the allowlist filter
- ADR-015 D-15.4 — policy decision; this BC is the implementation contract

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin's `env_allowlist` is empty (no entries) | VSDD_TRACE_ID and VSDD_PARENT_SPAN_ID still injected; other env vars blocked by allowlist |
| EC-002 | Plugin's `env_allowlist` explicitly lists VSDD_TRACE_ID | No double-set; dispatcher-injected value takes precedence; no error |
| EC-003 | VSDD_TRACE_ID is not set in the dispatcher's own process environment (no parent trace) | Dispatcher generates a per-invocation UUID at startup; that UUID is used |
| EC-004 | Plugin spawns a subprocess that itself calls exec_subprocess | Inner subprocess receives same VSDD_TRACE_ID; VSDD_PARENT_SPAN_ID is updated to the intermediate plugin's span_id at each hop |
| EC-005 | exec_subprocess call is denied by capability check (no capability block) | Denial returns before env injection; trace variables are not injected into a process that is never spawned |

## Canonical Test Vectors

| Scenario | Expected subprocess env |
|----------|------------------------|
| Plugin with empty env_allowlist calls exec_subprocess | subprocess env contains VSDD_TRACE_ID and VSDD_PARENT_SPAN_ID; no other env vars from plugin process |
| Plugin with `env_allowlist = ["HOME"]` calls exec_subprocess | subprocess env contains VSDD_TRACE_ID, VSDD_PARENT_SPAN_ID, HOME; nothing else |
| No VSDD_TRACE_ID in dispatcher process env | subprocess env contains VSDD_TRACE_ID set to dispatcher's per-invocation UUID |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | `VSDD_TRACE_ID` present in subprocess env on all allowed exec_subprocess calls | property-based test (proptest) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | S-10.03 (Wave 1 enrichment implementation) |
| ADR | ADR-015 D-15.4 (dispatcher-side mandatory injection) |
| OQ Resolved | OQ-3 (resolved via D-15.4; this BC formalizes the implementation contract) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — subprocess spawn; env injection is pure in-memory prior to spawn |
| Global state access | YES — reads dispatcher-level trace_id from HostContext |
| Deterministic | YES given fixed HostContext state |
| Thread safety | YES — HostContext is immutable after startup; env builder is per-call |
| Overall classification | Effectful shell (subprocess spawn) with pure env-construction step |
