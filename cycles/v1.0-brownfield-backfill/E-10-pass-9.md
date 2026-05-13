---
pass: 9
date: 2026-05-13
producer: adversary
artifacts_reviewed:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md (v1.10)
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md (v1.6)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.001.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md (v1.2)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.002.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.003.md (v1.5)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.004.md (v1.5)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md (v1.4)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md (v1.5)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md (v1.4)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.009.md (v1.4)
  - .factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md (v1.4)
  - .factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md (v1.5)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.09.001.md (v1.1)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.02.002.md (v1.2)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.01.003.md (v1.2)
  - .factory/stories/S-10.02-adr015-wave1-filesink-single-stream.md (v1.4)
  - .factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md (v1.5)
  - .factory/specs/behavioral-contracts/BC-INDEX.md (v1.84)
  - .factory/specs/architecture/ARCH-INDEX.md (v1.98)
  - .factory/specs/architecture/SS-01-hook-dispatcher.md (last_amended 2026-05-08)
  - .factory/specs/architecture/SS-02-hook-sdk.md
  - .factory/specs/architecture/decisions/ADR-011-dual-hook-routing-tables.md
  - .factory/specs/architecture/decisions/ADR-008-parallel-within-tier.md
  - .factory/specs/architecture/decisions/ADR-004-toml-config.md
  - .factory/specs/domain-spec/invariants.md (v1.11)
  - .factory/specs/domain-spec/capabilities.md (v1.1)
verdict: HIGH
findings_count:
  CRITICAL: 0
  HIGH: 3
  MEDIUM: 1
  LOW: 1
  NITPICK: 0
fix_burst: D-344 (proposed)
seal_dispatch: D-345 (proposed)
engine_baseline: develop@d3ae26a5 (rc.18 + PR #124 + PR #136)
nitpick_only_counter_before: 0
nitpick_only_counter_after: 0
trend: "22→11→16→16→12→2→1→4→5"
---

# Adversarial Review — Pass 9 (E-10 spec package)

## Closure-Axis Verifications (CC / DD / EE)

**CC — D-15.x decision-number citation correctness [VERIFIED PASS]:** All ADR-015 D-15.x citations in the E-10 spec package correctly identify the target decision. BC-3.05.004 v1.5 (D-328) confirms Description line 33 is "OQ-1 (resolved in SS-03-event-emission.md)" + D-15.1, not D-15.4 (which governs trace propagation). Invariants.md DI-013 reads D-15.1 (post D-334 fix). Chain remains internally consistent.

**DD — trace_id field-name consistency [PARTIAL FAIL]:** See F-1. The DI-017 rename sweep from D-336 was INCOMPLETE — SS-01-hook-dispatcher.md retained 2 unannotated `dispatcher_trace_id` occurrences (lines 58, 90), and ADR-011 retained 1 unannotated wire-field occurrence (line 239).

**EE — BC version bumps + CHANGELOG accuracy [VERIFIED PASS]:** BC-1.11.001 v1.3 changelog correctly documents D-336 PC2 sweep. BC-INDEX line 384 row matches body. 14 BC version bumps from D-336 sweep traceable in changelogs.

## Closure-Axis Verifications (FF / GG) — pass-8 directive

**FF — DI-017 dispatcher_trace_id → trace_id rename propagation completeness [FAIL]:** Literal-shell verification surfaced 3 missed-rename sites (see F-1). Pass-8 D-336 closure narrative claimed "Architect also swept SS-01-hook-dispatcher.md (2 hits) ... ADR-011 (3 hits)" but verification shows SS-01 retains 2 additional unannotated hits (lines 58, 90) and ADR-011 retains 1 unannotated wire-field hit (line 239). The pass-8 sweep was numerically reported but content-incomplete (partial closure regression per S-7.01 partial-fix discipline).

**GG — schema_version=1 vs schema_version=2 differentiation completeness [PARTIAL FAIL]:** ARCH-INDEX line 339 is correctly differentiated. BC-3.05.004 + S-10.02 correctly cite v2. DRIFT detected at SS-01-hook-dispatcher.md (3 sites still cite REGISTRY_SCHEMA_VERSION = 1; contradicts ARCH-INDEX line 339) and ADR-004 (2 sites). Both attributable to F2 ADR-019 sibling-sweep gap; F2 work did not propagate to SS-01 + ADR-004 even though ARCH-INDEX was updated. SS-09-config-activation.md correctly cites = 2 with explicit "Corrected: REGISTRY_SCHEMA_VERSION = 2 post-ADR-019" note.

## Findings

### F-1 [HIGH] DI-017 rename sweep incomplete — SS-01-hook-dispatcher.md + ADR-011 retain unannotated `dispatcher_trace_id` (3 occurrences)

**File 1:** `.factory/specs/architecture/SS-01-hook-dispatcher.md`
**Locations:** Lines 58, 90.
**Defect:** Pass-8 D-336 narrative claimed "Architect also swept SS-01-hook-dispatcher.md (2 hits)". Verification shows pass-8 swept lines 37 + 132 (now properly annotated) but missed line 58 (Modules table row `crates/factory-dispatcher/src/host/context_fns.rs` exposed shims without annotation) and line 90 (Internal Structure step 1 reads "assign dispatcher_trace_id" — dispatcher-side narrative without DI-017 annotation).

**File 2:** `.factory/specs/architecture/decisions/ADR-011-dual-hook-routing-tables.md`
**Location:** Line 239 (Auto-Enrichment Fields table).
**Defect:** Table row `| dispatcher_trace_id | HostContext.dispatcher_trace_id | .with_trace_id(...) |` shows WIRE emission field name as `dispatcher_trace_id`. Per DI-017 v1.1: "the field name is exclusively `trace_id`. The legacy alias `dispatcher_trace_id` MUST NOT appear in serialized output." Directly contradicts DI-017 + ADR-015 v1.7. Pass-8 D-336 said it swept "ADR-011 (3 hits)" but line 239 is a 4th hit in the routing-table.

**Routing:** Architect (D-344 fix). SS-01 v[next], ADR-011 v[next].

**Closure proposal:**
1. SS-01 line 58: annotate `dispatcher_trace_id` → `trace_id (renamed from dispatcher_trace_id per DI-017 v1.1 / ADR-015 v1.7)` or canonicalize to `trace_id`.
2. SS-01 line 90: annotate or canonicalize "assign `dispatcher_trace_id`" → `assign trace_id`.
3. ADR-011 line 239: change table column 1 from `dispatcher_trace_id` to `trace_id` (the wire field name post-DI-017).

**[process-gap]:** Pass-8 narrative reported "swept X hits" without verifying X count equaled actual remaining-zero count. F5 cycle's D-449(a) literal-shell-execution-evidence rule applies retroactively. Brownfield fix-burst should adopt `grep -c <pattern> <file>` post-fix verification.

### F-2 [HIGH] SS-01-hook-dispatcher.md cites `REGISTRY_SCHEMA_VERSION = 1` in 3 sites — contradicts ARCH-INDEX line 339 + DI-014 v1.2

**File:** `.factory/specs/architecture/SS-01-hook-dispatcher.md`
**Locations:** Lines 49, 79, 144.
**Defect:** Three locations cite hooks-registry schema as `REGISTRY_SCHEMA_VERSION = 1`. Per F2 ADR-019 (2026-05-07) bumped this to 2. ARCH-INDEX line 339 correctly cites `hooks-registry.toml schema_version=2`. SS-01 missed during F2 sibling-sweep.

**Scope adjudication:** F2 ADR-019 sibling-sweep gap is cross-cycle, but the contradiction with ARCH-INDEX (POLICY 6 canonical) is an active spec-vs-spec defect affecting E-10's axis GG sweep verification.

**Routing:** Architect (D-344 fix; SS-01 v[next] with cross-reference to F2 ADR-019).

**Closure proposal:**
1. SS-01 line 49: `REGISTRY_SCHEMA_VERSION = 2 (post-ADR-019)`
2. SS-01 line 79: `Schema version REGISTRY_SCHEMA_VERSION = 2 (post-ADR-019; v1→v2 hard-error per BC-7.06.001)`
3. SS-01 line 144: `REGISTRY_SCHEMA_VERSION = 2 (post-ADR-019); INTERNAL_EVENT_SCHEMA_VERSION = 1; observability-config.toml schema_version=2 (post-ADR-015 D-15.1)`

**[process-gap]:** Cross-cycle propagation between concurrent F2 (ADR-019) + E-10 (ADR-015) cycles lacked integration sweep. Recommendation: when two cycles concurrently touch overlapping subsystems, second cycle's state-manager seal should re-grep the contended subsystem docs.

### F-3 [HIGH] SS-01-hook-dispatcher.md line 122 cites stale subsystem name "SS-03 (Observability Sinks)"

**File:** `.factory/specs/architecture/SS-01-hook-dispatcher.md`
**Location:** Line 122.
**Defect:** Dependencies section: `SS-03 (Observability Sinks) — sinks.rs loads SinkRegistry; submit_all fans events to sink-file, sink-otel-grpc, and future drivers.` Per ADR-015 D-15.1 (2026-05-04): SS-03 renamed to "Event Emission (OTel-Aligned)"; file renamed SS-03-observability-sinks.md → SS-03-event-emission.md. The prose `submit_all fans events to sink-file, sink-otel-grpc` is inconsistent with BC-1.12.001 + BC-1.12.007 + D-15.1 — post-Wave-1, FileSink is the SOLE writer; Router/SinkRegistry retired.

POLICY 6 violation: ARCH-INDEX line 229 is canonical subsystem-name SoT; SS-01 must align.

**Routing:** Architect (D-344 fix; SS-01 v[next]).

**Closure proposal:**
1. SS-01 line 122: `SS-03 (Event Emission (OTel-Aligned)) — host::emit_event writes through FileSink directly to events-*.jsonl (post-ADR-015 D-15.1); Router/SinkRegistry/DlqWriter retired per BC-1.12.001 + BC-1.12.007.`

### F-4 [MEDIUM] ADR-004-toml-config.md cites hooks-registry `schema_version = 1` (out of date per F2 ADR-019)

**File:** `.factory/specs/architecture/decisions/ADR-004-toml-config.md`
**Locations:** Lines 44, 96.
**Defect:** Line 44: `remains at schema_version = 1 (REGISTRY_SCHEMA_VERSION: u32 = 1 in registry.rs)`. Line 96: `hooks-registry.toml ships as TOML with schema_version = 1`. Both contradict F2 ADR-019 + ARCH-INDEX line 339 + DI-014 v1.2.

**Scope:** Strictly out of E-10 ADR-015 fix scope (ADR-004 owned by F2 ADR-019). Flagging per pass-8 axis GG broad differentiation-completeness verification request. Severity downgraded to MEDIUM because E-10 cycle does not own ADR-004.

**Routing (pending intent verification):** Architect; OR defer to F2 ADR-019 cycle re-open.

**Closure proposal (if accepted in scope):**
1. ADR-004 line 44: `bumped to schema_version = 2 (REGISTRY_SCHEMA_VERSION: u32 = 2 in registry.rs) by F2 ADR-019 (2026-05-07)`
2. ADR-004 line 96: `hooks-registry.toml ships as TOML with schema_version = 2 (post-ADR-019; bumped from 1 to 2 per F2 cycle)`

### F-5 [LOW] SS-02-hook-sdk.md cites SDK shim function as `vsdd::dispatcher_trace_id()` unannotated (3 occurrences)

**File:** `.factory/specs/architecture/SS-02-hook-sdk.md`
**Locations:** Lines 53, 91, 168.
**Defect:** SS-02 references SDK exported function as `vsdd::dispatcher_trace_id()` in 3 sites. Per DI-017 v1.1, the WIRE identifier is `trace_id`. The SDK shim function name may legitimately still be `dispatcher_trace_id()` (no F5 cycle rename of SDK API surface visible), OR may have been renamed; intent is ambiguous.

**Severity:** LOW — pending intent verification per S-7.01.

**Routing:** Architect (intent adjudication) → state-manager (if fix needed).

**Closure proposal (if fix needed):** Replace `dispatcher_trace_id` with `trace_id` at all 3 sites OR add annotation paragraph distinguishing WIRE vs SDK-fn surfaces.

## Observations

### O-1 [LOW] [carryover] BC-1.12.004 changelog row ordering inverted

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.12.004.md`
Cosmetic; not blocking.

### O-2 [info] Pass-8 D-336 narrative count vs verified count divergence

Pass-8 stated swept "14 additional BCs (~40 occurrences)". Verification confirms BC scope accurate but 40-occurrence count not exhaustively re-derived. **Recommendation:** future seal-bursts should embed `grep -c '<pattern>' <files>` results in closure record.

### O-3 [info] F5 cycle has been touching E-10 BCs during 2026-05-09+ amendments

BC-1.12.004 v1.5 dated 2026-05-09 (post-E-10-pause-at-D-343 2026-05-07) — F5 cycle's cross-doc propagation work has been touching E-10 BCs. No defect, documentary observation.

## Novelty Assessment

**Novelty: 7/10** — pass-9 surfaces a NEW class of finding (partial-fix verification gap from pass-8 closure narrative). The DI-017 rename sweep was claimed complete at pass-8 but is structurally incomplete (3 sites missed across SS-01 + ADR-011). The schema_version differentiation gap (F-2, F-4) is a cross-cycle propagation defect.

Pattern: pass-8 closure narratives used numeric counts without mechanical verification. F5 cycle's D-449(a) literal-shell-execution-evidence rule was codified BECAUSE this kind of narrative-attestation gap occurred at META-LEVEL-24. The E-10 brownfield cycle's pass-8 fix-burst was operating UNDER the same vulnerability. **Pass-9 directly demonstrates the F5 META-LEVEL-24 lesson applies retroactively to E-10 pass-8.**

**Trend:** 22 → 11 → 16 → 16 → 12 → 2 → 1 → 4 → **5**. Oscillating; finding count grows when new audit axes (FF, GG) are applied. The 5 findings are partial-fix regressions from pass-8 axes, not novel content defects. Structurally healthy: prior axes were under-verified; pass-9's fresh-context grep verification surfaces the gap.

**ADR-013 convergence counter:** does NOT advance. Remains 0/3 NITPICK_ONLY.

## Verdict

**HIGH** — five findings (3 HIGH + 1 MED + 1 LOW). Three structurally significant within ADR-015's authoritative axes. F-4 is cross-cycle-scope (ADR-004 / F2 owns). F-5 is pending intent verification.

**Pass-10 primary axes recommendation:**
- **HH (new, from F-1):** Mechanical post-fix verification — every "swept N hits" closure narrative MUST embed `grep -c <pattern>` output equaling 0 in burst record.
- **II (new, from F-2/F-3):** Cross-cycle propagation audit — when concurrent cycles touch overlapping subsystems, pass-N+1 verification must include sibling-doc sync.
- **FF/GG re-verify:** Confirm D-344 fix burst closes F-1/F-2/F-3 exhaustively (literal-shell grep evidence per F5 D-449(a)).
- **CC/DD/EE re-verify:** Expected PASS post-D-345 seal.

## Fix-Burst Proposal Sketch (D-344) — NOT EXECUTED, only proposed

**Architect (primary, F-1/F-2/F-3 fixes):**
- SS-01-hook-dispatcher.md v[next]: lines 49, 58, 79, 90, 122, 144 fixes per Closure proposals above
- ADR-011-dual-hook-routing-tables.md v[next]: line 239 fix
- (Optional, F-4) ADR-004-toml-config.md v[next]: lines 44, 96 fixes

**Architect (F-5 intent adjudication):**
- SS-02-hook-sdk.md SDK API surface — keep `vsdd::dispatcher_trace_id()` (with annotation) OR canonicalize.

**State-manager (seal D-345):**
- ARCH-INDEX v[next]: changelog row citing D-344 + post-fix grep result
- BC-INDEX cite-refresh
- Closure with literal-shell-execution evidence per F5 D-449(a) discipline applied to brownfield: `grep -rn 'dispatcher_trace_id' .factory/specs/architecture/SS-01-hook-dispatcher.md .factory/specs/architecture/decisions/ADR-011-dual-hook-routing-tables.md | grep -v 'renamed from\|changelog'` MUST return zero rows post-fix.

---

**Pass-9 produced 5 findings (3 HIGH + 1 MED + 1 LOW). NITPICK_ONLY counter stays at 0/3. Convergence requires three consecutive NITPICK_ONLY passes per BC-5.39.001 / ADR-013.**
