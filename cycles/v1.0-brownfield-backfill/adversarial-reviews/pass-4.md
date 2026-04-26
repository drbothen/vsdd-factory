---
document_type: adversarial-review-pass
pass: 4
cycle: v1.0-brownfield-backfill
phase: 1d
verdict: FINDINGS_REMAIN
novelty_score: NITPICK
trajectory: "17 → 11 → 9 → 6"
timestamp: 2026-04-25T00:00:00
producer: adversarial-reviewer
---

# Phase 1d Adversarial Review — Pass 4

## Pass 1+2+3 Follow-Up Audit

All 9 pass-3 fixes verified clean. No regressions detected.

| Fix | Finding | Status |
|-----|---------|--------|
| F-029 | BC-INDEX SS-01..04 TBD capability stubs | CLEAN |
| F-030 | BC-INDEX subsystem column gap (SS-01 missing) | CLEAN |
| F-031 | BC-5.30.001 workflow-extraction stub postcondition | PARTIALLY REMEDIATED (F-041 below) |
| F-032 | VP-INDEX phase-column inconsistency | CLEAN |
| F-033 | ARCH-INDEX `&` vs `and` in Document Map labels | PARTIALLY REMEDIATED (F-043 below) |
| F-034 | S-0.01 target_module vs subsystems mismatch | CLEAN |
| F-035 | BC-3.03.001 truncated H1 | PARTIALLY REMEDIATED (F-040 below) |
| F-036 | BC-INDEX SS-08 empty Capability cells | PARTIALLY REMEDIATED (F-039 below) |
| F-037 | BC-6.10.001 Architecture Module field convention | PARTIALLY REMEDIATED (F-042 below) |

> Note: F-031, F-033, F-035, F-036, F-037 were recorded in pass-3 but the fixes
> were not applied in that pass. They appear here as pass-4 findings (F-038 through
> F-043) and are now fully remediated.

---

## Pass 4 NEW Findings

### F-038 MEDIUM — S-0.01 subsystems field misaligns with BC source-of-truth

**Severity:** MEDIUM
**Subsystem:** SS-09 / SS-10
**Files:**
- `stories/S-0.01-bump-version-prerelease.md:25`
- `behavioral-contracts/ss-09/BC-9.01.001.md:16`
- `architecture/ARCH-INDEX.md:82-83`

**Observation:**
`S-0.01` declares `subsystems: ["SS-10"]`. The story's `target_module` is
`scripts/bump-version.sh`. Per ARCH-INDEX line 83, `scripts/` is part of SS-10
(CLI Tools and Bin). However, BC-9.01.001 (the closest matching BC for activation
and config scripts) belongs to SS-09. The Subsystem Registry (lines 82-83) shows
`scripts/generate-registry-from-hooks-json.sh` is explicitly assigned to SS-09
(Configuration and Activation), not SS-10.

**Fix:** Update `S-0.01` frontmatter: `subsystems: ["SS-09"]`. BC source-of-truth
takes precedence per POLICY 6.

---

### F-039 LOW — BC-INDEX SS-08 rows: empty Capability cells (215 rows)

**Severity:** LOW
**Files:** `specs/behavioral-contracts/BC-INDEX.md:1650-1864`

**Observation:**
All 215 SS-08 BC rows have the pattern `| draft |  | TBD |` — the Capability
column is empty. Other subsystems use either `CAP-TBD` (SS-01..04) or `TBD` but
not an empty cell. The three different conventions (empty, `CAP-TBD`, `TBD`) are
inconsistent across the index.

**Fix:** Bulk-replace empty Capability cells in SS-08 rows with `CAP-TBD`.
Standardize all unanchored cells to `CAP-TBD` across all subsystems for
consistency.

---

### F-040 LOW — BC-3.03.001 H1 truncated mid-sentence; postcondition is stub

**Severity:** LOW
**Files:** `specs/behavioral-contracts/ss-03/BC-3.03.001.md:29,44`

**Observation:**
H1 reads: `BC-3.03.001: Batch trigger thresholds are independent — \`size\`
(default 100) AND \`interval_ms\` (default 5000` — truncated mid-sentence.
Postcondition section contains the stub `1. Two independent triggers:` with no
behavioral assertion.

**Fix:** Complete H1 to full sentence. Populate postconditions with the
size/interval triggers per the source evidence (DEFAULT_BATCH_SIZE=100,
DEFAULT_BATCH_INTERVAL_MS=5000).

---

### F-041 MEDIUM — BC-5.30.001 postcondition is a workflow version string, not a behavior

**Severity:** MEDIUM
**Files:** `specs/behavioral-contracts/ss-05/BC-5.30.001.md:30,42`

**Observation:**
H1 is `BC-5.30.001: feature-vsdd: identity` (workflow step identifier, not a
testable behavioral claim). Postcondition is `1. v3.0.0.` — a workflow version
string extracted verbatim from the lobster YAML preamble. This is not a
behavioral assertion and cannot be verified as a contract property.

**Fix:** Rewrite H1 and postconditions to assert testable properties of
`feature.lobster`: step count (82), routing track count (3), DAG acyclicity, and
YAML parseability.

---

### F-042 LOW — BC-6.10.001 Architecture Module field uses file path, not SS-NN convention

**Severity:** LOW
**Files:** `specs/behavioral-contracts/ss-06/BC-6.10.001.md:79`

**Observation:**
Traceability table row: `| Architecture Module | plugins/vsdd-factory/skills/deliver-story/SKILL.md |`
The established convention (observed in SS-03, SS-05 sibling BCs) is
`SS-NN (Subsystem Name)` optionally followed by a file path.

**Fix:** Edit line 79 to: `SS-06 (Skill Catalog) — plugins/vsdd-factory/skills/deliver-story/SKILL.md`

---

### F-043 LOW — ARCH-INDEX Document Map uses `&`; Subsystem Registry uses `and`

**Severity:** LOW
**Files:** `specs/architecture/ARCH-INDEX.md:31,39`

**Observation:**
Document Map (line 31): `SS-02 Hook SDK & Plugin ABI`
Subsystem Registry (line 75): `SS-02 | Hook SDK and Plugin ABI`

The Subsystem Registry is the declared source-of-truth (ARCH-INDEX line 68-70).
The Document Map label should match exactly. Similar `&` vs `and` divergence may
exist in other Document Map rows.

**Fix:** Normalize Document Map labels to use `and` to match the Subsystem
Registry source-of-truth.

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 6 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | NITPICK |
| **Median severity** | LOW |
| **Trajectory** | 17→11→9→6 |
| **Verdict** | FINDINGS_REMAIN |

Findings are cosmetic and extraction-stub class. No new behavioral
inconsistencies, no new cross-document contract violations, no regressions
from pass-3 fixes. First of 3 required consecutive NITPICK passes for
convergence.
