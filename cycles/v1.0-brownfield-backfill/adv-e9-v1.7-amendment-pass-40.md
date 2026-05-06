# Adversarial Review — Pass 40 (E-9 v1.7 amendment surface, epic v1.36 sealed at 5e432b2)

## 1. Angle (NEW per TD-VSDD-057)

**Contract-completeness audit.** Treats the BC pair (BC-1.05.035 + BC-1.05.036) as a black-box specification of the `vsdd::exec_subprocess` host-call contract. Methodology: enumerate every distinguishable input state crossed with every capability-state, then verify the BC pair specifies a (return code, event class, side-effect) outcome for each cell. Identify cells where the BC pair is silent OR where the BC pair's described mechanism contradicts source code at the cited line.

**Differentiation:** Prior 39 angles covered narrative coherence, text-mechanism mismatch, term-family sweeps, TV-witness presence, diff-only-of-latest-burst, architectural-anchor sweeps. None enumerated the dispatcher contract's full input × capability surface and asked "for each cell, what does the BC pair promise?" This angle is structural rather than narrative — derives the contract's domain/codomain from `register()` (host/exec_subprocess.rs:33-95) and the error code set (host/mod.rs:178-185), then maps each cell against the BC pair.

## 2. Findings

### HIGH-P40-001 — internal_log.write return-type misdescription contradicts source code (TD-VSDD-081 4th-gen violation)

**Severity:** HIGH **Confidence:** HIGH
**Location:** BC-1.05.036 P6/EC-010/TV row 11; source-of-truth: `internal_log.rs:228` declares `pub fn write(&self, event: &InternalEvent)` — return type `()`, NOT `Result`. Lines 229-237 do `if let Err(e) = self.write_inner(event) { eprintln!(...) }` — IO failure path is NOT silent (eprintln to stderr) AND the event IS pushed to events queue.

The mechanism description was added in D-281 to close pass-38 MED-P38-002 and revised in D-282 to add TV row 11. Both fix bursts cited host/mod.rs:111 but neither verified `log.write`'s return type.

**Recommendation:** Reword P6/EC-010/TV row 11 to describe actual mechanism (`write` returns `()`; eprintln on Err; events queue push proceeds). Distinguish "internal_log write" (current, dispatcher-internal-*.jsonl) from "ADR-015 FileSink" (future, events-*.jsonl).

### HIGH-P40-002 — TV row 11 unreachable in test contexts; internal_log: None branch unspecified

**Severity:** HIGH **Confidence:** HIGH
**Location:** host/mod.rs:96 (HostContext::new sets internal_log: None); BC-036 P4, TV row 11

Production wiring sets `internal_log: Some(...)`; test helpers set `None`. In the None branch at host/mod.rs:110, `log.write` is NEVER called — TV row 11's premise unreachable in standard test fixtures. False-positive test risk.

**Recommendation:** Add P4 bifurcation note + TV row covering `internal_log: None` branch.

### HIGH-P40-003 — Two distinct OUTPUT_TOO_LARGE paths conflated

**Severity:** HIGH **Confidence:** HIGH
**Location:** exec_subprocess.rs:86-88 (envelope.len() > result_buf_cap) vs :278-283 (subprocess output > max_output_bytes); BC-036 EC-005 only describes path B.

Path A (buffer-too-small) is recoverable via larger guest buffer. Path B (policy violation) is not. BC pair gives no guidance.

**Recommendation:** Split EC-005 into 5A (subprocess-output-overflow) + 5B (result_buf_cap-overflow). Document 12-byte envelope overhead from `encode_envelope` at exec_subprocess.rs:101-109.

### HIGH-P40-004 — cwd_allow unenforcement undocumented (security gap)

**Severity:** HIGH **Confidence:** HIGH
**Location:** registry.rs:83 (`pub cwd_allow: Vec<String>` declared); exec_subprocess.rs:248-250 (uses `ctx.cwd` directly, NO consultation of `caps.cwd_allow`); gap-analysis Section 1 PARTIAL row.

`cwd_allow` is a no-op field. Plugin can declare `cwd_allow = ["/sandbox"]` and dispatcher runs in any caller-controlled `ctx.cwd` silently. Operators reading the BC pair will assume enforcement.

**Recommendation:** Add EC explicitly stating cwd_allow no-op semantics; file OQ-W16-007.

### HIGH-P40-005 — Host-side panic semantics unspecified for new canonicalize step

**Severity:** HIGH (security observability) **Confidence:** MEDIUM-HIGH
**Location:** host/mod.rs:72 (planned `internal.host_function_panic` event class); exec_subprocess.rs:230 (Command::new — no catch_unwind); BC-035 P1 (mandates new canonicalize)

Adding canonicalize to the host call expands panic surface without specifying behavior. A panic mid-allow-check leaves dispatcher in indeterminate state.

**Recommendation:** Add panic-handling spec; file OQ-W16-008.

### MED-P40-001 — args non-UTF-8 silent lossy conversion not specified

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** exec_subprocess.rs:127 (`String::from_utf8_lossy`); BC-035 P2 specifies cmd strict UTF-8 enforcement

Asymmetric: cmd strict-rejects non-UTF-8; args lossy-substitutes (U+FFFD). Plugins receive subprocess with mangled args.

**Recommendation:** Add EC documenting the asymmetry.

### MED-P40-002 — timeout_ms=0 and max_output_bytes=0 boundary semantics unspecified

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** exec_subprocess.rs:270, :278-281

`timeout_ms=0` causes immediate TIMEOUT. `max_output_bytes=0` causes any-output-fails. Both surprising.

**Recommendation:** Add ECs for both edge cases.

### MED-P40-003 — env_allow requested name absent from env_view: silent skip

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** exec_subprocess.rs:243-247

Plugin can't distinguish "name set to empty" from "name absent from dispatcher env."

**Recommendation:** Add EC documenting silent-skip best-effort env-forwarding.

### MED-P40-004 — binary_allow path-separator pathological-config not specified

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** exec_subprocess.rs:191 (basename-or-full match); BC-035 EC-009 covers directory-cmd analog only

`binary_allow = ["passwd"]` allows `../etc/passwd` via canonicalize+basename match. Operator audit responsibility unstated.

**Recommendation:** Add sibling EC mirroring EC-009 directory case for binary-name family.

### MED-P40-005 — internal_log: None branch in P4 dependency chain not specified

**Severity:** MEDIUM
**Location:** BC-036 P4

Closed by HIGH-P40-002 P4 bifurcation fix.

### LOW-P40-001 — cmd="" empty-string case not in EC list

**Severity:** LOW
**Location:** BC-035 Edge Cases

Distinguishable input from missing-binary. Same outcome but distinct row warranted.

### LOW-P40-002 — encode_envelope overhead constant not surfaced in BC

**Severity:** LOW
**Location:** BC-036 P2; exec_subprocess.rs:101-109 (envelope = 4+4+4 = 12 bytes overhead)

Closed by HIGH-P40-003 EC-005B fix.

## 3. Verdict

**SUBSTANTIVE.** 5 HIGH + 5 MED + 2 LOW. ADR-013 clock RESETS to 0_of_3.

## 4. Process-Gap Tagging

- **HIGH-P40-001** is a 4th-generation TD-VSDD-081 violation involving host/mod.rs:111. The pattern "BC cites line N to describe behavior of function F; citation correct as line-anchor but incorrect as mechanism-description" has occurred 3+ times in this BC pair. **[process-gap]:** extend TD-VSDD-081 mechanization (validate-bc-terminology-family.sh hook) to also cross-check that any cited function call site's return type matches the BC's prose description.
- **HIGH-P40-004** (cwd_allow unenforcement) is documented in gap-analysis but absent from BC pair — documentation-propagation gap. **[process-gap]:** extend story-writer / state-manager checklist to verify gap-analysis Section 1 PARTIAL/NO rows are explicitly documented as known-limitations in their corresponding BCs.

## 5. Source-of-Truth Verification Log

12 files read; 15 source-claim verifications. Critical discoveries:
- `internal_log.rs:228` — `pub fn write` returns `()`, not Result (HIGH-P40-001 anchor)
- `internal_log.rs:229-237` — eprintln-fallback path; NOT silent (HIGH-P40-001 anchor)
- `host/mod.rs:96` — HostContext::new sets `internal_log: None` (HIGH-P40-002 anchor)
- `registry.rs:83` + `exec_subprocess.rs:248-250` — cwd_allow declared but unenforced (HIGH-P40-004 anchor)
- `host/mod.rs:72` comment — `internal.host_function_panic` planned but unimplemented (HIGH-P40-005 anchor)
- `exec_subprocess.rs:101-109` — encode_envelope overhead = 12 bytes (LOW-P40-002 anchor)

## 6. TD-VSDD Lesson Awareness (057-087 reviewed)

Confirmed reviewed. Pass-40 angle (contract-completeness) NEW; not in 39-angle inventory. HIGH-P40-001 demonstrates 4th-generation TD-VSDD-081 recurrence — confirms mechanization mandate via TD-VSDD-080 hook is overdue.
