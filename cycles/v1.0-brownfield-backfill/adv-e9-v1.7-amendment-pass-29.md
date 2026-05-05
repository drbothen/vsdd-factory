---
document_type: adversarial-review
level: ops
version: "1.0"
status: sealed
producer: adversary
timestamp: 2026-05-05T00:00:00Z
phase: D-4
cycle: v1.0-brownfield-backfill
pass: 29
target_version: "1.27"
verdict: SUBSTANTIVE
finding_counts: {high: 2, med: 0, low: 0}
adr_013_clock_action: RESET
adr_013_clock_after: 0_of_3
sealed_by: D-272
---

# Adversarial Review: E-9 v1.7 Amendment — Pass 29

**Target:** E-9 v1.27 and in-scope BC + arch-doc files  
**Angle:** Cross-document terminology drift detection (NEW per TD-VSDD-057)  
**Verdict:** SUBSTANTIVE — 2 HIGH / 0 MED / 0 LOW  
**ADR-013 clock:** RESET to 0_of_3 (was 0_of_3 after pass-28; remains at 0 due to SUBSTANTIVE verdict)

---

## Pass Methodology

Cross-document terminology drift detection angle: audits all in-scope BC files and arch-doc files simultaneously, scanning for prohibited terminology families across ALL normative sections. Specifically applies the TD-VSDD-079 8-term grep checklist (`sink chain`, `Router`, `SinkRegistry`, `DlqWriter`, `multi-sink`, `fan-out`, `Datadog`, `Honeycomb`, `try_send`) across all 5 in-scope files (BC-1.05.035, BC-1.05.036, gap-analysis-w16-subprocess.md, audit-w16.md, perf-baseline-w16.md). Cross-references TD-VSDD-079 codification text in E-9 v1.27 H3 against the actual grep run performed by the v1.27 burst (D-271).

---

## Findings

### H-P29-001 [HIGH] — TD-VSDD-079 self-violation: v1.27 burst codified 8-term family grep but ran only 2-term grep

**Location:** BC-1.05.036.md line 51 (Postcondition 4)  
**Current text:** "external fan-out to Datadog/Honeycomb is handled by OTel Collector OUTSIDE the dispatcher"  
**Prohibited terms hit:** `fan-out` (TD-VSDD-079 prohibited family term) AND `Datadog` AND `Honeycomb` (TD-VSDD-079 prohibited family terms)

**Defect:** The v1.27 burst (D-271) codified TD-VSDD-079 with explicit 8-term grep checklist in the v1.27 H3 changelog text. However, the v1.27 burst's own post-edit grep verification (recorded at BC-1.05.036.md line 1461) ran only `grep -n 'sink chain\|try_send' BC-1.05.036.md` — a 2-term grep. This grep satisfied "the literal phrase cited by the prior adversary (pass-28)" but did NOT run the full codified 8-term family check.

Result: The burst that codified TD-VSDD-079 simultaneously violated it. `fan-out`, `Datadog`, and `Honeycomb` all remain at BC-1.05.036 Postcondition 4 line 51 in non-changelog body text.

**This is the same self-violation pattern** previously caught at:
- pass-24 H-P24-001 (v1.23 codified TD-VSDD-076 but had truncated:bool annotation inconsistency)
- pass-25 M-P25-001 (v1.24 fix EC-003 sibling-aligned only after caught)
- pass-28 H-P28-001/002 (v1.26 silence-audit scrubbed line 51 only)

**Fix required:** BC-1.05.036:51 — replace "external fan-out to Datadog/Honeycomb is handled by OTel Collector OUTSIDE the dispatcher" with vendor-neutral, family-term-free wording. Suggested: "external export to remote observability backends is handled by OTel Collector outside the dispatcher"

**Process-gap:** TD-VSDD-079 must be mechanized as a pre-commit hook, not narrative discipline. Codify as TD-VSDD-080.

---

### H-P29-002 [HIGH] — BC-1.05.035 §Description NUL-byte attribution contradicts 3 normative sections

**Location:** BC-1.05.035.md line 35 (§Description body)  
**Current text:** "Canonicalization resolves symlinks, eliminates `..` segments, and rejects NUL bytes"

**Defect:** Intra-document semantic contradiction. §Description attributes NUL-byte rejection to `Path::canonicalize()`. But 3 other normative sections attribute it to the existing `read_wasm_string` error path:

1. **§Postcondition 2 (line 46):** "If `cmd` contains a NUL byte or fails basic string validation, returns `codes::INVALID_ARGUMENT` (-4) — existing `read_wasm_string` error path." (No mention of canonicalize)
2. **§Precedence Ladder (line 50):** "(1) NUL byte in `cmd` → `Err(INVALID_ARGUMENT -4)`; (2) `Path::new(cmd).canonicalize()` returns Err → ..." — NUL check at step (1) is BEFORE canonicalize at step (2)
3. **§EC-005 (line 84):** "Returns `INVALID_ARGUMENT` (-4) via existing `read_wasm_string` error path before canonicalize"

The §Description claim that canonicalize "rejects NUL bytes" contradicts the other 3 normative sections which place NUL rejection BEFORE canonicalization. Additionally, `Path::canonicalize()` on Unix does not actually reject NUL bytes — it returns `io::Error` which maps to `CAPABILITY_DENIED` (-1) per §Postcondition 3, NOT `INVALID_ARGUMENT` (-4) per §Postcondition 2/§EC-005.

**Fix required:** BC-1.05.035:35 §Description — align with the 3 normative sections by removing the NUL-byte-rejection claim from canonicalize and redirecting to `read_wasm_string` error path.

**Suggested replacement:** "Canonicalization resolves symlinks and eliminates `..` segments. NUL-byte rejection is performed earlier by the existing `read_wasm_string` error path (see §Postcondition 2 and the Precedence Ladder)."

---

## Process Gap

**PG-P29-001 — TD-VSDD-079 narrative-discipline consistently fails at first application; must mechanize as pre-commit hook.**

Pattern: v1.27 burst (D-271) codified TD-VSDD-079 with an explicit 8-term grep checklist, yet the burst's own pre-commit grep ran only 2 terms. This is the 5th consecutive instance where a lesson-codification burst violated its own rule:
- pass-24 H-P24-001 (TD-VSDD-076 self-violation at codification)
- pass-25 M-P25-001 (TD-VSDD-076 self-violation one burst later)
- pass-28 H-P28-001/002 (TD-VSDD-076 self-violation three bursts later)
- pass-29 H-P29-001 (TD-VSDD-079 self-violation at codification)

Narrative discipline has failed 5 consecutive times. Mechanical enforcement is required: implement `validate-bc-terminology-family.sh` pre-commit hook. Codify as TD-VSDD-080.

---

## ADR-013 Clock

- Before this pass: 0_of_3 (reset by pass-28 SUBSTANTIVE)
- This pass: SUBSTANTIVE (2 HIGH)
- After this pass: 0_of_3 (REMAINS at 0 — SUBSTANTIVE verdict; no advance)

Three consecutive NITPICK_ONLY passes (30/31/32) needed to reach CONVERGENCE_REACHED.

---

## Convergence Trajectory (passes 1–29)

pass-1 NITPICK → pass-2 NITPICK → pass-3 SUB → v1.8 fix → pass-4 SUB → v1.9 fix → pass-5 SUB → v1.10 fix → pass-6 SUB → v1.11 fix → pass-7 SUB → v1.12 fix → pass-8 SUB → v1.13 fix → pass-9 NITPICK → pass-10 NITPICK → pass-11 SUB → v1.14 fix → pass-12 SUB → v1.15 fix → pass-13 SUB → v1.16 fix → pass-14 SUB → v1.17 fix → pass-15 SUB → v1.18 fix → pass-16 NITPICK → pass-17 SUB → v1.19 fix → pass-18 SUB → v1.20 fix → pass-19 NITPICK → pass-20 SUB → v1.21 fix → pass-21 SUB → v1.22 fix → pass-22 SUB → v1.23 fix → pass-23 NITPICK → pass-24 SUB → v1.24 fix → pass-25 SUB → v1.25 fix → pass-26 NITPICK → pass-27 SUB → v1.26 fix → pass-28 SUB → **v1.27 fix → pass-29 SUB (this pass)**

---

*Sealed by D-272. Fixed in v1.28 burst.*
