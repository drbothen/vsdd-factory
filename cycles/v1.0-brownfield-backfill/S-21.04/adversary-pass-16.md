---
pass: 16
verdict: NOT-CLEAN
reviewed_head: 8b39277b
novelty: 0.42
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-15.md"
---

## Summary

Pass-16 fresh-context adversarial review of S-21.04 at reviewed_head `8b39277b` (worktree `.worktrees/S-21.04`, base develop `948f0fb1`). **6 findings: B1 / H3 / M1 / L1.** Novelty 0.42 vs pass-15 Part A (F-S2104-P16-003 is a wholly novel mechanism — extraction-anchor hijack, an axis no prior pass has touched; F-S2104-P16-005 targets attestation text first authored at v1.13; F-S2104-P16-004 is a *regression* of a closure pass-15 confirmed; F-S2104-P16-001 and -002 are one-hop re-seedings of the P15-001 and P15-002 classes via novel mechanisms). Trajectory 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6. Streak: **0/3** (BC-5.39.001 reset).

Baseline by literal execution at HEAD: `bats story-worktree-write-path-discipline.bats` → `1..9`, 9/9 ok; `bats worktree-identity-preflight.bats` → `1..14`, 14/14 ok.

The pass-15 wave did honest, substantive work and I independently re-proved **both** of its recorded vectors RED at HEAD from their verbatim Part A text — M-P15-A fires Gate 1(a) at bats line 564, M-P15-B fires Gate 6(a) at bats line 636. The reflow-then-sentence-split normalization is real and the line-rewrap axis is genuinely closed. **The gates are nevertheless defeated by a sixth-generation vector**, and the reason is a shift of axis, not of granularity: pass-15 normalized the *domain* the predicates evaluate over (physical line → reflowed sentence) but left two other properties unconstrained. First, Gate 1(a)'s affirmative predicate `MUST[^.]*use[^.]*canonical[[:space:]]+absolute` matches the string **`MUST NOT use canonical absolute`** — the `[^.]*` wildcard steps straight across the negation token, so the one gate whose stated job is "the mandate must be affirmative" cannot detect a negated mandate. Second, only the mandate sentence is polarity-checked at all; every other sentence in the block is checked for exactly two token conjunctions (absolute+FORBIDDEN, MUST+relative-form), so any harmful instruction phrased outside those two conjunctions passes untouched. Combining the two, I inverted BC-6.26.001 PC1 **end-to-end — the normative paragraph and all three worked examples** — with the full suite reporting 9/9 ok.

Second structural theme, and the third consecutive pass to exhibit it: the *sibling* of the just-hardened gate ships to the pre-hardening standard. Gate 6 now enforces two-part polarity on the traversal bullet, but no gate anywhere asserts the **CWD-relative** bullet is Forbidden — the single most literal rendering of issue #523 in the whole document. Relabelling it `**Correct:**` and widening one parenthetical from "relative traversal" to "relative path traversal" leaves T-001 GREEN.

Third theme, novel: the prohibition-block extractor anchors on the *first* line matching `All.*\.factory.*artifact writes` and stops at the first blank line. That anchor is not asserted unique. Inserting a compliant two-line decoy paragraph earlier in §Spec-Path Discipline captures every prohibition-block gate, after which even the M-P15-A-shaped inversion the pass-15 wave recorded as RED goes GREEN. I ran the no-decoy control to confirm the decoy is the load-bearing element.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P16-001 | BLOCKER | `story-worktree-write-path-discipline.bats` §T-001 prohibition-block Gates 1/2/4/5 vs `_shared-context.md` §Spec-Path Discipline → §Write Discipline normative prohibition paragraph | Sixth-generation polarity paper-gate. Gate 1(a)'s affirmative predicate matches `MUST NOT use canonical absolute` (the `[^.]*` wildcard crosses the negation); only the mandate sentence is polarity-checked; Gate 2 is polarity-blind across the joined block. M-P16-A inverts PC1 across the paragraph **and all three worked examples** at 9/9 ok. M-P16-C2 shows a single rendered sentence can carry both halves by splitting on `cf. ` | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13, 15; TD-VSDD-059 |
| F-S2104-P16-002 | HIGH | `bats` §T-001 Gates 3 + 6 vs `_shared-context.md` §Spec-Path Discipline `**Forbidden:**` example bullets | No gate asserts the **CWD-relative** example bullet is Forbidden. Gate 3's `**Forbidden:**`+`relative path` conjunction is satisfiable by the traversal bullet alone; Gate 6 covers only `../`. M-P16-D relabels the CWD-relative bullet `**Correct:**` at 9/9 ok. One-hop re-seed of the F-S2104-P15-002 class into the sibling bullet of the same AC clause | BC-6.26.001 PC1; POLICY 11, 13, 15 (SAME-AC GATE AUDIT) |
| F-S2104-P16-003 | HIGH | `bats` `_extract_write_discipline_prohibition_block` + `_extract_spec_path_discipline_section` | Extraction anchor `All.*\.factory.*artifact writes` is not asserted unique within §Spec-Path Discipline, and awk takes the first match. A compliant two-line decoy paragraph inserted earlier captures every prohibition-block gate; M-P16-B then leaves the recorded M-P15-A-shaped inversion GREEN (no-decoy control: RED). Novel axis — prior passes hardened section-bounding (P14-005) and domain shape (P15-001), never anchor uniqueness | BC-6.26.001 PC1; POLICY 11, 13, 15 |
| F-S2104-P16-004 | HIGH | story `S-21.04-...md` §Acceptance Criteria row AC-001, Gate column | Gate cell (unchanged at v1.20) enumerates the **retired** pass-14R predicates: Gate 1's negative half as `not canonical absolute\|not absolute` (deleted at `8b39277b`, zero occurrences at HEAD), Gates 4/5 as "per extracted line"/"per line", Gate 6 as presence marker `\.\./\|relative traversal`. Regression of the F-S2104-P14R-006 closure pass-15 confirmed; the cell now also mandates the per-physical-line domain shape POLICY 13's NORMALIZED-DOMAIN MANDATE forbids | POLICY 4, 13, 14/17 (legs 1+4), 5; TD-VSDD-059, TD-VSDD-060 |
| F-S2104-P16-005 | MEDIUM | red-gate-log v1.13 §Pass-15 assertion-site attestation — per-gate audit table closing line, and the `M-P15-A proof (RED)` mutant text | (a) "All gates: independent, polarity-complete, zero degrees of freedom" is falsified by four surviving mutants at the attested HEAD. (b) The text recorded under the label `M-P15-A` is not the adversary's M-P15-A — the decoy sentence is replaced and the inverted `Canonical absolute…are FORBIDDEN` sentence (the element Gate 4 exists to catch) is reverted to the original — so the record is not a re-runnable proof of the vector it names | POLICY 15 (verbatim + exact-substituted-text), 3; TD-VSDD-059; D-448(a) class |
| F-S2104-P16-006 | LOW | `adversary-pass-15.md` §Fix Mapping row F-S2104-P15-005 | Row attests "T-009 gate comment updated to acknowledge that the broader character class was chosen specifically to catch `bcs:` inside backtick code spans". No commit in the pass-15 wave touched the `bcs:` gate or its comment, and the comment at HEAD states a different rationale (compound-identifier false-hit avoidance). The substantive AC-010 leg did land; the record overstates the wave's scope | POLICY 15; TD-VSDD-059 |

---

### F-S2104-P16-001 — BLOCKER — the affirmative-mandate predicate matches a negated mandate; PC1 invertible end-to-end

**Stable anchors.** Gate site: `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`, T-001 (`@test "T-001 S-21.04 AC-003: stray-file-blocks …"`), the block introduced by the comment `# Gate 1: affirmative mandate polarity — sentence-scoped (F-S2104-P15-001 / F-S2104-P14R-001(a))` through `# Gate 5 (NEGATIVE, sentence-scoped; …)` (lines 542-616 at HEAD). Target: `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`, §Spec-Path Discipline → `#### Write Discipline — .factory/** artifact writes from story worktrees`, the normative prohibition paragraph beginning `All \`.factory/**\` artifact writes performed during story delivery` (lines 66-70), plus the three worked-example bullets that close the section (lines 112-114).

The normalization and the four predicates read, verbatim (`bats:540`, `:556-557`, `:562`, `:569`, `:581`, `:593-594`, `:610-612`):

```
  joined_block="$(printf '%s\n' "$prohibition_block" | tr '\n' ' ')"
  mandate_sentence="$(printf '%s\n' "$joined_block" | \
    sed 's/\. /\n/g' | grep 'artifact writes' | head -1)"
  printf '%s\n' "$mandate_sentence" | grep -qE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' || {
  if printf '%s\n' "$mandate_sentence" | grep -qE 'CWD-relative|worktree-relative|relative[[:space:]]+paths?'; then
  printf '%s\n' "$prohibition_block" | tr '\n' ' ' | grep -qE '(CWD-relative|relative path).*FORBIDDEN|FORBIDDEN.*(CWD-relative|relative path)' || {
  forbidden_absolute_sentences="$(printf '%s\n' "$joined_block" | \
    sed 's/\. /\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)' || true)"
  must_relative_sentences="$(printf '%s\n' "$joined_block" | \
    sed 's/\. /\n/g' | grep -E 'MUST' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' || true)"
```

Three independent gaps compose:

1. **Gate 1(a) admits negation.** `MUST[^.]*use[^.]*canonical[[:space:]]+absolute` matches `MUST NOT use canonical absolute` — `[^.]*` excludes only periods, so it spans ` NOT `. The gate's own message asserts "the mandate must be affirmative"; the predicate cannot express that.
2. **Only the mandate sentence is polarity-checked.** Gate 1(b) constrains the `artifact writes` sentence alone. Every other sentence is screened solely for `absolute`+`FORBIDDEN` (Gate 4) and `MUST`+relative-form (Gate 5). A harmful instruction using neither conjunction — "Writers anchor every `.factory/**` write to the story-worktree CWD" — passes all five gates.
3. **Gate 2 is polarity-blind.** It requires `(CWD-relative|relative path)` and `FORBIDDEN` to co-occur anywhere in the *whole joined block*, in any relation. The sentence "CWD-relative paths were formerly FORBIDDEN … that prohibition is retired" satisfies it. Gate 2 is the only gate asserting the prohibition exists, and it is satisfied by text that explicitly retires the prohibition.

**M-P16-A — exact substituted text.** Replaces the §Write Discipline normative prohibition paragraph (HEAD lines 66-70):

```
All `.factory/**` artifact writes performed during story delivery MUST NOT use canonical absolute
paths anchored to the main-checkout root. Writers anchor every `.factory/**` write to the
story-worktree CWD so the worktree's shadow `.factory/` subtree receives the artifact. CWD-relative
paths were formerly FORBIDDEN under the issue #523 reading; that prohibition is retired
(BC-6.26.001 Invariant 5).
```

and replaces the three worked-example bullets (HEAD lines 112-114):

```
- **Forbidden:** `Write(file_path="$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md", ...)` (absolute — bypasses the worktree shadow subtree)
- **Correct:** `Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)` (CWD-relative — lands in the story worktree shadow subtree)
- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative path traversal — brittle and error-prone)
```

Rendered, this instructs every story agent to perform exactly the write that destroyed the issue #523 artifacts, and marks the canonical form as forbidden.

**Per-gate literal shell, captured stdout** (`B` = extracted prohibition block, `J` = joined, `MS` = mandate sentence, on a scratch copy of the full `plugins/` tree):

```
$ printf '%s\n' "$J" | sed 's/\. /\n/g' | nl -ba
     1	All `.factory/**` artifact writes performed during story delivery MUST NOT use canonical absolute paths anchored to the main-checkout root
     2	Writers anchor every `.factory/**` write to the story-worktree CWD so the worktree's shadow `.factory/` subtree receives the artifact
     3	CWD-relative paths were formerly FORBIDDEN under the issue #523 reading; that prohibition is retired (BC-6.26.001 Invariant 5)
     4	
$ printf '%s\n' "$MS" | grep -qE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' && echo "MATCHES -> gate PASSES (mutant survives)" || echo "FIRES (RED)"
MATCHES -> gate PASSES (mutant survives)
$ printf '%s\n' "$MS" | grep -oE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute'
MUST NOT use canonical absolute
$ printf '%s\n' "$MS" | grep -qE 'CWD-relative|worktree-relative|relative[[:space:]]+paths?' && echo "FIRES (RED)" || echo "does not fire -> mutant survives"
does not fire -> mutant survives
$ printf '%s\n' "$B" | tr '\n' ' ' | grep -qE '(CWD-relative|relative path).*FORBIDDEN|FORBIDDEN.*(CWD-relative|relative path)' && echo "PASSES (mutant survives)" || echo "FIRES (RED)"
PASSES (mutant survives)
$ printf '%s\n' "$J" | sed 's/\. /\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)' || echo "(empty) -> does not fire -> mutant survives"
(empty) -> does not fire -> mutant survives
$ printf '%s\n' "$J" | sed 's/\. /\n/g' | grep -E 'MUST' | grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' || echo "(empty) -> does not fire -> mutant survives"
(empty) -> does not fire -> mutant survives
```

Line 2 of the split is the harmful mandate and no gate examines it. Gate 1(a) is satisfied by the negated mandate on line 1.

**End-to-end proof** (scratch copy of the full `plugins/` tree, M-P16-A applied, unmodified bats suite):

```
$ bats story-worktree-write-path-discipline.bats
1..9
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
ok 2 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
ok 3 T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds
ok 4 T-004 S-21.04 AC-006: pc2c-halt — find error (non-path-absent) HALTS teardown; exit code+stderr surfaced; worktree-remove NOT called
ok 5 T-005 S-21.04 AC-002 EC-007: file-at-path — regular file at .factory → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 6 T-006 S-21.04 AC-002: symlink-at-path — symlink at .factory pointing at real dir → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 7 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
ok 8 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
ok 9 F-S2104-P4-003: agents/devops-engineer.md §Worktree Cleanup — preflight-verification mandate
$ bats worktree-identity-preflight.bats | tail -1
ok 14 test_BC_step_d5_preflight_assertion_must_pass_before_findings
```

**M-P16-C2 — secondary vector: abbreviation splits a single rendered sentence.** `sed 's/\. /\n/g'` treats `cf. `, `i.e. `, `e.g. ` and `§4.1. ` as sentence boundaries, so one *rendered* sentence can place `MUST` in one fragment and the prohibited subject in the next, defeating Gate 5's same-sentence conjunction while Gate 1(a) stays satisfied. Exact substituted text:

```
All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute
paths only when the target is outside the worktree, cf. CWD-relative paths for every in-worktree
ledger, which land in the story worktree's shadow `.factory/` subtree and are preserved at
teardown. Duplicating a ledger onto the main checkout is FORBIDDEN (BC-6.26.001 Invariant 5).
```

```
$ printf '%s\n' "$B" | tr '\n' ' ' | sed 's/\. /\n/g' | nl -ba
     1	All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths only when the target is outside the worktree, cf
     2	CWD-relative paths for every in-worktree ledger, which land in the story worktree's shadow `.factory/` subtree and are preserved at teardown
     3	Duplicating a ledger onto the main checkout is FORBIDDEN (BC-6.26.001 Invariant 5)
$ bats story-worktree-write-path-discipline.bats | head -2
1..9
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

Severity is BLOCKER on sixth-generation recurrence of the story's primary BC postcondition remaining ungated (F-P12-003 → F-P13-001 → F-P14-001 → F-S2104-P14R-001 → F-S2104-P15-001 → here), now with the worked examples inverted alongside the prose.

**Zero-degrees-of-freedom fix predicate.** (a) Gate 1(a) MUST reject a negated mandate: after extracting the mandate sentence, assert it does NOT match `MUST[[:space:]]+(NOT|not)` and does NOT match a negation token between `MUST` and `canonical absolute` — concretely, require the mandate sentence to match `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` with no intervening negation, and add an explicit negative gate on `MUST[^.]*(NOT|not|never)[^.]*canonical[[:space:]]+absolute`. The permissive `[^.]*` between `MUST` and `use` must not be reintroduced. (b) Polarity checking MUST cover **every** sentence of the block, not only the mandate sentence: assert that no sentence in the block directs a `.factory/**` write to a worktree-anchored target — a prohibited-target alternation over the syntactic-form class `CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|worktree'?s?[[:space:]]+shadow` may not co-occur in any sentence with a directive token (`MUST|SHOULD|anchor|use|write`) unless that same sentence also carries a prohibition token (`FORBIDDEN|forbidden|MUST NOT|prohibited`). (c) Gate 2 MUST be sentence-scoped, not joined-block-scoped: require at least one *single sentence* in which a prohibited-subject form co-occurs with `FORBIDDEN`, and add a negative gate rejecting retirement/waiver language (`formerly|retired|no longer|waived|exempt`) anywhere in the block. (d) The bullet set MUST be polarity-gated per F-S2104-P16-002. (e) The fix MUST be shown RED against M-P16-A, M-P16-C2, and M-P16-D verbatim, **and** GREEN against the unmodified document, with captured stdout, **and** must retain RED for M-P14-A, M-P14R-A, the `worktree-relative` synonym vector, M-P15-A and M-P15-B (all five re-verified RED at HEAD by this review — do not regress them).

### F-S2104-P16-002 — HIGH — no gate asserts the CWD-relative worked example is Forbidden

**Stable anchors.** Gate sites: `bats` T-001, the assertion introduced by `# Gate 3 (kept from pass-12): **Forbidden:** example marker must co-occur with 'relative path'` (line 619) and the two-part block introduced by `# Gate 6 (two-part polarity; F-S2104-P14R-003 / F-S2104-P15-002)` (lines 623-641). Target: `_shared-context.md` §Spec-Path Discipline, the second worked-example bullet (`Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)`). Contract: story AC-001(a)(i).

Story AC-001 states the obligation verbatim:

> `(a) forbid BOTH forbidden forms named in BC-6.26.001 PC1: (i) CWD-relative paths (".factory/..." from story-worktree CWD — silently writes to shadow tree) and (ii) relative-traversal paths ("../../.factory/..." — brittle traversal form);`

Gate 6 covers clause (ii) with genuine two-part polarity. Clause (i) has no polarity gate at all. Gate 3 is the only gate touching that bullet, and its predicate is a `**Forbidden:**`-plus-`relative path` same-line conjunction over `$spec_path_section` — satisfiable by *any* bullet, including the traversal bullet once its parenthetical says "relative path traversal". Gate 6(b) forbids `**Correct:**` only on lines containing `../`; the CWD-relative bullet contains no `../`.

**M-P16-D — exact substituted text** (two single-line substitutions in §Spec-Path Discipline):

```
- **Correct:** `Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)` (CWD-relative — lands in the story worktree shadow subtree)
- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative path traversal — brittle and error-prone)
```

**Captured stdout** (scratch copy of the full `plugins/` tree, unmodified bats suite):

```
$ grep -n '\*\*Correct:\*\*\|\*\*Forbidden:\*\*' .../_shared-context.md
112:- **Correct:** `Write(file_path="$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md", ...)`
113:- **Correct:** `Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)` (CWD-relative — lands in the story worktree shadow subtree)
114:- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative path traversal — brittle and error-prone)
$ bats story-worktree-write-path-discipline.bats | head -3
1..9
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
ok 2 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
```

The delivered skill-doc now presents the exact write that caused issue #523 as the **Correct** form, with the S-21.04 gate suite at 9/9. This is the F-S2104-P15-002 class one hop over: that finding closed the traversal bullet's polarity in the same commit that left the sibling bullet — the more dangerous of the two — presence-only.

**Zero-degrees-of-freedom fix predicate.** Add a Gate 7 mirroring Gate 6's two-part shape on the CWD-relative form, over `$spec_path_section`: (a) positive — some line MUST match `\*\*Forbidden:\*\*` and contain a bare-relative write target on that same line (`file_path="\.factory/`); (b) negative — NO line containing `file_path="\.factory/` may match `\*\*Correct:\*\*`. Additionally tighten Gate 3 so it cannot be satisfied by the traversal bullet: scope its `relative path` conjunction to a line that also contains `file_path="\.factory/`. Record both halves with exact substituted text — M-P16-D above for (b), and a deletion mutant for (a) — each with captured RED stdout and a GREEN restore.

### F-S2104-P16-003 — HIGH — the prohibition-block extraction anchor is not unique, so a decoy paragraph captures every gate

**Stable anchors.** `bats`, the helpers `_extract_spec_path_discipline_section()` and `_extract_write_discipline_prohibition_block()` (lines 116-143). Target: `_shared-context.md` §Spec-Path Discipline.

The extractor, verbatim (`bats:137-143`):

```
_extract_write_discipline_prohibition_block() {
  _extract_spec_path_discipline_section | awk '
    /All.*\.factory.*artifact writes/ { found=1 }
    found && /^$/ { exit }
    found { print }
  '
}
```

`found=1` latches on the **first** match and the block terminates at the first blank line. Nothing asserts that the anchor pattern occurs exactly once within §Spec-Path Discipline. The helper's own MUTANT note contemplates only relocation *out* of the section ("relocate prohibition paragraph outside §Spec-Path Discipline → extractor finds nothing"); a decoy *inside* the section is unconsidered. Every prohibition-block gate — the absent-block guard, Gates 1(a), 1(b), 2, 4, 5 — then evaluates the decoy, and the normative paragraph below is never read by any assertion.

**M-P16-B — exact inserted text**, placed immediately before the line `**All spec, BC, and ADR files passed to specialists MUST be canonical repo-root absolute paths**` in §Spec-Path Discipline (two content lines plus the terminating blank line):

```
All `.factory/**` artifact writes MUST use canonical absolute paths anchored to the main-checkout root.
CWD-relative shadow-tree writes are FORBIDDEN.

```

paired with an inversion of the real normative paragraph (HEAD lines 66-70) in exactly the M-P15-A/M-P14-A shape that red-gate-log v1.13 attests RED:

```
All `.factory/**` artifact writes performed during story delivery MUST use CWD-relative paths
resolved from the story worktree CWD. Canonical absolute paths anchored to the main-checkout root
are FORBIDDEN — CWD-relative writes land in the story worktree's shadow `.factory/` subtree, which
is preserved at teardown (BC-6.26.001 Invariant 5).
```

**Captured stdout** (scratch copy of the full `plugins/` tree, unmodified bats suite):

```
$ awk '/^### Spec-Path Discipline/{f=1;next} f&&/^### /{exit} f&&/^## /{exit} f{print}' .../_shared-context.md \
    | awk '/All.*\.factory.*artifact writes/{f=1} f&&/^$/{exit} f{print}'
All `.factory/**` artifact writes MUST use canonical absolute paths anchored to the main-checkout root.
CWD-relative shadow-tree writes are FORBIDDEN.
$ grep -n -A3 'MUST use CWD-relative paths' .../_shared-context.md
69:All `.factory/**` artifact writes performed during story delivery MUST use CWD-relative paths
70-resolved from the story worktree CWD. Canonical absolute paths anchored to the main-checkout root
71-are FORBIDDEN — CWD-relative writes land in the story worktree's shadow `.factory/` subtree, which
72-is preserved at teardown (BC-6.26.001 Invariant 5).
$ bats story-worktree-write-path-discipline.bats | head -3
1..9
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
ok 2 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
```

**No-decoy control** (same inversion, decoy paragraph omitted — proves the decoy is the load-bearing element of the mutant, not the inversion text):

```
$ bats -f "T-001" story-worktree-write-path-discipline.bats
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 564)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block affirmative-mandate (sentence-scoped)]: the mandate sentence (containing 'artifact writes') must contain MUST...use...canonical absolute — the mandate must be affirmative; absent or wrong mandate fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001 / F-S2104-P14R-001)
```

This is the deepest of the three: every future polarity hardening of the prohibition-block gates is void while the anchor can be shadowed, because the hardened predicates will be applied to attacker-chosen text.

**Zero-degrees-of-freedom fix predicate.** (a) `_extract_write_discipline_prohibition_block` MUST assert its anchor is unique: count matches of `All.*\.factory.*artifact writes` within `$spec_path_section` and fail with an explicit `ambiguous anchor` message unless the count is exactly 1. (b) The extractor MUST additionally be bounded to the `#### Write Discipline` child heading rather than the whole `### Spec-Path Discipline` section, so a decoy placed in the read-discipline prose is outside the extraction domain by construction. (c) Record the M-P16-B decoy insertion verbatim as the mutant, with captured RED stdout for the uniqueness gate and a GREEN restore, plus a second mutant proving a decoy placed inside `#### Write Discipline` but before the normative paragraph is also caught.

### F-S2104-P16-004 — HIGH — story AC-001's Gate cell enumerates predicates deleted at `8b39277b`

**Stable anchor.** `.factory/stories/S-21.04-story-worktree-write-path-discipline.md`, §Acceptance Criteria, row `AC-001`, Gate column.

The cell reads, verbatim (in relevant part):

> `Gate 1 mandate-polarity two-part (positive \`MUST[^.]*use[^.]*canonical[[:space:]]+absolute\` + negative no-inversion \`not canonical absolute\|not absolute\` on the same extracted line); Gate 2 joined FORBIDDEN co-occurrence; Gate 3 §Spec-Path Discipline **Forbidden:** example marker (stable-anchor label); Gate 4 negative (no FORBIDDEN+absolute per extracted line); Gate 5 negative POLICY-13 alternation (\`CWD-relative\|worktree-relative\|relative[[:space:]]+path\` vs MUST per line); Gate 6 traversal-form marker (\`\.\./\|relative traversal\`)`

Four of the six enumerated gates no longer exist in that form at HEAD. The pass-15 wave replaced them and story-writer `6fccdcc3` (v1.20) amended only AC-010, leaving this cell at its v1.19 state:

```
$ grep -nE "not\[\[:space:\]\]\+canonical|not canonical absolute|not\[\[:space:\]\]\+absolute" story-worktree-write-path-discipline.bats
(no output)
$ grep -n "_assert_doc_marker '\\\\.\\\\./" story-worktree-write-path-discipline.bats
(no output)
$ git diff 26b85d8c..8b39277b -- plugins/vsdd-factory/tests/ | grep -E '^[+-].*(grep -|sed )' | grep -vE '^[+-]\s*#'
-  printf '%s\n' "$prohibition_block" | grep -qE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' || {
+  printf '%s\n' "$mandate_sentence" | grep -qE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' || {
-  if printf '%s\n' "$prohibition_block" | grep -E 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' | grep -qE 'not[[:space:]]+canonical[[:space:]]+absolute|not[[:space:]]+absolute'; then
+  if printf '%s\n' "$mandate_sentence" | grep -qE 'CWD-relative|worktree-relative|relative[[:space:]]+paths?'; then
-  if printf '%s\n' "$prohibition_block" | grep -qE 'absolute.*(FORBIDDEN|forbidden)|FORBIDDEN.*absolute|forbidden.*absolute'; then
+    sed 's/\. /\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)' || true)"
-  _assert_doc_marker '\.\./|relative[[:space:]]+traversal' \
+  printf '%s\n' "$spec_path_section" | grep -qE '\*\*Forbidden:\*\*.*\.\./|\.\./.*\*\*Forbidden:\*\*' || {
```

Specifically: Gate 1's negative half is no longer `not canonical absolute|not absolute` (that predicate has zero occurrences at HEAD — it was replaced by the prohibited-subject alternation on the mandate sentence); Gates 4 and 5 are no longer "per extracted line" but per reflowed sentence; Gate 6 is no longer the presence marker `\.\./\|relative traversal` but a two-part polarity gate. Two compounding consequences: the AC↔gate traceability leg — the artifact a reviewer consults to learn what AC-001 is actually gated by — describes a gate set that cannot be found in the code, and the cell now *mandates* the per-physical-line domain shape that POLICY 13's NORMALIZED-DOMAIN MANDATE (new at policies.yaml v1.4.12, the same registry version this cascade is reviewed under) declares inadmissible. F-S2104-P14R-006 closed exactly this cell against exactly this defect and pass-15 recorded it CONFIRMED-CLOSED; the pass-15 wave reopened it by changing the gates without the story leg. POLICY 14's quintuple parity was satisfied on four legs (BC, ADR, red-gate-log v1.13, STORY-INDEX v4.263) and missed on the story.

**Zero-degrees-of-freedom fix predicate.** Rewrite the AC-001 Gate cell to the HEAD gate set with the actual predicates: absent-block guard; section-bounded extractor; reflow-to-`joined_block` plus `sed 's/\. /\n/g'` sentence-split normalization named explicitly; Gate 1(a) positive on the extracted mandate sentence (`MUST[^.]*use[^.]*canonical[[:space:]]+absolute`) and Gate 1(b) negative on the same sentence (`CWD-relative|worktree-relative|relative[[:space:]]+paths?`); Gate 2 joined-block co-occurrence; Gate 3 `**Forbidden:**`+`relative path` same-line; Gate 4 negative per-sentence (`absolute` with `FORBIDDEN|forbidden`); Gate 5 negative per-sentence (`MUST` with the prohibited-subject alternation); Gate 6(a)/(b) two-part traversal polarity. The words "per extracted line" and "per line" MUST NOT appear. Bump the story version with all five POLICY 14/17 legs, and verify with captured stdout that every regex quoted in the cell has a matching occurrence in the bats file. Any subsequent burst that changes a T-001 predicate MUST update this cell in the same burst — that same-burst coupling is the mechanism whose absence produced this regression.

### F-S2104-P16-005 — MEDIUM — the Pass-15 attestation's completeness claim is falsified, and its `M-P15-A` text is not M-P15-A

**Stable anchors.** `.factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md`, §`Pass-15 assertion-site attestation (8b39277b)` → the `### Per-gate same-AC audit table (AC-001 / T-001 gates)` closing line, and the `**M-P15-A proof (RED):**` mutant-text block under `### F-S2104-P15-001 — Gates 1/4/5 sentence-scoped refactor`.

**(a) Falsified completeness claim.** The audit table closes, verbatim:

> `All gates: independent, polarity-complete, zero degrees of freedom.`

Four mutants surviving at the attested HEAD `8b39277b` falsify each of the three predicates in that sentence: M-P16-A (paragraph plus all bullets inverted, 9/9), M-P16-C2 (single rendered sentence, 9/9), M-P16-D (CWD-relative bullet relabelled Correct, 9/9), M-P16-B (extractor hijack, 9/9). The audit table itself is gate-indexed rather than obligation-indexed — it enumerates the eight gates that exist and describes each one's domain and mutant, which is honest work, but it never asks whether the union of the eight covers AC-001(a)'s two clauses. The blanket closing claim converts an inventory into a completeness attestation the inventory does not support. Note the table's own G2 row (`Affirmative: CWD-relative AND FORBIDDEN co-occur | Block-empty deletion → RED`) accurately discloses that G2 is deletion-proven only; the closing sentence contradicts that disclosure.

**(b) Mislabelled mutant text.** The block headed `**M-P15-A proof (RED):**` records this substituted text:

```
All `.factory/**` artifact writes performed during story delivery MUST
use CWD-relative paths anchored to the story-worktree CWD.
CANONICAL ABSOLUTE PATHS MUST use canonical absolute paths exclusively.
CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN — such writes land silently in the story
worktree's shadow `.factory/` subtree and are permanently destroyed at teardown (issue #523
gitignored-shadow mechanism; BC-6.26.001 Invariant 5).
```

M-P15-A as recorded in `adversary-pass-15.md` Part A is:

```
All `.factory/**` artifact writes performed during story delivery MUST use
CWD-relative paths anchored to the story-worktree CWD.
Writers MUST use canonical absolute paths only when reading spec ground-truth from the main checkout.
Canonical absolute artifact-write paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`)
are FORBIDDEN — relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.
```

Two material divergences. The decoy sentence — the element that made M-P15-A a *plausible* document rather than an obviously broken one — is replaced by an implausible shouted placeholder. More consequentially, the third sentence is reverted to the **original** `CWD-relative … are FORBIDDEN` rather than the mutant's inverted `Canonical absolute artifact-write paths … are FORBIDDEN`, which is precisely the element Gate 4 exists to catch; the recorded mutant therefore never exercises Gate 4, and the attestation acknowledges this obliquely by attributing Gate 4's coverage to an unnamed "M-P15-A variant". The recorded stdout is reproducible for the text as written, and I independently confirmed the **actual** M-P15-A is RED at HEAD (Gate 1(a), bats line 564) — so the closure itself holds. But the record does not prove it, and under POLICY 15's exact-substituted-text discipline a mutant recorded under a label must be that label's text.

**Zero-degrees-of-freedom fix predicate.** (a) Replace the closing line with a claim the table supports and that names the gap — e.g. `Gate inventory as of this HEAD; polarity coverage proven for the mutants listed per row. Not a completeness claim: obligation-indexed coverage of AC-001(a)(i)/(ii) is asserted separately below.` — and add an obligation-indexed coverage table with one row per AC-001 clause naming the gate(s) that assert it and the mutant that proves each. (b) Replace the `M-P15-A proof (RED)` block's text with the verbatim five-line M-P15-A from `adversary-pass-15.md` Part A, re-run it, and record the captured stdout; if a distinct simplified vector is also recorded, label it separately (e.g. `M-P15-A-simplified`) rather than under the adversary's ID. (c) Preserve the existing v1.13 entry unmodified and record the correction as a new changelog row with an explicit error-acknowledgment clause, per the discipline BC-6.26.001 v1.11 and story v1.13 already established for this class.

### F-S2104-P16-006 — LOW — pass-15 Fix Mapping attests a bats-comment leg that never landed

**Stable anchor.** `.factory/cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-15.md`, §Fix Mapping, row `F-S2104-P15-005`.

The row reads, verbatim:

> `story-writer 6fccdcc3 — story v1.20 option-a: AC-010 carve-out aligned; T-009 gate comment updated to acknowledge that the broader character class was chosen specifically to catch bcs: inside backtick code spans; AC-010 text clarified`

No commit in the pass-15 wave touched the `bcs:` gate or its comment, and the comment at HEAD states a different rationale:

```
$ git diff 26b85d8c..8b39277b -- plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats | grep -n 'bcs'
(no output)
$ sed -n '1386,1390p' plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
  # (ii) Negative: stale bcs: as standalone field token must NOT appear in adversary.md.
  # Pattern (^|[^a-zA-Z0-9_])bcs: matches bcs: when preceded by any non-identifier character
  # (start-of-line, space, backtick, quote, or punctuation) — avoids false hits on compound
  # identifiers like 'subbcs:' or 'xbcs:' while catching both YAML-field and prose forms.
  if grep -qE '(^|[^a-zA-Z0-9_])bcs:' "$ADVERSARY_MD"; then
```

The comment mentions backticks only inside a parenthetical list of non-identifier characters; its stated purpose is compound-identifier false-hit avoidance, not code-span capture. The attributed agent (story-writer) does not own the bats file in any case. The substantive half of the fix did land in full — AC-010's carve-out is dropped, the criterion now reads "MUST NOT carry any standalone stale `bcs:` token" with the code-span rationale stated inline, and the gate matches the widened criterion — so F-S2104-P15-005 is closed. Only the record overstates the wave's scope. This is the same class as the falsified "zero bare pins verified" claim that F-S2104-P15-004 flagged one pass ago.

**Zero-degrees-of-freedom fix predicate.** Amend the Fix Mapping row to drop the unlanded bats-comment clause, leaving `story-writer 6fccdcc3 — story v1.20 option-a: AC-010 carve-out dropped; code-span prohibition rationale stated inline; gate unchanged (already matches the widened criterion)`. If the comment clarification is judged worth making, route it to test-writer as its own leg with its own commit — do not attest it retroactively against a story-writer commit.

---

## Observations (NOT findings)

**Both pass-15 vectors independently re-proven RED from verbatim Part A text.** M-P15-A fires Gate 1(a) (`DOC-PARITY FAIL [write-discipline prohibition block affirmative-mandate (sentence-scoped)]`, bats line 564) and M-P15-B fires Gate 6(a) (`DOC-PARITY FAIL [write-discipline §Spec-Path Discipline traversal-Forbidden bullet absent]`, bats line 636). The reflow-then-sentence-split normalization is real work, and the line-wrap axis that defeated passes 12-14R is genuinely closed. F-S2104-P16-001 is a gap in the *logical form* of the predicates and the *scope* of polarity checking, not a false attestation of the domain-shape fix.

**POLICY 15 attestation-location satisfied for the predicate diff.** I diffed `26b85d8c..8b39277b` for assertion-predicate changes and cross-checked each against red-gate-log v1.13. Every changed predicate — Gate 1(a) domain change to `$mandate_sentence`, Gate 1(b) replacement, Gates 4 and 5 sentence-split rewrites, Gate 6(a) and 6(b) — has a matching attestation subsection. No predicate changed without a record, and no attestation content leaked into the shipped bats file.

**Epic v1.8 leg is genuinely and completely closed.** All six E-21 BC Traceability pins now match their targets exactly (`BC-4.16.001 v1.8`, `BC-5.43.001 v1.4`, `BC-5.44.001 v1.5`, `BC-6.10.002 v1.5`, `BC-6.26.001 v1.11`, `BC-6.27.001 v1.4`), the BC-6.26.001 row carries the trailing-slash `find "<worktree-path>/.factory/" -type f` form, and every live-body BC version cite (epic lines 23, 84, 90, 96, 100, 103, 115) is current. `grep -cE 'ADR-031 v[0-9]'` returns 5, all in `last_amended`, `modified[]`, or body Changelog rows — historical-by-construction and POLICY 5 exempt. The eleven live-body POLICY 19 tokens pass-15 found are gone.

**STORY-INDEX v4.263 blockquote is HEAD-reproducible.** All six input-hash values in the `> **E-21 delivery:**` blockquote match the live story frontmatter exactly (S-21.01=32aaccc, S-21.02=8bd32e5, S-21.03=59e687e, S-21.04=1165b1f, S-21.05=c9265f0, S-21.06=b807086), distinctness holds, and the annotation `[Refreshed D-914; values live in story frontmatter]` was added. The S-21.04 catalog row and the blockquote no longer disagree.

**Bare-pin sweep is complete in both perimeter bats files.** `grep -nE '~:[0-9]+|line ~[0-9]+|lines? [0-9]+(/[0-9]+)?|:[0-9]{2,4}-[0-9]{2,4}'` returns zero matches across `story-worktree-write-path-discipline.bats` and `worktree-identity-preflight.bats`, and the `(e)` docblock's future tense is rewritten to past ("the prior stale-snapshot prohibition assertion … *would have blocked* the implementer's residue sweep"). No harness-emitted line numbers appear in authored prose in either file, so the new POLICY 5 carve-out is not even load-bearing here.

**CHANGELOG lead-in is now count-sound.** `Delivered as skill-doc mandates (no new WASM or shell script, per POLICY 21) — the two BC-6.26.001 protocol requirements plus the propagation and awareness legs:` — the "two" now scopes precisely to items (1)-(2), with (3)-(5) covered by "the propagation and awareness legs". The reader-facing arithmetic contradiction is gone.

**`devops-engineer.md` fence lead-in closed.** `e7ac3aef` added a blank line and the lead-in `Cleanup command:` before the ```bash fence. The pass-15 cosmetic observation is resolved.

**ADR-031 v1.13 carries one self-referential version token, correctly.** Body line 331 reads `ADR-031 v1.0 incorrectly …` — an error-acknowledgment statement about a superseded version, not a load-bearing cross-reference. POLICY 19 is not implicated.

**[process-gap] — three passes of hardening have each fixed the *domain* while leaving the predicate's logical form and its extraction anchor unconstrained.** Pass-13 added polarity gates; pass-14R widened the token alternation to a syntactic-form class; pass-15 normalized the evaluation domain from physical line to reflowed sentence. Each was the correct fix for the vector in front of it, and each left the *shape of the assertion* untouched: a token-order regex with a permissive `[^.]*` wildcard that steps across negation tokens, applied to text selected by a non-unique anchor, with polarity asserted on exactly one sentence of the block. POLICY 13's NORMALIZED-DOMAIN MANDATE closed the domain axis and says nothing about either of the other two. Candidate codification, in two parts: **(i) negation-transparency** — a polarity predicate over prose MUST NOT use a wildcard that can span a negation token between the modal and the mandated subject; every affirmative-mandate gate MUST carry a paired negative gate proving a `MUST NOT`/`never`/`no longer` insertion at the matched position turns it RED, with that insertion recorded as a mandatory mutant class; and **(ii) anchor-uniqueness** — any extractor whose output feeds a semantic gate MUST assert its anchor matches exactly once within its bounding section, with a decoy-insertion mutant recorded. Either one alone would have caught this pass; together they close the two axes that survived passes 13 through 15.

**[process-gap] — the SAME-AC GATE AUDIT is gate-indexed, so it cannot detect an unrepresented obligation.** POLICY 15's new clause was honoured in form: red-gate-log v1.13 carries a per-gate audit table covering all eight AC-001 gates with domain shape, polarity direction, and mutant coverage per row. It still missed F-S2104-P16-002, because a table whose rows are *gates* answers "is each gate sound?" and never "is each obligation gated?" AC-001(a) has two clauses; clause (ii) has a two-part polarity gate and clause (i) has none, and no gate-indexed table can surface that asymmetry. Candidate codification: the SAME-AC GATE AUDIT table MUST be **obligation-indexed** — one row per enumerated clause of the AC, each naming the gate(s) that assert it, the polarity direction asserted, and the mutant proving it, with any clause whose gate column is empty or presence-only marked as an open gap rather than omitted. The same inversion applies to the closing completeness claim (F-S2104-P16-005(a)): "all gates sound" and "all obligations gated" are different propositions, and only the second is what a Red Gate attestation is for.

---

## Per-Pass-15 Verification

| Finding | Status | Notes |
|---------|--------|-------|
| F-S2104-P15-001 | PARTIAL (6th-gen re-seed) | Sentence-scoped refactor landed and is load-bearing: `joined_block` (`tr '\n' ' '`) plus `sed 's/\. /\n/g'` normalization present at bats:540/556-557; Gates 1(a), 1(b), 4, 5 all operate on the reflowed domain; I re-proved verbatim M-P15-A RED at HEAD (Gate 1(a), bats line 564), and M-P14-A, M-P14R-A and the `worktree-relative` synonym vector all remain RED. The line-rewrap axis is genuinely closed. But Gate 1(a)'s `MUST[^.]*use` wildcard matches `MUST NOT use canonical absolute`, only the mandate sentence is polarity-checked, and Gate 2 is polarity-blind across the joined block: M-P16-A inverts PC1 across the paragraph and all three bullets at 9/9; M-P16-C2 does it inside one rendered sentence → F-S2104-P16-001, F-S2104-P16-003 |
| F-S2104-P15-002 | PARTIAL | Gate 6 is now genuinely two-part: 6(a) requires `**Forbidden:**`+`../` same-line (bats:634), 6(b) fires on any `../` line matching `**Correct:**` (bats:638). I re-proved verbatim M-P15-B RED at HEAD (Gate 6(a), bats line 636). The traversal clause AC-001(a)(ii) is closed. But the sibling CWD-relative bullet — AC-001(a)(i), the literal issue #523 write — has no polarity gate: M-P16-D relabels it `**Correct:**` at 9/9 → F-S2104-P16-002 |
| F-S2104-P15-003 | CONFIRMED-CLOSED | Epic `version: "v1.8"`; all six BC Traceability pins match their targets exactly; BC-6.26.001 row carries the trailing-slash find form; all live-body BC version cites current (lines 23/84/90/96/100/103/115); `grep -cE 'ADR-031 v[0-9]'` → 5, every one in `last_amended`/`modified[]`/body-Changelog (historical-by-construction, POLICY 5 exempt). The eleven live-body POLICY 19 tokens are gone |
| F-S2104-P15-004 | CONFIRMED-CLOSED | `grep -nE '~:[0-9]+\|line ~[0-9]+\|lines? [0-9]+(/[0-9]+)?\|:[0-9]{2,4}-[0-9]{2,4}'` → zero matches across both perimeter bats files; extractor docblock, T-001 comment block, and Gate 2/4/5 comments all carry stable semantic anchors; the `(e)` docblock's future tense is rewritten to past. No authored-prose line pins remain, so the new POLICY 5 harness carve-out is not load-bearing here |
| F-S2104-P15-005 | CONFIRMED-CLOSED | Option (a) taken in full: AC-010 reads `MUST NOT carry any standalone stale \`bcs:\` token` with no carve-out, the code-span prohibition rationale is stated inline, and the widening is recorded in `modified[]` + `last_amended` at v1.20. Gate (bats:1390, `(^\|[^a-zA-Z0-9_])bcs:`) now enforces exactly the criterion it is registered against; positive side verified (`grep -c 'behavioral_contracts:' adversary.md` → 5, matching AC-010's stated site count). The Fix Mapping's extra bats-comment claim is a separate record defect → F-S2104-P16-006 |
| F-S2104-P15-006 | CONFIRMED-CLOSED | CHANGELOG lead-in now reads `Delivered as skill-doc mandates (no new WASM or shell script, per POLICY 21) — the two BC-6.26.001 protocol requirements plus the propagation and awareness legs:`; the count "two" scopes to items (1)-(2) and (3)-(5) are covered by the trailing clause. Items (1)-(5) were correctly not renumbered |
| F-S2104-P15-007 | CONFIRMED-CLOSED | STORY-INDEX `version: "4.263"`; blockquote values all match live story frontmatter (S-21.01=32aaccc, S-21.02=8bd32e5, S-21.03=59e687e, S-21.04=1165b1f, S-21.05=c9265f0, S-21.06=b807086); distinctness holds; annotation `[Refreshed D-914; values live in story frontmatter]` added; the internal contradiction with the S-21.04 catalog row is resolved |

Tally: **5 CONFIRMED-CLOSED / 2 PARTIAL / 0 REGRESSED** against the pass-15 finding set. Both PARTIALs are one-hop re-seedings on the polarity axis, now the sixth consecutive pass exhibiting that pattern — but the mechanism has changed character: passes 12-15 each found the *same* predicate defeated at a finer granularity, whereas this pass found two *different* properties of the assertion (negation-transparency of the predicate, uniqueness of the extraction anchor) that no prior hardening addressed at any granularity. Separately, F-S2104-P16-004 is a **regression** of the F-S2104-P14R-006 closure that pass-15 verified CONFIRMED-CLOSED: the pass-15 wave changed four T-001 predicates without the same-burst story Gate-cell leg. The five substantive closures — epic pins, bare pins, AC-010 alignment, CHANGELOG, STORY-INDEX — are complete and independently verified, and the pass-15 wave's own gate attestations contain no fabricated stdout.
