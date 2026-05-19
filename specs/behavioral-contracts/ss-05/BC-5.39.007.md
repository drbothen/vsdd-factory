---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-05-18T00:00:00Z
phase: section-12-step-3M3a-r-pass-2
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
  - .factory/cycles/v1.0-brownfield-backfill/decision-log.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.004.md
  - .factory/specs/architecture/decisions/ADR-022-hook-current-pass-context-discovery.md
input-hash: "ad1c745"
traces_to: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
extracted_from: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - 2026-05-18
  - 2026-05-19
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.007
section: "5.39"
last_amended: "2026-05-18 (v1.1) — Pass-1 adversary fix-burst (product-owner; brownfield-backfill M3 3M3a-r fix-burst). Closes F-BC007P1-001 (CRITICAL Closes format: ### h3 → **Closes:** bold-prefix-line per lessons.md corpus ground truth), F-BC007P1-002 (HIGH PC2a marker regex verbatim), F-BC007P1-003 (HIGH Phase 1/2 boundary crisply enumerated), F-BC007P1-004 (HIGH STATE.md 512 KiB read cap explicit in Preconditions), F-BC007P1-005 (HIGH EC-018 path clarified as precondition-violation input-validation path), F-BC007P1-006 (MEDIUM PC1 trivially-satisfied attestation added), F-BC007P1-007 (MEDIUM EC-019 regex verbatim), F-BC007P1-008 (MEDIUM PC2 split into PC2a+PC2b with explicit AND), F-BC007P1-009 (MEDIUM HookResult rewritten to use actual SDK variants: HookResult::Block / HookResult::Continue — no Advisory variant in hook-sdk), F-BC007P1-010 (MEDIUM Phase 2 ADR-022 gate inline summary added), F-BC007P1-011 (MEDIUM EC-020 empty/zero-byte STATE.md added; renumbered from prior EC-020), F-BC007P1-012 (MEDIUM hook ordering with BC-5.39.005 specified), F-BC007P1-013 (LOW sub-contracts none statement added), F-BC007P1-014 (LOW LENGTH==3 off-by-one test vector row added), F-BC007P1-015 (LOW EC-017 formatting trimmed), F-BC007P1-016 (LOW changelog section authored), F-BC007P1-017 (LOW invariant numbering verified contiguous 1-10), F-BC007P1-018 (LOW PC identifiers substituted in Test Vectors), F-BC007P1-019 (LOW adversary pass coverage note added), F-BC007P1-020 (NIT Phase 1 capitalization standardized), F-BC007P1-021 (NIT SS-05 anchor confirmed). [Prior: 2026-05-18 (v1.0) — Initial authoring (product-owner; brownfield-backfill S-15.03 M3 wave 3M3a BC authoring). Anchors D-419(c)+D-420(e)+D-441(c)+D-442(c)+D-443(b)+D-448(b). BC-5.39.007 allocated as next monotonic ID after BC-5.39.006 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.12 merge). Phase 2 (cross-cell agreement) is reserved for v1.1 in S-15.13 scope per ADR-022.]"
---

# BC-5.39.007: validate-closes-completeness Phase 1 WASM hook MUST block on missing Closes blocks in burst-log entries and lessons.md entries, malformed cite IDs in Closes blocks, forbidden per-mechanism annotations, and umbrella citation sites missing the sample-vs-exhaustive flag in decision-log.md, STATE.md, INDEX.md, and lessons.md

## Description

The `validate-closes-completeness` Phase 1 WASM hook enforces structural integrity of
Closes annotations across the four documents that carry them: `decision-log.md`,
`STATE.md`, `INDEX.md`, and `lessons.md`. The hook fires PostToolUse on Edit/Write to
any of these four files and validates four classes of constraints:

1. **Closes block presence** (D-448(b)): every lesson entry in `lessons.md` MUST have
   a `**Closes:**` bold-prefix-line (not `### Closes` h3 heading); absence is a hard
   block. (Burst-log h2 Closes presence is governed by the existing `validate-burst-log`
   hook, BC-5.39.004.)

2. **Closes cite format** (D-419(c)+D-420(e)): Closes block entries MUST use structured
   ID references (D-NNN, F-PNN-NNN, TD-VSDD-NNN, PG-NNN, L-EDP1-NNN); forbidden patterns
   include `N items per D-413(b) mandate` phrasing and per-finding mechanism annotations.

3. **Sample-vs-exhaustive flag** (D-441(c)+D-442(c)): umbrella citation patterns
   (`D-\d+\.\.D-\d+`) in STATE.md, INDEX.md, and decision-log.md MUST carry the
   sample-vs-exhaustive flag; absence of the flag on a bare range cite is a hard block.

4. **Documentary-historical exemption** (D-443(b)): when exempting pre-existing entries
   from the rules above, the exemption declaration MUST be explicit (not silent omission).

Phase 2 (cross-cell agreement between citation sites) is gated on ADR-022 (current-pass
pointer file, Option c — state-manager writes `.factory/current-adversary-pass.txt` at
Commit A). Phase 2 will be specified in BC-5.39.007 v1.2 as part of S-15.13 authorship.
If Phase 2 is never shipped, Phase 1's blocking coverage governs all structural violation
classes; cross-site staleness (a cite ID appearing correctly formatted but referencing a
nonexistent decision-log row) remains a Phase 1 advisory-log item (not a block) **forever**
— this is an explicit, indefinite false-negative window accepted per ADR-022 Option c gate.
This accepted defect will be re-evaluated at S-15.13 if Phase 2 ships; if S-15.13 is
permanently abandoned, the false-negative window persists indefinitely as a
production-grade known limitation. Consumers of this hook MUST NOT assume cross-site
staleness is detected in Phase 1 deployments.

## Dispatch Arm Routing

The hook fires on Edit/Write to any of 4 files. Different preconditions apply per arm:

| File Target | Applicable PCs | Arm-Specific Notes |
|-------------|---------------|-------------------|
| `lessons.md` | PC1, PC4, PC5, PC6 | PC5 (lesson entry detection) is **lessons.md-specific**. PC2a/PC2b (trajectory-tail marker/LENGTH) do NOT apply — lessons.md has no `current_step:` field. A PC2a failure on a lessons.md write MUST NOT produce a false-positive Block. |
| `STATE.md` | PC1, PC2a, PC2b, PC4, PC6 | PC2a (trajectory-tail marker present) and PC2b (LENGTH==4) are **STATE.md-specific** — they operate on the `current_step:` frontmatter field. |
| `INDEX.md` | PC1, PC4, PC6 | Umbrella-flag check only. No lesson-entry or trajectory-tail checks. |
| `decision-log.md` | PC1, PC4, PC6 | Umbrella-flag check only. No lesson-entry or trajectory-tail checks. |

This routing is a top-level structural feature of the hook. PC2a/PC2b are per-arm
preconditions for the STATE.md arm only; they MUST NOT be evaluated on lessons.md,
INDEX.md, or decision-log.md writes. Failure to isolate arm routing risks
false-positive Block on lessons.md when PC2a fails on a file that has no
`trajectory-tail ` marker (because it is not STATE.md).

## No Sub-Contracts

BC-5.39.007 has no sub-contracts. The `bcs` frontmatter array contains only this BC's
own ID. No child BCs are referenced in the body.

## Hook Ordering

BC-5.39.007 (`validate-closes-completeness`) fires independently of BC-5.39.005
(`validate-state-structure`). When both hooks fire on the same STATE.md write:

- The hooks run in parallel (independent PostToolUse plugins, no explicit ordering
  constraint in the dispatcher).
- A state-structure violation (BC-5.39.005 block) does NOT short-circuit the closes-
  completeness check (BC-5.39.007). Both violations are surfaced independently.
- If state-structure validation emits a block on STATE.md and closes-completeness
  also emits a block on STATE.md, the author receives two independent block messages
  and must correct both before the write passes.

## Adversary Pass Coverage

This BC v1.1 was produced after the first adversary pass (pass-1, 2026-05-18). Finding
counts: 21 adversary findings (F-BC007P1-001..021). This v1.1 fix-burst closes all 21.
Subsequent adversary passes (pass-2+) will resume the 3-CLEAN convergence cascade per
BC-5.39.001.

## Preconditions

### PC1 — Hook invocation (trivially satisfied)

1. A PostToolUse Edit/Write event has fired on a file whose `file_name` path component is
   exactly `decision-log.md`, `STATE.md`, `INDEX.md`, or `lessons.md`
   (path-component-strict matching via `Path::new(file_path).file_name() == Some(<name>)` —
   NOT suffix-`ends_with`). Files like `/some/dir/xSTATE.md` MUST NOT match.

   PC1 is trivially satisfied by the dispatcher: the dispatcher invokes this plugin only
   when its registered file triggers match. No runtime evaluation is required from the
   plugin to satisfy PC1; it is guaranteed by the hook registry configuration.

### PC2a — Trajectory-tail marker present in STATE.md

2. For STATE.md writes only: the hook checks whether the `current_step:` frontmatter
   value contains the literal canonical marker `trajectory-tail ` (with trailing space).
   This is a PRECONDITION for the trajectory-tail marker-present path; absence of the
   marker triggers PC2a-violation handling per the input-validation path.

   **PC2a is distinct from PC2b.** Both PC2a (marker presence) AND PC2b (LENGTH == 4)
   must be satisfied for the trajectory-tail check to pass. These are two independent
   sub-conditions within what was formerly a single PC2.

### PC2b — Trajectory-tail LENGTH == 4

3. For STATE.md writes only (PC2a must be satisfied first): the substring between the
   `trajectory-tail ` marker (exclusive of marker) and the first `;` segment-separator
   (or end-of-value if no `;` follows) MUST contain exactly 4 arrow-separated values
   matching `→(\d+)` globally. The canonical regex applied is `→(\d+)` and the count of
   matches MUST equal 4 (LENGTH == 4 per D-451(c); canonical form
   `trajectory-tail →N→N→N→N`). LENGTH == 3 is an off-by-one violation; LENGTH == 5
   is an over-count violation. Both are blocking per D-451(c).

   This is the same scoping logic as BC-5.39.006 invariant 6(b): apply `→(\d+)` globally
   to the first-semicolon segment after the `trajectory-tail ` marker.

### PC3 — Dispatcher invocation and file read

4. The dispatcher has invoked the `validate-closes-completeness` WASM plugin with the
   write payload.
5. The file content is read via `host::read_file`. The hook does NOT inspect
   `tool_input.content`; the filesystem value is the source of truth for validation.

### PC4 — File read cap (META-LEVEL-24 false-green prevention)

6. `host::read_file` is configured with `max_bytes = 524288` (512 KiB) and
   `timeout_ms = 2000` per call. The registry-level hook timeout is `timeout_ms = 5000`.

   **Rationale (file-size constraint):** STATE.md is approximately 95 KB on production
   factory-artifacts at the time of this BC authoring. The hook-sdk `host::read_file`
   default cap on some configurations is 64 KiB (as established by BC-5.39.004 precedent
   for the S-15.11 validate-burst-log hook). A 64 KiB cap would silently truncate
   STATE.md, causing the hook to validate only the first 64 KiB and miss violations in
   the remainder — the META-LEVEL-24 false-green class (silent inert validator caught as
   F-P5-002 in BC-5.39.005). This BC EXPLICITLY sets `max_bytes = 524288` (512 KiB) to
   prevent truncation on production STATE.md. The registry entry for this hook MUST
   declare `max_bytes = 524288` in its `host::read_file` call.

   For `lessons.md`, which grows monotonically: the 512 KiB cap is sufficient through the
   current cycle (lessons.md soft cap 3500 lines per D-442(e)); if lessons.md exceeds
   512 KiB, the hook degrades to fail-open per invariant 9, not silent truncation.

### PC5 — Lesson entry detection

7. For `lessons.md`: the hook identifies lesson entries as h2 sections (`## L-` or
   `## PG-` headings). Each h2 lesson entry must contain a `**Closes:**` bold-prefix
   line. The canonical Closes block format is `**Closes:** <cite-list>` on a single line
   (as observed in `lessons.md` corpus via literal grep of `.factory/cycles/v1.0-brownfield-backfill/lessons.md`).
   The `### Closes` h3 heading format is NOT the canonical format and MUST NOT be used
   as the detection pattern.

### PC6 — Umbrella-range detection

8. For the umbrella-range check: the pattern `D-\d+\.\.D-\d+` identifies a bare umbrella
   citation. The flag is present if the same sentence or paragraph also contains one of:
   `(sample)`, `(exhaustive)`, `sample-vs-exhaustive`, or
   `see decision-log.md for full range`. Absence of any of these markers on a bare range
   cite is a violation per D-441(c)+D-442(c).

## Postconditions

### lessons.md arm (file_name == "lessons.md")

1. If every h2 lesson entry (`## L-` or `## PG-` headed section) in `lessons.md` contains
   at least one `**Closes:**` bold-prefix line with non-empty content, the hook emits
   `HookResult::Continue` for the Closes-block-presence check on that file.

2. If any h2 lesson entry in `lessons.md` lacks a `**Closes:**` bold-prefix line (or has
   a `**Closes:**` line with no content beyond the label itself), the hook emits
   `HookResult::Block { reason: block_with_fix(...) }` naming the entry (by h2 heading
   text) and citing D-448(b).

   **Closes format ground truth:** The canonical form is `**Closes:** <cite-list>` as a
   standalone line within the lesson entry body. The `### Closes` h3 heading form is NOT
   canonical and MUST NOT be accepted as a valid Closes block by this hook.

3. If a `**Closes:**` line exists in a lesson entry but contains forbidden per-mechanism
   annotations (pattern: `\(per D-413\(b\) mandate\)`) or the phrase
   `N items per D-413(b)`, the hook emits `HookResult::Block { reason: block_with_fix(...) }`
   citing D-420(e) and naming the offending line.

### decision-log.md arm (file_name == "decision-log.md")

4. If no bare umbrella citation `D-\d+\.\.D-\d+` is found without a sample-vs-exhaustive
   flag, the hook emits `HookResult::Continue` for the umbrella-flag check.
5. If any bare umbrella citation `D-\d+\.\.D-\d+` exists in the file WITHOUT an adjacent
   sample-vs-exhaustive flag marker, the hook emits `HookResult::Block { reason: block_with_fix(...) }`
   naming the offending range and citing D-441(c)+D-442(c).

### STATE.md arm (file_name == "STATE.md")

6. Same umbrella-flag rule as postcondition 5, applied to STATE.md content. A bare
   `D-\d+\.\.D-\d+` cite without an adjacent sample-vs-exhaustive flag marker is a
   `HookResult::Block { reason: block_with_fix(...) }` citing D-441(c)+D-442(c).

### INDEX.md arm (file_name == "INDEX.md")

7. Same umbrella-flag rule as postconditions 5-6, applied to INDEX.md content.

### All arms — cite ID validation

8. If any `**Closes:**` line entry contains a cite that does not match a recognized
   structured ID pattern (`D-\d+`, `F-P\d+-\d+`, `TD-VSDD-\d+`, `PG-[A-Za-z0-9-]+`,
   `L-EDP1-\d+`, `ADV-EDP1-P\d+-[A-Z]+-\d+`) — i.e., is freeform prose or an arbitrary
   label — the hook emits `HookResult::Block { reason: block_with_fix(...) }` citing
   D-419(c) and naming the malformed entry.

### All arms — cascade and fail-open

9. Multiple violations in one write produce a single `HookResult::Block` message
   enumerating all violations together.
10. If `host::read_file` returns an error for any target file (HostError of any kind), the
    hook emits `HookResult::Continue` and logs a warning via `host::log_warn` — fail-open.

### Phase 1 advisory vs blocking — explicit boundary

The following table enumerates which violation classes are Phase 1 blocking, Phase 1
advisory-log-only (non-blocking), and Phase 2-only. **If Phase 2 is never shipped, the
advisory-log items remain non-blocking forever** — this is an acknowledged false-negative
window per ADR-022 Option c gate.

| Violation Class | Phase 1 Behavior | Phase 2 Behavior | Notes |
|-----------------|-----------------|-----------------|-------|
| Missing `**Closes:**` line in lesson entry | **BLOCK** (PC5/postcondition 2) | — (Phase 1 covers) | Hard block per D-448(b) |
| Empty `**Closes:**` line (label only, no content) | **BLOCK** (postcondition 2) | — | Same rule as absent |
| `### Closes` h3 heading (wrong format) | **BLOCK** (postcondition 2) | — | `### Closes` is not a valid `**Closes:**` line |
| Forbidden per-mechanism annotation in Closes | **BLOCK** (postcondition 3) | — | Pattern `\(per D-413\(b\) mandate\)` |
| Bare umbrella cite without sample-vs-exhaustive flag | **BLOCK** (postconditions 4-7) | — | D-441(c)+D-442(c) |
| Malformed cite ID (no structured pattern) | **BLOCK** (postcondition 8) | — | D-419(c) |
| Length-violation in trajectory-tail (PC2b) | **BLOCK** (postcondition for STATE.md writes; see BC-5.39.006 for trajectory-tail gate) | — | BC-5.39.006 owns trajectory-tail; this BC governs Closes and umbrella cites only |
| Cross-site staleness (cite ID correct format, but references nonexistent D-NNN) | **Advisory log via `host::log_warn`; Continue** (not block) | **BLOCK** | Phase 1 false-negative window; ADR-022 gate required for full verification |

## Invariants

1. The hook NEVER writes to any file. It has no `write_file` capability in its registry
   entry. It is a read-only post-write validator.
2. The hook fires PostToolUse only — it never prevents a write; it signals AFTER the write
   has completed. The dispatcher records the block signal; the author must correct and
   re-write.
3. Path-component-strict matching applies to all four target paths:
   `Path::new(file_path).file_name() == Some(<name>)`. Using `ends_with(...)` on the raw
   path string MUST NOT be substituted (false-positive on `xSTATE.md` / `xINDEX.md` etc.).
4. The sample-vs-exhaustive flag check applies ONLY to umbrella range patterns
   (`D-\d+\.\.D-\d+`). Single D-NNN cites and parenthetical enumerations do NOT trigger
   this check. The flag markers recognized are: literal substring `(sample)`, literal
   substring `(exhaustive)`, literal substring `sample-vs-exhaustive`, or literal substring
   `see decision-log.md for full range`. At least one of these MUST appear in the same
   prose unit (sentence or paragraph) as the bare range pattern.
5. The forbidden-annotation check in Closes blocks applies specifically to the pattern
   `per D-413(b) completeness mandate` and the `N items per D-413(b)` phrase used as a
   shorthand. Individual per-item cites (e.g., `F-P39-001`) in a Closes block are NOT
   forbidden — only the aggregate annotation is. Explicitly verbatim: patterns matching
   `\(per D-413\(b\) completeness mandate\)` MUST NOT appear in Closes block cite lines
   per D-420(e). The `(?:completeness )?` optional-group form is the regex used for
   detection; the EC-004 example uses the literal `(per D-413(b) completeness mandate)`
   form (with the word "completeness" present) which is the canonical production instance
   that triggers the block. Both with and without "completeness" are blocked by the regex.
6. Lesson-entry detection in `lessons.md` is h2-heading-based. A line matching
   `^## (L-|PG-)` begins a new lesson entry. The `**Closes:**` requirement applies to
   entries after the `## L-` or `## PG-` heading. Pre-existing entries that predate the
   D-448(b) codification date MUST carry an explicit documentary-historical exemption
   declaration per D-443(b) to be skipped by this validation; absence of the exemption
   declaration means the entry is in scope.
7. The documentary-historical exemption declaration per D-443(b) is recognized by the
   presence of a comment or inline note containing the literal text `(documentary-historical)`
   or `(pre-D-448(b) exemption)` adjacent to or within the non-conforming block. A silent
   omission of the `**Closes:**` line without this declaration is NOT a valid exemption.
8. The cite-ID validation in postcondition 8 applies ONLY to lines beginning with
   `**Closes:**` or lines within Closes blocks, not to the full file body. Lines
   containing NO recognizable structured ID pattern and also not matching a blank line or
   a markdown formatting line (e.g., `---`, `**...**`) are candidates for the
   malformed-cite block.
9. All `host::read_file` calls are fail-open: read errors produce Continue + log_warn,
   not Block. The total timeout budget is bounded by the registry `timeout_ms = 5000`
   limit.
10. All byte-index slice expressions operating on content strings MUST use
    `is_char_boundary()` guards where multi-byte UTF-8 input is possible (em-dash,
    en-dash, NBSP, typographic apostrophes are plausible in lessons.md narrative text).
    Slice without boundary guard is a runtime panic risk per the S-15.11 cascade lesson
    F-P4-001.

## Phase 2 — reserved (S-15.13 scope; ADR-022 Option c gate)

Phase 2 covers **cross-cell Closes set agreement**: the hook reads the current adversary
review file (located via `.factory/current-adversary-pass.txt` pointer written by
state-manager at every Commit A per ADR-022 Option c) to extract the canonical finding
set, then cross-validates finding IDs against the 8 prescribed citation sites (burst-log
Dim-5, Closes block, STATE.md Decisions Log, INDEX.md Convergence Status, lessons.md,
decision-log.md, ARCH-INDEX, BC-INDEX).

**Phase 2 is NOT implemented in this v1.1 BC.** Its precondition is ADR-022 Option c
(state-manager writes `.factory/current-adversary-pass.txt` at Commit A). Phase 2
invariants will be specified in BC-5.39.007 v1.2 as part of S-15.13 authorship.

**ADR-022 gate trigger:** Phase 2 becomes implementable when S-15.12 is MERGED (Phase 1
shipped) AND state-manager's Commit A sequence writes `.factory/current-adversary-pass.txt`
(ADR-022 Option c state-manager obligation codified as S-15.13 AC).

A cite ID that appears correctly formatted (passes postcondition 8) but references a
nonexistent decision-log row or finding ID is **logged as advisory via `host::log_warn`
in Phase 1** (non-blocking) — cross-cell agreement validation requires Phase 2's
cross-document resolver.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `lessons.md` h2 entry `## L-EDP1-007` has no `**Closes:**` line | `HookResult::Block` naming the entry heading and citing D-448(b) |
| EC-002 | `lessons.md` h2 entry `## L-EDP1-007` has `**Closes:**` label but no content after the colon | `HookResult::Block`: empty Closes line is equivalent to absent; cite D-448(b) |
| EC-003 | `lessons.md` h2 entry `## PG-S-15.14-tdd-micro-commit` has `**Closes:** F-P1-007` line | `HookResult::Continue` for that entry |
| EC-004 | Closes line reads `**Closes:** F-P39-001, F-P39-002 (per D-413(b) completeness mandate)` | `HookResult::Block` citing D-420(e); forbidden annotation pattern matched |
| EC-005 | Closes line reads `**Closes:** 5 items per D-413(b) mandate` | `HookResult::Block` citing D-420(e); aggregate-shorthand pattern matched |
| EC-006 | `STATE.md` contains `D-389..D-480` with no flag | `HookResult::Block`: umbrella cite without sample-vs-exhaustive flag; D-441(c)+D-442(c) |
| EC-007 | `STATE.md` contains `D-389..D-480 (sample; see decision-log.md for full range)` | `HookResult::Continue`: sample-vs-exhaustive flag present |
| EC-008 | `decision-log.md` contains `D-401..D-454 (exhaustive)` | `HookResult::Continue`: exhaustive flag present |
| EC-009 | `INDEX.md` contains `D-389..D-454` without flag on same line or sentence | `HookResult::Block` citing D-441(c)+D-442(c) |
| EC-010 | Closes line reads `**Closes:** D-999` where D-999 does not exist in decision-log.md | `HookResult::Continue` with advisory log via `host::log_warn` (Phase 1); not a block |
| EC-011 | Closes line reads `**Closes:** fixed the thing` (no structured ID) | `HookResult::Block` citing D-419(c): malformed cite (no recognized structured ID pattern) |
| EC-012 | `lessons.md` pre-D-448(b) entry has no `**Closes:**` line but carries `(pre-D-448(b) exemption)` inline declaration | `HookResult::Continue`: exemption declared per D-443(b); entry skipped |
| EC-013 | `lessons.md` pre-D-448(b) entry silently omits `**Closes:**` line with no exemption declaration | `HookResult::Block`: silent omission is NOT a valid exemption per invariant 7 |
| EC-014 | Multiple violations across lessons.md and decision-log.md in a single write | Single `HookResult::Block` enumerating all violations (postcondition 9) |
| EC-015 | Path is `/some/dir/xSTATE.md` (ends_with "STATE.md" but file_name differs) | `HookResult::Continue` (path-component-strict guard; not a target file) |
| EC-016 | `host::read_file` returns HostError::Timeout for lessons.md (partial read or full failure) | `HookResult::Continue` + `host::log_warn`; fail-open. **Cascade order with EC-018:** if partial content is returned before timeout and that partial content contains a `### Closes` block (wrong format), EC-016 (read-failure fail-open) takes precedence — the hook MUST NOT block on potentially-incomplete data. Rationale: a partial read cannot establish that a `**Closes:**` line is absent; false-positive block on truncated data is worse than the advisory miss. |
| EC-017 | Closes line contains `**Closes:** L-EDP1-052` (valid structured ID) | `HookResult::Continue` for that cite |
| EC-018 | `lessons.md` h2 entry has `### Closes` h3 heading instead of `**Closes:**` bold-prefix line | `HookResult::Block` citing D-448(b): `### Closes` is an input-validation failure — the canonical format is `**Closes:**` bold-prefix-line per lessons.md corpus ground truth. This is a precondition-violation path (wrong format), not a postcondition-assertion failure. The hook returns block with message citing the wrong format used. |
| EC-019 | Closes line contains only a dash and whitespace (`- `) adjacent to `**Closes:**` | Treat as empty/malformed cite; `HookResult::Block` citing D-419(c). The canonical `→(\d+)` regex (applied to detect non-structured content) matches nothing; blank content after Closes label is a structural violation. |
| EC-020 | Multiple `### Closes` blocks within a single burst-log h2 entry | The validate-burst-log hook (BC-5.39.004) governs burst-log; this BC governs lessons.md only. For lessons.md: only `**Closes:**` bold-prefix lines are recognized; `### Closes` h3 headings in lessons.md are treated as the wrong format (EC-018 path). |
| EC-021 | `lessons.md` is empty (zero bytes) or STATE.md is zero bytes | `HookResult::Continue`: empty file has no entries and no umbrella cites; no violations possible. Log advisory via `host::log_warn` noting empty file. |
| EC-022 | `lessons.md` entry has `**Closes:** ` (label present, content after colon is only whitespace) | `HookResult::Block` citing D-448(b): whitespace-only content is equivalent to empty |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Preconditions Exercised | Decision |
|----------|----------------|---------------------|------------------------|----------|
| lessons.md entry with valid Closes | `## L-EDP1-052` entry; `**Closes:** D-444 codified` line present | `HookResult::Continue` | PC1, PC5 satisfied | PASS |
| lessons.md entry missing Closes | `## L-EDP1-007` entry; no `**Closes:**` line; no exemption declaration | `HookResult::Block` citing D-448(b) | PC1 satisfied, PC5 violated | BLOCK |
| lessons.md entry with empty Closes | `## PG-discipline` entry; `**Closes:**` label with no content | `HookResult::Block` citing D-448(b) | PC1 satisfied, PC5 violated | BLOCK |
| lessons.md entry with wrong Closes format | `## L-EDP1-010` entry; `### Closes` h3 heading instead of `**Closes:**` line | `HookResult::Block` citing D-448(b): wrong format (input-validation path) | PC5 format-validation | BLOCK |
| Forbidden aggregate annotation | Closes line contains `(per D-413(b) completeness mandate)` | `HookResult::Block` citing D-420(e) | PC1 satisfied | BLOCK |
| STATE.md bare umbrella cite | STATE.md `D-389..D-480` with no flag | `HookResult::Block` citing D-441(c)+D-442(c) | PC1, PC6 satisfied; PC6 flag absent | BLOCK |
| STATE.md umbrella cite with sample flag | STATE.md `D-389..D-480 (sample; see decision-log.md for full range)` | `HookResult::Continue` | PC1, PC6 satisfied | PASS |
| INDEX.md bare umbrella cite | INDEX.md `D-389..D-454` without adjacent flag | `HookResult::Block` citing D-441(c)+D-442(c) | PC1, PC6 satisfied; PC6 flag absent | BLOCK |
| decision-log.md umbrella with exhaustive flag | `D-401..D-454 (exhaustive)` | `HookResult::Continue` | PC1, PC6 satisfied | PASS |
| Malformed cite in Closes | Closes line `**Closes:** fixed the thing` (no structured ID) | `HookResult::Block` citing D-419(c) | PC5 cite-validation | BLOCK |
| Cross-site staleness (Phase 2 scope) | Closes line `**Closes:** D-999` where D-999 nonexistent in decision-log.md | `HookResult::Continue` + `host::log_warn` advisory (Phase 1 only; not block) | PC5 format-check passes; Phase 2 cross-site PENDING | PASS (Phase 1) |
| Pre-D-448(b) exemption declared | lessons.md entry with `(pre-D-448(b) exemption)` inline note and no Closes | `HookResult::Continue` | PC5 exemption path | PASS |
| Pre-D-448(b) exemption NOT declared | lessons.md entry with no Closes and no exemption declaration | `HookResult::Block` citing D-448(b) | PC5 violated | BLOCK |
| Read failure | `host::read_file` returns HostError::CapabilityDenied | `HookResult::Continue` + `host::log_warn` | PC4 fail-open | PASS (fail-open) |
| xSTATE.md path | file_name is "xSTATE.md" | `HookResult::Continue` (path-component-strict guard) | PC1 not triggered | PASS (not target) |
| Multiple violations in one write | Empty Closes + bare umbrella cite + malformed cite ID | Single `HookResult::Block` enumerating all 3 violations | PC5, PC6 violated | BLOCK |
| LENGTH == 3 off-by-one (trajectory-tail) | STATE.md `current_step:` has `trajectory-tail →9→9→9` (3 components after marker) | `HookResult::Block` via BC-5.39.006 trajectory-tail check (that BC owns this gate; this BC does not duplicate it) | Governed by BC-5.39.006 PC2b | BLOCK (BC-5.39.006) |
| Empty STATE.md | STATE.md is zero bytes | `HookResult::Continue` + `host::log_warn` advisory | PC4 fail-open path (no umbrella cites possible) | PASS |

## D-NNN Anchor Coverage

| D-NNN Sub-Clause | Gate Enforced | Postcondition |
|-----------------|---------------|---------------|
| D-419(c) | `(per D-413(b) completeness mandate)` annotation form blocked in Closes cites | PC3/PC8 |
| D-420(e) | Per-finding mechanism annotations in Closes blocks forbidden | PC3 |
| D-441(c) | Umbrella citation sites in STATE.md, INDEX.md, decision-log.md MUST carry sample-vs-exhaustive flag | PC5/PC6/PC7 |
| D-442(c) | Retroactive sweep for sample-vs-exhaustive flag across umbrella citation sites | PC5/PC6/PC7 |
| D-443(b) | Documentary-historical exemption must be explicit; silent omission forbidden | invariant 7 |
| D-448(b) | Lesson entries in lessons.md MUST have `**Closes:**` bold-prefix line | PC1/PC2 |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | Missing-Closes-Block Invariant — hook emits Block when lessons.md entry lacks `**Closes:**` line | bats integration test (fail-lessons-missing-closes fixture) |
| (pending) | Empty-Closes-Block Invariant — hook emits Block when lessons.md `**Closes:**` line exists but has no content | bats integration test (fail-lessons-empty-closes fixture) |
| (pending) | Wrong-Format Closes Invariant — hook emits Block when `### Closes` h3 used instead of `**Closes:**` line | bats integration test (fail-lessons-wrong-closes-format fixture) |
| (pending) | Forbidden-Annotation Invariant — hook emits Block when Closes line contains `(per D-413(b) completeness mandate)` pattern | bats integration test (fail-forbidden-annotation fixture) |
| (pending) | Umbrella-Flag STATE.md Invariant — hook emits Block on bare umbrella cite in STATE.md | bats integration test (fail-state-umbrella-no-flag fixture) |
| (pending) | Umbrella-Flag INDEX.md Invariant — hook emits Block on bare umbrella cite in INDEX.md | bats integration test (fail-index-umbrella-no-flag fixture) |
| (pending) | Umbrella-Flag decision-log.md Invariant — hook emits Block on bare umbrella cite in decision-log.md | bats integration test (fail-decisionlog-umbrella-no-flag fixture) |
| (pending) | Umbrella-Flag Pass Invariant — hook emits Continue when umbrella cite has sample-vs-exhaustive flag | bats integration test (pass-umbrella-with-flag fixture) |
| (pending) | Malformed-Cite Invariant — hook emits Block when Closes line has no structured ID | bats integration test (fail-malformed-cite fixture) |
| (pending) | Exemption-Declared Pass Invariant — hook emits Continue when pre-D-448(b) exemption declared | bats integration test (pass-exemption-declared fixture) |
| (pending) | Fail-open Invariant — hook emits Continue when file is unreadable | bats integration test (fail-open-unreadable fixture) |
| (pending) | Phase-1-Advisory Invariant — cross-site staleness (nonexistent D-NNN) is NOT a Phase 1 block | bats integration test (pass-phase1-advisory-only fixture) |

VP IDs are pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — Closes annotation completeness and format automation sub-capability) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes the PostToolUse gate that mechanically prevents the Closes-annotation structural incompleteness class codified in D-419(c), D-420(e), D-441(c), D-442(c), D-443(b), and D-448(b). The hook targets decision-log.md, STATE.md, INDEX.md, and lessons.md writes — all governance artifacts, not runtime subsystem artifacts. E-12 as used in BC-5.39.003/004/005/006 per capabilities.md §E-12 convention for engine-discipline automation sub-capabilities. |
| Architecture Module | `crates/hook-plugins/validate-closes-completeness/` (Rust WASM plugin, new crate); `plugins/vsdd-factory/hooks-registry.toml` (registry entry); `plugins/vsdd-factory/hook-plugins/validate-closes-completeness.wasm` (compiled binary) |
| D-NNN Sub-Clauses Closed | D-419(c) (Closes annotation format — no aggregate mandate annotation); D-420(e) (per-finding mechanism annotations forbidden in Closes); D-441(c) (sample-vs-exhaustive flag required on umbrella cites in STATE.md+INDEX.md+decision-log.md); D-442(c) (retroactive sweep for flag); D-443(b) (documentary-historical exemption must be explicit); D-448(b) (lessons.md entries must have `**Closes:**` bold-prefix line) |
| ADR References | ADR-022 (Phase 2 gate — current-pass pointer file Option c; S-15.13 scope; ACCEPTED 2026-05-15) |
| Stories | S-15.12 (Phase 1); S-15.13 (Phase 2 extension, pending) |
| L2 Invariants | (none currently assigned — this BC is a process-automation gate; no L2 domain invariants apply) |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.12 must achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.004 — governs validate-burst-log hook (sister PostToolUse hook; burst-log structural completeness including Closes block presence in burst-log entries — distinct from this BC which governs lessons.md Closes presence and format constraints on all four Closes-bearing files)
- BC-5.39.005 — governs validate-state-structure Phase 1 hook (sister PostToolUse hook; runs independently; no short-circuit coupling)
- BC-5.39.006 — governs validate-dispatch-advance WASM hook (sister PostToolUse hook; overlapping STATE.md domain — that BC validates `current_step:` frontmatter structural validity including trajectory-tail LENGTH; this BC validates Closes annotation format and umbrella cite flags in STATE.md body)

## Architecture Anchors

- `crates/hook-plugins/validate-closes-completeness/` — new hook crate (Phase 1 + Phase 2 extension); lesson-entry detection in `find_lesson_entries`; Closes-presence check in `check_closes_present`; umbrella-flag check in `check_umbrella_flag`; forbidden-annotation check in `check_forbidden_annotations`
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook; `host::log_warn(message)` for advisory-level non-blocking log entries
- `crates/hook-sdk/src/result.rs` — `HookResult` enum: `Continue`, `Block { reason }`, `Error { message }`; `HookResult::block_with_fix(hook, reason, recommendation, code)` constructor for canonical block messages; NOTE: there is NO `Advisory` variant — advisory behavior is implemented via `HookResult::Continue` + `host::log_warn`
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `tool = "Edit|Write"` and four file targets `decision-log.md`, `STATE.md`, `INDEX.md`, `lessons.md`
- `specs/architecture/decisions/ADR-022-hook-current-pass-context-discovery.md` — Phase 2 gate; current-pass pointer file protocol Option c

## Story Anchor

S-15.12 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M3 story; Phase 1 scope)
S-15.13 — v1.0-brownfield-backfill (Phase 2 extension; BLOCKED on ADR-022 + S-15.12 SHIPPED)

## VP Anchors

VP IDs pending VP-INDEX allocation by state-manager at S-15.12 post-merge burst.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.2 | 2026-05-19 | Pass-2 adversary fix-burst (product-owner; brownfield-backfill M3 3M3a-r pass-2; INV-017 applied). Closes F-BC007P2-002 (HIGH: Phase-2-never-shipped false-negative window declared explicitly), F-BC007P2-003 (HIGH: PC2/PC5 renumber propagation — Phase-1 boundary table + Test Vectors updated), F-BC007P2-004 (MEDIUM: Dispatch Arm Routing section added), F-BC007P2-005 (MEDIUM: EC-016/EC-018 cascade order declared), F-BC007P2-007 (LOW: invariant 5 regex parenthetical aligned with EC-004). F-BC007P2-001 handled in BC-5.39.006 v1.4. F-BC007P2-006 handled in BC-5.39.008 v1.2. |
| 1.1 | 2026-05-18 | Pass-1 adversary fix-burst. Closes all 21 F-BC007P1-NNN findings. CRITICAL: Postconditions rewritten to use `**Closes:**` bold-prefix line (not `### Closes` h3) per lessons.md corpus ground truth (F-BC007P1-001); PC2 split into PC2a (marker presence) + PC2b (LENGTH == 4) with explicit AND semantics (F-BC007P1-008); trajectory-tail regex `→(\d+)` first-semicolon-segment verbatim from BC-5.39.006 v1.3 (F-BC007P1-002); Phase 1/2 boundary table added explicitly enumerating all violation classes (F-BC007P1-003); PC4 file-read cap 512 KiB with META-LEVEL-24 false-green rationale (F-BC007P1-004); EC-018 clarified as input-validation/precondition-violation path for wrong-format detection (F-BC007P1-005); PC1 trivially-satisfied attestation added (F-BC007P1-006); EC-019 canonical regex noted (F-BC007P1-007); HookResult::Advisory references replaced with HookResult::Continue + host::log_warn — no Advisory variant exists in hook-sdk crates/hook-sdk/src/result.rs (F-BC007P1-009); Phase 2 ADR-022 gate trigger condition added inline (F-BC007P1-010); EC-021 empty/zero-byte STATE.md added (F-BC007P1-011); hook ordering with BC-5.39.005 specified (independent/parallel, no short-circuit) (F-BC007P1-012); sub-contracts none statement added (F-BC007P1-013); LENGTH==3 off-by-one test vector row added (F-BC007P1-014); EC-017 trimmed (F-BC007P1-015); PC identifier columns added to Test Vectors (F-BC007P1-018); adversary pass coverage note added (F-BC007P1-019); Phase 1 capitalization standardized throughout (F-BC007P1-020); SS-05 anchor confirmed (F-BC007P1-021). |
| 1.0 | 2026-05-18 | Initial authoring (product-owner; brownfield-backfill S-15.03 M3 wave 3M3a BC authoring). Anchors D-419(c)+D-420(e)+D-441(c)+D-442(c)+D-443(b)+D-448(b). BC-5.39.007 allocated as next monotonic ID after BC-5.39.006 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.12 merge). Phase 2 (cross-cell agreement) reserved for v1.1 in S-15.13 scope per ADR-022 Option c. Preemptive cascade lessons applied: path-component-strict guard; is_char_boundary() invariant 10; fail-open invariant 9; 524288 max_bytes; Phase-1-advisory for cross-site staleness (postcondition and EC-010). |
