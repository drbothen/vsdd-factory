---
pass: 20
verdict: NOT-CLEAN
reviewed_head: a2112e8d
fixes_landed_head: a5068252
novelty: 0.25
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-19.md"
---

## MANDATORY PROVENANCE DISCLOSURE

**This pass-20 review was NOT produced by the adversary agent.** Three consecutive `vsdd-factory:adversary` dispatches (`adv-p20`, `adv-p20b`, `adv-p20c`) each ran and then went idle producing zero output. Attempt 1 was full-scope; attempt 2 was re-scoped with a partial-output guard; attempt 3 was reduced to a single question with the gate code pre-loaded into the prompt. All three failed identically.

This review was therefore **orchestrator-authored**. Per L-EDP1-074 (orchestrator-authored class predicates), this is permitted as a recovery measure.

**BC-5.39.001 consequence:** This does NOT satisfy the fresh-context / information-asymmetry requirement. The orchestrator had full visibility of fix history and prior findings. **Streak remains 0/3.** Pass-20 MUST NOT be counted toward a CLEAN streak under any reading.

**Open blocker:** Adversary-agent output delivery is a confirmed OPEN BLOCKER for the cascade. Remediation anchor: adversary delivery failure must be diagnosed before cascade can proceed normally. See new lesson `L-BB-adversary-agent-delivery-silent`.

---

## Summary

Pass-20 orchestrator-authored review of S-21.04 at `reviewed_head a2112e8d` (pre-fix, post-pass-19 bats count-word fix). **3 findings: B2 / H0 / M1.** Trajectory 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12→3. Streak: **0/3** (NOT-CLEAN resets per BC-5.39.001; provenance deviation noted above).

Baseline by literal execution at HEAD: `bats story-worktree-write-path-discipline.bats` → `1..9`, 9/9 ok; `bats worktree-identity-preflight.bats` → `1..14`, 14/14 ok. (Both suites confirmed passing at `a2112e8d`.)

**This is the tenth-generation recurrence of the invertible-primary-postcondition class — a RE-SEED of F-S2104-P19-001.** Two regressions from the pass-19 fix composed to produce a new evasion vector M-P20-A that evades BOTH Gate PW-B and the write-directive gate simultaneously.

Mechanism:
1. F-S2104-P19-001 named BOTH `Gate PW-B` (`polarity_violations`) and the `write-directive gate` as targets for clause-scoping. The fix clause-scoped only the write-directive gate; PW-B was left sentence-scoped.
2. The same burst removed `artifact` from the write-directive referent trigger (`grep -E '\.factory/|ledger'`) to silence two pristine false positives, removing the backstop coverage that would have caught clause 1 of M-P20-A independently.

MUTANT `M-P20-A` (evaded BOTH gates at `a2112e8d`):
```
Writers MUST anchor every artifact write to the story worktree CWD; duplicating the ledger onto the main checkout is forbidden.
```

Clause decomposition the write-directive gate sees after clause-split on `[;—]`:
```
1  Writers MUST anchor every artifact write to the story worktree CWD
2  duplicating the ledger onto the main checkout is forbidden.
```

Clause 1 inverts BC-6.26.001 PC1. Gate PW-B matched `worktree CWD` on the whole sentence then `grep -Ev 'forbidden'` excluded all of it (sentence-scoped). Write-directive dropped clause 1 at the referent filter (`artifact` removed from `\.factory/|ledger`) and clause 2 at the directive filter.

**Orchestrator-executed gate evidence at `a2112e8d` (pre-fix), verbatim:**
```
MUTANT   PW-B=SILENT(GREEN)  WD=SILENT(GREEN)   <- evades BOTH gates
CTRL-1   PW-B=FIRES(RED)     WD=SILENT(GREEN)   <- 'forbidden'->'discouraged'
CTRL-2   PW-B=SILENT(GREEN)  WD=FIRES(RED)      <- 'artifact'->'.factory/'
```

Controls prove each evasion vector independently load-bearing. CTRL-1 confirms PW-B sentence-scope (one word change restores coverage). CTRL-2 confirms referent-narrowing effect (restoring `artifact` in the referent pattern makes the write-directive gate fire on clause 1).

---

## Part A — Findings

| ID | Severity | Location | Description | Refs |
|----|----------|----------|-------------|------|
| F-S2104-P20-001 | BLOCKER | `story-worktree-write-path-discipline.bats` Gate PW-B (`polarity_violations`) | Gate PW-B clause-scoping gap: pass-19 F-S2104-P19-001 named both Gate PW-B and the write-directive gate as whole-sentence-escaped. The fix applied clause-scoping only to the write-directive gate. PW-B retained its sentence-scoped prohibition-token exclusion (`grep -Ev 'forbidden&#124;...'`), so any prohibition token anywhere in a sentence continues to exempt every clause of it. M-P20-A clause 1 (`Writers MUST anchor every artifact write to the story worktree CWD`) is a verbatim BC-6.26.001 PC1 inversion; a trailing `;`-clause with `forbidden` renders it sentence-scoped GREEN at PW-B. | BC-6.26.001 PC1; POLICY 11, 13, 15; F-S2104-P19-001 RE-SEED; TD-VSDD-059 |
| F-S2104-P20-002 | BLOCKER | `story-worktree-write-path-discipline.bats` write-directive gate (`write_directive_violations`) | Write-directive referent narrowed: the pass-19 fix removed `artifact` from the referent predicate (pattern `\.factory/&#124;ledger`, dropping the prior `artifact[[:space:]]+writes?` class) to silence two pristine false positives in `_shared-context.md`. This removed the backstop that would have caught M-P20-A clause 1 independently of PW-B. The two regressions (F-S2104-P20-001 + F-S2104-P20-002) composed: M-P20-A evades both gates simultaneously. CTRL-2 proves referent-narrowing is load-bearing: restoring `artifact` makes the write-directive gate fire on clause 1 at `a2112e8d`. | BC-6.26.001 PC1; POLICY 11; F-S2104-P19-003 regression; TD-VSDD-059 |
| F-S2104-P20-003 | MEDIUM | `story-worktree-write-path-discipline.bats` Gate PW-B | PW-B has no directive requirement: PW-B fires on any prohibited-target form lacking a prohibition token, regardless of whether the clause is a mandate or explanatory prose. Clause-scoping (Leg A fix for P20-001) exposes a genuine false positive: prose such as `such writes land silently in the story worktree's shadow .factory/ subtree` — which describes the consequence of forbidden behavior rather than commanding it — fires PW-B after clause-split. The pass-20 remedy rewrote `_shared-context.md` to dodge the specific pattern, but this is a syntactic workaround, not a structural fix. Future explanatory prose mentioning the prohibited-target pattern will false-positive again. **Structural fix risk:** adding a directive/mandate requirement to PW-B NARROWS the gate and could reopen mutants from generations 1–9 (M-P16-*, M-P17-*, M-P18-*, M-P19-*) for which PW-B is the primary catcher. Any such change REQUIRES re-verifying every prior mutant still RED. **Deliberately deferred — anchor as pass-21 lead item.** | BC-6.26.001 PC1; POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION; TD-VSDD-059 |

---

### F-S2104-P20-001 — BLOCKER — Gate PW-B left sentence-scoped after pass-19 clause-scoping fix

The pass-19 closure burst applied clause-scoping to the **write-directive gate** (adding `perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g'` between sentence-split and the violation grep). F-S2104-P19-001 explicitly named both PW-B and the write-directive gate as requiring clause-scoped escapes. The fix applied to only one.

**Zero-degrees-of-freedom fix predicate (orchestrator):** Apply the same clause splitter to Gate PW-B: after the sentence-splitter produces `$polarity_violations_prose`, further split each sentence on `[;—]` and `,\s+(?:and|or|but)\s+` before running the `grep -E 'worktree CWD|...'` probe and the `grep -Ev 'forbidden|...'` escape. The `_shared-context.md` prerequisite: the em-dash form `FORBIDDEN — such writes land...` in S2 must be reworded to avoid the prohibited-target class triggering PW-B on its description-of-consequence clause after clause-split.

I verified the targeted form at `a2112e8d`:
- M-P20-A → PW-B=SILENT (sentence-scoped escape via `forbidden` in clause 2)
- CTRL-1 (`forbidden`→`discouraged`) → PW-B=FIRES (prohibition token removed)
- This confirms PW-B sentence-scope is the mechanism.

---

### F-S2104-P20-002 — BLOCKER — Write-directive referent narrowed, removing backstop coverage

The pass-19 fix replaced `grep -E 'anchor|write|writes'` (action-word list) with `grep -E '\.factory/|ledger'` (referent predicate). During the same burst, `artifact` was removed from the referent predicate to silence two pristine false positives in `_shared-context.md`. The form `artifact writes?` would have matched clause 1 of M-P20-A (`artifact write to the story worktree CWD`) and fired the write-directive gate independently of PW-B.

**Zero-degrees-of-freedom fix predicate (orchestrator):** Extend the referent predicate to `\.factory/|ledger|artifact[[:space:]]+writes?`. This re-adds the `artifact writes?` class (which is the specific phrasing used in harmful mandates of this generation) while preserving the false-positive fix for the two `_shared-context.md` sites (which used `artifact write` in the context of describing consequences, not issuing mandates — those sites are now reworded by the doc prerequisite).

I verified at `a2112e8d`:
- CTRL-2 (`artifact`→`.factory/` in referent pattern) → WD=FIRES on clause 1
- This confirms referent narrowing is the mechanism.

---

### F-S2104-P20-003 — MEDIUM — PW-B has no directive requirement, fires on explanatory prose (OPEN — pass-21 lead item)

After clause-scoping (Leg A), the prohibited-target match in PW-B applies per-clause without verifying whether the clause is a mandate. Purely descriptive prose that mentions a prohibited target without a prohibition token fires the gate. Example:

```
$ run_pwb "<S2 prose: 'such writes land silently in the story worktree's shadow .factory/ subtree and are permanently destroyed at teardown.'>"
such writes land silently in the story worktree's shadow .factory/ subtree and are permanently destroyed at teardown.
```

This clause describes a consequence of forbidden behavior — it is not a mandate. PW-B's prohibition-token escape handles the same pattern in the write-directive gate (which requires a directive token AND a referent match, so explanatory prose without a directive verb is invisible to it), but PW-B has no equivalent directive filter.

**The pass-20 remedy:** Reworded `_shared-context.md` S2 to use `shadow .factory/ subtree of the story worktree` instead of `worktree's shadow .factory/ subtree`, avoiding the PW-B trigger pattern. Semantics unchanged.

**Why this is OPEN / deferred:** Adding a directive requirement to PW-B is the correct structural fix, but it NARROWS the gate. PW-B is the primary catcher for mutant classes M-P16-A, M-P17-A/C, M-P18-C, and others. A narrowed PW-B must be re-verified against every prior mutant to confirm none escape. This is a full regression battery run, not a trivial change. The pass-20 doc workaround is safe; the structural fix is the pass-21 lead item.

---

## Pass-19 finding closure verification

All 12 F-S2104-P19-001..012 findings verified CLOSED at `a2112e8d`:

| Finding | Status at a2112e8d |
|---------|-------------------|
| F-S2104-P19-001 | CONFIRMED-CLOSED (write-directive clause-scoped) — PW-B clause-scoping gap opened as F-S2104-P20-001 |
| F-S2104-P19-002 | CONFIRMED-CLOSED (write-directive domain widened to spec_path_prose_nosplit) |
| F-S2104-P19-003 | CONFIRMED-CLOSED-with-regression (referent predicate lands; `artifact` removal creates P20-002) |
| F-S2104-P19-004 | CONFIRMED-CLOSED (boundary-completeness assertion present) |
| F-S2104-P19-005 | CONFIRMED-CLOSED (link-ref-def strip predicate fixed) |
| F-S2104-P19-006 | CONFIRMED-CLOSED (canonical-target widened to **Correct:** bullets) |
| F-S2104-P19-007 | CONFIRMED-CLOSED (scope-restriction gate present; 2b(a) widened) |
| F-S2104-P19-008 | CONFIRMED-CLOSED (bats lead-in count-words Twenty-one; story v1.25) |
| F-S2104-P19-009 | CONFIRMED-CLOSED (STORY-INDEX three-way equality) |
| F-S2104-P19-010 | CONFIRMED-CLOSED (NAME-SET EQUALITY THREE, 21 gates) |
| F-S2104-P19-011 | CONFIRMED-CLOSED (balanced-fence well-formedness rationale corrected) |
| F-S2104-P19-012 | CONFIRMED-CLOSED (escape-discrimination controls in red-gate-log) |

**Overall:** 10 CONFIRMED-CLOSED / 2 CONFIRMED-CLOSED-with-regression (P19-001 PW-B scope not applied → P20-001; P19-003 `artifact` removed → P20-002). No regression of F-S2104-P19-002/004/005/006/007/008/009/010/011/012 axes.

---

## Fixes applied (orchestrator, post-review)

Fix landed at `a5068252` (test-writer commit):

- **Leg A (F-S2104-P20-001):** Gate PW-B clause-scoped. Added the same `perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g'` splitter to PW-B between sentence-split and prohibited-target grep.
- **Doc prerequisite (F-S2104-P20-001(b)):** `_shared-context.md` S2 prose reworded: `worktree's shadow .factory/ subtree` → `shadow .factory/ subtree of the story worktree`. Semantics unchanged; avoids false positive from clause-split on em-dash.
- **Leg B (F-S2104-P20-002):** Referent predicate extended to `\.factory/|ledger|artifact[[:space:]]+writes?`.

**Orchestrator-executed verification at `a5068252`, verbatim:**
```
M-P20-A (was evading)    PW-B=FIRES(RED)     WD=FIRES(RED)
CONTROL-1                PW-B=FIRES(RED)     WD=FIRES(RED)
CONTROL-2                PW-B=FIRES(RED)     WD=FIRES(RED)
```

Both suites green at `a5068252`: `story-worktree-write-path-discipline.bats` 9/9; `worktree-identity-preflight.bats` 14/14.

F-S2104-P20-003 NOT fixed this pass (deliberately deferred as pass-21 lead item per structural risk noted above).

---

fixes_landed_head: a5068252

## Fix Mapping — Pass-20 (F-S2104-P20-001..003)

| Finding | Fix | Artifact(s) | Status |
|---------|-----|-------------|--------|
| F-S2104-P20-001 | Gate PW-B clause-scoped (same perl splitter as write-directive gate; `[;—]` + `,\s+(and&#124;or&#124;but)\s+`); `_shared-context.md` S2 prose reworded to avoid false positive from clause-split | bats (a5068252), `_shared-context.md` (a5068252) | CLOSED |
| F-S2104-P20-002 | Referent predicate extended: `\.factory/&#124;ledger&#124;artifact[[:space:]]+writes?` | bats (a5068252) | CLOSED |
| F-S2104-P20-003 | No fix this pass — deliberately deferred as pass-21 lead item; structural fix (add directive requirement to PW-B) requires full regression battery of M-P16-* through M-P19-* mutants | — | **OPEN** |
