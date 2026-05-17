---
document_type: adversary-pass-report
producer: adversary
pass_id: S-15.11-LOCAL-P2
diff_base: origin/develop@6fe7de4c
diff_head: feature/S-15.11-validate-burst-log@09a03aaa
verdict: HIGH
streak_status: "0/3 (HIGH finding resets per BC-5.39.001)"
timestamp: 2026-05-16T00:00:00Z
---

# S-15.11 LOCAL Adversary Pass-2

**Diff base:** `origin/develop@6fe7de4c`
**Diff head:** `feature/S-15.11-validate-burst-log@09a03aaa`
**Commits under review (8):** 3951cc86 (Red Gate) → e412619b (impl) → dc7a0f39 (Fix 1 h2 anchor) → cfb8b747 (Fix 2 Edit|Write doc-comment) → a9c59d07 (Fix 3 path-strict both hooks) → c5b529d3 (Fix 4 cosmetic) → ec4651b1 (fmt) → 09a03aaa (Fix-burst-1-extension lint-registry sibling-sweep)

## Part A: Findings

### F-S15.11-LOCAL-P2-001 — HIGH — production `path_allow` glob silently neuters the hook

**Severity:** HIGH. **File:** `plugins/vsdd-factory/hooks-registry.toml:871`.

The production registry uses `path_allow = [".factory/cycles/**"]`. The dispatcher's `path_allowed()` (`crates/factory-dispatcher/src/host/read_file.rs:122-148`) calls `canonicalize()` on each prefix; `canonicalize(".factory/cycles/**")` fails because no directory literally named `**` exists. Per line 142 the failed-canonicalize prefix is skipped via `Err(_) => continue`. The allow list is effectively empty. Every `host::read_file` from validate-burst-log returns CapabilityDenied. The hook then takes the fail-open path at `lib.rs:456-461` (`Err(e) => { host::log_warn(...); return HookResult::Continue; }`). The hook silently never validates anything in production.

Bats tests cannot detect this — their inline `_write_registry()` heredocs use `.factory/cycles/` (no glob), which canonicalize()s correctly. Production-vs-bats registry drift; load-bearing capability semantics are exercised in bats but not against the production registry shape.

Sibling registry entries (validate-stable-anchors, validate-artifact-path, validate-index-cite-refresh) all use bare paths or explicit per-file lists — no `**` glob anywhere else in hooks-registry.toml.

**Fix:** Change line 871 to bare `".factory/cycles"`. Add a load-bearing integration test that exercises the PRODUCTION registry shape (not the bats inline shape) against `host::read_file` capability boundary, asserting the hook actually blocks for an obvious 6-block fixture.

### F-S15.11-LOCAL-P2-002 — MEDIUM — registry comment + spec citations cite stale `ends_with` guard form

**Severity:** MEDIUM. **Files:**
- `plugins/vsdd-factory/hooks-registry.toml:857` (preceding-comment narrative)
- `.factory/stories/S-15.11-validate-burst-log.md:308` (T-8 note)
- `.factory/stories/S-15.11-validate-burst-log.md:214` (AC-10 verification predicate)

The path-component-strict refactor in commits a9c59d07 + 09a03aaa did not propagate to the registry preceding-comment or the story spec. AC-10's grep predicate (`grep 'ends_with.*burst-log.md'`) now matches only the rejected-form doc-comment at lib.rs:384 — not the load-bearing path-strict code. This is a TD-VSDD-060 sibling-site sweep regression and a POLICY 11 test-tautology trap.

**Fix:** Update registry comment + spec line 308 + AC-10 grep predicate to cite path-component-strict form. AC-10 grep must match load-bearing code (e.g., `grep 'Path::new.*file_name' crates/hook-plugins/validate-burst-log/src/lib.rs`).

### F-S15.11-LOCAL-P2-003 — MEDIUM — bats fixture for AC-4 exercises the no-h2 path, not the malformed-h2 path

**Severity:** MEDIUM. **File:** `plugins/vsdd-factory/tests/fixtures/validate-burst-log/fail-malformed-h2/factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md:1`.

The fixture begins `## Fix Burst: Pass-44 description without parenthesized date` — prefix `## Fix Burst:`, NOT `## Burst:`. The hook's `extract_latest_burst()` doesn't match this as a `## Burst:` line, returns None, and falls into the no-h2 code path — same as `fail-no-h2.bats`. AC-4 silently degrades to AC-6. The `validate_h2_heading → false` integration path (h2 with `## Burst:` but malformed date) has no bats coverage.

**Fix:** Amend the fixture (or replace) with `## Burst: Pass-44 description with bad date (2026-5-12)` — h2 with canonical prefix but malformed (non-zero-padded month) date.

### F-S15.11-LOCAL-P2-004 — LOW — `check_dim1_cardinality` silent-pass on Dim-1-present-headline-unparseable

**Severity:** LOW. **File:** `crates/hook-plugins/validate-burst-log/src/lib.rs:259-293`.

When Dim-1 block exists but headline integer is unparseable, `extract_dim1_headline_count` returns None, `?` short-circuits, function returns None — silently treats unparseable headline as valid. Defensive depth gap; current ACs pass because `check_block_presence` flags Dim-1 absence first, but unparseable-headline case isn't surfaced.

**Fix:** Restructure to distinguish "Dim-1 block absent" from "Dim-1 block present with unparseable headline"; emit a violation for the latter. Add unit test.

## Part B: Observations

### O-P2-001 — NITPICK — `lib.rs:384` doc-comment quotes `ends_with` as the REJECTED form (correct, not a finding).

### O-P2-002 — NITPICK — `validate_h2_heading` strict-space check disallows tab between description and `(`. Spec-correct; common-author-tool tolerance not relevant.

### O-P2-003 — [process-gap] — bats inline `_write_registry()` heredocs diverge from production registry shape with no automated parity check. The HIGH F-001 finding is a manifestation of this gap. Recommend codifying a same-burst gate: bats inline `path_allow` arrays MUST be byte-identical to production registry entry for the same hook (or extract-from-production via tooling). Routes to S-15.03 PRIORITY-A automation wave consideration.

## Part C: Policy Rubric Compliance

| # | Policy | Verdict |
|---|--------|---------|
| 1 | append_only_numbering | PASS |
| 2 | lift_invariants_to_bcs | N/A |
| 3 | state_manager_runs_last | N/A (per-story phase) |
| 4 | semantic_anchoring_integrity | **VIOLATION (MEDIUM)** — F-002 stale `ends_with` citations |
| 5 | creators_justify_anchors | PASS |
| 6 | architecture_is_subsystem_name_source_of_truth | PASS |
| 7 | bc_h1_is_title_source_of_truth | PASS |
| 8 | bc_array_changes_propagate_to_body_and_acs | PASS |
| 9 | vp_index_is_vp_catalog_source_of_truth | N/A |
| 10 | demo_evidence_story_scoped | N/A |
| 11 | no_test_tautologies | **VIOLATION (MEDIUM)** — F-002 AC-10 predicate now matches doc-comment-only; F-003 AC-4 fixture degrades to AC-6 |
| 12 | bc_tv_emitter_consistency | N/A |
| 13 | hh_n_regex_alternation_predicates | PASS (canonical `Edit\|Write` order respected; zero `Write\|Edit` in feature-branch-touched files) |
| 14 | kk_n_tripartite_parity_gate | N/A (no factory-artifacts changelog row in feature branch) |
| 15 | ll_n_verbatim_stdout_discipline | N/A |
| 16 | mm_n_cross_cycle_namespace_gate | PASS |
| 17 | nn_n_frontmatter_parity_full_file_type_scope | N/A |
| 18 | oo_input_hash_mechanical_verification | N/A |

## Part D: Verdict + Streak

**Verdict:** HIGH (1H + 2M + 1L + 1 process-gap observation).
**Rationale:** F-001 is a load-bearing production defect — hook silently neutered. F-002 + F-003 are TD-VSDD-060 sibling-sweep regressions / POLICY 11 test-tautology traps. F-004 is a defensive-depth gap.
**Streak:** 0/3 (HIGH resets per BC-5.39.001).

## Part E: Recommendations for Fix-Burst-2

1. **F-001 (HIGH, fix-first):** Change registry line 871 to bare `.factory/cycles`. Add production-shape integration test (failed-before/passed-after revert verification required).
2. **F-002 (MEDIUM):** Update registry comment + story spec line 308 + AC-10 grep predicate to cite path-component-strict form. AC-10 grep must match load-bearing code.
3. **F-003 (MEDIUM):** Amend AC-4 fixture to exercise `validate_h2_heading → false` for malformed canonical-form (not no-prefix).
4. **F-004 (LOW):** Tighten `check_dim1_cardinality` 3-state distinction + unit test.
5. **O-P2-003 (process-gap):** Surface to orchestrator for S-15.03 PRIORITY-A consideration: codify bats-vs-production registry parity gate.

Anti-paper-fix verification on next pass: confirm F-001 fix is the registry line change AND a load-bearing test that exercises production registry capability shape (not just bats inline shape).
