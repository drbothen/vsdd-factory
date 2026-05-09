---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 20
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-08T22:00:00Z
---

# F5 Pass-20 Adversary Review

## Verdict

**HIGH** — 2 H + 1 M + 2 L. Two HIGH findings: (1) L-P19-001 violated by its own codifying fix-burst — fix-burst-18 sub-burst 2 ran the L-P18-002 grep (singular "line") but did not extend it to the plural "lines NNN-NNN" form, leaving 4+ unmigrated prose-form refs in active BC body content (BC-1.14.001:118, BC-5.34.003:43, BC-6.11.026:36+48). (2) ARCH-INDEX BC-INDEX cite is 10 versions stale (v1.33 in body line 130 vs current v1.43) — the codified rule "any BC-INDEX version bump triggers ARCH-INDEX cite refresh in same burst" (ARCH-INDEX v1.18 amendment) was not applied across 15 consecutive fix-bursts (4 through 18).

ADR-013 clock: **0_of_3 → 0_of_3** (RESETS).

## Trajectory

17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4→HIGH(P18)→HIGH(P19)→**HIGH(P20)**.

Recurrence pattern continues: pass-N codifies a discipline lesson; fix-burst-N closes the literal finding but the codified rule does not propagate to sibling instances; pass-(N+1) finds the very pattern the lesson was meant to prevent. Pass-19's L-P19-001 was an explicit attempt to break this loop. Fix-burst-18 violated L-P19-001 in its very first opportunity to apply it.

## Findings

### F-P20-001 [HIGH] L-P19-001 self-violated — corpus-wide sweep used L-P18-002 literal pattern, missing plural-form line refs

**Category:** Codified-lesson-not-applied recurrence (S-7.01 partial-fix discipline).

**Evidence:**
- Surviving prose-form line references in active body content:
  - `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md:118` — §Architecture Anchors: `Kani proof harnesses at lines 148-224`
  - `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.34.003.md:43` — §Invariants: `defined separately at lines 575-731`
  - `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-06/BC-6.11.026.md:36` — §Description: `Threshold table at lines 110-113`
  - `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-06/BC-6.11.026.md:48` — §Invariants: `Threshold table at lines 110-113`
- L-P19-001 codified at lessons.md:109-133 mandates corpus-wide retroactive sweep, but fix-burst-18 used the literal singular grep, missing plural/range forms.

**Confidence:** HIGH

**Impact:** Each line range is a stale citation that drifts as code/lobster files evolve.

**Recommended fix:**
1. Extended grep: `\bat lines? [0-9]+(-[0-9]+)?\b|\bbetween lines? [0-9]+ and [0-9]+\b|\blines? [0-9]+-[0-9]+\b`
2. Migrate matches in active BC/ADR/VP body to stable symbol anchors.
3. Update L-P19-001 disposition: amend grep pattern from literal to pattern class.

---

### F-P20-002 [HIGH] ARCH-INDEX BC-INDEX cite is 10 versions stale (v1.33 vs current v1.43) — codified self-rule un-applied across 15 fix-bursts

**Category:** Recurring propagation gap; codified rule not retroactively enforced.

**Evidence:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/ARCH-INDEX.md:130`: `Total BCs: 1,947 (per BC-INDEX v1.33; ...)`
- BC-INDEX is at v1.43.
- ARCH-INDEX v1.18 explicitly codified: "Future burst-close protocol enhancement: any BC-INDEX version bump must trigger ARCH-INDEX cite refresh in same burst." Reinforced in v1.19.
- Last ARCH-INDEX refresh: v1.22 (F5 fix-burst-3); 15 subsequent fix-bursts did not refresh.

**Confidence:** HIGH

**Impact:** ARCH-INDEX index-of-indexes audit-trail integrity is broken. Self-codified rule was systematically ignored across 15 fix-bursts.

**Recommended fix:**
1. ARCH-INDEX v1.22 → v1.23 with body cite v1.33 → v1.43.
2. Cross-check VP-INDEX, STORY-INDEX cites in ARCH-INDEX body.
3. Process-gap: needs hook-based enforcement, not another prose codification.

---

### F-P20-003 [MEDIUM] L-P19-002 retroactive application audit trail incomplete — VP-071 and VP-077 not noted as verified

**Category:** Codified-lesson incomplete retroactive corpus check.

**Evidence:**
- L-P19-002 disposition only audits VP-070; VP-INDEX line 103 enumerates THREE active Kani VPs (VP-070, VP-071, VP-077).
- Independent audit during this review verified VP-071 and VP-077 Kani harnesses are consistent with production code.

**Confidence:** MEDIUM

**Impact:** Audit trail does not document verification of VP-071/VP-077.

**Recommended fix:** Amend L-P19-002 disposition to cite corpus check: VP-070 (assumption tightened); VP-071 (passes_clean threshold matches production at lib.rs:131); VP-077 (n<=4 is tractability bound, no behavior assumption to drift).

---

### F-P20-004 [LOW] Lobster-line `(lines NNN-NNN)` deferral lacks documented decision

**Category:** Audit-trail gap.

**Evidence:** ~20 `(lines NNN-MMM)` lobster-file refs exist (e.g., BC-5.32.032:98, BC-5.32.006:99, BC-5.32.016:102, BC-5.34.008:100). Pass-19 mentioned deferral but no documentation in burst-log/TD register/open-questions.

**Confidence:** LOW (procedural)

**Recommended fix:** Add note to burst-log: "Lobster-file `(lines NNN-NNN)` refs deferred as separate class — lobster files have stable section structure, drift risk LOW." Or file TD entry.

---

### F-P20-005 [LOW] BC-7.06.001:271 contains `main.rs:NNN` form line citations in active Amendment narrative

**Category:** Reader-clarity defect.

**Evidence:** BC-7.06.001:271 reads: `**grep verification (HEAD e5108a2):** RegistryError::AsyncBlockConflict arm at main.rs:139; ...` Inside `## Amendment 2026-05-08 (v1.7 → v1.8)` block.

**Confidence:** LOW

**Recommended fix:** Reword to `**grep verification (HEAD e5108a2 at amendment time):**` to disambiguate point-in-time evidence from stable anchor.

---

## Process-gap findings (tagged for codification follow-up)

- **[process-gap]** L-P19-001 self-violation pattern: codified lesson must use SEMANTIC pattern class, not LITERAL grep.
- **[process-gap]** ARCH-INDEX cite-refresh discipline ignored across 15 fix-bursts. Recommend hook-based enforcement (parallel to validate-stable-anchors). Open follow-up story.
- **[process-gap]** L-P19-002 retroactive scope was VP-070 only; same root cause as F-P20-001 (literal vs class).

## Notable observations

- BC-4.11.001 v1.2 amendment well-formed; Invariant 8 specifies relative form, absolute form, leading-slash discipline. POLICY 7 verified.
- VP-070 v1.2 Kani harness fix logically sound; both locations (lib.rs:597-600, kani_path_matching.rs:277-280) use tightened assumption identically.
- L-P18-001/003/004 applied correctly in fix-bursts 17-18.
- VP-INDEX arithmetic verified: 79 = 40+22+10+1+3+3.
- BC-INDEX total_bcs 1947 = sum of ARCH-INDEX Subsystem Registry counts. Internally consistent.
- TD-031 entry comprehensively updated but ~6 fix-bursts of context concatenated — readability LOW observation stands.
- STATE.md at 188 lines (within budget).
- Kani harness audit (L-P19-002 retroactive verification): VP-071 (validate-per-story-adversary-convergence/src/lib.rs:632 `kani::assume(passes < 3)` matches threshold at line 131); VP-077 (factory-dispatcher/src/partition.rs:156 `kani::assume(n<=4)` is tractability bound). No staleness detected.

## Convergence assessment

**Novelty: HIGH.** Two genuinely new HIGH defects. Three consecutive HIGH passes (P18, P19, P20) — all recurrence-of-codified-lesson patterns.

**Recurrence pattern continues at meta-level:** "Codify lesson in pass-N → apply with literal-pattern scope in fix-burst-N → re-find broader-class instances in pass-(N+1)." Structural fix is enforcement (hook/lint), not codification.

**ADR-013 protocol:** 0_of_3 → **0_of_3 (RESETS).** Fix-burst-19 priorities:
1. F-P20-002 (ARCH-INDEX cite refresh — single-line fix)
2. F-P20-001 (extended grep + migration of 4+ active body refs)
3. F-P20-003 (L-P19-002 disposition amendment)
4. **Strongly recommended:** open follow-up story for hook-based enforcement of ARCH-INDEX cite-refresh discipline + lessons.md retroactive-sweep verification.
