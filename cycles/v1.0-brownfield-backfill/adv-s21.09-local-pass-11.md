---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T16:00:00Z
phase: 11
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "830be36"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 11
previous_review: adv-s21.09-local-pass-10.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 11)

**Verdict: NOT CLEAN — 1 HIGH + 1 MEDIUM. Streak → 0/3.**
**Reviewed commit: `1c59a669` (feature/S-21.09)**
**LOCAL streak: 0/3 — eleven passes, zero CLEAN**
**D-chain: D-972**

Reviewed: story v1.21 against implementation at `.worktrees/S-21.09` @ `1c59a669`, governing BC-4.16.001 v1.8, full POLICY 1–22 rubric. **NOT a CLEAN pass.** Streak resets to 0/3.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P11` for this pass
- `<SEQ>`: Three-digit sequence

This pass's dispatch used the local finding labels `F-1`/`F-2` (as sourced from the adversary's dispatch report); the canonical cross-reference to the `ADV-BB-P11-<SEV>-NNN` form is: `F-1` = `ADV-BB-P11-HIGH-001`; `F-2` = `ADV-BB-P11-MED-001`. Both forms are retained verbatim below — the `F-N` labels in the finding bodies are NOT renamed, per D-448(a) source-attestation parity with the dispatched review text.

---

## Part A — Fix Verification (pass-10 carry-over items)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P10-MED-001 | MEDIUM | UNRESOLVED — not addressed this pass | `hook-plugins/sub/` directory-only staging control remains open; not re-verified in this dispatch. Carries to pass-12. |
| ADV-BB-P10-MED-004 | MEDIUM | ESCALATED → HIGH (see F-1, this pass) | Pass-10 MED-004 first identified that T-047's boundary proof is over-determined for the length conjunct under M2. This pass's F-1 independently re-derives the same M2 non-isolation, additionally finding that T-048 and the story's own kill-claims (T-047/T-048/lines 528–529/644/645) assert the opposite — a false mutation-kill attestation across three shipped locations (TD-VSDD-059 paper-claim). Severity escalated MEDIUM → HIGH to reflect the compounding false-attestation defect. |
| ADV-BB-P10-LOW-001 | LOW | UNRESOLVED — not addressed this pass | NUL byte / trailing-space name admission remains open; not re-verified in this dispatch. Carries to pass-12. |
| ADV-BB-P10-LOW-002 | LOW | UNRESOLVED — not addressed this pass | Fail-open arms guarded only by unasserted call ordering remain open; not re-verified in this dispatch. Carries to pass-12. |
| ADV-BB-P10-LOW-003 | LOW | UNRESOLVED — not addressed this pass | `workspace_root()` untested directly remains open; not re-verified in this dispatch. Carries to pass-12. |

---

## Part B — New Findings

### HIGH

#### F-1 [POLICY 13 mutation-completeness / TD-VSDD-059 paper-claim] — The containment-predicate length conjunct (`>`) is never isolated; mutant "M2" survives despite explicit kill-claims in T-047, T-048, and the story

Location: `detect_ungated_declarations` (containment predicate), pinned by `test_S_21_09_ac006_T047_outside_repo_declaration_tightest_margin_fires` and `test_S_21_09_ac006_T048_totality_property_partition`; story Red-Gate rows T-047/T-048 and narrative lines 528–529 / 644 / 645.

The containment predicate is a two-conjunct AND: `in_repo = joined_parts.len() > root_parts.len() && root_parts.iter().enumerate().all(|(i, p)| joined_parts.get(i) == Some(p))`. Both the T-047 test (comment: "M2 (len>→>=) would cause it to pass containment and fire UNGATED instead") and the T-048 property test ("Kills M2 (len>→>=): outside-repo paths satisfy the relaxed containment check and would fire UNGATED instead of OUTSIDE"), plus the module-header docstring (T-048 row: "kills M2 (len>→>=)") and story line 645, claim the negative-identifier assertion kills the mutant that relaxes `>` to `>=`.

This claim is false. The sole OUTSIDE probe used to "prove" it is `../../../ghost.wasm`. Resolved from `registry_parent = root/plugins/vsdd-factory`, three ParentDir pops yield `joined_parts = [root[0]..root[n-2], "ghost.wasm"]`, so `joined_parts.len() == root_parts.len()` AND the prefix `.all()` fails at index n-1 (`"ghost.wasm"` ≠ the tempdir leaf name). Applying M2 (`>`→`>=`) makes the length conjunct true, but the prefix conjunct independently remains false, so `in_repo` stays false and the path is still classified OUTSIDE-REPO-DECLARATION. The mutant survives T-047 and T-048 unchanged. This is exactly the "over-determined boundary proof" pattern: at `../../../ghost.wasm` both conjuncts fail simultaneously, so neither is isolated. The story itself admits this at line 644 ("does not isolate which conjunct is individually operative — no control exists that satisfies the prefix conjunct while failing only the length conjunct") and then contradicts itself at line 645 / the T-048 row by asserting the negative-identifier check kills M2. To actually flip class under M2 you need a declaration resolving exactly to the worktree root (`joined_parts == root_parts`), i.e. a filename-less path — no such candidate exists in the 18-case table or anywhere in the suite.

Impact: label-only at a degenerate, filename-less input (the gate still returns Err either way), so production safety is not compromised — but the mutation-kill attestation is demonstrably false and replicated in three shipped locations inside a 3-CLEAN cascade whose purpose is precisely this class of gap.

Minimal fix: add a root-resolving, filename-less candidate — e.g. `("../..", "OUTSIDE-REPO-DECLARATION")` — to the T-048 partition (or a dedicated test). Under the live `>` code it classifies OUTSIDE; under M2 (`>=`) it becomes in_repo → delegates to extract_hook_plugin_name → gate-1 (len < expected_depth + 2) → None → UNGATED, so the negative-identifier assertion then genuinely fires RED. Correct the three narrative/comment sites accordingly and reconcile with the honest admission at story line 644. Confidence: HIGH.

### MEDIUM

#### F-2 [POLICY 4 semantic-anchoring / S-7.01 partial-fix propagation] — pass-9/pass-11 refactors (full-path return; removal of `is_hook_plugins`) did not propagate to sibling docstrings, a test name, and a story Red-Gate row, which now describe the opposite of the shipped behavior

- Module-header test-plan row T-032 (docstring): "yields `nested.wasm` (last component); proves non-flat declarations are not silently mis-named" — contradicts `test_S_21_09_ac006_T032_nested_subdir_yields_filename`, which asserts `refs.contains("hook-plugins/sub/nested.wasm")` AND `!refs.contains("nested.wasm")`. Story Red-Gate row (line 629) correctly says full path is returned — internal drift.
- Test function name `test_S_21_09_ac006_T032_nested_subdir_yields_filename` — "yields_filename" encodes obsolete basename semantics.
- Module-header test-plan row T-023 (docstring): "traversal/absolute forms now INCLUDED via lexical normalisation" — absolute forms are EXCLUDED (None / OUTSIDE-REPO-DECLARATION), contradicting T-026/T-047/T-048.
- `parse_plugin_refs` doc comment: "…uses `last()` as the filename." — current function returns `joined_parts[expected_depth..].join("/")` (full path), not `last()`.
- Story Red-Gate row T-043 (line 640) references `is_hook_plugins` failing — that symbol was removed in the pass-11 single-copy refactor and no longer exists.

Impact: actual tests are correct, so no verification hole — but a reader/implementer consulting the test-plan header would conclude T-032 verifies basename extraction (the opposite of reality) and would look for a deleted `is_hook_plugins` symbol. Mis-anchored descriptions block convergence under POLICY 4. Minimal fix: rewrite T-032/T-023 header rows and `parse_plugin_refs` doc for full-path return / absolute-exclusion; rename T-032 fn (append-only note); drop `is_hook_plugins` from story line 640. Confidence: HIGH.

---

## Observations (non-blocking)

- No duplicate T-038 (POLICY 1 append-only clean — every T-NNN maps to exactly one #[test] fn).
- POLICY 11 clean — all fixture-driven controls invoke real gate functions; no assert-on-self-constructed-data tautology.
- POLICY 7 clean — BC-4.16.001 H1 matches story Behavioral-Contracts-table Title verbatim.
- EC-005a/b, staged-not-committed, gitignored-probe, gate-3 isolation genuinely mutation-controlled; the three extract_hook_plugin_name gates ARE individually isolated; only the separate containment length conjunct is not (F-1).
- Read-only limitation: AC-001 requires the WASM in the git index; validate-factory-path-staging.wasm present on disk but index state unverifiable without shell; presumed satisfied (T-012 would fail otherwise) — flagged as un-verifiable-under-read-only, not a finding.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 1 |
| LOW | 0 |

**Overall Assessment:** block
**Convergence:** 1 HIGH (F-1, escalated from pass-10 MED-004) + 1 MEDIUM (F-2, new); 4 pass-10 carry-over items (MED-001, LOW-001, LOW-002, LOW-003) remain open, not re-addressed this pass — see Part A.
**Readiness:** requires revision

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 11 |
| **Story version reviewed** | v1.21 |
| **Reviewed commit** | 1c59a669 |
| **New findings** | 2 (F-1 HIGH; F-2 MEDIUM) |
| **Duplicate/variant findings** | 1 (F-1 is a substantive escalation-with-new-evidence of pass-10 MED-004, not a pure duplicate — counted as new per the false-attestation discovery it adds) |
| **Novelty score** | 2 / (2 + 0) = 1.00 |
| **Median severity** | HIGH/MEDIUM boundary |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3→2→1→3→1 |
| **Total finding trajectory** | 3→3→2→13→11→9→9→8→8→15→2 (pass-1: 3; pass-2: 3; pass-3: 2; pass-4: 13; pass-5: 11; pass-6: 9; pass-7: 9; pass-8: 8; pass-9: 8; pass-10: 15; pass-11: 2) |
| **Verdict** | FINDINGS_REMAIN |

Novelty HIGH — F-1 is a substantive mutation-completeness gap with a self-contradiction in the story text; F-2 is a genuine impl-vs-doc drift cluster from unswept refactors. Neither is a formatting nitpick.

## Verdict

NOT CLEAN — 1 HIGH + 1 MEDIUM. Streak → 0/3.
