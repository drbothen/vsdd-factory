---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 36
verdict: MED
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T00:00:00Z
strategic_recommendation: continue-protocol-per-user-directive
---

# F5 Pass-36 Adversary Review

## Verdict

**MED** (1M, 0H, 0L). 19th consecutive non-NIT. ADR-013 RESETS to 0_of_3.

## Findings

### F-P36-001 [MEDIUM] BC-INDEX vs source body Stories cell drift across 12 BCs (F3 propagation gap)

12 BCs have BC-INDEX rows that include `S-15.01` (post-F3) but source body Traceability tables still say `TBD` (pre-F3). Bidirectional sibling-cell drift; 5th META-self-application failure of L-P28-001 family.

| BC ID | BC-INDEX Stories | Source body |
|-------|------------------|-------------|
| BC-1.01.001 | S-15.01 | TBD (re-anchor in Phase 1.8) |
| BC-1.01.007 | S-15.01 | TBD |
| BC-1.08.001 | S-2.07, S-15.01 | S-2.07; TBD |
| BC-1.08.002 | S-2.07, S-15.01 | S-2.07; TBD |
| BC-1.14.001 | S-15.01 | TBD |
| BC-7.06.001 | S-15.01 | TBD |
| BC-9.01.006 | S-15.01 | TBD |
| BC-3.08.001 | S-15.01 | TBD |
| BC-4.04.004 | S-5.01, S-15.01 | S-5.01 only |
| BC-4.05.004 | S-5.02, S-15.01 | S-5.02 only |
| BC-4.07.003 | S-5.03, S-15.01 | S-5.03 only |
| BC-4.08.002 | S-5.04, S-15.01 | S-5.04 only |

**Fix:** Propagate S-15.01 to source body Traceability `Stories` rows (option a — F3 decision is canonical; PR #106 merged). Bump versions on each touched BC.

## Notable observations

- Fix-burst-34 closure of F-P35-001 VERIFIED clean.
- Implementer's CAP-TBD note investigated: 6 sampled source BCs (BC-1.01.001 etc.) carry `capability: "CAP-TBD"` in source frontmatter — legitimate brownfield placeholder, not drift.
- Per-cell tabulation on 4 VP-INDEX + 3 STORY-INDEX rows: all clean.
- Index versions confirmed.
- 5th META-self-application failure: L-P28-001 reinforcement audit FORMAT was specified but not corpus-wide SCOPE.

## Convergence assessment

19th non-NIT. Per user directive: continue protocol.
