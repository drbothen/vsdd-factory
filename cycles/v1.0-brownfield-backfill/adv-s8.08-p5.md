---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.08-native-port-track-agent-start.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.08-p4.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.079.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.080.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - crates/hook-sdk/src/host.rs
  - crates/hook-sdk/src/result.rs
  - plugins/vsdd-factory/hooks/track-agent-start.sh
  - plugins/vsdd-factory/hooks-registry.toml
  - .factory/stories/S-8.03-native-port-track-agent-stop.md
input-hash: "e90faab"
traces_to: prd.md
story_id: S-8.08
pass_number: 5
story_version: "1.3"
story_input_hash: "e90faab"
pass: p5
previous_review: adv-s8.08-p4.md
target: story
target_file: .factory/stories/S-8.08-native-port-track-agent-start.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 1
findings_medium: 1
findings_low: 2
findings_nit: 0
---

# Adversarial Review Pass-5 — S-8.08 v1.3

## Finding ID Convention

Finding IDs use the format `F-S808-P5-<SEQ>`.
- `F`: Fixed prefix
- `S808`: Story identifier
- `P5`: Pass number
- `<SEQ>`: Three-digit sequence

## Part A — Pass-4 Fix Verification

| ID | Pass-4 Severity | Status | Evidence |
|----|-----------------|--------|----------|
| F-S808-P4-001 Sibling S-8.03 agent_id asymmetry justification | LOW | OPEN | No new prose added to Goal section explaining why S-8.08 emits agent_id and S-8.03 does not. The asymmetry remains unannotated. SKIP-FIX per S-7.03 NIT/LOW gate (clock advanced under p4 NITPICK_ONLY rule) — but underlying gap re-surfaces in pass-5 evidence (see F-S808-P5-001 below) |
| F-S808-P4-002 BC-7.03.079 registry line range mismatch | LOW | NOT-IN-SCOPE | Fix lives in BC-7.03.079.md (BC source-of-truth), not the story file. Story T-6 still cites lines 763-781, which I confirmed against current registry (hooks-registry.toml:763-781). BC file at 698-716 still stale — but adversary cannot adjudicate fix-in-place without writing access |
| F-S808-P4-003 AC-002a wording fixture aspirational vs T-5 reality | LOW | OPEN | AC-002a (story line 176-178) still references "a paired PreToolUse:Agent start + SubagentStop fixture using the same dispatcher session produces identical agent_id values (both sourced from the same host::session_id()) in both events." T-5 conditional fallback at lines 354-358 was added, but the AC-002a body was NOT brought into agreement with T-5 reality. This is the partial-fix regression discipline gap (S-7.01) — fix went into T-5 only, AC-002a body unchanged |

**Summary:** 0/3 closed, 2/3 OPEN, 1/3 NOT-IN-SCOPE. Pass-4 fix burst was not applied to v1.3 (changelog at story:435 only mentions pass-3 burst as v1.3 effort). Story v1.3 predates pass-4 review.

## Part B — New Findings (Pass-5)

### HIGH

#### F-S808-P5-001: WASM port adds two NEW event fields (`agent_id`, `tool_name`) not emitted by bash source — violates "behavior parity is the sole requirement" (E-8 D-2)

- **Severity:** HIGH
- **Confidence:** HIGH
- **Category:** spec-fidelity / parity-violation
- **Location:** Story Goal lines 143-146; T-3 lines 332-339; bash source `plugins/vsdd-factory/hooks/track-agent-start.sh:43-44`
- **Evidence:**
  - Bash source emits exactly: `type=agent.start hook=track-agent-start matcher=Agent subagent="$SUBAGENT" ${STORY_ID:+story_id="$STORY_ID"}` (track-agent-start.sh lines 43-44).
  - Bash emits NO `agent_id` field. Bash emits NO `tool_name` field.
  - Story Goal (line 144-146) requires WASM emission of `agent_id=<session_id>`.
  - Story T-3 emit_event call (lines 332-339) emits BOTH `agent_id` AND `tool_name` fields.
  - Story Goal (line 150) explicitly states: "Behavior parity is the sole requirement (E-8 D-2)."
  - BC-7.03.080 postcondition 1 verbatim: "Extracts `tool_input.subagent_type` and prompt; greps prompt for `S-[0-9]+\.[0-9]+` then `STORY-[0-9]+`. Emits `agent.start` with subagent and optional story_id." — the BC postcondition explicitly enumerates the emitted fields as `subagent` and `optional story_id` only.
- **Description:** The story specifies WASM emits two fields (`agent_id`, `tool_name`) that the bash source does not emit and that BC-7.03.080 does not authorize. Adding net-new fields is a behavioral change, not a port. This contradicts the story's own stated invariant (Goal line 150: "Behavior parity is the sole requirement"). The asymmetry with sibling S-8.03 (which emits no agent_id, per S-8.03 AC-003 line 181-184) confirms the addition is an unforced expansion of contract surface. Pass-4 F-001 flagged the agent_id asymmetry as LOW/pending-intent; pass-5 evidence (bash source comparison + BC postcondition 1 verbatim) elevates this to a parity violation, not just a sibling-symmetry concern.
- **Proposed Fix:** Either (a) remove `agent_id` and `tool_name` from the T-3 emit_event call and from the Goal section to restore strict bash parity per E-8 D-2; or (b) explicitly document a sanctioned scope expansion: file an E-8 epic ADR authorizing additive telemetry fields for Tier 1 ports, update BC-7.03.080 to authorize the new fields, and update sibling S-8.03 to match. Option (a) is preferred per the story's own parity invariant.
- **POLICY:** POLICY 6 (Architecture-is-source-of-truth) HIGH — bash source is the behavioral spec for parity ports; story diverges silently.

### MEDIUM

#### F-S808-P5-002: AC-002a body and T-5 fixture are out of sync (partial-fix regression — S-7.01)

- **Severity:** MEDIUM
- **Confidence:** HIGH
- **Category:** spec-coherence
- **Location:** Story AC-002a lines 176-178; T-5 lines 354-358
- **Description:** The AC-002a body asserts a cross-event equality test ("paired ... in both events"). T-5 falls back to a single-event self-equality test because S-8.03 doesn't emit agent_id. The fallback path renders the AC-002a body's wording false in practice — the bats test will only ever exercise the fallback. AC-002a body and T-5 fixture must be consistent.
- **Proposed Fix:** Rewrite AC-002a body to: "Bats test verifies: agent.start sink-file output contains an `agent_id` field whose value equals the `host::session_id()` returned by the dispatcher test-harness session context. (Cross-event consistency with agent.stop is out of scope for S-8.08; sibling S-8.03 does not emit agent_id.)"
- **POLICY:** POLICY 5 (Anchor justification + S-7.01 partial-fix regression discipline) MEDIUM.

### LOW

#### F-S808-P5-003: Capability Anchor Justification still lists "SS-04, SS-06" as CAP-022 source-of-truth, but S-8.08 itself anchors to "SS-01, SS-04, SS-07" — SS-06 cited but never defined

- **Severity:** LOW
- **Confidence:** MEDIUM
- **Category:** anchor-coherence
- **Location:** Story line 88-89
- **Evidence:**
  - Line 88-89: "CAP-022 declared Subsystems (SS-04, SS-06) are the source of truth for hook plugin framework concerns."
  - Story `subsystems:` frontmatter (line 41): `["SS-01", "SS-04", "SS-07"]` — SS-06 is not in the subsystem list.
- **Proposed Fix:** Either (a) verify CAP-022 declares SS-04, SS-06 as canonical and add a sentence: "SS-06 is canonical for CAP-022 framework concerns but not exercised by this story"; or (b) correct the disclosure if "SS-06" is a typo for SS-01 / SS-07.

#### F-S808-P5-004: Token budget table cites "BC files (~400 tokens each)" but BC files are ~125 lines each (~1.5K tokens combined, not 800)

- **Severity:** LOW
- **Confidence:** MEDIUM
- **Category:** estimate-accuracy
- **Location:** Story Token Budget table line 289
- **Description:** Token budget under-estimates BC file size. The actual BC file context cost is ~3x what the table claims. Total token budget would shift from ~8350 to ~9950 — still under the 20% gate at 200K, so non-blocking.

## Open Questions

- **OQ-A1 (carried from p4):** Does the orchestrator/PO intend the agent_id field to be a sanctioned scope expansion (telemetry enrichment beyond bash parity) or a parity violation that should be removed? F-S808-P5-001 elevates this to HIGH given the bash source comparison; resolution required before status=ready.
- **OQ-A2 (new):** Does CAP-022 in `domain-spec/capabilities.md` declare SS-06 as a canonical subsystem?

## Pass-6 Priors

- Verify F-S808-P5-001 fix outcome: either both `agent_id` and `tool_name` removed from T-3 (parity restored) OR explicit scope-expansion ADR linked from Goal section.
- Verify AC-002a body / T-5 alignment per F-S808-P5-002.
- Verify SS-06 disclosure reconciled per F-S808-P5-003.
- Re-verify pass-4 carry-overs: F-S808-P4-001 and F-S808-P4-003.
- Re-verify host::session_id() usage remains correct (no fabricated host::agent_id() regression).

## Verdict

**SUBSTANTIVE** — clock RESETS to 0_of_3.

- 0 CRITICAL, 1 HIGH, 1 MEDIUM, 2 LOW, 0 NIT.
- The HIGH (F-S808-P5-001) is a genuine novel finding made possible by fresh-context re-derivation: pass-3 escalation focused on agent_id sourcing (host::session_id() vs fabricated host::agent_id()), pass-4 noted sibling asymmetry as LOW. Pass-5's fresh re-read of bash source revealed that BOTH `agent_id` AND `tool_name` are net-new fields not present in the bash emission — a parity violation that survived 4 prior passes because no prior pass cross-checked the WASM emit field set against the bash emission directly.
- Per ADR-013, any HIGH or MEDIUM finding triggers SUBSTANTIVE verdict and clock reset. Clock advances 1_of_3 → 0_of_3.

## Trajectory

| Pass | C | H | M | L | NIT | Total |
|------|---|---|---|---|-----|-------|
| p1 | 0 | 4 | 5 | 2 | 1 | 12 |
| p2 | 0 | 2 | 5 | 2 | 0 | 9 |
| p3 | 0 | 2 | 1 | 1 | 0 | 4 |
| p4 | 0 | 0 | 0 | 3 | 0 | 3 |
| p5 | 0 | 1 | 1 | 2 | 0 | 4 |

Pass-5 breaks the monotonic decay with a HIGH finding. This is expected fresh-context behavior — adversary value increases with each pass even when prior passes converged on lower-severity findings, because new attack axes (in this case, bash-source field-set parity check) yield novel evidence.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 4 (1 HIGH, 1 MED, 2 LOW) |
| **Closures** | 0 (pass-4 LOW findings remain OPEN; pass-4 was post-v1.3) |
| **Novelty score** | 1.0 (4/4 novel attack axes) |
| **Median severity** | LOW-MED |
| **Trajectory** | 12→9→4→3→4 |
| **Verdict** | SUBSTANTIVE — clock resets to 0/3 |

**Novelty rationale:** F-S808-P5-001 (HIGH) is genuinely new — derived from cross-checking the WASM `emit_event` field set against the bash `_emit` field set, an axis not exercised in passes 1-4.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 1 |
| LOW | 2 |
| NIT | 0 |

**Overall Assessment:** SUBSTANTIVE — pass-5 surfaces a genuine HIGH finding (F-S808-P5-001) on bash-parity field-set divergence. The story currently specifies WASM emits two fields (`agent_id`, `tool_name`) absent from the bash source, contradicting the story's own "behavior parity is the sole requirement" invariant (E-8 D-2). The PO/orchestrator must adjudicate whether this is a sanctioned scope expansion (in which case explicit ADR linkage is required) or a parity violation requiring removal. Sibling S-8.03 confirms parity-only emission as the established pattern.

**Convergence:** RESET — clock 1_of_3 → 0_of_3 per ADR-013. Three additional consecutive NITPICK_ONLY passes required for convergence.

**Readiness:** NOT READY — F-S808-P5-001 (HIGH) and F-S808-P5-002 (MED) must be resolved before status=ready. The HIGH finding has architectural implications (BC-7.03.080 may need amendment if scope expansion is sanctioned) and cannot be SKIP-FIX'd.
