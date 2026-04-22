# Step 3: Traceability Chain Extension

Update the traceability chain to include the new feature, appending new links.

## Inputs

- New requirements, stories, tests, implementations from Phases F2-F4
- Existing traceability chain at `.factory/cycles/**/convergence/traceability-chain.md`

## Actions

1. For each new requirement, build the full chain using 4-level hierarchy:
   ```
   BC-S.SS.NNN -> VP-NNN -> test_xxx -> src/xxx.rs -> ADV-PASS-N -> KANI-xxx-PASS
   ```
2. For cross-references (new feature depends on existing feature):
   ```
   BC-S.SS.NNN depends_on BC-S.SS.MMM (existing)
   STORY-NNN extends STORY-MMM (existing)
   ```
3. Write the extended traceability chain to `.factory/phase-f7-convergence/traceability-chain-delta.md`
4. APPEND (not replace) new links to `.factory/cycles/**/convergence/traceability-chain.md`

## Outputs

- `.factory/phase-f7-convergence/traceability-chain-delta.md`
- Updated `.factory/cycles/**/convergence/traceability-chain.md` (appended)

## Completion Criteria

- Every new requirement has a complete chain (no gaps)
- Cross-references link new features to existing features they depend on
- Main traceability chain is extended, not replaced
