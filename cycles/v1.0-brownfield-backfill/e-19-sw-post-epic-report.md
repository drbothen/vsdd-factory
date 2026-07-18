---
document_type: post-epic-report
report_id: E-19-SW-POST-EPIC
epic_id: "E-19"
epic_title: "Post-rc.22 Operator Hardening — host ABI hardening"
version: "1.0"
status: final
author: session-reviewer
timestamp: "2026-07-17T00:00:00Z"
covers_stories:
  - S-19.01
  - S-19.02
  - S-19.03
  - S-19.04
  - S-19.05
  - S-19.06
  - S-19.07
  - S-19.08
  - S-19.09
develop_head_at_epic_close: "6db4c9fc"
epic_close_date: "2026-07-17"
total_points: 55
waves: 3
---

# E-19 Post-Epic Story Closure Report

**Epic:** E-19 — Post-rc.22 Operator Hardening (host ABI hardening)
**Epic status:** COMPLETE — 9/9 stories merged; develop HEAD 6db4c9fc (2026-07-17)
**Total story points:** 55 across 9 stories, 3 waves
**Date of closure:** 2026-07-17 (PR #670 squash-merge at 22:25:06Z; D-851)

This report covers all six mandatory closure sections: delivery ledger, AC coverage, scope evolution, carried/deferred items, E-20 candidates, and decomposition lessons. It is report-only; no story or index files were modified.

---

## Section 1 — Per-Story Delivery Ledger

| Story | Pts | PR | Merge SHA | Date | BC(s) | POL-14 Outcome | Cascade (type / last-fix pass) |
|-------|-----|----|-----------|------|-------|----------------|-------------------------------|
| S-19.01 | 8 | #613 | 8d1721f7 | 2026-07-13 | BC-5.42.001 v1.7 | PROMOTED draft→active | E-19 cycle-level; last behavioral fix pass-22 (cite sweeps to pass-43) |
| S-19.02 | 8 | #610 | f5ea12e9 | 2026-07-13 | BC-4.13.001 v1.15 Phase-A | PASS-ALREADY-ACTIVE (v1.15 amendment to active BC) | E-19 cycle-level; last behavioral fix pass-29 (cite sweeps to pass-35; D-827 human-amendment at v1.18) |
| S-19.03 | 5 | #611 | 091ce499 | 2026-07-13 | BC-2.07.001 v1.5, BC-2.02.011 v1.7 | PASS-ALREADY-ACTIVE (both BCs previously active) | E-19 cycle-level; last behavioral fix pass-15 (cite sweeps to pass-45) |
| S-19.04 | 5 | #639 | d4a23a02 | 2026-07-14 | — | N/A (no BCs in behavioral_contracts) | LOCAL cascade; last fix pass-13 |
| S-19.05 | 8 | #640 | 7b35c8e4 | 2026-07-14 | BC-3.08.001 v1.23 | PASS-ALREADY-ACTIVE (BC-3.08.001 active pre-E-19) | LOCAL cascade; last fix pass-15 |
| S-19.06 | 8 | #657 | 9787c056 | 2026-07-15 | BC-1.17.001 v1.6 | PROMOTED draft→active (new BC for read_prefix) | LOCAL 3-CLEAN at pass-11 (passes 9+10+11) |
| S-19.07 | 3 | #670 | 6db4c9fc | 2026-07-17 | BC-4.13.001 v1.17 Phase-B | PASS-ALREADY-ACTIVE (D-545; v1.17; input-hash daa1363) | LOCAL 3-CLEAN at pass-18 (passes 16+17+18) |
| S-19.08 | 5 | #646 | 1304d280 | 2026-07-14 | BC-5.40.001 v1.3 | PROMOTED draft→active (new BC added mid-epic D-835) | LOCAL 3-CLEAN at pass-10 (passes 8+9+10) |
| S-19.09 | 5 | #659 | 13ece92c | 2026-07-15 | BC-1.17.001 v1.7, BC-3.08.001 v1.24 | PASS-ALREADY-ACTIVE (both BCs active post-S-19.06/S-19.05 merges) | LOCAL 3-CLEAN at pass-13 (passes 11+12+13) |

**Totals:** 55 points, 9 PRs merged, 3 new BCs promoted (BC-5.42.001, BC-1.17.001, BC-5.40.001), 2 BCs amended in-place (BC-4.13.001 Phase-A → Phase-B, BC-3.08.001 v1.23→v1.24), 2 BCs with PASS-ALREADY-ACTIVE at story BC level (BC-2.07.001, BC-2.02.011).

**Cascade type note:** S-19.01–S-19.03 were driven by the E-19 cycle-level adversary (same adversary session reviewing all three stories in each pass). S-19.04–S-19.09 each had independent LOCAL 3-CLEAN cascades (separate adversary invocations per story). The shift to LOCAL cascades mid-epic corresponds to stories with CRITICAL-classified deliverables (D19 in S-19.09 was identified after S-19.04–S-19.06 were already in LOCAL cascades).

---

## Section 2 — AC Coverage Closure

This section reports per-story AC delivery status and any accepted-risk deviations.

### S-19.01 — pr-manager hardening (BC-5.42.001 v1.7)

All four ACs delivered and gated:

- **AC-001** (READY SHA pinning via `pr-manager-completion-guard.wasm` hook; `check-stale-verdict.sh <pr_number> <covered_sha>`; READY_SHA_MISSING advisory on missing pin; gh-failure arm exits non-zero with READY_SHA_FETCH_FAILED on stderr) — DELIVERED. Gate includes both positive and negative control fixtures per BC-5.42.001 EC-001.
- **AC-002** (stale-verdict detection: `covered_sha` on HEAD ≠ merge SHA triggers READY_SHA_MISMATCH) — DELIVERED.
- **AC-003** (`enforce-merge-strategy.sh` as sole gateway for all `gh pr merge` invocations; positional strategy `$2`; residual `"${@:3}"` governed pass-through; deny-list STRATEGY_SMUGGLING_FORBIDDEN; release-branch guard per ADR-030 §Decision 3) — DELIVERED with F-P7-001 at v1.18 adding EC-009 best-effort-delete verification and Invariants 6–7.
- **AC-004** (darwin-leg: bats-darwin-leg-macos CI job on macos-latest; `/bin/bash 3.2` preflight discipline extracted from release.yml loop and executed under `/bin/bash 3.2` with fixture registry) — DELIVERED. CI job verified in ci.yml.

No accepted-risk deviations. F-P22-001 at pass-22 corrected the WASM path hooks/ → hook-plugins/; all sites swept.

### S-19.02 — verify-factory-lock cap raise / frontmatter parse (BC-4.13.001 v1.15 Phase-A)

All six ACs delivered and gated:

- **AC-001** (`STATE_MD_MAX_BYTES = 262144` in plugin source) — DELIVERED.
- **AC-002** (reads >64 KiB successfully; no StateReadError::OutputTooLarge in [65536, 262144] range) — DELIVERED.
- **AC-003** (`extract_frontmatter(bytes: &[u8]) -> &[u8]` pub function in `crates/factory-lock-parse/src/lib.rs`; frontmatter-only slice returned) — DELIVERED. F-P29-001 at pass-29 relocated from inline to dedicated `crates/factory-lock-parse/` pure crate per architect ruling.
- **AC-004** (zero `output_too_large` events when reading a 70 KiB file) — DELIVERED.
- **AC-005** (byte-exact delimiter boundary: inclusive opening `---` bytes, exclusive closing `---` bytes; parse_factory_lock handles document-start marker) — DELIVERED. Boundary semantics authoritatively documented in BC-4.13.001 v1.15 Invariant 9.
- **AC-006** (soft-warn `state_md_approaching_cap` when bytes in range (200000, 262144]; reads succeed at 262144-exact; fail at 262145+) — DELIVERED. EC-017 (CRLF support) added at v1.18 per D-827 human-approved spec evolution.

One accepted-risk item: the proptest harness (VP-096, `crates/factory-lock-parse/tests/proptest_extract_frontmatter.rs`) was added at pass-28 (F-P28-003) after the initial AC set. No behavioral deviation; the addition strengthened the verification coverage beyond original scope.

### S-19.03 — read_file NOT_FOUND(-5) / file_not_found telemetry (BC-2.07.001 v1.5, BC-2.02.011 v1.7)

All six ACs delivered and gated:

- **AC-001** (path resolution using ancestor-fallback rejoin: `canonicalize` deepest existing ancestor, rejoin non-existent tail, then `starts_with` canonical prefix; reason token `path_resolution_failed` on canonicalize failure; injectable mock canonicalize fn per BC-2.07.001 EC-007) — DELIVERED. F-P12-004 at pass-12 decomposed the two-step prepare() pattern architecturally.
- **AC-002** (zero `capability_denied` events from `warn-pending-wave-gate` when wave-state.yaml is absent) — DELIVERED. Gate scoped to `plugin_name=warn-pending-wave-gate` per O-P9-005.
- **AC-003** (`codes::NOT_FOUND = -5` in `pub mod codes` in `host/mod.rs`; `HostError::NotFound` named variant in hook-sdk `host.rs` without `#[non_exhaustive]`) — DELIVERED. Architecture Compliance: `#[non_exhaustive]` explicitly forbidden (prevents exhaustive pattern matching in plugin code; breaking change if added post-release).
- **AC-004** (`internal.file_not_found` internal telemetry event emitted on NOT_FOUND path) — DELIVERED.
- **AC-005** (`warn-pending-wave-gate` handles absent wave-state.yaml gracefully, returns Continue) — DELIVERED.
- **AC-006** (zero `path_not_allowed reason=path_not_allowed` events when plugin is invoked without an existing wave-state.yaml file; `path_resolution_failed` excluded per BC-2.07.001 EC-007) — DELIVERED. Gate hardened at pass-15 (F-P15-007) from pipefail+grep-c to jq-validity-check + wc-l count.

No accepted-risk deviations. Architecture note: `path_util.rs` (new shared module for path resolution) was added at F-P12-004; hoisting to shared module was explicitly instructed by story AC-001.

### S-19.04 — registry/bundle hygiene, tool-filter anchoring (VP-099)

All seven ACs delivered:

- **AC-001** (hello-hook.wasm BUILD and COPY steps removed from release.yml; `vsdd_context_resolvers.wasm` and `wasm_resolver_export.wasm` moved to skip path at both staging sites; keep-assertions for vsdd-context-resolvers.wasm in resolvers-registry.toml) — DELIVERED. F-P5-002 at pass-5 corrected the initial framing (resolvers pair already in case-arm; hello-hook was the active build).
- **AC-002** (`vsdd-context-resolvers.wasm` keep-assertion: test -f hook-plugins/vsdd-context-resolvers.wasm AND grep -q hook-plugins/vsdd-context-resolvers.wasm resolvers-registry.toml) — DELIVERED.
- **AC-003** (bundle test confirms 3 true orphans removed; WASM count regression) — DELIVERED.
- **AC-004** (tool-filter regex fully anchored: all entries match `^...$` form; no unanchored entries; positive and negative control fixtures including both-ends test) — DELIVERED. F-P6-001 at pass-6 corrected to require both leading `^` and trailing `$`.
- **AC-005** (verify-factory-lock fires on MultiEdit fixture post-anchoring; `^(Edit|Write|MultiEdit|Agent)$` still triggers) — DELIVERED.
- **AC-006** (`crates/factory-dispatcher/tests/bundle_orphan_check.rs` Rust workspace test; dual-registry orphan check; no-new-.sh policy compliance) — DELIVERED. No-new-.sh standing policy established 2026-07-13 (human directive); AC-006 converted from bats to Rust at v1.12 per this policy.
- **AC-007** (cargo test `stage_release_bundle` validates no hello-hook in artifact staging; some_new_stub_lib glob-proof fixture) — DELIVERED.

**POLICY 20** (`release_bundle_no_dev_samples`) introduced by this story. **No-new-.sh standing policy** established (human directive 2026-07-13; full scope including CI/test helpers; see Section 5 for E-20 migration candidates).

### S-19.05 — async completion telemetry + VSDD_SINK_FILE release-mode (BC-3.08.001 v1.23, DI-019)

All nine ACs delivered:

- **AC-001** (async plugins emit `plugin.completed` event with all 9 Event 6 mandatory fields: type, trace_id, session_id, plugin_name, plugin_version, entry_index, exit_code, elapsed_ms, fuel_consumed) — DELIVERED.
- **AC-002** (async plugins emit `plugin.abandoned` event with `entry_index: u32` field when drain-window expires; 7 mandatory fields: type, trace_id, session_id, plugin_name, entry_index, drain_window_ms, timestamp) — DELIVERED.
- **AC-003** (existing `plugin.invoked` events for async path unaffected) — DELIVERED.
- **AC-004** (`VSDD_ASYNC_DRAIN_WINDOW_MS` gated `cfg(any(debug_assertions, feature = "test-support"))`; `ENV_SINK_FILE` and `flush_sink_file` NOT cfg-gated in release; awk-scoped absence of cfg gate; test-support feature ONLY in ci.yml, never in release.yml) — DELIVERED. F-P8-001 at pass-8 expanded gate from `debug_assertions`-only to `any()` form per architect Option-A.
- **AC-005** (`VSDD_SINK_FILE` honored in release builds; absolute path constraint per SEC-003; traversal sequences rejected silently) — DELIVERED. `flush_sink_file` relocated to `crates/factory-dispatcher/src/vsdd_sink.rs` per F-P3-001.
- **AC-006** (File Structure reconciled against diff 091ce499..HEAD; lib.rs + emit_event.rs rows confirmed; 8-file diff) — DELIVERED at pass-9.
- **AC-007** (`test-support` feature in Cargo.toml; ci.yml `--features factory-dispatcher/test-support` annotation; feature absent from release.yml) — DELIVERED.
- **AC-008** (zero bare literals `"plugin.completed"`, `"plugin.timeout"` in production code of emit_event.rs; PLUGIN_COMPLETED + PLUGIN_TIMEOUT constants from internal_log.rs used at all sites) — DELIVERED via F-P1-002 in S-19.09 (scope-extension per TD-VSDD-060 sibling-sweep — the S-19.09 D21 sweep covered these four literals when S-19.05's D21 partial implementation was found internally inconsistent post-S-19.09 analysis).
- **AC-009** (domain_invariants: [DI-019] wired; DI-019 is the direct load-bearing anchor for VSDD_ASYNC_DRAIN_WINDOW_MS Architecture Compliance Rules) — DELIVERED at v1.20.

**Accepted-risk deviation:** AC-008's extension to `"plugin.completed"` and `"plugin.timeout"` literals in `emit_event.rs` was not in S-19.05's original scope; it was discovered and fixed as a TD-VSDD-060 sibling-sweep obligation in S-19.09 D21. The extension was correct-by-construction — naming constants was always the right behavior; S-19.09 completed what S-19.05 started. No behavioral regression.

### S-19.06 — host::read_prefix bounded partial read (BC-1.17.001 v1.6, VP-101)

All seven ACs delivered:

- **AC-001** (read_prefix FFI entry point in hook-sdk; `pub safe fn read_prefix(path_ptr, path_len, max_bytes, out_ptr, out_len_ptr, timeout_ms)` 6-param extern in `hook-sdk/src/ffi.rs`; safe wrapper `pub fn read_prefix(...)` in `hook-sdk/src/host.rs`) — DELIVERED.
- **AC-002** (head-c semantics: max_bytes=0 → empty payload, file NOT opened; max_bytes>0 → at most max_bytes bytes from file start) — DELIVERED.
- **AC-003** (NEVER returns OUTPUT_TOO_LARGE (-3); guarantee is architectural) — DELIVERED. BC-1.17.001 v1.6 Invariant N encodes this guarantee.
- **AC-004** (capability schema in DISTINCT "Capability Schemas" preamble comment block in hooks-registry.toml; schema format parity with read_file) — DELIVERED.
- **AC-005** (`read_prefix` registered in `setup_linker` (`Linker<HostContext>` test path) in `host/mod.rs`) — DELIVERED. **Note:** Registration in `setup_host_on_store_data` (production path) was absent — this was the CRITICAL D19 gap discovered post-S-19.06 delivery and fixed by S-19.09.
- **AC-006** (VP-101 verification: Kani-style property that `output_too_large` never emitted for read_prefix) — DELIVERED.
- **AC-007** (read_prefix.rs new source file; fuel budget semantics; error code table parity with read_file) — DELIVERED.

**Accepted-risk deviation (LOW, accepted-with-record):** The WASM binary (`read-prefix-example.wasm` or equivalent) was not rebuilt at pass-1 merge point due to cross-compilation environment gap. Accepted LOW risk on record per pass-11 3-CLEAN closure. The binary is rebuilt by release.yml on every release.

**Critical gap resolved in S-19.09:** AC-005 covered only the test-path linker. The production dispatch path (`setup_host_on_store_data` in `invoke.rs`) was found absent during S-19.09 decomposition (D19 CRITICAL). This is addressed in Section 3 (scope evolution) and Section 4 (carried items).

### S-19.07 — verify-factory-lock read_prefix migration Phase-B (BC-4.13.001 v1.17, VP-095 v1.5)

All three ACs delivered:

- **AC-001** (NO non-comment `host::read_file`, `STATE_MD_MAX_BYTES`, or `TooLarge` use in `verify-factory-lock/src/lib.rs`; Block-comment-strip + line-comment-filter gate; portable sed `-e` form per F-P4-002) — DELIVERED. F-P1-001 BLOCKER resolved: 8192-byte bound was premise-false (closing `---` at byte 35,175 of 178,742-byte STATE.md; `last_amended` alone 32,648 bytes); corrected bound 262144 per ADR-025 §Decision 15 v1.18.
- **AC-002** (registry entry migrated to `capabilities.read_prefix`; `STATE_MD_MAX_BYTES` constant removed; soft-warning threshold removed per Phase-B) — DELIVERED. Both `verify-factory-lock` and `verify-factory-lock-bash` entries swept per F-P8-007.
- **AC-003** (real-shape ~35 KB frontmatter fixture: plugin blocks on foreign unexpired lock; continues when no lock; regression fixture for small-frontmatter also retained) — DELIVERED at v1.21 per F-P4-003 correcting the offset (~27 KB absolute form → ~35 KB from start, immediately before closing `---`).

No accepted-risk deviations. VP-095 v1.5 (Phase-B form) covers this story; Phase-A VP-095 test (`integration_ac004_no_output_too_large.rs`) deleted at 0cbf913e because STATE_MD_MAX_BYTES cannot compile post-Phase-B (F-P6-002).

### S-19.08 — verify-state-timestamp-refresh cap raise / extract_frontmatter (BC-5.40.001 v1.3, VP-096)

All three ACs delivered:

- **AC-001** (`STATE_MD_MAX_BYTES = 262144` in verify-state-timestamp-refresh plugin; reads >64 KiB successfully) — DELIVERED.
- **AC-002** (`extract_frontmatter` wired from `crates/factory-lock-parse/`; proptest VP-096 reused; byte-exact boundary enforced) — DELIVERED. Lesson: remediation sweeps must cover the entire defect class for the crate — S-19.08 was added because S-19.02's remediation of the verify-factory-lock plugin did not initially sweep the verify-state-timestamp-refresh plugin (same defect class).
- **AC-003** (zero `output_too_large` events on 70 KiB read; soft-warn at >200000 bytes per BC-5.40.001 Invariant 9) — DELIVERED.

No accepted-risk deviations.

### S-19.09 — post-E-19 host ABI fixes D19–D22 (BC-1.17.001 v1.7, BC-3.08.001 v1.24)

All nine ACs delivered:

- **AC-001** (`read_prefix` func_wrap block present in `setup_host_on_store_data` in `invoke.rs`; WAT module with `vsdd::read_prefix` import instantiates without link error on production path) — DELIVERED (D19).
- **AC-002** (round-trip: `setup_host_on_store_data` reads allowlisted file via `read_prefix`; bytes correct; `out_ptr > 0`) — DELIVERED (D19).
- **AC-003** (capability absent → `CAPABILITY_DENIED (-1)` via production path) — DELIVERED (D19).
- **AC-004** ("epoch interruption" text absent from `read_file.rs` and `read_prefix.rs`; "structurally unenforced" text present) — DELIVERED (D20). Bats hygiene gates T-004 through T-007.
- **AC-005** (`setup_host_on_store_data` two-linker duality comment present in `read_file.rs`) — DELIVERED (D20). Bats gate T-008.
- **AC-006** (`INTERNAL_FILE_NOT_FOUND` constant value asserted byte-identical to literal "internal.file_not_found") — DELIVERED (D21).
- **AC-007** (`PLUGIN_ABANDONED` constant value asserted byte-identical to literal "plugin.abandoned") — DELIVERED (D21).
- **AC-008** (zero bare literals `"internal.file_not_found"`, `"plugin.abandoned"`, `"plugin.completed"`, `"plugin.timeout"` in production code of `read_file.rs`, `read_prefix.rs`, `emit_event.rs`; awk-scoped before `#[cfg(test)]` boundary) — DELIVERED (D21 + F-P1-002 four-literal extension).
- **AC-009** (`emit_plugin_completed_async` emits event with `fields["timestamp"]` present as non-empty string; §Postconditions Event 6 Mandatory-fields) — DELIVERED (D22). F-P8-001/F-P9-001 through pass-13 corrected BC-trace citation class from §Common Fields to §Postconditions.

No accepted-risk deviations.

---

## Section 3 — Scope Evolution During the Epic

### 3.1 Stories Added Mid-Epic

E-19 opened with 7 stories (S-19.01–S-19.07). Two stories were added during execution:

**S-19.08 — added per D-835 (same defect class as S-19.02, missed in original sweep)**

S-19.02 addressed the `STATE_MD_MAX_BYTES = 65536` OUTPUT_TOO_LARGE defect in the `verify-factory-lock` plugin. After S-19.02 converged, the cycle-level adversary identified that `verify-state-timestamp-refresh` — a sibling plugin with an identical pattern — had the same defect (reading large STATE.md files through a 64 KiB cap). D-835 authorized S-19.08 as a PRIORITY-A scope extension. Wave 2, P2, 5 points.

Authorization provenance: D-835; lesson codified as L-BB-remediation-sweeps-must-cover-the-entire-crate-for-the-finding-class.

**S-19.09 — added per D-847 (CRITICAL production-path gap discovered post-S-19.06)**

S-19.06 delivered `host::read_prefix` and registered it in `setup_linker` (`Linker<HostContext>`, the unit-test invocation path). The production dispatch path `setup_host_on_store_data` (`Linker<StoreData>`) was not updated. This made every operator-visible `read_prefix` call non-functional on the actual plugin dispatch path — a CRITICAL gap. D-847 authorized S-19.09 as PRIORITY-A to close D19 (production path registration), D20 (timeout_ms framing corrections), D21 (telemetry constants), and D22 (missing timestamp field) before S-19.07 could begin.

Authorization provenance: D-847; D-847 also added S-19.09 to S-19.07's `depends_on` array at S-19.07 v1.17.

### 3.2 Depends-On Rewires

- **S-19.07** `depends_on` was originally `[S-19.02, S-19.06]`. At v1.17 (D-847 propagation), `S-19.09` was added, making it `[S-19.02, S-19.06, S-19.09]`. This correctly captured the D19 production-path requirement: S-19.07's `verify-factory-lock` Phase-B migration would link-fail on production dispatch without D19's `read_prefix` registration in `setup_host_on_store_data`.
- **S-19.03** originally had `blocks: []`; at pass-3 F-P3-010, `blocks: [S-19.06]` was added. This reflected the architectural dependency: S-19.06 (read_prefix) needed S-19.03's `codes::NOT_FOUND` infrastructure for consistent error propagation.

### 3.3 Decomposition Quality Assessment

**Strengths:**
- Wave structure (W1: S-19.01–S-19.03, W2: S-19.04–S-19.06+S-19.08, W3: S-19.07+S-19.09) correctly sequenced dependencies. No wave-gate blocking occurred due to incorrect ordering.
- S-19.01–S-19.03 addressed the originally identified rc.22 smoke findings (FINDING-1, FINDING-2, plus pr-manager findings) cleanly in Wave 1.
- S-19.06's "additive FFI entry point" framing was correct — the new function was non-breaking and traceable to BC-1.17.001.

**Gaps in original decomposition:**
1. **Sibling-plugin sweep incomplete at epic kick-off.** S-19.02 was scoped to `verify-factory-lock` only. The sister plugin `verify-state-timestamp-refresh` had the same defect. A systematic "find all plugins that call `host::read_file` with `STATE_MD_MAX_BYTES`" at epic start would have identified S-19.08's scope in Wave 1. This required S-19.08 to be inserted mid-epic as Wave 2 catch-up, consuming 5 additional points.
2. **Production-path linker registration not validated at S-19.06 acceptance.** S-19.06's acceptance gate verified `setup_linker` registration (test-path) but did not include a gate on `setup_host_on_store_data` (production path). The dual-linker architecture (`Linker<HostContext>` vs `Linker<StoreData>`) is a pattern in this codebase (documented post-S-19.09 in AC-005). A standard gate for new host function stories should verify BOTH linker registrations. This required S-19.09 D19 (CRITICAL) to be added post-S-19.06 delivery, blocking S-19.07.
3. **Original epic had no E-20 surface-area audit.** The no-new-.sh standing policy introduced in S-19.04 immediately surfaced 6+ existing `.sh` files in `plugins/vsdd-factory/bin/` that predate the policy and require future migration. This E-20 obligation was not visible at epic start.

**Net assessment:** E-19's original 7-story decomposition was structurally sound for the known rc.22 findings. The two additions (S-19.08, S-19.09) were genuine discovery-during-execution finds — not planning failures. The 55-point actual vs original ~42-point estimate (+31%) is within normal range for a hardening epic where defect class sweep is done incrementally. The critical path (S-19.02 → S-19.06 → S-19.09 → S-19.07) was not on the original plan and added 3 days to the epic schedule.

---

## Section 4 — Carried/Deferred Items with Routing Status

### 4.1 S-19.09 Micro-Cleanup Records (3 items — proposed routing)

These three items were identified during S-19.09's scope analysis and LOCAL cascade but were explicitly excluded from S-19.09's scope by ADR-025 §Decision 19 ("No changes to `hook-sdk`, `hooks-registry.toml`, `HOST_ABI_VERSION`, or any `.factory/` spec files").

**Item 1: timeout_ms epoch-interrupt structural enforcement**

S-19.09 D20 corrected the `timeout_ms` doc comments in `read_file.rs` and `read_prefix.rs` from "epoch interruption" (aspirational) to "structurally unenforced" (accurate). Both functions accept `timeout_ms: i32` and drop it unconditionally with `let _ = timeout_ms;`. The actual mechanism for epoch-interrupt-based WASM preemption at a timeout boundary is architecturally undesigned. `HOST_ABI_VERSION` has not been incremented to reflect a semantics change.

Proposed routing: **`vsdd-factory:architect`** — requires ADR-025 amendment to §Decisions 17–18 with a concrete epoch-interrupt design decision, followed by a new E-20 story for implementation. The architect story should also adjudicate whether `HOST_ABI_VERSION` must increment when timeout_ms gains enforcement semantics (breaking change for plugins that pass non-zero timeout_ms expecting no-op behavior).

**Item 2: hook-sdk timeout_ms wrapper contract alignment**

The `hook-sdk` safe wrappers for `read_file` and `read_prefix` accept `timeout_ms: i32` and pass it through to FFI. After D20's framing correction, the SDK wrappers' doc comments should reflect the "structurally unenforced" reality to prevent plugin authors from relying on timeout enforcement. S-19.09 §Decision 19 excluded hook-sdk changes; this sweep was not done.

Proposed routing: **`vsdd-factory:implementer`** (doc-only; can proceed once architect confirms the final framing for the SDK contract; blocked on Item 1 architect decision). Estimated effort: 1 hour.

**Item 3: SEC-003 CWE-833 structural verification post-enforcement**

SEC-003 (CWE-833, LOW severity) was closed in S-19.09 with the framing correction. The CWE-833 concern (lock order inversion / deadlock potential) is LOW severity precisely because timeout_ms is currently unenforced — no actual blocking wait occurs. Once Item 1 implements epoch-interrupt preemption, the CWE-833 risk profile changes: the dispatcher will hold locks while performing bounded-wait operations, which could deadlock if lock order is incorrect.

Proposed routing: **`vsdd-factory:security-reviewer`** — re-open SEC-003 as a verification gate on the architect follow-up story from Item 1. The security reviewer should validate that the epoch-interrupt design does not introduce lock-order hazards before the story merges.

### 4.2 F-670-02/03/08 LOW Dispositions (PR #670 — S-19.07 merge review)

These three findings emerged from the PR #670 code review (S-19.07: verify-factory-lock read_prefix migration). Their dispositions were adjudicated at epic closure session:

**F-670-02 (LOW): bats-exempt**

Finding: A bats test in `verify-factory-lock-read-prefix.bats` uses a string pattern that would fail on BSD-incompatible sed syntax on macOS. Disposition: bats-exempt. The bats test suite runs exclusively on Linux CI (`bats-full-suite` job); the darwin-leg `bats-darwin-leg-macos` job covers only S-19.01 enforcement scripts (not verify-factory-lock bats). The incompatible sed syntax was already corrected in the story spec at F-P4-002 (portable `-e` form). No action required.

**F-670-03 (LOW): RELEASE-GATE-covered**

Finding: The `verify-factory-lock.wasm` binary committed at S-19.07 HEAD was built from the Phase-B source but the build provenance is not verified in CI. Disposition: RELEASE-GATE-covered. The `release.yml` workflow rebuilds all WASM binaries from source at release time via cross-compilation; the binary in source is a development artifact. The WASM binary committed to the feature branch is replaced by the release pipeline binary at every `release/v*` branch cut. No action required; this is standard operating procedure for WASM hook plugins.

**F-670-08 (LOW): folds-into-architect-follow-up**

Finding: The `capabilities.read_prefix` schema in the hooks-registry.toml preamble block describes `max_bytes` semantics but does not specify the interaction between `max_bytes` and `timeout_ms` in the presence of a future enforcement implementation. If `timeout_ms` is enforced (see Item 1 above), the schema would need to describe the partial-read-on-timeout behavior. Disposition: folds-into-architect-follow-up. The schema amendment is a natural output of the architect story for Item 1 (timeout_ms epoch-interrupt design). No standalone action required.

### 4.3 Story Frontmatter Deferred Items

No story in E-19 carries a `tech_debt_register:` entry or explicit deferred-items frontmatter array. The `closes:` arrays in all 9 stories are fully resolved (no open finding references at epic close). The only structural deferrals are the three micro-cleanup items enumerated in Section 4.1 above.

One boundary note: S-19.05's `VSDD_ASYNC_DRAIN_WINDOW_MS` gate (`cfg(any(debug_assertions, feature = "test-support"))`) is intentionally NOT removed. The test-support feature is CI-only. This is a design decision, not a deferral — it prevents drain-window saturation in production without sacrificing testability.

---

## Section 5 — E-20 Candidates from the Story Lens

Based on the deliverables and scope boundaries of E-19, the following items have been identified as E-20 candidates. This section does not create stories; it surfaces anchored signals for the next epic decomposition.

### 5.1 POLICY 21 Shell Script Migration (D-846; anchored to E-20)

The no-new-.sh standing policy (human directive 2026-07-13; codified in POLICY 20 context as standing rule) prohibits any new shell scripts, including CI/test helpers. POLICY 21 (`.sh migration`) requires EXISTING shell scripts in `plugins/vsdd-factory/bin/` to be migrated to WASM hook plugins or Rust workspace tests.

Currently identified `.sh` scripts that predate the standing policy and are candidates for migration:

- `plugins/vsdd-factory/bin/check-stale-verdict.sh` (S-19.01 delivery; invoked by pr-manager-completion-guard WASM as an external binary)
- `plugins/vsdd-factory/bin/enforce-merge-strategy.sh` (S-19.01 delivery; sole gateway for `gh pr merge` calls)
- Any pre-existing `.sh` hook scripts in `plugins/vsdd-factory/hooks/` — convergence-tracker.sh and similar

Per D-846, this migration is anchored to E-20. The architect follow-up story (also D-846) is needed first to adjudicate the migration strategy — in particular whether `check-stale-verdict.sh` and `enforce-merge-strategy.sh` can be collapsed into the `pr-manager-completion-guard` WASM plugin or require separate WASM plugins.

Proposed routing for decomposition: **`vsdd-factory:architect`** (D-846 follow-up story) to produce the migration ADR, then **`vsdd-factory:story-writer`** to decompose into per-script migration stories.

### 5.2 timeout_ms Epoch-Interrupt Implementation (from Section 4.1 Item 1)

An architect story is needed to design the epoch-interrupt mechanism for `host::read_file` and `host::read_prefix`. This is a HOST_ABI_VERSION-affecting change. Estimated as a small architect story + medium implementer story.

### 5.3 STATE.md Structured Changelog Migration (S-15.03 PRIORITY-A; inlined last_amended byte-bloat)

S-19.07 F-P1-001 BLOCKER root cause analysis identified that the `last_amended` inline frontmatter field is the primary driver of STATE.md byte growth (measured 32,648 bytes for `last_amended` alone on 2026-07-16). This explains why the 262144-byte cap is necessary even with line-count discipline (ADR-026 §Decision 7 is a line-count rule, not a byte-count rule). S-15.03 PRIORITY-A structured-changelog migration is the correct long-term fix. This is an existing backlog item but E-19 provides strong measured evidence for prioritization.

### 5.4 hook-sdk WASM Fuel Budget Alignment for read_prefix

S-19.06 introduced `read_prefix` with the same fuel budget semantics as `read_file`. If `read_prefix` is used by multiple plugins in a future wave, the combined fuel consumption of partial reads may exhaust plugin budgets. No current finding; this is a proactive monitoring candidate for E-20's first sprint.

### 5.5 Dual-Linker Gate for New Host Function Stories

The production-path gap found in S-19.09 (D19) reveals a missing standard gate: new host function stories should verify registration in BOTH `setup_linker` (test path) and `setup_host_on_store_data` (production path). This gate should be added to the implementer checklist template or as an Architecture Compliance Rule in the host function story template. Proposed routing: **`vsdd-factory:story-writer`** (template update) + **`vsdd-factory:architect`** (ACR wording approval).

---

## Section 6 — Lessons for Future Epic Decomposition

These observations are process-level lessons drawn from E-19's execution. Formal codification into `lessons.md` (L-EDP1-NNN) stays with the state-manager.

### L1: Sibling-plugin defect-class sweep at epic start saves mid-epic insertions

S-19.08 was added because S-19.02's remediation of `verify-factory-lock` did not sweep all plugins sharing the same defect class (`host::read_file` with a hard-coded byte cap). For future hardening epics, the first story should include an explicit "sibling sweep" gate: identify all plugins (or all code sites) in the same defect class before authoring individual stories. This converts scope surprises into planned work.

### L2: Dual-linker registration is a load-bearing gate for all host function stories

The `Linker<HostContext>` (test path) and `Linker<StoreData>` (production path) are distinct registration targets in the factory-dispatcher. S-19.06 passed AC verification on the test-path linker only. S-19.09 D19 (CRITICAL) was the consequence. Future host function stories — any story registering a new or modified host function — must gate on BOTH linker registration sites before declaring the story converged. This should be a mandatory Architecture Compliance Rule in the host-function story template.

### L3: Claim-class sweeps must cover semantic equivalents, not syntactic patterns

S-19.09 required three passes (F-P8-001, F-P9-001, F-P10-001) to fully correct the `§Common Fields` vs `§Postconditions Event 6 Mandatory-fields` citation class for the `timestamp` field in BC-3.08.001. Each pass corrected one syntactic form but left equivalent forms untouched. The lesson: when a sweep corrects a semantic claim-class (not just a version number), it must also search for synonyms and paraphrases of the incorrect class. A semantic-class gate (`grep -n "Common Fields"`) should be run as the closure assertion, not a targeted single-pattern replace_all.

### L4: Mid-epic critical-path additions require explicit blocking-chain reassessment

When S-19.09 was added with priority P1 (D-847), S-19.07's `depends_on` was correctly updated. But this also changed the critical path from S-19.06 → S-19.07 (6 days) to S-19.06 → S-19.09 → S-19.07 (10 days). Future mid-epic story insertions should include an explicit critical-path recalculation step as part of the D-NNN decision record, so the schedule impact is immediately visible to the orchestrator.

### L5: Scope boundaries between related stories benefit from cross-story integration gates

S-19.05 delivered `VSDD_SINK_FILE` for release builds and established `flush_sink_file`. S-19.09 D21 found that `emit_plugin_completed_async` and `emit_plugin_timeout_async` in `emit_event.rs` used bare literals after D21 swept the sibling functions — because the sweep was scoped per-function rather than per-file. When multiple stories touch the same file, a per-file hygiene gate (e.g., zero bare event-type literals across the entire file) should be added to each story touching that file. This prevents partial sweeps that pass story-level acceptance but fail cross-story consistency.

### L6: Accepted-risk deferrals need explicit E-20 anchors at time of acceptance

The F-670-02, F-670-03, F-670-08 findings from S-19.07 were adjudicated at epic close, not during the story delivery cascade. Future findings accepted with LOW severity should carry an explicit anchor at time of acceptance: either a TD register entry (if human-directed) or an explicit statement of which E-20 story will address them. This prevents ambiguity at epic close where routing is "undecided" rather than pre-assigned.

### L7: New standing policies introduced mid-epic generate immediate carry obligations

The no-new-.sh standing policy (human directive 2026-07-13, S-19.04 delivery) immediately created a POLICY 21 migration obligation for 6+ pre-existing `.sh` files. Future standing policies that have retroactive scope should generate a migration stub story in the current epic's task list at the time they are established, anchored to the next epic. This prevents the obligation from floating until the next epic decomposition session.

---

*Report authored 2026-07-17. Develop HEAD at epic close: 6db4c9fc. All story files, STORY-INDEX.md, BC files, and VP files are unmodified by this report.*
