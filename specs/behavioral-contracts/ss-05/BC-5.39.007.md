---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-18T00:00:00Z
phase: section-12-step-3M3a
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
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.007
section: "5.39"
last_amended: "2026-05-18 (v1.0) — Initial authoring (product-owner; brownfield-backfill S-15.03 M3 wave story authoring 3M3a). Anchors D-419(c)+D-420(e)+D-441(c)+D-442(c)+D-443(b)+D-448(b). BC-5.39.007 allocated as next monotonic ID after BC-5.39.006 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.12 merge). Phase 2 (cross-cell agreement) is reserved for v1.1 in S-15.13 scope per ADR-022."
---

# BC-5.39.007: validate-closes-completeness Phase 1 WASM hook MUST block on missing Closes blocks in burst-log entries and lessons.md entries, malformed cite IDs in Closes blocks, forbidden per-mechanism annotations, and umbrella citation sites missing the sample-vs-exhaustive flag in decision-log.md, STATE.md, INDEX.md, and lessons.md

## Description

The `validate-closes-completeness` Phase 1 WASM hook enforces structural integrity of
Closes annotations across the four documents that carry them: `decision-log.md`,
`STATE.md`, `INDEX.md`, and `lessons.md`. The hook fires PostToolUse on Edit/Write to
any of these four files and validates four classes of constraints:

1. **Closes block presence** (D-448(b)): every burst-log h2 entry in `burst-log.md` (handled
   by the existing `validate-burst-log` hook, BC-5.39.004) and every lesson entry in `lessons.md`
   MUST have a `### Closes` block; absence is a hard block.

2. **Closes cite format** (D-419(c)+D-420(e)): Closes block entries MUST use structured
   ID references (D-NNN, F-PNN-NNN, TD-VSDD-NNN, PG-NNN, L-EDP1-NNN); forbidden patterns
   include `N items per D-413(b) mandate` phrasing and per-finding mechanism annotations.

3. **Sample-vs-exhaustive flag** (D-441(c)+D-442(c)): umbrella citation patterns
   (`D-NNN..D-MMM`) in STATE.md, INDEX.md, and decision-log.md MUST carry the sample-vs-exhaustive
   flag; absence of the flag on a bare range cite is a hard block.

4. **Documentary-historical exemption** (D-443(b)): when exempting pre-existing entries from
   the rules above, the exemption declaration MUST be explicit (not silent omission).

Phase 2 (cross-cell agreement between citation sites) is gated on ADR-022 (current-pass pointer
file) and is reserved for BC-5.39.007 v1.1 in S-15.13 scope. See "Phase 2 — reserved" below.

## Preconditions

1. A PostToolUse Edit/Write event has fired on a file whose `file_name` path component is
   exactly `decision-log.md`, `STATE.md`, `INDEX.md`, or `lessons.md`
   (path-component-strict matching via `Path::new(file_path).file_name() == Some(<name>)` —
   NOT suffix-`ends_with`). Files like `/some/dir/xSTATE.md` MUST NOT match.
2. The dispatcher has invoked the `validate-closes-completeness` WASM plugin with the write
   payload.
3. The file content is read via `host::read_file`. The hook does NOT inspect
   `tool_input.content`; the filesystem value is the source of truth for validation.
4. `host::read_file` is available with `max_bytes = 524288` (512 KiB) and
   `timeout_ms = 2000` per call. The registry-level hook timeout is `timeout_ms = 5000`.
5. For `lessons.md`: the hook identifies lesson entries as h2 sections (`## L-` or
   `## PG-` headings). Each h2 lesson entry must contain a `### Closes` block.
6. For the umbrella-range check: the pattern `D-\d+\.\.D-\d+` identifies a bare umbrella
   citation. The flag is present if the same sentence or paragraph also contains one of:
   `(sample)`, `(exhaustive)`, `sample-vs-exhaustive`, or
   `see decision-log.md for full range`. Absence of any of these markers on a bare range
   cite is a violation per D-441(c)+D-442(c).

## Postconditions

### lessons.md arm (file_name == "lessons.md")

1. If every h2 lesson entry (`## L-` or `## PG-` headed section) in `lessons.md` contains
   at least one `### Closes` sub-section with non-empty content, the hook emits
   `HookResult::Continue` for the Closes-block-presence check on that file.
2. If any h2 lesson entry in `lessons.md` lacks a `### Closes` sub-section (or has a
   `### Closes` section with no content body beyond the heading line), the hook emits
   `HookResult::BlockWithFix` naming the entry (by h2 heading text) and citing D-448(b).
3. If a `### Closes` block exists in a lesson entry but contains forbidden per-mechanism
   annotations (pattern: `\(per D-413\(b\) mandate\)`) or the phrase
   `N items per D-413(b)`, the hook emits `HookResult::BlockWithFix` citing D-420(e)
   and naming the offending line.

### decision-log.md arm (file_name == "decision-log.md")

4. If no bare umbrella citation `D-\d+\.\.D-\d+` is found without a sample-vs-exhaustive
   flag, the hook emits `HookResult::Continue` for the umbrella-flag check.
5. If any bare umbrella citation `D-\d+\.\.D-\d+` exists in the file WITHOUT an adjacent
   sample-vs-exhaustive flag marker, the hook emits `HookResult::BlockWithFix` naming the
   offending range and citing D-441(c)+D-442(c).

### STATE.md arm (file_name == "STATE.md")

6. Same umbrella-flag rule as postcondition 5, applied to STATE.md content. A bare
   `D-\d+\.\.D-\d+` cite without an adjacent sample-vs-exhaustive flag marker is a
   `HookResult::BlockWithFix` citing D-441(c)+D-442(c).

### INDEX.md arm (file_name == "INDEX.md")

7. Same umbrella-flag rule as postconditions 5-6, applied to INDEX.md content.

### All arms

8. If any Closes block entry contains a cite that does not match a recognized structured ID
   pattern (`D-\d+`, `F-P\d+-\d+`, `TD-VSDD-\d+`, `PG-[A-Za-z0-9-]+`, `L-EDP1-\d+`,
   `ADV-EDP1-P\d+-[A-Z]+-\d+`) — i.e., is freeform prose or an arbitrary label —
   the hook emits `HookResult::BlockWithFix` citing D-419(c) and naming the malformed entry.
9. Multiple violations in one write produce a single `HookResult::BlockWithFix` message
   enumerating all violations together.
10. If `host::read_file` returns an error for any target file (HostError of any kind), the
    hook emits `HookResult::Continue` and logs a warning via `host::log_warn` — fail-open.

## Invariants

1. The hook NEVER writes to any file. It has no `write_file` capability in its registry
   entry. It is a read-only post-write validator.
2. The hook fires PostToolUse only — it never prevents a write; it signals AFTER the write
   has completed. The dispatcher records the block signal; the author must correct and re-write.
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
   `\(per D-413\(b\) (?:completeness )?mandate\)` MUST NOT appear in Closes block cite
   lines per D-420(e).
6. Lesson-entry detection in `lessons.md` is h2-heading-based. A line matching
   `^## (L-|PG-)` begins a new lesson entry. The `### Closes` requirement applies to
   entries after the `## L-` or `## PG-` heading. Pre-existing entries that predate the
   D-448(b) codification date MUST carry an explicit documentary-historical exemption
   declaration per D-443(b) to be skipped by this validation; absence of the exemption
   declaration means the entry is in scope.
7. The documentary-historical exemption declaration per D-443(b) is recognized by the
   presence of a comment or inline note containing the literal text `(documentary-historical)`
   or `(pre-D-448(b) exemption)` adjacent to or within the non-conforming block. A silent
   omission of the `### Closes` block without this declaration is NOT a valid exemption.
8. The cite-ID validation in postcondition 8 applies ONLY to lines within `### Closes`
   blocks, not to the full file body. Lines containing NO recognizable structured ID pattern
   and also not matching a blank line or a markdown formatting line (e.g., `---`, `**...**`)
   are candidates for the malformed-cite block.
9. All `host::read_file` calls are fail-open: read errors produce Continue + log_warn, not
   Block. The total timeout budget is bounded by the registry `timeout_ms = 5000` limit.
10. All byte-index slice expressions operating on content strings MUST use
    `is_char_boundary()` guards where multi-byte UTF-8 input is possible (em-dash, en-dash,
    NBSP, typographic apostrophes are plausible in lessons.md narrative text). Slice without
    boundary guard is a runtime panic risk per the S-15.11 cascade lesson F-P4-001.

## Phase 2 — reserved (S-15.13 scope; ADR-022 gate)

Phase 2 covers **cross-cell Closes set agreement**: the hook reads the current adversary
review file to extract the canonical finding set, then cross-validates finding IDs against
the 8 prescribed citation sites (burst-log Dim-5, Closes block, STATE.md Decisions Log,
INDEX.md Convergence Status, lessons.md, decision-log.md, ARCH-INDEX, BC-INDEX).

**Phase 2 is NOT implemented in this v1.0 BC.** Its precondition is ADR-022 Option c
(state-manager writes `.factory/current-adversary-pass.txt` at Commit A — see
`decisions/ADR-022-hook-current-pass-context-discovery.md`). Phase 2 invariants will be
specified in BC-5.39.007 v1.1 as part of S-15.13 authorship.

A cite ID that appears correctly formatted (passes postcondition 8) but references a
nonexistent decision-log row or finding ID is **ADVISORY in Phase 1** (not BLOCK) — cross-cell
agreement validation requires Phase 2's cross-document resolver.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `lessons.md` h2 entry `## L-EDP1-007` has no `### Closes` section | BlockWithFix naming the entry heading and citing D-448(b) |
| EC-002 | `lessons.md` h2 entry `## L-EDP1-007` has `### Closes` heading but body is empty (no content lines) | BlockWithFix: empty Closes block is equivalent to absent; cite D-448(b) |
| EC-003 | `lessons.md` h2 entry `## PG-S-15.14-tdd-micro-commit` has `### Closes` with `- F-P1-007` cite | Continue for that entry |
| EC-004 | Closes block line reads `- F-P39-001, F-P39-002 (per D-413(b) completeness mandate)` | BlockWithFix citing D-420(e); forbidden annotation pattern matched |
| EC-005 | Closes block line reads `- 5 items per D-413(b) mandate` | BlockWithFix citing D-420(e); aggregate-shorthand pattern matched |
| EC-006 | `STATE.md` contains `D-389..D-480` with no flag | BlockWithFix: umbrella cite without sample-vs-exhaustive flag; D-441(c)+D-442(c) |
| EC-007 | `STATE.md` contains `D-389..D-480 (sample; see decision-log.md for full range)` | Continue: sample-vs-exhaustive flag present |
| EC-008 | `decision-log.md` contains `D-401..D-454 (exhaustive)` | Continue: exhaustive flag present |
| EC-009 | `INDEX.md` contains `D-389..D-454` without flag on same line or sentence | BlockWithFix citing D-441(c)+D-442(c) |
| EC-010 | Closes block line reads `- D-999` where D-999 does not exist in decision-log.md | Advisory only (Phase 1); continue (Phase 2 cross-cell validation is reserved) |
| EC-011 | Closes block line reads `- fixed the thing` (no structured ID) | BlockWithFix citing D-419(c): malformed cite (no recognized structured ID pattern) |
| EC-012 | `lessons.md` pre-D-448(b) entry has no `### Closes` but carries `(pre-D-448(b) exemption)` inline declaration | Continue: exemption declared per D-443(b); entry skipped |
| EC-013 | `lessons.md` pre-D-448(b) entry silently omits `### Closes` with no exemption declaration | BlockWithFix: silent omission is NOT a valid exemption per invariant 7 |
| EC-014 | Multiple violations across lessons.md and decision-log.md in a single write | Single BlockWithFix enumerating all violations (postcondition 9) |
| EC-015 | Path is `/some/dir/xSTATE.md` (ends_with "STATE.md" but file_name differs) | Continue (path-component-strict guard; not a target file) |
| EC-016 | `host::read_file` returns HostError::Timeout for lessons.md | Continue + log_warn; fail-open |
| EC-017 | Closes block contains `- L-EDP1-052` (valid structured ID) | Continue for that cite |
| EC-018 | `lessons.md` h2 entry `## L-EDP1-007` has `### Closes` with `- (per D-413(b) completeness mandate)` as sole cite | BlockWithFix citing D-420(e): forbidden annotation; the only cite is the forbidden pattern |
| EC-019 | Closes block line contains only a dash and whitespace (`- `) | Treat as empty/malformed cite; BlockWithFix citing D-419(c) |
| EC-020 | Multiple `### Closes` blocks within a single burst-log h2 entry | The validate-burst-log hook (BC-5.39.004) governs burst-log; this BC governs lessons.md only for Closes-presence validation. Multiple Closes blocks in a lessons.md lesson entry: only the FIRST is recognized; second is treated as body content |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Decision |
|----------|----------------|---------------------|----------|
| lessons.md entry with valid Closes | `## L-EDP1-052` entry; `### Closes` present; `- D-444 codified` line | `HookResult::Continue` | PASS |
| lessons.md entry missing Closes | `## L-EDP1-007` entry; no `### Closes` section; no exemption declaration | `HookResult::BlockWithFix` citing D-448(b) | BLOCK |
| lessons.md entry with empty Closes | `## PG-discipline` entry; `### Closes` heading with no body lines | `HookResult::BlockWithFix` citing D-448(b) | BLOCK |
| Forbidden aggregate annotation | Closes block contains `(per D-413(b) completeness mandate)` | `HookResult::BlockWithFix` citing D-420(e) | BLOCK |
| STATE.md bare umbrella cite | STATE.md `D-389..D-480` with no flag | `HookResult::BlockWithFix` citing D-441(c)+D-442(c) | BLOCK |
| STATE.md umbrella cite with sample flag | STATE.md `D-389..D-480 (sample; see decision-log.md for full range)` | `HookResult::Continue` | PASS |
| INDEX.md bare umbrella cite | INDEX.md `D-389..D-454` without adjacent flag | `HookResult::BlockWithFix` citing D-441(c)+D-442(c) | BLOCK |
| decision-log.md umbrella with exhaustive flag | `D-401..D-454 (exhaustive)` | `HookResult::Continue` | PASS |
| Malformed cite in Closes | Closes block line `- fixed the thing` (no structured ID) | `HookResult::BlockWithFix` citing D-419(c) | BLOCK |
| Cross-site staleness (Phase 2 scope) | Closes block `- D-999` where D-999 nonexistent in decision-log.md | `HookResult::Continue` (Phase 2 advisory; not Phase 1 block) | PASS (Phase 1) |
| Pre-D-448(b) exemption declared | lessons.md entry with `(pre-D-448(b) exemption)` inline note and no Closes | `HookResult::Continue` | PASS |
| Pre-D-448(b) exemption NOT declared | lessons.md entry with no Closes and no exemption declaration | `HookResult::BlockWithFix` citing D-448(b) | BLOCK |
| Read failure | `host::read_file` returns HostError::CapabilityDenied | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| xSTATE.md path | file_name is "xSTATE.md" | `HookResult::Continue` (path-component-strict guard) | PASS (not target) |
| Multiple violations in one write | Empty Closes + bare umbrella cite + malformed cite ID | Single `HookResult::BlockWithFix` enumerating all 3 violations | BLOCK |

## D-NNN Anchor Coverage

| D-NNN Sub-Clause | Gate Enforced | Postcondition |
|-----------------|---------------|---------------|
| D-419(c) | `(per D-413(b) completeness mandate)` annotation form blocked in Closes cites | PC3/PC8 |
| D-420(e) | Per-finding mechanism annotations in Closes blocks forbidden | PC3 |
| D-441(c) | Umbrella citation sites in STATE.md, INDEX.md, decision-log.md MUST carry sample-vs-exhaustive flag | PC5/PC6/PC7 |
| D-442(c) | Retroactive sweep for sample-vs-exhaustive flag across umbrella citation sites | PC5/PC6/PC7 |
| D-443(b) | Documentary-historical exemption must be explicit; silent omission forbidden | invariant 7 |
| D-448(b) | Lesson entries in lessons.md MUST have Closes block | PC1/PC2 |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | Missing-Closes-Block Invariant — hook emits BlockWithFix when lessons.md entry lacks Closes | bats integration test (fail-lessons-missing-closes fixture) |
| (pending) | Empty-Closes-Block Invariant — hook emits BlockWithFix when lessons.md Closes block exists but is empty | bats integration test (fail-lessons-empty-closes fixture) |
| (pending) | Forbidden-Annotation Invariant — hook emits BlockWithFix when Closes block contains `(per D-413(b) completeness mandate)` pattern | bats integration test (fail-forbidden-annotation fixture) |
| (pending) | Umbrella-Flag STATE.md Invariant — hook emits BlockWithFix on bare umbrella cite in STATE.md | bats integration test (fail-state-umbrella-no-flag fixture) |
| (pending) | Umbrella-Flag INDEX.md Invariant — hook emits BlockWithFix on bare umbrella cite in INDEX.md | bats integration test (fail-index-umbrella-no-flag fixture) |
| (pending) | Umbrella-Flag decision-log.md Invariant — hook emits BlockWithFix on bare umbrella cite in decision-log.md | bats integration test (fail-decisionlog-umbrella-no-flag fixture) |
| (pending) | Umbrella-Flag Pass Invariant — hook emits Continue when umbrella cite has sample-vs-exhaustive flag | bats integration test (pass-umbrella-with-flag fixture) |
| (pending) | Malformed-Cite Invariant — hook emits BlockWithFix when Closes block line has no structured ID | bats integration test (fail-malformed-cite fixture) |
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
| D-NNN Sub-Clauses Closed | D-419(c) (Closes annotation format — no aggregate mandate annotation); D-420(e) (per-finding mechanism annotations forbidden in Closes); D-441(c) (sample-vs-exhaustive flag required on umbrella cites in STATE.md+INDEX.md+decision-log.md); D-442(c) (retroactive sweep for flag); D-443(b) (documentary-historical exemption must be explicit); D-448(b) (lessons.md entries must have Closes block) |
| ADR References | ADR-022 (Phase 2 gate — current-pass pointer file; S-15.13 scope; ACCEPTED 2026-05-15) |
| Stories | S-15.12 (Phase 1); S-15.13 (Phase 2 extension, pending) |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.12 must achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.004 — governs validate-burst-log hook (sister PostToolUse hook; burst-log structural completeness including Closes block presence in burst-log entries — distinct from this BC which governs lessons.md Closes presence and format constraints on all four Closes-bearing files)
- BC-5.39.005 — governs validate-state-structure Phase 1 hook (sister PostToolUse hook)
- BC-5.39.006 — governs validate-dispatch-advance WASM hook (sister PostToolUse hook; overlapping STATE.md domain — that BC validates `current_step:` frontmatter structural validity; this BC validates Closes annotation format and umbrella cite flags in STATE.md body)

## Architecture Anchors

- `crates/hook-plugins/validate-closes-completeness/` — new hook crate (Phase 1 + Phase 2 extension); lesson-entry detection in `find_lesson_entries`; Closes-presence check in `check_closes_present`; umbrella-flag check in `check_umbrella_flag`; forbidden-annotation check in `check_forbidden_annotations`
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `tool = "Edit|Write"` and four file targets `decision-log.md`, `STATE.md`, `INDEX.md`, `lessons.md`
- `specs/architecture/decisions/ADR-022-hook-current-pass-context-discovery.md` — Phase 2 gate; current-pass pointer file protocol

## Story Anchor

S-15.12 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M3 story; Phase 1 scope)
S-15.13 — v1.0-brownfield-backfill (Phase 2 extension; BLOCKED on ADR-022 + S-15.12 SHIPPED)

## VP Anchors

VP IDs pending VP-INDEX allocation by state-manager at S-15.12 post-merge burst.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-18 | Initial authoring (product-owner; brownfield-backfill S-15.03 M3 wave 3M3a BC authoring). Anchors D-419(c)+D-420(e)+D-441(c)+D-442(c)+D-443(b)+D-448(b). BC-5.39.007 allocated as next monotonic ID after BC-5.39.006 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.12 merge). Phase 2 (cross-cell agreement) reserved for v1.1 in S-15.13 scope per ADR-022 Option c. Preemptive cascade lessons applied: path-component-strict guard; is_char_boundary() invariant 10; fail-open invariant 9; 524288 max_bytes; Phase-1-advisory for cross-site staleness (postcondition and EC-010). |
