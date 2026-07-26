---
pass: 14
verdict: NOT-CLEAN
reviewed_head: 6f928350
novelty: 0.62
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-13.md"
rerun: true
---

## Record Provenance (state-manager, D-911)

The original pass-14 review (B0/H2/M4/L2, 8 findings F-S2104-P14-001..008, novelty 0.50, reviewed_head 09cfce81) was REVIEWED in-session on 2026-07-26 but its verbatim Part A was LOST at the D-910 session wrap before persistence (transcript-only; never relayed to a persist burst). The human ruled on 2026-07-26 (AskUserQuestion, this session) to RE-RUN pass 14 rather than reconstruct. This record is the verbatim re-run report, reviewed at 6f928350 (post-fix-wave state). Finding IDs in this record use the F-S2104-P14R-* namespace to avoid collision: citations of F-S2104-P14-001..008 in commits 6f928350/77aa0d55, story v1.18, and the bats attestation block refer to the LOST review and are NOT dangling. The re-run's finding count (13) supersedes the lost pass's count (8) as the authoritative pass-14 trajectory value.

## Summary

Pass-14 RE-RUN adversarial review of S-21.04 at reviewed_head 6f928350 (worktree `.worktrees/S-21.04`, base develop 948f0fb1). 13 findings: B1 / H2 / M7 / L3. Novelty 0.62 vs pass-13 Part A (8 of 13 novel in class; 5 are one-hop re-seedings of P13-001/002/003/007/008). Trajectory 14→18→17→12→11→11→9→9→10→11→7→10→10→13. Streak: **0/3** (BC-5.39.001 reset).

Baseline established by literal execution at HEAD: `bats story-worktree-write-path-discipline.bats` → `1..9`, 9/9 ok; `bats worktree-identity-preflight.bats` → `1..14`, 14/14 ok. The suite is green, and that green is the problem — F-S2104-P14R-001 proves end-to-end that a **complete polarity inversion of the BC-6.26.001 PC1 normative prohibition paragraph leaves T-001 GREEN**, i.e. the story's primary write-discipline postcondition is still ungated after three consecutive passes of "polarity" remediation (F-P12-003 → F-P13-001 → F-P14-001). The pass-14 fix wave recorded the claim "polarity-COMPLETE" in the artifact; that claim is falsified below. Severity escalated to BLOCKER on third-generation recurrence plus a false completeness attestation (TD-VSDD-059).

Second structural theme: the pass-14 red-gate attestation was written into the **shipped bats file** as a 112-line comment block labelled "D-910 verbatim transcription", while the red-gate-log SoT (`implementation/red-gate-log.md`, v1.11) still ends at the pass-13 section and still cites Summary HEAD `09cfce81`. Nine of the line-pin claims inside that block are already wrong at the commit that introduced them.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P14R-001 | BLOCKER | bats:523-573 vs _shared-context.md:66-70 | Third-generation polarity paper-gate: full inversion of the PC1 prohibition paragraph (mandate CWD-relative, FORBID canonical absolute) survives all five gates — T-001 GREEN, proven end-to-end. "polarity-COMPLETE" claim at bats:497-522 is false | BC-6.26.001 PC1, Invariant 1; POLICY 11, 13, 15; TD-VSDD-059 |
| F-S2104-P14R-002 | HIGH | red-gate-log.md:14,:45,:303-307 | No pass-14 attestation section; Summary HEAD cite still `09cfce81` at reviewed_head `6f928350`; version/last_amended still D-909. 4th-generation of the F-P10-007→F-P12-002→F-P13-002 class | POLICY 3, 15; TD-VSDD-059 |
| F-S2104-P14R-003 | HIGH | BC-6.26.001.md:158-159 vs _shared-context.md:112-113 | BC PC1 enumerates TWO Forbidden write forms; the relative-traversal form (`../../.factory/...`) has zero propagation into §Write Discipline, zero story mention, zero gate — PC1 under-instantiated | BC-6.26.001 PC1; POLICY 8, 11 |
| F-S2104-P14R-004 | MEDIUM | bats:1370-1477 vs decision-log.md:12157 | Red-gate-log attestation content placed in the shipped test file, labelled "D-910 verbatim transcription" — D-910 itself records that transcription as "NOT STARTED". Wrong SoT, wrong owner, ships in the plugin | POLICY 15, 16; CLAUDE.md Companion Principle (routing) |
| F-S2104-P14R-005 | MEDIUM | bats:1430-1433,:1457-1464,:572 | Nine bare line-pin claims in the attestation block are stale at HEAD (off by ~40); the surviving `(~:113)` pin in an emitted label falsifies P13-007's "zero bare line pins" closure | TD-VSDD-091; POLICY 5 |
| F-S2104-P14R-006 | MEDIUM | story:95 | AC-001 Gate cell not extended for the gates added at 6f928350 (negative Gates 4/5, section-bounded extractor) — byte-identical class to F-P13-008 one pass later | POLICY 8, 14/17 |
| F-S2104-P14R-007 | MEDIUM | STORY-INDEX.md:717,:727 | Epic pin `v1.5` vs epic v1.7; S-21.04 row pins `story v1.17` vs story v1.18; Refs range stops at P13 | POLICY 3, 14/17 |
| F-S2104-P14R-008 | MEDIUM | adversary.md:40,:275-278 | `bcs:` → `behavioral_contracts:` rewrite at 5 sites is undeclared — no AC, no story File-Structure note, no CHANGELOG item, no gate | POLICY 8; TD-VSDD-060 |
| F-S2104-P14R-009 | MEDIUM | red-gate-log.md:67-69 vs fixtures/story-worktree/README.md:14-15 | Red-gate-log Fixture column claims `fixtures/story-worktree/` supplies T-001/002/003 fixtures; the README states fixtures are created dynamically in `$(mktemp -d)` and the directory holds only README.md. Story repeats the false claim twice | POLICY 11; TD-VSDD-059 |
| F-S2104-P14R-010 | MEDIUM | red-gate-log.md:307 | Pass-13 mutant record uses an elided paraphrase (`'MUST use CWD-relative … absolute paths FORBIDDEN'`) instead of the exact substituted text | POLICY 15 |
| F-S2104-P14R-011 | LOW | bats:63 | `FIXTURE_DIR` assigned and never read — dead stub residue surviving 13 passes | POLICY 11 |
| F-S2104-P14R-012 | LOW | devops-engineer.md:356-365 | `### Worktree Cleanup` lost its trigger sentence ("After story PR merges:"); section no longer states when cleanup occurs and the bash fence dangles | BC-6.26.001 Precondition 3 |
| F-S2104-P14R-013 | LOW | step-g-cleanup.md §G.1 vs BC-6.26.001.md:176-200 | §G.1 nests the symlink→PC2b and non-directory→PC2b paragraphs inside the "PC2a — No stray files (teardown authorized)" block, splitting PC2a sub-case (a) from sub-case (b); BC keeps PC2a's sub-cases together | BC-6.26.001 PC2; POLICY 4 |

---

### F-S2104-P14R-001 — BLOCKER — polarity gate set is complete only against its own recorded mutant token

`bats:497-522` records the claim, verbatim:

```
  # FORBIDDEN and that canonical absolute paths are MANDATED. Five independently mutant-proven
  # gates (pass-14 adds Gates 4+5 for polarity-COMPLETE per M-P14-A surviving mutant; F-S2104-P14-001):
```

The five gates are: absent-block check (`bats:526`), Gate 1 `MUST.*absolute|absolute.*MUST` per line (`bats:533`), Gate 2 joined `(CWD-relative|relative path).*FORBIDDEN|FORBIDDEN.*(CWD-relative|relative path)` (`bats:543`), Gate 4 negative `absolute.*(FORBIDDEN|forbidden)|FORBIDDEN.*absolute|forbidden.*absolute` per line (`bats:555`), Gate 5 negative `MUST.*(CWD-relative)|(CWD-relative).*MUST` per line (`bats:565`).

Gate 5 — named "the PRIMARY polarity-catching gate" at `bats:564` — keys on the **single literal token `CWD-relative`**, which is the token that happened to appear in M-P14-A. Substituting the synonym `relative` for `CWD-relative` in the inverted mandate defeats it. The file's own comment at `bats:1390-1391` already concedes Gate 1 cannot distinguish polarity:

```
#       Note: Gate 1 (MUST+absolute) also passes for M-P14-A — mutant line 1 contains both "MUST" and
#         "not canonical absolute paths", so Gate 1 alone cannot distinguish polarity. Gate 5 is required.
```

**M-P14R-A — exact substituted text** (replaces `_shared-context.md:66-70`):

```
All `.factory/**` artifact writes performed during story delivery MUST use relative paths, not canonical absolute paths
anchored to the main-checkout root. Canonical absolute paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`)
are FORBIDDEN — relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.
```

**Per-gate literal shell, captured stdout:**

```
$ [ -z "$B" ] && echo "FIRES (RED)" || echo "passes"
passes
$ printf '%s\n' "$B" | grep -qE 'MUST.*absolute|absolute.*MUST' && echo "passes (mutant survives)" || echo "FIRES (RED)"
passes (mutant survives)
$ printf '%s\n' "$B" | tr '\n' ' ' | grep -qE '(CWD-relative|relative path).*FORBIDDEN|FORBIDDEN.*(CWD-relative|relative path)' && echo "passes (mutant survives)" || echo "FIRES (RED)"
passes (mutant survives)
$ printf '%s\n' "$B" | grep -qE 'absolute.*(FORBIDDEN|forbidden)|FORBIDDEN.*absolute|forbidden.*absolute' && echo "FIRES (RED)" || echo "passes (mutant survives)"
passes (mutant survives)
$ printf '%s\n' "$B" | grep -qE 'MUST.*(CWD-relative)|(CWD-relative).*MUST' && echo "FIRES (RED)" || echo "passes (mutant survives)"
passes (mutant survives)
```

**End-to-end proof** (scratch copy of the full `plugins/` tree, `_shared-context.md:66-70` replaced with M-P14R-A, unmodified bats suite):

```
$ awk 'NR>=66 && NR<=68' $SCR/plugins/.../_shared-context.md
All `.factory/**` artifact writes performed during story delivery MUST use relative paths, not canonical absolute paths
anchored to the main-checkout root. Canonical absolute paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`)
are FORBIDDEN — relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.
$ bats -f "T-001" story-worktree-write-path-discipline.bats
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
bats exit=0
```

Gate 2 survives because `relative paths` matches the `relative path` alternand while `FORBIDDEN` follows it in the joined text — the gate confirms the *tokens* co-occur, never *which* subject is forbidden. Gate 3 (`bats:571`) reads `$spec_path_section`, not the mutated paragraph, so the untouched `**Forbidden:**` example at `_shared-context.md:113` keeps it GREEN. Net effect: the delivered skill-doc could instruct every story agent to do exactly what issue #523 did, and the S-21.04 gate suite would report 9/9 ok.

**Zero-degrees-of-freedom predicate the fixer must satisfy:** (a) Gate 1 must assert an *affirmative* mandate, not token co-occurrence — a line in `$prohibition_block` must match `MUST[^.]*use[^.]*canonical absolute paths` AND that same line must NOT match `not[[:space:]]+canonical absolute|not[[:space:]]+absolute`; (b) Gate 5's token set must be a POLICY-13 alternation over every syntactic form of the prohibited subject, at minimum `CWD-relative|worktree-relative|relative[[:space:]]+path`, applied per line against `MUST`; (c) each of (a) and (b) must carry its own recorded mutant with exact substituted text — M-P14R-A above is the mandatory vector for (b), and M-P14-A (already recorded) for Gate 5's original form. A fix that only appends `|relative path` to Gate 5 is insufficient: it must be shown RED against M-P14R-A **and** GREEN against `_shared-context.md:66-70` unmodified, with captured stdout.

### F-S2104-P14R-002 — HIGH — red-gate-log carries no pass-14 attestation and cites a superseded HEAD

`red-gate-log.md:45` (Summary row), verbatim:

> `All GREEN at worktree HEAD 09cfce81 (orchestrator-executed: 9/9 + 14/14, 2026-07-26).`

reviewed_head is `6f928350`. `red-gate-log.md:4` is `version: "1.11"`; `:14` `last_amended` terminates at `2026-07-26 D-909`; the final section of the file is `:303` `### Pass-13 assertion-site attestation (09cfce81)`. The assertion sites added or strengthened at `6f928350` — `_extract_write_discipline_prohibition_block` section-bounding (`bats:137-143`), Gate 2 re-implementation (`bats:543`), Gate 4 (`bats:555`), Gate 5 (`bats:565`) — have **no** attestation, no mutant record, and no Green-commit row in the SoT. This is the fourth consecutive generation of the same class (F-P10-007 → F-P12-002 → F-P13-002 → here). Predicate: append a `### Pass-14 assertion-site attestation (6f928350)` section enumerating each new/changed assertion site with its exact-substituted-text mutant and captured RED/GREEN stdout, advance the Summary HEAD cite to `6f928350`, and bump version/`last_amended`/`modified[]` in the same burst.

### F-S2104-P14R-003 — HIGH — BC PC1's second Forbidden form is unpropagated and ungated

`BC-6.26.001.md:158-159`, verbatim:

```
- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative
  traversal — brittle and error-prone)
```

`_shared-context.md:112-113` carries only the Correct form and the single CWD-relative Forbidden form; the traversal form is absent:

```
$ grep -n '\.\./\.\.' plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md
NO MATCH — BC PC1 third Forbidden bullet absent from skill-doc
$ grep -n 'traversal' .factory/stories/S-21.04-story-worktree-write-path-discipline.md
NO MATCH in story
```

The only `traversal` token in the bats suite is `bats:934`, inside T-004's PC2c error-condition alternation — unrelated to PC1. Story AC-001(a) narrows the obligation to "relative paths (`".factory/..."` from story-worktree CWD)", which is strictly weaker than PC1. A `../../` write is not caught by Gate 2 or Gate 5 either (neither `CWD-relative` nor `relative path` appears in a traversal literal). Predicate: add the traversal form as a third bullet under `_shared-context.md` §Write Discipline load-bearing examples, extend AC-001(a) to name both forbidden forms, and add a T-001 assertion requiring a `\.\./` traversal example inside the §Spec-Path Discipline section with a recorded deletion mutant.

### F-S2104-P14R-004 — MEDIUM — attestation written into the shipped test file and mis-cited to D-910

`bats:1370-1373`, verbatim:

```
# ===========================================================================
# RED-GATE-LOG ATTESTATION — D-910 verbatim transcription (pass-14 legs)
# F-S2104-P14-001 / F-S2104-P14-002 / F-S2104-P14-005 / F-S2104-P14-007 / F-S2104-P14-008
# ===========================================================================
```

`decision-log.md:12157` (D-910), verbatim:

> `D-910-class closure burst (P14-003 STORY-INDEX epic pin v1.5→v1.7; P14-006 mutant-record precision; verbatim pass-14 record persist) NOT STARTED.`

D-910 is the session-wrap decision, not a red-gate-log transcription decision, and it explicitly records the transcription as not started — so the block's own provenance label is unsupported at authoring time (POLICY 16). Structurally: 108 of the file's 1478 lines are factory-process bookkeeping in a `plugins/vsdd-factory/tests/` artifact that ships in the marketplace tarball; the content duplicates what `red-gate-log.md` owns, creating a two-SoT divergence surface (state-manager owns the log; test-writer authored this). Predicate: move the block verbatim into `red-gate-log.md` as the pass-14 attestation section (closing F-S2104-P14R-002), delete it from the bats file, and cite the D-NNN actually allocated for the closure burst.

### F-S2104-P14R-005 — MEDIUM — the attestation block's own line pins are wrong at the commit that introduced them

LEG 5 (`bats:1457-1464`) and LEG 3 (`bats:1430-1433`) pin positions that do not exist at HEAD. Claimed vs actual (`grep -n` at reviewed_head):

| claimed | actual | text |
|---|---|---|
| `line ~696` | 736 | `# Deleting the absent-path clause from §G.1 must fail this assertion.` |
| `line ~700` | 740 | `...breaks the absent-path contract — F-S2104-P2-009)` |
| `line ~754` | 794 | `# Explicitly covers EC-003 (empty dir scenario distinct from EC-005 absent-path scenario).` |
| `line ~1106` | 1146 | banner `absent-path/find-error as unordered` |
| `line ~1114` | 1154 | `#   (ii) NOT present \`find\` as the first action with absent-path/find-error...` |
| `line ~1115` | 1155 | `#        (anti-pattern: inline bare find-first command without explicit absent-path-first ordering)` |
| `line ~1134` | 1174 | `echo "DOC-PARITY FAIL [anti-pattern present in $label]: ...absent-path check is first...` |
| `line ~1184` ("After") | 1224 | `# AC-009 specialist agent awareness — obligation-asserting...` |
| `line ~1272` ("After") | 1312 | `# AC-008: executor-side defensive preflight — verify-PASS + run-it-yourself obligations` |

All nine are off by exactly the +40 lines that the same commit inserted — the pins were computed against `09cfce81` and never re-derived, so they describe positions that never existed in the committed artifact. Separately, `bats:572` retains a bare pin inside an **emitted** DOC-PARITY label:

```
    "_shared-context.md §Spec-Path Discipline: **Forbidden:** example marker must co-occur with 'relative path' on the same line (~:113) — deleting the **Forbidden:** example line fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P12-003)" \
```

`git show 09cfce81:...bats | grep -n '~:113'` returns `532:` — i.e. this pin was already present in the commit that pass-13's Fix Mapping records as "zero bare line pins; predicate stdout", so that closure claim is falsified. Predicate: replace every `line ~NNN` / `(~:NNN)` with a stable anchor (heading, function name, or verbatim token), and where the attestation needs positional evidence use `grep -n`-free structural-form output per POLICY 5.

### F-S2104-P14R-006 — MEDIUM — AC-001 Gate cell not extended for the pass-14 gates

`story:95` Gate cell, verbatim:

> `bats T-001: Write Discipline clause-content gates (CANONICAL_FACTORY_ROOT, DELIVERY ledger, pr-review.md, story-frontmatter, EC-006 WARNING) + AC-001(a) prohibition gates (polarity-aware paragraph extraction + Forbidden-example marker); manual-fallback only for prose review`

Story v1.18 (`77aa0d55`) changed only Task 10 (`modified[]:46` — "Task 10 suite-green mirror removed"). The cell still describes pass-13's two-gate shape ("polarity-**aware** paragraph extraction + Forbidden-example marker") while the delivered gate set at `bats:526-573` is five gates including two negative polarity gates and a section-bounded extractor. Pass-13 F-P13-008 was the identical defect one generation earlier ("AC-001 Gate cell not extended for the gates added at 264f53b6"). Predicate: extend `story:95` to enumerate the absent-block guard, Gate 1 mandate-polarity, Gate 2 joined co-occurrence, Gate 3 Forbidden-marker, Gate 4 and Gate 5 negatives, and the section-bounded extractor — and per F-S2104-P14R-001 do **not** carry the word "complete" until M-P14R-A is RED.

### F-S2104-P14R-007 — MEDIUM — STORY-INDEX epic and story pins stale

`STORY-INDEX.md:717`, verbatim:

> `## Epic E-21 — Factory State Data-Loss Hardening (v1.0-brownfield-backfill) — draft, v1.5`

`epics/E-21-...md:4` is `version: "v1.7"`. `STORY-INDEX.md:727` (S-21.04 catalog row) reads `story v1.17` and terminates its Refs at `F-S2104-P13-001..010, F-S2104-P13-D1`, while the story is v1.18. POLICY 14/17 leg 5 (upstream-index cell) was not advanced in the same burst as the v1.18 bump. Predicate: `:717` `v1.5`→`v1.7`; `:727` `story v1.17`→`story v1.18` with the pass-14 Refs range appended; sibling-sweep the E-21 blockquote pins in the same edit.

### F-S2104-P14R-008 — MEDIUM — undeclared frontmatter-field rewrite in adversary.md

The diff `948f0fb1..6f928350` rewrites `bcs:` → `behavioral_contracts:` at five sites in `adversary.md` (Perimeter-1 scope sentence `:40`, and bidirectional-BC-completeness items 1–4 at `:275-278`). The change is correct in itself, but it is anchored nowhere: no AC covers it (AC-009 covers only the corrected shadow-write model and the §G.1 reference), the story's §File Structure Requirements row for `adversary.md` (`story:269`) describes only the stale-snapshot retraction and §G.1 awareness, and the CHANGELOG's four-item S-21.04 entry never mentions it (`grep -n "behavioral_contracts" CHANGELOG.md` returns only lines 1987/5301/5306/5498, all inside previously-released sections). No test gates it. Predicate: either add an AC + File-Structure row + CHANGELOG clause + T-009 assertion for the field-name correctness obligation, or move the rewrite to its own story — an ungated ungoverned edit in a shipped agent prompt is not deliverable under the production-grade default.

### F-S2104-P14R-009 — MEDIUM — fixture provenance contradicts the fixture README

`red-gate-log.md:67-69` Fixture column, verbatim: `fixtures/story-worktree/ (stray \`.factory/stories/S-021-DELIVERY.md\`)` for T-001, `fixtures/story-worktree/ (empty shadow \`.factory/\`)` for T-002, `fixtures/story-worktree/ (stray file then relocated)` for T-003.

`fixtures/story-worktree/README.md:14-15`, verbatim:

> `The fixture is created **dynamically** in a \`$(mktemp -d)\` temp directory by the bats \`setup()\` function — there is no persistent on-disk state to check out.`

```
$ ls fixtures/story-worktree/
README.md
```

The directory contains no fixture. `story:134` (§Architecture Mapping) and `story:258` (§File Structure Requirements) repeat the claim — the latter as `create | Fixture worktree directory for bats tests (stray \`.factory/\` file scenarios)`. Three documents assert a fixture source that does not exist; the README is the only accurate one. Predicate: correct the red-gate-log Fixture column to `tmpfs $(mktemp -d) per setup()` and re-word the two story rows to describe the directory as fixture *documentation*, matching the README's own statement.

### F-S2104-P14R-010 — MEDIUM — pass-13 mutant record is elided, not exact

`red-gate-log.md:307`, verbatim:

> `(b) POLARITY INVERSION ('MUST use CWD-relative … absolute paths FORBIDDEN') → T-001 RED 'DOC-PARITY FAIL [write-discipline prohibition block mandate-polarity]' exit 1;`

The ellipsis means the substituted text is not recoverable from the record, so the mutant cannot be re-run to confirm the gate is still load-bearing — which is precisely how the surviving vector in F-S2104-P14R-001 went unnoticed. Predicate: replace with the exact three-line substituted block (as `bats:1377-1380` does for M-P14-A) for every recorded mutant in the log.

### F-S2104-P14R-011 — LOW — dead `FIXTURE_DIR`

```
$ grep -n "FIXTURE_DIR" story-worktree-write-path-discipline.bats
63:  FIXTURE_DIR="$PLUGIN_ROOT/tests/fixtures/story-worktree"
```

Single occurrence — assigned in `setup()`, never read by any test or helper. Stub residue from `63b7fb79`, surviving 13 passes. Predicate: delete the assignment.

### F-S2104-P14R-012 — LOW — devops-engineer §Worktree Cleanup lost its trigger context

`devops-engineer.md:356-365` at HEAD:

```
### Worktree Cleanup

Before executing `git worktree remove` on a story worktree, verify that
the dispatching caller ran the `plugins/.../step-g-cleanup.md §G.1`
preflight (PASS result). If not evident from the dispatch, run the §G.1 preflight yourself first
(BC-6.26.001 Invariant 2, PC2, Precondition 3; ...).
```bash
git worktree remove .worktrees/STORY-NNN
```
```

The diff replaced the section's only trigger sentence (`After story PR merges:`) rather than inserting before it, so the section no longer states *when* cleanup runs and the fenced command has no lead-in. T-007's gates (`bats:1344-1367`) do not detect this because they assert only the preflight obligation tokens. Predicate: restore a trigger sentence ("After the story PR merges, and before executing `git worktree remove`, …") preserving both the timing and the Precondition-3 obligation.

### F-S2104-P14R-013 — LOW — §G.1 block nesting diverges from BC-6.26.001 PC2 structure

In `step-g-cleanup.md` §G.1 the bolded `**PC2a — No stray files (teardown authorized):**` block opens with `*Sub-case (a) — \`.factory/\` path absent:*`, is then interrupted by `**Symlink at \`.factory/\` path → PC2b BLOCKED...**` and `**Non-directory at \`.factory/\` path → PC2b BLOCKED...**`, and only then resumes with `*Sub-case (b) — \`find\` exits 0, empty output:*`. `BC-6.26.001.md:176-183` keeps PC2a's sub-cases (a) and (b) in one paragraph and places the non-directory-or-symlink paragraph *after* PC2a (`:185-194`). The runtime ordering in §G.1 is correct (existence → `[ -L ]` → `[ ! -d ]` → `find`); the defect is that a PC2a-authorized sub-case is rendered downstream of two BLOCKED branches under an "teardown authorized" label. Predicate: reorder §G.1 to BC structure — PC2a with both sub-cases, then the symlink/non-directory→PC2b paragraph, then PC2b, PC2c — and re-run T-006's `[ -L ]`-precedes-`find` ordering gate (`bats:1093-1106`) to confirm it stays GREEN.

---

## Observations (NOT findings)

**Behavioral axis holds.** The full discrimination chain is reproduced faithfully in both the doc and the harness: `_run_teardown_preflight` (`bats:256-358`) implements steps 1–3 hardcoded and step 4 doc-derived, and every routing branch is exercised — PC2a(a) `bats:783-791`, PC2a(b) `bats:796-806`, PC2b stray-files `bats:669-694`, PC2b non-directory `bats:1020-1044`, PC2b symlink `bats:1110-1138`, PC2c `bats:951-969`. The `REMOVE_LOG` sentinel is load-bearing on every BLOCKED/HALT path. The anti-tautology extraction gate (`bats:267-279`) is genuinely doc-derived: a `-type d` or `2>/dev/null` doc-mutant empties `find_cmd_line` and both T-001 and T-002 fail. The `[ -L ]`-before-`find` ordering gate is a real awk line-number comparison, not a token check.

**Six-surface and awareness propagation verified independently.** All six T-008 surfaces carry the fully-qualified `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` path and none inlines a bare `find`; the anti-pattern regex at `bats:1173` correctly handles the quoted canonical form. T-009's three obligation gates per file are satisfied by real prose at `adversary.md:54` ("checks out NOTHING under", "not dismissed as a pathing artifact", "step-g-cleanup.md §G.1") and `adversarial-review/SKILL.md:77,:93` ("no `.factory/` directory is created at worktree-checkout time", "not used as spec ground-truth"). T-007's four obligation gates are satisfied at `devops-engineer.md:358-362`.

**Spec chain otherwise consistent.** ADR-031 v1.13 §Decision 4 five-surface enumeration matches BC-6.26.001 v1.11 §Traceability Architecture Module and §Architecture Anchors (`:311`, `:323-327`) exactly; epic v1.7 is fully synced to six stories (`:48`, `:154` EAC-001); BC-6.27.001 v1.4 sibling pin holds; POLICY 19 ADR cites are stable-form (`ADR-031 §Decision 4`, `§Rationale`) with no load-bearing version tokens; POLICY 21 satisfied — zero new `.sh` files, `fixtures/story-worktree/` contains only `README.md`.

**CHANGELOG substantively accurate** for items (1), (2) and (4); item (3)'s adversary reporting-semantics description matches `adversary.md:54`. Its only gap is the omission covered by F-S2104-P14R-008.

**Retired-class sweeps CLEAN.** `absent-dir` survives only inside the two `_assert_no_doc_marker 'absent-dir'` negative gates (`bats:610`, `bats:657`) that forbid the token in gated docs — LEG 5's classification is correct on that point. `2>/dev/null` appears on live preflight commands nowhere in `step-g-cleanup.md` §G.1 or BC-6.26.001. No `stale-snapshot` residue remains in `adversary.md` (`grep -n "stale"` → only `:221`, an unrelated severity-rubric row).

**[process-gap] — mutant-token generality is not a codified requirement.** Three consecutive passes have closed the polarity class by adding a gate keyed to the exact token in the *previously surviving* mutant, and each time the next pass found a one-token synonym that walks around it. The missing rule is POLICY 13 applied to *mutant-derived* gates: when a gate is added in response to a specific mutant, the gate's predicate must be an alternation over the syntactic-form class of the mutated token, and the attestation must record at least one *synonym-substituted* mutant proving the alternation — not only the original vector. Codifying this is the only mechanism that stops the fourth generation.

**[process-gap] — red-gate attestation is being written wherever the authoring agent has write access.** Pass-13's attestation reached `red-gate-log.md` only via a state-manager burst; pass-14's went into the bats file because that burst had not run. The gap is that no gate asserts *location*. Candidate codification: a fix wave that adds or strengthens a bats assertion site MUST NOT be pushed until the matching `red-gate-log.md` section exists at that commit, verified by literal shell (`grep -c "assertion-site attestation (<HEAD>)"` → `1`).

---

## Per-Pass-13 Verification

| Finding | Status | Notes |
|---------|--------|-------|
| F-P13-001 | PARTIAL (3rd-gen re-seed) | Gates 4/5 landed at `bats:555`/`:565` and M-P14-A is genuinely RED; but Gate 5's single-literal `CWD-relative` token is walked around by the synonym form — M-P14R-A leaves T-001 `ok` (literal `bats -f T-001` exit 0 on mutated scratch tree). PC1 core still ungated → F-S2104-P14R-001 |
| F-P13-002 | PARTIAL | Pass-13 leg closed: `red-gate-log.md:303` `### Pass-13 assertion-site attestation (09cfce81)` present; `:289` count phrase replaced by COUNT-FREE pointer. Class re-seeded for pass-14: no pass-14 section, Summary HEAD `:45` still `09cfce81` → F-S2104-P14R-002 |
| F-P13-003 | CONFIRMED-CLOSED | `:719` now `six data-loss hardening stories` + `3 waves`; `:748` `35 E-21`; `:761` carries `[Historical v1.0 tally` marker. New sibling site (`:717` epic pin) is a distinct defect → F-S2104-P14R-007 |
| F-P13-004 | CONFIRMED-CLOSED | `red-gate-log.md:67` T-001 row and `:97` §Traces both carry `AC-003; AC-001 (...); AC-002 (...); AC-007 (a)-(c)`; `:68`/`:98` T-002 carry `AC-004; AC-002` |
| F-P13-005 | CONFIRMED-CLOSED | `story:130` — step-d5 annotated `(story-scoped — not a BC §Architecture Anchors surface; swept for the corrected shadow-write model as part of this story)`; no residue |
| F-P13-006 | CONFIRMED-CLOSED | `bats:53` header maps `T-009  AC-009  adv-awareness`; banner `bats:1224` is `# AC-009 specialist agent awareness` — AC-007(d) removed. `grep -n 'AC-007(d)'` returns only T-008/six-surface contexts (52, 1143, 1147, 1151, 1174, 1186) |
| F-P13-007 | PARTIAL | The `line 60` pin is gone, but the emitted label at `bats:572` still carries `(~:113)` — present unchanged at `09cfce81` (`git show 09cfce81:...bats \| grep -n '~:113'` → `532:`), so the "zero bare line pins" closure was never true. Nine further pins added at `6f928350`, all stale → F-S2104-P14R-005 |
| F-P13-008 | PARTIAL | `story:95` AC-001 Gate cell was extended for pass-13's gates at `106bb5f5`, then not extended for the gates added at `6f928350` — same defect, next generation → F-S2104-P14R-006 |
| F-P13-009 | CONFIRMED-CLOSED | T-005 sites now carry closing SHAs: `bats:1007` `was RED at 60f0d2d6 until implementer adds non-directory-case paragraph at 73c2bade`; `bats:1014` same form |
| F-P13-010 | CONFIRMED-CLOSED | `story:183-196` — Tasks 1–14 all `[x]`, each with a closing SHA or an explicit SoT pointer (Task 10 → red-gate-log Summary per v1.18) |
| F-P13-D1 | CONFIRMED-CLOSED | Epic v1.7: `:48` `E-21 collects six hardening stories`; `:154` `EAC-001 \| All six stories S-21.01..S-21.06`; `modified[]` records the S-21.06 registration at v1.6 |

Tally: 7 CONFIRMED-CLOSED / 4 PARTIAL / 0 REGRESSED. Every PARTIAL is a one-hop re-seeding of the class the pass-13 wave was closing — the diagnosis recorded in pass-13's Observations is confirmed for a fourth consecutive pass, and F-S2104-P14R-001 shows the pattern now extends to the story's primary BC postcondition with a falsified completeness attestation attached.
