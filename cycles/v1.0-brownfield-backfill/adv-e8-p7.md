---
document_type: adversarial-review-pass
target: .factory/stories/epics/E-8-native-wasm-migration.md
target_version_reviewed: "1.4"
target_version_after_fix_burst: "1.5"
pass_label: ADV-E8-P7
pass_number: 7
date: 2026-04-30
adversary_session: a4f73270c9a44ba33
verdict: SUBSTANTIVE
clock: 0_of_3
clock_event: RESET
clock_reset_reason: "F-P7-001 (HIGH) registry arithmetic contradiction triggers ADR-013 reset 0_of_3 (no advance from prior 0_of_3)"
findings_total: 3
findings_substantive: 2
findings_critical: 0
findings_high: 1
findings_medium: 1
findings_low: 1
findings_nitpick: 0
fix_burst_status: PENDING
---

# ADV-E8-P7 — Adversarial Review Pass 7: E-8 Native WASM Migration Epic

## Verdict
SUBSTANTIVE — 1 HIGH + 1 MED + 1 LOW. Clock remains at 0_of_3 (no advance possible from RESET state when HIGH/MED present).

**F-P7-001 is foundational:** registry-vs-tier-sum arithmetic survived 6 prior passes because each pass verified internal section consistency, not cross-section propagation from primary inventory tables. The F-P6-004 fix (D-10 going 33→34 Tier 2/3 entries) made the local D-10 numbers internally correct AND made the global discrepancy newly visible by tightening one half of the arithmetic.

## Trajectory
| Pass | Substantive | LOW | Verdict | Clock |
|------|------------|-----|---------|-------|
| 1 | 18 (12H+6M) | 0 | SUBSTANTIVE | 0_of_3 |
| 2 | 7 (1H+4M) | 2 | SUBSTANTIVE | 0_of_3 |
| 3 | 0 | 2 | NITPICK_ONLY | 1_of_3 |
| 4 | 1 (MED) | 2 | SUBSTANTIVE | 0_of_3 (RESET) |
| 5 | 0 | 0 | NITPICK_ONLY | 1_of_3 (ADVANCE) |
| 6 | 2 (MED) | 5 | SUBSTANTIVE | 0_of_3 (RESET) |
| 7 | 2 (1H+1M) | 1 | SUBSTANTIVE | 0_of_3 (no advance) |

## P6 Closure Verification (all 7 fixes verified)
- F-P6-002 [LOW]: CLOSED (Goals AC note at lines 131-133)
- F-P6-003 [LOW]: CLOSED (D-7 BEFORE/AFTER event-grouped at lines 387-411 / 420-444); but introduces F-P7-002
- F-P6-004 [MED]: CLOSED locally (D-10 says "34 entries / 33 unique"); but reveals F-P7-001 globally
- F-P6-005 [LOW]: CLOSED (R-8.10 at line 595)
- F-P6-006 [LOW]: CLOSED (W-16 line 728-730 + W-17 line 738-740 BC-anchor bullets reference D-2)
- F-P6-007 [MED]: CLOSED (Goal #6 + D-12 + restatement all "Per-Tier-2 hook")
- F-P6-008 [LOW]: CLOSED (D-8 line 528 "~123-155 = 123 base + 32 buffer")

## Pass-7 Findings

### F-P7-001 [HIGH] Registry-vs-tier-sum arithmetic contradiction
- Top-level text: "44 entries / 43 unique adapter-routed scripts"
- Tier inventories sum to: 42 unique / 43 entries (Tier 1: 9/9; Tier 2: 23/23; Tier 3: 10/11)
- Phantom 43rd unique script — D-1 explicitly excludes verify-git-push from adapter routing, so it can't account for the gap
- Likely root cause: pass-1 fix corrected "ported by E-8 = 42" but left upstream "43 unique" intact
- Severity HIGH — load-bearing for D-3, D-10, AC-3 audit, and entire scope-of-work narrative
- Recommended Scenario A: "42 unique / 43 entries / 42 ported (verify-git-push registered separately, not adapter-routed)"

### F-P7-002 [MED] D-7 line 465 stale "one per event group"
- Post-F-P6-003 event-grouped JSON shows 3 dispatcher entries (PostToolUse:E|W, PreToolUse:E|W, PreToolUse:Bash) = 3 (event, matcher) tuples, NOT one per event
- Internal contradiction: lines 374, 380, 446 all use "(event, matcher) tuple" wording; line 465 lapses to "per event group"
- Fix: change line 465 to "Dispatcher-routing entries (one per (event, matcher) tuple)"

### F-P7-003 [LOW] DRIFT-010 stale "26 unported" reference
- Line 824 description: "DRIFT-010 (26 unported bash hooks block Windows native)"
- Actual count is 42 (per E-8 scope) — same stale snapshot D-9 already corrects via release-notes
- Fix: update description to note original-vs-current framing

## Process-gap observations
- Cross-section arithmetic re-derivation from primary inventory tables surfaces drift that section-internal consistency checks miss (P3, P4 missed F-P4-001; P5 missed F-P6-004/007; P7 caught F-P7-001 via item (d) of pass-7 prompt's hard-scrutiny instructions).
- Pattern across 7 passes: every "clean" pass-N is followed by SUBSTANTIVE pass-N+1 finding new defects on identical content. Validates the multi-pass discipline; single-pass acceptance is unsafe.

## Convergence Status
- Trajectory: 18 → 7 → 0 → 1 → 0 → 2 → 3 (1H+1M+1L)
- Clock: 0_of_3 (held)
- Recommended next: empirical fix burst v1.4 → v1.5 (architect verifies registry counts from hooks-registry.toml directly, picks canonical scenario, propagates); then pass-8
- Estimated path-to-convergence: 3 more clean passes from 0_of_3 — assuming fix burst introduces no new drift
