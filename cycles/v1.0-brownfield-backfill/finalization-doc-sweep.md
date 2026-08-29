# S-17.05 Finalization Doc-Sweep Backlog

**Anchor:** This file is the "concrete future step" anchor required by VSDD Canonical Principle Rule 3 for all
batched LOW-documentary findings deferred under D-1127. Items here MUST be swept before the S-17.05 PR is
created, by story-writer, in a single doc-sweep commit on `feature/S-17.05`.

**Governance basis:** D-1127 (2026-08-28) — Human-ratified policy: LOW-only documentary findings during the
S-17.05 local BC-5.39.001 3-CLEAN run are BATCHED and swept in a single finalization doc-sweep after local
3-CLEAN is reached, NOT fixed mid-run. This prevents the frozen-artifact-reset trap (L-EDP1-007/051/061):
fixing LOW doc items mid-run would bump story version and input-hash, requiring fresh-context re-validation
of a fully-converged implementation.

**When to execute:** After passes 13 + 14 both return CLEAN (local BC-5.39.001 3-CLEAN achieved), BEFORE
submitting the S-17.05 PR. Routing: story-writer dispatched by orchestrator.

---

## Batched Items

### F-P12-001 — Red Gate prose tally sentence (stale counts)

| Field | Value |
|-------|-------|
| **Finding ID** | F-P12-001 |
| **Severity** | LOW / documentary |
| **Source pass** | Pass 12 (adv-s17.05-local-pass-12.md) |
| **File** | `stories/S-17.05-stamp-state-timestamp.md` |
| **Section** | `## Red Gate / Test Suite Minimum` prose summary sentence |
| **Current text** | "The test suite implements at least 28 Rust unit tests in the `guard_logic` module... ensuring at least 31 Rust unit tests are present in total." |
| **Correct text** | "The test suite implements at least 30 Rust unit tests in the `guard_logic` module... ensuring at least 32 Rust unit tests are present in total." |
| **Root cause** | Summary counts 28/31 drafted when story had fewer tests; 4 regression tests were added during the local cascade (O-P11-2/O-P11-3 and prior). The normative Red Gate TABLE (AC-032 floor) is correct — only the prose summary is stale. |
| **Routing** | story-writer |
| **Blocking?** | No — normative Red Gate TABLE is met in full; this is a cosmetic documentary correction |

---

### O-P13-1 — `guard_logic` GAP-4 literal vs. `STATE_MD_MAX_BYTES` (optional hardening)

| Field | Value |
|-------|-------|
| **Finding ID** | O-P13-1 |
| **Severity** | ADVISORY / OPTIONAL-HARDENING |
| **Source pass** | Pass 13 (adv-s17.05-local-pass-13.md) |
| **File** | `crates/verify-state-timestamp-refresh/src/guard_logic.rs` (or equivalent guard_logic module) |
| **Observation** | The GAP-4 soft-warn upper-bound check uses the hardcoded literal `262_144` rather than `flp::STATE_MD_MAX_BYTES`. |
| **Spec status** | SPEC-CONFORMANT — AC-018 / BC-4.17.001 Invariant 8 explicitly mandate the verbatim boundary `(200000, 262144]` and the verbatim `("cap_bytes","262144")` event. The literal IS the normative value; no behavioral discrepancy exists. |
| **Hardening option** | Replace `262_144` literal in the GAP-4 comparison with `flp::STATE_MD_MAX_BYTES` to eliminate latent-drift risk if the constant is ever changed. |
| **Won't-fix basis** | The spec mandates the verbatim boundary value, so a BC amendment to the constant would require updating both the constant AND the spec simultaneously — no silent drift path exists in practice. May be accepted at finalization without code change. |
| **Routing** | Decide at finalization review: harden (implementer, ~5 min) OR mark accepted (story-writer: add "accepted — spec mandates verbatim literal" note). |
| **Blocking?** | No — ADVISORY only; does NOT reset streak; does NOT affect convergence. |

---

## Status

| Item | Status | Resolved by |
|------|--------|-------------|
| F-P12-001 | OPEN — awaiting pass 14 CLEAN (1 of 2 remaining) | story-writer finalization doc-sweep |
| O-P13-1 | OPEN (OPTIONAL) — decide at finalization: harden or accept | implementer (harden) or story-writer (accept) |

*Last updated: 2026-08-28 (S1705-P13-CLEAN-BURST)*
