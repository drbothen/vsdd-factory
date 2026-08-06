# Adversarial Review — S-21.07 LOCAL cascade, Pass 7

```yaml
---
review_type: local-story-adversarial
story_id: S-21.07
cycle: v1.0-brownfield-backfill
pass: 7
passes: 7
reviewed_head: "fbb5183c (feature/S-21.07-validate-cross-site-correspondence HEAD)"
reviewed_branch: feature/S-21.07-validate-cross-site-correspondence
worktree: /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.07
story_version: "1.8"
bc: "BC-5.39.010 v1.13"
adrs_read: [ADR-037 v1.3, ADR-038 v1.3, ADR-039 v1.1]
verdict: CLEAN
findings_count: 0
severity_breakdown:
  BLOCKER: 0
  HIGH: 0
  MEDIUM: 0
  LOW: 0
observations_count: 2
streak: 1/3
trajectory_append: 0
trajectory: 47 → 18 → 25 → 25 → 24 → 20 → 0
gates_independently_executed:
  cargo_test: "162 passed / 0 failed / 17 ignored — VERIFIED (BC-INDEX v1.13 bump closes test_BC_corpus_version_sync_all_indexed_bcs_match_frontmatter)"
  cargo_fmt: "exit 0 — VERIFIED"
  cargo_clippy: "exit 0, --all-targets -D warnings — VERIFIED"
  wasm_artifact: "231,661 bytes; sha256 853c802e74ec372864912448130f3b0740aeeae6f92b8230c7eb25f639dc32b8 — VERIFIED cmp byte-identical"
  bats: "35/35 — VERIFIED"
prior_pass_records_read:
  - adversary-pass-6.md — Part A only (Iron Law honoured)
---
```

## Part A — Findings

*This pass is CLEAN. No findings raised. All 20 pass-6 findings (F-S2107-P7-001 through F-S2107-P7-020) independently verified closed against HEAD `fbb5183c`.*

### Pass-6 findings closure summary

| Finding | Severity | Closure evidence |
|---------|----------|-----------------|
| F-S2107-P7-001 | BLOCKER | Code burst `fbb5183c` committed; `.factory/` D-957 committed; working-tree-only failure mode eliminated |
| F-S2107-P7-002 | BLOCKER | Arm B2 self-lock resolved (ADR-038 v1.3 PC13b Option 1 carve-out); corpus test `corpus_arm_b2` at T-048 RED gate wired; PC13b STORY-INDEX.md listed in `allowed_stale_inputs` array |
| F-S2107-P7-003 | BLOCKER | `.factory/` mount step moved ABOVE `cargo test` in `cargo-host` CI job; `CI_REQUIRE_ARTIFACTS: "1"` set on cargo test step; corpus tests no longer vacuously skip in CI |
| F-S2107-P7-004 | BLOCKER | BC-5.39.010 v1.13 PC5/PC6 rewritten to `rightmost-in-chain` algorithm; `extract_story_bc_version_citations` Phase 2 `rightmost_in_chain()` implemented; reverse-field removed |
| F-S2107-P7-005 | HIGH | `lib.rs` module-level doc comment updated to BC-5.39.010 v1.13; ~136 historical cite sites in `crates/` are frozen-historical and exempt per D-TD-VSDD-091 |
| F-S2107-P7-006 | HIGH | All 4 `invariant-10` mis-anchors corrected (2 in lib.rs + 2 in test files); `invariant_10` renamed to `invariant_arm_a1_b1_none_conflation` |
| F-S2107-P7-007 | HIGH | BC-INDEX BC-5.39.010 row v1.12 appended (D-957 state-manager leg; closed prior burst) |
| F-S2107-P7-008 | HIGH | ADR-038 PC13b adjudication ruling; `allowed_stale_inputs` carve-out specified; arm B2 path defined |
| F-S2107-P7-009 | HIGH | `red-gate-log.md` pass-6 fix burst attestation section added with assertion-site SHA |
| F-S2107-P7-010 | HIGH | CWE re-classification: primary CWE-636 (Not Failing Securely), secondary CWE-390 (Detection of Error Condition Without Action); CWE-778 retired from F-P7-010 record; BC-5.39.010 v1.13 §Security section updated |
| F-S2107-P7-011 | HIGH | `test_BC_corpus_version_sync_all_indexed_bcs_match_frontmatter` given real teeth: 574KB BC-5.39.010 fixture + fuel-headroom warning; guard fails on mismatched version |
| F-S2107-P7-012 | HIGH | `extract_bc_index_version_state` None-conflation fixed: `RowAbsent` vs `RowPresentNoVersion` now distinct; ≥1700 BC rows correctly parsed |
| F-S2107-P7-013 | MEDIUM | PC4a verbatim mandate propagated to sibling advisory clauses PC4b/PC4c (same burst, same spec file); class-level sweep confirmed |
| F-S2107-P7-014 | MEDIUM | `arm_a2` normalization asymmetry fixed; both extraction sites use canonical form |
| F-S2107-P7-015 | MEDIUM | `test_BC_corpus_version_sync` teeth: `VSDD_CORPUS_ROOT` fails-open validated; panic branch covered by dedicated test T-049 |
| F-S2107-P7-016 | MEDIUM | Coverage gate counts execution sites (T-047 skip-count assertion), not declaration counts; exact-count assertion binds the skip-site population |
| F-S2107-P7-017 | MEDIUM | PC5/PC6 implementation aligned to rightmost-in-chain algorithm (F-P7-004 umbrella); no separate residual |
| F-S2107-P7-018 | MEDIUM | `arm_e1` normalization asymmetry fixed; arm E1 extraction uses same form as arm A1 |
| F-S2107-P7-019 | LOW | D-693 attestation stale WASM size (226,794 bytes, pass-5) retracted in STATE.md Drift Items; live artifact at `fbb5183c` is 231,661 bytes |
| F-S2107-P7-020 | MEDIUM | 18 BCs with non-canonical v-prefixed version recorded as drift item; `extract_story_bc_version_citations` Phase 2 rightmost-scan correctly handles both forms |

## Part B — Observations

### O-P8-01 — F-P7-010 CWE retroactive correction [documentation]

The initial filing of F-S2107-P7-010 cited CWE-778 (Insufficient Logging) as the governing weakness. The correct classification is:
- **Primary:** CWE-636 — Not Failing Securely ("Failing Open") — the validator exhaustion path silently returns `Ok` rather than blocking
- **Secondary:** CWE-390 — Detection of Error Condition Without Action — the fuel exhaustion signal is observable internally but not propagated as a blocking outcome

CWE-778 describes missing audit logs; it does not describe a gate that fails-open under resource exhaustion. The BC-5.39.010 v1.13 §Security section should reflect this correction. **Not a new finding — historical correction to F-P7-010 classification.**

### O-P8-02 — BC-INDEX self-contradictory count [documentation]

BC-INDEX frontmatter `total_bcs: 1983` vs subsystem summary table `Total: **1975**` — 8-BC gap. The gap predates this pass and is carried as a drift item. This burst adds 2 new SS-01 BCs (BC-1.01.016 + BC-1.03.017), making the gap 8 after state-manager reconciliation. **Not actionable in this pass — state-manager records drift item in STATE.md.**

## Part C — Analysis and Verdict

**VERDICT: CLEAN (0 findings, 0 BLOCKERS)**

All 20 pass-6 findings (F-S2107-P7-001 through F-S2107-P7-020, comprising 4 BLOCKER + 7 HIGH + 8 MEDIUM + 1 LOW) verified independently closed against HEAD `fbb5183c`.

**Convergence streak:** 0/3 → **1/3** (first CLEAN pass in S-21.07 LOCAL cascade).

**Trajectory:** 47 → 18 → 25 → 25 → 24 → 20 → **0** (6 passes NOT-CLEAN; pass-7 CLEAN; total 7 adversary reviews).

**Dominant pattern resolved:** The `fix-named-site-not-class` anti-pattern (5 instances in pass-6) has been corrected — the rightmost-scan Phase 2 implementation covers the class, not just the named site. The coverage gate now counts execution sites (not declarations). The CI wiring for the corpus test is confirmed: `.factory/` mount precedes `cargo test`.

**ADR compliance verified:** ADR-037 volatile-inputs boundary respected; ADR-038 PC13b carve-out implemented with `allowed_stale_inputs` array; ADR-039 Phase 1 schema leg (BC-1.01.016) and Phase 4 enforcement leg (BC-1.03.017) authored and registered.

**BC-5.39.010 v1.13:** Six-arm structure intact; rightmost-in-chain algorithm in PC5/PC6; Phase 2 reverse-field removed (declared NON-CONFORMING in v1.13; implementation aligned); CWE-636/CWE-390 security classification correct.

**Next step:** Pass-8 dispatch (if streak advances to 2/3) OR human stop signal.
