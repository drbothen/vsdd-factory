---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-26T20:15:00Z
phase: 2-re-anchor
inputs:
  - .factory/stories/S-1.01-cargo-workspace-setup.md
  - .factory/stories/S-1.02-dispatcher-core.md
  - .factory/stories/S-1.04-host-function-surface.md
  - .factory/stories/S-1.05-wasmtime-integration.md
  - .factory/stories/S-1.06-tokio-parallel-tier-execution.md
  - .factory/stories/S-1.07-dispatcher-internal-log.md
  - .factory/stories/S-3.04-emit-event-host-function.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "3471ea6"
traces_to: ""
cycle: v1.0-brownfield-backfill
sub_cycle: wave-1-ss-01-re-anchor
pass: 2
previous_review: wave-1-ss-01-pass-1.md
verdict: FINDINGS_REMAIN
finding_count: 4
convergence_step: 0_of_3
po_commit_reviewed: 754734a
---

# Adversarial Review — Wave 1 SS-01 Re-anchor — Pass 2

## Finding ID Convention

Pass-2 findings use `F-1NN` numbering (vs. pass-1's `F-0NN`) to disambiguate origin pass.

## Part A — Pass-1 Fix Verification

| Finding | Pass-1 Severity | Fix Applied? | Evidence | Notes |
|---|---|---|---|---|
| F-001 | HIGH | YES | S-1.01 line 24 `behavioral_contracts: []`; body table at line 167 reads `_(None — see note above.)_` | Both surfaces clean |
| F-002 | HIGH | YES | S-1.01 lines 66, 69, 74, 79 use transitive-prerequisite framing | Coherent rewrite |
| F-003 | HIGH | YES | S-1.04 AC#8 retracted BC-1.05.011 trace; v1.1 candidate logged | Clean |
| F-004 | MED | YES | BC-1.08.003 in S-1.02 only | Sole owner ✓ |
| F-005 | MED | YES | S-1.01 traces_to/FRs empty with body rationale | Documented |
| F-006 | MED | YES | S-1.05 AC#10 acknowledges partial coverage; v1.1 backlog | Honest |
| F-007 | MED | **PARTIAL** | CAP-010 + S-1.07 fixed; recommended cross-CAP sweep NOT performed | See F-102 below |
| F-008 | LOW | YES | 93 unique BCs anchored; arithmetic verified |  |
| F-009 | LOW | YES | S-1.01 line 149 informational disclaimer present |  |
| F-010 | LOW | YES | S-3.04 Partial Status table present |  |

**Summary:** 9 of 10 fully fixed; F-007 partially fixed (sweep deferred → triggered new finding F-102).

## Part B — New Findings

### F-101 [MED] — S-1.06 AC#3 trace claim ("subsequent tiers skipped") not anchored

**Affected:** `.factory/stories/S-1.06-tokio-parallel-tier-execution.md` line 73-74

AC#3: "If any plugin in a tier returns Block, subsequent tiers are skipped (traces to BC-1.03.009 ... EC-001)". BC-1.03.009 contracts: block_intent flag, remaining tier plugins still fire, exit_code=2. Says nothing about tier-skip. BC-1.03.007 EC-001 explicitly says non-block tier failures DO continue to subsequent tiers.

The "tier-skip on block_intent" semantics is real implementation behavior with no SS-01 BC contract.

**Remediation:** (a) v1.1 BC candidate "tier-skip on block_intent" + backlog entry mirroring pass-1 F-006 pattern, (b) reword AC#3 without "subsequent tiers skipped" claim, or (c) backfill new BC-1.03.NNN: "executor halts further tier dispatch when any tier records block_intent".

### F-102 [MED] — capabilities.md CAP-003 Subsystems missing SS-01 [process-gap]

**Affected:** `.factory/specs/domain-spec/capabilities.md` line 39 (CAP-003)

CAP-003 declares `Subsystems: SS-03, SS-10.`. S-3.04 anchors `capabilities: [CAP-003]` with `subsystems: ["SS-01", "SS-03"]`. S-3.04's target_module is `crates/factory-dispatcher/src/host/emit_event.rs` (canonically SS-01 per ARCH-INDEX line 74).

Same drift class as pass-1 F-007 (CAP-010 was missing SS-01). Pass-1 fix burst updated CAP-010 only; F-007 explicitly recommended a cross-CAP sweep but it wasn't performed.

**Remediation:** (a) Update CAP-003 Subsystems to `SS-01, SS-03, SS-10`. (b) Perform full 28-CAP sweep against ARCH-INDEX before Wave 2. Likely candidates: CAP-007, CAP-008, CAP-027.

### F-103 [LOW] — S-1.04 AC#7 conflates OUTPUT_TOO_LARGE with OOB memory access

**Affected:** `.factory/stories/S-1.04-host-function-surface.md` line 100-101

AC#7: "All functions handle OOB memory access gracefully (traces to BC-1.05.031 postcondition: decode_args rejects truncated buffer; BC-1.05.005 postcondition: OUTPUT_TOO_LARGE returned not panic)".

BC-1.05.005 is about exec_subprocess result envelope exceeding `result_buf_cap` (output buffer overflow), not OOB memory bounds. Distinct concerns:
- BC-1.05.031: argument decode buffer truncation — fits OOB theme ✓
- BC-1.05.005: output too large — different concern

**Remediation:** (a) Split AC into two (one for OOB, one for output-cap), or (b) drop BC-1.05.005 from this AC and keep only BC-1.05.031.

### F-104 [LOW, pending intent verification] — Several AC trace clauses paraphrase BC content rather than quote literally

**Affected:** Multiple stories (S-1.04 line 109, line 97; S-1.05 line 80; etc.)

Examples:
- S-1.04 line 109 trace: "BC-1.05.020 invariant: log level discriminants 0..=4 pinned" — actual BC text: "Cross-crate level mapping is byte-stable" (semantic faithful, not literal)
- S-1.04 line 97 trace: "BC-1.05.029 postcondition: SHELL_NAMES set includes bash/sh/zsh/pwsh" — actual: "Returns true iff cmd's basename is in SHELL_NAMES"
- S-1.05 line 80 trace: "BC-1.09.002 invariant: thread-safe via Mutex<HashMap>" — actual: "Mutex prevents data races; cache is thread-safe"

These are not fabricated like pass-1 F-002 — they faithfully describe BC semantics. Pattern raises convention question: must trace clauses be literal quotes or are semantic restatements acceptable?

**Remediation:** (a) Adjudicate convention. If "literal quote" required, reword these. If "semantic faithful" acceptable, no action. PO/orchestrator decision.

## Part C — Process-Gap Sweep

| CAP | Subsystems Field | Stories Anchored | Gap? |
|---|---|---|---|
| CAP-002 | SS-01, SS-02, SS-04 | S-1.01,02,04,06 | none |
| CAP-003 | SS-03, SS-10 | S-3.04 (SS-01,SS-03) | **GAP — missing SS-01** (F-102) |
| CAP-009 | SS-02 | S-1.01 informational | none |
| CAP-010 | SS-01, SS-03, SS-10 (post-fix) | S-1.07 (SS-01) | none |
| CAP-011 | SS-01 | S-1.05 (SS-01) | none |

Recommend full 28-CAP sweep before Wave 2.

## Self-Validation Pass

All 4 findings re-verified via Bash grep + file reads. No findings withdrawn.

## Novelty Assessment

| Field | Value |
|---|---|
| **Pass** | 2 |
| **New findings count** | 4 |
| **Duplicate count** | 0 (no pass-1 finding regressed; F-102 is sibling pattern of F-007 not regression) |
| **Novelty score** | 1.0 (all findings novel; 4/4) |
| **Median severity** | LOW-MED boundary |
| **Severity distribution** | 0 CRIT, 0 HIGH, 2 MED, 2 LOW |
| **Trajectory** | 10 (pass-1) → 4 (pass-2): 60% reduction; no HIGHs; no fabrications |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Status

**0 of 3** consecutive NITPICK-only passes per ADR-013. Pass-2 has 2 MEDIUMs which exceed NITPICK-only threshold.

Estimate: pass-3 should achieve clean status if F-101 and F-102 are addressed and the cross-CAP sweep is performed.

## Findings by Axis

| Axis | Findings |
|---|---|
| A — BC Existence | (none) ✓ |
| B — Semantic Anchoring | F-103 |
| C — Coverage Completeness | F-101 |
| D — AC↔BC Bidirectional | F-101, F-104 |
| E — Capability Justification | (none) ✓ |
| F — Subsystem/FR Hygiene | F-102 |
| G — VP Soundness | (none) ✓ |
| H — CAP Choice | F-102 |
| I — Spec-First Gate | (none) ✓ |
| J — POLICY 1 Reuse | (none) ✓ |
| K — Edge Cases | (none) ✓ |
| L — Bookkeeping | F-104 |

## Summary

Pass-2 verdict: FINDINGS_REMAIN (4 findings, 2 MED + 2 LOW).
Convergence: 0 of 3 NITPICK-only passes.
Trajectory: healthy (10→4, no HIGHs).
Pass-3 should achieve NITPICK-only if F-101, F-102 fixed and cross-CAP sweep performed.
