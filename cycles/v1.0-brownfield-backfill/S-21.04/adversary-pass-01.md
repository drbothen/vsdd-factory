---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-25T00:00:00Z
phase: 5
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/S-21.04-story-worktree-write-path-discipline.md"
pass: 1
previous_review: null
story: S-21.04
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 1849d6b3
reviewed_branch: feature/S-21.04-story-worktree-write-path-discipline
base_commit: 948f0fb1
date: 2026-07-25
---

# S-21.04 LOCAL Adversary Pass-1 — NOT-CLEAN

**Date:** 2026-07-25
**Story:** S-21.04 — story-worktree factory artifact write-path discipline and teardown preflight
**Pass:** 1 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B1 / H8 / M3 / L2 / NITPICK0 / OBS0
**Total findings:** 14 findings
**Reviewed diff:** HEAD 1849d6b3 on feature/S-21.04-story-worktree-write-path-discipline vs base 948f0fb1

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2104-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2104`: Story identifier (S-21.04 compact form)
- `P<PASS>`: Pass number (e.g., `P1`, `P2`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `F-S2104-P1-001` (BLOCKER finding, pass 1, first finding), `F-S2104-P1-014` (LOW finding, pass 1, last finding).

---

## Part B — New Findings (or all findings for pass 1)

### BLOCKER

#### F-S2104-P1-001 — Primary dispatch path in `_shared-context.md` does NOT propagate §G.1 preflight mandate

- **Severity:** BLOCKER
- **Category:** spec-fidelity / coverage-gap
- **Location:** `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` — §Spec-Path Discipline / §Write Discipline clause
- **Description:** The §G.1 preflight mandate is present in `step-g-cleanup.md` but NOT propagated to the primary agent dispatch path in `_shared-context.md`. The §Write Discipline clause added by this story describes write-path discipline for `.factory/**` artifacts but does not include a corresponding preflight gate callout for the teardown flow. Story-delivery agents that operate from `_shared-context.md` as their behavioral contract will proceed to `git worktree remove` without the §G.1 preflight guard, because the primary dispatch path references `step-g-cleanup.md` only for the cleanup step itself — it does not surface the mandatory preflight that must precede that step. The protection exists in the sub-document but is invisible from the primary dispatch surface.
- **Evidence:** `_shared-context.md` §Write Discipline clause added by commit `1849d6b3` covers `CANONICAL_FACTORY_ROOT` path resolution for write operations only. No corresponding teardown preflight callout or cross-reference to `step-g-cleanup.md §G.1` appears in the primary `_shared-context.md` dispatch surface. An agent reading only `_shared-context.md` has no indication that a preflight is required before teardown.
- **Proposed Fix:** Add an explicit teardown preflight mandate to `_shared-context.md` §Write Discipline (or a dedicated §Teardown Discipline subsection) that references `step-g-cleanup.md §G.1` and makes the BC-6.26.001 PC2 requirement visible from the primary dispatch surface.
- **Routing:** implementer (propagate the §G.1 mandate reference to primary dispatch paths in `_shared-context.md`)

---

### HIGH

#### F-S2104-P1-002 — Bats test assertions use document-token grep, not semantic execution parity

- **Severity:** HIGH
- **Category:** test-coverage-gap
- **Location:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` — T-001/T-002/T-003 assertion pattern
- **Description:** T-001/T-002/T-003 use `_assert_doc_marker` gates that verify token presence in the extracted §G.1 section (`find.*\.factory`, `PREFLIGHT BLOCKED`, etc.). While the `_extract_g1_section` awk gate extracts a section boundary, the assertions only confirm that certain tokens are present within that section. A partial implementation containing only comments with the required tokens would pass all three tests without implementing the actual PC2a/PC2b state machine. Tests verify documentation structure, not behavioral correctness.
- **Evidence:** `_assert_doc_marker` is called with token patterns against the extracted doc content. No bats test constructs a fixture, executes a script against it, and asserts on stdout/stderr/exit-status from live execution. All assertions are structural/textual.
- **Proposed Fix:** Extend bats tests to include behavioral assertions: (a) invoke a real shell function or script excerpt against a fixture with a stray file and assert `PREFLIGHT BLOCKED` appears in stdout and `git worktree remove` is NOT called; (b) invoke against a clean fixture and assert `git worktree remove` IS called (or a sentinel confirming the clean path).
- **Routing:** test-writer (behavioral execution parity assertions)

---

#### F-S2104-P1-003 — AC-006 (PC2c fail-closed) has no test vector

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` — missing T-004 class test
- **Description:** BC-6.26.001 PC2c mandates that non-path-absent `find` errors (e.g., permission denied, path traversal failure) cause teardown to HALT and surface the exact `find` exit code and stderr to the operator. AC-006 codifies this requirement. The bats suite covers PC2a (clean path, T-002) and PC2b (stray-file blocked, T-001/T-003) but has no test vector that injects a `find` non-path-absent error and verifies the HALT branch fires with the required exit code and stderr surface. AC-006 is untested.
- **Evidence:** Bats suite at commit `8e3c432e` contains 3 tests (T-001/T-002/T-003); grep for "permission\|exit.*code\|PC2c\|AC-006\|non-path" in the bats file returns no matches.
- **Proposed Fix:** Add T-004: inject a fixture where `find` exits non-zero for a non-path-absent reason (e.g., via a wrapper that fakes a permission-denied error); assert teardown HALTS and the exit code + stderr are surfaced. Verify `git worktree remove` is NOT called.
- **Routing:** test-writer (PC2c/AC-006 test vector)

---

#### F-S2104-P1-004 — No exit-status assertions — behavioral gate invisible to test suite

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` — all test assertions
- **Description:** T-001/T-002/T-003 assert via `_assert_doc_marker` (document structural presence) but do not assert the shell exit code of the preflight execution. If an implementation emits `PREFLIGHT BLOCKED` to stdout but exits 0 (allowing `git worktree remove` to proceed), all tests would still pass (or fail for structural reasons unrelated to the behavioral defect). The behavioral consequence — HALT vs proceed — is not verified by any exit-status assertion.
- **Evidence:** No `run` + `[ "$status" -eq N ]` pattern in the bats suite. All assertions are textual/structural.
- **Proposed Fix:** Add exit-status assertions alongside doc-marker assertions: BLOCKED path → non-zero exit; clean path → zero exit. These are the observable behavioral consequences that the test suite must enforce.
- **Routing:** test-writer (exit-status assertions)

---

#### F-S2104-P1-005 — Primary-path parity gap: `_shared-context.md` §Write Discipline has no behavioral test

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` — AC-001 coverage
- **Description:** T-001/T-002/T-003 test the §G.1 path in `step-g-cleanup.md` only. There are no bats tests asserting that `_shared-context.md §Write Discipline` requires agents to use `CANONICAL_FACTORY_ROOT` or `git -C <main-worktree> rev-parse --show-toplevel` for `.factory/**` writes (AC-001, PC1). AC-001 coverage is doc-parity only (a structural token check if present), not a behavioral execution test verifying that write-path resolution functions correctly.
- **Evidence:** Bats suite has no fixture or assertion targeting `_shared-context.md §Write Discipline` behavioral execution. AC-001 is not exercised in the bats test matrix.
- **Proposed Fix:** Add at least one bats test exercising the `_shared-context.md §Write Discipline` clause: verify that a simulated agent write using `CANONICAL_FACTORY_ROOT` routes to the main-checkout `.factory/` mount, not to a CWD-relative shadow path.
- **Routing:** test-writer (primary-path parity test for AC-001)

---

#### F-S2104-P1-006 — `2>/dev/null` on `find` makes PC2c structurally unreachable; PC2c HALT clause absent

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1` — `find` invocation
- **Description:** The `find <worktree-path>/.factory -type f 2>/dev/null` invocation suppresses all `find` error output. BC-6.26.001 PC2c requires non-path-absent `find` errors to trigger a HALT with the exact exit code and stderr surfaced to the operator. With `2>/dev/null`, all `find` errors (permission denied, path traversal failure, etc.) are silently discarded and the `find` exit code is masked by the shell pipeline. The PC2c path is structurally unreachable — no non-path-absent error can ever reach the HALT branch. Additionally, the §G.1 block contains no PC2c HALT clause at all; even removing `2>/dev/null` would leave the HALT behavior unimplemented.
- **Evidence:** `step-g-cleanup.md §G.1` `find` command: `find <worktree-path>/.factory -type f 2>/dev/null`. No PC2c HALT clause in the §G.1 block.
- **Proposed Fix:** (a) Remove `2>/dev/null` from the `find` invocation. (b) Add an explicit PC2c HALT clause: after `find` exits non-zero AND `.factory/` directory is present (i.e., non-path-absent error), emit the exit code + stderr and abort teardown without calling `git worktree remove`.
- **Routing:** implementer (remove error suppression + add PC2c HALT clause)

---

#### F-S2104-P1-007 — `_shared-context.md §Context` false claim: `git worktree add` creates shadow `.factory/` directory

- **Severity:** HIGH
- **Category:** spec-fidelity (architectural false premise)
- **Location:** `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md §Context` + ADR-031 §Context
- **Description:** `_shared-context.md §Context` (added by commit `1849d6b3`) states that `git worktree add` creates a shadow `.factory/` directory in the new story worktree. This is architecturally false: `.factory/` is gitignored on the product branch, so `git worktree add` does NOT check out `.factory/` content. No shadow directory is created at worktree-creation time. The shadow directory arises only when a story-delivery agent performs a CWD-relative Write that lands in the worktree's `.factory/` subtree. The false-provenance claim misrepresents the root cause and could mislead recovery operators.
- **Evidence:** `.gitignore` on product branch contains `.factory/`; `git worktree add` populates only tracked files. The shadow directory is created by the errant CWD-relative Write itself — this is the mechanism described in BC-6.26.001 Invariant 5 ("gitignored-shadow false-negative").
- **Proposed Fix:** Correct `_shared-context.md §Context` to state: `git worktree add` checks out nothing for gitignored `.factory/`; the shadow directory is created by the errant CWD-relative Write. Same correction required in ADR-031 §Context (architect routing for ADR-031).
- **Routing:** story-writer (story spec provenance correction in `_shared-context.md`); architect (ADR-031 §Context correction → ADR-031 v1.8→v1.9)

---

#### F-S2104-P1-008 — Sibling-site sweep gap: 5 teardown sites in `_shared-context.md` invoke `git worktree remove` without §G.1 preflight

- **Severity:** HIGH
- **Category:** coverage-gap (TD-VSDD-060 sibling-site sweep)
- **Location:** `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` — multiple `git worktree remove` call sites
- **Description:** The §G.1 preflight mandate was added to `step-g-cleanup.md §G.1` only. A TD-VSDD-060 sibling-site sweep of `_shared-context.md` reveals 5 additional locations where `git worktree remove` is invoked or referenced without routing through the §G.1 preflight gate. Agents following any of these 5 paths will proceed directly to `git worktree remove` without executing the PC2a/PC2b/PC2c preflight checks. The story's protection is localized to one dispatch path while 5 others remain unguarded.
- **Evidence:** `grep -n "git worktree remove" _shared-context.md` → 5 matches outside `step-g-cleanup.md §G.1` path. None cross-reference the §G.1 preflight.
- **Proposed Fix:** For each of the 5 unguarded call sites: either (a) add a mandatory §G.1 preflight sub-step before the `git worktree remove` call, or (b) redirect to `step-g-cleanup.md §G.1` with an explicit "MUST run §G.1 preflight" cross-reference.
- **Routing:** implementer (sibling-site sweep for 5 unguarded `git worktree remove` call sites)

---

#### F-S2104-P1-009 — `red-gate-log.md` contains fabricated RG-004/RG-005, mis-mapped T-001→RG-003, and AC-002 attribution error

- **Severity:** HIGH
- **Category:** attestation-defect
- **Location:** `cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md` — Bats Tests table, Traces section, Hand-Off task 1
- **Description:** The red-gate-log contains three compounded defects: (1) T-001 is mapped to `RG-003` in the Bats Tests table and Traces section — the correct story RG ID for this test is `RG-001`; (2) T-002 maps to `RG-004` and T-003 maps to `RG-005` — these RG IDs do not exist in the story's Red Gate Test Plan (story v1.3 defines `RG-001`, `RG-002`, `RG-003` only; `RG-004` and `RG-005` are fabricated); (3) Hand-Off task 1 attributes "Unblocks AC-001/AC-002" to `_shared-context.md` amendments — AC-002 (mandatory teardown preflight sub-step) is defined against `step-g-cleanup.md §G.1`, not `_shared-context.md`. These defects create a false audit trail linking tests to non-existent requirements and misattribute AC scope.
- **Evidence:** red-gate-log.md Bats Tests table: `T-001 | AC-003 / RG-003`; `T-002 | AC-004 / RG-004`; `T-003 | AC-005 / RG-005`. Story v1.3 Red Gate Test Plan: `RG-001` (PC2b stray-file), `RG-002` (PC2a clean-path), `RG-003` (PC2b→PC2a retry). Hand-Off task 1: "Unblocks AC-001/AC-002." BC-6.26.001 AC-002 definition: `step-g-cleanup.md contains a mandatory teardown preflight sub-step`.
- **Proposed Fix:** Append an erratum section to red-gate-log.md (do NOT silently rewrite): correct T-001→RG-001, T-002→RG-002, T-003→RG-003; remove fabricated RG-004/RG-005 references; correct Hand-Off task 1 to "Unblocks AC-001." (AC-002 unblocked by Hand-Off task 2).
- **Routing:** state-manager (attestation record correction via erratum append)

---

### MEDIUM

#### F-S2104-P1-010 — `step-g-cleanup.md §G.1` clean-path sub-clause uses `git worktree remove --force` (BC mandate violation)

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1` — PC2a clean-path teardown command
- **Description:** The PC2a clean-path sub-clause in §G.1 invokes `git worktree remove --force` when the preflight result is empty. BC-6.26.001 PC2a mandates plain `git worktree remove` (no `--force`). The `--force` flag bypasses git's built-in unclean-worktree protection for non-gitignored untracked files — it is a BC-prohibited flag class. The rationale in the original commit claims `--force` "guards against dirty worktree state", but this inverts the actual semantics: `--force` removes the protection, it does not add it.
- **Evidence:** `step-g-cleanup.md §G.1` PC2a clause: `git worktree remove --force <worktree-path>`. BC-6.26.001 PC2a: "proceeds with plain `git worktree remove` (no `--force`; `--force` is prohibited by BC-6.26.001 as a BC mandate)".
- **Proposed Fix:** Replace `git worktree remove --force` with `git worktree remove` (no `--force`) in the PC2a clean-path clause. Correct the rationale: `--force` is prohibited because it strips git's unclean-worktree protection, not because it provides it.
- **Routing:** implementer (--force removal + rationale correction)

---

#### F-S2104-P1-011 — PC2a sub-cases (a) and (b) not distinguished; `.factory/` absent treated same as PC2c error

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1` — PC2a clause
- **Description:** `step-g-cleanup.md §G.1` PC2a clause does not distinguish between: (a) `.factory/` directory absent entirely (`find` exits non-zero with "No such file or directory" — expected clean state; proceed with teardown), and (b) `.factory/` directory present but `find` returns empty result (`find` exits 0, no files — proceed with teardown). Without this distinction, an implementation that does not explicitly handle the absent-directory case may treat PC2a sub-case (a) as a PC2c error (non-path-absent `find` non-zero exit), falsely triggering the HALT path when `.factory/` simply does not exist (the expected clean state for most story worktrees).
- **Evidence:** §G.1 PC2a clause: handles `find` returning empty result but does not document the sub-case (a) absent-directory exit behavior. BC-6.26.001 AC-002(a): "absent `.factory/` dir is PC2a sub-case (a), not a PC2c error".
- **Proposed Fix:** Explicitly document PC2a sub-case (a) in §G.1: "if `find <worktree>/.factory` exits non-zero because `.factory/` directory does not exist, treat as PC2a sub-case (a) — no stray files, teardown authorized; this is NOT a PC2c error." Add the path-absent exit-code recognition check (e.g., `[ -d <worktree>/.factory ]` guard before treating `find` non-zero as PC2c).
- **Routing:** implementer (PC2a sub-case (a) explicit handling)

---

#### F-S2104-P1-013 — `CANONICAL_FACTORY_ROOT` fallback resolution chain undefined for agents running in story worktree

- **Severity:** MEDIUM
- **Category:** spec-fidelity (completeness)
- **Location:** `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md §Write Discipline`
- **Description:** The §Write Discipline clause mandates using `CANONICAL_FACTORY_ROOT` or `git -C <main-worktree> rev-parse --show-toplevel` for `.factory/**` write-path resolution. However, the clause does not define how `<main-worktree>` is identified when the calling agent is running inside a story worktree (the primary use case for this protection). An agent running from `.worktrees/S-21.04/` cannot trivially resolve the main worktree path without additional context. If `CANONICAL_FACTORY_ROOT` is not set in the dispatch environment, the fallback `git -C <main-worktree>` invocation is circular: the agent must already know `<main-worktree>` to resolve `CANONICAL_FACTORY_ROOT`.
- **Evidence:** §Write Discipline clause: "use `CANONICAL_FACTORY_ROOT` or `git -C <main-worktree> rev-parse --show-toplevel`" — no definition of how to determine `<main-worktree>` from within a story worktree context. No fallback chain specified.
- **Proposed Fix:** Extend the §Write Discipline clause with an explicit fallback resolution chain: (1) prefer `CANONICAL_FACTORY_ROOT` environment variable (injected at dispatch time); (2) fallback: `git worktree list --porcelain` from the story worktree to locate the main worktree path; (3) if all else fails, error and surface to operator.
- **Routing:** story-writer (clarify `CANONICAL_FACTORY_ROOT` fallback resolution chain in §Write Discipline)

---

### LOW

#### F-S2104-P1-012 — CHANGELOG `[Unreleased]` entry missing for write-path discipline + teardown preflight

- **Severity:** LOW
- **Category:** delivery-completeness
- **Location:** `CHANGELOG.md` — `[Unreleased]` section
- **Description:** No CHANGELOG entry exists for the write-path discipline and teardown preflight feature delivered by S-21.04. Per project convention (CHANGELOG task added to story template at aa594c9a), every story delivery MUST add a CHANGELOG `[Unreleased]` entry describing the new behavior.
- **Evidence:** `CHANGELOG.md [Unreleased]` section does not contain any reference to S-21.04, write-path discipline, teardown preflight, `step-g-cleanup.md §G.1`, or `_shared-context.md §Write Discipline`.
- **Proposed Fix:** Add a CHANGELOG `[Unreleased]` entry: "Add write-path discipline and teardown preflight to `deliver-story` skill — all `.factory/**` writes must use absolute paths via `CANONICAL_FACTORY_ROOT`; `step-g-cleanup.md §G.1` preflight blocks `git worktree remove` when stray `.factory/` content is detected (BC-6.26.001, S-21.04)."
- **Routing:** implementer (CHANGELOG entry)

---

#### F-S2104-P1-014 — Story v1.3 internal BC cites still reference BC-6.26.001 v1.3 after PC2c amendment

- **Severity:** LOW
- **Category:** cross-document consistency (TD-VSDD-091)
- **Location:** `S-21.04-story-worktree-write-path-discipline.md` — BC table, Token Budget, narrative cites
- **Description:** The story was authored targeting BC-6.26.001 v1.3. The implementation exposes a gap that triggered BC-6.26.001 amendment to v1.4 (PC2c fail-closed clause added; AC-006 added; EC-005 updated). After this amendment, story v1.3 internal BC-version cites (BC table, Token Budget, narrative clauses) still reference v1.3, creating version-drift between the story's behavioral-contract table and the canonical BC document.
- **Evidence:** Story v1.3 BC table row: `| BC-6.26.001 | ... | v1.3 | ...`; Token Budget: `BC-6.26.001 v1.3`. BC document: amended to v1.4 (PC2c clause, AC-006). Story internal cites lag by one version.
- **Proposed Fix:** Update story internal BC-version cites (BC table, Token Budget, narrative references) from v1.3 to v1.4 to match the canonical BC document post-amendment.
- **Routing:** story-writer (BC cite propagation v1.3→v1.4 in story BC table + Token Budget)

---

## Fix-Burst Routing Table

| Finding | Severity | Routing | Status |
|---------|----------|---------|--------|
| F-S2104-P1-001 | BLOCKER | implementer | CLOSED — commit a65df476 |
| F-S2104-P1-002 | HIGH | test-writer | CLOSED — commit 7d38b9e6 |
| F-S2104-P1-003 | HIGH | test-writer | CLOSED — commit 7d38b9e6 |
| F-S2104-P1-004 | HIGH | test-writer | CLOSED — commit 7d38b9e6 |
| F-S2104-P1-005 | HIGH | test-writer | CLOSED — commit 7d38b9e6 |
| F-S2104-P1-006 | HIGH | implementer | CLOSED — commit 19271a65 |
| F-S2104-P1-007 | HIGH | story-writer + architect | CLOSED — story 0a82dbb2; ADR-031 v1.9 (architect) |
| F-S2104-P1-008 | HIGH | implementer | CLOSED — commit d3546a77 |
| F-S2104-P1-009 | HIGH | state-manager | CLOSED — erratum append D-895 (this burst) |
| F-S2104-P1-010 | MEDIUM | implementer | CLOSED — commit 19271a65 |
| F-S2104-P1-011 | MEDIUM | implementer | CLOSED — commit 19271a65 |
| F-S2104-P1-012 | LOW | implementer | CLOSED — commit 2d6297da |
| F-S2104-P1-013 | MEDIUM | story-writer | CLOSED — commit 0a82dbb2 |
| F-S2104-P1-014 | LOW | story-writer | CLOSED — commit 0594e2c2 (story v1.3→v1.4; BC-INDEX v1.3→v1.4) |

**Worktree HEAD after all fix legs:** `2d6297da`
**BC-6.26.001:** v1.3→v1.4 (story-writer commit 0594e2c2; PC2c fail-closed + AC-006 added)
**ADR-031:** v1.8→v1.9 (architect; §Context shadow-directory false-premise corrected)

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 1 |
| HIGH     | 8 |
| MEDIUM   | 3 |
| LOW      | 2 |
| NITPICK  | 0 |
| OBS      | 0 |

**Overall Assessment:** NOT-CLEAN
**Convergence:** findings remain — iterate
**Streak:** 0/3 (BC-5.39.001 3-CLEAN protocol)

**Recommended fix burst dispatch (concurrent):**
1. **implementer** — F-001 (primary dispatch path), F-006 (2>/dev/null + PC2c HALT), F-008 (sibling sweep 5 sites), F-010 (--force removal), F-011 (PC2a sub-cases), F-012 (CHANGELOG)
2. **test-writer** — F-002 (semantic execution parity), F-003 (PC2c/AC-006 test vector), F-004 (exit-status assertions), F-005 (primary-path parity)
3. **story-writer** — F-007 (story provenance correction), F-013 (CANONICAL_FACTORY_ROOT fallback chain), F-014 (BC cite propagation v1.3→v1.4)
4. **architect** — F-007 (ADR-031 §Context correction v1.8→v1.9)
5. **state-manager** — F-009 (red-gate-log erratum append)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 14 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (14 / (14 + 0)) |
| **Median severity** | HIGH |
| **Trajectory** | 14 |
| **Verdict** | FINDINGS_REMAIN |
