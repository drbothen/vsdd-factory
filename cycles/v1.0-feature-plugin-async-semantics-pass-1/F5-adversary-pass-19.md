---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 19
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-08T20:00:00Z
---

# F5 Pass-19 Adversary Review

## Verdict

**HIGH** — 2 H + 2 M + 1 L. Two HIGH findings: (1) **the very lesson L-P18-002 codified to enforce prose-form sweep discipline was not retroactively applied** — sibling BCs in the active spec corpus contain the exact same prose-form `at line NNN` pattern that F-P18-002 fixed in only 3 of N affected BCs; (2) **VP-070 Kani harness assumption is now unsound vs the implementation introduced by 8b4f697f** — the proof would fail if Kani were executed.

ADR-013 clock: **0_of_3 → 0_of_3** (RESETS).

## Trajectory

17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4→HIGH(P18)→**HIGH(P19)**.

Recurrence pattern: F5 pass-N codifies a sibling-discipline lesson; fix-burst-(N) closes the primary findings for that pass; F5 pass-(N+1) finds the **same lesson un-applied** to siblings the implementer's narrow scope missed. Pass-18 was a sibling-miss (validate-artifact-path was sibling to cc5a016b's validate-stable-anchors fix). Pass-19 finds two new sibling-misses on lessons codified in pass-18 itself.

## Findings

### F-P19-001 [HIGH] L-P18-002 not retroactively applied — prose-form `at line NNN` pattern survives in 4+ active spec files

**Category:** Sibling propagation gap (S-7.01 partial-fix discipline) + lesson-not-applied-retroactively recurrence pattern.

**Evidence:**
- L-P18-002 codified at `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/lessons.md:42-58` — explicitly mandates *"TD-VSDD-091 sweeps MUST include a manual prose-form sweep step"* with the exact grep pattern.
- Fix-burst-17 sub-burst 2 (`fadafca5`) applied this only to BC-1.05.035, BC-1.05.036, BC-2.02.011 — the three BCs flagged in pass-18.
- Surviving prose-form references in spec body text:
  - `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md:267,275,279,281,283,289` — 6 separate `line NNN` references in active Amendment narrative
  - `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md:414` — `at line 204` in active Amendment v1.7→v1.8 body
  - `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-07/BC-7.03.009.md:31,45` — `set +e at line 33` in §Description AND §Postcondition 1
  - `/Users/jmagady/Dev/vsdd-factory/.factory/specs/open-questions.md:113` — `pre-spawn Path::is_file() check at line 152.5` in active OQ-W16-005 body

**Confidence:** HIGH

**Impact:** Future implementer reading any of these BCs sees a stale `at line NNN` reference; cited line drifts as underlying file evolves; reader either follows stale ref (wrong code) or has to manually re-grep. The four affected files are all active-status spec corpus.

**Recommended fix:** Apply L-P18-002 grep pattern across the entire `.factory/specs/` and `.factory/specs/open-questions.md` corpus. For each match in body text (not changelog), migrate the prose-form line ref to a stable symbol/section anchor.

---

### F-P19-002 [HIGH] VP-070 Kani harness assumption now unsound vs 8b4f697f implementation — spec-verification-corpus inconsistency

**Category:** Spec-impl drift on verification artifact.

**Evidence:**
- 8b4f697f (validate-artifact-path lib.rs) introduced absolute-path matching at lib.rs:156-166 (matches_canonical normalization) and lib.rs:412-413 (hook_logic predicate).
- VP-070 Proof 2 (`proof_vp070_non_factory_path_returns_nomatch`) asserts: `kani::assume(!path.starts_with(".factory/"))` then `kani::assert(result == MatchResult::NoMatch)`. After 8b4f697f, an absolute path `/abs/proj/.factory/specs/...` does NOT start with `.factory/` (passes the assume) but DOES match (returns `MatchResult::Block`). The kani::assert WILL FAIL.
- Three locations contain this stale assumption:
  - `crates/hook-plugins/validate-artifact-path/src/lib.rs:593`
  - `crates/hook-plugins/validate-artifact-path/tests/kani_path_matching.rs:271`
  - `.factory/specs/verification-properties/VP-070.md:103`
- TD-031 acknowledges deferral but VP-070.md does NOT carry an `unsound-vs-impl` annotation.

**Confidence:** HIGH

**Impact:** (a) VP-070.md spec is now wrong — states a property the production code does not satisfy. (b) Spec verification corpus is internally inconsistent. (c) The deferral is acknowledged in TD register but not visible in the artifact itself.

**Recommended fix:** Either (a) update kani::assume to also exclude `path.contains("/.factory/")`; (b) split into two proofs; (c) bump VP-070 to v1.1 with §Pending Update annotation citing TD-031. Sync all three locations together.

---

### F-P19-003 [MEDIUM] BC-4.11.001 postconditions silent on absolute-path matching introduced by 8b4f697f

**Category:** Spec-impl drift; new behavior shipped without BC version bump.

**Evidence:**
- BC-4.11.001 frontmatter version still `1.1`. Last_amended `2026-05-07`.
- 8b4f697f introduced explicit absolute-path acceptance via leading-slash discipline.
- BC-4.11.001 Postconditions 2, 7 describe path predicate informally as `tool_input.file_path targets .factory/` — never explicitly distinguishing relative vs absolute.

**Confidence:** MEDIUM

**Impact:** A v1.1 plugin author re-implementing from BC-4.11.001 spec alone may produce a hook that only accepts relative paths and silently bypasses absolute-path Edit/Write — re-introducing the F-P18-001 bug.

**Recommended fix:** Bump BC-4.11.001 v1.1 → v1.2 with Postcondition clarifying both relative and absolute path acceptance with leading-slash discipline.

---

### F-P19-004 [LOW] Test count claim drift — fix-burst-17 burst-log claim "54→58" is approximate

**Category:** Burst-log audit-trail accuracy.

**Confidence:** LOW

**Impact:** Audit trail readers cannot distinguish "58 tests in validate-artifact-path crate total" from "58 tests in lib.rs::tests module" from runner output.

**Recommended fix:** Qualify the count in TD-031 prose. LOW priority — informational only.

---

### F-P19-005 [LOW] Lessons L-P18-001..004 codified but no enforcement mechanism beyond prose

**Category:** Process-gap codification incompleteness.

**Confidence:** LOW (subjective)

**Impact:** L-P18-001..004 are process rules whose enforcement is "implementer remembers". This is the same pattern that caused TD-031 to recur for 3+ passes before getting an enforcement hook.

**Recommended fix:** Open a follow-up story for fix-burst checklist template hardening that incorporates L-P18-001..004.

---

## Process-gap findings (tagged for codification follow-up)

- **[process-gap]** Recurrence-pattern observation: lessons codified in pass-N+1 are not retroactively applied in fix-burst-(N+1). 3rd-order pattern (TD-031 codified pass-15→recurred pass-16; L-P18-002 codified pass-18→un-applied pass-19). Codification without an automatic backfill step is structurally insufficient. Suggested: every `[codified]` lesson tag must trigger a one-time corpus-wide grep for the codified pattern as part of the SAME fix-burst that codifies the lesson.
- **[process-gap]** VP-XX Kani-harness updates lag implementation changes by an unbounded number of cycles. Recommend a generic policy that any production change to a function with a Kani harness MUST update the harness in the same commit OR mark it `#[ignore]` with a TD entry.

## Notable observations

- STATE.md is at 197 lines (within 200-line budget).
- TD-031 entry now spans ~7 fix-bursts of context. Approaching unreadable. Consider splitting.
- BC-INDEX changelog v1.42 entry is well-scoped — names all 7 commits. **L-P18-003 was applied correctly.**
- VP-INDEX v1.28 entry is more terse but consistent. VP-077 row reflects v1.11.
- **L-P18-001 was applied correctly** — sibling-hook predicate sweep was performed thoroughly. Confirmed via grep across all `crates/hook-plugins/*/src/lib.rs`: only validate-stable-anchors and validate-artifact-path use spec-path-matching predicates.

## Convergence assessment

**Novelty: HIGH.** Two genuinely new HIGH defects:
- F-P19-001 is novel — pass-18 found a partial-scope sweep miss; pass-19 finds the codified lesson un-applied retroactively (a meta-recurrence of the very pattern pass-18 codified).
- F-P19-002 is novel — VP-070 Kani harness drift was deferred in TD-031 but the proof itself remains in an unsound state in the spec corpus; pass-18 did not flag this.

**Recurrence pattern continues:** "Codify lesson in pass-N → fail to apply lesson retroactively in fix-burst-(N) → re-find in pass-(N+1)".

**ADR-013 protocol:** 0_of_3 → **0_of_3 (RESETS).** Fix-burst-18 should prioritize F-P19-001 (broad prose-form sweep across full BC corpus) and F-P19-002 (VP-070 harness sync OR explicit deferral annotation).
