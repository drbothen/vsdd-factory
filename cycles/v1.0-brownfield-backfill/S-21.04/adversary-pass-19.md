---
pass: 19
verdict: NOT-CLEAN
reviewed_head: a4ec37d3
novelty: 0.58
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-18.md"
---

## Summary

Pass-19 fresh-context adversarial review of S-21.04 at `reviewed_head a4ec37d3` (worktree `.worktrees/S-21.04`, base develop `948f0fb1`, clean tree). **12 findings: B2 / H3 / M7.** Novelty 0.58 vs pass-18 Part A. Trajectory 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12. Streak: **0/3** (BC-5.39.001 reset).

Baseline by literal execution at HEAD: `bats story-worktree-write-path-discipline.bats` → `1..9`, 9/9 ok; `bats worktree-identity-preflight.bats` → `1..14`, 14/14 ok.

**The pass-18 wave landed completely and I found no regression.** All seven pass-18 gate-level vectors are RED at HEAD from their verbatim Part A text — M-P18-A (write-directive gate), M-P18-B (`No. 523`, Gate 4 via boundary-rule splitter), M-P18-C (in-fence harmful line, PW-B, correct in-fence placement at the `CANONICAL_FACTORY_ROOT="$(git -C …)"` line), M-P18-C(b) (balanced-fence, odd count 3), M-P18-D (`[//]: # (…)`, rendered-domain anchor count 0), M-P18-E/F (2b(a) widened class), M-P18-G (2b(c) section-wide). I additionally re-proved M-P17-A, M-P17-C, M-P17-G, M-P17-H, M-P16-A, M-P16-B in-section decoy and M-P15-A S1 RED, and confirmed the write-directive gate's canonical-absolute escape is **load-bearing, not vacuous** (replacing `MUST anchor …` with `MUST use canonical absolute paths` returns GREEN). The two POLICY 15 mechanisms both work: I reproduced NAME-SET EQUALITY at HEAD with `diff` empty over 19 labels, and the obligation-indexed table carries the AC-001(b)/(c) OPEN GAP markers.

**The gate set is nevertheless defeated seven independent ways at 9/9, and for the second consecutive pass the primary defeats come from the fix machinery itself rather than from axes the fix did not reach.** Two are BLOCKERs.

The first is the escape clause. Pass-18 correctly inverted PW-B and built the write-directive gate as an open trigger with a closed escape — but both gates implement the escape as a **whole-sentence** `grep -Ev`, so a single appended clause carrying any prohibition token exempts the entire sentence, harmful mandate included. `M-P19-A` is the **verbatim M-P17-A sentence** with `; duplicating the ledger onto the main checkout is forbidden.` appended: 9/9 GREEN. `CONTROL-A`, identical except `forbidden` → `discouraged`, is RED via PW-B. One ordinary English word, semantically consistent with the section, restores the exact vector the last three passes were built to catch. The canonical-absolute branch of the same escape behaves identically (`M-P19-B`).

The second is the domain boundary. Every section-wide gate reads `_extract_write_discipline_section`, which starts at `^#### Write Discipline`. The read-discipline prose **above** that heading — still inside `### Spec-Path Discipline`, still rendered to the reader three lines from the mandate — is ungated by PW-B, 2b(a), 2b(c), Gate 4, Gate 5 and the write-directive gate. `M-P19-H` places verbatim M-P17-A **and** M-P17-C S2 text there at 9/9. This is F-S2104-P16-003(b)'s own bounding: making that region out-of-domain neutralised the harmless M-P16-B decoy and simultaneously created an ungated region for harmful mandates. Nobody probed it because the bounding was introduced as a false-positive fix.

The remaining five are one-hop re-seedings on mechanisms pass-18 created: the boundary-rule splitter closed the false-boundary direction and left the **missed-boundary** direction fail-open for the two exclusion-based gates (`M-P19-D`, with `CONTROL-D` proving a single lowercase initial is the whole difference); the link-reference-definition strip predicate requires whitespace after the colon, so the equally-valid `[//]:# (…)` form — which I confirmed renders as nothing under both `marked` and `pandoc -f commonmark` — hides the mandate again (`M-P19-F`, `CONTROL-F` differing by one space character); the write-directive gate's action class is a closed three-verb list (`M-P19-C`, `saved`); the canonical-target gate's domain is gated on the literal `file_path=` (`M-P19-E`); and the nullification class is still closed with a backstop that cannot reach directive-free prose (`M-P19-G`, `is not applicable`).

On the record side both pass-18 record findings closed properly with error acknowledgment, and I could not fault the battery's 35 checks or the 19-gate audit table's predicates. Five defects remain: the shipped test file still declares **"Seventeen"** gates at two sites; the STORY-INDEX carries the **red-gate-log's own** input-hash in the story's catalog row and blockquote; the NAME-SET EQUALITY partition paragraph says "TWO" and enumerates three; the balanced-fence gate's rationale describes a fence-aware domain that no longer exists at three sites; and the escape-clause GREEN control that F-S2104-P18-001's fix predicate leg (c) explicitly required is absent from the battery.

Every fix predicate below is verified by literal shell — **empty on pristine and firing on the mutant** — before being proposed.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P19-001 | BLOCKER | `story-worktree-write-path-discipline.bats` T-001 Gate PW-B (`polarity_violations`) + write-directive gate (`write_directive_violations`) | Both gates implement their escape as a **whole-sentence** `grep -Ev`, so any prohibition token or `MUST use canonical absolute` occurring anywhere in a sentence exempts every clause of it. M-P19-A = **verbatim M-P17-A** + `; duplicating the ledger onto the main checkout is forbidden.` → 9/9. CONTROL-A (`forbidden`→`discouraged`) → RED via PW-B. M-P19-B abuses the canonical-absolute branch with an unlisted destination → 9/9. Ninth-generation recurrence of the primary BC postcondition remaining invertible | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13 (FAIL-CLOSED-IMPLICATION-DIRECTION), 15; TD-VSDD-059 |
| F-S2104-P19-002 | BLOCKER | `bats` T-001 `_extract_write_discipline_section` | All six section-wide gates read a domain bounded to `^#### Write Discipline`. The rendered prose **above** that heading inside `### Spec-Path Discipline` is ungated. M-P19-H puts verbatim M-P17-A + M-P17-C S2 text three lines above the heading at 9/9. The F-S2104-P16-003(b) bounding that neutralised the M-P16-B decoy created the hole | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13 (DOMAIN-COMPLETENESS, RENDER-FIDELITY), 15 |
| F-S2104-P19-003 | HIGH | `bats` T-001 write-directive gate action class | The open trigger is `(write-directive) AND (anchor\|write\|writes)` — a closed three-member verb list. M-P19-C (`Ledger artifacts SHOULD be saved to the story worktree's own .factory/ subtree`) carries a directive and an unlisted verb → 9/9. Directive-free imperatives (`Anchor every … to the delivery sandbox root.`) escape the other conjunct. POLICY 13 declares the open side must be a class, not a list | BC-6.26.001 PC1; POLICY 11, 13, 15; TD-VSDD-059 |
| F-S2104-P19-004 | HIGH | `bats` T-001 boundary-rule splitter (`perl -pe 's/\.[[:space:]]+(?=[A-Z*\`\[])/.\n/g'`) | Pass-18 closed the tokenizer's false-boundary direction and left the **missed-boundary** direction open. A sentence beginning with a lowercase word, digit or quote is not split, merging it into the preceding sentence — fail-closed for Gates 4/5 but fail-**open** for the two exclusion-based gates, whose `grep -Ev` then covers the merged harmful clause. M-P19-D (`git-resolved ledger paths SHOULD be anchored to the story worktree CWD.`) → 9/9; CONTROL-D (capital `G`) → RED via PW-B | BC-6.26.001 PC1; POLICY 13 (NORMALIZATION-ADVERSARIALITY, NORMALIZED-DOMAIN), 15 |
| F-S2104-P19-005 | HIGH | `bats` T-001 `rendered_write_discipline` link-ref-def strip | The strip predicate `^[[:space:]]{0,3}\[[^]]*\]:[[:space:]]` requires whitespace after the colon. CommonMark makes that whitespace optional, so `[//]:# (…)` is a valid link reference definition rendered as **nothing** — confirmed by `marked` and `pandoc -f commonmark`. M-P19-F hides the whole mandate in that form at 9/9; CONTROL-F, differing by one space, is RED. One-hop re-seed of F-S2104-P18-004 through the strip mechanism's own regex | BC-6.26.001 PC1, Invariant 1; POLICY 13 (RENDER-FIDELITY, NORMALIZATION-ADVERSARIALITY), 15 |
| F-S2104-P19-006 | MEDIUM | `bats` T-001 canonical-target gate (`noncanonical_correct_bullets`) | The gate's domain is `**Correct:**` bullets filtered by literal `file_path=`. A `**Correct:**` bullet naming a non-canonical target without that token is outside the domain entirely. M-P19-E (`- **Correct:** \`Write\` the DELIVERY ledger to \`.factory/stories/S-NNN-DELIVERY.md\` resolved from the worktree root`) → 9/9 | BC-6.26.001 PC1; AC-001(a)(i); POLICY 13, 15 |
| F-S2104-P19-007 | MEDIUM | `bats` T-001 Gate 2b(a) + 2b(c) + write-directive backstop | The 25-member nullification class is still closed and the declared backstop cannot reach directive-free prose. M-P19-G (`The prohibition stated above is not applicable to ledger writes made from the delivery sandbox.`) matches no 2b(a) member, carries no adversative for 2b(c), and carries no write-directive token → 9/9. One-hop re-seed of F-S2104-P18-005 | BC-6.26.001 PC1; POLICY 11, 13 (ALTERNATION-WIDENING-DIRECTION-STATEMENT), 15 |
| F-S2104-P19-008 | MEDIUM | `bats` T-001 lead-in comment, two sites | The shipped test file declares `Seventeen independently mutant-proven gates` and `All seventeen gates survive independently` while the gate set, the story v1.23 cell, the audit table and the NAME-SET EQUALITY check are all 19. The same-burst coupling note that produced the 19-gate cell did not sweep the file it describes | POLICY 14 (quintuple parity, predicate↔Gate-cell coupling), 4, 5 (sibling-sweep); TD-VSDD-060 |
| F-S2104-P19-009 | MEDIUM | `STORY-INDEX.md` v4.266 S-21.04 catalog row + `> **E-21 delivery:**` blockquote | Both carry `input-hash f86871a`, which is the **red-gate-log's own** frontmatter input-hash, while the live story frontmatter is `input-hash: "1165b1f"`. Four of six E-21 stories satisfy frontmatter = catalog = blockquote; S-21.04 now breaks it. Story v1.23 never claims an input-hash change | POLICY 14, 17; POLICY 5 (sibling-sweep) |
| F-S2104-P19-010 | MEDIUM | red-gate-log v1.16 §`NAME-SET EQUALITY … at a4ec37d3` partition paragraph | `TWO additional T-001 assertions exist OUTSIDE that partition` then enumerates **three**. The pass-17 paragraph correctly said `THREE`; the v1.16 rewrite regressed it while preserving the same enumeration | POLICY 15 (count-after-enumeration), 4; TD-VSDD-059 |
| F-S2104-P19-011 | MEDIUM | `bats` balanced-fence comment; story v1.23 Gate cell item (2); red-gate-log v1.16 audit table `balanced-fence` row | All three justify the gate by domain truncation that cannot occur at HEAD: F-S2104-P18-002(b) removed the fence-stripping `awk` and **no fence-aware domain remains**. The bats comment describes `in_fence` state in the present tense against code that no longer exists; the audit row asserts `odd count means section domain is truncated`; the tilde-fence fail-safe note is likewise vacuous | POLICY 4, 14, 15; TD-VSDD-059 (doc-comment claiming a mechanism with no mechanism) |
| F-S2104-P19-012 | MEDIUM | red-gate-log v1.16 §`Battery table — vectors at a4ec37d3` | F-S2104-P18-001's fix predicate leg (c) required a GREEN control changing only `MUST anchor` to `MUST use canonical absolute`, proving the escape is load-bearing rather than vacuous. No such row exists (the only `*-control` row is M-P17-C-control) and no captured stdout records it. I verified it GREEN myself; the omission is precisely the mechanism F-S2104-P19-001 exploits | POLICY 15 (mutant-proving, verbatim stdout); TD-VSDD-059 |

---

### F-S2104-P19-001 — BLOCKER — the escape clause is sentence-scoped, so one appended prohibition word restores M-P17-A

**Stable anchors.** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`, T-001 (`@test "T-001 S-21.04 AC-003: stray-file-blocks …"`): the block introduced by `# Gate PW-B (SECTION-WIDE SENTENCE POLARITY, F-S2104-P16-001(b) strengthened F-S2104-P17-002):` assigning `polarity_violations` (bats:813-821), and the block introduced by `# Gate write-directive (POSITIVE, open-trigger; F-S2104-P18-001 / F-S2104-P18-005(d)):` assigning `write_directive_violations` (bats:966-976). Target: `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`, `#### Write Discipline`. Contract: BC-6.26.001 PC1 and Invariant 1.

Both predicates end in whole-sentence exclusions:

```
    grep -Ev 'FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid' || true)"
```
```
    grep -Ev 'FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid' | \
    grep -Ev 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute' || true)"
```

Pass-18 got the *direction* right: PW-B's prohibition set sits in the failing position, and the write-directive gate's trigger is genuinely open. But the unit of evaluation is the sentence, and a sentence can contain both a harmful directive and a compliant clause. The escape therefore does not certify the sentence — it only certifies that *somewhere* in the sentence a prohibition word appears. That is an unconditional fail-open available to any author who can write a semicolon.

**M-P19-A — exact inserted text**, placed immediately after the normative prohibition paragraph's terminating blank line (between it and `**Load-bearing cases (BC-6.26.001 Invariant 4):**`), nothing else changed. The first two lines are **verbatim M-P17-A**:

```

**Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every
`.factory/**` artifact write to the story worktree CWD; duplicating the ledger onto the main
checkout is forbidden.
```

**Captured stdout:**

```
########## M-P19-A (prohibition-token co-occurrence escape; verbatim M-P17-A + trailing 'forbidden' clause) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
$ bats story-worktree-write-path-discipline.bats | grep -c '^ok '
9
```

**Load-bearing control — CONTROL-A**, identical except `forbidden` → `discouraged`:

```
########## CONTROL-A (identical minus the prohibition token: 'forbidden' -> 'discouraged') ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 820)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide sentence polarity (Gate PW-B, F-S2104-P16-001(b)/F-S2104-P17-002)]: a sentence in the Write Discipline section contains a prohibited-target form (…) without a prohibition token (…)
# **Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every `.factory/**` artifact write to the story worktree CWD; duplicating the ledger onto the main checkout is discouraged.
```

The control isolates the mechanism exactly: not the wording, not the destination, not the paragraph — the single word `forbidden`. `story worktree CWD` still matches PW-B's `story[[:space:]]+worktree[[:space:]]+CWD` member; `MUST` + `anchor` still matches the write-directive trigger; both gates see the sentence and both discard it.

**Leg (b) — the canonical-absolute branch. M-P19-B — exact inserted text**, same placement:

```

**Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST use canonical absolute
paths when reading specs, and MUST anchor every `.factory/**` artifact write to the worktree's
`.factory/` subtree.
```

**Captured stdout:**

```
########## M-P19-B (canonical-absolute escape + unnamed destination) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
$ bats story-worktree-write-path-discipline.bats | grep -c '^ok '
9
```

PW-B does not fire (the destination is M-P18-A's unlisted `worktree's .factory/ subtree`); the write-directive gate does not fire because the same sentence contains `MUST use canonical absolute`. The escape clause the record calls "the load-bearing constant" is also the bypass. Note the asymmetry the record misses: M-P18-A was caught *only* because it did not happen to mention the canonical form; adding the mention makes it invisible.

Both mutants leave a rendered section containing the mandate and its own negation, and the negation covers the exact case that destroyed the issue #523 artifacts.

Severity is BLOCKER on ninth-generation recurrence (F-P12-003 → F-P13-001 → F-P14-001 → F-S2104-P14R-001 → F-S2104-P15-001 → F-S2104-P16-001 → F-S2104-P17-002 → F-S2104-P18-001 → here) via a mechanism requiring no negation, no abbreviation, no comment, no conditional, no unlisted destination and no capitalisation trick — only a semicolon.

**Zero-degrees-of-freedom fix predicate.** (a) Make the write-directive gate **clause-scoped**: after the boundary-rule sentence split, split further on `[;—]` and on `,\s+(and|or|but)\s+`, then apply the trigger and both exclusions per clause. I verified this predicate is empty on pristine and fires on all three affected mutants by literal shell:

```
$ wd() { sec "$1" | perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' \
    | perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' \
    | grep -E 'MUST|SHOULD|permits|is acceptable|is the required form|is preferred|may' \
    | grep -E 'anchor|write|writes' \
    | grep -Ev 'FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid' \
    | grep -Ev 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute' || true; }
$ wd PRISTINE
[end]                                                   <- EMPTY, no false positive
$ wd M-P19-A
**Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every `.factory/**` artifact write to the story worktree CWD
$ wd M-P19-B
MUST anchor every `.factory/**` artifact write to the worktree's `.factory/` subtree.
$ wd M-P19-D
BC-6.26.001 Invariant 5).   git-resolved ledger paths SHOULD be anchored to the story worktree CWD.
```

The pristine emptiness is not accidental and must be preserved deliberately: the section's own descriptive continuation clauses (`such writes land silently in the story worktree's shadow .factory/ subtree`, `All writes to any .factory/** path are covered by this rule`, `- **Forbidden:** … (relative path — silently writes to shadow tree)`) all survive because they carry **no** write-directive token. That asymmetry — directive clauses are gated, descriptive clauses are not — is what makes clause-scoping admissible for this gate and inadmissible for PW-B (see (b)).

(b) Do **not** clause-scope PW-B. I verified that PW-B evaluated per clause fires on the pristine em-dash continuation of S2, so clause-scoping PW-B is a false-positive regression. PW-B stays sentence-scoped as the destination-naming layer with its recorded M-P17-A / M-P17-C / in-worktree-residual proofs intact; the clause-scoped write-directive gate is what closes this class, exactly as F-S2104-P18-001 intended the two to be complementary.

(c) Record M-P19-A, M-P19-B and CONTROL-A verbatim with captured RED stdout and GREEN restores, and — closing F-S2104-P19-012 in the same burst — record the escape-clause GREEN control that leg (c) of F-S2104-P18-001 required.

(d) State in the write-directive gate's ALTERNATION-DIRECTION entry that the **escape** is now clause-scoped, and why the trigger's openness is worthless without it. The current entry claims "No new member can be added to 'evade' this gate"; M-P19-A and M-P19-B are counter-examples that require no new member at all, and the entry must be corrected rather than merely extended.

(e) The fix MUST retain RED for every vector this review re-proved RED at HEAD (enumerated in Observations).

---

### F-S2104-P19-002 — BLOCKER — the rendered section extends above the gated heading, and that region is ungated

**Stable anchors.** `bats` `_extract_write_discipline_section` (bats:147-155), whose `awk` begins at `/^#### Write Discipline/`; every section-wide gate consumes its output via `write_discipline_prose_nosplit` (bats:718-722). Target: `_shared-context.md` `### Spec-Path Discipline (canonical repo-root paths only)`, the paragraphs between the `**Enforcement:**` paragraph and the `#### Write Discipline` heading.

The extractor is:

```
_extract_write_discipline_section() {
  awk '
    /^#### Write Discipline/ { found=1; next }
    found && /^#### / { exit }
    found && /^### / { exit }
    found && /^## / { exit }
    found { print }
  ' "$SHARED_CONTEXT_MD"
}
```

F-S2104-P16-003(b) introduced this `####` bounding deliberately, and its stated benefit is recorded in the docblock: "read-discipline content above `#### Write Discipline` … is outside this bounding section by construction — M-P16-B decoy inserted in that region is excluded automatically." That reasoning is sound for a *decoy* — a compliant paragraph placed to create a second anchor. It is exactly wrong for a *harmful mandate*. The reader of the rendered document sees one continuous `### Spec-Path Discipline` section; the gates see only its last child. Nothing in the gate set constrains the prose immediately above the heading, and there is no assertion anywhere that harmful content cannot be placed there.

**M-P19-H — exact inserted text**, placed immediately after the `**Enforcement:**` paragraph's terminating blank line, three lines above the `#### Write Discipline` heading. Line 1-2 are the verbatim M-P17-A mandate; the trailing clause is the verbatim M-P17-C S2 assertion:

```

**Ledger writes:** Writers MUST anchor every `.factory/**` artifact write to the story worktree
CWD; CWD-relative paths are the required form for in-worktree ledgers.
```

**Captured stdout:**

```
########## M-P19-H (harmful mandate in Spec-Path Discipline ABOVE #### Write Discipline) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
$ bats story-worktree-write-path-discipline.bats | grep -c '^ok '
9
$ sed -n '61,68p' _shared-context.md
**Enforcement:** Before building the context package for any specialist dispatch involving spec files, the orchestrator MUST resolve the canonical repo-root path for each spec file and pass that path — not `<worktree>/.factory/<anything>`. If the canonical path cannot be resolved (e.g., factory-artifacts worktree is not mounted), STOP and report to the human before dispatching.

**Ledger writes:** Writers MUST anchor every `.factory/**` artifact write to the story worktree
CWD; CWD-relative paths are the required form for in-worktree ledgers.

#### Write Discipline — `.factory/**` artifact writes from story worktrees (BC-6.26.001 PC1, Invariants 1, 3, 4)
```

This vector carries **two** separately recorded RED vectors — `story worktree CWD` (a listed PW-B prohibited-target, M-P17-A's exact phrasing) and `CWD-relative paths are the required form` (M-P17-C S2's exact phrasing, which also carries a Gate-5 alternation member alongside `MUST`) — and every gate that catches them is blind because the text sits three lines too high. `$spec_path_section`-scoped gates (3, 6(a), 7(a), canonical-target, the clause-content markers, the no-prescriptive-revparse negative) do read this region, but all of them are bullet-shaped or presence-shaped and none constrains prose polarity.

Severity is BLOCKER: it is the primary BC postcondition remaining invertible with **zero paraphrase** — the harmful text is copied verbatim out of the recorded battery.

**Zero-degrees-of-freedom fix predicate.** (a) Extend the **write-directive gate's** domain from `#### Write Discipline` to the whole `### Spec-Path Discipline` section, retaining the F-S2104-P19-001(a) clause splitter. I verified by literal shell that this is empty on pristine and fires on M-P19-H:

```
$ secW() { awk '/^### Spec-Path Discipline/{f=1;next} f&&/^### /{exit} f&&/^## /{exit} f{print}' "$1" \
    | tr '\n' ' ' | sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g'; }
$ wdW PRISTINE
[end]                                                   <- EMPTY over the whole parent section
$ wdW M-P19-H
**Ledger writes:** Writers MUST anchor every `.factory/**` artifact write to the story worktree CWD
```

(b) Do **not** naively widen PW-B, Gate 4, Gate 5 or Gate 2b to `### Spec-Path Discipline` without a paired doc change: I verified PW-B fires on two pristine read-discipline sentences under a naive widening —

```
$ secW PRISTINE | perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | grep -E '<PW-B target class>' | grep -Ev '<PW-B prohibition class>'
Any `.factory/` content found in a story worktree is therefore live shadow-write evidence (issue #523 class) — an agent wrote to a CWD-relative `.fact…
Passing any worktree-local `.factory/` path to the adversary or any spec-reading specialist causes phantom "absent BC", "missing story spec", or "outd…
```

Either keep those four gates bounded to `#### Write Discipline` (the write-directive gate alone closes the harmful-mandate class over the parent section, per (a)), **or** widen them and bring the two read-discipline sentences into prohibition-carrying form in the same burst — the second sentence in particular currently describes a consequence (`causes phantom … findings`) where BC-6.26.001 Invariant 1 wants a prohibition, so making it compliant is a spec-quality improvement rather than a gate concession. Whichever option is taken, state it explicitly in the code comment, the story Gate cell and the audit row in the same burst, and record the pristine-empty stdout for the chosen predicate — the undocumented per-gate domain difference is the F-S2104-P18-002(d)/F-S2104-P18-005(a) failure shape repeating.

(c) Correct the `_extract_write_discipline_section` docblock: the sentence claiming the `####` bounding excludes above-heading content "by construction" must state that this holds for decoys **only**, and must name the write-directive gate as the mechanism covering harmful mandates in that region.

(d) Record M-P19-H verbatim with captured RED stdout and a GREEN restore, plus a second placement mutant in the `**No .factory/ directory is created…**` paragraph proving the widened domain reaches every above-heading paragraph, not only the one adjacent to the heading.

---

### F-S2104-P19-003 — HIGH — the write-directive gate's action class is a three-verb list, so the "open trigger" is a conjunction of two closed sides

**Stable anchor.** `bats` T-001, the block introduced by `# Gate write-directive (POSITIVE, open-trigger; F-S2104-P18-001 / F-S2104-P18-005(d)):`, the second stage of `write_directive_violations`:

```
    grep -E 'MUST|SHOULD|permits|is acceptable|is the required form|is preferred|may' | \
    grep -E 'anchor|write|writes' | \
```

The record's ALTERNATION-DIRECTION entry declares this trigger **OPEN** and asserts: "No new member can be added to 'evade' this gate by finding an unlisted paraphrase — the gate is triggered by the action word class plus any write-directive." That is false in both conjuncts. `anchor|write|writes` is a three-member surface list, and the directive list is a seven-member surface list; a sentence needs to miss only one of them.

**M-P19-C — exact inserted text**, placed immediately after the prohibition paragraph's terminating blank line:

```

**Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Ledger artifacts SHOULD be saved to
the story worktree's own `.factory/` subtree so the record lands beside the code it documents.
```

**Captured stdout:**

```
########## M-P19-C (action-word evasion: 'saved' not in anchor|write|writes) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
$ bats story-worktree-write-path-discipline.bats | grep -c '^ok '
9
```

`SHOULD` matches the directive conjunct; `saved` matches nothing in the action conjunct, so the sentence never reaches the escape stage. PW-B is blind because `the story worktree's own .factory/ subtree` names no listed destination (`worktree's[[:space:]]+shadow` and `shadow[[:space:]]+subtree` both require the literal word `shadow`). Gate 5 needs `MUST`; the sentence uses `SHOULD`. Gate 4 needs `absolute`. Gate 2b needs a nullification synonym or an adversative. Nothing fires.

The unlisted-verb class is large and every member is ordinary prose for this domain: `saved`, `stored`, `placed`, `persisted`, `emitted`, `recorded`, `deposited`, `created`, `landed`, `kept`, `materialised`. The directive conjunct has the same problem in the other direction: a bare imperative — `Anchor every \`.factory/**\` artifact write to the delivery sandbox root.` — carries a listed action word and no listed directive, and reads to an agent as a mandate.

**Zero-degrees-of-freedom fix predicate.** (a) Replace the action conjunct with a **referent** predicate rather than a verb list: trigger on any clause that contains `\.factory/` **or** `ledger` **or** `artifact` (the objects the obligation governs), which cannot be paraphrased away without ceasing to talk about the subject. (b) Replace the directive conjunct with the union of the current list and the **imperative/modal-free** case: trigger additionally on any clause whose first token after optional `**bold:**` labelling is a bare verb from a class including `Anchor|Write|Save|Store|Place|Record|Emit|Persist|Resolve|Use`. (c) Verify by literal shell that the combined predicate is empty over the pristine `### Spec-Path Discipline` section (this is the leg most likely to false-positive; the `- **Correct:**` / `- **Forbidden:**` bullets and the `**Canonical root determination:**` fence all reference `.factory/`, so the escape-clause and prohibition-token exclusions must be checked to carry them) and fires on M-P19-C and on the bare-imperative variant. (d) Record M-P19-C and the bare-imperative variant verbatim with RED stdout and GREEN restores, one mutant per added conjunct member. (e) Correct the ALTERNATION-DIRECTION entry's "no new member can evade this gate" claim, naming M-P19-C as the counter-example, per the error-acknowledgment discipline used at v1.14/v1.15/v1.16.

---

### F-S2104-P19-004 — HIGH — the splitter's missed-boundary direction is fail-open for the exclusion-based gates

**Stable anchor.** `bats` T-001, the boundary-rule splitter `perl -pe 's/\.[[:space:]]+(?=[A-Z*\`\[])/.\n/g'`, applied at every gate site (bats:751, 814, 845, 864, 891, 911, 936, 967). Record anchor: red-gate-log v1.16 §`Extraction and normalization mechanisms … at a4ec37d3`, row `Boundary-rule sentence-splitter`.

POLICY 13's NORMALIZATION-ADVERSARIALITY clause requires "false-boundary mutants for tokenizers", and pass-18 delivered exactly that: M-P18-B plus six abbreviation variants, all RED. The clause has a second direction nobody probed. Splitting *only* before `[A-Z*\`\[]` means a sentence beginning with a lowercase word, a digit, a quote or an opening paren is **not** split — it is merged into its predecessor. For Gates 4 and 5 merging is fail-closed (it makes co-occurrence more likely). For Gate PW-B and the write-directive gate, whose final stage is `grep -Ev`, merging is fail-**open**: the exclusion applied to the predecessor's prohibition token now covers the appended harmful clause.

**M-P19-D — exact inserted text**, placed immediately after the prohibition paragraph's terminating blank line:

```

git-resolved ledger paths SHOULD be anchored to the story worktree CWD.
```

Because the section is flattened with `tr '\n' ' '` before splitting, paragraph boundaries are already gone; the lowercase `g` therefore merges this sentence into the prohibition paragraph's S2, which carries `FORBIDDEN`.

**Captured stdout:**

```
########## M-P19-D (missed-boundary: lowercase sentence start merges with FORBIDDEN sentence) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
$ bats story-worktree-write-path-discipline.bats | grep -c '^ok '
9
```

**Load-bearing control — CONTROL-D**, identical except the initial `g` is capitalised:

```
########## CONTROL-D (identical, capital G -> boundary IS created) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 820)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide sentence polarity (Gate PW-B, …)]: …
# Git-resolved ledger paths SHOULD be anchored to the story worktree CWD.
```

One character of casing is the entire difference between RED and GREEN. Lowercase sentence-initial tokens are not exotic in this codebase's prose — `git`, `find`, `bats`, `cargo`, `pr-review.md` and `\`.factory/\`` all begin sentences naturally in the surrounding documents. Note also that `SHOULD` rather than `MUST` is required to keep Gate 5 quiet, which is a one-word adjustment.

**Zero-degrees-of-freedom fix predicate.** (a) The structural fix is F-S2104-P19-001(a): once the exclusion is clause-scoped, a merged clause no longer inherits its neighbour's prohibition token. I verified this closes M-P19-D — see the `wd M-P19-D` stdout under F-S2104-P19-001(a), which surfaces the merged clause as a violation. (b) In addition, add a **boundary-completeness assertion** so the tokenizer cannot silently under-split: assert that the number of clauses produced by the splitter is not less than the count of `\.[[:space:]]` occurrences in the domain minus the protected-abbreviation count, and RED with an explicit `sentence boundary missed` message otherwise. (c) Record M-P19-D and CONTROL-D verbatim as the missed-boundary pair, and add the symmetric variants that begin with a digit and with a double quote, one mutant each. (d) Amend the `Boundary-rule sentence-splitter` row in the extraction-mechanisms table: it currently records only the false-boundary direction and asserts the mechanism "MUST carry mutants … manufacturing a false boundary from ordinary prose". POLICY 13 NORMALIZATION-ADVERSARIALITY must be read as requiring **both** directions, and the row must say so — a tokenizer that can silently merge two sentences is as dangerous as one that can silently split one.

---

### F-S2104-P19-005 — HIGH — the link-ref-def strip requires whitespace CommonMark makes optional

**Stable anchor.** `bats` T-001, the block introduced by `# F-S2104-P18-004(a): rendered_write_discipline domain — strip non-rendered Markdown before` assigning `rendered_write_discipline` (bats:682-685):

```
  rendered_write_discipline="$(printf '%s\n' "$write_discipline_section" | \
    sed 's/<!--[^>]*-->//g' | \
    grep -Ev '^[[:space:]]{0,3}\[[^]]*\]:[[:space:]]')"
```

The drop predicate requires at least one whitespace character after the colon. The CommonMark specification defines a link reference definition as a link label, indented up to three spaces, followed by a colon, **optional** whitespace, a destination, optional whitespace and an optional title. Zero whitespace is conformant, so `[//]:# (…)` is a link reference definition and renders as nothing — while failing the drop predicate. I confirmed both halves:

```
$ printf '[//]: # (with space)\n[//]:# (no space)\n' | grep -Ev '^[[:space:]]{0,3}\[[^]]*\]:[[:space:]]'
[//]:# (no space)
```
```
$ printf '[//]:# (HIDDEN MANDATE TEXT)\nVisible line.\n' > /tmp/lrd.md
$ npx -y marked /tmp/lrd.md
<p>Visible line.</p>
$ pandoc -f commonmark -t html /tmp/lrd.md
<p>Visible line.</p>
```

Two independent CommonMark implementations emit nothing for the hidden text.

**M-P19-F — exact substituted text**, replacing the normative prohibition paragraph (`_shared-context.md` lines 66-70):

```
[//]:# (All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths anchored to the main-checkout root. CWD-relative paths are FORBIDDEN and land in the shadow subtree.)
Writers anchor every `.factory/**` artifact to the delivery sandbox root so the ledger lands beside the code it documents.
```

**Captured stdout:**

```
########## M-P19-F ([//]:# no-space link-ref-def hides mandate) ##########
$ sed -n '64,68p' _shared-context.md
#### Write Discipline — `.factory/**` artifact writes from story worktrees (BC-6.26.001 PC1, Invariants 1, 3, 4)

[//]:# (All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths anchored to the main-checkout root. CWD-relative paths are FORBIDDEN and land in the shadow subtree.)
Writers anchor every `.factory/**` artifact to the delivery sandbox root so the ledger lands beside the code it documents.

1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
$ bats story-worktree-write-path-discipline.bats | grep -c '^ok '
9
```

**Load-bearing control — CONTROL-F**, byte-identical except one space after the colon (i.e. the pass-18 M-P18-D form):

```
########## CONTROL-F (identical but WITH space after colon — pass-18 M-P18-D form) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 695)
#   `false' failed
# DOC-PARITY FAIL [ambiguous anchor in #### Write Discipline rendered domain (F-S2104-P16-003(a)/F-S2104-P18-004(b))]: found 0 match(es) of anchor 'All.*\.factory.*artifact writes' in the rendered #### Write Discipline domain (expected exactly 1); …
```

With the space, the anchor is stripped and the count falls to 0 → RED. Without it, the anchor stays in the rendered domain, count = 1, `prohibition_block` is extracted from inside the invisible definition, Gate 1(a) matches `MUST use canonical absolute` there, Gate 2a finds `CWD-relative … FORBIDDEN` there, and the reader of the rendered document sees exactly one instruction — directing `.factory/**` writes to the delivery sandbox. The `<!--` absence gate is silent because there is no HTML comment.

This is the F-S2104-P18-004 class re-seeded through the strip mechanism's own regex rather than through a new construct, which is the NORMALIZATION-ADVERSARIALITY pattern one level deeper than pass-18 reached.

**Zero-degrees-of-freedom fix predicate.** (a) Correct the drop predicate to `^[[:space:]]{0,3}\[[^]]*\]:` — colon-terminated label with **no** whitespace requirement, matching CommonMark. Verify by literal shell that the pristine section survives the corrected stripping unchanged (no line in `#### Write Discipline` matches the label form, so the rendered domain must be byte-identical to the raw section modulo the comment `sed`) and that the mandate sentence is still found and Gate 1(a) still passes. (b) Record M-P19-F and CONTROL-F verbatim with RED stdout and GREEN restore, plus the bare `[//]:#` form with no parenthesised title and the three-space-indented variant, one mutant each. (c) Add the **positive** render-fidelity assertion the class needs and F-S2104-P18-004(b) only partially delivered: assert that the mandate sentence found in `rendered_write_discipline` is byte-identical to the mandate sentence found in the raw section, so any future stripping asymmetry surfaces as a diff rather than as a silent domain shrink. (d) In the extraction-mechanisms table, record that the `rendered_write_discipline` **strip predicates are themselves gate surfaces** carrying their own mutants, and add a note that a fourth-space indent is an indented code block (rendered, therefore correctly *not* stripped) so a future burst does not "helpfully" widen the indent range and start stripping visible content.

---

### F-S2104-P19-006 — MEDIUM — the canonical-target gate's domain is gated on the literal `file_path=`

**Stable anchor.** `bats` T-001, the block introduced by `# Gate canonical-target (NEGATIVE, F-S2104-P17-004(b)):` assigning `noncanonical_correct_bullets` (bats:1031-1039):

```
  noncanonical_correct_bullets="$(printf '%s\n' "$spec_path_section" | \
    grep -E '\*\*Correct:\*\*' | grep -E 'file_path=' | \
    grep -Ev 'file_path=["'"'"']?(\$CANONICAL_FACTORY_ROOT|/)' || true)"
```

Pass-18 recorded this gate as the strongest single piece of work in the cascade, and its *predicate* is genuinely class-based — I could not defeat the canonical-target test itself. But its *domain* is not: the second stage restricts evaluation to bullets containing the literal `file_path=`. A `**Correct:**` bullet that names a non-canonical target in any other rendering is outside the domain, so the gate never sees it.

**M-P19-E — exact inserted text**, appended as a fourth bullet after the traversal `**Forbidden:**` bullet:

```
- **Correct:** `Write` the DELIVERY ledger to `.factory/stories/S-NNN-DELIVERY.md` resolved from the worktree root
```

**Captured stdout:**

```
########## M-P19-E (**Correct:** bullet, non-canonical target, no file_path= token) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
$ bats story-worktree-write-path-discipline.bats | grep -c '^ok '
9
```

The bullet is rendered beside three correctly-labelled bullets and instructs the reader to write a CWD-relative ledger path. Gates 3, 6(a) and 7(a) are positive existence assertions and still pass. The write-directive gate is blind twice over: capital `Write` misses the case-sensitive `write|writes` class, and no directive token is present. `resolved from the worktree root` names no PW-B destination. Other renderings in the same class: `path=`, `target=`, `--output`, or a bare backticked path with no keyword at all.

**Zero-degrees-of-freedom fix predicate.** (a) Widen the domain conjunct from `file_path=` to **any path-bearing token** on a `**Correct:**` bullet: replace `grep -E 'file_path='` with `grep -E '\.factory/'`, so every `**Correct:**` bullet that names a `.factory/` path is in scope regardless of the keyword used. (b) Widen the canonical predicate correspondingly to accept a canonical path in any position: require that every `.factory/` occurrence on a `**Correct:**` bullet be immediately preceded by `$CANONICAL_FACTORY_ROOT/` or by `/`. Verify by literal shell that the predicate is empty over the pristine `### Spec-Path Discipline` section (the one live `**Correct:**` bullet is `file_path="$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md"`, which satisfies it) and fires on M-P19-E, M-P17-G, M-P16-D and M-P15-B. (c) Record M-P19-E verbatim with RED stdout and GREEN restore, plus a `path=`-keyword variant and a keyword-free backticked-path variant, one mutant each. (d) Update the canonical-target row in the audit table and item (19) of the story Gate cell in the same burst: both currently describe the domain as "**Correct:** bullets with `file_path=`", which after this fix is no longer the domain.

---

### F-S2104-P19-007 — MEDIUM — the nullification class is still closed and its declared backstop cannot reach directive-free prose

**Stable anchors.** `bats` T-001, `retirement_language` (Gate 2b(a), bats:863-870) and `forbidden_sentences_with_adversative` (Gate 2b(c), bats:890-898). Record anchor: red-gate-log v1.16 §`ALTERNATION-DIRECTION STATEMENTS`, the `Gate 2b(a)` entry.

The direction statement declares Gate 2b(a) "(b) **backed by write-directive gate** (open trigger)" and justifies it: "any sentence that concedes writes are permitted or acceptable (`permits`, `may`, `is acceptable`) triggers the write-directive gate if it also contains an action word." The justification only covers nullifications phrased as concessions carrying a listed directive. A nullification phrased as a plain predication carries none.

**M-P19-G — exact inserted text**, placed immediately after the prohibition paragraph's terminating blank line:

```

**Scope note:** The prohibition stated above is not applicable to ledger writes made from the
delivery sandbox.
```

**Captured stdout:**

```
########## M-P19-G ('is not applicable' — unlisted nullification, no directive token) ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
$ bats story-worktree-write-path-discipline.bats | grep -c '^ok '
9
```

Three gates are individually blind for three different reasons. Gate 2b(a): `is not applicable` matches no member of the 25-item class — `does[[:space:]]+not[[:space:]]+apply` requires the verb form `apply`, not the adjective `applicable`, and the listed `exempt` is absent. Gate 2b(c): the sentence *does* match the widened trigger (`prohibition`, `above`) but carries no member of the 9-item adversative class. Write-directive gate: `writes` is present but no directive token is, so the trigger's first conjunct fails. The rendered section's net instruction is that the prohibition does not cover the only case that caused the data loss. `is not applicable` is one morphological inflection away from a listed member — the same one-hop distance as M-P18-E's `does not bind` from `does not apply`, which is the third consecutive pass in which widening this list has failed on the first paraphrase tried.

**Zero-degrees-of-freedom fix predicate.** (a) Add the structural gate that makes the list non-load-bearing, which is what F-S2104-P17-003(c) and F-S2104-P18-005(d) both attempted and neither achieved: assert that **no** sentence matching the prohibition-reference trigger (`FORBIDDEN|forbidden|prohibition|prohibited|the rule|this rule|the constraint|above`) may also match a **scope-restriction** class — `not applicable|does not|is not|except|outside|limited to|only (covers|applies)|other than|save for|apart from|excluding`. Verify by literal shell that this is empty over the pristine section before landing (the pristine `above`-bearing sentence is `All writes to any \`.factory/**\` path are covered by this rule — not only DELIVERY ledgers.`, which contains `not only`; the predicate must therefore be checked and, if it false-positives, the sentence reworded to `— DELIVERY ledgers are not the only case` in the same burst rather than the class narrowed) and fires on M-P19-G. (b) Add the F-S2104-P19-003(a) referent-based trigger to the write-directive gate so `ledger writes` reaches the gate without needing a listed directive — that closes M-P19-G a second way and is the backstop the direction statement claims to already have. (c) Add `not applicable|inapplicable|does not cover|does not extend|out of scope` to the 2b(a) class with one mutant each, but record explicitly that this widening is subordinate to (a) — per POLICY 13 ALTERNATION-WIDENING-DIRECTION-STATEMENT a widening that is not accompanied by an open-trigger companion is a paper-fix by construction. (d) Correct the Gate 2b(a) direction statement: its claim that the write-directive gate closes the axis is refuted by M-P19-G and must be restated, with M-P19-G named, rather than left standing.

---

### F-S2104-P19-008 — MEDIUM — the shipped test file declares seventeen gates against a nineteen-gate set

**Stable anchors.** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`, the T-001 lead-in comment beginning `# --- DOC-PARITY §Spec-Path Discipline: AC-001(a) CWD-relative-path PROHIBITION (F-S2104-P12-003 .. F-S2104-P18-001/002/003/004/005) ---` (the sentence containing the count, bats:548), and the comment closing the same block (bats:642).

Verbatim:

```
$ grep -n 'seventeen\|Seventeen\|nineteen\|Nineteen' story-worktree-write-path-discipline.bats
548:  # FORBIDDEN and that canonical absolute paths are MANDATED. Seventeen independently mutant-proven
642:  # All seventeen gates survive independently.
```

Both counts are stale. The enumeration immediately beneath the first site correctly describes the pass-18 additions, and the three artifacts that state a count all say nineteen: story v1.23's Gate cell (`bats T-001 (19 gates, …)` with items numbered (1)–(19)), red-gate-log v1.16 §`Gate-indexed audit table (T-001 / AC-001 gates at a4ec37d3 — 19 gates; Gates 6(b)/7(b) RETIRED)`, and the NAME-SET EQUALITY check, which I reproduced at HEAD:

```
$ diff /tmp/story_g.txt /tmp/log_g.txt && echo "NAME-SET EQUALITY: PASS ($(wc -l < /tmp/story_g.txt) gates, diff empty)"
NAME-SET EQUALITY: PASS (      19 gates, diff empty)
```

This is a POLICY 14 quintuple-parity break in the artifact with the most authority — the test file is the SoT the other four describe. It also defeats the point of the NAME-SET EQUALITY mechanism: that check compares the story cell against the audit table and never reads the bats file's own count claim, so the one site that can contradict the predicate set is outside the only gate built to detect contradictions. Same class as F-S2104-P17-006/P17-007, one pass after both were closed, and it slipped because the same-burst coupling note in the story cell binds *the Gate cell* to predicate changes and does not bind the bats file's own prose to itself.

**Zero-degrees-of-freedom fix predicate.** (a) Correct both sites to `Nineteen` / `All nineteen gates survive independently`. (b) Add the count to the NAME-SET EQUALITY check's scope so it cannot drift again: extract the count word from the bats lead-in via literal shell (`grep -oE '(Seventeen|Nineteen|Twenty[a-z-]*) independently mutant-proven'`), map it to the integer, and assert equality with `wc -l` of the extracted label set, recording the stdout. A count that no mechanism reads is a count that will be wrong. (c) Extend the story Gate cell's coupling note so it binds in both directions: any burst that changes a T-001 predicate must update the Gate cell **and** the bats lead-in enumeration and count in the same burst. (d) Sibling-sweep the remaining count claims in the same file by literal shell and record the result — the block at bats:546-642 also enumerates per-pass additions, and each enumeration is a count claim in prose form.

---

### F-S2104-P19-009 — MEDIUM — STORY-INDEX carries the red-gate-log's input-hash in the story's row

**Stable anchors.** `.factory/stories/STORY-INDEX.md` v4.266, the `| S-21.04 |` catalog row (the `input-hash` clause in its notes cell) and the `> **E-21 delivery:**` blockquote (`S-21.04=…`). Compared against `.factory/stories/S-21.04-story-worktree-write-path-discipline.md` frontmatter `input-hash:`.

The convention is established by the four uncontested siblings — catalog row, blockquote and live story frontmatter agree — and was verified in that form one pass ago. S-21.04 now breaks it:

```
--- S-21.04  frontmatter=1165b1f   catalog-row input-hash occurrences: input-hash f86871a
    body blockquote (line 731): S-21.04=f86871a
--- S-21.02  frontmatter=8bd32e5   catalog=8bd32e5   blockquote=8bd32e5   MATCH
--- S-21.03  frontmatter=59e687e   catalog=59e687e   blockquote=59e687e   MATCH
--- S-21.05  frontmatter=c9265f0   catalog=c9265f0   blockquote=c9265f0   MATCH
--- S-21.06  frontmatter=b807086   catalog=b807086   blockquote=b807086   MATCH
```

`f86871a` is not a stale value and not a drifted computation — it is the **red-gate-log's own** frontmatter hash:

```
$ grep -n '^input-hash:' .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
12:input-hash: "f86871a"
```

The red-gate-log's transition `4b26b3b→f86871a` is legitimate — its inputs include the story file, which changed at v1.23. What is not legitimate is copying that value into the index row that mirrors the **story's** input-hash. Story v1.23 makes no input-hash claim in either `last_amended` or `modified[]`, consistent with the `input-hash unchanged (hook-authoritative per L-EDP1-073)` pattern used at v1.14 and v1.17–v1.19, so the story's stored `1165b1f` is what the story asserts and the index is what moved. The STORY-INDEX `last_amended` records the mis-propagation as fact: `input-hash 1165b1f→f86871a (story v1.23); blockquote S-21.04=1165b1f→f86871a`.

This is not the known-context dual-binary divergence — no attestation is missing, and the value is concrete and traceable to the wrong artifact. It is a cross-artifact hash confusion that makes the index assert something about the story that the story does not assert, and it is already propagating: the pass-19 dispatch brief repeated it as "story input-hash changed to f86871a at v1.23".

**Zero-degrees-of-freedom fix predicate.** (a) Restore both STORY-INDEX sites to the live story frontmatter value `1165b1f`, or — if the story's stored hash is genuinely stale — run `compute-input-hash` against the story, update the story frontmatter first, then propagate the computed value to both index sites in the same burst. Decide which by executing the tool and recording its stdout; do not choose by inspection. (b) Record the correction as a new v4.267 changelog row carrying an explicit error-acknowledgment clause naming both sites and stating that `f86871a` is the red-gate-log's hash, per the discipline used at v1.14/v1.15/v1.16. (c) Add the parity check as a literal-shell gate at closure: for each E-21 story, assert frontmatter `input-hash` equals the catalog-row value equals the blockquote value, and record the stdout. The pass-18 review checked only that the six hashes were *distinct*, which is why a wrong-but-distinct value passed. (d) Sibling-sweep: my run of the same check surfaced a second divergence outside this story's perimeter — `S-21.01` has frontmatter `32aaccc` and blockquote `32aaccc` but catalog row `fde01eb`. That one predates this burst and is not attributable to it; route it, do not silently fold it in, and record the routing.

---

### F-S2104-P19-010 — MEDIUM — the partition paragraph says TWO and enumerates three

**Stable anchor.** red-gate-log v1.16 §`NAME-SET EQUALITY gate-label parity check at a4ec37d3`, the `**Partition definition (updated from pass-17):**` paragraph.

Verbatim, in relevant part: `TWO additional T-001 assertions exist OUTSIDE that partition and are recorded in the gate-indexed table with \`partition: clause-content/structural\` markers: EC-006-presence (AC-001(b)), no-revparse-outside-WARNING (AC-001(b)), mandate-sentence-present (structural guard).`

Three items are enumerated. The pass-17 paragraph, preserved unmodified in the same document, has it right:

```
$ grep -n 'additional T-001 assertions exist OUTSIDE' red-gate-log.md
806:… THREE additional T-001 assertions exist OUTSIDE that partition … EC-006-presence (AC-001(b)), no-revparse-outside-WARNING (AC-001(b)), mandate-sentence-present (structural guard). …
1032:… TWO additional T-001 assertions exist OUTSIDE that partition … EC-006-presence (AC-001(b)), no-revparse-outside-WARNING (AC-001(b)), mandate-sentence-present (structural guard). …
```

The v1.16 rewrite regressed a correct count while carrying the identical enumeration forward. The out-of-partition set is exactly what makes the NAME-SET EQUALITY check's 19-label partition sound; a paragraph that miscounts its own complement undermines the disclosure F-S2104-P17-006(a) was raised to obtain. This is the POLICY 15 count-after-enumeration discipline applied to itself.

**Zero-degrees-of-freedom fix predicate.** (a) Correct `TWO` to `THREE` at the v1.16 site; leave the v1.15 site unmodified. (b) Record the correction as a new changelog row with an explicit error-acknowledgment clause naming the site. (c) Add a literal-shell assertion at closure that the stated count equals the number of semicolon-or-comma-separated items in the enumeration that follows it, and record the stdout — the same mechanical discipline D-449(a) applies to gate attestations, applied to record counts. (d) Re-run the NAME-SET EQUALITY check after the edit and record its stdout, since the partition paragraph is the check's own scope definition.

---

### F-S2104-P19-011 — MEDIUM — the balanced-fence gate's rationale describes a mechanism that no longer exists

**Stable anchors.** Three sites, all describing the same gate. (i) `bats` T-001, the comment block beginning `# F-S2104-P18-002(a): balanced-fence assertion — an unbalanced opening fence flips the awk` (bats:656-663). (ii) story v1.23 AC-001 Gate cell, item `(2) balanced-fence POSITIVE`. (iii) red-gate-log v1.16 audit table, row `| balanced-fence |`.

Verbatim:

```
  # F-S2104-P18-002(a): balanced-fence assertion — an unbalanced opening fence flips the awk
  # in_fence state and keeps it flipped forever, silently dropping every remaining line of the
  # section from the prose domain. Three characters can erase PW-B, Gate 2b(a), Gate 4 and
  # Gate 5 across the whole section (M-P18-C(b) at 9/9).
```
```
(2) balanced-fence POSITIVE — fence-marker count … must be even; unbalanced opening fence silently drops section remainder from all fence-aware domains (M-P18-C(b): 3 fences → PW-B/2b(a)/4/5 silenced; …
```
```
| balanced-fence | Raw `#### Write Discipline` section — `grep -cE '^[[:space:]]*```'` | POSITIVE: count must be even; odd count means section domain is truncated; tilde (~~~) fences NOT matched — fail safe by construction |
```

All three are false at HEAD, because F-S2104-P18-002(b) removed the fence-stripping `awk` in the same commit:

```
$ grep -nE "awk .*in_fence" story-worktree-write-path-discipline.bats
(no fence-stripping awk present)
$ grep -n 'in_fence' story-worktree-write-path-discipline.bats
657:  # in_fence state and keeps it flipped forever, silently dropping every remaining line of the
$ grep -n 'write_discipline_prose="' story-worktree-write-path-discipline.bats
719:  write_discipline_prose="$(printf '%s\n' "$write_discipline_section" | tr '\n' ' ')"
```

No domain is fence-aware. An unbalanced fence cannot truncate anything, cannot silence PW-B/2b(a)/4/5, and the "tilde fences fail safe" note is vacuous because nothing excludes fences of either delimiter. The only surviving reference to `in_fence` in the file is the comment asserting its behaviour in the present tense.

The gate itself is fine and I am not proposing its removal — an odd fence count is a real well-formedness defect and M-P18-C(b) is legitimately RED against it. The defect is that all three records state a *causal* justification that HEAD contradicts, which is the F-S2104-P18-006 class (record attesting behaviour its own code refutes) reappearing on a different gate, and which under TD-VSDD-059 is the doc-comment-claiming-an-absent-mechanism pattern. Its practical cost is that a future burst reading these three records will believe the section-wide gates have a fence-truncation exposure they do not have, and may reintroduce fence-awareness to "restore" a guard that F-S2104-P18-002(b) deliberately deleted.

**Zero-degrees-of-freedom fix predicate.** (a) Rewrite all three sites to state the gate's actual property: the `#### Write Discipline` section must be well-formed Markdown (paired fence delimiters), asserted so that a malformed section cannot be authored, **and** record that no gate domain is fence-aware at HEAD, so the assertion is a well-formedness invariant rather than a truncation guard. Preserve the historical account — the truncation exposure was real at `c89bef22` and M-P18-C(b) documents it — by attributing it explicitly to that commit rather than to HEAD. (b) Delete or rewrite the tilde-fence note: with no fence-aware domain there is no fail-safe property to preserve, and the note as written invites exactly the "generalise the fence matcher" change F-S2104-P18-002(c) warned against. (c) Correct the balanced-fence audit row's `odd count means section domain is truncated` to the well-formedness statement, and align story cell item (2) in the same burst per the coupling note. (d) Add a literal-shell assertion to the closure burst that `grep -c 'in_fence' <bats>` returns 0 excluding comment lines, or that no comment claims a mechanism whose identifier is absent from the executable body — the general form of the D-449(a) discipline applied to code comments.

---

### F-S2104-P19-012 — MEDIUM — the escape-clause GREEN control the pass-18 fix predicate required is not in the record

**Stable anchor.** red-gate-log v1.16 §`Battery table — vectors at a4ec37d3`, and §`Verbatim captured stdout — new vectors (T-001 only)`.

F-S2104-P18-001's fix predicate leg (c) reads, verbatim: "Record M-P18-A verbatim with captured RED stdout and a GREEN restore, plus a control that changes only `MUST anchor` to `MUST use canonical absolute` and stays GREEN, proving the affirmative-escape clause is load-bearing rather than vacuous."

The RED half landed with verbatim stdout. The GREEN control did not. The battery table's only control row is `M-P17-C-control`, and the verbatim-stdout block has no escape-clause entry:

```
$ grep -nE '^\| .*control' red-gate-log.md
660:| M-P17-C-control | M-P17-C with `are the required form` → `are used` | Gate 1(d): …
```

The audit table's write-directive row substitutes a different attestation — `GREEN control: adversary verified empty-on-pristine by literal shell at c89bef22` — which is a true statement about a *different* property at a *different* commit. Empty-on-pristine proves the gate does not false-positive. It does not prove the escape clause discriminates, and the escape clause is the load-bearing constant the whole open-trigger design rests on. I supplied the missing control myself and it is GREEN:

```
########## CONTROL write-directive escape (M-P18-A with 'MUST use canonical absolute') ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

The omission is not bookkeeping. The escape clause is precisely the mechanism F-S2104-P19-001 leg (b) exploits, and the control that was skipped is the one whose *scope* a reviewer would have had to reason about to notice that a whole-sentence substring match exempts every clause in the sentence. A control recorded only as "the escape works" and never as "the escape works and covers exactly this much text" is how a sentence-scoped exclusion shipped as a closed pair.

**Zero-degrees-of-freedom fix predicate.** (a) Record the escape-clause GREEN control as a battery row with captured stdout, under a label distinguishing it from empty-on-pristine. (b) Add its **negative twin**, which is the control that would have caught F-S2104-P19-001: the same sentence with the escape phrase moved into a trailing independent clause (`Writers MUST anchor every \`.factory/**\` artifact write to the story worktree CWD; writers MUST use canonical absolute paths for spec reads.`) must be **RED** after the F-S2104-P19-001(a) fix, proving the escape is clause-scoped rather than sentence-scoped. (c) Correct the write-directive audit row's `GREEN control` cell to distinguish the two attestations, and state that empty-on-pristine at `c89bef22` was verified against the *proposed* predicate before the gate existed. (d) Codify the general rule in the extraction-mechanisms table or the ALTERNATION-DIRECTION section: every gate carrying an **escape clause** must record two controls — one proving the escape fires (non-vacuous) and one proving its scope is no wider than the clause containing the trigger (non-over-broad). Pass-18 required the first and got it; nothing required the second, and that is the structural gap that produced this pass's BLOCKER.

---

## Observations (NOT findings)

**Twelve recorded vectors independently re-proven RED at HEAD from verbatim Part A / battery text.** M-P18-A → write-directive gate (bats:975); M-P18-B → Gate 4 via the boundary-rule splitter; M-P18-C → PW-B, with the harmful comment placed inside the bash fence at the `CANONICAL_FACTORY_ROOT="$(git -C "$main_worktree_path" rev-parse --show-toplevel)"` line (note: the pass-18 report's line-offset placement no longer lands inside the fence at HEAD — the correct anchor is the `CANONICAL_FACTORY_ROOT=` line, and I record that here so a future burst reproducing M-P18-C does not conclude the vector has regressed, as my first attempt did); M-P18-C(b) → balanced-fence, odd count 3; M-P18-D → rendered-domain anchor count 0; M-P18-E and M-P18-F → Gate 2b(a) widened class; M-P18-G → Gate 2b(c) section-wide; M-P17-A → PW-B; M-P17-G → canonical-target; M-P17-H → HTML-comment absence; M-P16-A → Gate 1(a); M-P16-B in-section decoy → anchor-uniqueness count 2; M-P15-A S1 → Gates 1(a)/(c). No pass-18 leg that landed is a paper-fix, and none of my findings is a false attestation of one.

**The write-directive gate's escape clause is genuinely non-vacuous.** Replacing M-P18-A's `MUST anchor … subtree` with `MUST use canonical absolute paths` returns 9/9 GREEN. The gate discriminates; F-S2104-P19-001 is about the escape's *scope*, not its existence.

**NAME-SET EQUALITY reproduces exactly at HEAD.** I re-ran the recorded `grep -oP` extraction against story v1.23 and diffed against the audit table's 19 labels: diff empty, 19 labels. The `write-directive gate` and `balanced-fence` labels were correctly added to the extraction's lookahead alternation, which is the kind of same-burst mechanism update that usually gets missed. Story v1.23's Gate cell is bidirectionally accurate against T-001 on every predicate I checked: the domain groupings (items (1)–(2) raw section, (3)–(4) rendered domain, (5)–(9) `joined_block_nosplit`, (10)–(15) `write_discipline_prose_nosplit`, (16)–(19) `$spec_path_section`) all match the code, the 25-member nullification class and 9-member adversative class are quoted correctly, and Gate 5's alternation correctly reads `relative[[:space:]]+path` with the `not paths?` note.

**The Gate-5 sequence-shadowing disclosure is honest and complete.** The F-S2104-P18-006 correction landed at both sites, the false Gate-5 claim on M-P17-A is gone, and the `SEQUENCE-SHADOWED DEFENSE-IN-DEPTH` framing accurately describes what the recorded vectors do and do not prove. Per the dispatch brief this is known context; I record only that I verified it rather than assumed it.

**Zero live POLICY 19 tokens in the S-21.04 perimeter.** Every `ADR-031 v[0-9]` and `BC-6.26.001 v[0-9]` occurrence in the story, BC-6.26.001, the epic and the red-gate-log resolves to a `modified[]` array entry, a `last_amended` clause, or a changelog-table row — historical-by-construction, exempt per POLICY 5. Cross-document version parity holds otherwise: BC-6.26.001 v1.11, BC-6.27.001 v1.4, ADR-031 v1.13, epic v1.8, STORY-INDEX v4.266, story v1.23, red-gate-log v1.16 with `traces_to` correctly adding `story v1.23`, and the red-gate-log Summary HEAD correctly advanced to `a4ec37d3`.

**CHANGELOG is accurate.** The count-free lead-in survives (`the two BC-6.26.001 protocol requirements plus the propagation and awareness legs`), items (1)–(5) match the delivered surfaces in the 19-file diff, all five named sibling teardown sites appear in the diff, and item (1)'s prose correctly describes `$CANONICAL_FACTORY_ROOT` as the repo root rather than the mount.

**Both suites are GREEN and the fixture tree is clean.** `git -C .worktrees/S-21.04 status --porcelain` is empty at `a4ec37d3`; 9/9 and 14/14. Every mutant in this report was run against a scratch copy of the `plugins/` tree with the unmodified bats suite, and the pristine `_shared-context.md` was restored after each.

**[process-gap] — the escape clause is the new frontier, and it is structurally invisible to the current review protocol.** Passes 12–17 attacked triggers; pass-18 attacked normalization mechanisms; pass-19's BLOCKER is the first that attacks the **exclusion** side of a fail-closed implication. The pattern is exact and general: `grep -E '<trigger>' | grep -Ev '<escape>'` is sound only when trigger and escape are evaluated over the same syntactic unit as the obligation they encode. Here the obligation is per-clause (a clause either mandates a canonical write or it does not) while the evaluation is per-sentence, so any sentence containing one compliant clause launders every other clause in it. Candidate codification — **ESCAPE-SCOPE-PARITY**: for every gate of the form *trigger implies escape*, the record MUST state the syntactic unit over which each side is evaluated, and the two MUST be the same unit; where they differ, the gate MUST carry a mutant that places a harmful clause and an escape clause in the same unit. This is the natural companion to FAIL-CLOSED-IMPLICATION-DIRECTION: pass-18 codified that the *trigger* must be open, and pass-19 shows that an open trigger with an over-broad escape is no stronger than a closed trigger. Note the self-application: F-S2104-P19-012 shows the record already had a slot for this — leg (c) of the pass-18 predicate asked for an escape control and got none — so the codification is as much about *executing* the required control as about adding a new one.

**[process-gap] — a domain boundary introduced to suppress a false positive is an attack surface, and no review step asks which side of it harmful content can sit on.** F-S2104-P16-003(b) bounded the gate domain to `#### Write Discipline` to neutralise the M-P16-B decoy, and recorded the benefit in the docblock as holding "by construction". It does hold for decoys. F-S2104-P19-002 shows the same boundary is a shelter for harmful mandates in rendered text three lines away. The same shape appears in F-S2104-P19-006 (the `file_path=` filter, added to make the canonical-target predicate tractable, excludes non-`file_path=` renderings entirely) and in F-S2104-P19-005 (the strip predicate's whitespace requirement, added to match one comment idiom, excludes an equally valid one). Candidate codification — **BOUNDARY-POLARITY**: whenever a gate's domain is narrowed — by heading bound, by line filter, by token filter, or by exclusion — the burst MUST record (i) the class of content the narrowing was introduced to exclude, (ii) whether harmful content of the *opposite* polarity can occupy the excluded region while remaining rendered to the reader, and (iii) a mutant proving the answer. A narrowing justified only by the false positive it suppresses is half an argument, and every one of this pass's five non-record HIGH/MEDIUM findings lives in the unexamined half.

**[process-gap] — the tokenizer mutant requirement was implemented in one direction only, because the codification named one direction.** POLICY 13's NORMALIZATION-ADVERSARIALITY clause says "false-boundary mutants for tokenizers", and pass-18 delivered seven of them faithfully. F-S2104-P19-004 is the missed-boundary direction, which the clause does not name and which nobody added. Candidate codification: amend the clause to read "false-boundary **and** missed-boundary mutants", and require the extraction-mechanisms table to record both directions per tokenizer with the fail-open consequence of each stated explicitly — merging is fail-closed for co-occurrence negatives and fail-open for exclusion-based gates, and a table that records only "prevents false boundary on `No. 523`" cannot surface that asymmetry.

**[process-gap] — cross-artifact hash confusion is not detected by any distinctness check.** F-S2104-P19-009's wrong value is distinct, well-formed, concrete, and traceable — to the wrong document. The pass-18 review checked that the six E-21 hashes were distinct and they were; distinctness is invariant under substituting one real hash for another. The check that catches it is three-way equality per story (frontmatter = catalog row = blockquote), which four of six stories satisfy and which no burst executes. Candidate codification: the closure burst's literal-shell gate set MUST include per-story three-way input-hash equality with captured stdout, and MUST NOT accept distinctness as a substitute.

---

## Per-Pass-18 Verification

| Finding | Status | Notes |
|---------|--------|-------|
| F-S2104-P18-001 | PARTIAL (the gate landed and is load-bearing; the escape re-seeds it in one hop) | The write-directive gate exists at `write_directive_violations` (bats:966-976), is genuinely open-triggered on the directive axis, and I re-proved verbatim M-P18-A RED at bats:975. The escape clause is non-vacuous: substituting `MUST use canonical absolute` returns GREEN. But the escape is evaluated over the **sentence**, so M-P19-B places the escape phrase and a harmful mandate in one sentence at 9/9, and the same sentence-scoped exclusion in PW-B lets M-P19-A restore verbatim M-P17-A with one appended `forbidden` clause at 9/9 (CONTROL-A RED) → F-S2104-P19-001. The action-word conjunct is a closed three-verb list, so M-P19-C escapes with `saved` → F-S2104-P19-003. The ALTERNATION-DIRECTION entry's claim that no unlisted paraphrase can evade the gate is refuted by both. Leg (c) of the predicate — the escape-clause GREEN control — was not recorded → F-S2104-P19-012 |
| F-S2104-P18-002 | CONFIRMED-CLOSED (both legs), with a stale rationale | (a) The balanced-fence assertion exists (bats:664-669) and M-P18-C(b) is RED with the recorded odd-count-3 message. (b) The fence exclusion is genuinely gone — `write_discipline_prose` is now `tr '\n' ' '` with no `awk`, and I re-proved M-P18-C RED via PW-B once placed correctly inside the fence. The adversary-requested verification that all four section-wide gates are empty over the unexcluded pristine section holds (9/9 GREEN). One residue: because no domain is fence-aware any more, the truncation rationale asserted at three sites is false at HEAD → F-S2104-P19-011. The gate is correct; only its stated justification is stale |
| F-S2104-P18-003 | PARTIAL (false-boundary direction closed; missed-boundary direction opened) | The boundary-rule splitter replaced `sed 's/\. /\n/g'` at all eight sites, and M-P18-B plus all six recorded abbreviation variants are RED. The `No. 523` class is genuinely closed. But splitting only before `[A-Z*\`\[]` means a lowercase-, digit- or quote-initial sentence is **merged** into its predecessor, and merging is fail-open for the two exclusion-based gates: M-P19-D survives at 9/9 while CONTROL-D, differing only in capitalising the initial letter, is RED → F-S2104-P19-004. The extraction-mechanisms row records the fix's direction and the NORMALIZATION-ADVERSARIALITY mandate in the false-boundary direction only |
| F-S2104-P18-004 | PARTIAL (the rendered domain is real; its strip predicate re-seeds the class) | (a)/(b) `rendered_write_discipline` exists (bats:682-685), anchor-uniqueness and the empty-block guard read it, and M-P18-D is RED at bats:695 with count 0. (c) The `<!--` absence gate is retained as defence-in-depth and M-P17-H is RED. (d) The GREEN control holds — the pristine section survives stripping and Gate 1(a) still passes. But the link-ref-def drop predicate requires whitespace after the colon, which CommonMark makes optional: `[//]:# (…)` is a conformant definition that both `marked` and `pandoc -f commonmark` render as nothing, and M-P19-F hides the whole mandate in it at 9/9 while CONTROL-F, differing by one space, is RED → F-S2104-P19-005. The second option of F-S2104-P17-001(b) was taken correctly; the mechanism implementing it carries the residue |
| F-S2104-P18-005 | PARTIAL (all four structural legs land; the classes remain open) | (a) Gate 2b(c)'s domain is now `write_discipline_prose_nosplit`, matching 2b(a), and M-P18-G is RED at bats:897 — the paired-domain divergence is genuinely fixed. (b) The adversative class is 9 members and (c) the nullification class is 25; M-P18-E and M-P18-F are both RED. (d) The write-directive trigger was widened with `permits\|is acceptable\|is the required form\|is preferred\|may`. But the class-closure is unchanged in kind: M-P19-G (`The prohibition stated above is not applicable to ledger writes made from the delivery sandbox.`) matches no 2b(a) member, carries no adversative for 2b(c), and carries no directive token for the declared backstop, at 9/9 → F-S2104-P19-007. The Gate 2b(a) direction statement's claim that the write-directive gate closes this axis is refuted by that vector |
| F-S2104-P18-006 | CONFIRMED-CLOSED | Both attested sites are corrected. The battery M-P17-A row now reads `Gate PW-B (primary …) + write-directive gate (secondary …). Gate 5 does NOT independently fire — M-P17-A contains no \`CWD-relative\|worktree-relative\|relative[[:space:]]+path\` member.` The Gate 5 audit row drops M-P17-A and cites only vectors whose text contains an alternation member (M-P15-A S1, M-P14-A, M-P14R-A, the `worktree-relative` synonym, M-P16-C2). The isolating Gate-5 sibling-paragraph vector was added with captured stdout, and its sequence-shadowing by PW-B is disclosed rather than papered over — the disclosure is accurate: I confirmed PW-B fires first at bats:820 on that vector. The correction is recorded as a new v1.16 changelog row with error acknowledgment and the v1.15 entries preserved. NAME-SET EQUALITY was re-run after the edit as leg (d) required, and I reproduced it |
| F-S2104-P18-007 | CONFIRMED-CLOSED | Both bracketed placeholders are gone. `last_amended` now reads `input-hash 4b26b3b→f86871a (story v1.23 drift)` with the D-918 clause corrected to the literal `[see frontmatter]→4b26b3b` / `[updated by compute-input-hash]→4b26b3b`, and every sibling row records literal old→new values. `grep -c 'updated by compute-input-hash\|\[see frontmatter\]'` over the red-gate-log returns 0 for live placeholder form. The append-only chain's old-matches-prior-new property is restored within the red-gate-log. Note that the *value* `f86871a` is correct for the red-gate-log and incorrect where it was subsequently propagated — that is F-S2104-P19-009, a distinct defect in a different artifact, not a regression of this finding |

---

Tally: **1 CONFIRMED-CLOSED-with-residue / 3 CONFIRMED-CLOSED / 4 PARTIAL / 0 REGRESSED** against the pass-18 finding set — precisely, F-S2104-P18-002/006/007 CONFIRMED-CLOSED (P18-002 with a stale-rationale residue raised separately) and F-S2104-P18-001/003/004/005 PARTIAL. No regression: every pass-18 leg that landed is load-bearing, all twelve recorded vectors I re-ran are RED, and the two record findings closed with proper error acknowledgment.

The four PARTIALs share one shape, and it is a different shape from pass-18's. Pass-18's PARTIALs were "the fix closed the vector and the axis, but not the class, because the class boundary was a list." Pass-19's are "the fix closed the class on the side it examined, and the mechanism it built to do so is unexamined on its other side" — the escape rather than the trigger, the missed boundary rather than the false one, the strip predicate's own regex rather than the construct it strips, the excluded region rather than the included one. Every fix predicate above is mechanically checkable; four of them (F-S2104-P19-001(a), F-S2104-P19-002(a) and its PW-B counter-verification, F-S2104-P19-004(a)) are already verified empty-on-pristine and firing-on-mutant by literal shell in this report; and none requires a new spec decision. Two require a paired doc change in `_shared-context.md` and I have flagged exactly which sentences and why.

---

fixes_landed_head: 657fce61

## Fix Mapping — Pass-19 (F-S2104-P19-001..012)

| Finding | Fix | Primary artifacts | Status |
|---------|-----|-------------------|--------|
| F-S2104-P19-001 | write-directive gate clause-scoped split (ESCAPE-SCOPE-PARITY): after boundary-rule sentence split, further split on `[;—]` and `,\s+(and\|or\|but)\s+`; escape unit must match trigger unit (clause, not sentence) | bats T-001 (a2112e8d), story v1.25 AC-001 Gate cell | CLOSED |
| F-S2104-P19-002 | write-directive gate domain widened from `write_discipline_prose_nosplit` to `spec_path_prose_nosplit` (whole `### Spec-Path Discipline` section) | bats T-001 (657fce61), story v1.25 AC-001 Gate cell | CLOSED |
| F-S2104-P19-003 | referent predicate `\.factory/\|ledger` replaces closed action-word list (`anchor\|write\|writes`) | bats T-001 (657fce61), story v1.25 AC-001 Gate cell | CLOSED |
| F-S2104-P19-004 | boundary-completeness assertion added: `bc_expected_splits` (`grep -oE '\.[[:space:]]+[A-Z*\`\[' write_discipline_prose_nosplit \| wc -l`) must equal `bc_actual_splits - 1`; pristine verified bc_expected_splits=13 = bc_actual_splits-1=13 | bats T-001 (657fce61), story v1.25 AC-001 Gate cell | CLOSED |
| F-S2104-P19-005 | link-ref-def strip predicate: whitespace-after-colon requirement removed — CommonMark `[label]:` form without trailing space is now correctly stripped | bats T-001 (657fce61), story v1.25 AC-001 Gate cell | CLOSED |
| F-S2104-P19-006 | canonical-target gate domain widened from `file_path=` keyword to any `**Correct:**` bullet containing `\.factory/` | bats T-001 (657fce61), story v1.25 AC-001 Gate cell | CLOSED |
| F-S2104-P19-007 | Gate scope-restriction NEGATIVE added; Gate 2b(a) widened +5 members (`not applicable\|inapplicable\|does not cover\|does not extend\|out of scope`); scope-restriction made primary defense, Gate 2b(a) defense-in-depth | bats T-001 (657fce61), story v1.25 AC-001 Gate cell | CLOSED |
| F-S2104-P19-008 | bats lead-in count-word Nineteen→Twenty-one (2 sites); story AC-001 Gate cell coupling note updated; STORY-INDEX catalog row story v1.23→v1.25 | bats (a2112e8d), story v1.25, STORY-INDEX v4.267 | CLOSED |
| F-S2104-P19-009 | STORY-INDEX S-21.04 catalog row input-hash f86871a→1165b1f; S-21.01 catalog row input-hash fde01eb→32aaccc; blockquote S-21.04=f86871a→1165b1f; three-way equality gate (frontmatter=catalog=blockquote) literal-shell verified PASS | STORY-INDEX v4.267 (factory-artifacts burst) | CLOSED |
| F-S2104-P19-010 | red-gate-log NAME-SET EQUALITY partition paragraph: TWO→THREE (count was wrong vs. listed 3 items) | red-gate-log v1.17 (factory-artifacts burst) | CLOSED |
| F-S2104-P19-011 | balanced-fence gate rationale corrected at all 3 sites: NOT a truncation guard; well-formedness invariant (no fence-aware domain at HEAD per c89bef22); original truncation mechanism described historically | bats T-001 comment (657fce61), story v1.25 AC-001 Gate cell, red-gate-log v1.17 audit row | CLOSED |
| F-S2104-P19-012 | escape-discrimination controls transcribed to red-gate-log: CONTROL write-directive escape (GREEN; clause-scoped escape in trigger clause) + CONTROL negative-twin (RED; escape in sibling clause only) with verbatim stdout | red-gate-log v1.17 Pass-19 attestation (factory-artifacts burst) | CLOSED |
