---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T08:00:00Z
phase: 7
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "b6161ce"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 7
previous_review: adv-s21.09-local-pass-6.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 7)

**Verdict: NOT-CLEAN**
**Finding summary: 0 BLOCKER / 3 HIGH / 3 MEDIUM / 2 LOW / 1 NIT**
**Reviewed commit: `e0cc5480` (feature/S-21.09)**
**LOCAL streak: 0/3 — seven passes, zero CLEAN**
**D-chain: D-972**

**Convergence note:** Pass 7 reviewed story v1.15 (33 tests T-006..T-038 all green at e0cc5480). Five pass-6 findings resolved (HIGH-001 extract_hook_plugin_name boundary T-036/T-037 added; MED-002 T-028 structured ThreatModelAcceptance assertion; LOW-002 T-019 exit-code check added; LOW-003 T-027 misattributed comment removed; NIT-002 T-034 renamed). Mutation testing expanded: 14 mutants applied to the new git-query layer (`git_tracked_wasm_names`); 11 killed by T-006..T-038; 3 survived, all in the git-query layer — surfacing three new HIGH findings. Two MEDs and two NITs from prior passes carry forward. ADR-043 still not ratified.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P07` for this pass
- `<SEQ>`: Three-digit sequence

---

## Part A — Fix Verification

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P06-HIGH-001 | HIGH | RESOLVED | Mutation survivor in `extract_hook_plugin_name` 5-step normalisation closed: v1.15 refactors to four-step normalisation with `lex_norm` module-level sharing; T-036 exercises boundary where registry-parent-relative path requires two normalisation rounds; T-037 exercises path with trailing `.wasm` double-suffix. 33-test suite T-006..T-038 all green at e0cc5480. |
| ADV-BB-P06-MED-001 | MEDIUM | DEFERRED | ADR-043 still not ratified; no implementer action possible. Carries as ADV-BB-P07-MED-001. |
| ADV-BB-P06-MED-002 | MEDIUM | RESOLVED | T-028 replaced with structured assertion on `ThreatModelAcceptance` constant (`cwe_class: CweClass::Toctou`, `accepted_by: "D-972"`). String-presence gate eliminated at e0cc5480. |
| ADV-BB-P06-LOW-001 | LOW | UNRESOLVED | Module doc "refuses to execute setuid binaries" claim unchanged at e0cc5480. Carries as ADV-BB-P07-LOW-001. |
| ADV-BB-P06-LOW-002 | LOW | RESOLVED | T-019 updated: `assert_eq!(output.status.code(), Some(1))` added alongside substring match. Exit-code assertion now present at e0cc5480. |
| ADV-BB-P06-LOW-003 | LOW | RESOLVED | T-027 misattributed D-971 waiver comment removed at e0cc5480. |
| ADV-BB-P06-LOW-004 | LOW | UNRESOLVED | T-031 stale `merged_count: 107` fixture unchanged. Carries as ADV-BB-P07-LOW-002. |
| ADV-BB-P06-NIT-001 | NIT | UNRESOLVED | `use super::*` glob import unchanged. Carries as ADV-BB-P07-NIT-001. |
| ADV-BB-P06-NIT-002 | NIT | RESOLVED | T-034 renamed to `test_full_suite_declared_tracked_parity_ok` at e0cc5480; naming convention now satisfied. |

---

## Part B — New Findings

### HIGH

#### ADV-BB-P07-HIGH-001: `git_tracked_wasm_names` provenance — git-index vs committed-tree not distinguished (mutation survivor)

- **Severity:** HIGH
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `git_tracked_wasm_names` helper
- **Description:** `git_tracked_wasm_names` calls `git ls-files -- '*.wasm'`. `git ls-files` without `--cached --exclude-standard` will include files staged in the index but not yet committed. A WASM artifact that has been `git add`-ed but not committed will satisfy the declared-→-tracked gate even though it is not part of any committed revision. Mutation testing survivor: mutant that deleted the `add -f` call in Half 1 fixture setup still produced a passing T-012 because the index state inherited from a prior test run made the file appear tracked.
- **Evidence:** `git ls-files --error-unmatch` resolves against the index, not HEAD. Under `git ls-files -- '*.wasm'` without `--cached`, staged-but-uncommitted artifacts satisfy the query.
- **Proposed Fix:** Replace `git ls-files -- '*.wasm'` with `git ls-tree --name-only HEAD -- '*.wasm'` (committed-tree only) or add `--cached` and pair with explicit `git rev-parse --verify HEAD:path` for each result to confirm HEAD ancestry.

#### ADV-BB-P07-HIGH-002: committed-vs-index distinction unverified in `run_t012_gate` (mutation survivor)

- **Severity:** HIGH
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `run_t012_gate`
- **Description:** `run_t012_gate` invokes the orphan check against a fixture repository. The fixture uses `git add -f` to track the WASM artifact, which places it in the index. Mutant that replaced `git add -f` with a no-op still killed 11 other tests but survivor-mutant on T-012 confirmed the gate passes on index presence alone. The gate SHOULD require committed presence (HEAD-reachable), not index presence, to match the production invariant that only shipped binaries are in committed history.
- **Evidence:** `git show e0cc5480:crates/factory-dispatcher/tests/bundle_orphan_check.rs` — `run_t012_gate` fixture setup: `Command::new("git").args(["add", "-f", "validate-factory-path-staging.wasm"])`. No subsequent `git commit` call; gate resolves on index state only.
- **Proposed Fix:** Add `git commit -m "fixture: track wasm artifact" --allow-empty-message` after `git add -f` in the T-012 fixture, and update the gate implementation to query `git ls-tree HEAD` rather than `git ls-files`.

#### ADV-BB-P07-HIGH-003: BOUNDARY-POLARITY inversion in `detect_ungated_declarations` step 2c (confirmed-unsafe, mutation survivor)

- **Severity:** HIGH
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` step 2c gate arm, `detect_ungated_declarations`
- **Description:** The step 2c gate arm in `detect_ungated_declarations` applies a polarity inversion: the `is_excluded` boolean is negated before being used to filter declarations. In the intended logic, `is_excluded = true` means the declaration is in an exclusion zone and should NOT be flagged. After the inversion, `!is_excluded` is passed to the filter, meaning declarations IN the exclusion zone ARE flagged and declarations outside are NOT — the opposite of the intended behavior. This is a BOUNDARY-POLARITY inversion: the boundary between "excluded" and "ungated" is crossed in the wrong direction. Mutation survivor: the mutant that toggled the `!` prefix survived T-006..T-038 because no test exercises a declaration that is simultaneously in-scope for the exclusion gate and expected to be reported as ungated. The current test suite only verifies the true-positive path (ungated declaration reported) and the true-negative path (clean registry clean), but not the false-negative path (excluded declaration that would be incorrectly suppressed).
- **Evidence:** `git show e0cc5480:crates/factory-dispatcher/tests/bundle_orphan_check.rs` — step 2c closure: `declarations.into_iter().filter(|d| !is_excluded(d, &exclusion_zones)).collect()`. The intended filter should be `filter(|d| is_ungated(d) && !is_excluded(d, ...))` — but the inversion means excluded-true items are retained and ungated-true items not in exclusion zones are dropped. Confirmed-unsafe: a real hooks-registry.toml with an exclusion zone and an ungated declaration in that zone would silently pass the gate.
- **Proposed Fix:** Remove the polarity inversion; add T-039: fixture with a declaration in an exclusion zone asserting it is NOT reported; add T-040: fixture with a declaration outside all exclusion zones asserting it IS reported.

### MEDIUM

#### ADV-BB-P07-MED-001: T-012 fixture resolves `allowed_binaries` via pre-resolved absolute paths — ADR-043-gated (carry from P06-MED-001)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-012 fixture
- **Description:** Unchanged from ADV-BB-P06-MED-001. ADR-043 still not ratified. No implementer action possible.
- **Proposed Fix:** Pending ADR-043 ratification: add T-012b (bare name resolved via `trusted_prefixes`) and T-012c (bare name not under any prefix; assert failure).

#### ADV-BB-P07-MED-002: `lex_norm` module lacks boundary test for consecutive-separator collapse (new)

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `lex_norm` module
- **Description:** The new `lex_norm` module-level function is shared between `extract_hook_plugin_name` and `detect_ungated_declarations`. The function normalises path separators and collapses consecutive slashes. No test exercises the consecutive-separator input: `"plugins//vsdd-factory//hook-plugins/foo.wasm"`. Under the current implementation, double slashes are collapsed, but a mutant that removed the collapse step survived because no test asserts on this boundary condition. The `lex_norm` shared surface makes this boundary important: a misconfigured hooks-registry.toml with double-slash paths could bypass the parity check.
- **Evidence:** `git show e0cc5480:crates/factory-dispatcher/tests/bundle_orphan_check.rs` — `lex_norm` test coverage: T-036 exercises single trailing `.wasm` and two-round normalisation; T-037 exercises double-suffix. No test for `//`-separated path input.
- **Proposed Fix:** Add T-041: `lex_norm("plugins//vsdd-factory//hook-plugins/foo.wasm")` asserts equal to `lex_norm("plugins/vsdd-factory/hook-plugins/foo.wasm")`.

#### ADV-BB-P07-MED-003: `detect_ungated_declarations` three defensive unreachable exits — no test reaches any (new)

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `detect_ungated_declarations` three defensive `unreachable!()` exits
- **Description:** Story v1.15 adds "three defensive unreachable ? exits" in `detect_ungated_declarations`. These exits are never exercised by T-006..T-038. An unreachable arm that is reachable in production but declared unreachable in tests is a diagnostic blind spot: if the exit fires in production it will panic rather than propagate a structured error.
- **Evidence:** `git show e0cc5480:crates/factory-dispatcher/tests/bundle_orphan_check.rs` — 33 tests T-006..T-038; grep for `unreachable` in `detect_ungated_declarations` returns 3 hits; grep in test file for any fixture expected to trigger those arms returns 0 hits.
- **Proposed Fix:** Either convert the `unreachable!()` arms to `Err(DetectionError::InternalInvariantViolation(...))` and add tests that trigger them via crafted malformed registry input, or document why each arm is structurally unreachable with a proof sketch in a `// SAFETY:` comment.

### LOW

#### ADV-BB-P07-LOW-001: Module doc claims "refuses to execute setuid binaries" — gate is inert (carry from P06-LOW-001)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` module-level doc
- **Description:** Unchanged. Module doc states "refuses to execute setuid binaries" but the `refuse_setuid` gate never fires in production. D-971 records this as a HIGH SECURITY drift item.
- **Proposed Fix:** Remove the claim or implement proper path-resolve-then-stat logic.

#### ADV-BB-P07-LOW-002: T-031 sprint-state fixture uses stale `merged_count: 107` (carry from P06-LOW-004)

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/sprint_state_tests.rs` T-031
- **Description:** Unchanged. Fixture `merged_count: 107` not asserted upon; stale latent fragility.
- **Proposed Fix:** Assert on `merged_count` or remove from fixture.

### NIT

#### ADV-BB-P07-NIT-001: `use super::*` glob import in test module (carry from P06-NIT-001)

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` test module
- **Description:** Unchanged. `use super::*` conflicts with workspace `wildcard_imports` lint.
- **Proposed Fix:** Replace with explicit imports.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 3 |
| MEDIUM | 3 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** spec-vs-reality drift: **zero**; pre-existing-code defects: **zero**; remaining: git-query layer correctness gaps (3 HIGH mutation survivors) + carry-over verification gaps
**Readiness:** requires revision

ADV-BB-P07-HIGH-001 and HIGH-002 expose a committed-vs-index distinction gap in the git-query layer that allows staged-but-uncommitted WASM artifacts to satisfy the parity gate. ADV-BB-P07-HIGH-003 is a BOUNDARY-POLARITY inversion in `detect_ungated_declarations` step 2c — confirmed-unsafe: a real exclusion zone with an ungated declaration inside it would silently pass the gate. BC-5.39.001 3-CLEAN protocol requires zero findings of any severity for a CLEAN pass.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 7 |
| **Story version reviewed** | v1.15 |
| **Reviewed commit** | e0cc5480 |
| **New findings** | 5 (HIGH-001/002/003; MED-002/003) |
| **Carry-over findings** | 4 (MED-001; LOW-001/002; NIT-001) |
| **Resolved this pass** | 5 (P06-HIGH-001; P06-MED-002; P06-LOW-002; P06-LOW-003; P06-NIT-002) |
| **Mutation testing** | 14 mutants applied to git-query layer; 11 killed; 3 survived |
| **Novelty score** | 5 / (5 + 4) = 0.56 |
| **Median severity** | MEDIUM |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3 |
| **Total finding trajectory** | →11→9→9 (pass-5 total 11; pass-6 total 9; pass-7 total 9) |
| **Verdict** | FINDINGS_REMAIN |
