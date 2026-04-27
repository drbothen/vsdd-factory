---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T00:00:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-2.01-legacy-bash-adapter.md
  - .factory/stories/S-3.01-port-capture-commit-activity.md
  - .factory/stories/S-3.02-port-capture-pr-activity.md
  - .factory/stories/S-3.03-port-block-ai-attribution.md
  - .factory/stories/S-5.01-session-start-hook.md
  - .factory/stories/S-5.02-session-end-hook.md
  - .factory/stories/S-5.03-worktree-hooks.md
  - .factory/stories/S-5.04-post-tool-use-failure.md
  - .factory/specs/prd.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.03.001.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.01.002.md
input-hash: "a0e02d7"
traces_to: ".factory/specs/prd.md"
cycle: v1.0-brownfield-backfill
sub_cycle: wave-3-ss-04-re-anchor
pass: 2
verdict: FINDINGS_REMAIN
finding_count: 7
convergence_step: 0_of_3
po_commit_reviewed: a0e02d7
previous_review: wave-3-ss-04-pass-1.md
---

# Adversarial Review — Wave 3 SS-04 Re-anchor — Pass 2

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix from `.factory/current-cycle` (e.g., `P1CONV`, `P3PATCH`)
  - If no current-cycle file exists, omit the cycle segment (falls back to `ADV-P<PASS>-<SEV>-<SEQ>`)
- `<PASS>`: Two-digit pass number (e.g., `P01`, `P24`)
- `<SEV>`: Severity abbreviation (`CRIT`, `HIGH`, `MED`, `LOW`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `ADV-P1CONV-P03-CRIT-001`, `ADV-P3PATCH-P24-HIGH-002`, `ADV-P01-MED-003` (no cycle)

The cycle prefix prevents ID collisions when multiple convergence cycles coexist in the same project.

## Pass-1 Closure Verification (Partial-Fix Regression Discipline, S-7.01)

| Pass-1 Finding | Status | Evidence |
|---|---|---|
| F-001 BC-4.03.001 sanction | CLOSED | All 5 stretch-anchored stories carry "Wave 2 F-007 precedent, sanctioned Wave 3 F-001" disclosure (S-3.02:56, S-5.01:56, S-5.02:56, S-5.03:56, S-5.04:56) |
| F-002 S-3.03 re-anchor | PARTIAL — see F-101 | bcs: frontmatter is now [BC-2.01.002]; subsystems: [SS-02, SS-04]. BUT verification_properties: [VP-044] still references legacy-bash-adapter VP that the same F-002 reasoning excludes |
| F-003 FR-045/046 split | CLOSED-AS-PROPOSAL | S-3.02 prose now proposes FR-046; S-5.01-04 prose proposes FR-045. Neither yet in PRD §2.4/§7 (out-of-scope per pass-1) |
| F-004 dual-anchor S-3.03 | CLOSED | functional_requirements: ["FR-013", "FR-032"] verified at S-3.03:16 |
| F-005 BC-format full IDs | PARTIAL — see F-104 | S-3.01/02/03 architecture rules updated. SIBLING REGRESSION: S-5.01-04 still cite stale "BC-1.01" prefix |
| F-006 cycle scope note | CLOSED | Documented as STATE.md note (out-of-band) |
| F-007 AC retrace | CLOSED | S-3.01 AC#5 retraced to postcondition 2; AC#7 marked [process-gap] |
| F-008 VP scoping | CLOSED-WITH-RESIDUAL — see F-106 | S-3.01:58 per-VP scope text added; closing sentence still mis-groups VP-043 |
| F-009 PRD §14 vs §7 | DEFERRED | Acknowledged out-of-scope |
| F-010 BC-INDEX TBD | DEFERRED | Acknowledged out-of-scope |
| F-011 SS-04 arch doc | DEFERRED | Acknowledged out-of-scope |

## Part B — New Findings (7 total: 0 CRIT, 3 HIGH, 2 MED, 2 LOW)

### F-101 [HIGH] — S-3.03 verification_properties: [VP-044] is legacy-bash-adapter scoped, contradicts F-002 native-WASM ruling

**File:** `.factory/stories/S-3.03-port-block-ai-attribution.md` lines 23-24, 64-65
**Policies:** POLICY 4 (semantic_anchoring_integrity), POLICY 5 (creators_justify_anchors)
**Confidence:** HIGH

Pass-1 F-002 removed BC-4.02.002/003 from S-3.03 with explicit reasoning: "those BCs are scoped to the legacy bash-adapter (SS-04/SS-07 boundary); a native WASM plugin does not go through the adapter" (S-3.03:59, retained in current story). By the identical reasoning, VP-044 ("Legacy Bash Adapter Exit Code Mapping Is Correct") is bash-adapter scoped and does not apply to the native WASM block-ai-attribution plugin.

The frontmatter `verification_properties: [VP-044]` (line 24) and the body VP table (lines 64-65) claim VP-044 "Verifies that exit 2 from any plugin (including block-ai-attribution) maps to HookResult::Block" — generalizing a bash-adapter VP to all plugins. This is the exact mis-anchor pattern F-002 closed for BCs but left open for VPs.

S-3.01:58 per-VP scope text correctly identifies VP-044 as "SS-04/SS-07 — bash-adapter exit-code mapping" — the same scope statement is what makes VP-044 inappropriate for S-3.03.

**Fix:** Remove VP-044 from S-3.03 verification_properties; rewrite the Verification Properties section to state "No existing VPs directly verify block-ai-attribution WASM behavior; v1.1 VP candidate (HookResult::Block contract for native plugins) is implied by BC-2.01.002". Mis-anchoring blocks convergence per BC-5.04.003.

### F-102 [MEDIUM] — S-3.03 subsystems [SS-02, SS-04] but CAP-008 declares only [SS-01, SS-04, SS-07]

**File:** `.factory/stories/S-3.03-port-block-ai-attribution.md` lines 25, 29; `.factory/specs/domain-spec/capabilities.md` lines 47-50
**Policies:** POLICY 4, POLICY 6 (architecture_is_subsystem_name_source_of_truth)
**Confidence:** HIGH

F-002 fix added SS-02 to S-3.03 `subsystems:` frontmatter (line 29) because the BC anchor BC-2.01.002 is in SS-02. But the story's `capabilities: ["CAP-008"]` (line 25) requires CAP-008's declared subsystems to cover the story's subsystems.

CAP-008 in capabilities.md:
> "Subsystems: SS-01, SS-04, SS-07"

SS-02 is not in CAP-008's declared subsystem list. Either:
(a) CAP-008 should be expanded to include SS-02 (because the SDK's HookResult exit-code contract is part of the gating capability), or
(b) The story should add a secondary `capabilities:` entry like CAP-009 (SS-02 SDK-author capability) to cover the SS-02 work.

This pattern was caught for CAP-003/CAP-010 in Wave 2 (capabilities.md:160-163 process-gap note). Same drift class recurring.

**Fix:** capabilities.md F-007-style update to add SS-02 to CAP-008, or add CAP-009 to S-3.03 capabilities array.

### F-103 [MEDIUM] — S-5.04 subsystems [SS-01, SS-04] but CAP-013 declares only [SS-04, SS-07]

**File:** `.factory/stories/S-5.04-post-tool-use-failure.md` lines 24, 28; `.factory/specs/domain-spec/capabilities.md` lines 66-69
**Policies:** POLICY 4, POLICY 6
**Confidence:** HIGH

S-5.04 frontmatter declares `subsystems: ["SS-01", "SS-04"]` and `capabilities: ["CAP-013"]`. capabilities.md CAP-013 says "Subsystems: SS-04, SS-07". SS-01 is not in CAP-013's declared subsystem list.

Pass-1 F-006 noted dual-subsystem [SS-01, SS-04] for S-5.01-04 was "internally coherent with FR-007" but did not check against the CAP-NNN subsystem declaration. This is a sibling drift pattern.

S-5.01/S-5.02/S-5.03 use CAP-002 which declares [SS-01, SS-02, SS-04] — those three are clean (SS-01, SS-04 ⊆ CAP-002). Only S-5.04 has the gap because it cites CAP-013 instead of CAP-002.

**Fix:** Either add SS-01 to CAP-013's declared subsystems (because dispatcher routing of PostToolUseFailure lives in SS-01), or change S-5.04 capability anchor to CAP-002 (which already includes SS-01).

### F-104 [HIGH] — Partial-fix-regression: F-005 fix to BC prefix format not propagated to S-5.01-04 sibling stories

**Files:**
- `.factory/stories/S-5.01-session-start-hook.md` line 133 ("BC-1.01 registry schema")
- `.factory/stories/S-5.02-session-end-hook.md` line 134 ("BC-1.01")
- `.factory/stories/S-5.03-worktree-hooks.md` line 138 ("BC-1.01")
- `.factory/stories/S-5.04-post-tool-use-failure.md` line 133 ("BC-1.01 event type config")

**Policies:** POLICY 4, POLICY 7 (bc_h1_is_title_source_of_truth), BC-5.36.005, BC-5.36.006
**Confidence:** HIGH

Pass-1 F-005 closed for S-3.01/S-3.02/S-3.03 (now using full BC-S.SS.NNN). Per S-7.01 partial-fix-regression discipline, the same defect class in sibling files in the same architectural layer (SS-04 plugin-ecosystem stories) should have been swept. All four S-5.NN stories have the identical "BC-1.01" stale prefix in Architecture Compliance Rules tables.

Blast radius = 4 files; per S-7.01 severity rule "Blast radius = 2+ files: HIGH". Same defect, missed siblings, fix should have been atomic.

**Fix:** Update each of S-5.01:133, S-5.02:134, S-5.03:138, S-5.04:133 to cite full canonical BC-1.01.NNN ID. Pending intent verification per BC-INDEX (BC-1.01.001..BC-1.01.015).

### F-105 [HIGH] — POLICY 8 violation: 5 stories have AC traces to BCs not in `bcs:` frontmatter

**Files:**
- `.factory/stories/S-3.03-port-block-ai-attribution.md` line 91 (AC#6 traces to BC-1.01.001; bcs:[BC-2.01.002])
- `.factory/stories/S-5.01-session-start-hook.md` line 76 (AC#1 traces to BC-1.02.005; bcs:[BC-4.03.001])
- `.factory/stories/S-5.02-session-end-hook.md` line 76 (AC#1 traces to BC-1.02.005; bcs:[BC-4.03.001])
- `.factory/stories/S-5.03-worktree-hooks.md` line 77 (AC#1 traces to BC-1.02.005; bcs:[BC-4.03.001])
- `.factory/stories/S-5.04-post-tool-use-failure.md` line 76 (AC#1 traces to BC-1.02.004; bcs:[BC-4.03.001])

**Policies:** POLICY 8 (bc_array_changes_propagate_to_body_and_acs), BC-5.06.012, BC-5.06.014
**Confidence:** HIGH

POLICY 8 requires bidirectional sync: AC traces ⊆ bcs: frontmatter ∪ body BC table. The five stories above use BC-1.NN.NNN identifiers in AC traces but those BCs are not in the story's `bcs:` array OR the body's Behavioral Contracts table. The story prose excuses these as "cross-subsystem informational references", but POLICY 8 doesn't have an "informational" exception.

Per Story Frontmatter-Body Coherence Review Axis: "Systematic pattern across 3+ stories: HIGH with pattern flag". Five stories qualify.

**Fix options (author adjudication):**
(a) Add the cross-subsystem BCs to `bcs:` arrays AND body BC tables AND expand `subsystems:` accordingly — establishes them as anchoring contracts.
(b) Remove the BCs from AC traces and replace with `[process-gap]` markers + v1.1 BC candidates — establishes them as informational only, not anchoring.

Option (b) is consistent with the story prose ("informational"), but the AC traces should not silently cite BCs while disclaiming anchoring elsewhere.

### F-106 [LOW] — F-008 residual: S-3.01 line 58 self-contradicts ("All three are legacy-bash-adapter scoped")

**File:** `.factory/stories/S-3.01-port-capture-commit-activity.md` line 58
**Policies:** POLICY 4
**Confidence:** HIGH

The fix narrative correctly added per-VP scope: "VP-043 (SS-07/SS-01 — hooks-registry routing invariant), VP-044 (SS-04/SS-07 — bash-adapter exit-code mapping), VP-045 (SS-04 — plugin_config stripping)". But the very next clause says "All three are legacy-bash-adapter scoped". Per the inline scope tags themselves, only VP-044 and VP-045 are legacy-bash-adapter; VP-043 is hooks-registry routing (SS-07/SS-01), not adapter.

**Fix:** Replace "All three are legacy-bash-adapter scoped" with "VP-044 and VP-045 are legacy-bash-adapter scoped; VP-043 is hooks-registry routing scoped".

### F-107 [LOW] — S-5.03 body references SS-03 scope but subsystems frontmatter omits it

**File:** `.factory/stories/S-5.03-worktree-hooks.md` lines 28, 83
**Policies:** POLICY 6
**Confidence:** MEDIUM

Frontmatter declares `subsystems: ["SS-01", "SS-04"]`. Body line 83 explicitly says "no SS-04 or SS-03 BC contracts auto-generated sink config". The "auto-generated sink configuration" work touches SS-03 (Observability Sinks) — the goal section line 71 calls out "auto-registers new worktrees with the observability stack". SS-03 should be in subsystems if the story performs sink-config writes.

**Fix:** Either add SS-03 to subsystems frontmatter OR clarify in prose that the sink config write is delegated to a separate SS-03-owned story (not in scope here). Pending intent verification.

## Observations

- [process-gap] PRD §7 line 1076 "Total: 44 FRs across 10 subsystems" vs §14 line 1323 "FRs defined | 43" vs §47 "43 logical FRs". Pass-1 F-009 acknowledged. With FR-045/046 PROPOSED but not yet authored into PRD §2.4, drift is poised to grow. Wave 3 close should reconcile or formalize before declaring complete.
- [process-gap] CAP→SS drift sweep recommended after Wave 2; pass-2 found F-102, F-103 — sweep still pending. Recommend architect/business-analyst run a 28-CAP audit before Wave 4.
- [process-gap] BC-5.36.005 ("partial-fix-regression check") correctly applied (F-104) — validates the rule.

## CAP Subsystem Drift Sweep — DRIFT FOUND

Two CAP→SS drifts identified (F-102 CAP-008, F-103 CAP-013). Pass-1 declared CAP sweep CLEAN but did not check subsystems frontmatter against CAP declared subsystems — only checked the reverse direction. Pass-2 found this gap.

## Bidirectional `depends_on`↔`blocks` Symmetry Sweep

Not exhaustively swept; sample check on S-3.01 not verified end-to-end. Marking as `(pending intent verification)`. No new findings.

## Cross-Subsystem Leakage Sweep

S-5.03 may have legitimate SS-03 leakage (F-107). Other stories appear self-consistent.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 3 |
| MEDIUM | 2 |
| LOW | 2 |

**Overall Assessment:** pass-with-findings
**Convergence:** findings remain — clock RESETS per BC-5.04.003 (3 HIGH findings)
**Readiness:** requires revision before pass-3

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 2 |
| **New findings count** | 7 |
| **Duplicate count** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | MEDIUM |
| **Severity distribution** | 0 CRIT, 3 HIGH, 2 MED, 2 LOW |
| **Trajectory** | pass-1=11 → pass-2=7 (decreasing); HIGH count 4→3 |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3.** 3 HIGH findings (F-101 VP semantic mis-anchor, F-104 sibling-regression on BC prefix, F-105 POLICY 8 systematic AC-trace pattern) reset the clock per BC-5.04.003.

## Findings by Axis

| Axis | Findings |
|---|---|
| Semantic Anchoring (B) | F-101 |
| FR/Subsystem Hygiene (F) | F-102, F-103, F-107 |
| AC↔BC Bidirectional (D) | F-105 |
| Bookkeeping (L) | F-104, F-106 |
| Pre-existing TD/out-of-scope | (Observations) |

## Trajectory Baseline

Pass-1 = 11 (4H/4M/3L); Pass-2 = 7 (3H/2M/2L). Net reduction 4; HIGH count down 1. Three NEW HIGH findings (F-101 incomplete F-002 propagation; F-104 sibling regression; F-105 systematic pattern not previously swept) — fresh-context value validated.

## Verdict

**FINDINGS_REMAIN.** Three HIGH findings require remediation:
1. F-101 — VP-044 mis-anchor in S-3.03 (extension of pass-1 F-002 closure gap)
2. F-104 — sibling-regression of pass-1 F-005 fix to S-5.01-04 architecture compliance rules
3. F-105 — systematic POLICY 8 violation across 5 stories

Two MEDIUM CAP→SS drifts (F-102, F-103) require either capabilities.md update or story re-anchor.

Pass-3 target: confirm F-101/F-104/F-105 closure; resweep remaining stories; if NITPICK_ONLY, advance convergence_step to 1_of_3.
