---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: story-writer
timestamp: 2026-05-16T00:00:00Z
phase: section-12-step-3
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
  - .factory/cycles/v1.0-brownfield-backfill/architect-m2-2026-05-16.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/stories/S-15.11-validate-burst-log.md
input-hash: "ad1c745"
traces_to: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
extracted_from: .factory/stories/S-15.11-validate-burst-log.md
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - 2026-05-16
  - 2026-05-16
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.004
section: "5.39"
last_amended: "2026-05-16 (v1.1) — F-S15.11-LOCAL-P3-001 closure: BC precondition 1 path-language migrated from string-ends_with to path-component-strict (Path::file_name() == Some(\"burst-log.md\")); aligns BC body with implementation is_burst_log_target semantics and story spec v1.1 narrative."
---

# BC-5.39.004: validate-burst-log hook MUST block on structurally incomplete burst-log entries

## Description

The `validate-burst-log` WASM hook enforces that any Edit/Write to a `burst-log.md` file does not
leave a structurally incomplete burst entry. The hook fires PostToolUse and validates three
structural properties of the latest burst entry:

1. **h2 heading format** (D-421(e)+D-438(d)+D-439(a)): the latest h2 heading must match the
   canonical pattern `## Burst: <description> (YYYY-MM-DD)`.

2. **9-block completeness** (D-444(c)+D-446(a)): every burst entry must contain all 9 required
   bold-heading block types: Parent-commit, Adversary verdict, Files touched (Dim-1), Codifications,
   Dim-2 Attestation, Dim-5 Attestation, Dim-6 Attestation, Dim-7 Attestation, Closes.

3. **Dim-1 cardinality parity** (D-432(e)+D-448(d)(i)): the integer in the Dim-1 headline
   ("N unique files") must equal the count of files in the Dim-1 enumerated list.

If any of these properties is violated, the hook emits a `block_with_fix` signal naming the
specific violation and the required remediation. This BC closes the recurring class of adversary
findings in the F5 engine-discipline cycle where burst-log structural incompleteness was discovered
N bursts after the write rather than at write time.

## Preconditions

1. A PostToolUse Edit/Write event has fired on a file whose `file_name` path component is exactly
   `burst-log.md` (path-component-strict matching via
   `Path::new(file_path).file_name() == Some("burst-log.md")` — NOT suffix-`ends_with`. Paths
   like `/some/dir/xburst-log.md` MUST NOT match per Q5/Q6 canonical lock +
   `crates/hook-plugins/validate-burst-log/src/lib.rs`:`is_burst_log_target`).
2. The dispatcher has invoked the `validate-burst-log` WASM plugin with the write payload.
3. The burst-log file content is available — either from the write payload or via `host::read_file`.
4. `host::read_file` is available with `max_bytes = 65536` and `timeout_ms = 2000` per call.

## Postconditions

1. If ALL of the following hold, the hook emits `HookResult::Continue` (pass):
   - The latest h2 heading matches the pattern `^## Burst: .+\(\d{4}-\d{2}-\d{2}\)`.
   - All 9 required block types are present in the latest burst entry (from latest h2 to next
     h2 or EOF): Parent-commit, Adversary verdict, Files touched (Dim-1), Codifications,
     Dim-2, Dim-5, Dim-6, Dim-7, Closes — detected via `\*\*TOKEN\*\*` bold-heading pattern.
   - The Dim-1 headline integer equals the count of list items in the Dim-1 block body.
2. If the latest h2 heading does not match the canonical format, the hook emits
   `HookResult::BlockWithFix` with a message naming the malformed heading and the required
   format `## Burst: <description> (YYYY-MM-DD)`.
3. If any required block type is absent from the latest burst entry, the hook emits
   `HookResult::BlockWithFix` with a message naming each missing block type by name.
4. If the Dim-1 headline integer does not equal the list item count, the hook emits
   `HookResult::BlockWithFix` naming the headline count, the actual list count, and the
   instruction to reconcile them.
5. Multiple violations in one write produce a single `HookResult::BlockWithFix` message
   enumerating all violations together.
6. If `host::read_file` returns an error for the burst-log file (HostError of any kind), the
   hook emits `HookResult::Continue` and logs a warning via `host::log_warn` — a read failure
   is NOT treated as a structural violation (fail-open).

## Invariants

1. The hook NEVER writes to any file. It has no `write_file` capability in its registry entry.
   It is a read-only post-write validator.
2. The hook fires PostToolUse only — it never prevents a write; it signals AFTER the write
   has completed. The dispatcher records the block signal; the author must correct and re-write.
3. Block type detection uses exactly 9 bold-heading tokens:
   `Parent-commit`, `Adversary verdict`, `Files touched (Dim-1)`, `Codifications`,
   `Dim-2`, `Dim-5`, `Dim-6`, `Dim-7`, `Closes`. No other block name patterns match.
   The Dim-2/5/6/7 tokens use prefix-match (contains `**Dim-N`) to accommodate
   attestation-suffix variants such as `**Dim-2 Attestation**`.
4. Only the latest burst entry is validated (from the last `## Burst:` h2 to the next h2 or
   EOF). Prior burst entries are not re-validated on each write.
5. All `host::read_file` calls are fail-open: read errors produce Continue + log_warn, not Block.
   The total timeout budget is bounded by the registry `timeout_ms = 5000` limit.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | burst-log.md newly created with only h2 heading and no blocks | BlockWithFix naming all 9 missing block types |
| EC-002 | h2 heading is `## Fix Burst: description` (wrong prefix, no parenthesized date) | BlockWithFix naming malformed heading and required format |
| EC-003 | h2 heading is `## Burst: Pass-41 fix burst (2026-05-12)` — correct format | Continue (if blocks and Dim-1 also valid) |
| EC-004 | 8 of 9 blocks present; Closes block absent | BlockWithFix naming "Closes" as missing block |
| EC-005 | Dim-1 headline "5 unique files" but enumerated list has 7 items | BlockWithFix: "Dim-1 headline states 5 but enumerated list has 7 files" |
| EC-006 | Dim-1 headline "7 unique files" and list has exactly 7 items | Continue for Dim-1; rest of validation proceeds |
| EC-007 | File content truncated at 65536-byte cap; missing blocks past cap | Hook validates what it can; emits log_warn if truncation suspected; may emit false Continue |
| EC-008 | Two burst entries; old entry incomplete; latest entry complete | Hook validates only latest entry; pass |
| EC-009 | `host::read_file` returns HostError::Timeout | Continue + log_warn; fail-open |
| EC-010 | No h2 heading at all in burst-log.md | BlockWithFix: no canonical h2 found |
| EC-011 | Multiple violations in one write (bad h2 + missing blocks + Dim-1 mismatch) | Single BlockWithFix enumerating all violations |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Decision |
|----------|----------------|---------------------|----------|
| Complete valid entry | h2 `## Burst: Pass-41 fix burst (2026-05-12)`; all 9 blocks; Dim-1 "3 unique files" with 3-item list | `HookResult::Continue` | PASS |
| Only 6 blocks | Correct h2; 6 blocks (Dim-2, Dim-5, Dim-6 absent) | `HookResult::BlockWithFix` naming Dim-2, Dim-5, Dim-6 as missing | BLOCK |
| Dim-1 mismatch | All 9 blocks; Dim-1 "5 unique files" but 7-item list | `HookResult::BlockWithFix` citing headline=5, list=7 | BLOCK |
| Malformed h2 | `## Fix Burst: description` (no date parentheses) | `HookResult::BlockWithFix` naming malformed heading | BLOCK |
| Prior entry incomplete, current complete | Old incomplete h2+entry; then `## Burst: current (2026-05-16)` with 9 blocks | `HookResult::Continue` (only latest validated) | PASS |
| No h2 present | Blocks present but no `## Burst:` heading | `HookResult::BlockWithFix` — no canonical h2 found | BLOCK |
| Read failure | `host::read_file` returns HostError::CapabilityDenied | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| Missing only Closes | 8 blocks present; only Closes absent | `HookResult::BlockWithFix` naming Closes as missing | BLOCK |
| All violations combined | Bad h2 + 3 missing blocks + Dim-1 mismatch | Single `HookResult::BlockWithFix` enumerating all 5 sub-violations | BLOCK |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | Block Invariant — hook emits BlockWithFix when any required block is absent | bats integration test (fail-6-blocks fixture) |
| (pending) | Pass Invariant — hook emits Continue when entry is structurally complete | bats integration test (pass-complete-entry fixture) |
| (pending) | Cardinality Invariant — hook emits BlockWithFix when Dim-1 count mismatches | bats integration test (fail-dim1-cardinality fixture) |
| (pending) | Fail-open Invariant — hook emits Continue when file is unreadable | bats integration test (fail-open-unreadable fixture) |

VP IDs are pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — burst-log structural completeness automation sub-capability) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes the PostToolUse gate that mechanically prevents the burst-log structural incompleteness class codified in D-421(e), D-438(d), D-439(a), D-444(c), D-446(a), D-432(e), D-448(d)(i), and D-443(e)(ii). The hook targets burst-log.md writes — a governance artifact, not a runtime subsystem artifact. |
| Architecture Module | `crates/hook-plugins/validate-burst-log/` (Rust WASM plugin); `plugins/vsdd-factory/hooks-registry.toml` (registry entry); `plugins/vsdd-factory/hook-plugins/validate-burst-log.wasm` (compiled binary) |
| D-NNN Sub-Clauses Closed | D-421(e) (h2 heading format gate); D-438(d) (h2 canonical form); D-439(a) (h2 enforcement); D-444(c) (9-block completeness); D-446(a) (own-burst completeness gate); D-432(e) (Dim-1 cardinality); D-448(d)(i) (Dim-1 source-attestation parity); D-443(e)(ii) (own-burst h2 real-time at Commit A) |
| Stories | S-15.11 |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.11 must
  achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.003 — governs validate-index-cite-refresh hook (sister PostToolUse hook; structural
  analog for version-cite staleness detection vs burst-log completeness detection)
- BC-4.11.001 — validates write targets against artifact-path-registry (sister PostToolUse hook;
  structural analog for path validation)

## Architecture Anchors

- `crates/hook-plugins/validate-burst-log/src/lib.rs` — hook implementation (pure logic functions + effectful orchestration)
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `tool = "Edit|Write"` (canonical Q5 form)

## Story Anchor

S-15.11 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M2 Wave-2)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.1 | 2026-05-16 | F-S15.11-LOCAL-P3-001 closure: BC precondition 1 path-language migrated from string-ends_with to path-component-strict (`Path::file_name() == Some("burst-log.md")`); aligns BC body with implementation `is_burst_log_target` semantics and story spec v1.1 narrative. |
| 1.0 | 2026-05-16 | Initial authoring (story-writer; brownfield-backfill S-15.03 M2 wave-2 story authoring). Anchors D-421(e)+D-438(d)+D-439(a)+D-444(c)+D-446(a)+D-432(e)+D-448(d)(i)+D-443(e)(ii). BC-5.39.004 allocated as next monotonic ID after BC-5.39.003 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.11 merge). |
