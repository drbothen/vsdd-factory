---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 37
verdict: MED
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T18:00:00Z
strategic_recommendation: continue-protocol-per-user-directive
---

# F5 Pass-37 Adversary Review

## Verdict

**MED** (1M, 1L pending intent). 20th consecutive non-NIT. ADR-013 RESETS to 0_of_3.

6th META-self-application failure of L-P28-001 family. The corpus-wide-on-first-application scope clause was added fix-burst-35 but applied body→index direction only, missing the reverse direction (BC-INDEX missing entries that exist in source bodies).

## Findings

### F-P37-001 [MEDIUM] BC-INDEX missing S-10.04 in 3 BCs (BC-1.12.003, BC-1.12.004, BC-1.12.005)

| BC ID | BC-INDEX row | Source body Stories row | bcs: frontmatter |
|-------|--------------|------------------------|------------------|
| BC-1.12.003 | `S-10.03` (line 259) | `S-10.03, S-10.04` (line 231) | S-10.03+S-10.04 |
| BC-1.12.004 | `S-10.02, S-10.03` (line 260) | `S-10.02, S-10.03, S-10.04` (line 280) | S-10.02+S-10.03+S-10.04 |
| BC-1.12.005 | `S-10.02, S-10.03` (line 261) | `S-10.02, S-10.03, S-10.04` (line 186) | S-10.02+S-10.03+S-10.04 |

S-10.04 frontmatter `bcs:` lists all 3 BCs; story body BC table enumerates all 3. BC-INDEX rows missing S-10.04 in each.

**Fix:** Add S-10.04 to BC-INDEX rows 259/260/261. Bump BC versions on touched BCs (frontmatter changelogs). BC-INDEX v1.53→v1.54. ARCH-INDEX v1.33→v1.34 cite refresh.

### F-P37-002 [LOW pending intent] STORY-INDEX S-3.03 Depends-On missing S-1.03
- STORY-INDEX:254 cell: `S-2.08, S-3.04`
- Source frontmatter: `["S-1.03", "S-2.08", "S-3.04"]`
- Possibly intentional convention to omit scaffolding deps. Adjudicate.

## Notable observations

- Fix-burst-35 closure VERIFIED: all 12 BC body Stories rows updated; index versions confirmed; L-P28-001 scope clause present.
- VP-INDEX × VP body sample audit (5 VPs): all clean.
- STORY-INDEX × story body sample audit (5 stories, beyond S-3.03): all clean.
- 6th META-self-application failure documented in detail.

## Convergence assessment

20th non-NIT. Per user directive: continue protocol.
