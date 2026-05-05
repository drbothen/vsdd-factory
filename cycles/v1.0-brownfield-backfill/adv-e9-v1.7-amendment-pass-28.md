---
document_type: adversarial-review
level: ops
cycle: v1.0-brownfield-backfill
pass: 28
target: E-9 v1.26 (BC-1.05.036 sibling-sweep scope; diff-only + sibling-coverage angle NEW per TD-VSDD-057)
verdict: SUBSTANTIVE
findings: 2H/3M/1L
producer: adversary
timestamp: 2026-05-05T00:00:00Z
adr_013_clock_before: 0_of_3
adr_013_clock_after: 0_of_3
adr_013_action: RESET (2H findings)
---

# Adversarial Review — E-9 v1.26 Amendment Pass-28

**Target:** BC-1.05.036 (diff-only v1.26 changes + full sibling-coverage sweep)
**Angle:** Diff-only line-by-line of v1.26 burst + sibling-coverage sweep (NEW per TD-VSDD-057)
**Verdict:** SUBSTANTIVE — 2 HIGH, 3 MED, 1 LOW
**ADR-013 clock:** 0_of_3 → RESET 0_of_3 (2H findings)

---

## HIGH Findings

### H-P28-001 — BC-1.05.036:38 §Description retains "normal sink chain"

**Location:** BC-1.05.036 line 38 (`## Description`, second paragraph)

**Finding:** Line 38 reads:
> "…the dispatcher MUST emit a `host.exec_subprocess.completed` event through **the normal sink chain**."

The phrase "normal sink chain" is stale ADR-005-era terminology. The v1.26 burst (D-270) correctly scrubbed Postcondition 4 (line 51) where the same multi-sink language appeared — but did NOT sweep the §Description section which carries parallel "normal sink chain" wording. ADR-015 D-15.1 removes the multi-sink stanza model and retires Router/SinkRegistry entirely; the correct phrasing names `ctx.emit_internal` and the single-stream FileSink per ADR-015 D-15.1.

**Impact:** §Description is the first section read by a story-writer or implementer. A stale sink-chain reference here directly conflicts with the ADR-015 awareness block that was added at v1.21 (D-263) just below it — creating an internal contradiction at lines 32-38.

**Required fix:** Replace "through the normal sink chain" with: "through `ctx.emit_internal` to the single-stream `FileSink` per ADR-015 D-15.1 (multi-sink stanza model removed; Router/SinkRegistry retired)"

**Process-gap:** Third recurrence of TD-VSDD-076 self-violation pattern. v1.26 burst scrubbed line 51 only; §Description (line 38) is a semantic sibling that TD-VSDD-076 explicitly requires be swept. Recurrence threshold (3+) met — warrants TD-VSDD-079 codification.

---

### H-P28-002 — BC-1.05.036:135 §Purity Classification: "sink chain" + fabricated "non-blocking try_send"

**Location:** BC-1.05.036 line 135 (`#### Purity Classification` table, `I/O operations` row)

**Finding:** Line 135 reads:
> `| **I/O operations** | YES — emits event via ctx.emit_internal sink chain (non-blocking try_send) |`

TWO defects in this single cell:

1. **"sink chain" stale per ADR-015:** Same retired multi-sink terminology as H-P28-001. The v1.26 burst did not sweep §Purity Classification.

2. **"non-blocking try_send" is FABRICATED:** `crates/factory-dispatcher/src/host/mod.rs:109-116` shows `emit_internal` implementation uses **synchronous `Mutex::lock` + `Vec::push`** (not any channel send):
   ```rust
   pub(crate) fn emit_internal(&self, event: InternalEvent) {
       if let Some(log) = self.internal_log.as_ref() {
           log.write(&event);
       }
       if let Ok(mut events) = self.events.lock() {
           events.push(event);
       }
   }
   ```
   There is no `try_send`, no channel, no async operation. The claim "non-blocking try_send" is entirely invented — it describes a channel-based async sink pattern that does not exist in this codebase.

**Impact:** The fabricated "non-blocking try_send" claim directly misleads implementers about the threading model and blocking behavior of emit_internal. A story-writer reading §Purity Classification would incorrectly conclude emit_internal is a non-blocking async channel send and make wrong assumptions about thread-safety, error handling, and performance characteristics.

**Required fix:** Replace cell content with: "YES — emits event via `ctx.emit_internal`: synchronous `Mutex::lock` + `Vec::push` to events queue per `host/mod.rs:105-116`, then host writes to single-stream `FileSink` per ADR-015 D-15.1 (no channel send; not async)"

---

## MED Findings

### M-P28-001 — §Edge Cases lacks EC-007 row for INTERNAL_ERROR (-99) no-event path

**Location:** BC-1.05.036 §Edge Cases table (lines 81-88)

**Finding:** The v1.26 burst added INTERNAL_ERROR (-99) to Postcondition 5 as a third no-event error path (alongside TIMEOUT -2 and OUTPUT_TOO_LARGE -3). The §Edge Cases table has EC-004 for TIMEOUT and EC-005 for OUTPUT_TOO_LARGE, but has NO EC-007 row for INTERNAL_ERROR. Postcondition 5 and §Edge Cases are semantic siblings (per TD-VSDD-076); the v1.26 burst correctly updated the postcondition but did not sweep §Edge Cases.

**Impact:** An implementer reading §Edge Cases would see EC-004 (TIMEOUT) and EC-005 (OUTPUT_TOO_LARGE) with "NO event emitted in v1" qualifiers, but miss INTERNAL_ERROR entirely. The contract is incomplete for an error path that actually exists in source code (spawn failure, stdin take/write, stdout/stderr take, try_wait error — all per exec_subprocess.rs).

**Required fix:** Add EC-007 row to §Edge Cases table after EC-006 with content:
> Subprocess spawn fails / pipe take/write fails / try_wait error → Returns `Err(INTERNAL_ERROR -99)`; **NO event emitted in v1** (per Postcondition 5; spawn at exec_subprocess.rs:252, stdin take/write at :258/:262, stdout/stderr take at :267-268, try_wait at :299); `host.exec_subprocess.completed` NOT emitted

---

### M-P28-002 — §Canonical Test Vectors lacks INTERNAL_ERROR row

**Location:** BC-1.05.036 §Canonical Test Vectors table (lines 92-99)

**Finding:** Parallel defect to M-P28-001. The §Canonical Test Vectors table has rows for TIMEOUT and OUTPUT_TOO_LARGE error paths but no row for INTERNAL_ERROR (-99). Postcondition 5 and §Canonical Test Vectors are semantic siblings. The v1.26 burst updated Postcondition 5 but did not sweep §Canonical Test Vectors.

**Impact:** Test authors and story implementers reading §Canonical Test Vectors would have an incomplete picture of the required test matrix — the INTERNAL_ERROR error path (spawn failure, pipe failure, try_wait error) has no canonical test vector.

**Required fix:** Append INTERNAL_ERROR row to §Canonical Test Vectors after the OUTPUT_TOO_LARGE row with content:
> Subprocess spawn fails (or pipe take/write fails or try_wait error) → Returns `Err(INTERNAL_ERROR -99)`; NO event emitted in v1 per Postcondition 5; `host.exec_subprocess.completed` NOT emitted → category: error

---

### M-P28-003 — EC-005 + Test Vector line 98 missing "NO event emitted in v1" qualifier

**Location:** BC-1.05.036 line 87 (EC-005) and line 98 (Test Vector, OUTPUT_TOO_LARGE row)

**Finding:** EC-004 (line 86) uses the fully-qualified form:
> "Returns `Err(TIMEOUT -2)`; **NO event emitted in v1** (per Postcondition 5; future error-path emit is out-of-scope); `host.exec_subprocess.completed` NOT emitted"

EC-005 (line 87) uses a truncated form that omits the "NO event emitted in v1" qualifier:
> "`OUTPUT_TOO_LARGE` path; `host.exec_subprocess.completed` NOT emitted"

Similarly, the Test Vector OUTPUT_TOO_LARGE row (line 98) uses:
> "`OutputTooLarge` path; `host.exec_subprocess.completed` NOT emitted"

Both should be aligned to the EC-004 sibling form which explicitly states the Postcondition 5 authority and out-of-scope qualifier — this is load-bearing wording that prevents a story-writer from inferring OUTPUT_TOO_LARGE should emit an event in v1.

**Required fix:** Align EC-005 and Test Vector OUTPUT_TOO_LARGE row to EC-004 form with explicit "NO event emitted in v1 (per Postcondition 5; future error-path emit is out-of-scope)" qualifier.

---

## LOW Findings

### L-P28-001 — BC-1.05.036:51 "retired" verb for ADR-015 line 154 should be "removed"

**Location:** BC-1.05.036 line 51 (Postcondition 4, v1.26 fix text)

**Finding:** Postcondition 4 (line 51, applied by v1.26 burst) reads:
> "…(the multi-sink stanza model and `Router`/`SinkRegistry` are retired per ADR-015 lines 130, 154…)"

ADR-015 uses distinct verbs with distinct semantic meaning:
- **Line 130:** "`sink-otel-grpc` crate AND the `Router`, `SinkRegistry` types… are **retired**" — verb: "retired" (physically deleted from repository at Wave 5)
- **Line 154:** "The multi-sink stanza model **is removed**. Operators who need remote export configure the OTel Collector…" — verb: "removed" (configuration model eliminated)

Using "retired" for BOTH targets conflates ADR-015's two-phase lifecycle taxonomy (deprecated-then-retired for crates vs removed for the stanza model). ADR-015 lines 132-143 define "retired" specifically as "crates physically deleted from the repository" while "removed" applies to the stanza model.

**Required fix (LOW — verb-precision only):** Change Postcondition 4 to preserve both verbs per ADR-015: "(multi-sink stanza model removed per ADR-015 line 154; Router/SinkRegistry retired per ADR-015 line 130)"

---

## Process Gap

### PG-P28-001 — Third TD-VSDD-076 self-violation recurrence → codify TD-VSDD-079

**Pattern:** Three TD-VSDD-076 self-violations recorded:
- pass-24 H-P24-001 (v1.23 burst codified TD-VSDD-076 but its OWN BC-1.05.036 EC-006 truncated:bool annotation had template inconsistency)
- pass-25 M-P25-001 (v1.24 fix aligned EC-003 to Postcondition 5 after being caught)
- pass-28 H-P28-001/002 + M-P28-001/002/003 (v1.26 silence-audit burst scrubbed Postcondition 4 line 51 only; §Description line 38, §Purity Classification line 135, §Edge Cases, §Canonical Test Vectors all retained stale or missing wording)

S-7.02 recurrence threshold (3+) met. TD-VSDD-076 application is consistently incomplete because each fix burst greps for the EXACT phrase the prior adversary cited rather than the BROADER terminology family.

**Codification required (TD-VSDD-079):** Extend TD-VSDD-076 with explicit grep-checklist for amendment-class sibling-sweep fixes: before commit, grep for ALL retired-terminology variants across the BC (e.g., "sink chain", "Router", "SinkRegistry", "multi-sink", "fan-out", "datadog", "honeycomb", "try_send") — NOT just the literal phrase the adversary cited. Sweep ALL normative sections: §Description, §Postconditions, §Invariants, §Edge Cases, §Canonical Test Vectors, §Purity Classification, §Refactoring Notes.

---

## Summary

| ID | Severity | Section | Description |
|----|----------|---------|-------------|
| H-P28-001 | HIGH | §Description line 38 | "normal sink chain" stale per ADR-015 |
| H-P28-002 | HIGH | §Purity Classification line 135 | "sink chain" stale + "non-blocking try_send" fabricated |
| M-P28-001 | MED | §Edge Cases | EC-007 INTERNAL_ERROR row missing |
| M-P28-002 | MED | §Canonical Test Vectors | INTERNAL_ERROR test vector row missing |
| M-P28-003 | MED | §Edge Cases EC-005 + Test Vector line 98 | Missing "NO event emitted in v1" qualifier vs sibling EC-004 form |
| L-P28-001 | LOW | Postcondition 4 line 51 | "retired" → "removed" verb-precision per ADR-015 dual-verb taxonomy |
| PG-P28-001 | PROCESS | — | Third TD-VSDD-076 self-violation recurrence; codify TD-VSDD-079 |

**ADR-013 clock:** 0_of_3 RESET. Three consecutive NITPICK_ONLY passes (29/30/31) needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.
