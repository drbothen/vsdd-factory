---
document_type: security-review
wave: 15
gate_run: rerun-1
producer: security-reviewer
date: 2026-05-02
branch_under_review: 1ab1d6f
verdict: CONVERGED
input_hash: rerun-1-1ab1d6f
---

# W-15 Wave Gate Re-run #1 — Security Review

**Branch:** develop @ 1ab1d6f (post PR #59 fix-burst)

## Verdict: CONVERGED

No new security findings introduced by PR #59. All pre-existing security items retain their prior dispositions.

### SEC-003 (VSDD_SINK_FILE path injection): CLOSED

`#[cfg(debug_assertions)]` gate applied — VSDD_SINK_FILE env var only active in debug builds. Path-traversal rejection added: paths must be absolute and must not contain `..` components. Production builds do not expose this surface.

### SEC-001 (WASI preopened_dir surface): DEFERRED-DOCUMENTED

HOST_ABI.md Filesystem Access Model section added explaining the WASI preopened_dir threat model: plugins can access any file under the preopened directories. The disposition is documentation-only for W-15; a capability-tightening story is tracked as a v1.1 enhancement candidate.

### SEC-002 (split-brain in write_file resolution): NOT-CLOSED / pre-existing

Production invocation path uses invoke.rs; standalone test path uses write_file.rs directly. The two paths have slightly different resolution semantics. Pre-existing disposition maintained: tracked as TD item, not blocking for rc.3.

### SEC-004 (unbounded HookPayload deserialization): NOT-CLOSED / pre-existing

No 1 MiB cap on incoming hook payload deserialization. Pre-existing disposition maintained: tracked as TD item with recommendation to add a serde size limit. Not blocking for rc.3.

### SEC-005 (binary_allow bare names): NOT-CLOSED / pre-existing

`binary_allow` in hooks-registry.toml uses bare names without path constraints. Pre-existing disposition maintained. Tracked as TD item.

### SEC-006 / TD-014 (Tier 2/3 adapter retirement): NOT-CLOSED / pre-existing

Legacy bash adapter still active for Tier 2/3 hooks. Pre-existing disposition: calendar-gated to v1.0 GA close via W-16. Not in scope for rc.3.

### Summary

Security reviewer is satisfied that PR #59 introduces no new attack surface. CONVERGED on security dimension. CRIT-PR59-001 identified by adversary (dispatcher AND-gate logic) is a functional correctness defect, not a security issue — deferred to adversary-owned finding.
