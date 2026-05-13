# Review Findings: fix/crashed-gate-hook-fail-closed

## Convergence Table

| Cycle | Findings | Blocking | Fixed | Remaining |
|-------|----------|----------|-------|-----------|
| 1     | 2        | 1        | 0     | 2         |

---

## Cycle 1 Findings

### Finding 1 — IMPORTANT (coverage gap)

**ID:** F-001  
**Severity:** important (should-fix)  
**Category:** coverage  
**Location:** `crates/factory-dispatcher/tests/full_stack_plugin_invocation.rs`

**Finding:**  
TC-8 tests Crashed + on_error=Block at the integration level. The corresponding Timeout + on_error=Block case is covered by unit tests only (`fail_closed_timeout_with_on_error_block` in `executor.rs`). For a CRITICAL security fix where timeout is explicitly listed as a fail-closed trigger in the doc-comment and `plugin_fail_closed()` implementation, an integration-level TC exercising Timeout + on_error=Block (using a sync hang WAT module with a short timeout_ms) would give stronger end-to-end confidence.

**Rationale for "important" not "blocking":**  
The unit test is direct and unambiguous — `plugin_fail_closed()` is a pure function, and the unit test verifies the exact invariant. The integration test gap does not represent a correctness defect. However, given this is a security/gate fix, an e2e timeout TC strengthens the invariant chain.

**Route:** test-writer (add sync Timeout + on_error=Block integration TC)

---

### Finding 2 — SUGGESTION (description completeness)

**ID:** F-002  
**Severity:** suggestion  
**Category:** description  
**Location:** PR body

**Finding:**  
The PR body's "Root cause" section accurately describes the Crashed path but does not explicitly mention Timeout as an equally affected variant. A reader skimming "root cause" might think Timeout was an afterthought. One sentence calling out that Timeout has the same stdout-absent property (timer expiry aborts before plugin emits) would make the description self-contained.

**Route:** pr-manager (update PR body)

---

## Triage Routing

| ID   | Severity  | Category  | Route To    | Action                                              |
|------|-----------|-----------|-------------|-----------------------------------------------------|
| F-001 | important | coverage  | test-writer | Add sync Timeout + on_error=Block integration TC    |
| F-002 | suggestion | description | pr-manager | Clarify Timeout root cause in PR body              |

---

## Verdict

**REQUEST_CHANGES** — 1 important (coverage) finding should be addressed before merge for a CRITICAL security fix. 1 suggestion (description).

No blocking findings. No coherence, size, or dependency issues. Async/sync boundary is structurally sound. `plugin_fail_closed()` implementation is correct and complete. TC-8 assertion is correct.
