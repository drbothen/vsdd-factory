---
document_type: adversary-pass
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F2
pass: 5
producer: adversary
producer_model: claude-opus-4-7[1m]
fresh_context: true
verdict: SUBSTANTIVE
finding_count: { high: 1, medium: 0, low: 2, nit: 0 }
adr-013_clock_action: reset
clock: 0_of_3
timestamp: 2026-05-07T00:00:00Z
---

═══════════════════════════════════════
[BEGIN]

# Adversary Pass-5 Findings — F2 spec package, v1.0-feature-plugin-async-semantics-pass-1

## Verdict
**SUBSTANTIVE.** Trajectory 19→19→7→6→3 (improving). Clock RESETS to 0/3.

## Counts
HIGH: 1, MEDIUM: 0, LOW: 2, NIT: 0

## Findings

### F-P5-001 [HIGH] — BC-4.04.004 / BC-4.05.004 / BC-4.07.003 / BC-4.08.002 H1 ↔ BC-INDEX title drift (4-BC sibling pattern; pass-3 close-burst claim incorrect)

**Evidence:**
- BC-4.04.004.md line 30 H1: `... once:true and synchronous envelope (async:true removed per ADR-019)` vs BC-INDEX.md line 322: `... once:true; synchronous at envelope (async:true removed per ADR-019)` (`and` vs `;`; `synchronous envelope` vs `synchronous at envelope`)
- BC-4.05.004.md line 30 vs BC-INDEX.md line 327: same drift
- BC-4.07.003.md line 30 vs BC-INDEX.md line 331: drift (`synchronous envelope` vs `synchronous at envelope`)
- BC-4.08.002.md line 30 vs BC-INDEX.md line 334: drift (`synchronous envelope` vs `synchronous at envelope`)

POLICY 7 (`bc_h1_is_title_source_of_truth`) HIGH violation. POLICY 8 compounds — BC-INDEX changelog line 17 (F2 pass-3 fix burst) explicitly claimed `BC-4.04.004/4.05.004/4.07.003/4.08.002 confirmed matching` — confirmation incorrect.

**Adjudication:** BC H1 is canonical per POLICY 7 — BC-INDEX rows must be re-synced to match H1 wording.

### F-P5-002 [LOW] — ARCH-INDEX cites stale BC-INDEX version

ARCH-INDEX.md line 116: `Total BCs: 1,947 (per BC-INDEX v1.22; ...)` but BC-INDEX is now v1.23 (frontmatter line 4; ARCH-INDEX's own changelog at line 20 records the bump in F2 pass-4).

### F-P5-003 [LOW] — ADR-019 §References table missing VP-079

ADR-019 §References (lines 305-316) lists VP-077 and VP-078 with file paths but omits VP-079. VP-079 IS listed in §Implementation Pointers (line 278). Minor parity gap.

## Policy compliance

- POLICY 7 (HIGH): **VIOLATED** — 4 sibling BCs H1↔BC-INDEX drift
- POLICY 8 (HIGH): **degraded** — pass-3 close-burst "confirmed matching" claim was unverified
- All other policies: clean

## Top 3

1. **F-P5-001 (HIGH)** — 4-BC sibling drift on hooks.json.template envelope-sync amendments
2. **F-P5-002 (LOW)** — ARCH-INDEX stale BC-INDEX version cite
3. **F-P5-003 (LOW)** — ADR-019 §References missing VP-079

[END]
═══════════════════════════════════════
