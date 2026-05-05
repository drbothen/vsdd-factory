# Adversarial Review — E-9 v1.25 Amendment — Pass 27

**Document:** E-9 v1.7 amendment surface (spec package v1.25)
**Pass:** 27
**Angle:** Negative-coverage / silence audit (NEW per TD-VSDD-057)
**Verdict:** SUBSTANTIVE — 1 HIGH + 1 MED + 0 LOW
**ADR-013 clock:** RESET to 0_of_3 (was 1_of_3 after pass-26 NITPICK_ONLY)
**Date:** 2026-05-05
**Sealed by:** D-270

---

## Angle Description

Negative-coverage / silence audit: methodically enumerate what the spec does NOT say and test whether each silence is intentional or an accidental gap. Focus on: no-event paths, out-of-scope carve-outs, error codes enumerated in postconditions vs. what source actually contains. Complementary to the positive-coverage traceability sweep of pass-25.

---

## Findings

### H-P27-001 [HIGH] — BC-1.05.036:51 stale ADR-005-era multi-sink wording

**Location:** `BC-1.05.036.md` Postcondition 4 (line 51)

**Finding:** Postcondition 4 reads: "Event is routed through the normal `ctx.emit_internal` sink chain (file/datadog/honeycomb per config), the same path as the existing `emit_denial` call."

The phrase "file/datadog/honeycomb per config" describes the ADR-005-era multi-sink model that ADR-015 D-15.1 explicitly retires:
- ADR-015 line 99: "All events… are written to one physical file: `.factory/logs/events-YYYY-MM-DD.jsonl`"
- ADR-015 line 130: "The `sink-otel-grpc` crate AND the `Router`, `SinkRegistry` types within `sink-core` are retired"
- ADR-015 line 154: "The multi-sink stanza model is removed. Operators who need remote export configure the OTel Collector as the second hop, not the dispatcher."

An implementer reading Postcondition 4 gets a directly contradicted contract vs. ADR-015 D-15.1. The BC was amended at v1.21 to add ADR-015 awareness in the Description block but the stale multi-sink wording in Postcondition 4 was not updated.

**Impact:** Implementer confusion. If Postcondition 4 is read as the normative routing contract, implementer may attempt to configure multi-sink fan-out to Datadog/Honeycomb inside the dispatcher, which would violate ADR-015 D-15.1.

**Fix required:** Replace "file/datadog/honeycomb per config" with single-stream FileSink description per ADR-015 D-15.1.

---

### M-P27-001 [MED] — BC-1.05.036 Postcondition 5 incomplete: INTERNAL_ERROR (-99) no-event path missing

**Location:** `BC-1.05.036.md` Postcondition 5 (line 52)

**Finding:** Postcondition 5 enumerates only two no-event error paths: "TIMEOUT (-2) and OUTPUT_TOO_LARGE (-3) paths return error codes WITHOUT emitting any event." However, source code contains a THIRD no-event error path: `INTERNAL_ERROR (-99)`.

Source evidence (per TD-VSDD-078 enumeration discipline):
- `exec_subprocess.rs:252`: `command.spawn().map_err(|_| codes::INTERNAL_ERROR)?` — spawn failure
- `exec_subprocess.rs:258`: `child.stdin.take().ok_or(codes::INTERNAL_ERROR)?` — stdin take failure
- `exec_subprocess.rs:262`: `return Err(codes::INTERNAL_ERROR)` — stdin write_all failure
- `exec_subprocess.rs:267`: `child.stdout.take().ok_or(codes::INTERNAL_ERROR)?` — stdout take failure
- `exec_subprocess.rs:268`: `child.stderr.take().ok_or(codes::INTERNAL_ERROR)?` — stderr take failure
- `exec_subprocess.rs:299`: `Err(_) => return Err(codes::INTERNAL_ERROR)` — try_wait error

Constant defined at `crates/factory-dispatcher/src/host/mod.rs:184`: `pub const INTERNAL_ERROR: i32 = -99;`

All 5 INTERNAL_ERROR paths return `Err(codes::INTERNAL_ERROR)` before any `emit_internal` call — no event is emitted. Postcondition 5's enumeration of no-event paths is incomplete per TD-VSDD-078 source-of-truth-enumeration discipline.

**Fix required:** Add INTERNAL_ERROR (-99) enumeration to Postcondition 5 with source-code line citations.

---

## Convention Checks

- **TD-VSDD-059 frontmatter coherence:** `version: "1.25"` matches summary table latest non-reserved row. PASS.
- **TD-VSDD-075 source-code citations:** All source claims in v1.25 amendment verified PASS (continuing pass-25 verification).
- **TD-VSDD-076 intra-document semantic siblings:** H-P27-001 requires propagation check — see fix burst (no additional sibling sites found for Postcondition 4 wording beyond line 51).
- **TD-VSDD-078 enumeration discipline:** M-P27-001 is a direct application — Postcondition 5 enumeration incomplete.
- **ADR-015 D-15.1 awareness block (Description §):** Correctly present. The issue is Postcondition 4 body text not updated to match.

---

## Trajectory

| Pass | Verdict | H | M | L | Clock |
|------|---------|---|---|---|-------|
| 25 | SUBSTANTIVE | 1 | 2 | 2 | 0_of_3 |
| 26 | NITPICK_ONLY | 0 | 0 | 3 | 1_of_3 |
| 27 | SUBSTANTIVE | 1 | 1 | 0 | **RESET 0_of_3** |

---

## Disposition

Fix burst D-270 required before pass-28. After v1.26 fix burst: 3 fresh-context NITPICK_ONLY passes (28/29/30) needed to reach CONVERGENCE_REACHED per ADR-013.
