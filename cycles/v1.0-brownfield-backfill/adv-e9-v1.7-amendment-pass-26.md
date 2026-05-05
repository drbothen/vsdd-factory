# Adversarial Review — E-9 v1.25 Source-Truth Burst (D-268) — Pass 26

**Date:** 2026-05-05
**Commit reviewed:** bae2801 (v1.24 → v1.25)
**Cumulative surface:** v1.7..v1.25 (4 files + open-questions.md + 2 BCs)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 0_of_3 → 1_of_3 advance
**Pass methodology angle:** Workflow-level coverage audit — simulate a CI/CD build engineer translating amendment-surface MUSTs/postconditions into pre-merge gates. Identify test-impossible claims, missing fixtures, and contracts that would slip through CI without explicit verification scaffolding. NEW per TD-VSDD-057.

## Summary

Pass-26 walked the v1.7..v1.25 amendment surface from a CI engineer's perspective: for each normative MUST/postcondition, can a deterministic pre-merge gate be authored that would catch a regression? 22 contracts have a clear path-to-test. v1.25 source-truth corrections (denial enumeration, line 252 spawn cite, EC-003 enumeration) verified PASS against `exec_subprocess.rs:147-171` and `crates/factory-dispatcher/src/host/mod.rs:120-137`. All 7 amendment-touched files carry `last_amended: 2026-05-05`. v1.7..v1.25 changelog summary rows intact; v1.26 reserved row present. No fix-burst-internal IDs leak into permanent specs body. Three LOW observations describe workflow-affordance gaps that do not block dispatch.

## Findings

### HIGH
None.

### MED
None.

### LOW

**L-P26-001 [LOW] [process-gap]: TD-VSDD-078 codifies BC enumeration source-grep discipline but no automated validator script ships.**
- BC-1.05.036:52 + :85 hard-code denial-reason literals citing exec_subprocess.rs:148/155/162/169.
- TD-VSDD-075 + TD-VSDD-078 codify discipline as fix-burst MUST but provide no executable enforcement.
- Test-impossible-without-scaffolding: CI engineer would write `grep -nE 'emit_denial\(ctx, cmd, "([^"]+)"' crates/.../exec_subprocess.rs | sort -u` and diff against BC enumeration. That validator does not exist.
- Severity LOW because (a) discipline codified in lessons.md, (b) v1.25 fix presently correct. Risk is future-burst regression. (pending intent verification — author may intend separate codification cycle).

**L-P26-002 [LOW] [process-gap]: BC-1.05.036 §Postcondition 2 enumerates 8 payload fields but no Rust struct, JSON schema, or canonical event fixture exists at `crates/factory-dispatcher/`.**
- BC-1.05.036:48-49 lists `{plugin_id, binary, args_count, exit_code, duration_ms, stdout_bytes, stderr_bytes, truncated}` — 8 fields with declared types.
- Implementer free to omit fields or rename them; only manual review catches.
- Out of v1.7..v1.25 scope per S-7.03 SHIP-AS-IS pattern. (pending intent verification — D-9.4 strategy is "no new BCs"; canonical event fixtures may belong in S-9.07 burst).

**L-P26-003 [LOW] [process-gap]: perf-baseline §"Sampling Variance" requires median-of-3 hyperfine sessions but `measure-bundle-sizes.sh` runs only 1.**
- perf-baseline-w16.md:317-320 (per-wave protocol) vs `.factory/measurements/measure-bundle-sizes.sh` (single-session impl).
- Wave-2 implementer running existing tool gets non-contract-compliant single-session value.
- Out of v1.7..v1.25 amendment scope; pre-existing S-9.00 implementation gap. Requirement correctly documented as downstream-wave obligation.

## Out-of-scope-but-noted

- `crates/factory-dispatcher/src/host/mod.rs:118` builds every denial event with single constant `INTERNAL_CAPABILITY_DENIED = "internal.capability_denied"` (`internal_log.rs:82`). The reason is a *field*, not the event name. Spec matches source. Positive-evidence anchor.
- E-10 v1 epic exists at `.factory/stories/epics/E-10-single-stream-otel-event-emission.md` (last_amended 2026-05-04). v1.25 does not touch E-10. Confirms OQ-W16-001's owner ("E-10 Wave 1 architect") has real anchor.

## Process-gaps

3 LOW process-gaps codified above (L-P26-001/002/003).

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.25): PASS
- All 7 amendment-touched files have `last_amended: 2026-05-05`: PASS
- BC-1.05.035 + BC-1.05.036 ADR-015 awareness clauses: PASS
- BC-1.05.036 §Postcondition 5 enumerates correct 4 denial reasons with line citations: PASS (line 52)
- BC-1.05.036 §EC-003 sibling-aligned to Postcondition 5: PASS (line 85)
- BC-1.05.036 §Postcondition 3 Instant cite corrected (line 252 spawn, not line 270 deadline): PASS (line 50)
- v1.7-v1.25 summary rows intact (POLICY 1): PASS
- v1.26 preemptive reserved row: PASS (line 489)
- v1.25 H3 section present: PASS (line 1351)
- v1.7-v1.24 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.25: PASS
- H3 version count matches summary table: PASS
- audit-w16.md sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible: PASS
- Citation line numbers accurate: PASS
- No retired-figure residue in non-changelog body: PASS

## Angle-specific outputs

CI/CD pipeline simulation: 9 contracts immediately authorable (BC-1.05.035 P-1/P-4/precedence; BC-1.05.036 P-1/P-3; ADR-015 D-15.1 single-stream; AC-3 advisory soft cap; AC-6 HOST_ABI_VERSION grep). 3 require scaffolding (BC-1.05.036 P-2 8-field schema; BC-1.05.036 P-5 enumeration validator; per-wave median-of-3 protocol). No spec-internal contradictions blocked simulation.
