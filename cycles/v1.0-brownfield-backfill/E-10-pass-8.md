---
pass: 8
date: 2026-05-06
producer: adversary
artifacts_reviewed:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md (v1.9)
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md (v1.6)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.001.md (v1.2 → v1.3 post-fix)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md (v1.2)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.002.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.003.md (v1.4)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.004.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md (v1.5)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md (v1.4)
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.009.md (v1.4)
  - .factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md (v1.4)
  - .factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md (v1.5)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.09.001.md (v1.1)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.02.002.md (v1.2)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.01.003.md (v1.2)
  - .factory/stories/S-10.02-adr015-wave1-filesink-single-stream.md (v1.4)
  - .factory/stories/S-10.03-adr015-wave1-resource-attribute-enrichment.md (v1.4)
  - .factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md (v1.5)
  - .factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md (v1.4 → v1.5 post-fix)
  - .factory/stories/S-10.09-adr015-wave5-crate-retirement-ss03-rewrite.md (v1.3)
  - .factory/specs/behavioral-contracts/BC-INDEX.md (v1.13 → v1.14 post-seal)
  - .factory/specs/architecture/ARCH-INDEX.md (v1.6 → v1.7 post-fix)
  - .factory/specs/domain-spec/capabilities.md (v1.3)
  - .factory/specs/domain-spec/invariants.md (v1.2 post-D-334)
verdict: HIGH
findings_count:
  HIGH: 4
  MED: 0
  LOW: 0
fix_burst: D-336
seal_dispatch: D-337
post_seal_sha: "(D-337 commit — see factory-artifacts HEAD)"
engine_baseline: v1.0.0-rc.12 @ 4cf59bc
nitpick_only_counter_before: 0
nitpick_only_counter_after: 0
---

# Adversarial Review — Pass 8 (E-10 spec package)

## Closure-Axis Verifications (CC / DD / EE)

**CC — D-15.x decision-number citation correctness [VERIFIED PASS]:** All ADR-015 D-15.x citations in the spec package correctly identify the target decision. No D-15.4/D-15.1 transposition instances found in the primary E-10 artifacts (invariants.md DI-013 was fixed by D-334 per pass-7; the fix was verified clean entering pass-8).

**DD — BC-3.05.004 PC7 ↔ DI-013 ↔ DI-014 ↔ ADR-015 D-15.1 chain [VERIFIED PASS]:** BC-3.05.004 Postcondition 7 correctly cites ADR-015 D-15.1 + OQ-1 resolution for warn-and-skip behavior. DI-013 "Refined by:" paragraph correctly reads D-15.1 (post D-334 fix). Chain is internally consistent.

**EE — Sibling-paragraph residue (D-15.4→D-15.1 pattern) [VERIFIED PASS]:** Post-D-334, no additional D-15.4 misattributions found in vicinity of D-15.1 spec references. Sibling-residue pattern closed.

## High Findings

### F-1 [HIGH] BC-1.11.001 PC2 uses `dispatcher_trace_id` — should be `trace_id` per DI-017

**File:** `.factory/specs/behavioral-contracts/ss-01/BC-1.11.001.md`
**Location:** Postcondition 2 (PC2)

**Defect:** PC2 referenced the legacy field name `dispatcher_trace_id` in the VSDD_TRACE_ID injection contract. DI-017 v1.1 and ADR-015 v1.7 canonicalized the renamed field to `trace_id`. This was the primary F-1 for pass-8.

**Routing:** PO (D-336 primary fix). Additionally: comprehensive sweep across 15 BC files revealed ~40 additional occurrences of the legacy name `dispatcher_trace_id`. All swept in D-336.

**Closure:** BC-1.11.001 v1.2→v1.3. 14 additional BCs bumped (v1.0→v1.1 or v1.1→v1.2 as applicable). D-336 sweep covered: BC-1.05.010, BC-1.05.012, BC-1.05.018, BC-1.05.033, BC-1.06.007, BC-1.06.008, BC-1.06.009, BC-1.10.001, BC-3.03.008, BC-3.05.003, BC-4.04.001, BC-4.05.001, BC-4.07.001, BC-4.07.002, BC-4.08.001.

**[process-gap]:** Second distinct rename-propagation pattern observed (first was D-15.4→D-15.1 in 4 occurrences). DI-017 dispatcher_trace_id rename affected 15 files / 40+ occurrences. Codification candidate: validate-rename-propagation.sh lint hook after 3rd distinct pattern is observed.

### F-2 [HIGH] ARCH-INDEX line 152 Cross-Cutting Concerns uses `dispatcher_trace_id`

**File:** `.factory/specs/architecture/ARCH-INDEX.md`
**Location:** Line 152, Cross-Cutting Concerns section

**Defect:** Cross-Cutting Concerns row named the field `dispatcher_trace_id` in the "renamed-from" parenthetical. Per DI-017 / ADR-015 v1.7, the canonical name is `trace_id`.

**Routing:** Architect (D-336). ARCH-INDEX 1.6→1.7.

**Closure:** ARCH-INDEX line 152 corrected to `trace_id`. Architect also swept SS-01-hook-dispatcher.md (2 hits), ADR-008 (1 hit), ADR-011 (3 hits).

### F-3 [HIGH] ARCH-INDEX line 151 schema_version undifferentiated between config types

**File:** `.factory/specs/architecture/ARCH-INDEX.md`
**Location:** Line 151, Schema versioning row

**Defect:** The schema_version row stated `schema_version=1` without differentiating between hooks-registry.toml (uses v1) and observability-config.toml (uses v2 per ADR-015 D-15.1). This was a post-ADR-015 accuracy gap.

**Routing:** Architect (D-336). Differentiated per-config in ARCH-INDEX + cited BC-3.05.004 PC4 migration hint and DI-014. Also swept ADR-004 for schema_version=1 references (2 hits amended).

**Closure:** ARCH-INDEX line 151 now reads `hooks-registry: schema_version=1; observability-config: schema_version=2 (ADR-015 D-15.1)` with SS-03 added to subsystem list.

### F-4 [HIGH] S-10.05 AC-008 missing BC-2.06.001 v1.3+v1.4 CHANGELOG content requirements

**File:** `.factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md`
**Location:** AC-008 (CHANGELOG acceptance criterion)

**Defect:** AC-008 required a CHANGELOG entry be present but did not specify the content requirements derived from BC-2.06.001 v1.3 (Breaking Changes section) and v1.4 (Added/New API section, HookResult::block_with_fix). The acceptance criterion was underspecified relative to the BC that governs it.

**Routing:** Story-writer (D-336 Option A). S-10.05 v1.4→v1.5.

**Closure:** AC-008 extended with explicit CHANGELOG content requirements: Breaking Changes section (per BC-2.06.001 v1.3 PC7) + Added/New API section documenting HookResult::block_with_fix (per BC-2.06.001 v1.4).

## Observations

### O-1 [LOW] [carryover from pass-7] BC-1.12.004 raw line citation
BC-1.12.004 body postcondition retains `main.rs:143` raw-line citation. Tracked to cleanup story #116. Not re-flagged.

## Novelty Assessment

Novelty: 5/10 — pass-8 found four findings across two distinct pattern axes: (1) the DI-017 dispatcher_trace_id rename propagation gap (F-1 primary + F-2/F-3 collateral) which represents a NEW systemic propagation gap not previously observed at this scale (15 files / 40+ occurrences), and (2) spec underspecification relative to a cited BC (F-4). The DI-017 gap is oscillating — the spec was stable for passes 1-7 and then a new audit angle (rename-propagation completeness) revealed a large accumulated gap. This is structurally different from the D-15.4→D-15.1 transposition pattern (4 instances, fixed incrementally).

Trend: pass-1 CRIT (22) → pass-2 CRIT (11) → pass-3 HIGH (16) → pass-4 HIGH (16) → pass-5 HIGH (12) → pass-6 HIGH (2) → pass-7 HIGH (1) → **pass-8 HIGH (4)**. Oscillating — finding count increased from 1 (pass-7) to 4 (pass-8). This is expected when a new systematic angle is applied.

ADR-013 convergence counter does NOT advance. Remains at 0.

## Fix-Burst Summary (D-336)

Three parallel agents dispatched:

**PO (primary):** BC-1.11.001 PC2 fixed (F-1). Comprehensive sweep: 14 additional BC files corrected (~40 `dispatcher_trace_id` → `trace_id` occurrences). lessons.md entry appended: "DI-017 dispatcher_trace_id rename propagation gap [process-gap]".

**Architect:** ARCH-INDEX 1.6→1.7 (F-2 line 152 + F-3 line 151). SS-01 1.0→1.1 (2 dispatcher_trace_id + 1 schema_version=1 fixes). ADR-008, ADR-011 amended (dispatcher_trace_id sweep). ADR-004 amended (schema_version=1 sweep + amendment block).

**Story-writer:** S-10.05 1.4→1.5 (F-4 AC-008 extended). 4 other E-10 stories swept clean (no changes needed).

## Verdict

HIGH — four HIGH-severity findings (F-1/F-2/F-3/F-4) all resolved by D-336 fix burst. D-337 seals the fix cycle. Pass-9 is next dispatch.

**Pass-9 primary axes:**
- FF (new): DI-017 dispatcher_trace_id→trace_id rename propagation completeness verification (was the D-336 sweep exhaustive?)
- GG (new): schema_version=1 vs schema_version=2 differentiation completeness (was the D-336 sweep exhaustive?)
- CC/DD/EE: Re-verify closure axes (expected PASS)
