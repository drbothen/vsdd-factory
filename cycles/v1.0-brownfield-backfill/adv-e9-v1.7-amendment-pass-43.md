# Adversarial Review — Pass 43 (E-9 v1.7 amendment surface, epic v1.39 sealed at 589fdf9)

## 1. Angle (NEW per TD-VSDD-057)

**TD-VSDD-089 self-application audit.** Apply the just-codified TD-VSDD-089 NORMATIVE rule (4-axis sibling sweep mandate) to the v1.39 burst itself. D-285 is the FIRST burst that BOTH codified AND purportedly applied TD-VSDD-089. Mirrors pass-39's TD-VSDD-085 self-application audit which found 3 self-violations.

## 2. 4-Axis Sibling Sweep Results

All 4 axes audited. Two MEDIUM findings + 3 LOWs surfaced.

## 3. Findings

### MED-P43-001 — BC-035 line 50 denial-reason name order contradicts source-of-truth

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.035.md line 50 (Postcondition 4)

BC-035 line 50 lists denial-reason names paired with line numbers `:148/:155/:162/:169` but the names are in WRONG ORDER. Verified source-of-truth at exec_subprocess.rs:148/155/162/169:
- :148 → `no_exec_subprocess_capability`
- :155 → `binary_not_on_allow_list`
- :162 → `shell_bypass_not_acknowledged`
- :169 → `setuid_or_setgid_binary`

BC-036 P5 + EC-003 have correct positional ordering. BC-035 line 50 was authored with different ordering. Sibling-sweep should have caught it when v1.39 explicitly addressed BC-036 P5 enumeration.

### MED-P43-002 — TD-VSDD-089 codification text has misplaced `**Burst:**` trailer (codification artifact self-violation)

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** lessons.md line 860 (under TD-VSDD-089's section)

TD-VSDD-088 codification (lines 813-837) ends with `**Date:** 2026-05-05` but has NO `**Burst:**` trailer. TD-VSDD-089 (line 859) has its correct `**Burst:** D-285` trailer. Line 860 contains a SECOND `**Burst:** D-283 (FIRST application of corrected routing pattern)` — content semantically belongs to TD-VSDD-088, not TD-VSDD-089. The codification of the very rule (TD-VSDD-089 sibling-sweep mandate) failed its own discipline at the lessons.md sibling level.

### LOW-P43-001 — BC-035 line 65 lacks explicit source-frame qualifier present in BC-036 P5

**Severity:** LOW (cosmetic per S-7.03 SHIP-AS-IS)

### LOW-P43-002 — BC-036 P5 "All three no-event error paths" wording mismatches enumerated path count

**Severity:** LOW (interpretive)

### LOW-P43-003 — BC-035 line 52 ladder step (2) parenthetical names ETIMEDOUT without dedicated EC

**Severity:** LOW

## 4. Verdict

**SUBSTANTIVE.** 0 HIGH + 2 MEDIUM + 3 LOW. ADR-013 clock RESETS to 0_of_3.

## 5. Process-Gap Tagging

- [process-gap] MED-P43-002 indicates TD-VSDD-089 sweep scope must extend to lessons.md sibling TD entries.
- [process-gap] Pattern flag (NOT yet codified per S-7.02): 2 instances of "codification burst violates own rule" — pass-39 TD-VSDD-085; pass-43 TD-VSDD-089. Below 3+ threshold; tracking only.

## 6. Source-of-Truth Verification Log

Verified all 4 emit_denial call sites at exec_subprocess.rs:148/155/162/169 against current source. Verified TD-VSDD-088/089 codification text against lessons.md current state.

## 7. TD-VSDD Lesson Awareness

Reviewed TD-VSDD-057 through TD-VSDD-089. Pass-43 angle (TD-VSDD-089 self-application) NEW. Findings net-new at the self-application axis.
