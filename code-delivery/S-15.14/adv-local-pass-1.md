---
document_type: adversary-pass-report
level: ops
title: "S-15.14 LOCAL Adversary Cascade — Pass 1"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.14
pass: 1
verdict: CRITICAL
finding_count: { critical: 2, high: 5, medium: 4, low: 3, nitpick: 2, process_gap: 2 }
streak_3_clean: "0/3"
---

# S-15.14 LOCAL Adversary Cascade — Pass 1

## Part A — Findings

### F-P1-001 — CRITICAL (HIGH confidence) — False-positive: real STATE.md current_step has no `D-382..D-N` pattern → hook blocks every dispatch-advance write

- **Severity:** CRITICAL
- **Category:** spec-vs-artifact-reality drift / silent-inert-validator (META-LEVEL-class)
- **Location:** `crates/hook-plugins/validate-dispatch-advance/src/lib.rs:419-431` (`check_d_chain_currency`)
- **Evidence:** `/Users/jmagady/Dev/vsdd-factory/.factory/STATE.md` line 15: `current_step: "...trajectory-tail →9→9→9→9 (F5 cycle; unchanged); D-chain cite D-476 latest brownfield..."`. The current `current_step:` uses prose `D-chain cite D-476 latest brownfield` — NOT the `D-382..D-N` form. Implementation emits `Violation` "D-chain cite absent" when the literal substring `D-382..D-` is missing.
- **Issue:** On the very next state-manager dispatch-side advance that writes STATE.md, the hook will block. BC-5.39.006 PC-5 and invariant 7 require the `D-382..D-N` pattern, but production state-manager has never written `current_step:` in that exact form. The pattern is aspirational/wishful spec, not derived from real artifact.
- **Recommendation:** product-owner — relax BC-5.39.006 invariant 7 to detect `D-(\d+)` anywhere in current_step (taking the max integer found) AND amend pre-existing fixtures + tests accordingly. Then implementer applies the loosened detection.

### F-P1-002 — CRITICAL (HIGH confidence) — INDEX.md validator flags every existing cycle INDEX.md as having wrong column count

- **Severity:** CRITICAL
- **Category:** spec-vs-artifact-reality drift / over-broad validator scope
- **Location:** `crates/hook-plugins/validate-dispatch-advance/src/lib.rs:607-680` (`validate_index_md`)
- **Evidence:** Real `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/INDEX.md` line 17-23 adversary-pass rows have 4 columns. Real `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md` line 58+ has 5 columns. Implementation flags ALL table rows where `pipe_count != 7` — no semantic awareness of row class (adversary-pass vs story listing vs epic listing).
- **Issue:** Hook will emit BlockWithFix on every state-manager Commit A. BC-5.39.006 invariant 8 was written for "adversary-pass row" semantics, but implementation has no row-class discrimination. Additionally: the "6-column" mandate is not aligned with current artifact reality.
- **Recommendation:** product-owner + implementer. BC-5.39.006 invariant 8 must be tightened to identify adversary-pass rows specifically (h2 context: "## Adversarial Reviews" — detect via preceding h2 heading scan). Implementation must add h2-section state machine.

### F-P1-003 — HIGH (HIGH confidence) — BC-5.39.006 invariant 8 has arithmetic error contradicting implementation and tests

- **Severity:** HIGH
- **Category:** spec-internal-contradiction / paper-fix
- **Location:** BC-5.39.006 line 150; lib.rs:629-663 comments
- **Evidence:** BC-5.39.006 line 150: *"pipe_count is not 8 (i.e., 7 internal pipes = 6 columns + 2 border pipes)"*. Implementation contains 30+ lines of comments concluding "We follow the tests as the authoritative specification" (line 654). For `| c1 | c2 | c3 | c4 | c5 | c6 |` the actual pipe count is 7, not 8.
- **Issue:** Standing Rule violation. CLAUDE.md §"Source of Truth" rule 12: SPEC wins. The implementer paper-fixed via comments instead of routing to product-owner for BC amendment.
- **Recommendation:** product-owner — amend BC-5.39.006 invariant 8 to: `pipe_count is not 7 (i.e., 1 leading + 5 internal + 1 trailing = 7 pipes for 6 columns)`. Sweep S-15.14 story body and EC-013/EC-014 wording.

### F-P1-004 — HIGH (HIGH confidence) — STATE.md current_step extraction asymmetric fail behavior (extract-failure → fail-closed)

- **Severity:** HIGH
- **Category:** invariant-9 inconsistency
- **Location:** lib.rs:172-181, 553-585
- **Evidence:** If second `---` isn't found in extract_current_step, returns None, causing validate_state_md to emit violation "could not extract current_step: value". For in-progress writes where state-manager truncated the file mid-edit, this fail-loud behavior is OPPOSITE of BC-5.39.006 invariant 7/9 fail-open principle.
- **Issue:** read-error → Continue (fail-open); extract-failure → Block (fail-closed) is inconsistent.
- **Recommendation:** implementer — when extract_current_step returns None, emit Continue + log_warn consistent with invariant 9.

### F-P1-005 — HIGH (HIGH confidence) — Hook validates ALL INDEX.md table rows, not adversary-pass rows specifically

- **Severity:** HIGH
- **Category:** validator-scope-overreach
- **Location:** lib.rs:610-676
- **Evidence:** Real engine-discipline INDEX.md lines 35-45 story-listing rows (6 cols) pass; lines 49-51 epic-listing rows (3 cols) flagged. Compounds F-P1-002.
- **Recommendation:** implementer — add h2-section state machine: only validate rows inside `## Adversarial Reviews` h2 section.

### F-P1-006 — HIGH (HIGH confidence) — Slice panic risk in scan_max_d_nnn_in_body range-prefix branch

- **Severity:** HIGH
- **Category:** code-hygiene / sibling-sweep-incomplete
- **Location:** lib.rs:509
- **Evidence:** `search = &search[skip + range_terminal_len..];` lacks the `>= search.len()` guard present at line 528 in the sibling branch.
- **Recommendation:** implementer — add `if skip + range_terminal_len >= search.len() { break; }` guard, paralleling line 528 (TD-VSDD-060 sibling-site sweep).

### F-P1-007 — HIGH [process-gap] (HIGH confidence) — Single-commit instead of micro-commits per PC

- **Severity:** HIGH
- **Category:** [process-gap] TDD-micro-commit-discipline
- **Location:** commit 342364d3 (single 562/-54 commit)
- **Evidence:** Commit replaces 6 todo!() bodies in one atomic unit. Functions are independently testable (lib.rs:887-1073). Implementer's claim "test suite demanded coordinating functions" not borne out by structure.
- **Recommendation:** [process-gap] orchestrator — require implementer to chunk by PC for future hook stories.

### F-P1-008 — MEDIUM (HIGH confidence) — Spec drift: "We follow the tests as the authoritative specification" violates CLAUDE.md SPEC-wins rule

- **Severity:** MEDIUM
- **Category:** paper-fix / Standing Rule violation
- **Location:** lib.rs:654
- **Evidence:** Comment violates CLAUDE.md §"Architectural Authority" rule 12: code-vs-spec → spec wins.
- **Recommendation:** product-owner amends BC-5.39.006 invariant 8 (F-P1-003); implementer removes the comments.

### F-P1-009 — MEDIUM (HIGH confidence) — Workspace member order not alphabetical

- **Severity:** MEDIUM
- **Category:** sibling-discipline / story-spec violation
- **Location:** /Users/jmagady/Dev/vsdd-factory/.worktrees/S-15.14/Cargo.toml line 26-29
- **Evidence:** Alphabetical: `burst-log, dispatch-advance, index-cite-refresh, state-structure`. New entry inserted out of order. Violates S-15.14 story T-5 directive.
- **Recommendation:** implementer — reorder.

### F-P1-010 — MEDIUM (MEDIUM confidence) — Cargo.toml dev-dep duplicates dependency (sibling-drift carry-forward)

- **Severity:** MEDIUM
- **Category:** sibling-drift / template clarity
- **Location:** validate-dispatch-advance/Cargo.toml line 31-33
- **Evidence:** Re-declares `vsdd-hook-sdk` as dev-dep when already in [dependencies]. Sibling validate-state-structure has same pattern.
- **Recommendation:** architect adjudicates — is dev-dep duplication intentional template pattern?

### F-P1-011 — MEDIUM (MEDIUM confidence) — Test validate_production_state_md_no_false_positive silently skips on missing STATE.md

- **Severity:** MEDIUM
- **Category:** silent-inert-test (META-LEVEL-class)
- **Location:** lib.rs:1119-1124
- **Evidence:** `Err(_) => return;` — silent skip on Err means in CI the test always passes vacuously. Per F-P1-001, when STATE.md IS readable in worktree, the test SHOULD currently be failing — either passing falsely OR implementer never ran cargo test.
- **Recommendation:** implementer + test-writer — replace silent skip with eprintln! + fail-loud guard.

### F-P1-012 — LOW (HIGH confidence) — Forbidden meta-commentary returns first match; multi-violation enumeration precision gap

- **Severity:** LOW
- **Category:** within-check enumeration precision
- **Location:** lib.rs:231-291
- **Evidence:** Returns `Option<Violation>` (singular). BC-5.39.006 PC-6 suggests multi-violation enumeration; satisfied across-checks but not within-check for forbidden-meta.
- **Recommendation:** implementer (minor) — return Vec<Violation>.

### F-P1-013 — LOW [process-gap] (HIGH confidence) — Hook priority comment narrative-attests grep without literal evidence (D-449(a) META-LEVEL-24 self-application)

- **Severity:** LOW
- **Category:** [process-gap] persistence-layer-evidence
- **Location:** hooks-registry.toml line 925-926
- **Evidence:** Comment narrates `verified: 153 confirmed by grep` without captured stdout per D-449(a).
- **Recommendation:** [process-gap] state-manager — codify "registry priority allocation must cite literal grep evidence in inline comment".

### F-P1-014 — LOW (MEDIUM confidence) — Unused parameter `_terminal_n` in scan_max_d_nnn_in_body

- **Severity:** LOW
- **Category:** code-hygiene
- **Location:** lib.rs:492
- **Recommendation:** implementer — drop parameter or document reservation rationale.

### F-P1-015 — NITPICK — Stub-architect's GREEN-BY-DESIGN claim for path-guard functions verified correct

- **Category:** positive observation
- **Location:** lib.rs:112-117, 131-136
- Pure delegations to Path::file_name. BC-5.38.001 acceptable opt-out holds.

### F-P1-016 — NITPICK — Long single-line description in Cargo.toml (parity with sibling, not drift)

- **Category:** aesthetic / parity
- **Location:** validate-dispatch-advance/Cargo.toml line 9
- Sibling parity confirmed.

## Part B — Summary

**Verdict:** CRITICAL
**Counts:** 2 Critical + 5 High + 4 Medium + 3 Low + 2 Nitpick = 16 findings total
**Streak:** 0/3 → 0/3 (CRITICAL+HIGH findings reset)

**Top 3 most important:**
1. F-P1-001 — production STATE.md current_step does NOT contain `D-382..D-N`; hook would block every dispatch-advance write the moment it deploys.
2. F-P1-002 — INDEX.md validator flags every existing cycle INDEX.md table row; no row-class discrimination, no h2-section awareness.
3. F-P1-003 + F-P1-008 — BC-5.39.006 invariant 8 pipe arithmetic error; implementer paper-fixed via 30-line comment ("follow tests") instead of routing to product-owner for spec amendment (CLAUDE.md "SPEC wins" violation).

**Recommended fix-routing by category:**
- F-P1-001, F-P1-002, F-P1-003, F-P1-008 → product-owner (BC-5.39.006 spec amendments + paper-fix removal)
- F-P1-004, F-P1-005, F-P1-006, F-P1-009, F-P1-011, F-P1-012, F-P1-014 → implementer (semantic alignment + sibling discipline)
- F-P1-007 → [process-gap] orchestrator
- F-P1-010 → architect (dev-dep template adjudication)
- F-P1-013 → [process-gap] state-manager (codify registry priority literal-evidence pattern)

**Novelty:** HIGH — first pass, all findings are fresh-context substantive. F-P1-001 and F-P1-002 are existential deployment blockers.
