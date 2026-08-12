---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T04:00:00Z
phase: 18
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "0979676"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 18
previous_review: adv-s21.09-local-pass-17.md
---

# Adversarial Review — S-21.09 (LOCAL cascade pass)

Artifacts reviewed: story spec S-21.09-...md v1.28 (full, 1120 lines); implementation crates/factory-dispatcher/tests/bundle_orphan_check.rs (all gate functions + all 51 tests) and crates/factory-dispatcher/src/registry.rs (production Registry::parse_str/validate + unit tests); governing BC BC-4.16.001.md v1.8; policies.yaml (1–13+); live-gate ground truth at worktree 12d0fe98.

## What I attacked and found sound

Count-parity (all sites cross-checked): bundle_orphan_check.rs contains exactly 51 #[test] functions, contiguous T-006..T-056 with zero ID gaps; S-21.09-owned = T-012..T-056 = 45; plus the one registry.rs unit test (on_error_falls_back_to_registry_defaults_when_entry_omits_it). Matches story §Test tables, AC-006, and dispatch prompt exactly.

Live-gate ground truth (the T-012 run against real workspace_root() depends on all of these): plugins/vsdd-factory/ top level holds exactly {hooks-registry.toml, resolvers-registry.toml} (inventory gate passes); hooks-registry.toml schema_version = 2 with 75 [[hooks]] and declares hook-plugins/validate-factory-path-staging.wasm; resolvers-registry.toml schema_version = 1; validate-factory-path-staging.wasm present on disk. All consistent with the story's empirical annotations.

Mutation/isolation completeness (POLICY 11 / sole-determinant hunt): every gate, conjunct, assert, sentinel, and fail-open arm has a dedicated isolating control — gate-1 T-033, gate-2 T-026(b), gate-3 T-035; in_repo length conjunct T-050 and prefix conjunct T-051 (orthogonality empirically cross-verified); hooks Registry::parse_str block T-052; resolvers assert_eq! block T-053; resolvers .unwrap_or(-1) fail-closed sentinel T-054 (absent-key path, distinct from T-053's wrong-integer); detect_ungated_declarations fail-open Err(_) arm T-055 (direct); lex_norm CurDir arm T-056 (direct). All #[should_panic] attributes present with expected= substrings that match the live panic messages. The T-054/T-052 bodies that discard the Result are correct precisely because the panic is raised inside run_t012_gate and gated by the matching #[should_panic].

Fail-open arms are not guarded only by unasserted ordering: the detect_ungated_declarations read-failure/parse-failure arms are unreachable in the gate path because parse_plugin_refs + Registry::parse_str read/validate the same files first (panicking); T-055 pins the parse arm directly rather than relying on the ordering. SURV-01 (lex_norm RootDir | Prefix parts.clear()) is an honestly-characterized, provably-un-isolatable accepted residual (parts is always empty when a root/prefix component fires first), not a suppressed gap.

Traceability / anchoring (POLICY 4/6/7/8): body BC table carries BC-4.16.001 v1.8 with the verbatim H1 (no enrichment) and clause list PC1..PC4, Precondition 3, Invariant 1; Token Budget cites v1.8; behavioral_contracts: [BC-4.16.001] (len 1); every AC-001..007 traces to a real BC-4.16.001 clause, and each is bidirectionally present in the body table. subsystems: [SS-04] matches BC subsystem: SS-04; target_module matches BC §Architecture Module. BC PC1/PC2/PC3 ↔ AC-003/004/005 map exactly. AC-006's trace to Precondition 3 is explicitly flagged as an indirect anchor with a routing proposal — a defensible, honestly-labelled choice, not a mis-anchor.

VP gap (POLICY 9): verification_properties: [] is correct — BC-4.16.001 §Verification Properties genuinely lists all 4 rows as "(TBD — to be assigned by state-manager after VP authoring pass)"; the story documents this as owed with routing. No orphaned/invented VP IDs.

Production surface: registry.rs parse_str/validate return Result (no unwrap/expect in the critical path), with fail-closed schema/regex/async-block/uniqueness checks fully covered.

SHA cites: 62fbcf1a (develop head) matches the repo log; 7bb0e797, 12f280d1 are historical and honestly annotated (byte-provenance flagged host-toolchain-specific). No current-state cite is stale vs 12d0fe98.

## Critical / High / Medium / Low / Nit Findings

None. Nothing at any severity rises to a genuine defect. The isolation-control coverage, count-parity, traceability, and live-gate ground truth are internally consistent and independently corroborated; the accepted residuals and the VP/AC-002-vacuity notes are historically accurate honest characterizations.

## Verdict: CLEAN

This is a legitimately well-hardened deliverable. This pass advances the 3-CLEAN LOCAL streak.
