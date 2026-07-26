---
pass: 15
verdict: NOT-CLEAN
reviewed_head: 26b85d8c
fixes_landed_head: 8b39277b
novelty: 0.57
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-14.md"
---

## Summary

Pass-15 fresh-context adversarial review of S-21.04 at reviewed_head `26b85d8c` (worktree `.worktrees/S-21.04`, base develop `948f0fb1`). **7 findings: B1 / H2 / M2 / L2.** Novelty 0.57 vs pass-14R Part A (4 of 7 novel in class: the epic version-pin leg, the AC-010 gate/AC divergence, the CHANGELOG count lead-in, and the STORY-INDEX input-hash snapshot; F-S2104-P15-002 is a partial-novel re-seed into a gate that did not exist at pass-14; F-S2104-P15-001 and -004 are one-hop re-seedings of P14R-001 and P14R-005). Trajectory 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7. Streak: **0/3** (BC-5.39.001 reset).

Baseline established by literal execution at HEAD: `bats story-worktree-write-path-discipline.bats` → `1..9`, 9/9 ok; `bats worktree-identity-preflight.bats` → `1..14`, 14/14 ok.

The pass-14R fix wave did honest work — I independently re-proved both recorded vectors RED at HEAD (M-P14R-A fires Gate 1's inversion guard at the `polarity-inversion` message; the `worktree-relative` synonym vector fires Gate 5's POLICY-13 alternation). The gates are load-bearing against every mutant the wave recorded. **They are nevertheless defeated by a fifth-generation vector**, because Gates 1, 4 and 5 are `grep` predicates evaluated **per physical line** over a soft-wrapped markdown paragraph, and Gate 1's affirmative half is a block-level presence check that any decoy sentence satisfies. Moving `MUST` and the prohibited-subject token onto different physical lines and adding one plausible decoy sentence inverts the BC-6.26.001 PC1 mandate end-to-end with the suite at 9/9. Markdown line-wrap position is not semantic content; a per-line predicate over a wrapped paragraph cannot be a polarity gate. That is the root cause the last four generations have each re-instantiated at a finer grain.

Second structural theme: the newly added Gate 6 (traversal form, F-S2104-P14R-003) is presence-only. It asserts the string `../` appears somewhere in §Spec-Path Discipline; it does not assert the traversal form is *forbidden*. Relabelling the bullet `**Forbidden:**` → `**Correct:**` leaves T-001 GREEN — the P14R-001 defect class re-seeded, one pass later, into the gate built to close a sibling clause of the same AC.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P15-001 | BLOCKER | `story-worktree-write-path-discipline.bats` §T-001 prohibition-block Gates 1/4/5 vs `_shared-context.md` §Spec-Path Discipline → §Write Discipline prohibition paragraph | Fifth-generation polarity paper-gate: Gates 1/4/5 are per-**physical-line** greps over a soft-wrapped paragraph, and Gate 1's positive half is a block-level presence check. M-P15-A (line-rewrapped inversion + one decoy affirmative sentence) mandates CWD-relative writes and forbids canonical absolute, and the full suite reports 9/9 ok | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13, 15; TD-VSDD-059 |
| F-S2104-P15-002 | HIGH | `bats` §T-001 Gate 6 (`_assert_doc_marker '\.\./\|relative[[:space:]]+traversal'`) vs story AC-001(a)(ii) | Gate 6 is presence-only, polarity-blind: relabelling the traversal bullet `**Forbidden:**` → `**Correct:**` leaves T-001 GREEN. AC-001(a) requires the clause **forbid** the traversal form; only its existence is gated. One-hop re-seed of the F-S2104-P14R-001 class into the gate added to close F-S2104-P14R-003 | BC-6.26.001 PC1; POLICY 11, 13 |
| F-S2104-P15-003 | HIGH | Epic `E-21-factory-state-data-loss-hardening.md` §BC Traceability table + body prose | Epic v1.7 pins BC-6.26.001 at `v1.3` (actual v1.11 — eight versions stale) and carries eleven live-body `ADR-031 v1.3` load-bearing version tokens (ADR-031 is v1.13). POLICY 19 is HIGH-class; the F-P11-007 "class-death" closure for ADR-031 version cites is falsified in an in-perimeter document | POLICY 19, 14/17 (leg 5), 5; TD-VSDD-059, TD-VSDD-060 |
| F-S2104-P15-004 | MEDIUM | `bats` §Write-Discipline-extractor comment + §T-001 Gate 1/Gate 2 comments; `worktree-identity-preflight.bats` §test (e) comment | Six bare line pins survive in the two perimeter bats files, one of them (`Restore (c): line ~:66`) in exactly the `line ~NNN` form F-S2104-P14R-005's predicate named. The P14R Fix Mapping claim "zero bare pins verified" is false. The sibling-file comment also narrates a completed sweep in future tense | TD-VSDD-091; POLICY 5, 15 |
| F-S2104-P15-005 | MEDIUM | story AC-010 criterion text vs `bats` §T-009 stale-`bcs:` negative gate | AC-010 carves out "outside code blocks or historical references"; the gate matches `bcs:` unconditionally and the bats comment states the character class was widened *specifically to match inside backtick code spans*. The gate enforces a strictly stronger contract than the AC it claims to implement | POLICY 8, 11 |
| F-S2104-P15-006 | LOW | `CHANGELOG.md` §Unreleased → §Added → S-21.04 entry | Lead-in "Two complementary protocol requirements delivered as skill-doc mandates … :" introduces a single numbered list running `(1)`–`(5)`. Count-bearing lead-in contradicts its own enumeration (D-902 count-bearing-crossref-residue class) | POLICY 4; D-902 class |
| F-S2104-P15-007 | LOW | `STORY-INDEX.md` §E-21 delivery blockquote | Blockquote asserts `S-21.04=df0d623` and "All 6 distinct" as present-tense fact; the story frontmatter and the S-21.04 catalog row both carry `1165b1f`. Five of six values are stale and the blockquote carries no historical marker (contrast the sibling blockquote which carries `[Historical v1.0 tally`) | POLICY 5 (HEAD-reproducibility), 18, 3 |

---

### F-S2104-P15-001 — BLOCKER — polarity gates are per-physical-line predicates over a soft-wrapped paragraph

**Stable anchors.** Gate site: `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`, T-001 (`@test "T-001 S-21.04 AC-003: stray-file-blocks …"`), the block introduced by the comment `# Gate 1: affirmative mandate polarity (F-S2104-P14R-001(a))` through `# Gate 5 (NEGATIVE, F-S2104-P14R-001(b))`. Target: `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`, §Spec-Path Discipline → `#### Write Discipline — .factory/** artifact writes from story worktrees`, the normative prohibition paragraph beginning `All \`.factory/**\` artifact writes performed during story delivery` (lines 66-70 at HEAD).

The three polarity gates read, verbatim (`bats:540`, `:546`, `:568`, `:580`):

```
  printf '%s\n' "$prohibition_block" | grep -qE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' || {
  if printf '%s\n' "$prohibition_block" | grep -E 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' | grep -qE 'not[[:space:]]+canonical[[:space:]]+absolute|not[[:space:]]+absolute'; then
  if printf '%s\n' "$prohibition_block" | grep -qE 'absolute.*(FORBIDDEN|forbidden)|FORBIDDEN.*absolute|forbidden.*absolute'; then
  if printf '%s\n' "$prohibition_block" | grep -qE 'MUST.*(CWD-relative|worktree-relative|relative[[:space:]]+path)|(CWD-relative|worktree-relative|relative[[:space:]]+path).*MUST'; then
```

Every one is line-scoped (`grep` without `tr '\n' ' '`). Gate 2 is the only joined gate, and the file's own comment at `bats:554` concedes why the authors chose joined form there — and, in doing so, documents the dependency this finding exploits:

```
  # Per-line form was not used: spec text :66-70 has CWD-relative on line 67 and FORBIDDEN on line 68
  # (adjacent lines, not same line) — per-line check fails on correct text (see comment block above).
```

Because paragraph line breaks in markdown are non-semantic (the rendered sentence is identical regardless of where the source wraps), a mutant needs only to (i) place `MUST` and the prohibited-subject token on **different** physical lines, and (ii) supply one line anywhere in the block that satisfies Gate 1's *positive* presence check without containing `not canonical absolute`. A sentence permitting canonical absolute paths for *reads* is a natural decoy — §Spec-Path Discipline is already a read-discipline section.

**M-P15-A — exact substituted text** (replaces the §Write Discipline normative prohibition paragraph, HEAD lines 66-70):

```
All `.factory/**` artifact writes performed during story delivery MUST use
CWD-relative paths anchored to the story-worktree CWD.
Writers MUST use canonical absolute paths only when reading spec ground-truth from the main checkout.
Canonical absolute artifact-write paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`)
are FORBIDDEN — relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.
```

Rendered, this instructs every story agent to do exactly what issue #523 did.

**Per-gate literal shell, captured stdout** (`B` holds M-P15-A verbatim):

```
$ [ -z "$B" ] && echo "FIRES (RED)" || echo "passes"
passes
$ printf '%s\n' "$B" | grep -qE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' && echo "passes (mutant survives)" || echo "FIRES (RED)"
passes (mutant survives)
$ printf '%s\n' "$B" | grep -E 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute' | grep -qE 'not[[:space:]]+canonical[[:space:]]+absolute|not[[:space:]]+absolute' && echo "FIRES (RED)" || echo "passes (mutant survives)"
passes (mutant survives)
$ printf '%s\n' "$B" | tr '\n' ' ' | grep -qE '(CWD-relative|relative path).*FORBIDDEN|FORBIDDEN.*(CWD-relative|relative path)' && echo "passes (mutant survives)" || echo "FIRES (RED)"
passes (mutant survives)
$ printf '%s\n' "$B" | grep -qE 'absolute.*(FORBIDDEN|forbidden)|FORBIDDEN.*absolute|forbidden.*absolute' && echo "FIRES (RED)" || echo "passes (mutant survives)"
passes (mutant survives)
$ printf '%s\n' "$B" | grep -qE 'MUST.*(CWD-relative|worktree-relative|relative[[:space:]]+path)|(CWD-relative|worktree-relative|relative[[:space:]]+path).*MUST' && echo "FIRES (RED)" || echo "passes (mutant survives)"
passes (mutant survives)
$ printf '%s\n' "$B" | grep -nE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute'
3:Writers MUST use canonical absolute paths only when reading spec ground-truth from the main checkout.
```

Gate 1's positive half is satisfied by line 3 — the decoy — never by the normative mandate sentence.

**End-to-end proof** (scratch copy of the full `plugins/` tree, prohibition paragraph replaced with M-P15-A, unmodified bats suite):

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
```

Severity is BLOCKER on fifth-generation recurrence of the story's primary BC postcondition remaining ungated (F-P12-003 → F-P13-001 → F-P14-001 → F-S2104-P14R-001 → here).

**Zero-degrees-of-freedom fix predicate.** (a) Gates 1, 4 and 5 MUST evaluate the prohibition block **reflowed**, not per physical line — either joined (`| tr '\n' ' '`) or split into sentences on `.[[:space:]]`; a per-physical-line predicate over a wrapped markdown paragraph is not admissible as a polarity gate and must not be reintroduced. (b) Gate 1's affirmative half MUST be scoped to the normative mandate sentence — the sentence containing `artifact writes` — and not satisfiable by any other sentence in the block: extract that sentence and require it to match `MUST[^.]*use[^.]*canonical[[:space:]]+absolute` **and** to NOT match the POLICY-13 prohibited-subject alternation `CWD-relative|worktree-relative|relative[[:space:]]+paths?`. (c) The fix MUST be shown RED against M-P15-A verbatim **and** GREEN against the unmodified paragraph, with captured stdout, **and** must retain RED for M-P14-A, M-P14R-A, and the `worktree-relative` vector already recorded in red-gate-log §Pass-14R (all three re-verified RED at HEAD by this review — do not regress them). (d) A fix that only appends a token to Gate 5's alternation is insufficient: M-P15-A contains no `MUST`-plus-relative-form on any single line, so no alternation extension reaches it.

### F-S2104-P15-002 — HIGH — Gate 6 gates the traversal example's existence, not its prohibition

**Stable anchors.** Gate site: `bats` T-001, the assertion introduced by `# Gate 6 (F-S2104-P14R-003): traversal-form example`. Target: `_shared-context.md` §Spec-Path Discipline, the third `**Forbidden:**` example bullet. Contract: story AC-001(a)(ii).

Story AC-001 states the obligation, verbatim:

> `(a) forbid BOTH forbidden forms named in BC-6.26.001 PC1: (i) CWD-relative paths (".factory/..." from story-worktree CWD — silently writes to shadow tree) and (ii) relative-traversal paths ("../../.factory/..." — brittle traversal form);`

The gate, verbatim (`bats:596`):

```
  _assert_doc_marker '\.\./|relative[[:space:]]+traversal' \
```

This is a presence assertion over `$spec_path_section`. It cannot distinguish a forbidden traversal example from a mandated one. `\.\./` occurs exactly once in the file, so the gate *is* load-bearing against deletion (as red-gate-log §Pass-14R attests) — but not against inversion:

```
$ grep -n '\.\./' plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md
114:- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative traversal — brittle and error-prone)
```

**M-P15-B — exact substituted text** (single-line substitution in §Spec-Path Discipline):

```
- **Correct:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative traversal — resolves to the canonical mount from a story worktree)
```

**Captured stdout** (scratch copy of the full `plugins/` tree, unmodified bats suite):

```
$ sed -n '112,114p' .../_shared-context.md
- **Correct:** `Write(file_path="$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md", ...)`
- **Forbidden:** `Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)` (relative path — silently writes to shadow tree)
- **Correct:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative traversal — resolves to the canonical mount from a story worktree)
$ bats -f "T-001" story-worktree-write-path-discipline.bats
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

Gate 3 does not compensate: it requires `**Forbidden:**` to co-occur with `relative path` on one line, which the untouched CWD-relative bullet supplies. So the delivered skill-doc could present relative traversal as the *correct* form for `.factory/**` writes with the S-21.04 gate suite at 9/9.

**Zero-degrees-of-freedom fix predicate.** Gate 6 must become a two-part polarity gate on `$spec_path_section`: (a) positive — some line MUST match `\*\*Forbidden:\*\*` **and** contain `\.\./` on that same line; (b) negative — NO line containing `\.\./` may match `\*\*Correct:\*\*`. Both halves must carry recorded mutants with exact substituted text: the deletion mutant already in red-gate-log §Pass-14R for (a), and M-P15-B above for (b), each with captured RED stdout and a GREEN restore.

### F-S2104-P15-003 — HIGH — epic v1.7 carries an eight-version-stale BC pin and eleven POLICY-19 ADR version tokens

**Stable anchor.** `.factory/stories/epics/E-21-factory-state-data-loss-hardening.md`, §BC Traceability table row `BC-6.26.001`, and the §PRD Capabilities / §Description / §INV-E21 body prose.

The BC Traceability row reads, verbatim:

> `| BC-6.26.001 | v1.3 | Story-worktree write-path discipline (canonical absolute path via \`CANONICAL_FACTORY_ROOT\`) + teardown preflight (\`find <worktree>/.factory -type f\` before \`git worktree remove\`) | CAP-036 | S-21.04 (skill-doc: \`_shared-context.md\` + \`step-g-cleanup.md\`) |`

BC-6.26.001 is `version: "1.11"`. The description text in that row also still carries the retired non-trailing-slash `find <worktree>/.factory -type f` form, which ADR-031's own changelog records as swept to zero live occurrences at v1.11 ("TD-VSDD-060 class sweep: … non-trailing-slash `.factory" -type f` forms … zero live occurrences post-fix"). The epic body repeats the stale pin: `preflight): BC-6.26.001 v1.3. Implemented by S-21.04 (skill-doc amendment to`.

These pins ARE maintained by convention — the epic's own changelog records `v1.3 … CAP-036 BC-6.26.001 v1.2 → v1.3` and `v1.5 … BC-6.27.001 v1.2→v1.3 in PRD Capabilities + BC Traceability table`. They simply were never advanced across BC v1.4→v1.11 while STORY-INDEX's catalog row was kept current at `[BC-6.26.001 v1.11]`. Sibling sweep — all six E-21 BC pins are stale:

```
BC-4.16.001: epic v1.2 / actual 1.8
BC-5.43.001: epic v1.3 / actual 1.4
BC-5.44.001: epic v1.3 / actual 1.5
BC-6.10.002: epic v1.3 / actual 1.5
BC-6.26.001: epic v1.3 / actual 1.11
BC-6.27.001: epic v1.3 / actual 1.4
```

Separately, POLICY 19 (`adr_version_cite_prohibition`, HIGH) forbids load-bearing ADR version tokens. Eleven live-body occurrences remain, ADR-031 being v1.13:

```
$ grep -nE 'ADR-031 v[0-9]' epics/E-21-factory-state-data-loss-hardening.md
68:   CAP-038; ADR-031 v1.3 §Decision 8).
85:INV-E21-001..006 (cross-cutting invariants catalogued in ADR-031 v1.3 §Decision 1) govern
106:v1.1, ADR-031 v1.3, and all six BCs (BC-4.16.001/BC-5.43.001/BC-6.27.001 at v1.2;
125:E-21 introduces the following PRD capabilities, defined in ADR-031 §Decision 7 (CAP-034..037) and ADR-031 v1.3 §Decision 7 (CAP-038):
135:  §Inter-Wave Rebase skill-doc amendment; ADR-031 v1.3 §Consequences #5 confirms this
147:  `--is-ancestor` check): BC-6.10.002 v1.3 (ADR-031 v1.3 §Decision 7 + §Decision 8).
258:  (v1.2 or v1.3 per ADR-031 v1.3) and no spec amendments are required.
267:| BC-5.43.001 | v1.3 | …
268:| BC-5.44.001 | v1.3 | …
273:**INV-E21-001..006 cross-cutting invariants** (ADR-031 v1.3 §Decision 1):
289:  ADR-031 v1.3 §Decision 8 + CAP-038).
```

Line 125 carries both forms in one sentence — the stable `ADR-031 §Decision 7` and the versioned `ADR-031 v1.3 §Decision 7` — direct evidence that a partial sweep ran and stopped. STORY-INDEX v4.259 records this class as closed: `S-21.02 catalog-row ADR-031 v1.3 §Consequences #5 → ADR-031 §Consequences #5 (F-P11-007 class-death)`. The epic, in the same review perimeter, retains eleven instances, so that class-death claim is falsified (TD-VSDD-059). By contrast the story and BC-6.26.001 are clean — their only `ADR-031 v1.N` tokens are in `last_amended`, `modified[]`, and Changelog rows, which POLICY 5 exempts as historical-by-construction.

**Zero-degrees-of-freedom fix predicate.** (a) Epic §BC Traceability row `BC-6.26.001`: `v1.3`→`v1.11`, and the row's preflight description advanced to the trailing-slash form `find "<worktree-path>/.factory/" -type f`; (b) epic body `BC-6.26.001 v1.3` → `v1.11`; (c) sibling-sweep the remaining five BC pins to their actual versions in the same burst (POLICY 5 (a)-(j)); (d) strip the version token from every live-body `ADR-031 v1.3 §…` occurrence, leaving the stable `ADR-031 §Decision N` / `ADR-031 §Consequences #N` form, and leave changelog/`modified[]` rows untouched; (e) verify with captured stdout that `grep -nE 'ADR-031 v[0-9]'` returns only historical-by-construction sites, and bump epic version with all five POLICY 14/17 legs.

### F-S2104-P15-004 — MEDIUM — six bare line pins survive; the P14R-005 "zero bare pins" closure is false

**Stable anchors.** `story-worktree-write-path-discipline.bats`: the `_extract_write_discipline_prohibition_block` docblock; the T-001 `# --- DOC-PARITY §Spec-Path Discipline: AC-001(a) CWD-relative-path PROHIBITION` comment block; the `# Gate 2:` comment. `worktree-identity-preflight.bats`: the `# (e) AC-005:` docblock.

```
$ grep -nE '~:[0-9]+|line ~[0-9]+|:[0-9]{2,4}\b' story-worktree-write-path-discipline.bats
128:#   ~:66 of _shared-context.md). End: first blank line (paragraph boundary before **Load-bearing
498:  #   (1) Paragraph-level extractor (_extract_write_discipline_prohibition_block, ~:66-70) +
504:  #       Restore (c): line ~:66 "MUST use canonical absolute paths" → gate GREEN.
554:  # Per-line form was not used: spec text :66-70 has CWD-relative on line 67 and FORBIDDEN on line 68
$ grep -nE 'lines? [0-9]+(/[0-9]+)?' worktree-identity-preflight.bats
105:#     a retracted premise and would block the implementer's residue sweep at lines 44/59.
109:#     Both pass NOW and keep passing after implementer sweeps stale residue at lines 44/59.
```

F-S2104-P14R-005's fix predicate was explicit: "replace every `line ~NNN` / `(~:NNN)` with a stable anchor (heading, function name, or verbatim token)". Line 504 is verbatim `line ~:66`. Its Fix Mapping row nevertheless claims `FIXED 26b85d8c — test-writer — (~:113) → stable anchor; zero bare pins verified`. The emitted-label pin was indeed fixed; the four comment pins in the same file were not, so the "zero bare pins verified" attestation is false (TD-VSDD-059).

The two sibling-file pins carry a second defect: the comment says "keep passing **after** implementer sweeps stale residue at lines 44/59" — future tense for work already complete, and the cited positions no longer hold stale residue:

```
$ grep -niE 'stale.*(worktree|snapshot)|snapshot.*stale' plugins/vsdd-factory/agents/adversary.md
NO MATCH
```

**Zero-degrees-of-freedom fix predicate.** Replace all six pins with stable anchors — `_shared-context.md` §Spec-Path Discipline → §Write Discipline normative prohibition paragraph (first line: `All \`.factory/**\` artifact writes…`) for the four in `story-worktree-write-path-discipline.bats`; `adversary.md` §Worktree-Identity Preflight opening paragraph and rule 6 SPEC/ADR/BC/VP bullet for the two in `worktree-identity-preflight.bats` — and rewrite the `(e)` docblock's future tense to past ("the implementer's residue sweep landed at 4265c96c; both assertions hold at HEAD"). Verify with captured stdout that `grep -nE '~:[0-9]+|line ~[0-9]+|lines? [0-9]+/[0-9]+'` returns no matches in either file.

### F-S2104-P15-005 — MEDIUM — the AC-010 code-block carve-out is not implemented by its gate

**Stable anchors.** Story §Acceptance Criteria row AC-010; `bats` T-009, the assertion block introduced by `# --- F-S2104-P14R-008: behavioral_contracts field-name correctness in adversary.md ---`.

AC-010 criterion text, verbatim:

> `plugins/vsdd-factory/agents/adversary.md` MUST reference the canonical `behavioral_contracts:` frontmatter field name and MUST NOT carry any standalone stale `bcs:` token **outside code blocks or historical references**.

The negative gate, verbatim (`bats:1347`):

```
  if grep -qE '(^|[^a-zA-Z0-9_])bcs:' "$ADVERSARY_MD"; then
```

There is no code-block or historical-reference exclusion, and the file's own comment (`bats:1344-1346`) plus the red-gate-log §Pass-14R "Negative-gate pattern note" state that the wider character class was chosen **in order to** match inside code spans: "`(^|[^a-zA-Z0-9_])bcs:` used (not `(^|[[:space:]])bcs:`) because `bcs:` appears inside backtick code spans; the broader character-class exclusion catches the backtick-preceded form". The gate therefore enforces a strictly stronger contract than the AC it is registered against. Any future legitimate code-block example documenting the retired field name — which AC-010 explicitly permits — turns T-009 RED. The condition is currently latent (`grep -nE 'bcs' adversary.md` → `NO MATCH`), which is precisely why it will not surface until someone hits it.

Positive-side counts check out: `grep -c 'behavioral_contracts:' adversary.md` → `5`, at the Perimeter-1 §Scope sentence and the four §Story Frontmatter-Body Coherence bidirectional-BC-completeness items, matching AC-010's "5 sites total".

**Zero-degrees-of-freedom fix predicate.** Choose one and make both sides agree, in the same burst: either (a) drop the carve-out from AC-010 so the criterion reads "MUST NOT carry any standalone stale `bcs:` token" with a stated rationale that code-span examples of the retired field name are also prohibited in this file — and record that widening in the story `modified[]`/`last_amended`; or (b) implement the carve-out in the gate by stripping fenced blocks and backtick spans before the negative grep, and record a mutant proving a code-span `bcs:` stays GREEN while a prose `bcs:` goes RED. Option (a) is the smaller, safer change and matches the gate's existing recorded mutant.

### F-S2104-P15-006 — LOW — CHANGELOG lead-in count contradicts its own enumeration

**Stable anchor.** `CHANGELOG.md` §Unreleased → §Added → the `**S-21.04 — story-worktree write-path discipline + teardown preflight**` entry.

Verbatim:

> `Two complementary protocol requirements delivered as skill-doc mandates (no new WASM or shell script, per POLICY 21):`
> `(1) **Write-path discipline** …`
> `(2) **Teardown preflight** …`
> `(3) **Adversary reporting-semantics** …`
> `(4) **Test-(e) re-anchor** …`
> `(5) **Adversary agent** …`

The colon introduces one numbered sequence of five items under a lead-in asserting two. Items (1)–(2) are the two BC-6.26.001 postconditions; (3)–(5) are ancillary deliverables sharing the same numbering, so a reader counts five against a stated two. This is the count-bearing-crossref-residue class the project has been eliminating at definition sites (D-902; BC-6.26.001 v1.9 replaced "Three cases:" for exactly this reason), and it was introduced by this story's own CHANGELOG entry — item (5) landed in the pass-14R wave.

**Zero-degrees-of-freedom fix predicate.** Replace the lead-in with a count-free form and separate the scopes, e.g. `Delivered as skill-doc mandates (no new WASM or shell script, per POLICY 21) — the two BC-6.26.001 protocol requirements plus the propagation and awareness legs:`. Do not renumber items (1)–(5) (POLICY 1 append-only numbering is not implicated, but stable cross-references in the story §Tasks and red-gate-log are).

### F-S2104-P15-007 — LOW — STORY-INDEX §E-21 delivery blockquote asserts a stale input-hash set as present fact

**Stable anchor.** `.factory/stories/STORY-INDEX.md`, the `> **E-21 delivery:**` blockquote.

Verbatim: `Input-hashes: S-21.01=1fb8246; S-21.02=7768f31; S-21.03=1a639a0; S-21.04=df0d623; S-21.05=17729a2; S-21.06=b807086. All 6 distinct.`

Actuals from story frontmatter:

```
S-21.01: 32aaccc    S-21.02: 8bd32e5    S-21.03: 59e687e
S-21.04: 1165b1f    S-21.05: c9265f0    S-21.06: b807086
```

Five of six are stale; only S-21.06 matches. The S-21.04 catalog row in the same file is correct (`input-hash 1165b1f`), so STORY-INDEX states two different values for the same story. The blockquote's own parenthetical acknowledges an earlier refresh (`Input-hash refresh D-862: prior values … were pre-cascade`), which shows the author knew these values move, yet the enumeration is present-tense and carries no historical marker — unlike the sibling blockquote in the same section, which is explicitly annotated `[Historical v1.0 tally; current: 6 stories/35 pts/3 waves — see row summary]`. Under POLICY 5 a body blockquote is not a historical-by-construction site and must be HEAD-reproducible or marked. Severity is LOW because the condition is epic-wide and pre-dates this story's cascade; it is reported because the S-21.04 leg is a direct sibling site of the S-21.04 catalog row that F-S2104-P14R-007 advanced.

**Zero-degrees-of-freedom fix predicate.** Either refresh all six values to the current frontmatter hashes and re-verify distinctness with captured stdout, or append a historical marker in the sibling blockquote's exact form — `[Historical D-862 snapshot; current values live in each story's frontmatter and the per-story catalog rows]` — and delete the present-tense "All 6 distinct" claim. Refreshing is preferred: the distinctness property is load-bearing for compute-input-hash drift detection (POLICY 18) and is worth keeping verifiable.

---

## Observations (NOT findings)

**Behavioral axis is sound and the pass-14R attestation is honest.** I independently re-proved both recorded pass-14R vectors RED at HEAD on scratch copies of the full `plugins/` tree — M-P14R-A fires Gate 1's inversion guard (`DOC-PARITY FAIL [write-discipline prohibition block polarity-inversion]`, bats line 548) and the `worktree-relative` synonym vector fires Gate 5's POLICY-13 alternation (`DOC-PARITY FAIL [write-discipline prohibition block MUST-relative-polarity]`, bats line 582). Nothing in red-gate-log §Pass-14R is fabricated. The gates are genuinely load-bearing against every mutant recorded; F-S2104-P15-001 is a gap in the *shape* of the predicate, not a false attestation.

**POLICY 15 attestation-location gate satisfied.** I diffed `6f928350..26b85d8c` for assertion-predicate changes and cross-checked each against red-gate-log v1.12. Every changed predicate — Gate 1 positive rewrite, Gate 1 negative (new), Gate 5 alternation widening, Gate 6 (new), and both T-009 `behavioral_contracts:`/`bcs:` gates — has a matching attestation subsection with exact substituted text and captured RED/GREEN stdout. The attestation block is gone from the shipped bats file, whose last line is now `# (attestation content removed per F-S2104-P14R-004 — state-manager owns red-gate-log.md SoT)`.

**§G.1 PC2 prose structure now matches the BC.** `step-g-cleanup.md` §G.1 orders PC2a with sub-cases (a) and (b) contiguous, then the symlink→PC2b paragraph, then non-directory→PC2b, then PC2b, then PC2c — matching BC-6.26.001 §PC2's sequence (PC2a → non-directory-or-symlink → PC2b → PC2c). T-006's `[ -L ]`-precedes-`find` awk ordering gate stays GREEN because the `[ -L ]` shell expression appears in the discrimination chain while the only indented `find` command lives in the later §Preflight command block.

**Six-surface propagation and the executor mandate verified independently.** All six AC-007(d) surfaces carry the fully-qualified `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` path and none inlines a bare `find`. `devops-engineer.md` §Worktree Cleanup opens `After the story PR merges, and before executing \`git worktree remove\` on a story worktree, verify that the dispatching caller ran the … §G.1 preflight (PASS result).` — trigger sentence and all four T-007 obligation tokens intact. `rules/worktree-protocol.md` now states the git-clean-state blind spot affirmatively rather than the retired false-safety claim.

**Fixture-provenance and dead-code legs clean.** `ls fixtures/story-worktree/` → `README.md` only; the red-gate-log Fixture column reads `dynamic $(mktemp -d) fixture per bats setup() (fixtures/story-worktree/ holds README documentation only)`; story §Architecture Mapping and §File Structure Requirements both describe the directory as fixture *documentation*. `grep -n 'FIXTURE_DIR'` → no match.

**BC-6.26.001 §Changelog row ordering.** The table runs `1.11 … 1.4`, then `1.0`, then `1.3, 1.2, 1.1` — the `1.0` row is out of descending order. Row-for-row parity with `modified[]` is intact (every version is present), and no policy governs table row order, so this is cosmetic. Worth folding into the next BC touch rather than a dedicated burst.

**`devops-engineer.md` fenced block has no lead-in.** The `\`\`\`bash` fence follows the obligation paragraph with no blank line and no introducing sentence. CommonMark permits a fenced block to interrupt a paragraph so it renders correctly; F-S2104-P14R-012's substantive half (the missing trigger sentence) is closed. Cosmetic only.

**Line numbers inside captured bats stdout.** red-gate-log §Pass-14R quotes bats failure output containing `(in test file …, line 548)` / `line 582` / `line 596` / `line 1349`. All four resolve correctly at `26b85d8c`. POLICY 5's "line numbers forbidden in captured stdout" and POLICY 15's verbatim-stdout requirement are in tension here — bats emits line numbers and redacting them would falsify the verbatim record. Not flagged; worth an explicit carve-out in POLICY 5 for harness-emitted positions.

**[process-gap] — predicate SHAPE, not predicate TOKEN SET, is the recurring failure.** Passes 12–14R each closed the polarity class by widening the *token set* a gate matches (add `FORBIDDEN`-polarity gate → add `CWD-relative` gate → add a `CWD-relative|worktree-relative|relative path` alternation), and each time the next pass found a vector that never touches the token set. POLICY 13's new mutant-derived clause mandates alternation over the syntactic-form class of the mutated *token* — it says nothing about the *domain* the predicate is evaluated over. F-S2104-P15-001 walks around a fully POLICY-13-compliant alternation purely by relocating a line break. Candidate codification: **a semantic gate over prose MUST be evaluated on a normalized domain** — for a markdown paragraph, the reflowed (newline-joined) text or explicit sentence splits — and any per-physical-line predicate over a soft-wrapped paragraph must carry a recorded line-rewrap mutant proving the wrap position is not load-bearing. This is the one mechanism that would have caught passes 13, 14R and 15 in a single stroke.

**[process-gap] — new gates inherit the defect class of the gates they sit beside.** Gate 6 was authored in the same commit as the Gate 1/Gate 5 polarity hardening, to close a sibling clause of the same AC, and shipped as presence-only — the exact shape the neighbouring gates had just been condemned for. Candidate codification: when a fix wave hardens a gate from presence-only to polarity-asserting, every *other* gate covering a clause of the same AC must be audited to the same standard in that wave, with the audit recorded in the red-gate-log attestation section. F-S2104-P15-002 exists because Gate 6 was written to the pre-hardening standard beside its own remediation.

---

## Per-Pass-14R Verification

| Finding | Status | Notes |
|---------|--------|-------|
| F-S2104-P14R-001 | PARTIAL (5th-gen re-seed) | Gate 1's two-part affirmative check and Gate 5's POLICY-13 alternation both landed and are genuinely load-bearing — I re-proved M-P14R-A and the `worktree-relative` vector RED at HEAD on scratch trees. But all three polarity gates are per-**physical-line** and Gate 1's positive half is a block-level presence check: M-P15-A (line-rewrapped inversion + decoy affirmative sentence) inverts PC1 with the full suite at `1..9`, 9/9 ok → F-S2104-P15-001 |
| F-S2104-P14R-002 | CONFIRMED-CLOSED | red-gate-log `version: "1.12"`; Summary row `All GREEN at worktree HEAD 26b85d8c (test-writer-executed: 9/9 + 14/14, 2026-07-26)`; `### Pass-14R assertion-site attestation (26b85d8c)` present with exact substituted text and captured RED/GREEN stdout for every predicate changed at `26b85d8c` (verified by assertion-predicate diff `6f928350..26b85d8c`); `last_amended`/`modified[]` advanced under D-912 |
| F-S2104-P14R-003 | PARTIAL | Three legs landed: traversal bullet at `_shared-context.md` §Spec-Path Discipline (`- **Forbidden:** \`Write(file_path="../../.factory/…")\` (relative traversal — brittle and error-prone)`), AC-001(a) extended to both PC1 forms, Gate 6 added with a recorded deletion mutant. But Gate 6 asserts only that `../` exists, not that it is forbidden — the `**Forbidden:**`→`**Correct:**` substitution leaves T-001 `ok` → F-S2104-P15-002 |
| F-S2104-P14R-004 | CONFIRMED-CLOSED | The 108-line attestation block is gone from the shipped bats file; final line reads `# (attestation content removed per F-S2104-P14R-004 — state-manager owns red-gate-log.md SoT)`. Content transcribed into red-gate-log §Pass-14R under the correctly-allocated D-912 |
| F-S2104-P14R-005 | PARTIAL | The emitted-label pin is fixed — the Gate 3 label now reads `(§Spec-Path Discipline **Forbidden:** example line)` — and the nine LEG pins vanished with the attestation block. But four comment pins remain in the same file (`~:66`, `~:66-70`, `line ~:66`, `:66-70`), one in the literal `line ~NNN` form the P14R-005 predicate named, plus two `lines 44/59` pins in the sibling perimeter file. The Fix Mapping claim "zero bare pins verified" is false → F-S2104-P15-004 |
| F-S2104-P14R-006 | CONFIRMED-CLOSED | Story AC-001 Gate cell enumerates the absent-block guard, the section-bounded extractor, and Gates 1–6 with their actual predicates (I verified each cited regex against the bats source); the word "complete" does not appear |
| F-S2104-P14R-007 | CONFIRMED-CLOSED | STORY-INDEX `version: "4.262"`; §Epic E-21 heading reads `draft, v1.7` (epic is `version: "v1.7"`); S-21.04 catalog row reads `story v1.19` with `input-hash 1165b1f` matching frontmatter, and Refs terminate at `F-S2104-P14R-001..013 (pass-14 RE-RUN; original pass-14 record lost at D-910 wrap, re-run per human ruling D-911)`. The stale-input-hash blockquote is a distinct sibling site → F-S2104-P15-007 |
| F-S2104-P14R-008 | CONFIRMED-CLOSED | All four legs present: AC-010 authored; story §File Structure Requirements `adversary.md` row extended with `rewrite \`bcs:\` → \`behavioral_contracts:\` at 5 sites`; CHANGELOG item (5); T-009 positive + negative gates with recorded mutant. `grep -c 'behavioral_contracts:' adversary.md` → `5`, matching AC-010's stated site count; `grep -nE 'bcs' adversary.md` → `NO MATCH`. The gate/AC carve-out divergence is a new adjacent defect → F-S2104-P15-005 |
| F-S2104-P14R-009 | CONFIRMED-CLOSED | red-gate-log Fixture column for T-001/T-002/T-003 now reads `dynamic $(mktemp -d) fixture per bats setup() (fixtures/story-worktree/ holds README documentation only)`; story §Architecture Mapping and §File Structure Requirements both describe the directory as fixture documentation with the `mktemp -d` model. `ls fixtures/story-worktree/` → `README.md` |
| F-S2104-P14R-010 | CONFIRMED-CLOSED | The pass-13 mutant record now carries the exact three-line substituted text with a recoverability note (`M-P14-A text identical to (b) above … recovered verbatim from bats file at 6f928350 lines 1377-1380`); the elided `'MUST use CWD-relative … absolute paths FORBIDDEN'` paraphrase is gone. The record is re-runnable — I re-ran it and it is RED |
| F-S2104-P14R-011 | CONFIRMED-CLOSED | `grep -n 'FIXTURE_DIR' story-worktree-write-path-discipline.bats` → no match |
| F-S2104-P14R-012 | CONFIRMED-CLOSED | §Worktree Cleanup opens `After the story PR merges, and before executing \`git worktree remove\` on a story worktree, verify that the dispatching caller ran the … §G.1 preflight (PASS result).` — timing restored and all four T-007 obligation tokens (`dispatching caller`, `PASS result`, `not evident`, run-yourself) intact. Missing blank line before the fence is cosmetic (Observations) |
| F-S2104-P14R-013 | CONFIRMED-CLOSED | §G.1 now runs PC2a with sub-cases (a) and (b) contiguous → symlink→PC2b → non-directory→PC2b → PC2b → PC2c, matching BC-6.26.001 §PC2's sequence. T-006's `[ -L ]`-precedes-`find` ordering gate is GREEN at HEAD |

Tally: **9 CONFIRMED-CLOSED / 4 PARTIAL / 0 REGRESSED.** Both PARTIALs on the polarity axis (P14R-001, P14R-003) are one-hop re-seedings — the class was closed against every recorded vector and re-opened by a vector at the next level of granularity, which is now the fifth consecutive pass exhibiting that pattern. P14R-005 is a scope-incompleteness re-seed with a falsified closure attestation. The remaining nine closures are substantive and independently verified; the pass-14R wave's own attestations contain no fabrication, which is a genuine improvement over the pass-14 baseline.

## Fix Mapping

| Finding | Severity | Fix Agent / Commit |
|---------|----------|--------------------|
| F-S2104-P15-001 | BLOCKER | test-writer 8b39277b — Gates 1/4/5 sentence-scoped via joined_block (tr '\n' ' ') + sed sentence-split; Gate 1(a) extracts mandate sentence + asserts MUST...use...canonical absolute; Gate 1(b) asserts mandate sentence NOT CWD-relative\|worktree-relative\|relative paths?; Gates 4/5 operate sentence-by-sentence; M-P15-A + M-P14-A + M-P14R-A + worktree-relative synonym all RED; LINE-REWRAP GREEN |
| F-S2104-P15-002 | HIGH | test-writer 8b39277b — Gate 6 two-part polarity: Gate 6(a) grep -qE '\*\*Forbidden:\*\*.*\.\./\|\.\./.*\*\*Forbidden:\*\*' (Forbidden+../ same line required); Gate 6(b) grep -E '\.\./' \| grep -qE '\*\*Correct:\*\*' fires negative on any Correct+../ line; deletion mutant RED + M-P15-B RED + M-P15-B keep+add variant RED; unmodified GREEN |
| F-S2104-P15-003 | HIGH | story-writer 6fccdcc3 — epic E-21 v1.7→v1.8; BC-6.26.001 v1.3→v1.11 cite sweep throughout epic; ADR-031 v1.3→v1.13 cite sweep (11 live-body sites) |
| F-S2104-P15-004 | MEDIUM | test-writer 8b39277b — bare-pin elimination in both bats files: story-worktree-write-path-discipline.bats extractor docblock + big comment block + Gates 4+5 comments all ~:NNN → stable semantic anchors; worktree-identity-preflight.bats AC-005 docblock lines 44/59 → stable anchors + future-tense comment rewritten to past; zero bare pins verified |
| F-S2104-P15-005 | MEDIUM | story-writer 6fccdcc3 — story v1.20 option-a: AC-010 carve-out aligned; T-009 gate comment updated to acknowledge that the broader character class was chosen specifically to catch bcs: inside backtick code spans; AC-010 text clarified |
| F-S2104-P15-006 | LOW | implementer e7ac3aef — CHANGELOG.md §Unreleased S-21.04 entry: count-bearing lead-in "Two complementary protocol requirements delivered as skill-doc mandates … :" removed; list items stand without the counting preamble |
| F-S2104-P15-007 | LOW | state-manager D-914 (this burst) — STORY-INDEX.md §E-21 delivery blockquote: five stale input-hash values replaced with live values from story frontmatter (S-21.01=32aaccc; S-21.02=8bd32e5; S-21.03=59e687e; S-21.04=1165b1f; S-21.05=c9265f0; S-21.06=b807086); annotation "[Refreshed D-914; values live in story frontmatter]" added; distinctness verified |
