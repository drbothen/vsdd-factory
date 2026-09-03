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

> **SWEEP COMPLETE 2026-09-03 (D-1156, state-manager, S25.01-FINALIZATION-DOC-SWEEP-COMPLETE burst).** Every
> item below is now RESOLVED / ACCEPTED (won't-fix) / VERIFIED CONFORMANT / DEFERRED-with-concrete-anchor. No
> item remains OPEN without a disposition. `feature/S-25.01` is **READY-FOR-PR @ `3e463cdc`**
> (`3919ebcb` LOCAL-3-CLEAN-certified base → `f1400e35` O-P18-002 test-tightening GREEN → `b46f48f6` LOW-1 fix +
> TD-VSDD-060 sibling sweep → `3e463cdc` demo evidence under `docs/demo-evidence/S-25.01/`). Two items
> (`[process-gap]` registry-comment-lint and O-P16-1) are tracked as `.factory/STATE.md` Drift Items per the
> Cycle-Closing Checklist S-7.02 minimum (justified deferral with concrete anchor, in lieu of a full follow-up
> story) rather than swept in-scope, consistent with their PROCESS-GAP (not S-25.01-code) nature. O-P17-001 and
> O-P18-001 are DEFERRED to dedicated follow-up stories (anchors also recorded in `.factory/STATE.md` Drift
> Items, D-1156) — O-P18-001's full architect analysis is persisted at
> `cycles/v1.0-brownfield-backfill/O-P18-001-timestamp-utc-vs-offset-analysis.md`. See the Status table below
> for the per-item disposition and the commit/anchor that closes each.

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
| LOW-1 (RegistryError::AsyncBlockConflict msg) | **RESOLVED 2026-09-03** — reworded to name the non-blocking remedies (`on_error=continue`/`advisory`) instead of a still-rejected blocking policy; test-writer RED + implementer GREEN, commit `b46f48f6` (`fix(dispatcher): E-REG-002 message names actual on_error value, not hardcoded "block"`); TD-VSDD-060 sibling sweep swept `registry.rs` def+construction, `main.rs` match arm, `async_partition_integration.rs` destructure; 291 lib+integration tests GREEN, fmt+clippy clean | test-writer + implementer, `feature/S-25.01` `b46f48f6` |
| OBS-3 (write_indeterminate_marker .tmp orphan) | **RESOLVED (already fixed by F-P3-004)** — best-effort `.tmp` cleanup before propagating the rename error was already present, verified at `3919ebcb` in `indeterminate_marker.rs`; no further action needed | verified-fixed, `feature/S-25.01` @ `3919ebcb` (F-P3-004) |
| [process-gap] registry-comment-lint | **DEFERRED — tracked in `.factory/STATE.md` Drift Items (D-1156), anchored E-12 follow-up story (no ID allocated yet)** | orchestrator/E-12 follow-up |
| OBS-1 (crash posture) | **VERIFIED CONFORMANT — no action** | adversary pass 1 |
| OBS-2 (quoting) | **VERIFIED CONFORMANT — no action** | adversary pass 1 |
| O-P16-1 (`[process-gap]` adversary dispatch template stale WASM plugin path) | **DEFERRED — tracked in `.factory/STATE.md` Drift Items (D-1156), anchored E-12 follow-up story (no ID allocated yet); process-improvement, does not block PR** | orchestrator/E-12 follow-up |
| O-P16-2 (`classify_outcome` `_policy` unused param, already documented in-code) | **ACCEPTED (won't-fix)** 2026-09-03 — intentional: retained for AC-004 signature parity; already documented in-code via orchestrator-ruling NOTE comment and in the story's orchestrator note; adversary pass 16 itself noted "no action required" | adversary pass 16 disposition; state-manager finalization sweep |
| O-P16-3 (`reconcile_raw_delete` today-only + 256KB-tail bounds) | **VERIFIED CONFORMANT — no action (bounded/best-effort per BC-3.08.001 Inv 3 / ADR-048 §D4)** | adversary pass 16 |
| O-P17-001 (`[audit-robustness]` REVALIDATED-clear guard/emission read-function asymmetry — audit gap on malformed markers) | **DEFERRED to follow-up story** 2026-09-03 — unreachable via any current production path (`write_indeterminate_marker` is the sole production writer and always writes all six `MarkerFields` atomically; only an externally-tampered marker or a future schema change could trigger this gap, outside the single-operator threat model). Tracked in `.factory/STATE.md` Drift Items (D-1156), anchored to a new tampered/malformed-marker audit-robustness hardening follow-up story (no ID allocated yet) | implementer, IF actioned (follow-up story) |
| O-P17-002 (VP-108 Event 9/10 wire tables omit `session_id`) | **ACCEPTED (won't-fix)** 2026-09-03 — presentational, not a contract conflict: VP-108's per-event tables are an intentional content-bearing-field subset view, and the omitted host-injected common fields are documented centrally in BC-3.08.001 §Common Fields; adversary disposition "no action required" | adversary pass 17 disposition; state-manager finalization sweep |
| O-P18-001 (audit-timestamp LOCAL-offset ISO-8601 vs ADR-048 §D4 "ISO-8601 UTC" wording) | **DEFERRED to dedicated follow-up story; architect recommends Direction A; POLICY 22 ratification pending** 2026-09-03 — full architect analysis persisted at `cycles/v1.0-brownfield-backfill/O-P18-001-timestamp-utc-vs-offset-analysis.md`; project-wide, outside S-25.01 delta; NOT fixed this cascade. Tracked in `.factory/STATE.md` Drift Items (D-1156) | architect/product-owner (dedicated follow-up story, precondition = human Direction A/B/hybrid selection) |
| O-P18-002 (VP-108 PC1 REVALIDATED integration test missing `trace_id`-equality assertion) | **RESOLVED 2026-09-03** — one-line `assert_eq!(cleared_events[0]["trace_id"], "trace-integ-test")` added; test-writer commit `f1400e35` (`test(dispatcher): tighten VP-108 PC1 REVALIDATED clear to assert trace_id provenance`), GREEN | test-writer, `feature/S-25.01` `f1400e35` |

*S-25.01 section added: 2026-08-31 (S2501-LOCAL-ADV-PASS1-CLEAN-STREAK-1of3-2026-08-31 — state-manager; BC-5.39.001 streak 1/3; artifact FROZEN @ 92990371)*

---

### O-P16-1 — Adversary dispatch template cites stale/nonexistent WASM plugin path

| Field | Value |
|-------|-------|
| **Finding ID** | O-P16-1 |
| **Severity** | LOW / `[process-gap]` |
| **Source pass** | Pass 16 (LOCAL adversary pass 16 CLEAN 2026-09-03, D-1153) |
| **File** | S-25.01 adversary dispatch template (code-perimeter list) |
| **Observation** | The template names the gate WASM plugin at `plugins/vsdd-factory/hooks/validate-unvalidated-mutation-marker/src/lib.rs`, which does not exist (confirmed via literal `ls` — ENOENT). The real path is `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs` (confirmed present). |
| **Impact** | No review gap resulted this pass — the adversary found and reviewed the correct file despite the stale template path. Purely a template-hygiene issue that wastes a glob on future passes. |
| **Fix** | Correct the S-25.01 adversary dispatch template's code-perimeter path to `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs`. |
| **Routing** | story-writer/orchestrator (dispatch-template owner) |
| **Blocking?** | No — cosmetic template correction only |

---

### O-P16-2 — `classify_outcome`'s `_policy` parameter unused (already documented, candidate signature refinement)

| Field | Value |
|-------|-------|
| **Finding ID** | O-P16-2 |
| **Severity** | LOW |
| **Source pass** | Pass 16 (LOCAL adversary pass 16 CLEAN 2026-09-03, D-1153) |
| **File** | `crates/factory-dispatcher/src/executor.rs` — `classify_outcome` (~lines 132-140 in the frozen `feature/S-25.01` worktree) |
| **Observation** | `classify_outcome`'s `_policy: FailurePolicy` parameter is genuinely unused inside the function body — classification is policy-independent; `policy` is consumed downstream by `should_write_marker` instead. Already documented in-code via an orchestrator-ruling NOTE comment (confirmed via literal `sed` excerpt) and in the story's orchestrator note. |
| **Disposition** | Retained as-is for AC-004 signature parity (spec-mandated signature; spec-wins). Candidate spec-signature refinement — surface to product-owner as a possible follow-up; not a defect. |
| **Routing** | product-owner (spec-signature adjudication, if actioned) |
| **Blocking?** | No — documented, spec-conformant, no behavior gap |

---

### O-P16-3 — `reconcile_raw_delete` today-only + 256KB-tail scan bounds (verified conformant, no action)

| Field | Value |
|-------|-------|
| **Finding ID** | O-P16-3 |
| **Severity** | OBSERVATION — VERIFIED CONFORMANT |
| **Source pass** | Pass 16 (LOCAL adversary pass 16 CLEAN 2026-09-03, D-1153) |
| **Observation** | A T3 raw-delete whose `marker.written` audit event fell outside `reconcile_raw_delete`'s today-only + 256KB-tail scan window will not reconcile to `OPERATOR_OVERRIDE`. |
| **Disposition** | VERIFIED CONFORMANT — explicitly spec'd bounded/best-effort behavior per BC-3.08.001 Invariant 3 and ADR-048 §D4. Correct-by-spec; noted for completeness only. No action required. |

---

*Pass 16 items added: 2026-09-03 (S2501-PASS16-CLEAN-STREAK-ADVANCE-BOOKKEEPING — state-manager; BC-5.39.001 streak 0/3→1/3; artifact FROZEN @ 3919ebcb; D-1153)*

---

### O-P17-001 — REVALIDATED-clear guard reads `read_marker_plugin_name` but emission reads `read_all_marker_fields` (audit-robustness gap on malformed markers)

| Field | Value |
|-------|-------|
| **Finding ID** | O-P17-001 |
| **Severity** | LOW / `[audit-robustness]` |
| **Source pass** | Pass 17 (LOCAL adversary pass 17 CLEAN 2026-09-03, D-1154) |
| **File** | `crates/factory-dispatcher/src/executor.rs` — `execute_tier` (~lines 547/553) and `spawn_async_plugin` (~lines 860/865) in the frozen `feature/S-25.01` worktree |
| **Observation** | Both callsites guard the REVALIDATED-clear branch on `read_marker_plugin_name(&marker_path)` but read the emission fields separately via `read_all_marker_fields(&marker_path).ok().flatten()`. A marker that parses `plugin_name`+`artifact_path` (satisfying the guard) but is missing one of `timestamp`/`cause`/`trace_id` (failing the stricter `read_all_marker_fields` parse) would still be deleted by `delete_marker_if_pass` (`Ok(true)`), but with `all_fields = None` — so NO `marker.cleared(REVALIDATED)` audit event is emitted. A BLOCKING→ALLOWING state transition would occur with no audit record, a NIST AU-3 gap of exactly the class ADR-048 §D4's audited-clear-event design exists to close. |
| **Reachability** | NOT reachable via any current production path — confirmed via literal shell (burst-log D-1154 Block 5): `write_indeterminate_marker` is the sole production writer and always writes all six `MarkerFields` atomically via temp+rename; cross-pair overwrites also always write complete markers. Only an externally-tampered marker file or a future schema change with a lenient guard/strict-emission asymmetry could trigger this gap — outside the single-operator threat model this story's spec (BC-1.18.001/002/003) is scoped to. |
| **Fix (if ever actioned)** | Emit `marker.cleared(REVALIDATED)` from the guard's own `read_marker_plugin_name` result plus a synthesized minimal `MarkerFields` (using the guard-read `plugin_name`, the known `artifact_path`, and placeholder/best-effort values for the remaining fields) when `all_fields` is `None` but `delete_marker_if_pass` returned `Ok(true)` — so the audit record is never silently dropped even for a malformed-but-guard-passing marker. |
| **Routing** | implementer (candidate fix, if actioned) |
| **Blocking?** | No — unreachable via any current production write path; audit-completeness hardening only |

---

### O-P17-002 — VP-108 Event 9/Event 10 wire-format tables omit `session_id` (doc-completeness, not a contract conflict)

| Field | Value |
|-------|-------|
| **Finding ID** | O-P17-002 |
| **Severity** | LOW / `[doc-completeness]` |
| **Source pass** | Pass 17 (LOCAL adversary pass 17 CLEAN 2026-09-03, D-1154) |
| **File** | `.factory/specs/verification-properties/VP-108.md` — Event 9 (lines 393-404) and Event 10 (lines 406-416) wire-format tables |
| **Observation** | Both tables omit `session_id` (and `ts_epoch`/`schema_version`), which `BC-3.08.001` §Common Fields declares present on all ten event types (host-injected via `HostContext::emit_internal`'s common-field enrichment, RESERVED_FIELDS — never plugin-supplied) and which the code does in fact emit on Events 9 and 10 like every other event. Confirmed via literal `grep`: VP-108's tables have zero `session_id` hits; BC-3.08.001's §Common Fields row explicitly states "Present on all ten event types." |
| **Disposition** | Not a contract conflict — VP-108's per-event wire-format tables are a content-bearing-field subset view (the fields that event's own emitting call constructs/supplies), not a full-wire-envelope view; the common host-injected fields (`trace_id`, `session_id`, `ts`, `ts_epoch`) are documented once, centrally, in BC-3.08.001 §Common Fields rather than repeated on every per-event table. Presentational completeness gap only. |
| **Fix (if ever actioned)** | Add a one-line cross-reference note to VP-108's two wire-format tables pointing to BC-3.08.001 §Common Fields for the host-injected common fields, so a reader of VP-108 alone does not infer `session_id` is absent from the actual wire event. |
| **Routing** | architect (VP-108 owner, if actioned) |
| **Blocking?** | No — presentational documentation completeness only; no behavioral or contract gap |

---

*Pass 17 items added: 2026-09-03 (S2501-PASS17-CLEAN-STREAK-ADVANCE-BOOKKEEPING — state-manager; BC-5.39.001 streak 1/3→2/3; artifact FROZEN @ 3919ebcb; D-1154)*

---

### O-P18-001 — Audit event timestamps use LOCAL-offset ISO-8601, not the "ISO-8601 UTC" wording ADR-048 §D4 uses (project-wide convention question, not an S-25.01-specific defect)

| Field | Value |
|-------|-------|
| **Finding ID** | O-P18-001 |
| **Severity** | LOW / `[spec-vs-code-convention]` — REQUIRES ARCHITECT/PRODUCT-OWNER ADJUDICATION |
| **Source pass** | Pass 18 (LOCAL adversary pass 18 CLEAN 2026-09-03 — LOCAL BC-5.39.001 3-CLEAN CONVERGENCE ACHIEVED, D-1155) |
| **File** | `crates/factory-dispatcher/src/internal_log.rs` (`InternalEvent::now`/`with_ts`), `indeterminate_marker.rs` (`emit_marker_cleared`), `executor.rs` (`emit_indeterminate`) — all in the frozen `feature/S-25.01` worktree |
| **Observation** | Audit event timestamps (`marker.cleared`/`marker.written`/`plugin.indeterminate`) use LOCAL-offset ISO-8601 via `InternalEvent::now`/`with_ts` (`Local::now()` + `%z`, e.g. `2026-08-30T12:00:00-0500`), while ADR-048 §D4's field contract says "ISO-8601 UTC" in 4 places (confirmed via literal `grep`, burst-log D-1155 Block 5, e.g. its §Wire Format Event 9 field table row for `timestamp` reads "ISO-8601 UTC", "YES"). |
| **Scope** | The value is a valid offset-unambiguous ISO-8601 instant and is the UNIFORM dispatcher-wide convention across EVERY BC-3.08.001 event (Event 8 included, already shipped/audited) — so there is NO consumer ambiguity, and this is NOT an S-25.01-specific defect. It is a pre-existing, dispatcher-wide convention that predates this story. |
| **Reconciliation options** | (a) Relax the ADR-048 §D4 wording from "ISO-8601 UTC" to "ISO-8601 with offset" (documentary fix, matches shipped behavior); or (b) normalize all emitters to a UTC field (code change, risks a breaking change to existing audit-log consumers who may already parse the local-offset form). |
| **Disposition** | Recorded and marked **PENDING ARCHITECT/PRODUCT-OWNER ADJUDICATION — project-wide, outside S-25.01 delta.** NOT fixed in this cascade; fixing it in-scope of S-25.01 would either edit the frozen 3-CLEAN-certified artifact (option b) or edit an ADR outside S-25.01's own delta (option a) without the cross-component reasoning a project-wide convention change requires. |
| **Routing** | architect (ADR-048 owner) + product-owner (BC-3.08.001 owner), joint adjudication |
| **Blocking?** | No — ADVISORY only; does NOT affect S-25.01 convergence (3-CLEAN already ACHIEVED) |

---

### O-P18-002 — VP-108 PC1 REVALIDATED integration test does not assert `trace_id` equality (test-tightening, transitively covered elsewhere)

| Field | Value |
|-------|-------|
| **Finding ID** | O-P18-002 |
| **Severity** | LOW / `[test-tightening]` |
| **Source pass** | Pass 18 (LOCAL adversary pass 18 CLEAN 2026-09-03 — LOCAL BC-5.39.001 3-CLEAN CONVERGENCE ACHIEVED, D-1155) |
| **File** | `crates/factory-dispatcher/tests/marker_integration.rs` — `test_BC_1_18_003_named_plugin_pass_clears_marker_via_execute_tiers` (VP-108 PC1) in the frozen `feature/S-25.01` worktree |
| **Observation** | The test asserts `cleared_events[0]["clear_mode"] == "REVALIDATED"` plus a non-empty `timestamp` field (confirmed via literal `sed` excerpt, burst-log D-1155 Block 5, lines 247-256), but does NOT assert that the emitted event's `trace_id` equals the marker's own fixture value (`"trace-integ-test"`, confirmed via literal `grep`, Block 5, lines 139/153). |
| **Coverage status** | Covered transitively via `emit_marker_cleared`'s shared unit tests exercising PC2/PC3/PC5's `trace_id` provenance paths — this is a coverage-density gap on THIS integration test specifically, not an unverified production behavior. |
| **Fix** | Add `assert_eq!(cleared_events[0]["trace_id"], "trace-integ-test");` to the test body. One line. |
| **Routing** | test-writer |
| **Blocking?** | No — does not affect S-25.01 convergence (3-CLEAN already ACHIEVED); transitively covered by sibling unit tests |

---

*Pass 18 items added: 2026-09-03 (S2501-PASS18-3CLEAN-CONVERGED-BURST — state-manager; BC-5.39.001 streak 2/3→3/3 CONVERGED; artifact FROZEN @ 3919ebcb; D-1155)*

*S-25.01 finalization sweep COMPLETE: 2026-09-03 (D-1156, state-manager, S25.01-FINALIZATION-DOC-SWEEP-COMPLETE burst) — all items above disposed (RESOLVED/ACCEPTED/VERIFIED-CONFORMANT/DEFERRED-with-anchor); `feature/S-25.01` READY-FOR-PR @ `3e463cdc`.*
