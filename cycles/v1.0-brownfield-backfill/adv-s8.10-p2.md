---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.10-sdk-extension-write-file.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.10-p1.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - crates/hook-sdk/src/host.rs
  - crates/hook-sdk/src/ffi.rs
  - crates/hook-sdk/HOST_ABI.md
input-hash: "e441e99"
traces_to: prd.md
pass: p2
previous_review: adv-s8.10-p1.md
target: story
target_file: .factory/stories/S-8.10-sdk-extension-write-file.md
verdict: MINOR
clock: 1_of_3
findings_critical: 0
findings_high: 0
findings_medium: 2
findings_low: 4
findings_nit: 1
---

# Adversarial Review: S-8.10 v1.1 (Pass 2)

## Finding ID Convention

Finding IDs use the format: `ADV-S810-P02-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `S810`: Story identifier
- `P02`: Two-digit pass number
- `<SEV>`: Severity abbreviation (`HIGH`, `MED`, `LOW`, `NIT`)
- `<SEQ>`: Three-digit sequence

## Part A — Fix Verification (18/18 closed)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| HIGH-001 input paths | HIGH | CLOSED | Input paths corrected to .factory/cycles/v1.0-brownfield-backfill/ |
| HIGH-002 max_bytes signature | HIGH | CLOSED | max_bytes: u32 added to AC-1; signature pinned as write_file(path, data, max_bytes) |
| HIGH-003 FFI input-pointer | HIGH | CLOSED | FFI input-pointer protocol pinned in AC-1 |
| HIGH-004 AC-5 conditional | HIGH | CLOSED | max_bytes made mandatory; conditional removed from AC-5 |
| HIGH-005 BC family BC-2.02.011 | HIGH | CLOSED | Verified BC-INDEX:148-169; BC-2.02.011 correctly cited |
| MED-001 through MED-007 | MEDIUM | CLOSED | All 7 medium findings verified closed in v1.1 |
| LOW-001 through LOW-005 | LOW | CLOSED | All 5 low findings verified closed in v1.1 |
| NIT-001 | NIT | CLOSED | Closed in v1.1 |

All 18 pass-1 findings confirmed CLOSED. No regressions observed.

## Part B — New Findings (7)

### MEDIUM

#### ADV-S810-P02-MED-001: Path-resolver helper for write_file unspecified

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.10 AC-1 and host.rs integration discussion
- **Description:** The read_file host function uses `resolve_for_read` with documented symlink-following and CLAUDE_PROJECT_DIR rooting semantics. write_file requires an analogous `resolve_for_write` with explicit semantics around symlink handling, parent-directory creation, and path-traversal restrictions. The story does not specify which resolver variant applies or whether a new one is needed. Security gap: if implementer reuses `resolve_for_read` for write paths, parent-creation semantics differ. This is a content defect, not a process gap.
- **Proposed Fix:** Add a sentence to AC-1 specifying that write_file uses `resolve_for_write` (new helper) with: CLAUDE_PROJECT_DIR rooting required, no parent-directory auto-creation, symlink resolution before write, path-traversal rejection.

#### ADV-S810-P02-MED-002: EC-006 error code conflicts with Rule 4 pattern

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.10 EC-006 and sibling read_file error-code table
- **Description:** EC-006 maps missing-parent-directory to `-99 INTERNAL_ERROR`. The read_file sibling pattern (empirically from host.rs) uses `-1 CAPABILITY_DENIED` for analogous structural-precondition failures (path outside project root, etc.). Using a different error code for a comparable structural failure breaks the rule-4 consistency pattern that consumers rely on for error-handling logic. Pending intent verification: if divergence is intentional, require explicit justification in EC-006 prose.
- **Proposed Fix:** Either align EC-006 to `-1 CAPABILITY_DENIED` to match read_file sibling pattern, or add explicit rationale prose explaining the intentional divergence and why INTERNAL_ERROR better characterizes this specific failure mode.

### LOW

#### ADV-S810-P02-LOW-001: AC-5(b) tmp-dir test does not cite CLAUDE_PROJECT_DIR rooting

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.10 AC-5(b)
- **Description:** AC-5(b) specifies a test where the target path is a tmp directory outside the project root and expects CAPABILITY_DENIED. The test correctly exercises the rooting constraint but does not cite CLAUDE_PROJECT_DIR by name, creating ambiguity about which root boundary is tested. A reader implementing the test might use a different root variable.
- **Proposed Fix:** Add parenthetical "(outside CLAUDE_PROJECT_DIR)" to AC-5(b) test description.

#### ADV-S810-P02-LOW-002: AC-4(b) "without panic" tautology (POLICY 11)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.10 AC-4(b)
- **Description:** AC-4(b) states "operation completes without panic." This is a tautology — all production Rust code is expected to not panic in normal operation. The criterion is untestable and adds no precision. Analogous tautologies are flagged under POLICY 11 (no_test_tautologies). The meaningful AC is the error-return behavior, which is already covered by EC-006.
- **Proposed Fix:** Replace "without panic" with the actual observable: "returns Err(WriteError::PermissionDenied) and leaves the file unmodified."

#### ADV-S810-P02-LOW-003: BC-2.02.002 title scope drift (pending intent verification)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.10 BC table row for BC-2.02.002
- **Description:** BC-2.02.002 title in the story's BC trace table lists "bounded-call mandate" with scope language that references read_file and exec_subprocess only. If write_file is meant to be governed by BC-2.02.002 as well, the BC title needs widening discussion. If the story is asserting a new invariant under an existing BC, the scope mismatch should be noted as a pending-intent item for PO adjudication.
- **Proposed Fix:** Pending PO intent verification. If BC-2.02.002 governs write_file, add write_file to the BC title scope list. If not, document the explicit non-applicability in the BC trace row.

#### ADV-S810-P02-LOW-004: depends_on contains S-8.00 only; sibling SDK stories absent (pending intent verification)

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.10 frontmatter depends_on field
- **Description:** depends_on contains S-8.00 only. Sibling SDK-surface stories S-8.07 and S-8.08 also reference the hook-sdk crate and may be affected by the HOST_ABI_VERSION boundary established in S-8.10. If S-8.07/S-8.08 implementations must coordinate on HOST_ABI_VERSION (additive export per S-5.06 semver), the dependency graph is incomplete. Pending intent verification: orchestrator decides whether sibling coordination is required.
- **Proposed Fix:** Pending orchestrator intent verification. If sibling coordination is required, add S-8.07 and S-8.08 to blocks (not depends_on — S-8.10 blocks them, they do not block it).

### NIT

#### ADV-S810-P02-NIT-001: bcs frontmatter empty; missing spec-gate name citation

- **Severity:** NIT
- **Category:** spec-fidelity
- **Location:** S-8.10 frontmatter bcs field
- **Description:** bcs frontmatter field is empty (`bcs: []`). Per S-7.01 Spec-First Gate, stories that author new BCs should self-cite their BC IDs in frontmatter. The comment currently reads "pending PO author" without citing the gate by name.
- **Proposed Fix:** Replace comment with explicit citation: `# OQ-A1 open: BC-2.02.011 pending PO author per Spec-First Gate S-7.01`.

## Open Questions

| ID | Question | Owner | Status |
|----|----------|-------|--------|
| OQ-A1 | PO must author BC-2.02.011 before story status can flip to ready | PO | OPEN (carried from p1) |
| OQ-A2 | BC-2.02.002 title widening: does the bounded-call mandate govern write_file? | PO | OPEN (new) |
| OQ-A3 | depends_on/blocks completeness: does S-8.10 block S-8.07 and S-8.08? | Orchestrator | OPEN (new) |

## Pass-3 Priors

- BC-2.02.011 authored by PO → populate frontmatter bcs field
- MED-001: `resolve_for_write` helper semantics pinned in AC-1
- MED-002: EC-006 error-code reconciled with read_file Rule 4 pattern or justified divergence
- LOW-002: AC-4(b) tautology replaced with observable behavior assertion
- LOW-003: BC-2.02.002 scope adjudicated

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 2 |
| LOW | 4 |
| NIT | 1 |

**Overall Assessment:** minor — iterate
**Convergence:** converging (61% decay; 100% HIGH closure)
**Readiness:** requires revision (MED-001 + MED-002 must close)

## Verdict

**MINOR** — 0 CRITICAL, 0 HIGH, 2 MED, 4 LOW, 1 NIT. Foundationally sound; all pass-1 HIGH/MED findings closed cleanly. MED-001 and MED-002 are content defects requiring fix before clock advance.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 5 | 7 | 5 | 1 | 18 |
| p2 | 0 | 2 | 4 | 1 | 7 |

61% reduction pass-1 → pass-2; 100% HIGH closure. Healthy convergence shape.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 2 |
| **New findings** | 7 |
| **Closures** | 18 |
| **Novelty score** | 1.0 (7/7 novel) |
| **Median severity** | LOW |
| **Trajectory** | 18→7 |
| **Verdict** | CONVERGING |
