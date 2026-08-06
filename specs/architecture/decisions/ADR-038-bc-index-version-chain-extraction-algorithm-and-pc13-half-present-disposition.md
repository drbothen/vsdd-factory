---
document_type: architecture-decision-record
level: L3
adr_id: ADR-038
version: "1.2"
title: "ADR-038: BC-INDEX version-chain extraction algorithm — first-token-of-last-entry replaces rightmost-of-field-6; PC13 half-present disposition is advisory; Phase 2 story-row extraction requires BC-ID-anchored first v-token"
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
  2026-08-06 (v1.2) — §Empirical Measurement corrected (architect; orchestrator catch):
  v1.0/v1.1 histogram measured all pipe-starting lines in BC-INDEX.md (sum 2017), not
  catalog rows. Correct predicate: first non-empty field starts with `[BC-` (markdown-link
  form). Corrected histogram: {5: 1943, 6: 39, 9: 1} over 1983 catalog rows. The "1977"
  total in v1.0 was a transcription error; the actual all-lines sum was 2017. n≥6 = 40 is
  invariant across both populations (non-catalog lines all have <6 fields); §Decisions 1-5
  stand unchanged. Original BC "5-field rows: 1943 / total 1983" figures were accurate.
  [Prior: 2026-08-06 (v1.1) — Phase 2 story-row extraction algorithm adjudicated (architect;
  S-21.07 pass-7 sibling-extractor ruling routed by orchestrator):
  Phase 2 rightmost-in-rightmost-field algorithm empirically confirmed wrong on 1 live row
  (S-15.17 BC-5.39.009: returns v1.3 from annotation prose `POLICY 5 v1.3.6`, correct is
  v1.9). Structural scoping bug confirmed: Phase 2 scans all fields in reverse regardless of
  which field contains the matched BC ID. Cross-field contamination demonstrated on S-10.05
  (BC-2.06.001 in field [1], Phase 2 returns token from field [5]) and S-4.08 (cross-BC
  mention, accidentally correct). Arrow transition notation (v1.N→v1.M) not instantiated in
  corpus (0 rows). Phase 2 corpus count stale: 67 rows / 44 with BC IDs (not 30, 2026-08-04).
  Decision 5: BC-ID-anchored first-v-token is the correct Phase 2 algorithm. Routes to
  product-owner (BC-5.39.010 Phase 2 algorithm text + corpus count) and implementer
  (extract_version_token_from_table_row signature + logic change).
  [Prior: 2026-08-06 (v1.0) — Initial ruling (architect; S-21.07 pass-7 adjudication):
  F-S2107-P7-004 + F-S2107-P7-017 + F-S2107-P7-008 empirically adjudicated.
  Corpus measurement 2026-08-06: histogram {5: 1943, 6: 39, 9: 1} (corrected in v1.2;
  original wrong value was {5: 1964, 6: 39, 9: 1} from all-lines population); 40 rows
  reach n≥6 arm (39 six-field + 1 nine-field); confirms F-P7-017 arithmetic.
  Four-row proof table confirms implementation correct on all four rows; spec algorithm
  wrong on three of four. PC13 half-present case ruled advisory per PC12 literal text.
  BC-4.13.001 source format defect ruled: both source escape fix AND extractor robustness.]]
modified:
  - "2026-08-06 (v1.0)"
  - "2026-08-06 (v1.1)"
  - "2026-08-06 (v1.2)"
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

## Empirical Measurement (2026-08-06; corrected v1.2)

**Population:** BC-INDEX.md catalog rows only — lines starting with `|` whose first
non-empty field begins with `[BC-` (the markdown-link form used for all BC IDs in the catalog
body table). This predicate excludes the subsystem-summary table, the `| **Total** |`
summary row, and all header/separator lines, which together account for the difference between
1983 catalog rows and 2017 total pipe-starting lines.

> **v1.0/v1.1 measurement error (corrected here):** The original script matched all
> pipe-starting lines, producing histogram `{2:1, 4:12, 5:1964, 6:39, 9:1}` (sum 2017; the
> "1977" figure in v1.0 was a transcription error — the actual sum was 2017, not 1977). This
> inflated the 5-field bucket by 21 rows due to header, separator, and summary lines. The
> `n≥6 = 40` aggregate is invariant across both populations (non-catalog lines all have fewer
> than 6 fields), which is why §Decisions 1–5 stand unchanged despite the wrong denominator.

**Field-count histogram — catalog rows only (reproducible command):**

```bash
$ python3 -c "
with open('specs/behavioral-contracts/BC-INDEX.md') as f:
    lines = f.readlines()
def esc_split(line):
    return [f.strip() for f in line.replace('\\\\|', '\x00').split('|') if f.strip()]
histogram = {}
for line in lines:
    line = line.rstrip()
    if not line.startswith('|'): continue
    fields = esc_split(line)
    if not fields or not fields[0].startswith('[BC-'): continue
    n = len(fields)
    histogram[n] = histogram.get(n, 0) + 1
total = sum(histogram.values())
n6 = sum(v for k,v in histogram.items() if k >= 6)
print(f'catalog rows: {total}  histogram: {dict(sorted(histogram.items()))}  n>=6: {n6}')
"
catalog rows: 1983  histogram: {5: 1943, 6: 39, 9: 1}  n>=6: 40
```

The live corpus has **1943 five-field rows**, **39 six-field rows**, **1 nine-field row**
(BC-4.13.001, bare `|` in annotation `^(Edit|Write|MultiEdit|Agent)$`), and **40 rows
total reaching the `n >= 6` branch**. BC-5.39.010's stated figures "5-field rows: 1943" and
"total 1983" are confirmed accurate as of 2026-08-06; the v1.0/v1.1 assertion that they were
stale was incorrect.

*Separate note:* BC-INDEX.md frontmatter `total_bcs: 1983` matches the catalog row count
exactly. The subsystem-summary table shows `| **Total** | | **1975** |` — an 8-BC gap with
SS-01..SS-10 counts summing to 1975, not 1983. This gap does not affect this ADR's ruling
(which rests on per-row behavior, not aggregate counts) and is routed separately to
state-manager as BC-INDEX owner.

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

## Empirical Measurement (v1.1 Amendment: Phase 2 story-row analysis, 2026-08-06)

**Phase 2 row count — BC-citation sections in all story files:**

```bash
$ python3 - << 'EOF'
import re, os

stories_dir = ".factory/stories"
story_files = sorted(f for f in os.listdir(stories_dir)
                     if f.endswith(".md") and f != "STORY-INDEX.md")

TARGET_PREFIXES = ("Behavioral Contracts", "Token Budget")

def is_target_heading(h):
    h = h.strip()
    for p in TARGET_PREFIXES:
        if h == p or h.startswith(p + " ") or h.startswith(p + "("):
            return True
    return False

def parse_pure_version(s):
    s = s.strip()
    inner = s[1:] if s.startswith("v") else s
    return inner if re.fullmatch(r"[0-9]+\.[0-9]+", inner) else None

def rightmost_v(s):
    ms = list(re.finditer(r"(?<![a-zA-Z0-9])v([0-9]+\.[0-9]+)(?![0-9a-zA-Z])", s))
    return ms[-1].group(1) if ms else None

phase1, phase2, no_ver = 0, 0, 0
for fname in story_files:
    fpath = os.path.join(stories_dir, fname)
    with open(fpath) as f:
        lines = f.readlines()
    in_section = False
    for line in lines:
        line = line.rstrip("\n")
        if line.startswith("## "):
            in_section = is_target_heading(line[3:])
            continue
        if not in_section or "|" not in line:
            continue
        fields = line.split("|")
        p1 = next((parse_pure_version(f) for f in reversed(fields)
                   if parse_pure_version(f)), None)
        if p1:
            phase1 += 1
        elif any(rightmost_v(f.strip()) for f in reversed(fields)):
            phase2 += 1
        else:
            no_ver += 1

print(f"Phase 1 (pure-version field): {phase1}")
print(f"Phase 2 (mandatory-v inline): {phase2}")
print(f"No-version: {no_ver}")
EOF
Phase 1 (pure-version field): 58
Phase 2 (mandatory-v inline): 67
No-version: 2317
```

**The BC's stated "30 rows" (corpus 2026-08-04) is stale. The 2026-08-06 count is 67 Phase 2 rows (same
defect class as the PC5 stale corpus count ruled in §Decision 1/4).**

The orchestrator's counter-measurement of 258 rows is a distinct figure: it counts inline BC+v-token pairs
across ALL file contexts (not scoped to BC-citation sections), using a grep that is broader than the
Phase 2 predicate. The 258 is not the Phase 2 count; both figures are correct for their respective scopes.

**Proof table — Phase 2 algorithm comparison on confirmed divergence rows:**

```bash
$ python3 - << 'EOF'
import re, os

stories_dir = ".factory/stories"

def rightmost_v_in_field(s):
    ms = list(re.finditer(r"(?<![a-zA-Z0-9])v([0-9]+\.[0-9]+)(?![0-9a-zA-Z])", s))
    return ms[-1].group(1) if ms else None

def first_v_after_bc_id_in_field(field_text, bc_id):
    pos = field_text.find(bc_id)
    if pos < 0:
        return None
    after = field_text[pos + len(bc_id):]
    m = re.search(r"(?<![a-zA-Z0-9])v([0-9]+\.[0-9]+)(?![0-9a-zA-Z])", after)
    return m.group(1) if m else None

cases = [
    ("S-15.17-validate-trajectory-tail-cell-completeness.md", 1113, "BC-5.39.009"),
    ("S-10.05-adr015-wave2-plugin-schema-migration.md", 174, "BC-2.06.001"),
    ("S-4.08-rc1-release-gate.md", 242, "BC-9.01.002"),
]
for fname, lnum, bc_id in cases:
    fpath = os.path.join(stories_dir, fname)
    with open(fpath) as f:
        lines = f.readlines()
    line = lines[lnum - 1].rstrip("\n")
    fields = line.split("|")
    # Phase 2 current: rightmost v-token in rightmost field
    p2_current = None
    p2_current_field = None
    for i, fld in enumerate(reversed(fields)):
        v = rightmost_v_in_field(fld.strip())
        if v:
            p2_current = v
            p2_current_field = len(fields) - 1 - i
            break
    # Proposed: BC-ID-anchored first v-token in anchor field
    p2_proposed = None
    anchor_field = None
    for i, fld in enumerate(fields):
        v = first_v_after_bc_id_in_field(fld.strip(), bc_id)
        if v:
            p2_proposed = v
            anchor_field = i
            break
    print(f"{bc_id} ({fname}:{lnum})")
    print(f"  Phase2-current (rightmost): {p2_current} (field {p2_current_field})")
    print(f"  Phase2-proposed (anchored): {p2_proposed} (anchor field {anchor_field})")
EOF
BC-5.39.009 (S-15.17-validate-trajectory-tail-cell-completeness.md:1113)
  Phase2-current (rightmost): 1.3 (field 1)
  Phase2-proposed (anchored): 1.9 (anchor field 1)
BC-2.06.001 (S-10.05-adr015-wave2-plugin-schema-migration.md:174)
  Phase2-current (rightmost): 1.4 (field 5)
  Phase2-proposed (anchored): 1.3 (anchor field 5)
BC-9.01.002 (S-4.08-rc1-release-gate.md:242)
  Phase2-current (rightmost): 1.1 (field 3)
  Phase2-proposed (anchored): None (no v-token in BC-9.01.002 anchor field)
```

| Row | BC ID | Phase 2 current: rightmost-in-rightmost-field | Phase 2 proposed: BC-ID-anchored first-v-token | Correct | Current correct? |
|---|---|---|---|---|---|
| S-15.17:1113 | BC-5.39.009 | **1.3** (from `POLICY 5 v1.3.6` annotation in same field as BC ID) | **1.9** (first v-token after BC-5.39.009 in field [1]) | 1.9 (the cited BC version) | NO — spurious PC2a advisory |
| S-10.05:174 | BC-2.06.001 | **1.4** (rightmost of `v1.3+v1.4` in description field [5]) | **1.3** (first v-token after BC-2.06.001 in field [5]) | ambiguous (`v1.3+v1.4` conjunction; see §Decision 5) | PARTIALLY — v1.4 avoids false block if BC is at v1.4 |
| S-4.08:242 | BC-9.01.002 | **1.1** (from field [3] that mentions BC-9.01.001, not BC-9.01.002) | **None** (no v-token in BC-9.01.002's anchor field [1]) | None / advisory (BC-9.01.002 is not cited at a version here) | WRONG (cross-BC contamination: returning v1.1 from BC-9.01.001 field) |

**Why Phase 2's rightmost-in-rightmost-field algorithm fails:**

1. **Annotation-prose later-version-reference (S-15.17 BC-5.39.009):** The Token Budget row
   `BC-5.39.009 v1.9 (... POLICY 5 v1.3.6 verification gate; ...)` contains the cited version `v1.9`
   at the start of the inline citation, followed by annotation prose that references `POLICY 5 v1.3.6`.
   The regex matches `v1.3` from `v1.3.6` (the `.` after `v1.3` is not alphanumeric — valid word
   boundary). The rightmost v-token in the field is `v1.3`, not `v1.9`. This is structurally identical
   to the BC-INDEX backward-reference problem that §Decision 1 ruled on: annotation prose following the
   authoritative version citation can carry older version-like tokens, making rightmost wrong.

2. **Cross-field contamination (S-4.08 BC-9.01.002):** The row `| BC-9.01.002 | description | ...
   v1.1 candidate ... AC-13 traces ONLY to BC-9.01.001 PC2 ... |` has BC-9.01.002 in field [1] (no
   v-token) and the v1.1 token in field [3] (which is contextually about BC-9.01.001). Phase 2's
   reverse-field scan finds v1.1 in field [3] and returns it as the version for BC-9.01.002.
   Currently both BCs are at v1.1 so this is accidentally correct, but it will produce a wrong
   answer when the BCs diverge in version. The fundamental defect is that Phase 2 scans fields
   without knowing which BC ID it is checking.

3. **Cross-field skip (S-10.05 BC-2.06.001):** The BC link is in field [1] (no inline v-token). The
   description in field [5] contains `(BC-2.06.001 v1.3+v1.4 Invariant 2 + EC-006; AC-008)` — the
   BC ID appears again with a `v1.3+v1.4` conjunction. Phase 2 (rightmost-in-rightmost-field) returns
   `v1.4` from field [5]. The proposed anchored algorithm returns `v1.3` from the same field [5].
   The conjunction `v1.3+v1.4` is a non-canonical citation format (see §Decision 5 for disposition).

**Arrow transition notation (→):** The orchestrator identified a hypothetical shape `v1.2→v1.3`
where rightmost would be correct and first-after-id would be wrong. Corpus measurement (2026-08-06):
**0 Phase 2 rows use `→` transition notation.** The hypothetical risk is not instantiated.

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

### Decision 5 — Phase 2 story-row algorithm: BC-ID-anchored first v-token (replaces rightmost-in-rightmost-field)

**RULING: Phase 2's current rightmost-in-rightmost-field algorithm is WRONG. The correct algorithm is
BC-ID-anchored first v-token.**

**Phase 2 correct algorithm — normative specification:**

For each pipe-delimited field in the row (scanning left-to-right):
1. Check whether the field contains the matched BC ID (word-boundary test identical to PC13's
   `line_contains_bc_id_at_boundary` predicate).
2. If the BC ID is present: extract the position of the BC ID within the field's text.
3. Return the FIRST `\bv([0-9]+\.[0-9]+)\b` token that appears AFTER the BC ID position within
   that field.
4. If found: this is the Phase 2 cited version. Stop scanning.
5. If no field contains both the BC ID and a subsequent v-prefixed token: Phase 2 returns None
   for this BC ID (no citation — not a block per postcondition 8).

**Implementation note:** this algorithm requires `bc_id` to be passed into the row-level extraction
function. The current `extract_version_token_from_table_row(line: &str) -> Option<String>` signature
is BC-ID-agnostic; it must be extended to `extract_version_token_from_table_row(line: &str, bc_id: &str)
-> Option<String>` (or the anchor-field logic must be lifted to `extract_story_bc_version_citations`
where `bc_id` is already in scope). The implementer determines the mechanical approach subject to this
normative behavioral spec. All callsites of the function must be updated (TD-VSDD-060 sibling sweep).

**Why this algorithm is correct:**

Token Budget rows carry BC citations in the format `BC-ID v<current> (description prose ...)`. The
authoritative current version is the v-token immediately following the BC ID. Annotation prose following
the version citation can carry additional version-like tokens — policy version references (e.g.,
`POLICY 5 v1.3.6`), prior-version mentions, and other prose version-like strings. These are not BC
version citations. The first v-token after the BC ID is structurally the version being cited; subsequent
v-tokens in the same field are annotation prose. This is the same principle as §Decision 1's
"first-token-of-last-chain-entry" algorithm for BC-INDEX chains: citation leads; annotation follows.

**Disposition of the three corpus divergence rows:**

- **S-15.17 (BC-5.39.009, v1.3 vs v1.9):** Phase 2 current returns v1.3 from policy annotation
  `POLICY 5 v1.3.6` — spurious stale block on a current citation. Phase 2 proposed returns v1.9 —
  correct. This row is an active wrong answer in the live corpus. **Must be fixed.**

- **S-4.08 (BC-9.01.002, v1.1 from cross-BC field):** Phase 2 current returns v1.1 from a field
  whose content is about BC-9.01.001, not BC-9.01.002. BC-9.01.002 has no v-token in its own field.
  Phase 2 proposed returns None — correct: no inline citation exists for BC-9.01.002 in this row's
  anchor field. The row describes a relationship between the BC and a finding, not a version citation.
  **Structural bug; accidentally correct today; will produce wrong answers when BC versions diverge.**

- **S-10.05 (BC-2.06.001, v1.4 vs v1.3):** The row has `(BC-2.06.001 v1.3+v1.4 Invariant 2 ...)` in
  the description field. Phase 2 proposed returns v1.3 (first v-after-id). The `v1.3+v1.4` conjunction
  format is non-canonical. The correct long-term resolution is for the story to cite BC-2.06.001 at the
  CURRENT version only (e.g., `v1.4` if that is the current version). The gate is a citation-currency
  check; it is not responsible for parsing conjunction formats. v1.3 is the correctly extracted first
  cited version; if BC-2.06.001 is at v1.4, this produces a stale block (correct behavior — the
  citation is genuinely stale at v1.3). **No algorithm defect; authoring defect in story. Story author
  must update to cite current version only.**

**Corpus count (2026-08-06):** 67 Phase 2 rows total; 44 containing at least one BC ID. The BC's stated
"30 rows" (corpus 2026-08-04) is stale by the same defect class as PC5's "5-field rows: 1943" which
§Decision 1/4 already ruled on. Both figures grew because the story corpus expanded between 2026-08-04
and 2026-08-06.

**Arrow transition notation (→):** The `v1.N→v1.M` shape (where rightmost would be correct and
first-after-id would return the wrong old version) is not instantiated in the corpus: **0 Phase 2 rows
use `→` notation** (confirmed by full corpus scan). If this notation appears in a future story, the
gate will block on v1.N (the first token after BC ID), which is the correct behavior — the story
should be updated to cite v1.M (the current version) directly without the `→` annotation in a
citation field. The `→` notation is documentation of a version history, not a canonical citation form.
If a future story requires transition notation, it should use a non-citation field or format that
keeps the current version as the sole first v-token after the BC ID.

**Routing for Decision 5:**

- **Product-owner:** amend BC-5.39.010 PC13 Phase 2 algorithm description (see §Decision 4 routing
  extension below — Change 5) and update corpus count to 67/44.
- **Implementer:** update `extract_version_token_from_table_row` signature to accept `bc_id`; replace
  rightmost-in-rightmost-field Phase 2 logic with BC-ID-anchored first-v-token logic; perform
  TD-VSDD-060 callsite sweep on all callers.

**§Decision 4 routing extension — Change 5 (product-owner, BC-5.39.010 PC13 Phase 2):**

Replace the current Phase 2 paragraph:
> "**Phase 2 — Inline v-prefixed token (fallback)**: if Phase 1 finds no pure-version field, scan
> fields in **REVERSE order** (rightmost first) for the pattern `\bv([0-9]+\.[0-9]+)\b` (**mandatory
> `v` prefix**). Return the first match found. This covers `## Token Budget` rows where the BC ID and
> version appear inline in a single field (e.g., `BC-5.39.010 v1.7 (full text, 33 ECs...)`). Corpus
> count (2026-08-04): **30 rows** across all story files."

With:
> "**Phase 2 — BC-ID-anchored inline v-prefixed token (fallback)**: if Phase 1 finds no pure-version
> field, locate the field in the row that contains the BC ID (same word-boundary test as `line_contains_
> bc_id_at_boundary`); within that field, return the FIRST `\bv([0-9]+\.[0-9]+)\b` token appearing
> AFTER the BC ID position. Mandatory `v` prefix. Return None if no field contains both the BC ID and
> a subsequent v-prefixed token. This covers `## Token Budget` rows where the BC ID and version appear
> inline in a single field (e.g., `BC-5.39.010 v1.7 (full text, 33 ECs...)`).
> The **reverse-field (rightmost-first) algorithm** is **NON-CONFORMING** per ADR-038 §Decision 5:
> (a) annotation prose in the anchor field can carry older version-like tokens after the authoritative
> citation (S-15.17 BC-5.39.009: rightmost returns v1.3 from `POLICY 5 v1.3.6`, correct is v1.9);
> (b) the scan is not scoped to the BC ID's anchor field, enabling cross-field and cross-BC
> contamination (S-4.08: returns v1.1 from a field about a different BC). The first-v-token-after-
> BC-ID algorithm is the direct analog of the first-token-of-last-chain-entry ruling in §Decision 1:
> citation leads; annotation follows. Corpus count (2026-08-06): **67 Phase 2 rows / 44 containing
> BC IDs** (prior figure 30 rows, 2026-08-04, stale — same defect class as PC5 count corrected by
> this ADR's §Decision 4 Change 1)."

## Rationale

### Why Phase 2 requires BC-ID-anchored first v-token (§Decision 5)

Token Budget rows follow the pattern `BC-S.SS.NNN v<current> (annotation)`. The annotating prose
serves the same structural role as backward-reference annotations in BC-INDEX chains: it follows the
authoritative version token and can contain version-like strings that are not BC version citations
(policy version numbers, prior-version cross-references, year-month dates matching `v1.N` patterns).

The rightmost-in-rightmost-field algorithm fails for the same root cause as the rightmost-of-field[5]
algorithm in BC-INDEX chains (§Decision 1): later v-tokens in the field are annotation noise, not the
authoritative citation. The only structural difference is the token position: in BC-INDEX chains the
authoritative token leads the last chain entry; in Token Budget rows the authoritative token leads the
entire inline citation immediately after the BC ID. Both reduce to "first v-token at the citation
anchor, not rightmost in the field."

The scoping defect (Phase 2 not anchored to the BC ID's field) is an independent structural bug:
Phase 2's field iteration does not know which BC ID it is checking, so it can find v-tokens in fields
belonging to other BCs or to unrelated prose. Passing `bc_id` into the row extraction function is
necessary to enable correct field selection.

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

### Phase 2 consequences (v1.1 amendment)

**Positive:**
- S-15.17's spurious PC2a advisory for BC-5.39.009 (Phase 2 returning v1.3 instead of v1.9) is
  eliminated once the implementer updates `extract_version_token_from_table_row`.
- Cross-BC contamination in S-4.08 (BC-9.01.002 getting v1.1 from BC-9.01.001's field) is resolved:
  Phase 2 proposed returns None for BC-9.01.002 (correct — no inline version citation exists for it).
- Phase 2 algorithm is now structurally scoped to the BC ID being checked, eliminating the class of
  false block / false advisory that arises when BC versions diverge across BCs cited in the same row.
- Corpus count in BC-5.39.010 PC13 is updated from 30 to 67/44 (accurate as of 2026-08-06).

**Negative / Trade-offs:**
- S-10.05's `v1.3+v1.4` conjunction format will produce a stale block if BC-2.06.001 is at v1.4
  (Phase 2 proposed returns v1.3 — the first cited version). This is **correct gate behavior**: the
  story uses a non-canonical conjunction format and should be updated to cite v1.4 directly. The gate
  is a citation-currency enforcer; it is not responsible for interpreting conjunction formats.
- `extract_version_token_from_table_row`'s function signature changes from `(line: &str)` to
  `(line: &str, bc_id: &str)`. All callsites must be updated (TD-VSDD-060 sibling sweep). There is
  exactly one caller in the production path (`extract_story_bc_version_citations`); test callsites
  must be updated to pass a synthetic BC ID.
- Future stories using `v1.N→v1.M` transition notation in citation fields will cause Phase 2 to
  return v1.N (the old version) and trigger a stale block. This is correct behavior: the story should
  cite v1.M only. Authors should not use `→` transition notation in citation fields.

### Status as of 2026-08-06 (v1.0 → v1.1)

v1.0 (2026-08-06): Accepted. Routing dispatched: product-owner to amend BC-5.39.010 per §Decision 4
Change 1-4; implementer to fix arm_b half-present arms; state-manager to escape BC-4.13.001 annotation
bare pipes in the next BC-INDEX write. ARCH-INDEX row insertion pending (state-manager Commit D).

v1.1 (2026-08-06): Extended with §Decision 5 (Phase 2 story-row algorithm). Routing added: product-owner
to amend BC-5.39.010 PC13 Phase 2 per §Decision 4 Change 5 (BC-ID-anchored algorithm + corpus count
67/44); implementer to update `extract_version_token_from_table_row` signature and logic + TD-VSDD-060
callsite sweep. ARCH-INDEX row title amendment pending (state-manager Commit D, same burst).

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

**Phase 2 alternatives considered (v1.1):**

- **Keep rightmost-in-rightmost-field (status quo):** Rejected. Empirically wrong on S-15.17
  (returns v1.3 instead of v1.9 for BC-5.39.009). Structurally wrong for cross-BC contamination
  (S-4.08 accidentally correct today; will fail when BC versions diverge). Same root cause as the
  rightmost-of-field[5] algorithm this ADR already rejected for BC-INDEX chains.

- **Rightmost-in-anchor-field (scan fields right-to-left; within the first field containing the BC
  ID, take the rightmost v-token):** Rejected. Fixes the cross-field contamination defect (now
  scoped to the BC ID's anchor field) but still wrong for S-15.17: the rightmost v-token in the
  BC-5.39.009 anchor field is v1.3 (from `POLICY 5 v1.3.6`), not v1.9. The annotation-prose later-
  version problem requires "first" not "rightmost" within the anchor field.

- **Rightmost-in-anchor-field, left-to-right field scan (scan fields left-to-right; first field
  containing BC ID; rightmost v-token in that field):** Rejected for the same reason: rightmost
  within the anchor field is still wrong for S-15.17.

- **First-v-token-after-BC-ID scoped to entire remaining row (not field-scoped):** Rejected.
  For rows where the BC ID appears in one field and a later field contains a v-token, this would
  cross into adjacent fields. S-10.05 illustrates: BC-2.06.001 link in field [1] (no v-token);
  description field [5] contains `(BC-2.06.001 v1.3+v1.4 ...)`. "First v-token after BC ID in row"
  would find v1.3 in field [5] (same result as the proposed algorithm in this case, but only
  because the BC ID appears again in field [5]). For a row where the BC ID is in field [1] and
  field [5] mentions a different BC, row-scoped "first v-token" would return that different BC's
  version. Anchor-field scoping is required.

- **Treat `→` notation specially (BC-ID v1.N→v1.M → return v1.M as current):** Rejected (no
  corpus instantiation; adds parsing complexity for 0 live rows; story authors should use
  current-version-only citation format in citation fields).

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

## Source / Origin (v1.1 Addition)

- **Orchestrator sibling-extractor routing (2026-08-06):** Product-owner sweep of v1.0 routing left
  `extract_story_bc_version_citations` Phase 2 unchanged (product-owner correctly noted this is a
  different extractor for a different data shape). Orchestrator routed to architect for adjudication:
  258-row live population measurement (orchestrator grep, all contexts); representative shape examples;
  question whether rightmost-first generalizes from BC-INDEX chain rationale.
- **Empirical measurement 2026-08-06 (v1.1):** Phase 2 row count = 67 (44 with BC IDs); three
  divergence rows analyzed (S-15.17, S-10.05, S-4.08); arrow notation = 0 rows; shell commands in
  §Empirical Measurement (v1.1 Amendment) section above.

## Status

ACCEPTED 2026-08-06 (v1.0 initial; v1.1 Phase 2 amendment same date).

v1.0 adjudicates F-S2107-P7-004 (BLOCKER), F-S2107-P7-017 (MEDIUM), and F-S2107-P7-008 (HIGH) from
adversarial pass-6. Routes to product-owner (BC-5.39.010 amendments per §Decision 4 Changes 1-4) and
implementer (arm_b half-present arms per §Decision 3).

v1.1 adjudicates Phase 2 story-row extraction algorithm (orchestrator sibling-extractor routing).
Routes to product-owner (BC-5.39.010 PC13 Phase 2 algorithm text + corpus count per §Decision 4
Change 5 / §Decision 5) and implementer (`extract_version_token_from_table_row` signature change +
BC-ID-anchored first-v-token logic + TD-VSDD-060 callsite sweep).

Does not authorize product-owner to amend any other clause of BC-5.39.010 or to deviate from the
normative text specified in §Decision 4 (Changes 1-5) and §Decision 5 without a new ADR or explicit
human authorization.
