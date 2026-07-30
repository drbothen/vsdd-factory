---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-07-30T00:00:00Z
phase: v1.0-feature-engine-discipline-pass-1
cycle: v1.0-feature-engine-discipline-pass-1
inputs:
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/policies.yaml
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-28.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-29.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-30.md
input-hash: "fbd1185"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
extracted_from: null
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: draft
introduced: v1.0-feature-engine-discipline-pass-1
modified:
  - "2026-07-30"
  - "2026-07-30 (v1.2)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.010
section: "5.39"
last_amended: "2026-07-30 (v1.2) — Registry entry corrected: tools = [...] array replaced with tool = \"^(Edit|Write|MultiEdit)$\" regex string (field name singular + MultiEdit added; POLICY 13 ESCAPE-SCOPE-PARITY). Fuel-exhaustion note added to Gate Specifications per ADR-035 §Decision 5. BC-version-pin datum-copy ruling added to Postconditions §Part A Arm2. (product-owner.) [Prior: 2026-07-30 (v1.1) — Part A Arm2 (story-file-side trigger) added; advisory rationales made explicit for every advisory arm; Class D tokenizer namespace-exclusion list added (D-, S-, BC-, VP-, R-, L-, ADR-, EC-, NFR-, ASM-, FM-); EC-024 rationale corrected; Class A coverage-gap routing replaced with correctly-sized latency-gap explanation; Invariant 11 (fabricated vs stale hash provenance) added; EC-026/027/028/029 added; Gate Spec updated with run_part_a_arm2; VP table extended to 17 entries. (product-owner; coordinator review.) [Prior: 2026-07-30 (v1.0) — Initial authoring (product-owner; pre-pass-30 fix-burst). BC-5.39.010 allocated after BC-5.39.009. input-hash d248fc3 per hook-authoritative marketplace binary. lifecycle_status: draft.]]"
---

# BC-5.39.010: validate-cross-site-correspondence WASM hook MUST block on stale BC-INDEX version-cite after a BC frontmatter bump (Class A Arm1), stale story body BC-table and Token Budget citations after a story edit (Class A Arm2), STORY-INDEX three-way input-hash inequality (Class B), and frontmatter version↔last_amended text-prefix mismatch and modified[] non-monotonicity (Class E); MUST emit advisory on finding-ID namespace format violations in Closes/Refs lines (Class D); Class C count/enumeration parity is not mechanically checkable in WASM

## Description

The `validate-cross-site-correspondence` WASM hook enforces value-equivalence invariants across
multiple artifact sites that hold the same semantic datum. Six consecutive adversarial review
passes (passes 28-30) produced the same class of finding: a BC or story was correctly updated
at its primary site, but one or more secondary sites holding the same datum were left stale.
In all observed failures **the secondary site was present — its value was stale**. A
presence-only gate would have passed every one of the six failures. Every invariant in this BC
compares values across sites, not the existence of sites.

This BC specifies four mechanically-gateable classes (A, B, D, E) via a single WASM hook named
`validate-cross-site-correspondence`, and one honest gap (Class C). All four gated arms fire
PostToolUse (Edit/Write); none prevent a write; all signal after the write has completed. Class A
has two arms: Arm1 fires on a BC file write and checks BC-INDEX.md; Arm2 fires on a story file
write and checks the story's own BC version citations against each cited BC's current frontmatter
version. Both arms close the same recurring failure pattern from complementary directions with no
directory enumeration required. Part B covers Class B (three-way input-hash equality per POLICY 18
D-923). Part D covers Class D (finding-ID namespace format advisory in Closes/Refs lines). Part E
covers Class E (frontmatter version↔last_amended text-prefix parity + modified[] monotonicity per
POLICY 14 leg 4 / POLICY 17).

Class C is documented in §Honest Gap. It is NOT deferred arbitrarily — it is declared ungatable
by a generic WASM hook for specific structural reasons, and routed to a Rust workspace integration
test per POLICY 21.

## Preconditions

### Part A — Arm1 (BC-File Trigger): BC-INDEX Version-Cite Correspondence (Class A)

1. A PostToolUse Edit/Write event has fired on a file satisfying ALL of:
   - `Path::new(file_path).file_name()` matches `^BC-[0-9]+\.[0-9]+\.[0-9]+\.md$` (regexp on
     basename — path-component-strict per BC-5.39.008 precedent)
   - The file is under `.factory/specs/behavioral-contracts/` (checked via
     `components().any(|c| c.as_os_str() == "behavioral-contracts")`)
   - The file is NOT `BC-INDEX.md` itself (basename guard)
2. The hook reads the BC file via `host::read_file` and extracts the YAML frontmatter `version:`
   field. Strip surrounding quotes. Result: decimal string like `1.18`.
3. The BC identifier is derived from the basename by stripping `.md` (e.g., `BC-5.39.010`).
4. The hook reads `.factory/specs/behavioral-contracts/BC-INDEX.md` via `host::read_file` with
   `max_bytes = 1048576` (1 MiB) and `timeout_ms = 3000`. The 1 MiB cap prevents the
   META-LEVEL-24 silent-truncation false-green class.
5. The hook locates the body-table row for the BC ID post-frontmatter (pattern `^\| \[<id>\]` or
   `^\| <id> \|`). The last non-empty pipe-delimited column is the version cell.
6. Version cell is normalized by stripping a leading `v`. Both values compared as case-sensitive
   strings after normalization.

### Part A — Arm1 read failure semantics

7. `host::read_file` for the BC file returns any HostError: `HookResult::block_with_fix(...)`
   naming the error class and path. Primary target, PostToolUse — no fail-open path
   (BC-5.39.008 v1.6 fail-closed ruling).
8. `host::read_file` for BC-INDEX.md returns `HostError::NotFound`: `host::log_warn` advisory +
   `HookResult::Continue` (legitimate bootstrap).
   `HostError::CapabilityDenied` or any capability-class error on BC-INDEX.md:
   `HookResult::block_with_fix(...)` naming the error. Sandbox misconfiguration on a secondary
   target is blocking.

### Part A — Arm2 (Story-File Trigger): Story BC-Citation Currency (Class A)

9. A PostToolUse Edit/Write event has fired on a file satisfying ALL of:
   - `Path::new(file_path).file_name()` matches `^S-[0-9]+\.[0-9]+.*\.md$` (basename)
   - The file is under directory component `stories` (path-component-strict)
   - The file is NOT `STORY-INDEX.md` itself (basename guard)
10. The hook reads the story file via `host::read_file` with `max_bytes = 524288` and
    `timeout_ms = 3000`. Extracts the YAML frontmatter `behavioral_contracts:` sequence.
    If `behavioral_contracts:` is absent or empty: Arm A2 skips entirely (`HookResult::Continue`).
11. For each BC ID in `behavioral_contracts:`, the hook derives the BC file path deterministically:
    - Strip `BC-` prefix from the ID; split on `.`; take first component as the major section
      integer S (e.g., `BC-5.39.010` → S=5)
    - Path: `.factory/specs/behavioral-contracts/ss-<S zero-padded to 2 digits>/<BC-ID>.md`
    - Example: `BC-5.39.010` → S=5 → `ss-05` → `.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md`
    - Example: `BC-6.26.001` → S=6 → `ss-06` → `.factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md`
    - No directory enumeration required; path is mechanically derived from the BC ID.
12. For each BC, the hook reads the BC file via `host::read_file` with `max_bytes = 524288` and
    `timeout_ms = 3000`. Extracts the BC's `version:` from frontmatter.
13. Within the story file content, the hook finds all version citations for the given BC ID.
    A version citation is any table row (contains `|`) that:
    - Contains the BC ID as an exact token (`\bBC-S\.SS\.NNN\b`)
    - Contains a version token matching `\bv([0-9]+\.[0-9]+)\b`
    The hook extracts the LAST version-like token per row (version is conventionally the last
    column). All matching rows are checked: Token Budget table rows, body BC-table rows, etc.
    If no version-citing row is found for a given BC ID: skip that BC (not all BCs are explicitly
    version-tracked in every story's visible cells — absence is not a violation here).

### Part A — Arm2 read failure semantics

14. `host::read_file` for the story file returns any HostError: `HookResult::block_with_fix(...)`
    — primary target, fail-closed.
15. `host::read_file` for a BC file returns `HostError::NotFound`: `host::log_warn` advisory for
    that specific BC ID + continue checking remaining BCs.
    `HostError::CapabilityDenied` on any BC file: `HookResult::block_with_fix(...)` naming the BC
    path — sandbox misconfiguration is blocking regardless of whether the target is primary.

### Part B — Three-Way Input-Hash Equality (Class B)

Three sites that must hold identical values for each story S-NNN:

- **Site B1**: story frontmatter `input-hash:` field
- **Site B2**: STORY-INDEX.md body-table catalog row, `input-hash` token
- **Site B3**: STORY-INDEX.md aggregation blockquote `S-NNN=HHHHHHH`

### Arm B1 Preconditions (story file write)

16. PostToolUse on a story file (basename `S-[0-9]+\.[0-9]+.*\.md`, component `stories`, NOT
    `STORY-INDEX.md`).
17. Story ID extracted from basename: `^(S-[0-9]+\.[0-9]+)`.
18. `host::read_file` reads the story file; extracts `input-hash:` from frontmatter.
    If `input-hash:` is absent or null: Arm B1 skips entirely.
19. `host::read_file` reads `.factory/stories/STORY-INDEX.md` with `max_bytes = 1048576` and
    `timeout_ms = 3000`.
20. Catalog row located (post-frontmatter). `input-hash` token extracted:
    `\binput-hash\s+([0-9a-f]{7,40})\b`. If absent from row, Site B2 is absent.
21. Aggregation blockquote (`^> ` lines) scanned. Pattern `\b<id>=([0-9a-f]{7,40})\b` extracts
    Site B3. If story ID absent from blockquote, Site B3 is absent.

### Arm B2 Preconditions (STORY-INDEX.md write)

22. PostToolUse on `STORY-INDEX.md` (basename guard + component `stories`).
23. `host::read_file` reads STORY-INDEX.md with `max_bytes = 2097152` and `timeout_ms = 5000`.
24. All `S-NNN.MM=HHHHHHH` pairs extracted from aggregation blockquote region.
25. For each story ID in the blockquote set, the catalog row is located and `input-hash` token
    extracted. No individual story file reads are performed in Arm B2.

### Part B — read failure semantics

26. Arm B1: story file HostError → block (primary). STORY-INDEX.md `HostError::NotFound` →
    advisory + Continue. `HostError::CapabilityDenied` → block.
27. Arm B2: STORY-INDEX.md IS the primary target. Any HostError → block.

### Part D — Finding-ID Namespace Format (Class D)

28. PostToolUse on a file satisfying ANY of:
    - Basename `burst-log.md` AND component `cycles`
    - Basename `lessons.md` AND component `cycles`
    - Basename `INDEX.md` AND component `cycles`
29. `host::read_file` with `max_bytes = 2097152` and `timeout_ms = 5000`.
30. **Scope-limited extraction** (frozen-provenance exclusion by structural position):
    - `burst-log.md`: last H2 section (text from last `^## ` heading through end-of-file)
    - `lessons.md`: last `^L-EDP1-[0-9]+-[0-9]+:` anchor block; if absent, last 200 lines
    - `INDEX.md`: `## Adversarial Reviews` section (between that heading and the next `^## `)
31. Extract all lines matching `^Closes:\s*(.+)$` and `^Refs:\s*(.+)$` from the scoped region.
32. For each Closes/Refs line: tokenize by comma and whitespace. A token is classified as
    **finding-like** if and only if BOTH conditions hold:
    - It matches shape `[A-Za-z][A-Za-z0-9-]*[0-9]+` (starts with letter, ends with digit)
    - It does NOT start with any known-safe namespace prefix:
      `D-`, `S-`, `BC-`, `VP-`, `R-`, `L-`, `ADR-`, `EC-`, `NFR-`, `ASM-`, `FM-`
    Finding-like tokens that do NOT start with `F-` are flagged for advisory.

### Part D — read failure semantics

33. `HostError::CapabilityDenied` on the cycle artifact: block. `HostError::NotFound`:
    advisory + Continue. `HostError::Timeout` or other: block.

### Part E — Frontmatter Internal Parity (Class E)

34. PostToolUse on a file satisfying ANY of:
    - Under `.factory/specs/behavioral-contracts/ss-*/` with basename `BC-*.md` (not `BC-INDEX.md`)
    - Under `.factory/specs/verification-properties/ss-*/` with basename `VP-*.md`
    - Under `.factory/stories/` with basename `S-*.md`
35. `host::read_file` with `max_bytes = 524288` and `timeout_ms = 3000`. If content does not begin
    with `---`: `HookResult::Continue` immediately.
36. **version: extraction**: extract `version:` YAML field; strip quotes. Result: e.g., `1.6`.
37. **last_amended: outermost version extraction**: apply regex
    `^\d{4}-\d{2}-\d{2}\s+\(v([0-9]+(?:\.[0-9]+)*)\)` at CHARACTER POSITION 0 of the field value.
    Captures the outermost (active) version. `[Prior:` chains appear later in the string and are
    excluded structurally by the positional anchor. If regex fails to match: `host::log_warn`
    advisory + `HookResult::Continue` (do NOT block on unparseable format).
38. **modified: extraction**: extract YAML sequence under `modified:`. Strip annotation suffixes
    (e.g., `" (v1.3)"`); compare date strings lexicographically. If absent or empty: skip E2.
39. Any HostError on the primary target file: `HookResult::block_with_fix(...)` — fail-closed.

## Postconditions

### Part A Arm1 postconditions

1. BC ID found in BC-INDEX body table with matching version cell (normalized): `HookResult::Continue`.
2. BC ID found with DIFFERENT version cell:
   `HookResult::block_with_fix(...)`:
   `"validate-cross-site-correspondence [Class A Arm1]: BC-INDEX.md body-table row for <id> cites
   v<index_version> but frontmatter version: is \"<fm_version>\" — stale cite. Update BC-INDEX
   body-table same-burst per POLICY 14 leg 5."`.
3. BC ID NOT in BC-INDEX body table AND frontmatter `version:` is `"1.0"`:
   `host::log_warn` advisory + `HookResult::Continue`.
   **Advisory rationale**: a v1.0 BC not yet in BC-INDEX is the expected state immediately after
   writing a new BC, before the INDEX update tool call completes in the same burst. Blocking would
   make correct BC authoring impossible — the BC file is always written before the INDEX row. This
   is NOT a "partial check = advisory" rationale; it is a "blocking causes systematic false
   positives in correct authoring bursts" rationale. When version > 1.0 and the row is absent, the
   hook blocks (postcondition 4) — so advisory is selective, not the default for all absent rows.
4. BC ID NOT in BC-INDEX body table AND `version:` > `"1.0"`:
   `HookResult::block_with_fix(...)` — version > 1.0 means the BC was previously registered; an
   absent row is a structural fault, not bootstrap ordering.
5. Multiple simultaneous BC writes: each Write is a separate hook invocation; violations are NOT
   accumulated across invocations.

**Class A latency gap (not a coverage gap)**: Arm1 fires on every BC write and catches stale
BC-INDEX immediately. Arm2 fires on every story write and catches stale story citations when the
story is edited. The remaining gap is **latency**: if a BC bumps version in a burst that does not
also touch the story, Arm2 does not fire until the story is next edited. This is not a coverage
gap — in all six observed Class A failures (passes 28-30), the story WAS edited in the same burst
as the BC bump, meaning Arm2 would have caught every observed failure. The empirically relevant
trigger path is fully covered. The POLICY 14 leg 5 obligation to update story citations same-burst
as a BC bump remains binding on the author; Arm2 provides the gate for that case.

**BC-version-pin design ruling (product-owner, 2026-07-30)**: BC-version cells in STORY-INDEX and
all BC-version citations in story bodies carry **datum-copy semantics** — they must match the BC's
current frontmatter `version:` field at the time of any story edit. They do NOT carry
reconciliation-marker semantics (a frozen record of "which BC version the ACs were last reconciled
against that could legitimately lag"). Rationale: (1) POLICY 14 leg 5's "update same-burst"
obligation is incoherent under reconciliation-marker semantics — a reconciliation marker could
intentionally lag, making a same-burst update obligation meaningless; (2) the six Class A failures
across passes 28-30 were correctly identified and closed as defects — retroactive reclassification
as "correct-but-lagging markers" has no new evidentiary basis and would reopen closed findings; (3)
STORY-INDEX.md language "BCs cells reconciled with source frontmatter" describes the act of
updating values to match their source of truth (datum maintenance), not the preservation of a
frozen point-in-time marker. **Implication for Arm A2**: Arm A2 correctly blocks on any
present-but-stale version citation in a story body. No change to Arm A2 blocking logic or behavior
is warranted by this ruling.

### Part A Arm2 postconditions

6. For a given BC ID in `behavioral_contracts:`: all version-citing rows in the story agree with
   the BC's current frontmatter `version:` (normalized): `HookResult::Continue` for that BC.
7. One or more version-citing rows show a version that does NOT match BC frontmatter version:
   `HookResult::block_with_fix(...)`:
   `"validate-cross-site-correspondence [Class A Arm2]: story <story_id> cites <bc_id> at
   v<cited> (in <location>) but BC frontmatter version: is \"<bc_version>\". Update story
   citation same-burst per POLICY 14 leg 5."`.
   All mismatching BCs reported in one combined block message (cascade — do not stop on first).
8. No version-citing row found for a given BC ID: skip that BC, `HookResult::Continue`. The hook
   does NOT block on missing citations; only on present-but-stale citations.
9. `behavioral_contracts:` absent or empty: Arm A2 skips, `HookResult::Continue`.
10. BC file returns `HostError::NotFound` for a cited BC: advisory per PC15; continue checking
    remaining BCs. `HookResult::Continue` for that BC.
    **Advisory rationale**: a cited BC may be retired, may have a non-standard path, or may be a
    new BC added in the same burst whose file exists but the path derivation encounters an edge
    case. `HostError::NotFound` is a known-legitimate transient. `HostError::CapabilityDenied` on
    a BC file blocks (PC15) — sandbox misconfiguration is never legitimate, regardless of target
    type.

### Part B postconditions

11. Arm B1 — all three sites present and equal: `HookResult::Continue`.
12. Arm B1 — B2 or B3 absent: `host::log_warn` advisory + `HookResult::Continue`.
    **Advisory rationale**: a story with `input-hash:` populated but not yet in STORY-INDEX.md is
    the expected state when a story is first authored or when STORY-INDEX.md is updated in a later
    tool call within the same burst. This arm is NOT "advisory because the check is partial." The
    hook blocks when present values disagree (postcondition 13) — it blocks on what it CAN verify.
    Advisory fires only when absence is the anomaly, and absence has a known-legitimate
    interpretation (correct burst ordering). Blocking on absence would cause systematic false
    positives in correct new-story authoring bursts.
13. Arm B1 — B2 or B3 present but differs from B1:
    `HookResult::block_with_fix(...)`:
    `"validate-cross-site-correspondence [Class B]: Story <id> input-hash three-way mismatch:
    frontmatter=<h1> STORY-INDEX-catalog=<h2 or absent> STORY-INDEX-blockquote=<h3 or absent>.
    All three present sites must agree. Update per POLICY 18 (D-923)."`.
14. Arm B2 — catalog and blockquote agree for all blockquote stories: `HookResult::Continue`.
15. Arm B2 — catalog ≠ blockquote for any story: `HookResult::block_with_fix(...)` reporting ALL
    mismatching stories in one message (cascade).

**Fabricated vs stale distinction — not resolved by this hook**: Class B detects cross-site
*inconsistency*. It cannot distinguish a stale hash (previously valid, needs sweep) from a
fabricated hash (never a valid computed value). See Invariant 11 for the remediation protocol.

### Part D postconditions

16. All finding-like tokens on Closes/Refs lines start with `F-`: `HookResult::Continue`.
17. Any finding-like token does NOT start with `F-`: `host::log_warn` per token + `HookResult::Continue`.
    Message: `"validate-cross-site-correspondence [Class D] advisory: non-canonical finding-ID
    token '<token>' on line '<line>' in <section> of <file>. Finding IDs must start with 'F-'.
    Verify this is not a phantom ID or retracted reference."`.
    **Advisory rationale**: blocking on syntactic format alone, even after namespace exclusions,
    causes false positives on tokens from unknown future namespaces added in later cycles. The
    namespace exclusion list (PC32) catches known-legitimate non-finding tokens (`D-NNN`, `S-NNN`,
    etc.). The residual advisory population is tokens that look like finding IDs (right shape, not
    in any known-safe namespace) but don't start with `F-`. Blocking these would prevent forward-
    compatibility as new ID classes are introduced. This is NOT "partial check = advisory"; it is
    "the namespace registry cannot be closed, so blocking cannot be made false-positive-free."
18. `HostError::CapabilityDenied` on the cycle artifact: block.

**Gap — semantic existence**: whether a cited `F-` ID exists in an adversary pass record is
infeasible in WASM (unbounded scan). Routed to Rust workspace test per POLICY 21.

### Part E postconditions

19. E1 version match: `HookResult::Continue`.
20. E1 mismatch: `HookResult::block_with_fix(...)`:
    `"validate-cross-site-correspondence [Class E1]: frontmatter version: \"<ver_fm>\" does not
    match last_amended: outermost text-prefix \"(v<ver_la>)\". Update last_amended: text-prefix
    to (v<ver_fm>) per POLICY 14 leg 4 / POLICY 17."`.
21. E2 ascending: `HookResult::Continue`.
22. E2 non-monotonic: first out-of-order pair: `HookResult::block_with_fix(...)`.
23. Combined E1 + E2 violations: ONE combined block enumerating both.

### Cross-arm combination

When a single file write triggers multiple arms, all run independently. Violations from all arms
are combined into one `HookResult::block_with_fix(...)`. Part D advisories are logged regardless
of block state.

## Honest Gap — Class C (Count/Enumeration/Word-Token Parity)

Three observed failure shapes from passes 29-30:

1. **Numeric assertion vs enumeration length** (pass-29 H01, pass-30 H04): prose claims "N gates"
   while the enumeration has N-1 items.
2. **Two count-word tokens disagreeing** (pass-30 H05): `bats:643` `Twenty-four` vs `bats:786`
   `twenty-three`.
3. **Asserted count vs runtime-derived count**: coupling gate compared two literal strings, not a
   mechanical count.

**Why NOT gatable in a generic WASM hook**:

(a) English cardinal parsing (`"twenty-three"` → 23) requires a non-closed lookup table; coupling
    a generic hook to this creates maintenance drag.
(b) The structural anchor for "the count assertion" vs "the enumeration" is file-specific. No
    generic regex reliably distinguishes the asserted count from line numbers or unrelated integers.
(c) Three-way correlation requires semantic region identification with no machine-readable separator.

**Recommended alternative (POLICY 21-compliant)**: Rust workspace integration test with per-file
fixtures. Parameterized test: (i) count enumeration items by structural anchor; (ii) extract
stated count by stable anchor; (iii) assert equality. Flagged for architect to commission as a
follow-on story.

**Class D semantic existence gap**: `F-` ID existence check requires enumerating
`adv-cycle-pass-*.md` files. Same routing: Rust workspace test or CI scan.

## Invariants

1. The hook NEVER writes to any file. All arms are read-only post-write validators.
2. The hook fires PostToolUse only — writes succeed before the hook fires; the hook signals after.
3. All file-path matching uses path-component-strict guards. Raw string `ends_with` is forbidden.
4. **Fail-closed for primary targets**: any HostError on the PostToolUse trigger file →
   `HookResult::block_with_fix(...)`. No fail-open path for the primary target.
   (BC-5.39.008 v1.6 fail-closed ruling; Canonical Principle + TD-VSDD-059.)
5. **Selective fail-open for secondary targets on NotFound only**: BC-INDEX.md, STORY-INDEX.md,
   and BC files cited in story `behavioral_contracts:` return advisory + Continue on
   `HostError::NotFound` (bootstrap/ordering). `HostError::CapabilityDenied` on any secondary
   target is blocking — sandbox misconfiguration is never a legitimate state.
6. **Class D is advisory-only, never blocking**: see postcondition 17 for the specific reason.
7. **No cross-arm suppression**: all arms run independently; violations combine into one block;
   Part D advisories are logged regardless of block state.
8. **Class B Arm B2 cascade**: all STORY-INDEX mismatches reported in ONE combined block.
9. **is_char_boundary() guard**: byte-index slicing on extracted strings MUST use
   `is_char_boundary()` checks where multi-byte UTF-8 is possible (BC-5.39.008 inv-11).
10. **POLICY 21 compliance**: no `.sh` scripts. All gating uses WASM plugin or Rust workspace
    tests. Class C and the Class D existence-check gap are routed to Rust workspace tests.
11. **Stale vs fabricated hash provenance (Class B)**: this hook detects cross-site value
    *inconsistency* only — it cannot distinguish a stale hash (a previously valid computed value
    that is no longer current, requiring a sweep fix) from a fabricated hash (a value that was
    never the output of `compute-input-hash --update` for this file at any revision, constituting
    a POLICY 18 violation). Both trigger Class B. The distinction is load-bearing for remediation:
    stale → run `compute-input-hash --update`; fabricated → acknowledge the provenance break in
    the burst-log per POLICY 18 before running `--update`. Pass-30 M02 found story provenance
    terminating at fabricated `1acf3c6` with no such acknowledgment, which was a POLICY 18
    violation. When Class B blocks, the fix team MUST verify: (a) trace the stored hash to a prior
    valid `--update` run (stale path), or (b) document the provenance break in the burst-log
    (fabricated path). A hash corrected without provenance verification restarts the fabricated
    class silently. The hook block message SHOULD note this distinction to guide remediation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | BC file written with version "1.0" not yet in BC-INDEX body table | Advisory + Continue. New BC; registration expected in same burst. |
| EC-002 | BC bumped v1.17→v1.18; BC-INDEX row still says v1.17 | Block: Class A Arm1 (v1.17 vs v1.18). |
| EC-003 | BC file write; BC-INDEX.md returns `HostError::NotFound` | Advisory + Continue. Secondary target NotFound = bootstrap. |
| EC-004 | BC file write; `host::read_file` for the BC file returns `HostError::CapabilityDenied` | Block. Primary target fail-closed. |
| EC-005 | Story S-21.04 frontmatter hash "47a65c9"; STORY-INDEX catalog "4be9d21"; blockquote "S-21.04=47a65c9" | Block: Class B (B2 ≠ B1). |
| EC-006 | Story file has no `input-hash:` field | Continue — Arm B1 skips. |
| EC-007 | STORY-INDEX.md written; blockquote "S-21.04=47a65c9"; catalog row "input-hash 47a65c9" | Continue — B2 == B3 (Arm B2 passes). |
| EC-008 | STORY-INDEX.md written; blockquote "S-21.04=47a65c9"; catalog row "input-hash 1acf3c6" | Block: Class B Arm B2 mismatch. |
| EC-009 | Story file write; STORY-INDEX.md returns `HostError::CapabilityDenied` | Block. Capability-class error on secondary target. |
| EC-010 | burst-log.md; last H2 "Closes: F-S2104-P29-H01, F-S2104-P29-H02" | Continue (no advisory). |
| EC-011 | burst-log.md; last H2 "Closes: B01, F-S2104-P29-H02" | Advisory for "B01" (finding-like, not F-, not excluded); Continue. |
| EC-012 | lessons.md; latest L-EDP1 entry "Closes: 001" (bare numeric) | `001` starts with a digit → does NOT match shape `[A-Za-z][A-Za-z0-9-]*[0-9]+` → NOT flagged. Continue. |
| EC-013 | lessons.md; historical Changelog section contains `P45-001` | Continue — Changelog is outside the scope-limited window (last L-EDP1 entry only). |
| EC-014 | BC written with `version: "1.6"` and `last_amended: "2026-07-29 (v1.6) — ..."` | Part E passes. |
| EC-015 | BC written with `version: "1.33"` and `last_amended: "2026-07-29 (v1.31) — ..."` | Block: Class E1 (v1.33 vs v1.31). |
| EC-016 | BC written with `modified: ["2026-05-14", "2026-05-18 (v1.1)", "2026-05-15"]` | Block: Class E2 (2026-05-15 follows 2026-05-18). |
| EC-017 | BC written with `modified: ["2026-05-14", "2026-05-18", "2026-05-20 (v1.3)"]` | Part E passes (ascending after suffix strip). |
| EC-018 | BC with `last_amended:` containing `[Prior: ... (v1.5) ...]`; version "1.6"; outermost prefix "(v1.6)" | Part E passes — positional anchor matches `(v1.6)` at date position; Prior tokens excluded structurally. |
| EC-019 | BC write triggers Class A Arm1 (index stale) + Class E1 (version mismatch) | Single combined block enumerating both violations. |
| EC-020 | VP file written with `version: "2.4"` and `last_amended: "2026-05-20 (v2.4)"` | Part E passes. VP files are in Part E scope (PC34). |
| EC-021 | Story S-21.04 written; B1 = "47a65c9"; B2 absent; B3 absent | Advisory + Continue. New story pre-registration. |
| EC-022 | `last_amended:` does not match `\d{4}-\d{2}-\d{2}\s+\(v` | Advisory + Continue. Do NOT block on unparseable format. |
| EC-023 | STORY-INDEX.md written | Only Arm B2 fires. STORY-INDEX.md does not match `S-*.md` pattern. |
| EC-024 | burst-log.md; last H2 "Refs: D-944" | `D-944` matches shape BUT `D-` is in the exclusion list (PC32) → NOT flagged. Continue. |
| EC-025 | BC file write; both BC file AND BC-INDEX return `HostError::CapabilityDenied` | Combined block citing both failures. |
| EC-026 | Story S-21.04 written; `behavioral_contracts: [BC-6.26.001]`; Token Budget row cites "v1.17"; BC-6.26.001 fm "1.18" | Block: Class A Arm2 (story cites v1.17 vs BC v1.18). |
| EC-027 | Story S-21.04 written; `behavioral_contracts: [BC-5.39.010]`; BC-5.39.010.md returns `HostError::NotFound` | Advisory for NotFound BC + Continue. |
| EC-028 | Story S-21.04 written; `behavioral_contracts: [BC-6.26.001, BC-5.39.008]`; both stale | Single combined block listing both BCs (cascade). |
| EC-029 | Story written; `behavioral_contracts: [BC-6.26.001]`; BC cited only in prose, no version token in any table row | Arm A2 finds no version-citing rows → skip → Continue. |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Output | Part | Mutant | Control |
|----------|----------------|-----------------|------|--------|---------|
| A Arm1 — new BC | v1.0; no INDEX row | advisory + Continue | A Arm1 | v1.1, no row → block | v1.0 with INDEX row v1.0 → Continue |
| A Arm1 — stale | BC-5.39.008 v1.6; INDEX "v1.5" | block | A Arm1 | INDEX "v1.6" → Continue | |
| A Arm2 — current | S-21.04; `behavioral_contracts: [BC-6.26.001]`; story Token Budget "v1.18"; BC fm "1.18" | Continue | A Arm2 | BC fm "1.19" while story says "v1.18" → block | `behavioral_contracts:` empty → Continue |
| A Arm2 — stale | S-21.04; story cites "v1.17"; BC fm "1.18" | block | A Arm2 | Both "v1.18" → Continue | |
| B Arm1 — match | hash "47a65c9"; catalog "47a65c9"; blockquote "47a65c9" | Continue | B Arm1 | blockquote "4be9d21" → block | no input-hash → Continue |
| B Arm2 — mismatch | STORY-INDEX catalog "47a65c9"; blockquote "4be9d21" | block | B Arm2 | both "47a65c9" → Continue | |
| D — excluded token | "Closes: F-S2104-P29-H01, D-944" | Continue (D-944 excluded) | D | "Closes: B01" → advisory | |
| D — phantom | "Closes: B01, F-S2104-P29-H01" | advisory for B01 + Continue | D | only "F-..." → Continue | |
| E1 — match | version "1.6"; last_amended "(v1.6)" | Continue | E | "(v1.5)" → block | Prior chain "(v1.5)" deeper → Continue |
| E2 — out-of-order | modified: ["2026-05-14","2026-05-18","2026-05-15"] | block | E | Ascending → Continue | |
| Combined A+E | INDEX stale + E1 mismatch | single combined block | A+E | each alone → block | both fixed → Continue |

## Gate Specifications

### Registry entry

```toml
[[hooks]]
name = "validate-cross-site-correspondence"
plugin = "hook-plugins/validate-cross-site-correspondence.wasm"
event = "PostToolUse"
tool = "^(Edit|Write|MultiEdit)$"
tier = "sync"
on_error = "continue"
path_allow = [
  ".factory/specs/behavioral-contracts/",
  ".factory/specs/verification-properties/",
  ".factory/stories/",
  ".factory/cycles/",
]
timeout_ms = 8000
```

`on_error = "continue"`: fuel exhaustion or plugin crash is non-blocking. Fuel exhaustion is the
primary risk for large artifacts: BC-INDEX.md and `lessons.md` approaching 3,000+ lines may exhaust
the WASM sandbox fuel budget before validation logic runs, causing a silent skip that becomes
observable once the host-level fuel advisory log is shipped (ADR-035 §Decision 5). The `max_bytes`
caps in PC4 (1 MiB), PC10/12/15/35 (512 KiB), and PC19/23/29 (2 MiB) are calibrated to bound
reads inside the fuel budget at current artifact sizes. Fuel exhaustion is silenced at the registry
level via `on_error = "continue"` — there is no WASM-side handling for it in this hook.

**`fuel_cap` field**: ADR-035 §Decision 5 introduced a per-plugin `fuel_cap` registry field. This
hook does NOT require a non-default cap — the `max_bytes` limits already bound the read-dominated
computation per invocation. A future implementer MUST NOT add a `fuel_cap` entry without first
confirming that `max_bytes` caps are genuinely insufficient; adding a cap without that evidence is
premature optimization against a non-observed exhaustion scenario at current artifact scales.

### Internal dispatch logic

```
fn run(payload):
    file_path = payload.tool_input.file_path
    violations = []
    advisories = []

    if is_bc_file(file_path):           // Part A Arm1 + Part E
        content = read_primary(file_path, 524288, 3000)
        violations += run_part_a_arm1(file_path, content)
        violations += run_part_e(file_path, content)
    elif is_story_file(file_path):      // Part A Arm2 + Part B Arm1 + Part E
        content = read_primary(file_path, 524288, 3000)
        violations += run_part_a_arm2(file_path, content)
        violations += run_part_b_arm1(file_path, content)
        violations += run_part_e(file_path, content)
    elif is_story_index(file_path):     // Part B Arm2
        content = read_primary(file_path, 2097152, 5000)
        violations += run_part_b_arm2(file_path, content)
    elif is_cycle_artifact(file_path):  // Part D
        content = read_primary(file_path, 2097152, 5000)
        advisories += run_part_d(file_path, content)

    for adv in advisories: host::log_warn(adv)
    if violations: return combined_block(violations)
    return HookResult::Continue
```

### Part A Arm1: `run_part_a_arm1(file_path, content)`

```
bc_id = basename(file_path).trim_suffix(".md")
fm_version = extract_frontmatter_field(content, "version") |> strip_v_prefix
index_content = host::read_file(BC_INDEX_PATH, 1048576, 3000)
  // NotFound → log_warn advisory + return []
  // CapabilityDenied → return [block(cap_denied)]
index_row = find_bc_body_table_row(index_content, bc_id)
if index_row is None:
    if fm_version == "1.0": log_warn(new_bc_advisory); return []
    else: return [block(not_found_v_gt_1_message)]
index_version = extract_version_cell(index_row) |> strip_v_prefix
if fm_version != index_version: return [block(stale_arm1_msg(bc_id, index_version, fm_version))]
return []
```

### Part A Arm2: `run_part_a_arm2(story_path, story_content)`

```
bc_ids = extract_frontmatter_sequence(story_content, "behavioral_contracts")
if bc_ids.is_empty(): return []
story_id = extract_story_id_prefix(story_path)
violations = []
for bc_id in bc_ids:
    // Deterministic path derivation — no list_dir
    section = bc_id.trim_start_matches("BC-").split('.').next().parse::<u32>()
    bc_path = format!(".factory/specs/behavioral-contracts/ss-{:02}/{}.md", section, bc_id)
    bc_content = match host::read_file(bc_path, 524288, 3000):
        Ok(c) => c
        Err(HostError::NotFound) => { log_warn(not_found_advisory(bc_id)); continue }
        Err(e) => return [block(bc_read_error(bc_id, bc_path, e))]
    bc_version = extract_frontmatter_field(bc_content, "version") |> strip_v_prefix
    citations = extract_story_bc_version_citations(story_content, bc_id)
    // Vec<(location: String, cited_version: String)>
    // finds all table rows containing bc_id + a version token; location = row description
    for (location, cited_ver) in citations:
        if strip_v_prefix(cited_ver) != bc_version:
            violations.push(stale_arm2_msg(story_id, bc_id, location, cited_ver, bc_version))
return violations
```

### Parts B, D, E

(Algorithms for `run_part_b_arm1`, `run_part_b_arm2`, `run_part_d`, `run_part_e` are unchanged
from v1.0 specification. See Changelog v1.0.)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | A Arm1 Stale-Index Block | bats integration test |
| (pending) | A Arm1 New-BC Advisory | bats integration test |
| (pending) | A Arm1 Primary-CapabilityDenied Block | bats integration test |
| (pending) | A Arm2 Stale-Citation Block | bats integration test (story Token Budget stale) |
| (pending) | A Arm2 No-Citation Skip (Continue) | bats integration test (no version-citing rows) |
| (pending) | A Arm2 BC-NotFound Advisory (Continue) | bats integration test |
| (pending) | A Arm2 BC-CapabilityDenied Block | bats integration test |
| (pending) | B Arm1 Three-Way Mismatch Block | bats integration test |
| (pending) | B Arm1 Absent-Sites Advisory | bats integration test |
| (pending) | B Arm2 Internal Mismatch Block | bats integration test |
| (pending) | D Namespace-Excluded Token Pass (D-944) | bats integration test |
| (pending) | D Phantom-ID Advisory | bats integration test |
| (pending) | D Historical-Excluded Pass | bats integration test |
| (pending) | E1 Version-Mismatch Block | bats integration test |
| (pending) | E2 Non-Ascending Block | bats integration test |
| (pending) | E Prior-Chain Exclusion Pass | bats integration test |
| (pending) | Combined A+E Block | bats integration test |

VP IDs pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — cross-site value correspondence enforcement) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes PostToolUse gates enforcing cross-site value correspondence invariants recurring across passes 28-30 of the F5 adversarial cycle: Class A = POLICY 14 leg 5 (two-arm: BC-INDEX Arm1 + story citation Arm2); Class B = POLICY 18 THREE-WAY-INPUT-HASH-EQUALITY-GATE (D-923); Class E = POLICY 14 leg 4 / POLICY 17; Class D = finding-ID namespace advisory. No formal CAP-NNN from domain-spec/capabilities.md covers engine process governance automation at this layer; E-12 is the established sub-capability anchor for this BC family per BC-5.39.008 §Traceability. |
| Architecture Module | `crates/hook-plugins/validate-cross-site-correspondence/` (new WASM crate); `plugins/vsdd-factory/hooks-registry.toml`; `plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm` |
| D-NNN Sub-Clauses Closed | POLICY 14 leg 5 (BC-INDEX body-table + story citation sync; Class A Arm1 + Arm2); POLICY 18 D-923 (Class B); POLICY 14 leg 4 / POLICY 17 (Class E); Canonical Principle + TD-VSDD-059 (fail-closed) |
| Stories | TBD |
| L2 Invariants | (none — process-automation gate) |

## Related BCs

- BC-5.39.003 — `validate-index-cite-refresh`: Class A Arm1 is the per-BC-write counterpart
- BC-5.39.004 — `validate-burst-log`: burst-log structural completeness
- BC-5.39.007 — `validate-closes-completeness`: Closes block presence; Class D checks format
- BC-5.39.008 — `validate-policies-schema`: establishes fail-closed ruling governing Invariant 4
- BC-6.26.001 — primary motivating artifact for Class A; repeatedly re-staled in passes 28-30

## Architecture Anchors

- `crates/hook-plugins/validate-cross-site-correspondence/`
- `crates/hook-sdk/src/host.rs` — `host::read_file`, `host::log_warn`
- `crates/hook-sdk/src/result.rs` — `HookResult::Continue`, `HookResult::block_with_fix`
- `derive_bc_path(bc_id)` — deterministic BC file path derivation from BC ID (no list_dir)
- `extract_story_bc_version_citations(content, bc_id)` — finds version-citing table rows for a given BC ID; returns Vec<(location, version)>
- `extract_frontmatter_sequence(content, field)` — parses YAML sequence field from frontmatter

## Story Anchor

TBD — no story allocated yet.

## VP Anchors

VP IDs pending VP-INDEX allocation by state-manager at post-merge burst.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.2 | 2026-07-30 | Registry entry corrected: `tools = [...]` array replaced with `tool = "^(Edit\|Write\|MultiEdit)$"` regex string (field name singular + MultiEdit added; all 41 Edit/Write hooks in live registry guard this pattern; omitting MultiEdit was a POLICY 13 ESCAPE-SCOPE-PARITY gap identical in class to F-S2104-P29-H02). Fuel-exhaustion note added to Gate Specifications per ADR-035 §Decision 5: `on_error = "continue"` silences the hook non-blockingly on fuel exhaustion (not WASM-side logic); `max_bytes` caps bound reads inside the fuel budget; `fuel_cap` not required. BC-version-pin datum-copy ruling added as design note in §Postconditions Part A Arm2. (product-owner.) |
| 1.1 | 2026-07-30 | Part A Arm2 (story-file-side trigger) added: PostToolUse on story → read each `behavioral_contracts:` BC via deterministic path derivation → compare against story version citations (Token Budget + BC-table rows). No list_dir required. Latency gap correctly sized (all 6 observed failures occurred during story-editing bursts). Advisory rationales made explicit for every advisory arm; confirmed no arm is advisory merely because the check is partial. Class D tokenizer namespace-exclusion list added (D-, S-, BC-, VP-, R-, L-, ADR-, EC-, NFR-, ASM-, FM-); EC-024 rationale corrected (D-944 matches shape BUT is excluded by namespace list). Invariant 11 added: stale vs fabricated hash provenance — stale = sweep fix, fabricated = POLICY 18 acknowledgment required (pass-30 M02 precedent). EC-026/027/028/029 added. Gate Spec updated with run_part_a_arm2 pseudocode and story-file dispatch branch. VP table extended to 17 entries. |
| 1.0 | 2026-07-30 | Initial authoring (product-owner; pre-pass-30 fix-burst). Classes A Arm1/B/D/E gated; Class C honest-gap + Rust test recommendation. |
