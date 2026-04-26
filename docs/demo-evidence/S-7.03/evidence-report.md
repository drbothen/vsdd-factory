# S-7.03 Demo Evidence — TDD Discipline Hardening

Story spec: `.factory/stories/S-7.03-tdd-discipline-hardening.md`

Branch: `feat/tdd-discipline-hardening`
HEAD SHA: `121d24c`
Date: 2026-04-26

**Summary:** 18/18 bats tests GREEN; 4-layer defense codified across 9 plugin-source files.

This is plugin-source delivery (Markdown + Bash + bats — no UI, no CLI binary, no Rust crate).
The bats suite and per-AC grep evidence comprise the full demo set in lieu of VHS/Playwright recordings.
VHS is available on this machine but does not apply — there is no interactive CLI binary to record.

---

## Evidence Files

| File | Description | Size |
|------|-------------|------|
| `bats-run.log` | Full TAP output from `bats tdd-discipline-gate.bats` — 18/18 ok lines | 20 lines |
| `grep-evidence.md` | Per-AC (AC-001 through AC-011) canonical string verification with file:line, grep output, and source commit SHAs | 11 sections |
| `evidence-report.md` | This file — top-level index with AC coverage map and architecture summary | — |

---

## AC Coverage Table

| AC | Status | Evidence | Bats tests |
|----|--------|----------|------------|
| AC-001 | covered | [grep-evidence.md#ac-001](grep-evidence.md) | ok 1 test_stub_architect_uses_todo_for_nontrivial_bodies |
| AC-002 | covered | [grep-evidence.md#ac-002](grep-evidence.md) | ok 2 test_green_by_design_excluded_from_red_ratio_denominator |
| AC-003 | covered | [grep-evidence.md#ac-003](grep-evidence.md) | ok 3 test_wiring_exempt_excluded_from_red_ratio_denominator |
| AC-004 | covered | [grep-evidence.md#ac-004](grep-evidence.md) | ok 4 test_anti_precedent_guard_in_deliver_story_skill |
| AC-005 | covered | [grep-evidence.md#ac-005](grep-evidence.md) | ok 5 test_anti_precedent_guard_in_per_story_delivery, ok 6 test_self_check_question_in_stub_architect_prompt |
| AC-006 | covered | [grep-evidence.md#ac-006](grep-evidence.md) | ok 7 test_red_ratio_threshold_section_present, ok 8 test_red_ratio_formula_present, ok 9 test_remediation_options_ab_present |
| AC-007 | covered | [grep-evidence.md#ac-007](grep-evidence.md) | ok 10 test_validate_red_ratio_blocks_on_low_ratio, ok 11 test_validate_red_ratio_passes_on_sufficient_ratio, ok 12 test_validate_red_ratio_passes_on_option_b_election, ok 13 test_validate_red_ratio_registered_in_hooks_registry |
| AC-008 | covered | [grep-evidence.md#ac-008](grep-evidence.md) | ok 14 test_tdd_mode_field_in_story_template, ok 15 test_tdd_mode_comment_documents_both_values |
| AC-009 | covered | [grep-evidence.md#ac-009](grep-evidence.md) | ok 16 test_facade_mode_section_present |
| AC-010 | covered | [grep-evidence.md#ac-010](grep-evidence.md) | ok 17 test_wave_gate_mutation_section_present, ok 18 test_wave_gate_mutation_threshold_80_present |
| AC-011 | covered | [bats-run.log](bats-run.log) | All 18 — see bats-run.log for full TAP output |

---

## Architecture Map — Files Touched Per Layer

### Layer 1 — Stub-commit obligations (AC-001, AC-002, AC-003)

| File | Action | Canonical strings introduced |
|------|--------|------------------------------|
| `plugins/vsdd-factory/agents/stub-architect.md` | modified | `todo!() Obligation (BC-5.38.001)` section; GREEN-BY-DESIGN + WIRING-EXEMPT reporting protocol; verbatim self-check question |
| `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` | modified | `GREEN-BY-DESIGN` and `WIRING-EXEMPT` exclusion from denominator in Red Gate Density Check section |

Source commits: `c4413e1` (stub-architect.md), `8cd16e9` (per-story-delivery.md)

### Layer 2 — Anti-precedent guard (AC-004, AC-005)

| File | Action | Canonical strings introduced |
|------|--------|------------------------------|
| `plugins/vsdd-factory/skills/deliver-story/SKILL.md` | modified | Verbatim `ANTI-PRECEDENT GUARD:` block with all four SHA refs (aa706543, 6d2d005e, 20b4a12a, e86d03f2) before Step 2 |
| `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` | modified | Same `ANTI-PRECEDENT GUARD:` block before Step 2 (BC-5.38.006 invariant 1 — both files updated atomically) |
| `plugins/vsdd-factory/agents/stub-architect.md` | modified | Verbatim self-check question: "If I include this real implementation, will the test for this function pass trivially without any implementer work?" |

Source commits: `d89b928` (deliver-story/SKILL.md), `8cd16e9` (per-story-delivery.md), `c4413e1` (stub-architect.md)

### Layer 3 — Red Gate density check (AC-006, AC-007)

| File | Action | Canonical strings introduced |
|------|--------|------------------------------|
| `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` | modified | `Red Gate Density Check (BLOCKING before Step 4)` section; `RED_RATIO` formula; `≥ 0.5` threshold; Option A and Option B remediation; red-gate-log format |
| `plugins/vsdd-factory/hooks/validate-red-ratio.sh` | created | New PostToolUse hook enforcing RED_RATIO gate; integer-division-based arithmetic; `hook.block` emission on violation |
| `plugins/vsdd-factory/hooks-registry.toml` | modified | `validate-red-ratio` hook registration entry (name, trigger, script_path) |

Source commits: `8cd16e9` (per-story-delivery.md), `f53bf43` (validate-red-ratio.sh), `3a9614c` (hooks-registry.toml)

### Layer 4 — tdd_mode frontmatter (AC-008, AC-009)

| File | Action | Canonical strings introduced |
|------|--------|------------------------------|
| `plugins/vsdd-factory/templates/story-template.md` | modified | `tdd_mode: strict  # strict \| facade. strict = full TDD Iron Law enforced (todo!() + Red Gate ≥0.5 required); facade = scaffold+impl combined, mutation testing at wave gate` |
| `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` | modified | `## tdd_mode: facade — Modified Flow` section describing facade-mode delivery semantics |
| `plugins/vsdd-factory/agents/story-writer.md` | modified | `tdd_mode` added to required frontmatter checklist (BC-8.30.001 postcondition 3) |

Source commits: `94b653c` (story-template.md), `8cd16e9` (per-story-delivery.md), `121d24c` (story-writer.md)

### Layer 5 — Mutation testing wave-gate (AC-010)

| File | Action | Canonical strings introduced |
|------|--------|------------------------------|
| `plugins/vsdd-factory/skills/wave-gate/SKILL.md` | modified | Mutation Testing section: `cargo mutants -p <crate> --jobs ... --timeout 300`; `killed * 100 / total >= 80`; disposition table columns (A/B/C); no-facade-stories skip log; cargo-mutants-not-found BLOCK |

Source commit: `fa07d94` (wave-gate/SKILL.md)

### Test file (AC-011)

| File | Action |
|------|--------|
| `plugins/vsdd-factory/tests/tdd-discipline-gate.bats` | created — 18 tests organized Layers 1-5 |

Source commit: `020518b` (RED phase test creation)

---

## VHS Recording

VHS is installed (`/opt/homebrew/bin/vhs`) but was not used. This story is plugin-source delivery
(Markdown + Bash + bats). There is no interactive CLI binary to record a terminal session of.
The bats-run.log (TAP output showing 18/18 ok) is the definitive machine-readable pass signal.

---

## Bats Final Result

```
1..18
ok 1 test_stub_architect_uses_todo_for_nontrivial_bodies
ok 2 test_green_by_design_excluded_from_red_ratio_denominator
ok 3 test_wiring_exempt_excluded_from_red_ratio_denominator
ok 4 test_anti_precedent_guard_in_deliver_story_skill
ok 5 test_anti_precedent_guard_in_per_story_delivery
ok 6 test_self_check_question_in_stub_architect_prompt
ok 7 test_red_ratio_threshold_section_present
ok 8 test_red_ratio_formula_present
ok 9 test_remediation_options_ab_present
ok 10 test_validate_red_ratio_blocks_on_low_ratio
ok 11 test_validate_red_ratio_passes_on_sufficient_ratio
ok 12 test_validate_red_ratio_passes_on_option_b_election
ok 13 test_validate_red_ratio_registered_in_hooks_registry
ok 14 test_tdd_mode_field_in_story_template
ok 15 test_tdd_mode_comment_documents_both_values
ok 16 test_facade_mode_section_present
ok 17 test_wave_gate_mutation_section_present
ok 18 test_wave_gate_mutation_threshold_80_present
```

**18/18 GREEN.**
