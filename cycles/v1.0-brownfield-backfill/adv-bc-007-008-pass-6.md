---
document_type: adversary-review
level: ops
version: "1.0"
status: complete
producer: adversary
verifier: orchestrator
timestamp: 2026-05-20
phase: m3-bc-cascade-pass-6
cycle: v1.0-brownfield-backfill
streak: "1/3"
verdict: NITPICK
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
input-hash: "d314c4e"
traces_to: STATE.md
---

# Adversarial Review — BC-5.39.006 + BC-5.39.007 + BC-5.39.008 Pass-6 (M3 BC Cascade)

## ORCHESTRATOR-VERIFIED OVERRIDES

> These overrides are prepended by the orchestrator ABOVE the adversary's Part A findings.
> They were verified by the orchestrator before this persistence dispatch (per D-449(a)).

### Override 1: Verdict NITPICK — ORCHESTRATOR-VERIFIED

Both findings are documentary-only INV-019 RESIDUAL. No load-bearing impact. NITPICK severity confirmed. **Streak 0/3 → 1/3 advance per BC-5.39.001 3-CLEAN protocol.**

### Override 2: POLICY 14 5-Leg Quintuple Parity Production-Validation

PO commit `c4be5fde` literal-shell verified all 5 legs synced same-burst for all 3 BCs. **INV-020 codification (D-490) practically viable in production.** No regression of pass-5 HIGH cross-file propagation class.

### Override 3: F-BC007P5-001 Full BC-006-Parity Sweep Correctness

~46 bare-Block → assoc-fn conversions across BC-007/008 body. 10+ samples reviewed; **NO conversion defects.** Semantic preservation confirmed.

### Override 4: NO PO FIX-BURST REQUIRED

Per BC-5.39.001 3-CLEAN protocol, NITPICK findings advance the streak without requiring a fix-burst. Documentary cleanup is deferred to OPTIONAL future BC-INDEX bump per POLICY 1 append-only.

### Override 5: Net Status

- 0 CRITICAL / 0 HIGH / 0 MEDIUM / 0 LOW / 2 NITPICK
- STREAK 0/3 → 1/3 (first advance in the cascade)
- Cascade trajectory: 41 → 14 → 8 → 3 → 5 → 2 NIT (steep decay restored)
- CRITICAL=0 sustained 5 passes; HIGH=0 RESTORED
- Next: adversary pass-7 dispatch (target CLEAN; 1/3 → 2/3; need 3-CLEAN for convergence)

## PART A — Adversary Findings

### Finding Counts (pass-6)

| Pass | BC-006 | BC-007 | BC-008 | Total | Streak |
|------|--------|--------|--------|-------|--------|
| Pass-1 | ~0 | ~21 | ~20 | ~41 | 0/3 |
| Pass-2 | 1 (HIGH) | 7 | 6 | 14 | 0/3 |
| Pass-3 | 3 | 2 | 2 | 8 | 0/3 |
| Pass-4 | 1 (LOW) | 1 (NIT) | 1 (MED) | 3 | 0/3 |
| Pass-5 | 3 (1H+2L) | 1 (LOW pending-intent) | 1 (HIGH shared) | 5 | 0/3 RESET |
| **Pass-6** | **2 (NIT)** | **0** | **0** | **2 NIT** | **1/3 (FIRST ADVANCE)** |

### Verdict: NITPICK (0 CRIT / 0 HIGH / 0 MED / 0 LOW / 2 NIT)

Both findings are INV-019 RESIDUAL (post-commit accounting drift applied to line numbers and counts). POLICY 14 5-leg quintuple parity VALIDATED in production at pass-5 PO fix-burst (`c4be5fde`). F-BC007P5-001 full BC-006-parity sweep adversary-verified (10+ conversions sampled across BC-007/008 EC and TV; NO conversion defects; semantic preservation confirmed). STREAK 0/3 → **1/3 FIRST ADVANCE IN 6-PASS CASCADE**. CRITICAL=0 sustained 5 consecutive passes; HIGH=0 RESTORED.

### Findings

#### F-BC006P6-001 NITPICK — BC-INDEX v2.43 changelog row cites stale body-table row range

- **Policy:** POLICY 15 (LL-N verbatim stdout) — documentary accuracy
- **Scope:** BC-INDEX line 16 (v2.43 changelog row prose) + line 18 (v2.42 row)
- **Defect:** v2.43 row cites `"rows 1233-1235 updated"`; v2.42 row cites `"body table rows 1231-1233 corrected"`. Actual body table rows for BC-5.39.006/007/008 are lines 1235/1236/1237 (off by 2). Hardcoded row-number drift across two consecutive same-day changelog rows. Same META-class as INV-019 (post-commit accounting drift) applied to LINE NUMBERS rather than counts.
- **Evidence (literal shell):**
  ```
  $ grep -nE '^\| \[BC-5\.39\.00[678]\]' .factory/specs/behavioral-contracts/BC-INDEX.md
  1235:| [BC-5.39.006](...) | ... | active | E-12 | S-15.14 | v1.7 |
  1236:| [BC-5.39.007](...) | ... | draft | E-12 | S-15.12 | v1.5 |
  1237:| [BC-5.39.008](...) | ... | draft | E-12 | S-15.15 | v1.5 |
  $ grep -nE 'rows 123[0-9]-123[0-9]' .factory/specs/behavioral-contracts/BC-INDEX.md
  16:    change: "v2.43 (... Body table leg-5: rows 1233-1235 updated ..."
  18:    change: "v2.42 (... body table rows 1231-1233 corrected ..."
  ```
- **INV-019 cure:** (a) line-range-exclude (this finding cites a different file than BC-INDEX, so self-match impossible)
- **Severity:** Documentary-only; cite directs reader to wrong location but reader can find correct rows via H1-of-row grep. NOT load-bearing. Routing OPTIONAL.
- **Routing:** OPTIONAL state-manager amend at next BC-INDEX bump (defer-acceptable; documentary-only)

#### F-BC007P6-001 NITPICK — Cross-SoT count narrative inconsistency

- **Policy:** POLICY 15 (LL-N) — documentary accuracy
- **Scope:** BC-007 v1.5 changelog row L471; BC-008 v1.5 row L490; D-491 decision-log row L129; BC-INDEX v2.43 row L16; L-M3-BC-cascade-pass-5-PO-fix-burst lesson L2138
- **Defect:** Five artifacts narrate slightly different counts for F-BC007P5-001 conversions:
  - BC-007 v1.5: "EC 15 + TV 8 + PC9 1 = 24 total" (exact)
  - BC-008 v1.5: "EC 10 + TV 9 = 19 total" (exact)
  - D-491 row: "~23 (BC-007) + ~22 (BC-008) = ~46 total" (approx)
  - BC-INDEX v2.43: "~24 / ~19" (approx)
  - lessons.md: "~23 + ~22 = ~46" (approx)
  Direct count: BC-008 EC has 9 (not 10); TV has 8 (not 9). Approximation-vs-exact form inconsistency. Approximation-form does not violate INV-019 by construction; exact-form invites drift.
- **Evidence (literal shell):**
  ```
  $ grep -cE 'HookResult::block_with_fix\(\.\.\.\)' .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
  27
  $ grep -cE 'HookResult::block_with_fix\(\.\.\.\)' .factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md
  30
  ```
- **INV-019 cure:** (b) inline-acknowledge (counts may drift post-commit; acknowledged in prose)
- **Severity:** Documentary-only; no load-bearing impact. INV-019 forward-application discipline gap (cure (c) by-construction would produce uniform ~tilde across all 5 SoTs).
- **Routing:** OPTIONAL state-manager note at next pass (defer-acceptable; if amended, recommend uniform cure (c) approximations)

### META-LEVEL Analysis

- **NO new INV class emerged.** Both findings are INV-019 RESIDUAL (post-commit accounting drift applied to row numbers and counts). Both NITPICK documentary-only.
- **POLICY 14 5-leg quintuple parity VALIDATED in production by PO commit `c4be5fde`.** All 5 legs synced same-burst for all 3 BCs. INV-020 codification practically viable.
- **F-BC007P5-001 full BC-006-parity sweep correctness:** 10+ conversions sampled across BC-007 EC/TV and BC-008 EC/TV. NO conversion defects. Semantic preservation confirmed; no accidental Block→Continue flips; complex multi-clause semantics (EC-022 multi-advisory) preserved.
- **Streak math:** 0/3 (pass-5 RESET) → **1/3** (pass-6 NITPICK advances per BC-5.39.001)
- **CRITICAL=0 sustained 5 consecutive passes; HIGH=0 RESTORED at pass-6.**

## PART B — Recommendations

1. **Strong CLEAN-CLEAN-CLEAN trajectory available.** Pass-6 NITPICK sufficient to advance streak; two more clean passes converges.
2. Optional documentary cleanup at next BC-INDEX bump (POLICY 1 append-only; cite in NEXT row).
3. Adopt uniform cure (c) by-construction (approximation-tilde) for cross-artifact count narratives.
4. NO new D-NNN codification required; streak advance is mechanical.
