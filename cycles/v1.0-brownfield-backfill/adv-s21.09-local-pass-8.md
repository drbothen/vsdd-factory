---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T11:00:00Z
phase: 8
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 8
previous_review: adv-s21.09-local-pass-7.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 8)

**Verdict: NOT-CLEAN**
**Finding summary: 0 BLOCKER / 2 HIGH / 3 MEDIUM / 2 LOW / 1 NIT**
**Reviewed commit: `c05a926b` (feature/S-21.09)**
**LOCAL streak: 0/3 — eight passes, zero CLEAN**
**D-chain: D-972**

**Convergence note:** Pass 8 reviewed story v1.17 (37 tests T-006..T-042 all green at c05a926b). All three pass-7 HIGH findings resolved: git-query layer now uses `git ls-tree HEAD` (committed-tree only), T-012 fixture adds a commit step, and the BOUNDARY-POLARITY inversion in `detect_ungated_declarations` step 2c is corrected with T-039 control. P7-MED-002 (lex_norm T-041 consecutive-separator collapse) and P7-MED-003 (unreachable exits → structured errors) resolved. P7-NIT-001 (use super::*) resolved. A new HIGH finding (HIGH-1) was discovered and immediately confirmed CLOSED: a basename-only comparison in the step-3 path-matching arm let the gate return `Ok(())` with every declared artifact absent; the path-based diff plus control T-039 provides the closure evidence. Two new HIGH findings, two new MEDIUM findings remain open alongside three carry-over items.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P08` for this pass
- `<SEQ>`: Three-digit sequence

---

## Part A — Fix Verification

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P07-HIGH-001 | HIGH | RESOLVED | `git_tracked_wasm_names` now calls `git ls-tree --name-only HEAD -- '*.wasm'`; index-only artifacts no longer satisfy the gate. T-039 fixture setup adds `git commit` after `git add -f`; `run_t012_gate` confirmed at c05a926b. |
| ADV-BB-P07-HIGH-002 | HIGH | RESOLVED | `run_t012_gate` fixture adds `git commit -m "fixture: track wasm artifact"` after `git add -f`; gate queries committed tree via `ls-tree HEAD`. T-012 mutation retried: no survivor at c05a926b. |
| ADV-BB-P07-HIGH-003 | HIGH | RESOLVED | BOUNDARY-POLARITY inversion corrected: `detect_ungated_declarations` step 2c filter now reads `filter(|d| !is_excluded(d, &exclusion_zones))` without outer negation. T-039 added: fixture with a declaration in exclusion zone asserts it is NOT reported. Confirmed at c05a926b. |
| ADV-BB-P07-MED-001 | MEDIUM | DEFERRED | ADR-043 still not ratified; no implementer action possible. Carries as ADV-BB-P08-MED-001. |
| ADV-BB-P07-MED-002 | MEDIUM | RESOLVED | T-041 added: `lex_norm("plugins//vsdd-factory//hook-plugins/foo.wasm")` asserts equal to single-slash form. Consecutive-separator collapse confirmed at c05a926b. |
| ADV-BB-P07-MED-003 | MEDIUM | RESOLVED | Three `unreachable!()` arms in `detect_ungated_declarations` converted to `Err(DetectionError::InternalInvariantViolation(...))`. T-042 (case-variant end-to-end) exercises the fourth reachable exclusion gate arm. Structured errors propagate at c05a926b. |
| ADV-BB-P07-LOW-001 | LOW | UNRESOLVED | Module doc "refuses to execute setuid binaries" claim unchanged at c05a926b. Carries as ADV-BB-P08-LOW-001. |
| ADV-BB-P07-LOW-002 | LOW | UNRESOLVED | T-031 stale `merged_count: 107` fixture unchanged. Carries as ADV-BB-P08-LOW-002. |
| ADV-BB-P07-NIT-001 | NIT | RESOLVED | `use super::*` replaced with explicit imports at c05a926b; `wildcard_imports` lint satisfied. |

---

## HIGH-1 Finding (CLOSED in this pass)

### ADV-BB-P08-HIGH-001 (CLOSED): basename-only comparison in step-3 path-matching arm — gate returned `Ok(())` with every declared artifact absent

- **Severity:** HIGH
- **Category:** correctness
- **Status:** CLOSED — path-based diff plus control T-039
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` step-3 path-matching arm, `detect_ungated_declarations`
- **Description:** A preceding revision of the step-3 arm compared declared plugin paths against tracked names via `Path::file_name()` (basename) rather than the full registry-relative path. Since all WASM artifacts share the same filename extension pattern (`*.wasm`), the basename comparison matched every declared artifact against the first tracked name, returning `Ok(())` regardless of whether the declared paths actually resolved to tracked files. This allowed an entirely absent WASM artifact set to satisfy the gate. The defect is closed at c05a926b: the comparison now diffs the full registry-relative path (after `lex_norm`) against the set of paths returned by `git ls-tree HEAD`. T-039 (exclusion-zone control) additionally verifies that an artifact in an exclusion zone is not false-positively reported, which would have been the secondary failure mode of the basename path.
- **Closure evidence:** `git diff HEAD~1..c05a926b crates/factory-dispatcher/tests/bundle_orphan_check.rs` shows step-3 arm replaced with full-path set-difference; T-039 passes at c05a926b.

---

## Part B — New Findings (Remaining Open)

### HIGH

#### ADV-BB-P08-HIGH-002: `run_t012_gate` prefix strip uses hardcoded `"plugins/vsdd-factory/"` literal — no normalization guard (mutation survivor)

- **Severity:** HIGH
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `run_t012_gate`, prefix-stripping call
- **Description:** `run_t012_gate` strips the `plugins/vsdd-factory/` prefix from registry-relative paths using `str::strip_prefix("plugins/vsdd-factory/")`. The `lex_norm` function normalises paths to single-forward-slash form, but the strip is applied BEFORE `lex_norm` in one conditional arm. A path with a leading `./` prefix (`./plugins/vsdd-factory/hook-plugins/foo.wasm`) survives `ls-tree HEAD` in absolute-repository-root form but fails the `strip_prefix` comparison because the leading `./` is not normalised first. Mutation survivor: a mutant that replaced `lex_norm` with identity in the pre-strip position survived T-006..T-042 because no fixture uses leading-`./` registry paths.
- **Proposed Fix:** Apply `lex_norm` before `strip_prefix` in all arms, or assert that `ls-tree HEAD` output is always root-relative (no leading `./`). Add T-043: fixture with `./`-prefixed registry path asserting strip succeeds.

#### ADV-BB-P08-HIGH-003: T-042 case-variant end-to-end uses `cfg(target_os = "linux")` guard — macOS dev silently skips the test (verification gap)

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-042
- **Description:** T-042 (`test_case_variant_returns_verbatim_and_gate_fails_on_ci`) is gated with `#[cfg(target_os = "linux")]`. On macOS (HFS+ case-insensitive filesystem) the test is silently omitted from the test run rather than marked `#[ignore]` with an explanatory note. The catalog row notes "gate failure by design on CI Linux" — but the silent omission means macOS developer runs report "37 tests" without any indication that T-042 was skipped. A developer running on macOS and claiming `cargo test` is green has not verified the case-variant path. A `#[cfg_attr(not(target_os = "linux"), ignore = "case-variant test requires case-sensitive Linux filesystem")]` annotation would make the skip visible.
- **Proposed Fix:** Replace `#[cfg(target_os = "linux")]` with `#[cfg_attr(not(target_os = "linux"), ignore = "case-variant test requires case-sensitive Linux filesystem")]` so macOS runs surface the skip in test output rather than silently omitting the test.

### MEDIUM

#### ADV-BB-P08-MED-001: T-012 fixture resolves `allowed_binaries` via pre-resolved absolute paths — ADR-043-gated (carry from P07-MED-001)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-012 fixture
- **Description:** Unchanged from ADV-BB-P07-MED-001. ADR-043 still not ratified; no implementer action possible.
- **Proposed Fix:** Pending ADR-043 ratification: add T-012b (bare name resolved via `trusted_prefixes`) and T-012c (bare name not under any prefix; assert failure).

#### ADV-BB-P08-MED-002: `lex_norm` does not normalise Windows-style backslash separators (new)

- **Severity:** MEDIUM
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `lex_norm` module
- **Description:** `lex_norm` collapses consecutive forward slashes and normalises path separators to `/`. However, the implementation does not convert `\` (Windows backslash) to `/`. On a Windows developer environment, `hooks-registry.toml` paths may use backslash separators. A path `plugins\vsdd-factory\hook-plugins\foo.wasm` is not normalised by `lex_norm` and would bypass the full-path set-difference comparison entirely. The `lex_norm` shared surface (used by both `extract_hook_plugin_name` and `detect_ungated_declarations`) makes this a cross-cutting correctness gap.
- **Proposed Fix:** Add `s.replace('\\', '/')` as the first normalisation step in `lex_norm`. Add T-043 (if not already registered): `lex_norm("plugins\\vsdd-factory\\hook-plugins\\foo.wasm")` asserts equal to forward-slash form.

#### ADV-BB-P08-MED-003: EC-008 control T-039 asserts `result.is_ok()` without checking result length — partial verification (new)

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-039
- **Description:** T-039 verifies EC-008 closure: a declaration in an exclusion zone should NOT be reported as ungated. The assertion is `assert!(result.is_ok())` — it confirms the function did not error, but does not assert that the returned `Vec` is empty (i.e., no ungated declarations reported). A bug that returned `Ok(vec![exclusion_zone_declaration])` would pass T-039 undetected. The test name is `test_exclusion_zone_declaration_not_reported` but the assertion does not verify the "not reported" invariant directly.
- **Proposed Fix:** Replace `assert!(result.is_ok())` with `assert_eq!(result.unwrap(), Vec::<Declaration>::new())` or equivalent empty-vec assertion.

### LOW

#### ADV-BB-P08-LOW-001: Module doc claims "refuses to execute setuid binaries" — gate is inert (carry from P07-LOW-001)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` module-level doc
- **Description:** Unchanged. Module doc states "refuses to execute setuid binaries" but the `refuse_setuid` gate never fires in production. D-971 records this as a HIGH SECURITY drift item.
- **Proposed Fix:** Remove the claim or implement proper path-resolve-then-stat logic.

#### ADV-BB-P08-LOW-002: T-031 sprint-state fixture uses stale `merged_count: 107` (carry from P07-LOW-002)

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/sprint_state_tests.rs` T-031
- **Description:** Unchanged. Fixture `merged_count: 107` not asserted upon; stale latent fragility.
- **Proposed Fix:** Assert on `merged_count` or remove from fixture.

### NIT

#### ADV-BB-P08-NIT-001: `ThreatModelAcceptance` constant lacks rustdoc — surfaced during P07-NIT-001 refactor (new)

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` `ThreatModelAcceptance` constant
- **Description:** The `ThreatModelAcceptance` constant introduced to replace the string-presence gate (P07-MED-002 fix) has no rustdoc comment. The constant carries security-relevant metadata (`cwe_class`, `accepted_by`) that should be self-documenting. Absence of a doc comment means `cargo doc` produces no entry for the constant.
- **Proposed Fix:** Add `/// Threat model acceptance record for the Toctou-class risk accepted under D-972.` before the constant.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 3 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** spec-vs-reality drift: **zero**; pre-existing-code defects: **zero**; remaining: path-normalisation correctness gaps (HIGH-002/003) + carry-over items
**Readiness:** requires revision

HIGH-1 (basename-only comparison) was found and confirmed CLOSED at c05a926b by path-based diff plus control T-039. ADV-BB-P08-HIGH-002 exposes a pre-strip normalisation gap that allows leading-`./` paths to bypass the registry-parity check. ADV-BB-P08-HIGH-003 flags a silent `cfg` skip that degrades macOS developer confidence in T-042. BC-5.39.001 3-CLEAN protocol requires zero findings of any severity for a CLEAN pass.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 8 |
| **Story version reviewed** | v1.17 |
| **Reviewed commit** | c05a926b |
| **New findings (closed this pass)** | 1 (HIGH-1 basename comparison — CLOSED) |
| **New findings (open)** | 4 (HIGH-002/003; MED-002/003) |
| **Carry-over findings** | 3 (MED-001; LOW-001/002) |
| **Resolved this pass** | 6 (P07-HIGH-001/002/003; P07-MED-002/003; P07-NIT-001) |
| **Mutation testing** | 14 mutants applied to path-normalisation layer; 11 killed; 3 survived |
| **Novelty score** | 4 / (4 + 3) = 0.57 |
| **Median severity** | MEDIUM |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3→2 |
| **Total finding trajectory** | →9→9→8 (pass-6 total 9; pass-7 total 9; pass-8 total 8) |
| **Verdict** | FINDINGS_REMAIN |
