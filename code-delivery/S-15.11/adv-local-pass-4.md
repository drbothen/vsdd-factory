---
document_type: adversary-pass-report
producer: adversary
pass_id: S-15.11-LOCAL-P4
diff_base: origin/develop@6fe7de4c
diff_head: feature/S-15.11-validate-burst-log@568fe198
verdict: MEDIUM
streak_status: "0/3 (MEDIUM resets per BC-5.39.001)"
timestamp: 2026-05-16T00:00:00Z
---

# S-15.11 LOCAL Adversary Pass-4

**Diff base:** `origin/develop@6fe7de4c`
**Diff head:** `feature/S-15.11-validate-burst-log@568fe198`
**Commits under review (10):** Red Gate + impl + 4 fix-burst-1 + ext + 2 fix-burst-2.

## Part A: Findings

### F-S15.11-LOCAL-P4-001 — `validate_h2_heading` slices `&str` at byte index not guaranteed to be on UTF-8 char boundary

**Severity:** MEDIUM. **Confidence:** HIGH.
**Location:** `crates/hook-plugins/validate-burst-log/src/lib.rs:175-179`.
**Policy axes:** Production-grade default + sibling-site parity (TD-VSDD-060) + Accumulate Invariants (S-15.07 RECOVERY lesson).

**The defect:**
```rust
if last_paren > 0 {
    let before = &after_prefix[last_paren - 1..last_paren];
    if before != " " {
        return false;
    }
}
```

`after_prefix.rfind('(')` returns the byte index of `(`. If the description preceding `(` ends with a multi-byte UTF-8 character (em-dash `—` U+2014 3 bytes, en-dash, non-breaking space, typographic apostrophe, or any non-ASCII glyph) immediately followed by `(`, then `last_paren - 1` falls inside the multi-byte sequence. The slice expression panics at runtime: `byte index N is not a char boundary; it is inside 'X'...`.

**Concrete failing input:** `validate_h2_heading("## Burst: foo—(2026-05-12)")` panics.

**Why MEDIUM, not LOW:**

1. **Sibling-site parity gap (TD-VSDD-060).** Sibling crate `validate-index-cite-refresh/src/lib.rs:244-247` uses `is_char_boundary()` defensively for analogous string-scanning. Defensive pattern not propagated to validate-burst-log.

2. **Recurring failure-class precedent.** S-15.07 RECOVERY explicitly cites "UTF-8 panic on em-dash" as one of the 2 real bugs caught during API 500 recovery. Lesson should have propagated.

3. **Fail-open absorbs the panic but defeats the gate.** WASM panic with `on_error="continue"` → `plugin.crashed` event → Continue. Same anti-pattern as F-S15.11-LOCAL-P2-001 (production-registry `**` glob silently fail-opening).

4. **Production exploitability is non-zero.** Project prose-style conventions use em-dashes throughout (decision-log, lessons.md, PR descriptions). Accidental introduction plausible.

**Fix:**
```rust
if last_paren > 0 {
    if !after_prefix.is_char_boundary(last_paren - 1) {
        return false;
    }
    let before = &after_prefix[last_paren - 1..last_paren];
    if before != " " {
        return false;
    }
}
```

Add unit tests for em-dash, en-dash, NBSP cases + ASCII-space control.

## Part B: Observations (compliance commendations)

### O-P4-001 — Helper-function trio `is_*_target` parity consistent across sibling hooks (commendation)

### O-P4-002 — Canonical `tool = "Edit|Write"` 5-reference-class sweep clean (zero `Write|Edit` reverse-form)

### O-P4-003 — `ends_with` filename-guard residue audit clean (all 6 remaining refs are documentary changelog narrative)

### O-P4-004 — Production-registry capability-shape regression test is load-bearing (`integration-production-registry.bats` Scenario B catches `**` regression)

### O-P4-005 — TD-VSDD-059 paper-fix discipline holds for prior-pass closures (F-P1-001, F-P2-002, F-P2-003, F-P2-004, F-P3-001 each have load-bearing tests)

### O-P4-006 — `cited_raw: String` structural plumbing present

## Part C: Policy Rubric Compliance

| Policy | Status |
|--------|--------|
| POL-1..18 | All PASS or N/A; no policy violations |

(See full table in main report sections — every policy either PASS or N/A; no violations found this pass. Sole finding is sibling-parity TD-VSDD-060 axis + production-grade Accumulate-Invariants axis.)

## Part D: Verdict + Streak

**Verdict:** MEDIUM (1 finding).
**Streak:** 0/3 (MEDIUM resets).

**Implementation health:** Structurally sound except for the F-P4-001 UTF-8 panic gap. Prior-pass paper-fix discipline holds; canonical sweeps clean; cross-crate parity intact for filename-guards (gap on UTF-8 boundary handling only).

## Part E: Recommendations for Fix-Burst-4

1. **Apply F-P4-001 structural fix** in `validate_h2_heading`: add `is_char_boundary()` guard before unsafe slice.
2. **Add load-bearing unit tests** for em-dash, en-dash, NBSP UTF-8 cases.
3. **(Same-burst hygiene)** Audit other byte-index slice expressions in validate-burst-log (`extract_dim1_headline_count`, `count_dim1_list_items`) for analogous UTF-8 boundary safety.
4. **(Defer-codification candidate)** Suggests cross-hook lesson: "WASM hook string-scanning that indexes into `&str` by computed byte position MUST use `is_char_boundary()` defensively or document safe-by-construction." Routes to state-manager for L-EDP1-NNN codification consideration.

**Pass-5 expected disposition:** With UTF-8 guard added, expect CLEAN or NITPICK_ONLY; streak advances to 1/3.
