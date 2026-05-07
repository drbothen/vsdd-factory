---
document_type: adversary-pass
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F2
pass: 2
producer: adversary
producer_model: claude-opus-4-7[1m]
fresh_context: true
verdict: SUBSTANTIVE
finding_count: { high: 6, medium: 7, low: 4, nit: 2 }
adr-013_clock_action: reset
clock: 0_of_3
timestamp: 2026-05-07T00:00:00Z
---

# Adversary Pass-2 Findings — F2 spec package, v1.0-feature-plugin-async-semantics-pass-1

## 1. Verdict

**SUBSTANTIVE**. Multiple high-severity issues that would mislead implementers or break the verification harness. Fresh-context surfaced new findings invisible to prior passes — particularly the VP-078 harness renumbering ripple effect, the BC-7.06.001 "Invariant 1" cross-reference that names the wrong invariant, and a body-contradicts-postcondition pair in BC-4.07.003.

Clock RESETS to 0/3.

## 2. Finding count summary

| Severity | Count |
|---|---|
| HIGH | 6 |
| MEDIUM | 7 |
| LOW | 4 |
| NIT | 2 |

## 3. Findings

### F-P2-001 [HIGH] — VP-077 Harness 1 cites a non-existent BC-7.06.001 invariant for plugin-name uniqueness

**Evidence:** VP-077 lines 64-65 cite "BC-7.06.001 Invariant 1" for plugin-name uniqueness. BC-7.06.001 Invariant 1 (line 60-61) is the `on_error = "block"` ⇒ `async = false` rule — NOT a uniqueness invariant. Actual `hooks-registry.toml` has duplicate names by design: `worktree-hooks` ×2 (WorktreeCreate + WorktreeRemove), `protect-secrets` ×2 (Bash + Read).

**Why it matters:** Kani harness Precondition is built on a citation to a non-existent invariant; the empirical claim that registry-load enforces uniqueness is false; production code provides no such guarantee.

**Required action:** Either author a name-uniqueness invariant in BC-7.06.001 (scoped to (event_name, plugin_name) tuples to allow legitimate multi-event entries) and verify it really fires at parse time, or drop the uniqueness assumption from Harness 1.

### F-P2-002 [HIGH] — VP-077 forward-references "VP-078 Harness 3" but VP-078 v1.3 renumbered serde-default to Harness 4

**Evidence:** VP-077 lines 50, 215 reference "VP-078 Harness 3" for serde-default semantics. VP-078 v1.3 lines 233-285 — Harness 3 is now positive telemetry classification. Lines 290-385 — Harness 4 is the renumbered serde-default tests. Lines 455-460 — VP-078 amendment explicitly notes the renumber.

**Why it matters:** Sibling pointer not propagated after VP-078 renumber. Reader following VP-077 lands on positive-classification harness, not serde-default tests.

**Required action:** Update VP-077 lines 50, 215 to "VP-078 Harness 4".

### F-P2-003 [HIGH] — BC-4.04.004, BC-4.05.004 reference stale "BC-7.06.001 Postcondition 7"

**Evidence:** BC-4.04.004 line 45 + BC-4.05.004 line 45 both cite "BC-7.06.001 Postcondition 7" for telemetry classification. BC-7.06.001 lines 56-57 — Postcondition 7 is now the pre-commit lint layer. Line 75 — Invariant 6 holds the classification list. Lines 182-183 — amendment explicitly notes the promotion.

**Why it matters:** Sibling BCs cite a clause ID whose content moved during the fix burst. Reader looking up "BC-7.06.001 Postcondition 7" finds pre-commit lint, not classification.

**Required action:** Update BC-4.04.004 line 45 + BC-4.05.004 line 45 to "BC-7.06.001 **Invariant 6**".

### F-P2-004 [HIGH] — BC-4.07.003 Description body contradicts Postcondition 5

**Evidence:** BC-4.07.003 line 34 (Description): "Both entries have `async: true` and `timeout: 10000`." Line 47 (Postcondition 5): "The `async` key is ABSENT from both `WorktreeCreate` and `WorktreeRemove` hook entries (per ADR-019)." Lines 122, 128-134 explicitly remove `async: true` per amendment.

**Why it matters:** Description is the first paragraph an implementer reads. Direct contradiction within a single BC. POLICY 7 indirectly violated.

**Required action:** Edit Description to "Both entries have `timeout: 10000` (the `async` key is absent per ADR-019)" — strike "async: true and".

### F-P2-005 [HIGH] — BC-1.14.001 Postcondition numbering broken (6 appears before 5)

**Evidence:** BC-1.14.001 lines 46-73 — Postconditions enumerated as 1, 2, 3, 4, **6**, **5**. Line 71 contains an apologetic note acknowledging the gap.

**Why it matters:** Insertion artifact during fix burst. Implementers tracing "PC5" or "PC6" land on wrong content. Adjacent BCs (BC-1.08.002) reference postcondition numbers and rely on canonical sequence.

**Required action:** Renumber Postconditions to monotonic 1-2-3-4-5-6 sequence.

### F-P2-006 [HIGH] — BC-7.06.001 Invariant 6 enumerates 6 plugins; registry has more telemetry-irrelevant plugins not classified

**Evidence:** BC-7.06.001 lines 74-75 — Invariant 6 names exactly 6 plugins. `hooks-registry.toml` lines 758-765, 889-895, 846-863, 131-142 contain `track-agent-start` (PreToolUse), `track-agent-stop` (SubagentStop), `session-learning` (Stop), `warn-pending-wave-gate` (Stop), `regression-gate` (PostToolUse) — all `on_error="continue"`, telemetry-class but NOT in Invariant 6.

**Why it matters:** ADR-019 Consequences estimates 30-100ms peak latency assuming only block-relevant plugins remain sync. With ≥5 unclassified continue plugins forced sync, latency budget shifts. Invariant 6 is presented as a regression ratchet but is incomplete from day 1.

**Required action:** Either expand Invariant 6 to enumerate ALL telemetry-only plugins requiring async classification, OR explicitly document that Invariant 6 is a partial mandatory list and the dispatcher's sync-group includes additional `on_error=continue` plugins as a deliberate design.

### F-P2-007 [MEDIUM] — VP-078 Harness 1 + VP-079 use `factory-dispatcher --registry <path> --event <type>` CLI flags that may not exist

**Evidence:** VP-078 line 158, VP-079 lines 148-150, 191-193, 242-244, 293-295. Standard dispatcher reads from CLAUDE_PLUGIN_ROOT-relative path; no documented `--registry` or `--event` CLI flag in dispatcher's public API per ARCH-INDEX.

**Why it matters:** Harness assumes CLI surface that production binary may not expose; bats test un-instantiable as written.

**Required action:** Either add CLI surface contracting to BC-1.14.001 / BC-7.06.001 (documenting flags as part of dispatcher API) or rewrite harnesses to use actual invocation surface (env vars + stdin envelope).

### F-P2-008 [MEDIUM] — VP-079 fixtures use `script = "exit 2"` — invalid registry shape

**Evidence:** VP-079 lines 135-145 fixture uses top-level `script = "exit 2"`. Real schema (hooks-registry.toml lines 90-100, SS-07 line 71-83) uses `plugin = "hook-plugins/<name>.wasm"` + nested `[hooks.config] script_path = "..."`.

**Why it matters:** Fixtures won't parse against real schema. Implementer hits RegistryError::ParseError before testing intended behavior.

**Required action:** Replace with `plugin = "hook-plugins/legacy-bash-adapter.wasm"` + `[hooks.config] script_path = "test-fixtures/exit2.sh"`, or add doc-comment that "script" is placeholder pseudocode.

### F-P2-009 [MEDIUM] — BC-1.14.001 Error Paths omits async-plugin-exit-2 case that EC-005 documents

**Evidence:** BC-1.14.001 lines 85-93 (Error Paths table) — no row for "Async plugin returns exit code 2". Line 125 (EC-005) — documents exact case with `plugin.async_block_discarded` emission.

**Why it matters:** Error Paths and Edge Cases overlap by convention. Important behavior class (async exit-2 → discard event emission) appears in EC but not Error Paths. Reader of Error Paths only misses diagnostic emission obligation.

**Required action:** Add row to BC-1.14.001 Error Paths: "Async plugin returns exit code 2 | `plugin.async_block_discarded` event logged with reason; dispatcher exit code unaffected."

### F-P2-010 [MEDIUM] — BC-3.08.001 Architecture Module misclassifies registry.rs as SS-07

**Evidence:** BC-3.08.001 line 197: "SS-07 — `crates/factory-dispatcher/src/registry.rs` (schema_mismatch + registry_invalid emission sites)". ARCH-INDEX line 95 — SS-01 owns `crates/factory-dispatcher/src/{main,registry,routing,executor,invoke,engine,plugin_loader,payload}.rs`. Line 101 — SS-07 owns `plugins/vsdd-factory/hooks/*.sh` + `hooks-registry.toml`.

**Why it matters:** POLICY 6 (architecture is subsystem name source of truth) HIGH severity violation. ARCH-INDEX is canonical. registry.rs lives in SS-01.

**Required action:** Change BC-3.08.001 Architecture Module to "SS-01 — `crates/factory-dispatcher/src/registry.rs`" (drop SS-07 qualifier).

### F-P2-011 [MEDIUM] — VP-077 Property Statement lists 6 properties; VP-INDEX title enumerates 4

**Evidence:** VP-077 lines 39-59 enumerates 6 (totality, async-field respect, disjointness, union completeness, exit-code independence, aggregation correctness). VP-INDEX line 140 title: 4 properties only. ADR-019 lines 235-237: also 4 properties.

**Why it matters:** POLICY 9 (VP-INDEX is VP catalog source of truth) HIGH. Three places describe same VP with different property sets.

**Required action:** Pick one source. Expand VP-INDEX/ADR-019 title to match VP-077 (6 properties) or trim VP-077 to match VP-INDEX (4 properties; Harnesses 2+4 out-of-scope).

### F-P2-012 [MEDIUM] — BC-1.14.001 Precondition 2 includes `PermissionRequest` but ADR-019 §Decision 1 omits it

**Evidence:** BC-1.14.001 line 41 — 10 events including PermissionRequest. ADR-019 lines 74-78 — 9 events, no PermissionRequest. BC-9.01.006 line 46 — 10 events.

**Why it matters:** ADR-019 is architectural source of truth. Either ADR forgot PermissionRequest (already sync pre-ADR per line 38) or BCs are over-enumerating.

**Required action:** Update ADR-019 §Decision 1 to enumerate PermissionRequest as no-op clarification, OR remove from BC-1.14.001 Precondition 2 + BC-9.01.006 Postcondition 2.

### F-P2-013 [MEDIUM] — BC-7.06.001 Postcondition 3 framing contradicts "no backwards compat" decision

**Evidence:** BC-7.06.001 line 49: "All existing plugin entries that do not declare `async` are treated as `async = false`. **This preserves the behavior of every validator and governance plugin** ..." ADR-019 §Decision 5 (lines 131-135): hard schema v2 cut, no compat. Line 51 (Postcondition 4): also says no backward compat.

**Why it matters:** Postcondition 3 frames absent-async-defaults-to-false as preserving v1 plugin behavior, suggesting graceful upgrade. ADR-019 prohibits compat shim. Confusing tension.

**Required action:** Reword Postcondition 3 to remove "preserves the behavior of every validator" framing — replace with "Within a `schema_version = 2` registry, entries that do not declare `async` are parsed as `async = false`. Operators migrating from v1 must bump the schema_version header."

### F-P2-014 [LOW] — DI-014 BC range "BC-1, BC-3, BC-7" misleading post-reanchor

**Evidence:** invariants.md line 108 BC range cites BC-7. BC-7.06.001 line 18 — `subsystem: "SS-01"` (reanchored per F-P1-006). BC-INDEX line 1785 — comment notes reanchor.

**Why it matters:** DI-014 cites BC-7 as enforcement arm but actual BC-7.06.001 is now SS-01 by frontmatter. Reader confusion plausible.

**Required action:** Reword to "BC-1, BC-3, BC-7 (BC-7.06.001 ID retained per POLICY 1; authoritative subsystem is SS-01)" or drop BC-7.

### F-P2-015 [LOW] — BC-1.14.001 Precondition 4 cites load-time invariant without explicit BC pin

**Evidence:** BC-1.14.001 line 43 — "Registry validation has already confirmed that no entry has both `on_error = block` and `async = true`". No explicit "(per BC-7.06.001 Invariant 1)" pin.

**Why it matters:** Precondition is satisfied externally by BC-7.06.001 Invariant 1; explicit pin makes dependency clear.

**Required action:** Append "(per BC-7.06.001 Invariant 1)" to BC-1.14.001 Precondition 4.

### F-P2-016 [LOW] — VP-079 trace_id assertion may emit false positives per its own DI-017 note

**Evidence:** VP-079 lines 48-49 (Property #4: "non-null trace_id"). Line 323 (False-positive scenarios: `trace_id: null` flagged as DI-017 violation). BC-3.08.001 lines 135-136 (Error Paths: trace_id may be null as last-resort fallback).

**Why it matters:** VP punishes behavior BC permits. Harness fails on corner case BC says is OK.

**Required action:** Either tighten BC-3.08.001 to forbid `trace_id: null`, or relax VP-079 Property 4 to permit DI-017 fallback case.

### F-P2-017 [LOW] — BC-1.08.001 Stories field unattached to new cycle's story

**Evidence:** BC-1.08.001 line 76 — Stories: "S-2.07 (Wave 9 SS-01 straggler re-anchor)". BC-1.14.001 + BC-7.06.001 both have "TBD — single story per ADR-019 §6". BC-1.08.001 amended this cycle but Stories field unchanged.

**Why it matters:** New cycle's story will exercise Invariant 2 (fail-closed exception). POLICY 1 append-only — Stories field should be appended.

**Required action:** Append: "TBD — single story per ADR-019 §6".

### F-P2-018 [NIT] — VP-077 Lifecycle/Traceability "Subsystem: SS-01" duplicates frontmatter `scope: SS-01`

**Evidence:** VP-077 line 230. Cosmetic.

### F-P2-019 [NIT] — BCs use mixed `events-*.jsonl` and `events-YYYY-MM-DD.jsonl` filename forms

**Evidence:** BC-1.14.001 line 65 + BC-3.08.001 lines 35, 121 use glob form; DI-008 invariants.md line 70 specifies date-stamped form. Glob OK in prose; inconsistent with canonical pattern.

## 4. Policy compliance summary

- **POLICY 1 (append_only_numbering):** observed; BC-7.06.001 ID retained after subsystem change. Compliant.
- **POLICY 2 (lift_invariants_to_bcs):** DI-014 cites BC-1.14.001 + BC-7.06.001. Compliant.
- **POLICY 4 (semantic_anchoring_integrity):** **VIOLATED** by F-P2-001 (VP-077 cites wrong invariant) and F-P2-010 (BC-3.08.001 misclassifies registry.rs).
- **POLICY 6 (architecture_is_subsystem_name_source_of_truth):** **VIOLATED** by F-P2-010.
- **POLICY 7 (bc_h1_is_title_source_of_truth):** **VIOLATED** by F-P2-004 (BC-4.07.003 body contradicts H1 + postconditions).
- **POLICY 9 (vp_index_is_vp_catalog_source_of_truth):** drift via F-P2-002 and F-P2-011.
- **S-7.01 partial-fix regression discipline:** **VIOLATED** by F-P2-002, F-P2-003 (sibling pointers not propagated).

## 5. Open questions

- **OQ-P2-001:** Should VP-077 Kani harness's plugin-name-uniqueness `assume` be replaced with handling for multi-event-same-name entries, or should BC-7.06.001 add (event_name, plugin_name)-tuple uniqueness invariant?
- **OQ-P2-002:** Are `track-agent-start`, `track-agent-stop`, `session-learning`, `warn-pending-wave-gate`, `regression-gate` deliberately sync (accepting latency) or accidentally not in BC-7.06.001 Invariant 6?
- **OQ-P2-003:** Does the dispatcher binary actually expose `--registry` and `--event` CLI flags?

## 6. Top 3 findings (SUBSTANTIVE)

1. **F-P2-001 (HIGH)** — VP-077 Kani harness cites non-existent BC-7.06.001 Invariant 1 for plugin-name uniqueness; real registry has duplicate names by design.
2. **F-P2-002 (HIGH)** — VP-077 references "VP-078 Harness 3" for serde-default; VP-078 v1.3 renumbered serde-default to Harness 4. Cross-doc drift.
3. **F-P2-004 (HIGH)** — BC-4.07.003 Description still says "async: true" while postconditions/invariants prohibit the key — body-postcondition contradiction post-amendment.
