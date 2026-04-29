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
input-hash: "0b97a0a"
traces_to: ".factory/specs/prd.md"
pass: 2
previous_review: ADV-S5.03-P01.md
verdict: CLOCK_RESET
convergence_step: 0_of_3
findings_count:
  CRIT: 3
  HIGH: 7
  OBS: 5
  total: 15
---

# ADV-S5.03-P02 — Pass-2 Adversarial Review for S-5.03 (WorktreeCreate/WorktreeRemove)

## Finding ID Convention

Pass-2 findings use severity-prefixed IDs: `CRIT-P02-NNN`, `HIGH-P02-NNN`, `OBS-P02-NNN`.

## Part B — New Findings (15 total: 3 CRIT, 7 HIGH, 5 OBS)

### CRITICAL

#### CRIT-P02-001: BC-4.07.001/.002 internal contradiction — 4+3+1 split vs 4+4 rollup
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** BC-4.07.001 (Description §RESERVED_FIELDS + rollup totals), BC-4.07.002
- **Description:** The pass-1 HIGH-003 fix introduced a 4+3+1 sub-group split in the RESERVED_FIELDS prose, but the rollup "Wire payload: 10 fields (2 plugin-set + 4 host-enriched + 4 construction-time)" retained the old 4+4 two-bucket framing. The same BC body stated both groupings within 20 lines — an internal logical contradiction. POLICY 4 violation.
- **Evidence:** BC-4.07.001 Description: "sub-groups (a) 4 host-enriched, (b) 3 InternalEvent::now(), (c) 1 construction-time" then immediately below: "4 host-enriched + 4 construction-time" rollup. The two framings are mutually exclusive.
- **Proposed Fix:** Revert HIGH-003 in full. Restore the original 4+4 two-bucket framing ("4 host-enriched + 4 construction-time") which matches BC-4.04.001 and BC-4.05.001. The 4-vs-3 distinction is an emit_event.rs implementation detail not surfaced in HOST_ABI.md; it adds complexity without spec value.

#### CRIT-P02-002: once-key absence not propagated to EC-001 + test vector
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** BC-4.07.001 EC-001 + test vector row 4; BC-4.07.002 EC-001
- **Description:** The pass-1 HIGH-002 fix pinned "once key ABSENT" in VP-067 and BC-4.07.003 PC-4, but BC-4.07.001 EC-001 and BC-4.07.002 EC-001 still cited "`once: false` (or absent)" — the softer pre-fix language. The three sibling artifacts are now inconsistent on the authoritative absence pin. POLICY 4 violation.
- **Evidence:** BC-4.07.003 PC-4 (pass-1 fixed): "`once` key ABSENT". BC-4.07.001 EC-001 (unfixed): "`once: false` (or absent)". Semantically different: `once: false` is a present key with false value; absence is no key at all.
- **Proposed Fix:** Replace "`once: false` (or absent)" with "`once` key ABSENT" in BC-4.07.001 EC-001, BC-4.07.001 test vector row 4, and BC-4.07.002 EC-001. Aligns with BC-4.07.003 PC-4 and VP-067 exactly.

#### CRIT-P02-003: HIGH-003 sibling-sweep narrative was materially false
- **Severity:** CRITICAL
- **Category:** spec-fidelity
- **Location:** BC-4.04.001/.003/.005 (pass-1 changelog narrative); BC-4.05.001/.005 (pass-1 changelog narrative)
- **Description:** The pass-1 fix burst added changelog narratives to BC-4.04.001 and BC-4.05.001 stating they had been updated to adopt the 4+3+1 RESERVED_FIELDS split. But those BCs still contain the 4+4 grouping in their body text — the narratives misrepresented coverage. Readers reading the changelog would believe a change was applied that was not.
- **Evidence:** BC-4.04.001 body (post-pass-1): still shows "4 host-enriched + 4 construction-time". BC-4.04.001 changelog (pass-1): claims HIGH-003 sibling sweep applied. Direct contradiction.
- **Proposed Fix:** REVERSAL of HIGH-003 (closing CRIT-P02-001 simultaneously). With the revert, the 4+4 grouping becomes canonical again — sibling BCs are now correct by definition. Append clarifying narrative to affected sibling changelogs noting the HIGH-003 reversal.

### HIGH

#### HIGH-P02-001: BC-INDEX line 252 (BC-4.05.001) title has wrong fields
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** .factory/specs/behavioral-contracts/BC-INDEX.md line 252
- **Description:** BC-4.05.001 title in BC-INDEX read "event, plugin, timeout_ms" — those are BC-4.05.005 registry-hook fields, not BC-4.05.001 plugin-set fields. BC-4.05.001 H1 is about plugin-set hook registration, not hook-registry fields.
- **Evidence:** BC-4.05.001 H1: "Plugin-Set Hook Registration — well-formed hook-plugin payload accepted and routed". BC-INDEX line 252: incorrect title with BC-4.05.005 field names.
- **Proposed Fix:** Correct BC-INDEX line 252 title to match BC-4.05.001 H1 verbatim.

#### HIGH-P02-002: BC-INDEX line 259 (BC-4.07.003) still says "once:false"
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** .factory/specs/behavioral-contracts/BC-INDEX.md line 259
- **Description:** BC-INDEX entry for BC-4.07.003 retained "once:false" in the title description after BC-4.07.003 H1 was updated to "once key ABSENT" in the pass-1 burst. POLICY 9 violation: BC-INDEX must reflect current H1 titles.
- **Evidence:** BC-4.07.003 H1 (post-pass-1): "once key ABSENT". BC-INDEX line 259: still "once:false".
- **Proposed Fix:** Update BC-INDEX line 259 to match BC-4.07.003 H1 verbatim.

#### HIGH-P02-003: Story BC table descriptions use old 4+4 grouping vs new 4+3+1 prose
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-5.03 BC table (behavioral contracts section)
- **Description:** After pass-1 updated the BCs to 4+3+1 in prose, the story body BC table descriptions still used the old "4 host-enriched + 4 construction-time" two-bucket framing. Story and BC prose now inconsistent.
- **Evidence:** S-5.03 BC table row for BC-4.07.001: "4+4 construction-time". BC-4.07.001 body (post-pass-1): 4+3+1 sub-groups.
- **Proposed Fix:** Revert story BC table descriptions to 4+4 framing — this flows naturally from the CRIT-P02-001 fix (revert HIGH-003 everywhere).

#### HIGH-P02-004: input-hash 0b97a0a stale across 5 derivative artifacts
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-4.07.001, BC-4.07.002, BC-4.07.003, BC-4.07.004, VP-067 (frontmatter input-hash)
- **Description:** All 5 artifacts still carry input-hash `0b97a0a`, which predates the pass-1 fix burst changes. After the pass-1 edits, input-hash must be regenerated for each artifact whose body changed.
- **Evidence:** BC-4.07.001 frontmatter: `input-hash: "0b97a0a"`. This hash was assigned at foundation burst; pass-1 body changes invalidate it.
- **Proposed Fix:** Regenerate input-hash for all 5 artifacts. Expected: BC-4.07.001–004 → `4553104`; VP-067 → `ea362a9`.

#### HIGH-P02-005: HOST_ABI.md cited as authoritative for 4-vs-3 split but doesn't distinguish them
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-4.07.001 rationale / HIGH-003 fix prose
- **Description:** The pass-1 fix narrative stated "per HOST_ABI.md §emit_event (authoritative production contract)" as justification for the 4+3+1 split. But HOST_ABI.md §emit_event describes the combined 8-field RESERVED_FIELDS block; it does not split HostContext-enriched vs InternalEvent::now() fields into separate sub-groups. The citation-anchor is false.
- **Evidence:** HOST_ABI.md §emit_event: lists 8 RESERVED_FIELDS without HostContext vs InternalEvent::now() sub-grouping distinction. The 3-vs-1 distinction comes from emit_event.rs source code, not from HOST_ABI.md.
- **Proposed Fix:** Closed by HIGH-003 REVERSAL. With 4+4 opaque bucket restored, the false citation-anchor is eliminated.

#### HIGH-P02-006: Token Budget incomplete — hooks-registry.toml + hooks.json.template rows missing
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** S-5.03 Token Budget section
- **Description:** Token Budget table added in pass-1 fix is missing rows for hooks-registry.toml and hooks.json.template artifacts. Compared to S-5.02 format which includes all file-type rows, S-5.03 Token Budget is incomplete. Total estimate understated.
- **Evidence:** S-5.02 Token Budget: includes hooks-registry.toml row (300 tokens) + hooks.json.template row (300 tokens). S-5.03 Token Budget: no such rows. Estimated total: 6300; corrected: 6600.
- **Proposed Fix:** Add hooks-registry.toml row (~300 tokens) and hooks.json.template row (~300 tokens) to Token Budget table. Recalculate: 6300 → 6600; usage 3.2% → 3.3%.

#### HIGH-P02-007: Story BC table titles don't match BC H1s (POLICY 7 violation)
- **Severity:** HIGH
- **Category:** policy
- **Location:** S-5.03 BC table (behavioral contracts section)
- **Description:** POLICY 7 requires story BC table title column to match BC H1 verbatim. Several rows in the S-5.03 BC table used summary titles that diverged from the exact BC H1 text after pass-1 edits updated those H1s.
- **Evidence:** POLICY 7: "BC table title column MUST match BC H1 verbatim." S-5.03 BC table row for BC-4.07.003: summary title not matching updated H1.
- **Proposed Fix:** Sync all S-5.03 BC table title column entries to match current BC H1s verbatim.

### Observations

#### OBS-P02-001: ARCH-INDEX SS-04 count drift (23 vs BC-INDEX 27)
- **Severity:** OBS
- **Category:** bookkeeping
- **Location:** .factory/specs/architecture/ARCH-INDEX.md (SS-04 row)
- **Description:** ARCH-INDEX SS-04 row stated 23 BCs; BC-INDEX shows 27 BCs in SS-04 (BC-4.01 through BC-4.07 with all sub-IDs). Count drift of 4. Non-blocking but creates reader confusion.
- **Proposed Fix:** Update ARCH-INDEX SS-04 BC count from 23 to 27. Closed: PO updated ARCH-INDEX.

#### OBS-P02-002 [process-gap]: Sibling-sweep changelog discipline
- **Severity:** OBS
- **Category:** process
- **Description:** The CRIT-P02-003 finding class (changelog narrative misrepresenting actual body state) arose because the sibling-sweep author recorded the intent to apply a change rather than verifying the change was applied. Recommendation: sibling-sweep commits must diff the body, not just add a changelog row.
- **Proposed Fix:** Recorded in sidecar-learning.md as process discipline guidance.

#### OBS-P02-003: BC-4.07.003/.004 body unchanged by pass-1 (input-hash update only)
- **Severity:** OBS
- **Category:** bookkeeping
- **Description:** BC-4.07.003 and BC-4.07.004 had no substantive body changes in pass-1; only input-hash updates are required. Confirms scope of pass-2 fix is limited to BC-4.07.001/.002 for the CRIT-P02-002 once-absence propagation.
- **Proposed Fix:** No fix required for body content. input-hash regen sufficient.

#### OBS-P02-004: VP-067 once-absence fix from pass-1 remains correct
- **Severity:** OBS
- **Category:** verification-gaps
- **Description:** VP-067's `assert!(entry.get("once").is_none())` fix from pass-1 HIGH-002 is correct and consistent with BC-4.07.003 PC-4 post-revert. No regression from the CRIT-P02-001 revert on VP-067.
- **Proposed Fix:** No fix required. VP-067 body is correct.

#### OBS-P02-005: Pass-3 risk assessment
- **Severity:** OBS
- **Category:** convergence
- **Description:** After the pass-2 fix burst, the main remaining risk is minor changelog formatting consistency. The architectural decision (4+4 canonical) is now consistent across all 9 affected BCs. Story-writer noted no open mechanical gaps remain.
- **Proposed Fix:** No fix required. Informational only.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 3 |
| HIGH | 7 |
| MEDIUM | 0 |
| LOW | 0 |
| OBS | 5 |

**Overall Assessment:** block
**Convergence:** CLOCK_RESET — 3 CRIT findings block convergence. Counter resets to 0 of 3.
**Readiness:** requires revision

## Fix Burst Outcome

PO fix burst: 12 files modified (BC reverts + once-absence propagation to BC-4.07.001/.002 EC-001 + test vector + BC-INDEX/ARCH-INDEX fixes + input-hash regen for 5 artifacts + sibling-changelog narrative clarifications).
Story-writer follow-up: 1 file (story body 4+4 revert + Token Budget hooks rows added + BC table titles synced to H1s + pass-2 reversal footnote + v2.2 Changelog row).
Net: HIGH-003 reverted (pass-1 was a wrong direction); 4+4 grouping is canonical across SS-04 hook-plugin BCs (BC-4.04.*, BC-4.05.*, BC-4.07.*).
Convergence step: 0 of 3 (reset). Pass-3 risk: LOW.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 15 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 |
| **Median severity** | HIGH |
| **Severity distribution** | 3 CRIT, 7 HIGH, 0 MED, 0 LOW, 5 OBS |
| **Trajectory** | 14 → 15 (CLOCK_RESET; pass-1 fix introduced new contradictions) |
| **Verdict** | CLOCK_RESET |
