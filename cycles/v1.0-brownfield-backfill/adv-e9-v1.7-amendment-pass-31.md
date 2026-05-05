# Adversarial Review — E-9 v1.28 Inverse-Traceability Fix (D-274) — Pass 31

**Date:** 2026-05-05
**Commit reviewed:** e8f74ad (v1.28; D-272 cross-doc terminology drift fix)
**Cumulative surface:** v1.7..v1.28 (E-9 epic + gap-analysis-w16-subprocess.md + audit-w16.md + perf-baseline-w16.md + BC-1.05.036 + BC-1.05.035)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 1_of_3 → 0_of_3 RESET (by MED findings)
**Pass methodology angle:** Inverse-traceability — start from normative ADR-015 obligations (D-15.1 Rationale, D-15.2, D-15.3, D-15.4) and walk backward through amendment surface to verify all normative claims are correctly framed. NEW per TD-VSDD-057. Checks: (1) tense discipline (normative future-state vs present-tense overstatement), (2) mandated fields present (outcome enum per D-15.2), (3) line-cite accuracy, (4) paraphrase fidelity.

## Summary

Pass-31 inverse-traceability angle caught two MEDs and three LOWs. The angle differs from prior 30 passes in that it walks FROM normative ADR-015 text backward to amendment landing sites, rather than FROM amendment surface forward to cited docs. This reverse walk exposed tense-discipline overstatement in gap-analysis-w16-subprocess.md:334-337 + audit-w16.md B-7 row (both claim trace-id injection is already present as "automatic"/"invariant" when ADR-015 D-15.4 frames it as normative MUST/future-state); and missing outcome enum in BC-1.05.036 Postcondition 2 per ADR-015 D-15.2:270.

ADR-013 clock resets to 0_of_3. Three consecutive NITPICK_ONLY passes (32/33/34) needed to reach CONVERGENCE_REACHED.

## Findings

### HIGH

None.

### MED

**MED-P31-001 [MED]: gap-analysis:334-337 + audit-w16 B-7 row — trace-id injection framed as present-tense "automatic invariant" contradicts ADR-015 D-15.4 normative future-state**

- Citation 1: `gap-analysis-w16-subprocess.md:334-337` — "`VSDD_TRACE_ID` and `VSDD_PARENT_SPAN_ID` are injected by the dispatcher into every `exec_subprocess` invocation unconditionally (ADR-015 D-15.4). The `validate-wave-gate-prerequisite` subprocess hop (S-9.07) inherits trace context automatically — no per-plugin manifest change needed."
- Citation 2: `audit-w16.md line 37, B-7 row` — "this is automatic (dispatcher-side invariant); the plugin does not need manifest changes"
- Reality per ADR-015 D-15.4:407-419: "MUST be injected by the dispatcher" (normative future-state obligation); current `execute_bounded` at `exec_subprocess.rs:242-247` does `env_clear()` + selective `env_allow` forward ONLY — no `VSDD_TRACE_ID` or `VSDD_PARENT_SPAN_ID` injection present.
- Impact: S-9.07 implementer reading gap-analysis or audit-w16 in present tense incorrectly assumes trace wiring exists and skips implementation work. Injection is pending E-10 Wave 1 (OQ-W16-001 owner).
- Severity: MED (implementer confusion; tense overstatement on normative requirement not yet implemented).

**MED-P31-002 [MED]: BC-1.05.036 Postcondition 2 (lines 48-49) omits `outcome` enum field mandated by ADR-015 D-15.2:270**

- Citation: BC-1.05.036 §Postcondition 2 — lists 8 fields: `{plugin_id, binary, args_count, exit_code, duration_ms, stdout_bytes, stderr_bytes, truncated}`.
- Reality per ADR-015 D-15.2:270: "| `outcome` | canonical enum: `success` \| `failure` \| `error` \| `timeout` \| `skipped` \| `blocked` |" — this field is mandated by the host-enrichment layer for every event in the single-stream schema.
- No exit_code→outcome mapping rule exists in BC-1.05.036.
- Impact: BC implementer building `host.exec_subprocess.completed` event does not know to stamp `outcome` field or how to derive it from `exit_code`.
- Severity: MED (missing normative field; implementer omits it without specification guidance).

### LOW

**LOW-P31-003 [LOW]: BC-1.05.036 Postcondition 5 + EC-007 — stdin write-failure cite `:262` should be `:259`**

- Citation: BC-1.05.036:52 "stdin take/write failure (:258, :262)" and BC-1.05.036:89 (EC-007) "stdin take/write at :258/:262".
- Reality per exec_subprocess.rs:259: `child_stdin.write_all(stdin_bytes).is_err()` — the `write_all().is_err()` check is at line 259, not line 262. Line 262 is `let _ = child.kill();` (kill call inside the error branch).
- Impact: Line cite is wrong; minor but misleading to TD-VSDD-075-class source-code traceability verification.
- Severity: LOW (off-by-3 line cite; content correct, cite wrong).

**LOW-P31-004 [LOW]: perf-baseline-w16.md line 353 "sub-millisecond I/O" is an undocumented paraphrase**

- Citation: perf-baseline-w16.md:353 "adds negligible overhead (sub-millisecond I/O)".
- Reality per ADR-015 D-15.1 Rationale:432-440: "a single FileSink append to `events-YYYY-MM-DD.jsonl` handles 10k events/minute without measurable overhead" — ADR-015 does NOT use "sub-millisecond" language.
- Impact: Paraphrase introduces uncited quantitative claim; implementer may cite "sub-millisecond" as ADR-015 sourced when it is a perf-baseline-local paraphrase.
- Severity: LOW (paraphrase fidelity; quantitative claim without source citation).

**LOW-P31-005 [LOW]: BC-1.05.036 Postcondition 4 "Same code path as emit_denial" — tense conflation about single-stream wiring**

- Citation: BC-1.05.036:51 Postcondition 4 "Event is routed through `ctx.emit_internal` to the single-stream `FileSink` writing to `events-*.jsonl` per ADR-015 D-15.1 ... Same code path as the existing `emit_denial` call."
- Reality: "Same code path as emit_denial" is structurally accurate (both route via `ctx.emit_internal`) but the FileSink single-stream wiring itself is normative future-state under ADR-015 D-15.1 — the claim that the code path exists in the same form as `emit_denial` today implies wiring is present.
- Severity: LOW (cosmetic tense conflation; structurally accurate; borderline NITPICK).
- Status: DEFERRED per S-7.03 SHIP-AS-IS — the "same code path as emit_denial" claim is not materially misleading. The D-274 H3 documents this SKIP rationale.

## Out-of-scope-but-noted

- L-P30-001 carried deferred from pass-30 (BC-1.05.035:33 §-name vs lines mismatch from v1.22) — remains out of scope for this pass.
- v1.6 body and prior out-of-scope items per ongoing POLICY 1 constraint.

## Process-gaps

None new.

## Convergence trajectory update

Pass-1: NITPICK | Pass-2: NITPICK | Pass-3: SUBSTANTIVE | Pass-4: SUBSTANTIVE | Pass-5: SUBSTANTIVE | Pass-6: SUBSTANTIVE | Pass-7: SUBSTANTIVE | Pass-8: SUBSTANTIVE | Pass-9: NITPICK | Pass-10: NITPICK | Pass-11: SUBSTANTIVE | Pass-12: SUBSTANTIVE | Pass-13: SUBSTANTIVE | Pass-14: SUBSTANTIVE | Pass-15: SUBSTANTIVE | Pass-16: NITPICK | Pass-17: SUBSTANTIVE | Pass-18: SUBSTANTIVE | Pass-19: NITPICK | Pass-20: SUBSTANTIVE | Pass-21: SUBSTANTIVE | Pass-22: SUBSTANTIVE | Pass-23: NITPICK | Pass-24: SUBSTANTIVE | Pass-25: SUBSTANTIVE | Pass-26: NITPICK | Pass-27: SUBSTANTIVE | Pass-28: SUBSTANTIVE | Pass-29: SUBSTANTIVE | Pass-30: NITPICK | **Pass-31: SUBSTANTIVE (0H/2M/3L — ADR-013 clock RESET 0_of_3)**
