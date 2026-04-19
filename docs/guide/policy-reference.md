# Policy Reference

Quick-reference for all 9 governance policies. When you see a `POLICY N VIOLATION` or `BLOCKED` message, find the policy number below for the fix procedure.

---

## Enforcement Matrix

| # | Policy | Severity | Hook | Criteria | Key Agents |
|---|--------|----------|------|----------|------------|
| 1 | `append_only_numbering` | HIGH | — | 32, 77 | product-owner, spec-steward |
| 2 | `lift_invariants_to_bcs` | MEDIUM | — | 74 | product-owner, adversary |
| 3 | `state_manager_runs_last` | HIGH | — | — | orchestrator |
| 4 | `semantic_anchoring_integrity` | MEDIUM | — | 70-73 | adversary, consistency-validator |
| 5 | `creators_justify_anchors` | MEDIUM | — | — | product-owner, architect, story-writer, business-analyst |
| 6 | `architecture_is_subsystem_name_source_of_truth` | HIGH | `validate-subsystem-names.sh` | 76 | product-owner, architect, adversary |
| 7 | `bc_h1_is_title_source_of_truth` | HIGH | `validate-bc-title.sh` | 75 | product-owner, adversary |
| 8 | `bc_array_changes_propagate_to_body_and_acs` | HIGH | `validate-story-bc-sync.sh` | 67-69 | story-writer, product-owner, adversary |
| 9 | `vp_index_is_vp_catalog_source_of_truth` | HIGH | `validate-vp-consistency.sh` | 78-80 | architect, adversary, product-owner |

**Hook column:** PostToolUse hooks that fire on every Edit|Write to relevant files. `—` means gate-only (consistency-validator checks at phase boundaries).

**Criteria column:** Consistency-validator criterion numbers. See `agents/consistency-validator.md` for full descriptions.

---

## Violation Playbooks

### Policy 1: `append_only_numbering`

**Error:** Consistency-validator finding: "ID reuse detected" or "cross-cycle numbering conflict"

**What happened:** An existing ID (BC, VP, STORY, etc.) was renumbered, reused after retirement, or a filename slug was changed.

**How to fix:**
1. **Never renumber.** Create a NEW artifact with the next sequential ID instead
2. **Mark the old one retired:** Set `status: retired` and `replaced_by: <new-ID>` in the old artifact's frontmatter
3. **In the new artifact:** Set `replaces: <old-ID>` in frontmatter
4. **Update references:** Stories, indexes, and traceability tables should point to the new ID
5. **Keep the filename:** Even if the title changed, the original filename slug stays (e.g., `BC-2.01.001-parse-markdown.md` keeps that name)

---

### Policy 2: `lift_invariants_to_bcs`

**Error:** Consistency-validator finding: "Orphan invariant DI-NNN — not cited by any BC"

**What happened:** A domain invariant was declared in `domain-spec/invariants.md` but no behavioral contract references it in its Traceability L2 Invariants field.

**How to fix:**
1. Read `domain-spec/invariants.md` — find the orphan DI-NNN
2. Determine which BC(s) should enforce this invariant
3. In each enforcing BC file, add the DI-NNN to the Traceability section's "L2 Invariants" field
4. In `invariants.md`, verify the Scope/enforcer column names those BCs
5. Run consistency-validator to confirm the orphan is resolved

---

### Policy 3: `state_manager_runs_last`

**Error:** Version-race regression — STORY-INDEX or BC-INDEX citations reference stale versions

**What happened:** State-manager was dispatched before story-writer or product-owner finished, causing index citations to reference pre-update versions.

**How to fix:**
1. This is an orchestrator sequencing error — the human must verify dispatch ordering
2. Re-run state-manager AFTER all artifact-producing agents in the burst have completed
3. State-manager should always be the LAST agent in every burst

---

### Policy 4: `semantic_anchoring_integrity`

**Error:** Adversary or consistency-validator finding: "Mis-anchor" at MEDIUM+ severity

**What happened:** An anchor claim (BC→capability, story→subsystem, VP→anchor_story, traceability description) is syntactically valid but semantically wrong — it references the wrong thing.

**How to fix:**
1. Read both the source artifact and the target artifact cited by the anchor
2. Determine the correct target — does the anchor actually describe what the source does?
3. If wrong: update the anchor to the correct target with a justification sentence
4. If no correct target exists: flag for the relevant agent (architect for subsystems, product-owner for capabilities)
5. Mis-anchoring is NEVER deferred as an "Observation" — it always blocks convergence

---

### Policy 5: `creators_justify_anchors`

**Error:** Adversary finding: "Mechanical citation without body substantiation"

**What happened:** An agent created an anchor (e.g., BC→CAP-NNN) without explaining WHY that anchor is correct. The citation exists but the justification is missing or trivial.

**How to fix:**
1. For each anchor, write a one-sentence justification citing the source-of-truth:
   - BC→CAP: "Anchoring to CAP-NNN because this BC describes <purpose>, which is exactly what CAP-NNN defines per capabilities.md"
   - Story→SS: "SS-XX owns this story's scope because <reason> per ARCH-INDEX Subsystem Registry"
   - VP→Story: "Anchor story is STORY-NNN because that story builds the test vehicle where VP test code will live"
2. If you cannot write the justification, stop and ask — do not guess

---

### Policy 6: `architecture_is_subsystem_name_source_of_truth`

**Error:** `POLICY 6 VIOLATION (architecture_is_subsystem_name_source_of_truth)` from `validate-subsystem-names.sh`

**What happened:** A BC file's `subsystem:` field or a story file's `subsystems:` field uses an SS-ID that doesn't exist in `ARCH-INDEX.md` Subsystem Registry.

**How to fix:**
1. Read the error — it shows the invalid SS-ID AND lists all valid IDs with names (e.g., `SS-01 (Core Engine)`)
2. Open the BC or story file
3. Change the `subsystem:` (BC) or `subsystems:` (story) field to the correct SS-NN ID from ARCH-INDEX
4. Save — the hook will re-validate on the next edit

**Common causes:** Using a subsystem name instead of ID ("Core Engine" vs "SS-01"), typos in ID ("SS-1" vs "SS-01"), referencing a subsystem that hasn't been registered yet.

---

### Policy 7: `bc_h1_is_title_source_of_truth`

**Error:** `POLICY 7 VIOLATION (bc_h1_is_title_source_of_truth)` from `validate-bc-title.sh`

**What happened:** A BC file's H1 heading (`# BC-S.SS.NNN: <title>`) doesn't match the title in BC-INDEX.md.

**How to fix:**
1. Read the error — it shows both the H1 title and the BC-INDEX title
2. **The H1 is authoritative.** Decide which title is correct:
   - If the H1 is correct: update BC-INDEX.md to match the H1
   - If the H1 is wrong: fix the H1 heading first, then update BC-INDEX to match
3. **If enrichment was added downstream** (e.g., "(Fail-Closed for Writes)" appears in BC-INDEX but not in the H1): move the enrichment INTO the H1 heading
4. Save — the hook will re-validate on the next edit

---

### Policy 8: `bc_array_changes_propagate_to_body_and_acs`

**Error:** `POLICY 8 VIOLATION (bc_array_changes_propagate_to_body_and_acs)` from `validate-story-bc-sync.sh`

**What happened:** A story file's frontmatter `bcs:` array doesn't match the BCs listed in the body's Behavioral Contracts table or AC trace annotations. A BC was added to or removed from one representation but not the others.

**How to fix:**
1. Read the error — it names the specific BCs that are out of sync
2. Open the story file and verify all three representations match:
   - **Frontmatter `bcs:` array** — the machine-readable list
   - **Body Behavioral Contracts table** — the human-readable table with BC ID + title columns
   - **AC trace annotations** — `(traces to BC-S.SS.NNN ...)` in acceptance criteria headings
3. For each missing BC:
   - If missing from body table: add a row with the BC ID and its current title from BC-INDEX
   - If missing from AC traces: add at least one AC with `(traces to BC-S.SS.NNN)` annotation
   - If missing from frontmatter: add it to the `bcs:` array
4. Update the Token Budget "BC files (N BCs)" count to match `len(bcs)`
5. Save — the hook will re-validate on the next edit

---

### Policy 9: `vp_index_is_vp_catalog_source_of_truth`

**Error:** `POLICY 9 VIOLATION (vp_index_is_vp_catalog_source_of_truth)` from `validate-vp-consistency.sh`

**What happened:** VP-INDEX.md was changed (VP added, retired, reassigned) but the two architecture anchor documents weren't updated to match.

**How to fix:**
1. Read the error — it identifies the specific VP and mismatch type
2. Open all three files:
   - `specs/verification-properties/VP-INDEX.md` — the source of truth
   - `specs/architecture/verification-architecture.md` — must match VP-INDEX
   - `specs/architecture/verification-coverage-matrix.md` — must match VP-INDEX
3. For each discrepancy:
   - **VP missing from arch docs:** Add the VP row to the Provable Properties Catalog AND the VP-to-Module table
   - **Tool/module/phase mismatch:** Update the arch doc entry to match VP-INDEX
   - **Arithmetic mismatch:** Recalculate Totals row and per-tool summary counts
   - **Orphaned reference:** If a VP appears in arch docs but not VP-INDEX, remove it from arch docs
4. Save any of the three files — the hook will re-validate

**Arithmetic invariant:** VP-INDEX total must equal the sum of per-tool counts (Kani + Proptest + Fuzz + ...) and must equal the VP row count. Coverage matrix Totals row must match these numbers.

---

## Data Safety Guards

These hooks prevent irreversible damage but are not governance policies — they're safety nets.

| Guard | Hook | What It Prevents |
|-------|------|-----------------|
| Destructive commands | `destructive-command-guard.sh` | `rm -rf .factory/`, `git reset --hard`, `git clean -f`, `rm STATE.md` |
| Branch protection | `verify-git-push.sh` | Force push, direct push to main/master/develop |
| Factory branch | `factory-branch-guard.sh` | Writing to `.factory/` when not on `factory-artifacts` branch |

**If blocked by a safety guard:** Read the error message — it includes a `Suggestion:` line with the safe alternative.
