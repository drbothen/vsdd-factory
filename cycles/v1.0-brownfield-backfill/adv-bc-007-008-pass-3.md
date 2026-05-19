---
document_type: adversary-review
level: ops
version: "1.0"
status: complete
producer: adversary
verifier: orchestrator
timestamp: 2026-05-19
phase: m3-bc-cascade-pass-3
cycle: v1.0-brownfield-backfill
streak: "0/3"
verdict: CRITICAL
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
input-hash: "c28758d"
traces_to: STATE.md
---

# Adversarial Review — BC-5.39.006 + BC-5.39.007 + BC-5.39.008 Pass-3 (M3 BC Cascade)

## ORCHESTRATOR-VERIFIED OVERRIDES

> These overrides are prepended by the orchestrator ABOVE the adversary's Part A findings.
> They were verified via literal shell execution BEFORE this persistence dispatch (per D-449(a)).

### Override 1: F-BC006P3-001 VERIFIED CRITICAL

**Adversary claim:** BC-5.39.006 v1.4 sibling-sweep is INCOMPLETE. PO replaced 16× `HookResult::BlockWithFix` but bare `BlockWithFix` (CamelCase, without prefix) survives across EC tables, VP tables, D-NNN Anchor Coverage, and rationale text.

**Orchestrator verification (literal shell):**

```
$ grep -cE 'BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
28

$ grep -cE 'HookResult::BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md
0
```

**Result: VERIFIED CRITICAL.** 28 total `BlockWithFix` occurrences remain, 0 are in the correctly-prefixed `HookResult::BlockWithFix` form. All 28 are bare `BlockWithFix` residuals — a non-existent SDK construct. The PO changelog at v1.4 claims "16 occurrences replaced" — this was the `HookResult::BlockWithFix` (prefixed) form. The separate bare `BlockWithFix` class (28 occurrences) was not swept. TDD authors following EC table rows would write tests for a non-existent SDK construct. CRITICAL because the defect class is structurally identical to F-BC008P2-002 (false factual claim about SDK API surface) but in a different syntactic form.

Note: Adversary reported "17+"; orchestrator-verified count is 28 bare residuals, 0 prefixed-form residuals.

### Override 2: F-BC007P3-001 VERIFIED HIGH

**Adversary claim:** BC-5.39.007 v1.2 D-NNN Anchor Coverage table cites retired/non-existent PC anchors. Line 396 cites "PC3/PC8" (PC8 does not exist; PC2 was split by pass-1 fix into PC2a/PC2b) and line 401 cites "PC1/PC2" (PC2 was retired by the pass-1 PC2a/PC2b split).

**Orchestrator verification (literal shell):**

```
$ grep -n 'PC3/PC8\|PC1/PC2' .factory/specs/behavioral-contracts/ss-05/BC-5.39.007.md
396:| D-419(c) | `(per D-413(b) completeness mandate)` annotation form blocked in Closes cites | PC3/PC8 |
401:| D-448(b) | Lesson entries in lessons.md MUST have `**Closes:**` bold-prefix line | PC1/PC2 |
```

**Result: VERIFIED HIGH.** PCs are PC1, PC2a, PC2b, PC3, PC4, PC5, PC6, PC7 (PC2 was retired by pass-1 split; PC8 does not exist). Line 396 cites "PC3/PC8" (PC8 non-existent) and line 401 cites "PC1/PC2" (PC2 retired). The F-BC007P2-003 closure in pass-2 applied the renumber to the Phase-1 boundary table + Test Vectors but did NOT propagate to the D-NNN Anchor Coverage table. Sibling-sweep-of-own-fix regression.

### Override 3: F-BC008P3-001 VERIFIED HIGH

**Adversary claim:** BC-5.39.008 v1.2 D-NNN Anchor Coverage table mis-anchors POLICY 13 → PC3/PC6 and POLICY 16 → PC3/PC7. PC3 in BC-5.39.008 is "tool_input.content is not source of truth" (semantically unrelated to lint_hook/codified_at validation). POLICY 4 semantic-anchoring-integrity violation.

**Orchestrator verification (literal shell):**

```
$ grep -n 'POLICY 13\|POLICY 16' .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
401:| POLICY 13 | `lint_hook` field required per POLICY 13 codification at D-472 | PC3/PC6 |
402:| POLICY 16 | `codified_at` field required per POLICY 16 codification at D-472 | PC3/PC7 |

$ sed -n '88,93p' .factory/specs/behavioral-contracts/ss-05/BC-5.39.008.md
3. The file content is read via `host::read_file`. The hook does NOT inspect
   `tool_input.content`; the filesystem value is the source of truth for validation.
```

**Result: VERIFIED HIGH.** PC3 is "tool_input.content is not source of truth for validation." POLICY 13 (lint_hook field required) enforces postcondition 6 (the lint_hook validation block), not PC3. POLICY 16 (codified_at field required) enforces postcondition 7 (the codified_at coupling rationale), not PC3. The correct postcondition citations are PC6 and PC7 (postconditions 6 and 7 respectively, where these enforcement behaviors are defined), not PC3 at all. Citing PC3 means implementers would write test fixtures for "file-read-methodology" when the actual gate being tested is "schema field validation."

### Override 4: Net Blocking Status

- 1 verified CRITICAL finding: F-BC006P3-001 (28 bare BlockWithFix residual in BC-5.39.006 v1.4)
- 2 verified HIGH findings: F-BC007P3-001 (BC-5.39.007 D-NNN Anchor Coverage retired PC anchors) + F-BC008P3-001 (BC-5.39.008 D-NNN Anchor Coverage mis-anchors)
- 5 additional MEDIUM/LOW/NITPICK findings
- **STREAK: 0/3 → 0/3 RESET** (1 verified CRITICAL prevents advance; cascade continues to pass-4 required per BC-5.39.001 3-CLEAN protocol)

---

## PART A — Adversary Findings

### Finding Counts (pass-3)

| Pass | Findings (BC-006) | Findings (BC-007) | Findings (BC-008) | Total | Streak |
|------|-------------------|-------------------|-------------------|-------|--------|
| Pass-1 | ~0 | ~21 | ~20 | ~41 | 0/3 |
| Pass-2 | 1 (HIGH) | 7 | 7 | 14 | 0/3 |
| Pass-3 | 3 (1C+1M+1NIT) | 2 (1H+1M) | 2 (1H+1L) | 8 (per finding IDs below) | 0/3 |

> Note: Pass-3 introduces BC-5.39.006 as a primary review target (not merely sibling). F-BC008P3-001 (HIGH) and F-BC008P3-003 (LOW cross-BC cite) are both BC-008 namespace; no ID collision with adversary F-BC008P3-001 designation — the adversary's original HIGH finding is retained at this ID; the LOW is numbered F-BC008P3-003 per orchestrator renaming convention (see PART A LOW findings). F-BC008P3-002 is the separate PC4 over-specification finding.

---

### CRITICAL Findings (1)

**F-BC006P3-001 — CRITICAL — BC-5.39.006 v1.4 sibling-sweep INCOMPLETE: bare `BlockWithFix` (without `HookResult::` prefix) survives 28 times across EC tables, VP tables, D-NNN Anchor Coverage, and rationale text**

BC-5.39.006 v1.4 changelog row documents that the PO replaced 16 occurrences of `HookResult::BlockWithFix` with `HookResult::block_with_fix(...)`. This was a narrow sweep targeting the fully-prefixed `HookResult::BlockWithFix` pattern. The broader semantic class — bare `BlockWithFix` (CamelCase, without the `HookResult::` prefix) — was not swept. 28 such occurrences remain.

These 28 bare `BlockWithFix` tokens appear in EC table "Expected Result" cells, VP table "Property" descriptions, D-NNN Anchor Coverage "Gate Enforced" text, and rationale paragraphs. A TDD test author following any EC table row would write:

```rust
assert_eq!(result, HookResult::BlockWithFix { ... })  // or similar
```

`BlockWithFix` is not a variant of `HookResult`. The enum has only `Continue`, `Block { reason: String }`, and `Error { message: String }`. Test authors following the EC table rows would write tests that do not compile.

Orchestrator-verified evidence: see Override 1 above.

**Severity: CRITICAL** (same class as F-BC008P2-002 — false factual claim about SDK API surface; TDD authors would write structurally broken tests).

---

### HIGH Findings (2)

**F-BC007P3-001 — HIGH — BC-5.39.007 v1.2 D-NNN Anchor Coverage table cites retired PC2 and non-existent PC8**

The F-BC007P2-003 closure at pass-2 was a partial fix: PC2 → PC2a/PC2b renumber was applied in the Preconditions section, the Phase-1 boundary table, and the Test Vectors. The D-NNN Anchor Coverage table was not updated.

- Line 396: `| D-419(c) | ... | PC3/PC8 |` — PC8 does not exist in BC-5.39.007 v1.2. PCs are PC1, PC2a, PC2b, PC3, PC4, PC5, PC6, PC7.
- Line 401: `| D-448(b) | Lesson entries in lessons.md MUST have **Closes:** bold-prefix line | PC1/PC2 |` — PC2 was retired by the pass-1 PC2a/PC2b split; the correct reference is PC2a (marker presence) or PC2a/PC2b depending on the intended enforcement site.

D-448(b) is the canonical lessons.md Closes-presence gate — the most critical anchor in the table. Mis-anchoring its enforcement site to a retired "PC2" is worse than mis-anchoring a secondary row. An implementer reading this table would not know which precondition the gate validates against.

Orchestrator-verified evidence: see Override 2 above.

**Severity: HIGH** (sibling-sweep-of-own-fix regression; F-BC007P2-003 closure incomplete).

---

**F-BC008P3-001 — HIGH — BC-5.39.008 v1.2 D-NNN Anchor Coverage table cites PC3 for POLICY 13 and POLICY 16 enforcement — but PC3 is "tool_input.content is not source of truth" (semantically unrelated)**

The D-NNN Anchor Coverage table rows for POLICY 13 and POLICY 16:

```
| POLICY 13 | `lint_hook` field required per POLICY 13 codification at D-472 | PC3/PC6 |
| POLICY 16 | `codified_at` field required per POLICY 16 codification at D-472 | PC3/PC7 |
```

PC3 in BC-5.39.008 is: "The file content is read via `host::read_file`. The hook does NOT inspect `tool_input.content`; the filesystem value is the source of truth for validation."

PC3 has no semantic relationship to `lint_hook` or `codified_at` field-presence validation. The lint_hook enforcement gate is postcondition 6; the codified_at enforcement gate is postcondition 7. The correct postcondition citations for these POLICY rows are:

- POLICY 13 → postconditions 3/6 (field-presence-required + lint_hook-existence check)
- POLICY 16 → postconditions 3/7 (field-presence-required + codified_at coupling)

An implementer following the D-NNN Anchor Coverage table to write enforcement tests for POLICY 13 would write a test for the file-read methodology (PC3), not for the field-schema validation (postcondition 3/6). This is a POLICY 4 semantic-anchoring-integrity violation.

Orchestrator-verified evidence: see Override 3 above.

**Severity: HIGH** (POLICY 4 violation; implementer-facing test-routing defect).

---

### MEDIUM Findings (2)

**F-BC006P3-002 — MEDIUM — BC-5.39.006 v1.4 changelog row line 383 has self-referential typo: replace-target and replacement are textually identical**

The v1.4 changelog row (line 383) reads:

> `HookResult::block_with_fix(...)` variant non-existence — 16 occurrences replaced with canonical `HookResult::block_with_fix(...)`

Both the replace-target description and the replacement description are `HookResult::block_with_fix(...)`. The intended wording was:

> `HookResult::BlockWithFix` variant non-existence — 16 occurrences replaced with canonical `HookResult::block_with_fix(...)`

The replace-target was `HookResult::BlockWithFix` (the non-existent CamelCase variant form). This typo undermines the traceability of exactly what was changed in the v1.4 sweep. POLICY 15 LL-N verbatim quality defect.

**Severity: MEDIUM** (POLICY 15 LL-N; traceability defect in changelog row).

---

**F-BC007P3-002 — MEDIUM — F-BC007P3-001 refinement: D-448(b) row specifically mis-anchors the canonical lessons.md Closes-presence gate to retired PC1/PC2**

This is a refinement of F-BC007P3-001 calling out the D-448(b) row specifically because of its critical status. D-448(b) is not an ordinary D-NNN sub-clause — it is the canonical enforcement anchor for the `**Closes:**` bold-prefix-line requirement that was the subject of the CRITICAL finding F-BC007P1-001 at pass-1. The D-448(b) row in the D-NNN Anchor Coverage table:

```
| D-448(b) | Lesson entries in lessons.md MUST have `**Closes:**` bold-prefix line | PC1/PC2 |
```

`PC1/PC2` is the pre-split retired naming. PC2 was split into PC2a (trajectory-tail marker presence) + PC2b (LENGTH == 4). The D-448(b) enforcement site is the Closes-block structure validation — this corresponds to PC2a semantics (presence of the structural element). Mis-anchoring D-448(b) to the retired "PC2" label means an implementer looking for the canonical Closes-enforcement site would route to a non-existent precondition.

**Severity: MEDIUM** (specific D-448(b) anchor defect; POLICY 4 + POLICY 15).

---

### LOW Findings (2)

**F-BC008P3-003 — LOW — Cross-BC closure citation inconsistency: BC-5.39.008 v1.2 frontmatter closes `F-BC007P2-006` (a BC-007 namespace finding)**

BC-5.39.008 v1.2 changelog or frontmatter cites `F-BC007P2-006` as a finding it closes. F-BC007P2-006 is in the BC-5.39.007 namespace. While this is mechanically defensible (the pass-2 finding was promoted by orchestrator from a BC-008 finding), it creates a cross-namespace citation in BC-5.39.008 frontmatter. A reader scanning BC-5.39.008 closures would not expect to find BC-007-namespace IDs. The correct tracking home is BC-5.39.007.

**Severity: LOW** (tracking inconsistency; not load-bearing).

---

**F-BC008P3-002 — LOW — BC-5.39.008 v1.2 PC4 `[1, 999]` integer range is over-specified without rationale**

BC-5.39.008 v1.2 PC4 (postcondition 4) specifies that POLICY IDs must be in range `[1, 999]`. Production policies.yaml uses ids 1..18. The range [1, 999] is over-specified: it has no direct grounding in any documented invariant, D-NNN decision, or POLICY-format canonical source. A range this wide (3 orders of magnitude beyond current usage) without rationale is a Production-grade-default Rule 1 defer-rationalization smell — the spec appears to have chosen a "safe" round number rather than the production-grounded value.

If the intent is "any positive YAML integer" (no upper bound), the BC should say so with rationale. If the intent is "three-digit ceiling per POLICY-NNN three-digit ID format," the upper bound should be 999 with explicit rationale citing the three-digit-ID canonical form (POLICY 15).

**Severity: LOW** (over-specification without rationale; not load-bearing for correctness).

---

### NITPICK Findings (1)

**F-BC006P3-NIT — NITPICK — BC-5.39.006 v1.4 frontmatter `modified:` array tripartite-parity compliance observation**

The BC-5.39.006 v1.4 frontmatter `modified:` array lists amendment history. For tripartite-parity compliance (BC content + story AC + implementation), the modified array should trace to co-amended story-version and implementation-commit when the BC amendment is substantive. This is an observation only — no structural gap relative to the current version's schema — but worth noting for future amendments.

**Severity: NITPICK** (no defect; advisory observation only).

---

## PART B — Adversary Meta-Assessment

### Streak Status

**STREAK: 0/3 CLEAN** (1 verified CRITICAL finding prevents advance; cascade to pass-4 required per BC-5.39.001 3-CLEAN protocol).

### Novelty Calibration

- F-BC006P3-001 CRITICAL: The v1.4 PO fix was a NARROW sweep (16 occurrences of the prefixed `HookResult::BlockWithFix` form). The BROADER semantic class (bare `BlockWithFix` without prefix) was not identified. This is a new finding not detectable by INV-017 alone — INV-017 requires embedded grep stdout in the changelog row, and the PO DID embed such stdout (showing 0 residual `HookResult::BlockWithFix`). However, the grep was narrow-pattern-only. The broader residual class (bare `BlockWithFix`) still survived 28 times.

- F-BC007P3-001 + F-BC007P3-002 HIGH: Sibling-sweep-of-own-fix regression of F-BC007P2-003. The pass-2 fix applied renumber to 3 of 4 consumer sites (Preconditions section, Phase-1 boundary table, Test Vectors); the D-NNN Anchor Coverage table was missed. This is the same class as F-BC007P2-003 itself (sibling-sweep-of-own-fix).

- F-BC008P3-001 HIGH: A static semantic-anchoring defect in BC-5.39.008 v1.2 (POLICY 13/16 anchor pointing to PC3 which is semantically unrelated). This was not caught at pass-2 because pass-2 focused on the CRITICAL findings (policies.yaml format + exec_subprocess) and the cascading fixes; the D-NNN Anchor Coverage table was not subjected to semantic-anchoring audit.

### CASCADE TRAJECTORY

| Pass | Total Findings | CRITICAL | HIGH | MEDIUM | LOW | NIT |
|------|---------------|----------|------|--------|-----|-----|
| Pass-1 | ~41 | 2 | ~17 | ~10 | ~10 | ~2 |
| Pass-2 | 14 | 2 | 4 | 5 | 3 | 1 |
| Pass-3 | 8 | 1 | 2 | 2 | 2 | 1 |

Trend: improving (41 → 14 → 8). CRITICAL count declining (2 → 2 → 1). HIGH count declining (17 → 4 → 2).

### META-LEVEL INV-018-CANDIDATE Detection

**META-LEVEL INV-018-CANDIDATE: "Per-fix-burst literal-shell evidence (INV-017) catches the NARROW pattern claimed by the changelog row but does NOT catch the BROADER semantic class. The discipline must include both a narrow-pattern post-fix grep AND a residual-class post-fix grep."**

The PO faithfully applied INV-017 at pass-2 (re-verified: all 6 embedded stdouts re-execute and match the claimed results). The INV-017 evidence for F-BC007P2-001 closure included:

```
grep -cE 'HookResult::BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md → 0
```

This grep returned 0. The PO correctly reported zero residual `HookResult::BlockWithFix` occurrences. The grep was for the NARROW pattern `HookResult::BlockWithFix` — the exact form being replaced. It did NOT check for the BROADER semantic class: bare `BlockWithFix` tokens without the `HookResult::` prefix.

The result: the grep evidence was accurate for the narrow pattern, but the broader residual class survived 28 times undetected. A changelog row claiming "16 occurrences of HookResult::BlockWithFix replaced with HookResult::block_with_fix" can be literally true (narrow-pattern count = 0 post-sweep) while 28 bare `BlockWithFix` tokens remain.

**INV-018 normative cure:** Every fix-burst grep evidence block for a "replace pattern X with pattern Y" closure MUST include BOTH:

1. **Narrow-pattern evidence:** `grep -nE '<exact-replaced-pattern>' <target>` → expected zero output (confirms the specific replaced form is gone)
2. **Residual-class sweep:** `grep -nE '<broader-semantic-class-pattern>' <target>` → expected zero output (confirms no broader-class tokens survive under any syntactic variant)

For the F-BC006P3-001 case, the required evidence would be:
```
# Narrow pattern (what was replaced):
$ grep -cE 'HookResult::BlockWithFix' BC-5.39.006.md → 0  (INV-017 satisfied)

# Residual-class sweep (broader class):
$ grep -cE 'BlockWithFix' BC-5.39.006.md → 0  (INV-018 required; was 28 at v1.4)
```

**Forward routing:** INV-018-CANDIDATE forwarded to SK-MCP-001 Appendix D for automation-engineering consideration. The cure is a dispatch-template requirement: for any "replace X with Y" closure, the changelog row evidence section MUST include both a narrow-pattern grep AND a residual-class grep, with both returning zero output (or explicit residual-listing if non-zero is acceptable).

**Relationship to prior META-LEVELs:** INV-015 (adversary-must-grep-canonical-source) → INV-016 (BC-authorship-must-grep-actual-artifact-format) → INV-017 (codified-discipline-must-be-shell-gate-not-narrative) → INV-018 (shell-gate-must-cover-narrow-AND-residual-class-sweep). Each layer reveals a structural insufficiency in the prior layer's cure.

---

## PART C — Policy Rubric Coverage

| Policy | ID | Coverage Status | Notes |
|--------|----|----------------|-------|
| `spec_is_authoritative_over_code` | POLICY 1 | APPLIES — OK (pass-3) | F-BC006P3-001 is a spec-internal error, not code conflict. Code has no `BlockWithFix`; spec claims it 28 times. Spec must be corrected. |
| `prd_is_requirements_source_of_truth` | POLICY 2 | NOT IN SCOPE | PRD not in scope of BC-006/007/008 pass-3 review. |
| `arch_index_is_subsystem_registry` | POLICY 3 | NOT IN SCOPE | ARCH-INDEX subsystem names not in scope. |
| `bc_h1_is_authoritative_title` | POLICY 4 | APPLIES — HIGH VIOLATION | F-BC008P3-001: D-NNN Anchor Coverage table cites PC3 for POLICY 13+16 enforcement but PC3 is semantically unrelated (file-read methodology). Semantic-anchoring-integrity violation. |
| `creators_justify_anchors` | POLICY 5 | APPLIES — OK | No new anchor justification gaps in pass-3. |
| `bc_index_is_bc_catalog_source_of_truth` | POLICY 6 | APPLIES — OK | BC-INDEX v2.39 unchanged; this is a state-manager persistence burst. |
| `bcs_frontmatter_array_changes_propagate_atomically` | POLICY 7 | APPLIES — PENDING (PO fix-burst pass-3) | PO fix-burst pass-3 must propagate any BC version bumps atomically. |
| `vp_index_is_vp_catalog_source_of_truth` | POLICY 8 | APPLIES — OK (pending) | VP allocations still pending architect dispatch per TD-VSDD-063. |
| `vp_changes_propagate_same_burst` | POLICY 9 | APPLIES — OK (pending) | Same as POLICY 8. |
| `story_index_is_story_catalog` | POLICY 10 | NOT IN SCOPE | No story changes this burst. |
| `no_silent_deferrals` | POLICY 11 | APPLIES — OK | All 8 findings explicitly enumerated; no silent deferrals. |
| `no_ai_attribution_in_commits` | POLICY 12 | APPLIES — OK | Commit message follows project convention. |
| `no_bypass_hook_chain` | POLICY 13 | APPLIES — OK | No `--no-verify` used. |
| `bc_authorship_must_grep_actual_artifact_format` | POLICY 14 | APPLIES — MEDIUM VIOLATION | INV-018 root cause: PO applied narrow-pattern grep per INV-017 but did not run residual-class sweep. INV-017 was satisfied (narrow grep → 0); INV-018 was not (broad grep → 28). POLICY 14's cure must now extend to BOTH patterns. |
| `three_digit_policy_id_canonical` | POLICY 15 | APPLIES — MEDIUM VIOLATION | F-BC006P3-002: changelog row typo — replace-target and replacement are textually identical strings, undermining LL-N verbatim quality. |
| `decisions_log_umbrella_range_auto_advance` | POLICY 16 | APPLIES — OK | D-range advanced to D-486 in this burst per D-452(e). |
| `sibling_sweep_on_value_changes` | POLICY 17 | APPLIES — OK | F-BC007P3-001 is a sibling-sweep-of-own-fix regression; POLICY 17 applies but the finding itself routes to PO fix-burst pass-3. |
| `production_grade_default` | POLICY 18 | APPLIES — OK (meta-level) | INV-018 is a production-grade systemic gap. All 8 findings are load-bearing or traceability defects; none are cosmetic-only. |

### INV-017 Evidence Verification Table (pass-3 audit of pass-2 PO fix-burst)

Per D-448(a) source-attestation gate, the orchestrator re-verified the 6 embedded stdouts from the BC-5.39.006/007/008 v1.4/v1.2/v1.2 changelog rows:

| Changelog Row Claim | Re-executed Grep | Result |
|--------------------|-----------------|---------| 
| `grep -cE 'HookResult::BlockWithFix' BC-5.39.006.md → 0` | `grep -cE 'HookResult::BlockWithFix' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md` | `0` — PASS (narrow pattern) |
| `grep -cE 'HookResult::Advisory' BC-5.39.006.md → 0` (implicit from changelog) | Confirmed at v1.4 | PASS |
| `grep -cE 'exec_subprocess IS' BC-5.39.008.md` (paraphrase; actual stdout embedded) | BC-5.39.008 PC10 rewrite | PASS |
| `grep -nE '^  - id:' policies.yaml → id: 1..5 format` (from BC-5.39.008 v1.2 rationale) | `grep -nE '^  - id:' .factory/policies.yaml | head -5` matches | PASS |
| BC-5.39.007 v1.2 PC2/PC5 renumber evidence in changelog | PC2a/PC2b in Preconditions, boundary table, Test Vectors verified | PASS |
| BC-5.39.007 v1.2 Phase-1 false-negative window bound | EC and Test Vectors updated | PASS |

**INV-017 verdict: ALL 6 embedded stdouts are faithful to the actual post-fix state. INV-017 was correctly applied. F-BC006P3-001 arises from INV-017's structural insufficiency (narrow-only sweep), not from INV-017 misapplication. INV-018 is a new layer, not a retroactive INV-017 failure.**
