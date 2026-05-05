# Adversarial Review — E-9 v1.7 Amendment (D-236) — Pass 3

**Date:** 2026-05-05
**Commit reviewed:** d9f2c86
**Files reviewed:** 4 (E-9 epic, gap-analysis-w16-subprocess.md, perf-baseline-w16.md, audit-w16.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 reset
**Pass methodology angle:** Forward-simulation + counter-example construction. Imagined story-writer reading the amendment to author S-9.01..S-9.07 ACs, then constructed a plugin satisfying E-9 v1.7's literal MUSTs while violating ADR-015 D-15.3's actual contract. Also walked the bidirectional reachability of each ADR-015 cross-reference (frontmatter `inputs:` / `references:` / body) to find propagation gaps.

## Summary

The amendment is largely correct and additive, but two interpretive ambiguities and one frontmatter-propagation gap are substantive enough to mislead story-writer. The "block-mode hooks MUST emit `outcome = blocked`" instruction conflicts with ADR-015 D-15.3, which assigns block-event emission to the **dispatcher**, not the plugin. The gap-analysis amendment text simultaneously prescribes `vsdd.host.exec_subprocess.completed.v1` as MUST while admitting the `vsdd.host.*` prefix is unregistered (would fall to `event.category = "unknown"`). The perf-baseline frontmatter `references:` list omits ADR-015 despite a 17-line ADR-015 body section being added.

## Findings (severity-ordered)

### HIGH

**H-1 [HIGH] (semantic anchoring / counter-example) — E-9 v1.7 line 295 misattributes block-event emission to plugin; conflicts with ADR-015 D-15.3 actual contract.**

- File: `/Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-9-tier-2-native-wasm-migration.md` line 294-296.
- E-9 v1.7 amendment text: "Block-mode hooks MUST emit `outcome = "blocked"` on block path (D-15.3 block path audit trail)."
- ADR-015 D-15.3 actual contract (lines 374-377): "When a plugin returns `HookResult::Block`, the dispatcher emits a `vsdd.block.plugin_blocked.v1` event with `outcome=blocked`, `plugin.name`, and `hook.tool_name` before exiting." Block-event emission is **dispatcher-side**, automatic from `HookResult::Block`.
- Counter-example: A conformant ADR-015 plugin that returns `HookResult::Block` and emits NOTHING from plugin code FULLY satisfies ADR-015 D-15.3. But under E-9 v1.7's literal MUST language, that plugin appears non-conformant. Story-writer reading this amendment will likely author redundant plugin-side emission ACs that produce duplicate audit-trail events.
- POLICY 4 (semantic_anchoring_integrity) violation.
- Same defect propagated to `audit-w16.md` line 35, line 37, and line 47-48. Blast radius: 2 files, 4 sites.
- Suggested fix: Either (a) reword the MUST to clarify the plugin emits an additional domain-event signaling its own block decision (separate from the dispatcher's `vsdd.block.plugin_blocked.v1`), or (b) drop the plugin-side MUST entirely — dispatcher's automatic emission satisfies D-15.3.

### MED

**M-1 [MED] (semantic anchoring) — gap-analysis amendment internally contradicts itself on `vsdd.host.exec_subprocess.completed.v1` registry status.**

- File: `gap-analysis-w16-subprocess.md` lines 320-324.
- The amendment text says: "The event MUST use `event.name = "vsdd.host.exec_subprocess.completed.v1"`" then immediately notes "a `vsdd.host.*` prefix would need a registry entry — story-writer or SS-01 implementer must confirm the canonical prefix for this event family with the dispatcher team."
- Verified ADR-015 registry table (lines 318-332): `vsdd.host.*` is NOT present. So the proposed event lands in `event.category = "unknown"` per D-15.2.b unrecognized-prefix default.
- Wave 3 acceptance criterion 2 in ADR-015 (lines 633-638) installs an `unknown_category_events` alert that would actively fire on this event.
- Suggested fix: Reword to "the proposed `event.name` is `vsdd.host.exec_subprocess.completed.v1` pending a registry-prefix decision from the dispatcher team; if `vsdd.host.*` is not added to the registry before E-10 Wave 1 ships, the event MUST use a registered prefix (e.g., `vsdd.dispatcher.subprocess_completed.v1` to inherit `lifecycle` category)."

**M-2 [MED] (semantic anchoring) — `internal.capability_denied` rename path is unresolved between two valid options.**

- File: `gap-analysis-w16-subprocess.md` lines 333-340.
- Verified ADR-015 line 329: `vsdd.capability.denied.*` IS in the registry (mapped to `audit`).
- The current name `internal.capability_denied` is doubly broken: (a) missing `vsdd.` namespace; (b) using `internal.*` rather than `capability.denied.*`. Two valid renames exist (`vsdd.capability.denied.exec_subprocess.v1` for audit category, or `vsdd.internal.capability_denied.v1` for lifecycle category) — the amendment doesn't pick one.
- Substantive gap: gap-analysis is the canonical pre-Tier-2 reference; leaving event.name unresolved on a dimension (audit vs lifecycle category) materially changes dispatcher dashboards.
- Suggested fix: Pick one (recommend `vsdd.capability.denied.exec_subprocess.v1` because semantics are denial = audit-category) or explicitly frame as a binary choice with rationale.

**M-3 [MED] (cross-doc reachability / frontmatter-body coherence) — `perf-baseline-w16.md` body adds 17-line ADR-015 section but frontmatter `references:` does not include ADR-015.**

- File: `perf-baseline-w16.md` lines 12-16 (frontmatter `references:`) versus lines 342-358 (body ADR-015 section).
- Frontmatter `references:` lists ADR-014 R-8.09, E-8 R-8.08, S-8.00 PR #47, ADR-013. ADR-015 absent.
- Propagation defect: a hook-validator scanning frontmatter would not detect ADR-015 as input.
- E-9 epic frontmatter (line 23) DID propagate ADR-015 to its `inputs:`. Inconsistent across the 4 files.
- Suggested fix: Append `- ADR-015 (single-stream OTel emit contract; emit-overhead N/A for this baseline)` to `references:` list.

### LOW

**L-1 [LOW] (semantic anchoring / wording precision) — E-9 v1.7 line 297-299 imprecise about what inherits VSDD_TRACE_ID/PARENT_SPAN_ID.**

- File: E-9 epic lines 297-299.
- ADR-015 D-15.4: dispatcher injects env vars into the **subprocess** environment. The plugin (WASM) does not inherit them — the bash subprocess (`verify-sha-currency.sh`) does.
- Suggested fix: "S-9.07's verify-sha-currency.sh subprocess inherits these env vars automatically — no plugin manifest change needed."

**L-2 [LOW] (cross-doc reachability / convention) — None of the 3 architecture docs carry a `last_amended:` frontmatter field.**

- Files: `audit-w16.md`, `gap-analysis-w16-subprocess.md`, `perf-baseline-w16.md`.
- E-9 epic added `last_amended: 2026-05-05` (line 20). The 3 architecture docs were also amended in d9f2c86 but their frontmatters retain only `timestamp:` (creation date).
- (pending intent verification) — in-place amendments may be by-design unmarked. (Note: D-239 lessons.md codified annotate-in-place vs version-bump as INTENT for arch docs; this LOW may be invalid given that lessons codification.)

## Out-of-scope-but-noted

- OOS-1: ADR-015 awareness block placed inside D-9.2 (a WITHDRAWN heading). Logically epic-wide; consider promoting to its own H3 in a future amendment.
- OOS-2: `audit-w16.md` and `gap-analysis-w16-subprocess.md` retain `status: draft` despite feeding accepted ADRs.

## Process-gaps

- **PG-1 [process-gap]:** No automated validator checks frontmatter `references:` propagation when a body section adds a new ADR cross-reference. M-3 was only catchable by manual diff review. Codify as a hook: when a doc body gains an `ADR-NNN` reference, the same commit must update frontmatter `references:`/`inputs:` or carry an explicit waiver.
- **PG-2 [process-gap]:** No template for in-place amendment markers across architecture docs. Codify whether amendments should bump `version:`, add `last_amended:`, or change `timestamp:` (D-239 lessons.md addressed this for the annotate-in-place case but the convention should be templated).

## Convention checks

- No "Lines: X → Y" footer in v1.7 section: PASS (lines 661-683).
- No "Lines: X → Y" footer in v1.6 section: PASS (lines 650-660).
- Summary table v1.7 row present: PASS (line 466).
- Summary table v1.8 preemptive reserved row: PASS (line 467).
- Changelog summary table v1.7 row content: PASS.
- H3 version heading count vs summary table row count: PASS (9 rows − v1.0 initial − v1.8 reserved = 7 expected; 7 H3 found).
- POLICY 1 (append_only_numbering): PASS.
- POLICY 6 (architecture_is_subsystem_name_source_of_truth): PASS.
- POLICY 9 (vp_index_is_vp_catalog_source_of_truth): PASS (zero VP changes).
- D-236 scope discipline (no new BCs/VPs/FRs): PASS.

## Anomaly verification

- (a) `internal.capability_denied` lacks `vsdd.` prefix flagged in amendment? PASS — gap-analysis lines 333-340. (See M-2 for residual unresolved fix path.)
- (b) `host.exec_subprocess.completed` lacks `vsdd.host.*` registry entry flagged in amendment? PASS — gap-analysis lines 320-324. (See M-1 for residual MUST-vs-pending contradiction.)

## Coverage / counter-example / boundary checks

**Counter-example construction (against H-1):** A plugin that returns `HookResult::Block` and emits zero events from its own code FULLY satisfies ADR-015 D-15.3 (dispatcher emits `vsdd.block.plugin_blocked.v1` automatically). But under E-9 v1.7's literal MUST language, that plugin appears non-conformant. The H-1 contradiction is not theoretical.

**Bidirectional reachability checks:**
- E-9 frontmatter `inputs:` includes ADR-015 + gap-analysis. ADR-015 itself does not back-reference E-9 (acceptable; one-way OK).
- gap-analysis body cites ADR-015 D-15.1, D-15.2, D-15.3, D-15.4 (lines 313-348). Reverse: ADR-015 makes no backreference (acceptable).
- audit-w16.md body cites ADR-015 D-15.1, D-15.2, D-15.3, D-15.4 (lines 21-60). Same.
- perf-baseline-w16.md body cites ADR-015 D-15.1 (lines 342-358). Frontmatter `references:` MISSING ADR-015 — see M-3.

**Boundary cases (what the amendment does NOT say):**
- Schema versioning of validate-* events: amendment prescribes `.v1` suffix but doesn't say what triggers a `.v2` bump. **Soft gap — silence acceptable for v1.7; flag for E-10 Wave 2 spec.**
- `event.schema_url` per-event URI: not addressed; silence acceptable per ADR-015 D-15.2.d.
- Retention policy: not echoed; orthogonal to W-16.
- `event.host_overrides` attribute: not addressed; host stamps it; plugins don't assert it.
