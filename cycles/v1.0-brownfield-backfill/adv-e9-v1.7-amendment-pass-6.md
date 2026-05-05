# Adversarial Review — E-9 v1.10 Fix Burst (D-246) — Pass 6

**Date:** 2026-05-05
**Commit reviewed:** dc9a71d (v1.9 → v1.10) — cumulative surface v1.7..v1.10
**Files reviewed:** 4 (E-9 epic, audit-w16.md, gap-analysis-w16-subprocess.md, perf-baseline-w16.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 reset
**Pass methodology angle:** Adversarial-implementer mindset + Boundary-cases / silence-audit hybrid (NEW per TD-VSDD-057). Read 4 files as a maliciously-buggy implementer would — looking for loopholes the amendment leaves wide enough for a wrong implementation that still satisfies stated requirements. Then cross-checked closure claims of v1.10 burst by examining what each "CLOSED" finding actually changed.

## Summary

The v1.10 fix burst correctly recovers POLICY 1 (M-P5-001 v1.8 prose restore) and closes frontmatter drift (H-P5-001). However, the M-P5-003 closure claim is only partially substantiated: audit-w16 B-7 row was added, but the "all 5 block-mode hooks" claim is factually wrong — B-2 and B-6 only get a parenthetical mention in the lumped "Standard." row at line 38, not the explicit dispatcher-emits-automatically treatment given to B-1/B-3/B-7. Plus 4 MED amendment-surface gaps (frontmatter convention drift, binary-choice tracking, event.host_overrides silence, schema_url silence).

## Findings

### HIGH

**H-P6-001 [HIGH]: M-P5-003 closure overstated — B-2 and B-6 missing explicit H-1 option (b) coverage in audit-w16.md**
- Files: E-9 epic lines 775-780 (closure claim "all 5 block-mode hooks now have explicit H-1 option (b) coverage") vs audit-w16.md lines 35-38.
- Evidence: audit-w16.md line 35 (B-1 row), line 36 (B-7 row), line 37 (B-3 row) each explicitly state "the dispatcher automatically emits `vsdd.block.plugin_blocked.v1` — no additional plugin-side emission required." Line 38 lumps B-2 + B-4 + B-5 + B-6 into a single "**Standard.**" row that only says `No subprocess or block-mode hooks except validate-input-hash (B-2, block-mode) and validate-template-compliance (B-6, block-mode).` — NO explicit H-1 option (b) treatment.
- Impact: implementer working from audit-w16 alone for S-9.02 (B-2) and S-9.06 (B-6) will not see H-1 option (b) and may write redundant plugin-side block emission. 3-of-5 vs claimed 5-of-5 coverage gap is a propagation defect from D-242's H-1 closure.
- Fix: amend audit-w16.md line 38 to either (a) split B-2 and B-6 into their own rows with same explicit dispatcher-emits-automatically wording as B-1/B-3/B-7, or (b) append an enumerating sentence to existing line 38 row.

### MED

**M-P6-001 [MED]: Frontmatter convention drift across 3 arch docs — D-239 annotate-in-place applied inconsistently**
- Files: gap-analysis-w16-subprocess.md line 5 (`version: "1.0"`), audit-w16.md line 5 (`version: "1.0"`) — neither carries `last_amended:` or ADR-015 in frontmatter despite v1.7 amendment annotations in body. perf-baseline-w16.md DOES carry ADR-015 in frontmatter `references:` (line 17).
- Impact: tool scanning frontmatter for "which docs cite ADR-015" finds perf-baseline-w16.md but misses audit-w16.md and gap-analysis-w16-subprocess.md.
- Fix: pick one — either add ADR-015 to inputs/references field of audit-w16.md + gap-analysis frontmatter (and add `last_amended: 2026-05-05`), OR remove the ADR-015 row from perf-baseline-w16.md's frontmatter.

**M-P6-002 [MED]: vsdd.host.* binary-choice has no tracking artifact**
- File: gap-analysis-w16-subprocess.md lines 320-326. M-1 closure forward-pointer says "SS-01 implementer or E-10 Wave 1 author MUST resolve this prefix choice before merging the host-emit-fix story" — but no OQ ID, no TD entry, no link to a decisions log.
- Impact: silent-failure boundary; if E-10 Wave 1 ships without addressing, no automated check detects the choice was never made; implementer picks one and spec drift becomes invisible.
- Fix: file OQ-W16-NNN (or TD entry) tracking the prefix-choice resolution with explicit acceptance criterion.

**M-P6-003 [MED]: Boundary case — event.host_overrides not addressed for E-9 plugins**
- Files: E-9 epic lines 282-305 (D-9.2 ADR-015 awareness block) vs ADR-015 line 274 + lines 350-372 (D-15.3 host_field_override visibility two-channel approach).
- Evidence: ADR-015 D-15.3 specifies that when a plugin supplies a host-owned field, dispatcher MUST: (1) emit `vsdd.internal.host_field_override.v1`, (2) stamp `event.host_overrides: [<list>]` on the offending domain event, (3) write a stderr warning. E-9 awareness block (lines 287-289) summarizes only "host wins unconditionally" half — silent on the 2 observability obligations.
- Impact: plugin author reading E-9 awareness block alone won't know about `event.host_overrides` channel or lifecycle event pairing; story-writer authors S-9.01..S-9.07 missing override-visibility ACs.
- Fix: extend E-9 line 290 bullet to enumerate override observability obligations: "If plugin accidentally stamps host-owned field, dispatcher emits `vsdd.internal.host_field_override.v1` AND stamps `event.host_overrides` on offending event (D-15.3 two-channel approach)."

**M-P6-004 [MED]: Boundary case — event.schema_url for vsdd.hook.validate.*.v1 unspecified**
- Files: E-9 epic line 291; audit-w16.md line 45 vs ADR-015 D-15.2.d (line 250) + D-15.3 schema versioning (line 388).
- Evidence: ADR-015 D-15.2.d defines `event.schema_url` as per-event-family schema URI. E-9 + audit-w16 specify event.name format `vsdd.hook.validate.<hook_slug>.v1` but say nothing about event.schema_url. D-15.3 schema versioning says breaking changes bump event.name suffix AND event.schema_url URI.
- Impact: plugin authors of validate-* hooks need either to emit per-event-family schema_url OR rely on Resource-level baseline. Neither specified. Maliciously-buggy implementer picks whichever; contract drifts.
- Fix: add sentence to E-9 line 290-292: "Plugin SHOULD set `event.schema_url` to `https://vsdd-factory.dev/schemas/events/v2/hook.validate.<hook_slug>` OR rely on Resource-level baseline schema_url (informational; no functional difference per ADR-015 D-15.2.d)."

### LOW

**L-P6-001 [LOW]: Retracted on self-validation.** H3 count = 10, summary table count = 10. PASS.

**L-P6-002 [LOW]: input-hash literal `37151a4` vs F-P2-010 closure note `[pending-recompute]` consistency**
- File: E-9 epic line 28 vs F-P2-010 closure line 588.
- Evidence: F-P2-010 says `[pending-recompute]`; line 28 actual frontmatter shows literal hex `"37151a4"`. No "Closure" entry explains transition. State-manager presumably handles. Low concern.

**L-P6-003 [LOW]: TD-VSDD-059 referenced in v1.10 H-P5-001 closure but unverified to exist by adversary**
- Note: pass-6 adversary did not have visibility to open-backlog-post-rc8.md (in cycles dir). State-manager confirmed in D-245 that TD-VSDD-059 + TD-VSDD-060 were filed there. This finding is information-asymmetry artifact, not real defect. Resolved by orchestrator at seal time.

## Out-of-scope-but-noted

- v1.6 body convergence preservation: stable across v1.7-v1.10. No drift.
- S-9.30 withdrawn audit trail: stable. No drift.

## Process-gaps

- [process-gap]: Inconsistent application of D-239 annotate-in-place convention across 3 arch docs (M-P6-001).
- [process-gap]: M-1 binary-choice resolution has no tracking artifact (silent-failure boundary; M-P6-002).
- (PG-L-P6-003 invalidated — TD-VSDD-059 IS filed; resolved at seal time.)

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.10): PASS (line 4 = "1.10")
- v1.7-v1.10 summary rows intact (POLICY 1): PASS (lines 469-472)
- v1.11 preemptive reserved row: PASS (line 473)
- v1.10 H3 section present: PASS (line 757)
- v1.8 block prose preserved as authored (POLICY 1 append-only): PASS (M-P5-001 restoration verified — line 723 retains original "Wave 3 AC-3" wording)
- No "Lines: X → Y" footer at v1.7-v1.10: PASS
- H3 version count matches summary table: PASS (10 = 10)

## Angle-specific outputs (Adversarial-implementer + Boundary-cases)

Implementer-loophole catalog:
1. event.host_overrides silent omission → 1/3 of D-15.3 obligations missing from awareness (M-P6-003)
2. event.schema_url per-event-family unspecified → drift risk (M-P6-004)
3. vsdd.host.* registry-prefix default → no enforcement signal (M-P6-002)
4. B-2/B-6 block-mode H-1 coverage gap → redundant plugin-side block emission risk (H-P6-001)

Silence-audit (acceptable silences):
- Retention policy of events-*.jsonl: config-level, not story-level. OK.
- trace_id propagation when subprocess fails to start: dispatcher concern. OK.
- HookResult::Block + dispatcher crash atomicity: ADR-015 silent too. OK per Audit category integrity note.
