---
document_type: adversarial-review
pass: 34
target: E-9 v1.7 Post-Audit Amendment (v1.30 surface)
angle: Definition-traceability + key-term consistency audit — NEW per TD-VSDD-057
verdict: SUBSTANTIVE
findings: 1H/3M/2L
adr_013_clock: RESET 0_of_3
date: 2026-05-05
reviewer: adversary
---

# Adversarial Review — Pass 34 (v1.30 surface)

**Angle:** Definition-traceability + key-term consistency (NEW per TD-VSDD-057). For each key term or mechanism cited in the BCs and architectural documents, trace back to the source-code definition and verify the asserted behavior is accurate — not just that the string is present.

**Verdict:** SUBSTANTIVE — 1 HIGH + 3 MED + 2 LOW

**ADR-013 clock:** RESET to 0_of_3 (HIGH finding; 3 consecutive NITPICK_ONLY passes needed for CONVERGENCE_REACHED).

---

## HIGH Findings

### HIGH-P34-001: BC-1.05.035 NUL byte rejection mechanism factually wrong — `read_wasm_string` does NOT reject NUL bytes

**Severity:** HIGH  
**Location:** BC-1.05.035 Postcondition 2, Postcondition 1 (preamble), EC-005, Precedence Ladder step (1)  
**Source-of-truth:** `crates/factory-dispatcher/src/host/memory.rs:47-54`

**Finding:** v1.30 Fix 3 (MED-P33-003) corrected the misleading parenthetical in Postcondition 1 and added a clarifying note to EC-001. However, it introduced or preserved a factual error: multiple locations in BC-1.05.035 assert that NUL bytes in `cmd` are rejected via the `read_wasm_string` error path, returning `INVALID_ARGUMENT` (-4) at Precedence Ladder step (1).

Source-code verification of `host/memory.rs:47-54`:
```rust
pub fn read_wasm_string(
    caller: &mut Caller<'_, HostContext>,
    ptr: u32,
    len: u32,
) -> Result<String, HostCallError> {
    let bytes = read_wasm_bytes(caller, ptr, len)?;
    String::from_utf8(bytes).map_err(|_| HostCallError::InvalidUtf8)
}
```

`String::from_utf8(vec![0x00])` **succeeds** — NUL byte (0x00) is valid UTF-8 (code point U+0000). `read_wasm_string` only rejects sequences that are NOT valid UTF-8 (e.g., 0xFF, overlong sequences, surrogate halves). NUL bytes pass through `read_wasm_string` without error.

**Actual NUL handling:** `Path::new(cmd).canonicalize()` on Unix converts the path string to a CString internally. CString construction fails with EINVAL for any string containing a NUL byte (C strings are NUL-terminated; an embedded NUL is illegal). This maps to Precedence Ladder step (2) `canonicalize() fails → CAPABILITY_DENIED (-1)`, NOT step (1) INVALID_ARGUMENT (-4).

**Impact:** BC-1.05.035 Postcondition 2 claims NUL bytes → `INVALID_ARGUMENT` (-4). EC-005 claims NUL bytes → `INVALID_ARGUMENT` (-4) via `read_wasm_string`. Precedence Ladder step (1) lists "NUL byte in `cmd`" as triggering `INVALID_ARGUMENT`. All three are factually wrong. A test-writer following this spec would write an incorrect test; an implementer would add an unnecessary NUL-byte check to `read_wasm_string`.

**Correct behavior:**
- Postcondition 2: `read_wasm_string` rejects non-UTF-8 byte sequences → `INVALID_ARGUMENT` (-4). NUL bytes are valid UTF-8 and are NOT rejected here.
- Precedence Ladder step (1): Non-UTF-8 bytes → `INVALID_ARGUMENT` (-4). NUL bytes pass to step (2).
- NUL-containing paths: fail at `Path::new(cmd).canonicalize()` returning Err with EINVAL via Unix CString conversion → Precedence Ladder step (2) → `CAPABILITY_DENIED` (-1).
- EC-005: NUL byte → `CAPABILITY_DENIED` (-1) via Precedence Ladder step (2), NOT INVALID_ARGUMENT.

**Fix required:** Drop NUL byte claim from Postcondition 2 description; correct EC-005 to `CAPABILITY_DENIED` (-1); correct Precedence Ladder step (1) to "Non-UTF-8 byte sequence" (not NUL byte). Add NOTE to each corrected location clarifying that NUL bytes are valid UTF-8 and follow step (2).

---

## MED Findings

### MED-P34-001: BC-1.05.035 EC-001 outcome cell assumes `binary_allow` shape without precondition binding

**Severity:** MED  
**Location:** BC-1.05.035 §Edge Cases EC-001  
**Finding:** EC-001 outcome cell reads "Returns `CAPABILITY_DENIED` (-1); allow-list match never reached (caught by existing allow-list miss: basename 'passwd' not in `binary_allow` → emit_denial('binary_not_on_allow_list') at exec_subprocess.rs:155 → CAPABILITY_DENIED. Pre-canonicalize string-level `../` reject is NOT a separate guard.)"

The outcome explanation assumes "passwd" is not in `binary_allow`, but the EC-001 precondition does not specify the shape of `binary_allow`. A test-writer setting `binary_allow = ["passwd"]` would get a different outcome (allow-list match succeeds, then `cmd = "../etc/passwd"` proceeds to canonicalize which would fail with CAPABILITY_DENIED -1 — but via Postcondition 3, not via allow-list miss). The EC-001 precondition must explicitly bind the `binary_allow` shape for the stated outcome to be uniquely determined.

**Fix required:** Add `binary_allow = ["bash"]` (or equivalent typical shape per OQ-3) explicitly to EC-001 precondition. Cite source (e.g., "typical S-9.07 capability shape per OQ-3").

---

### MED-P34-002: BC-1.05.036 §Related BCs missing sibling-disclosure of novel INVALID_ARGUMENT+capability_denied pairing

**Severity:** MED  
**Location:** BC-1.05.036 §Related BCs, row referencing BC-1.05.035  
**Finding:** v1.30 added §Description pairing rationale to BC-1.05.035 documenting the novel 5th denial path: `INVALID_ARGUMENT (-4) + internal.capability_denied` (symlink-traversal escape). BC-1.05.036 §Related BCs references BC-1.05.035 as "path canonicalization guard (sibling extension from same gap analysis)" but does NOT disclose that BC-1.05.035 introduces a 5th denial-event path with a DIFFERENT error code than the existing 4.

Test-writers building a denial-event taxonomy from BC-1.05.036 (which documents the canonical `internal.capability_denied` event) would see 4 denial paths in §Postcondition 5 (all returning CAPABILITY_DENIED -1) and miss the 5th path (INVALID_ARGUMENT -4 + same event) documented only in BC-1.05.035.

**Fix required:** Append a sibling-disclosure NOTE to the BC-1.05.035 row in BC-1.05.036 §Related BCs, clarifying the novel pairing and its test-writing implications.

---

### MED-P34-003: `INTERIM` terminology used in BCs without source-of-truth anchor in gap-analysis

**Severity:** MED  
**Location:** BC-1.05.035 §Description ADR-015 awareness clause; BC-1.05.036 §Description ADR-015 awareness clause  
**Finding:** Both BCs use the term "INTERIM" to tag `internal.capability_denied` and `host.exec_subprocess.completed` as INTERIM event names pending ADR-015 rename. However, `gap-analysis-w16-subprocess.md §"Existing denial-path telemetry"` (lines 341-351) — the cited authority — uses the phrase "MUST be renamed" but does NOT itself use or define the word "INTERIM" as a lifecycle marker.

The INTERIM tag in the BCs is a BC-introduced term with no source-of-truth definition. If the gap-analysis were updated to explicitly declare the INTERIM lifecycle marker and its sunset trigger, the cross-reference would be closed.

**Fix required:** Append to gap-analysis §"Existing denial-path telemetry" (after the rename clause at line 351) a sentence declaring `internal.capability_denied` as INTERIM with E-10 Wave 1 as the sunset event. BCs already cite this section; the cross-reference then closes bidirectionally.

---

## LOW Findings

### LOW-P34-001: `outcome` enum field defined in 3 separate locations without single source-of-truth consolidation

**Severity:** LOW  
**Location:** BC-1.05.036 §Postcondition 2, EC-008, and §Canonical Test Vectors rows  
**Finding:** The `outcome` enum (`success | failure | error | timeout | skipped | blocked`) is documented in Postcondition 2 (definition + mapping rule), EC-008 (witness), and two Canonical Test Vector rows. The mapping rule (`exit_code == 0 → 'success'; exit_code != 0 → 'failure'`) is restated three times. A future ADR-015 change to the outcome taxonomy would require updating all three locations — cosmetic duplication risk.

**Disposition recommendation:** SKIP per S-7.03 SHIP-AS-IS. Refactoring to a single source-of-truth would require structural BC changes; the duplication is cosmetic and does not introduce bugs. Document as LOW-P34-001 SKIPPED in v1.31 H3.

---

### LOW-P34-002: Postcondition 1 grammar awkward after v1.30 MED-P33-003 fix

**Severity:** LOW  
**Location:** BC-1.05.035 Postcondition 1  
**Finding:** After v1.30 Fix 3 replaced the `(../` absent, no NUL bytes)` parenthetical with a reference to `read_wasm_string` error path, the resulting Postcondition 1 reads: "If `cmd` passes the existing `read_wasm_string` error path (NUL bytes rejected per Postcondition 2) AND `Path::new(cmd).canonicalize()` succeeds AND..." — the parenthetical "(NUL bytes rejected per Postcondition 2)" is now factually wrong per HIGH-P34-001. The Fix 1 rewrite (HIGH-P34-001 fix) will naturally correct this grammar as part of removing the NUL byte claim entirely.

**Disposition:** De facto closed by Fix 1 (HIGH-P34-001 fix rewrites Postcondition 1 preamble). No separate fix needed.

---

## Summary

| ID | Severity | Location | Description |
|----|----------|----------|-------------|
| HIGH-P34-001 | HIGH | BC-1.05.035 Postconditions 1+2, EC-005, Precedence Ladder | NUL byte rejection mechanism factually wrong — `read_wasm_string` passes NUL bytes; actual path is `canonicalize()` → CAPABILITY_DENIED |
| MED-P34-001 | MED | BC-1.05.035 EC-001 | binary_allow shape not bound in preconditions; outcome cell assumes shape |
| MED-P34-002 | MED | BC-1.05.036 §Related BCs | Missing sibling-disclosure of novel INVALID_ARGUMENT+capability_denied 5th denial path |
| MED-P34-003 | MED | gap-analysis §"Existing denial-path telemetry" | INTERIM terminology used in BCs without source-of-truth declaration in gap-analysis |
| LOW-P34-001 | LOW | BC-1.05.036 §Postcondition 2, EC-008, §Canonical Test Vectors | outcome enum mapping rule duplicated in 3 locations |
| LOW-P34-002 | LOW | BC-1.05.035 Postcondition 1 | Grammar awkward after v1.30 fix; de facto closed by Fix 1 rewrite |

**ADR-013 clock RESET to 0_of_3.** Three consecutive NITPICK_ONLY passes (35/36/37) required for CONVERGENCE_REACHED.
