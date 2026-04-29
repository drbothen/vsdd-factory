---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-28T00:00:00Z
phase: 5
inputs:
  - .factory/stories/S-5.03-worktree-hooks.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.002.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.003.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md
  - .factory/specs/verification-properties/VP-067.md
  - .factory/specs/verification-properties/VP-INDEX.md
input-hash: "d7a5acd"
traces_to: ".factory/specs/prd.md"
pass: 1
previous_review: null
verdict: CLOCK_RESET
convergence_step: 0_of_3
findings_count:
  CRIT: 3
  HIGH: 6
  OBS: 5
  total: 14
---

# ADV-S5.03-P01 — Pass-1 Adversarial Review for S-5.03 (WorktreeCreate/WorktreeRemove)

## Finding ID Convention

Pass-1 findings use severity-prefixed IDs: `CRIT-NNN`, `HIGH-NNN`, `OBS-NNN`.

## Part B — New Findings (14 total: 3 CRIT, 6 HIGH, 5 OBS)

### CRITICAL

#### CRIT-001: CAP-003 mis-anchor in BC-4.07.001 and S-5.03
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** BC-4.07.001 (rationale prose), S-5.03:58
- **Description:** Both files referenced "CAP-003 (filesystem-write capability) deferred to v1.1". CAP-003 is the active "Stream observability events" capability. No CAP ID exists for filesystem-write at v1.0. POLICY 4 violation.
- **Evidence:** CAP registry: CAP-003 = "Stream observability events" (P0, active). The filesystem-write capability has no allocated CAP ID at v1.0.
- **Proposed Fix:** Remove CAP-003 parenthetical. State that filesystem-write capability has no CAP ID at v1.0; provisional v1.1 name `CAP-NNN-filesystem-write` to be assigned when capability registry is expanded.

#### CRIT-002: BC-1.05.022 mis-anchor — wrong deny-by-default BC cited
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** BC-4.07.004 (Postcondition 6), S-5.02 BC-4.05.005:33,47
- **Description:** BC-1.05.022 is the `read_file` SUCCESS path (capability granted + returns data), not the deny path. Deny-by-default requires two BCs: BC-1.05.001 (exec_subprocess deny) + BC-1.05.021 (read_file deny). Sibling sweep affects S-5.02 BC-4.05.005. POLICY 4 violation.
- **Evidence:** BC-INDEX: BC-1.05.022 title = "read_file success path — capability granted, data returned". BC-1.05.021 title = "read_file deny — capability not declared". BC-1.05.001 title = "exec_subprocess deny — capability not declared".
- **Proposed Fix:** Replace BC-1.05.022 with BC-1.05.001 + BC-1.05.021 in all affected Traceability/Postcondition rows. Sibling sweep: S-5.02 BC-4.05.005.

#### CRIT-003: event_type vs event_name — wrong envelope field name
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** BC-4.07.001:102, BC-4.07.004:39+119
- **Description:** Multiple BC files used `event_type` as the HookPayload field name. The canonical field per HOST_ABI.md and BC-1.02.001/002 is `event_name`. This is a repeat of S-5.01 lesson 1 (event_type → event_name). POLICY 4 violation.
- **Evidence:** HOST_ABI.md §HookPayload: `event_name: String` (canonical). BC-1.02.001/002 both reference `event_name`. S-5.01 adversarial history lesson 1: "Always use `event_name` not `event_type`."
- **Proposed Fix:** Replace all `event_type` occurrences with `event_name` in BC-4.07.001 and BC-4.07.004 prose, rationale, and architecture notes.

### HIGH

#### HIGH-001: Stale input-hash `d7a5acd`
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-5.03:19 (frontmatter)
- **Description:** Story inputs[] expanded to 7 entries (BC-4.07.001–004 + VP-067 added) but input-hash was not regenerated from the new inputs set. Hash `d7a5acd` predates the foundation burst.
- **Evidence:** PO foundation burst commit acknowledged this as known risk #4. Hash mismatch detectable via `vsdd-factory:check-input-drift`.
- **Proposed Fix:** Regenerate input-hash from the 7 current inputs entries. Expected: `286c8bf`.

#### HIGH-002: VP-067 `once` assertion ambiguity — `assert_ne!(once, Bool(true))` insufficient
- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** VP-067 (Property Statement + Proof Harness Skeleton)
- **Description:** `assert_ne!(once, Bool(true))` passes when `once` is `false`, absent, OR string-typed. The behavioral contract requires the `once` key to be completely absent (not just non-true). `Bool(false)` and absent have different semantics under future Claude Code versions. Defensive omission pattern requires `is_none()` pin.
- **Evidence:** hooks.json.template behavioral intent: worktree events intentionally omit the `once` key entirely (vs session events that carry `once: true`). `assert_ne!` only excludes `Bool(true)` — leaves `Bool(false)` and `Null` as passing values.
- **Proposed Fix:** Replace `assert_ne!(once, Bool(true))` with `assert!(entry.get("once").is_none())` in VP-067 property statement, harness skeleton, and notes.

#### HIGH-003: RESERVED_FIELDS 4-vs-4 split contradicts HOST_ABI.md
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-4.07.001 (Description §RESERVED_FIELDS), BC-4.07.002
- **Description:** Files stated RESERVED_FIELDS = 4 host-enriched + 4 construction-time (4-vs-4 framing). HOST_ABI.md §emit_event specifies three sub-groups: (a) 4 host-enriched from HostContext, (b) 3 from InternalEvent::now(), (c) 1 at construction from type argument. Correct split: 4+3+1.
- **Evidence:** HOST_ABI.md §emit_event: sub-group (a) dispatcher_trace_id, session_id, plugin_name, plugin_version; sub-group (b) ts, ts_epoch, schema_version; sub-group (c) type. Total = 8 = 4+3+1.
- **Proposed Fix:** Update all RESERVED_FIELDS prose to reflect 4+3+1 three-sub-group split per HOST_ABI.md.

#### HIGH-004: DI-007 mis-citation — dispatcher self-telemetry scope, not plugin events
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-4.07.001 Traceability, BC-4.07.002 Traceability, BC-4.07.004 Traceability; Sibling: S-5.01 BC-4.04.001/003/005, S-5.02 BC-4.05.001/005
- **Description:** DI-007 = "Dispatcher self-telemetry is always-on" — scoped to dispatcher-internal-YYYY-MM-DD.jsonl and SS-03 internal_log.rs. It does NOT govern plugin-emitted events. Cited in 8 BC files across S-5.01+S-5.02+S-5.03.
- **Evidence:** invariants.md: DI-007 scope = "SS-03 dispatcher internal logging". Plugin-emitted events via `emit_event` host fn are not governed by DI-007 at v1.0.
- **Proposed Fix:** Remove DI-007 from all 8 affected Traceability rows. Replace with "no current DI for plugin event emission unconditionally; v1.1 candidate per PRD §S-5.03 flag." Sibling sweep mandatory.

#### HIGH-005: VP-INDEX frontmatter `total_vps: 66` not bumped
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** .factory/specs/verification-properties/VP-INDEX.md (frontmatter)
- **Description:** VP-067 was created in the foundation burst, bringing the total to 67 VPs. VP-INDEX frontmatter `total_vps` was not updated from 66 to 67. POLICY 9 violation.
- **Evidence:** VP-INDEX listing ends at VP-067. Frontmatter states `total_vps: 66`.
- **Proposed Fix:** Bump `total_vps: 66` → `total_vps: 67` in VP-INDEX frontmatter.

#### HIGH-006: Token Budget arithmetic missing 4 BCs + VP-067
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-5.03 Token Budget section
- **Description:** The Token Budget table omits line items for BC-4.07.001, BC-4.07.002, BC-4.07.003, BC-4.07.004, and VP-067. These 4 BCs + 1 VP were created in the foundation burst but not reflected in the arithmetic. Total token estimate understated.
- **Evidence:** Token Budget section row count vs. actual artifacts produced in foundation burst.
- **Proposed Fix:** Add rows for BC-4.07.001–004 and VP-067 to the Token Budget table. Recalculate totals.

### Observations

#### OBS-001: Wave-16 loading (informational)
S-5.03 is earmarked for wave-16. Wave scheduling noted but no fix required.

#### OBS-002: BC-4.06 numbering skip (intentional)
BC-4.06 subsystem block is reserved for a future plugin. The gap between BC-4.05 and BC-4.07 is intentional per PO decision. No fix.

#### OBS-003: F-8 ruling consistent
F-8 (once:false semantics) ruling from S-5.01 applied consistently to S-5.03 hooks.json.template. No finding.

#### OBS-004: v1.2 → v2.0 story version jump defensible
The v1.2 (legacy) to v2.0 (brownfield-backfill) version jump in S-5.03 is defensible per the cross-cycle versioning convention. No fix.

#### OBS-005: Process-gap — recurring DI/BC mis-anchors across S-5.01+S-5.02+S-5.03
DI-007 mis-citations and BC-1.05.022 mis-anchors recurred across 3 stories (S-5.01, S-5.02, S-5.03) and were not caught in 23+ prior passes. Recommendation: enhance adversary prompt to systematically verify "BC-NNN references in prose match BC-INDEX titles" and "DI-NNN citations match invariants.md scope description." Recorded in sidecar-learning.md.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 3 |
| HIGH | 6 |
| MEDIUM | 0 |
| LOW | 0 |
| OBS | 5 |

**Overall Assessment:** block
**Convergence:** CLOCK_RESET — 3 CRIT findings block convergence. Counter resets to 0 of 3.
**Readiness:** requires revision

## Fix Burst Outcome

All 3 CRIT + 5 of 6 HIGH closed via PO + story-writer fix burst. HIGH-006 closed via story-writer Token Budget update. Sibling sweep: 8 DI-007 removals + 3 BC-1.05.022 replacements across S-5.01+S-5.02+S-5.03. Story v2.0 → v2.1. input-hash regenerated (d7a5acd → 286c8bf). VP-INDEX total_vps bumped 66 → 67. Convergence step counter: 0 of 3 (reset). Pass-2 risk: LOW.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 14 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | HIGH |
| **Severity distribution** | 3 CRIT, 6 HIGH, 0 MED, 0 LOW, 5 OBS |
| **Trajectory** | starting baseline (14) |
| **Verdict** | CLOCK_RESET |
