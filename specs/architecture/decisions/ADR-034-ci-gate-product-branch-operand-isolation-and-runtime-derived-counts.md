---
document_type: adr
adr_id: ADR-034
version: "1.1"
title: "ADR-034: CI gate product-branch operand isolation and runtime-derived counts"
status: proposed
date: 2026-07-30
producer: architect
timestamp: 2026-07-30T00:00:00Z
deciders:
  - architect
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-027 (factory-artifacts worktree path discipline — established canonical `.factory/` mount convention this ADR constrains for required CI gates)
  - ADR-030 (pr-manager merge operation integrity enforcement — adjacent governance gate design; same fail-closed requirement class)
anchors:
  - SS-05
  - SS-06
subsystems_affected:
  - SS-05
  - SS-06
last_amended: "2026-07-30 (v1.1) — Decision 2 revised (architect): counting predicate corrected from _assert_doc_marker call sites (wrong surface) to inline echo-DOC-PARITY-FAIL blocks within AC-001(a) section (correct surface, verified count = 24). T001_GATE_COUNT declared value updated to 24. Story routing updated."
modified:
  - "2026-07-30 (v1.0) — initial ruling"
  - "2026-07-30 (v1.1) — Decision 2 amendment: correct counting surface, declared value 24"
---

# ADR-034: CI gate product-branch operand isolation and runtime-derived counts

## Context

Pass-29 of the S-21.04 adversarial cascade raised two coupled findings against
T-016 (`test_coupling_gate_story_gate_count_matches_bats_count_word`) in
`plugins/vsdd-factory/tests/worktree-identity-preflight.bats`:

**F-S2104-P29-H05 — cross-branch operand in a required CI check.**
T-016 compares two operands that live on different branches:

- `story_count` — extracted from the AC-001 table row of
  `.factory/stories/S-21.04-story-worktree-write-path-discipline.md`,
  which lives on the `factory-artifacts` orphan branch (mounted at `.factory/`
  via the `Mount factory artifacts` step in the `bats-full-suite` CI job).
- `bats_count` — derived from the prose comment "Twenty-three independently
  mutant-proven gates" in `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`,
  which lives on the product branch.

The mount is **unpinned to tip**: CI fetches and mounts `origin/factory-artifacts`
at the time the job runs. Any state-manager burst that changes S-21.04's AC-001
gate count immediately reds `bats-full-suite` on every open PR repo-wide — including
release PRs — until a product-branch PR lands. This outage has already materialized
twice in this cascade (Nineteen→Twenty-one at pass-19; Twenty-one→Twenty-three at
pass-27). After S-21.04 merges to `develop`, the coupling becomes permanent against
a story document that will keep being amended.

**F-S2104-P29-H01 — neither operand is derived from executing assertions.**
Both operands are literal string tokens:

- `story_count` — the integer extracted from `\([0-9]+ gates` in the AC-001 table
  cell. This is a prose annotation in the story document, not a count of executing
  assertions.
- `bats_word` — the word extracted by `grep -oiE 'Twenty-[a-z]+'` matching the
  comment "Twenty-three independently mutant-proven gates" in the bats suite.
  This is a narrative comment inside T-001, not a count of executing assertion blocks.

Consequence: deleting or modifying any of the AC-001(a) gate assertions within T-001 while both
tokens still read "twenty-three" is undetected by T-016, T-001, and the story.
Confirmed by literal grep:

```
$ grep -rn 'gate_count\|gates_total\|GATE_COUNT' plugins/vsdd-factory/tests/
  → only the test's own name
```

**Assertion structure of T-001 (measured, literal shell).**
T-001 (`story-worktree-write-path-discipline.bats` lines 504–1870) has five
load-bearing layers. The `_assert_doc_marker` and `_assert_no_doc_marker` helper
functions are used exclusively in the §G.1 and Primary-paths layers; they are
absent from the AC-001(a) Write Discipline layer. Counted by literal shell:

```
$ awk 'NR>=504 && NR<=1870' story-worktree-write-path-discipline.bats \
    | grep -oE '^\s*_assert[a-z_]*' | sort | uniq -c
  21 _assert_doc_marker
   4 _assert_no_doc_marker
```

The AC-001(a) Write Discipline layer uses exclusively inline
`if … echo "DOC-PARITY FAIL" … false … fi` blocks. Counting those within the
section bounded by stable comment markers:

```
$ awk '/# --- DOC-PARITY §Spec-Path Discipline: AC-001\(a\)/,\
        /# --- DOC-PARITY §Spec-Path Discipline: EC-006 WARNING/' \
    story-worktree-write-path-discipline.bats \
  | grep -cE '^\s+echo "DOC-PARITY FAIL'
24
```

The 24 blocks span: HTML-comment absence, balanced-fence, anchor uniqueness,
prohibition-block absent, boundary-completeness, mandate-sentence absent,
Gates 1(a)/1(b)/1(c)/1(d)/1(e)/1(f), Gate PW-B, Gate 2a, Gates 2b(a)/2b(c),
scope-restriction, Gates 3/4/5/6(a)/7(a), write-directive, and canonical-target.
The total inline DOC-PARITY FAIL count across all T-001 layers is 29: 4 in §G.1,
24 in AC-001(a), and 1 in AC-001(b) EC-006 WARNING.

The comment at line 642 says "Twenty-three independently mutant-proven gates."
That count excluded boundary-completeness: its own FAIL message states "CONTROL-D
(capital G) fires Gate PW-B independently and serves as the missed-boundary mutant,"
meaning boundary-completeness is not independently proven (PW-B already catches
CONTROL-D). For a mechanical predicate this historic exclusion must be an explicit
rule, not implicit author judgment. Decision 2 instead counts all 24 DOC-PARITY FAIL
blocks in the AC-001(a) section and sets `T001_GATE_COUNT=24`.

**These two findings share one rewrite.** A design that fixes H05 (cross-branch
read) without fixing H01 (string-vs-string comparison) closes the branch-coupling
hazard while leaving the assertion-count enforcement hollow. The correct resolution
eliminates both defects in a single redesign of T-016.

**Boundary between spec-validation suites and coupling-count gates.**
Multiple bats suites (`f2-process-gap-lesson-gates.bats`, `pure-parse-invariant-gate.bats`,
`perf-baseline.bats`) also read from `.factory/` via `$FACTORY_ROOT`. These suites
validate spec documents (BCs, VPs, stories) whose authoritative source IS the
`factory-artifacts` branch by design. Their cross-branch reads are appropriate: the
subject matter is the spec content, not a derived count used as a CI operand.
T-016 is different in kind: it uses a cross-branch prose token as an operand in
a required equality check. This distinction governs the ruling.

**BC-5.39.008 v1.6 constraint.** The product-owner ruled that fail-open is never
valid for a governance gate that cannot read its target. Any redesign that
silently passes or skips when `.factory/` is absent violates this ruling.

## Decision

### Decision 1 — Remove the cross-branch read from T-016 entirely

T-016 MUST NOT read from `$fa_wt` (the mounted `factory-artifacts` worktree).
The test is self-contained on the product branch. When `.factory/` is absent, T-016
runs correctly because it no longer needs it.

The `fa_wt` discovery block (the worktree-list probe and the hard-fail guard when
`fa_wt` is empty), the `story_file` path assignment, and the `story_count` extraction
block MUST all be removed from T-016's rewrite.

### Decision 2 — Replace `bats_word` with a runtime-derived DOC-PARITY FAIL block count

T-016's `actual_count` operand MUST be derived by counting inline
`echo "DOC-PARITY FAIL"` lines within the AC-001(a) Write Discipline block of
`story-worktree-write-path-discipline.bats` at test-run time.

The AC-001(a) block is defined by two stable section-marker comment lines that
bracket exactly those gates:

```
# --- DOC-PARITY §Spec-Path Discipline: AC-001(a) CWD-relative-path PROHIBITION …
```
(opening marker — the text beginning the comment block)

```
# --- DOC-PARITY §Spec-Path Discipline: EC-006 WARNING …
```
(closing marker — the text opening the next, distinct AC-001(b) sub-section)

**Do NOT count `_assert_doc_marker` or `_assert_no_doc_marker` calls.** Those helper
functions are used only in the §G.1 and Primary-paths layers. Every AC-001(a) gate is
implemented as a bare `if … echo "DOC-PARITY FAIL" … false … fi` block. A count of
`_assert_doc_marker` calls gives 21 (§G.1 + Primary-paths), which is neither the
pre-pass-26 gate count nor the post-pass-26 gate count and must not be used.

The extraction method (awk section-range, grep, etc.) is test-writer's implementation
choice. The constraint is that the scope is the AC-001(a) block, not the full file and
not a prose comment. Verified count: **24** (literal shell, see Context above).

If `actual_count` is zero, T-016 MUST fail loudly — this guards against the
extraction silently failing to find the section markers or T-001.

### Decision 3 — Replace `story_count` with a product-branch declared constant

T-016's expected-count operand MUST come from a product-branch sentinel, not the
story document on `factory-artifacts`. The canonical form is a constant comment
on its own line at the head of the AC-001(a) Write Discipline block, immediately
after the `# --- DOC-PARITY §Spec-Path Discipline: AC-001(a)` opening marker and
before the first gate code:

```bash
# T001_GATE_COUNT=24
```

**Declared value is 24.** This matches the mechanical DOC-PARITY FAIL block count
of 24 verified in Decision 2 (literal shell). The existing prose comment
"Twenty-three independently mutant-proven gates" is off by one relative to this
predicate because it historically excluded the boundary-completeness assertion
(F-S2104-P19-004) as not independently proven; that exclusion requires author
judgment and is not mechanically reproducible. Setting `T001_GATE_COUNT=24` uses
the fully mechanical count; the story's gate count cell must be updated from
"twenty-three" to "twenty-four" concurrently (route: story-writer).

T-016 extracts this constant from the bats suite. If the sentinel is absent,
T-016 MUST fail loudly.

The declared constant is maintained on the product branch. When assertions are added
or removed from T-001, the developer updates `T001_GATE_COUNT` in the same commit —
this is the **only hand-maintained token**. T-016 catches any mismatch between the
declared count and the actual DOC-PARITY FAIL block count. Stable under gate
addition: adding one gate increments `actual_count` by exactly 1; the developer bumps
`T001_GATE_COUNT` by 1; no other token changes.

### Decision 4 — T-016 remains a required check, fail-closed

T-016 remains in `bats-full-suite` as a required (not advisory) check. Fail-open
alternatives are rejected on the grounds of BC-5.39.008 v1.6.

The gate's purpose shifts from "story document agrees with bats prose token" to
"declared constant in bats suite agrees with actual assertion count." This is a
stronger invariant: it is mechanically enforceable within the product branch alone
and catches real assertion additions or removals without requiring any cross-branch read.

### Decision 5 — Governance pattern for future coupling gates

Any future bats gate that enforces coupling between a spec document (on
`factory-artifacts`) and an implementation artifact (on the product branch) MUST
follow this pattern:

| If the gate is... | Cross-branch read | Permitted |
|-------------------|-------------------|-----------|
| Spec-validation: validates that spec documents pass format/lint/invariant checks | YES — spec content lives on factory-artifacts by design | Permitted |
| Required coupling check: derives an operand for comparison in a required CI equality assertion | NO — operand must come from the product branch or be runtime-derived from actual executing code | Forbidden in a `required` check |
| Advisory coupling check: cross-branch read in an advisory/optional CI job | YES — advisory jobs may fail-open on absent mount | Permitted with explicit advisory label |

A "coupling gate" that reads from `factory-artifacts` to produce an operand for a
required equality check is prohibited by this ADR. Such a gate MUST be redesigned to
derive the spec-side operand from a product-branch constant (Decision 3 pattern) or
removed from the required check path.

## Rationale

**Why not option (b) (skip-with-warning on story version mismatch) or option (c)
(advisory job)?**
BC-5.39.008 v1.6 ruled that fail-open is never valid for a governance gate that
cannot read its target. Both options are fail-open shapes: (b) passes silently when
versions mismatch; (c) passes the required check even when coupling fails. Both are
rejected on this ruling explicitly.

**Why does removing the cross-branch read not weaken governance?**
T-016's original intent was to catch drift between the spec's stated gate count and
the bats suite's actual count. Under the new design, the declared constant
(`T001_GATE_COUNT`) on the product branch IS the spec-side operand. The developer
who adds a gate updates the constant; T-016 catches any mismatch. This is strictly
stronger than the old design (which could not detect assertions removed from T-001
while both prose tokens still read the same number).

The story document on `factory-artifacts` remains authoritative documentation of
what the spec says. It is no longer an operand in a CI check. An adversarial review
pass catches story drift (where the story says "N gates" but the constant says M).
That is a documentation-review concern, not a CI-gate concern.

**Why are the two findings one rewrite?**
If we derive `actual_count` from executing assertions (H01 fix) but keep
`story_count` as the cross-branch operand (H05 open), the gate still fails whenever
a state-manager burst amends the story's gate count, even if the actual assertions
are unchanged. The cross-branch mount remains the failure vector. A fix that closes
H01 but leaves H05 open does not eliminate the CI outage pattern. The two findings
must be resolved together.

**Why is the product-branch sentinel the right source for the expected count?**
The assertion count is a property of the product branch: it is determined by the
inline DOC-PARITY FAIL blocks that developers write and modify in the bats suite.
The spec document on `factory-artifacts` references that count as documentation.
Reversing this relationship — making the spec document's prose token the source of
truth for a CI equality assertion — inverts the authority chain and creates the
coupling hazard that H05 describes. The product branch owns the assertions; the
product branch must own the expected count.

**Why 24, not 23?**
The comment "Twenty-three independently mutant-proven gates" excluded
boundary-completeness (F-S2104-P19-004) because that assertion's own FAIL message
acknowledges it fires simultaneously with Gate PW-B on CONTROL-D and is therefore
not independently proven. However, a mechanical predicate cannot reproduce this
judgment without inspecting FAIL message prose. The predicate must be a simple,
stable rule: count all `echo "DOC-PARITY FAIL"` lines in the AC-001(a) block. The
literal count is 24. Setting `T001_GATE_COUNT=24` makes the declared value agree
with the mechanical predicate without requiring any exclusion rule. The story count
("twenty-three") and T-001's comment ("twenty-three") must each be updated to
"twenty-four" as part of the T-016 rewrite (route: story-writer for the story;
test-writer for the comment inline with the T-016 rewrite commit).

## Consequences

### Positive

- CI outage pattern eliminated: state-manager bursts to `factory-artifacts` cannot
  red `bats-full-suite` on unrelated product PRs. S-21.04 merge to `develop` does
  not introduce a permanent cross-branch coupling via T-016.
- T-016 detects real assertion additions or removals (H01 closed): the comparison
  is now a runtime-derived count of inline DOC-PARITY FAIL blocks in the AC-001(a)
  section vs. the product-branch sentinel `T001_GATE_COUNT`, not prose token vs.
  prose token.
- T-016 is self-contained on the product branch: it runs correctly when `.factory/`
  is absent (e.g., on a fresh feature worktree without the factory-artifacts mount).
- Governance pattern established (Decision 5): future coupling gates have a clear
  design rule preventing recurrence of the H05 class.

### Negative / Trade-offs

- Drift between the story document's stated gate count and the product-branch
  constant is not caught by T-016. It is caught by adversarial review of the story
  document. This is an acceptable tradeoff: CI gates enforce implementation
  consistency; spec reviews enforce spec accuracy.
- The developer must maintain `T001_GATE_COUNT` in sync with the actual assertions.
  A forgotten update will be caught by T-016, which is the intent.
- Other bats suites that legitimately read from `$FACTORY_ROOT/.factory/` are
  unaffected by this ruling. They validate spec content, which lives on
  `factory-artifacts` by design, and that pattern is confirmed as Permitted under
  Decision 5.

### Status as of v1.1 (2026-07-30)

Proposed. Test-writer implements T-016 rewrite per Decisions 1-3.

Story AC-001 gate count cell must be updated from "twenty-three" to "twenty-four"
to match `T001_GATE_COUNT=24` (route: story-writer). The inline comment in T-001
body that says "Twenty-three independently mutant-proven gates" must also be updated
to "Twenty-four" (route: test-writer, same commit as T-016 rewrite).

BC-6.26.001 does not specify the mechanism of the coupling gate or the gate count;
no BC amendment is required.

## Alternatives Considered

- **Option (a) as-proposed by adversary (move operands to product branch via product-branch file):** This ADR adopts Option (a) with a precise implementation: the product-branch operand is a sentinel comment `T001_GATE_COUNT=N` in the bats suite itself, not a separate file. Both operands then live in a single product-branch file. Accepted.

- **Option (b) skip-with-warning when story version differs:** Fail-open. Rejected under BC-5.39.008 v1.6: a governance gate cannot silently pass when it cannot read its target.

- **Option (c) restrict to advisory job:** Fail-open for the primary required check. Rejected under BC-5.39.008 v1.6. An advisory copy of the coupling check in a separate non-blocking job is not prohibited, but does not replace the required check.

- **Retain cross-branch read, add retry/caching:** Does not eliminate the hazard. State-manager bursts can change the story at any time; retrying with a stale cache would produce false negatives. Rejected.

- **Fix only H01 (runtime-derived count) while keeping cross-branch story read:** Closes H01 but leaves H05 open. CI outage pattern persists whenever the story document's gate count changes. Rejected because the two findings must resolve together.

## Source / Origin

- **F-S2104-P29-H05** — cross-branch operand in required CI check, raised by pass-29
  adversarial review of S-21.04. Root cause: T-016
  (`test_coupling_gate_story_gate_count_matches_bats_count_word` in
  `plugins/vsdd-factory/tests/worktree-identity-preflight.bats`) reads
  `.factory/stories/S-21.04-story-worktree-write-path-discipline.md` (factory-artifacts
  branch) for an equality operand in a `bats-full-suite`-required check.
- **F-S2104-P29-H01** — string-vs-string comparison, neither operand runtime-derived.
  Confirmed by: `grep -rn 'gate_count|gates_total|GATE_COUNT' plugins/vsdd-factory/tests/`
  returning no results other than the test's own name.
- **`test_coupling_gate_story_gate_count_matches_bats_count_word`** in
  `plugins/vsdd-factory/tests/worktree-identity-preflight.bats` — T-016 implementation
  under ruling.
- **`bats-full-suite (linux)` job** in `.github/workflows/ci.yml` — the required CI job
  running `run-all.sh` with `SKIP_SUITES=()` (empty, non-skippable) on every PR to
  `main` or `develop`.
- **BC-5.39.008 v1.6** — product-owner ruling: fail-open is never valid for a
  governance gate that cannot read its target. Governs rejection of options (b) and (c).
- **ADR-027** (factory-artifacts worktree path discipline) — established the canonical
  `.factory/` mount convention; this ADR constrains that convention for required CI gates
  that derive equality-check operands.

### Downstream Routing

| Artifact | Change | Route |
|----------|--------|-------|
| Story `S-21.04-story-worktree-write-path-discipline.md` AC-001 gate count cell | Update from "(twenty-three gates)" to "(twenty-four gates)" to match `T001_GATE_COUNT=24` | story-writer |
| `story-worktree-write-path-discipline.bats` T-001 inline comment | Update "Twenty-three independently mutant-proven gates" to "Twenty-four" (same commit as T-016 rewrite) | test-writer |
| `plugins/vsdd-factory/tests/worktree-identity-preflight.bats` T-016 | Implement Decisions 1-3: remove `$fa_wt` block; add `T001_GATE_COUNT=24` sentinel extraction; count DOC-PARITY FAIL blocks in AC-001(a) section as `actual_count`; assert `actual_count == T001_GATE_COUNT` | test-writer |
| `BC-6.26.001` | No changes required — BC does not specify the coupling gate mechanism or gate count | — |
