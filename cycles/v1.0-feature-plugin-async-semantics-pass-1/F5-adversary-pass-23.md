---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 23
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T00:00:00Z
strategic_recommendation: halt-and-implement-S-15.03
---

# F5 Pass-23 Adversary Review

## Verdict

**HIGH — sixth consecutive HIGH pass, four substantive findings (3H + 1M).**

Fix-burst-21 was the broadest sweep yet, but pass-23 finds THREE NEW sibling-class recurrence instances at different layers — confirming L-P22-001's structural-non-convergence prediction:
1. F-P23-001 [H]: Within-subsystem syntactic variant — 27 ss-05 BCs with `Step \`\`<step>\` (line N)` postcondition form unswept
2. F-P23-002 [H]: Cross-subsystem scope — BC-6.04.027 (ss-06) `red-flag table (line 27)` outside ss-05
3. F-P23-003 [H]: Within-file multi-cite-site — BC-1.07.005/006 H1 titles + BC-INDEX rows still cite fabricated symbols
4. F-P23-004 [M]: L-P21-001 disposition audit gap

**STRATEGIC RECOMMENDATION: HALT-AND-IMPLEMENT-S-15.03.** Per L-P22-001 process directive — orchestrator MUST surface decision to user before pass-24.

## Trajectory

17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4→HIGH(P18)→HIGH(P19)→HIGH(P20)→HIGH(P21)→HIGH(P22)→**HIGH(P23)**

**SIX CONSECUTIVE HIGH PASSES.** Recurrence pattern confirmed structural; empirical prior on next prose-only pass finding sibling-class instance: ~96%.

## Findings

### F-P23-001 [HIGH] Lobster-line-cite class — 27 ss-05 BCs with postcondition-form `Step \`\`<step>\` (line N)` were not swept by F-P22-001

**Confidence:** HIGH

**Evidence:**
- F-P22-001 sweep grep targeted description-form `Step \`X\` (line N). ... Source M-K.` → 88 BCs swept.
- SECOND variant exists in 27 sibling ss-05 BCs: postcondition-form `Step \`\`<step>\` (line N) — ...`. Grep:
  `grep -rEn 'Step \`\`[^\`]+\` \(line [0-9]+\)' .factory/specs/behavioral-contracts/ss-05/`
  returns 27 matches.
- BC-5.24.003.md:42 — `Step \`\`holdout-gate\` (line 28) — gate type, depends \`[holdout-evaluation]\`. Source 28-38.\`...`. v1.1, NO carve-out.
- 26 additional sibling files: BC-5.21.019, BC-5.21.009, BC-5.27.003, BC-5.22.003, BC-5.29.002, BC-5.20.013, BC-5.30.003, BC-5.25.002, BC-5.30.002, BC-5.20.017, BC-5.22.002, BC-5.20.015, BC-5.27.002, BC-5.29.003, BC-5.28.003, BC-5.26.002, BC-5.25.003, BC-5.28.002, BC-5.26.003, BC-5.21.015, BC-5.24.002, BC-5.21.018, BC-5.21.011, BC-5.23.002, BC-5.21.007, BC-5.21.013.

**Impact:** L-P19-001 + L-P20-001 + L-P22-001 violation. Same-burst sweep missed variant pattern.

---

### F-P23-002 [HIGH] Lobster-line-cite class — sibling-class miss OUTSIDE ss-05 (BC-6.04.027 / ss-06)

**Confidence:** HIGH

**Evidence:**
- F-P22-001 sweep was scoped to `ss-05/` per BC-INDEX changelog v1.46.
- BC-6.04.027.md:36 — `... explicit "0.84 fails. No rounding." in red-flag table (line 27).` (in §Description, active body)
- Same file :48 — `1. 0.85 / 0.60 thresholds; explicit "0.84 fails. No rounding." in red-flag table (line 27).` (in §Invariants, active body)
- File version: v1.1, no carve-out comment.

**Impact:** Sweep scoped one directory too narrow. Per-subsystem instead of per-corpus. Sibling-class miss at NEW layer.

---

### F-P23-003 [HIGH] L-P21-001 retroactive fix on BC-1.07.005/006 patched §Source Evidence only — H1 title and BC-INDEX rows still cite fabricated symbols

**Confidence:** HIGH

**Evidence:**
- BC-1.07.005.md line 27 (H1): `# Behavioral Contract BC-1.07.005: factory-dispatcher::loads_legacy_registry::every_entry_routes_through_legacy_bash_adapter — ...` → fabricated.
- BC-1.07.006.md line 27 (H1): `# Behavioral Contract BC-1.07.006: factory-dispatcher::loads_legacy_registry::every_entry_carries_a_script_path — ...` → fabricated.
- BC-INDEX.md line 226 + 227: same fabricated symbols cited.
- `grep -n "fn every_entry_routes_through_legacy_bash_adapter|fn every_entry_carries_a_script_path" crates/` → 0 matches.
- Actual fn: `loads_generated_registry_from_disk` at `crates/factory-dispatcher/tests/loads_legacy_registry.rs:34`.

**Impact:** Violates L-P21-001 itself: "every cited production symbol MUST be grep-verifiable." H1 + BC-INDEX cites fail this. Also POLICY 1 BC-H1↔BC-INDEX-title sync violation. Textbook L-P22-001 outcome (fix bounded by author's grep query — only Source Evidence patched, not H1 or index).

---

### F-P23-004 [MEDIUM] L-P21-001 lesson disposition narrative does not record fix-burst-21 retroactive sweep

**Evidence:**
- lessons.md:229 — L-P21-001 closing: `[codified] — fix-burst-20 sub-burst 2.`
- No mention of fix-burst-21 retroactive sweep that found and corrected 7 additional fabrications.
- L-P19-002 has `**Verified retroactively in fix-burst-18 + fix-burst-19:**` block; L-P21-001 lacks this.

**Impact:** Audit trail for L-P21-001 retroactive verification is split between BC-INDEX changelog and TD-031.

**Recommended fix:** Append to L-P21-001: `**Verified retroactively in fix-burst-21 sub-burst 2:**` block listing the 7 additional fabrications corrected.

---

## Process-gap findings

### O-P23-001 [process-gap] Sixth consecutive HIGH from L-P22-001 sibling-class recurrence — strategic halt criterion satisfied

L-P22-001 process directive: "if pass-23 also produces HIGH from a sibling-class recurrence, the F5 chain MUST halt."

Pass-23 produces THREE HIGHs all from sibling-class recurrence:
- F-P23-001: same drift class, same subsystem, different syntactic shape
- F-P23-002: same drift class, different subsystem
- F-P23-003: same L-P21-001 rule applied to different cite location class

**HALT-AND-IMPLEMENT-S-15.03.**

### O-P23-002 [process-gap] L-P21-001 fix-completion criterion missing "all cite sites" requirement

L-P21-001 reads: "every cited production symbol MUST be grep-verifiable." Does NOT specify "every cite site within a file MUST be fixed when one is fixed." F-P23-003 demonstrates this gap.

If S-15.03 ships, the `validate-symbol-cite` hook should require ALL cite sites of a fabricated symbol to be patched.

### O-P23-003 [process-gap] STATE.md size at 195 lines — close to 200-line budget; flag for next state-manager run

## Notable observations

1. F-P22-002 BC-1.14.001 fix verified clean — all 4 symbols grep-verified.
2. F-P22-003/004/005 E-12 fixes verified clean.
3. L-P21-001 independent symbol-cite sample (20 cites): 100% TP. Per-cite fabrication rate is converging; remaining drift is per-FILE multi-cite-site coverage (F-P23-003).
4. L-P21-002 retroactive on non-E-12 epics (sample 5 stories): all PASS.
5. Index frontmatter alignment correct: BC-INDEX v1.46 / VP-INDEX v1.30 / STORY-INDEX v2.52 / ARCH-INDEX v1.26.
6. ARCH-INDEX BC-INDEX cite refresh applied (line 138).
7. L-P22-001 strategic-vs-discipline classification: strategic process observation; no corpus sweep applies.
8. TD-031 fix-burst-21 follow-up entry recorded.
9. **Lobster-line-cite class scope is now fully characterized:** at least 4 syntactic variants —
    (a) `lines N-K` range form (swept fix-burst-19/20)
    (b) singular `Step \`X\` (line N). ... Source N-K.` description form (swept fix-burst-21 sub-burst 1)
    (c) backtick-wrapped `Step \`\`<step>\` (line N) — ... \` postcondition form (UNSWEPT, F-P23-001)
    (d) bare `(line N)` cite outside ss-05 lobster files (UNSWEPT, F-P23-002)

Only mechanical enforcement (S-15.03) can close the recurrence.

## Convergence assessment

**Novelty: HIGH.** All 4 findings genuinely new at different recurrence layers.

**ADR-013 clock: 0_of_3** (RESET — pass-23 HIGH).

**Pass count:** 23 adversary passes total, 21 fix-bursts dispatched. SIX consecutive HIGH passes (P18..P23). Fix-burst-21 was the broadest yet — pass-23 STILL finds three NEW sibling-class instances on three different layers.

**Recurrence pattern is empirically irreducible by prose discipline alone.** Pass-23 confirms L-P22-001's structural-non-convergence prediction.

## Strategic recommendation

**HALT prose-only fix-bursts and implement S-15.03 (mechanical hook enforcement).**

Per L-P22-001 process directive: pass-23 satisfies the halt criterion. Three HIGH findings, each from sibling-class recurrence at different layers:

| Finding | Recurrence layer |
|---------|------------------|
| F-P23-001 | Within-subsystem syntactic variant |
| F-P23-002 | Cross-subsystem scope |
| F-P23-003 | Within-file multi-cite-site coverage |

Each layer would require a different grep query to catch in prose-only sweep. The combinatorial layer space is unbounded; S-15.03's `validate-index-cite-refresh` + `validate-lesson-retroactive-sweep` + (proposed extension) `validate-symbol-cite` hooks address all three with mechanical enforcement.

**Expected halt-and-implement outcome:**
- S-15.03 implementation absorbs F-P23-001/002/003/004 closure as part of hook validation.
- F5 cycle resumes at pass-24 with hook gate active. ADR-013 clock starts fresh.
- Probability of pass-24 NITPICK_ONLY post-S-15.03: substantially higher.
- Probability of pass-24 HIGH if continuing prose-only: ~96% (six-pass empirical prior).

**Orchestrator MUST surface this halt-decision to the user before any pass-24 dispatch.**
