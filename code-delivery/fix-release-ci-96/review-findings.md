# Review Findings: PR #96 — fix(release-ci)

## Convergence Table

| Cycle | Findings | Blocking | Fixed | Remaining |
|-------|----------|----------|-------|-----------|
| 1     | 0        | 0        | 0     | 0 → APPROVE |

## Cycle 1 Findings

**Verdict: APPROVE — no blocking findings.**

### Finding Analysis

All changes reviewed exhaustively:

#### 1. _SELF_DIR fallback pattern — CORRECT
Pattern applied uniformly to all 31 hook files:
```bash
_SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_BLOCK_SH="${CLAUDE_PLUGIN_ROOT:+${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh}"
_BLOCK_SH="${_BLOCK_SH:-${_SELF_DIR}/lib/block.sh}"
if [ -f "$_BLOCK_SH" ]; then source "$_BLOCK_SH"; fi
```
This is the canonical safe pattern for location-independent sourcing in bash. Consistent across all 31 files.

#### 2. Test assertion completeness — COMPLETE
All old reason codes confirmed removed from assertions in the feature branch:
- `input_hash_invalid_format` → `input_hash_format` ✓
- `novelty_assessment_incomplete` → `novelty_section_missing` ✓
- `anchor_capabilities_mismatch` → `anchor_caps_drift` ✓
- `demo_evidence_not_story_scoped` → `pol_010_violation` ✓
- `factory_path_worktree_relative` → `factory_path_relative` ✓
- `wave_gate_prerequisite_not_passed` → `wave_gate_blocking` ✓
- `policy6_subsystem_name_mismatch` → `policy_6_violation` ✓
- `policy7_bc_title_mismatch` → `bc_h1_index_drift` ✓
- `policy8_bc_array_desync` → `policy_8_violation` ✓
- `policy9_vp_inconsistency` → `policy_9_violation` ✓
- `finding_id_legacy_format` → `id_format_violation` ✓
- `table_cell_count_mismatch` → `table_cell_count` ✓
- `changelog_not_monotonic` → `changelog_duplicate_version` ✓
- `state_bloat` → `state_md_bloat` ✓
- `state_version_pin_drift` → `state_pin_stale` ✓
- `template_noncompliant` → `template_drift` ✓
- `factory_not_worktree` → `factory_no_worktree` ✓
- `red_gate_strict_violation` → `red_gate_strict` ✓

Scan for residual old reason codes in assertions returned zero results.

#### 3. Test name vs assertion — VERIFIED
Two test names still contain old code names (`wave_gate_prerequisite_not_passed`, `factory_not_worktree`, `red_gate_strict_violation`) but their **assertions** correctly use the new codes. Test names are documentation only; not a functional issue.

#### 4. Secondary field removal — INTENTIONAL AND CORRECT
Removed secondary field assertions (`.issue`, `.verdict`, `.worktree`, `.expected`, `.actual`, `.matcher`, `.bc_id`, `.h1_title`, `.index_title`, `.line_count`, `.limit`, `.file_path`, `.command`) — these k=v pairs are no longer emitted by `block_pre` canonical format. Correct to remove.

#### 5. perf-baseline.bats skip — JUSTIFIED
`perf-baseline.bats` requires `.factory` worktree mount (confirmed by reading the file). CI `validate` job explicitly mounts `factory-artifacts` via `git worktree add .factory origin/factory-artifacts` before running tests. Not a Slice 3 regression. Skip in local run is correct.

#### 6. validate-subsystem-names.sh CANONICAL_MAP removal — CLEAN
`CANONICAL_MAP` awk block was removed. Scan confirms `CANONICAL_MAP` is not referenced anywhere in the branch version. Dead code removal — clean.

#### 7. validate-wave-gate-prerequisite.sh HOOK_OUTPUT removal — CLEAN
`HOOK_OUTPUT=$(bash "$SHA_HOOK" ...)` changed to direct `if bash "$SHA_HOOK" ...`. `HOOK_OUTPUT` was never read after capture in the original — captured only for the exit code. Clean simplification.

#### 8. input-hash.bats placeholder/null tests — BEHAVIOR CHANGE ALIGNED WITH HOOK
`status -eq 0` changed to `status -eq 2` for placeholder/null hash. Verified: hook emits `block_pre` with code `input_hash_missing` for these cases (line 85), which exits 2. Test was asserting wrong behavior (exit 0); now correctly asserts exit 2. This is a test correctness fix.

#### 9. skills.bats 10→11 policy count — VERIFIED AGAINST TEMPLATE
`policies-template.yaml` has exactly 11 policies (confirmed via yq). Previous assertion of 10 was stale. Correct update.

#### 10. corpus-lint.bats output string changes — CORRECT
`UNION VIOLATION` → `BLOCKED`, `FACTORY PATH ERROR` → `BLOCKED`, `worktree instead of project root` → `.worktrees/STORY-001`. Verified against hook output format (canonical `BLOCKED by ...` prefix from `block_pre`).

#### 11. hook-robustness.bats policy VIOLATION string softening — CORRECT
`POLICY N VIOLATION` → `POLICY N` (substring match only). Hooks still output strings containing `POLICY N`; test is now less brittle to wording changes. Not a loss of coverage.

#### 12. destructive-guard.bats Suggestion→Fix — CORRECT
`Suggestion:` → `Fix:` in block message test. Verified: `lib/block.sh` canonical format outputs `Fix:` not `Suggestion:`. Test was asserting a stale label.

## Triage Summary

| Finding ID | Severity | Category | Route | Status |
|------------|----------|----------|-------|--------|
| — | — | — | — | No findings |

## Status

**APPROVE** — All 47 changed files reviewed. Zero blocking findings. Zero suggestions. All assertions cross-verified against hook implementation. perf-baseline skip is justified and documented. Merge authorized.
