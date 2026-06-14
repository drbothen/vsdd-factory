---
document_type: adversarial-review
level: ops
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.0 + S-15.17 v1.1"
cycle: brownfield-backfill
pass: 1
producer: adversary
timestamp: 2026-05-28
input-hash: "7fe95f1"
inputs:
  - .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-75.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md
  - .factory/cycles/v1.0-brownfield-backfill/decision-log.md
  - .factory/policies.yaml
  - .factory/STATE.md
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/hook-sdk/src/host.rs
  - crates/hook-plugins/validate-policies-schema/src/lib.rs
  - crates/hook-plugins/validate-state-structure/src/lib.rs
verdict: HIGH
finding_count: 14
finding_count_by_severity:
  critical: 0
  high: 5
  medium: 5
  low: 3
  nitpick: 1
  process_gap: 0
streak_status: "STREAK 0/3 after pass-1"
---

# Adversarial Review — BC-5.39.009 v1.0 + S-15.17 v1.1 Spec Cascade Pass 1

## Part A — Finding Set

### F-S15.17-SP1-001 — HIGH — `max_bytes: u64` in story T-5 pseudocode breaks compilation against `host::read_file` signature

**File:** `.factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md`
**Anchor:** Story §Tasks T-5 (line 250)
**Problem:** Story T-5 pseudocode declares `const MAX_BYTES: u64 = 524_288;`. The actual SDK signature is `pub fn read_file(path: &str, max_bytes: u32, timeout_ms: u32) -> Result<Vec<u8>, HostError>` (`crates/hook-sdk/src/host.rs:187`). The story's pseudocode line `host::read_file(file_path, MAX_BYTES, 2000)` will fail to compile with `expected u32, found u64`. Every sibling crate (`validate-policies-schema/src/lib.rs:51` `pub const MAX_BYTES: u32 = 524_288;`; `validate-state-structure/src/lib.rs:60` `pub const MAX_BYTES_STATE_MD: u32 = 524_288;`) uses `u32`. BC-5.39.009 invariant 7 cites "`max_bytes = 524288`" without a type, which is fine; the defect is in the story's executable spec.
**Evidence:**
```
$ grep -n 'pub fn read_file' crates/hook-sdk/src/host.rs
187:pub fn read_file(path: &str, max_bytes: u32, timeout_ms: u32) -> Result<Vec<u8>, HostError> {
$ grep -n 'pub const MAX_BYTES' crates/hook-plugins/validate-policies-schema/src/lib.rs
51:pub const MAX_BYTES: u32 = 524_288;
$ grep -n 'MAX_BYTES_STATE_MD' crates/hook-plugins/validate-state-structure/src/lib.rs
60:pub const MAX_BYTES_STATE_MD: u32 = 524_288;
$ grep -n 'const MAX_BYTES' .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
250:  const MAX_BYTES: u64 = 524_288;  // 512 KiB — META-LEVEL-24 prevention
```
**Routing:** story-writer
**Recommended fix:** Change `u64` to `u32` in story T-5 pseudocode. Also strongly recommend that BC-5.39.009 invariant 7 explicitly cite "`max_bytes: u32 = 524_288`" to match sibling convention and to function as a compile-time guard against this class of recurrence.

### F-S15.17-SP1-002 — HIGH — BC-5.39.009 frontmatter cites a non-existent ADR-017 file path

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md`
**Anchor:** BC frontmatter line 19 (`inputs:` array)
**Problem:** Frontmatter cites `.factory/specs/architecture/decisions/ADR-017-per-story-adversarial-convergence-gate.md`. This file does NOT exist. The actual filename is `ADR-017-per-story-adversary-phasing.md` (per ARCH-INDEX.md and a Glob confirmation). Three other narrative references in BC §Traceability use the alias-style citation "ADR-017 (Per-Story Adversarial Convergence Gate)" which is the ADR title, but the frontmatter `inputs:` array requires the actual filesystem path. This will silently break input-hash computation and any downstream tooling that resolves frontmatter input paths.
**Evidence:**
```
$ ls .factory/specs/architecture/decisions/ADR-017*
.factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md
$ sed -n '19p' .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
  - .factory/specs/architecture/decisions/ADR-017-per-story-adversarial-convergence-gate.md
```
**Routing:** product-owner
**Recommended fix:** Replace the frontmatter input path with `.factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md`. (POLICY 4 semantic_anchoring_integrity violation.)

### F-S15.17-SP1-003 — HIGH — Story ACs trace to wrong BC postcondition numbers (PC10/PC11/PC12 mis-mapping)

**File:** `.factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md`
**Anchor:** Acceptance Criteria table — AC-14 and AC-15 (lines 131-132)
**Problem:** BC postcondition numbering in the SoT BC file is: PC10 = "lessons.md trend-table missing tail → advisory" (line 228-231); PC11 = "any target file too large → Continue + log_warn" (line 235-238); PC12 = "host::read_file HostError → Continue + log_warn" (line 240-242); PC13 = "all sites present → Continue" (line 246-248). Story AC-14 says "Target file larger than MAX_BYTES → Continue + log_warn" and traces to "BC-5.39.009 postcondition 10 + invariant 7" — BUT PC10 is the lessons.md advisory, NOT file-too-large. AC-15 traces "host::read_file HostError" to PC11 — BUT PC11 is file-too-large. The traces are shifted by ONE. Additionally, AC-1 traces "A new WASM hook crate ... priority 158 ... Continue" to "BC-5.39.009 postcondition 13" (the pass case) and "invariant 1" — the AC concerns registry presence, not pass behavior; postcondition 13 anchoring is semantically wrong.
**Evidence:** (see adversary chat output for full PC-by-PC quoted evidence)
**Routing:** story-writer
**Recommended fix:** Renumber AC-14 trace to PC11; AC-15 to PC12. Re-anchor AC-1 to BC §Architecture Anchors "Hook registry entry (priority 158)" + BC invariant 1 (read-only) rather than PC13. Sweep all 21 ACs for similar off-by-one PC mis-anchors. (POLICY 4 + POLICY 8 violations.)

### F-S15.17-SP1-004 — HIGH — STATE.md section heading specs do not match actual STATE.md structure

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md`
**Anchor:** BC §Postconditions PC2 + PC5 + §Architecture Anchors (lines 169-191, 423-430)
**Problem:** BC PC2 says "Last Updated body section ... extracted by scanning for the heading or table row labeled 'Last Updated'." Verified against actual STATE.md: "Last Updated" is a TABLE ROW INSIDE the `## Project Metadata` table (line 57) — there is no `## Last Updated` heading. PC5 says "Session Resume section (specifically Section 1 content, the first numbered or labeled sub-section of the Session Resume Checkpoint)" extracted by "scanning for `## Session Resume` heading" — but actual heading is `## Session Resume Checkpoint (2026-05-28 — D-513 ...)` (line 280) with parenthetical suffix. PC3 says "scanning for a heading or label matching 'Phase Progress' and capturing until the next heading" — `## Phase Progress` (line 61) IS present, but it contains ~120 rows of historical entries spanning STATE.md compaction archives. The "latest pass row" extraction logic is not specified — what if the next-newest row is a compaction marker, not a trajectory-bearing row? PC4 "Concurrent Cycles section or row" is correct (`## Concurrent Cycles` at line 188). The hook will either Block falsely (no `## Last Updated` heading) or fail-open silently (heading not found → no section to check → cannot detect missing tail) depending on extractor behavior.
**Evidence:**
```
$ grep -nE '^## |Last Updated' .factory/STATE.md | head -20
48:## Project Metadata
57:| **Last Updated** | 2026-05-28 ... Trajectory-tail carry-across →9→9→9→11. |
61:## Phase Progress
146:## Current Phase Steps
176:## Active Branches
188:## Concurrent Cycles
198:## Decisions Log
280:## Session Resume Checkpoint (2026-05-28 — D-513 BC-5.39.009 v1.0 AUTHORED + S-15.17 v1.1 PROPAGATED; next: adversarial cascade on BC+story)
286:### §1. Where We Are
```
The story acknowledges this risk at line 684 (Risk row HIGH "STATE.md section heading format changes ...") and at line 531 (Architecture Compliance "STATE.md section heading labels must be read from ACTUAL STATE.md") — but defers the resolution to T-5 implementation. The SoT BC does not specify the actual canonical headings, which means the implementer will need to guess. This is a SOUL.md #4-class silent-failure risk: extractor that doesn't find the heading returns None → site "treated as missing tail" (per PC1 default for absent key) → BLOCK is emitted — but the BLOCK message will be confusing ("Last Updated cell missing trajectory_tail") when the actual cause is the heading does not exist.
**Routing:** product-owner (BC site definitions must be authoritative)
**Recommended fix:** PO MUST update BC postcondition 2 to specify that "Last Updated" is a table cell in the `## Project Metadata` table (not a heading); update PC5 to specify the actual heading prefix is `## Session Resume Checkpoint` (with optional parenthetical suffix); update PC3 to specify which Phase Progress row is "latest" (e.g., "the last non-archive, non-summary row in the table"). This is a HIGH because it goes to whether the hook can correctly identify sites at all. Production-grade default: do NOT defer this to implementer guesswork.

### F-S15.17-SP1-005 — HIGH — BC inv-4 LENGTH≥4 contradicts sibling BC-5.39.006 inv-6(b) LENGTH=4 strict

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md` (and BC-5.39.006.md)
**Anchor:** BC-5.39.009 invariant 4 + PC1; BC-5.39.006 invariant 6(b) + EC-007
**Problem:** BC-5.39.009 invariant 4 says trajectory_tail present "if ... at least one contiguous match" — LENGTH≥4 acceptable. Sibling BC-5.39.006 invariant 6(b) + EC-007 explicitly says LENGTH must equal exactly 4 and LENGTH=5 BLOCKS with `HookResult::block_with_fix(...)`. Two sibling BCs operating on the same canonical trajectory_tail concept disagree on the LENGTH=5 case. The story's `has_trajectory_tail` pseudocode returns true on the FIRST sequence reaching arrows >= 4, which is consistent with LENGTH≥4 semantics but inconsistent with BC-5.39.006's strict LENGTH=4 enforcement. Implementer cannot resolve this cross-BC contradiction without PO adjudication.
**Evidence:**
```
$ grep -n 'LENGTH' .factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md | head -8
107:     trailing space), and the substring after that marker contains exactly 4 `→(\d+)` matches
118:   `HookResult::block_with_fix(...)` naming the actual match count, the required count (4), and citing
174:   the LENGTH count). Absent-marker is fail-closed (block), not fail-open.
275:| EC-007 | `current_step:` contains `trajectory-tail ` marker and first-semicolon segment has 5 `→N` groups (LENGTH=5) | `HookResult::block_with_fix(...)`: "trajectory-tail has 5 components; required LENGTH=4 per D-451(c)" |
$ grep -n 'at least one\|LENGTH' .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md | head -10
154:6. The canonical trajectory_tail pattern is the regex `(→[0-9]+){4}` (LENGTH exactly 4 arrow-
155:   segments). A site is "present" if the extracted section or field text contains at least one
158:   match of this pattern anywhere in it. LENGTH=3 (three arrow-segments) is insufficient and
161:   The pattern accepts multi-digit axis values: `→9→9→9→9`, `→10→12→11→13`, etc. are all valid
269:4. **Canonical trajectory_tail pattern LENGTH=4.** The canonical arrow-sequence regex is
270:   `(→[0-9]+){4}`: exactly 4 arrow-segments, each followed by one or more decimal digits. A site
271:   is "present" if and only if the extracted text contains at least one contiguous match.
```
**Routing:** product-owner
**Recommended fix:** PO must resolve the LENGTH=4 vs LENGTH≥4 semantic divergence. Per actual STATE.md production tail (`→9→9→9→11`, LENGTH=4) and per the original D-433(e)+D-439(c) "LENGTH=4" codification, the defensible read is LENGTH=4 strict (matching BC-5.39.006). BC-5.39.009 invariant 4 + PC1 must be amended to specify "exactly 4 arrow-segments; LENGTH=5+ is also a violation". Add a test vector for LENGTH=5 analogous to BC-5.39.006 EC-007. Alternative: PO explicitly justifies why BC-5.39.009 is intentionally more permissive than BC-5.39.006.

### F-S15.17-SP1-006 — MEDIUM — Story Edge Case numbering diverges from BC Edge Case numbering

**File:** `.factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md`
**Anchor:** Story §Edge Cases EC-014..EC-019 vs BC §Edge Cases EC-014..EC-017
**Problem:** Story EC-018 + EC-019 are story-local additions for multi-line YAML. Story EC-017 is the partial-presence case, but BC EC-017 is the multi-line YAML case. AC-21 says "traces to BC-5.39.009 EC-017" — correct trace TO BC, but the story's own body EC-017 row is a different case. PO and story-writer using different EC numbering creates implementer confusion.
**Routing:** story-writer
**Recommended fix:** Renumber story ECs to match BC numbering 1:1, OR add a "BC EC" column to the story ECs table explicitly mapping each story EC to its BC EC counterpart.

### F-S15.17-SP1-007 — MEDIUM — BC Precondition 5 contradicts EC-016 on frontmatter-absent behavior

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md`
**Anchor:** BC Precondition 5 (lines 143-149) vs BC EC-016 (line 331)
**Problem:** Precondition 5 says absent frontmatter → log_warn advisory + Continue (fail-open). EC-016 says absent frontmatter → log_warn advisory + treats current_step as missing → Block for that site. Contradictory.
**Routing:** product-owner
**Recommended fix:** Reconcile to log_warn + Continue (matching sibling BC fail-open precedent). Update EC-016 accordingly.

### F-S15.17-SP1-008 — MEDIUM — Story BC Table claim "invariants 1-11" exercised inaccurate

**File:** `.factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md`
**Anchor:** §Behavioral Contracts Table (line 144)
**Problem:** Claim "invariants 1-11" exercised by ACs is wrong; invariants 2 (PostToolUse only), 9 (no Advisory variant), 11 (is_char_boundary) are enforced via Architecture Compliance code review, not by ACs.
**Routing:** story-writer
**Recommended fix:** Update cell to "invariants 1, 3, 4, 5, 6, 7, 8, 10 (invariants 2/9/11 enforced via Architecture Compliance Rules)".

### F-S15.17-SP1-009 — MEDIUM — BC traceability cites BC-5.39.008 as path_allow sibling but BC-5.39.008 has different path_allow scope

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md`
**Anchor:** §Architecture Anchors line 425 (Story T-7 lines 444-446)
**Problem:** BC-5.39.008 path_allow scope is `[".factory", "plugins/vsdd-factory"]` (needs hooks-registry.toml read). BC-5.39.009 path_allow is `.factory` only. Closer sibling for path_allow is BC-5.39.006 (validate-dispatch-advance). BC narrative misleads implementer about structural lineage.
**Routing:** product-owner
**Recommended fix:** Cite BC-5.39.006 as closest path_allow sibling in §Description/§Preconditions/§Architecture Anchors.

### F-S15.17-SP1-010 — MEDIUM — BC missing `on_error = "continue"` invariant

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md`
**Anchor:** §Invariants
**Problem:** Story T-7 includes `on_error = "continue"` in registry entry. BC does NOT mention on_error anywhere. Behavioral property (plugin crash → graceful degradation, consistent with fail-open inv-10) must be specified in BC.
**Routing:** product-owner
**Recommended fix:** Add invariant specifying `on_error = "continue"` mirroring BC-5.39.004/005/006/007/008 precedent.

### F-S15.17-SP1-011 — LOW — BC §D-NNN Anchor Coverage table mis-uses non-D-NNN identifiers

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md`
**Anchor:** §D-NNN Anchor Coverage (lines 362-372)
**Problem:** Table cites ADR-018, ADR-021, META-LEVEL-24, BC-5.39.001 alongside D-NNN entries. Sibling BC-5.39.008 D-NNN table contains only D-NNN/F-PASS/POLICY/ADR-Option-b citations.
**Routing:** product-owner
**Recommended fix:** Move ADR/META-LEVEL/BC refs out of D-NNN table to existing §ADR References / §Related BCs / inline invariant body.

### F-S15.17-SP1-012 — LOW — D-NNN table claims D-453(d) gates "1-10" but range coverage is confusing vs D-454(a) "1-9"

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md`
**Anchor:** §D-NNN Anchor Coverage (line 364)
**Problem:** D-454(a) row says "1-9" but per-cell granularity gate applies to PC1-10. Minor coherence defect.
**Routing:** product-owner
**Recommended fix:** Update D-454(a) Postcondition cell to "1-10" or scope-clarified range.

### F-S15.17-SP1-013 — LOW — Token Budget underestimates hooks-registry.toml read cost

**File:** `.factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md`
**Anchor:** §Token Budget Estimate (lines 605-625)
**Problem:** Estimate "~3,000" for hooks-registry.toml is order-of-magnitude underestimate (actual ~1100 lines ≈ 33,000 tokens). Total revised ~78,500 (~39%) still within margin but narrowly.
**Routing:** story-writer
**Recommended fix:** Update Token Budget line to "~33,000 (1100 lines); implementer should sed-extract surrounding 30-50 lines around priority 158 rather than reading full file".

### F-S15.17-SP1-014 — NITPICK — BC EC-017 "State.md" capitalization typo

**File:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md`
**Anchor:** §Edge Cases EC-017 (line 332)
**Problem:** "State.md" should be "STATE.md".
**Routing:** product-owner
**Recommended fix:** Lowercase → STATE.md.

## Part B — Convergence Assessment

- **Pass-1 verdict:** HIGH (5 HIGH, 5 MEDIUM, 3 LOW, 1 NITPICK)
- **Streak:** 0/3 (RESET — HIGH findings present per BC-5.39.001 3-CLEAN protocol)
- **Notable patterns:**
  - **POLICY 4 (semantic_anchoring_integrity) recurrence:** F-002 (ADR-017 filename) + F-003 (PC mis-anchoring) + F-006 (EC renumbering) — three independent anchor-correctness defects in a single spec package suggests systematic drift between PO and story-writer. Recommend PO+story-writer cross-check via literal grep at every PC/EC/AC anchor before next pass.
  - **Cross-sibling semantic divergence (F-005):** BC-5.39.009 invariant 4 says "at least one contiguous match" (LENGTH≥4); BC-5.39.006 invariant 6(b) says "exactly 4". Two sibling BCs on the same trajectory_tail concept disagree on the LENGTH=5 case.
  - **Type-system gap (F-001):** Pseudocode `u64` vs actual SDK `u32` — would have failed at first `cargo build`. Process gap, but not severe enough to tag `[process-gap]` yet (one occurrence).
  - **STATE.md heading specification gap (F-004):** Most concerning structural defect. BC's PC2/PC3/PC5 leave canonical extractor anchors to implementer guesswork. Partial recurrence of META-LEVEL-30 route-(b) INSIDE the cure BC: codified-without-extractor-anchor-specification permits silent extractor degradation.
- **META-LEVEL candidates:** None new; no META-LEVEL-30 sub-route-(d) candidate emerges from this finding set.
- **Process-gap signals:** None tagged. Single-occurrence content defects, not process-level patterns.
- **Next pass dispatch:** **fix-burst required**. PO must address F-002, F-004, F-005, F-007, F-009, F-010, F-011, F-012, F-014; story-writer must address F-001, F-003, F-006, F-008, F-013. After fix-burst, dispatch pass-2 fresh-context adversary.
- **Convergence plausibility:** PLAUSIBLE BUT NOT HIGH-CONFIDENCE. 5 HIGH split: 2 mechanical-propagation (closable in story-writer burst), 3 require PO authoritative adjudication. Realistic 3-CLEAN by pass-5..7, similar to S-15.14 / S-15.15 cascade trajectories.
