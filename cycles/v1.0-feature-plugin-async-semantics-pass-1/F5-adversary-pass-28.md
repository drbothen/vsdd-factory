---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 28
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T22:00:00Z
strategic_recommendation: continue-protocol-per-user-directive
---

# F5 Pass-28 Adversary Review

## Verdict

**HIGH** — 11th consecutive HIGH. 1H + 2M + 2L. ADR-013 clock RESETS to 0_of_3.

NEW LAYER: F-P27-005 closure was incomplete — VP-INDEX harmonized to `kani-proof` but VP-070.md and VP-071.md source frontmatter still say `proof_method: kani`. Same META-META class as P26/P27 but at field-value level (not symbol level).

## Findings

### F-P28-001 [HIGH] F-P27-005 closure incomplete — VP-INDEX harmonized, source VP files NOT updated
- VP-INDEX:191,192 say "kani-proof" for VP-070/VP-071.
- VP-070.md:17 frontmatter: `proof_method: kani` (wrong).
- VP-071.md:17 frontmatter: `proof_method: kani` (wrong).
- VP-077.md:19 frontmatter: `proof_method: kani-proof` (correct).
- POLICY 9 + POLICY 4 violation. Field-value drift between index and source.
- **Fix:** VP-070.md:17 + VP-071.md:17 → `proof_method: kani-proof`. Bump VP-070 v1.2→v1.3, VP-071 v1.0→v1.1.

### F-P28-002 [MEDIUM] L-P26-002 schema doesn't define `merged_in: none` sentinel
- 21 historic merged stories use `merged_in: none` (sub-burst 1 of fix-burst-26 default for pre-PR merges).
- L-P26-002 schema only allows `PR-NNN` form.
- **Fix:** add migration clause to L-P26-002 documenting `merged_in: none` sentinel for pre-GitHub merges.

### F-P28-003 [MEDIUM] STATE.md "Merged (63)" miscount — S-3.04 reclassified partial
- STATE.md:122 lists "57 stories + 6 = 63" but S-3.04:7 says `status: partial`, `superseded_by: ADR-015`.
- **Fix:** STATE.md → "Merged (62)" with note excluding S-3.04.

### F-P28-004 [LOW] hooks-registry.toml header docstring stale — claims "every entry routes through legacy-bash-adapter" but only 35/56 do
- 35 entries route via legacy-bash-adapter.wasm; 21 use direct native WASM plugins.
- VP-043 title may also be stale.
- **Fix:** edit docstring to "Most entries"; audit VP-043 title vs current registry.

### F-P28-005 [LOW process-gap] POLICY 8 verification steps reference `bcs:` but corpus uses `behavioral_contracts:`
- 1 story uses `bcs:` (S-15.03 only); 60+ use `behavioral_contracts:`.
- POLICY 8 verification steps are inactionable as written.
- **Fix:** edit policies.yaml POLICY 8 to use `behavioral_contracts:` or alias clause.

## Notable observations

- F-P27-001..007 closures all VERIFIED.
- Index versions: BC-INDEX v1.51 / VP-INDEX v1.35 / STORY-INDEX v2.56 / ARCH-INDEX v1.31.
- POLICY 6 + 9 arithmetic ALIGNS.
- STATE.md at 196 lines (within 200 budget; trending up).

## Convergence assessment

11 consecutive HIGH. New layer: source frontmatter field-value drift. Per user directive: continue protocol.
