---
document_type: adversary-pass-report
producer: adversary
pass_id: S-15.11-LOCAL-P1
diff_base: origin/develop@6fe7de4c
diff_head: feature/S-15.11-validate-burst-log@e412619b
verdict: LOW
streak_status: "0/3 (first pass — resets due to LOW finding)"
timestamp: 2026-05-16T00:00:00Z
---

# S-15.11 LOCAL Adversary Pass-1

**Diff base:** `origin/develop@6fe7de4c`
**Diff head:** `feature/S-15.11-validate-burst-log@e412619b`
**Commits under review:**
- `3951cc86` test(S-15.11): failing bats integration tests for validate-burst-log WASM hook (Red Gate)
- `e412619b` feat(hooks): validate-burst-log WASM hook — burst-log structural completeness gate (S-15.11)

## Part A: Findings

### F-S15.11-LOCAL-P1-001 — `validate_h2_heading` does not enforce end-of-line anchor on date close-paren

**Severity:** LOW
**Confidence:** HIGH
**File:** `crates/hook-plugins/validate-burst-log/src/lib.rs`, function `validate_h2_heading` (lines 153–225).

**Evidence:** The story spec line 561 (and BC-5.39.004 postcondition 2 line 69) declare the canonical pattern as `^## Burst: .+\(\d{4}-\d{2}-\d{2}\)$` — with an explicit end-of-string `$` anchor. The implementation validates bytes 0–10 of `inside` (the slice after `(`) match `YYYY-MM-DD)` but does NOT assert `inside.len() == 11`. Concrete failing case: `validate_h2_heading("## Burst: foo (2026-05-12)xyz")` returns `true` — `rfind('(')` finds the only `(`; `inside = "2026-05-12)xyz"`; `bytes.len() = 14 >= 11`; bytes 0–10 validate; function returns true. The trailing `xyz` slips through.

**Impact:** Realistic burst-log writes don't typically have trailing junk after the date parens, so production impact is low. But the spec is unambiguous and the gap is exploitable by malformed-but-near-canonical h2 lines.

**Fix (structural):** After validating bytes 0–10 of `inside`, add `if inside.len() != 11 { return false; }`. Add a unit test: `assert!(!validate_h2_heading("## Burst: foo (2026-05-12)abc"))`.

## Part B: Observations

### O-P1-001 — Doc-comment narrative does not mention `Edit|Write` token explicitly (pending intent verification)

**File:** `crates/hook-plugins/validate-burst-log/src/lib.rs`, doc-comments.

The story spec T-15 and architect Q5/Q6 lock identify five reference classes that must carry the canonical `Edit|Write` form: production registry, test inline registries, spec body, AC verification predicates, and **doc-comment narrative**. The validate-burst-log lib.rs has ZERO occurrences of the literal `Edit|Write` token. By contrast, the canonical sibling validate-index-cite-refresh/src/lib.rs has 2 doc-comment references (lines 485 and 510) making the dispatcher routing path explicit. Tagged pending intent verification: the omission may be intentional (one canonical reference per repo) or sibling-sweep drift (TD-VSDD-060). Five-reference-class sweep audit recommended.

### O-P1-002 — `file_path.ends_with("burst-log.md")` matches false-positive paths like `xburst-log.md` [process-gap candidate]

**File:** `crates/hook-plugins/validate-burst-log/src/lib.rs` line 408.

`"xburst-log.md".ends_with("burst-log.md")` evaluates to `true`. The guard fires on any file whose name ends with `burst-log.md` regardless of actual filename. Spec line 308 mandates this exact form; sibling validate-index-cite-refresh has the identical pattern at its line 512 (`ends_with("ARCH-INDEX.md")`). Stricter form: `Path::file_name() == Some("burst-log.md")` or `ends_with("/burst-log.md") || == "burst-log.md"`. NOT counted as a finding because spec-compliant; tagged `[process-gap]` since the pattern recurs across both M2-wave hooks.

### O-P1-003 — `count_dim1_list_items` redundant predicate (cosmetic only)

**File:** `crates/hook-plugins/validate-burst-log/src/lib.rs` lines 321–326. Two equivalent `if trimmed.starts_with("**")` branches; first branch's `ends_with("**")` clause is subsumed by second. Behavior unchanged. Cosmetic NITPICK.

### O-P1-004 — Cargo.toml workspace inheritance correct (commendation)

Dual lib + bin targets; all workspace fields inherit correctly.

### O-P1-005 — `path_allow = [".factory/cycles/**"]` glob form is correct (commendation)

Burst-log files live across multiple cycle directories. Glob form is the right choice (more flexible than sibling's explicit per-file list).

### O-P1-006 — Unit tests are non-tautological (POLICY 11 compliance commendation)

Every `test_BC_5_39_004_*` test calls a production function (`validate_h2_heading`, `check_block_presence`, etc.). No POLICY 11 violation.

### O-P1-007 — `cited_raw: String` plumbing is structurally present (TD-VSDD-059 commendation)

The `Violation` struct has mandatory `cited_raw: String` field, populated at violation sites with actual raw body-literal text via `.trim_end().to_string()`. Structural plumbing, not doc-comment workaround.

## Part C: Policy Rubric Compliance

| Policy | Status | Notes |
|--------|--------|-------|
| POLICY 1 (append_only_numbering) | PASS | BC-5.39.004 monotonically allocated. |
| POLICY 2 (lift_invariants_to_bcs) | PASS | No new domain invariants. |
| POLICY 3 (state_manager_runs_last) | N/A | Per-story impl pass. |
| POLICY 4 (semantic_anchoring_integrity) | PASS | All D-NNN anchors map correctly; BC subsystem SS-05 semantically correct. |
| POLICY 5 (creators_justify_anchors) | PASS | Story spec lines 379–383 carry justification. |
| POLICY 6 (architecture_is_subsystem_name_source_of_truth) | PASS | SS-05 matches ARCH-INDEX. |
| POLICY 7 (bc_h1_is_title_source_of_truth) | PASS | BC-5.39.004 H1 matches story body BC table. |
| POLICY 8 (bc_array_changes_propagate_to_body_and_acs) | PASS | BC-5.39.004 in frontmatter; body BC table + ACs consistent. |
| POLICY 9 (vp_index_is_vp_catalog_source_of_truth) | N/A | No VPs in this story. |
| POLICY 10 (demo_evidence_story_scoped) | N/A | No demo artifacts. |
| POLICY 11 (no_test_tautologies) | PASS | See O-P1-006. |
| POLICY 12 (bc_tv_emitter_consistency) | PASS | Violation struct is internal-only, not serialized. |
| POLICY 13 (hh_n_regex_alternation_predicates) | PASS | No pre-fix predicates added. |
| POLICY 14 (kk_n_tripartite_parity_gate) | PASS | BC frontmatter last_amended + modified parity. |
| POLICY 15 (ll_n_verbatim_stdout_discipline) | N/A | No literal-shell-evidence claims. |
| POLICY 16 (mm_n_cross_cycle_namespace_gate) | PASS | D-NNN refs valid globally. |
| POLICY 17 (nn_n_frontmatter_parity_full_file_type_scope) | PASS | BC + story parity holds. |
| POLICY 18 (oo_input_hash_mechanical_verification) | N/A | No input-hash claims. |

## Part D: Verdict + Streak

**Verdict:** LOW
**Rationale:** One finding at LOW severity (F-S15.11-LOCAL-P1-001 — h2 trailing-content anchor gap). Spec pattern explicitly anchors with `$`; impl misses it. Real spec-implementation drift even though practical exploit case is unlikely.
**Streak:** 0/3 (first pass — finding resets streak).

## Part E: Recommendations for Fix-Burst-1

1. **F-S15.11-LOCAL-P1-001 fix (LOW, MANDATORY):** Add `inside.len() != 11` guard in `validate_h2_heading` + unit test for trailing-content cases.
2. **O-P1-001 (production-grade default):** Add 1–2 doc-comment references to `Edit|Write` literal in lib.rs to parity-match the sibling validate-index-cite-refresh.
3. **O-P1-002 (production-grade default, expand scope):** Harden `ends_with` to `Path::file_name`-strict in BOTH validate-burst-log AND validate-index-cite-refresh. Spec is silent on internal strictness; conservative refinement permitted.
4. **O-P1-003 (cosmetic):** Remove redundant predicate branch in `count_dim1_list_items`.
