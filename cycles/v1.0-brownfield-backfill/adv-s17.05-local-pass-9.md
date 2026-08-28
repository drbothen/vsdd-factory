# S-17.05 LOCAL Adversarial Review Pass 9

**Reviewed artifact:** S-17.05 implementation at `fcc0fb7f` (stamp-state-timestamp PostToolUse hook + factory-lock crates; 26 commits ahead develop)
**Review date:** 2026-08-28
**Verdict:** FINDINGS (1 MEDIUM + 2 LOW)
**LOCAL BC-5.39.001 streak:** 0/3 (RESET — was 1/3 from pass 8 CLEAN)
**D-chain:** none (per per-story local-cascade convention — no D-NNN allocated)

## Part A — Finding Set

**MEDIUM (1):**

- **F-P9-001 (MEDIUM, POLICY 4 mis-anchor):** BC-4.17.001 named `crates/factory-lock` as the canonical home of `TTL_SECONDS` in four live-body loci: Precondition 3 (sourcing sentence), Invariant 3 (parenthetical), VP-TBD-4 (verification claim), and Architecture Anchors (constant home attribution). Ground-truth grep confirms `pub const TTL_SECONDS: u32 = 2700` is declared ONLY in `crates/factory-lock-parse/src/lib.rs`; there is zero declaration of `TTL_SECONDS` in `crates/factory-lock/src/lib.rs`. This is a direct POLICY 4 mis-anchor — the authoritative constant home is `crates/factory-lock-parse`, not `crates/factory-lock`. Propagated from D-1124/D-1126 boundary-correction sweep gap.

**LOW (2):**

- **F-P9-002 (LOW, POLICY 5 SDK-grounding):** BC-4.17.001 Precondition 4 and VP-TBD-7 stated that the deregistered/retired `verify-state-timestamp-refresh` crate "no longer declares `STATE_MD_MAX_BYTES`." Ground-truth grep of `crates/verify-state-timestamp-refresh/src/lib.rs` confirms `pub const STATE_MD_MAX_BYTES: u32 = 262144` is still present in the crate's source (dormant per ADR-046 Decision 2 deferral; crate retained on disk per D-1124). The claim that it "no longer declares" the constant is factually incorrect; the constant is present but dormant. POLICY 5 SDK-grounding requires the spec to accurately reflect actual source.

- **F-P9-003 (LOW, test-fidelity / TD-VSDD-059):** `crates/hook-plugins/stamp-state-timestamp/src/tests.rs` test `test_expired_self_held_lock_never_renewed` had a Red Gate row (in story S-17.05 AC-table) promising the test asserts "no `exec_subprocess` call is made" — i.e., that the identity-check does not fire when the lock is already expired. The test body omitted the load-bearing `exec_called` flag and assertion, relying only on the absence of a panic. This is a TD-VSDD-059 paper-fix: the test passed vacuously rather than asserting the behavioral guarantee. The Red Gate promise was unfulfilled.

## Part B — Disposition

All three findings fixed in-scope this same burst:

- **F-P9-001:** FIXED by product-owner. Four live-body loci in BC-4.17.001 corrected: Precondition 3, Invariant 3, VP-TBD-4, and Architecture Anchors now cite `crates/factory-lock-parse/src/lib.rs` as the canonical TTL_SECONDS home. BC-4.17.001 bumped v1.27→v1.28. Input-hash recomputed by state-manager (ee0c840→8706b2f).

- **F-P9-002:** FIXED by product-owner. BC-4.17.001 Precondition 4 and VP-TBD-7 reworded to acknowledge dormant copy: the retired crate still holds a dormant `STATE_MD_MAX_BYTES` declaration; `crates/factory-lock-parse` is the canonical active source. BC-4.17.001 v1.28 (same bump as F-P9-001).

- **F-P9-003:** FIXED by test-writer. Added `exec_called: bool` flag + assertion to `test_expired_self_held_lock_never_renewed`. `cargo test -p stamp-state-timestamp` passed 32/32 tests; `cargo fmt --check` and `cargo clippy` clean. feature/S-17.05 HEAD advanced fcc0fb7f→a8d85160247d6cbb8f1c91c3202963195ed68581 (pushed to origin).

BC-5.39.001 LOCAL streak: **RESET 1/3→0/3**. Next: re-run pass 10 fresh against `feature/S-17.05 @ a8d85160`.

**[process-gap observation]:** The adversary dispatch for this pass lacked the formal identity tuple `(worktree-abs-path, feature-HEAD-SHA, story-id, canonical-repo-root)` in the dispatch package. Orchestrator is self-correcting going forward; no follow-up story required at this pre-convergence stage.
