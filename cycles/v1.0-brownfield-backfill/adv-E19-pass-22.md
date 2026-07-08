# Adversarial Review — E-19 Pass 22 (post-D-775 delta; perimeter = ADR-030 v1.1 + BC-5.42.001 v1.2 + S-19.01 v1.12)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section + ADR-030 + BC-5.42.001 + BC-1.17.001
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 1 / HIGH 1 / MEDIUM 2 / LOW 0 (4 findings; 5 observations)
**Streak:** 0/3 RESET (streak was 0/3 entering pass-22 per D-775)
**Model family:** Claude Opus 4.7
**Delta artifact versions verified:** ADR-030 v1.1 (was v1.0); BC-5.42.001 v1.2 (was v1.1); S-19.01 v1.12 (was v1.11).

## Part A — D-775 Delta Verification + New Findings

### Amendment 1 — ADR-030 v1.0 → v1.1 (F-W1V-001..004 interface drift fix)

§Script Path Canonical Location: bin/ confirmed for check-stale-verdict.sh and enforce-merge-strategy.sh ✓. §Invocation Signatures: positional forms `<pr_number> <covered_sha>` / `<pr_number> [flag]` ✓. §Error Taxonomy: READY_SHA_MISSING / READY_SHA_FETCH_FAILED / CHECK_STALE_VERDICT_ERROR / unclassified arms present ✓. POLICY 14 5-leg parity: version v1.1; last_amended; modified[]; body Changelog row; ARCH-INDEX row v1.0→v1.1 ✓.

**F-P22-003 MEDIUM — ADR-030 Decision 1 TOML tool filter insufficiently anchored.**

ADR-030 Decision 1 specifies the SubagentStop hook entry uses `tool = "^Agent"` in the TOML. This regex is unanchored at the end — it matches any tool name with prefix "Agent" (e.g., "AgentX", "AgentHelper", "AgentDispatch"). S-19.04 D-f convention (hooks-registry.toml tool-filter anchoring discipline) requires fully-anchored singleton form `^Agent$` to prevent substring matches on AgentX-style tool names. ADR-030 should cite `^Agent$` in its canonical Decision 1 TOML stanza. The production hooks-registry.toml uses the correct anchored form (ground truth); ADR-030 description text and example stanza lag behind.

**Locus:** ADR-030 Decision 1 TOML stanza + ADR-030 decision prose citing tool filter.
**Fix:** ADR-030 v1.1→v1.2: update tool filter to `^Agent$` (fully-anchored singleton per S-19.04 D-f convention).

### Amendment 2 — BC-5.42.001 v1.1 → v1.2 (hooks/ → bin/ path propagation)

§Architecture Anchors: `plugins/vsdd-factory/bin/check-stale-verdict.sh` and `plugins/vsdd-factory/bin/enforce-merge-strategy.sh` ✓. POLICY 14 5-leg parity ✓.

**F-P22-001 BLOCKER — BC-5.42.001 §Architecture Anchors WASM path mis-citation.**

BC-5.42.001 §Architecture Anchors sub-bullet for the WASM hook cites `hooks/pr-manager-completion-guard.wasm`. Ground truth: hooks-registry.toml `plugin` field specifies `hook-plugins/pr-manager-completion-guard.wasm` (the `hook-plugins/` directory, not `hooks/`). This is the same path-namespace that the D-775 fix corrected for the bin/ scripts, but the WASM plugin sub-bullet was not swept at the same time. S-19.01 §File Structure now correctly cites `hook-plugins/pr-manager-completion-guard.wasm` (post-v1.12) but BC-5.42.001 §Architecture Anchors retains the stale `hooks/` path. BC-spec anchor and story implementation spec disagree — BLOCKER under VSDD spec-wins rule.

**Locus:** BC-5.42.001 §Architecture Anchors, WASM plugin sub-bullet: `hooks/pr-manager-completion-guard.wasm`.
**Fix:** BC-5.42.001 v1.2→v1.3: update WASM path to `hook-plugins/pr-manager-completion-guard.wasm` per ground truth hooks-registry.toml plugin field.

### Amendment 3 — S-19.01 v1.11 → v1.12 (bin/ + positional + named codes + exit 1)

§Architecture Mapping + §File Structure: `plugins/vsdd-factory/bin/` paths ✓. §Invocation Signatures: positional forms ✓. §Error Taxonomy: 4-arm named codes ✓. input-hash 8ec7188 ✓. POLICY 14 5-leg parity ✓.

**F-P22-002 HIGH — STORY-INDEX E-19 BC-coverage summary stale cites.**

STORY-INDEX E-19 section contains two BC-coverage cite locations:
(1) E-19 epic summary paragraph (~line 685): cites `BC-1.17.001 v1.2 LANDED`. After BC-1.17.001 advanced to v1.3 (D-763 fix burst), this cite is stale.
(2) BC coverage footer (~line 702): cites `BC-5.42.001 v1.1 (S-19.01)` and `BC-1.17.001 v1.2 LANDED (S-19.06; read_prefix FFI)`. Both should be v1.3.

The D-775 fix burst updated S-19.01 input-hash and story row annotations but did not sweep the BC-coverage summary lines. F-P22-002 is a sibling-sweep miss of the STORY-INDEX-prose BC-cite class (D-768 / D-769 discipline).

**Locus:** STORY-INDEX.md E-19 section, BC-coverage summary lines 685 and 702.
**Fix:** State-manager: update all four stale version cites to current versions (BC-5.42.001 v1.3; BC-1.17.001 v1.3 at both line 685 and line 702).

### Full E-19 Epic and Story Suite Review

**F-P22-004 MEDIUM — BC-1.17.001 §Architecture Anchors missing ffi.rs raw wire-ABI bullet.**

BC-1.17.001 §Architecture Anchors cites `crates/hook-sdk/src/host.rs` (safe Rust wrapper) but omits `crates/hook-sdk/src/ffi.rs` (raw wire-ABI extern + host_stubs). S-19.06 AC-007 Gate 2 explicitly verifies ffi.rs: clause (i) checks pub safe fn read_prefix in host.rs; clause (ii) checks #[link(wasm_import_module="vsdd")] attr in ffi.rs raw ABI; clause (iii) checks cfg(not(wasm32)) stub in ffi.rs. The BC's §Architecture Anchors section must reflect the two-file layering it normatively establishes (SDK/wire-ABI layering per F-P12-002 Ruling-1 parenthetical). Omission creates an anchor gap: the BC normatively requires a two-layer implementation (host.rs + ffi.rs) but §Architecture Anchors cites only one layer.

**Locus:** BC-1.17.001 §Architecture Anchors sub-bullets.
**Fix:** BC-1.17.001 v1.2→v1.3: add ffi.rs raw wire-ABI bullet to §Architecture Anchors alongside existing host.rs bullet.

No other new findings on frozen stories S-19.02/S-19.03/S-19.04/S-19.05/S-19.07 or epic.

## Part B — Dimensions

| Dimension | Status |
|-----------|--------|
| Dim-1: BC/VP coverage | PARTIAL — F-P22-001 (BC-5.42.001 stale WASM path) + F-P22-004 (BC-1.17.001 missing ffi.rs anchor) |
| Dim-2: AC gate execution | Not re-run (frozen stories; no gate changes in D-775 delta) |
| Dim-3: POLICY 14 5-leg parity | PASS on all 3 D-775 delta amendments |
| Dim-4: POLICY 8 BC propagation | PASS |
| Dim-5: Input-hash consistency | PASS — S-19.01 8ec7188 matches post-D-775 compute |
| Dim-6: STORY-INDEX BC-coverage | FAIL — F-P22-002 two stale cite sites |
| Dim-7: ADR/BC interface parity | FAIL — F-P22-001 (BC anchor lags S-19.01 ground truth); F-P22-003 (ADR tool filter unanchored) |

## Observations

**O-P22-001 [process-gap]:** F-P22-001 (WASM path) and F-P22-002 (STORY-INDEX stale cites) are both sibling-sweep misses from the D-775 fix burst. D-775 applied the bin/ path correction to the script sub-bullets but did not sweep the adjacent WASM plugin sub-bullet in the same §Architecture Anchors section. The TD-VSDD-060 sibling-sweep discipline requires intra-section sweep when one sub-bullet in a list is corrected. Recommend: after any §Architecture Anchors edit, grep all sub-bullet paths in the same section against ground-truth registry before committing.

**O-P22-002:** ADR-030 Decision 1 TOML tool filter anchoring (F-P22-003) follows the existing S-19.04 D-f convention. The fix is mechanical (append `$` to `^Agent`). No architectural adjudication required.

**O-P22-003:** BC-1.17.001 v1.3 ffi.rs bullet addition (F-P22-004) is a documentation anchor gap, not a behavioral gap — S-19.06 AC-007 already gates on ffi.rs at multiple clauses. The BC body's two-layer layering parenthetical (F-P12-002 Ruling-1) is present; §Architecture Anchors just needs the second anchor entry to match.

**O-P22-004:** The hook-plugins/ vs hooks/ path class (F-P22-001) has now surfaced across ADR-030, BC-5.42.001, and S-19.01 in consecutive bursts. Consider: a corpus-wide grep for `hooks/.*\.wasm` (non-hook-plugins path) in specs and stories as a standing preflight gate before any wave-boundary dispatch.

**O-P22-005:** STORY-INDEX BC-coverage summary at lines 685 and 702 is a recurring drift location (D-768 established the STORY-INDEX-prose leg; D-769 codified sweep-count reconciliation). F-P22-002 is the second escape from this location. The STORY-INDEX-prose leg of the BC-cite preflight (D-768) should include explicit coverage of the BC-coverage footer and epic summary paragraph — not just per-story row annotations.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 1 |
| HIGH | 1 |
| MEDIUM | 2 |
| LOW | 0 |
| Observations | 5 |

Actionable findings: 4. D-776 fix burst applied (architect ADR-030 v1.1→v1.2; product-owner BC-5.42.001 v1.2→v1.3 + BC-1.17.001 v1.2→v1.3; story-writer S-19.01 v1.12→v1.13 + S-19.06 v1.13→v1.14 + epic v1.15→v1.16; state-manager STORY-INDEX stale BC cites F-P22-002).

**Overall Assessment:** NOT-CLEAN — 4 findings (B1/H1/M2/L0). Streak 0/3. NEXT: pass-23.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 22 |
| New findings | 4 |
| Duplicate/variant findings | 0 |
| Novelty score | 4/4 distinct classes |
| Median severity | MEDIUM |
| Novel observation | 5 |
| Verdict | NOT-CLEAN — streak 0/3; pass-23 NEXT |

## Coverage Attestation

Artifacts read in full: ADR-030 v1.1 (1-end); BC-5.42.001 v1.2 (1-end); BC-1.17.001 v1.2 (1-end); S-19.01 v1.12 (1-end); STORY-INDEX E-19 section (680-710); policies.yaml POLICY 14+17; hooks-registry.toml SubagentStop entry (ground truth for tool filter + plugin path).
Spot-checked (no changes): S-19.02 v1.9; S-19.03 v1.12; S-19.04 v1.11; S-19.05 v1.13; S-19.07 v1.8; E-19 epic v1.15.
Not read (Iron Law): decision-log; burst-log; lessons.md; STATE.md; checkpoints; adv passes 1-21; fix-burst records.

## Fix Burst Closure (D-776)

**Leg 1 — Architect:** ADR-030 v1.1→v1.2. Decision 1 TOML tool filter `^Agent` → `^Agent$` (fully-anchored singleton; S-19.04 D-f convention). POLICY 14 5-leg parity applied. **CLOSED F-P22-003.**

**Leg 2 — Product-owner (BC-5.42.001):** BC-5.42.001 v1.2→v1.3. §Architecture Anchors WASM plugin path `hooks/pr-manager-completion-guard.wasm` → `hook-plugins/pr-manager-completion-guard.wasm` per ground truth hooks-registry.toml plugin field. POLICY 14 5-leg parity applied. **CLOSED F-P22-001.**

**Leg 3 — Product-owner (BC-1.17.001):** BC-1.17.001 v1.2→v1.3. §Architecture Anchors: ffi.rs raw wire-ABI bullet added alongside existing host.rs bullet. POLICY 14 5-leg parity applied. **CLOSED F-P22-004.**

**Leg 4 — Story-writer:** S-19.01 v1.12→v1.13 (BC-5.42.001 v1.2→v1.3 cite propagation: §Architecture Mapping WASM path + input-hash 8ec7188→2a9f0b4); S-19.06 v1.13→v1.14 (BC-1.17.001 v1.2→v1.3 cite propagation 8 sites: Narrative 'The fix' paragraph, §Behavioral Contracts table Version cell, Token Budget, AC-007 Gate + BC Trace, §Architecture Compliance Rules); E-19 epic v1.15→v1.16 (BC-1.17.001 v1.2→v1.3 cite propagation 4 sites). POLICY 14 5-leg parity applied to all three. STORY-INDEX v4.153→v4.154 (S-19.01 + S-19.06 + epic row syncs).

**Leg 5 — State-manager:** STORY-INDEX stale BC-coverage summary cites updated: line 685 `BC-1.17.001 v1.2 LANDED` → `v1.3 LANDED`; line 702 `BC-5.42.001 v1.1` → `v1.3` + `BC-1.17.001 v1.2 LANDED` → `v1.3 LANDED`. **CLOSED F-P22-002.**

4-index after D-776: BC-INDEX v3.78 / VP-INDEX v2.53 UNCHANGED / STORY-INDEX v4.154 / ARCH-INDEX v2.92. Streak 0/3. **NEXT: pass-23.**
