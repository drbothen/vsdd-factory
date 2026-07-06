---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-06T00:00:00Z
phase: 2
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 2
previous_review: adv-E19-pass-1.md
perimeter: E-19 epic + S-19.01..S-19.05 + STORY-INDEX E-19 section
verdict: NOT-CLEAN
blocker_count: 0
high_count: 3
medium_count: 6
low_count: 4
observation_count: 5
streak: 0/3
parent_decision: D-752
---

# Adversarial Review — E-19 Pass 2 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.05 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law)
**Date:** 2026-07-06
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 3 / MEDIUM 6 / LOW 4 + 5 observations
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P2-001`, `F-P2-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-18 and E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-1 NOT-CLEAN B1/H9/M5/L1 (15 findings + 5 observations). Same-burst fix by 4 specialist legs (D-752). Fresh-context adversary reads only prior Part A — findings F-P1-001..F-P1-015. All 15 findings verified CLOSED by artifact evidence at pass-2 perimeter entry. New findings from pass-2 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften.*

F-P2-001 — HIGH — S-19.04 blanket regex-anchoring removes MultiEdit coverage from safety-critical hooks. The S-19.04 v1.2 fix anchors all tool-filter patterns (e.g., `^Edit$|^Write$|^Agent$`) to prevent substring matches — correct for the declared goal. However, ADR-025's 2026-06-11 note records that `verify-factory-lock` fires on `Edit|Write` precisely because substring semantics covers `MultiEdit` via the `Edit` substring. The anchored form without explicit `MultiEdit` is a silent P0 single-writer-gate regression: a MultiEdit call to STATE.md would bypass the lock gate entirely. AC-004 in S-19.04 does not include a MultiEdit positive-control test. Routing: product-owner + architect.

F-P2-002 — HIGH — S-19.01 ACs are LLM-agent prompt-spec changes with no mechanical enforcement code path. AC-001 specifies pr-manager story verdict pinning behavior; AC-003 specifies a GitHub CLI enforcement flag; but neither has a mechanical gate: bats cannot gate non-deterministic LLM agent output, and pr-manager-completion-guard.wasm exists in the registry but is unreferenced by any AC. The story ships changes to an agent's LLM prompt spec with no verifiable enforcement mechanism. Routing: architect.

F-P2-003 — HIGH — S-19.02 and BC-4.13.001 headroom rationale materially stale. BC Precondition 3 and S-19.02 cite ~90 KB / 466 lines as the file-size basis for the 262144-byte cap justification. At review time STATE.md is 193,220 bytes / 488 lines — 2× growth in 2 days, reducing stated 28% headroom to approximately 14%. The cap was retained as correct but the justification is now factually wrong and will fail fresh-context adversary verification within 1-2 passes. Routing: product-owner + architect.

F-P2-004 — MEDIUM — S-19.03 path_util hoist changes internal API anchored by BC-2.02.011. S-19.03 v1.2 adds path_util module extraction as a task; BC-2.02.011 anchors the write_file.rs behavioral contract to a specific file-level module structure. Moving path_util to a new module location changes the §Architecture Anchors for BC-2.02.011 at implementation time without a BC amendment. The story's sibling-sweep acknowledges write_file.rs but does not include a BC-2.02.011 anchor-update obligation. Routing: spec-steward + product-owner.

F-P2-005 — MEDIUM — S-19.02 cap-raise treats symptom; read_bounded is all-or-nothing by design. The root defect (FINDING-1 from smoke Leg 2) is that `verify-factory-lock` triggers `OutputTooLarge` for large STATE.md files. S-19.02 v1.2 raises the cap to 262144 bytes, but `read_bounded()` performs a `metadata.len()` preflight check — if the file exceeds cap, the read returns an error, no partial data. For continued STATE.md growth the cap will be breached again. A bounded-partial-read function (`read_prefix`) that reads up to N bytes regardless of total file size is the durable fix; cap-raise is a temporary symptom treatment. Routing: architect (scope expansion candidate).

F-P2-006 — MEDIUM — E-19 epic `depends_on: [E-18]` is soft ordering mis-typed as a hard DAG dependency. All 5 stories (S-19.01..S-19.05) correctly have `depends_on: []`. The epic-level `depends_on: [E-18]` is legacy narrative carry from draft authoring; in VSDD story-graph semantics, a `depends_on` on the epic does not gate the stories' W1 dispatch and creates false DAG entries. Routing: story-writer.

F-P2-007 — MEDIUM — S-19.03 cites nonexistent path `crates/factory-dispatcher/src/host/codes.rs`. The codes module is inline in `crates/factory-dispatcher/src/host/mod.rs`, not a separate file. Per POLICY 5, anchors must cite existing artifacts. A wrong path in the anchor table causes AC gate failures and is a POLICY 5 violation. Routing: story-writer.

F-P2-008 — MEDIUM — S-19.05 name-only abandoned set-difference fragile under registry multi-entry-per-name idiom. S-19.05 AC-002 specifies identifying abandoned plugins by `plugin_name` alone. `verify-factory-lock` appears twice in hooks-registry.toml (two separate entries — PreToolUse and PostToolUse). A name-only set-difference that expects singleton names will incorrectly mark the second entry as abandoned. The correct key is `(plugin_name, entry_index)`. A schema field for `entry_index` is also missing from the BC. Routing: architect + product-owner.

F-P2-009 — MEDIUM — S-19.04 AC-004 gate flunks legitimate singletons. The proposed lint gate rejects tool-filter patterns where a plugin targets a single canonical tool like `Bash` or `Read`. Under current taxonomy those are legitimate singleton registrations, not under-anchored patterns. The gate has no carve-out and would fail valid registry entries on every run. A policy decision is needed on whether singletons require explicit justification or are always permitted. Routing: architect.

F-P2-010 — LOW — S-19.05 AC-006 CLAUDE.md gate lacks stale-text negative control. The AC-006 grep gate verifies VSDD_SINK_FILE appears in CLAUDE.md but has no negative control proving the grep would fail if the text were removed. A positive-only gate can silently pass on stale CLAUDE.md content. Routing: technical-writer/story-writer.

F-P2-011 — LOW — S-19.02 AC-005 "parity-with-full-file-parse" tests a path Invariant 9 forbids. BC-4.13.001 Invariant 9 mandates frontmatter-only extraction to avoid full-file parse overhead. AC-005 requires byte-identical output between `read_bounded` and full-file read for files within cap — this assertion requires internally running the full-file path to generate the comparison, violating Invariant 9. The AC should be recast as a byte-boundary property test. Routing: product-owner/story-writer.

F-P2-012 — LOW — No baseline test-count regression capture in Red Gate protocol across all 5 stories. None of S-19.01..S-19.05 specifies a test-count baseline in the Red Gate table or AC-level gate (e.g., `cargo test --workspace 2>/dev/null | tail -1` → N tests). Without a baseline, implementers cannot confirm that stub compilation doesn't silently drop tests. Routing: story-writer.

F-P2-013 — LOW — S-19.01 macos-latest pointer drift unmanaged. EC-003 adds a dedicated `bats-darwin-leg-macos` CI job targeting `macos-latest`. `macos-latest` is a GitHub-managed pointer that can advance to a new macOS major without warning; ADR-025 Decision 14 records that Apple patched 3.2.57 specifically on the current runner. When GitHub advances the pointer, the workaround assumption breaks silently. Routing: devops/architect.

Observations: O-P2-001 bundle-policy authority (inline policy in story narrative → route to policies.yaml registration rather than new ADR). O-P2-002 `#[non_exhaustive]` question on future codes additions — Other(i32) catch-all would maintain forward-compatibility. O-P2-003 cfg-gated Mutex import moves with sink block; no correctness issue but the import guard could migrate with the block on the next refactor. O-P2-004 epic EAC-005 conflicts with S-19.04 keep-assertion absent dual-registry clause — the keep-assertion added at v1.1 is not reflected in EAC-005 wording, creating a surface ambiguity. O-P2-005 `verification_properties: []` across all stories despite the P0-gate changes in S-19.03/S-19.04 — VP determination has not been run; VPs should be assigned before implementation begins.

Verdict: NOT-CLEAN. BLOCKER 0 / HIGH 3 / MEDIUM 6 / LOW 4.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 3 |
| MEDIUM | 6 |
| LOW | 4 |
| Observations | 5 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Severity decay from pass 1:** B1/H9/M5/L1 → B0/H3/M6/L4 (BLOCKER resolved; HIGH count reduced)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 13 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (13 / 13) |
| **Median severity** | MEDIUM |
| **Trajectory** | 15 → 13 (severity decay: B resolved, H from 9→3) |
| **Verdict** | FINDINGS_REMAIN — pass 3 dispatched with fresh context |

---

## Fix-Burst Closure Section (D-753)

**Full-scope fix burst (human-approved including S-19.06 scope expansion + full VP/BC authorship) executed across 6 specialist legs. 13 findings closed + 5 observations adjudicated.**

**Human approvals:** (a) S-19.06 new story for `read_prefix` bounded partial read (F-P2-005 durable fix — scope expansion); (b) full VP/BC authorship in this burst (VP-094..VP-101 NEW + VP-079 amended; 3 new BCs); (c) retroactive correction of stale 4-index quads in BC-INDEX changelog rows v3.58/v3.59 (wrong-at-authoring, correctable per orchestrator adjudication).

**Streak: 0/3** — pass-2 verdict is NOT-CLEAN regardless of same-burst fixes per D-628; pass-3 NEXT with fresh context.

### Architect leg — Decisions D-a..D-g + 2 adjudications

- **Decision D-a (F-P2-001 MultiEdit parity):** `verify-factory-lock` anchoring table MUST include explicit `MultiEdit` entry per ADR-025 2026-06-11 note; anchored form preserves substring-semantics parity. S-19.04 AC-004 positive-control test for MultiEdit added.
- **Decision D-b (F-P2-002 3-component S-19.01 enforcement):** pr-manager-completion-guard.wasm extension + `check-stale-verdict.sh` script + `enforce-merge-strategy.sh` script constitutes the 3-component mechanical enforcement. Bats integration tests provide verifiable gate. S-19.01 ACs updated accordingly.
- **Decision D-c (F-P2-003 cap RETAINED):** 262144-byte cap is correct and retained. Headroom rationale updated in BC-4.13.001 Precondition 3 + S-19.02 to cite live values at review time (193,220 bytes / 488 lines) with explicit soft_warn_threshold=200000 (Invariant 10).
- **Decision D-d (F-P2-005 read_prefix scope expansion):** `read_prefix` approved as new host ABI entry point (`host::read_prefix(path, max_bytes) → Result<Vec<u8>, HostError>`); reads up to max_bytes regardless of total file size; NEVER returns `OutputTooLarge`; additive (HOST_ABI_VERSION=1 unchanged per ADR-025). S-19.06 created as W2 story (8pts, depends_on S-19.03, BC-1.17.001/VP-101). ABI ruling recorded in ADR-025 Decision 15.
- **Decision D-e ((plugin_name, entry_index) keying for F-P2-008):** Abandoned-set key changed to `(plugin_name, entry_index)` composite; `entry_index: u32` schema field added to BC-3.08.001 Event 5 `plugin.abandoned`; S-19.05 AC-002 updated.
- **Decision D-f (uniform singleton anchoring for F-P2-009):** Singletons (Bash, Read, Agent, etc.) that match a single canonical tool by design are permitted without warning; AC-004 lint gate carve-out added. POLICY decision: legitimate singletons do NOT require explicit justification field.
- **Decision D-g (macos-latest retained for F-P2-013):** macos-latest retained with pre-flight sentinel: CI step asserts `sw_vers -productVersion` major ≥ 14 (current Sonoma baseline) and emits a structured warning if the pointer advances. No pin at this time; pointer-drift detection via sentinel suffices.
- **Adjudication 1 (O-P2-001 bundle-policy):** bundle-policy authority belongs in `policies.yaml` registration, NOT a new ADR. Registered as POLICY 20 `release_bundle_no_dev_samples`. No ADR-030 created.
- **Adjudication 2 (O-P2-002 #[non_exhaustive]):** `Other(i32)` catch-all adopted in `HostError` enum; `#[non_exhaustive]` NOT added (the Other variant already provides forward-compatibility without the unstable-pattern problem).
- **ADR-025 v1.7→v1.8:** Decision 15 read_prefix (new host fn; additive; ABI v1 unchanged); headroom rationale updated; MultiEdit parity note for anchoring table. ARCH-INDEX v2.86→v2.87.
- **VP-094..VP-101 NEW (8 VPs) + VP-079 amended:** VP-094 (S-19.01 completion-guard bats gate); VP-095 (S-19.01 check-stale-verdict shell integration); VP-096 (S-19.02 frontmatter extraction correctness); VP-097 (S-19.02 soft_warn_threshold emit); VP-098 (S-19.03 absent-file NOT_FOUND return); VP-099 (S-19.04 MultiEdit anchoring gate); VP-100 (S-19.05 abandoned-set entry_index key); VP-101 (S-19.06 read_prefix byte-boundary correctness); VP-079 amended (singleton-carve-out predicate). VP-INDEX v2.51→v2.52. POLICY 9 propagation to verification-architecture.md + verification-coverage-matrix.md CONFIRMED same-burst.

### Spec-steward leg

- **BC-2.02.011 anchor prescriptions confirmed (F-P2-004):** §Architecture Anchors updated to include path_util.rs bullet; §Traceability Architecture Module updated. BC-2.02.011 v1.3→v1.4. Sibling-sweep of stories referencing BC-2.02.011 confirmed.
- **7-VP determination completed (O-P2-005):** VP assignments resolved for all 5 stories + new S-19.06; VP-094..VP-101 routing confirmed with architect.
- **2 BC gaps closed:** BC-5.42.001 (pr-manager READY-verdict pin covering covered_sha + check-stale-verdict.sh + enforce-merge-strategy.sh; S-19.01) and BC-2.07.001 (host::read_file absent-file semantics: codes::NOT_FOUND=-5 additive, HostError::NotFound, rejoin path-allowed resolution, zero false-positive capability_denied; S-19.03) and BC-1.17.001 (host::read_prefix bounded partial read, head-c semantics, NEVER OUTPUT_TOO_LARGE, additive FFI entry point; S-19.06) all authored and routed to product-owner.

### Product-owner leg

- **BC-4.13.001 v1.4→v1.5:** MultiEdit explicitly enumerated in tool-filter table; Precondition 3 rationale updated to 193,220 bytes/488 lines (2026-07-06 measurement); cap RETAINED 262144; Invariant 10 soft_warn_threshold=200000 added. Closes F-P2-001 (BC leg), F-P2-003 (BC leg).
- **BC-3.08.001 v1.15→v1.16:** `plugin.abandoned` Event 5 adds `entry_index: u32` field; Invariant 6 terminal-semantics key extended to `trace_id + plugin_name + entry_index`. Closes F-P2-008 (BC leg).
- **BC-2.02.011 v1.3→v1.4:** §Architecture Anchors path_util.rs bullet appended; §Traceability Architecture Module path_util.rs appended. Closes F-P2-004.
- **NEW BC-5.42.001 v1.0:** pr-manager READY-verdict covered_sha pin + check-stale-verdict.sh + enforce-merge-strategy.sh behavioral contract (S-19.01). SS-05.
- **NEW BC-2.07.001 v1.0:** host::read_file absent-file semantics (codes::NOT_FOUND=-5, HostError::NotFound, path-allowed rejoin, zero false-positive capability_denied) (S-19.03). SS-02.
- **NEW BC-1.17.001 v1.0:** host::read_prefix bounded partial read, head-c semantics, NEVER OUTPUT_TOO_LARGE, additive FFI entry point (S-19.06). SS-01.
- **BC-INDEX v3.59→v3.65** (6 version bumps: v3.60 BC-4.13.001 v1.5 + BC-3.08.001/F-P2-003; v3.61 BC-3.08.001 v1.16; v3.62 BC-2.02.011 v1.4; v3.63 BC-5.42.001 v1.0 NEW; v3.64 BC-2.07.001 v1.0 NEW; v3.65 BC-1.17.001 v1.0 NEW). total_bcs 1,974→1,977.
- **RETROACTIVE REMEDIATION:** BC-INDEX changelog rows v3.58/v3.59 (authored at D-752 burst) contained stale 4-index quads (wrong-at-authoring due to cross-lane architect edit); quads re-derived from live headers at D-753 authoring time per L-BB-4index-parity-rederive-from-live-headers. Orchestrator-approved correctable per POLICY 1 append-only (retrospective accuracy correction, not retroactive rewriting of facts).

### Story-writer leg

- **S-19.06 v1.0 NEW:** `host::read_prefix` bounded partial read implementation story (8pts; W2; depends_on: [S-19.03]; behavioral_contracts: [BC-1.17.001]; verification_properties: [VP-101]). Closes F-P2-005 durable-fix path.
- **S-19.01 v1.1→v1.2:** 3-component mechanical enforcement: pr-manager-completion-guard.wasm extension + check-stale-verdict.sh + enforce-merge-strategy.sh; target_module agents path verified; EC-003 darwin-leg sentinel added; AC-001 positive assertion retained; behavioral_contracts updated to include BC-5.42.001; verification_properties VP-094/VP-095 assigned.
- **S-19.02 v1.1→v1.2:** AC-005 recast as byte-boundary property test (Invariant 9 compliant, not full-file compare); soft_warn_threshold=200000 AC added; verification_properties VP-096/VP-097 assigned.
- **S-19.03 v1.2→v1.3:** `crates/factory-dispatcher/src/host/codes.rs` path corrected to `crates/factory-dispatcher/src/host/mod.rs codes module`; behavioral_contracts updated to include BC-2.07.001; verification_properties VP-098 assigned. Closes F-P2-007.
- **S-19.04 v1.2→v1.3:** MultiEdit positive-control test AC added; singleton carve-out in AC-004 lint gate documented; verification_properties VP-099 assigned. Closes F-P2-001 (story leg), F-P2-009.
- **S-19.05 v1.1→v1.2:** AC-002 `(plugin_name, entry_index)` composite key; entry_index schema documented; verification_properties VP-100 assigned. Closes F-P2-008 (story leg).
- **E-19 epic v1.1→v1.2:** `depends_on: [E-18]` corrected to `depends_on: []`; EAC-005 dual-registry clause added (keep-assertion alignment); story count updated to 6 (S-19.01..S-19.06; 42pts). Closes F-P2-006.
- **STORY-INDEX v4.131→v4.132** (S-19.06 NEW + all 5 story bumps + epic v1.2 + DRIFT ITEM surfaced: STORY-INDEX frontmatter lists legacy input `.factory/stories/v1.0/EPIC.md` which no longer exists, blocking `compute-input-hash` on STORY-INDEX; deferred to next maintenance sweep per POLICY 1 DRIFT tracking).

### POLICY 20 registration

- `release_bundle_no_dev_samples` registered as POLICY 20 (id 20; task referenced "next sequential id 17" but ids 17-19 already existed per no-collision verification; next available is 20). Scope: release, bundle. Severity: HIGH. Source: architect O-P2-001 adjudication + S-19.04 + L-BB-orphan-status-requires-dual-registry-check.

### Artifact versions at pass-2 closure

| Artifact | Version |
|----------|---------|
| BC-4.13.001 | v1.5 |
| BC-3.08.001 | v1.16 |
| BC-2.02.011 | v1.4 |
| BC-5.42.001 | v1.0 (NEW) |
| BC-2.07.001 | v1.0 (NEW) |
| BC-1.17.001 | v1.0 (NEW) |
| BC-INDEX | v3.65 (total_bcs 1,977) |
| ADR-025 | v1.8 |
| ARCH-INDEX | v2.87 |
| VP-079 | amended |
| VP-094..VP-101 | v1.0 (NEW) |
| VP-INDEX | v2.52 |
| S-19.01 | v1.2 |
| S-19.02 | v1.2 |
| S-19.03 | v1.3 |
| S-19.04 | v1.3 |
| S-19.05 | v1.2 |
| S-19.06 | v1.0 (NEW) |
| E-19 epic | v1.2 |
| STORY-INDEX | v4.132 |
| POLICY 20 | registered |

**NEXT:** E-19 adversarial pass-3 (fresh context; streak 0/3).
