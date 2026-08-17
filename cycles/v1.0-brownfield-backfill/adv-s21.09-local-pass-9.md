---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T12:00:00Z
phase: 9
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 9
previous_review: adv-s21.09-local-pass-8.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 9)

**Verdict: NOT-CLEAN**
**Finding summary: 1 BLOCKER / 1 HIGH / 3 MEDIUM / 2 LOW / 1 NIT**
**Reviewed commit: `b951461a` (feature/S-21.09)**
**LOCAL streak: 0/3 — nine passes, zero CLEAN**
**D-chain: D-972**

**Convergence note:** Pass 9 reviewed story v1.19 (42 tests T-006..T-047 all green at b951461a). ADV-BB-P08-HIGH-002 (prefix-strip normalisation gap) is resolved: `lex_norm` now applied before `strip_prefix` in all arms; T-043..T-047 added covering leading-`./` and worktree-root boundary forms. A new BLOCKER was discovered and confirmed CLOSED in this pass: gate-1-failing declaration forms (bare name and `../`-prefix) escaped both the declared set and `detect_ungated_declarations`, allowing a declared-but-absent artifact to satisfy the gate with `Ok(())` for both probes; the fix replaces length-based filtering with a worktree-root containment predicate yielding a total three-class invariant (gated / `UNGATED-DECLARATION` / `OUTSIDE-REPO-DECLARATION`, no silent drops). One remaining HIGH (T-042 cfg macOS skip), three MEDIUM, two LOW, and one NIT carry forward.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P09` for this pass
- `<SEQ>`: Three-digit sequence

---

## Part A — Fix Verification

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P08-HIGH-002 | HIGH | RESOLVED | `lex_norm` now applied before `strip_prefix` in all arms of `run_t012_gate`; T-043 asserts that a leading-`./`-prefixed registry path strips correctly. Mutation retried at b951461a: no survivor. |
| ADV-BB-P08-HIGH-003 | HIGH | UNRESOLVED | `#[cfg(target_os = "linux")]` guard on T-042 unchanged at b951461a. Carries as ADV-BB-P09-HIGH-001. |
| ADV-BB-P08-MED-001 | MEDIUM | DEFERRED | ADR-043 still not ratified; no implementer action possible. Carries as ADV-BB-P09-MED-001. |
| ADV-BB-P08-MED-002 | MEDIUM | UNRESOLVED | `lex_norm` Windows-backslash normalisation not added at b951461a. Carries as ADV-BB-P09-MED-002. |
| ADV-BB-P08-MED-003 | MEDIUM | UNRESOLVED | T-039 still asserts `result.is_ok()` without length check at b951461a. Carries as ADV-BB-P09-MED-003. |
| ADV-BB-P08-LOW-001 | LOW | UNRESOLVED | `refuse_setuid` module doc claim unchanged at b951461a. Carries as ADV-BB-P09-LOW-001. |
| ADV-BB-P08-LOW-002 | LOW | UNRESOLVED | T-031 stale `merged_count: 107` fixture unchanged. Carries as ADV-BB-P09-LOW-002. |
| ADV-BB-P08-NIT-001 | NIT | UNRESOLVED | `ThreatModelAcceptance` constant still lacks rustdoc at b951461a. Carries as ADV-BB-P09-NIT-001. |

---

## BLOCKER Finding (CLOSED in this pass)

### ADV-BB-P09-BLK-001 (CLOSED): gate-1-failing declaration forms escape declared set and `detect_ungated_declarations` — declared-but-absent artifact returns `Ok(())` for both probes

- **Severity:** BLOCKER
- **Category:** correctness
- **Status:** CLOSED — worktree-root containment predicate replacing length-based filtering; total predicate invariant established
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` step-2c `detect_ungated_declarations`, gate-1 declared-set construction
- **Description:** Certain declaration forms — specifically bare plugin names without a path component (e.g., `foo.wasm`) and paths carrying a `../`-prefix (e.g., `../hook-plugins/foo.wasm`) — satisfied neither the declared set membership check nor the `detect_ungated_declarations` gate-2c filter. Both probes returned `Ok(())` even when the declared artifact was entirely absent from the tracked set. The root defect was a length-based filtering heuristic: declarations whose path component count fell below the expected registry-relative depth were silently dropped rather than classified as ungated. This is the story's own defect class — `validate-factory-path-staging` was introduced to prevent silent artifact absence, yet the gate itself silently dropped the same artifact forms it was designed to catch.

  An initial fix replaced the length check with a worktree-root boundary predicate (requiring the resolved path to remain within the worktree root). A subsequent orchestrator probe revealed the first fix had only moved the boundary one level shallower: paths of the form `../../hook-plugins/foo.wasm` resolved to a path one directory above the worktree root and were still silently dropped. The final fix applies a strict containment check (`starts_with(worktree_root)`) and routes all non-conforming forms to one of two explicit identifiers — `UNGATED-DECLARATION` (path resolves outside declared prefix) or `OUTSIDE-REPO-DECLARATION` (path escapes worktree root entirely) — with no silent drop path remaining. The total predicate invariant is: every declaration is either (1) gated (present in declared set and tracked), (2) `UNGATED-DECLARATION`, or (3) `OUTSIDE-REPO-DECLARATION`; the `Ok(())` path is only reachable when the full declared set is accounted for.

- **Closure evidence:** `git diff HEAD~1..b951461a crates/factory-dispatcher/tests/bundle_orphan_check.rs` shows step-2c filter replaced with containment predicate; T-043..T-047 cover bare-name, `../`-prefix (single), `../../`-prefix (double), and OUTSIDE-REPO boundary forms; both `UNGATED-DECLARATION` and `OUTSIDE-REPO-DECLARATION` identifiers are exercised with mutation probes showing no silent-drop survivor. Two orchestrator probe iterations documented at b951461a commit message.

---

## Part B — New Findings (Remaining Open)

### HIGH

#### ADV-BB-P09-HIGH-001: T-042 case-variant end-to-end uses `cfg(target_os = "linux")` guard — macOS dev silently skips the test (carry from P08-HIGH-003)

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-042
- **Description:** Unchanged from ADV-BB-P08-HIGH-003. `#[cfg(target_os = "linux")]` silently omits T-042 on macOS rather than surfacing it as `#[ignore]`. A developer claiming `cargo test` is green on macOS has not verified the case-variant path at all.
- **Proposed Fix:** Replace `#[cfg(target_os = "linux")]` with `#[cfg_attr(not(target_os = "linux"), ignore = "case-variant test requires case-sensitive Linux filesystem")]`.

### MEDIUM

#### ADV-BB-P09-MED-001: T-012 fixture resolves `allowed_binaries` via pre-resolved absolute paths — ADR-043-gated (carry from P08-MED-001)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/tests/path_staging.rs` T-012 fixture
- **Description:** Unchanged from ADV-BB-P08-MED-001. ADR-043 still not ratified; no implementer action possible.
- **Proposed Fix:** Pending ADR-043 ratification: add T-012b (bare name resolved via `trusted_prefixes`) and T-012c (bare name not under any prefix; assert failure).

#### ADV-BB-P09-MED-002: `lex_norm` does not normalise Windows-style backslash separators (carry from P08-MED-002)

- **Severity:** MEDIUM
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `lex_norm` module
- **Description:** Unchanged from ADV-BB-P08-MED-002. `lex_norm` does not convert `\` to `/`; Windows-backslash paths bypass the full-path set-difference comparison.
- **Proposed Fix:** Add `s.replace('\\', '/')` as the first normalisation step in `lex_norm`.

#### ADV-BB-P09-MED-003: EC-008 control T-039 asserts `result.is_ok()` without checking result length — partial verification (carry from P08-MED-003)

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-039
- **Description:** Unchanged from ADV-BB-P08-MED-003. T-039 asserts `result.is_ok()` but does not assert the returned `Vec` is empty; a bug returning `Ok(vec![exclusion_zone_declaration])` passes undetected.
- **Proposed Fix:** Replace `assert!(result.is_ok())` with `assert_eq!(result.unwrap(), Vec::<Declaration>::new())` or equivalent.

### LOW

#### ADV-BB-P09-LOW-001: Module doc claims "refuses to execute setuid binaries" — gate is inert (carry from P08-LOW-001)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` module-level doc
- **Description:** Unchanged. Module doc states "refuses to execute setuid binaries" but the `refuse_setuid` gate never fires in production. D-971 records this as a HIGH SECURITY drift item.
- **Proposed Fix:** Remove the claim or implement proper path-resolve-then-stat logic.

#### ADV-BB-P09-LOW-002: T-031 sprint-state fixture uses stale `merged_count: 107` (carry from P08-LOW-002)

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/policy15-gate/tests/sprint_state_tests.rs` T-031
- **Description:** Unchanged. Fixture `merged_count: 107` not asserted upon; stale latent fragility.
- **Proposed Fix:** Assert on `merged_count` or remove from fixture.

### NIT

#### ADV-BB-P09-NIT-001: `ThreatModelAcceptance` constant lacks rustdoc (carry from P08-NIT-001)

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/policy15-gate/src/exec_subprocess.rs` `ThreatModelAcceptance` constant
- **Description:** Unchanged. The `ThreatModelAcceptance` constant carries security-relevant metadata with no rustdoc comment.
- **Proposed Fix:** Add `/// Threat model acceptance record for the Toctou-class risk accepted under D-972.` before the constant.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 1 (CLOSED this pass) |
| HIGH | 1 |
| MEDIUM | 3 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** spec-vs-reality drift: **zero**; pre-existing-code defects: **zero**; remaining: case-variant cfg guard (HIGH) + carry-over items
**Readiness:** requires revision

The BLOCKER (ADV-BB-P09-BLK-001) — gate-1-failing declaration forms (bare name, `../`-prefix) escaping both the declared set and `detect_ungated_declarations` — was discovered and confirmed CLOSED at b951461a via worktree-root containment predicate with explicit `UNGATED-DECLARATION` / `OUTSIDE-REPO-DECLARATION` routing and no silent-drop path. The two-iteration fix history (length-based filter → shallow containment → strict `starts_with(worktree_root)` containment) is documented in the closure evidence. BC-5.39.001 3-CLEAN protocol requires zero findings of any severity for a CLEAN pass; streak remains 0/3 after nine passes.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 9 |
| **Story version reviewed** | v1.19 |
| **Reviewed commit** | b951461a |
| **New findings (closed this pass)** | 1 (BLOCKER — gate-1 declaration-form escape — CLOSED) |
| **New findings (open)** | 0 |
| **Carry-over findings** | 7 (HIGH-001; MED-001/002/003; LOW-001/002; NIT-001) |
| **Resolved this pass** | 1 (P08-HIGH-002 prefix-strip normalisation) |
| **Mutation testing** | T-043..T-047 cover five new declaration-form categories; zero survivors on containment-predicate mutations |
| **Novelty score** | 1 / (1 + 7) = 0.13 |
| **Median severity** | MEDIUM |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3→2→1 |
| **Total finding trajectory** | →9→9→8→8 (pass-6 total 9; pass-7 total 9; pass-8 total 8; pass-9 total 8) |
| **Verdict** | FINDINGS_REMAIN |
