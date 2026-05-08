---
document_type: adversary-pass
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F2
pass: 7
producer: adversary
producer_model: claude-opus-4-7[1m]
fresh_context: true
verdict: SUBSTANTIVE
finding_count: { high: 0, medium: 2, low: 0, nit: 2 }
adr-013_clock_action: reset
clock: 0_of_3
timestamp: 2026-05-07T00:00:00Z
---

═══════════════════════════════════════
[BEGIN]

# Adversary Pass-7 Findings — F2 spec package

## Verdict
**SUBSTANTIVE.** No HIGH findings (first pass without HIGH; major progress). Two MEDIUM and two NIT. Trajectory 19→19→7→6→3→5→4. Clock RESETS to 0/3.

## Counts
HIGH: 0, MEDIUM: 2, LOW: 0, NIT: 2

## Findings

### F-P7-001 [MEDIUM] — VP-079 retains 9 inline `100ms` literals; sibling-fix gap from DI-019 canonical-home propagation

**Evidence:** VP-079.md contains inline `100ms` at lines:
- 67, 74 (Property 5)
- 178 (Scenario 1 comment)
- 329 (Scenario 4 comment)
- 389, 399 (Scenario 5)
- 472 (Feasibility Assessment)
- 488 (Traceability)

Counter-evidence (sibling fixes propagated):
- ADR-019 v1.7→v1.8 removed `(governed by DI-019; default 100ms)` → `(governed by DI-019)`
- BC-1.14.001 v1.4→v1.5 removed three inline `100 ms` literals
- BC-3.08.001 v1.2→v1.3 removed inline `100 ms` from Traceability

**Impact:** S-7.01 partial-fix regression. VP-079 is the most heavily citing artifact for DI-019 and was missed. Future DI-019 value change will leave 9 stale literals in VP-079.

**Fix:** Replace inline `100ms` with `ASYNC_DRAIN_WINDOW_MS` (symbolic) for formula contexts and `(per DI-019)` for citations.

### F-P7-002 [MEDIUM] — BC-9.01.006 frontmatter `inputs:` cites nonexistent ADR-019 path

**Evidence:** BC-9.01.006.md line 13: `  - .factory/specs/architecture/ADR-019.md`. Glob returns no file. Actual path: `.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md`.

**Impact:** State-manager input-hash recomputation will fail. POLICY 4 semantic_anchoring violation.

**Fix:** Update line 13 to canonical path.

### F-P7-003 [NIT] — VP-079 line 65 stale "BC-1.14.001 v1.4" citation

**Evidence:** VP-079.md line 65: "DI-019 per BC-1.14.001 v1.4 traceability". BC-1.14.001 is now v1.5.

**Fix:** Drop version qualifier or update to "BC-1.14.001 PC4 traceability".

### F-P7-004 [NIT] — BC-1.14.001 line 170 redundant "(per DI-019)" parenthetical

**Evidence:** Cell starts with "DI-019 — `ASYNC_DRAIN_WINDOW_MS` (per DI-019; PC4 ...)". The "(per DI-019;" is redundant given the row identifier already cites DI-019.

**Fix:** Optional. Remove redundant parenthetical for terseness.

## Policy compliance

| Policy | Status |
|---|---|
| 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12 | PASS (sampled byte-for-byte where applicable) |
| 4 semantic_anchoring | PASS w/F-P7-002 (one stale path) |

## Top 3

1. **F-P7-001 (MEDIUM)** — VP-079 sibling-fix gap; 9 inline `100ms` literals
2. **F-P7-002 (MEDIUM)** — BC-9.01.006 stale ADR-019 path in inputs
3. **F-P7-003 (NIT)** — VP-079 stale BC-1.14.001 v1.4 cite

[END]
═══════════════════════════════════════
