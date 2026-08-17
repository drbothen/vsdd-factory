---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-11T23:00:00Z
phase: 5
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 5
previous_review: adv-s21.09-local-pass-4.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 5)

**Verdict: NOT-CLEAN**
**Finding summary: 0 BLOCKER / 1 HIGH / 4 MEDIUM / 4 LOW / 2 NIT**
**Reviewed commit: `54ab6802` (feature/S-21.09)**
**LOCAL streak: 0/3 — five passes, zero CLEAN**
**D-chain: D-972**

**Convergence note:** Pass 5 found **zero story-vs-reality drift** and **zero pre-existing-code defects**. The spec has converged to the implementation; what remains is control-completeness on the gate: test comments incorrectly describe assertion direction after semantic inversion, one structural gap requires ADR-043 ratification, and several carry-over verification gaps persist.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P05` for this pass
- `<SEV>`: `HIGH`, `MED`, `LOW`
- `<SEQ>`: Three-digit sequence

---

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P04-HIGH-001 | HIGH | DEFERRED | Requires ADR-043 ratification; no implementer action possible without ratification ruling; tracked as ADV-BB-P05-MED-001 at reduced severity since no structural regression introduced |
| ADV-BB-P04-HIGH-002 | HIGH | PARTIALLY_RESOLVED | Prose threat-model acceptance added to module doc replacing ADR-043 cite; T-026 inversion introduced to establish acceptance boundary — semantics of T-026 inverted correctly at 4 assertion sites but doc comment not updated; introduces ADV-BB-P05-HIGH-001 |
| ADV-BB-P04-MED-001 | MEDIUM | RESOLVED | T-030 now parses `hooks-registry.toml` at test time via `toml::Value` to derive `hook_plugins_dir`; hardcoded string removed |
| ADV-BB-P04-MED-002 | MEDIUM | RESOLVED | T-033 added: `trusted_prefixes: []` fixture; gate returns `Err(PolicyViolation::EmptyPrefixList)`; AC-2 now has load-bearing coverage |
| ADV-BB-P04-MED-003 | MEDIUM | RESOLVED | T-015: `std::fs::set_permissions(&tmp, Permissions::from_mode(0o777))` added before gate call; world-writable rejection path now exercised |
| ADV-BB-P04-MED-004 | MEDIUM | RESOLVED | Story frontmatter `bcs:` updated: BC-1.05.035 v2.4 → v2.5 in story v1.11 |
| ADV-BB-P04-MED-005 | MEDIUM | PARTIALLY_RESOLVED | T-021 and T-022 now use `env!("CARGO_MANIFEST_DIR")` to derive `artifact-path-registry.yaml` path; T-023 still uses hardcoded relative path — see ADV-BB-P05-MED-003 |
| ADV-BB-P04-LOW-001 | LOW | UNRESOLVED | Module doc "refuses to execute setuid binaries" claim still present; gate still inert for bare names |
| ADV-BB-P04-LOW-002 | LOW | UNRESOLVED | T-019 assertion still `output.contains("registry parity check failed")`; no exit-code check added |
| ADV-BB-P04-LOW-003 | LOW | PARTIALLY_RESOLVED | T-028 now checks `doc.contains("TOCTOU risk accepted under defined threat model")` rather than `doc.contains("ADR-043")`; check remains a string-presence gate — see ADV-BB-P05-MED-004 |
| ADV-BB-P04-LOW-004 | LOW | UNRESOLVED | T-027 `// passes per D-971 waiver: W3 wave-gate WAIVED` comment unchanged; misattribution persists |
| ADV-BB-P04-LOW-005 | LOW | UNRESOLVED | T-031 `merged_count: 107` fixture unchanged; stale latent fragility persists |
| ADV-BB-P04-NIT-001 | NIT | UNRESOLVED | `use super::*` glob import in test module unchanged |

---

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### ADV-BB-P05-HIGH-001: CWE-20 — T-026 doc comment claims rejection; post-inversion test asserts acceptance

- **Severity:** HIGH
- **Category:** false-verification-signal
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-026
- **Description:** During the pass-4 fix burst, T-026's semantics were inverted to establish the positive-control side of the threat-model acceptance boundary (from testing that an undeclared-plugin path triggers orphan-parity failure, to testing that a declared+git-tracked plugin path produces `Ok(())`). All four assertion sites were correctly changed from `assert!(result.is_err())` to `assert!(result.is_ok())`. However, T-026's inline doc comment `// verifies that undeclared plugins trigger the orphan-parity failure` was not updated. The comment is now factually false: T-026 no longer verifies a failure; it verifies a success. This is a mutation-proof sentence — a grep-based test gate checking for "orphan-parity" would pass regardless of the assertion direction, providing a false signal that the failure path is covered.
- **Evidence:** `git show 54ab6802:crates/factory-dispatcher/tests/bundle_orphan_check.rs` shows T-026 with `assert!(result.is_ok())` immediately following the comment `// verifies that undeclared plugins trigger the orphan-parity failure`. The four updated assertion sites each assert `Ok(())`. No assertion in T-006..T-026 tests the undeclared-plugin failure path that T-026's comment claims to cover.
- **Proposed Fix:** Update T-026's doc comment to accurately describe post-inversion semantics: `// verifies that a declared + git-tracked plugin produces Ok(()) from the orphan-check gate`. Audit T-006..T-034 for any other test comments whose polarity does not match their assertion direction.

### MEDIUM

#### ADV-BB-P05-MED-001: C-1 structural gap persists — T-012 fixture pre-resolves path; bare-name→prefix-resolution chain absent (downgraded from HIGH)

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** BC-1.05.035 AC-1; `crates/policy15-gate/tests/path_staging.rs` T-012
- **Description:** The load-time resolution gap identified as ADV-BB-P04-HIGH-001 is structurally unchanged. T-012's fixture still provides `allowed_binaries: ["/usr/local/bin/cat"]` (pre-resolved), bypassing the `trusted_prefixes`-based resolution step entirely. The gap is downgraded from HIGH to MEDIUM because the finding is explicitly gated on ADR-043 ratification and no new structural regression was introduced. The blocking security issue C-1 (CWE-706) remains open.
- **Evidence:** `grep -n "allowed_binaries" crates/policy15-gate/tests/path_staging.rs` returns pre-resolved absolute paths in T-012 fixture; no fixture has a bare-name entry with `trusted_prefixes` exercising the resolution chain.
- **Proposed Fix:** Pending ADR-043 ratification: add T-012b (bare name `"cat"` resolved via `trusted_prefixes: ["/usr/local/bin"]`; assert success) and T-012c (bare name not under any prefix; assert failure). No implementer action before ADR-043 disposition.

#### ADV-BB-P05-MED-002: T-034 covers positive-resolution path; negative-resolution failure path absent

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-034
- **Description:** T-034 (added in v1.11) verifies that a plugin declared in `hooks-registry.toml` and present in the git index produces `Ok(())` from the orphan check (positive control). No corresponding negative control exists where the plugin is declared but NOT git-tracked (i.e., `git ls-files --error-unmatch` would fail). The T-026 inversion removed the previous failure-case test; T-034 replaces only the positive case. The failure path is now untested.
- **Evidence:** `grep -n "fn test_" crates/factory-dispatcher/tests/bundle_orphan_check.rs | grep -v "ok\|pass\|success"` returns no results for T-026..T-034. All tests in that range assert `Ok(())`.
- **Proposed Fix:** Add T-035: fixture declares a plugin in `hooks-registry.toml` that is NOT present in `git ls-files`; assert `Err(OrphanCheckError::PluginNotTracked { name: "..." })`.

#### ADV-BB-P05-MED-003: T-023 artifact-path still hardcoded — partial fix from ADV-BB-P04-MED-005

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** `crates/policy15-gate/tests/` T-023
- **Description:** T-021 and T-022 were fixed to use `env!("CARGO_MANIFEST_DIR")` for `artifact-path-registry.yaml` path derivation; T-023 was not updated and still uses the hardcoded relative path `plugins/vsdd-factory/config/artifact-path-registry.yaml`. This test will fail in operator-level installations where `config/` is at a different absolute path.
- **Evidence:** `grep -n "artifact-path-registry.yaml" crates/policy15-gate/tests/*.rs` returns one hit in T-023 with the hardcoded string; zero hits in T-021/T-022 (now using `CARGO_MANIFEST_DIR`).
- **Proposed Fix:** Update T-023 to use `env!("CARGO_MANIFEST_DIR")` for path derivation, consistent with T-021/T-022.

#### ADV-BB-P05-MED-004: T-028 updated but remains vacuous string-presence gate

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-028
- **Description:** ADV-BB-P04-HIGH-002 required replacing the ADR-043 citation in the module doc with a prose threat-model acceptance. The implementer complied: module doc now contains "TOCTOU risk accepted under defined threat model." T-028 was updated from `assert!(doc.contains("ADR-043"))` to `assert!(doc.contains("TOCTOU risk accepted under defined threat model"))`. The check is still a string-presence gate on a prose sentence — mutation-proof in the same way as the original. T-028 cannot distinguish between a correct acceptance boundary and a fabricated one; it verifies the string exists, not that the boundary is operationally correct.
- **Evidence:** `git show 54ab6802:crates/policy15-gate/tests/path_staging.rs` shows T-028 with `assert!(doc.contains("TOCTOU risk accepted under defined threat model"))`. No structured assertion on acceptance class or boundary condition.
- **Proposed Fix:** Replace the string-presence check with a structured assertion: verify that the module-level `#[doc]` attribute or `const THREAT_MODEL_ACCEPTANCE: &str` constant contains a machine-checkable field (e.g., `cwe_class: CWE-362` and `accepted_by: "human-ratification"`). This converts the gate from prose-presence to structured-value.

### LOW

#### ADV-BB-P05-LOW-001: Module doc claims "refuses to execute setuid binaries" — gate is inert (carry-over)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` module-level doc
- **Description:** Unchanged from ADV-BB-P04-LOW-001. Module doc states "refuses to execute setuid binaries" but the `refuse_setuid` gate never fires in production (bare names always produce ENOENT before path lookup). The doc is a false safety claim. D-971 recorded this as a HIGH SECURITY finding in the drift items table; the implementation has not changed.
- **Proposed Fix:** Remove the claim or implement proper path-resolve-then-stat logic per D-971 drift item direction.

#### ADV-BB-P05-LOW-002: T-019 assertion uses substring match — no exit-code check (carry-over)

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/registry_parity.rs` T-019
- **Description:** Unchanged from ADV-BB-P04-LOW-002. `assert!(output.contains("registry parity check failed"))` matches any output containing that string, including panic messages. No exit-code assertion.
- **Proposed Fix:** Assert exit code is non-zero AND structured log field `check_name: "registry_parity"` appears in output.

#### ADV-BB-P05-LOW-003: T-027 misattributed D-971 waiver comment (carry-over)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-027
- **Description:** Unchanged from ADV-BB-P04-LOW-004. Inline comment `// passes per D-971 waiver: W3 wave-gate WAIVED` misattributes the D-971 wave-gate sequencing waiver to a test requirement. D-971 waived the W3 wave-gate for story ordering, not for any test.
- **Proposed Fix:** Remove the misattributed comment; attach gap to the correct decision or add a TD entry with explicit human direction.

#### ADV-BB-P05-LOW-004: T-031 sprint-state fixture uses stale `merged_count: 107` (carry-over)

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/sprint_state_tests.rs` T-031
- **Description:** Unchanged from ADV-BB-P04-LOW-005. Fixture sets `merged_count: 107` but assertion does not check `merged_count`. Stale fixture introduces latent fragility.
- **Proposed Fix:** Either assert on `merged_count` (and update the value) or remove `merged_count` from the fixture.

### NIT

#### ADV-BB-P05-NIT-001: `#[cfg(test)]` module uses `use super::*` glob import (carry-over)

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` test module
- **Description:** Unchanged from ADV-BB-P04-NIT-001. `use super::*` conflicts with workspace lint (`wildcard_imports` forbidden per CI clippy config).
- **Proposed Fix:** Replace with explicit imports of symbols actually used.

#### ADV-BB-P05-NIT-002: T-034 function name diverges from established naming convention

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-034
- **Description:** T-034 is named `test_parity_check_passes_full_suite`. The established convention for T-006..T-033 uses the form `test_<noun>_<verb>_<condition>` (e.g., `test_declared_plugin_tracked_parity_ok`). The divergent name breaks the naming pattern and makes automated test-name filtering harder.
- **Proposed Fix:** Rename to `test_full_suite_declared_tracked_parity_ok` following established convention.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 4 |
| LOW | 4 |
| NIT | 2 |

**Overall Assessment:** block
**Convergence:** spec-vs-reality drift: **zero**; pre-existing-code defects: **zero**; remaining: control-completeness gaps
**Readiness:** requires revision

ADV-BB-P05-HIGH-001 is a direct result of the pass-4 fix burst inverting T-026 semantics without updating T-026's doc comment. MED-001 remains gated on ADR-043 ratification (a human decision). MED-002 through MED-004 and all LOWs are implementer-addressable. However BC-5.39.001 3-CLEAN protocol requires zero findings of any severity for a CLEAN pass.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 2 (HIGH-001 new from T-026 inversion; MED-002 new from T-034 coverage gap) |
| **Carry-over / downgrade findings** | 9 (MED-001 downgraded from P04-HIGH-001; MED-003 carry from P04-MED-005 partial; MED-004 carry from P04-LOW-003 partial; LOW-001..004 unchanged; NIT-001 unchanged; NIT-002 new) |
| **Novelty score** | 2 / (2 + 9) = 0.18 |
| **Median severity** | LOW/NIT boundary |
| **Severity trajectory (BLOCKER)** | 2→2→0→0→0 |
| **Severity trajectory (HIGH)** | 3→2→3→2→1 |
| **Total finding trajectory** | →13→11 (pass-4 total 13; pass-5 total 11) |
| **Verdict** | FINDINGS_REMAIN |
