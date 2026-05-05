# Adversarial Review — E-9 v1.13 Combined Burst (D-251) — Pass 10

**Date:** 2026-05-05
**Commit reviewed:** 088f46d (v1.13; unchanged from pass-9)
**Cumulative surface:** v1.7..v1.13 (4 files + open-questions.md)
**Verdict:** NITPICK_ONLY
**ADR-013 clock at end of pass:** 1_of_3 → 2_of_3 advance
**Pass methodology angle:** Exhaustive ADR-015 D-15.x clause enumeration — for each clause (D-15.1, D-15.2.a..e, D-15.3, D-15.4) verify whether the amendment surface correctly addresses or correctly leaves silent. Cross-checked clause coverage map against amendment landing sites in 4 files. NEW per TD-VSDD-057.

## Summary

Pass-10 enumerates ADR-015 clauses D-15.1, D-15.2 (with sub-clauses .a–.e), D-15.3, and D-15.4 against amendment landing sites. The amendment surface explicitly addresses D-15.1 (single-stream routing), D-15.2 (event.name, outcome enum, taxonomy registry), D-15.3 (host enrichment, block-path automatic emission), and D-15.4 (trace propagation). Sub-clauses D-15.2.a/.b/.c/.d/.e are intentionally silent per documented deferrals (M-P6-003/004). The amendment correctly avoids redefining ADR-015 contracts. Three LOW findings around terminology precision, transitive subprocess env propagation, and FileSink durability silence. No HIGH or MED findings. v1.13 is structurally consistent and convention-conformant.

## Findings

### HIGH
None.

### MED
None.

### LOW

**L-P10-001 [LOW]: D-15.4 amendment uses Rust-stdlib API noun "Command::new" in policy-level prose**
- File: E-9 epic line 301-302 + 713-714. ADR-015 v1.7 changelog Polish-2'd D-15.4 line 426 from "env-allowlist" to "dispatcher-injected invariants" to remove implementation-leak ambiguity. E-9 v1.8 H-1 closure + D-9.2 awareness-block describe mechanism as "Command::new env setup" — anchors specific Rust stdlib type (`std::process::Command::new`) in policy prose. Both citations technically accurate (dispatcher's Rust impl does call `Command::new`); ADR-015's explicit Polish-2 chose policy-level "dispatcher-injected invariants". Re-aligning amendment wording would match ADR-015 vocabulary. Confidence: HIGH (file:line evidence). Severity: LOW (terminology drift, semantically correct).

**L-P10-002 [LOW]: Amendment silent on D-15.4 transitive forwarding for verify-sha-currency.sh grandchildren**
- File: gap-analysis-w16-subprocess.md lines 333-336. ADR-015 D-15.4 lines 418-419: "Subprocess plugins that spawn further subprocesses inherit and forward both vars." validate-wave-gate-prerequisite invokes verify-sha-currency.sh (S-9.07), which itself execs git, python3, grep. Amendment guarantees dispatcher injects VSDD_TRACE_ID/VSDD_PARENT_SPAN_ID into bash's env; silent on whether bash → git/python3 transitive propagation is required, expected, or relied-upon. Bash default env-inheritance produces correct behavior, but silence leaves implementer uncertain whether to rely on default semantics or add explicit forwarding. Confidence: MEDIUM. Severity: LOW (latent obligation gap; will likely Just Work).

**L-P10-003 [LOW]: Amendment silent on D-15.1 FileSink write-failure semantics for block-mode audit trail durability**
- File: audit-w16.md lines 35-38 + E-9 epic lines 296-299. ADR-015 D-15.1 lines 108-119 define unconditional fallback to dispatcher-internal-*.jsonl + stderr WARN when FileSink::write fails. Amendment claims for block-mode hooks dispatcher's automatic vsdd.block.plugin_blocked.v1 emission "fully satisfies D-15.3" — true for emission step but silent on durability path: if FileSink write of block-event fails on block path, audit-trail durability depends on D-15.1's debug-file fallback. Amendment doesn't reference D-15.1 fallback path; story-writer authors S-9.01..S-9.07 ACs around block-path durability without explicit anchor. Confidence: MEDIUM. Severity: LOW (downstream-implementer concern, not current contradiction).

## Out-of-scope-but-noted

None.

## Process-gaps

None new this pass.

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.13): PASS
- v1.7-v1.13 summary rows intact (POLICY 1): PASS (lines 469-475)
- v1.14 preemptive reserved row: PASS (line 476)
- v1.13 H3 section present: PASS (line 944, format `### v1.13 (2026-05-05) — D-251 ...`)
- v1.7-v1.12 block prose preserved as authored (POLICY 1 append-only): PASS (no in-place mutations; v1.8 prose was already restored at v1.10 per M-P5-001)
- No "Lines: X → Y" footer at v1.7-v1.13: PASS (only v1.2-v1.5 carry footers; v1.6+ none)
- H3 version count matches summary table: PASS (v1.0..v1.13 = 14 H3 sections; summary table = 14 rows + reserved)
- audit-w16.md line 38 sibling-wording-template consistency: PASS (parenthetical, "are block-mode", bare D-15.3)
- No fix-burst-internal IDs leak into permanent specs body: PASS (0 matches in 4 files outside changelog blocks)
- Outbound decision-ID anchors semantically compatible (TD-VSDD-065): PASS (perf-baseline line 156 = E-9 AC-3 per v1.13 fix; AC-3 at E-9 line 368 is the gate-model AC — correct anchor)

## Angle-specific outputs

ADR-015 D-15.x clause coverage map for v1.7..v1.13 amendment surface:

| ADR-015 clause | Amendment landing | Verdict |
|---|---|---|
| D-15.1 single-stream | E-9 line 286 + perf-baseline line 350 + audit-w16 line 55 | Addressed correctly |
| D-15.1 FileSink write-failure semantics | (silent) | Silent (acceptable; story-writer responsibility — see L-P10-003) |
| D-15.1 sink-otel-grpc retirement | (silent) | Correctly silent (E-10 territory) |
| D-15.2 event.name reverse-DNS + .v1 | E-9 line 291-293 + audit-w16 line 44-45 | Addressed correctly |
| D-15.2 outcome enum | E-9 line 293-296 + audit-w16 line 46-50 | Addressed correctly |
| D-15.2.a registry compile-time | OQ-W16-001 acknowledges (open-questions line 28) | Correctly anchored |
| D-15.2.b unrecognized prefix → unknown | E-9 line 293 + gap-analysis line 322-327 | Addressed |
| D-15.2.c Resource fallback cascade | (silent) | Correctly silent (host-side concern) |
| D-15.2.d per-event event.schema_url | (silent) | Correctly silent per M-P6-004 deferral |
| D-15.2.e dual-emit identity contract | (silent) | Correctly silent (E-10 territory) |
| D-15.3 host enrichment contract | E-9 line 287-290 + audit-w16 line 51-53 | Addressed correctly |
| D-15.3 host_field_override visibility | (silent) | Correctly silent per M-P6-003 deferral |
| D-15.3 block path automatic emission | E-9 line 294-299 + audit-w16 line 35-38 (5 hooks enumerated) | Addressed correctly |
| D-15.3 bash hook parity | (silent) | Correctly silent (Phase H / R-W16-001) |
| D-15.3 schema versioning | (silent) | Correctly silent (initial v1 ship) |
| D-15.4 dispatcher-injected env | E-9 line 300-302 + gap-analysis line 333-336 + audit-w16 line 36 | Addressed (terminology nit per L-P10-001) |
| D-15.4 transitive subprocess forward | (silent) | Silent (relevant to S-9.07; see L-P10-002) |

Counter-example test: implementer with only the amendment surface (without reading ADR-015 directly) correctly derives: emit to events-*.jsonl ✓; reverse-DNS .v1 event.name ✓; canonical outcome enum ✓; don't stamp host fields ✓; block-mode: return HookResult::Block, no plugin-side emit needed ✓. The amendment correctly steers them. Three LOW findings are around silence on transitive concerns — not clause-level mis-direction.

Reverse check: all amendment-cited clauses (D-15.1, D-15.2, D-15.3, D-15.4) exist in ADR-015. No fabricated clause references.
