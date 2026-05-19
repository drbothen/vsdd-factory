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
  - .factory/policies.yaml
  - .factory/specs/architecture/decisions/ADR-021-wasm-cargo-audit-sandboxing.md
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
bc_id: BC-5.39.008
section: "5.39"
last_amended: "2026-05-18 (v1.0) — Initial authoring (product-owner; brownfield-backfill S-15.03 M3 wave story authoring 3M3a). Anchors F-PASS14-004+F-PASS14-006+TD-74-Option-b. BC-5.39.008 allocated as next monotonic ID after BC-5.39.007 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.15 merge). Part C (TD#74 cargo-audit lint) is gated on ADR-021 Option b. TD-VSDD-101 (CI env-var paper-fix) does not affect this BC's invariants — invariants are expressible regardless of how CI test execution is structured."
---

# BC-5.39.008: validate-policies-schema WASM hook MUST block on policies.yaml missing required header fields, non-canonical POLICY ID format, duplicate POLICY IDs, missing lint_hook or codified_at fields, and referenced lint-hook plugins absent from hooks-registry.toml; and MUST emit advisory on dispatch packages recommending crates with known RUSTSEC advisories

## Description

The `validate-policies-schema` WASM hook enforces structural integrity of `.factory/policies.yaml`
on every Edit/Write to that file, and (Part C) enforces cargo-audit advisory checking on
`td-*-dispatch.md` files when those files are written. The hook has two distinct activation
arms triggered by different target file patterns.

Part A (policies.yaml fix) is a one-time content edit performed by state-manager; it is NOT
enforced by this BC — it is a prerequisite content change that makes the policies.yaml file
schema-valid so Part B validation can pass. Part B (validate-policies-schema hook) is what
this BC specifies.

Part C (TD #74 Option b cargo-audit lint) is gated on ADR-021 Option b (bash pre-commit script
writes `cargo-audit-cache.json`; WASM hook reads cache via `host::read_file`). Part C is
included in this v1.0 BC because ADR-021 is already ACCEPTED (2026-05-15).

TD-VSDD-101 (CI env-var `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` paper-fix) is anchored to
S-15.15 but does NOT affect this BC's invariants. The BC's postconditions are fully
expressible and implementable regardless of how the CI test harness is structured.

## Preconditions

### Part B (policies.yaml arm)

1. A PostToolUse Edit/Write event has fired on the file `policies.yaml` at path matching
   `.factory/policies.yaml` (path-component-strict matching via
   `Path::new(file_path).file_name() == Some("policies.yaml")` — NOT suffix-`ends_with`).
2. The dispatcher has invoked the `validate-policies-schema` WASM plugin with the write
   payload.
3. The file content is read via `host::read_file`. The hook does NOT inspect
   `tool_input.content`; the filesystem value is the source of truth for validation.
4. `host::read_file` is available with `max_bytes = 524288` (512 KiB) and
   `timeout_ms = 2000` per call. The registry-level hook timeout is `timeout_ms = 5000`.
5. The policies.yaml file is YAML-parseable (if not parseable, invariant 6 applies — hard
   block with parse-error location).
6. Before schema validation runs, YAML anchors (`&anchor` / `*alias`) MUST be resolved.
   The hook's YAML parser resolves anchors before applying schema checks; anchor-aliased
   fields are validated under their resolved form.
7. YAML comments (`#...`) do not interfere with schema validation. Comments are stripped by
   the YAML parser before schema checks run.
8. The hooks-registry.toml file is accessible via `host::read_file` at the canonical path
   `plugins/vsdd-factory/hooks-registry.toml` for lint_hook reference validation (Part B
   precondition 8). If this file is not accessible, the lint_hook-existence check fails-open
   per invariant 9(b).

### Part C (dispatch package arm)

9. A PostToolUse Edit/Write event has fired on a file whose `file_name` path component
   matches `td-*-dispatch.md` glob pattern (i.e., basename starts with `td-` and ends with
   `-dispatch.md`; path-component-strict check on filename, not full path).
10. The `cargo-audit-cache.json` file at `.factory/hooks/cargo-audit-cache.json` is
    accessible via `host::read_file`. If the file is absent, the hook emits an advisory-level
    log message and `HookResult::Continue` (non-blocking per ADR-021 absent-file advisory
    policy). If the file is present but invalid JSON, the hook emits `HookResult::Continue`
    and logs a parse-error warning — fail-open.

## Postconditions

### Part B — policies.yaml arm

1. If the policies.yaml file fails YAML parsing (syntax error), the hook emits
   `HookResult::BlockWithFix` with the parse-error location (line and column if available)
   and the message `"policies.yaml: YAML parse error at line N: <message>"`.
2. If policies.yaml is YAML-parseable but lacks the required frontmatter block header fields
   (`document_type: governance-policy-registry`, `version`, `last_amended`), the hook emits
   `HookResult::BlockWithFix` naming each missing field and citing F-PASS14-004.
3. If any policy entry in the `policies` list lacks the required fields `id`, `name`,
   `severity`, `scope`, `description`, `lint_hook`, or `codified_at`, the hook emits
   `HookResult::BlockWithFix` naming the missing field(s) and the policy ID (or index if
   `id` is itself absent) and citing the canonical policy schema.
4. If any policy entry has an `id` field that does NOT match the canonical three-digit format
   `POLICY \d{3}` (e.g., `POLICY 001` through `POLICY 999`), the hook emits
   `HookResult::BlockWithFix` naming the non-conforming ID and citing F-PASS14-006 and the
   human-direction canonical form (three-digit POLICY 001-018 per 2026-05-15 human decision).
5. If any two policy entries share the same `id` value (duplicate POLICY ID), the hook emits
   `HookResult::BlockWithFix` naming the duplicated ID and citing the no-duplicate-IDs
   invariant.
6. If a policy entry's `lint_hook` field is non-null AND the referenced plugin name does not
   appear in `hooks-registry.toml` plugin entries, the hook emits `HookResult::BlockWithFix`
   naming the missing plugin reference and the policy ID, citing the lint_hook-existence
   invariant. A `lint_hook: null` value is valid (policy has no automated enforcement yet).
7. If `lint_hook` is null and the policy's `codified_at` field does not match the pattern
   `D-\d+` (a bare D-NNN decision reference), the hook emits `HookResult::BlockWithFix`
   naming the malformed `codified_at` value and the policy ID.
8. If policies.yaml contains fields NOT in the canonical schema (extra/unknown fields), the
   hook emits `HookResult::Advisory` (not BlockWithFix) logging the unknown field name and
   policy context. Forward-compatibility allows extra fields to exist without blocking.
9. If ALL of the following hold, the hook emits `HookResult::Continue` (pass):
   - YAML is parseable.
   - Required header fields present.
   - All policy entries have required fields.
   - All `id` values match canonical three-digit format.
   - No duplicate `id` values.
   - All non-null `lint_hook` references exist in hooks-registry.toml.
   - All `codified_at` values match `D-\d+` pattern.
10. Multiple violations produce a single `HookResult::BlockWithFix` message enumerating ALL
    violations (schema-violation cascade: one bad field does not mask others — all violations
    are reported together).
11. If `host::read_file` returns an error for policies.yaml (HostError of any kind), the hook
    emits `HookResult::Continue` and logs a warning via `host::log_warn` — fail-open.

### Part C — dispatch package arm (td-*-dispatch.md)

12. If `cargo-audit-cache.json` is absent or unreadable, the hook emits
    `HookResult::Continue` with advisory log message: `"cargo-audit-cache.json absent or
    unreadable; skipping advisory check. Run cargo-audit-cache.sh to populate."` per ADR-021
    absent-file advisory policy.
13. If `cargo-audit-cache.json` is present and parseable, the hook extracts crate dependency
    lines from the dispatch package file via regex (pattern: lines matching
    `^\s*[\w-]+\s*=\s*"[\d.]+"` or TOML-style `crate_name = { version = "..."}` forms).
    For each extracted crate+version, the hook checks the advisory lookup table embedded in
    the WASM binary (or cache JSON if the cache is populated with advisory data). If a
    crate+version has a known RUSTSEC advisory, the hook emits `HookResult::Advisory`
    (NOT BlockWithFix) citing the RUSTSEC ID and the crate name+version.
14. If `host::read_file` returns an error for `td-*-dispatch.md` (HostError of any kind),
    the hook emits `HookResult::Continue` and logs a warning — fail-open.

## Invariants

1. The hook NEVER writes to any file. It has no `write_file` capability in its registry
   entry. It is a read-only post-write validator.
2. The hook fires PostToolUse only — it never prevents a write; it signals AFTER the write
   has completed. The dispatcher records the block/advisory signal; the author must correct
   and re-write if a block was emitted.
3. Path-component-strict matching: `Path::new(file_path).file_name() == Some("policies.yaml")`
   for Part B. For Part C: the filename must match `^td-.*-dispatch\.md$` via regex on the
   basename only. Using `ends_with("policies.yaml")` or raw-string path matching MUST NOT
   be substituted.
4. The canonical POLICY ID format is exactly `POLICY \d{3}` where `\d{3}` is exactly three
   digits (000-999). Two-digit forms like `POLICY 01` or `POLICY 1` are non-canonical and
   MUST trigger the BlockWithFix per F-PASS14-006 human direction (2026-05-15). Leading
   zeros are required for single- and double-digit IDs (e.g., `POLICY 001`, not `POLICY 1`).
5. The required policy entry fields are: `id` (string, three-digit POLICY NNN form),
   `name` (string), `severity` (enum: `P0`/`P1`/`P2`/`P3`/`P4` or `CRITICAL`/`HIGH`/
   `MEDIUM`/`LOW`/`INFO`), `scope` (string), `description` (string), `lint_hook`
   (string or null), `codified_at` (string matching `D-\d+`). Every policy entry MUST
   include all 7 fields.
6. The YAML anchor resolution invariant: the hook's YAML parser MUST resolve `&anchor` /
   `*alias` references before schema checks run. Schema checks operate on the resolved
   (post-alias-expansion) form. An alias that resolves to a valid value is valid; an alias
   that resolves to a non-conforming value fails the schema check on the resolved value.
7. The comment-pass-through invariant: YAML comments (`# ...`) are discarded by the YAML
   parser before any schema validation. Comments do NOT interfere with field detection or
   value validation.
8. Schema-violation cascade invariant (postcondition 10): the hook MUST NOT stop on first
   violation. It MUST enumerate all violations across all policy entries before emitting the
   single combined BlockWithFix. This ensures the author can fix all issues in one edit
   rather than iterating one-at-a-time. The cascade applies to Part B only; Part C emits
   advisories per crate rather than batching.
9. Fail-open invariants:
   (a) If `host::read_file` for policies.yaml returns any HostError, the hook emits
       `HookResult::Continue` + `host::log_warn`. Unreadable file cannot be validated;
       fail-open prevents false-positive blocks during setup.
   (b) If `host::read_file` for hooks-registry.toml returns any HostError, the hook skips
       the lint_hook-existence check and logs a warning — fail-open for the existence check
       only; other schema checks still run.
   (c) If `cargo-audit-cache.json` is absent or returns HostError, the hook emits advisory
       and Continue; never blocks.
10. TD-VSDD-101 independence: the CI env-var `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` in
    `.github/workflows/ci.yml` is a CI-test-infrastructure paper-fix unrelated to this BC's
    invariants. This BC's postconditions are satisfied by the WASM hook implementation
    regardless of CI test skipping. The Part B + Part C behavioral contracts are verifiable
    via bats integration tests against fixture files; no production factory-artifacts mount
    is required for the schema validation tests.
11. All byte-index slice expressions operating on content strings MUST use
    `is_char_boundary()` guards where multi-byte UTF-8 input is possible. Slice without
    boundary guard is a runtime panic risk per the S-15.11 cascade lesson F-P4-001.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | policies.yaml has invalid YAML syntax (unmatched `{`) | BlockWithFix with parse-error location (line N); F-PASS14-004 class |
| EC-002 | policies.yaml missing `version:` header field | BlockWithFix naming `version` as missing; citing F-PASS14-004 |
| EC-003 | Policy entry has `id: "POLICY 01"` (two-digit) | BlockWithFix citing F-PASS14-006 non-canonical ID format |
| EC-004 | Policy entry has `id: "POLICY 001"` (three-digit canonical) | Continue for ID format check |
| EC-005 | Two policy entries both have `id: "POLICY 003"` | BlockWithFix citing duplicate ID |
| EC-006 | Policy entry has `lint_hook: "validate-dispatch-advance"` and that plugin exists in hooks-registry.toml | Continue for lint_hook-existence check |
| EC-007 | Policy entry has `lint_hook: "nonexistent-plugin"` (not in hooks-registry.toml) | BlockWithFix naming missing plugin reference |
| EC-008 | Policy entry has `lint_hook: null` | Continue for lint_hook check; null is valid (no automation yet) |
| EC-009 | Policy entry has `codified_at: "D-472"` (canonical form) | Continue for codified_at check |
| EC-010 | Policy entry has `codified_at: "pass-72"` (non-D-NNN form) | BlockWithFix citing malformed codified_at value |
| EC-011 | policies.yaml has YAML `&anchor` / `*alias` resolving to valid field | Continue; anchor resolved before check |
| EC-012 | policies.yaml has YAML comments (`# ...`) on field lines | Continue; comments stripped before validation |
| EC-013 | policies.yaml has an extra unknown field `custom_flag: true` in a policy entry | Advisory (not BlockWithFix); forward-compatible extra field |
| EC-014 | Multiple violations: missing `codified_at` + duplicate ID + non-canonical format | Single BlockWithFix enumerating all 3 violations (cascade per invariant 8) |
| EC-015 | `host::read_file` returns HostError::Timeout for policies.yaml | Continue + log_warn; fail-open |
| EC-016 | `host::read_file` for hooks-registry.toml fails | Skip lint_hook-existence check; log_warn; other checks continue |
| EC-017 | `td-99-dispatch.md` recommends `serde_yaml = "0.9.34"`; cache has RUSTSEC-2025-0068 advisory for serde_yaml | Advisory (not BlockWithFix) citing RUSTSEC-2025-0068; file write succeeds |
| EC-018 | `cargo-audit-cache.json` is absent | Advisory log + Continue; no block |
| EC-019 | `cargo-audit-cache.json` is present but invalid JSON | Continue + parse-error warning; fail-open |
| EC-020 | File path is `/some/dir/xpolicies.yaml` (ends_with "policies.yaml" but file_name is "xpolicies.yaml") | Continue; path-component-strict guard; not a target file |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Decision |
|----------|----------------|---------------------|----------|
| Valid policies.yaml | All fields correct; all 3-digit IDs; no duplicates; null lint_hooks; D-NNN codified_at | `HookResult::Continue` | PASS |
| YAML parse error | `policies.yaml` has unmatched brace | `HookResult::BlockWithFix` with parse-error location | BLOCK |
| Missing header field | policies.yaml lacks `version:` header | `HookResult::BlockWithFix` naming `version`; F-PASS14-004 | BLOCK |
| Non-canonical ID (two-digit) | `id: "POLICY 01"` | `HookResult::BlockWithFix` citing F-PASS14-006 | BLOCK |
| Duplicate ID | Two entries with `id: "POLICY 003"` | `HookResult::BlockWithFix` naming duplicated ID | BLOCK |
| Missing lint_hook field | Policy entry has no `lint_hook` key | `HookResult::BlockWithFix` naming missing field | BLOCK |
| Nonexistent lint_hook plugin | `lint_hook: "ghost-plugin"` not in hooks-registry.toml | `HookResult::BlockWithFix` naming missing plugin | BLOCK |
| null lint_hook | `lint_hook: null` | `HookResult::Continue` for that field | PASS |
| Malformed codified_at | `codified_at: "pass-72"` | `HookResult::BlockWithFix` citing malformed value | BLOCK |
| Extra unknown field | Policy has `custom_note: "..."` | `HookResult::Advisory` (not block) | ADVISORY |
| Cascade: 3 violations | Missing `codified_at` + dup ID + two-digit format | Single `HookResult::BlockWithFix` enumerating all 3 | BLOCK |
| Part C: known advisory crate | `td-dispatch.md` recommends `serde_yaml = "0.9.34"`; cache has RUSTSEC-2025-0068 | `HookResult::Advisory` citing RUSTSEC-2025-0068 | ADVISORY |
| Part C: cache absent | `cargo-audit-cache.json` not found | `HookResult::Continue` + advisory log | PASS (advisory) |
| Part C: clean dispatch | `td-dispatch.md` recommends `serde_norway = "0.9.0"`; no advisory in cache | `HookResult::Continue` | PASS |
| Read failure policies.yaml | `host::read_file` returns HostError | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| xpolicies.yaml path | file_name is "xpolicies.yaml" | `HookResult::Continue` (path-component-strict guard) | PASS (not target) |

## D-NNN Anchor Coverage

| D-NNN Sub-Clause | Gate Enforced | Postcondition |
|-----------------|---------------|---------------|
| F-PASS14-004 | policies.yaml frontmatter header required fields | PC2 |
| F-PASS14-006 | Three-digit POLICY ID canonical format (human direction 2026-05-15) | PC4 |
| ADR-021 Option b | cargo-audit cache file provisioning for Part C advisory checks | PC12/PC13 |
| POLICY 13 | `lint_hook` field required per POLICY 13 codification at D-472 | PC3/PC6 |
| POLICY 16 | `codified_at` field required per POLICY 16 codification at D-472 | PC3/PC7 |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | YAML-Parse-Block Invariant — hook emits BlockWithFix on YAML parse failure | bats integration test (fail-yaml-parse-error fixture) |
| (pending) | Missing-Header-Field Block Invariant — hook emits BlockWithFix when required header field absent | bats integration test (fail-missing-header-field fixture) |
| (pending) | Non-Canonical-ID Block Invariant — hook emits BlockWithFix when POLICY ID is non-three-digit | bats integration test (fail-two-digit-id fixture) |
| (pending) | Duplicate-ID Block Invariant — hook emits BlockWithFix on duplicate POLICY ID | bats integration test (fail-duplicate-id fixture) |
| (pending) | Missing-Required-Field Block Invariant — hook emits BlockWithFix when policy entry missing required field | bats integration test (fail-missing-lint-hook fixture) |
| (pending) | Nonexistent-Lint-Hook Block Invariant — hook emits BlockWithFix when lint_hook plugin not in hooks-registry.toml | bats integration test (fail-nonexistent-plugin fixture) |
| (pending) | Null-Lint-Hook Pass Invariant — hook emits Continue when lint_hook is null | bats integration test (pass-null-lint-hook fixture) |
| (pending) | Malformed-codified_at Block Invariant — hook emits BlockWithFix when codified_at is non-D-NNN | bats integration test (fail-malformed-codified-at fixture) |
| (pending) | Cascade-Invariant — hook reports ALL violations in single BlockWithFix | bats integration test (fail-cascade-violations fixture) |
| (pending) | Extra-Field Advisory Invariant — hook emits Advisory (not block) for unknown fields | bats integration test (pass-extra-field-advisory fixture) |
| (pending) | Part-C Advisory Invariant — hook emits Advisory (not block) when known RUSTSEC advisory found | bats integration test (fail-advisory-rustsec fixture) |
| (pending) | Part-C Cache-Absent Pass Invariant — hook emits Continue when cargo-audit-cache.json absent | bats integration test (pass-cache-absent fixture) |
| (pending) | Fail-open Invariant — hook emits Continue when file is unreadable | bats integration test (fail-open-policies-unreadable fixture) |

VP IDs are pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — policies.yaml schema enforcement automation; Part B) and E-13 (Artifact Integrity — cargo-audit advisory check; Part C) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. Part B of this BC formalizes the PostToolUse gate that mechanically enforces the policies.yaml schema invariants codified at D-472 (POLICY 13/16), F-PASS14-004 (header fields), and F-PASS14-006 (three-digit ID canonical form per human direction 2026-05-15). The hook targets policies.yaml — the governance policy registry artifact. Part C enforces artifact integrity for dispatch packages by cross-referencing crate dependencies against known RUSTSEC advisories (TD #74 Option b). E-12 and E-13 as used in the BC-5.39.xxx family per engine-discipline automation sub-capability convention. |
| Architecture Module | `crates/hook-plugins/validate-policies-schema/` (Rust WASM plugin, new crate); `plugins/vsdd-factory/hooks-registry.toml` (registry entry); `plugins/vsdd-factory/hook-plugins/validate-policies-schema.wasm` (compiled binary); `.factory/hooks/cargo-audit-cache.json` (Part C data file, written by pre-commit bash script) |
| D-NNN Sub-Clauses Closed | F-PASS14-004 (policies.yaml frontmatter header); F-PASS14-006 (three-digit POLICY ID canonical form); POLICY 13+16 schema requirements (D-472 codification); ADR-021 Option b (cargo-audit cache reader) |
| ADR References | ADR-021 (WASM Plugin Cargo-Audit Integration Sandboxing — Option b bash cache + WASM reader; ACCEPTED 2026-05-15; gates Part C) |
| Stories | S-15.15 |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.15 must achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.004 — governs validate-burst-log hook (sister PostToolUse hook; same hook-sdk pattern + fail-open + path-component-strict guard)
- BC-5.39.005 — governs validate-state-structure Phase 1 hook (sister PostToolUse hook)
- BC-5.39.006 — governs validate-dispatch-advance WASM hook (sister PostToolUse hook; same crate scaffolding pattern)
- BC-4.11.001 — validates write targets against artifact-path-registry (sister PostToolUse hook; structural analog for path validation)

## Architecture Anchors

- `crates/hook-plugins/validate-policies-schema/` — hook implementation (pure logic functions + effectful orchestration); schema check in `validate_schema`; ID format check in `check_policy_id_format`; lint_hook existence check in `check_lint_hook_exists`; YAML parse in `parse_policies_yaml`; Part C in `check_cargo_advisory`
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `tool = "Edit|Write"` and two file targets: `policies.yaml` (Part B) and `td-*-dispatch.md` glob (Part C)
- `.factory/policies.yaml` — target governance file; schema defined by this BC
- `specs/architecture/decisions/ADR-021-wasm-cargo-audit-sandboxing.md` — Part C gate; cargo-audit cache provisioning architecture

## Story Anchor

S-15.15 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M3 story; Parts A + B + C)

## VP Anchors

VP IDs pending VP-INDEX allocation by state-manager at S-15.15 post-merge burst.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-18 | Initial authoring (product-owner; brownfield-backfill S-15.03 M3 wave 3M3a BC authoring). Anchors F-PASS14-004+F-PASS14-006+POLICY-13/16-D-472+ADR-021-Option-b. BC-5.39.008 allocated as next monotonic ID after BC-5.39.007 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.15 merge). Part C gated on ADR-021 (ACCEPTED 2026-05-15). TD-VSDD-101 CI env-var paper-fix does not affect this BC's invariants (invariant 10). Preemptive cascade lessons applied: path-component-strict guard for both policies.yaml and td-*-dispatch.md arms; is_char_boundary() invariant 11; fail-open invariant 9; 524288 max_bytes; cascade-all-violations invariant 8; advisory-not-block for Part C per ADR-021; advisory for extra fields (forward-compat). |
