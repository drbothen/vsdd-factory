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
  - 2026-05-19
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.008
section: "5.39"
last_amended: "2026-05-19 (v1.2) — Pass-2 adversary fix-burst (product-owner; brownfield-backfill M3 3M3a-r fix-burst pass-2; INV-017 applied). Closes F-BC008P2-001 (CRITICAL: invariant 4 + PC4 + EC-003/004/005 + Test Vectors rewritten from POLICY \\d{3} to integer id: format per production policies.yaml), F-BC008P2-002 (CRITICAL: PC10 false exec_subprocess claim replaced with ADR-021 canonical sandboxing rationale), F-BC008P2-003 (HIGH: invariant 5 severity self-contradiction removed; HIGH+MEDIUM strict, P0/P4 future-amendment paragraph deleted), F-BC008P2-004 (MEDIUM: PC2 scope clarification orphan renamed to ### PC2 — Scope clarification with proper anchor), F-BC008P2-005 (MEDIUM: ADR-021 line 251 cite replaced with section-anchor cite), F-BC007P2-006 (MEDIUM: ADR-021 Open Sub-Questions §2 cited in PC13 as source of advisory-severity-threshold answer), F-BC008P2-007 (LOW: phase updated to section-12-step-3M3a-r-pass-2), F-BC008P2-008/009 (NITPICK: changelog v1.2 row structured per BC-5.39.006 convention). [Prior: 2026-05-18 (v1.1) — Pass-1 adversary fix-burst (product-owner; brownfield-backfill M3 3M3a-r fix-burst). F-BC008P1-001 DO NOT ACT (FALSE POSITIVE — TD-VSDD-101 registered at tech-debt-register.md:45; env-var present in origin/develop ci.yml; adversary grepped stale local main). Closes F-BC008P1-002..020. [Prior: 2026-05-18 (v1.0) — Initial authoring (product-owner; brownfield-backfill S-15.03 M3 wave story authoring 3M3a). Anchors F-PASS14-004+F-PASS14-006+TD-74-Option-b. BC-5.39.008 allocated as next monotonic ID after BC-5.39.007 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.15 merge).]"
---

# BC-5.39.008: validate-policies-schema WASM hook MUST block on policies.yaml missing required header fields, non-canonical POLICY ID format, duplicate POLICY IDs, missing lint_hook or codified_at fields, and referenced lint-hook plugins absent from hooks-registry.toml; and MUST emit advisory on dispatch packages recommending crates with known RUSTSEC advisories

## Description

The `validate-policies-schema` WASM hook enforces structural integrity of `.factory/policies.yaml`
on every Edit/Write to that file, and (Part C) enforces cargo-audit advisory checking on
`td-*-dispatch.md` files when those files are written. The hook has two distinct activation
arms triggered by different target file patterns.

Part A (policies.yaml content fix) is a one-time content edit performed by state-manager; it is
NOT enforced by this BC — it is a prerequisite content change that makes the policies.yaml file
schema-valid so Part B validation can pass. Part B (validate-policies-schema hook) is what
this BC specifies.

Part C (TD #74 Option b cargo-audit lint) is gated on ADR-021 Option b (bash pre-commit script
writes `cargo-audit-cache.json`; WASM hook reads cache via `host::read_file`). Part C is
included in this v1.1 BC because ADR-021 is already ACCEPTED (2026-05-15).

TD-VSDD-101 (CI env-var `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` paper-fix) is anchored to
S-15.15 but does NOT affect this BC's invariants. The BC's postconditions are fully
expressible and implementable regardless of how the CI test harness is structured.

### Hook Invocation Order (Part A → Part B → Part C)

The hook processes in strict sequential order:
1. **Part A (schema validation):** policies.yaml schema validation runs first. This is fast,
   pure-logic, in-memory. If Part A produces a block, Part B and Part C do not run.
2. **Part B (lint_hook existence):** hooks-registry.toml cross-reference runs second. This
   requires an additional `host::read_file` call for the registry file.
3. **Part C (cargo-audit advisory):** runs last because it requires reading an external
   cache file and performs per-crate matching against the advisory list. Part C is the
   most I/O-bound step and should not run if Parts A or B already block.

This ordering applies only when triggered by a policies.yaml write (Part B arm). For
`td-*-dispatch.md` writes (Part C arm), only Part C runs.

## Adversary Pass Coverage

This BC v1.1 was produced after the first adversary pass (pass-1, 2026-05-18). Finding
counts: 20 adversary findings (F-BC008P1-001..020); F-BC008P1-001 reclassified as FALSE
POSITIVE by orchestrator (adversary grepped stale local main; TD-VSDD-101 IS registered).
This v1.1 fix-burst closes all actionable findings (001 DO NOT ACT, 002-020 closed).

## Preconditions

### Part B (policies.yaml arm)

1. A PostToolUse Edit/Write event has fired on the file `policies.yaml` at path matching
   `.factory/policies.yaml` (path-component-strict matching via
   `Path::new(file_path).file_name() == Some("policies.yaml")` — NOT suffix-`ends_with`).
2. The dispatcher has invoked the `validate-policies-schema` WASM plugin with the write
   payload.
3. The file content is read via `host::read_file`. The hook does NOT inspect
   `tool_input.content`; the filesystem value is the source of truth for validation.

### Part B — file read cap (META-LEVEL-24 false-green prevention)

4. `host::read_file` is configured with `max_bytes = 524288` (512 KiB) and
   `timeout_ms = 2000` per call. The registry-level hook timeout is `timeout_ms = 5000`.

   **Rationale (file-size constraint):** policies.yaml grows monotonically as governance
   policies are added. The hook-sdk `host::read_file` default cap on some configurations
   is 64 KiB (per BC-5.39.004 precedent). A 64 KiB cap could silently truncate a large
   policies.yaml, causing the hook to validate only the first portion and miss violations
   in the remainder — the META-LEVEL-24 false-green class. This BC EXPLICITLY sets
   `max_bytes = 524288` (512 KiB) to prevent truncation. The registry entry for this hook
   MUST declare `max_bytes = 524288` in its `host::read_file` call.

### Part B — YAML preconditions

5. The policies.yaml file is YAML-parseable (if not parseable, Part A postcondition 1
   applies — hard block with parse-error location).
6. Before schema validation runs, YAML anchors (`&anchor` / `*alias`) MUST be resolved.
   The hook's YAML parser resolves anchors before applying schema checks; anchor-aliased
   fields are validated under their resolved form.
7. YAML comments (`#...`) do not interfere with schema validation. Comments are stripped by
   the YAML parser before schema checks run.

### Part B — hooks-registry cross-reference

8. The hooks-registry.toml file is accessible via `host::read_file` at the canonical path
   `plugins/vsdd-factory/hooks-registry.toml` for lint_hook reference validation. If this
   file is not accessible, the lint_hook-existence check fails-open per invariant 9(b).

### PC2 — Scope clarification

PC2 scope: schema validation applies to **declared-mandatory keys only** (the 7 required
fields: `id`, `name`, `severity`, `scope`, `description`, `lint_hook`, `codified_at`).
PC2 does NOT require that all top-level keys be present — it requires only that the
declared-mandatory keys are present and conforming. Extra/unknown top-level keys emit
an advisory log (not a block) per postcondition 8.

### Part C (dispatch package arm)

9. A PostToolUse Edit/Write event has fired on a file whose `file_name` path component
   matches `td-*-dispatch.md` glob pattern (i.e., basename starts with `td-` and ends with
   `-dispatch.md`; path-component-strict check on filename, not full path).
10. The `cargo-audit-cache.json` file at `.factory/hooks/cargo-audit-cache.json` is
    accessible via `host::read_file`. If the file is absent, the hook emits an advisory-log
    message via `host::log_warn` and `HookResult::Continue` (non-blocking per ADR-021
    absent-file advisory policy). If the file is present but invalid JSON, the hook emits
    `HookResult::Continue` and logs a parse-error warning — fail-open.

    **ADR-021 Option (b) WASM integration rationale:** The bash-data-provisioner +
    WASM-decision-maker layering exists for three reasons per ADR-021
    §"The sandboxing constraint" (lines 41-52) and §"D-337 scope clarification" (lines 81-97):

    1. **Network access:** `cargo audit --json` fetches the RustSec advisory database
       over the network. WASM plugins have no outbound network access under the
       `path_allow` capability model (ADR-018, ADR-002).
    2. **Binary allow-list:** The subprocess imports available to WASM are constrained
       to the hook-sdk registered host imports. `cargo-audit` is an external binary
       not on this allow-list.
    3. **Cache-freshness model:** The bash script provisions the freshest advisory data
       available on the developer's machine; the WASM hook reads the cache deterministically
       via `host::read_file`.

    Note: `host::exec_subprocess` IS a registered host import in `crates/hook-sdk/src/host.rs`
    (INV-017 evidence: `grep -nE '^pub fn exec_subprocess' crates/hook-sdk/src/host.rs` →
    `299:pub fn exec_subprocess(`). However, its use for cargo-audit would still require
    network access and the external binary — constraints the bash+cache layering solves per
    ADR-021 Option (b). ADR-021 Option (a) (embedded RUSTSEC lookup table) is REJECTED per
    ADR-021 §"Alternatives Considered — Option (a)" (lines 245-258); this BC reflects
    Option (b) only.

## Postconditions

### Part A — policies.yaml arm

1. If the policies.yaml file fails YAML parsing (syntax error), the hook emits
   `HookResult::Block { reason: block_with_fix(...) }` with the parse-error location
   (line and column if available) and the message
   `"policies.yaml: YAML parse error at line N: <message>"`.
2. If policies.yaml is YAML-parseable but lacks the required frontmatter block header
   fields (`document_type: governance-policy-registry`, `version`, `last_amended`), the
   hook emits `HookResult::Block { reason: block_with_fix(...) }` naming each missing
   field and citing F-PASS14-004.
3. If any policy entry in the `policies` list lacks any of the 7 required fields (`id`,
   `name`, `severity`, `scope`, `description`, `lint_hook`, `codified_at`), the hook emits
   `HookResult::Block { reason: block_with_fix(...) }` naming the missing field(s) and
   the policy ID (or index if `id` is itself absent) and citing the canonical policy
   schema.
4. If any policy entry has an `id` field that is not a YAML integer scalar in range [1, 999],
   the hook emits `HookResult::block_with_fix(hook, reason, recommendation, code)` naming
   the non-conforming ID value and citing the canonical integer format (bare YAML integer,
   range [1, 999]) per production policies.yaml. String values, negative integers, zero, and
   `POLICY NNN`-prefixed strings are all non-conforming.
5. If any two policy entries share the same `id` value (duplicate POLICY ID), the hook
   emits `HookResult::Block { reason: block_with_fix(...) }` naming the duplicated ID
   and citing the no-duplicate-IDs invariant.

### Part B — lint_hook and codified_at validation

6. If a policy entry's `lint_hook` field is non-null AND the referenced plugin name does
   not appear in `hooks-registry.toml` plugin entries, the hook emits
   `HookResult::Block { reason: block_with_fix(...) }` naming the missing plugin
   reference and the policy ID, citing the lint_hook-existence invariant. The `lint_hook`
   field accepts: (a) a simple plugin slug `^[a-z0-9-]+$` (e.g., `validate-burst-log`),
   OR (b) a namespaced slug `^[a-z0-9-]+:[a-z0-9-]+$` (e.g.,
   `vsdd-factory:validate-burst-log`). Both forms are valid; the existence check
   normalizes to the basename for hooks-registry.toml lookup.
   A `lint_hook: null` value is valid (policy has no automated enforcement yet).

7. If `lint_hook` is non-null AND `codified_at` does not match the pattern `D-\d+` (a
   bare D-NNN decision reference), the hook emits
   `HookResult::Block { reason: block_with_fix(...) }` naming the malformed `codified_at`
   value and the policy ID.

   **Coupling rationale (PC7):** `lint_hook` and `codified_at` are validated together
   because they serve complementary governance functions: `lint_hook` proves the policy has
   an automated enforcement mechanism (implementation backing), and `codified_at` proves the
   policy has an auditable decision history entry (D-NNN traceability). A policy with
   `lint_hook` set but no `codified_at` is missing its audit trail; a policy with
   `codified_at` but no `lint_hook` may still be valid (some policies are human-enforced).
   When `lint_hook` is non-null, the full enforcement contract requires both fields.

8. If policies.yaml contains fields NOT in the canonical schema (extra/unknown fields), the
   hook logs an advisory-level message via `host::log_warn` and emits `HookResult::Continue`
   (not `HookResult::Block`). Forward-compatibility allows extra fields to exist without
   blocking. NOTE: there is NO `HookResult::Advisory` variant in hook-sdk — advisory
   behavior is implemented as `HookResult::Continue` + `host::log_warn`.

9. If ALL of the following hold, the hook emits `HookResult::Continue` (pass):
   - YAML is parseable.
   - Required header fields present.
   - All policy entries have the 7 required fields.
   - All `id` values match canonical three-digit format.
   - No duplicate `id` values.
   - All non-null `lint_hook` references exist in hooks-registry.toml.
   - All non-null `lint_hook` entries have `codified_at` matching `D-\d+`.
10. Multiple violations produce a single `HookResult::Block { reason: block_with_fix(...) }`
    message enumerating ALL violations (schema-violation cascade: one bad field does not
    mask others — all violations are reported together).
11. If `host::read_file` returns an error for policies.yaml (HostError of any kind), the
    hook emits `HookResult::Continue` and logs a warning via `host::log_warn` — fail-open.

### Part C — dispatch package arm (td-*-dispatch.md)

12. If `cargo-audit-cache.json` is absent or unreadable, the hook emits
    `HookResult::Continue` with advisory log message via `host::log_warn`:
    `"cargo-audit-cache.json absent or unreadable; skipping advisory check. Run
    update-cargo-audit-cache.sh to populate."` per ADR-021 absent-file advisory policy.

13. If `cargo-audit-cache.json` is present and parseable, the hook extracts crate
    dependency lines from the dispatch package file via regex (pattern: lines matching
    `^\s*[\w-]+\s*=\s*"[\d.]+"` or TOML-style `crate_name = { version = "..."}` forms).
    For each extracted crate+version, the hook checks the cache advisory list (from the
    cache JSON populated by `cargo audit --json` per ADR-021 Option b). The hook does NOT
    use an embedded lookup table (ADR-021 Option (a) REJECTED).

    **Advisory escalation threshold:** If a matched advisory has severity `HIGH` or
    `CRITICAL` (as reported in the `cargo-audit-cache.json` `severity` field), the hook
    emits `HookResult::block_with_fix(hook, reason, recommendation, code)` citing the
    RUSTSEC ID and the crate name+version. If all matched advisories are `MEDIUM` or below,
    the hook logs an advisory message via `host::log_warn` and emits `HookResult::Continue`
    (non-blocking). Zero matched advisories: `HookResult::Continue`. This threshold answers
    ADR-021 §"Open Sub-Questions for S-15.15 Part C Implementer" §2 (lines 269-272):
    "HIGH and CRITICAL block; MEDIUM/LOW emit warning only" — the threshold question is
    closed by this BC (INV-017 evidence: `grep -n "Open Sub-Questions\|advisory severity"
    ADR-021-wasm-cargo-audit-sandboxing.md` → `262: Open Sub-Questions for S-15.15 Part C
    Implementer`, `269: Advisory severity threshold: … HIGH and CRITICAL block; MEDIUM/LOW
    emit warning only`).

    **Per-advisory emission:** Part C emits one advisory log entry per matched advisory
    (not batched). If 3 advisories are found, 3 `host::log_warn` calls are made, each
    citing the RUSTSEC ID and affected crate. The final `HookResult` is a single Block
    (if any HIGH/CRITICAL found) or Continue (all MEDIUM/LOW).

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
4. The canonical policy `id` field is a YAML integer in the range [1, 999] (e.g., `id: 1`
   through `id: 18` in the current production file). String forms, negative integers, and
   zero are non-conforming and MUST trigger the Block. The `id` field MUST be a bare YAML
   integer scalar, not a quoted string and not a `POLICY NNN` prefixed string.

   **INV-017 evidence — production policies.yaml id format:**
   ```
   $ grep -nE '^  - id:' .factory/policies.yaml | head -5
   33:  - id: 1
   47:  - id: 2
   62:  - id: 3
   75:  - id: 4
   90:  - id: 5

   $ grep -nE 'POLICY [0-9]{3}' .factory/policies.yaml | grep -v '^[0-9]*:#'
   (zero output — no POLICY NNN pattern in production data fields)
   ```
   Production format is bare integer `id: N` (N in [1,18] currently). `POLICY \d{3}` does
   NOT appear in production data fields. Hook MUST validate integer range [1, 999], not a
   `POLICY \d{3}` string pattern.
5. The required policy entry fields and their constraints:
   - `id`: YAML integer scalar in range [1, 999] (canonical production format; NOT a string,
     NOT a `POLICY NNN`-prefixed value). INV-017 evidence: `grep -nE '^  - id:' .factory/policies.yaml`
     returns `33: - id: 1` through `282: - id: 18` — bare integers only.
   - `name`: non-empty string
   - `severity`: string; allowed values are `HIGH` and `MEDIUM` only (per current policies.yaml
     corpus). INV-017 evidence: `grep -nE 'severity:' .factory/policies.yaml | head -10` returns
     entries using only `HIGH` or `MEDIUM`. P0/P1/P2/P3/P4 vocabulary is NOT accepted in v1.2;
     a future v1.3 amendment may add it with an explicit invariant change.
   - `scope`: non-empty string
   - `description`: non-empty string
   - `lint_hook`: string (slug or namespaced slug per postcondition 6) OR null
   - `codified_at`: string matching `D-\d+` (required when `lint_hook` is non-null)
   Every policy entry MUST include all 7 fields.
6. The YAML anchor resolution invariant: the hook's YAML parser MUST resolve `&anchor` /
   `*alias` references before schema checks run. Schema checks operate on the resolved
   (post-alias-expansion) form.
7. The comment-pass-through invariant: YAML comments (`# ...`) are discarded by the YAML
   parser before any schema validation. Comments do NOT interfere with field detection or
   value validation.
8. Schema-violation cascade invariant (postcondition 10): the hook MUST NOT stop on first
   violation. It MUST enumerate all violations across all policy entries before emitting the
   single combined Block. This ensures the author can fix all issues in one edit rather than
   iterating one-at-a-time. The cascade applies to Part B only; Part C emits per-advisory
   advisory logs (non-batched) with a single final HookResult.
9. Fail-open invariants:
   (a) If `host::read_file` for policies.yaml returns any HostError, the hook emits
       `HookResult::Continue` + `host::log_warn`. Unreadable file cannot be validated;
       fail-open prevents false-positive blocks during setup.
   (b) If `host::read_file` for hooks-registry.toml returns any HostError, the hook skips
       the lint_hook-existence check and logs a warning — fail-open for the existence check
       only; other schema checks still run.
   (c) If `cargo-audit-cache.json` is absent or returns HostError, the hook logs advisory
       via `host::log_warn` and emits `HookResult::Continue`; never blocks.
10. TD-VSDD-101 independence: the CI env-var `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` in
    `.github/workflows/ci.yml` is a CI-test-infrastructure paper-fix unrelated to this BC's
    invariants. This BC's postconditions are satisfied by the WASM hook implementation
    regardless of CI test skipping. The Part A + Part B + Part C behavioral contracts are
    verifiable via bats integration tests against fixture files.
11. All byte-index slice expressions operating on content strings MUST use
    `is_char_boundary()` guards where multi-byte UTF-8 input is possible. Slice without
    boundary guard is a runtime panic risk per the S-15.11 cascade lesson F-P4-001.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | policies.yaml has invalid YAML syntax (unmatched `{`) | `HookResult::Block` with parse-error location (line N); Part A postcondition 1 |
| EC-002 | policies.yaml missing `version:` header field | `HookResult::Block` naming `version` as missing; citing F-PASS14-004 |
| EC-003 | Policy entry has `id: "POLICY 01"` (string, not integer) | `HookResult::Block` citing non-conforming id format; expected bare YAML integer in [1,999] |
| EC-004 | Policy entry has `id: 1` (bare integer, canonical) | `HookResult::Continue` for ID format check |
| EC-005 | Two policy entries both have `id: 3` | `HookResult::Block` citing duplicate ID |
| EC-006 | Policy entry has `lint_hook: "validate-dispatch-advance"` and that plugin exists in hooks-registry.toml | `HookResult::Continue` for lint_hook-existence check |
| EC-007 | Policy entry has `lint_hook: "nonexistent-plugin"` (not in hooks-registry.toml) | `HookResult::Block` naming missing plugin reference |
| EC-008 | Policy entry has `lint_hook: null` | `HookResult::Continue` for lint_hook check; null is valid (no automation yet) |
| EC-009 | Policy entry has `codified_at: "D-472"` (canonical form) | `HookResult::Continue` for codified_at check |
| EC-010 | Policy entry has `codified_at: "pass-72"` (non-D-NNN form) | `HookResult::Block` citing malformed codified_at value |
| EC-011 | policies.yaml has YAML `&anchor` / `*alias` resolving to valid field | `HookResult::Continue`; anchor resolved before check |
| EC-012 | policies.yaml has YAML comments (`# ...`) on field lines | `HookResult::Continue`; comments stripped before validation |
| EC-013 | policies.yaml has an extra unknown field `custom_flag: true` in a policy entry | `HookResult::Continue` + `host::log_warn` advisory; forward-compatible extra field (not block) |
| EC-014 | Multiple violations: missing `codified_at` + duplicate ID + non-canonical format | Single `HookResult::Block` enumerating all 3 violations (cascade per invariant 8) |
| EC-015 | `host::read_file` returns HostError::Timeout for policies.yaml | `HookResult::Continue` + `host::log_warn`; fail-open |
| EC-016 | `host::read_file` for hooks-registry.toml fails | Skip lint_hook-existence check; `host::log_warn`; other checks continue |
| EC-017 | `td-99-dispatch.md` recommends `serde_yaml = "0.9.34"`; cache has RUSTSEC-2025-0068 with severity `high` | `HookResult::Block` citing RUSTSEC-2025-0068; HIGH advisory blocks |
| EC-018 | `cargo-audit-cache.json` is absent | `host::log_warn` advisory + `HookResult::Continue`; no block |
| EC-019 | `cargo-audit-cache.json` is present but invalid JSON | `HookResult::Continue` + parse-error warning; fail-open |
| EC-020 | File path is `/some/dir/xpolicies.yaml` (ends_with "policies.yaml" but file_name is "xpolicies.yaml") | `HookResult::Continue`; path-component-strict guard; not a target file |
| EC-021 | `td-dispatch.md` recommends two crates; cache has RUSTSEC advisory for each (both `medium` severity) | `host::log_warn` for each crate (2 separate log calls); `HookResult::Continue`; MEDIUM does not block |
| EC-022 | `td-dispatch.md` recommends crate with CRITICAL advisory and another with MEDIUM advisory | `HookResult::Block` citing CRITICAL advisory; `host::log_warn` for MEDIUM advisory (per-advisory emission, single final Block) |
| EC-023 | Policy entry has `lint_hook: "vsdd-factory:validate-burst-log"` (namespaced slug) | `HookResult::Continue` for lint_hook format check; namespaced slug is valid per postcondition 6 |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Preconditions Exercised | Decision |
|----------|----------------|---------------------|------------------------|----------|
| Valid policies.yaml | All fields correct; integer ids 1..N; no duplicates; null lint_hooks; D-NNN codified_at | `HookResult::Continue` | PC1-PC8 all satisfied | PASS |
| YAML parse error | `policies.yaml` has unmatched brace | `HookResult::Block` with parse-error location | PC5 violated (parse failure) | BLOCK |
| Missing header field | policies.yaml lacks `version:` header | `HookResult::Block` naming `version`; F-PASS14-004 | PC5 satisfied, PC2 header check violated | BLOCK |
| Non-conforming ID (string) | `id: "POLICY 01"` (quoted string, not integer) | `HookResult::Block` citing non-conforming id format; expected integer in [1,999] | PC5 satisfied; ID format violated | BLOCK |
| Duplicate ID | Two entries with `id: 3` | `HookResult::Block` naming duplicated ID | PC5 satisfied; duplicate check violated | BLOCK |
| Missing lint_hook field | Policy entry has no `lint_hook` key | `HookResult::Block` naming missing field | PC2 (required field absent) | BLOCK |
| Nonexistent lint_hook plugin | `lint_hook: "ghost-plugin"` not in hooks-registry.toml | `HookResult::Block` naming missing plugin | PC8 satisfied; lint_hook existence violated | BLOCK |
| null lint_hook | `lint_hook: null` | `HookResult::Continue` for that field | PC2 satisfied (null valid) | PASS |
| Malformed codified_at | `codified_at: "pass-72"` | `HookResult::Block` citing malformed value | PC7 rationale: lint_hook non-null; codified_at invalid | BLOCK |
| Extra unknown field | Policy has `custom_note: "..."` | `HookResult::Continue` + `host::log_warn` (not block) | PC2 scope: mandatory fields only | ADVISORY |
| Cascade: 3 violations | Missing `codified_at` + dup ID (both `id: 3`) + string-format id | Single `HookResult::Block` enumerating all 3 | PC2, ID format, duplicate checks all violated | BLOCK |
| Part C: HIGH advisory crate | `td-dispatch.md` recommends `serde_yaml = "0.9.34"`; cache has RUSTSEC-2025-0068 severity `high` | `HookResult::Block` citing RUSTSEC-2025-0068 | PC9, PC10; HIGH threshold triggers block | BLOCK |
| Part C: MEDIUM advisory only | `td-dispatch.md` crate with MEDIUM advisory in cache | `HookResult::Continue` + `host::log_warn` | PC9, PC10; MEDIUM below block threshold | ADVISORY |
| Part C: cache absent | `cargo-audit-cache.json` not found | `HookResult::Continue` + `host::log_warn` advisory | PC10 absent-file path | PASS (advisory) |
| Part C: clean dispatch | `td-dispatch.md` recommends `serde_norway = "0.9.0"`; no advisory in cache | `HookResult::Continue` | PC9, PC10; no advisories found | PASS |
| Read failure policies.yaml | `host::read_file` returns HostError | `HookResult::Continue` + `host::log_warn` | PC4 fail-open | PASS (fail-open) |
| xpolicies.yaml path | file_name is "xpolicies.yaml" | `HookResult::Continue` (path-component-strict guard) | PC1 not triggered | PASS (not target) |
| Namespaced lint_hook slug | `lint_hook: "vsdd-factory:validate-burst-log"` (exists in registry) | `HookResult::Continue` | PC6; namespaced slug valid | PASS |

## D-NNN Anchor Coverage

| D-NNN Sub-Clause | Gate Enforced | Postcondition |
|-----------------|---------------|---------------|
| F-PASS14-004 | policies.yaml frontmatter header required fields | PC2 |
| F-PASS14-006 | Three-digit POLICY ID canonical format (human direction 2026-05-15) | PC4 |
| ADR-021 Option b | cargo-audit cache file provisioning for Part C advisory checks (Option (a) REJECTED per ADR-021 §"Alternatives Considered — Option (a)" (lines 245-258)) | PC12/PC13 |
| POLICY 13 | `lint_hook` field required per POLICY 13 codification at D-472 | PC3/PC6 |
| POLICY 16 | `codified_at` field required per POLICY 16 codification at D-472 | PC3/PC7 |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | YAML-Parse-Block Invariant — hook emits Block on YAML parse failure | bats integration test (fail-yaml-parse-error fixture) |
| (pending) | Missing-Header-Field Block Invariant — hook emits Block when required header field absent | bats integration test (fail-missing-header-field fixture) |
| (pending) | Non-Canonical-ID Block Invariant — hook emits Block when POLICY ID is non-three-digit | bats integration test (fail-two-digit-id fixture) |
| (pending) | Duplicate-ID Block Invariant — hook emits Block on duplicate POLICY ID | bats integration test (fail-duplicate-id fixture) |
| (pending) | Missing-Required-Field Block Invariant — hook emits Block when policy entry missing required field | bats integration test (fail-missing-lint-hook fixture) |
| (pending) | Nonexistent-Lint-Hook Block Invariant — hook emits Block when lint_hook plugin not in hooks-registry.toml | bats integration test (fail-nonexistent-plugin fixture) |
| (pending) | Null-Lint-Hook Pass Invariant — hook emits Continue when lint_hook is null | bats integration test (pass-null-lint-hook fixture) |
| (pending) | Malformed-codified_at Block Invariant — hook emits Block when codified_at is non-D-NNN | bats integration test (fail-malformed-codified-at fixture) |
| (pending) | Cascade-Invariant — hook reports ALL violations in single Block | bats integration test (fail-cascade-violations fixture) |
| (pending) | Extra-Field Advisory Invariant — hook emits Continue + log_warn (not block) for unknown fields | bats integration test (pass-extra-field-advisory fixture) |
| (pending) | Part C HIGH-Advisory Block Invariant — hook emits Block when HIGH RUSTSEC advisory found | bats integration test (fail-advisory-rustsec-high fixture) |
| (pending) | Part C MEDIUM-Advisory Continue Invariant — hook emits Continue + log_warn when only MEDIUM advisory found | bats integration test (pass-advisory-rustsec-medium fixture) |
| (pending) | Part C Cache-Absent Pass Invariant — hook emits Continue when cargo-audit-cache.json absent | bats integration test (pass-cache-absent fixture) |
| (pending) | Fail-open Invariant — hook emits Continue when file is unreadable | bats integration test (fail-open-policies-unreadable fixture) |
| (pending) | Namespaced-Lint-Hook Pass Invariant — hook accepts namespaced slug `vsdd-factory:plugin-name` | bats integration test (pass-namespaced-lint-hook fixture) |

VP IDs are pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — policies.yaml schema enforcement automation; Part A + Part B) and E-13 (Artifact Integrity — cargo-audit advisory check; Part C) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. Part A + Part B of this BC formalizes the PostToolUse gate that mechanically enforces the policies.yaml schema invariants codified at D-472 (POLICY 13/16), F-PASS14-004 (header fields), and F-PASS14-006 (three-digit ID canonical form per human direction 2026-05-15). The hook targets policies.yaml — the governance policy registry artifact. Part C enforces artifact integrity for dispatch packages by cross-referencing crate dependencies against known RUSTSEC advisories (TD #74 Option b per ADR-021 Option b ACCEPTED 2026-05-15). E-12 and E-13 as used in the BC-5.39.xxx family per engine-discipline automation sub-capability convention. |
| Architecture Module | `crates/hook-plugins/validate-policies-schema/` (Rust WASM plugin, new crate); `plugins/vsdd-factory/hooks-registry.toml` (registry entry); `plugins/vsdd-factory/hook-plugins/validate-policies-schema.wasm` (compiled binary); `.factory/hooks/cargo-audit-cache.json` (Part C data file, written by pre-commit bash script per ADR-021 Option b) |
| D-NNN Sub-Clauses Closed | F-PASS14-004 (policies.yaml frontmatter header); F-PASS14-006 (integer id field canonical form per production policies.yaml); POLICY 13+16 schema requirements (D-472 codification); ADR-021 Option b (cargo-audit cache reader; Option (a) REJECTED per ADR-021 §"Alternatives Considered — Option (a)" (lines 245-258)) |
| ADR References | ADR-021 (WASM Plugin Cargo-Audit Integration Sandboxing — Option b bash cache + WASM reader; ACCEPTED 2026-05-15; gates Part C; Option (a) REJECTED) |
| Stories | S-15.15 |
| L2 Invariants | (none currently assigned — this BC is a process-automation gate; no L2 domain invariants apply) |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.15 must achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-5.39.004 — governs validate-burst-log hook (sister PostToolUse hook; same hook-sdk pattern + fail-open + path-component-strict guard)
- BC-5.39.005 — governs validate-state-structure Phase 1 hook (sister PostToolUse hook)
- BC-5.39.006 — governs validate-dispatch-advance WASM hook (sister PostToolUse hook; same crate scaffolding pattern)
- BC-4.11.001 — validates write targets against artifact-path-registry (sister PostToolUse hook; structural analog for path validation)

## Architecture Anchors

- `crates/hook-plugins/validate-policies-schema/` — hook implementation (pure logic functions + effectful orchestration); schema check in `validate_schema`; ID format check in `check_policy_id_format`; lint_hook existence check in `check_lint_hook_exists`; YAML parse in `parse_policies_yaml`; Part C in `check_cargo_advisory`
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook; `host::log_warn(message)` for advisory-level non-blocking log entries
- `crates/hook-sdk/src/result.rs` — `HookResult` enum: `Continue`, `Block { reason }`, `Error { message }`; `HookResult::block_with_fix(hook, reason, recommendation, code)` constructor for canonical block messages; NOTE: there is NO `HookResult::Advisory` variant — advisory behavior is implemented as `HookResult::Continue` + `host::log_warn`
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `tool = "Edit|Write"` and two file targets: `policies.yaml` (Part B) and `td-*-dispatch.md` glob (Part C)
- `.factory/policies.yaml` — target governance file; schema defined by this BC
- `plugins/vsdd-factory/hooks/update-cargo-audit-cache.sh` — bash data-provisioning script (NOT a hook plugin); invokes `cargo audit --json` and writes cache file; NOT registered as a hook plugin per ADR-021 D-337 scope note
- `specs/architecture/decisions/ADR-021-wasm-cargo-audit-sandboxing.md` — Part C gate; cargo-audit cache provisioning architecture; Option (b) ACCEPTED; Option (a) REJECTED per §"Alternatives Considered — Option (a)" (lines 245-258)

## Story Anchor

S-15.15 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M3 story; Parts A + B + C)

## VP Anchors

VP IDs pending VP-INDEX allocation by state-manager at S-15.15 post-merge burst.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.2 | 2026-05-19 | Pass-2 adversary fix-burst (product-owner; brownfield-backfill M3 3M3a-r pass-2; INV-017 applied). Closes F-BC008P2-001 (CRITICAL: id format), F-BC008P2-002 (CRITICAL: PC10 exec_subprocess), F-BC008P2-003 (HIGH: invariant 5 self-contradiction), F-BC008P2-004 (MEDIUM: PC2 orphan section), F-BC008P2-005 (MEDIUM: ADR-021 cite), F-BC007P2-006 (MEDIUM: ADR-021 Open Sub-Questions PC13 cite), F-BC008P2-007 (LOW: phase), F-BC008P2-009 (NITPICK: changelog format). See body changes for INV-017 evidence. |
| 1.1 | 2026-05-18 | Pass-1 adversary fix-burst. F-BC008P1-001 DO NOT ACT (FALSE POSITIVE — TD-VSDD-101 registered; env-var present; adversary grepped stale local main). Closes F-BC008P1-002..020. CRITICAL: PC13 completely rewritten — ADR-021 Option (a) (embedded RUSTSEC lookup table) is REJECTED at line 251; PC13 now reflects Option (b): WASM hook reads cargo-audit-cache.json via host::read_file; bash script provisions cache; no embedded table in WASM binary (F-BC008P1-002). HIGH: WASM sandboxing constraint documented: host::exec_subprocess NOT available; WASM reads cache file only (F-BC008P1-003); Part C advisory escalation threshold: 1+ HIGH/CRITICAL → block; all MEDIUM/LOW → advisory-log + Continue (F-BC008P1-004); PC7 lint_hook+codified_at coupling rationale documented (F-BC008P1-005); Invariant 5 severity enum: HIGH and MEDIUM per policies.yaml corpus (line 16 comment) with P0/P1/P2/P3/P4 alternative accepted (F-BC008P1-006); policies.yaml file-size cap 512 KiB explicit with META-LEVEL-24 rationale (F-BC008P1-007). MEDIUM: PC2 scope clarified: required-mandatory keys only (F-BC008P1-008); EC-021 Part-C per-advisory emission specified (not batched) (F-BC008P1-009); HookResult::Advisory references replaced with HookResult::Continue + host::log_warn — no Advisory variant in hook-sdk (F-BC008P1-010); Part A/B/C invocation order documented: A→B→C, C last due to I/O cost (F-BC008P1-011); ADR-021 Option (b) WASM integration documented inline in PC10 (F-BC008P1-012); EC-021 YAML syntax error EC added; prior EC-021 renumbered EC-022 (F-BC008P1-013). LOW: lint_hook multi-segment slug regex `^[a-z0-9-]+:[a-z0-9-]+$` specified in postcondition 6 (F-BC008P1-015); invariant numbering verified contiguous 1-11 (F-BC008P1-016); YAML parse error test vector row added (F-BC008P1-017); PC identifier columns added to Test Vectors (F-BC008P1-018). NIT: Part A/B/C capitalization standardized (F-BC008P1-019); SS-05 anchor confirmed (F-BC008P1-020). |
| 1.0 | 2026-05-18 | Initial authoring (product-owner; brownfield-backfill S-15.03 M3 wave 3M3a BC authoring). Anchors F-PASS14-004+F-PASS14-006+POLICY-13/16-D-472+ADR-021-Option-b. BC-5.39.008 allocated as next monotonic ID after BC-5.39.007 in ss-05/. lifecycle_status: draft (POL-14 auto-promotion to active on S-15.15 merge). Part C gated on ADR-021 (ACCEPTED 2026-05-15). TD-VSDD-101 CI env-var paper-fix does not affect this BC's invariants (invariant 10). Preemptive cascade lessons applied: path-component-strict guard for both policies.yaml and td-*-dispatch.md arms; is_char_boundary() invariant 11; fail-open invariant 9; 524288 max_bytes; cascade-all-violations invariant 8; advisory-not-block for Part C per ADR-021; advisory for extra fields (forward-compat). |
