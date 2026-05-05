# Adversarial Review — E-9 v1.8 Fix Burst (D-242) — Pass 4

**Date:** 2026-05-05
**Commit reviewed:** c3855ae (v1.7 → v1.8)
**Files reviewed:** 4 (E-9 epic, gap-analysis-w16-subprocess.md, perf-baseline-w16.md, audit-w16.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 (reset)
**Pass methodology angle:** Exhaustive ADR-015 cross-reference citation verification — NEW relative to passes 1 (positive verification), 2 (reverse-trace ADR→landing sites), 3 (forward-simulation/counter-examples). I read ADR-015 as the source of truth and treated every numeric/structural citation in the v1.8 diff as a hypothesis to falsify against ADR-015's actual contents.

## Summary

The v1.8 fix burst correctly closes H-1 (block-event misattribution at 4 sites), L-1 (trace-id wording), M-1 (vsdd.host.* MUST/pending), and M-3 (perf-baseline frontmatter). However, the M-2 closure introduces a fabricated cross-reference: the rationale's leg (c) cites "Wave 3 acceptance criterion 3" which does not exist in ADR-015 — only AC-1 (pr_throughput) and AC-2 (unknown_category_events) are defined. POLICY 4 (semantic_anchoring_integrity) violation; clock resets.

## Findings (severity-ordered)

### HIGH

**H-P4-001 [HIGH]: M-2 fix cites non-existent "Wave 3 acceptance criterion 3" — fabricated ADR-015 anchor**

- File:line: `gap-analysis-w16-subprocess.md` lines 342-344 ("(c) Wave 3 acceptance criterion 3 queries `event.category=audit` for security review dashboards"); E-9 epic lines 719-721 (v1.8 changelog M-2 closure entry: "Wave 3 AC-3 queries `event.category=audit` for SIEM dashboards").
- Counter-evidence: ADR-015 Wave 3 (lines 623-638) defines exactly TWO acceptance criteria — AC-1 line 631 (`pr_throughput`); AC-2 line 634 (`unknown_category_events`). Greps for `Acceptance criterion|criterion` across ADR-015 confirm only criteria 1 and 2 exist (lines 631, 634, 746).
- Why HIGH (not MED): The M-2 closure rests its rationale on this non-existent SIEM query as a third leg of justification. The choice itself is correct on legs (a) + (b); leg (c) is fabricated. POLICY 4 says "every anchor claim must be semantically correct... mis-anchoring always blocks convergence." Implementer reading this rationale, trying to verify in ADR-015, finds nothing — erodes trust in all four files' cross-refs.
- Fix: Drop leg (c) entirely (legs a + b sufficient), OR rewrite leg (c) to cite actual basis: "Audit category events queryable in SIEM dashboards by `event.category=audit` filter (ADR-015 D-15.2 §`event.category` taxonomy registry, lines 295-333)." The choice itself stands; only the citation is broken.

### MED

(none)

### LOW

**L-P4-001 [LOW]: v1.8 changelog "Site 1: E-9 lines ~294-296" off-by-2 from actual landing site**

- File:line: E-9 epic line 703 cites "Site 1: E-9 lines ~294-296"; actual block-mode H-1 closure text lives at lines 294-299 (or ~294-302 to bracket full ADR-015 awareness block edits).
- Self-reference within same file's changelog; does not affect implementer correctness; `~` qualifier explicitly tolerates approximation.
- Fix (optional): "lines ~294-299" or "lines ~294-302".

## Out-of-scope-but-noted

None observed. The v1.8 fix burst respected scope: 4 files only, no new BCs/VPs/FRs, no story body changes.

## Process-gaps

**P-P4-001 [process-gap]:** Adversary review skill should require fix-burst rationale citations to be re-verified against authoritative source. The v1.8 fix burst added a rationale leg "(c) Wave 3 AC-3" without re-reading ADR-015 to confirm "AC-3" exists. This class of error (fabricated cross-reference invented during fix-burst justification writing) recurs and is hard to catch via index-level checks because the cited target is in a different document. Recommendation: extend `adversarial-review-skill` and architect prompts so any rationale leg of form "ADR-XXX [line N | section §X | acceptance criterion N] says Y" must include a copy-paste of cited text in fix-burst commit message OR architect MUST re-read cited section before merging. The adversary cannot adjudicate intent — only that citation as-written is broken.

## Convention checks

- v1.7 summary row intact (POLICY 1): PASS — line 469.
- v1.8 summary row appended: PASS — line 470.
- v1.9 reserved row preemptive: PASS — line 471 (`| 1.9 | — | — | (reserved) |`).
- v1.8 H3 section present and consistent: PASS — line 689 H3, content lines 691-730 closes H-1/M-1/M-2/M-3/L-1, notes L-2 SKIPPED. H3 count (8 sections v1.1-v1.8) matches summary table (10 rows minus v1.0 initial minus v1.9 reserved = 8).
- No "Lines: X → Y" footer at v1.8 (or v1.7): PASS — neither v1.7 (665-687) nor v1.8 (689-730) contain a line-count footer.

## Regression check (did v1.8 reopen any v1.7-converged matter?)

No structural regressions:
- v1.7 ADR-015 awareness block (lines 282-305) edited cleanly to absorb H-1 option (b) + L-1; preserves bulleted list shape, citations for D-15.1/D-15.2/D-15.3/D-15.4 intact at line 303.
- v1.7 R-W16-003 ADR-015 note (line 353) untouched.
- Changelog summary table v1.0 → v1.8 rows are append-only intact (POLICY 1 satisfied); v1.9 reserved preemptive row preserved.
- audit-w16.md edits at 35/37/47-50 cleanly drop plugin-side MUST and replace with "Plugins do NOT emit a redundant event on the block path (D-15.3)".
- perf-baseline-w16.md M-3 fix (line 17 ADR-015 row appended after ADR-013) is strict additive.

No regressions on v1.6-converged matter (D-9.4, D-9.5, AC table, Library Table, Architecture Mapping, Dependency Graph all untouched in v1.8 diff).

One regression introduced: the M-2 "Wave 3 acceptance criterion 3" citation (H-P4-001). NEW defect introduced by v1.8 fix burst, not a reopened v1.7 finding.

## Angle-specific outputs

ADR-015 cross-reference verification matrix:

| v1.8 cited claim | Source file:line | ADR-015 anchor verified? |
|---|---|---|
| `vsdd.capability.denied.* → audit` (M-2 leg a) | gap-analysis line 339-340 | YES — ADR-015 line 329 |
| `vsdd.internal.* → lifecycle` (M-2 leg b) | gap-analysis line 341 | YES — ADR-015 line 331 |
| **"Wave 3 acceptance criterion 3 queries `event.category=audit`"** (M-2 leg c) | gap-analysis lines 342-343 + E-9 line 720 | **NO — fabricated. Wave 3 has only AC-1 + AC-2.** See H-P4-001. |
| `vsdd.dispatcher.*` registry → fallback `vsdd.dispatcher.subprocess_completed.v1` "lifecycle" (M-1) | gap-analysis lines 323-325 | YES — ADR-015 line 319 |
| `vsdd.host.*` not in registry, forward-pointer (M-1) | gap-analysis line 322 | YES — full registry table 317-332 contains no `vsdd.host.*` entry |
| D-15.4 dispatcher-injected `VSDD_TRACE_ID`/`VSDD_PARENT_SPAN_ID` unconditional (L-1) | E-9 lines 300-302; gap-analysis lines 330-333 | YES — D-15.4 lines 401-419 explicitly says "dispatcher-side mandatory injection" |
| D-15.3 block path automatic emit `vsdd.block.plugin_blocked.v1` (H-1 option b) | E-9 lines 295-299; audit-w16 lines 35/37/47-50 | YES — D-15.3 lines 374-378 |
| `Command::new` env setup (L-1 wording) | E-9 line 301 | YES — `crates/factory-dispatcher/src/host/exec_subprocess.rs:230` confirms `Command::new(cmd)` |
| ADR-015 row added to perf-baseline frontmatter `references:` after ADR-013 (M-3) | perf-baseline-w16.md line 17 | YES — append-only after line 16 ADR-013 row |

**Summary: 8 of 9 cited claims correctly anchored; 1 of 9 fabricated (HIGH severity).**
