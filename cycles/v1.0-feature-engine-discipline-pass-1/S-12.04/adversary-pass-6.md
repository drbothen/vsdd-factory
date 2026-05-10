# Adversarial Review — S-12.04 Pass 6

## Metadata
- Story: S-12.04 WASM Resolver Loading, Lifecycle, and Error Isolation
- Branch SHA reviewed: d9ca5e70
- Pass: 6
- Reviewer: adversary (fresh context)
- Classification: HIGH
- Within-story finding count: 3 (1 HIGH, 1 MEDIUM, 1 LOW observation)
- Recommendation: PROCEED_TO_FIX

## Findings

### F-S12.04-P6-001 — HIGH — HOST_ABI.md resolver.error field table is incomplete vs BC-4.12.004 v1.2 PC2 + impl [S-7.01 partial-fix regression]

**File(s):** `crates/hook-sdk/HOST_ABI.md` lines 1107-1112
**Anchor:** BC-4.12.004 v1.2 PC2 (lines 65-75); executor.rs:530-543
**Description:** BC-4.12.004 v1.2 PC2 enumerates 8 fields for resolver.error: type, resolver_name, error_kind, error_detail, event_type, trace_id, plugin_name, session_id. Executor emits all 7 wire fields (envelope type + 6 payload fields). HOST_ABI.md table at 1107-1112 lists ONLY 4 fields (resolver_name, error_kind, error_detail, event_type). Missing: trace_id, session_id, plugin_name. Pass-5 amended BC + tests + executor but missed HOST_ABI propagation.
**Why it matters:** HOST_ABI is canonical for alternate-language SDK authors. Cross-language tooling consuming the wire format will under-declare. Same edit window (pass-5) updated HOST_ABI line 1112 (event_type rename) but missed the 3 sibling rows.
**Suggested resolution:** Add 3 rows to the table:
- trace_id | string | Dispatcher trace ID for the dispatch event.
- session_id | string | Claude Code session identifier.
- plugin_name | string | The hook plugin name that declared this resolver in needs_context.

### F-S12.04-P6-002 — MEDIUM — plugin_name has no positive-coverage assertion in resolver.error integration test [POL-11 tautology gap]

**File(s):** `crates/factory-dispatcher/tests/executor_resolver_integration.rs:475-531`
**Anchor:** BC-4.12.004 v1.2 PC2; executor.rs:533 (with_plugin_name(&hook_name_err))
**Description:** Pass-4 added trace_id positive-coverage assertion (F-P4-005), pass-5 added session_id (F-P5-002). The provenance triplet trace_id/session_id/plugin_name should be symmetric. plugin_name has NO assertion. If `.with_plugin_name(...)` is silently dropped, no test fails.
**Why it matters:** Asymmetric POL-11 coverage — two of three trace fields guarded, third not. Same regression class as the trace_id and session_id assertions added in pass-4/5 to close.
**Suggested resolution:** Add `assert!(all_log_content.contains(<entry.name>), "F-S12.04-P6-002: ...")` parallel to the trace_id and session_id assertions.

### O-S12.04-P6-001 — LOW (observation) — F-P5-003 deferred-integration tag self-references its own finding ID

**File(s):** `crates/factory-dispatcher/tests/resolver_load_test.rs:1061`
**Description:** Pass-5 burst added a TODO comment with "Tracking: F-P5-003" — the same finding ID as the deferral itself. Adversary cannot adjudicate whether the self-reference is intentional or clerical drift. Test function name preserves F-P4-004 token (presumably for historical traceability). Pending intent verification.
**Suggested resolution:** Confirm self-reference is intentional. If not, replace with a permanent TD/anchor reference.

## Cross-cutting observations

- F-P5-001 (BC-4.12.004 v1.2 PC2) propagated to BC body and tests but missed HOST_ABI sibling — captured as F-P6-001.
- F-P5-002 session_id positive-coverage solid; trace_id (F-P4-005) solid. plugin_name parallel gap captured as F-P6-002.
- F-P5-003 test rename + tag clear modulo O-P6-001 self-reference question.
- VP-074 spec-impl signature drift `classify_resolver_trap(trap_code: TrapCode)` vs `(resolver_name: &str, trap: wasmtime::Trap)` is pre-existing and out-of-scope for per-story perimeter (architect/wave-gate scope).

## Convergence assessment

- Within-story findings: 3 (1 HIGH, 1 MEDIUM, 1 LOW observation)
- Severity floor: HIGH
- Classification: HIGH
- Reasoning: F-P6-001 is sibling-blast-radius drift on canonical ABI doc; per "Mis-anchoring is NEVER an Observation" axis. F-P6-002 is parallel POL-11 tautology gap. Pass-6 is BLOCKING. passes_clean: 0 (regressed at pass-5; remains at 0 because pass-6 is BLOCKING).
- Recommendation: PROCEED_TO_FIX (HOST_ABI 3 rows + plugin_name assertion). Re-dispatch pass-7 after fixes.
