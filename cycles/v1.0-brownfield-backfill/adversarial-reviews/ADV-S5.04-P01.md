---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-29T00:00:00Z
phase: 5
inputs:
  - .factory/stories/S-5.04-post-tool-use-failure.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.001.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.002.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md
  - .factory/specs/verification-properties/VP-068.md
  - .factory/specs/verification-properties/VP-INDEX.md
input-hash: "6799477"
traces_to: ".factory/specs/prd.md"
pass: 1
previous_review: null
verdict: CLOCK_RESET
convergence_step: 0_of_3
findings_count:
  CRIT: 3
  HIGH: 6
  OBS: 7
  total: 16
---

# ADV-S5.04-P01 — Pass-1 Adversarial Review for S-5.04 (PostToolUseFailure)

## Finding ID Convention

Pass-1 findings use severity-prefixed IDs: `CRIT-P01-NNN`, `HIGH-P01-NNN`, `OBS-P01-NNN`.

## Part B — New Findings (16 total: 3 CRIT, 6 HIGH, 7 OBS)

### CRITICAL

#### CRIT-P01-001: Phantom citation — "legacy story v1.2 line 78 listed session_id as plugin-set"
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** BC-4.08.001 (Description, Invariant 8, Related BCs), VP-068 (Property Statement §1 + Notes), PRD line 460, S-5.04 (Task 3 + body prose). 9+ locations total.
- **Description:** Multiple BC bodies, VP-068, PRD, and story cited "legacy story v1.2 line 78 listed session_id as plugin-set field" as justification for the RESERVED_FIELDS scoping decision. The legacy story file is 77 lines and never mentions session_id. The citation is fabricated. The PO scoping decision itself (session_id is RESERVED, host-enriched) is correct; the supporting evidence was invented.
- **Evidence:** Legacy file `.factory/stories/v1.0-legacy/S-5.4-post-tool-use-failure.md` is 77 lines. Line 78 does not exist. No occurrence of "session_id" appears in the legacy file. Phantom citation pattern matches CRIT-P01-001 class from prior adversarial passes.
- **Proposed Fix:** Strip all "legacy story v1.2 line 78 listed…" phrases. Replace with positive framing: "session_id is RESERVED_FIELDS host-enriched per BC-1.05.012; plugin MUST NOT set it." Apply across all 9+ locations. No authoritative-correction framing needed — positive statement only.

#### CRIT-P01-002: Legacy fields is_interrupt and tool_use_id silently dropped without v1.1 candidates
- **Severity:** CRITICAL
- **Category:** missing-story
- **Location:** BC-4.08.001 (Related BCs section), S-5.04 (scope section)
- **Description:** The PostToolUseFailure envelope in the Claude Code SDK carries `is_interrupt` (bool — distinguishes user-interrupt from genuine tool error) and `tool_use_id` (string — correlation ID for joining tool.error events to tool invocation logs). Neither field is emitted by the v1.0 plugin, and neither was documented as a deliberate deferral with a v1.1 candidate entry. Silent omission violates BC process — fields present in the envelope but not emitted must be tracked as v1.1 candidates.
- **Evidence:** SDK HookPayload schema for PostToolUseFailure: `is_interrupt: bool`, `tool_use_id: Option<String>`. BC-4.08.001 Related BCs lists no v1.1 candidates for these fields. S-5.04 scope section does not mention deferral.
- **Proposed Fix:** Add BC-4.08.005 (v1.1 candidate — `tool-failure-is-interrupt-field`) and BC-4.08.006 (v1.1 candidate — `tool-failure-tool-use-id-correlation`) as Related BCs in BC-4.08.001. Add brief deferral justification in the story scope section.

#### CRIT-P01-003: error_message truncation limit 1000→2000 chars unjustified regression
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** BC-4.08.001 (Description, Postcondition 2, Invariant 4, EC-001, EC-005, Canonical Test Vectors), VP-068 (all truncation test assertions)
- **Description:** The foundation burst set error_message truncation to 2000 characters. Legacy intent (established across S-5.01+S-5.02+S-5.03) is 1000 characters, consistent with sink stream budget constraints and the canonical truncation limit used in all sibling BCs. No PO scoping decision document justifies the 2000-char uplift. Regression from established baseline.
- **Evidence:** BC-4.07.001 (S-5.03 sibling): 1000 chars. BC-4.05.001 (S-5.02 sibling): 1000 chars. BC-4.04.001 (S-5.01 sibling): 1000 chars. BC-4.08.001 v1.0: 2000 chars (diverges from all siblings). No ADR or scoping note authorizes 2000.
- **Proposed Fix:** Revert all 2000-char occurrences to 1000 in BC-4.08.001 body + all VP-068 test assertions. Rename VP-068 test from `test_bc_4_08_001_error_message_truncated_at_2000_chars` to `test_bc_4_08_001_error_message_truncated_at_1000_chars`. Input vector: use 1500-char string (truncated to 1000).

### HIGH

#### HIGH-P01-001: CAP-013 vs CAP-002 capability anchor inconsistency
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-4.08.001 (frontmatter capability, Traceability), BC-4.08.002 (same), BC-4.08.003 (same), S-5.04 (frontmatter capabilities[])
- **Description:** All three BC-4.08.* files and the S-5.04 story referenced CAP-013 as the capability anchor. PRD FR-046 declares CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") as the Source CAP for the PostToolUseFailure event family. Siblings BC-4.04.*/BC-4.05.*/BC-4.07.* all anchor CAP-002 consistently. PostToolUseFailure is a lifecycle hook event — it belongs to the CAP-002 family, not CAP-013 (which covers the PostToolUse success path, a separate event type out of S-5.04 scope).
- **Evidence:** PRD FR-046 Source CAP: CAP-002. BC-4.07.001 frontmatter: `capability: "CAP-002"`. BC-4.05.001 frontmatter: `capability: "CAP-002"`. BC-4.08.001 v1.0 frontmatter: `capability: "CAP-013"` (wrong). S-5.04 v2.0 frontmatter: `capabilities: ["CAP-013"]` (wrong).
- **Proposed Fix:** Change all three BC-4.08.* frontmatter `capability:` fields and Traceability `L2 Capability` rows from CAP-013 → CAP-002. Change S-5.04 frontmatter `capabilities:` from `["CAP-013"]` → `["CAP-002"]`. Update Capability Anchor Justification to positive CAP-002 statement.

#### HIGH-P01-002: BC-1.02.005 mis-anchor for session_id "unknown" sentinel
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-4.08.001 EC-006, BC-4.07.001 EC-005, BC-1.05.012 (line 55 area)
- **Description:** EC-006 in BC-4.08.001 (and EC-005 in BC-4.07.001) cited "BC-1.02.005 for session_id unknown sentinel handling." BC-1.02.005 governs `tool_name=""` default when absent — it is the tool_name dispatcher-envelope-parsing BC, not a session_id sentinel BC. The session_id field is host-enriched by the emit_event host fn from HostContext per BC-1.05.012; the sentinel value (if any) when HostContext.session_id is empty is a host fn internal implementation detail, not specified at BC-1.02.005.
- **Evidence:** BC-INDEX: BC-1.02.005 title = "dispatcher envelope parsing — tool_name empty string default". BC-1.05.012 title = "emit_event host fn unconditionally enriches session_id from HostContext". session_id sentinel handling belongs to BC-1.05.012 scope or a new BC-1.02.NNN, not BC-1.02.005.
- **Proposed Fix:** Remove BC-1.02.005 citation from EC-006 (BC-4.08.001) and EC-005 (BC-4.07.001). Replace with positive statement: "session_id is RESERVED_FIELDS host-enriched by the emit_event host fn from HostContext per BC-1.05.012." Sibling sweep into BC-1.05.012 body for any matching mis-citation.

#### HIGH-P01-003: VP-068 source_bc singleton vs bcs[] array semantics unclear
- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** VP-068 frontmatter (source_bc field)
- **Description:** VP-068 frontmatter uses `source_bc: BC-4.08.001` (singleton string), but the property covers postconditions from all three BC-4.08.* files. VP template allows either singleton or array. The singleton implies VP-068 only traces to BC-4.08.001, leaving BC-4.08.002 and BC-4.08.003 without VP coverage traceability. Process-gap; the coverage itself is present in the test harness body.
- **Evidence:** VP-068 frontmatter: `source_bc: BC-4.08.001`. VP body explicitly tests BC-4.08.002 and BC-4.08.003 postconditions. VP-067 sibling uses `source_bcs: [BC-4.07.001, BC-4.07.002, BC-4.07.003, BC-4.07.004]` (array form). Inconsistency.
- **Proposed Fix:** Change VP-068 frontmatter `source_bc: BC-4.08.001` → `source_bcs: [BC-4.08.001, BC-4.08.002, BC-4.08.003]` to match VP-067 pattern and make multi-BC coverage explicit. Defensible process-gap; mark as LOW-RISK if PO prefers singleton.

#### HIGH-P01-004: PRD line 460 inherits phantom citation from CRIT-P01-001
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** .factory/specs/prd.md line 460
- **Description:** PRD line 460 contains the phantom "legacy story v1.2 line 78" citation carried from BC-4.08.001 into the PRD narrative. Covered by the CRIT-P01-001 fix but listed separately because PRD is a separate file requiring explicit sweep.
- **Evidence:** prd.md line 460: contains "legacy story v1.2, line 78 listed session_id" substring (confirmed by grep).
- **Proposed Fix:** Strip phantom citation from prd.md line 460. Apply same positive-statement replacement as CRIT-P01-001.

#### HIGH-P01-005: Tier F vs Tier G inconsistency in S-5.04
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-5.04 frontmatter, S-5.04 story body (Tier line)
- **Description:** S-5.04 frontmatter had `tier: G` (or similar) while the story body stated `**Tier:** F`. Tier F is the correct value — PostToolUseFailure wiring is the same tier as WorktreeCreate/WorktreeRemove wiring (S-5.03, Tier F). Tier G would imply a higher engineering complexity tier without justification.
- **Evidence:** S-5.03 story: `**Tier:** F`. S-5.04 v2.0: frontmatter tier inconsistent with body Tier F line. Adjudicated Tier F by PO.
- **Proposed Fix:** Align frontmatter and body to Tier F. Remove any Tier G references from story body (not from changelog historical rows).

#### HIGH-P01-006: Missing platform variant verification in Task 6 / VP-068
- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** S-5.04 Task 6, VP-068 test harness skeleton
- **Description:** S-5.03 PR-cycle-1 lesson (documented in lessons.md) states: "always regenerate all 5 platform variant hooks.json files after hooks.json.template edit; verify all 5 variants contain the new entry." S-5.04 Task 6 (hooks.json.template update) did not include a verification step for per-platform variants. VP-068 test harness skeleton had no test asserting all 5 variants carry the PostToolUseFailure key. Lesson incomplete.
- **Evidence:** S-5.03 lessons.md PR-cycle-1: platform variant regeneration lesson. VP-067: `test_bc_4_07_003_platform_variants_in_sync` tests all 5 variants. VP-068 v1.0: no equivalent test present. S-5.04 Task 6: no `scripts/generate-hooks-json.sh` step listed.
- **Proposed Fix:** Add Task 6b to S-5.04: "Run `scripts/generate-hooks-json.sh`; verify all 5 platform variant files contain `PostToolUseFailure` key." Add `test_bc_4_08_002b_platform_variants_in_sync` skeleton to VP-068 test harness.

### Observations

#### OBS-P01-001: session_id "unknown" sentinel asymmetry across siblings
tool_name uses "unknown" sentinel (documented in EC-002). session_id has no sentinel specification. Asymmetry is defensible (session_id is host-enriched, not plugin-set) but worth noting for completeness. No fix required.

#### OBS-P01-002: 4+4 RESERVED_FIELDS grouping (opaque) vs 4+3+1 HOST_ABI split
BC-4.08.* uses 4+4 opaque RESERVED_FIELDS grouping (host-enriched + construction-time) which is the spec-layer convention applied consistently across S-5.01–S-5.04. HOST_ABI.md uses 4+3+1 internally. The opaque grouping is intentional and defensible per PO scoping. No fix.

#### OBS-P01-003: Dual CountingMock pattern in VP-068 test harness
VP-068 uses two CountingMock instances (one for exec_subprocess, one for emit_event). VP-067 uses the same pattern. Consistent. No finding.

#### OBS-P01-004: BC-INDEX count math
BC-INDEX `total_bcs` should be verified after BC-4.08.001–003 addition. Likely needs bump from prior count. Low-risk if PO swept during foundation burst. Informational.

#### OBS-P01-005: VP-INDEX status field for VP-068
VP-INDEX entry for VP-068 status should be `active` post-foundation-burst. Verify frontmatter. Informational.

#### OBS-P01-006: production-schema field-name compliance
BC-4.08.003 documents `timeout_ms` (not `epoch_budget_ms`, not `timeout`) consistent with S-5.01 pass-1 lesson. Pattern correctly applied. No finding.

#### OBS-P01-007: 4+4 grouping note in BC-4.08.001 Description matches postconditions
The 4+4 grouping note in the Description is consistent with the Postconditions body. No finding; noted for future adversary reference.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 3 |
| HIGH | 6 |
| MEDIUM | 0 |
| LOW | 0 |
| OBS | 7 |

**Overall Assessment:** block
**Convergence:** CLOCK_RESET — 3 CRIT findings block convergence. Counter resets to 0 of 3.
**Readiness:** requires revision

## Fix Burst Outcome

PO scope (6 files): BC-4.08.001 v1.1 + BC-4.08.002 v1.1 + BC-4.08.003 v1.1 + VP-068 v1.1; sibling sweep BC-4.07.001 v1.3 + BC-1.05.012 v1.1; PRD phantom citation strip line 460. Changes: capability anchor CAP-013 → CAP-002 (3 BCs); 1000-char truncation revert (2000→1000 throughout BC-4.08.001 + VP-068); phantom citation strip 9+ locations; v1.1 candidates BC-4.08.005 + BC-4.08.006 added to BC-4.08.001 Related BCs; EC-006 BC-1.02.005 mis-citation removed.

Story-writer scope: S-5.04 v2.0 → v2.1. Changes: Tier G → Tier F; CAP-013 → CAP-002; 1000-char truncation aligned; phantom citation stripped (4 story-body locations); v1.1 BC candidate scope note added; Task 6b platform variant verification step added; Changelog updated; input-hash regenerated.

State-manager scope: pre-commit cleanup scan caught residual "2000 characters" in BC-4.08.001 Description body (PO missed during description rewrite) — fixed. Capability Anchor Justification CAP-013 contextual clause removed from all 3 BCs — simplified to positive CAP-002 statement only.

Convergence step: 0_of_3 (reset).
Pass-2 expectation: LOW risk — all 3 CRIT + 6 HIGH addressed; cleanup scan covered residuals. Expect NITPICK_ONLY = 1_of_3 at pass 2.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 16 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | HIGH |
| **Severity distribution** | 3 CRIT, 6 HIGH, 0 MED, 0 LOW, 7 OBS |
| **Trajectory** | starting baseline (16) |
| **Verdict** | CLOCK_RESET |
