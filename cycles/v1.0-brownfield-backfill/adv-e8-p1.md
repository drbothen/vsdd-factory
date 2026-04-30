---
document_type: adversarial-review-pass
target: .factory/stories/epics/E-8-native-wasm-migration.md
target_version_reviewed: "1.0"
target_version_after_fix_burst: "1.1"
pass_label: ADV-E8-P1
pass_number: 1
date: 2026-04-30
adversary_session: a6a43e0985ef0d0ff
consistency_validator_session: a150cf8b78bbe3855
verdict: SUBSTANTIVE
clock: 0_of_3
findings_total: 18
findings_substantive: 18
findings_critical: 0
findings_high: 12
findings_medium: 6
findings_low: 0
findings_nitpick: 0
fix_burst_status: COMPLETE
fix_burst_session: a6d7bc98c3fa0ceec
---

# ADV-E8-P1 — Adversarial Review Pass 1: E-8 Native WASM Migration Epic

## Verdict
SUBSTANTIVE — pass-1 baseline; 18 findings (12 HIGH + 6 MED). Trajectory: pass-1=18 baseline.

## Trajectory
| Pass | Findings | Verdict | Clock |
|------|---------|---------|-------|
| 1 | 18 (12H+6M) | SUBSTANTIVE | 0_of_3 |

## Pass-1 Findings (full enumeration)

### F-001 [HIGH] CAP-003 anchor wrong — should be CAP-022
**Policy:** POLICY 4 (semantic_anchoring_integrity), POLICY 7
**Location:** frontmatter line 8 + body table line 33-34
**Evidence:** prd_capabilities: [CAP-002, CAP-003] but real CAP-003 is "Stream observability events to multiple configurable sinks" (not WASM execution). Real CAP-022 = "Port hook plugins from bash to native WASM" (capabilities.md:150-153) is the literal anchor.
**Recommendation:** Replace CAP-003 → CAP-022; consider also adding CAP-008 (PreToolUse), CAP-013 (PostToolUse).
**Fix-burst resolution:** v1.1 frontmatter `prd_capabilities: [CAP-002, CAP-008, CAP-013, CAP-022]`

### F-002 [HIGH] CAP-002 title mis-quoted
**Policy:** POLICY 4, POLICY 7
**Location:** body table line 33
**Evidence:** Body says "Cross-platform hook execution without platform-native shell dependency" but canonical H1 (capabilities.md:32) is "Hook Claude Code tool calls and session/worktree lifecycle events with sandboxed WASM plugins"
**Fix-burst resolution:** Body table uses verbatim canonical H1.

### F-003 [HIGH] D-3 Tier 2 internal arithmetic contradiction (21 vs 22 vs 23)
**Location:** lines 194/203-220/222/255
**Evidence:** Three different Tier-2 counts in same section.
**Fix-burst resolution:** Standardized on 23 unique Tier 2 hooks; "21 Edit|Write-scoped" framing dropped.

### F-004 [HIGH] D-3 Revised Totals math incoherent
**Location:** lines 253-257
**Evidence:** "44 minus verify-git-push minus regression-gate" double-subtracts; regression-gate is included not subtracted.
**Fix-burst resolution:** Replaced with: "Registry has 52 [[hooks]] entries; 44 route via legacy-bash-adapter; protect-secrets is dual-registered → 43 unique scripts; minus verify-git-push → 42 ported by E-8."

### F-005 [HIGH] "44 bash scripts" vs 43 unique vs 42 ported drift
**Location:** lines 24, 71, 87, 96-97, 106
**Fix-burst resolution:** Standardized "43 unique → 42 ported" everywhere.

### F-006 [HIGH] D-7 hooks.json shape gap; AC-3 as-written breaks system
**Location:** lines 328-352 + AC-3
**Evidence:** Deleting all hooks.json command entries leaves Claude Code with no entry-point to invoke dispatcher → no native plugins fire.
**Fix-burst resolution:** D-7 expanded with explicit "hooks.json contains exactly one dispatcher-routing entry per (event,matcher) tuple"; BEFORE/AFTER sketch added; AC-3 reworded to "zero per-script command entries (only dispatcher-routing remains); zero adapter-wasm references in registry."

### F-007 [HIGH] D-10 adapter retirement contradicts D-4 sequencing (33 dangling registry entries)
**Location:** lines 429-447
**Evidence:** Adapter crate deleted at end of W-15 leaves 33 Tier 2/3 registry entries pointing to deleted .wasm.
**Fix-burst resolution:** D-10 inverted — adapter retained through W-17, deleted at S-8.28 close. R-8.06 + AC-2 + Wave Schedule updated.

### F-008 [HIGH] Wave ID gap audit
**Location:** lines 478-481
**Evidence:** STATE.md history has W-10 and W-12 absent.
**Fix-burst resolution:** Wave IDs marked provisional; "S-5.07 may consume next free wave first" caveat; [process-gap] codification noted for STATE.md "next free wave ID" field.

### F-009 [HIGH] AC-7 vacuous "5 highest-frequency hooks"
**Location:** line 470, 618-619, 680-684
**Fix-burst resolution:** Promoted OQ-4 to S-8.00 (perf baseline + BC-anchor pre-work, 5 pts). AC-7 now reads "per-hook latency does not regress vs S-8.00 baseline > 20%."

### F-010 [HIGH] R-8.07 / TD-007 sequencing — bin/emit-event removal must pin to S-8.28
**Location:** line 458, 646
**Fix-burst resolution:** R-8.07 mitigation reworked; explicit AC: "bin/emit-event removed only after all Tier 3 ports merge (S-8.28)."

### F-011 [MED] Block-mode validators need elevated parity testing
**Location:** D-3 Tier 2 inventory
**Fix-burst resolution:** Callout added: 3 of 23 Tier 2 validators are `on_error = "block"`; AC-8 expanded to require false-block negative test fixtures for those 3.

### F-012 [MED] Goal #5 wording ambiguous re: ABI extension
**Location:** Goal #5 line 110
**Fix-burst resolution:** Reworded: "HOST_ABI_VERSION = 1 unchanged throughout E-8 (additive extension to host fn surface allowed per D-6; no version bump)."

### F-013 [MED] D-1 rationale shaky (verify-git-push out-of-scope)
**Fix-burst resolution:** D-1 strengthened with explicit Windows-Claude-Code matrix citation + 3-part rationale.

### F-014 [MED] TD-010 dependency unjustified for E-8
**Fix-burst resolution:** Either dropped or contextualized — confirm in pass-2.

### F-015 [MED] OQ-1 BC anchors for Tier 1 telemetry
**Fix-burst resolution:** Resolved by S-8.00 (BC-anchor verification table for all 9 Tier 1 hooks; new BCs created if implicit behavior).

### F-016 [MED] Frontmatter schema drift
**Fix-burst resolution:** Added `inputs` + `input-hash` to frontmatter; documented `tech_debt_ref`/`anchor_strategy`/`priority`/`target_release` as proposed schema additions.

### F-017 [MED] Bundle B-3a/B-3b split arbitrary
**Fix-burst resolution:** Either merged into one story OR explicit rationale added — confirm in pass-2.

### F-018 [MED] Cumulative WASM startup cost on Edit/Write
**Fix-burst resolution:** R-8.08 added; AC-7b: aggregate Edit|Write latency ≤ 200ms p95.

## Open Questions Resolution
- OQ-1: resolved by F-015 (S-8.00 BC-anchor verification)
- OQ-2: defer-to-default (release-notes mention) conditional on F-006 resolved
- OQ-3: defer-to-default (single crate per worktree-hooks precedent)
- OQ-4: resolved by F-009 (S-8.00 baseline)
- OQ-5: keep as story-writer audit task for S-8.17
- OQ-6: keep as security-reviewer pre-implementation gate for S-8.09

## Process-gaps surfaced
- Wave-ID allocation policy is implicit; codify next-free-wave field in STATE.md.
- Epic frontmatter schema undefined; canonical template needs verification + propose update.

## Consistency-validator parallel audit
Confirmed:
- BLOCKER-1 (CAP-003 wrong) = F-001
- BLOCKER-2 (registry has 52 entries not 44) = F-004 + adjacent
- ADVISORY-1 (BC anchor strategy sound) — Tier 1 BCs exist in BC-7.03.x family; OQ-1 pre-answered
- ADVISORY-2 (S-8.* namespace clean)
- ADVISORY-3 (epic schema drift) = F-016
- ADVISORY-7 (W-15/16/17 clear at Wave 14 close)
- ADVISORY-8 (OQ-5 write_file host fn confirmed missing)

Sampled BCs verified for Tier 1 hooks: BC-7.03.042..045 (handoff-validator), BC-7.03.076..078 (session-learning), BC-7.03.091..092 (warn-pending-wave-gate), BC-7.03.079..080 (track-agent-start), BC-7.03.020..025 (convergence-tracker), BC-7.03.049..052 (protect-bc), BC-7.03.060..061 (protect-vp), BC-7.03.062..064 (purity-check), BC-7.03.065..075 (red-gate). All exist in BC-INDEX (status=draft, CAP-TBD per TD-001/TD-002 known limbo).

## Next pass
Pass-2 will verify:
- All 18 findings closed in v1.1
- F-006 D-7 dispatcher routing decision concrete
- F-007 D-10 adapter timing inverted correctly
- F-008 wave-ID provisional language acceptable
- F-009 / S-8.00 design adequate
- F-014 TD-010 stance clarified
- F-017 bundle decision concrete
- Any new findings introduced by the fix burst
