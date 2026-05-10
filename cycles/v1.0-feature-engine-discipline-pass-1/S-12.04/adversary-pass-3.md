# Adversarial Review — S-12.04 Pass 3

**Story:** S-12.04 — WASM Resolver Host ABI & Engine Dispatch  
**Pass:** 3  
**Date:** 2026-05-10  
**Branch SHA at review:** 5baec21c  
**Classification:** HIGH (BLOCKERS_PRESENT)  
**Findings:** 4 (1 HIGH, 2 MEDIUM, 1 LOW)

---

## Summary

Pass-3 adversarial review of S-12.04 after pass-2 remediation. The four HIGH blockers from
pass-2 (F-001 through F-004) show partial remediation: `set_epoch_deadline` is now called and
`context_key` filtering is wired. However, four new or residual issues remain. One HIGH blocker
is introduced by a `resolver_name` collision in the dispatch table that can silently route
invocations to the wrong resolver. Two MEDIUM findings cover epoch interruption classification
(Wasmtime raises `Trap` not a distinct timeout error, breaking the caller's error-handling
branch) and `fail_closed` eprintln drift (the eprintln error message differs from the
structured log field name, causing observability gaps). One LOW finding covers a minor doc
omission.

---

## Findings

### F-S12.04-P3-001 — HIGH: resolver_name collision in dispatch table

**Location:** Engine dispatch / resolver registry  
**Description:** The resolver dispatch table uses `resolver_name` as the map key without
normalizing case or stripping whitespace. A manifest that registers `"MyResolver"` and another
that registers `"myresolver"` (or `"MyResolver "` with a trailing space) are treated as
distinct entries at insert time but may collide under case-insensitive filesystem lookups on
macOS, causing the engine to silently dispatch to whichever entry was inserted last. More
critically, if two manifests legitimately share the same normalized name (e.g., two plugins
both declaring `resolver_name = "default"`), the second silently overwrites the first with no
warning logged.  
**Impact:** Silent mis-dispatch — invocations routed to the wrong resolver module. Security
boundary violation if a lower-privilege resolver overwrites a higher-privilege one in the
table.  
**Required fix:** Normalize `resolver_name` (lowercase, trim) before insertion and lookup.
Detect and error on duplicate normalized names at registry-build time. Add a test asserting
duplicate registration returns `Err`.

---

### F-S12.04-P3-002 — MEDIUM: epoch interruption surfaces as Trap, not timeout error

**Location:** Engine dispatch / WASM epoch interruption handling  
**Description:** `set_epoch_deadline` is now called (F-P2-001 remediated), but when the epoch
deadline fires, Wasmtime raises a `wasmtime::Trap` with kind `TrapCode::Interrupt`. The
dispatch layer's error-handling branch matches on a custom `ResolverError::Timeout` variant
that is never populated — the `Trap` propagates up as an opaque `anyhow::Error` instead.
Callers that check for timeout to apply `fail_closed` policy see `ResolverError::Other` and
may apply the wrong policy.  
**Impact:** Epoch-enforced timeouts are misclassified at the policy layer — `fail_closed`
timeout handling silently degrades to `fail_open` behavior for timed-out resolvers depending
on caller error-matching.  
**Required fix:** In the epoch-trap catch path, downcast the `anyhow::Error` to
`wasmtime::Trap`, inspect `trap.trap_code() == Some(TrapCode::Interrupt)`, and convert to
`ResolverError::Timeout`. Cover with a test that asserts a deliberately time-limited resolver
returns `ResolverError::Timeout` (not `Other`).

---

### F-S12.04-P3-003 — MEDIUM: fail_closed eprintln message drifts from structured log field

**Location:** Resolver error handling / `fail_closed` branch  
**Description:** The `fail_closed` error path now correctly propagates the error upward
(F-P2-003 remediated). However, the diagnostic `eprintln!` in that branch emits the string
`"resolver failed: fail_open policy rejected"` — incorrectly naming `fail_open` in a
`fail_closed` context. Additionally, the surrounding structured log event uses field name
`policy_applied` while the eprintln uses the raw string, so grep-based log triage will not
correlate the two. In production, operators searching for `fail_closed` in logs will miss these
events entirely.  
**Impact:** Operational observability gap — `fail_closed` rejections are invisible to
structured log queries and the eprintln message is actively misleading.  
**Required fix:** Correct the eprintln string to `"resolver failed: fail_closed policy
applied"` and align the field name with the structured log event. Add a test that captures
stderr and asserts the correct policy name appears in the output when `fail_closed` fires.

---

### F-S12.04-P3-004 — LOW: HOST_ABI "Resolver Memory Protocol" missing memory limit enforcement hook description

**Location:** HOST_ABI spec / Resolver Memory Protocol section  
**Description:** The "Resolver Memory Protocol" section was authored as required by the pass-2
process gap finding. It covers memory layout, `(offset, length)` packing as `u32`, and the
`(0,0)` no-result sentinel correctly. However, the "memory limit enforcement hook points" item
listed in the pass-2 required fix is absent: the section does not describe how the engine
enforces the configured `max_memory_bytes` limit before or after resolver execution, nor where
implementors should hook custom enforcement.  
**Impact:** Low — correctness of existing enforcement is not affected, but resolver authors
implementing custom enforcement will find no guidance in the spec.  
**Suggested fix:** Add a subsection describing the enforcement hook point (e.g., post-growth
check via the `ResourceLimiter` trait) and reference the relevant Wasmtime API. Can be
addressed in a doc-only commit before pass-4.

---

## Pass Verdict

**BLOCKERS_PRESENT — normalized to HIGH**

F-S12.04-P3-001 (HIGH) is a blocker: silent resolver mis-dispatch is a security boundary
violation. F-S12.04-P3-002 (MEDIUM) must also be fixed before pass-4 because timeout
misclassification directly affects `fail_closed` policy correctness. F-S12.04-P3-003 (MEDIUM)
should be fixed in the same remediation burst (low effort, high observability value).
F-S12.04-P3-004 (LOW) may be deferred to a doc-only commit.

Pass-4 may proceed only after remediation of F-S12.04-P3-001, F-S12.04-P3-002, and
F-S12.04-P3-003.
