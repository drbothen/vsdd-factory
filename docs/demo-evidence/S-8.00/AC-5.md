---
story_id: S-8.00
ac: AC-5
title: BC-gap remediation — 0 of 9 gaps found
---

# AC-5: BC-gap remediation

**Statement:** For each Tier 1 hook where Gap-Found = Y (no existing BC covers the hook's behavior): a new BC is drafted. If >5 of 9 Tier 1 hooks lack BC coverage, file OQ-9 in E-8 epic and evaluate W-16 deferral.

## Evidence

### Audit summary from `.factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md`

```markdown
## Audit Summary

- **Total Tier 1 hooks audited:** 9
- **Hooks with Gap-Found = Y (no existing BC):** 0
- **Hooks with Gap-Found = N (BC confirmed):** 9
- **Hooks with Spec-Current = N (BC exists but stale):** 0
- **EC-005 OQ-6 deferred entries:** 1 (regression-gate)
- **AC-5 >5-gap threshold triggered:** No (0 of 9)
- **New BCs drafted:** 0
- **S-8.09 BC-anchor confirmation deferred:** Yes (OQ-6 unresolved)
```

### AC-5 threshold assessment

- **Gaps found:** 0 of 9
- **>5 threshold triggered:** No (0 < 5)
- **OQ-9 filing required:** No
- **W-16 deferral evaluation required:** No
- **New BCs drafted:** 0
- **Story point bumps triggered:** 0 (no +1pt BC-creation overhead for any S-8.01..S-8.09)

### Narrative

All 9 Tier 1 hooks have pre-existing BC coverage in the BC-7.03 and BC-7.04 families. The E-8 D-2 Option C BC strategy (reuse existing BCs, no new migration family) is fully satisfied. The one deferred entry (regression-gate / OQ-6) is not a Gap-Found = Y entry — existing BCs (BC-7.01.003, BC-7.03.071..075) cover the hook's current bash behavior; the deferral applies only to the downstream question of WASM port subprocess capability profile, which is S-8.09's responsibility.

**Result:** AC-5 SATISFIED. 0 of 9 Tier 1 hooks have Gap-Found = Y. The >5 threshold was not triggered. 0 new BCs drafted. No OQ-9 filed. No story point bumps required.
