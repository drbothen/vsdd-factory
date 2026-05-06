---
pass_id: 60
angle: "TV (Canonical Test Vector) coverage matrix vs Postcondition coverage audit — for BC-1.05.035 and BC-1.05.036, build PC/EC → TV mapping; flag PCs/ECs without TV witnesses, orphan TVs, missing boundary cases, missing failure modes per TD-VSDD-085 broad adversary axis"
surface: "E-9 epic v1.52 (c176bc2) + BC-1.05.035 + BC-1.05.036 + BC-INDEX (post-D-304) + lessons.md (TD-VSDD-085, TD-VSDD-093) + STORY-INDEX v2.11 + open-questions.md + pass-52..59 review files; cross-checked against source-of-truth Rust at exec_subprocess.rs / host/mod.rs / internal_log.rs"
anchor_commit: "c176bc2"
date: "2026-05-06"
adversary_model: "claude-opus-4-7[1m]"
prior_clock_state: "0_of_3 (RESET by D-304 closing HIGH-P59-001)"
final_verdict: "SUBSTANTIVE — 0 HIGH / 4 MEDIUM / 1 LOW + 2 non-blocking observations"
findings_count:
  HIGH: 0
  MEDIUM: 4
  LOW: 1
  observations: 2
clock_state_output: "0_of_3 → 0_of_3 (HOLD; SUBSTANTIVE verdict — 4 MEDIUM + 1 LOW found; clock cannot advance with MEDIUM findings open)"
defect_class: "CTV coverage matrix gaps — normative PCs and documented ECs lacking TV witnesses; sibling-class to HIGH-P39-003 (TD-VSDD-085 self-violation); novel CTV-coverage-matrix angle vs prior 59 passes"
td_vsdd_093_application: "10-row quote-verification log; all PASS"
---

# Adversarial Review Pass 60 — E-9 v1.52

## CTV Coverage Matrix vs Postcondition Coverage Audit

**Pass ID:** 60
**Surface:** E-9 epic v1.52 (c176bc2) + BC-1.05.035 + BC-1.05.036 + BC-INDEX (post-D-304) + lessons.md (TD-VSDD-085, TD-VSDD-093) + STORY-INDEX v2.11 + open-questions.md + pass-52..59 review files; cross-checked against source-of-truth Rust at exec_subprocess.rs / host/mod.rs / internal_log.rs
**Angle:** TV (Canonical Test Vector) coverage matrix vs Postcondition coverage audit — build PC/EC → TV mapping for BC-1.05.035 and BC-1.05.036; flag PCs/ECs without TV witnesses, orphan TVs, missing boundary cases, missing failure modes per TD-VSDD-085 broad adversary axis
**Anchor commit:** c176bc2
**Date:** 2026-05-06
**Prior clock state:** 0_of_3 (RESET by D-304 closing HIGH-P59-001)
**Model:** claude-opus-4-7[1m]

---

## Procedure Summary

| Step | Action | Result |
|------|--------|--------|
| 1 | Read BC-1.05.035 §Postconditions, §Edge Cases, §Canonical Test Vectors | Complete — 2 PCs, 15 ECs, 9 CTV rows |
| 2 | Read BC-1.05.036 §Postconditions, §Edge Cases, §Canonical Test Vectors | Complete — 6 PCs, 20 ECs, 19 CTV rows |
| 3 | Build PC/EC → TV bidirectional mapping for BC-1.05.035 | PC-1: TV-1..9 (witness via deny-path rows). PC-2: NO DIRECT TV WITNESS → MED-P60-001 |
| 4 | Build PC/EC → TV bidirectional mapping for BC-1.05.036 | PC-1/2: TV-1..19 partial. PC-3: NO ACTIVE TIMING WITNESS → MED-P60-002. EC-013A boundary split: TV-4/5/6 cover upper-bound note but no lower-bound (0) or 5ms-floor TV → MED-P60-003. EC-013B bifurcation: single TV row for "max_output_bytes = 0" but does not separately witness any-byte→error vs zero-byte→success branches → MED-P60-004 |
| 5 | Inspect TV-9 EC-007 collapse note for by-design framing | TV-9 outcome cell lacks "by-design cause-collapse NOTE" explicitly citing TD-VSDD-092 framing → LOW-P60-001 |
| 6 | TD-VSDD-085 broad adversary axis: orphan TVs | No orphan TVs found (all CTV rows trace to at least one PC or EC) |
| 7 | TD-VSDD-093 10-row quote-verification log | All 10 rows PASS (see §TD-VSDD-093 Quote-Verification Log) |
| 8 | 5-axis sibling sweep per TD-VSDD-089 | Completed — see §TD-VSDD-089 5-Axis Sibling Sweep |
| 9 | Novelty assessment | CTV coverage-matrix primary angle is novel vs 59 prior passes (sibling to TD-VSDD-085 failure-mode matrix; prior passes used failure-mode-coverage or partial-fix-regression frames, not systematic PC/EC→TV bidirectional mapping) |
| 10 | Verdict determination | SUBSTANTIVE: 4 MEDIUM + 1 LOW coverage gaps found; clock cannot advance |

---

## BC-1.05.035 Coverage Matrix

| Postcondition / Edge Case | TV Witnesses | Gap? |
|---------------------------|-------------|------|
| PC-1: deny → emit_denial + CAPABILITY_DENIED (-1) | TV-1..TV-6 (deny-path rows: binary_canonicalize_failed, binary_not_on_allow_list, shell_bypass, setuid/setgid, NUL-path, symlink-loop) | COVERED |
| PC-2: non-UTF-8 cmd → INVALID_ARGUMENT (-4) pre-canonicalize | TV-7..TV-9 do NOT directly test `cmd = b"\xFF\xFE"` (non-UTF-8); TV-7 tests binary_not_on_allow_list; TV-9 tests capability passed (success-path handoff). Precedence Ladder step (1) normative postcondition — the HostCallError::InvalidUtf8 path — is unwitnessed by any named TV row. | GAP → MED-P60-001 |
| EC-001..EC-014: all edge cases | BC-035 CTVs partially cover allow-list misses, NUL bytes, symlink loops, args lossy UTF-8, env_allow silent-skip, capability-checks | All ECs except the PC-2 read_wasm_string Err path have at least one witness |

*BC-1.05.035 has 9 CTV rows at v1.52. PC-2 normative witness gap confirmed.*

---

## BC-1.05.036 Coverage Matrix

| Postcondition / Edge Case | TV Witnesses | Gap? |
|---------------------------|-------------|------|
| PC-1: single event emitted on success | TV-1 (basic success path) | COVERED |
| PC-2: best-effort-read payload fields | TV-1, TV-16..TV-17 (stdout_bytes semantic) | COVERED |
| PC-3: duration_ms measured from before spawn to exit | No TV row exercises `duration_ms` field active timing — TV rows either assert event presence or error outcomes; none assert `duration_ms ∈ [expected_range]` for a subprocess with known runtime | GAP → MED-P60-002 |
| PC-4: emit_internal bifurcation (internal_log: Some/None) | TV-18 (internal_log: None test context), TV-12 (Some production path indirect) | COVERED |
| PC-5: error paths emit no event | TV-9 (INTERNAL_ERROR), TV-11 (TIMEOUT), TV-13 (OUTPUT_TOO_LARGE) | COVERED |
| PC-6: Postcondition 6 emit IO error best-effort | TV-11 (EC-010 indirect) | COVERED |
| EC-013A: timeout_ms=0, small values, u32::MAX | TV-4..TV-6 contain `timeout_ms` boundary notes but only EC-013A prose describes 0-lower-bound behavior and 5ms granularity floor (LOW-P51-005). No CTV row tests `timeout_ms = 0` explicitly (lower bound of kill-immediately). No CTV row tests `timeout_ms = 1` vs 5ms-floor (LOW-P51-005). EC-013A `timeout_ms = u32::MAX` (LOW-P52-002) was added as prose note to EC-013A but no dedicated TV row was authored | GAP → MED-P60-003 |
| EC-013B: max_output_bytes=0 bifurcation | EC-013B describes two cases: (a) any-byte subprocess → OUTPUT_TOO_LARGE; (b) zero-byte subprocess → success. Single TV row for "max_output_bytes = 0" does not bifurcate these two behaviors into separate witnessing rows | GAP → MED-P60-004 |
| EC-007: INTERNAL_ERROR (TV-9) collapse | TV-9 witnesses INTERNAL_ERROR outcome. Outcome cell does not explicitly state the 4-path cause-collapse (spawn, stdin write_all, try_wait, pipe take) as a by-design note framing per TD-VSDD-092 BC-SOUL4-coverage v1 known-limitation | GAP → LOW-P60-001 |

*BC-1.05.036 has 19 CTV rows at v1.52. PC-3 timing witness gap, EC-013A 3-boundary gap, EC-013B bifurcation gap, EC-007 cause-collapse framing gap confirmed.*

---

## Findings

### MED-P60-001 — BC-1.05.035 PC-2 read_wasm_string Err path (HostCallError::InvalidUtf8) lacks TV witness

**Severity:** MEDIUM
**Class:** CTV coverage gap — normative postcondition without TV witness

**Evidence (TD-VSDD-093 quote-verified):**

BC-1.05.035 §Postconditions PC-2 (quote from BC body):
> "If `read_wasm_string` returns Err (non-UTF-8 byte sequence in WASM memory), the existing host-call error path returns `codes::INVALID_ARGUMENT` (-4) before any canonicalize attempt."

BC-1.05.035 §Canonical Test Vectors v1.52 rows TV-1..TV-9 (reviewed):
- TV-1: `cmd = "/usr/bin/false"` (allow-list miss via canonicalize: binary_canonicalize_failed) — witnesses PC-1 deny path
- TV-2: `cmd = "/bin/bash"` allow-list miss — witnesses PC-1 deny path
- TV-3: NUL-containing cmd — witnesses CAPABILITY_DENIED via canonicalize EINVAL
- TV-4..TV-6: binary_not_on_allow_list, shell_bypass, setuid/setgid
- TV-7: args lossy UTF-8 — witnesses EC-012
- TV-8: env_allow silent-skip — witnesses EC-014
- TV-9: capability passed (success path handoff to BC-036)

None of TV-1..TV-9 provide a row where `cmd = b"\xFF\xFE"` (or any non-UTF-8 byte sequence) causing `read_wasm_string` to return `HostCallError::InvalidUtf8` at memory.rs:53, returning `INVALID_ARGUMENT` (-4) via the Precedence Ladder step (1) catch-all `Err(_) =>` arm at exec_subprocess.rs:51-52. PC-2 is a normative postcondition; per TD-VSDD-085 broad adversary axis, normative postconditions MUST have TV witnesses.

**Source-of-truth verification:** exec_subprocess.rs:51-52 `Err(_) => return Err(codes::INVALID_ARGUMENT)` — confirmed as the PC-2 error return site. memory.rs:53 `String::from_utf8(bytes).map_err(|_| HostCallError::InvalidUtf8)?` — confirmed as the InvalidUtf8 generation site.

**Fix:** Add TV-10: `cmd = b"\xFF\xFE"` → `INVALID_ARGUMENT` (-4) witness. Include coverage extension note documenting all 3 HostCallError variants (InvalidUtf8 at memory.rs:53; MemoryOverflow at memory.rs:33; OutOfBounds at memory.rs:35-40) collapsing to same -4 via the same catch-all arm per LOW-P51-001 cause-collapse note.

---

### MED-P60-002 — BC-1.05.036 PC-3 duration_ms active timing witness missing

**Severity:** MEDIUM
**Class:** CTV coverage gap — normative postcondition without active TV witness

**Evidence (TD-VSDD-093 quote-verified):**

BC-1.05.036 §Postconditions PC-3 (quote from BC body):
> "`duration_ms` is measured from `Instant::now()` at `Command::spawn()` to process exit; implementer adds a `let started = Instant::now();` capture immediately before `command.spawn()` at exec_subprocess.rs:252 (which is the actual spawn point); the existing deadline `Instant` at exec_subprocess.rs:270 is post-spawn and is NOT the duration reference."

Review of BC-1.05.036 §Canonical Test Vectors v1.52 rows TV-1..TV-19: no row asserts `duration_ms ∈ [expected_range]` for a subprocess with a known runtime (e.g., `sleep 0.5` with `timeout_ms = 5000` → `duration_ms ∈ [490, 600]`). Without an active timing assertion, the PC-3 semantic — that `started` is captured BEFORE spawn, not from the post-spawn deadline Instant — is unwitnessed in the CTV corpus. A misimplementation using the deadline Instant would produce an observably different `duration_ms` only with a test that asserts the expected range.

**Source-of-truth verification:** exec_subprocess.rs:252 `command.spawn()` — spawn point. exec_subprocess.rs:270 `let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64)` — post-spawn deadline. PC-3 normative requirement: `started` capture before line 252.

**Fix:** Add TV-20: `command = "sleep"`, `args = ["0.5"]`, `timeout_ms = 5000` → `duration_ms ∈ [490, 600]`. Document that this witnesses PC-3 pre-spawn timing semantics.

---

### MED-P60-003 — BC-1.05.036 EC-013A missing 3 boundary TV witnesses (timeout_ms=0 lower, ~5ms floor, u32::MAX upper)

**Severity:** MEDIUM
**Class:** CTV coverage gap — documented edge case boundaries without dedicated TV witnesses

**Evidence (TD-VSDD-093 quote-verified):**

BC-1.05.036 §Edge Cases EC-013A (quote from BC body, timeout_ms=0 lower bound):
> "**Lower-bound (zero/small):** Deadline is set to `Instant::now() + Duration::from_millis(0)` at exec_subprocess.rs:270. On the first poll iteration at the `if Instant::now() >= deadline` check (exec_subprocess.rs:292), the deadline has already expired..."

BC-1.05.036 §Edge Cases EC-013A (quote from BC body, 5ms floor):
> "**5ms busy-poll granularity (LOW-P51-005):** The polling loop at exec_subprocess.rs:297 sleeps `std::thread::sleep(Duration::from_millis(5))` on each `Ok(None)` (child still running) iteration... A caller specifying `timeout_ms = 1` will not observe a timeout at 1ms..."

BC-1.05.036 §Edge Cases EC-013A (quote from BC body, u32::MAX):
> "**Upper-bound (`timeout_ms = u32::MAX`, LOW-P52-002):** At exec_subprocess.rs:270, `Duration::from_millis(timeout_ms as u64)` accepts `u32::MAX` (4,294,967,295) cast to `u64` without overflow..."

Review of BC-1.05.036 §Canonical Test Vectors v1.52: no CTV row tests `timeout_ms = 0` explicitly (lower-bound kill-immediately behavior). No CTV row tests `timeout_ms = 1` with a subprocess known to run longer than 1ms (to witness the 5ms granularity floor from LOW-P51-005). LOW-P52-002 upper-bound was added as EC-013A body prose but has no dedicated CTV row.

Three distinct boundary behaviors documented in EC-013A; none have dedicated TV witnesses. Per TD-VSDD-085 broad adversary axis, documented edge cases with boundary semantics require TV witnesses.

**Source-of-truth verification:** exec_subprocess.rs:270 deadline construction. exec_subprocess.rs:292 deadline check. exec_subprocess.rs:297 `std::thread::sleep(Duration::from_millis(5))`.

**Fix:** Add 3 CTV rows:
- TV-21: `timeout_ms = 0`, any subprocess → TIMEOUT (-2) witnesses lower bound (kill immediately on first poll)
- TV-22: `timeout_ms = 1`, `sleep 0.01` → effective TIMEOUT at ~5ms witnesses 5ms granularity floor (LOW-P51-005 / EC-013A)
- TV-23: `timeout_ms = u32::MAX`, `/usr/bin/true` → success (witnesses LOW-P52-002 operationally-unbounded upper bound)

---

### MED-P60-004 — BC-1.05.036 EC-013B max_output_bytes=0 bifurcation lacks 2 separate TV witnesses

**Severity:** MEDIUM
**Class:** CTV coverage gap — documented bifurcation behavior without separate TV witnesses per branch

**Evidence (TD-VSDD-093 quote-verified):**

BC-1.05.036 §Edge Cases EC-013B (quote from BC body):
> "`max_output_bytes = 0` ... `stdout_buf.len() > 0 as usize` is true if subprocess produces ANY output (even 1 byte); `truncated = true` fires immediately; returns `Err(OUTPUT_TOO_LARGE -3)`. If subprocess produces exactly zero bytes of stdout AND zero bytes of stderr, `truncated = false` and the success path proceeds with empty buffers."

Review of BC-1.05.036 §Canonical Test Vectors v1.52: the single CTV row for `max_output_bytes = 0` (added per MED-P52-001 boundary semantic context) tests the boundary-success path where `len == max_output_bytes` evaluates `0 > 0 = false`. It does NOT separately test the any-byte → OUTPUT_TOO_LARGE branch and the zero-byte → success branch as distinct rows. The EC-013B bifurcation has two distinct observable outcomes and requires two CTV rows per TD-VSDD-085.

**Source-of-truth verification:** exec_subprocess.rs:278 `if stdout_buf.len() > max_output_bytes as usize` — strict `>` check. The bifurcation at `max_output_bytes = 0` produces OUTPUT_TOO_LARGE (-3) for any subprocess with ≥1 byte, and success for zero-byte subprocess.

**Fix:** Add 2 CTV rows:
- TV-24: `max_output_bytes = 0`, `/bin/echo x` (2 bytes stdout) → OUTPUT_TOO_LARGE (-3) witnesses any-byte branch
- TV-25: `max_output_bytes = 0`, `/usr/bin/true` (0 bytes) → success witnesses zero-byte branch; reinforces MED-P52-001 boundary semantic

---

### LOW-P60-001 — BC-1.05.036 TV-9 EC-007 outcome cell lacks by-design cause-collapse NOTE per TD-VSDD-092

**Severity:** LOW
**Class:** BC framing clarity — silent-cause-collapse pattern undisclosed at CTV level

**Evidence:**

BC-1.05.036 §Canonical Test Vectors v1.52 TV-9 outcome cell (summary): "Returns `Err(INTERNAL_ERROR -99)`; NO event emitted in v1 per Postcondition 5 (spawn at :252, stdin take/write at :258/:259-262, stdout/stderr take at :267-268, try_wait at :299); `host.exec_subprocess.completed` NOT emitted."

EC-007 body documents the 4-path cause-collapse: spawn io::Error at exec_subprocess.rs:252 (MED-P50-001), stdin write_all at exec_subprocess.rs:259 (LOW-P51-003), try_wait at exec_subprocess.rs:299 (LOW-P51-004), stdout/stderr pipe take at exec_subprocess.rs:267-268. TD-VSDD-092 BC-SOUL4-coverage v1 known-limitation framing acknowledges mock-injection-required witnesses deferred to v2 (consistent with TV-19 EC-016 precedent). However, TV-9's outcome cell itself does not acknowledge this by-design collapse — a reader inspecting only the CTV table would not know that this single row witnesses 4 distinct cause paths by design, or that the deferred v2 expansion is intentional.

Per TD-VSDD-092 BC-SOUL4-coverage v1 framing, cause-collapse patterns in the CTV layer should acknowledge the known-limitation inline.

**Fix:** Update TV-9 outcome cell to append "By-design cause-collapse NOTE (LOW-P60-001 / TD-VSDD-092 v1 known-limitation): This single TV row witnesses the OUTCOME for ALL 4 cause-erasure paths enumerated in EC-007 (spawn at :252, stdin write_all at :259, try_wait at :299, pipe take at :267-268). 4 paths produce IDENTICAL observable outcomes; mock-injection-required individual-path witnesses deferred to v2 (consistent with TV-19 EC-016 precedent)."

---

### Obs-P60-001 — BC-1.05.035 and BC-1.05.036 CTV density asymmetry (non-blocking)

**Severity:** Non-blocking observation
**Class:** Coverage density note

BC-1.05.035 at v1.52 has 9 CTV rows covering 2 PCs and 15 ECs. BC-1.05.036 at v1.52 has 19 CTV rows covering 6 PCs and 20 ECs. After D-305 closes (BC-035 grows to 10 rows; BC-036 grows to 25 rows), the density gap narrows: BC-035 is naturally less CTV-dense because its postconditions (deny-path telemetry) are more uniform across ECs, while BC-036 covers success-path + multiple error paths with distinct behavioral semantics requiring more witnesses. The asymmetry is expected and appropriate. No action required.

---

### Obs-P60-002 — TD-VSDD-085 broad adversary axis self-application: no orphan TVs found (non-blocking)

**Severity:** Non-blocking observation
**Class:** Positive finding

All TV rows in BC-1.05.035 (TV-1..TV-9) and BC-1.05.036 (TV-1..TV-19) trace to at least one normative PC or documented EC. No orphan TVs (rows without backward-traceable anchor) were found. The CTV additions recommended in this pass (TV-10, TV-20..TV-25) are all anchored to specific PCs or ECs. No orphan-TV cleanup is needed.

---

## TD-VSDD-093 Quote-Verification Log

| Row | Claimed Content | Source File | Quote Verified | Result |
|-----|----------------|-------------|----------------|--------|
| 1 | BC-1.05.035 PC-2: "returns `codes::INVALID_ARGUMENT` (-4) before any canonicalize attempt" | BC-1.05.035.md §Postconditions PC-2 | Direct read of BC body | PASS |
| 2 | exec_subprocess.rs:51-52 catch-all `Err(_) => return Err(codes::INVALID_ARGUMENT)` | exec_subprocess.rs | Source-of-truth Rust | PASS |
| 3 | memory.rs:53 `String::from_utf8(bytes).map_err(\|_\| HostCallError::InvalidUtf8)?` | host/memory.rs:53 | Source-of-truth Rust | PASS |
| 4 | BC-1.05.036 PC-3: "implementer adds a `let started = Instant::now();` capture immediately before `command.spawn()` at exec_subprocess.rs:252" | BC-1.05.036.md §Postconditions PC-3 | Direct read of BC body | PASS |
| 5 | exec_subprocess.rs:270 `let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64)` | exec_subprocess.rs:270 | Source-of-truth Rust | PASS |
| 6 | exec_subprocess.rs:297 `std::thread::sleep(Duration::from_millis(5))` | exec_subprocess.rs:297 | Source-of-truth Rust | PASS |
| 7 | BC-1.05.036 EC-013B: "stdout_buf.len() > 0 as usize is true if subprocess produces ANY output (even 1 byte)" | BC-1.05.036.md §Edge Cases EC-013B | Direct read of BC body | PASS |
| 8 | exec_subprocess.rs:278 `if stdout_buf.len() > max_output_bytes as usize` strict `>` check | exec_subprocess.rs:278 | Source-of-truth Rust | PASS |
| 9 | BC-1.05.036 TV-9 outcome: "Returns `Err(INTERNAL_ERROR -99)`; NO event emitted in v1 per Postcondition 5" | BC-1.05.036.md §Canonical Test Vectors TV-9 | Direct read of BC body | PASS |
| 10 | memory.rs:33 `start.checked_add(len as usize)` MemoryOverflow; memory.rs:35-40 OutOfBounds span check | host/memory.rs:33/35-40 | Source-of-truth Rust | PASS |

**TD-VSDD-093 PASS — all 10 rows verified against source-of-truth.**

---

## 5-Axis Sibling Sweep (TD-VSDD-089)

1. **Postcondition ↔ Edge Case parity:** PC-2 (BC-1.05.035) and PC-3 (BC-1.05.036) are normative postconditions without corresponding ECs that expand on them — the structural asymmetry is by design (PCs specify outcomes; ECs provide boundary conditions). The CTV coverage gaps identified here are at the TV witness level, not at the PC/EC structural level. No new ECs required; TV additions close the gaps.

2. **Cross-BC reference accuracy:** BC-1.05.035 PC-2 and BC-1.05.036 PC-3 do not cross-reference each other. No cross-BC anchor drift from this audit.

3. **Numeric enumeration:** BC-1.05.035 at v1.52 has TV-1..TV-9 (9 rows). BC-1.05.036 at v1.52 has TV-1..TV-19 (19 rows). After D-305: BC-1.05.035 will have TV-1..TV-10 (10 rows); BC-1.05.036 will have TV-1..TV-25 (25 rows). Numeric coherence confirmed.

4. **Parenthetical lists:** EC-007 body parenthetical "(spawn at :252, stdin take/write at :258/:259-262, stdout/stderr take at :267-268, try_wait at :299)" verified against exec_subprocess.rs source-of-truth. LOW-P60-001 TV-9 NOTE matches EC-007 enumeration. No drift.

5. **Codification artifact sibling integrity:** No new NORMATIVE rule born in this pass (novel angle, but existing TD-VSDD-085 + TD-VSDD-092 frameworks govern the findings; no S-7.02 threshold met for new rule). No lessons.md update required. STATE.md + STORY-INDEX updated in Phase 2 state-manager seal.

---

## Novelty Assessment

This pass uses the CTV coverage matrix as the primary lens — building a complete bidirectional PC/EC ↔ TV mapping and auditing for coverage gaps in both directions. This angle is **novel**: prior passes 1..59 used the following lenses (partial list):
- Passes 1..12: diff-only, versioning, citation-grounding, frontmatter coherence
- Passes 13..30: mechanism-correctness, cross-doc consistency, terminology drift
- Passes 31..40: inverse-traceability, failure-mode coverage matrix (TD-VSDD-085 origin), SOUL #4 silent-failure sweep
- Passes 41..50: type-signature-verification, partial-fix-regression, cross-BC sibling-symmetry, SOUL #4 systemic
- Passes 51..59: signal-flow/data-flow, TV-derivation, whole-document re-read, NORMATIVE rule cross-application, markdown-table well-formedness, frontmatter schema, glossary/terminology sweep, capability anchoring

The CTV coverage-matrix angle (build full PC/EC→TV bidirectional mapping; flag missing witnesses in both directions; apply to BOTH BCs simultaneously) was untouched as a primary angle in 59 prior passes. The 4 MEDIUM findings + 1 LOW finding represent genuine pre-burst coverage gaps that prior single-mechanism or partial-fix-regression-frame audits did not surface.

**Sibling-class to HIGH-P39-003:** HIGH-P39-003 (D-282 v1.36 fix burst) found 3 TVs missing for signal-death/emit-IO/Mutex-poison per TD-VSDD-085 failure-mode coverage audit. The current pass is a structural continuation of that audit applied as a comprehensive bidirectional mapping — not a failure-mode-specific sweep but a full PC/EC exhaustion test.

---

## Final Verdict

**SUBSTANTIVE — 0 HIGH / 4 MEDIUM / 1 LOW + 2 non-blocking observations**

ADR-013 clock: 0_of_3 → 0_of_3 (HOLD; SUBSTANTIVE verdict — 4 MEDIUM + 1 LOW found; clock cannot advance with MEDIUM findings open).

D-305 Phase 1 (PO) + Phase 2 (state-manager) required to close all findings before pass-61.
