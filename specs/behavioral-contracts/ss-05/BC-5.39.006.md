---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-17T00:00:00Z
phase: section-12-step-3
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.004.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.003.md
input-hash: "5af355e"
traces_to: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
extracted_from: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
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
bc_id: BC-5.39.006
section: "5.39"
last_amended: "2026-05-17 (v1.0) — Initial authoring (product-owner; brownfield-backfill S-15.03 M2 wave-4 story authoring). Anchors D-440(a)+D-441(a)+D-442(a)+D-443(a)+D-439(b)+D-441(b)+D-451(c). BC-5.39.006 allocated as next monotonic ID after BC-5.39.005 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.14 merge)."
---

# BC-5.39.006: validate-dispatch-advance WASM hook MUST block on forbidden meta-commentary in current_step, missing 4-index version citations, trajectory-tail cardinality violations, stale D-chain cites in STATE.md, and non-6-column adversary-pass rows in INDEX.md

## Description

The `validate-dispatch-advance` WASM hook enforces that any Edit/Write to `STATE.md` does not
leave a structurally invalid `current_step:` frontmatter field, and that any Edit/Write to a
cycle `INDEX.md` does not leave adversary-pass rows with the wrong column count. The hook fires
PostToolUse and validates two classes of artifacts:

1. **STATE.md `current_step:` field validation** (D-440(a)+D-441(a)+D-442(a)+D-443(a)+D-439(b)+D-451(c)):
   the `current_step:` frontmatter value MUST NOT contain forbidden meta-commentary patterns, MUST
   cite all 4 index versions, MUST have a trajectory-tail with exactly 4 arrow-separated values, and
   MUST cite a current D-chain range that is not stale (i.e., includes the latest D-NNN).

2. **INDEX.md adversary-pass row validation** (D-441(b)): every adversary-pass row in a cycle
   `INDEX.md` MUST use the strict 6-column schema. Rows with 5 or 7 columns are a violation.

If any of these properties is violated, the hook emits a `block_with_fix` signal naming the
specific violation class and the required remediation. This BC closes the recurring class of
adversary findings in the F5 engine-discipline cycle where `current_step:` meta-commentary
labels, missing index cites, trajectory-tail cardinality drift, stale D-chain cites, and
malformed INDEX.md adversary-pass rows were discovered by the adversary N bursts after the write
rather than at write time.

## Preconditions

1. A PostToolUse Edit/Write event has fired on a file whose `file_name` path component is exactly
   `STATE.md` (path-component-strict matching via
   `Path::new(file_path).file_name() == Some("STATE.md")` — NOT suffix-`ends_with`. Paths like
   `/some/dir/xSTATE.md` MUST NOT match), OR on a file whose `file_name` path component is
   exactly `INDEX.md` (same path-component-strict guard:
   `Path::new(file_path).file_name() == Some("INDEX.md")`).
2. The dispatcher has invoked the `validate-dispatch-advance` WASM plugin with the write payload.
3. The file content is read via `host::read_file` (filesystem-authoritative). The hook does NOT
   inspect the payload's `tool_input.content` field; the filesystem value is the source of truth
   for validation.
4. `host::read_file` is available with `max_bytes = 524288` (512 KiB; matching the cap
   established by BC-5.39.005 F-P5-002 fix-burst for STATE.md; sufficient for INDEX.md) and
   `timeout_ms = 2000` per call. The whole-hook registry-level timeout is `timeout_ms = 5000`
   (hooks-registry.toml; distinct from per-call file-read timeout).

## Postconditions

### STATE.md path (file_name == "STATE.md")

1. If ALL of the following hold, the hook emits `HookResult::Continue` (pass):
   - The `current_step:` frontmatter value does NOT match the forbidden meta-commentary regex
     `META-LEVEL-\d+ WATCH|self-app TEST|expected verdict`.
   - All 4 index version patterns are present in the `current_step:` value:
     BC-INDEX vX, VP-INDEX vX, STORY-INDEX vX, ARCH-INDEX vX (where X is a version string).
   - The trajectory-tail string within `current_step:` contains exactly 4 `→(\d+)` matches
     (LENGTH=4 per D-451(c); canonical form `→N→N→N→N`).
   - The D-chain cite in `current_step:` is not stale — it includes a reference to the latest
     D-NNN recorded in the cycle decision-log (validated by checking that the cited D-NNN range
     upper bound matches the most recently codified decision visible in the file or, if
     inaccessible, that the cite pattern `D-382..D-\d+` is present and the terminal integer is
     >= the previously-observed maximum).
2. If the `current_step:` value matches the forbidden meta-commentary regex, the hook emits
   `HookResult::BlockWithFix` naming the offending pattern and citing D-440(a)+D-441(a)+D-442(a).
3. If any of the 4 index version patterns is absent from `current_step:`, the hook emits
   `HookResult::BlockWithFix` naming each missing index cite and citing D-439(b).
4. If the trajectory-tail in `current_step:` does not contain exactly 4 `→(\d+)` matches, the
   hook emits `HookResult::BlockWithFix` naming the actual match count, the required count (4),
   and citing D-451(c).
5. If the D-chain cite in `current_step:` is stale (upper bound does not include the latest
   D-NNN), the hook emits `HookResult::BlockWithFix` naming the stale cite and citing D-443(a).
6. Multiple violations in one write produce a single `HookResult::BlockWithFix` message
   enumerating all violations together.
7. If `host::read_file` returns an error for STATE.md (HostError of any kind), the hook emits
   `HookResult::Continue` and logs a warning via `host::log_warn` — fail-open.

### INDEX.md path (file_name == "INDEX.md")

8. If every adversary-pass row in the INDEX.md content is a 6-column table row, the hook emits
   `HookResult::Continue` (pass).
9. If any adversary-pass row has a column count other than 6, the hook emits
   `HookResult::BlockWithFix` naming the row (by h2 context or line position), the actual
   column count, the required count (6), and citing D-441(b).
10. If `host::read_file` returns an error for INDEX.md (HostError of any kind), the hook emits
    `HookResult::Continue` and logs a warning via `host::log_warn` — fail-open.

## Invariants

1. The hook NEVER writes to any file. It has no `write_file` capability in its registry entry.
   It is a read-only post-write validator.
2. The hook fires PostToolUse only — it never prevents a write; it signals AFTER the write has
   completed. The dispatcher records the block signal; the author must correct and re-write.
3. Path-component-strict matching is applied to BOTH trigger paths:
   `Path::new(file_path).file_name() == Some("STATE.md")` for the STATE.md arm, and
   `Path::new(file_path).file_name() == Some("INDEX.md")` for the INDEX.md arm. Using
   `ends_with("STATE.md")` or `ends_with("INDEX.md")` on the raw path string MUST NOT be
   substituted (false-positive on `xSTATE.md` / `xINDEX.md`).
4. The forbidden meta-commentary regex is anchored to the `current_step:` frontmatter field
   only — NOT applied to the full STATE.md body content. The regex pattern is:
   `META-LEVEL-\d+ WATCH|self-app TEST|expected verdict`. These exact tokens are verbatim
   per D-440(a); the hook MUST NOT expand or narrow this set without a spec amendment.
5. The 4 index version patterns required in `current_step:` are: the literal strings
   `BC-INDEX v`, `VP-INDEX v`, `STORY-INDEX v`, `ARCH-INDEX v` (each followed by a version
   token). All 4 must be present. Missing any one is a violation per D-439(b).
6. Trajectory-tail detection within `current_step:` uses regex `→(\d+)` applied globally to
   the `current_step:` value. The match count must equal exactly 4. A tail with 3 matches
   (LENGTH=3) or 5 matches (LENGTH=5) is a violation per D-451(c).
7. D-chain currency validation: the `current_step:` value MUST contain a D-chain range citation
   of the form `D-382..D-N` where N >= the latest D-NNN codified in the cycle. Since the hook
   cannot load the full decision-log at WASM runtime, the staleness check is: if the `current_step:`
   does NOT contain the pattern `D-382\.\.D-\d+`, the cite is absent and is a violation. If the
   pattern is present, the hook records the terminal integer. Cross-burst staleness (terminal
   integer below the latest known D-NNN) is detected by comparing against the highest D-NNN
   observable in STATE.md itself (e.g., in the Decisions Log table rows). If no D-NNN appears
   higher than the terminal integer in `current_step:`, the cite is treated as current. This
   design is fail-open for staleness to avoid false-positive blocks on legitimate in-progress
   writes; the adversary cycle catches genuine staleness at pass time.
8. Adversary-pass row column detection in INDEX.md: row detection uses `|` pipe-count per line.
   A line whose trimmed content begins and ends with `|` and whose `|` count is not 8 (i.e.,
   7 internal pipes = 6 columns + 2 border pipes) is a violation row. Header rows (`| --- |`
   pattern) and non-row lines are excluded from validation.
9. All `host::read_file` calls are fail-open: read errors produce Continue + log_warn, not Block.
   The total timeout budget is bounded by the registry `timeout_ms = 5000` limit.
10. All byte-index slice expressions operating on content strings MUST use `is_char_boundary()`
    guards where multi-byte UTF-8 input is possible (em-dash, en-dash, NBSP, typographic
    apostrophes are plausible in STATE.md narrative text). Slice without boundary guard is a
    runtime panic risk per the S-15.11 cascade lesson F-P4-001.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `current_step:` contains literal text `META-LEVEL-5 WATCH: self-application` | BlockWithFix citing forbidden meta-commentary pattern and D-440(a)+D-441(a)+D-442(a) |
| EC-002 | `current_step:` contains `self-app TEST` substring | BlockWithFix citing forbidden pattern |
| EC-003 | `current_step:` contains `expected verdict` substring | BlockWithFix citing forbidden pattern |
| EC-004 | `current_step:` has BC-INDEX, VP-INDEX, STORY-INDEX v cites but omits ARCH-INDEX v | BlockWithFix naming ARCH-INDEX as missing cite; D-439(b) |
| EC-005 | `current_step:` has all 4 index cites | Continue for index-cite check; validate other conditions |
| EC-006 | `current_step:` trajectory-tail has 3 `→N` groups (LENGTH=3) | BlockWithFix: "trajectory-tail has 3 components; required LENGTH=4 per D-451(c)" |
| EC-007 | `current_step:` trajectory-tail has 5 `→N` groups (LENGTH=5) | BlockWithFix: "trajectory-tail has 5 components; required LENGTH=4 per D-451(c)" |
| EC-008 | `current_step:` has no D-chain cite matching `D-382..D-N` | BlockWithFix citing absent D-chain cite; D-443(a) |
| EC-009 | `current_step:` has `D-382..D-476` but STATE.md Decisions Log table shows row D-477 | BlockWithFix citing stale D-chain; D-443(a); terminal integer 476 < latest 477 |
| EC-010 | `current_step:` has `D-382..D-477`; highest D-NNN in STATE.md body is 476 | Continue for D-chain cite (current per invariant 7 fail-open staleness check) |
| EC-011 | All `current_step:` validations pass | Continue |
| EC-012 | All 4 STATE.md validations fail simultaneously | Single BlockWithFix enumerating all 4 violation classes |
| EC-013 | INDEX.md adversary-pass row has 5 columns (7 pipe characters including borders) | BlockWithFix naming row, actual count=5, required=6; D-441(b) |
| EC-014 | INDEX.md adversary-pass row has 7 columns (9 pipe characters including borders) | BlockWithFix naming row, actual count=7, required=6; D-441(b) |
| EC-015 | INDEX.md adversary-pass row has exactly 6 columns | Continue |
| EC-016 | INDEX.md contains multiple rows; 1 of 5 rows has 5 columns | BlockWithFix naming the non-conforming row |
| EC-017 | `host::read_file` returns HostError::CapabilityDenied for STATE.md | Continue + log_warn; fail-open |
| EC-018 | `host::read_file` returns HostError::Timeout for INDEX.md | Continue + log_warn; fail-open |
| EC-019 | File path is `/some/dir/xSTATE.md` (ends_with "STATE.md" but file_name differs) | Continue (is_state_md_target returns false; path-component-strict guard) |
| EC-020 | File path is `/some/dir/xINDEX.md` (ends_with "INDEX.md" but file_name differs) | Continue (is_index_md_target returns false; path-component-strict guard) |
| EC-021 | STATE.md `current_step:` contains valid 4-arrow tail in prose body but `current_step:` itself has only 3-arrow tail | BlockWithFix on trajectory-tail (hook extracts tail from `current_step:` value only, not full body) |
| EC-022 | Banner narrative in `current_step:` contains `→N→N→N→N` digit-preceded narrative arrow sequence alongside a valid 4-arrow canonical tail | Continue; disambiguation: canonical tail is whitespace-preceded, narrative sequence is digit-preceded per BC-5.39.005 discriminator lesson applied to `current_step:` extraction |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Decision |
|----------|----------------|---------------------|----------|
| All STATE.md valid | `current_step:` — no forbidden patterns; all 4 index cites present; tail `→9→9→9→9`; D-382..D-477 with D-477 = max in STATE.md | `HookResult::Continue` | PASS |
| META-LEVEL WATCH in current_step | `current_step:` contains `META-LEVEL-5 WATCH: ...` | `HookResult::BlockWithFix` citing forbidden pattern | BLOCK |
| Missing ARCH-INDEX cite | `current_step:` has 3 of 4 index cites (ARCH-INDEX absent) | `HookResult::BlockWithFix` naming missing ARCH-INDEX cite | BLOCK |
| Tail LENGTH=3 in current_step | `current_step:` has tail `→9→9→9` (3 components) | `HookResult::BlockWithFix` citing 3 vs required 4 | BLOCK |
| Stale D-chain | `current_step:` cites `D-382..D-476`; STATE.md Decisions Log shows D-477 | `HookResult::BlockWithFix` citing stale D-chain | BLOCK |
| All 4 STATE.md violations | Forbidden pattern + missing 2 index cites + tail LENGTH=5 + stale D-chain | Single `HookResult::BlockWithFix` enumerating all violations | BLOCK |
| INDEX.md 5-column row | INDEX.md adversary-pass table row with 5 columns | `HookResult::BlockWithFix` naming row, citing D-441(b) | BLOCK |
| INDEX.md 7-column row | INDEX.md adversary-pass table row with 7 columns | `HookResult::BlockWithFix` naming row, citing D-441(b) | BLOCK |
| INDEX.md all rows 6 columns | INDEX.md adversary-pass rows all 6-column compliant | `HookResult::Continue` | PASS |
| Read failure STATE.md | `host::read_file` returns HostError::CapabilityDenied | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| Read failure INDEX.md | `host::read_file` returns HostError::Timeout | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| xSTATE.md path | file_name is "xSTATE.md" | `HookResult::Continue` (is_state_md_target false) | PASS (not target) |
| xINDEX.md path | file_name is "xINDEX.md" | `HookResult::Continue` (is_index_md_target false) | PASS (not target) |

## D-NNN Anchor Coverage

| D-NNN Sub-Clause | Gate Enforced | Postcondition |
|-----------------|---------------|---------------|
| D-440(a) | Forbidden meta-commentary patterns blocked in `current_step:` | PC2 |
| D-441(a) | Verbatim-strict `current_step:` — no meta-commentary injection; part of forbidden-pattern gate | PC2 |
| D-442(a) | Prescribed clause order / completeness — forbidden-pattern gate prevents rogue clause reordering markers | PC2 |
| D-443(a) | D-chain cite currency gate — stale cite blocked | PC5 |
| D-439(b) | All 4 index version patterns present in `current_step:` | PC3 |
| D-441(b) | 6-column INDEX.md adversary-pass row schema strict | PC9 |
| D-451(c) | Trajectory-tail LENGTH=4 derived from `current_step:` | PC4 |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | Meta-Commentary Block Invariant — hook emits BlockWithFix when `current_step:` matches forbidden pattern | bats integration test (fail-meta-commentary fixture) |
| (pending) | Index-Cite Pass Invariant — hook emits Continue when all 4 index cites present | bats integration test (pass-all-valid fixture) |
| (pending) | Missing-Index-Cite Block Invariant — hook emits BlockWithFix when any index cite absent | bats integration test (fail-missing-arch-index-cite fixture) |
| (pending) | Trajectory-Tail Block Invariant — hook emits BlockWithFix when tail in `current_step:` is not LENGTH=4 | bats integration test (fail-tail-3-components + fail-tail-5-components fixtures) |
| (pending) | D-Chain Stale Block Invariant — hook emits BlockWithFix when D-chain terminal < latest D-NNN in STATE.md | bats integration test (fail-stale-d-chain fixture) |
| (pending) | INDEX.md Column Block Invariant — hook emits BlockWithFix when adversary-pass row is not 6 columns | bats integration test (fail-5-col-row + fail-7-col-row fixtures) |
| (pending) | INDEX.md Column Pass Invariant — hook emits Continue when all rows are 6 columns | bats integration test (pass-index-6-col fixture) |
| (pending) | Fail-open Invariant STATE.md — hook emits Continue when file is unreadable | bats integration test (fail-open-state-unreadable fixture) |
| (pending) | Fail-open Invariant INDEX.md — hook emits Continue when file is unreadable | bats integration test (fail-open-index-unreadable fixture) |

VP IDs are pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — dispatch-advance structural validation automation sub-capability) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes the PostToolUse gate that mechanically prevents the `current_step:` meta-commentary, missing 4-index-cite, trajectory-tail cardinality, stale D-chain, and INDEX.md column-count violation classes codified in D-440(a), D-441(a), D-442(a), D-443(a), D-439(b), D-441(b), and D-451(c). The hook targets STATE.md frontmatter writes and INDEX.md adversary-pass table writes — both governance artifacts, not runtime subsystem artifacts. |
| Architecture Module | `crates/hook-plugins/validate-dispatch-advance/` (Rust WASM plugin); `plugins/vsdd-factory/hooks-registry.toml` (registry entry); `plugins/vsdd-factory/hook-plugins/validate-dispatch-advance.wasm` (compiled binary) |
| D-NNN Sub-Clauses Closed | D-440(a) (forbidden meta-commentary gate); D-441(a) (verbatim-strict `current_step:` — meta-commentary arm); D-442(a) (prescribed clause order — meta-commentary arm); D-443(a) (D-chain cite currency gate); D-439(b) (4-index version cite presence); D-441(b) (6-column INDEX.md adversary-pass row schema); D-451(c) (trajectory-tail LENGTH=4 in `current_step:`) |
| Stories | S-15.14 |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.14 must
  achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.003 — governs validate-index-cite-refresh hook (sister PostToolUse hook; detects stale
  4-index version cites in ARCH-INDEX, STATE.md, and INDEX.md — overlapping domain; this BC
  focuses on `current_step:` frontmatter field structural validity, not cross-file version
  freshness)
- BC-5.39.004 — governs validate-burst-log hook (sister PostToolUse hook; burst-log structural
  completeness; same crate scaffolding pattern and path-component-strict guard)
- BC-5.39.005 — governs validate-state-structure Phase 1 hook (sister PostToolUse hook; STATE.md
  banner line-count, dual-margin, and trajectory-tail in STATE.md body; this BC validates the
  `current_step:` frontmatter field specifically — distinct validation domain)
- BC-4.11.001 — validates write targets against artifact-path-registry (sister PostToolUse hook;
  structural analog for path validation)

## Architecture Anchors

- `crates/hook-plugins/validate-dispatch-advance/src/lib.rs` — hook implementation (pure logic functions + effectful orchestration)
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `tool = "Edit|Write"` and dual file targets `STATE.md` + `INDEX.md` (canonical Q5 form)

## Story Anchor

S-15.14 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M2 Wave-4)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-17 | Initial authoring (product-owner; brownfield-backfill S-15.03 M2 wave-4 story authoring). Anchors D-440(a)+D-441(a)+D-442(a)+D-443(a)+D-439(b)+D-441(b)+D-451(c). BC-5.39.006 allocated as next monotonic ID after BC-5.39.005 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.14 merge). Preemptive cascade lessons applied: path-component-strict guard for both STATE.md + INDEX.md arms (is_state_md_target + is_index_md_target); is_char_boundary() invariant 10; fail-open invariant 9; 524288 max_bytes matching BC-5.39.005 cap; D-chain currency invariant 7 fail-open design to prevent false-positive blocks on in-progress writes. |
