---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T05:00:00Z
phase: 19
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 19
previous_review: adv-s21.09-local-pass-18.md
---

# Adversarial Review — S-21.09 (LOCAL cascade pass)

Artifacts: story S-21.09-wasm-artifact-restore-and-registry-parity.md (v1.28); worktree .worktrees/S-21.09 @ 12d0fe98 — crates/factory-dispatcher/tests/bundle_orphan_check.rs, crates/factory-dispatcher/src/registry.rs; BC .factory/specs/behavioral-contracts/ss-04/BC-4.16.001.md.

## BLOCKER
None.
## HIGH
None.
## MEDIUM
None.
## LOW
None.
## NIT
None.

## Verdict: CLEAN
Nothing at any severity rises to a genuine defect. This is a clean pass.

## What I attacked and why each candidate was dismissed (not suppressed)
Count parity (51 / T-006..T-056 / 45-owned + 1 registry) — VERIFIED EXACT. Grepped every test function: T-006..T-011 = 6 (S-19.04/S-19.06), T-012..T-056 = 45 distinct functions (one per ID, no gaps/dupes), total 51; plus registry::tests::on_error_falls_back_to_registry_defaults_when_entry_omits_it in registry.rs (out-of-gate). Every recurrence of the count in the spec agrees.

SHA currency — CLEAN. All current-state cites (12d0fe98) equal HEAD. 12f280d1 is an explicitly-framed historical byte-provenance annotation ("host-toolchain-specific… not necessarily byte-identical") — legitimate empirical record, not a load-bearing volatile pin.

Determinant isolation (POLICY 13) — every conjunct/gate has a dedicated firing control. The two-conjunct in_repo predicate is isolated by T-050 (length conjunct, plugin="../.." → exact root_parts self-match, kills >→>=) and T-051 (prefix conjunct, ../../../sib/ghost.wasm, kills .all→.any / .all→true); orthogonality is real. The three reachable gates in extract_hook_plugin_name are isolated by T-033 (gate-1 min-length), T-026(b) (gate-2 prefix loop), T-035 (gate-3 hook-plugins component). The two production-validation determinants in run_t012_gate are isolated by T-052 (hooks Registry::parse_str, schema_version=3) and T-053 (resolvers schema_version==1 assert), plus T-054 isolating the .unwrap_or(-1) fail-closed sentinel from the assert (absent-key vs wrong-integer fixtures — genuinely distinct). All #[should_panic(expected=...)] strings verified against the actual panic messages.

Accepted-residual (SURV-01) — honestly characterized. lex_norm's RootDir | Prefix(_) => parts.clear() arm is provably a no-op: std::path::Components always yields a root/prefix component first, before any Normal pushes to parts, so parts is empty whenever the arm fires — a std::path invariant. The doc-comment-only disposition (no vacuous test) is correct. SURV-02/03 (CurDir arm; detect_ungated_declarations Err(_)=>Vec::new() fail-open) are genuinely dead in the gate path and pinned by direct-call unit tests T-056/T-055.

No tautologies (POLICY 11). Every S-21.09 test drives a real gate/production function. T-056 asserts against the live lex_norm, not self-constructed data.

Traceability / BC parity — CLEAN. BC-4.16.001 H1 matches the story's Behavioral Contracts table title verbatim; version v1.8 matches frontmatter; lifecycle_status: active confirms the story's "treat draft as active" note. AC→clause anchors resolve: AC-003/004/005 → PC1/PC2/PC3; AC-001/002/006 → Precondition 3; AC-007 → Invariant 1. subsystems: [SS-04] matches the BC subsystem. Fixture-inventory ground-truth holds: plugins/vsdd-factory/*.toml = exactly {hooks-registry.toml, resolvers-registry.toml}, so the real T-012 inventory gate passes; validate-factory-path-staging.wasm present on the gitignored path, consistent with the git add -f tracking premise.

Disclosed residuals are honest, fail-loud, and conservative — not hidden defects. The enabled=false false-positive class (latent today, fail-loud MISSING:) and the case-variant-declaration-always-MISSING-by-design behavior are over-strict/conservative directions, correctly documented per POLICY 13 — no security or false-negative gap.

No process-gap tags warranted.
