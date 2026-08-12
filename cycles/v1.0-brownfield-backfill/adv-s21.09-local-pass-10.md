---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T13:30:00Z
phase: 10
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "0fb0e79"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 10
previous_review: adv-s21.09-local-pass-9.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 10)

**Verdict: NOT-CLEAN**
**Finding summary: 2 BLOCKER / 3 HIGH / 6 MEDIUM / 3 LOW / 1 NIT**
**Reviewed commit: `b951461a` (feature/S-21.09)**
**LOCAL streak: 0/3 — ten passes, zero CLEAN**
**D-chain: D-972**

**Convergence note:** Pass 10 reviewed story v1.19 (42 tests T-006..T-047 all green at b951461a). All eight pass-9 carry-over items verified: P09-BLK-001 confirmed closed; P09-MED-002/003 resolved in fix wave; P09-LOW-001 reclassified as out-of-scope (tracked as HIGH SECURITY drift under D-971); P09-LOW-002 resolved; P09-NIT-001 resolved. P09-MED-001 (ADR-043-gated) deferred — ADR-043 still not ratified, removed from active tracking. P09-HIGH-001 (T-042 cfg guard) carries forward unresolved. Two new BLOCKERs discovered: (1) `run_t012_gate` is invoked at two independent call sites that can observe divergent I/O snapshots — fix requires collapsing into a single call (T-048); (2) proptest covers fewer than eighteen distinct path-prefix candidates, leaving adversarial boundary forms untested (fix: expand to eighteen candidates, T-049). Both BLOCKERs are confirmed CLOSED in `1c59a669`. Five findings remain open after the full fix wave: two MEDIUM (directory-only staging control; prefix-conjunct isolating control) and three LOW (NUL/trailing-space names; fail-open arms with unasserted call-ordering; `workspace_root()` untested directly).

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P10` for this pass
- `<SEQ>`: Three-digit sequence

---

## Part A — Fix Verification

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P09-BLK-001 | BLOCKER | VERIFIED CLOSED | Worktree-root containment predicate confirmed in place at b951461a; both `UNGATED-DECLARATION` and `OUTSIDE-REPO-DECLARATION` routes exercised; no silent-drop path reachable. |
| ADV-BB-P09-HIGH-001 | HIGH | UNRESOLVED | `#[cfg(target_os = "linux")]` guard on T-042 unchanged at b951461a. Carries as ADV-BB-P10-HIGH-001. |
| ADV-BB-P09-MED-001 | MEDIUM | DEFERRED | ADR-043 still not ratified; no implementer action possible. Removed from active pass-10 finding set; remains a permanent external blocker until ADR-043 is ratified. |
| ADV-BB-P09-MED-002 | MEDIUM | RESOLVED | `lex_norm` updated to convert `\` to `/` as the first normalisation step; T-040 asserts that a Windows-backslash registry path normalises to the expected slash-separated form before comparison. Mutation retried: no survivor. |
| ADV-BB-P09-MED-003 | MEDIUM | RESOLVED | T-039 now asserts `assert_eq!(result.unwrap(), Vec::<Declaration>::new())` replacing the weaker `result.is_ok()` assertion; a bug returning a non-empty Vec would now be caught. |
| ADV-BB-P09-LOW-001 | LOW | RECLASSIFIED | `refuse_setuid` module doc claim reclassified as out-of-scope for this story; tracked as HIGH SECURITY drift item under D-971. No story-scope action available without separate security story. |
| ADV-BB-P09-LOW-002 | LOW | RESOLVED | T-031 fixture `merged_count: 107` assertion added; stale latent fragility closed. |
| ADV-BB-P09-NIT-001 | NIT | RESOLVED | `ThreatModelAcceptance` constant now carries rustdoc: `/// Threat model acceptance record for the TOCTOU-class risk accepted under D-972.` |

---

## BLOCKER Findings (BOTH CLOSED in `1c59a669`)

### ADV-BB-P10-BLK-001 (CLOSED): `run_t012_gate` invoked at two independent call sites — divergent I/O snapshots possible; gate result inconsistency on concurrent staging writes

- **Severity:** BLOCKER
- **Category:** correctness
- **Status:** CLOSED — collapsed into single `run_t012_gate` call shared by both checks; T-048 covers single-call identity invariant
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` staged-artifact consistency check and registry-parity check call sites
- **Description:** `run_t012_gate` was invoked independently from two distinct call sites in `bundle_orphan_check.rs`: once from the staged-artifact consistency check and once from the registry-parity check. Both calls operated on the same logical input tuple (plugin name, artifact path, staging snapshot), but because they issued separate I/O queries, a concurrent `git add` or WASM plugin update occurring between the two invocations could cause one call to observe `Gated` while the other observed `Ungated`. The downstream merge of the two results treated any `Ungated` result as a gate failure, but had no mechanism to distinguish a genuine ungated artifact from a race-induced transient — the gate could fire spuriously on a valid staged artifact or miss a genuine ungated one depending on I/O ordering. Beyond the correctness problem, the duplicate evaluation doubled the I/O cost of every gate invocation, violating the single-evaluation discipline that the story's `validate-factory-path-staging` guard is meant to enforce.

  The fix collapses both call sites into a single `run_t012_gate` invocation whose result object is shared by both the consistency check and the parity check. T-048 asserts the single-call identity invariant: given a fixed staging snapshot, the gate result observed by the consistency check and the parity check are identical (same object, not merely equal).

- **Closure evidence:** `git diff b951461a..1c59a669 crates/factory-dispatcher/tests/bundle_orphan_check.rs` shows the duplicate call site replaced with a single shared `gate_result` binding used by both checks; T-048 present and passing; mutation on the shared-binding path shows no survivor.

---

### ADV-BB-P10-BLK-002 (CLOSED): Proptest candidate set contains fewer than eighteen path-prefix entries — adversarial boundary forms at coverage boundaries untested

- **Severity:** BLOCKER
- **Category:** verification-gaps
- **Status:** CLOSED — proptest expanded to eighteen candidates covering all boundary forms; T-049 covers proptest expansion
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` proptest candidate generation
- **Description:** The proptest coverage for the worktree-root containment predicate generated path-prefix candidates from a fixed set that contained fewer than eighteen entries. Adversarial path-boundary forms — specifically paths with unusual prefix-count structures at the containment boundary — were not represented in the candidate set. Adversarial review confirmed that at least three boundary forms reachable from the deployed environment were absent: (1) a path whose `components().count()` equals exactly the `worktree_root.components().count()` (zero-depth artifact at root level); (2) a path under a symlinked subdirectory that resolves to a sibling of the worktree root when followed; (3) a path beginning with a double-slash that `std::path::Path` normalises to a single slash on POSIX but not on all platforms. These gaps were confirmed by constructing hand-crafted examples that escaped the containment predicate without triggering any existing test.

  The fix expands the proptest candidate set to exactly eighteen entries, each documented with its boundary-form class. T-049 drives the expanded proptest, asserts coverage of all three boundary forms above, and verifies that the containment predicate correctly classifies each.

- **Closure evidence:** `git diff b951461a..1c59a669 crates/factory-dispatcher/tests/bundle_orphan_check.rs` shows proptest candidate set expanded to eighteen entries; T-049 present and passing; all three boundary forms covered.

---

## Part B — New Findings

### HIGH

#### ADV-BB-P10-HIGH-001: T-042 case-variant end-to-end uses `cfg(target_os = "linux")` guard — macOS dev silently skips the test (carry from P09-HIGH-001)

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-042
- **Description:** Unchanged from ADV-BB-P09-HIGH-001. `#[cfg(target_os = "linux")]` silently omits T-042 on macOS rather than surfacing it as an ignored test. A developer claiming `cargo test` green on macOS has not verified the case-variant path.
- **Status in fix wave:** CLOSED — `#[cfg_attr(not(target_os = "linux"), ignore = "case-variant test requires case-sensitive Linux filesystem")]` applied.
- **Proposed Fix (from P09):** Replace `#[cfg(target_os = "linux")]` with `#[cfg_attr(not(target_os = "linux"), ignore = "case-variant test requires case-sensitive Linux filesystem")]`.

#### ADV-BB-P10-HIGH-002: `detect_ungated_declarations` returns `Ok(vec![])` on an I/O error reading the staging snapshot — gate silently passes on unreadable staging area

- **Severity:** HIGH
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `detect_ungated_declarations` error arm
- **Description:** When `detect_ungated_declarations` encounters an I/O error reading the staging snapshot (e.g., git index locked during a concurrent rebase), it returns `Ok(Vec::new())` — an empty result that the caller interprets as "no ungated declarations found, gate passes." The correct behaviour on a read error is to propagate the error or return a sentinel that causes the gate to fail-closed. The fail-open arm was introduced when the function was refactored to return `Result<Vec<Declaration>, GateError>` but the `?` propagation was accidentally dropped in one error arm during the refactor, leaving a silent success path.
- **Status in fix wave:** CLOSED — I/O error arm now returns `Err(GateError::StagingSnapshotUnavailable(e))`; test added asserting that an unreadable snapshot causes gate failure.
- **Proposed Fix:** Replace `Ok(Vec::new())` in the I/O error arm with `Err(GateError::StagingSnapshotUnavailable(e))`.

#### ADV-BB-P10-HIGH-003: Registry parity check calls `wasm_artifacts_equal` with paths not normalised through `lex_norm` — normalised vs. un-normalised comparison can yield false inequality

- **Severity:** HIGH
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` registry-parity comparison block
- **Description:** The registry parity check compared staged artifact paths against registry paths using `wasm_artifacts_equal`, but the staged artifact paths passed to this function were not normalised through `lex_norm` before comparison. The registry paths were stored in normalised form (lower-case extension, forward-slash separators). On a staging action that recorded an artifact with a capital extension or trailing dot, `wasm_artifacts_equal` returned false inequality — the parity check flagged a real match as a mismatch. The issue is orthogonal to the `lex_norm` Windows-backslash fix from P09-MED-002: that fix addressed the normalisation function itself; this finding addresses a call site that bypassed normalisation entirely.
- **Status in fix wave:** CLOSED — staged artifact paths now routed through `lex_norm` before passing to `wasm_artifacts_equal`; T-041b added asserting that a staged path with capital `.WASM` extension matches its lower-case registry counterpart.
- **Proposed Fix:** Apply `lex_norm(staged_path)` before the `wasm_artifacts_equal(lex_norm(staged_path), registry_path)` call at the parity check site.

### MEDIUM

#### ADV-BB-P10-MED-001: Staging guard accepts directory paths as valid staging targets — a directory whose name matches a registered artifact name passes the containment check

- **Severity:** MEDIUM
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` containment predicate; `crates/policy15-gate/src/` guard logic
- **Description:** The containment predicate (`starts_with(worktree_root)`) evaluates path containment without distinguishing between file paths and directory paths. A staging action that adds a directory named `foo.wasm` (rather than a regular file) passes the containment check and satisfies the `GATED` classification even though a directory cannot be a valid WASM artifact. BC-5.39.001 AC-007 requires the guard to verify that the staged entry is a regular file, not a directory. Without this check, an adversarial staging of a directory can suppress a genuine `UNGATED-DECLARATION` signal by occupying the expected artifact path with a non-file entry.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Add `entry.file_type().is_file()` assertion (or equivalent) to the containment predicate; route directory-at-artifact-path to a new `DIRECTORY-AT-ARTIFACT-PATH` identifier class.

#### ADV-BB-P10-MED-002: T-039 `Vec::<Declaration>::new()` comparison uses default `PartialEq` — two `Declaration` values with identical paths but different metadata fields compare equal

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` T-039
- **Description:** The `Declaration` type derives `PartialEq` using field-by-field comparison, but the test fixture only populates the `path` field; the `checksum` and `timestamp` metadata fields are left at `Default::default()`. A bug that altered the checksum or timestamp while preserving the path would pass T-039 undetected. The fix requires the test fixture to populate all metadata fields and the assertion to compare the full `Declaration` struct.
- **Status in fix wave:** CLOSED — T-039 fixture now populates `checksum` and `timestamp`; assertion upgraded to full-struct comparison.
- **Proposed Fix:** Populate all `Declaration` fields in T-039 fixture; assert full struct equality.

#### ADV-BB-P10-MED-003: `bundle_orphan_check` does not assert the ordering of entries in the returned `Vec<Declaration>` — two implementations with different orderings produce identical `is_ok()` assertions

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` multiple tests
- **Description:** Several tests that assert `result == expected_declarations` use a `Vec` comparison that is sensitive to ordering. If the implementation changes the iteration order of the staging snapshot (e.g., switching from `BTreeMap` to `HashMap`), the same logical result is returned in a different order and tests fail spuriously. Conversely, tests that do NOT sort before comparison silently pass when the implementation returns a correct set in the wrong order because the test fixture's `expected_declarations` happens to match the implementation's incidental ordering. The correct approach is to sort both sides before comparison or use an order-insensitive set comparison.
- **Status in fix wave:** CLOSED — all multi-entry `Vec<Declaration>` assertions now sort both sides by path before comparison.
- **Proposed Fix:** Use `let mut result = result.unwrap(); result.sort_by_key(|d| d.path.clone()); assert_eq!(result, expected_sorted);` pattern.

#### ADV-BB-P10-MED-004: Prefix-conjunct in containment predicate does not isolate against sibling-path false positives when multiple factory-root candidates are registered

- **Severity:** MEDIUM
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` containment predicate; `crates/policy15-gate/src/` guard logic
- **Description:** The containment predicate applies `starts_with(worktree_root)` as a prefix check, but does not append a path separator to the `worktree_root` before the comparison. A worktree root at `/home/user/repo` would pass the containment check for a path like `/home/user/repo-sibling/hook-plugins/foo.wasm` because the string prefix `/home/user/repo` is present. On a system where both `/home/user/repo` and `/home/user/repo-sibling` exist, a sibling-path artifact can be classified as `GATED` when it should be classified as `OUTSIDE-REPO-DECLARATION`. The fix requires the predicate to compare against `worktree_root.join("")` (ensuring a trailing separator) or to use a component-count-based prefix comparison rather than a string/bytes prefix check.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Replace `path.starts_with(worktree_root)` with `path.starts_with(worktree_root.join(""))` or a component-level prefix comparison; add T-NNN asserting that a sibling-directory path is classified as `OUTSIDE-REPO-DECLARATION`.

#### ADV-BB-P10-MED-005: `GateOutcome::Inert` is returned when the staging snapshot contains zero entries — spec requires `Gated` when staging is non-empty; `Inert` when truly empty

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `crates/policy15-gate/src/` gate logic; `crates/factory-dispatcher/tests/bundle_orphan_check.rs`
- **Description:** BC-5.39.001 AC-003 specifies that when the staging snapshot is empty, the gate MUST return `GateOutcome::Inert` (no staging action; skip). When the staging snapshot is non-empty, the gate MUST proceed to the full evaluation path. The implementation correctly returns `Inert` for an empty snapshot, but an edge case was found where a snapshot containing only directory entries (no regular files) also returned `Inert` rather than proceeding to evaluate whether any of the directory entries should have been classified as `OUTSIDE-REPO-DECLARATION` or `DIRECTORY-AT-ARTIFACT-PATH`. The early-exit `Inert` branch triggered on `is_empty()` of the file-only filtered view rather than of the raw snapshot, causing directory-only staging actions to escape gate evaluation entirely.
- **Status in fix wave:** CLOSED — early-exit `Inert` branch now checks `raw_snapshot.is_empty()` rather than `file_entries.is_empty()`; directory entries proceed to the full evaluation path.
- **Proposed Fix:** Change early-return guard from `if file_entries.is_empty()` to `if raw_snapshot.is_empty()`.

#### ADV-BB-P10-MED-006: `bundle_orphan_check` integration test uses a hard-coded absolute path fixture (`/tmp/vsdd-test-...`) — fails on Windows and in sandboxed CI environments

- **Severity:** MEDIUM
- **Category:** portability
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` test fixture setup
- **Description:** Three test fixtures in `bundle_orphan_check.rs` construct staging-snapshot paths using the hard-coded prefix `/tmp/vsdd-test-`. On Windows, `/tmp/` does not exist, causing fixture construction to fail before the test body runs. In sandboxed CI environments that mount `/tmp` read-only or map it to a different path, the hard-coded prefix produces an inaccessible path and the test fails with an I/O error rather than a test assertion failure, obscuring the root cause. BC-5.39.001 requires tests to use `tempfile::tempdir()` or equivalent portable temporary-directory construction.
- **Status in fix wave:** CLOSED — three fixtures updated to use `tempfile::tempdir()` with the temp-dir path passed to the fixture builder; hard-coded `/tmp/` prefix removed.
- **Proposed Fix:** Replace `PathBuf::from("/tmp/vsdd-test-...")` with `tempfile::tempdir()?.path().join("vsdd-test-...")` in all three affected fixtures.

### LOW

#### ADV-BB-P10-LOW-001: Staging guard does not handle filenames containing NUL bytes or trailing spaces — such names bypass normalisation and path-containment checks

- **Severity:** LOW
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `lex_norm`; staging snapshot reader
- **Description:** `lex_norm` does not strip NUL bytes (`\0`) or trailing spaces from filenames before normalisation. On POSIX systems, a filename like `foo.wasm\0` is technically distinct from `foo.wasm` and would not match the registry path `foo.wasm` during the parity check — an adversarially crafted artifact with a NUL-appended name would escape the gate. Similarly, a filename with trailing spaces (`foo.wasm   `) would bypass comparison on Windows (where trailing spaces are stripped by the filesystem) but not on Linux (where they are preserved). The guard does not normalise these forms before comparison, producing inconsistent gate behaviour across platforms.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Add NUL-byte stripping and trailing-space trimming as normalisation steps in `lex_norm`; add proptest candidates covering these forms.

#### ADV-BB-P10-LOW-002: Fail-open `Ok(Vec::new())` arms in `detect_ungated_declarations` are guarded only by unasserted call-ordering assumptions — reordering callers reintroduces silent pass

- **Severity:** LOW
- **Category:** correctness
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` `detect_ungated_declarations` residual fail-open arms
- **Description:** ADV-BB-P10-HIGH-002 addressed the primary fail-open arm (I/O error on snapshot read). However, two secondary fail-open arms remain: (1) when the `registry_lookup` call returns `None` for a declared path, the current code returns `Ok(Vec::new())` under the assumption that `None` means "not yet registered" (a valid pre-registration state); (2) when the staging snapshot contains an entry whose path component count is below a minimum threshold, the current code returns `Ok(Vec::new())` under the assumption that such paths are always system-generated temporaries. Both assumptions depend on unasserted call-ordering invariants: if the caller invokes `detect_ungated_declarations` before the registry is populated (violating assumption 1) or on a staging snapshot that includes legitimate short-path artifacts (violating assumption 2), the gate silently passes without error. Neither assumption is documented or structurally enforced.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Document both assumptions as `// INVARIANT:` comments with `debug_assert!` guards; add tests that verify the gate fails-closed when each invariant is violated.

#### ADV-BB-P10-LOW-003: `workspace_root()` helper not tested directly — only exercised via integration path; a regression in the helper would manifest as a confusing containment-predicate failure

- **Severity:** LOW
- **Category:** verification-gaps
- **Location:** `crates/factory-dispatcher/tests/bundle_orphan_check.rs` — no direct unit test for `workspace_root()`
- **Description:** The `workspace_root()` helper function, introduced as part of the worktree-root containment predicate in pass-9, is exercised only indirectly through the integration tests for `detect_ungated_declarations`. No unit test directly verifies that `workspace_root()` returns the correct path for a repository root, a nested worktree, and a path outside any repository. A regression in `workspace_root()` (e.g., returning the wrong directory level when the `.git` pointer is a gitlink file rather than a directory) would manifest as a spurious `OUTSIDE-REPO-DECLARATION` from the containment predicate rather than a clear `workspace_root() returned unexpected value` failure, making the root cause difficult to diagnose.
- **Status:** OPEN — not addressed in fix wave through `1c59a669`.
- **Proposed Fix:** Add three unit tests: `workspace_root_at_repo_root`, `workspace_root_in_nested_worktree`, `workspace_root_outside_any_repo` (returns `Err`).

### NIT

#### ADV-BB-P10-NIT-001: `GateError` variants lack rustdoc — error messages produced by the gate carry no in-source documentation for operators troubleshooting hook failures

- **Severity:** NIT
- **Category:** code-quality
- **Location:** `crates/policy15-gate/src/` `GateError` enum
- **Description:** The `GateError` enum variants (`StagingSnapshotUnavailable`, `RegistryReadError`, `ContainmentCheckFailed`) carry no rustdoc comments. When an operator encounters a gate failure in the dispatcher log, the error variant name is the only diagnostic information available. Adding one-line rustdoc on each variant describing the condition that triggers it would reduce time-to-diagnosis.
- **Status in fix wave:** CLOSED — one-line rustdoc added to each `GateError` variant.
- **Proposed Fix:** Add `/// <Condition description>.` before each `GateError` variant.

---

## Summary

| Severity | Count | Open | Closed |
|----------|-------|------|--------|
| BLOCKER | 2 | 0 | 2 (closed in `1c59a669`) |
| HIGH | 3 | 0 | 3 (closed in fix wave before `1c59a669`) |
| MEDIUM | 6 | 2 (MED-001, MED-004) | 4 |
| LOW | 3 | 3 | 0 |
| NIT | 1 | 0 | 1 |

**Overall Assessment:** block
**Convergence:** spec-vs-reality drift: **zero**; pre-existing-code defects: **zero**; remaining: 2 MEDIUM (directory-only control; prefix-conjunct sibling-path isolation) + 3 LOW (NUL/trailing-space names; fail-open arms; `workspace_root()` unit coverage)
**Readiness:** requires revision

Both BLOCKERs — duplicate gate evaluation at divergent call sites (ADV-BB-P10-BLK-001) and insufficient proptest candidate coverage (ADV-BB-P10-BLK-002) — are confirmed CLOSED in `1c59a669` (single-copy detect gate + T-048/T-049). All three HIGHs are confirmed CLOSED in the fix wave preceding `1c59a669`. BC-5.39.001 3-CLEAN protocol requires zero findings of any severity for a CLEAN pass; streak remains 0/3 after ten passes.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 10 |
| **Story version reviewed** | v1.19 |
| **Reviewed commit** | b951461a |
| **New findings (BLOCKERs closed in `1c59a669`)** | 2 (BLK-001 duplicate gate eval; BLK-002 proptest narrow candidates) |
| **New findings (open after `1c59a669`)** | 5 (MED-001/004; LOW-001/002/003) |
| **New findings (closed in fix wave)** | 8 (BLK-001/002 + HIGH-002/003 + MED-002/003/005/006 + NIT-001) |
| **Carry-over findings** | 1 (P09-HIGH-001 T-042 cfg guard → closed in fix wave) |
| **Resolved vs. prior pass** | 8 carry-overs resolved (P09-BLK-001 verified; P09-MED-002/003 fixed; P09-LOW-002 fixed; P09-NIT-001 fixed; P09-LOW-001 reclassified; P09-MED-001 deferred-dropped) |
| **Mutation testing** | T-048/T-049 cover single-copy gate identity and 18-candidate proptest boundary forms; no survivors on single-call-site or candidate-set mutations |
| **Novelty score** | 10 / (10 + 1) = 0.91 |
| **Median severity** | MEDIUM |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3→2→1→3 |
| **Total finding trajectory** | →9→9→8→8→15 (pass-7: 9; pass-8: 8; pass-9: 8; pass-10: 15 — regression driven by BLK+HIGH discovery) |
| **Verdict** | FINDINGS_REMAIN |
