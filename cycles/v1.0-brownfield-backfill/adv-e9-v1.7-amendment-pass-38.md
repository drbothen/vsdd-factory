# Adversarial Review — Pass 38 (E-9 v1.7 amendment surface, epic v1.34)

## 1. Angle

**Failure-mode coverage matrix audit.** Built a 2D matrix of (failure-input × specification-coverage) for the BC pair (BC-1.05.035, BC-1.05.036). For each enumerated distinguishable failure-mode, checked coverage via (a) Preconditions, (b) Postconditions, (c) Edge Cases, (d) Canonical Test Vectors, (e) source-of-truth alignment. Contrasts with 37 prior angles by auditing **outward coverage** (does the BC pair specify behavior for every distinguishable failure mode the production code can produce?), rather than internal coherence, citation validity, structural symmetry, or convention discipline.

Stress-tested v1.34 additions: `binary_canonicalize_failed` reason; canonical-path propagation EC-007; routing-INTERIM Postcondition 4; failure-modes added by `Path::canonicalize()` itself.

## 2. Failure-Mode Coverage Matrix (summary)

### BC-1.05.035

- cmd empty: PARTIAL (works by accident via step 2)
- cmd NUL byte: COVERED in spec; **TV witness MISSING** (HIGH-P38-001)
- cmd traversal: PARTIAL (EC-001 self-contradicts ladder — HIGH-P38-002)
- cmd missing on disk: COVERED
- cmd broken symlink: PARTIAL (no TV witness)
- cmd symlink loop ELOOP: MISSING (LOW-P38-001)
- cmd is directory: MISSING
- cmd PATH_MAX exceeded: MISSING
- TOCTOU canonical-vs-raw at line 230: PARTIAL (EC-007 has no TV witness — MED-P38-001)
- cmd is non-UTF-8: PARTIAL

### BC-1.05.036

- exit 0 / non-zero / timeout / OUTPUT_TOO_LARGE / spawn fail / stdin/stdout/stderr fails / try_wait err / capability check fail / outcome enum: COVERED
- subprocess killed by signal: MISSING (HIGH-P38-003 — exit_code=-1 collision)
- emit IO failure (FileSink full): MISSING (MED-P38-002)
- events Mutex poison: MISSING (MED-P38-003 — write silent, read panics)
- stdout_bytes/stderr_bytes timing under truncation: PARTIAL (MED-P38-004)

## 3. Findings

### HIGH-P38-001 — Missing Canonical Test Vector witness for `binary_canonicalize_failed`

**Severity:** HIGH  **Confidence:** HIGH
**Location:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md:91-99`

Pass-37 closed HIGH-P37-001 by adding `binary_canonicalize_failed` as the 5th `emit_denial` reason. POLICY 12 requires TV witnesses for emitter contracts. Current Test Vectors row 4 (non-existent binary) does NOT explicitly assert `binary_canonicalize_failed` emission. Asymmetric with row 3 which names `binary_not_on_allow_list`. This is the structurally novel addition of the burst with no testable witness.

**Recommendation:** Add explicit TV row asserting `emit_denial(ctx, cmd, "binary_canonicalize_failed", details)` for non-existent binary AND for NUL-byte cmd.

### HIGH-P38-002 — EC-001 Expected Behavior self-contradicts Precedence Ladder

**Severity:** HIGH  **Confidence:** HIGH
**Location:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md:83`

EC-001 says "via Precedence Ladder step (2) (`canonicalize()` succeeds for `../etc/passwd` if `/etc/passwd` exists OR fails with EINVAL if not)". Two defects: (1) when canonicalize SUCCEEDS, it's not step (2) firing — step (2) is the canonicalize-fails branch; the success branch falls through to step (3) allow-list miss. (2) "fails with EINVAL if not [exists]" is incorrect — `Path::canonicalize` on a missing path returns ENOENT (NotFound), not EINVAL. EINVAL is for NUL bytes (CString conversion).

**Recommendation:** Rewrite EC-001 to disambiguate the canonicalize-succeeds (→step 3) and canonicalize-fails (→step 2 ENOENT) branches, and remove the "EINVAL if not" misnomer.

### HIGH-P38-003 — Signal-death conflated with exit_code=-1

**Severity:** HIGH  **Confidence:** HIGH
**Location:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md:48-49`; `crates/factory-dispatcher/src/host/exec_subprocess.rs:286`

`status.code().unwrap_or(-1)` substitutes -1 when subprocess was killed by signal (Unix). The BC has no Edge Case row, no TV witness, no Postcondition for signal-death. SIGSEGV / SIGKILL / kill -9 emit `exit_code=-1, outcome='failure'` indistinguishable from a literal `_exit(-1)`. Major observability defect: segfault vs OOM-kill vs benign exit collapsed into one signal.

**Recommendation:** Add EC-009 specifying signal-death v1 semantics (substitute -1; note as known limitation; track as OQ-W16-NNN if signal disambiguation needed).

### MED-P38-001 — EC-007 (canonical-path propagation) has no TV witness

**Severity:** MEDIUM  **Confidence:** HIGH
**Location:** BC-1.05.035.md:89 (EC-007); 92-99 (TVs)

EC-007 added in v1.34 (D-280) is the load-bearing security postcondition. POLICY 12 + TD-VSDD-076/079 require TV witness. "covered by VP for this BC" deferral suspect because §VP Anchors says "(TBD)". TD-VSDD-084 instance.

**Recommendation:** Add TV row witnessing EC-007 negative case (raw cmd in execute_bounded → TOCTOU defect).

### MED-P38-002 — Event emit IO failure unspecified; silent-failure risk

**Severity:** MEDIUM  **Confidence:** HIGH
**Location:** BC-1.05.036.md:140-144 (Purity Classification); host/mod.rs:109-116

`emit_internal` silently drops `log.write` failures. Postcondition 1 says "exactly one event emitted via `ctx.emit_internal`" but if emit silently swallows IO failure, the event was NOT emitted to consumers. SOUL.md #4 silent-failure pattern.

**Recommendation:** Add Postcondition 6 or amend P1 to specify v1 best-effort silent-drop semantics + Edge Case witness.

### MED-P38-003 — Mutex poison asymmetry: emit_internal silent on poison; drain_events panics

**Severity:** MEDIUM  **Confidence:** HIGH
**Location:** BC-1.05.036.md (entire); host/mod.rs:113 (write `if let Ok`); host/mod.rs:102 (`drain_events` `.expect`)

Reader/writer asymmetry on the same Mutex: writer silently drops, reader panics dispatcher. BC Purity Classification line 143 claims "Thread safety: YES — follows same pattern" but the pattern HAS this asymmetry.

**Recommendation:** Add EC for Mutex poison semantics; harmonize or document as known limitation.

### MED-P38-004 — `stdout_bytes`/`stderr_bytes` measurement timing under truncation ambiguous

**Severity:** MEDIUM  **Confidence:** MEDIUM
**Location:** BC-1.05.036.md:88 (EC-006); :49 (P2 8-field payload)

P2 lists `stdout_bytes: u64` but doesn't specify whether it's pre-truncate or post-truncate. In v1 the question is moot (truncation aborts). At future ABI break (when truncated:bool is real per EC-006 reservation), the two diverge. POLICY 4 requires unambiguous semantics.

**Recommendation:** Specify `stdout_bytes` = bytes returned in envelope (post-truncate, capped at max_output_bytes).

### LOW-P38-001 — Symlink loop, canonicalize-on-directory, PATH_MAX not enumerated

**Severity:** LOW  **Confidence:** HIGH
**Location:** BC-1.05.035.md:79-90

EC-006 lumps "missing binary, broken symlink, permission denied" together. Three additional canonicalize failure modes not enumerated: (1) ELOOP symlink loop; (2) canonicalize-on-directory (succeeds — directory passes if basename in allow-list, then `Command::new` fails to spawn → INTERNAL_ERROR, masking a config defect); (3) ENAMETOOLONG.

**Recommendation:** Add EC-008 (symlink loop), EC-009 (cmd is a directory), optionally EC-010 (ENAMETOOLONG).

### LOW-P38-002 — args_len, stdin_len, ARG_MAX bounds not specified

**Severity:** LOW  **Confidence:** MEDIUM

Neither BC specifies bounds for `args_len`, `stdin_len`, total argv+envp ≤ ARG_MAX. Collapse to existing INVALID_ARGUMENT or INTERNAL_ERROR via implementation accident.

**Recommendation:** Add Precondition note about implicit bounds via memory.rs and command.spawn().

### LOW-P38-003 — macOS HFS+ NFD/NFC normalization on canonicalize not addressed

**Severity:** LOW  **Confidence:** LOW

`Path::canonicalize` on macOS HFS+ may return NFD-normalized form not byte-equal to NFC allow-list. For ASCII-only allow-list (typical), non-issue. BC claims general validity.

**Recommendation:** Add note or OQ for cross-platform UTF-8 normalization.

## 4. Verdict

**SUBSTANTIVE.** 3 HIGH + 4 MEDIUM + 3 LOW. ADR-013 clock RESETS to 0_of_3.

## 5. Process-Gap Tagging

No `[process-gap]` tags warranted (no class with recurrence ≥3 not already tracked).

Observation: HIGH-P38-001 + MED-P38-001 are 4th-and-5th observed "fix-burst-introduces-new-mechanism-but-omits-TV-witness". S-7.02 threshold met. TD-VSDD-080 already proposes hook mechanization — adversary recommends extending `validate-bc-terminology-family.sh` to also validate TV-witness-presence for newly-introduced reason strings.

## 6. Source-of-Truth Verification Log

| File | Lines | Verified |
|---|---|---|
| BC-1.05.035.md | 1-143 (full) | Postconditions, Edge Cases, TVs, Precedence Ladder |
| BC-1.05.036.md | 1-149 (full) | Postconditions, EC-007, TVs, Purity Classification |
| exec_subprocess.rs | 1-463 (full) | line 152 binary_allowed, 173 execute_bounded, 230 Command::new(cmd), 252 spawn, 258-262 stdin, 267-268 stdout/stderr, 285-289 encode_envelope+`status.code().unwrap_or(-1)`, 295 TIMEOUT, 299 try_wait, 304-309 emit_denial; 4 reasons at 148/155/162/169 |
| host/mod.rs | 1-237 (full) | emit_internal at 109-116 silent on `log.write` AND on Mutex poison; drain_events `.expect` at 102; INTERNAL_ERROR=-99 at 184 |
| memory.rs | 1-100 | read_wasm_string at 47-54 only rejects non-UTF-8 |
| read_file.rs | 115-175 | path_allowed at 122-148 uses canonicalize+starts_with — sibling pattern correct at conceptual level (canonicalize-then-allow-check), differs at matching-strategy (prefix vs basename) |

## 7. TD-VSDD Lesson Awareness

Reviewed TD-VSDD-057 through TD-VSDD-084. Pass-38 angle (failure-mode coverage matrix) NEW. No closed-out finding recapitulated. TD-VSDD-084 PROVISIONAL: MED-P38-001 is 2nd recurrence of the pattern; approaching but not yet at S-7.02 3+ threshold.

LOW-P38-002 and LOW-P38-003 may be deferred per S-7.03 SHIP-AS-IS pattern.
