---
document_type: adversarial-review-pass
target: .factory/stories/epics/E-8-native-wasm-migration.md
target_version_reviewed: "1.1"
target_version_after_fix_burst: "1.2"
pass_label: ADV-E8-P2
pass_number: 2
date: 2026-04-30
adversary_session: aba7188080b74161b
verdict: SUBSTANTIVE
clock: 0_of_3
findings_total: 7
findings_substantive: 7
findings_critical: 0
findings_high: 1
findings_medium: 4
findings_low: 2
findings_nitpick: 0
fix_burst_status: COMPLETE
fix_burst_session: aa609b97e4fdd157f
---

# ADV-E8-P2 — Adversarial Review Pass 2: E-8 Native WASM Migration Epic

## Verdict
SUBSTANTIVE — 7 findings (1 HIGH partial-closure regression, 4 MEDIUM, 2 LOW). Pass-1 fix burst was largely effective (15/18 cleanly closed, 3 partial). Two partial closures and several fresh defects warrant a focused fix burst before claiming clock advancement.

## Trajectory
| Pass | Findings | Verdict | Clock |
|------|---------|---------|-------|
| 1 | 18 (12H+6M) | SUBSTANTIVE | 0_of_3 |
| 2 | 7 (1H+4M+2L) | SUBSTANTIVE | 0_of_3 |

## Pass-1 Closure Verification Summary
- 14 of 18 cleanly CLOSED (F-001, F-002, F-003, F-004, F-005, F-007, F-008, F-009, F-011, F-012, F-014, F-015, F-017, F-018)
- 4 PARTIAL: F-006 (P2-001 sketch ambiguity), F-010 (P2-002 missing AC row), F-013 (P2-003 doc anchor), F-016 (P2-004 placement)
- 0 regressions

## Pass-2 New/Partial Findings

### P2-001 [MED] D-7 hooks.json BEFORE/AFTER sketch ambiguous on matcher consolidation
- F-006 partial closure
- D-7 BEFORE shows 1 inline command; AFTER shows same matcher routing to dispatcher; doesn't visually demonstrate how multiple inline-command entries collapse to 1 dispatcher entry per (event,matcher) tuple
- Fix-burst resolution: BEFORE expanded to 3+1 entries (validate-bc-title, validate-finding-format, protect-bc, protect-secrets); AFTER shows 1 dispatcher per matcher; explanatory comment added

### P2-002 [HIGH] R-8.07 mitigation references non-existent AC row
- F-010 partial closure
- R-8.07 mitigation quoted "Explicit AC: 'bin/emit-event removed after S-8.28'" but no such AC exists in D-12 or epic-level restatement
- Fix-burst resolution: AC-9 added to D-12 + epic-level AC restatement: "bin/emit-event binary not present in dispatcher binary tree post-S-8.28; validated by `find . -name emit-event` returns empty after S-8.28 merge". R-8.07 mitigation now references AC-9 by ID.

### P2-003 [MED] D-1 rationale (b) cites non-anchored "v1.0 support matrix"
- F-013 partial closure
- "the v1.0 support matrix" invokes authority but no document path; POLICY 5 violation
- Fix-burst resolution: D-1 rationale (b) annotated with OQ-7 reference; OQ-7 added: "Where does the v1.0 Windows-Claude-Code support matrix live? D-1 rationale (b) appeals to it but no canonical doc carries it. Either create one as part of S-5.07 release-notes work, or remove the appeal and ground D-1 only on rationale (a, c, d)."

### P2-004 [LOW] Frontmatter schema-extension annotation only in changelog
- F-016 partial closure
- 4 schema-extension fields (tech_debt_ref, anchor_strategy, priority, target_release) noted only in changelog v1.1 entry
- Fix-burst resolution: HTML comment inserted after frontmatter close referencing changelog v1.1 F-016 + process-gap.

### P2-005 [MED] D-8 Tier-1 sizing-table header says "9 stories" but lists 10
- New finding (post-pass-1 fix-burst regression)
- Header says "Tier 1 (9 stories...)"; table has S-8.00..S-8.09 = 10 rows
- Fix-burst resolution: Header corrected to "Tier 1 (10 stories: 1 pre-work + 9 hook ports, ~3-5 pts each)"

### P2-006 [LOW] R-8.02 mitigation references stale "D-7 era"
- New finding (post-pass-1 fix-burst regression)
- R-8.02 says "warm-pool + compile-cache mitigations in D-7 era" but D-7 is now the hooks.json shape decision
- Fix-burst resolution: replaced with "described under R-8.08 (line ~566)"

### P2-007 [MED] R-8.08 / AC-7b / Goal #6 use ungrounded 10ms-per-plugin estimate
- New finding (fresh adversary scan)
- "23 plugins × ~10ms each = 230ms+" not benchmarked or anchored
- Fix-burst resolution: OQ-8 added (per-plugin WASM warm-invocation latency baseline). R-8.08 re-scored MEDIUM/MEDIUM. Goal #6 + AC-7b marked tentative; baseline-derived adjustment allowed in S-8.00.

## Process-gap observations
- When inverting decision content (D-10, D-7), fix-burst ritual should sweep risk-register cross-references to renumbered/reframed decision IDs (P2-006 root cause)
- Schema-extension annotations should be inline near frontmatter, not buried in changelog (P2-004 root cause)
- Risk metrics with quantitative claims should cite source or mark as estimate-pending-baseline (P2-007 root cause)

## Open Questions Status
- OQ-2: defer-to-default conditional on F-006 [closed by P2-001 fix]
- OQ-5: keep as story-writer audit task for S-8.17
- OQ-6: keep as security-reviewer pre-implementation gate for S-8.09
- OQ-7: NEW — Windows support matrix canonical doc location
- OQ-8: NEW — Per-plugin WASM warm-invocation latency baseline (S-8.00 measurement)

## Convergence Status
- Pass-2 substantive findings: 7
- Trajectory: 18 → 7
- Clock: 0_of_3 (cannot advance — substantive findings present)
- Recommended next: pass-3 — verify P2 fixes landed; expect LOW/NITPICK ceiling if fixes correctly applied
