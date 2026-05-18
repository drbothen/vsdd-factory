---
document_type: adversary-pass-report
level: ops
title: "S-15.14 LOCAL Adversary Cascade — Pass 2"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.14
pass: 2
verdict: HIGH
finding_count: { critical: 0, high: 3, medium: 3, low: 2, nitpick: 1, process_gap: 2 }
streak_3_clean: "0/3"
---

# S-15.14 LOCAL Adversary Cascade — Pass 2

## Part A — Findings

### F-P2-001 — HIGH (HIGH confidence) — TD-VSDD-064 and TD-VSDD-065 IDs collide with existing lessons.md entries (POLICY 1 append_only_numbering violation)

- **Severity:** HIGH
- **Category:** policy-violation / paper-fix (rename without ID-uniqueness check)
- **Location:** `.factory/cycles/v1.0-brownfield-backfill/lessons.md:360,378` (pre-existing); `.factory/STATE.md:294-295` (new); `.factory/cycles/v1.0-brownfield-backfill/lessons.md:1626,1649` (new)
- **Evidence:** lessons.md:360 says `File as TD-VSDD-064 (Parallel-burst commit collision prevention rule).` (2026-05-05 from D-249/D-250). lessons.md:378 says `File as TD-VSDD-065 (Decision-ID outbound semantic-anchor check).` (2026-05-05 from D-251). The S-15.14 fix-burst REUSED these IDs for entirely different lessons (TDD micro-commit + Registry priority literal-evidence).
- **Issue:** POLICY 1 (append_only_numbering, severity HIGH) — IDs not retired; both LIVE codifications. Future grep for `TD-VSDD-064` surfaces 4 distinct semantic concepts.
- **Recommendation:** state-manager — re-allocate to TD-VSDD-095 + TD-VSDD-096 (next available; max in .factory/ is TD-VSDD-094). Sweep STATE.md:294-295, lessons.md:1626/1630/1649/1653 + all cross-references.

### F-P2-002 — HIGH (HIGH confidence) — S-15.14 pass-1 fix-burst has NO burst-log entry (D-444(c) 8-block gate violation)

- **Severity:** HIGH
- **Category:** structural-process-gap / D-444(c) + D-446(a) violation
- **Location:** `.factory/cycles/v1.0-brownfield-backfill/burst-log.md` — only h2 is `## Extracted from STATE.md on 2026-05-06`
- **Evidence:** `grep -n '^## ' .factory/cycles/v1.0-brownfield-backfill/burst-log.md` returns one line. STATE.md narratively claims fix-burst CLOSED but burst-log SoT is empty for this entry.
- **Issue:** D-444(c) requires 8 blocks per entry (Parent-commit, Adversary verdict, Files touched, Codifications, Dim-2/5/6/7 Attestations, Closes, Factory-artifacts commits). D-446(a) own-burst-log 8-block gate cannot have been INVOKED.
- **Recommendation:** state-manager — append `## S-15.14-pass-1-fix-burst` h2 entry with all 8 D-444(c) blocks. Invoke D-446(a) gate via literal shell per D-449(a).

### F-P2-003 — HIGH (HIGH confidence) — hooks-registry.toml description cites "non-6-column" contradicting BC v1.1 5-column amendment

- **Severity:** HIGH
- **Category:** content-defect / spec-vs-artifact drift after fix-burst (TD-VSDD-060 sibling-sweep miss)
- **Location:** `plugins/vsdd-factory/hooks-registry.toml:923-924`
- **Evidence:** "non-6-column adversary-pass rows" but BC v1.1 specifies 5-column / 6-pipe per D-442(b). Cargo.toml:9 correctly says 5-column.
- **Recommendation:** implementer — edit hooks-registry.toml:923-924 "non-6-column" → "non-5-column"; sibling-sweep all artifacts.

### F-P2-004 — MEDIUM (HIGH confidence) — hooks-registry.toml priority comment retains narrative without literal stdout (F-P1-013 paper-fix)

- **Severity:** MEDIUM
- **Category:** paper-fix (TD-VSDD-059 — lesson codified but artifact unchanged)
- **Location:** `plugins/vsdd-factory/hooks-registry.toml:925-926`
- **Evidence:** Line still: `# Priority 154 = one above validate-state-structure's 153 (verified: 153 confirmed by grep; 154 is unoccupied — next used priority above is 155).` Lesson PG-S-15.14-registry-priority-literal-evidence codified but offending comment NOT updated.
- **Recommendation:** implementer — replace narrative with literal grep stdout.

### F-P2-005 — MEDIUM (HIGH confidence) — lint-registry-async-invariant dev-dep duplication NOT swept (F-P1-010 sibling-sweep incomplete)

- **Severity:** MEDIUM
- **Category:** sibling-sweep-incomplete (TD-VSDD-060)
- **Location:** `crates/hook-plugins/lint-registry-async-invariant/Cargo.toml:31`
- **Evidence:** Line 23 declares vsdd-hook-sdk in [dependencies]; line 31 duplicates in [dev-dependencies]. Fix-burst swept 4 hook-plugins; missed this 5th. Also missing crate-type = ["cdylib", "rlib"].
- **Recommendation:** implementer — remove dev-dep duplicate; add crate-type for sibling parity (confirmed same template class per orchestrator inspection).

### F-P2-006 — MEDIUM (MEDIUM confidence) — `validate_production_state_md_no_false_positive` silently skips in worktree mode

- **Severity:** MEDIUM
- **Category:** silent-inert-test pattern in dev cycle (F-P1-011 partial fix)
- **Location:** `crates/hook-plugins/validate-dispatch-advance/src/lib.rs:1337-1371, 1376-1408`
- **Evidence:** Path resolution: manifest_dir + ../../../.factory/STATE.md from worktree → .worktrees/S-15.14/.factory/STATE.md (not mounted). Err branch always executes locally. CI=true guard only fires in GitHub Actions.
- **Issue:** Implementer's "real STATE.md returns Continue" claim cannot be verified in worktree. Cargo test reports 33/33 pass via vacuous skip.
- **Recommendation:** implementer — detect worktree mode and resolve via git rev-parse to main repo's .factory/STATE.md, OR add committed fixture, OR fail-loud locally (not just CI).

### F-P2-007 — LOW (MEDIUM confidence) — scan_max_d_nnn may pick D-NNN from banner narrative; no current-step-own-value exclusion

- **Severity:** LOW
- **Category:** within-check enumeration precision / spec interpretation gap
- **Location:** `crates/hook-plugins/validate-dispatch-advance/src/lib.rs:455-471`
- **Evidence:** scan_max_d_nnn scans full STATE.md content. Banner-comment forward-references (e.g., predicted D-500) would trigger false-positive block. No current production false-positive (D-476 caps).
- **Recommendation:** product-owner — clarify whether banner-narrative D-NNN should be excluded.

### F-P2-008 — LOW [process-gap] (HIGH confidence) — F-P1-007 codified as lesson but NOT structurally enforced; recurrence likely

- **Severity:** LOW
- **Category:** [process-gap] codification-without-automation (META-LEVEL-24 self-application)
- **Location:** PG-S-15.14-tdd-micro-commit-discipline at lessons.md:1626
- **Evidence:** Lesson is prose; no hook/gate. Next hook-implementer dispatch will not see it unless dispatch package incorporates rule.
- **Recommendation:** [process-gap] orchestrator — update implementer agent prompt or dispatch skill to require commit-cardinality-by-PC for multi-PC stories.

### F-P2-009 — NITPICK (HIGH confidence) — BC-5.39.006 Preconditions enumeration out-of-order

- **Severity:** NITPICK
- **Category:** spec-aesthetic / readability
- **Location:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md:62-81`
- **Evidence:** PC numbering reads 1, 5, 2, 3, 4. V1.1 amendment inserted PC5 in wrong slot.
- **Recommendation:** product-owner — renumber on next BC amendment.

## Part B — Summary

**Verdict:** HIGH
**Counts:** 0 Critical + 3 High + 3 Medium + 2 Low + 1 Nitpick = 9 findings + 2 [process-gap] tags
**Streak:** 0/3 → 0/3 (3 HIGH findings reset)

**Top 3 most important:**
1. F-P2-001 — TD-VSDD-064/065 ID collision (POLICY 1 violation; 4 distinct semantic concepts under 2 IDs).
2. F-P2-002 — Pass-1 fix-burst has no burst-log entry; D-444(c) gate structurally cannot have run.
3. F-P2-003 — hooks-registry.toml comment contradicts BC v1.1 schema (TD-VSDD-060 sibling-sweep miss).

**Recommended fix-routing by category:**
- F-P2-001, F-P2-002 → state-manager (ID re-allocation; burst-log append)
- F-P2-003, F-P2-004, F-P2-005, F-P2-006 → implementer (in-scope edits; sibling sweep)
- F-P2-007 → product-owner (spec scope clarification)
- F-P2-008 → orchestrator [process-gap] (implementer dispatch package update)
- F-P2-009 → product-owner (nitpick PC renumber)

**Pass-1 verification matrix:**
- CLOSED (9/16): F-P1-001, F-P1-002, F-P1-004, F-P1-005, F-P1-006, F-P1-008, F-P1-009, F-P1-012, F-P1-014
- PARTIAL-FIX (5/16): F-P1-003 (F-P2-003), F-P1-010 (F-P2-005), F-P1-010-SIDECAR, F-P1-011 (F-P2-006), F-P1-013 (F-P2-004 paper-fix)
- CLOSED-VIA-CODIFICATION-ONLY: F-P1-007 (no structural gate; F-P2-008 process-gap)
- NEW-CASE-DISCOVERED: F-P2-001 (TD ID collision)
- REGRESSION-INTRODUCED: F-P2-002 (missing burst-log entry — structurally novel)
- N/A (positive): F-P1-015, F-P1-016

**Novelty:** MEDIUM-HIGH — F-P2-001 + F-P2-002 are genuinely new fresh-context findings. F-P2-003/004/005/006 surface sibling-sweep gaps from pass-1 fixes.
