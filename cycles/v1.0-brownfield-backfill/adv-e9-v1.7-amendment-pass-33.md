---
document_type: adversarial-review
pass: 33
target: E-9 v1.7 Post-Audit Amendment (v1.29 surface)
angle: Test-Vector ↔ Postcondition coherence audit (inverse direction: postcondition→witness) — NEW per TD-VSDD-057
verdict: SUBSTANTIVE
findings: 0H/3M/1L
adr_013_clock: RESET 0_of_3
date: 2026-05-05
reviewer: adversary
---

# Adversarial Review Pass-33: E-9 v1.29 — Test-Vector ↔ Postcondition Coherence

## Audit Angle

Postcondition→witness coherence audit (inverse direction per TD-VSDD-057): for each normative Postcondition claim in the in-scope BCs, verify that at least one §Edge Cases row or §Canonical Test Vector row witnesses the postcondition outcome. This is the inverse of the forward direction (test-vector→postcondition) audited in pass-25 and pass-28.

## Verdict: SUBSTANTIVE — 0H / 3M / 1L

ADR-013 clock RESET to 0_of_3.

---

## MED-P33-001: BC-1.05.036 §Postcondition 2 outcome-enum mandate has no witness in §Edge Cases or §Canonical Test Vectors

**Severity:** MED  
**Location:** BC-1.05.036 §Postconditions line 49 (added in v1.29)  
**Finding:**  
BC-1.05.036 Postcondition 2 now mandates that the host stamps an `outcome` enum field per ADR-015 D-15.3 enrichment: `exit_code == 0 → 'success'`; `exit_code != 0 → 'failure'`. This was added by D-274 (MED-P31-002 closure, v1.29).

However, neither §Edge Cases nor §Canonical Test Vectors contains any row that witnesses this mapping. The existing EC-001 (exit_code=0) and EC-002 (exit_code=1) rows verify event emission only; neither asserts the host-stamped `outcome` field value. The Canonical Test Vector rows for "Capability passes; subprocess exits 0" and "Capability passes; subprocess exits 1" similarly verify `exit_code` payload but do not assert `outcome='success'` / `outcome='failure'`.

An implementer reading only §Edge Cases + §Canonical Test Vectors would have no test anchoring the Postcondition 2 outcome-enum mandate. This leaves a gap between the normative postcondition (added in v1.29) and the verifiable test contract.

**Recommended fix:** Add EC-008 and two Canonical Test Vector rows (outcome=success; outcome=failure) that explicitly witness the Postcondition 2 mapping.

---

## MED-P33-002: BC-1.05.035 §Postcondition 4 + EC-002 + Test Vector row 3 — event-emission witness missing; novel INVALID_ARGUMENT+capability_denied pairing undocumented

**Severity:** MED  
**Location:** BC-1.05.035 §Postcondition 4 (line 48); EC-002 (line 82); Canonical Test Vector row 3 (line 92)  
**Finding:**  
BC-1.05.035 Postcondition 4 states: "If the canonicalized path contains `..` components after resolution (symlink-based escape attempt), returns `codes::INVALID_ARGUMENT` (-4) **and the existing `internal.capability_denied` event is emitted**."

EC-002 verifies only: "returns `INVALID_ARGUMENT` (-4)". The `internal.capability_denied` event emission is not mentioned.  
Canonical Test Vector row 3 ("Symlink `cmd` resolving outside project dir") lists expected output as "`INVALID_ARGUMENT` (-4)". No event assertion.

Two compounding issues:
1. Test vectors provide no witness for the event-emission branch of Postcondition 4.
2. The INVALID_ARGUMENT+capability_denied pairing is novel (all 4 existing emit_denial sites — exec_subprocess.rs:148/155/162/169 — return CAPABILITY_DENIED -1, not INVALID_ARGUMENT -4). No §Description rationale explains why this pairing differs from the existing 4 denial paths.

**Recommended fix:** (a) Add §Description rationale explaining the INVALID_ARGUMENT+capability_denied pairing. (b) Append event-emission assertion to EC-002 outcome cell. (c) Append event-emission assertion to Test Vector row 3 outcome cell.

---

## MED-P33-003: BC-1.05.035 §Postcondition 1 misleading "(`../` absent, no NUL bytes)" parenthetical; `../` guard path undefined

**Severity:** MED  
**Location:** BC-1.05.035 §Postcondition 1 (line 45)  
**Finding:**  
Postcondition 1 currently reads: "If `cmd` passes string-level validation (`../` absent, no NUL bytes) AND `Path::new(cmd).canonicalize()` succeeds AND the canonicalized path does not contain `..` segments, the allow-list check proceeds normally."

The parenthetical "(`../` absent, no NUL bytes)" implies a pre-canonicalize string-level `../` reject guard exists as a distinct step. However:
- NUL bytes are handled by `read_wasm_string` error path (Postcondition 2 + Precedence Ladder step 1). Correct.
- `../` rejection is NOT a named guard. The Precedence Ladder (Postcondition 5, steps 1-4) has no "`../` literal present → reject" step. EC-001 (`cmd="../etc/passwd"`) expects CAPABILITY_DENIED (-1), which is explained by the allow-list miss path (basename `passwd` not in `binary_allow` → emit_denial("binary_not_on_allow_list") at exec_subprocess.rs:155 → CAPABILITY_DENIED), NOT by a pre-canonicalize string-level filter.

The parenthetical creates false expectation that a dedicated string-level `../` guard exists. An implementer who adds such a guard (instead of relying on allow-list miss) would implement duplicate/inconsistent behavior.

**Recommended fix:** Remove "(`../` absent, no NUL bytes)" from Postcondition 1; replace with reference to the `read_wasm_string` error path (NUL bytes only). Add clarifying note to EC-001 explaining the CAPABILITY_DENIED outcome is via allow-list miss path.

---

## LOW-P33-001: BC-1.05.035 §Description anchor cites wrong H3 for rename rationale

**Severity:** LOW  
**Location:** BC-1.05.035 §Description line 33  
**Finding:**  
Current text: `gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" lines 339-349`

Per gap-analysis-w16-subprocess.md:
- H3 `### How ADR-015 affects the telemetry gap` ends at line 339 (transitional sentence)
- H3 `### Existing denial-path telemetry (Section 1, row "Telemetry event on denial")` begins at line 341
- The rename rationale (`internal.capability_denied` → `vsdd.capability.denied.exec_subprocess.v1`) is in §"Existing denial-path telemetry" at lines 343-351

The current citation is pointing at the wrong H3 for lines 339-349 (which straddle the H3 boundary; line 341 is the new H3 header; lines 343-351 are the rename rationale). The correct citation is `§"Existing denial-path telemetry" lines 341-351`.

**Recommended fix:** Replace `§"How ADR-015 affects the telemetry gap" lines 339-349` with `§"Existing denial-path telemetry" lines 341-351`.

---

## Source-of-Truth Verification Applied

- gap-analysis-w16-subprocess.md: H3 `### Existing denial-path telemetry` begins at line 341 (confirmed). Rename rationale at lines 343-351 (confirmed).
- exec_subprocess.rs:148/155/162/169 emit_denial reasons (all 4 use CAPABILITY_DENIED -1): confirmed unchanged per prior passes 25/28/29.
- BC-1.05.035 Postcondition 4 event + INVALID_ARGUMENT code confirmed at line 48.
- BC-1.05.036 Postcondition 2 outcome-enum mandate confirmed at line 49 (added v1.29).

---

## Deferred / Skipped

None. All 4 findings are actionable in a single fix burst.

---

## ADR-013 Clock

**RESET to 0_of_3.** Pass-33 is SUBSTANTIVE (3M+1L). Three consecutive NITPICK_ONLY passes (34/35/36) now needed to reach CONVERGENCE_REACHED per ADR-013 + TD-VSDD-057.
