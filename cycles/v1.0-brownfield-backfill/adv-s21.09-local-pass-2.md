# S-21.09 LOCAL Adversarial Review Pass 2 — DO-NOT-RATIFY

**Reviewed artifact:** S-21.09 implementation at `12f280d1` (post pass-1 fix attempt; T-012 negative control added; comment corrected; T-028 assertion tightened)
**Review date:** 2026-08-11
**Verdict:** DO-NOT-RATIFY
**LOCAL streak:** 0/3 (reset by this pass)
**D-chain:** D-972

## Part A — Finding Set

**HIGH (1):**

- **H-1**: `extract_hook_plugin_name` test helper in `bundle_orphan_check.rs` reimplements the same bare-name extraction logic it uses to verify the registry parser. If the registry parser has a defect in bare-name extraction, `extract_hook_plugin_name` carries the same defect and the tests would pass despite the underlying bug. This is a structural test-independence violation: the control reimplements the gate.

**MEDIUM (1):**

- **M-1**: T-014 tests the `with_path: true` + declared + tracked path (PASS case) but does not test `with_path: true` + declared + NOT tracked (FAIL case). The `with_path` FAIL arm exists in the gate logic but has no test coverage.

**LOW (1):**

- **L-1**: `run_t012_gate` helper name conflicts with T-012 test ID — a reader cannot distinguish whether `run_t012_gate()` is a helper for T-012 specifically or a general gate runner callable from any T-NNN test.

## Part B — Disposition

H-1 is a structural issue requiring helper redesign (golden-value or external-oracle approach). Findings routed to implementer. S-21.09 streak reset to 0/3. Pass 3 required.
