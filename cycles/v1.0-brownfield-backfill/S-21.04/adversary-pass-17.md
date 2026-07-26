---
pass: 17
verdict: NOT-CLEAN
reviewed_head: 9ab1aa32
fixes_landed_head: c89bef22
novelty: 0.55
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-16.md"
---

## Summary

Pass-17 fresh-context adversarial review of S-21.04 at reviewed_head `9ab1aa32` (worktree `.worktrees/S-21.04`, base develop `948f0fb1`). **7 findings: B2 / H2 / M3.** Novelty 0.55 vs pass-16 Part A — the four gate findings attack axes no prior pass has touched: the *extent* of the gated domain (one paragraph vs the whole section), the *renderability* of the text that satisfies the positive gates (HTML comment), the *closure* of PW-B's two token alternations (fixed surface lists vs syntactic classes), and Gate 2b's unnormalized per-physical-line domain. Trajectory 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7. Streak: **0/3** (BC-5.39.001 reset).

Baseline by literal execution at HEAD: `bats story-worktree-write-path-discipline.bats` → `1..9`, 9/9 ok; `bats worktree-identity-preflight.bats` → `1..14`, 14/14 ok.

The pass-16 wave did real, verified work. I independently re-proved **all nine** recorded vectors RED at HEAD from their verbatim Part A text — M-P16-A and M-P15-A and M-P14R-A and the `worktree-relative` synonym all fire Gate 1(a) (zero-DoF `MUST[[:space:]]+use`), M-P16-C2 fires Gate 1(c) via the abbreviation-protected splitter, M-P16-D fires tightened Gate 3, M-P16-B fires Gate 1(a) with the `####` bounding neutralising the decoy, the in-section decoy fires anchor-uniqueness (count=2), M-P15-B fires Gate 6(a). Negation-transparency, anchor-uniqueness and the CWD-relative bullet polarity are all genuinely closed. I additionally probed a duplicate-`#### Write Discipline`-heading decoy (a bounding attack no pass has recorded): it is **caught**, because `_extract_write_discipline_section`'s first rule ends in `next`, so a second matching heading does not terminate extraction and both anchors land in one section → count=2 → RED.

**The gate set is nevertheless defeated three independent ways, and the shift is again one of axis rather than granularity.** Passes 13–16 progressively hardened *what the predicates say* about the text they are given. Nothing has ever constrained *which text they are given* beyond its first paragraph, or whether that text is visible to a reader. First: every prose gate reads `$prohibition_block`, which terminates at the first blank line, so a harmful second paragraph inside `#### Write Discipline` is examined by nothing. Second: nothing is comment-aware, so the entire compliant mandate can be moved inside `<!-- -->` — satisfying Gate 1(a) and Gate 2a with text a Markdown renderer never displays — leaving a single visible harmful sentence. Third, and inside the paragraph itself: Gate PW-B's directive-token list is `MUST|SHOULD|anchor|use|write`, so the phrasing "CWD-relative paths **are the required form**" carries no directive token and PW-B cannot fire; I inverted the normative paragraph end-to-end on that basis with all nine prose gates passing, and confirmed by control that the directive-token gap is the load-bearing element (inserting the word `used` turns it RED).

Second structural theme, and the fourth consecutive pass to exhibit it: the *sibling* of the just-hardened gate ships to the pre-hardening standard. Gate 2b (retirement language) was authored in the same commit as PW-B, yet it is the one prose gate still evaluated on the raw per-physical-line domain that POLICY 13's NORMALIZED-DOMAIN MANDATE declares inadmissible, and its synonym list is closed — both exploitable. Likewise Gates 3/6/7 pin exact bullet renderings, so a `**Correct:**` bullet whose target is `file_path="./.factory/..."` — one character from the write that destroyed the issue #523 artifacts — is unconstrained.

Third theme, on the record side: the obligation-indexed coverage table POLICY 15 mandated at v1.4.13 was authored, but it covers only AC-001(a) — AC-001(b) and AC-001(c) have no rows at all despite being presence-only gated, which is the exact omission the mandate exists to surface. And the 15-gate gate-indexed table omits both Gate PW-B and Gate 2b (counting two non-assertion helper mechanisms in their place). Gate 2b's absence from that audit is precisely why its unnormalized domain went unexamined.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P17-001 | BLOCKER | `story-worktree-write-path-discipline.bats` §T-001 `_extract_write_discipline_prohibition_block` + all prose gates vs `_shared-context.md` `#### Write Discipline` | The gated domain is neither the whole `#### Write Discipline` section nor the rendered content. (a) `prohibition_block` terminates at the first blank line, so a second paragraph inside `#### Write Discipline` is read by no gate — M-P17-A adds a "Story-worktree exception" paragraph mandating CWD-relative writes at 9/9. (b) No gate is comment-aware — M-P17-H moves the entire compliant mandate inside `<!-- -->`, satisfying Gate 1(a)+2a with invisible text while the only rendered sentence directs writes to the worktree CWD, at 9/9 | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13, 15; TD-VSDD-059 |
| F-S2104-P17-002 | BLOCKER | `bats` §T-001 Gate PW-B (`polarity_violations`) vs `_shared-context.md` `#### Write Discipline` normative paragraph | Gate PW-B's directive-token alternation (`MUST\|SHOULD\|anchor\|use\|write`) and prohibited-target alternation are closed surface-form lists, not syntactic classes. Directive-free predication ("CWD-relative paths **are the required form**") carries no listed token, so PW-B cannot fire. M-P17-C inverts the normative paragraph end-to-end — mandate scoped to out-of-worktree targets, CWD-relative declared required — with all nine prose gates passing at 9/9. Control proves the directive-token gap is load-bearing | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13, 15 |
| F-S2104-P17-003 | HIGH | `bats` §T-001 Gate 2b (`retirement_language`) | The only prose gate still evaluated on the raw per-physical-line domain, with a closed synonym list. (a) M-P17-D nullifies the prohibition with `rescinded and superseded` at 9/9 — neither token is in `formerly\|retired\|no longer\|waived\|exempt`. (b) M-P17-F wraps `no longer` across the paragraph's soft line break at 9/9 — the per-line grep matches neither half. POLICY 13 NORMALIZED-DOMAIN MANDATE violation in the same commit that codified it | BC-6.26.001 PC1; POLICY 11, 13, 15 |
| F-S2104-P17-004 | HIGH | `bats` §T-001 Gates 3 / 6(a)(b) / 7(a)(b) vs `_shared-context.md` §Spec-Path Discipline worked-example bullets | All five bullet gates pin exact surface renderings (`file_path="\.factory/` and `\.\./`). A `**Correct:**` bullet whose target is `file_path="./.factory/..."` matches neither, so Gates 6(b) and 7(b) are both blind to it and the positive gates stay satisfied by the untouched Forbidden bullets. M-P17-G adds it at 9/9. One-hop re-seed of the F-S2104-P16-002 class into a sibling rendering of the same clause | BC-6.26.001 PC1; POLICY 11, 13, 15 (SAME-AC GATE AUDIT) |
| F-S2104-P17-005 | MEDIUM | red-gate-log v1.14 §`Pass-16 assertion-site attestation (9ab1aa32)` — lead-in paragraph, TIER 1 table, gate-indexed audit table | Five gate descriptions do not match the code they attest, two of them self-contradicting the same document: (a) `§[0-9]+\.` abbreviation protection attested twice, zero occurrences at HEAD; (b) anchor-uniqueness domain attested as `$spec_path_section` twice — actual is the `#### Write Discipline` section, and the `$spec_path_section` reading contradicts rows for the out-of-section decoy; (c) TIER 1 M-P16-B row attributes anchor-uniqueness count=2, contradicting its own preamble and my execution (Gate 1(a), count=1); (d) absent-block guard domain misstated as `$spec_path_section`; (e) the G5 row states PW-B's predicate | POLICY 15 (verbatim + attestation-location), 3, 4; TD-VSDD-059; D-448(a) class |
| F-S2104-P17-006 | MEDIUM | red-gate-log v1.14 §`Gate-indexed audit table (… 15 gates)` + §`Obligation-indexed AC-001 coverage table` | The audit tables cannot detect the gaps they exist to detect. The 15-gate table omits Gate PW-B and Gate 2b — both real assertions — while counting the `#### Write Discipline` bounding and the abbreviation-protected splitter (helper mechanisms, not assertions) as gates, and renames code Gate 1(c) as `G1(b)`; the count 15 coincides with the story's while enumerating a different set. The obligation-indexed table has rows only for AC-001(a); AC-001(b) and AC-001(c) are omitted rather than marked open despite being presence-only gated — the exact omission POLICY 15's obligation-indexed mandate was codified to surface | POLICY 15 (SAME-AC GATE AUDIT, obligation-indexed form), 14; TD-VSDD-059 |
| F-S2104-P17-007 | MEDIUM | story `S-21.04-…md` §Acceptance Criteria row AC-001, Gate column | The Gate cell places item (8) Gate 2b inside the domain group introduced by "gates 3–10 evaluate on the joined+abbreviation-protected … prohibition block, sentence-split on `. ` boundaries". Gate 2b does neither: it greps raw `$prohibition_block` per physical line. The cell therefore attests the normalized domain for the one gate that lacks it — the same misdescription that let F-S2104-P17-003(b) through. Nitpick in the same cell: item (10) quotes Gate 5's alternation as `relative[[:space:]]+paths?`; the code has `relative[[:space:]]+path` | POLICY 4, 13, 14 (story Gate-cell leg); TD-VSDD-059 |

---

### F-S2104-P17-001 — BLOCKER — the gated domain is one paragraph, and it need not be visible

**Stable anchors.** Extractor: `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`, `_extract_write_discipline_prohibition_block()` (the helper whose docblock begins `Extracts the Write Discipline prohibition paragraph from _shared-context.md`). Prose gates: T-001 (`@test "T-001 S-21.04 AC-003: stray-file-blocks …"`), the block from the comment `# Anchor uniqueness gate: #### Write Discipline must have exactly one prohibition anchor.` through `# Gate 5 (NEGATIVE, sentence-scoped; …)`. Target: `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`, `#### Write Discipline — `.factory/**` artifact writes from story worktrees`.

The extractor, verbatim:

```
_extract_write_discipline_prohibition_block() {
  _extract_write_discipline_section | awk '
    /All.*\.factory.*artifact writes/ { found=1 }
    found && /^$/ { exit }
    found { print }
  '
}
```

`found && /^$/ { exit }` terminates at the first blank line. Every downstream prose gate derives from that one paragraph: `joined_block="$(printf '%s\n' "$prohibition_block" | tr '\n' ' ')"`, then `joined_block_nosplit`, then Gates 1(a)/(b)/(c), PW-B, 2a, 4, 5; Gate 2b reads `$prohibition_block` directly. The `#### Write Discipline` section runs to `### Story-Size Gate` and contains four further paragraphs plus the bullet set — none of which any prose gate reads.

**M-P17-A — exact inserted text**, placed immediately after the normative prohibition paragraph's terminating blank line (i.e. between it and `**Load-bearing cases (BC-6.26.001 Invariant 4):**`), with nothing else changed:

```

**Story-worktree exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every `.factory/**`
artifact write to the story worktree CWD; the canonical-absolute form applies only to spec reads
from the main checkout.
```

**Captured stdout** (scratch copy of the full `plugins/` tree, unmodified bats suite):

```
$ WD="$(awk '/^#### Write Discipline/{f=1;next} f&&/^#### /{exit} f&&/^### /{exit} f&&/^## /{exit} f{print}' _shared-context.md)"
$ printf '%s\n' "$WD" | awk '/All.*\.factory.*artifact writes/ {c++} END {print c+0}'
1
$ B="$(printf '%s\n' "$WD" | awk '/All.*\.factory.*artifact writes/{f=1} f&&/^$/{exit} f{print}')"
$ printf '%s\n' "$B"
All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
anchored to the main-checkout root. CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN — such writes land silently in the story
worktree's shadow `.factory/` subtree and are permanently destroyed at teardown (issue #523
gitignored-shadow mechanism; BC-6.26.001 Invariant 5).
$ printf '%s\n' "$B" | grep -c 'Story-worktree exception'
0
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
```

The anchor-uniqueness gate is not implicated: the inserted paragraph does not begin with `All`, so the count stays 1. The prohibition paragraph is untouched, so every polarity gate is satisfied by text that the added paragraph explicitly overrides for the only case that matters.

**M-P17-H — exact substituted text**, replacing the normative prohibition paragraph:

```
<!-- All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
anchored to the main-checkout root. CWD-relative paths are FORBIDDEN. -->
Writers MUST anchor every `.factory/**` artifact to the worktree CWD so the shadow subtree receives it.
```

**Captured stdout:**

```
$ bats -f "T-001" story-worktree-write-path-discipline.bats
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

Gate 1(a) matches inside the comment; Gate 2a's `CWD-relative … FORBIDDEN` co-occurrence is inside the comment; Gate 1(c) and Gate 5 see no prohibited-subject form in the mandate sentence because the mandate sentence *is* the commented one; PW-B does not fire on the visible sentence because `the worktree CWD` is not in its prohibited-target list (see F-S2104-P17-002). A reader of the rendered skill doc sees exactly one instruction, and it is the issue #523 write.

**Zero-degrees-of-freedom fix predicate.** (a) The prose polarity gates MUST evaluate the **whole `#### Write Discipline` section**, not the first paragraph: introduce a `write_discipline_prose` domain equal to `_extract_write_discipline_section` with fenced code blocks excluded, reflow it, sentence-split it, and apply Gate PW-B, Gate 2b, Gate 4 and Gate 5 over every sentence of that domain. Gates 1(a)/(b)/(c) and Gate 2a may remain scoped to the mandate sentence / prohibition paragraph, since they are positive existence assertions; the *negative* gates MUST NOT be paragraph-scoped. (b) Add a gate asserting the `#### Write Discipline` section contains **no HTML comment** (`grep -qE '<!--'` → RED with an explicit `comment-hidden normative text` message), so no positive gate can be satisfied by unrendered text; if comments are judged legitimate elsewhere, instead strip `<!-- … -->` spans from every gate domain before evaluation and add a positive gate proving the mandate sentence survives stripping. (c) Record M-P17-A and M-P17-H verbatim as mutants with captured RED stdout and a GREEN restore, plus a third mutant placing the harmful sentence in the `**Load-bearing cases**` paragraph to prove the extended domain reaches every paragraph. (d) The fix MUST retain RED for M-P16-A, M-P16-B (both placements), M-P16-C2, M-P16-D, M-P15-A, M-P15-B, M-P14-A, M-P14R-A and the `worktree-relative` synonym vector — all nine re-verified RED at HEAD by this review.

### F-S2104-P17-002 — BLOCKER — PW-B's token alternations are closed surface lists, so the normative paragraph inverts end-to-end

**Stable anchors.** `bats` T-001, the block introduced by the comment `# Gate PW-B (BLOCK-WIDE SENTENCE POLARITY, F-S2104-P16-001(b)):` (the assertion assigning `polarity_violations`). Target: `_shared-context.md` `#### Write Discipline` normative prohibition paragraph. Contract: BC-6.26.001 PC1 and Invariant 1 ("Worktree-relative paths are **categorically** forbidden for `.factory/**` writes").

The predicate, verbatim:

```
  polarity_violations="$(printf '%s\n' "$joined_block_nosplit" | sed 's/\. /\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow' | \
    grep -E 'MUST|SHOULD|anchor|use|write' | \
    grep -Ev 'FORBIDDEN|forbidden|MUST NOT|prohibited' || true)"
```

Three compounding properties. First, the directive-token stage is a five-token list; English predicates a requirement without any of them — "is the required form", "is the correct form", "is preferred", "applies to", "is what writers do". Second, Gate 1(a) requires only that the mandate sentence *contain* `MUST use canonical absolute`; it does not constrain what the sentence goes on to say, so the mandate can be scoped to a case that never occurs in practice. Third, Gate 2a is satisfied by any sentence pairing a prohibited-subject form with `FORBIDDEN`, including one that forbids something harmless.

**M-P17-C — exact substituted text**, replacing the normative prohibition paragraph:

```
All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
when the target lies outside the story worktree. For in-worktree ledgers, CWD-relative paths are
the required form, and they land in the story worktree's shadow `.factory/` subtree, which is
preserved at teardown. Duplicating a ledger onto the main checkout by `../../` traversal is
FORBIDDEN for relative paths of that kind (BC-6.26.001 Invariant 5).
```

**Per-gate literal shell, captured stdout** (`B` = extracted prohibition block, `JN` = joined + abbreviation-protected, `MS` = mandate sentence):

```
$ printf '%s\n' "$JN" | sed 's/\. /\n/g' | nl -ba
     1	All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths when the target lies outside the story worktree
     2	For in-worktree ledgers, CWD-relative paths are the required form, and they land in the story worktree's shadow `.factory/` subtree, which is preserved at teardown
     3	Duplicating a ledger onto the main checkout by `../../` traversal is FORBIDDEN for relative paths of that kind (BC-6.26.001 Invariant 5)
     4	
$ printf '%s\n' "$MS" | grep -qE 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute' && echo "PASSES (mutant survives)" || echo "FIRES RED"
PASSES (mutant survives)
$ printf '%s\n' "$MS" | grep -qE 'MUST[^.]*(NOT|not|never)[^.]*canonical[[:space:]]+absolute' && echo "FIRES RED" || echo "PASSES (mutant survives)"
PASSES (mutant survives)
$ printf '%s\n' "$MS" | grep -qE 'CWD-relative|worktree-relative|relative[[:space:]]+paths?' && echo "FIRES RED" || echo "PASSES (mutant survives)"
PASSES (mutant survives)
$ printf '%s\n' "$JN" | sed 's/\. /\n/g' | grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow' | grep -E 'MUST|SHOULD|anchor|use|write' | grep -Ev 'FORBIDDEN|forbidden|MUST NOT|prohibited' || echo "(empty) -> PASSES (mutant survives)"
(empty) -> PASSES (mutant survives)
$ printf '%s\n' "$JN" | sed 's/\. /\n/g' | grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' | grep -E 'FORBIDDEN|forbidden'
Duplicating a ledger onto the main checkout by `../../` traversal is FORBIDDEN for relative paths of that kind (BC-6.26.001 Invariant 5)
$ printf '%s\n' "$B" | grep -E 'formerly|retired|no longer|waived|exempt' || echo "(empty) -> PASSES (mutant survives)"
(empty) -> PASSES (mutant survives)
$ printf '%s\n' "$JN" | sed 's/\. /\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)' || echo "(empty) -> PASSES (mutant survives)"
(empty) -> PASSES (mutant survives)
$ printf '%s\n' "$JN" | sed 's/\. /\n/g' | grep -E 'MUST' | grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' || echo "(empty) -> PASSES (mutant survives)"
(empty) -> PASSES (mutant survives)
```

Sentence 2 is the harmful instruction. It carries the prohibited-target forms `CWD-relative` **and** `worktree's shadow`, and PW-B still cannot fire because none of `MUST|SHOULD|anchor|use|write` appears in it.

**End-to-end proof** (scratch copy of the full `plugins/` tree, M-P17-C applied, unmodified bats suite): `1..9`, 9/9 ok (all nine `ok` lines identical to the F-S2104-P17-001 listing above).

**Load-bearing control** — same paragraph with sentence 2 changed from `are the required form` to `are used`, i.e. inserting one listed directive token:

```
$ bats -f "T-001" story-worktree-write-path-discipline.bats
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 669)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block block-wide sentence polarity (Gate PW-B, F-S2104-P16-001(b))]: a sentence in the Write Discipline prohibition block co-occurs a prohibited-target form (CWD-relative|worktree-relative|relative paths?|story-worktree CWD|worktree's shadow) with a directive token (MUST|SHOULD|anchor|use|write) but no prohibition token (FORBIDDEN|MUST NOT|prohibited) — M-P16-A S2 'Writers anchor every .factory/** write to the story-worktree CWD' triggers this gate (BC-6.26.001 PC1; AC-001(a))
# For in-worktree ledgers, CWD-relative paths are used, and they land in the story worktree's shadow `.factory/` subtree, which is preserved at teardown
```

This proves the directive-token list — not the prohibited-target list, and not the sentence structure — is the element M-P17-C evades.

Severity is BLOCKER on seventh-generation recurrence of the story's primary BC postcondition remaining invertible (F-P12-003 → F-P13-001 → F-P14-001 → F-S2104-P14R-001 → F-S2104-P15-001 → F-S2104-P16-001 → here), now via a mechanism that requires no negation, no abbreviation, no decoy and no label swap — only ordinary declarative English.

**Zero-degrees-of-freedom fix predicate.** (a) Replace PW-B's directive-token whitelist with a **prohibition-token requirement**: for every sentence in the (extended, per F-S2104-P17-001(a)) domain that contains a prohibited-target form, that same sentence MUST carry a prohibition token from `FORBIDDEN|forbidden|MUST NOT|prohibited|never|forbid`. Drop the directive-token stage entirely — its only effect is to exempt sentences, and any sentence mentioning a prohibited target in prose that is not marking it prohibited is itself the defect. (b) Extend the prohibited-target alternation to a syntactic-form class covering the article-and-adjective variants: add `worktree[[:space:]]+CWD`, `shadow[[:space:]]+subtree`, `worktree-local`, `in-worktree`, `story[[:space:]]+worktree[[:space:]]+CWD`, and mutant-prove each addition. (c) Add a gate on the mandate sentence forbidding **conditional scoping** of the mandate: reject `only[[:space:]]+(when|where|if)|when[[:space:]]+the[[:space:]]+target|unless` in the mandate sentence, so `MUST use canonical absolute paths` cannot be narrowed to a sub-case; BC-6.26.001 Invariant 1's word is "categorically". (d) Tighten Gate 2a so the FORBIDDEN sentence must forbid the *write form*, not merely co-occur with a prohibited-subject token: require the same sentence to contain a prohibited-subject form AND `FORBIDDEN` AND not contain `traversal`-only qualification — concretely, require at least one sentence matching `(CWD-relative|worktree-relative)[^.]*FORBIDDEN`. (e) Record M-P17-C verbatim plus the `are used` control as a paired mutant/anti-mutant with captured stdout, and re-prove all nine prior vectors RED.

### F-S2104-P17-003 — HIGH — Gate 2b is the one prose gate on the unnormalized domain, and its synonym list is closed

**Stable anchor.** `bats` T-001, the assertion introduced by the comment `# Gate 2b (RETIREMENT LANGUAGE NEGATIVE, F-S2104-P16-001(c)):`, assigning `retirement_language`.

The predicate, verbatim:

```
  retirement_language="$(printf '%s\n' "$prohibition_block" | grep -E 'formerly|retired|no longer|waived|exempt' || true)"
```

Two defects in one line. It reads `$prohibition_block` — the **raw, line-wrapped** paragraph — while every sibling prose gate reads `$joined_block_nosplit`. And its token set is a five-item closed list where the semantic class (constraint-nullifying language) is open.

**M-P17-D — exact substituted text** (synonym gap), replacing the normative prohibition paragraph:

```
All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
anchored to the main-checkout root. CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN under the original issue #523 reading; that
constraint has since been rescinded and superseded (BC-6.26.001 Invariant 5).
```

**M-P17-F — exact substituted text** (domain gap — `no longer` split across the paragraph's soft line break):

```
All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
anchored to the main-checkout root. CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN under the initial reading, but that is no
longer the operative rule (BC-6.26.001 Invariant 5).
```

**Captured stdout** (scratch copy of the full `plugins/` tree, unmodified bats suite):

```
########## M-P17-D (retirement synonym 'rescinded/superseded') ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
########## M-P17-F (line-wrapped 'no longer' evades per-line Gate 2b) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

M-P17-F is the sharper of the two: it is the *identical token* the gate was written to catch, defeated purely by where the line wraps — exactly the failure class pass-15 closed for Gates 1/4/5 and POLICY 13 codified as inadmissible at v1.4.12. Both mutants leave a rendered paragraph whose net instruction is that the CWD-relative prohibition does not apply.

**Zero-degrees-of-freedom fix predicate.** (a) Change Gate 2b's domain to `$joined_block_nosplit` (extended per F-S2104-P17-001(a)) so the predicate is rewrap-invariant, and add a rewrap mutant proving the same paragraph re-wrapped at different word boundaries stays GREEN while M-P17-F stays RED. (b) Widen the alternation to the nullification class and mutant-prove each member: `formerly|retired|rescinded|superseded|relaxed|lifted|withdrawn|rescind|no[[:space:]]+longer|not[[:space:]]+longer|waived|exempt|obsolete|deprecated|does[[:space:]]+not[[:space:]]+apply|overridden|historical[[:space:]]+only`. (c) Add the complementary structural gate that makes the synonym list non-load-bearing: assert the prohibition paragraph contains **no adversative connective** attaching to the FORBIDDEN sentence — reject `but[[:space:]]|however|except[[:space:]]+that|though[[:space:]]` within the sentence carrying `FORBIDDEN`, so nullification cannot be expressed at all regardless of which verb is chosen. (d) Record M-P17-D and M-P17-F verbatim with captured RED stdout and GREEN restores. (e) The Gate cell in story v1.21 MUST be corrected in the same burst per F-S2104-P17-007.

### F-S2104-P17-004 — HIGH — a `./`-prefixed `**Correct:**` bullet is unconstrained by all five bullet gates

**Stable anchors.** `bats` T-001, the assertions introduced by `# Gate 3 (TIGHTENED, F-S2104-P16-002):`, `# Gate 6 (two-part polarity; F-S2104-P14R-003 / F-S2104-P15-002):` and `# Gate 7 (CWD-RELATIVE BULLET POLARITY, two-part; F-S2104-P16-002):`. Target: `_shared-context.md` §Spec-Path Discipline worked-example bullet set. Contract: story AC-001(a)(i).

Gate 7(b) fires on `printf '%s\n' "$spec_path_section" | grep -E 'file_path="\.factory/' | grep -qE '\*\*Correct:\*\*'`; Gate 6(b) fires on `grep -E '\.\.\/' | grep -qE '\*\*Correct:\*\*'`. A CWD-relative target written `"./.factory/…"` matches neither: after `file_path="` the next characters are `./`, so `\.factory/` cannot match at that position, and `./.` contains no `../`. Gates 3, 6(a) and 7(a) are positive and remain satisfied by the two untouched Forbidden bullets, so nothing fires.

**M-P17-G — exact inserted text**, appended after the traversal Forbidden bullet (all three existing bullets unchanged):

```
- **Correct:** `Write(file_path="./.factory/stories/S-NNN-DELIVERY.md", ...)` (worktree-local — lands in the story worktree shadow subtree)
```

**Captured stdout** (scratch copy of the full `plugins/` tree, unmodified bats suite):

```
$ sed -n '112,116p' _shared-context.md
- **Correct:** `Write(file_path="$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md", ...)`
- **Forbidden:** `Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)` (relative path — silently writes to shadow tree)
- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative traversal — brittle and error-prone)
- **Correct:** `Write(file_path="./.factory/stories/S-NNN-DELIVERY.md", ...)` (worktree-local — lands in the story worktree shadow subtree)
$ bats -f "T-001" story-worktree-write-path-discipline.bats
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

The rendered worked-example set now presents a CWD-relative shadow-tree write as **Correct** and truthfully describes its effect, directly contradicting the Forbidden bullet two lines above and BC-6.26.001 PC1's own bullet table. This is the F-S2104-P16-002 class one hop over: that finding closed the CWD-relative bullet's *label* polarity for one exact rendering, and left every other rendering of the same write form unconstrained.

**Zero-degrees-of-freedom fix predicate.** Generalise all five bullet gates from surface renderings to the **non-canonical-target class**, over `$spec_path_section`: (a) define the canonical-target predicate as `file_path="(\$CANONICAL_FACTORY_ROOT|/)` — an absolute path or the orchestrator variable; (b) NEGATIVE — no line matching `\*\*Correct:\*\*` and containing `file_path="` may fail the canonical-target predicate, so every `**Correct:**` example must show an absolute or variable-rooted target and any relative rendering (`.factory/`, `./`, `../`, `~/`, single-quoted, unquoted) is caught by construction; (c) retain Gates 3, 6(a) and 7(a) as the positive existence assertions for the two named Forbidden forms; (d) record M-P17-G verbatim as the mutant for (b), plus a single-quoted variant `file_path='.factory/…'` and a bare `file_path=.factory/…` variant, each with captured RED stdout and a GREEN restore, and re-prove M-P16-D and M-P15-B RED.

### F-S2104-P17-005 — MEDIUM — five Pass-16 gate descriptions do not match the gates, two contradicting the same document

**Stable anchors.** `.factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md`, §`Pass-16 assertion-site attestation (9ab1aa32)` — the `test-writer 9ab1aa32 changes:` lead-in paragraph; the TIER 1 table row `M-P16-B`; and the §`Gate-indexed audit table (T-001 / AC-001 gates at 9ab1aa32 — 15 gates)` rows `Absent-block guard`, `G5 sentence-complete polarity`, `Anchor-uniqueness`, `Abbreviation-protected splitter`.

**(a) Attested abbreviation protection that does not exist.** The lead-in states `abbreviation-protected sentence splitter (`cf\.\|i\.e\.\|e\.g\.\|§[0-9]+\.` protected before split)`, and the `Abbreviation-protected splitter` row repeats `cf\.\|i\.e\.\|e\.g\.\|§[0-9]+\.`. The code protects three abbreviations and no section-number form:

```
$ grep -oE "sed 's/cf.*ABBREV.*'" story-worktree-write-path-discipline.bats
sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g'
$ grep -cE '§\[0-9\]' story-worktree-write-path-discipline.bats
0
```

The attested `§[0-9]+\.` protection is a claimed defence against a real split boundary (`§4.1. ` splits on `. `) that no code implements.

**(b) Anchor-uniqueness domain misattested twice, and the misattestation contradicts the fix.** Both the lead-in (`anchor-uniqueness gate (count `All.*\.factory.*artifact writes` = 1 in `$spec_path_section`, else ambiguous-anchor error)`) and the audit-table row (`Anchor-uniqueness | `$spec_path_section` — count of `All.*\.factory.*artifact writes``) name `$spec_path_section`. `_assert_write_discipline_anchor_unique` counts within `_extract_write_discipline_section`. The distinction is the whole substance of the F-S2104-P16-003(b) fix, and the `$spec_path_section` reading is internally inconsistent with the same document's TIER 2 row `M-P16-B-out-of-section-instantiation` and the TIER 1 preamble, both of which state the out-of-section decoy leaves the count at 1.

**(c) TIER 1 M-P16-B row attributes the wrong gate, contradicting its own preamble.** The row's Gate(s)-triggered cell reads `Anchor-uniqueness gate fires: `All.*\.factory.*artifact writes` count = 2 in `$spec_path_section`; ambiguous-anchor error`. The preamble four lines above says `M-P16-B → Gate 1(a) line 633 decoy excluded by `#### Write Discipline` bounding, anchor-uniqueness count=1`. My execution of the verbatim M-P16-B (decoy immediately before `**All spec, BC, and ADR files…`, normative paragraph inverted) agrees with the preamble:

```
##### M-P16-B out-of-section decoy (read-discipline region) #####
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# DOC-PARITY FAIL [write-discipline prohibition block affirmative-mandate (sentence-scoped, zero-DoF, F-S2104-P16-001(a))]: …
```

The closure holds; the TIER 1 row does not prove it.

**(d) Absent-block guard domain misstated.** The row reads `Absent-block guard | `$spec_path_section` non-empty`. The assertion tests `[ -z "$prohibition_block" ]` — the extracted paragraph, not the section.

**(e) The G5 row states PW-B's predicate.** The row reads `Negative: no sentence has MUST AND `(CWD-relative\|worktree-relative\|relative[[:space:]]+path\|story-worktree[[:space:]]+CWD)` without co-occurring prohibition token`. Gate 5's actual predicate has neither `story-worktree CWD` nor any prohibition-token exception — both belong to Gate PW-B, which the table omits (see F-S2104-P17-006):

```
$ sed -n '729p' story-worktree-write-path-discipline.bats
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' || true)"
```

**Zero-degrees-of-freedom fix predicate.** For each of (a)–(e), correct the text to the predicate that exists at HEAD, verified by a literal `grep` whose stdout is recorded alongside the correction: (a) state the protected set as `cf. |i.e. |e.g. ` exactly, and either implement `§[0-9]+\.` protection or delete the claim — do not attest it unimplemented; (b) state the anchor-uniqueness domain as the `#### Write Discipline` section in both sites; (c) rewrite the TIER 1 M-P16-B Gate(s)-triggered cell to `Gate 1(a) affirmative fires on the inverted normative paragraph; the decoy is excluded by `#### Write Discipline` bounding (anchor count = 1)`, matching the preamble and the recorded stdout; (d) state the absent-block guard domain as `$prohibition_block`; (e) rewrite the G5 row to Gate 5's actual alternation with no prohibition-token clause. Preserve the v1.14 entries unmodified and record the corrections as a new changelog row carrying an explicit error-acknowledgment clause, per the discipline already established at v1.14 for the F-S2104-P16-005 class.

### F-S2104-P17-006 — MEDIUM — the audit tables omit the two gates and two obligations they exist to surface

**Stable anchors.** red-gate-log v1.14, §`Gate-indexed audit table (T-001 / AC-001 gates at 9ab1aa32 — 15 gates)` and §`Obligation-indexed AC-001 coverage table`.

**(a) The 15-gate table enumerates a different 15 than the code.** It contains no row for Gate PW-B (block-wide sentence polarity) and no row for Gate 2b (retirement language) — both real, load-bearing assertions in T-001:

```
$ sed -n '602,619p' red-gate-log.md | grep -cE 'PW-B|retirement|formerly'
0
```

In their place it counts `#### Write Discipline` bounding and the abbreviation-protected splitter as gates; neither is an assertion — they are helper mechanisms whose failure produces no message. It also names the code's Gate 1(c) as `G1(b)`, diverging from both the bats comments and story v1.21's Gate cell. The count 15 therefore matches the story's count by coincidence while describing a different set, which is precisely the condition that makes a count an unreliable audit signal. The consequence is concrete and already realised: Gate 2b's absence from the audit is why its unnormalized per-physical-line domain (F-S2104-P17-003(b)) was never examined, in the same burst that codified POLICY 13's NORMALIZED-DOMAIN MANDATE.

**(b) The obligation-indexed table covers one of AC-001's three clauses.** Its five rows are all AC-001(a) or cross-cutting (`AC-001(a)(i) normative mandate`, `AC-001(a)(i) CWD-relative bullet`, `AC-001(a)(ii) traversal bullet`, `AC-001 sentence-complete polarity`, `AC-001 extraction integrity`). Story AC-001 has three enumerated clauses, verbatim:

> `(a) forbid BOTH forbidden forms named in BC-6.26.001 PC1 …; (b) mandate canonical absolute paths via `git -C <worktree> rev-parse --show-toplevel` on the MAIN worktree or `CANONICAL_FACTORY_ROOT`; (c) name the DELIVERY ledger (`*-DELIVERY.md`) and `pr-review.md` as load-bearing cases.`

Clauses (b) and (c) have no rows. They are gated — by `_assert_doc_marker 'CANONICAL_FACTORY_ROOT'`, `'DELIVERY'`, `'pr-review\.md'`, `'story-frontmatter'`, and the EC-006 `WARNING` pair — but every one of those is a presence-only marker assertion with no polarity. POLICY 15's obligation-indexed mandate says one row per enumerated AC clause, with any clause whose coverage is empty or presence-only **marked as an open gap rather than omitted**. Omitting them is the failure mode the mandate was codified to prevent, one pass after codification.

**Zero-degrees-of-freedom fix predicate.** (a) Add gate-indexed rows for Gate PW-B and Gate 2b with their actual domains, polarity directions and mutants; remove `#### Write Discipline` bounding and the abbreviation-protected splitter from the gate table into a separate `Extraction and normalization mechanisms` table (they are not assertions and must not be counted as gates); rename `G1(b)` to `G1(c)` to match the bats comments and story v1.21; state the resulting gate count only after the rows are enumerated, and cross-check it against the story Gate-cell count with literal-shell stdout. (b) Add obligation-indexed rows for AC-001(b) and AC-001(c), each naming its gate(s), the polarity direction (`presence-only — no polarity asserted`), the mutant, and an explicit `OPEN GAP` marker; the deletion mutants for those markers are already load-bearing and should be recorded as such, but the absence of polarity must be stated, not implied. (c) Add a row per newly created gate from F-S2104-P17-001..004 in the same burst.

### F-S2104-P17-007 — MEDIUM — story v1.21's Gate cell attests the normalized domain for the one gate that lacks it

**Stable anchor.** `.factory/stories/S-21.04-story-worktree-write-path-discipline.md`, §Acceptance Criteria, row `AC-001`, Gate column.

The cell reads, verbatim in relevant part:

> `gates 3–10 evaluate on the joined+abbreviation-protected (`cf./i.e./e.g.` protected) prohibition block, sentence-split on `. ` boundaries: (3) Gate 1(a) POSITIVE zero-DoF … (8) Gate 2b NEGATIVE — block must NOT contain `formerly\|retired\|no longer\|waived\|exempt`; (9) Gate 4 NEGATIVE …`

Item (8) is inside the group scoped to the joined, abbreviation-protected, sentence-split domain. Gate 2b uses neither transform:

```
$ grep -n 'retirement_language=' story-worktree-write-path-discipline.bats
694:  retirement_language="$(printf '%s\n' "$prohibition_block" | grep -E 'formerly|retired|no longer|waived|exempt' || true)"
```

A reviewer consulting the Gate cell to learn what AC-001 is gated by would conclude Gate 2b is rewrap-invariant. It is not, and M-P17-F exploits exactly that. This is the same-burst coupling the cell's own closing note establishes — `any burst that changes a T-001 predicate MUST update this Gate cell in the same burst` — inverted: the predicate and the cell were written in the same wave, and the cell describes a domain the predicate never had.

The rest of the cell is accurate and materially improved: the anchor-uniqueness domain is correctly stated as `within `#### Write Discipline``, the empty-block guard correctly as the extracted paragraph, and PW-B's three alternations match the code exactly — all three of which the red-gate-log gets wrong (F-S2104-P17-005/006). Nitpick in the same cell: item (10) quotes Gate 5's alternation as `relative[[:space:]]+paths?` where the code has `relative[[:space:]]+path` (behaviourally equivalent by substring, but POLICY 15 verbatim discipline applies).

**Zero-degrees-of-freedom fix predicate.** Move item (8) out of the joined-domain group into its own clause naming its actual domain, or — preferred, and required if F-S2104-P17-003(a) is implemented — change Gate 2b's domain to the joined block first and leave the cell's grouping correct by construction. Correct item (10)'s alternation to `relative[[:space:]]+path`. Bump the story version with all POLICY 14/17 legs, and verify with captured stdout that every regex quoted in the cell has a matching occurrence in the bats file and that every gate in the bats file has a matching cell item — the bidirectional check, not only the forward one.

---

## Observations (NOT findings)

**All nine recorded vectors independently re-proven RED at HEAD from verbatim Part A text.** M-P16-A → Gate 1(a) zero-DoF; M-P16-C2 → Gate 1(c) via the abbreviation-protected splitter; M-P16-D → tightened Gate 3; M-P16-B (out-of-section) → Gate 1(a) with the `####` bounding excluding the decoy; M-P16-B (in-section) → `_assert_write_discipline_anchor_unique`; M-P15-A, M-P14R-A and the `worktree-relative` synonym → Gate 1(a); M-P15-B → Gate 6(a). The negation-transparency and anchor-uniqueness codifications are real work and the axes they close are genuinely closed. F-S2104-P17-001/002 are gaps in the *extent and renderability of the gated domain* and the *closure of PW-B's token lists*, not a false attestation of the pass-16 fixes.

**Duplicate-heading bounding attack probed and closed.** I inserted a decoy `#### Write Discipline (summary)` heading with a compliant anchor paragraph before the real heading and inverted the real paragraph. `_assert_write_discipline_anchor_unique` fires. The mechanism is worth recording because it is accidental: `_extract_write_discipline_section`'s first awk rule ends in `next`, so a second `^#### Write Discipline` heading does not trigger the `found && /^#### / { exit }` rule and both anchors land inside one extraction, yielding count=2. The helper's docblock claims the section "exits on the next `####` sibling", which is not true of a sibling that also matches the start anchor — a latent doc/behaviour divergence that currently fails *safe*. Worth a comment correction in the F-S2104-P17-001 burst, but not a finding: no mutant survives.

**Unicode/homoglyph substitution probed and closed.** Replacing the ASCII hyphen in `CWD-relative` with U+2011 (non-breaking hyphen) does not evade the gates: `relative[[:space:]]+paths?` still matches the trailing `relative paths` substring, and PW-B fires. The alternation's redundancy — listing both the compound `CWD-relative` and the bare `relative paths?` — is what saves it here. Recorded so the fix for F-S2104-P17-002(b) preserves that redundancy rather than collapsing the alternation to compounds.

**BC-6.27.001 v1.4 carries no obligation inside S-21.04's delivery perimeter.** Its PC1 (5-step factory-side restore sequence) and PC2 (dispatch-preamble branch assertion) govern pr-manager and are S-21.05's contract. The story's `behavioral_contracts` frontmatter is `[BC-6.26.001]`, and zero files in the 19-file diff reference BC-6.27.001 (literal per-file `grep -c` over the diff name-list, all zero). No parity leg is implicated.

**Story ↔ STORY-INDEX ↔ epic parity holds.** STORY-INDEX `version: "4.264"`; the S-21.04 catalog row cites `story v1.21`, `[BC-6.26.001 v1.11]`, `input-hash 1165b1f`, and Refs terminating in `F-S2104-P16-001..006 (pass-16)`. The `> **E-21 delivery:**` blockquote's `S-21.04=1165b1f` matches the live story frontmatter exactly and all six values remain distinct. The story's live-body BC table pins `v1.11`, matching BC-6.26.001's frontmatter. Sibling-sweep for stale `story v1.20` live-body pins returns six hits in STORY-INDEX, all of them E-19 catalog rows (S-19.03..S-19.07 and the E-19 delivery blockquote) whose own versions happen to be v1.20 — zero S-21.04-related stale pins. Zero live `red-gate-log v1.13` pins in the story.

**Zero live POLICY 19 tokens in the S-21.04 perimeter.** `grep -cE 'ADR-031 v[0-9]'` returns zero across the story, BC-6.26.001, BC-6.27.001 and red-gate-log outside frontmatter/changelog sites.

**CHANGELOG is accurate.** The pass-15 count-free lead-in survives unchanged (`the two BC-6.26.001 protocol requirements plus the propagation and awareness legs`), items (1)–(5) match the delivered surfaces, the five sibling teardown sites named (`worktree-manage, code-delivery, fix-pr-delivery, code-delivery.lobster, greenfield.lobster`) all appear in the 19-file diff, and the `bcs:` → `behavioral_contracts:` claim of "5 sites" matches `grep -c 'behavioral_contracts:' adversary.md` → 5.

**F-S2104-P16-005 and -006 record corrections both landed in full.** red-gate-log line 455 now reads `**M-P15-A-simplified proof (RED)** [Correction at v1.14 (D-916): the vector recorded here was a simplified form used in the original pass-15 attestation; the adversary-verbatim M-P15-A appears in the Pass-16 TIER 1 section below.]`, the pass-15 closing completeness claim is replaced with the qualified inventory statement plus an explicit error acknowledgment naming all four surviving vectors, and `adversary-pass-15.md` §Fix Mapping row F-S2104-P15-005 now carries `[Corrected at D-916 per F-S2104-P16-006: an earlier clause attesting a T-009 bats-comment update was erroneous — no pass-15 commit touched the bcs: gate or its comment.]`. The two-tier TIER 1 / TIER 2 structure with `-instantiation` suffixes is a genuine improvement and correctly discloses which vectors are simplified.

**[process-gap] — four passes of hardening have each constrained what the predicate says, never what text it is handed.** Pass-13 added polarity gates; pass-14R widened the token alternation to a syntactic-form class; pass-15 normalized the evaluation domain from physical line to reflowed sentence; pass-16 made the predicate negation-transparent and the extraction anchor unique. Every one was the right fix for the vector in front of it, and every one left the *domain boundary* at "the first paragraph after the anchor, terminated by the first blank line" and the *visibility* of that text unconstrained. Candidate codification, in two parts: **(i) domain-completeness** — a negative (prohibition-asserting) prose gate MUST evaluate the entire bounded section it governs, never a single paragraph within it; paragraph-scoping is admissible only for positive existence assertions, and every negative gate MUST carry a mutant that places the harmful text in a sibling paragraph of the same section; and **(ii) render-fidelity** — text that satisfies a positive doc-parity gate MUST be text a Markdown renderer displays; any gate domain MUST either exclude HTML-comment spans before evaluation or assert their absence, with a comment-hidden-mandate mutant recorded. Either one alone would have caught this pass.

**[process-gap] — closed token alternations are the recurring shape, and POLICY 13's mutant-derived requirement does not force closure.** POLICY 13 mandates "mutant-derived syntactic-class alternations + synonym mutant", and the wave complied in form: PW-B's prohibited-target list was derived from M-P16-A's text and Gate 2b's from M-P16-A's retirement phrasing. Both remain the *specific words the adversary happened to use*, and both fell to the first paraphrase I tried. The asymmetry is structural: an alternation that must *match* to fire (a negative gate's trigger) fails open on every unlisted synonym, whereas an alternation that must match to *pass* fails closed. Candidate codification: for any negative prose gate, the alternation MUST appear in the **failing** position of a logical implication whose other side is an open class — i.e. prefer "every sentence containing X must also contain a prohibition token" (open X, closed prohibition set, fails closed on paraphrase of X) over "no sentence may contain X together with Y" (closed X and Y, fails open on paraphrase of either). Concretely, the required review question is: *if the author paraphrases, does the gate fire or fall silent?* Any negative gate that falls silent on paraphrase is a paper-gate under TD-VSDD-059 regardless of how many mutants it is proven against, because the mutants are drawn from the same vocabulary as the predicate.

**[process-gap] — the audit table's count matched while its contents diverged.** Story v1.21 and red-gate-log v1.14 both say "15 gates" and enumerate different sets (F-S2104-P17-006(a)). The count agreed because two real assertions were dropped and two non-assertions added — a compensating error that no count-based check can see. Candidate codification: cross-document gate-inventory parity MUST be asserted by **name-set equality**, not cardinality — a literal-shell diff of the sorted gate labels extracted from the story Gate cell against those extracted from the red-gate-log audit table, with the diff's stdout recorded. This is the D-449(a) literal-shell principle applied to an inventory rather than a single field, and it is mechanically available today.

---

## Per-Pass-16 Verification

| Finding | Status | Notes |
|---------|--------|-------|
| F-S2104-P16-001 | PARTIAL (7th-gen re-seed) | The pass-16 legs all landed and are load-bearing: Gate 1(a) is now zero-DoF `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute`, Gate 1(b) is the paired negation-transparency gate, Gate PW-B adds block-wide sentence polarity, Gate 2a is sentence-scoped, Gate 2b guards retirement language, and the `cf./i.e./e.g.` abbreviation-protected splitter exists. I re-proved verbatim M-P16-A RED (Gate 1(a)) and verbatim M-P16-C2 RED (Gate 1(c)), and M-P15-A / M-P14-A / M-P14R-A / `worktree-relative` all remain RED. The negation axis is genuinely closed. But PW-B's directive-token list is closed, so M-P17-C inverts the paragraph end-to-end at 9/9 (control confirms the directive-token gap is load-bearing); the prose domain is one paragraph, so M-P17-A's sibling paragraph is unexamined at 9/9; no gate is comment-aware, so M-P17-H satisfies the positive gates with invisible text at 9/9; Gate 2b is per-physical-line with a closed synonym list, so M-P17-D and M-P17-F both survive → F-S2104-P17-001, -002, -003 |
| F-S2104-P16-002 | PARTIAL | Gate 7(a)/(b) exist as specified and Gate 3 is tightened to require `file_path="\.factory/` on the same line; I re-proved verbatim M-P16-D RED (tightened Gate 3 fires first, Gate 7 also). AC-001(a)(i)'s bullet label polarity is closed for the `".factory/…"` rendering. But all five bullet gates pin exact surface renderings, so a `**Correct:**` bullet with `file_path="./.factory/…"` matches neither `file_path="\.factory/` nor `\.\./`: M-P17-G adds it at 9/9 → F-S2104-P17-004 |
| F-S2104-P16-003 | CONFIRMED-CLOSED | Both legs present and load-bearing. `_extract_write_discipline_section` bounds extraction to the `#### Write Discipline` child heading; `_assert_write_discipline_anchor_unique` counts the anchor within that section and fails with an explicit `ambiguous anchor` message unless the count is 1. Verbatim M-P16-B (decoy in the read-discipline region) → RED via Gate 1(a), with the decoy excluded by bounding and the count at 1; the in-section decoy → RED via anchor-uniqueness (count 2). I additionally probed a duplicate-`#### Write Discipline`-heading decoy — also RED (both anchors land in one extraction). No surviving mutant on this axis. The record's description of which gate fires for the verbatim vector is wrong → F-S2104-P17-005(b)(c) |
| F-S2104-P16-004 | CONFIRMED-CLOSED | Story `version: "1.21"`; the AC-001 Gate cell enumerates the actual HEAD gate set with correct regexes for anchor-uniqueness (`within #### Write Discipline`), the empty-block guard, Gates 1(a)/(b)/(c), PW-B's three alternations, 2a, 2b, 4, 5, 3, 6(a)/(b), 7(a)/(b); the retired `not canonical absolute\|not absolute` predicate and the phrases `per extracted line` / `per line` are absent; the same-burst coupling note is present. `last_amended` + `modified[v1.21]` both record the change. One domain misdescription and one regex nitpick remain → F-S2104-P17-007 |
| F-S2104-P16-005 | PARTIAL | (a) The falsified closing line is replaced with `Gate inventory as of this HEAD (8b39277b); polarity coverage proven for the mutants listed per row. **Not a completeness claim…**` plus an explicit error-acknowledgment naming M-P16-A/C2/D/B, and an obligation-indexed coverage table was added — the mechanism requested. But the table covers only AC-001(a); AC-001(b) and AC-001(c) are omitted rather than marked open despite being presence-only gated, so the inversion POLICY 15 codified is not actually achieved → F-S2104-P17-006(b). (b) CONFIRMED-CLOSED: the mislabelled block is now `**M-P15-A-simplified proof (RED)** [Correction at v1.14 (D-916)…]` and the adversary-verbatim five-line M-P15-A is recorded in TIER 1 with its re-proof; dual independent execution is disclosed. (c) CONFIRMED-CLOSED: v1.13 content preserved, correction recorded as a new changelog row with error acknowledgment |
| F-S2104-P16-006 | CONFIRMED-CLOSED | `adversary-pass-15.md` §Fix Mapping row F-S2104-P15-005 now reads `story-writer 6fccdcc3 — story v1.20 option-a: AC-010 carve-out dropped; code-span prohibition rationale stated inline; gate unchanged (already matches the widened criterion). [Corrected at D-916 per F-S2104-P16-006: an earlier clause attesting a T-009 bats-comment update was erroneous — no pass-15 commit touched the bcs: gate or its comment.]` — the unlanded clause is dropped and the error is acknowledged in place, exactly as the fix predicate specified |

Tally: **3 CONFIRMED-CLOSED / 3 PARTIAL / 0 REGRESSED** against the pass-16 finding set. No regression this pass — notably, the F-S2104-P16-004 story Gate-cell leg that pass-16 found regressed is now closed *and* carries the same-burst coupling note that prevents its recurrence, and the two record-correction findings closed cleanly with error acknowledgments. The three PARTIALs are one-hop re-seedings, but the mechanism has again changed character: passes 12–16 each found the predicate defeated at a finer *granularity* of the same text, whereas this pass found that the text handed to the predicates is neither the whole section (M-P17-A), necessarily visible (M-P17-H), nor described by a closed vocabulary (M-P17-C, M-P17-D, M-P17-F, M-P17-G). Every fix predicate above is mechanically checkable and none requires a new spec decision.

---

## Fix Mapping (fixes_landed_head: c89bef22)

| Finding | Severity | Role | Commit | What landed |
|---------|----------|------|--------|-------------|
| F-S2104-P17-001 | BLOCKER | test-writer | 2e70faa8 | `write_discipline_prose_nosplit` built from whole `#### Write Discipline` section (fenced code excluded); HTML-comment absence gate added; Gate 1(d) conditional-scoping predicate added to mandate sentence; all existing gates extended to section-wide domain; 9/9 + 14/14 GREEN at c89bef22 |
| F-S2104-P17-002 | BLOCKER | test-writer | 2e70faa8 + 1859ef70 + c89bef22 | Gate PW-B rewritten: directive-token whitelist replaced by prohibition-token requirement (`FORBIDDEN\|Forbidden\|forbidden\|MUST NOT\|prohibited\|never\|forbid`); prohibited-target class extended to syntactic-form class (`story-worktree CWD`, `shadow subtree`, `worktree-local`, `in-worktree`, `story worktree CWD`, `worktree CWD`); word-boundary `(^\|[^[:alnum:]])[Ii]n-worktree` added at 1859ef70; `[Ww]orktree-local` bracket-class added at c89bef22; Gate 1(d) closes conditional-scoping axis |
| F-S2104-P17-003 | HIGH | test-writer | 2e70faa8 | Gate 2b(a) domain changed from per-physical-line `$prohibition_block` to `write_discipline_prose_nosplit` sentence-split (rewrap-invariant); nullification class widened (`rescinded`, `superseded`, `no longer`, `obsolete`, `deprecated`, plus 10 synonyms); Gate 2b(c) adversative-connective gate added (catches `but[[:space:]]\|however\|except[[:space:]]+that\|though[[:space:]]` within FORBIDDEN sentences) |
| F-S2104-P17-004 | HIGH | test-writer | 2e70faa8 | `canonical-target` gate added: every `**Correct:**` bullet with `file_path=` must satisfy `file_path=["']?($CANONICAL_FACTORY_ROOT\|/)` — catches `./.factory/`, `../../.factory/`, bare `.factory/`; Gates 6(b)/7(b) RETIRED (blind to `./.factory/` form) |
| F-S2104-P17-005 | MEDIUM | state-manager | this burst (D-918) | red-gate-log.md v1.14→v1.15: (a) abbreviation splitter corrected to 3 forms (`cf.`, `i.e.`, `e.g.`), §[0-9]+. claim retracted; (b) anchor-uniqueness domain corrected to `#### Write Discipline` section at both attestation sites; (c) M-P16-B TIER 1 Gate cell corrected (Gate 1(a) affirmative fires on inverted paragraph; decoy excluded by bounding, count=1, not anchor-uniqueness count=2); (d) absent-block guard domain corrected to `$prohibition_block`; (e) Gate 5 row corrected (actual alternation `CWD-relative\|worktree-relative\|relative[[:space:]]+path`, no prohibition-token clause) |
| F-S2104-P17-006 | MEDIUM | state-manager | this burst (D-918) | red-gate-log.md v1.15: gate-indexed audit table rebuilt to 17 gates — Gate PW-B and Gate 2b(a)/(c) rows added; bounding + abbreviation splitter moved to separate `Extraction and normalization mechanisms` table; `G1(b)` renamed to `G1(c)`; Gates 6(b)/7(b) RETIRED rows added; obligation-indexed table rebuilt with AC-001(b) (CANONICAL_FACTORY_ROOT + EC-006 WARNING gates) and AC-001(c) (DELIVERY + pr-review.md + story-frontmatter gates) rows; NAME-SET EQUALITY literal-shell diff recorded |
| F-S2104-P17-007 | MEDIUM | story-writer | c89bef22 | story v1.21→v1.22: Gate cell item (8) Gate 2b moved out of joined-domain group into own clause naming `write_discipline_prose_nosplit` domain (section-wide sentence-split); item (10) alternation corrected to `relative[[:space:]]+path`; 17-gate count updated; POLICY 14/17 parity legs applied |
