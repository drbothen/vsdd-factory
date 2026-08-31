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

### F-P14-001 — `guard_logic` write-back fail-open arm has no `log_warn` (optional hardening)

| Field | Value |
|-------|-------|
| **Finding ID** | F-P14-001 |
| **Severity** | ADVISORY / OPTIONAL-HARDENING |
| **Source pass** | Pass 14 (adv-s17.05-local-pass-14.md) |
| **File** | `crates/verify-state-timestamp-refresh/src/guard_logic.rs` |
| **Observation** | The Step-6 write-back fail-open arm (`let _ = write_file(...)`) swallows write errors without emitting any observability event (`log_warn` or equivalent). The read-side fail-open arms (GAP-2 / GAP-3) do emit observability annotations. This creates an asymmetry on the write path. |
| **Spec status** | SPEC-PERMITTED — BC-4.17.001 PC3/Invariant 4 mandates swallow-on-write-error; no AC, PC, EC, or VP requires write-failure observability. The implementation exactly matches its specification. |
| **Default disposition** | ACCEPT — "spec mandates swallow-on-write-error; no observability obligation in current BC/AC/VP; write-side fail-open intentional per PC3/Invariant 4." |
| **Hardening option** | Add `log_warn!("STATE.md write failed: {err}")` at the write-back fail-open locus. NOTE: hardening re-opens the frozen code perimeter and requires a new 3-CLEAN cascade — cost is high. Default is ACCEPT unless human/architect directs otherwise. |
| **Routing** | Decide at finalization: accept (story-writer: add rationale note) OR harden (human/architect direction required given perimeter re-open cost). |
| **Blocking?** | No — ADVISORY only; does NOT affect convergence (3-CLEAN already ACHIEVED). |

---

## Status

| Item | Status | Resolved by |
|------|--------|-------------|
| F-P12-001 | **RESOLVED** 2026-08-28 — story v1.8 doc-sweep complete (story-writer); all stale Red Gate counts corrected: tally-sentence 28→32/31→35/35→39; T-1 18→32 + 22→39; T-7 27→39; T-3 18→32 (x2); Purity table 18→32; File Structure table 18→32; Out-of-Scope + T-7 `~28` verify-state-timestamp-refresh counts de-numbered (drift-resistant). input-hash `6067e5f` UNCHANGED. | story-writer finalization doc-sweep (v1.8) |
| O-P13-1 | **ACCEPTED (won't-fix)** 2026-08-28 — spec-conformant: AC-018 / BC-4.17.001 Invariant 8 mandate the verbatim boundary `(200000, 262144]` and the verbatim `("cap_bytes","262144")` event; the literal `262144` IS the normative value; no silent drift path exists (a constant-change would require simultaneous spec amendment). Hardening would needlessly re-open the frozen 3-CLEAN-certified code perimeter. Recorded as optional future-hardening suggestion, not a defect. | story-writer 2026-08-28 |
| F-P14-001 | **ACCEPTED (won't-fix)** 2026-08-28 — spec-permitted: BC-4.17.001 PC3/Invariant 4 mandates swallow-on-write-error; no AC, PC, EC, or VP requires write-failure observability; the write-side fail-open is intentional per PC3/Invariant 4. Hardening would needlessly re-open the frozen 3-CLEAN-certified code perimeter. Recorded as optional future-hardening suggestion, not a defect. | story-writer 2026-08-28 |

*Last updated: 2026-08-28 (S1705-D1127-FINALIZATION-DOC-SWEEP-COMPLETE — story v1.8; F-P12-001 RESOLVED; O-P13-1 + F-P14-001 ACCEPTED won't-fix)*

---

# S-25.01 Finalization Doc-Sweep Backlog

**Anchor:** This section is the "concrete future step" anchor required by VSDD Canonical Principle Rule 3 for all
batched non-blocking items deferred under D-1127 governance ruling (applied to S-25.01 BC-5.39.001 3-CLEAN run).
Items here MUST be swept AFTER 3-CLEAN is achieved and BEFORE the S-25.01 PR is created.

**Governance basis:** D-1127 (2026-08-28) extended by D-1136 context — LOW/OBS/process-gap items during the
S-25.01 local BC-5.39.001 3-CLEAN run are BATCHED and swept in a single finalization doc-sweep after 3-CLEAN is
reached, NOT fixed mid-streak. Fixing mid-streak would bump story version/input-hash and trigger the
frozen-artifact-reset trap (L-EDP1-007/051/061).

**When to execute:** After passes 2 and 3 both return CLEAN (local BC-5.39.001 3-CLEAN achieved), BEFORE
submitting the S-25.01 PR. Owner: implementer (LOW-1/OBS-3), story-writer/orchestrator ([process-gap]).

**Frozen artifact:** feature/S-25.01 @ `92990371` — NO code/spec changes until 3-CLEAN is reached.

---

## Batched Items (S-25.01 passes 1-3 window)

### LOW-1 — `RegistryError::AsyncBlockConflict` hardcodes `on_error="block"` in error message

| Field | Value |
|-------|-------|
| **Finding ID** | LOW-1 |
| **Severity** | LOW / documentary/UX |
| **Source pass** | Pass 1 (LOCAL adversary pass 1 CLEAN 2026-08-31) |
| **File** | `crates/factory-dispatcher/src/registry.rs` ~lines 57-62 |
| **Observation** | `RegistryError::AsyncBlockConflict` message hardcodes `on_error="block"` in its guidance text, but the rejection also fires when `on_error="block_if_marker"` is set and the async constraint is violated. The remediation text says "set `on_error=block`" but that is already one of the REJECTED configurations. |
| **Correct behavior** | Reword the error message to cover BOTH blocking policies: "set `on_error=continue` or `on_error=advisory`" (i.e., non-blocking policies), rather than naming a specific blocking policy as the remedy. |
| **Routing** | implementer |
| **Blocking?** | No — does not affect behavior, only the diagnostic text |

---

### OBS-3 — `write_indeterminate_marker` may leave orphaned `.tmp` if `fs::rename` fails

| Field | Value |
|-------|-------|
| **Finding ID** | OBS-3 |
| **Severity** | OBSERVATION / low-risk resource hygiene |
| **Source pass** | Pass 1 (LOCAL adversary pass 1 CLEAN 2026-08-31) |
| **File** | `crates/factory-dispatcher/src/indeterminate_marker.rs` ~lines 157-160 |
| **Observation** | `write_indeterminate_marker` writes to a `.tmp` file then renames it atomically. If `fs::rename` fails, the `.tmp` file is left on disk (orphaned). |
| **Fix** | Add `let _ = fs::remove_file(&tmp_path);` on the rename-error branch to clean up the orphan before returning the error. |
| **Routing** | implementer |
| **Blocking?** | No — `.tmp` files are inert; worst case a stale file persists until next write or OS temp-cleanup |

---

### [process-gap] — No CI/lint validates hooks-registry.toml crash-policy comments vs `on_error` value

| Field | Value |
|-------|-------|
| **Finding ID** | [process-gap] registry-comment-lint |
| **Severity** | PROCESS-GAP / drift-risk |
| **Source pass** | Pass 1 (LOCAL adversary pass 1 CLEAN 2026-08-31) |
| **Observation** | The M-1 finding (prior fix burst) was a stale `on_error="continue"`-era phrasing comment ("dispatch proceeds on crash"/"crash→allow") adjacent to a `block_if_marker` entry in `hooks-registry.toml`. No automated lint detects this class of comment-vs-value drift. |
| **Candidate follow-up** | A lint or CI check that flags `on_error="continue"`-era phrasing ("dispatch proceeds"/"crash→allow") adjacent to entries whose `on_error` value is `block` or `block_if_marker`. This is a self-improvement candidate. |
| **Disposition per Cycle-Closing Checklist** | Per the Cycle-Closing Checklist, this process-gap must be TRACKED with a follow-up story OR a justified deferral before the cycle CLOSES. Recorded here as a tracked drift item so it is not lost. Do NOT open the story now — just track. |
| **Routing** | orchestrator at cycle-close: create follow-up story OR record justified deferral |
| **Blocking?** | No — does not block 3-CLEAN or PR |

---

### OBS-1 — Crash posture for block_if_marker (verified conformant, no action)

| Field | Value |
|-------|-------|
| **Finding ID** | OBS-1 |
| **Severity** | OBSERVATION — VERIFIED CONFORMANT |
| **Source pass** | Pass 1 (LOCAL adversary pass 1 CLEAN 2026-08-31) |
| **Observation** | Adversary noted the crash→BLOCK posture for non-expired markers under `on_error=block_if_marker`. |
| **Disposition** | VERIFIED CONFORMANT per ADR-048 + BC-1.18.002 v1.5. This is the specified behavior: crash+non-expired→BLOCK is the intentional fail-closed design. D-1135 fail-open SUPERSEDED by D-1136. No action required. |

---

### OBS-2 — Spec-ordered quoting in shell-words tokenizer (verified conformant, no action)

| Field | Value |
|-------|-------|
| **Finding ID** | OBS-2 |
| **Severity** | OBSERVATION — VERIFIED CONFORMANT |
| **Source pass** | Pass 1 (LOCAL adversary pass 1 CLEAN 2026-08-31) |
| **Observation** | Adversary noted quoting behavior in shell-words argument parsing. |
| **Disposition** | VERIFIED CONFORMANT per BC-1.18.002 v1.5 EC-024..EC-026 in-scope quoting vectors and EC-027..EC-029 out-of-scope vectors. Shell-words POSIX tokenizer behavior is spec-defined. No action required. |

---

## Status

| Item | Status | Resolved by |
|------|--------|-------------|
| LOW-1 (RegistryError::AsyncBlockConflict msg) | **OPEN — sweep after 3-CLEAN** | implementer (post-3-CLEAN doc-sweep) |
| OBS-3 (write_indeterminate_marker .tmp orphan) | **OPEN — sweep after 3-CLEAN** | implementer (post-3-CLEAN doc-sweep) |
| [process-gap] registry-comment-lint | **TRACKED — follow-up story or justified deferral at cycle-close** | orchestrator at cycle-close |
| OBS-1 (crash posture) | **VERIFIED CONFORMANT — no action** | adversary pass 1 |
| OBS-2 (quoting) | **VERIFIED CONFORMANT — no action** | adversary pass 1 |

*S-25.01 section added: 2026-08-31 (S2501-LOCAL-ADV-PASS1-CLEAN-STREAK-1of3-2026-08-31 — state-manager; BC-5.39.001 streak 1/3; artifact FROZEN @ 92990371)*
