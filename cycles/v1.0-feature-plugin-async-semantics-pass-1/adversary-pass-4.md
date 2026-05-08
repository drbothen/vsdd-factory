---
document_type: adversary-pass
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F2
pass: 4
producer: adversary
producer_model: claude-opus-4-7[1m]
fresh_context: true
verdict: SUBSTANTIVE
finding_count: { high: 1, medium: 2, low: 2, nit: 1 }
adr-013_clock_action: reset
clock: 0_of_3
timestamp: 2026-05-07T00:00:00Z
---

# Adversary Pass-4 Findings — F2 spec package, v1.0-feature-plugin-async-semantics-pass-1

## Verdict
**SUBSTANTIVE.** Trajectory 19→19→7→6 (improving). Mostly propagation residue from pass-3 user-correction round. Clock RESETS to 0/3.

## Counts
HIGH: 1, MEDIUM: 2, LOW: 2, NIT: 1

## Findings

### F-P4-001 [HIGH] BC-INDEX subsystem counts disagree with ARCH-INDEX after F2 pass-3 re-tally — POLICY 6 violation

**Evidence:**
- BC-INDEX lines 67-81 + 85, 1588: SS-01: 116, SS-07: 197, SS-05: 648, SS-08: 218
- ARCH-INDEX lines 99-114 (re-tallied per authoritative frontmatter subsystem): SS-01: 117, SS-07: 196, SS-05: 652, SS-08: 214
- Total 1947 still matches in both, but per-subsystem counts diverge by 1 (SS-01/07) and 4 (SS-05/08)

**Internal inconsistency in BC-INDEX itself:** BC-8.29.001/002/003 + BC-8.30.002 are listed under SS-05 section (line 990-993, frontmatter-authoritative). BUT BC-7.06.001 is listed under SS-07 section (line 1788, directory-based). Same situation, two different conventions in the same file.

**Severity:** HIGH (POLICY 6; two source-of-truth divergence within BC-INDEX itself).

**Fix:** BC-INDEX Summary table + section headers re-tallied to authoritative frontmatter subsystem matching ARCH-INDEX, AND BC-7.06.001 listing convention unified (move it to SS-01 section to match BC-8.29 convention).

### F-P4-002 [MEDIUM] VP-INDEX Domain Invariant column missing DI-019 for VP-079 — POLICY 9

**Evidence:**
- VP-079.md line 31: `domain_invariants: [DI-017, DI-019]`
- VP-INDEX line 150 — Domain Invariant column shows "—" (em-dash, no DIs listed)

**Fix:** VP-INDEX line 150 Domain Invariant column → `DI-017, DI-019`.

### F-P4-003 [MEDIUM] VP-078 frontmatter `scope: SS-07` vs VP-INDEX `SS-07, SS-01` — POLICY 9

**Evidence:**
- VP-078.md line 28: `scope: SS-07` (single-valued)
- VP-INDEX line 149: Scope column "SS-07, SS-01"
- VP-078.md Traceability section (lines 452-453): "SS-07 (config side); SS-01 (runtime-side enforcement)" — semantically dual

**Fix:** Expand VP-078 frontmatter to `scope: SS-07, SS-01` matching VP-INDEX and Traceability prose. (Compare VP-073/075 which use dual-subsystem convention in both places.)

### F-P4-004 [LOW] ADR-019 §Consequences "Async-task drain window" formula inlines literal `100ms` (line 215)

**Evidence:**
- ADR-019 line 209: formula uses symbolic `ASYNC_DRAIN_WINDOW_MS` ✓
- Line 215: `max(sync_plugin_durations) + 100ms` — inlines literal value
- DI-019 lift principle (v1.4→v1.5 amendment) elevated constant from inline to canonical DI

**Fix:** Replace `+ 100ms` with `+ ASYNC_DRAIN_WINDOW_MS` (symbolic) on line 215.

### F-P4-005 [LOW] BC-1.14.001 PC4 inline `100 ms` parenthetical contradicts §Constant Reference rule

**Evidence:**
- BC-1.14.001 line 67 PC4: contains "(per DI-019, default 100 ms)" — value inlined parenthetically
- Same file line 85 §Constant Reference: "Do not inline the constant value here — consult DI-019 for the authoritative value"
- Same risk in EC-011 line 141

**Fix:** Remove `default 100 ms` parenthetical in PC4 (lines 67, 68) and EC-011 (line 141) — keep only `per DI-019`.

### F-P4-006 [NIT] VP-077 frontmatter `domain_invariants: []` could cite DI-014 (debatable)

**Evidence:**
- VP-077 line 32: `domain_invariants: []`
- VP-077 verifies BC-1.14.001 partition contract; PC1 cites schema_version=2 which is DI-014's domain
- Defensible empty value: VP-077 is downstream of schema validation

**Fix (optional):** Add DI-014 to VP-077 `domain_invariants` if architectural choice is comprehensive citation; otherwise document the empty value as deliberate.

## Policy compliance

- POLICY 1: ✓
- POLICY 2: ✓
- POLICY 3: N/A
- POLICY 4: ✓
- POLICY 5: ✓
- POLICY 6: ✗ F-P4-001
- POLICY 7: ✓
- POLICY 8: ✓
- POLICY 9: ✗ F-P4-002, F-P4-003

## Open questions

1. ASYNC_DRAIN_WINDOW_MS = 0 or negative — DI-019 doesn't address. NOT a finding for this pass — defer per DI-019's "deferred decision" note.
2. Multiple async plugins competing for drain window — order not specified; acceptable per BC-1.14.001 Invariant 3.
3. Dispatcher crash during drain — DI-019 silent; partial events flushed = best-effort per BC-3.08.001. Acceptable.

## Top 3 (SUBSTANTIVE)

1. **F-P4-001 (HIGH)** — BC-INDEX Summary + SS-01/07/05/08 section headers not re-tallied to match ARCH-INDEX authoritative-subsystem counts. POLICY 6 violation.
2. **F-P4-002 (MEDIUM)** — VP-INDEX line 150 missing DI-019 in Domain Invariant column for VP-079.
3. **F-P4-003 (MEDIUM)** — VP-078 frontmatter `scope: SS-07` vs VP-INDEX line 149 `SS-07, SS-01`.
