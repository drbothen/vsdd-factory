---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-11T12:00:00
phase: 5
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "b304422"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 4
previous_review: adv-s21.09-local-pass-3.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 4)

**Verdict: DO-NOT-RATIFY**
**Finding summary: 0 BLOCKER / 2 HIGH / 5 MEDIUM / 5 LOW / 1 NIT**
**Reviewed commit: `12f280d1` (feature/S-21.09)**
**LOCAL streak: 0/3 — four passes, zero CLEAN**
**D-chain: D-972**

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P04` for this pass
- `<SEV>`: `HIGH`, `MED`, `LOW`
- `<SEQ>`: Three-digit sequence

---

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P01-HIGH-001 | HIGH | RESOLVED | T-012 without_path negative control added in pass-2 fix |
| ADV-BB-P01-MED-001 | MEDIUM | RESOLVED | extract_hook_plugin_name comment corrected |
| ADV-BB-P01-MED-002 | MEDIUM | RESOLVED | T-028 assertion strengthened |
| ADV-BB-P02-HIGH-001 | HIGH | RESOLVED | extract_hook_plugin_name redesigned with golden-value approach; T-014 FAIL arm added |
| ADV-BB-P02-MED-001 | MEDIUM | PARTIALLY_RESOLVED | Path normalization gap — T-016 added but relative-to-absolute normalization case not covered (see ADV-BB-P04-MED-002) |
| ADV-BB-P03-HIGH-001 | HIGH | PARTIALLY_RESOLVED | T-030 hook_plugins_dir path divergence — implementer added comment but hardcoded path remains; reopened as ADV-BB-P04-MED-001 at lower severity because partial fix acknowledged |

---

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### ADV-BB-P04-HIGH-001: CWE-706 — Load-time resolution absent; T-012 tests surrogate invariant

- **Severity:** HIGH
- **Category:** security-surface
- **Location:** BC-1.05.035 AC-1; `crates/policy15-gate/src/exec_subprocess.rs`
- **Description:** BC-1.05.035 AC-1 requires binary allow-list entries be resolved to absolute paths against trusted-prefix list entries at registry-load time. The implementation performs a bare-name string comparison against a pre-resolved allow-list fixture, not a load-time-resolve-then-compare operation. T-012 passes because its fixture contains the already-resolved absolute path — it never exercises the resolution step. The structural defect C-1 (CWE-706) that ADR-043 §Decision 1 was designed to close remains open in production code.
- **Evidence:** T-012 fixture constructs `allowed_binaries: ["/usr/local/bin/cat"]` (pre-resolved) — the gate code does a `contains()` check. No fixture exists where a bare name `"cat"` is resolved via a `trusted_prefixes: ["/usr/local/bin"]` lookup. The positive-control (bare name → prefix resolution → allow-list membership) path is structurally absent.
- **Proposed Fix:** Implement load-time resolution in the gate function. Add a T-012b fixture where the allow-list contains the bare name `"cat"` and `trusted_prefixes` contains `/usr/local/bin`; assert resolution succeeds. Add a T-012c fixture with a bare name not under any trusted prefix; assert resolution fails. This is gated on ADR-043 ratification.

#### ADV-BB-P04-HIGH-002: CWE-362 — TOCTOU threat-model citation references a proposed (unratified) ADR

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-1.05.035 AC-3; `crates/policy15-gate/src/exec_subprocess.rs` module doc
- **Description:** The module doc cites "TOCTOU risk accepted per ADR-043 §Decision 1 threat model." ADR-043 v1.5 is `status: proposed` — NOT RATIFIED. Citing a proposed ADR as authority for accepting a security risk constitutes a false attestation: BC-1.05.035 AC-3 requires the threat-model boundary be formally established. T-028 checks for the literal string "ADR-043" in the doc; it would pass even if ADR-043 were withdrawn or its status changed.
- **Evidence:** `adv-adr-043-pass-3.md` verdict is DO-NOT-RATIFY; ADR-043 v1.5 frontmatter `status: proposed`. T-028 uses `assert!(doc.contains("ADR-043"))` — a vacuous string-presence control.
- **Proposed Fix:** Either ratify ADR-043 (human decision) or replace the TOCTOU citation with a prose threat-model acceptance in the implementation doc that does not rely on ADR ratification status. T-028 should assert on the presence of the accepted-boundary sentence, not the ADR identifier.

### MEDIUM

#### ADV-BB-P04-MED-001: T-030 hook_plugins_dir divergence still open — comment is not a fix

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-030
- **Description:** The hardcoded test path `plugins/vsdd-factory/hook-plugins/` identified in pass-3 HIGH-001 is still present. The implementer added an inline comment `// TODO: derive from hooks-registry.toml hook_plugins_dir` but did not change the path derivation. Per CLAUDE.md Canonical Principle, a comment is not a fix. T-030 will pass even if `hook_plugins_dir` changes in the registry.
- **Evidence:** `git diff 12f280d1~1 12f280d1 -- tests/path_staging.rs` shows comment addition only; hardcoded string unchanged.
- **Proposed Fix:** Parse `hooks-registry.toml` at test time to derive `hook_plugins_dir`; use that derived value as the expected path. Remove the hardcoded string.

#### ADV-BB-P04-MED-002: Empty trusted-prefix list fallthrough untested

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** BC-1.05.035 AC-2; drift item `prefix_list_empty_fallthrough` (D-972)
- **Description:** No test exercises the `trusted_prefixes: []` case. BC-1.05.035 AC-2 specifies execution MUST be blocked when the prefix list is empty. The drift item registered in D-972 has no test coverage.
- **Evidence:** `grep -r "trusted_prefixes.*\[\]" crates/policy15-gate/tests/` returns no results.
- **Proposed Fix:** Add T-032 with `trusted_prefixes: []` fixture; assert gate returns `Err(PolicyViolation::EmptyPrefixList)` or equivalent.

#### ADV-BB-P04-MED-003: T-015 permission-bit fixture passes vacuously

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-015
- **Description:** T-015 is intended to verify that a world-writable prefix directory is rejected (C-4, CWE-284). The fixture creates a temp dir under `/tmp/` but never sets world-writable permissions (`chmod 0777`). On Linux CI the directory is not world-writable by default, so the "should fail because world-writable" assertion never exercises the actual check path.
- **Evidence:** T-015 calls `TempDir::new()` without any `set_permissions` call. The test title is `test_world_writable_prefix_rejected` but no world-writable condition is created.
- **Proposed Fix:** Add `std::fs::set_permissions(&tmp_dir, std::fs::Permissions::from_mode(0o777))` before the gate call; assert rejection.

#### ADV-BB-P04-MED-004: BC-1.05.035 version citation stale in story spec frontmatter

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `stories/S-21.09-wasm-artifact-restore-and-registry-parity.md` frontmatter `bcs:`
- **Description:** Story spec cites BC-1.05.035 v2.4. As of D-972, BC-1.05.035 is at v2.5. POLICY 8 requires atomic propagation of BC version bumps to referencing stories.
- **Evidence:** `grep "BC-1.05.035" stories/S-21.09-wasm-artifact-restore-and-registry-parity.md` returns `- BC-1.05.035 v2.4`.
- **Proposed Fix:** Update story frontmatter `bcs:` BC-1.05.035 citation to v2.5.

#### ADV-BB-P04-MED-005: artifact-path-registry.yaml path hardcoded in three test files

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** `crates/policy15-gate/tests/` T-021, T-022, T-023
- **Description:** Three tests use the hardcoded relative path `plugins/vsdd-factory/config/artifact-path-registry.yaml`. This path is valid only when tests run from the develop-tree root. An operator-level installation has `config/` at a different absolute path (relative to the plugin cache root). Tests will fail in operator contexts.
- **Evidence:** `grep -n "artifact-path-registry.yaml" crates/policy15-gate/tests/*.rs` shows three occurrences with the identical hardcoded string.
- **Proposed Fix:** Derive the path from `env!("CARGO_MANIFEST_DIR")` or equivalent workspace-relative lookup; exercise the operator-path variant via a separate fixture.

### LOW

#### ADV-BB-P04-LOW-001: Module doc claims "refuses to execute setuid binaries" — gate is inert

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` module-level doc
- **Description:** Module doc states "refuses to execute setuid binaries" but the `refuse_setuid` gate is not implemented (D-971 HIGH SECURITY finding). The doc is a false safety claim.
- **Evidence:** `grep -n "refuse_setuid\|setuid" crates/policy15-gate/src/exec_subprocess.rs` shows the doc comment but no implementation.
- **Proposed Fix:** Remove the "refuses to execute setuid binaries" claim from the module doc, or implement and test the gate.

#### ADV-BB-P04-LOW-002: T-019 assertion uses substring match on raw output

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/registry_parity.rs` T-019
- **Description:** `assert!(output.contains("registry parity check failed"))` matches any output containing that string, including debug dumps or panic messages. The test does not verify the structured log field or exit code.
- **Evidence:** T-019 captures stdout via `Command::output()` and calls `.contains()` only.
- **Proposed Fix:** Assert exit code is non-zero AND that the structured log field `check_name: "registry_parity"` appears in the output.

#### ADV-BB-P04-LOW-003: Test fixtures use inline `toml!{}` macro — no drift detection against production registry

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/` (multiple)
- **Description:** Fixtures construct hooks-registry data inline via macro rather than parsing the actual `hooks-registry.toml`. A drift between the inline fixture and the production registry (e.g., a renamed plugin) would not be caught.
- **Evidence:** `grep -c "toml!" crates/policy15-gate/tests/*.rs` shows 7 occurrences across 4 test files.
- **Proposed Fix:** Add one integration test that loads the actual `hooks-registry.toml` and verifies the gate produces consistent results. Keep unit tests with inline fixtures; add one round-trip test against production config.

#### ADV-BB-P04-LOW-004: T-027 cites "D-971 waiver" — no such waiver exists in decision-log

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-027
- **Description:** Inline comment `// passes per D-971 waiver: W3 wave-gate WAIVED`. D-971 waived the W3 wave-gate for story ordering, not for any test requirement. The attribution is a misidentification that could mislead a future reviewer into treating a gap as intentionally accepted.
- **Evidence:** `decision-log.md` D-971 block contains no test waiver clause.
- **Proposed Fix:** Remove the misattributed waiver comment; if the gap is intentional, attach it to the correct decision or add a TD entry with human direction per Canonical Principle Rule 3.

#### ADV-BB-P04-LOW-005: sprint-state.yaml fixture uses stale `merged_count`

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/sprint_state_tests.rs` T-031
- **Description:** Fixture uses `merged_count: 107` but the test does not assert on `merged_count`, making the fixture inaccurate without being a currently-failing test. This introduces latent fragility.
- **Evidence:** T-031 fixture explicitly sets `merged_count: 107`; assertion only checks `status` field.
- **Proposed Fix:** Either assert on `merged_count` (and update the value) or remove `merged_count` from the fixture to avoid stale-state confusion.

### NIT

#### ADV-BB-P04-NIT-001: `#[cfg(test)]` module uses `use super::*` glob import

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` test module
- **Description:** `use super::*` glob import conflicts with the workspace lint configuration (`unused_imports` + `wildcard_imports` forbidden per CI clippy config). No functional impact.
- **Evidence:** Test module opens with `use super::*;`.
- **Proposed Fix:** Replace with explicit imports of the symbols actually used.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 5 |
| LOW | 5 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

H-001 and H-002 are gated on ADR-043 ratification (a human decision per D-972 §ADR-043-status). The remaining 11 findings (5M + 5L + 1N) are implementer-addressable without awaiting ratification. However BC-5.39.001 3-CLEAN protocol requires zero findings of any severity for a CLEAN pass.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 11 |
| **Duplicate/variant findings** | 2 (H-001 is a restatement of C-1 from D-972; H-002 is a restatement of C-2 from D-972 at test-attestation level) |
| **Novelty score** | 11 / (11 + 2) = 0.85 |
| **Median severity** | 2.5 (LOW/MEDIUM boundary) |
| **Trajectory** | 3→3→2→13 (total finding count per pass) |
| **Verdict** | FINDINGS_REMAIN |
