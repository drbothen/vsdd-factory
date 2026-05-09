---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 22
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T00:00:00Z
---

# F5 Pass-22 Adversary Review

## Verdict

**HIGH** — five substantive findings. Lobster-line-cite sweep in fix-burst-20 covered 10 BCs but the same semantic pattern (lobster file + line citation in active body) exists in **88 sibling BCs in ss-05 alone**, none touched. Two POLICY-4 audit-trail integrity violations remain in BC-1.14.001 (cycle anchor BC) — 3 fabricated production symbols. Sibling-row swap drift in E-12 Stories Planned table for S-12.03/S-12.06.

**Fifth consecutive HIGH pass.** Strategic recommendation strengthens from "consider halt" (P21) to **"MUST halt-and-implement S-15.03 mechanical enforcement"** (P22). Continuing prose-only is empirically not converging.

## Trajectory

17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4→HIGH(P18)→HIGH(P19)→HIGH(P20)→HIGH(P21)→**HIGH(P22)**

**Five-pass HIGH streak.** Each fix-burst codified a slightly broader rule but still missed the next sibling-class. Pattern is structural, not author-error.

## Findings

### F-P22-001 [HIGH] Lobster-line-cite class sweep applied to 10 BCs missed 88 sibling BCs in ss-05

**Confidence:** HIGH

**Evidence:**
- Fix-burst-20 sub-burst 1 (823468ce) added carve-out + v1.2 bump to 10 BCs citing range form (`lines N-K`).
- `grep` for singular form `Step \`X\` (line N)... Source M-K` returns **88 file matches** in ss-05/, all in active §Description bodies.
- Examples (all unfixed at v1.1, no carve-out):
  - BC-5.31.018.md:31 — `Step \`create-pr\` (line 254). ... Source 254-266.`
  - BC-5.31.006.md:31 — `Step \`create-worktree\` (line 38). ... Source 38-46.` (BC-5.31.002 cites SAME line 38, IS in v1.2 sweep — sibling missed)
  - BC-5.34.009.md:31, BC-5.34.008.md:31, BC-5.34.007.md:31, BC-5.34.006.md:31 — multi-repo.lobster step BCs in same subsystem as BC-5.34.004 (F-P21-001 primary)
- **Textbook L-P20-001 violation**: range-form grep used; singular-form grep missed.

**Impact:** POLICY 4 + L-P19-001 + L-P20-001. Per S-7.01: 88 sibling files = HIGH. Fifth consecutive recurrence.

**Recommended fix:** Apply carve-out + v1.2 bump to all 88 sibling BCs in same fix-burst. Grep:
`grep -rn -E "(Step|Steps?) \`[^\`]+\`( |) \(line [0-9]+\)|Source [0-9]+-[0-9]+\." .factory/specs/behavioral-contracts/`

---

### F-P22-002 [HIGH] BC-1.14.001 §Architecture Anchors cites three fabricated production symbols

**Confidence:** HIGH

**Evidence:**
- BC-1.14.001.md:117 cites `RegistryEntry.async field`. `async` is a Rust keyword — cannot be a field name. Actual field: `async_flag` at `crates/factory-dispatcher/src/registry.rs:245`.
- BC-1.14.001.md:119 cites:
  - `crates/factory-dispatcher/src/engine.rs (or equivalent dispatch loop)` — `engine.rs` does not exist; actual file: `executor.rs`.
  - `sync group \`run_tiers()\`` — 0 grep matches. Actual: `execute_tiers` at `executor.rs:79`.
  - `async group \`spawn_detached()\`` — 0 grep matches. Actual: `spawn_async_plugin` at `executor.rs:247`.

**Impact:** POLICY 4 violation in **CYCLE ANCHOR BC** (BC-1.14.001 IS the dispatcher partition contract). L-P21-001 was just codified prescribing "every cited symbol MUST be grep-verifiable" — first L-P21-001 retroactive-sweep failure.

**Recommended fix:**
- `RegistryEntry.async field` → `RegistryEntry.async_flag field`
- `engine.rs (or equivalent...) — sync group \`run_tiers()\` + async group \`spawn_detached()\`` → `executor.rs — sync group via \`execute_tiers()\` + async group via \`spawn_async_plugin()\``
- BC-1.14.001 v1.10 → v1.11; cite-refresh BC-INDEX → ARCH-INDEX per L-P20-002.
- **Run the L-P21-001 retroactive sweep** that was supposed to ship with the lesson — corpus-wide grep every `<file>::<symbol>` cite against codebase.

---

### F-P22-003 [HIGH] E-12 Stories Planned table swaps subsystems for S-12.03 and S-12.06

**Confidence:** HIGH

**Evidence:**
- E-12-engine-governance.md:104 row: `S-12.06 ... SS-01`. Story declares `subsystems: ["SS-04"]` + `target_module: crates/hook-sdk` (SS-04).
- E-12-engine-governance.md:105 row: `S-12.03 ... SS-04`. Story declares `subsystems: ["SS-01"]` + `target_module: crates/factory-dispatcher` (SS-01).
- Two rows are SWAPPED. Story frontmatter is correct; epic table is wrong.

**Impact:** POLICY 6 + L-P21-002 (story epic anchor sync — bidirectional). S-15.03 added in fix-burst-20 at line 110 — sibling rows not audited.

**Recommended fix:** Swap subsystems in E-12 lines 104 + 105. Bump E-12 v1.2→v1.3. Apply L-P21-002 retroactive sweep to ALL 9 stories under E-12.

---

### F-P22-004 [MEDIUM] E-12 frontmatter version stale at "1.0"; body changelog records v1.2

**Evidence:**
- E-12-engine-governance.md:4 frontmatter: `version: "1.0"`.
- Same file line 19: `last_amended: "2026-05-08 (v1.2 — F-P21-003: S-15.03 re-anchored from E-15...)"`.
- Same file line 194 CHANGELOG row: `| v1.2 | 2026-05-08 | ...`.

**Recommended fix:** Bump frontmatter `version: "1.0"` → `version: "1.2"` to match `last_amended`/CHANGELOG.

---

### F-P22-005 [MEDIUM] E-12 frontmatter title stale vs body H1

**Evidence:**
- E-12-engine-governance.md:5 frontmatter: `title: "Engine Governance — Per-Story Adversarial Convergence Discipline"`.
- Same file line 29 H1: `# Epic E-12: Engine Governance — Per-Story Adversarial Convergence Discipline + WASM-Plugin Context Resolver Platform`.

**Recommended fix:** Sync frontmatter `title:` to match H1 exactly.

---

## Process-gap findings

### O-P22-001 [process-gap] Fifth consecutive HIGH from codified-lesson-applied-too-narrowly pattern; prose-only path empirically non-convergent

- fix-burst-17 codified L-P18-002 → P19-001 sibling
- fix-burst-18 codified L-P19-001 → P20-001 sibling (singular vs plural)
- fix-burst-19 codified L-P20-001/002 → P21-001 sibling (range vs single-line)
- fix-burst-20 applied to 10 BCs + codified L-P21-001/002 → P22-001 88 sibling BCs + P22-002 fabricated symbols + P22-003 E-12 swap

Recurrence is structural across **five consecutive fix-bursts**. Empirical prior on next prose-only pass finding sibling-class instance: ~95%. **Strategic recommendation upgraded to MUST halt-and-implement S-15.03.**

### O-P22-002 [process-gap] L-P21-001 codified but no corpus-wide symbol-cite sweep ran in same burst

L-P19-001 explicitly requires lessons.md append be accompanied by corpus sweep. L-P21-001 codification at fix-burst-20 sub-burst 2 should have triggered grep of all `<file>::<symbol>` cites against codebase. F-P22-002's 3 fabricated symbols in cycle anchor BC are direct evidence the sweep was not run.

### O-P22-003 [process-gap] STATE.md size at 192 lines — close to 200-line budget

Compaction needed within 1-2 fix-bursts.

## Notable observations

1. Fix-burst-20 successes verified clean: F-P21-001 primary fix, F-P21-002 lessons.md, F-P21-003 S-15.03 re-anchor, L-P21-001/002 codification, O-P21-002 retroactive correction. All VERIFIED.
2. Index versions match frontmatter: BC-INDEX v1.45, VP-INDEX v1.29, STORY-INDEX v2.51, ARCH-INDEX v1.25.
3. POLICY 6 (subsystem-name fidelity) for S-15.03: SS-01 + SS-04 both present in ARCH-INDEX. VERIFIED.
4. L-P20-002 same-burst cite-refresh: ARCH-INDEX v1.25 changelog notes BC-INDEX v1.44→v1.45 cite refresh in same burst. VERIFIED.
5. lobster-line carve-out scoping: sub-burst 1 applied to discovery, code-delivery, maintenance, planning, multi-repo .lobster files — but only range-form. Singular-form was missed.
6. Five-pass HIGH streak correctly recorded in STATE.md strategic note.

## Convergence assessment

**Novelty: HIGH.** All 5 findings are genuinely new fresh-context grep evidence.

**ADR-013 clock: 0_of_3** (RESET — HIGH).

**Strategic recommendation: MUST halt prose-only fix-bursts and implement S-15.03 mechanical enforcement before pass-23.** Five consecutive HIGH passes establish prose-only path as empirically non-convergent.

If user continues prose-only:
- Fix-burst-21 must close F-P22-001 (88 BCs swept with broad L-P20-001 class grep), F-P22-002 (3 symbol fixes + corpus L-P21-001 sweep), F-P22-003/004/005 (E-12 epic coherence).
- Same burst MUST run L-P21-001 corpus-wide symbol-cite sweep (the sweep that should have shipped at fix-burst-20).
- Probability of pass-23 NITPICK_ONLY: ~5%; expect sixth HIGH cycle on next sibling-class layer.
