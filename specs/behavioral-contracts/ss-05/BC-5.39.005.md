---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: story-writer
timestamp: 2026-05-17T00:00:00Z
phase: section-12-step-3
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
  - .factory/cycles/v1.0-brownfield-backfill/architect-m2-2026-05-16.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/stories/S-15.09-validate-state-structure-phase-1.md
input-hash: "5af355e"
traces_to: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
extracted_from: .factory/stories/S-15.09-validate-state-structure-phase-1.md
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - 2026-05-17
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.005
section: "5.39"
last_amended: "2026-05-17 (v1.1) — pass-2 fix-burst: add EC-015 for SIZE BUDGET banner narrative-arrow class (F-P2-004 MEDIUM). Banner-narrative arrow-digit sequences (e.g., `(363→310 lines)` D-NNN compaction narratives) MUST NOT be misidentified as canonical trajectory tail. Discriminator: trajectory-tail predicate requires >=3 `→(\\d+)` matches AND canonical body-line location, not banner-block prose."
---

# BC-5.39.005: validate-state-structure Phase 1 hook MUST block on banner line-count drift, dual-margin absence, and trajectory-tail cardinality violations in STATE.md

## Description

The `validate-state-structure` WASM hook (Phase 1) enforces that any Edit/Write to `STATE.md`
does not leave three classes of structural violations that the F5 engine-discipline cycle has
surfaced repeatedly as adversary findings:

1. **Banner line-count discipline** (D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+D-440(d)+D-442(d)):
   the banner line-count claim in STATE.md MUST match the actual number of lines in the file as
   written. A claim of "N lines" that diverges from the actual newline count is a violation.

2. **Dual-margin form** (D-446(c)): the banner MUST contain both margin values — the "from soft
   target" margin and the "from hard target" margin (or the canonical two-margin arithmetic form
   established by D-446(c)). A banner with only one margin value is a violation.

3. **Trajectory-tail LENGTH=4** (D-433(e)+D-439(c)+D-451(c)+D-432(b)): the trajectory-tail
   string in STATE.md MUST contain exactly 4 arrow-separated numeric components. The canonical
   form is `→N→N→N→N` where each N is a non-negative integer and the pattern contains exactly
   four `→(\d+)` matches. A trajectory tail with 3 or 5 components is a violation.

If any of these properties is violated, the hook emits a `block_with_fix` signal naming the
specific violation class and the required remediation. This BC closes the recurring class of
adversary findings in the F5 engine-discipline cycle where STATE.md structural violations were
discovered by the adversary N bursts after the write, rather than at write time.

## Preconditions

1. A PostToolUse Edit/Write event has fired on a file whose `file_name` path component is exactly
   `STATE.md` (path-component-strict matching via
   `Path::new(file_path).file_name() == Some("STATE.md")` — NOT suffix-`ends_with`. Paths
   like `/some/dir/xSTATE.md` MUST NOT match per Q5/Q6 canonical lock +
   `crates/hook-plugins/validate-state-structure/src/lib.rs`:`is_state_md_target`).
2. The dispatcher has invoked the `validate-state-structure` WASM plugin with the write payload.
3. The STATE.md file content is available — either from the write payload or via `host::read_file`.
4. `host::read_file` is available with `max_bytes = 65536` and `timeout_ms = 2000` per call.

## Postconditions

1. If ALL of the following hold, the hook emits `HookResult::Continue` (pass):
   - The banner line-count claim extracted from STATE.md content matches the actual number of
     lines (newlines) in the full content string.
   - The banner contains both margin values in the canonical dual-margin form per D-446(c).
   - The trajectory-tail string contains exactly 4 `→(\d+)` matches (LENGTH=4 per D-433(e)).
2. If the banner line-count claim does not match the actual line count, the hook emits
   `HookResult::BlockWithFix` with a message naming the claimed count, the actual count, and
   the instruction to reconcile the banner.
3. If the banner does not contain both margin values (dual-margin form absent), the hook emits
   `HookResult::BlockWithFix` with a message naming the missing margin form and citing D-446(c).
4. If the trajectory-tail does not contain exactly 4 `→(\d+)` matches, the hook emits
   `HookResult::BlockWithFix` with a message naming the actual match count, the required count
   (4), and citing D-433(e)+D-439(c).
5. Multiple violations in one write produce a single `HookResult::BlockWithFix` message
   enumerating all violations together.
6. If `host::read_file` returns an error for the STATE.md file (HostError of any kind), the
   hook emits `HookResult::Continue` and logs a warning via `host::log_warn` — a read failure
   is NOT treated as a structural violation (fail-open).

## Invariants

1. The hook NEVER writes to any file. It has no `write_file` capability in its registry entry.
   It is a read-only post-write validator.
2. The hook fires PostToolUse only — it never prevents a write; it signals AFTER the write
   has completed. The dispatcher records the block signal; the author must correct and re-write.
3. Line-count computation uses newline character counting (`\n` count) on the raw content string;
   the claim is extracted via a regex anchored to the banner line (e.g., a pattern matching
   `(\d+) lines` or the canonical banner form). The comparison is: extracted_integer == actual_newline_count.
4. Dual-margin detection matches the presence of two distinct margin expressions in the banner
   block: one relative to the soft target and one relative to the hard target (or the canonical
   arithmetic dual-margin form per D-446(c)). Both must be present for the banner to be valid.
5. Trajectory-tail detection uses regex `→(\d+)` applied globally to the trajectory-tail line
   or string. The match count must equal exactly 4. A tail with 3 matches (LENGTH=3) or 5
   matches (LENGTH=5) is a violation.
6. The `is_state_md_target` function implements path-component-strict matching:
   `Path::new(file_path).file_name() == Some("STATE.md")`. Using `ends_with("STATE.md")`
   on the raw path string MUST NOT be substituted (false-positive on `xSTATE.md`).
7. All `host::read_file` calls are fail-open: read errors produce Continue + log_warn, not Block.
   The total timeout budget is bounded by the registry `timeout_ms = 5000` limit.
8. All byte-index slice expressions operating on content strings MUST use `is_char_boundary()`
   guards where multi-byte UTF-8 input is possible (em-dash, en-dash, NBSP, typographic
   apostrophes are plausible in banner narrative text). Slice without boundary guard is a
   runtime panic risk per S-15.11 cascade lesson F-P4-001.
9. The trajectory-tail predicate MUST discriminate canonical body-line tails from banner-block
   narrative arrow-digit sequences. A line such as `(363→310 lines)` in the SIZE BUDGET banner
   block is a D-NNN compaction authorization narrative, NOT a trajectory tail. The predicate
   anchors on the trajectory-tail body line (not banner-block prose) and requires >=3 `→(\d+)`
   matches to qualify as a candidate; banner-narrative sequences with fewer arrow-digit groups
   (typically 1) MUST NOT satisfy the trajectory-tail LENGTH=4 check.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Banner claims "399 lines" but file has 400 newlines | BlockWithFix: "banner claims 399 lines but actual line count is 400 — reconcile banner" |
| EC-002 | Banner claims "400 lines" and file has 400 newlines | Continue (line-count correct; check other validations) |
| EC-003 | Banner contains only one margin value (soft-target margin absent) | BlockWithFix: "dual-margin form missing: banner must contain both margins per D-446(c)" |
| EC-004 | Banner contains both margin values in canonical form | Continue for dual-margin; check other validations |
| EC-005 | Trajectory-tail has 3 `→N` components (LENGTH=3) | BlockWithFix: "trajectory-tail has 3 components; required LENGTH=4 per D-433(e)" |
| EC-006 | Trajectory-tail has 5 `→N` components (LENGTH=5) | BlockWithFix: "trajectory-tail has 5 components; required LENGTH=4 per D-433(e)" |
| EC-007 | Trajectory-tail has exactly 4 `→N` components | Continue for trajectory-tail; check other validations |
| EC-008 | No trajectory-tail line found in STATE.md content | BlockWithFix: "no trajectory-tail found; expected `→N→N→N→N` form per D-432(b)" |
| EC-009 | All three validations fail simultaneously | Single BlockWithFix enumerating all 3 violation classes |
| EC-010 | `host::read_file` returns HostError::CapabilityDenied | Continue + log_warn; fail-open |
| EC-011 | `host::read_file` returns HostError::Timeout | Continue + log_warn; fail-open |
| EC-012 | File path is `/some/dir/xSTATE.md` (ends with STATE.md but file_name component differs) | Continue (is_state_md_target returns false; path-component-strict guard) |
| EC-013 | Banner narrative contains em-dash or other multi-byte UTF-8 characters | No panic; is_char_boundary() guard applied before any byte-index slice |
| EC-014 | STATE.md is newly created with empty content (no banner, no trajectory-tail) | BlockWithFix: both line-count and trajectory-tail violations (no banner line found; no tail found) |
| EC-015 | SIZE BUDGET banner contains narrative arrow-digit sequence (e.g., `(363→310 lines)` D-NNN compaction authorization narrative) AND body contains canonical `→N→N→N→N` trajectory tail | Continue for trajectory-tail; canonical tail in body satisfies LENGTH=4 invariant; banner-narrative arrow MUST NOT be misidentified as the canonical trajectory tail — predicate anchors on body-line location and requires >=3 `→(\d+)` matches, which banner-narrative sequences (typically 1 arrow-digit group) do not satisfy |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Decision |
|----------|----------------|---------------------|----------|
| All valid | Banner "400 lines" with file having 400 newlines; dual-margin present; tail `→9→9→9→9` | `HookResult::Continue` | PASS |
| Line-count off by 1 | Banner "399 lines" but file has 400 newlines; rest valid | `HookResult::BlockWithFix` citing line-count discrepancy | BLOCK |
| Dual-margin absent | Correct line-count; banner has only one margin; tail `→9→9→9→9` | `HookResult::BlockWithFix` citing missing dual-margin | BLOCK |
| Tail LENGTH=3 | Correct line-count; dual-margin present; tail `→9→9→9` | `HookResult::BlockWithFix` citing 3 components vs required 4 | BLOCK |
| Tail LENGTH=5 | Correct line-count; dual-margin present; tail `→9→9→9→9→9` | `HookResult::BlockWithFix` citing 5 components vs required 4 | BLOCK |
| No tail found | Correct line-count; dual-margin present; no trajectory-tail line | `HookResult::BlockWithFix` citing missing trajectory-tail | BLOCK |
| All 3 violations | Wrong line-count + no dual-margin + wrong tail length | Single `HookResult::BlockWithFix` enumerating all 3 violations | BLOCK |
| Read failure | `host::read_file` returns HostError::CapabilityDenied | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| xSTATE.md path | file_name is "xSTATE.md" | `HookResult::Continue` (is_state_md_target false) | PASS (not target) |
| Banner narrative arrow + valid body tail | Banner contains `(363→310 lines)` compaction narrative; body contains `→9→9→9→9` tail | `HookResult::Continue` — banner-narrative arrow not misidentified as tail; canonical tail satisfies LENGTH=4 | PASS |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | Line-Count Block Invariant — hook emits BlockWithFix when banner count diverges from actual | bats integration test (fail-banner-wc fixture) |
| (pending) | Line-Count Pass Invariant — hook emits Continue when banner count matches actual | bats integration test (pass-all-valid fixture) |
| (pending) | Dual-Margin Block Invariant — hook emits BlockWithFix when dual-margin absent | bats integration test (fail-no-dual-margin fixture) |
| (pending) | Trajectory-Tail Block Invariant — hook emits BlockWithFix when tail is not LENGTH=4 | bats integration test (fail-tail-3-components + fail-tail-5-components fixtures) |
| (pending) | Fail-open Invariant — hook emits Continue when file is unreadable | bats integration test (fail-open-unreadable fixture) |
| (pending) | Banner-Narrative Arrow Discrimination Invariant — hook emits Continue when banner contains narrative arrow-digit sequence but body has canonical LENGTH=4 tail (EC-015) | bats integration test (pass-banner-narrative-arrow fixture) |

VP IDs are pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — STATE.md structural validation automation sub-capability) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes the PostToolUse gate that mechanically prevents the STATE.md banner line-count, dual-margin, and trajectory-tail structural violation classes codified in D-421(c), D-422(c), D-424(b), D-428(d), D-438(a), D-440(d), D-442(d), D-446(c), D-433(e), D-439(c), D-451(c), and D-432(b). The hook targets STATE.md writes — a governance artifact, not a runtime subsystem artifact. |
| Architecture Module | `crates/hook-plugins/validate-state-structure/` (Rust WASM plugin); `plugins/vsdd-factory/hooks-registry.toml` (registry entry); `plugins/vsdd-factory/hook-plugins/validate-state-structure.wasm` (compiled binary) |
| D-NNN Sub-Clauses Closed | D-421(c) (banner wc-l discipline); D-422(c) (banner wc-l discipline); D-424(b) (banner wc-l discipline); D-428(d) (banner wc-l discipline); D-438(a) (banner wc-l discipline); D-440(d) (banner wc-l discipline); D-442(d) (banner wc-l discipline); D-446(c) (dual-margin form); D-433(e) (trajectory-tail LENGTH=4); D-439(c) (trajectory-tail LENGTH=4); D-451(c) (trajectory-tail extension); D-432(b) (trajectory-tail canonical form) |
| Stories | S-15.09 |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.09 must
  achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.003 — governs validate-index-cite-refresh hook (sister PostToolUse hook; structural
  analog for version-cite staleness detection)
- BC-5.39.004 — governs validate-burst-log hook (sister PostToolUse hook; structural analog for
  burst-log entry completeness; same crate scaffolding pattern and path-component-strict guard)
- BC-4.11.001 — validates write targets against artifact-path-registry (sister PostToolUse hook;
  structural analog for path validation)

## Architecture Anchors

- `crates/hook-plugins/validate-state-structure/src/lib.rs` — hook implementation (pure logic functions + effectful orchestration)
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `tool = "Edit|Write"` (canonical Q5 form)

## Story Anchor

S-15.09 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M2 Wave-3)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.1 | 2026-05-17 | F-P2-004 fix-burst: add EC-015 for SIZE BUDGET banner narrative-arrow class (banner-block trajectory predicate discriminator codified). Banner-narrative arrow-digit sequences (e.g., `(363→310 lines)` D-NNN compaction authorization narratives) MUST NOT be misidentified as canonical trajectory tail. Discriminator: predicate requires >=3 `→(\d+)` matches AND body-line anchor. Added Invariant 9 (narrative-arrow discrimination). Added EC-015 edge case row. Added banner-narrative-arrow test vector. Added Banner-Narrative Arrow Discrimination VP row. |
| 1.0 | 2026-05-17 | Initial authoring (story-writer; brownfield-backfill S-15.03 M2 wave-3 story authoring). Anchors D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+D-440(d)+D-442(d)+D-446(c)+D-433(e)+D-439(c)+D-451(c)+D-432(b). BC-5.39.005 allocated as next monotonic ID after BC-5.39.004 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.09 merge). Preemptive cascade lessons applied: path-component-strict precondition (is_state_md_target); is_char_boundary() invariant 8; fail-open invariant 7. |
