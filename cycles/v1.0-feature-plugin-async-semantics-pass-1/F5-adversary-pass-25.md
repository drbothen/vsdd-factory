---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 25
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T02:30:00Z
strategic_recommendation: continue-protocol-per-user-directive
---

# F5 Pass-25 Adversary Review

## Verdict

**HIGH** — 8th consecutive HIGH. 4H + 1M + 2L + 2 process-gaps. ADR-013 clock RESETS to 0_of_3.

Fix-burst-23's comprehensive corpus audit closed the explicitly-cataloged 10 fabricated symbols. However pass-25 finds the SAME pattern class extends to NEW symbols/layers not in the catalog: `run_event` (5 sites), `drain_async_tasks` (5 sites), `engine.rs` as emission-site host (4 sites), `RegistryEntry.async` prose-form variants (3 sites). The recurrence carrier is now identified: **S-15.01 §Implementation Modules tables freeze pre-merge pseudocode vocabulary** that has no canonical post-merge update path. Per user directive, no escalation prompt.

## Trajectory

→HIGH(P18)→HIGH(P19)→HIGH(P20)→HIGH(P21)→HIGH(P22)→HIGH(P23)→HIGH(P24)→**HIGH(P25)**

## Findings

### F-P25-001 [HIGH] `run_event` fabricated at 5 sites
- ADR-019:107, E-15:242, S-15.01:433/493/573
- `grep -rn "fn run_event" crates/factory-dispatcher/src` → 0 matches
- Sub-burst 1 catalog included `run_tiers` and `engine.rs as dispatch-loop host` but NOT `run_event` (pseudocode dispatch-loop function name from F1-delta-analysis pre-merge planning lineage).
- **Fix:** replace cites with reference to actual main.rs dispatch flow.

### F-P25-002 [HIGH] `drain_async_tasks` fabricated at 5 sites in S-15.01
- S-15.01:434/495/499/572/625
- `grep -rn "drain_async_tasks" crates/factory-dispatcher/src` → 0 matches
- Real implementation: inline drain logic at `main.rs:323-370` (tokio::select! + sleep timer)
- **Fix:** replace cites with reference to inline drain logic in main.rs OR refactor production to expose named function.

### F-P25-003 [HIGH] `engine.rs` as event-emission-site host at BC-3.08.001:224/278, VP-077:18, VP-079:16
- Real emission sites: `main.rs:46/423/550` (call sites) + `host/emit_event.rs:138-155` (emit fns)
- Same class as F-P24-002 patched in BC-7.06.001:130; sub-burst 1 catalog included "engine.rs as dispatch-loop host" but did NOT extend to "engine.rs as emission-site host"
- **Fix:** replace `engine.rs` with `main.rs` + `host/emit_event.rs` at all 4 sites.

### F-P25-004 [HIGH] `RegistryEntry.async` prose-form variants at E-15:240, S-15.01:174, S-15.01:426
- E-15:240: "`async` field on `RegistryEntry`"
- S-15.01:174: "#[serde(default)] on RegistryEntry"
- S-15.01:426: missing `rename = "async"` in serde attribute
- Sub-burst 1 grep was for literal `RegistryEntry.async` (with dot); missed prose-form variants
- **Fix:** replace with `async_flag` + correct serde attribute `#[serde(default, rename = "async")]`

### F-P25-005 [MEDIUM] BC-INDEX rows 259, 352 stale enrichment annotations
- BC-INDEX:259 BC-7.06.001 row: `(v1.9: F5 fix-burst-13 ...)` but file is at v1.11
- BC-INDEX:352 BC-3.08.001 row: `(v1.11: F5 fix-burst-16 ...)` but file is at v1.12
- POLICY 7 borderline: enrichment-not-in-H1
- **Fix:** strip parenthetical version annotations OR refresh to current.

### F-P25-006 [LOW] BC-3.08.001 frontmatter has duplicate `last_amended:` keys (lines 5 and 9)
YAML parser uses last value (still 2026-05-08) so semantically OK, but schema malformed.

### F-P25-007 [LOW pending intent] F1-delta-analysis.md:128, 137, 142 carries pre-merge pseudocode (`run_tiers`, `spawn_detached`, `run_event`)
F1 architect proposals NOT explicitly in L-P24-001 carve-out. Recommend annotation as carved-out audit record OR extend L-P24-001 to include `producer: architect, phase: F1, status: draft`.

## Process-gap findings

### O-P25-001 [process-gap] L-P24-002 codifying burst applied catalog narrowly — same failure mode L-P24-002 names

Sub-burst 1 swept ONLY the cataloged 10 symbols but the CLASS extends to additional pre-merge pseudocode (run_event, drain_async_tasks, engine.rs as emission-host). L-P24-002 needs extension: "complete historical catalog" must be derived SEMANTICALLY from F1-delta-analysis (the canonical pre-merge pseudocode source), not enumerated literally.

### O-P25-002 [process-gap] S-15.01 status: ready post-merge — story body not retrofitted to actual implementation

S-15.01 is in Merged bucket per STATE.md but frontmatter says `status: ready` and §Implementation Modules + §Tasks describe pre-merge pseudocode. Without post-merge story-body retrofit discipline, every adversary pass finds "fabricated symbols" that are actually frozen pre-merge planning vocabulary.

**Codification proposal:** when story moves to Merged, state-manager must rewrite §Implementation Modules + §Tasks to reflect merged code OR mark sections as `status: superseded-by-implementation`.

## Notable observations

1. Fix-burst-23 closure verified for explicit catalog (10 symbols, 11 sites): all VERIFIED.
2. Index versions confirmed: BC-INDEX v1.49 / VP-INDEX v1.32 / STORY-INDEX v2.53 / ARCH-INDEX v1.29.
3. POLICY 1 spot-check (5 BCs): clean modulo F-P25-005 enrichments.
4. POLICY 4 violations: F-P25-001/002/003/004 (4 layers).
5. POLICY 7: clean for sampled BCs.
6. POLICY 8: S-15.01 frontmatter sync clean.
7. **L-P24-002 retroactive-application audit:** sub-burst 3 ran comprehensive sweep ONLY for explicit catalog at trigger sites; did NOT retroactively apply L-P24-002 to verify PRIOR lessons (L-P19-001, L-P21-001, L-P23-001) were comprehensively swept. F-P25-001/002/003/004 are evidence.
8. STATE.md at 192 lines.

## Convergence assessment

**Novelty: HIGH.** New layers exercised: story §Implementation Modules tables, VP frontmatter `module:` fields, BC §Traceability `Architecture Module` rows, epic §Architecture Components tables.

**ADR-013 clock: 0_of_3** (RESET).

8 consecutive HIGH passes. Each fix-burst broader than the last; each next pass finds NEW layers. **Dominant carrier identified: S-15.01 frozen pre-merge planning vocabulary.** Without post-merge story-body retrofit + mechanical enforcement (S-15.03), recurrence continues.

Per user directive: continue protocol; no escalation prompt.
