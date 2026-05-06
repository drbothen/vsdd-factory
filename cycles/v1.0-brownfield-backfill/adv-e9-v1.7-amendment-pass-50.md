# Adversarial Review — Pass 50 (E-9 v1.44 SOUL #4 silent-failure systemic sweep)

## 1. Angle (NEW per TD-VSDD-057)

**Append-only POLICY 1 byte-level audit + SOUL.md #4 silent-failure systemic sweep.** Two-part: (A) verify no prior H3 changelog block has been silently modified; (B) for every specified mechanism that could fail silently, audit whether the BC pair acknowledges the silent-failure case. Concrete: walk source for `let _ =` and `map_err(|_|)` discard patterns and check BC pair coverage.

## 2. Findings

### HIGH-P50-001 — `stdout/stderr.read_to_end` silent IO error swallow not acknowledged

**Severity:** HIGH **Confidence:** HIGH
**Source:** `host/exec_subprocess.rs:276-277` `let _ = stdout.read_to_end(...); let _ = stderr.read_to_end(...);`
**Description:** When read_to_end errors mid-read, partial buffer is used, success envelope is emitted with under-counted stdout_bytes/stderr_bytes and outcome='success'. truncated=false because partial < max_output_bytes. Plugin caller cannot distinguish complete-read from IO-truncated-read.

### HIGH-P50-002 — `child.kill()` / `child.wait()` silent error swallow on TIMEOUT and stdin-fail paths

**Severity:** HIGH **Confidence:** HIGH
**Source:** `host/exec_subprocess.rs:260-261` (stdin-fail) and `:293-294` (TIMEOUT) — `let _ = child.kill(); let _ = child.wait();`
**Description:** On TIMEOUT path, if kill() errors AND wait() blocks (NFS D-state), dispatcher hangs with no secondary deadline. Same hazard on stdin-fail path. TIMEOUT enforcement covers deadline check only; cleanup-phase has no deadline.

### MED-P50-001 — `command.spawn().map_err(|_|)` io::Error reason discarded

**Severity:** MEDIUM **Confidence:** HIGH
**Source:** `host/exec_subprocess.rs:252` — `command.spawn().map_err(|_| codes::INTERNAL_ERROR)?`
**Description:** ENOENT/EACCES/ETXTBSY/ENOMEM/EAGAIN all collapse to undifferentiated INTERNAL_ERROR (-99) with no diagnostic.

### LOW-P50-001 — `emit_denial` denial-path best-effort symmetry not acknowledged

**Severity:** LOW
**Description:** All 5 denial paths route through emit_denial → ctx.emit_internal which has same best-effort eprintln-fallback as success-path event in BC-036 EC-010 + Postcondition 6. BC-035 doesn't acknowledge symmetrically.

## 3. Verdict

**SUBSTANTIVE.** 2H/1M/1L. ADR-013 clock RESETS 2_of_3 → 0_of_3.

POLICY 1 byte-level audit Part A: clean. SOUL #4 sweep Part B found 4 unacknowledged `let _ =` discards in `execute_bounded` that prior 49 angles missed.

## 4. Process-Gap Tagging

[process-gap]: 49-pass adversary inventory missed exhaustive `let _ =` and `map_err(|_|)` source-walk. Recommend: any BC governing a function with `let _ =` on Result MUST acknowledge silent-discard in EC row. Codify as TD-VSDD-092 NORMATIVE.

## 5. TD-VSDD Lesson Awareness

Reviewed 057-091. Pass-50 angle NEW.

## 6. Convergence Assessment

v1.44 syntax-converged but semantic-incomplete vs source-of-truth on silent-failure paths. v1.45 needed to acknowledge 4 silent-discards. After v1.45 + 3 NITPICK passes: genuinely convergence-ready.
