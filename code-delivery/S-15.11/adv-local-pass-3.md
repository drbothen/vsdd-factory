---
document_type: adversary-pass-report
producer: adversary
pass_id: S-15.11-LOCAL-P3
diff_base: origin/develop@6fe7de4c
diff_head: feature/S-15.11-validate-burst-log@568fe198
verdict: LOW
streak_status: "0/3 (LOW resets per BC-5.39.001)"
timestamp: 2026-05-16T00:00:00Z
---

# S-15.11 LOCAL Adversary Pass-3

**Diff base:** `origin/develop@6fe7de4c`
**Diff head:** `feature/S-15.11-validate-burst-log@568fe198`
**Commits under review (10):** Red Gate + impl + 6 fix-burst-1 commits + 2 fix-burst-2 commits.

## Part A: Findings

### F-S15.11-LOCAL-P3-001 — BC-5.39.004 precondition 1 stale: "ends with" contradicts path-component-strict implementation

**Severity:** MEDIUM. **Confidence:** HIGH.
**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.004.md` line 61.
**Policies tripped:** POLICY 4 (semantic_anchoring_integrity, MEDIUM); POLICY 8 (partial — bidirectional consistency between BC body and code).

**Evidence:** BC-5.39.004.md line 61 reads "A PostToolUse Edit/Write event has fired on a file whose path **ends with** `burst-log.md`." Implementation at `crates/hook-plugins/validate-burst-log/src/lib.rs:415-420` uses `Path::file_name() == Some("burst-log.md")` (path-component-strict). Story spec v1.1 swept the story body away from `ends_with` to path-component-strict language; BC body was NOT swept in parallel.

**Why it matters:** The path `/some/dir/xburst-log.md` "ends with burst-log.md" but implementation correctly rejects it. A future implementer reading the BC as source-of-truth would regress the false-positive guard explicitly closed in pass-1 fix-burst.

**Fix:** Update precondition 1 to path-component-strict language; BC-INDEX last_amended sweep or version bump.
**Routing:** product-owner (BC body content per Routing Table).

### F-S15.11-LOCAL-P3-002 — Story spec File Structure Requirements understates bats test count by 1

**Severity:** LOW (pending intent verification). **Confidence:** HIGH.
**File:** `.factory/stories/S-15.11-validate-burst-log.md` lines 487-496 + 524.
**Policies tripped:** POLICY 4.

**Evidence:** Story spec enumerates 7 bats test files; on-disk inventory shows 8 (adds `integration-production-registry.bats`, created in fix-burst-2 to close F-S15.11-LOCAL-P2-001 production-registry `**` glob bug). Line 524 totals "7 bats test files" — drift.

**Fix:** Either enumerate the 8th file in File Structure Requirements + bump count, OR document why the integration test is fix-burst-only ephemeral (NOT recommended — test is structurally load-bearing).
**Routing:** story-writer (spec body); state-manager (STORY-INDEX bump if version-affecting).

## Part B: Observations

### O-P3-001 — Cross-crate path-component-strict guard parity is EXEMPLARY (compliance commendation)

Three hook crates use identical `Path::file_name() == Some("<canonical>")` form:
- validate-burst-log:`is_burst_log_target` (line 415, `Some("burst-log.md")`)
- validate-index-cite-refresh:`is_arch_index_target` (line 490, `Some("ARCH-INDEX.md")`)
- lint-registry-async-invariant:`is_registry_target` (line 265, `Some("hooks-registry.toml")`)
Function naming, doc-comment language, error-handling, BC-trace comment placement: byte-uniform across siblings.

### O-P3-002 — F-S15.11-LOCAL-P2-001 fix verification is LOAD-BEARING (TD-VSDD-059 paper-fix check PASSES)

Production registry uses `path_allow = [".factory/cycles"]` (no `**`). Independent grep `path_allow.*\*\*` returns zero matches. `integration-production-registry.bats` Scenario B extracts the production path_allow verbatim, copies an incomplete fixture, and asserts the hook BLOCKS (exit 2). Regression detection: if `**` returns, canonicalize() fails → CapabilityDenied → fail-open Continue (exit 0) → test fails with `expected status 2, got 0`. Structural test, not paper-fix.

### O-P3-003 — F-S15.11-LOCAL-P2-004 fix verification is LOAD-BEARING

`check_dim1_cardinality` distinguishes 3 states. Unit test `test_BC_5_39_004_dim1_block_present_without_integer_headline_emits_violation` asserts the unparseable-headline case surfaces a violation with `headline_count == 0, list_count == 2`.

### O-P3-004 — Canonical `tool = "Edit|Write"` 5-reference-class sweep is CLEAN

Independent grep `Write|Edit` (reverse-form) across all feature-branch-touched files returns zero matches.

### O-P3-005 — Production-grade discipline observed across diff

No `unwrap()` in production paths; no `println!`; no MVP/for-now/TODO-for-architect rationalizations; no silent `Vec::new()` returns; `cited_raw: String` structurally plumbed.

### O-P3-006 — REQUIRED_BLOCK_TOKENS prefix-match edge (informational, not a finding)

Tokens `**Dim-2`, `**Dim-5`, `**Dim-6`, `**Dim-7` match via `contains()`; hypothetical `**Dim-25` would falsely satisfy `Dim-2`. Risk informational — burst-log convention has Dim-1 through Dim-7 only.

## Part C: Policy Rubric Compliance

| Policy | Result | Note |
|---|---|---|
| POLICY 1 | PASS | BC-INDEX v2.27, STORY-INDEX advance, BC-5.39.004 monotonic |
| POLICY 2 | PASS | All 8 D-NNN closures anchored in BC-5.39.004 Traceability table |
| POLICY 3 | PASS | feature/ branch diff; no factory-artifacts writes from non-state-manager visible |
| POLICY 4 | **PARTIAL FAIL** | F-P3-001 (MEDIUM): BC precondition drifts from impl; F-P3-002 (LOW): file-count drift |
| POLICY 5 | PASS | Subsystem + capability anchor justifications present |
| POLICY 6 | PASS | BC subsystem SS-05 matches Engine Governance per ARCH-INDEX |
| POLICY 7 | PASS | BC H1 matches BC-INDEX row title |
| POLICY 8 | **PARTIAL FAIL** | Story frontmatter ↔ body BC table consistent, but BC body language drifts from story body (F-P3-001) |
| POLICY 9 | PASS | BC-5.39.004 VP table marked "(pending)" with deferred allocation per spec |
| POLICY 10 | PASS | No demo artifacts required (hook-only story) |
| POLICY 11 | PASS | Bats tests exercise real dispatcher round-trip; assertions on specific exit codes + block_reason substrings |
| POLICY 12 | PASS | Canonical TV table describes scenarios matching bats fixtures |
| POLICY 13 | PASS | REQUIRED_BLOCK_TOKENS defines all 9 tokens; bats fixtures exercise all 9 |
| POLICY 14 | PASS | Story + BC + impl describe same 3 structural properties consistently |
| POLICY 15 | N/A | No burst-log Dim-2 attestations in this diff (pre-merge) |
| POLICY 16 | PASS | All D-NNN anchors exist in v1.0-feature-engine-discipline-pass-1 decision-log |
| POLICY 17 | PASS | Story bcs:, body BC table, AC trace columns bidirectionally consistent |
| POLICY 18 | PASS | Story + BC input-hashes present |

## Part D: Verdict + Streak

**Verdict:** LOW (1 MEDIUM + 1 LOW finding).
**Streak after this pass:** 0/3 (LOW resets per BC-5.39.001).

**Implementation health:** Clean. Hook logic structurally correct; cross-crate parity exemplary; canonical-form sweep zero-regression; paper-fix detection PASSES on both prior closures (F-P2-001 production-registry, F-P2-004 unparseable-headline); production-grade default observed.

**Findings are spec/BC artifact alignment issues, not correctness defects in hook code.**

## Part E: Recommendations for Fix-Burst-3

1. **F-P3-001 (MEDIUM, product-owner):** Amend BC-5.39.004.md precondition 1 to path-component-strict language; BC-INDEX bump v2.27→v2.28.
2. **F-P3-002 (LOW, story-writer):** Enumerate integration-production-registry.bats in story spec File Structure Requirements; bump count 7→8; story spec v1.1→v1.2; STORY-INDEX v3.36→v3.37.

**Pass-4 expected disposition:** Both findings are local spec/BC text edits with mechanical fixes. Pass-4 should converge to NITPICK_ONLY or CLEAN. Subsequent passes 5-6 advance streak to 3-CLEAN.

**Process-gap watch:** Story-spec v1.1 amendment did not propagate to linked BC body; if pass-4 reveals same drift in sibling BCs across other M2 stories, escalate to [process-gap] codification of "BC-body MUST be swept when story-spec body is amended" discipline.
