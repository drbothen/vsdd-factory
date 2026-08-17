# PR #780 — Final Fresh-Eyes Review

- **PR:** #780 `feat(S-21.10): failure_policy registry schema extension (ADR-039 Phase 1 — schema only, no enforcement change)`
- **Branch:** `feature/S-21.10` → `develop`
- **Reviewed HEAD:** `58c0435e62894523135d1d9d4ac8374259a20182`
- **Story:** S-21.10 (E-21 Wave 5) — ADR-039 Phase 1 schema leg
- **BC:** BC-1.01.016 v1.3 (PC1–PC7, EC-001..EC-007, 15 TDD tests)

## Verdict: REQUEST_CHANGES

Code content is approved on the merits, but merge is blocked by a red required check caused by an empty tip commit. No `covered_sha` is emitted because the fix changes the HEAD SHA.

## Code content — APPROVED

The substantive diff is production-grade and correct:

1. **`FailurePolicy` enum** (`registry.rs`) — `#[serde(rename_all = "kebab-case")]`, `#[default]` on `FailOpen`, `Copy`/`Eq` derives. Doc comment correctly flags the kebab-vs-snake hazard: copying the sibling `OnError`'s `snake_case` would have silently accepted `fail_closed` and opened a bypass.
2. **`RegistryEntry.failure_policy`** — `#[serde(default)]`; doc comment explicitly warns against refactoring to `Option<FailurePolicy>` and documents the per-plugin (not `RegistryDefaults`) design decision per ADR-039 §Decision 2 / SR-003.
3. **PC7 no-enforcement boundary respected** — `plugin_fail_closed` in `executor.rs` is unchanged; it appears only in doc comments in the diff. The RED-gate test `fail_closed_timeout_with_on_error_continue_is_open` is untouched.
4. **All `RegistryEntry` struct literals updated** — `executor.rs`, `partition.rs` kani proof, and 5 integration test files (`async_partition_integration.rs`, `executor_integration.rs`, `executor_resolver_integration.rs`, `full_stack_plugin_invocation.rs`, `resolver_error_isolation_test.rs`) all add `failure_policy`. Confirmed by clean `cargo-host` on linux + macos.
5. **15 TDD tests** in `mod s21_10_bc_1_01_016_failure_policy`, each traced to a BC-1.01.016 PC/EC. Verified coverage: PC1 (fail-closed parse), PC2 (fail-open parse), PC3 (unknown → Err), PC4 (absent → FailOpen), PC5/EC-006 (continue + fail-closed simultaneous), PC6/EC-007 (production registry all-FailOpen), PC7 (registry-side scope guard: `on_error()` accessor unaffected by `failure_policy`), EC-001 (wrong case), EC-002 (empty string), EC-003 (underscore, both variants), EC-005 (block + fail-open), plus round-trip serialize and `default()` invariant. EC-004 (duplicate key) correctly documented as a TOML-parser-layer concern, out of registry-unit scope.
6. **Prior findings resolved** — FINDING 1 (lib.rs re-export of `FailurePolicy`) and FINDING 2 (module comment EC range "EC-001..EC-003, EC-005..EC-007" with EC-004 note) confirmed fixed; the module-comment coverage claim is accurate against the actual tests.
7. **CHANGELOG** updated with the S-21.10 Added entry.

## MERGE BLOCKER — empty tip commit trips POLICY 15 gate

Required check `policy-15-attestation-location` is **RED** (exit 2). Root cause is not the code — it is the empty tip commit:

```
58c0435e ci(S-21.10): retrigger CI after factory-artifacts corpus-gate fix (D-1023)
```

`git diff --name-only 58c0435e^1 58c0435e` is empty. The POLICY 15 gate (`crates/policy15-attestation-gate/src/lib.rs:344-350`, introduced in #777) walks every commit in `merge-base..HEAD` and flags any single-parent commit with an empty diff as `EMPTY-or-UNREACHABLE: unmeasurable diff`, unconditionally, before the pinned-crate (`crates/hook-plugins/validate-cross-site-correspondence`) activation check. The gate log reads `unmeasurable diff at commit 58c0435e6289`.

**Remediation:** drop the empty commit so HEAD becomes `e6e86ba6` (`git rebase` to drop `58c0435e`, or `git reset --soft HEAD~1` then re-push). All remaining commits have non-empty diffs and none touch the pinned crate, so the gate resolves to a clean pass. Re-review at the new HEAD before merge.

## CI status

- `policy-15-attestation-location`: **FAIL** (empty tip commit — see above)
- `cargo-host` (ubuntu-latest, macos-latest): PASS
- `bats-full-suite` (linux), `bats-darwin-leg`, `bats-wave-handoff`: PASS
- `build-dispatcher` (linux-x64, linux-arm64): PASS; darwin/windows legs pending at review time
- `SAST (Semgrep)`, `validate`, `platforms-drift`, `attestation-gate-non-vacuity-controls`: PASS

## Covered SHA

None — verdict is REQUEST_CHANGES. Will re-verify and approve at the new HEAD once the empty commit is dropped and CI is green.
