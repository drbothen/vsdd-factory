---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T18:00:00Z
phase: 12
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "14dc258"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 12
previous_review: adv-s21.09-local-pass-11.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 12)

**Verdict: NOT CLEAN — 1 HIGH. Streak → 0/3.**
**Reviewed commit: `69663255` (feature/S-21.09)**
**LOCAL streak: 0/3 — twelve passes, zero CLEAN**
**D-chain: D-974**

Reviewed: story v1.22 against implementation at `.worktrees/S-21.09` @ `69663255`, governing BC-4.16.001 v1.8, full POLICY 1–22 rubric. Fresh-context per Iron Law — read only `adv-s21.09-local-pass-11.md` Part A. **NOT a CLEAN pass.** Streak resets to 0/3.

Artifacts: story spec v1.22, implementation worktree @ 69663255 (bundle_orphan_check.rs, src/registry.rs), governing BC-4.16.001 v1.8.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P12` for this pass
- `<SEQ>`: Three-digit sequence

This pass's dispatch used the local finding label `F-1` (as sourced from the adversary's dispatch report); the canonical cross-reference to the `ADV-BB-P12-<SEV>-NNN` form is: `F-1` = `ADV-BB-P12-HIGH-001`. Both forms are retained verbatim below — the `F-N` label in the finding body is NOT renamed, per D-448(a) source-attestation parity with the dispatched review text.

---

## Part A — Fix Verification (pass-11 carry-over items)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P11-HIGH-001 (pass-11 F-1) | HIGH | RESOLVED | T-050 length-conjunct isolation control added (`69663255`); the three false M2 mutation-kill-attestation sites (T-047 comment, T-048 comment + docstring, story lines 645) corrected. This pass independently confirms T-050 genuinely isolates the LENGTH conjunct — no residual finding on the length conjunct. This pass's own F-1 (below) finds a DIFFERENT, previously-unaudited conjunct (PREFIX) of the same predicate still unisolated — not a reopening of pass-11's finding. |
| ADV-BB-P11-MED-001 (pass-11 F-2) | MEDIUM | RESOLVED | 5 sibling docstring/test-name/story-row drift sites corrected (T-032 docstring + rename, T-023 docstring, `parse_plugin_refs` doc, story T-043/T-046 rows). No residual drift found this pass. |
| ADV-BB-P10-MED-001 | MEDIUM | UNRESOLVED — not addressed this pass | `hook-plugins/sub/` directory-only staging control remains open; not re-verified in this dispatch. Carries to pass-13. |
| ADV-BB-P10-LOW-001 | LOW | UNRESOLVED — not addressed this pass | NUL byte / trailing-space name admission remains open; not re-verified in this dispatch. Carries to pass-13. |
| ADV-BB-P10-LOW-002 | LOW | UNRESOLVED — not addressed this pass | Fail-open arms guarded only by unasserted call ordering remain open; not re-verified in this dispatch. Carries to pass-13. |
| ADV-BB-P10-LOW-003 | LOW | UNRESOLVED — not addressed this pass | `workspace_root()` untested directly remains open; not re-verified in this dispatch. Carries to pass-13. |

---

## Part B — New Findings

## HIGH

### F-1 — Containment-predicate PREFIX conjunct is not isolated by any test; the mutant survives (POLICY 13 mutation-completeness)

Location: `detect_ungated_declarations` (containment predicate), corroborated at crates/factory-dispatcher/tests/bundle_orphan_check.rs. The predicate is:
```
let in_repo = joined_parts.len() > root_parts.len()
    && root_parts.iter().enumerate().all(|(i, p)| joined_parts.get(i) == Some(p));
```
This has two conjuncts. The story spends an entire pass-11 fix-burst (F-1 closure) adding T-050 (`test_S_21_09_ac006_T050_length_conjunct_isolation_kills_m2`, plugin = "../..") precisely because it discovered that T-047 and T-048's OUTSIDE-REPO candidates were over-determined — they fail both conjuncts simultaneously, so neither conjunct is individually isolated. T-050 isolates the length conjunct.

The prefix conjunct (`root_parts.iter()...all(...)`) has the identical over-determination problem and NO isolation control:
- Every OUTSIDE-REPO candidate in the suite is either a short absolute path (`/outside-repo.wasm`, `/abs/hook-plugins/foo.wasm`, `/tmp/ghost.wasm` in T-048; `/abs/hook-plugins/ghost-absolute.wasm` in T-026(a)) whose lex_norm length is < root_parts.len() (fails the length conjunct — over-determined), or `../../../ghost.wasm` (T-047, T-048) whose resolved length == root_parts.len() (also fails the length conjunct — over-determined; the test's own comment admits "both conjuncts fail simultaneously").
- No candidate satisfies joined_parts.len() > root_parts.len() AND a divergent prefix — the only configuration under which the prefix conjunct is the sole determinant.
- T-026(b) isolates a different prefix check — extract_hook_plugin_name's gate-2 loop against parent_parts (root+plugins+vsdd-factory), reached via parse_plugin_refs, not detect_ungated_declarations's containment loop against root_parts. Verified: T-026(b) calls parse_plugin_refs directly, never run_t012_gate/detect_ungated_declarations.

Failure scenario (concrete input → surviving mutant). Declaration plugin = "../../../sib/ghost.wasm" from plugins/vsdd-factory/hooks-registry.toml:
- registry_parent = <root>/plugins/vsdd-factory (depth N+2). lex_norm(registry_parent.join("../../../sib/ghost.wasm")) pops vsdd-factory, plugins, and root's last component, then pushes sib, ghost.wasm → root_parts[0..N-1] + [sib, ghost.wasm], length N+1 > N (length conjunct TRUE), but index N-1 diverges (sib ≠ root's last component) → prefix conjunct FALSE → in_repo = false → OUTSIDE-REPO-DECLARATION (correct under live code).
- Apply mutant .all(...) → .any(...) (or .all(...) → true): for this candidate .any is TRUE (indices 0..N-1 match), so in_repo flips to TRUE → delegates to extract_hook_plugin_name, which fails its own gate-1 (N+1 < expected_depth+2 = N+4) → None → emits UNGATED-DECLARATION instead of OUTSIDE-REPO-DECLARATION. Classification flip; mutant survives the entire 45-test suite (T-012 real-registry gate is unaffected — real declarations are all valid in-repo).

This is exactly the class the story's own §2c "Total predicate invariant … No declaration is silently dropped" and T-048's totality-property partition claim to pin — but the sub-region "len > root ∧ divergent prefix" is unpinned, so the totality invariant is asserted in prose and only partially pinned by test.

POLICY 13 severity floor is HIGH (mutation-completeness of gate predicates). By the story's own established bar — F-1/T-050 treated the sibling length-conjunct gap as a blocking fix-burst — this omission is the same class.

Blast-radius note (transparency): the surviving mutant flips only the diagnostic identifier (UNGATED-DECLARATION ↔ OUTSIDE-REPO-DECLARATION); both branches return Err, so this is NOT a fail-open — no bad declaration slips the gate. This bounds the real-world harm, but does not change that a claimed-total gate predicate has an un-isolated conjunct.

Minimal fix route: add a dedicated isolation control (sibling of T-050) with fixture plugin = "../../../sib/ghost.wasm" (three-up-then-redescend-into-root-sibling), asserting the error contains OUTSIDE-REPO-DECLARATION: ../../../sib/ghost.wasm and NOT UNGATED-DECLARATION:. Empirically verify it goes RED under both .all→.any and .all→true mutants while T-047/T-048/T-050 stay GREEN, then record it in AC-006 §2c alongside the T-050 length-conjunct control.

## Items checked and found sound (no finding)
- T-050 / M2 length-conjunct kill claim — traced and verified correct (plugin = "../.." resolves to exactly root_parts; under >→>= classification flips OUTSIDE→UNGATED and T-050 fires RED while T-047/T-048 stay GREEN). Claim holds.
- T-048 totality biconditional — real (not tautological); 18 candidates through both extract_hook_plugin_name and detect_ungated_declarations with negative-identifier checks. Gap is only the un-covered sub-region above.
- Single-copy gate refactor (pass-11) — confirmed detect_ungated_declarations delegates its correctness gate to extract_hook_plugin_name; no duplicated three-gate logic remains. T-026(b)/T-033/T-035 isolate extract's gates 1/2/3 respectively.
- POLICY 11 (tautology) — fixture controls (T-015/016/019/020/021/022/030) call the real extracted run_t012_gate/check_declared_subset_tracked, not logic replicas. Not a harmful tautology.
- POLICY 7 (BC H1 parity) — BC-4.16.001 file H1, BC-INDEX row, and story body BC table match verbatim; version cite v1.8 current; lifecycle_status active (story notes doc-level status: draft is stale POL-14 metadata).
- POLICY 6 (subsystem) — story subsystems: [SS-04] matches BC subsystem: SS-04, capability: CAP-034.
- POLICY 5 / TD-VSDD-091 — story narrative uses function-name + test-ID + pass-N anchors and SHA 69663255; no file.rs:NNN load-bearing line pins.
- Test-count parity — T-012..T-050 = 39 distinct #[test] fns; T-006..T-050 = 45. Matches story claim.
- EC-005a vs EC-005b distinction, staged-vs-committed (git ls-files vs git ls-tree -r HEAD), and inventory *.toml fail-closed glob (T-024 underscore escape proof) — all coherent.

## Observations (disclosed, not blocking)
- The enabled = false false-positive class (LOW-3, documented at check_declared_subset_tracked doc) is disclosed and fail-loud, latent today (grep → 0). Acceptable as recorded.
- verification_properties: [] — VP allocation genuinely owed (BC-4.16.001's 4 VP rows read TBD). Properly surfaced in the story's BC Status / Routing Proposals as an owed authoring pass, not a silent omission. Not raised as a finding.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 0 |
| LOW | 0 |

**Overall Assessment:** block
**Convergence:** 1 HIGH (F-1, new — the un-audited PREFIX-conjunct sibling gap of pass-11's LENGTH-conjunct closure); both pass-11 findings (F-1 length-conjunct, F-2 drift sweep) verified CLOSED this pass; 4 pass-10 carry-over items (MED-001, LOW-001/002/003) remain open, not re-addressed this pass — see Part A.
**Readiness:** requires revision

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 12 |
| **Story version reviewed** | v1.22 |
| **Reviewed commit** | 69663255 |
| **New findings** | 1 (F-1 HIGH) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1 / (1 + 0) = 1.00 |
| **Median severity** | HIGH |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3→2→1→3→1→1 |
| **Total finding trajectory** | 3→3→2→13→11→9→9→8→8→15→2→1 (pass-1: 3; pass-2: 3; pass-3: 2; pass-4: 13; pass-5: 11; pass-6: 9; pass-7: 9; pass-8: 8; pass-9: 8; pass-10: 15; pass-11: 2; pass-12: 1) |
| **Verdict** | FINDINGS_REMAIN |

The F-1 finding is genuinely novel: it applies the exact over-determination reasoning the story itself pioneered for the length conjunct (T-050 fix-burst) to the second, un-audited conjunct of the same predicate. Fresh context surfaced it precisely because prior attention was anchored to the length conjunct that had just been fixed.

## Verdict

NOT CLEAN — 1 HIGH. Streak → 0/3.
