---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T03:00:00Z
phase: 6
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 6
previous_review: adv-s21.09-local-pass-5.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 6)

**Verdict: NOT-CLEAN**
**Finding summary: 0 BLOCKER / 1 HIGH / 2 MEDIUM / 4 LOW / 2 NIT**
**Reviewed commit: `b5ec1710` (feature/S-21.09)**
**LOCAL streak: 0/3 — six passes, zero CLEAN**
**D-chain: D-972**

**Convergence note:** Pass 6 reviewed story v1.13 (30 tests T-006..T-035 all green at b5ec1710). Three pass-5 findings resolved (HIGH-001 T-026 doc comment corrected; MED-002 T-035 added for failure-path negative control; MED-003 T-023 hardcoded path fixed). One new HIGH emerged from mutation analysis: 1 of 18 mutants survived. Two MEDs carry forward (ADR-043-gated structural gap; T-028 vacuous gate). All four LOWs and both NITs remain unchanged.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P06` for this pass
- `<SEQ>`: Three-digit sequence

---

## Part A — Fix Verification

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P05-HIGH-001 | HIGH | RESOLVED | T-026 doc comment updated to accurately describe post-inversion semantics: `// verifies that a declared + git-tracked plugin produces Ok(()) from the orphan-check gate`. Story v1.12 fix confirmed at b5ec1710. |
| ADV-BB-P05-MED-001 | MEDIUM | DEFERRED | ADR-043 still not ratified; no implementer action possible. Carries as ADV-BB-P06-MED-001. |
| ADV-BB-P05-MED-002 | MEDIUM | RESOLVED | T-035 added: fixture declares a plugin in `hooks-registry.toml` not present in `git ls-files`; asserts `Err(OrphanCheckError::PluginNotTracked { name: "..." })`. Failure path now exercised. 30-test suite T-006..T-035 all green at b5ec1710. |
| ADV-BB-P05-MED-003 | MEDIUM | RESOLVED | T-023 updated to use `env!("CARGO_MANIFEST_DIR")` for `artifact-path-registry.yaml` path derivation, consistent with T-021/T-022. |
| ADV-BB-P05-MED-004 | MEDIUM | UNRESOLVED | T-028 still checks `assert!(doc.contains("TOCTOU risk accepted under defined threat model"))`; vacuous string-presence gate unchanged. Carries as ADV-BB-P06-MED-002. |
| ADV-BB-P05-LOW-001 | LOW | UNRESOLVED | Module doc "refuses to execute setuid binaries" claim unchanged. Carries as ADV-BB-P06-LOW-001. |
| ADV-BB-P05-LOW-002 | LOW | UNRESOLVED | T-019 substring match without exit-code assertion unchanged. Carries as ADV-BB-P06-LOW-002. |
| ADV-BB-P05-LOW-003 | LOW | UNRESOLVED | T-027 misattributed D-971 waiver comment unchanged. Carries as ADV-BB-P06-LOW-003. |
| ADV-BB-P05-LOW-004 | LOW | UNRESOLVED | T-031 stale `merged_count: 107` fixture unchanged. Carries as ADV-BB-P06-LOW-004. |
| ADV-BB-P05-NIT-001 | NIT | UNRESOLVED | `use super::*` glob import unchanged. Carries as ADV-BB-P06-NIT-001. |
| ADV-BB-P05-NIT-002 | NIT | UNRESOLVED | T-034 function name diverges from convention; still present at b5ec1710. Carries as ADV-BB-P06-NIT-002. |

---

## Part B — New Findings

### HIGH

#### ADV-BB-P06-HIGH-001: CWE-1023 — 1 of 18 mutants survived mutation analysis; `extract_hook_plugin_name` normalisation boundary untested

- **Severity:** HIGH
- **Category:** false-verification-signal
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs`; `extract_hook_plugin_name` in `bundle_orphan_check.rs`
- **Description:** Mutation analysis applied 18 mutants to the T-006..T-035 suite; 17 killed. The 1 surviving mutant is in the `extract_hook_plugin_name` 5-step registry-parent-relative normalisation path: removing the final path-component strip (step 4 of 5 — the `file_name()` extraction before prefix comparison) still produces a passing suite. This means the test suite cannot distinguish between a correct 5-step normalisation and a 4-step normalisation that omits the final component strip. The surviving mutant produces subtly wrong plugin names in mixed-depth registry structures (plugins declared at non-canonical depths relative to `hooks_dir`), which no current test fixture exercises.
- **Evidence:** `cargo mutants --test-tool cargo -- crates/factory-dispatcher/tests/bundle_orphan_check.rs` at b5ec1710 reports `mutants: 18, killed: 17, survived: 1`. Surviving mutant: deletion of `path.file_name()?.to_str()?` extraction step in `extract_hook_plugin_name`. All T-006..T-035 pass with the mutation applied.
- **Proposed Fix:** Add T-036: fixture with a plugin declared at a non-canonical depth (e.g., `wasm/subdir/plugin.wasm` relative to `hooks_dir`); assert `extract_hook_plugin_name` returns the correct two-component relative path. This must be killed by the mutant.

### MEDIUM

#### ADV-BB-P06-MED-001: C-1 structural gap persists — ADR-043 gated (carry from P05-MED-001)

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** BC-1.05.035 AC-1; `crates/policy15-gate/tests/path_staging.rs` T-012
- **Description:** Unchanged from ADV-BB-P05-MED-001. T-012 fixture still provides `allowed_binaries: ["/usr/local/bin/cat"]` (pre-resolved), bypassing the `trusted_prefixes`-based resolution step. Gated on ADR-043 ratification. No implementer action before ADR-043 disposition.
- **Evidence:** `grep -n "allowed_binaries" crates/policy15-gate/tests/path_staging.rs` returns pre-resolved absolute paths in T-012 fixture.
- **Proposed Fix:** Pending ADR-043 ratification: add T-012b (bare name resolved via `trusted_prefixes`) and T-012c (bare name not under any prefix; assert failure).

#### ADV-BB-P06-MED-002: T-028 remains vacuous string-presence gate (carry from P05-MED-004)

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-028
- **Description:** Unchanged from ADV-BB-P05-MED-004. T-028 checks `assert!(doc.contains("TOCTOU risk accepted under defined threat model"))`. This is mutation-proof: any fabricated string in the module doc would satisfy the assertion. No structured assertion on acceptance class or operational correctness of the boundary.
- **Evidence:** `git show b5ec1710:crates/policy15-gate/tests/path_staging.rs` shows T-028 with string-presence check; no structured acceptance field assertion.
- **Proposed Fix:** Replace with structured assertion on a machine-checkable constant (e.g., `const THREAT_MODEL_ACCEPTANCE: ThreatModelAcceptance` with `cwe_class` and `accepted_by` fields).

### LOW

#### ADV-BB-P06-LOW-001: Module doc claims "refuses to execute setuid binaries" — gate is inert (carry from P05-LOW-001)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` module-level doc
- **Description:** Unchanged. Module doc states "refuses to execute setuid binaries" but the `refuse_setuid` gate never fires in production. D-971 records this as a HIGH SECURITY drift item.
- **Proposed Fix:** Remove the claim or implement proper path-resolve-then-stat logic.

#### ADV-BB-P06-LOW-002: T-019 assertion uses substring match — no exit-code check (carry from P05-LOW-002)

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/registry_parity.rs` T-019
- **Description:** Unchanged. `assert!(output.contains("registry parity check failed"))` matches any output containing that string. No exit-code assertion.
- **Proposed Fix:** Assert non-zero exit code AND structured log field `check_name: "registry_parity"`.

#### ADV-BB-P06-LOW-003: T-027 misattributed D-971 waiver comment (carry from P05-LOW-003)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-027
- **Description:** Unchanged. `// passes per D-971 waiver: W3 wave-gate WAIVED` misattributes the D-971 sequencing waiver to a test.
- **Proposed Fix:** Remove the misattributed comment.

#### ADV-BB-P06-LOW-004: T-031 sprint-state fixture uses stale `merged_count: 107` (carry from P05-LOW-004)

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/sprint_state_tests.rs` T-031
- **Description:** Unchanged. Fixture `merged_count: 107` not asserted upon; stale latent fragility.
- **Proposed Fix:** Assert on `merged_count` or remove from fixture.

### NIT

#### ADV-BB-P06-NIT-001: `use super::*` glob import in test module (carry from P05-NIT-001)

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` test module
- **Description:** Unchanged. `use super::*` conflicts with workspace `wildcard_imports` lint.
- **Proposed Fix:** Replace with explicit imports.

#### ADV-BB-P06-NIT-002: T-034 function name diverges from naming convention (carry from P05-NIT-002)

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-034
- **Description:** Unchanged. `test_parity_check_passes_full_suite` diverges from `test_<noun>_<verb>_<condition>` convention.
- **Proposed Fix:** Rename to `test_full_suite_declared_tracked_parity_ok`.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 2 |
| LOW | 4 |
| NIT | 2 |

**Overall Assessment:** block
**Convergence:** spec-vs-reality drift: **zero**; pre-existing-code defects: **zero**; remaining: mutation-coverage gap + carry-over verification gaps
**Readiness:** requires revision

ADV-BB-P06-HIGH-001 is new: 1 of 18 mutants survived the T-006..T-035 suite, revealing an untested normalisation boundary in `extract_hook_plugin_name`. MED-001 remains gated on ADR-043 (human decision). MED-002 and all LOWs/NITs are implementer-addressable. BC-5.39.001 3-CLEAN protocol requires zero findings of any severity for a CLEAN pass.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 6 |
| **Story version reviewed** | v1.13 |
| **Reviewed commit** | b5ec1710 |
| **New findings** | 1 (HIGH-001 mutation survivor) |
| **Carry-over findings** | 8 (MED-001/MED-002; LOW-001..004; NIT-001..002) |
| **Resolved this pass** | 3 (P05-HIGH-001 T-026 comment; P05-MED-002 T-035 added; P05-MED-003 T-023 CARGO_MANIFEST_DIR) |
| **Mutation testing** | 18 mutants applied; 17 killed; 1 survived |
| **Novelty score** | 1 / (1 + 8) = 0.11 |
| **Median severity** | LOW/NIT boundary |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1 |
| **Total finding trajectory** | →11→9 (pass-5 total 11; pass-6 total 9) |
| **Verdict** | FINDINGS_REMAIN |
