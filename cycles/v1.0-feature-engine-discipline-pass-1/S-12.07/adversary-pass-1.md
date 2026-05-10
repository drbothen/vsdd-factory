---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-10T00:00:00Z
phase: 5
inputs:
  - story_spec
  - bcs (BC-4.12.001..005)
  - vps (VP-073..076)
  - branch_diff (origin/develop..3c2b39b3)
input-hash: "[pending-recompute]"
traces_to: prd.md
pass: 1
previous_review: null
story_id: S-12.07
verdict: CRITICAL
convergence_reached: false
---

# Adversarial Review: S-12.07 Wave-Context Resolver (Pass 1)

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

Cycle prefix for this pass: `BF1` (v1.0-brownfield-backfill)

Examples from this pass: `ADV-BF1-P01-CRIT-001`, `ADV-BF1-P01-HIGH-001`

## Part A — Fix Verification (pass >= 2 only)

_Pass 1 — no prior findings to verify._

## Part B — New Findings (or all findings for pass 1)

**Severity Verdict:** CRITICAL. This branch ships a resolver crate that is structurally incapable of working end-to-end with the dispatcher it claims to integrate with. Three independent critical defects each individually break the E-12 platform: (1) the new `resolvers-registry.toml` entry is missing the REQUIRED `context_key` field — the dispatcher will fail to start with `ResolverLoadError::ParseError` on any factory that ships this registry; (2) the resolver emits its merge key as `"wave-context"` (hyphen) but the downstream consumer (S-12.08, ADR-018, AC-007 of S-12.08) reads `plugin_config["wave_context"]` (underscore) — the consumer will see a missing key in production; (3) the `WaveState` schema invented by the implementer (`current_cycle`, `current_wave`, `stories` at top level) does NOT match the actual `wave-state.yaml` schema used elsewhere in this repo (`waves: [{ wave, stories, stories_merged, gate_status }]`) — against any real wave-state.yaml, the resolver will always return `value: None`. Beyond the three CRITICAL items, there are HIGH-severity test-quality issues and architectural concerns.

### CRITICAL

#### ADV-BF1-P01-CRIT-001: resolvers-registry.toml missing required context_key field

- **Severity:** CRITICAL
- **Category:** code-defect
- **Location:** `plugins/vsdd-factory/resolvers-registry.toml:9-12`
- **Description:** Registry entry missing required `context_key` field; dispatcher exits at startup with `ResolverLoadError::ParseError`.
- **Evidence:** The TOML deserialization struct in `crates/factory-dispatcher/src/resolver_loader.rs:108-133` is annotated with `#[serde(deny_unknown_fields)]` AND declares `context_key: String` (no `#[serde(default)]`). The shipped registry entry has only `name`, `plugin`, `path_allow` — `context_key` is absent. `main.rs:312-350` calls `load_registry()` at startup and exits on error.
- **Proposed Fix:** Add `context_key = "wave_context"` to registry entry (also fixes ADV-BF1-P01-CRIT-002 naming).

#### ADV-BF1-P01-CRIT-002: Merge key collision — "wave-context" vs "wave_context"

- **Severity:** CRITICAL
- **Category:** contract-drift
- **Location:** `crates/vsdd-context-resolvers/src/lib.rs:64,74,109,117`; `plugins/vsdd-factory/resolvers-registry.toml:10`
- **Description:** Resolver emits merge key `"wave-context"` (hyphen) but all downstream authorities use `"wave_context"` (underscore). Consumer sees missing key in production.
- **Evidence:** Five authorities (BC-4.12.005 PC7, BC-4.12.005 canonical test vectors, ADR-018, S-12.08 AC-001, S-12.08 AC-007) use `wave_context` (underscore). The S-12.07 spec and implementation use `wave-context` (hyphen). Merge resolution in `factory-dispatcher/src/resolver.rs:265-310` uses registry-declared `context_key`, not output key — so the consumer queries `wave_context` and finds nothing.
- **Proposed Fix:** Rename to `wave_context` everywhere: lib.rs output key, registry name + context_key, story spec (AC-001/009). S-12.08 spec also needs sync.

#### ADV-BF1-P01-CRIT-003: WaveState schema fabricated — doesn't match canonical schema

- **Severity:** CRITICAL
- **Category:** sibling-coverage / code-defect
- **Location:** `crates/vsdd-context-resolvers/src/wave_context.rs:31-39`
- **Description:** S-12.07 invents a flat WaveState schema that doesn't match the actual wave-state.yaml written by the update-wave-state-on-merge producer. Against any real file, the resolver always returns `value: None`.
- **Evidence:** Canonical schema (used by `crates/hook-plugins/update-wave-state-on-merge/src/lib.rs:154-181` — the PRODUCER of wave-state.yaml) is `waves: Vec<WaveEntry { wave, stories, stories_merged, gate_status, ... }>`. S-12.07's struct is `current_cycle, current_wave, stories` flat. Against real file, all Option fields are None → `value: None` always. T-2 task explicitly says to read STATE.md for `current_cycle` — implementation does not.
- **Proposed Fix:** Adopt canonical schema. Read STATE.md for current_cycle. Determine active wave from `gate_status` (last incomplete wave).

### HIGH

#### ADV-BF1-P01-HIGH-001: Story spec AC-007 drift — plugin_config["project_dir"] vs top-level

- **Severity:** HIGH
- **Category:** spec-defect
- **Location:** `.factory/stories/S-12.07-...md:111-113`; `crates/hook-sdk/src/resolver.rs:18-25`
- **Description:** Story AC-007 says reads `input.plugin_config["project_dir"]`; actual ResolverInput has `project_dir: String` as TOP-LEVEL field. Implementation uses top-level correctly.
- **Evidence:** Spec text says `plugin_config["project_dir"]`; hook-sdk struct defines `project_dir` at top level.
- **Proposed Fix:** Amend AC-007 text.

#### ADV-BF1-P01-HIGH-002: AC-005/006 deferred without justification; VP-076 untested

- **Severity:** HIGH
- **Category:** test-defect / contract-drift
- **Location:** `crates/vsdd-context-resolvers/tests/capability_confinement_test.rs:30-80`
- **Description:** VP-076 lifecycle "Harness instantiated" anchored to S-12.07 delivery. Three integration tests are `#[ignore]` + `unimplemented!()` with no documented justification.
- **Evidence:** VP-076.md:243-251 lifecycle anchor = test-writer (S-12.07 delivery). Justification (BC-1.13.001 INV1) is one-way — dev-dependencies don't violate it. S-12.07 DoD line 291 explicitly requires VP-076 tests to pass.
- **Proposed Fix (Option B):** Add `wasmtime` + `factory-dispatcher = { path = "..." }` to `[dev-dependencies]`. Implement tests with real wasmtime + ResolverLoader. OR amend VP-076 spec to defer harness to S-12.08 bats — currently the deferral is undocumented.

#### ADV-BF1-P01-HIGH-003: AC-010 textual heuristic test is fragile; canonical clippy lint not configured

- **Severity:** HIGH
- **Category:** test-defect / process-gap
- **Location:** `crates/vsdd-context-resolvers/tests/wave_context_test.rs:352-381`
- **Description:** Textual contains-checks miss multi-line breaks, comments, edge cases. AC-010 canonical is `cargo clippy --deny clippy::unwrap_used` but neither crate nor workspace configures this deny.
- **Evidence:** crates/vsdd-context-resolvers/Cargo.toml has only `workspace = true` lints. Workspace lints don't deny `unwrap_used`. The `unwrap_or_default()` at lib.rs:82 passes only by accident (literal `.unwrap()` substring not present).
- **Proposed Fix:** Add `[lints.clippy] unwrap_used = "deny"; expect_used = "deny"; panic = "deny"` to Cargo.toml. Remove the textual test or note its weakness.

#### ADV-BF1-P01-HIGH-004: #[allow(clippy::expect_used)] is workaround for fixable macro debt

- **Severity:** HIGH
- **Category:** architecture / sibling-coverage
- **Location:** `crates/vsdd-context-resolvers/src/lib.rs:52`; `crates/hook-sdk-macros/src/resolver_macro.rs:88,93`
- **Description:** resolver_macro.rs emits two `.expect()` calls. Per BC-4.12.004 INV1, no `expect()` in production code. The allow attribute papers over a fixable macro issue.
- **Evidence:** `#[allow(clippy::expect_used)]` at lib.rs:52; resolver_macro.rs emits input-deserialization and output-serialization expects.
- **Proposed Fix:** Modify resolver_macro.rs to match-return on serde failures. Coordinate with hook-sdk if same pattern exists in `#[hook]`.

### MEDIUM

#### ADV-BF1-P01-MED-001: test_BC_4_12_002_project_dir_is_top_level_field tautology (POL-11)

- **Severity:** MEDIUM
- **Category:** test-defect
- **Location:** `crates/vsdd-context-resolvers/tests/wave_context_test.rs:234-251`
- **Description:** Test calls `make_input("/tmp/...")` and asserts the field equals the input — never calls production fn. Violates POL-11 without opt-out comment.
- **Proposed Fix:** Add `/// data-shape pin` doc comment per POL-11 opt-out, OR rewrite to exercise `resolve_wave_context_pure`.

#### ADV-BF1-P01-MED-002: resolve_wave_context_pure ignores _input parameter

- **Severity:** MEDIUM
- **Category:** code-defect / spec-drift
- **Location:** `crates/vsdd-context-resolvers/src/lib.rs:100`
- **Description:** Function body ignores `_input`. VP-075 determinism proof claims (input, wave_state) → output, but function reduces to (wave_state) → output. Proptest covers determinism over inputs that are never used.
- **Proposed Fix:** Drop unused param OR use `input.event_type` for filtering per BC-4.12.002 EC-006.

#### ADV-BF1-P01-MED-003: EC-002 event_type filtering choice undocumented

- **Severity:** MEDIUM
- **Category:** spec-defect / code-defect
- **Location:** `crates/vsdd-context-resolvers/src/lib.rs:100-121`
- **Description:** Implementation returns wave_context for ALL event types. Choice is permitted by spec but not documented or tested.
- **Proposed Fix:** Add comment + test affirming "returns regardless of event_type".

#### ADV-BF1-P01-MED-004: serde_yaml 0.9.34 deprecated; direct-dep surface accumulating

- **Severity:** MEDIUM
- **Category:** architecture
- **Location:** `Cargo.toml:48-51` workspace
- **Description:** Workspace pins last-release deprecated serde_yaml. New crate adds direct dep on it.
- **Proposed Fix:** Add TD entry tracking migration to serde_yml or yaml-rust2.

### LOW

#### ADV-BF1-P01-LOW-001: STATE.md not read; cycle_id source uncertain

- **Severity:** LOW
- **Category:** code-defect / spec-drift
- **Location:** `crates/vsdd-context-resolvers/src/lib.rs:53-85`
- **Description:** Story T-2 requires reading both `.factory/wave-state.yaml` AND `.factory/STATE.md`. Implementation reads only the former. Subsumed by ADV-BF1-P01-CRIT-003 fix.

#### ADV-BF1-P01-LOW-002: Workspace path traversal in test is brittle

- **Severity:** LOW
- **Category:** test-defect
- **Location:** `crates/vsdd-context-resolvers/tests/wave_context_test.rs:316-340`
- **Description:** Hardcoded `.parent().parent()` walk assumes depth 2 from workspace root.

#### ADV-BF1-P01-LOW-003: unwrap_or_else(|e| panic!()) in test bypasses AC-010 heuristic

- **Severity:** LOW
- **Category:** test-defect
- **Location:** `crates/vsdd-context-resolvers/tests/wave_context_test.rs:324-329,360`
- **Description:** Pattern not caught by AC-010 text scan. Internally consistent with carve-out (test files exempt) but worth documenting.

### NITPICK

#### ADV-BF1-P01-LOW-004: Module alias `use vsdd_hook_sdk::resolver as resolver_macro` is obscure

- **Severity:** NITPICK
- **Category:** code-defect
- **Location:** `crates/vsdd-context-resolvers/src/lib.rs:26`
- **Description:** Dual-namespace import works but is unusual and may confuse readers.

### Process-Gap Findings

#### ADV-BF1-P01-HIGH-005: No CI lint gate for clippy::unwrap_used on resolver/hook crates

- **Severity:** HIGH
- **Category:** process-gap
- **Description:** AC-010 expects clippy enforcement but neither crate nor workspace configures the deny. F-001 escaped CI.
- **Proposed Fix:** Add `[workspace.lints.clippy] unwrap_used = "deny"; expect_used = "deny"; panic = "deny"` at root.

#### ADV-BF1-P01-MED-005: No automated registry-vs-schema validation

- **Severity:** MEDIUM
- **Category:** process-gap
- **Description:** A test parsing `resolvers-registry.toml` against `ResolverEntryToml` would have caught ADV-BF1-P01-CRIT-001 at PR time.
- **Proposed Fix:** Add workspace integration test that parses shipped registry.

## Sibling-Coverage Audit (P-005)

**Pattern: wave-state.yaml schema** — `update-wave-state-on-merge` and `warn-pending-wave-gate` use canonical `waves: Vec<WaveEntry>`. S-12.07 invents flat schema. **DIVERGENT** — see ADV-BF1-P01-CRIT-003.

**Pattern: Cargo crate-type** — Other plugins use `[[bin]]` WASI command. Example resolver crate uses `["cdylib"]`. S-12.07 uses `["cdylib", "rlib"]`. New pattern; codify if intentional.

**Pattern: clippy::expect_used allow** — S-12.07 introduces the first function-level allow. If ADV-BF1-P01-HIGH-003 fix lands (workspace deny), every `#[hook]` crate may need similar allows UNLESS macros are fixed (ADV-BF1-P01-HIGH-004).

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 3 |
| HIGH | 5 |
| MEDIUM | 5 |
| LOW | 3 |
| NITPICK | 1 |

**Overall Assessment:** block
**Convergence:** FINDINGS_REMAIN — iterate
**Readiness:** requires revision — must address CRITICAL findings before any merge

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 17 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.00 |
| **Median severity** | 2.5 |
| **Trajectory** | 17 |
| **Verdict** | FINDINGS_REMAIN |

<!--
  Pass 1 fresh-context review. All findings are new. Convergence not reached.
  Must resolve ADV-BF1-P01-CRIT-001/002/003 + ADV-BF1-P01-HIGH-002/003 before Pass 2.
-->
