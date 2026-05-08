---
document_type: phase-handoff
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F5-pre-compact
producer: state-manager
version: "1.0"
status: ready
purpose: "Post-context-compaction F5 convergence resume reference"
timestamp: 2026-05-08T17:00:00Z
---

# F5 Pre-Compact Handoff — S-15.01 Convergence Cycle

This document captures everything orchestration needs to resume the F5 cycle
after `/compact`. Read this + STATE.md + the verification report below to
restore full context.

## 1. Cycle Status

| Field | Value |
|-------|-------|
| Cycle | v1.0-feature-plugin-async-semantics-pass-1 |
| Phase | F5 ADVERSARIAL — convergence-attempt mode |
| ADR-013 clock | 0_of_3 (pass-17 was HIGH; pass-18 dispatch pending) |
| Branch (long-lived) | `fix/S-15.01-F5-convergence` HEAD `bb661eaa` (33 commits ahead of develop) |
| Factory-artifacts HEAD | `1e9fa71f` + post-rename close commit (TBD this burst) |
| PR status | HELD until ADR-013 = 3_of_3 |
| Trajectory | 17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4 |

## 2. Pass Convergence History

| Pass | Verdict | Counter | Key findings |
|------|---------|---------|--------------|
| 1 | HIGH (5H/6M/4L/2NIT) | 0_of_3 | Many Path A/B/C/D issues |
| 2 | HIGH (3H/6M/4L/2NIT) | 0_of_3 | Drain refactor missing |
| 3 | MEDIUM (0H/2M/2L/2NIT) | 0_of_3 | VP-077 properties, doc gaps |
| 4 | MEDIUM (0H/1M/4L/0NIT) | 0_of_3 | Source-tree drift |
| 5 | NITPICK_ONLY (0/0/0/0) | **1_of_3** | First chain advance |
| 6 | MEDIUM | 0_of_3 RESET | Test coverage gaps |
| 7 | MEDIUM | 0_of_3 | Source-tree sweep |
| 8 | MEDIUM | 0_of_3 | DuplicateEntry silent fail |
| 9 | MEDIUM | 0_of_3 | Sibling cite drifts |
| 10 | HIGH | 0_of_3 | Lint plugin canonical string drift |
| 11 | LOW | 0_of_3 | More propagation gaps |
| 12 | HIGH | 0_of_3 | Vacuous mutation defect (pass-11 fix didn't work) |
| 13 | HIGH | 0_of_3 | Spec-source line drift recurrence |
| 14 | HIGH | 0_of_3 | Cross-BC contradiction |
| 15 | HIGH | 0_of_3 | F-P14-001 propagation gap into VP-079 |
| 16 | HIGH | 0_of_3 | TD-031 self-violation in fix-burst-14 |
| 17 | HIGH | 0_of_3 | TD-031 violations recur in fix-burst-15 |

## 3. THE PIVOT: TD-031 enforcement lint-hook

The cycle's recurrence loop was caused by:
1. Fresh-context adversary passes find real defects via new angles
2. Fix-bursts fix the primary site but introduce sibling-propagation drift
3. Next pass surfaces the new drift via different angle
4. Repeat indefinitely

Fix-burst-16 broke the loop by **implementing mechanical enforcement**:

### `validate-stable-anchors` WASM hook (renamed from `validate-td031-stable-anchors` in this burst)

- **Path:** `crates/hook-plugins/validate-stable-anchors/`
- **WASM artifact:** `plugins/vsdd-factory/hook-plugins/validate-stable-anchors.wasm` (174,462 bytes)
- **Registered:** `plugins/vsdd-factory/hooks-registry.toml` — PreToolUse + Edit|Write + on_error=block + priority=155
- **Detection:** `<word_chars>.rs:<digits>` pattern in `.factory/specs/**/*.md` body
- **Exempt zones:** `## Amendment`, `## Changelog` (case-insensitive), bash code fences containing `SITES=(`
- **Tests:** 40/40 pass; pure `hook_logic<R, E, L>` for testability without WASM runtime
- **HOST_ABI_VERSION:** 1
- **Compliance:** native WASM Rust (Decision 5 WASM-migration rule)

The rename commit is `bb661eaa`. The hook now generalizes beyond TD-031 — it's a stable-anchor convention enforcer for all spec content.

## 4. The 180-Violation Backlog

The lint-hook activation in fix-burst-16 surfaced **180 pre-existing violations across 60 files** in `.factory/specs/**/*.md`.

### Verification report (just completed)

**Sample size:** 22 stratified samples (VPs, BCs, invariants, stories, ADRs)
**True-positive rate: 98.9% (178 of 180)**
**False-positive rate: 1.1% (2 — both in VP-INDEX YAML frontmatter `changelog:` array)**

Root cause of 2 FPs: hook's exempt-zone state machine doesn't recognize YAML frontmatter delimiters.

### Top 5 files (52% of total = 93 violations)

| File | Violations |
|------|------------|
| `.factory/specs/behavioral-contracts/ss-01/BC-1.05.036.md` | 44 |
| `.factory/specs/behavioral-contracts/ss-01/BC-1.05.035.md` | 23 |
| `.factory/specs/.../open-questions.md` | 13 |
| `.factory/specs/architecture/decisions/ADR-010-storedata-linker.md` | 7 |
| `.factory/specs/behavioral-contracts/ss-02/BC-2.02.011.md` | 6 |

### Recommended next step (USER-APPROVED PATH)

**Mass sweep all 180 violations** — migrate `*.rs:NNN` patterns to stable symbol anchors per TD-VSDD-091. Most cites have stable symbol names (e.g., `RegistryError::Toml`, `HOST_ABI_VERSION`, `partition_plugins`) already in the surrounding sentence; rewrites are mechanical.

**Optional small tune:** extend hook's exempt-zone state machine to skip YAML frontmatter between `^---$` delimiters at line 1. Eliminates the 2 VP-INDEX false positives. Low priority (saves 1 file from sweep).

## 5. Post-Compact Resume Workflow

After `/compact`, orchestration should:

1. Read STATE.md
2. Read this handoff doc + the verification report at `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F5-verification-180-violations.md` (if persisted separately) OR the §4 sample table above
3. Decide on mass sweep approach:
   - **Option (a)** Single mass sweep dispatch (implementer): all 180 across 60 files in one burst — risk of context overflow
   - **Option (b)** Chunked sweep (recommended): 4-6 sub-bursts of ~10 files each, focused on the top-5 file clusters first
4. Dispatch sweep
5. After sweep: dispatch pass-18 adversary
6. If pass-18 NITPICK_ONLY → ADR-013 clock 0→1_of_3 → continue chain (passes 19, 20 for 2_of_3, 3_of_3 = CONVERGED)

## 6. Critical Paths

| Artifact | Path |
|----------|------|
| Story | `.factory/stories/S-15.01-plugin-async-semantics.md` (currently v1.20) |
| Follow-up story | `.factory/stories/S-15.02-dispatcher-cold-start-optimization.md` (v1.7) |
| Lint hook | `crates/hook-plugins/validate-stable-anchors/` |
| WASM artifact | `plugins/vsdd-factory/hook-plugins/validate-stable-anchors.wasm` |
| Registration | `plugins/vsdd-factory/hooks-registry.toml` |
| TD register | `.factory/tech-debt-register.md` (TD-031 P1) |
| Pass reports | `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F5-adversary-pass-{1..17}.md` |

## 7. Spec versions at HEAD (post-rename)

| Spec | Version |
|------|---------|
| BC-1.14.001 | v1.9 |
| BC-3.08.001 | v1.11 |
| BC-7.06.001 | v1.9 |
| BC-9.01.006 | v1.2 |
| VP-077 | v1.10 |
| VP-079 | v1.16 |
| DI-019 | v1.5 |
| ADR-019 | v1.8 |
| ADR-020 | v1.0 (last_amended bumped) |
| S-15.01 | v1.20 |
| S-15.02 | v1.7 |

## 8. PR strategy reminder

User explicitly directed (early in cycle): keep PR HELD until ADR-013 = 3_of_3 is achieved. Open ONE consolidated PR at convergence with all 33+ commits squash-merged.

## 9. User directives in force

1. "Most correct, not fastest" — no time-boxing; fix all real defects
2. "Continue with A until we fix everything" — Option A from pass-12 strategic decision
3. "We are migrating to WASM" — all new plugins MUST be native WASM Rust
4. WASM-migration rule applies retroactively to all session work
5. AC-016 budget revision (500ms→1500ms via ADR-020 Class A)
6. Single consolidated story (S-15.01); no phased rollout

## 10. Process-gap codifications (in TD register)

- TD-028: Spec-impl drift on fail-closed/fail-open classification (P2)
- TD-029: Bats integration tests should cover all RegistryError variants (P2)
- TD-030: Canonical-string sweeps need separate discipline from version-label sweeps (P2)
- TD-031: Recurrent post-EC-012 line-drift pattern — IMPLEMENTED via validate-stable-anchors hook (P1; enforcement now mechanical)
