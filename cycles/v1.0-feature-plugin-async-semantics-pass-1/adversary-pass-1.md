---
document_type: adversary-pass
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F2
pass: 1
producer: adversary
producer_model: claude-opus-4-7[1m]
fresh_context: true
verdict: SUBSTANTIVE
finding_count: { high: 6, medium: 7, low: 4, nit: 2 }
adr-013_clock_action: reset
clock: 0_of_3
timestamp: 2026-05-07T00:00:00Z
---

# Adversary Pass-1 Findings — F2 spec package, v1.0-feature-plugin-async-semantics-pass-1

## 1. Verdict

**SUBSTANTIVE.** The F2 package is internally coherent for the artifacts it covers, but is **incomplete**: 3 of the 8 events that ADR-019 §Decision 1 enumerates (PostToolUse, Stop, SubagentStop) have no governing template-registration BC and no amendment to remove `async: true`. SS-09 architecture text and DI-014 still describe schema_version=1. These omissions block convergence.

## 2. Finding count summary

| Severity | Count |
|---|---|
| HIGH | 6 |
| MEDIUM | 7 |
| LOW | 4 |
| NIT | 2 |

## 3. Findings

### F-P1-001 [HIGH]: PostToolUse / Stop / SubagentStop envelope changes lack any governing BC or amendment

**Location:** ADR-019 §Decision 1 (line 76-77); `plugins/vsdd-factory/hooks/hooks.json.template` lines 14-25, 37-60 (PostToolUse, Stop, SubagentStop entries still `"async": true`); BC-1.14.001 Precondition 2 (line 41).

**Observation:** ADR-019 §Decision 1 explicitly enumerates 9 events ("PreToolUse, PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove, PostToolUseFailure"). Only 5 of those events have BCs amending `async: true` removal (BC-4.04.004 SessionStart; BC-4.05.004 SessionEnd; BC-4.07.003 WorktreeCreate+Remove; BC-4.08.002 PostToolUseFailure). PostToolUse, Stop, and SubagentStop are NOT addressed by any amended BC, and no new BC has been authored for them. PreToolUse/PermissionRequest are already sync, but PostToolUse/Stop/SubagentStop currently carry `"async": true` in `hooks.json.template` (lines 21, 44, 56) and there is no specification authority that requires their removal.

**Why it matters:** This is the exact bug ADR-019 was created to fix. If PostToolUse remains async, the prism audit's 55 silently-discarded `validate-template-compliance` block decisions continue silently — because `validate-template-compliance` fires on PostToolUse. The F2 package is incomplete to fix the very bug it targets.

**Suggested fix direction:** Either (a) author 3 new BCs in SS-04 (BC-4.NN.NNN for each of PostToolUse, Stop, SubagentStop hooks.json.template registrations) that mandate `async` key absent, or (b) author a single SS-09 BC (BC-9.NN.NNN) that asserts the post-amendment uniformity invariant: "every event entry in hooks.json.template has the `async` key absent." The latter is cleaner and aligns with SS-09's role as the canonical owner of `hooks.json.template` per ARCH-INDEX line 101.

**Confidence:** HIGH

### F-P1-002 [HIGH]: SS-09 architecture document not amended; still documents `"async": true` and `schema_version = 1`

**Location:** `.factory/specs/architecture/SS-09-config-activation.md` line 45 (Modules table); lines 78-82 (Public Interface schema example shows `"async": true`); lines 101-104 ("Async vs sync hook events" section reads: "PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd use `\"async\": true`. PreToolUse and PermissionRequest are sync"); lines 106-107, 152-155 (`schema_version = 1` documented as canonical).

**Observation:** ADR-019 frontmatter lists `subsystems_affected: [SS-01, SS-07, SS-09]`, and §Subsystem Assignments explicitly says SS-09 is affected. But SS-09 has no F2 amendment. A reader of SS-09 in this F2 state would believe envelope async semantics are still in effect.

**Why it matters:** SS-09 is the canonical architecture document for `hooks.json.template`. Mis-anchored or stale architecture docs break the spec-as-source-of-truth contract. Per POLICY 6, drift between BCs and architecture docs is HIGH severity.

**Suggested fix direction:** Append F2 amendment section to SS-09: rewrite Modules row, replace JSON schema example, rewrite "Async vs sync hook events" paragraph, update both `schema_version = 1` references to `schema_version = 2`. Same treatment for SS-07-hook-bash.md line 45 and 69 (also references `schema_version = 1`).

**Confidence:** HIGH

### F-P1-003 [HIGH]: DI-014 declares `REGISTRY_SCHEMA_VERSION = 1`; conflicts with v2 cutover and is not cited by either new BC (POLICY 2)

**Location:** `.factory/specs/domain-spec/invariants.md` lines 106-110; BC-1.14.001 Traceability `L2 Domain Invariants: TBD` (line 153); BC-7.06.001 Traceability `L2 Domain Invariants: TBD` (line 141).

**Observation:** DI-014 currently reads "REGISTRY_SCHEMA_VERSION = 1, INTERNAL_EVENT_SCHEMA_VERSION = 1, and schema_version = 1 in observability config must match". F2 amendments bump registry schema to 2, but DI-014 has not been amended. Neither BC-1.14.001 nor BC-7.06.001 cites DI-014 in their L2 Domain Invariants field (both list "TBD"). DI-014's own enforcement-owner line says `BC range: BC-1, BC-3` — BC-7.06.001 is not pulled into that mapping.

**Why it matters:** Per POLICY 2, every domain invariant must be cited by at least one BC's Traceability/L2 Invariants field. Both new BCs are exactly the BCs that should be carrying DI-014, and both have "TBD". Additionally, DI-014's literal text now contradicts the canonical schema version.

**Suggested fix direction:** Amend DI-014 prose to "REGISTRY_SCHEMA_VERSION = 2 (post-ADR-019)". Add DI-014 to BC-1.14.001 and BC-7.06.001 Traceability "L2 Domain Invariants" fields. Update DI-014's "BC range" enforcement-owner to include BC-7.

**Confidence:** HIGH

### F-P1-004 [HIGH]: BC-1.14.001 Error Path for `schema_version != 2` claims `E-REG-001` AND fail-open exit 0, but contract is silent on whether the dispatcher exits or hard-errors visibly

**Location:** BC-1.14.001 line 82 (Error Paths row 1: "exit code per existing dispatcher error convention (non-2, typically 0 per BC-1.08.001 fail-open). No partition attempted."); line 121 (EC-006: "Hard error at load time; ... exit 0 (fail-open per BC-1.08.001); no plugins executed; no downgrade attempted"); ADR-019 §Decision 5 line 137-138 ("hard schema-version error at startup"); BC-7.06.001 Postcondition 1 line 45 ("Dispatcher hard-errors at load time; exit per BC-1.08.001 fail-open convention"); user explicit decision: "no backwards compatibility. v2 dispatcher hard-errors on v1 registry. Single hard cut."

**Observation:** "Hard error" and "fail-open exit 0" contradict each other in plain English. If the dispatcher exits 0, Claude Code receives a "continue" signal — exactly the silent-failure mode this entire cycle exists to eliminate.

**Why it matters:** Silent fail-open on schema mismatch creates the same class of bug ADR-019 fixes. SOUL.md #4 (silent failures) is directly violated.

**Suggested fix direction:** Reconcile by either: (a) explicitly state schema-version mismatch is fail-CLOSED (exit 2 or non-0 unique error code) and amend BC-1.08.001 to except this from fail-open; OR (b) state explicitly that exit code stays 0 to maintain BC-1.08.001 compatibility, but the diagnostic must be surfaced through Claude Code via stderr capture (which is sync-envelope-only — note this works post-this-cycle). Either way, the contradiction must be resolved with explicit reasoning. Both BC-1.14.001 EC-006 and Error Paths row 1 + BC-7.06.001 Postcondition 1 must agree.

**Confidence:** HIGH

### F-P1-005 [HIGH]: BC-1.14.001 Invariant 4 says invariant is enforced at registry-load time AND CI lint, but does not reference parse-time enforcement that ADR-019 §4 mandates as defense-in-depth

**Location:** ADR-019 §Decision 4 line 128: "**Defense in depth: parse-time, pre-commit, and CI.**" — three layers; BC-1.14.001 Invariant 4 line 75: load-time + CI lint — two layers; VP-078 §Trigger Points 1 says pre-commit hook required.

**Observation:** ADR-019 promises three enforcement layers, VP-078 verifies three layers, but BC-1.14.001 Invariant 4 and BC-7.06.001 Postcondition 5/6 describe only two — neither BC requires a separate pre-commit hook. There is no BC clause requiring this pre-commit hook to exist; therefore VP-078 may have a verification target with no contract authority.

**Why it matters:** Defense-in-depth was explicit. Missing the pre-commit layer means a CI-only check is the first line of defense; a developer can land a violating registry locally and only learn at PR time.

**Suggested fix direction:** Either add a pre-commit clause to BC-7.06.001 Postcondition (e.g., "Postcondition 8: a pre-commit hook scans hooks-registry.toml for the `on_error=block ⇒ async=false` invariant and fails the commit on violation"), or amend ADR-019 / VP-078 to drop the pre-commit layer. Spec must be self-consistent: 3 layers in ADR/VP or 2 layers in BCs/VP.

**Confidence:** HIGH

### F-P1-006 [HIGH]: BC-1.14.001 Architecture Module says SS-01; BC-7.06.001 Architecture Module says SS-07 + SS-01; but BC-7.06.001's frontmatter `subsystem: "SS-07"` is single-valued — the BC contracts an SS-01 invariant inside an SS-07-anchored BC

**Location:** BC-7.06.001 frontmatter line 18 `subsystem: "SS-07"`; Postcondition 5 (line 53) — `registry.rs::validate()` is SS-01; Architecture Module field line 142 declares both SS-07 and SS-01.

**Observation:** BC-7.06.001 binds two contracts: SS-07 registry-schema shape AND SS-01 runtime check. Frontmatter `subsystem:` is single-valued and points to SS-07, understating scope. Per POLICY 6, every reference must be canonical and verbatim. A BC whose Postcondition 5 specifies behavior of `crates/factory-dispatcher/src/registry.rs` (SS-01) cannot claim sole SS-07 ownership.

**Why it matters:** Story-writer or implementer selecting BCs by subsystem from BC-INDEX (subsystem column says "SS-07" only) will not see this BC when slicing SS-01 work. The runtime enforcement belongs to the dispatcher binary, not the registry TOML. Mis-anchoring per POLICY 4.

**Suggested fix direction:** Either (a) split BC-7.06.001 into two BCs — one SS-07 (registry shape) and one SS-01 (validate() invariant); OR (b) keep one BC, document multi-subsystem ownership in body, choose primary subsystem for frontmatter that reflects the *runtime* enforcement (typically SS-01). BC-1.14.001's frontmatter (`subsystem: "SS-01"`) is fine because partition is wholly SS-01.

**Confidence:** HIGH

### F-P1-007 [MEDIUM]: VP-077 Property #2 contradicts BC-1.14.001 Postcondition 1 on default semantics for missing `async` field

**Location:** VP-077 line 47-48; BC-1.14.001 Postcondition 1 line 49.

**Observation:** Aligned in intent. However VP-077's `partition_plugins(matched, registry)` Kani harness operates on `PluginEntry { async_flag: kani::any() }` with `async_flag` typed as `bool` (line 99). Kani's `bool` is binary — no representation of "field absent". Kani proof cannot distinguish "absent" from "false" because Rust type system collapsed them via `#[serde(default)]`. VP-077 only proves false/true split; does NOT prove serde-default behavior that BC-7.06.001 Invariant 2 mandates.

**Why it matters:** VP-077 has gap in coverage of serde-default semantics.

**Suggested fix direction:** Add separate verification for serde-default — Rust unit test (covered by BC-7.06.001 EC-001) or move field-absence test to VP-078's Rust unit-test block. Update VP-077's Property Statement to scope itself to "post-parse PluginEntry struct values".

**Confidence:** MEDIUM

### F-P1-008 [MEDIUM]: BC-1.14.001 Postcondition 4 introduces `plugin.async_block_discarded` event without SS-03 catalog amendment

**Location:** BC-1.14.001 Postcondition 4 line 65; EC-005 line 120; BC-1.08.002 Postcondition 3 line 44; BC-1.14.001 Error Paths line 84 (`plugin.timeout`); EC-008 line 123 (`dispatcher.registry_invalid`); ADR-019 (does not specify event names).

**Observation:** F2 introduces ≥3 new event-type strings without SS-03 BC amendment. New events appear without contract authority.

**Why it matters:** Sink consumers may silently drop these events. Sink-fan-out invariant (VP-028) bypassed.

**Suggested fix direction:** Add SS-03 BCs for each new event type with payload schemas, or amend an existing SS-03 cross-cutting event-catalog BC.

**Confidence:** MEDIUM

### F-P1-009 [MEDIUM]: BC-1.14.001 EC-005 is logically incoherent — `block_intent` semantics undefined

**Location:** BC-1.14.001 EC-005 line 120; Invariant 4 line 75.

**Observation:** Edge case posits scenario contract just guaranteed cannot happen. If Invariant 4 is enforced at load time + CI, no async plugin has `on_error = block`. So how can dispatcher observe block_intent=true from async? Two interpretations: (a) misbehaved plugin returns exit 2 with `on_error = continue`; (b) defense-in-depth path. `block_intent` derivation is ambiguous.

**Why it matters:** Implementers will write code for impossible cases or omit them.

**Suggested fix direction:** Define `block_intent` formally — likely as `exit_code == 2 && on_error == "block"`. Then EC-005 needs reframing. Or define `block_intent` as just `exit_code == 2` and explain why dispatcher discards it for async plugins.

**Confidence:** MEDIUM

### F-P1-010 [MEDIUM]: BC-1.14.001 and BC-7.06.001 Story Anchors both "TBD"; BC-7.06.001 references three sub-stories from F1 superseded plan

**Location:** BC-1.14.001 line 106, 156; BC-7.06.001 line 93, 144; ADR-019 §6 line 142-145.

**Observation:** F2 BCs reference Story A/B/D (three stories from F1 sketch), but ADR-019 §Decision 6 says "Single story delivery" per user decision. Stale reference inside BC body fields.

**Why it matters:** Implementer reading BC-7.06.001 faces contradiction.

**Suggested fix direction:** Replace BC-7.06.001 line 93 and 144 references to "Story A + B + D" with "TBD — single story per ADR-019 §6". Once story is created, both fields resolve to actual story ID.

**Confidence:** MEDIUM

### F-P1-011 [MEDIUM]: BC-1.14.001 Invariant 5 forbids "downgrade attempt" but Error Paths/EC-006 say `exit 0`; observationally indistinguishable from clean run

**Location:** BC-1.14.001 Invariant 5 line 76; Error Paths line 82 (exit 0); EC-006 line 121.

**Observation:** "Deterministic error" + "exit 0" not contradictory in pure logic, but observationally identical to clean run with empty matched plugins. From Claude Code's perspective, both produce exit 0, no stderr, no stdout. Same silent-failure class ADR-019 was designed to fix.

**Why it matters:** v1→v2 misconfiguration is exactly the rollout risk user accepted. If misconfiguration silent, user has no signal.

**Suggested fix direction:** See F-P1-004 — same fix.

**Confidence:** MEDIUM

### F-P1-012 [MEDIUM]: VP-078 bats test references fictional `factory-dispatcher --dry-validate` flag

**Location:** VP-078 line 146.

**Observation:** No specification authorizes `--dry-validate` CLI flag. No BC mandates it. Test depends on undeclared CLI surface.

**Why it matters:** VP harness with undeclared CLI surface = dead verification.

**Suggested fix direction:** Either (a) add a BC in SS-01 declaring the `--dry-validate` flag; or (b) rewrite harness to use natural startup path with `RUST_LOG=error`.

**Confidence:** MEDIUM

### F-P1-013 [MEDIUM]: BC-1.14.001 Postcondition 4 says "tokio task or equivalent" — implicit runtime assumption with no architectural anchor; telemetry preservation unclear

**Location:** BC-1.14.001 Postcondition 4 line 64; ADR-019 Decision 3 line 114.

**Observation:** Spec assumes tokio. Implications not addressed: (a) what happens if dispatcher exits before async tasks complete? (b) "fire-and-forget" implies no await; tokio tasks dropped at process exit. Telemetry plugins like `capture-commit-activity` may not finish writing to events-*.jsonl before dispatcher exits.

**Why it matters:** Telemetry plugins were classified async to "preserve current latency profile". But if their writes don't complete because dispatcher exits, telemetry is *worse* than before.

**Suggested fix direction:** Add postcondition clarifying async plugin lifetime: e.g., "Async plugins are spawned as tokio tasks; the dispatcher does NOT await them and exits as soon as sync_group completes; async plugin completion is best-effort and may be truncated by process exit." Then document this consequence in ADR-019 §Consequences.

**Confidence:** MEDIUM

### F-P1-014 [LOW]: BC-1.14.001 H1 title is 269 chars long — exceeds reasonable readability

**Location:** BC-1.14.001 line 32; BC-7.06.001 line 31.

**Observation:** Both new BCs encode multi-clause behavior in title. Compare to BC-1.13.001 ("Dispatcher MUST load `resolvers-registry.toml`...") which is shorter.

**Why it matters:** Markdown table rendering may truncate. Style drift.

**Suggested fix direction:** Optional refactor: shorten H1 to single declarative clause, move clarifications into Description.

**Confidence:** LOW

### F-P1-015 [LOW]: BC-1.14.001 EC-009 silent on async plugin spawn ordering relative to sync_group

**Location:** BC-1.14.001 EC-009 line 124; ADR-019 Decision 3 line 110-114.

**Observation:** ADR-019 pseudocode shows async plugins spawned AFTER sync_group completes. BC silent on ordering.

**Why it matters:** Story-writer might optimize by spawning async first concurrently with sync — diverges from contract.

**Suggested fix direction:** Add to Postcondition 4 or new Postcondition 6: "Async group plugins are spawned only after sync_group execution completes."

**Confidence:** LOW

### F-P1-016 [LOW]: BC-7.06.001 Postcondition 7 lists 4 telemetry plugins but lacks invariant-level coverage; no positive VP

**Location:** BC-7.06.001 Postcondition 7 line 57.

**Observation:** Postcondition is more like static-shape requirement (registry MUST have classifications), better as Invariant. No VP verifies "these specific plugins have async=true". VP-078 verifies inverse but not positive list.

**Why it matters:** Future engineer flipping `capture-commit-activity` to `async = false` not caught by VP-078.

**Suggested fix direction:** Add verification to VP-078 covering positive classification, or separate VP, or note as deliberate non-coverage.

**Confidence:** LOW

### F-P1-017 [LOW]: VP-077 Harness 1 `Vec<PluginEntry>` with `kani::any()` doesn't constrain unique IDs

**Location:** VP-077 lines 96-103, 110.

**Observation:** Disjointness check at line 110 via `!async_g.contains(p)` works iff plugin equality is well-defined. Kani's `kani::any()` over `PluginEntry` doesn't constrain uniqueness. May produce false negatives or vacuously satisfy.

**Why it matters:** Real registry can't have duplicate names; Kani doesn't know that.

**Suggested fix direction:** Add `kani::assume` constraints for unique plugin names, or operate on indices. Note in Feasibility Assessment that uniqueness is precondition.

**Confidence:** LOW

### F-P1-018 [NIT]: ADR-019 §Implementation Pointers uses `BC-1.NN.001` and `BC-7.NN.001` placeholders

**Location:** ADR-019 line 226, 228.

**Observation:** Stale placeholders post-burst.

**Suggested fix direction:** Replace with `BC-1.14.001` and `BC-7.06.001`.

**Confidence:** HIGH (not problem, stale text)

### F-P1-019 [NIT]: VP-077 + VP-078 frontmatter `bcs:` array uses tuple-style; surrounding VPs use ID-only syntax

**Location:** VP-077 line 30; VP-078 line 30.

**Observation:** Other VPs (e.g., VP-001) use `bcs: [BC-1.03.007, BC-1.03.008]`. New VPs use single-item list with descriptive parenthetical. Style drift.

**Suggested fix direction:** Normalize to `bcs: [BC-1.14.001]` and `bcs: [BC-7.06.001]`.

**Confidence:** HIGH (style polish)

## 4. Policy compliance summary

| Policy | Status | Note |
|---|---|---|
| 1. append_only_numbering | PASS | New BCs/VPs/ADR are next-available |
| 2. lift_invariants_to_bcs | **FAIL** | BC-1.14.001 + BC-7.06.001 L2 Domain Invariants TBD; DI-014 natural anchor. See F-P1-003. |
| 3. state_manager_runs_last | N/A | Pertains to story execution. |
| 4. semantic_anchoring_integrity | **FAIL** | BC-7.06.001 mis-anchors subsystem. See F-P1-006. |
| 5. creators_justify_anchors | PASS | Capability anchor justifications present. |
| 6. architecture_is_subsystem_name_source_of_truth | **FAIL** | SS-09 not amended; SS-07 still says schema_version=1. See F-P1-002. |
| 7. bc_h1_is_title_source_of_truth | PASS | All H1s match BC-INDEX. |
| 8. bc_array_changes_propagate_to_body_and_acs | N/A | No story authored yet. |
| 9. vp_index_is_vp_catalog_source_of_truth | PASS-with-caveat | VP-INDEX correctly counts 78. |
| 10. demo_evidence_story_scoped | N/A | No demo evidence. |
| 11. no_test_tautologies | PASS-with-caveat | Skeletons clean; F4 re-check. |
| 12. bc_tv_emitter_consistency | N/A | No emitter struct excluded fields. |

**Net: 3 FAIL (POLICY 2, 4, 6), 5 PASS, 4 N/A.**

## 5. Open questions

- **OQ-A:** Schema-mismatch fail-mode — fail-open exit 0 or fail-closed? See F-P1-004, F-P1-011.
- **OQ-B:** Async plugin lifetime — does dispatcher wait for spawn-then-exit telemetry? See F-P1-013.
- **OQ-C:** F1 OQ-2 (concurrency cap) — ADR-019 doesn't resolve or defer.
- **OQ-D:** `block_intent` definition. See F-P1-009.
- **OQ-E:** Pre-commit lint enforcement — promised in ADR/VP, missing from BC. See F-P1-005.

## 6. Top 3 findings (ranked)

1. **F-P1-001 [HIGH]** — PostToolUse / Stop / SubagentStop envelope flip has no governing BC. Cycle ships incomplete with respect to its own ADR §Decision 1 if these aren't covered. **Originating cycle bug fails to be fixed.**
2. **F-P1-002 [HIGH]** — SS-09 architecture document not amended; still describes async-true-PostToolUse and `schema_version = 1` as canonical.
3. **F-P1-004 [HIGH]** — Schema-mismatch error path says both "hard error" AND "exit 0 fail-open"; reproduces silent-failure pattern this cycle exists to eliminate.
