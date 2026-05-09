---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 29
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T23:00:00Z
strategic_recommendation: continue-protocol-per-user-directive
---

# F5 Pass-29 Adversary Review

## Verdict

**HIGH** — 12th consecutive HIGH. 2H + 2M. ADR-013 clock RESETS to 0_of_3.

## Findings

### F-P29-001 [HIGH] VP-074 carries `proof_method: kani` — L-P28-001 retroactive verification incomplete (META self-application failure)
- VP-074.md:18 frontmatter: `proof_method: kani` (should be kani-proof per L-P28-001)
- L-P28-001 verification block at lessons.md:528 deliberately enumerated subsystem/status/priority but EXCLUDED proof_method (the field the lesson is about).
- 1-second `grep '^proof_method: kani$' .factory/specs/verification-properties/` finds VP-074 unfixed.
- **Fix:** VP-074 frontmatter `proof_method: kani` → `kani-proof`. Bump version + Amendment.

### F-P29-002 [HIGH] STATE.md Story Status arithmetic doesn't balance
- Header claims "93 total" but bucket sum = 89.
- Partial (1) undercounts: actual partial = 2 (S-2.05 + S-3.04).
- Draft (25) overcounts: actual draft files = 23. Includes unauthored stub IDs S-9.01..S-9.07 + S-11.01..S-11.08.
- **Fix:** Reconcile counts with filesystem reality. Either drop stub IDs from Draft bucket or document them as "unauthored stubs".

### F-P29-003 [MEDIUM] L-P26-002 sentinel sub-rule amendment lacks verification block per L-P26-001
- Fix-burst-27 sub-burst 2 added the sentinel sub-rule but no fresh verification block enumerating which 21 stories were validated.
- L-P26-001 mandates verification block on substantive amendments.
- **Fix:** add verification block citing the 21 pre-PR-6 stories using `merged_in: none`.

### F-P29-004 [MEDIUM] VP-070 missing `last_amended` while sibling VP-071 has it
- VP-070.md frontmatter: no `last_amended` field
- VP-071.md frontmatter: `last_amended: 2026-05-09`
- Same fix-burst, same change, asymmetric treatment.
- **Fix:** add `last_amended: 2026-05-09` to VP-070 frontmatter.

## Notable observations

- Fix-burst-27 closures all VERIFIED for the trigger artifacts.
- Index versions: BC-INDEX v1.51, VP-INDEX v1.36, STORY-INDEX v2.56, ARCH-INDEX v1.31.

## Convergence assessment

12 consecutive HIGH. F-P29-001 is the META self-application failure of L-P28-001. Per user directive: continue protocol.
