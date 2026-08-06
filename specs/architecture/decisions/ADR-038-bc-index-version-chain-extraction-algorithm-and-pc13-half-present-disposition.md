---
document_type: architecture-decision-record
level: L3
adr_id: ADR-038
version: "1.0"
title: "ADR-038: BC-INDEX version-chain extraction algorithm — first-token-of-last-entry replaces rightmost-of-field-6; PC13 half-present disposition is advisory"
status: accepted
date: 2026-08-06
producer: architect
timestamp: 2026-08-06T00:00:00Z
deciders:
  - architect
subsystems_affected: [SS-05]
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
last_amended: |-
  2026-08-06 (v1.0) — Initial ruling (architect; S-21.07 pass-7 adjudication):
  F-S2107-P7-004 + F-S2107-P7-017 + F-S2107-P7-008 empirically adjudicated.
  Corpus measurement 2026-08-06: field-count histogram {5: 1964, 6: 39, 9: 1}; 40 rows
  reach n≥6 arm (39 six-field + 1 nine-field); confirms F-P7-017 arithmetic.
  Four-row proof table confirms implementation correct on all four rows; spec algorithm
  wrong on three of four. PC13 half-present case ruled advisory per PC12 literal text.
  BC-4.13.001 source format defect ruled: both source escape fix AND extractor robustness.
modified:
  - "2026-08-06 (v1.0)"
---

# ADR-038: BC-INDEX version-chain extraction algorithm — first-token-of-last-entry replaces rightmost-of-field-6; PC13 half-present disposition is advisory

## Context

Adversarial review pass-6 of story S-21.07 raised two coupled findings:

**F-S2107-P7-004 (BLOCKER):** BC-5.39.010 v1.12 PC5 and PC6 normatively mandate
**rightmost**-token extraction from the **6th non-empty field** of a BC-INDEX body-table row.
The shipped implementation (`extract_bc_index_version_state` in `arm_a1.rs`) implements
**first-token-of-last-chain-entry** extracted from a **join of fields[5..]**. The BC was
amended twice during the pass-6 fix burst and neither extraction-direction clause was
touched. Per CLAUDE.md §12, when code and spec disagree the spec wins — making the
implementation non-conforming. The human explicitly routed this to the architect for
adjudication rather than allowing a mechanical "spec wins, rewrite the code" resolution.

**F-S2107-P7-017 (MEDIUM, coupled):** BC-5.39.010 v1.12 PC5 claims the escape-aware split
"yields exactly 6" fields for all rows in the Version(v) state, and quotes a corpus figure
of "6-field rows: 40." Empirical measurement reveals the escape-aware split yields **9**
non-empty fields for BC-4.13.001, because the v1.16 annotation
`^(Edit|Write|MultiEdit|Agent)$` contains four unescaped `|` characters that become
field boundaries. The aggregate count "40 rows reach the n≥6 arm" is correct (39
six-field + 1 nine-field), but the "exactly 6" and "6-field rows: 40" claims are false.

A third coupled finding was also routed for adjudication:

**F-S2107-P7-008 (HIGH):** PC13's v1.11 split into PC13a (`B2==B3`) and PC13b (`B2≠B3`)
left the half-present case — exactly one of {B2, B3} present and differing from B1, the
other absent — with no normative disposition. The implementation blocks in this case, but
PC12 as written says "B2 or B3 absent: advisory + Continue" unconditionally. Two live
corpus instances exist (S-18.11, S-18.12 — catalog rows present, no blockquote entries).

## Empirical Measurement (2026-08-06)

**Field-count histogram — escape-aware split on all `|`-starting lines in BC-INDEX.md:**

```
python3 field_count_histogram.py specs/behavioral-contracts/BC-INDEX.md
→ {2: 1, 4: 12, 5: 1964, 6: 39, 9: 1}
→ total rows: 1977 pipe-starting lines
→ rows with n≥6 non-empty fields: 40 (39 six-field + 1 nine-field)
→ nine-field row: BC-4.13.001 (bare | in annotation ^(Edit|Write|MultiEdit|Agent)$)
```

The live corpus has **1964 five-field rows** (vs. BC's stated 1943) and **1 nine-field row**.
The aggregate count of 40 rows reaching the `n >= 6` branch is confirmed. The "exactly 6"
claim and "5-field rows: 1943" figure are stale — the BC-INDEX grew between the v1.8
corpus measurement (2026-08-04) and this measurement (2026-08-06).

**Four-row proof table — both algorithms against all rows the adversary cited:**

| BC ID | Frontmatter version (normalized) | SPEC algorithm: rightmost v-token in field[5] | IMPL algorithm: first v-token of last chain entry (fields[5..] joined) | Spec vs frontmatter | Impl vs frontmatter |
|---|---|---|---|---|---|
| BC-3.08.001 | 1.24 | **1.23** (from backward-ref `(promoted v1.23 D-839)` in last annotation entry) | **1.24** (from `BC-3.08.001 v1.24 already active` — first v-token of last entry) | MISMATCH → false PC2a advisory | MATCH → Continue ✓ |
| BC-7.03.079 | 1.5 | **1.4** (from backward-ref `[prior: v1.4]` at end of entry) | **1.5** (first v-token of only entry) | MISMATCH → false PC2a advisory | MATCH → Continue ✓ |
| BC-4.13.001 | 1.18 | **1.16** (field[5] cut off at bare `\|` before `Write` in regex annotation) | **1.18** (fields[5..] joined, last entry = `v1.18 (...)`, first v-token) | MISMATCH → false PC2a advisory | MATCH → Continue ✓ |
| BC-5.24.006 | 1.3 | **1.3** | **1.3** | MATCH → Continue ✓ | MATCH → Continue ✓ |

All four rows measured against the live BC-INDEX (2026-08-06). Shell commands captured
for reproducibility:

```bash
$ python3 -c "
import re
bc_ids = ['BC-3.08.001','BC-7.03.079','BC-4.13.001','BC-5.24.006']
with open('specs/behavioral-contracts/BC-INDEX.md') as f:
    content = f.read()

def esc_split(line):
    return [f.strip() for f in line.replace('\|','\x00').split('|') if f.strip()]

def rightmost_v(text):
    ms = list(re.finditer(r'(?<![a-zA-Z0-9])v([0-9]+\.[0-9]+)(?![0-9a-zA-Z])', text))
    return ms[-1].group(1) if ms else None

def first_v(text):
    m = re.search(r'(?<![a-zA-Z0-9])v([0-9]+\.[0-9]+)(?![0-9a-zA-Z])', text)
    return m.group(1) if m else None

def first_of_last_entry(cell):
    entries = [s for s in cell.split('\x00') if s.strip()]
    return first_v(entries[-1]) if entries else None

for bc_id in bc_ids:
    for line in content.split('\n'):
        if not line.startswith('|'): continue
        fields = esc_split(line)
        if not fields: continue
        first = fields[0]
        if first == bc_id or first.startswith('['+bc_id+']'):
            n = len(fields)
            if n < 6: break
            field5 = fields[5]
            joined = '|'.join(fields[5:])
            print(f'{bc_id}: n={n} spec={rightmost_v(field5)} impl={first_of_last_entry(joined)}')
            break
"
BC-3.08.001: n=6 spec=1.23 impl=1.24
BC-7.03.079: n=6 spec=1.4 impl=1.5
BC-4.13.001: n=9 spec=1.16 impl=1.18
BC-5.24.006: n=6 spec=1.3 impl=1.3
```

**Why the BC spec algorithm fails for three rows:**

Version chain entries are ordered oldest-to-newest and follow this format:
`v1.N annotation_prose [optional_backward_ref_to_v1.N-1]`

The **rightmost** v-token in the full cell is NOT reliably the current version because:

1. **Backward references in annotation prose** (BC-3.08.001, BC-7.03.079): annotation
   entries commonly end with `(promoted v1.N-1 D-NNN)` or `[prior: v1.N-1 ...]`
   cross-references. The rightmost v-token in the cell is then from the backward reference,
   not from the current entry header.

2. **Bare-pipe fragmentation** (BC-4.13.001): annotation text may contain unescaped `|`
   characters (e.g., `^(Edit|Write|MultiEdit|Agent)$`). The escape-aware split does not
   protect against bare (not backslash-prefixed) pipes. Field[5] is cut off mid-annotation,
   leaving the most-recent chain entries in fields[6..8] invisible to a field[5]-only scan.

The **first-token-of-last-chain-entry** algorithm is correct because:

1. Chain entries lead with their own version number as the first v-token. Annotation prose
   and backward references appear later in the entry text.

2. The `fields[5..].join("|")` reassembly reconstructs the full cell content regardless of
   how many phantom field boundaries bare pipes create.

3. Together, these two steps correctly extract the authoritative current version from any
   conformant BC-INDEX row in the live corpus.

## Decisions

### Decision 1 — Algorithm verdict: SPEC is factually wrong; implementation is correct

**RULING: (a) — the spec must be amended to match the implementation algorithm.**

The implementation's `first-token-of-last-chain-entry` + `fields[5..].join("|")` algorithm
is empirically correct for all four corpus rows. The BC spec's `rightmost-token-of-6th-field`
algorithm produces spurious PC2a advisories for three of four rows. CLAUDE.md §12 says "the
spec wins" for code-vs-spec conflicts where the question is purely implementation correctness
— but §12 also says "only the human can authorize spec amendment to match code." The human
has explicitly authorized this adjudication by routing the question here. This ruling
constitutes the authorization required by §12.

**The implementation MUST NOT be realigned to the spec's rightmost algorithm.** Doing so
would reintroduce the four bugs the pass-6 fix burst corrected (F-P6-019b/c/d). The correct
path is BC-5.39.010 PC5/PC6 spec amendment.

### Decision 2 — Corpus format defect: both source fix AND extractor robustness warranted

BC-4.13.001's BC-INDEX row contains unescaped `|` characters inside the v1.16 annotation
`^(Edit|Write|MultiEdit|Agent)$`. By Markdown table convention, pipe characters inside table
cells must be escaped as `\|`. The annotation text has bare `|` characters that violate this
convention and fragment field[5] across nine fields.

**Two actions are both warranted:**

1. **Source fix (state-manager):** Escape the bare pipes in BC-4.13.001's BC-INDEX row
   annotation. The text `^(Edit|Write|MultiEdit|Agent)$` in the v1.16 annotation must become
   `^(Edit\|Write\|MultiEdit\|Agent)$`. This makes the row conform to Markdown table
   conventions and eliminates the nine-field fragmentation. State-manager must apply this
   fix when performing the next BC-INDEX write that touches this row.

2. **Extractor robustness (no change required):** The `fields[5..].join("|")` approach in
   `extract_bc_index_version_state` handles bare pipes in any future annotation without
   requiring source-level escaping. This is defense-in-depth — the fix-at-source resolves
   BC-4.13.001's specific defect; the `join` handles any future occurrence. Removing the
   `join` in favor of `fields[5]` alone after fixing the source defect would leave the
   extractor brittle to the next occurrence. Retain `fields[5..].join("|")`.

### Decision 3 — PC13 half-present case: advisory + Continue (consistent with PC12)

**RULING:** The half-present case — exactly one of {B2, B3} present and differing from B1,
the other absent — MUST be treated as **advisory + Continue**, consistent with PC12's
unconditional "B2 or B3 absent → advisory + Continue" rule.

**Rationale:**

PC12 states: "Arm B1 — B2 or B3 absent: `host::log_warn` advisory + `HookResult::Continue`."
This is written as an unconditional inclusive-or: if EITHER B2 OR B3 is absent, the
disposition is advisory. The half-present case — one site present, one absent — satisfies
this predicate because B3 is absent (or B2 is absent in the mirror case).

The POLICY 3 ordering rationale for PC12 and PC13a applies here: state-manager writes both
the catalog row and the blockquote in the same STORY-INDEX.md commit. At the instant a
story file is written (the Arm B1 PostToolUse trigger), STORY-INDEX.md has not been touched
at all. The half-present state — catalog row updated in a prior commit but blockquote never
added — is a pre-existing data-quality issue (not a burst-ordering artifact), but the
correct response is still advisory + Continue because we cannot distinguish "midway through
an in-progress burst" from "permanently missing blockquote entry" at PostToolUse trigger
time. Blocking at story-write time for a STORY-INDEX state that the story-write itself
didn't cause produces a self-lock on S-18.11 and S-18.12 with no burst-ordering escape.

The implementation's current block behavior for `(Some(b2), None)` and `(None, Some(b3))`
contradicts PC12 as written. This is a BC-implementation divergence that must be resolved
in the implementer's favor — advisory + Continue — not by making PC12's "or" conditional.

**The two live instances** (S-18.11 catalog `c45c0fc` / no blockquote; S-18.12 catalog
`345086c` / no blockquote) are the precise counterexamples that caused F-S2107-P7-002 to
identify this defect as live. Their correct long-term remediation is for state-manager to
add their blockquote entries to STORY-INDEX.md, not for the hook to block indefinitely.

### Decision 4 — Routing: precise spec amendment text and implementation fix

**Product-owner must amend BC-5.39.010 with the following exact changes:**

**Change 1: PC5 "Escape-aware splitting" paragraph — correct the factual claims.**

Replace the paragraph currently reading:
> "A naive `|` split on a version-chain row produces 15+ fields instead of 6; the escape-aware
> split yields **exactly 6**. Corpus verification (2026-08-04): ... → `5-field rows: 1943 /
> 6+-field rows: 40 / total: 1983`."

With:
> "A naive `|` split on a version-chain row produces 15+ fields instead of 6 for rows with
> escaped-pipe separators. The escape-aware split yields **6 non-empty fields for most rows**
> in the Version(v) state, but **MAY yield more** when the version-chain annotation text
> contains unescaped `|` characters (e.g., regex annotations like
> `^(Edit\|Write\|MultiEdit\|Agent)$` that were recorded with bare `|`). The
> `fields[5..].join("|")` reassembly step is REQUIRED to reconstruct the complete version-chain
> cell before token extraction. Corpus measurement (2026-08-06): `5-field rows: 1964 /
> 6-field rows: 39 / 9-field rows: 1 (BC-4.13.001) / total rows with n≥6: 40`."

**Change 2: PC5 Version(v) bullet — replace extraction direction.**

Replace the extraction clause in the `Version(v)` bullet:
> "Extract the **latest (rightmost)** such token (mandatory `v` prefix — all real version-chain
> tokens use `v` prefix). Exactly **40 of 1,983** rows are in this state (corpus 2026-08-04)."

With:
> "Extract the current version using the **first-token-of-last-chain-entry algorithm** (see
> §Decision 4 of ADR-038 for the rationale and four-row proof): (1) join all non-empty fields
> from index 5 onward with `|` to reconstruct the complete version-chain cell, accounting for
> bare `|` characters in annotation text; (2) split the reconstructed cell on `\x00` (the
> escape sentinel substituted from `\|` by the escape-aware split) to isolate chain entries;
> (3) take the LAST non-empty entry — the most-recent entry in the chain; (4) extract the FIRST
> `\bv([0-9]+\.[0-9]+)\b` token from that entry. The first v-token in a chain entry is the
> authoritative current version; subsequent v-tokens in the same entry are annotation prose
> (backward references like `(promoted v1.23)` or `[prior: v1.4]`). The **rightmost-token-of-
> field[5]** algorithm is **NON-CONFORMING** — it produces spurious PC2a advisories for three of
> four corpus rows with annotation prose or bare-pipe fragmentation (empirically demonstrated
> 2026-08-06, ADR-038 §Empirical Measurement). 40 rows reach the `n≥6` arm in the current
> corpus (corpus 2026-08-06)."

**Change 3: PC6 — replace the `Version(v)` route extraction clause.**

Replace the text in PC6 reading:
> "`Version(v)` route: the version token is the LAST (rightmost) `\bv([0-9]+\.[0-9]+)\b` match
> in the 6th column's cell content, representing the current version in the chain; it is
> normalized by stripping the leading `v`; both values (frontmatter `version:` and extracted
> token) compared as case-sensitive decimal strings after normalization (postconditions 1-2).
> Note: the 6th column may contain multiple version tokens separated by `\|` (e.g.,
> `v1.3 \| v1.4 \| v1.5`); the rightmost token is always the current."

With:
> "`Version(v)` route: the version token is extracted by the first-token-of-last-chain-entry
> algorithm (ADR-038 §Decision 1): join non-empty fields[5..] with `|`, split on `\x00`
> (escape sentinel from `\|`), take the last non-empty entry, extract the FIRST
> `\bv([0-9]+\.[0-9]+)\b` token. The result is normalized by stripping the leading `v`; both
> values (frontmatter `version:` and extracted token) compared as case-sensitive decimal strings
> after normalization (postconditions 1-2). Note: the 6th column (and beyond, for rows with
> bare-pipe annotation fragmentation) contains version chain entries separated by `\|`
> (rendered as `\x00` after escape-aware split); the FIRST v-token of the LAST entry is
> always the current. The rightmost-of-cell algorithm is NON-CONFORMING per ADR-038."

**Change 4: PC12/PC13 — add explicit half-present sub-case.**

Expand PC12 or add PC13c to cover the half-present case, with this normative text:
> "**Half-present case (PC12 extension, ADR-038 §Decision 3)**: exactly one of {B2, B3} is
> present AND differs from B1; the other is absent. Disposition: `host::log_warn` advisory +
> `HookResult::Continue`. Rationale: the inclusive-or of PC12 ("B2 or B3 absent") covers this
> case — the absent site cannot be verified, and the present-but-differing site may reflect a
> mid-burst state where only one STORY-INDEX update has completed. Blocking produces a self-lock
> on stories with missing blockquote entries (e.g., S-18.11, S-18.12) that have no burst-ordering
> escape. The long-term remedy is state-manager adding the missing blockquote entries, not a
> hook block. Implementation MUST NOT block for `(Some(b2), None)` or `(None, Some(b3))` cases."

**Implementer must make one change:**

In `arm_b::run_arm_b1_with_index_result`, the `(Some(b2), None)` and `(None, Some(b3))` match
arms currently push a `Violation` when `b2 != story_hash` / `b3 != story_hash`. Change these
to emit advisory + Continue regardless of the hash comparison (consistent with PC12/PC13c):
the absent-site cases are advisory by definition; the present-but-different condition
cannot distinguish burst-ordering from stale data at trigger time.

## Rationale

### Why the implementation algorithm is correct and the spec must change

The production version-chain cell format appends entries ordered oldest-to-newest, with each
entry structured as `v1.N (annotation prose possibly referencing v1.N-1)`. Two independent
phenomena make the rightmost-token-of-field algorithm unacceptably fragile:

**Backward-reference annotations (BC-3.08.001, BC-7.03.079):** The last chain entry in a
version-chain cell frequently records operational events like auto-promotions or prior-version
cross-references. These entries are structured as `(2026-07-16 D-848: ... BC-3.08.001 v1.24
already active (promoted v1.23 D-839 S-19.05))` — the entry begins with a date, not a version
number, and the annotation contains backward references to v1.23. The rightmost v-token in the
entire cell is then `v1.23` from the backward reference, not `v1.24`. This is not an edge case:
it is the standard POL-14 auto-promotion and pass-record annotation pattern and appears in
multiple rows.

**Bare-pipe fragmentation (BC-4.13.001):** Regex annotations inside version-chain entries
legitimately contain `|` as an alternation operator. These are not `\|` escaped-pipe sequences;
they are normal prose inside an annotation cell that happens to contain regex syntax. The
escape-aware split (which only converts `\|` → `\x00`) does not protect against these. The
result is that field[5] is cut off mid-annotation, and the genuine current version (v1.18 in
the most-recent entry) ends up in fields[7..8] where the rightmost-of-field-5 algorithm cannot
see it.

The `first-token-of-last-chain-entry` + `fields[5..].join("|")` algorithm directly addresses
both phenomena: (a) it correctly identifies the last entry in the chain regardless of whether
that entry leads with a version number or with annotation prose, because it uses the chain's
structural separator (`\x00` from `\|`) not token order; and (b) it reconstructs the full cell
content before applying that separator, so bare pipes in annotation text do not fragment the
chain entries.

### Why CLAUDE.md §12 "spec wins" does not preclude this ruling

CLAUDE.md §12 says the spec wins for code-vs-spec conflicts, and that only the human authorizes
spec amendment to match code. The human exercised that authorization by routing this finding
here for adjudication. The function of this ADR is precisely that authorization. The corpus
evidence demonstrates unambiguously which algorithm produces correct results (four rows, three
wrong under spec, zero wrong under implementation). A mechanical "spec wins, realign the code"
resolution without adjudication would have reintroduced four bugs from the pass-6 fix burst.

### Why the half-present case must be advisory

The POLICY 3 ordering rationale for PC12/PC13a is determinative. State-manager writes both
catalog row and blockquote in the same STORY-INDEX commit. At the PostToolUse instant of a
story file write, STORY-INDEX has not been touched in the current burst — neither catalog nor
blockquote has been updated yet. The "one present, one absent" state — if it results from the
current burst's write ordering — would appear as B2=catalog (from the prior state of
STORY-INDEX, which was not yet touched) and B3=absent (blockquote not yet written). This is
indistinguishable from a data-quality defect (blockquote never added) at trigger time. Blocking
in this case creates an unescapable self-lock for S-18.11 and S-18.12 where the only remedy
(adding the blockquote entry) is itself blocked by the gate. PC12's unconditional
advisory-for-absent-site rule is the correct resolution.

## Consequences

### Positive

- BC-5.39.010 PC5/PC6 will accurately describe the algorithm the implementation actually uses,
  eliminating the F-S2107-P7-004 BLOCKER.
- The three-of-four spurious PC2a advisories for BC-3.08.001, BC-7.03.079, and BC-4.13.001
  will be eliminated once the spec amendment is propagated to a new WASM build.
- BC-4.13.001's BC-INDEX row annotation, once the bare pipes are escaped by state-manager,
  will produce 6 non-empty fields (not 9), resolving F-S2107-P7-017's corpus-claim defect at
  the source.
- The `fields[5..].join("|")` robustness mechanism protects against future bare-pipe
  annotations in any row without requiring per-row source fixes.
- S-18.11 and S-18.12 story writes will no longer be blocked by the half-present case once the
  implementer corrects the arm_b half-present arms from violation to advisory.

### Negative / Trade-offs

- Product-owner must amend PC5, PC6, and PC12/add PC13c in BC-5.39.010 — a version bump to
  v1.13 (or higher) is required. The BC-INDEX body-table row must be updated in the same burst
  per POLICY 14 leg 5.
- The existing corpus corpus-verification block in PC5 (`5-field rows: 1943 / 6+-field rows: 40`)
  must be updated to reflect the 2026-08-06 measurement. This is a routine corpus-count
  maintenance change; it does not affect the algorithm.
- The half-present advisory means that a genuinely corrupt STORY-INDEX state — one site written
  with a wrong value, the other absent — will emit only advisory at story-write time. The
  Arm B2 trigger (STORY-INDEX.md write) catches this case when the STORY-INDEX is next edited.
- State-manager has a pending mechanical fix to BC-4.13.001's BC-INDEX row annotation (escaping
  bare pipes) that must be bundled into the next BC-INDEX row update for that BC.

### Status as of 2026-08-06 (v1.0)

Accepted. Routing dispatched: product-owner to amend BC-5.39.010 per §Decision 4 Change 1-4;
implementer to fix arm_b half-present arms; state-manager to escape BC-4.13.001 annotation
bare pipes in the next BC-INDEX write. ARCH-INDEX row insertion pending (state-manager Commit D).

## Alternatives Considered

- **Option (b) — realign implementation to rightmost-of-field-5 per current BC:** Rejected.
  Produces three spurious PC2a advisories on live corpus rows (BC-3.08.001, BC-7.03.079,
  BC-4.13.001). Would reintroduce bugs F-P6-019b/c/d that the pass-6 burst already fixed.
  The empirical four-row proof is dispositive.

- **Option (c) — rightmost of reassembled cell (fields[5..].join("|"), then rightmost v-token):**
  Rejected. Fixes the bare-pipe fragmentation issue (BC-4.13.001 would return `v1.18`) but
  still fails on backward-reference annotations (BC-3.08.001 returns `v1.23`, BC-7.03.079
  returns `v1.4`). Two of four corpus rows still yield false PC2a advisories.

- **Fix source row only (escape BC-4.13.001's bare pipes) and keep rightmost-of-field-5:**
  Rejected. Eliminates the BC-4.13.001 fragmentation but still fails for BC-3.08.001 and
  BC-7.03.079 due to backward-reference annotations. The backward-reference pattern is endemic
  to the POL-14 auto-promotion chain entry format and will recur in future rows.

- **Block on half-present case (status quo in implementation):** Rejected. Contradicts PC12's
  literal "B2 or B3 absent → advisory" text. Creates an unescapable self-lock for S-18.11 and
  S-18.12 with no burst-ordering remedy path. The POLICY 3 ordering rationale applies equally
  to the single-absent-site case.

- **Mandate that all BC-INDEX chain annotations be written without backward references or regex
  content:** Rejected. This would require retroactive cleanup of all existing rows containing
  `(promoted v1.N-1)` or `[prior: v1.N-1]` annotations, and the `first-token-of-last-entry`
  algorithm already handles this case correctly without requiring annotation format discipline.
  The format convention ship has already sailed at 40 version-chain rows.

## Source / Origin

- **F-S2107-P7-004 (BLOCKER):** Adversary pass-6 of S-21.07 — PC5/PC6 rightmost-algorithm
  claim vs. implementation first-token-of-last-entry; spec amended twice without touching
  extraction direction; corpus evidence table with four rows.
- **F-S2107-P7-017 (MEDIUM):** Adversary pass-6 of S-21.07 — BC-4.13.001 nine-field row;
  "escape-aware split yields exactly 6" factually wrong; `fields[5..].join("|")` in the code
  exists precisely for this phenomenon.
- **F-S2107-P7-008 (HIGH):** Adversary pass-6 of S-21.07 — PC13 half-present case undefined;
  implementation blocks; PC12 says advisory; two live corpus instances (S-18.11, S-18.12).
- **Empirical measurement 2026-08-06:** Field-count histogram {5: 1964, 6: 39, 9: 1}; four-row
  algorithm comparison table (BC-3.08.001, BC-7.03.079, BC-4.13.001, BC-5.24.006); shell
  commands in §Empirical Measurement section above.
- **ADR-035:** Cross-site correspondence three-tier architecture (normative twin BC-5.39.010).
- **ADR-037:** Input-hash volatile-inputs exclusion (motivates PC40 and Class B arm context).

## Status

ACCEPTED 2026-08-06. Adjudicates F-S2107-P7-004 (BLOCKER), F-S2107-P7-017 (MEDIUM), and
F-S2107-P7-008 (HIGH) from adversarial pass-6. Routes to product-owner (BC-5.39.010
amendments per §Decision 4) and implementer (arm_b half-present arms per §Decision 3). Does
not authorize product-owner to amend any other clause of BC-5.39.010 or to deviate from the
normative text specified in §Decision 4 above without a new ADR or explicit human authorization.
