---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T20:00:00Z
phase: 13
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 13
previous_review: adv-s21.09-local-pass-12.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 13)

**Verdict: NOT CLEAN — 1 HIGH. Streak → 0/3.**
**Reviewed commit: `a922ad82` (feature/S-21.09)**
**LOCAL streak: 0/3 — thirteen passes, zero CLEAN**
**D-chain: D-975**

Reviewed: story v1.23 against implementation at `.worktrees/S-21.09` @ `a922ad82`, governing BC-4.16.001 v1.8, full POLICY 1–22 rubric. Fresh-context per Iron Law — read only `adv-s21.09-local-pass-12.md` Part A. **NOT a CLEAN pass.** Streak resets to 0/3.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P13` for this pass
- `<SEQ>`: Three-digit sequence

This pass's dispatch used the local finding label `F-1` (as sourced from the adversary's dispatch report); the canonical cross-reference to the `ADV-BB-P13-<SEV>-NNN` form is: `F-1` = `ADV-BB-P13-HIGH-001`. Both forms are retained verbatim below — the `F-N` label in the finding body is NOT renamed, per D-448(a) source-attestation parity with the dispatched review text.

---

## Part A — Fix Verification (pass-12 carry-over items)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P12-HIGH-001 (pass-12 F-1) | HIGH | RESOLVED | T-051 prefix-conjunct isolation control added (`a922ad82`), orthogonal sibling of pass-11's T-050. This pass independently re-verified the underlying gate/mutation machinery is sound — "no surviving-mutant, no fail-open, no path-normalization edge gap" (see Method note below). No residual finding on either conjunct of the two-conjunct `in_repo` predicate. |
| ADV-BB-P10-MED-001 | MEDIUM | UNRESOLVED — not addressed this pass | `hook-plugins/sub/` directory-only staging control remains open; not re-verified in this dispatch. Carries to pass-14. |
| ADV-BB-P10-LOW-001 | LOW | UNRESOLVED — not addressed this pass | NUL byte / trailing-space name admission remains open; not re-verified in this dispatch. Carries to pass-14. |
| ADV-BB-P10-LOW-002 | LOW | UNRESOLVED — not addressed this pass | Fail-open arms guarded only by unasserted call ordering remain open; not re-verified in this dispatch. Carries to pass-14. |
| ADV-BB-P10-LOW-003 | LOW | UNRESOLVED — not addressed this pass | `workspace_root()` untested directly remains open; not re-verified in this dispatch. Carries to pass-14. |

---

## Part B — New Findings

> The section below is the adversary's dispatched review text, persisted verbatim per D-448(a) source-attestation parity (standard template scaffolding only; finding body not paraphrased).

# Adversarial Review — S-21.09 LOCAL cascade (fresh-context pass)

Artifacts: story S-21.09-wasm-artifact-restore-and-registry-parity.md v1.23; worktree .worktrees/S-21.09 @ a922ad82; crates/factory-dispatcher/tests/bundle_orphan_check.rs (46 tests T-006..T-051); BC-4.16.001 v1.8. Policies 1–13+ applied.

## Method note
The functions named in the dispatch prompt (detect_ungated_declarations, extract_hook_plugin_name, run_t012_gate, parse_plugin_refs, check_declared_subset_tracked, workspace_root, lex_norm, git_tracked/committed_wasm_names) live in the test crate bundle_orphan_check.rs, not in registry.rs. Only Registry::parse_str is production (registry.rs). The story's Architecture Mapping correctly states this, so the prompt's "in registry.rs" framing is a prompt inaccuracy, not a story defect.

I verified the two conjuncts of the in_repo containment predicate are genuinely isolated: T-050 (plugin="../.." → joined_parts == root_parts, length conjunct sole determinant, negative-identifier assertion !err.contains("UNGATED-DECLARATION: ") kills M2) and T-051 (plugin="../../../sib/ghost.wasm", prefix conjunct sole determinant, kills .all→.any and .all→true). The colon-vs-equals distinction between the finding lines (OUTSIDE-REPO-DECLARATION: <path>) and the explanatory prose (OUTSIDE-REPO-DECLARATION = resolves…) makes the negative-identifier assertions load-bearing and correct. The three extract_hook_plugin_name gates are each isolated (T-033 gate-1 +2 boundary, T-026(b) gate-2 prefix loop, T-035 gate-3 hook-plugins component; T-031 eq_ignore_ascii_case). Floors are boundary-pinned (T-016 @30 pass / T-027 @29 fire / T-019 @1 fire; T-022 resolvers is_empty). The AC-001 WASM (validate-factory-path-staging.wasm) is present on disk in the worktree; git-tracking is transitively bound by T-012 (git ls-files against the real root) — cannot run git (read-only), but that is the binding check, not a gap. The mutation-coverage and gate logic are genuinely sound — no surviving-mutant, no fail-open, no path-normalization edge gap.

The one real defect is a count-parity / propagation gap.

## HIGH Findings

### F-1 [HIGH] — Stale test-range cite T-012..T-050 / T-013..T-050 omits T-051 (POLICY 13 / count-parity; Partial-Fix Regression Discipline)
Location:
- Story S-21.09-...md line 475 — AC-006 "Tests:" bullet: "…run_t012_gate(&workspace_root())) plus T-013..T-050 in crates/factory-dispatcher/tests/bundle_orphan_check.rs."
- Test file bundle_orphan_check.rs line 128 (module docstring): "Stories: S-19.04 (T-006..T-011), S-21.09 (T-012..T-050)".

Failure scenario: T-051 (test_S_21_09_ac006_T051_prefix_conjunct_isolation_kills_all_mutants) was added in the pass-12 fix-burst and IS an S-21.09/AC-006-owned test (test-plan table row, story line 61; described story lines 611–622). The range extension to T-051 propagated to story lines 418, 623, 665 (all correctly T-012..T-051) but NOT to story line 475 or test-file line 128, which still terminate at T-050. A reader auditing S-21.09's owned coverage from either the AC-006 "Tests:" bullet or the test-module ownership header would conclude the suite ends at T-050 and miss the prefix-conjunct isolation control entirely — precisely the coverage-audit failure mode count-parity integrity guards against.

Why HIGH: Under the Partial-Fix Regression Discipline severity table, "fix applied to primary, sibling not updated" with blast radius = 2+ files is HIGH. The stale value appears in two distinct files (story .md + test .rs), each of which also contains the corrected form elsewhere — a genuine intra-file propagation gap in both. Impact is documentary (correct range present elsewhere), so the orchestrator may adjudicate down, but the mechanical 2-file rule and the coverage-audit risk justify HIGH.

Minimal fix route: Change story line 475 T-013..T-050 → T-013..T-051; change test-file line 128 T-012..T-050 → T-012..T-051. No code change. (Owner: story-writer for the story line; test-file docstring is an implementer/test-writer edit — route via orchestrator.)

## Observations (non-blocking)
- [process-gap] BC-4.16.001 §Verification Properties carries 4 rows all reading "(TBD — to be assigned by state-manager after VP authoring pass)", so the governing guard behavior has zero allocated formal verification properties. The story discloses this (verification_properties: [], §BC Status) and routes it via §Routing Proposals. This is a BC-level gap outside S-21.09's per-story perimeter (VP authoring is not an ops-story deliverable), so it does not block this story's convergence — but the unallocated-VP state on an active security-boundary BC should not persist indefinitely. Flagging for the wave/phase perimeter.
- POLICY 11 (evaluated, NOT a violation): Tests T-015/T-016/T-019/T-020/T-021/T-022 assert on self-constructed HashSets, but they call the real shared gate check_declared_subset_tracked (extracted per TD-VSDD-059 so fixtures exercise the production gate, not replicas), and the top-level T-012 runs run_t012_gate against real registries + real git + the production Registry::parse_str. Correct extract-and-exercise pattern, not a tautology. No finding.
- BC-4.16.001 frontmatter status: draft (line 5) conflicts with lifecycle_status: active (line 22). The story explicitly adjudicates this as stale metadata (§BC Status). Disclosed; no finding. POLICY 7 H1↔BC-INDEX↔story-body-title sync verified clean; POLICY 6 subsystem SS-04 verified clean.
- detect_ungated_declarations fail-opens (return Vec::new()) on registry read/parse failure, but in run_t012_gate both registries are already validated by Registry::parse_str / toml::Value::parse before detect_ungated_declarations runs, so the fail-open arm is unreachable in the gate path. No finding.

## Novelty Assessment
Novelty: MEDIUM. The gate/mutation machinery is exceptionally well-covered and I add nothing there. The single substantive finding (F-1) is a genuine cross-file stale-range propagation gap that index-level and mutation-focused checks do not surface.

## Verdict
NOT CLEAN. One HIGH finding (F-1). Per BC-5.39.001, the LOCAL 3-CLEAN streak remains 0/3. Remediation is a two-line range-cite correction (story line 475, test line 128); no logic change required.

---

## VP-TBD Observation Linkage

The [process-gap] observation above (BC-4.16.001 §Verification Properties carries 4 TBD rows — zero allocated VPs on an active security-boundary BC) is **already tracked** as Drift Item **[D-945] VP-102..VP-118 pending allocation (anchored S-21.07 post-merge)** in STATE.md §Drift Items / Tech Debt. This pass's observation maps directly onto that existing entry; no new drift item is opened.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 0 |

**Overall Assessment:** block
**Convergence:** 1 HIGH (F-1, new — cross-file stale test-range cite omitting T-051); gate/mutation machinery independently re-confirmed sound (no surviving-mutant, no fail-open, no path-normalization edge gap); pass-12's F-1 verified CLOSED; 4 pass-10 carry-over items (MED-001, LOW-001/002/003) remain open, not re-addressed this pass — see Part A.
**Readiness:** requires revision

---

## Novelty Assessment (tabular)

| Field | Value |
|-------|-------|
| **Pass** | 13 |
| **Story version reviewed** | v1.23 |
| **Reviewed commit** | a922ad82 |
| **New findings** | 1 (F-1 HIGH) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1 / (1 + 0) = 1.00 |
| **Median severity** | HIGH |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3→2→1→3→1→1→1 |
| **Total finding trajectory** | 3→3→2→13→11→9→9→8→8→15→2→1→1 (pass-1: 3; pass-2: 3; pass-3: 2; pass-4: 13; pass-5: 11; pass-6: 9; pass-7: 9; pass-8: 8; pass-9: 8; pass-10: 15; pass-11: 2; pass-12: 1; pass-13: 1) |
| **Verdict** | FINDINGS_REMAIN |

The F-1 finding is a genuine cross-file stale-range propagation gap (POLICY 5 / TD-VSDD-060 sibling-sweep class) that the pass-12 fix-burst's own sweep (5 sites: Architecture Mapping, Purity Classification, Architecture Compliance Rules, File Structure Requirements, Token Budget Estimate) did not include — the AC-006 "Tests:" bullet and the test-file module docstring were not in that sweep's target list.

## Verdict (restated per template)

NOT CLEAN — 1 HIGH. Streak → 0/3.
