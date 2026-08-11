# S-21.09 LOCAL Adversarial Review Pass 1 — DO-NOT-RATIFY

**Reviewed artifact:** S-21.09 implementation at `12f280d1` (validate-factory-path-staging WASM artifact restore + per-name registry parity CI check)
**Review date:** 2026-08-11
**Verdict:** DO-NOT-RATIFY
**LOCAL streak:** 0/3 (reset by this pass)
**D-chain:** D-972

## Part A — Finding Set

**HIGH (1):**

- **H-1**: T-012 gate arm for `without_path` plugins has no negative control — there is no test that verifies a `without_path: true` plugin failing the orphan check produces a FAIL result (as opposed to a PASS or error). T-012 only exercises the positive path (declared plugin IS tracked). The without_path negative arm is structurally reachable but untested.

**MEDIUM (2):**

- **M-1**: `bundle_orphan_check.rs` line 147 comment reads "extract bare plugin name from .wasm path" but the extraction removes the `.wasm` extension AND any directory prefix — the comment only describes half the transformation, misleading future readers about the matching semantics.
- **M-2**: Test T-028 uses `assert!(output.contains("PASS"))` but `PASS` appears in non-gate output (test setup messages); test could pass vacuously if the gate line is absent from output.

## Part B — Disposition

Findings routed to implementer. S-21.09 streak reset to 0/3. Pass 2 required.
