---
pass: 18
verdict: NOT-CLEAN
reviewed_head: c89bef22
novelty: 0.62
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-17.md"
---

## Summary

Pass-18 fresh-context adversarial review of S-21.04 at reviewed_head `c89bef22` (worktree `.worktrees/S-21.04`, base develop `948f0fb1`). **7 findings: B2 / H3 / M2.** Novelty 0.62 vs pass-17 Part A. Trajectory 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7. Streak: **0/3** (BC-5.39.001 reset).

Baseline by literal execution at HEAD: `bats story-worktree-write-path-discipline.bats` → `1..9`, 9/9 ok; `bats worktree-identity-preflight.bats` → `1..14`, 14/14 ok.

The pass-17 wave did substantial, verified work. I independently re-proved **fifteen** recorded vectors RED at HEAD from their verbatim Part A / battery text: M-P16-A, M-P16-C2, M-P16-D, M-P16-B (in-section decoy), M-P15-A, M-P15-B, M-P14R-A, the `worktree-relative` synonym, M-P17-A (verbatim, second paragraph), M-P17-C, M-P17-D, M-P17-F, M-P17-G, M-P17-H, and the single-quoted + bare `**Correct:**`-bullet variants. The whole-section domain genuinely reaches every paragraph, Gate 1(d) genuinely closes conditional scoping, the canonical-target gate genuinely generalises the bullet class beyond surface renderings, and Gate 2b(a) is genuinely rewrap-invariant. None of my findings is a false attestation of a pass-17 fix.

**The gate set is nevertheless defeated five independent ways, and — for the first time in this cascade — two of the five are defeated *by the pass-17 fix mechanisms themselves* rather than by axes the fix did not reach.** The whole-section prose domain is constructed by *subtracting* fenced code blocks; that subtraction is an unconditional fail-open hole, and a single unbalanced opening fence deletes the entire section-wide domain in three characters. The sentence-splitter that makes the domain rewrap-invariant is driven by a three-item abbreviation whitelist; an unlisted ordinary abbreviation (`No. `) manufactures a false sentence boundary that silences Gate 4 and Gate 5, which — unlike PW-B — were never converted to the fail-closed implication form. So the pass-17 burst simultaneously codified POLICY 13's FAIL-CLOSED-IMPLICATION-DIRECTION clause and shipped two negative gates in the inadmissible closed-conjunction form, plus one new fail-open exclusion.

The other three are one-hop re-seedings on the axes pass-17 opened but did not close as classes rather than as lists. PW-B's *prohibition-token* side is now closed-in-failing-position, exactly as the process-gap note prescribed; its *trigger* side — which the same note required to be an **open class** — is still a ten-item surface list, and an ordinary paraphrase of the issue #523 destination ("the worktree's `.factory/` subtree") walks straight through it. Gate 2b's nullification class is likewise a list, and three separate unlisted nullifications survive in a sibling paragraph because the adversative-connective gate that was supposed to make the list non-load-bearing is scoped to the prohibition paragraph while the synonym gate is section-wide. And the render-fidelity gate asserts the absence of one surface form (`<!--`) rather than establishing that the gated domain equals the rendered domain, so the standard Markdown comment idiom `[//]: # (…)` — rendered as nothing by CommonMark and GitHub alike — hides the whole compliant mandate at 9/9.

On the record side the pass-17 attestation is a clear improvement and the two mechanisms POLICY 15 mandated at v1.4.14 both actually work: I reproduced the NAME-SET EQUALITY check at HEAD (`diff` empty, 17 labels, `grep -oP` yields exactly the enumerated set) and the obligation-indexed table now carries AC-001(b) and AC-001(c) rows with explicit OPEN GAP markers. Two defects remain: one gate attribution is false at two sites and is contradicted by the document's own captured stdout, and the input-hash transition is recorded as a bracketed placeholder where every sibling row records the literal value.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P18-001 | BLOCKER | `story-worktree-write-path-discipline.bats` §T-001 Gate PW-B (`polarity_violations`) vs `_shared-context.md` `#### Write Discipline` | PW-B's prohibited-target alternation is still a closed ten-item surface list, i.e. a **closed trigger**, which POLICY 13's FAIL-CLOSED-IMPLICATION-DIRECTION clause declares inadmissible. An ordinary paraphrase of the issue #523 destination evades it: M-P18-A adds a sibling paragraph mandating writes to `` the worktree's `.factory/` subtree `` at 9/9 — no listed form matches, because `worktree's[[:space:]]+shadow` needs the word "shadow" and `shadow[[:space:]]+subtree` needs adjacency. Eighth-generation recurrence of the primary BC postcondition remaining invertible | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13, 15; TD-VSDD-059 |
| F-S2104-P18-002 | BLOCKER | `bats` §T-001 `write_discipline_prose` construction (the fenced-code-exclusion `awk`) | The whole-section domain that F-S2104-P17-001(a) created is punctured by its own fenced-code **exclusion**, an unconditional fail-open hole in all four section-wide negative gates. (a) M-P18-C puts a harmful directive carrying the *listed* prohibited-target `worktree CWD` inside the existing `` ```bash `` block at 9/9 — rendered to the reader, invisible to every gate. (b) M-P18-C(b) inserts a single unbalanced opening fence after the prohibition paragraph, deleting the remainder of the section from the prose domain, at 9/9 | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13 (DOMAIN-COMPLETENESS), 15 |
| F-S2104-P18-003 | HIGH | `bats` §T-001 Gates 4 and 5 + the abbreviation-protected splitter | Gates 4 and 5 are closed-conjunction negatives (`no sentence contains both X and Y`) evaluated on a splitter whose abbreviation protection is a three-item list (`cf. `, `i.e. `, `e.g. `). An unlisted ordinary abbreviation manufactures a false sentence boundary that separates the conjuncts and silences the gate. M-P18-B — `Canonical absolute artifact-write targets, per issue No. 523, are FORBIDDEN for ledgers created inside the delivery sandbox.` — survives at 9/9; the control without `No. ` fires Gate 4. Neither gate was converted to the fail-closed implication form in the burst that codified the requirement | BC-6.26.001 PC1; POLICY 11, 13 (FAIL-CLOSED-IMPLICATION-DIRECTION, NORMALIZED-DOMAIN), 15 |
| F-S2104-P18-004 | HIGH | `bats` §T-001 HTML-comment-absence gate vs `_shared-context.md` `#### Write Discipline` | The render-fidelity gate asserts the absence of one surface form (`<!--`) instead of equating the gated domain with the rendered domain. M-P18-D hides the entire compliant mandate in a `[//]: # (…)` link-reference definition — the canonical Markdown comment idiom, rendered as nothing by CommonMark and GitHub — leaving one visible sentence directing writes to the delivery sandbox root, at 9/9. One-hop re-seed of the F-S2104-P17-001(b) class | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13 (RENDER-FIDELITY), 15 |
| F-S2104-P18-005 | HIGH | `bats` §T-001 Gate 2b(a) nullification class + Gate 2b(c) adversative gate | Gate 2b's nullification alternation is a closed sixteen-item list and the adversative-connective gate that was supposed to make that list non-load-bearing is scoped to `joined_block_nosplit` (prohibition paragraph) while 2b(a) is section-wide — so nullification in a sibling paragraph escapes both. Three vectors survive at 9/9: M-P18-E (`it does not bind`), M-P18-F (`has been supplanted`), M-P18-G (`however current practice permits …`). Each leaves a rendered section whose net instruction is that the prohibition does not apply | BC-6.26.001 PC1; POLICY 11, 13, 15 |
| F-S2104-P18-006 | MEDIUM | red-gate-log v1.15 §`Battery table — vectors at c89bef22` row `M-P17-A`; §`Gate-indexed audit table (… 17 gates)` row `Gate 5` | Both sites attest that Gate 5 fires on M-P17-A. It does not — Gate 5's alternation is `CWD-relative\|worktree-relative\|relative[[:space:]]+path` and M-P17-A's sentence contains none of them. The document's own captured stdout for M-P17-A shows Gate PW-B alone, so the claim is self-contradicted in the same file. Same class as F-S2104-P17-005(c), one pass after its correction | POLICY 15 (verbatim + attestation-location), 3, 4; TD-VSDD-059; D-448(a) class |
| F-S2104-P18-007 | MEDIUM | red-gate-log v1.15 frontmatter `last_amended` + `modified[D-918]` | The input-hash transition is recorded as a bracketed placeholder — `input-hash e6c640a→[updated by compute-input-hash]` in `modified[]` and `input-hash e6c640a→[see frontmatter]` in `last_amended` — where all nine sibling rows record literal old→new values (`3d12427→1baca60`, `c74e0f8→3d12427`, …). The value is knowable and present: frontmatter carries `input-hash: "4b26b3b"` | POLICY 15 (verbatim), 18; CLAUDE.md Canonical Principle Rule 6 (no placeholders when answerable in scope) |

---

### F-S2104-P18-001 — BLOCKER — PW-B's trigger is a closed list, so an ordinary paraphrase of the #523 destination walks through

**Stable anchors.** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`, T-001 (`@test "T-001 S-21.04 AC-003: stray-file-blocks …"`), the assertion introduced by the comment `# Gate PW-B (SECTION-WIDE SENTENCE POLARITY, F-S2104-P16-001(b) strengthened F-S2104-P17-002):` — the block assigning `polarity_violations`. Target: `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`, `` #### Write Discipline — `.factory/**` artifact writes from story worktrees ``. Contract: BC-6.26.001 PC1 and Invariant 1 ("Worktree-relative paths are **categorically** forbidden for `.factory/**` writes").

The predicate, verbatim:

```
  polarity_violations="$(printf '%s\n' "$write_discipline_prose_nosplit" | sed 's/\. /\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree' | \
    grep -Ev 'FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid' || true)"
```

The pass-17 fix inverted the *prohibition* side correctly: the directive-token whitelist is gone and the prohibition-token set now sits in the failing position, so a paraphrase of the prohibition cannot exempt a sentence. But POLICY 13's FAIL-CLOSED-IMPLICATION-DIRECTION clause requires the *other* side — the trigger — to be an **open class**. It is a ten-member closed surface list, and F-S2104-P17-002(b) responded to the closure by adding six more members to the list. Adding members to a closed list does not open it.

The gap is not exotic. Two of the listed members are compounds requiring exact adjacency: `worktree's[[:space:]]+shadow` requires the literal word "shadow" after the possessive, and `shadow[[:space:]]+subtree` requires "shadow" and "subtree" adjacent. The live document's own phrasing is `` the story worktree's shadow `.factory/` subtree `` — matched only by the first compound, via "worktree's shadow". Drop the word "shadow" and neither compound matches, even though the referent is identical and the sentence names the exact subtree that destroyed the issue #523 artifacts.

**M-P18-A — exact inserted text**, placed immediately after the normative prohibition paragraph's terminating blank line (between it and `**Load-bearing cases (BC-6.26.001 Invariant 4):**`), nothing else changed:

```

**Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every
`.factory/**` artifact write to the worktree's `.factory/` subtree; the canonical-absolute form
applies to spec reads from the main checkout.
```

**Captured stdout** (scratch copy of the full `plugins/` tree, unmodified bats suite):

```
$ sed -n '64,78p' _shared-context.md
#### Write Discipline — `.factory/**` artifact writes from story worktrees (BC-6.26.001 PC1, Invariants 1, 3, 4)

All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
anchored to the main-checkout root. CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN — such writes land silently in the story
worktree's shadow `.factory/` subtree and are permanently destroyed at teardown (issue #523
gitignored-shadow mechanism; BC-6.26.001 Invariant 5).

**Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every
`.factory/**` artifact write to the worktree's `.factory/` subtree; the canonical-absolute form
applies to spec reads from the main checkout.

**Load-bearing cases (BC-6.26.001 Invariant 4):** The DELIVERY ledger (`*-DELIVERY.md`),
story-frontmatter files, and `pr-review.md` records are the primary artifacts at risk. All writes
to any `.factory/**` path are covered by this rule — not only DELIVERY ledgers.
$ bats story-worktree-write-path-discipline.bats
1..9
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
ok 2 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
$ bats worktree-identity-preflight.bats
1..14
```

Gate PW-B is not triggered (no listed target form). Gate 5 is not triggered (`MUST` is present but no `CWD-relative|worktree-relative|relative path` form). Gate 4 is not triggered (the `absolute` sentence carries no `FORBIDDEN`). Gate 2b(a) is not triggered (no nullification synonym; "exception" is not "exempt"). Anchor-uniqueness is unaffected — the paragraph does not begin with `All`, so the count stays 1. Gates 1(a)–(d), 2a and 2b(c) are scoped to the untouched prohibition paragraph and all pass. The rendered section now contains the mandate and its own negation, and the negation covers the only case that caused the data loss.

Severity is BLOCKER on eighth-generation recurrence of the story's primary BC postcondition remaining invertible (F-P12-003 → F-P13-001 → F-P14-001 → F-S2104-P14R-001 → F-S2104-P15-001 → F-S2104-P16-001 → F-S2104-P17-002 → here), via a mechanism requiring no negation, no abbreviation, no decoy, no comment and no conditional — only a synonym for a subtree.

**Zero-degrees-of-freedom fix predicate.** (a) Add a **positive, open-trigger** gate over the section-wide domain that does not depend on enumerating destinations: every sentence in `write_discipline_prose_nosplit` that contains a write-directive — `MUST` or `SHOULD` **and** one of `anchor|write|writes` — MUST either carry a prohibition token from PW-B's set, or match the affirmative canonical mandate `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute`. Any other write-directive sentence is by construction a competing mandate and is the defect, whatever destination it names. I verified both directions by literal shell against HEAD's pristine text and against M-P18-A:

```
$ ex /tmp/p18_pristine.md | sed 's/\. /\n/g' | grep -E 'MUST|SHOULD' | grep -E 'anchor|write|writes' \
    | grep -Ev 'FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid' \
    | grep -Ev 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute'
(empty) OK
$ ex "$SC_with_M-P18-A" | sed 's/\. /\n/g' | grep -E 'MUST|SHOULD' | grep -E 'anchor|write|writes' \
    | grep -Ev 'FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid' \
    | grep -Ev 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute'
 **Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every `.factory/**` artifact write to the worktree's `.factory/` subtree; the canonical-absolute form applies to spec reads from the main checkout
```

The predicate is empty on HEAD (no false positive against the mandate sentence, the `MUST be determined` root-resolution sentence, the `MUST resolve it` sentence, the `never expand` sentence, or the `**Forbidden:**` bullets) and fires on M-P18-A. (b) Retain PW-B unchanged as the destination-specific layer — do not replace it; the two are complementary and PW-B carries the recorded M-P17-A / M-P17-C / in-worktree-residual proofs. (c) Record M-P18-A verbatim with captured RED stdout and a GREEN restore, plus a control that changes only `MUST anchor` to `MUST use canonical absolute` and stays GREEN, proving the affirmative-escape clause is load-bearing rather than vacuous. (d) The fix MUST retain RED for all fifteen vectors this review re-proved RED at HEAD (enumerated in Observations).

### F-S2104-P18-002 — BLOCKER — the whole-section domain is built by subtraction, and the subtraction is unconditional

**Stable anchors.** `bats` T-001, the block introduced by the comment `# Build section-wide prose domain: strip fenced code blocks, reflow, abbreviation-protect.`, assigning `write_discipline_prose` and `write_discipline_prose_nosplit`. Target: `_shared-context.md` `#### Write Discipline`, the `**Canonical root determination (BC-6.26.001 Invariant 3):**` bash fence.

The construction, verbatim:

```
  write_discipline_prose="$(printf '%s\n' "$write_discipline_section" | \
    awk '/^[[:space:]]*```/{in_fence=!in_fence; next} !in_fence{print}' | tr '\n' ' ')"
```

Every section-wide negative gate — PW-B, 2b(a), 4, 5 — reads this domain. The exclusion is unconditional: it removes fenced content from the gates' view regardless of what that content says. Fenced content is nonetheless *rendered*, and in this document the fence is a prescriptive bash recipe that agents are instructed to run. The stated rationale in the code comment is "to avoid false-positive gate fires on code examples that legitimately mention relative paths in a Forbidden example context" — but the section's only fence is the `git worktree list --porcelain` recipe, which mentions no relative `.factory/` path at all, so the exclusion buys nothing and costs the whole domain.

**M-P18-C — exact inserted text**, appended inside the existing bash fence immediately after the `CANONICAL_FACTORY_ROOT="$(git -C "$main_worktree_path" rev-parse --show-toplevel)"` line, nothing else changed:

```
  # Ledger writes are anchored to the worktree CWD, not the main checkout:
  DELIVERY_PATH=".factory/stories/S-NNN-DELIVERY.md"
```

**Captured stdout:**

```
$ sed -n '100,108p' _shared-context.md
    echo "HALT: canonical factory root could not be resolved — git worktree list returned no main worktree path; cannot proceed"
    exit 1
  }
  CANONICAL_FACTORY_ROOT="$(git -C "$main_worktree_path" rev-parse --show-toplevel)"
  # Ledger writes are anchored to the worktree CWD, not the main checkout:
  DELIVERY_PATH=".factory/stories/S-NNN-DELIVERY.md"
  ```

**WARNING (EC-006):** Running `git -C <story-worktree-path> rev-parse --show-toplevel` — i.e.,
$ bats story-worktree-write-path-discipline.bats
1..9
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

The inserted comment carries `worktree CWD`, which **is** a listed PW-B prohibited-target member, with no prohibition token. PW-B would fire on this exact string anywhere else in the section. It does not fire here solely because the fence removed the line before evaluation.

**M-P18-C(b) — the same hole used to delete the entire domain.** Exact inserted text, placed immediately after the prohibition paragraph's terminating blank line:

```
```text
Writers MUST anchor every `.factory/**` artifact write to the story worktree CWD.
```

An unbalanced opening fence flips `in_fence` and never flips it back, so every remaining line of the section — including the inserted harmful sentence, which carries the listed member `story worktree CWD` — is dropped from `write_discipline_prose`. **Captured stdout:**

```
########## M-P18-C(b) unbalanced fence ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

Three characters silence PW-B, Gate 2b(a), Gate 4 and Gate 5 across the whole section. This is a strictly worse failure than the paragraph-scoping F-S2104-P17-001(a) closed, because it is attacker-controlled from inside the document and leaves no trace in any gate's output.

For the record I also probed the opposite polarity and confirmed it fails **safe**: a `~~~` (tilde) fence is not recognised by the `awk` rule, so tilde-fenced content is *included* in the prose domain and remains gated.

```
$ printf '~~~\nHARMFUL worktree CWD line\n~~~\n' | awk '/^[[:space:]]*```/{f=!f;next} !f{print}'
~~~
HARMFUL worktree CWD line
~~~
```

**Zero-degrees-of-freedom fix predicate.** (a) Replace the exclusion with a **balanced-fence assertion plus in-fence gating**: first assert the `#### Write Discipline` section contains an even number of fence markers (`grep -cE '^[[:space:]]*```'` is even, else RED with an explicit `unbalanced code fence in gated section` message), so the domain cannot be silently truncated. (b) Do **not** exempt fenced content from the prohibited-target gates. Apply PW-B and Gate 2b(a) to the *entire* section including fenced lines; the exclusion's stated justification does not hold at HEAD, and if a future fence legitimately needs to show a forbidden form, the correct mechanism is the same one the bullets use — require the surrounding line to carry a prohibition token — not a blanket exemption. Verify by literal shell that PW-B, 2b(a), 4 and 5 are all empty over the unexcluded pristine section before landing. (c) Record M-P18-C and M-P18-C(b) verbatim with captured RED stdout and GREEN restores, plus the `~~~` control documenting that tilde fences fail safe, so a future fence-handling change cannot silently invert that. (d) If the fenced-code exclusion is retained for Gates 4 and 5 only (they legitimately co-occur `absolute` with prose that a bash recipe could trip), state that asymmetry explicitly in the code comment, the story Gate cell and the audit table in the same burst — an undocumented per-gate domain difference is precisely what let F-S2104-P17-003(b) through.

### F-S2104-P18-003 — HIGH — Gates 4 and 5 are closed conjunctions on a splitter with a closed abbreviation list

**Stable anchors.** `bats` T-001, the assertions introduced by `# Gate 4 (NEGATIVE, section-scoped; F-S2104-P14-001 / F-S2104-P15-001 / F-S2104-P17-001(a)):` (assigning `forbidden_absolute_sentences`) and `# Gate 5 (NEGATIVE, section-scoped; F-S2104-P14R-001(b) / F-S2104-P15-001 / F-S2104-P17-001(a)):` (assigning `must_relative_sentences`); and the splitter in the `write_discipline_prose_nosplit` / `joined_block_nosplit` assignments.

The predicates, verbatim:

```
  forbidden_absolute_sentences="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    sed 's/\. /\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)' || true)"
```
```
  must_relative_sentences="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    sed 's/\. /\n/g' | grep -E 'MUST' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' || true)"
```

Both are of the form POLICY 13's FAIL-CLOSED-IMPLICATION-DIRECTION clause declares a paper-gate: *no sentence may contain X together with Y*, with X and Y both closed. Two closed sides means two ways to fall silent — and a third, structural one that PW-B does not share: because the conjunction is evaluated *within* a sentence, anything that changes where sentences begin changes what the gate can see. The splitter's abbreviation protection is `sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g'` — three forms. Every other `<token>. ` in English manufactures a false boundary. Pass-17 already flagged the unimplemented `§[0-9]+\.` claim as a *record* defect (F-S2104-P17-005(a)) and the fix correctly retracted the claim; nobody asked whether the missing protection was itself exploitable. It is.

**M-P18-B — exact substituted text**, replacing the normative prohibition paragraph's final sentence terminator and appending one sentence (mandate sentence S1 and the CWD-relative FORBIDDEN sentence S2 both untouched):

```
All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
anchored to the main-checkout root. CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN — such writes land silently in the story
worktree's shadow `.factory/` subtree and are permanently destroyed at teardown (issue #523
gitignored-shadow mechanism; BC-6.26.001 Invariant 5). Canonical absolute artifact-write
targets, per issue No. 523, are FORBIDDEN for ledgers created inside the delivery sandbox.
```

**Captured stdout:**

```
########## M-P18-B ('No. ' false split -> Gate 4 blind) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

**Load-bearing control** — the identical sentence with `per issue No. 523, ` removed, so no false boundary is created:

```
########## CONTROL-B (no 'No. ' token -> Gate 4 must fire) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 836)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide FORBIDDEN-polarity (sentence-scoped; F-S2104-P17-001(a))]: a sentence in the Write Discipline section contains both 'absolute' and 'FORBIDDEN' — in the correct text absolute paths are MANDATED (MUST), not the FORBIDDEN subject; M-P15-A S3 'Canonical absolute artifact-write paths...are FORBIDDEN' triggers this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14-001 / F-S2104-P15-001)
```

The control proves the false boundary — not the wording, not the destination, not the polarity — is the element M-P18-B evades. The surviving text declares canonical absolute paths FORBIDDEN for in-sandbox ledgers, which is the M-P15-A S3 class that Gate 4 exists to catch, re-seeded through the normalization layer rather than through the predicate.

The split-inducing class is large and every member is ordinary prose: `No. `, `etc. `, `vs. `, `al. `, `Inc. `, `Fig. `, `§4.1. `, `Dr. `, and any decimal or ordinal (`v1. `, `rc. `).

**Zero-degrees-of-freedom fix predicate.** (a) Replace the unconditional `sed 's/\. /\n/g'` with a boundary rule that breaks only where a sentence can actually begin: split on `\.[[:space:]]+` **only when the next non-space character is one of `[A-Z*`\[]`**, retaining the existing `cf. `/`i.e. `/`e.g. ` protections for the case where an abbreviation is followed by a capitalised word. I verified this rule is both effective and regression-clean by literal shell:

```
$ # Gate 4 over M-P18-B, current splitter:
$ ex "$SC" | sed 's/\. /\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)'
(empty) BLIND
$ # Gate 4 over M-P18-B, splitter-B:
$ ex "$SC" | perl -pe 's/\.[ ]+(?=[A-Z*`\[])/.\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)'
Canonical absolute artifact-write targets, per issue No. 523, are FORBIDDEN for ledgers created inside the delivery sand
$ # splitter-B regression over PRISTINE — Gate 4 / Gate 5 / PW-B must all be empty:
G4:   (empty) OK
G5:   (empty) OK
PW-B: (empty) OK
```

(b) Additionally enumerate and mutant-prove the ordinal/abbreviation class explicitly — `No. `, `etc. `, `vs. `, `al. `, `Inc. `, `§4.1. ` — one recorded mutant each, so the class is documented rather than implied by the boundary rule. (c) Record M-P18-B verbatim with its RED stdout and CONTROL-B as the paired anti-mutant. (d) Add an isolating mutant for Gate 4 in its own right: the audit table's sole Gate-4 mutant is `M-P15-A S3`, but full M-P15-A fires Gate 1(a) first, so Gate 4 currently has no recorded vector that reaches it alone. CONTROL-B above **is** that vector — record it as Gate 4's isolating mutant. (e) Propagate the splitter change to the story v1.22 Gate cell's extraction-mechanisms lead-in and to the red-gate-log `Extraction and normalization mechanisms` table in the same burst, per the cell's own coupling note.

### F-S2104-P18-004 — HIGH — the render-fidelity gate names one surface form; the class of non-rendered constructs is open

**Stable anchor.** `bats` T-001, the assertion introduced by `# Gate: no HTML comment spans in #### Write Discipline (F-S2104-P17-001(b)).`, predicate `printf '%s\n' "$write_discipline_section" | grep -qE '<!--'`.

F-S2104-P17-001(b)'s fix predicate offered two options — assert absence of HTML comments, or strip non-rendered spans from every gate domain and prove the mandate survives stripping. The burst took the first, and implemented it as a single literal (`<!--`). That closes M-P17-H and nothing else: the gated property is "this one construct is absent", not "the gated text is the rendered text". Markdown has a second, widely-used comment idiom — a link reference definition with a `#` destination — which CommonMark parses as a definition and every renderer, GitHub included, emits nothing for.

**M-P18-D — exact substituted text**, replacing the normative prohibition paragraph:

```
[//]: # (All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths anchored to the main-checkout root. CWD-relative paths are FORBIDDEN.)
Writers anchor every `.factory/**` artifact to the delivery sandbox root so the ledger lands beside the code it documents.
```

**Captured stdout:**

```
$ sed -n '64,70p' _shared-context.md
#### Write Discipline — `.factory/**` artifact writes from story worktrees (BC-6.26.001 PC1, Invariants 1, 3, 4)

[//]: # (All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths anchored to the main-checkout root. CWD-relative paths are FORBIDDEN.)
Writers anchor every `.factory/**` artifact to the delivery sandbox root so the ledger lands beside the code it documents.

**Load-bearing cases (BC-6.26.001 Invariant 4):** The DELIVERY ledger (`*-DELIVERY.md`),
story-frontmatter files, and `pr-review.md` records are the primary artifacts at risk. All writes
to any `.factory/**` path are covered by this rule — not only DELIVERY ledgers.
$ bats -f "T-001" story-worktree-write-path-discipline.bats
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

The anchor line still matches `All.*\.factory.*artifact writes`, so `prohibition_block` extraction starts inside the definition and the mandate sentence satisfies Gate 1(a); Gate 2a finds `CWD-relative … FORBIDDEN` in the same hidden sentence; the comment gate sees no `<!--`. A reader of the rendered skill doc sees exactly one instruction, and it directs `.factory/**` writes to the story worktree.

**Zero-degrees-of-freedom fix predicate.** Take F-S2104-P17-001(b)'s second option, which is the one that closes the class rather than a member. (a) Define a `rendered_write_discipline` domain: strip `<!-- … -->` spans and drop any line matching the link-reference-definition form `^[[:space:]]{0,3}\[[^]]*\]:[[:space:]]` from `write_discipline_section` **before** any gate evaluates. (b) Re-scope the positive gates — Gate 1(a)/(b)/(c)/(d), Gate 2a, the anchor-uniqueness count and the empty-block guard — to that rendered domain, so no positive assertion can be satisfied by text a renderer discards; this is the property the gate set needs, and asserting absence of a construct is only ever a proxy for it. (c) Retain the `<!--` absence gate as a defence-in-depth negative with its recorded M-P17-H proof — the two are complementary. (d) Record M-P18-D verbatim with captured RED stdout and a GREEN restore, plus a second mutant using the bare `[label]: #` form without parenthesised title, and a GREEN control proving the unmodified section survives the stripping step unchanged (the mandate sentence is still found and Gate 1(a) still passes).

### F-S2104-P18-005 — HIGH — Gate 2b's nullification list is closed and the gate that was meant to make that irrelevant is paragraph-scoped

**Stable anchors.** `bats` T-001, the assertions introduced by `# Gate 2b (NULLIFICATION CLASS NEGATIVE, F-S2104-P16-001(c) strengthened F-S2104-P17-003):` (assigning `retirement_language`) and `# Gate 2b(c): FORBIDDEN sentence must NOT contain an adversative connective (F-S2104-P17-003(c)).` (assigning `forbidden_sentences_with_adversative`).

F-S2104-P17-003's fix predicate had three legs, and leg (c) existed precisely to make leg (b)'s list non-load-bearing: "assert the prohibition paragraph contains **no adversative connective** attaching to the FORBIDDEN sentence … so nullification cannot be expressed at all regardless of which verb is chosen." The burst implemented (a) on the **section-wide** domain and (c) on `joined_block_nosplit` — the **prohibition paragraph only**:

```
  retirement_language="$(printf '%s\n' "$write_discipline_prose_nosplit" | sed 's/\. /\n/g' | \
    grep -E 'formerly|retired|rescinded|superseded|relaxed|lifted|withdrawn|rescind|no[[:space:]]+longer|not[[:space:]]+longer|waived|exempt|obsolete|deprecated|does[[:space:]]+not[[:space:]]+apply|overridden|historical[[:space:]]+only' || true)"
```
```
  forbidden_sentences_with_adversative="$(printf '%s\n' "$joined_block_nosplit" | sed 's/\. /\n/g' | \
    grep -E 'FORBIDDEN|forbidden' | \
    grep -E 'but[[:space:]]|however|except[[:space:]]+that|though[[:space:]]' || true)"
```

The two domains differ, so a sibling paragraph is covered by the closed synonym list alone — and the structural gate that was supposed to backstop the list is not there. Three separate nullifications survive.

**M-P18-E / M-P18-F / M-P18-G — exact inserted text**, each placed immediately after the normative prohibition paragraph's terminating blank line, nothing else changed:

```
The prohibition above is authoritative for targets outside the story checkout; it does not bind
ledger writes made from the delivery sandbox.
```
```
**Scope note:** The rule above applies to the pre-#523 delivery model only and has been
supplanted for current deliveries.
```
```
**Scope note:** The rule stated above is descriptive, however current practice permits ledger
writes from the delivery sandbox.
```

**Captured stdout:**

```
### M-P18-E ('does not bind' nullification in sibling paragraph)
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
### M-P18-F ('supplanted' nullification synonym)
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
### M-P18-G ('however' adversative in sibling paragraph)
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

M-P18-G is the sharpest: it uses the *exact connective class* Gate 2b(c) enumerates (`however`), defeated purely by which paragraph it sits in — the same domain-scoping failure F-S2104-P17-003(a) closed for Gate 2b(a), left open in Gate 2b(c) in the same commit. M-P18-E and M-P18-F show the synonym list is independently open: "does not bind" is one word away from the listed "does not apply", and "supplanted" is a synonym of the listed "superseded".

**Zero-degrees-of-freedom fix predicate.** (a) Change Gate 2b(c)'s domain from `joined_block_nosplit` to `write_discipline_prose_nosplit`, matching Gate 2b(a), and widen its trigger from FORBIDDEN-sentences-only to **any sentence that references the prohibition** — concretely, any sentence matching `FORBIDDEN|forbidden|prohibition|prohibited|the rule|this rule|the constraint|above` must not match the adversative class. Verify by literal shell that the predicate is empty over HEAD's pristine section before landing. (b) Widen the adversative class to `but[[:space:]]|however|except[[:space:]]+that|though[[:space:]]|whereas|nevertheless|that[[:space:]]+said|in[[:space:]]+practice|notwithstanding`, one mutant per added member. (c) Add the members M-P18-E and M-P18-F expose to the 2b(a) nullification class — `supplanted|supersede|does[[:space:]]+not[[:space:]]+bind|does[[:space:]]+not[[:space:]]+govern|no[[:space:]]+longer[[:space:]]+binds|descriptive[[:space:]]+only|advisory[[:space:]]+only|pre-#?[0-9]+` — each mutant-proven. (d) The structural backstop that makes (b) and (c) non-load-bearing is the F-S2104-P18-001(a) write-directive gate: `however current practice permits ledger writes from the delivery sandbox` carries `writes` but no `MUST`/`SHOULD`, so extend that gate's directive trigger to include `permits|is acceptable|is the required form|is preferred|may` alongside `MUST|SHOULD`, and verify the widened form is still empty on pristine text. (e) Record M-P18-E, M-P18-F and M-P18-G verbatim with captured RED stdout and GREEN restores.

### F-S2104-P18-006 — MEDIUM — the record attests a Gate 5 fire that its own captured stdout contradicts

**Stable anchors.** `.factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md`, §`Pass-17 assertion-site attestation (c89bef22)` → `#### Battery table — vectors at c89bef22`, row `M-P17-A`; and §`Gate-indexed audit table (T-001 / AC-001 gates at c89bef22 — 17 gates; Gates 6(b)/7(b) RETIRED)`, row `Gate 5`.

The battery row's Gate(s)-triggered cell reads, verbatim: `Gate PW-B: "story worktree CWD" without prohibition token in second paragraph (now in write_discipline_prose_nosplit domain); Gate 5: "MUST anchor...story worktree CWD"`. The Gate 5 audit row reads, verbatim: `M-P17-A S1 (`MUST anchor...story worktree CWD` — note: `story worktree CWD` matches PW-B; M-P17-A also fires Gate 5 via any `MUST`+`relative path` form in the same sentence)`.

Gate 5's alternation is `CWD-relative|worktree-relative|relative[[:space:]]+path`. `story worktree CWD` is not a member and M-P17-A's sentence contains no member. Literal shell against the verbatim M-P17-A sentence:

```
$ MS="**Story-worktree exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every \`.factory/**\` artifact write to the story worktree CWD; the canonical-absolute form applies only to spec reads from the main checkout"
$ printf '%s\n' "$MS" | grep -E 'MUST' | grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' && echo "GATE5 FIRES" || echo "GATE5 DOES NOT FIRE"
GATE5 DOES NOT FIRE
```

The same document refutes itself: its §`Verbatim captured stdout — new vectors (T-001 only)` block for `########## M-P17-A ##########` shows one DOC-PARITY FAIL, `[write-discipline section-wide sentence polarity (Gate PW-B, …)]`, and no Gate 5 message. The audit row's hedge — "via any `MUST`+`relative path` form in the same sentence" — is a conditional attached to a claim of fact, and the condition is false for this vector.

The consequence is the same one F-S2104-P17-006(a) identified: an audit row that overstates a gate's mutant coverage makes that gate look proven when it is not. Gate 5's only *independently* isolating vector in the record is `M-P15-A S1`, which also fires Gate 1(a) and Gate 1(c) first.

**Zero-degrees-of-freedom fix predicate.** (a) Correct the battery table's M-P17-A row to `Gate PW-B only: "story worktree CWD" without prohibition token in the second paragraph (now in write_discipline_prose_nosplit domain)`, matching the recorded stdout. (b) Correct the Gate 5 audit row's mutant coverage to drop M-P17-A entirely and cite only vectors whose text contains a Gate-5 alternation member — `M-P15-A S1 (MUST use CWD-relative)`, `M-P14-A`, `M-P14R-A (MUST use relative paths)`, the `worktree-relative` synonym, `M-P16-C2 (via abbreviation splitter)` — and add an isolating Gate 5 mutant (a sibling-paragraph sentence `Writers MUST use relative paths for ledger writes.` with the mandate paragraph intact), with captured stdout. (c) Preserve the v1.15 entries unmodified and record the corrections as a new changelog row carrying an explicit error-acknowledgment clause naming the two sites, per the discipline established at v1.14 and v1.15. (d) Re-run the NAME-SET EQUALITY check after the edit and record its stdout, since the Gate 5 label is inside the compared partition.

### F-S2104-P18-007 — MEDIUM — the input-hash transition is recorded as a bracketed placeholder

**Stable anchors.** red-gate-log v1.15 frontmatter, `last_amended` (the `D-918` clause) and `modified[]` (the `2026-07-26 D-918: …` entry).

`modified[]` reads, verbatim in relevant part: `input-hash e6c640a→[updated by compute-input-hash]`. `last_amended` reads: `input-hash e6c640a→[see frontmatter]`. Every one of the nine sibling rows records the literal transition: `input-hash 3d12427→1baca60 (story v1.21 drift)`, `input-hash c74e0f8→3d12427 (story v1.20 drift)`, `input-hash 89efd7e→c74e0f8`, and so on. The value is not unknown — it is in the same frontmatter, two fields above:

```
$ grep -n 'input-hash:' .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
12:input-hash: "4b26b3b"
$ grep -c 'updated by compute-input-hash\|\[see frontmatter\]' .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
2
```

This is not the known-context dual-binary hash divergence — the stored hash is present and concrete. It is a record-form defect: a changelog row that defers its own content to a tool invocation, in a field whose purpose is to let a reader reconstruct the transition without re-running anything. It also breaks the append-only audit chain's one useful mechanical property, that each row's old value matches the prior row's new value.

**Zero-degrees-of-freedom fix predicate.** Replace both bracketed forms with the literal transition `input-hash e6c640a→4b26b3b`, verified by a `grep -n 'input-hash:'` whose stdout is recorded alongside the correction; preserve the rest of the D-918 row unmodified and note the substitution in the new changelog row. Sibling-sweep the same placeholder form across the S-21.04 perimeter (story, epic, STORY-INDEX, BC-6.26.001) with a literal `grep -rn 'updated by compute-input-hash\|\[see frontmatter\]'` and record the result.

---

## Observations (NOT findings)

**Fifteen recorded vectors independently re-proven RED at HEAD from verbatim Part A / battery text.** M-P16-A → Gate 1(a)/(b); M-P16-C2 → Gate 1(c) via the abbreviation-protected splitter; M-P16-D → tightened Gate 3 + canonical-target; M-P16-B in-section decoy → anchor-uniqueness (count=2); M-P15-A → Gate 1(a); M-P15-B → canonical-target; M-P14R-A and the `worktree-relative` synonym → Gate 1(a); M-P17-A verbatim (sibling paragraph) → Gate PW-B; M-P17-C → Gate 1(d); M-P17-D and M-P17-F → Gate 2b(a); M-P17-G → canonical-target; M-P17-H → HTML-comment absence; and the single-quoted `file_path='.factory/…'` plus bare `file_path=.factory/…` `**Correct:**`-bullet variants → canonical-target. The canonical-target gate is the strongest single piece of work in this cascade: it replaced two surface-pinned negatives with one class predicate and I could not find a rendering of a non-canonical target that escapes it.

**Both POLICY 15 v1.4.14 mechanisms were implemented and both actually work.** I reproduced the NAME-SET EQUALITY check at HEAD, including its `grep -oP` extraction, and it is faithful — the story v1.22 Gate cell yields exactly the seventeen labels the audit table enumerates, and the diff is empty:

```
$ grep -oP '(?<=\()[0-9]+\) [A-Za-z][A-Za-z0-9(). -]+(?= (?:POSITIVE|NEGATIVE|anchor|empty|HTML|Gate|canonical))' .factory/stories/S-21.04-story-worktree-write-path-discipline.md | sed 's/^[0-9]*) //' | sort > /tmp/sg.txt
$ diff /tmp/sg.txt /tmp/lg.txt && echo "DIFF EMPTY -> record claim REPRODUCED (17 labels)"
DIFF EMPTY -> record claim REPRODUCED (      17 labels)
```

The partition-definition paragraph that names the three out-of-partition assertions (EC-006-presence, no-revparse-outside-WARNING, mandate-sentence-present) is exactly the disclosure F-S2104-P17-006(a) asked for, and the obligation-indexed table now has AC-001(b) and AC-001(c) rows with explicit presence-only/OPEN-GAP markers. Both F-S2104-P17-006 legs are closed.

**Story v1.22's Gate cell is bidirectionally accurate.** I checked it in both directions against the bats file: every regex quoted in the cell has a matching occurrence in T-001 (including the corrected `relative[[:space:]]+path` for Gate 5, the `[Ww]orktree-local` bracket class, and the word-boundary `(^|[^[:alnum:]])[Ii]n-worktree`), and every gate in T-001 appears as a numbered cell item or is covered by the trailing `clause-content gates … and primary-path §G.1 mandate gates … also in T-001` clause. Item (8) Gate 2b(a) is correctly grouped under `write_discipline_prose_nosplit` and item (9) Gate 2b(c) correctly under `joined_block_nosplit` — the F-S2104-P17-007 misdescription is fully closed, and the cell is now *more* accurate than the audit table it is compared against.

**Tilde fences fail safe.** `~~~`-delimited content is not matched by the fence-exclusion `awk` rule and therefore remains inside the gated prose domain. Recorded so the F-S2104-P18-002 fix does not "helpfully" generalise the fence matcher to `~~~` and thereby widen the hole it is closing.

**Gate 4 has no isolating mutant in the record.** Its sole cited vector is `M-P15-A S3`, but the full M-P15-A fires Gate 1(a) first, so nothing in the record demonstrates Gate 4 reached. CONTROL-B in F-S2104-P18-003 is such a vector and should be recorded as Gate 4's isolating mutant. This is not a gate defect — Gate 4 is genuinely load-bearing, as CONTROL-B proves — only a coverage-record gap.

**`_extract_write_discipline_section`'s docblock is now accurate.** The pass-17 burst added the `NOTE:` explaining that a second `#### Write Discipline` heading re-triggers rule 1 rather than the exit rule, so both anchors land in one extraction and anchor-uniqueness catches it. That was the latent doc/behaviour divergence pass-17 recorded as an observation; it is closed.

**Cross-document parity holds across every POLICY 14/17 leg.** STORY-INDEX `version: "4.265"`; the S-21.04 catalog row cites `story v1.22`, `[BC-6.26.001 v1.11]`, `input-hash 1165b1f`, and Refs terminating in the pass-17 finding range. The `> **E-21 delivery:**` blockquote's `S-21.04=1165b1f` matches live story frontmatter and all six input-hashes remain distinct. BC-6.26.001 is v1.11 (matching the story's live-body BC table pin and the red-gate-log `traces_to`), BC-6.27.001 v1.4, ADR-031 v1.13, epic v1.8. red-gate-log `traces_to` correctly adds `story v1.22`. BC-6.27.001 carries no obligation inside S-21.04's delivery perimeter — its PCs govern pr-manager and are S-21.05's contract, and the story's `behavioral_contracts` frontmatter is `[BC-6.26.001]`.

**Zero live POLICY 19 tokens in the S-21.04 perimeter.** `grep -nE 'ADR-031 v[0-9]'` returns 3 hits in the story, 2 in BC-6.26.001, 5 in the epic and 0 in the red-gate-log; I inspected every one and all are in `last_amended` / `modified[]` / changelog-table rows — historical-by-construction sites, exempt per POLICY 5.

**CHANGELOG is accurate.** The count-free lead-in survives (`the two BC-6.26.001 protocol requirements plus the propagation and awareness legs`), items (1)–(5) match the delivered surfaces in the 19-file diff, the five sibling teardown sites named all appear in the diff, and item (1)'s prose (`CWD-relative paths are forbidden`; `$CANONICAL_FACTORY_ROOT` described as the repo root, not the mount) matches the shipped `_shared-context.md`.

**[process-gap] — for the first time, the fix's own mechanisms are the attack surface.** Passes 12–17 each found the predicate defeated at a finer granularity or a wider extent of the same text, and each fix was the right one. Pass-18 found two defeats *inside* the pass-17 fix machinery: the whole-section domain is built by an unconditional **subtraction** (fenced code), and the rewrap-invariance is delivered by a **whitelist-driven splitter**. Both are helper mechanisms, and the red-gate-log explicitly classifies them as "not assertion gates" — which is exactly why nobody adversarially probed them. Candidate codification: **normalization-adversariality** — every extraction, exclusion or normalization mechanism in a gate's data path MUST itself carry mutants, because a mechanism that can silently shrink a gate's domain is indistinguishable in effect from deleting the gate. Concretely: (i) any *exclusion* step (fence-stripping, comment-stripping, line filtering) MUST be accompanied by an assertion that the excluded region is well-formed (balanced delimiters) and by a mutant placing harmful text inside the excluded region; and (ii) any *tokenization* step (sentence splitting, joining) MUST be accompanied by a mutant that manufactures a false boundary from ordinary prose. The "Extraction and normalization mechanisms (not assertion gates)" table is the correct place to record this, and its current framing — that these are not gates and therefore need no mutant coverage — is the inversion that let both defects through.

**[process-gap] — "widen the alternation" is now a confirmed non-fix, three passes running.** F-S2104-P16-001 widened PW-B's target list; F-S2104-P17-002(b) widened it again by six members plus two character-class repairs; F-S2104-P17-003(b) widened Gate 2b's nullification list to sixteen members. Each widening was mutant-proven and each fell to the first paraphrase tried in the next pass (M-P17-C, then M-P18-A, then M-P18-E/F). POLICY 13 already says the trigger must be an open class; the practical gap is that no review step asks whether a *specific* alternation sits on the open or the closed side. Candidate codification: any burst that **adds a member to an existing alternation** must, in the same burst, either (a) demonstrate the alternation is in the failing position of an implication whose trigger is open, or (b) add a complementary open-trigger gate covering the same obligation — and must state which of (a) or (b) it did in the audit row. A widening that does neither is a paper-fix under TD-VSDD-059 by construction, regardless of how many mutants accompany it, because the mutants are drawn from the same vocabulary as the predicate.

**[process-gap] — the paired-gate legs in a multi-leg fix predicate were implemented on different domains.** F-S2104-P17-003 legs (a) and (c) were designed as a pair: (c) exists so that (b)'s list is not load-bearing. (a) landed section-wide, (c) landed paragraph-scoped, and F-S2104-P18-005 exploits precisely the difference. The same shape appears in F-S2104-P18-002(d): Gates 4/5 share PW-B's domain variable but may need a different exclusion policy, and nothing records that. Candidate codification: when a fix predicate specifies a **backstop** gate whose purpose is to make another gate's enumeration non-load-bearing, the two MUST share a domain, and the audit table MUST show their domains in adjacent rows so a divergence is visible on inspection. Where domains legitimately differ, the divergence MUST be stated in the code comment, the story Gate cell and the audit row in the same burst.

---

## Per-Pass-17 Verification

| Finding | Status | Notes |
|---------|--------|-------|
| F-S2104-P17-001 | PARTIAL (both legs land; both re-seed one hop) | (a) The whole-section domain is real and reaches every paragraph: `write_discipline_prose_nosplit` is built from `_extract_write_discipline_section`, Gates PW-B/2b(a)/4/5 all read it, and I re-proved verbatim M-P17-A RED via Gate PW-B plus the recorded `**Load-bearing cases**`-paragraph residual. Paragraph-scoping is genuinely closed. But the domain is constructed by *subtracting* fenced code, and that subtraction is an unconditional fail-open hole — M-P18-C hides a listed prohibited-target inside the existing bash fence at 9/9, and M-P18-C(b) deletes the entire section-wide domain with one unbalanced fence at 9/9 → F-S2104-P18-002. (b) The HTML-comment gate exists, is load-bearing, and M-P17-H is RED. But it asserts absence of one surface form rather than equating the gated domain with the rendered domain — M-P18-D hides the whole mandate in a `[//]: # (…)` link-reference definition at 9/9 → F-S2104-P18-004 |
| F-S2104-P17-002 | PARTIAL (7 of 8 sub-legs closed; the trigger side is not) | (a) CONFIRMED-CLOSED — the directive-token whitelist is gone and the prohibition-token set is in the failing position; the `are the required form` phrasing that defeated pass-16 now fires. (c) CONFIRMED-CLOSED — Gate 1(d) exists and verbatim M-P17-C is RED on `when the target`. (d) CONFIRMED-CLOSED — Gate 2a is narrowed to `CWD-relative\|worktree-relative`, so M-P17-C's traversal FORBIDDEN sentence no longer satisfies it; M-P16-C2 also still RED. (b) NOT CLOSED as a class — the prohibited-target alternation was widened by six members and two character-class repairs (`[Ww]orktree-local`, word-boundary `[Ii]n-worktree`, both verified load-bearing), but it remains a closed ten-item surface list where POLICY 13 requires an open trigger. M-P18-A's `` the worktree's `.factory/` subtree `` names the issue #523 destination and matches no member, at 9/9 → F-S2104-P18-001 |
| F-S2104-P17-003 | PARTIAL | (a) CONFIRMED-CLOSED — Gate 2b(a)'s domain is now `write_discipline_prose_nosplit`, joined and sentence-split, so it is rewrap-invariant; I re-proved M-P17-F RED (the `no longer` split across the soft line break) and M-P17-D RED (`rescinded and superseded`). The POLICY 13 NORMALIZED-DOMAIN violation is fixed. (b) The nullification class was widened to sixteen members and remains open to paraphrase: M-P18-E (`does not bind`) and M-P18-F (`supplanted`) both survive at 9/9. (c) Gate 2b(c) exists and fires on M-P17-F, but it was implemented on `joined_block_nosplit` while (a) is section-wide — so the backstop that was designed to make (b) non-load-bearing does not cover sibling paragraphs, and M-P18-G nullifies the prohibition using Gate 2b(c)'s own enumerated connective `however` at 9/9 → F-S2104-P18-005 |
| F-S2104-P17-004 | CONFIRMED-CLOSED | The canonical-target gate is present, class-based and load-bearing: `no **Correct:** bullet with file_path= may fail file_path=["']?(\$CANONICAL_FACTORY_ROOT\|/)`. I re-proved verbatim M-P17-G RED (`./.factory/`), M-P16-D RED (bare `.factory/`), M-P15-B RED (`../../.factory/`), and both the single-quoted `file_path='.factory/…'` and bare `file_path=.factory/…` variants RED. Gates 6(b)/7(b) are correctly retired with their reasons recorded, and Gates 3/6(a)/7(a) remain as the positive existence assertions. I could not construct a non-canonical `**Correct:**` target that escapes the predicate. No surviving mutant on this axis |
| F-S2104-P17-005 | CONFIRMED-CLOSED (all five sub-legs) | (a) the abbreviation splitter is now stated as `cf. `/`i.e. `/`e.g. ` exactly, with the `§[0-9]+\.` claim explicitly retracted in place (`never implemented — [Correction at v1.15 (D-918) …]`) rather than silently dropped. (b) the anchor-uniqueness domain reads `#### Write Discipline` section at both attestation sites. (c) the TIER 1 M-P16-B cell now attributes Gate 1(a) with the decoy excluded by bounding at count=1, matching the preamble and my own execution. (d) the absent-block guard domain reads `$prohibition_block`. (e) the Gate 5 row's alternation is corrected to `CWD-relative\|worktree-relative\|relative[[:space:]]+path` with no prohibition-token clause. The corrections are recorded as a new v1.15 changelog row with error acknowledgment and the v1.14 entries preserved, exactly as the fix predicate specified. A *new* misattestation appeared in the same row → F-S2104-P18-006 |
| F-S2104-P17-006 | CONFIRMED-CLOSED (both legs) | (a) The gate-indexed table is rebuilt to 17 rows with Gate PW-B and Gate 2b(a)/(c) present, `G1(b)` renamed to `G1(c)`, the bounding extractor and the abbreviation splitter moved out into a separate `Extraction and normalization mechanisms (not assertion gates)` table, and Gates 6(b)/7(b) recorded as RETIRED with reasons. The count is stated after enumeration, and the NAME-SET EQUALITY check — the mechanism the pass-17 process-gap note asked for — is implemented, cross-checked against the story cell by literal `diff`, and I reproduced it at HEAD with the diff empty. Count-only comparison is explicitly forbidden in the partition paragraph. (b) The obligation-indexed table has rows for AC-001(a)(i) mandate, AC-001(a)(i) bullet, AC-001(a)(ii) bullet, AC-001(b) and AC-001(c), and the two presence-only clauses carry explicit OPEN GAP markers with their gates named. This is the inversion POLICY 15 codified, achieved |
| F-S2104-P17-007 | CONFIRMED-CLOSED | Story `version: "1.22"`; item (8) Gate 2b(a) is moved into the `write_discipline_prose_nosplit` group and item (9) Gate 2b(c) is correctly grouped under `joined_block_nosplit`, so the domain misdescription is gone and the grouping is correct by construction after the F-S2104-P17-003(a) domain change. Item (13) Gate 5's alternation reads `relative[[:space:]]+path` with an explicit `not paths?` note. The 17-gate count is updated, the same-burst coupling note is retained and strengthened, and `last_amended` + `modified[v1.22]` both record the change. I verified the cell bidirectionally against T-001 and found no divergence |

Tally: **3 CONFIRMED-CLOSED / 4 PARTIAL / 0 REGRESSED** against the pass-17 finding set. No regression: every pass-17 leg that landed is load-bearing, all fifteen recorded vectors are RED, and the two record findings closed with proper error acknowledgment. The four PARTIALs share one shape — the fix closed the vector and the axis, but not the *class*, because in each case the class boundary was a list (PW-B's targets, 2b's nullifications), a surface form (`<!--`), or a helper mechanism nobody treated as adversarial (the fence exclusion, the splitter). Every fix predicate above is mechanically checkable, three of the five gate predicates I propose are already verified empty-on-pristine and firing-on-mutant by literal shell in this report, and none requires a new spec decision.
