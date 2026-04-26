# S-7.03 Per-AC Grep Evidence

Plugin-source delivery (Markdown + Bash + bats). Evidence is grep output from the
actual plugin source files showing the canonical strings introduced by the 9 feature
commits on `feat/tdd-discipline-hardening`.

Branch HEAD: `121d24c`

---

### AC-001 — todo!() obligation for non-trivial stub bodies

**Criterion:** stub-architect.md must contain the todo!() obligation guard text citing BC-5.38.001 in its Constraints section. No real business logic is permitted in non-trivial function bodies in a strict-mode stub commit.

**Grep evidence:**
```bash
grep -n 'todo!()' plugins/vsdd-factory/agents/stub-architect.md
grep -n 'BC-5.38.001' plugins/vsdd-factory/agents/stub-architect.md
```

**Output:**
```
plugins/vsdd-factory/agents/stub-architect.md:3:description: Use when generating compilable stubs for a story's file list. Produces todo!()-body skeletons that compile but fail all tests, enforcing Red Gate discipline per BC-5.38.001.
plugins/vsdd-factory/agents/stub-architect.md:17:list. All non-trivial function bodies use `todo!()` or `unimplemented!()`. The
plugins/vsdd-factory/agents/stub-architect.md:34:- `todo!()` bodies for all non-trivial functions
plugins/vsdd-factory/agents/stub-architect.md:45:### todo!() Obligation (BC-5.38.001)
plugins/vsdd-factory/agents/stub-architect.md:48:BC-8.30.001 invariant 2), every non-trivial function body MUST use `todo!()` or
plugins/vsdd-factory/agents/stub-architect.md:56:non-trivial bodies. Cite: BC-5.38.001.
```

**Source commit:** `c4413e1` — `feat(S-7.03): add self-check rule + GREEN-BY-DESIGN/WIRING-EXEMPT protocol to stub-architect.md (Task 8)`

**Bats test result:** ok 1 test_stub_architect_uses_todo_for_nontrivial_bodies (PASS)

---

### AC-002 — GREEN-BY-DESIGN exemption excluded from denominator

**Criterion:** per-story-delivery.md must document that GREEN-BY-DESIGN tests are excluded from the RED_RATIO denominator.

**Grep evidence:**
```bash
grep -n 'GREEN-BY-DESIGN' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
grep -n 'denominator\|exclud' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
```

**Output:**
```
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:54:- `EXEMPT_TESTS` = `GREEN-BY-DESIGN_count` + `WIRING-EXEMPT_count`
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:56:**GREEN-BY-DESIGN tests** (BC-5.38.002): Tests that exercise functions whose correct behavior is deterministic from the type system alone (e.g., enum variant labels, pure data accessors). These are excluded from the denominator because they cannot be made red without making the stub non-compilable.
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:68:When all effective tests (after excluding GREEN-BY-DESIGN + WIRING-EXEMPT) are themselves exempt (denominator = 0), the gate does NOT vacuously pass. The orchestrator must explicitly acknowledge this in the red-gate-log file (`.factory/logs/red-gate-log-<story-id>.md`) with `full_exception_path: true`.
```

**Source commit:** `8cd16e9` — `feat(S-7.03): add anti-precedent guard + Red Gate Density Check + facade-mode to per-story-delivery.md (Tasks 4,5,6)`

**Bats test result:** ok 2 test_green_by_design_excluded_from_red_ratio_denominator (PASS)

---

### AC-003 — WIRING-EXEMPT exemption excluded from denominator

**Criterion:** per-story-delivery.md must document that WIRING-EXEMPT tests are excluded from the RED_RATIO denominator alongside GREEN-BY-DESIGN entries.

**Grep evidence:**
```bash
grep -n 'WIRING-EXEMPT' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
```

**Output:**
```
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:54:- `EXEMPT_TESTS` = `GREEN-BY-DESIGN_count` + `WIRING-EXEMPT_count`
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:58:**WIRING-EXEMPT tests** (BC-5.38.003): Tests that verify infrastructure wiring (e.g., that a struct implements a trait, that a constructor returns the correct type). These are excluded from the denominator because they pass as soon as the correct type signature exists in the stub, not because of premature implementation.
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:68:When all effective tests (after excluding GREEN-BY-DESIGN + WIRING-EXEMPT) are themselves exempt (denominator = 0), the gate does NOT vacuously pass.
```

**Source commit:** `8cd16e9` — `feat(S-7.03): add anti-precedent guard + Red Gate Density Check + facade-mode to per-story-delivery.md (Tasks 4,5,6)`

**Bats test result:** ok 3 test_wiring_exempt_excluded_from_red_ratio_denominator (PASS)

---

### AC-004 — Anti-precedent guard verbatim in deliver-story SKILL.md

**Criterion:** deliver-story SKILL.md must contain the verbatim ANTI-PRECEDENT GUARD block before Step 2 scaffolding instructions, including all four SHA commits (aa706543, 6d2d005e, 20b4a12a, e86d03f2).

**Grep evidence:**
```bash
grep -n 'ANTI-PRECEDENT GUARD:' plugins/vsdd-factory/skills/deliver-story/SKILL.md
grep -n 'aa706543\|6d2d005e\|20b4a12a\|e86d03f2' plugins/vsdd-factory/skills/deliver-story/SKILL.md
```

**Output:**
```
plugins/vsdd-factory/skills/deliver-story/SKILL.md:63:> **ANTI-PRECEDENT GUARD:** Do not use sibling crates with pre-implemented stubs as templates for your stub work. If you observe that a sibling crate (e.g., a DTU clone or prior story's scaffold) contains full business logic rather than todo!() macros, treat it as a historical anti-pattern. Your stub must use todo!() for all non-trivial function bodies. Anti-precedent evidence: Prism commits aa706543, 6d2d005e, 20b4a12a. Model precedent: e86d03f2.
```

All four SHAs appear on line 63 in the single guard block.

**Source commit:** `d89b928` — `feat(S-7.03): add anti-precedent guard to deliver-story SKILL.md (Task 2)`

**Bats test result:** ok 4 test_anti_precedent_guard_in_deliver_story_skill (PASS)

---

### AC-005 — Anti-precedent guard in per-story-delivery.md + self-check question in stub-architect.md

**Criterion (guard):** per-story-delivery.md Step 2 must also contain the same verbatim ANTI-PRECEDENT GUARD text (BC-5.38.006 invariant 1 — both files updated atomically).

**Criterion (self-check):** stub-architect.md must contain the verbatim self-check question from BC-5.38.005: "If I include this real implementation, will the test for this function pass trivially without any implementer work?"

**Grep evidence:**
```bash
grep -n 'ANTI-PRECEDENT GUARD:' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
grep -n 'If I include this real implementation' plugins/vsdd-factory/agents/stub-architect.md
```

**Output:**
```
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:18:> **ANTI-PRECEDENT GUARD:** Do not use sibling crates with pre-implemented stubs as templates for your stub work. If you observe that a sibling crate (e.g., a DTU clone or prior story's scaffold) contains full business logic rather than todo!() macros, treat it as a historical anti-pattern. Your stub must use todo!() for all non-trivial function bodies. Anti-precedent evidence: Prism commits aa706543, 6d2d005e, 20b4a12a. Model precedent: e86d03f2.

plugins/vsdd-factory/agents/stub-architect.md:63:**"If I include this real implementation, will the test for this function pass trivially without any implementer work?"**
```

**Source commits:**
- Guard: `8cd16e9` — `feat(S-7.03): add anti-precedent guard + Red Gate Density Check + facade-mode to per-story-delivery.md (Tasks 4,5,6)`
- Self-check: `c4413e1` — `feat(S-7.03): add self-check rule + GREEN-BY-DESIGN/WIRING-EXEMPT protocol to stub-architect.md (Task 8)`

**Bats test results:** ok 5 test_anti_precedent_guard_in_per_story_delivery (PASS), ok 6 test_self_check_question_in_stub_architect_prompt (PASS)

---

### AC-006 — Red Gate Density Check section in per-story-delivery.md

**Criterion:** per-story-delivery.md gains a "Red Gate Density Check (BLOCKING before Step 4)" section between Step 3 and Step 4 containing: RED_RATIO formula, ≥0.5 threshold, remediation Options A and B, and log format.

**Grep evidence:**
```bash
grep -n 'Red Gate Density Check' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
grep -n 'RED_RATIO' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
grep -n '0\.5' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
grep -n 'Option A\|Option B' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
```

**Output:**
```
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:37:If Red Gate fails, dispatch a new test-writer to fix the tests. Do not proceed to Step 3.5 (Red Gate Density Check) or Step 4 until Red Gate is green (i.e., tests are correctly red).
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:41:## Red Gate Density Check (BLOCKING before Step 4)
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:43:After Step 3 Red Gate passes and before dispatching the Step 4 implementer, compute the RED_RATIO density check. This gate is BLOCKING for `tdd_mode: strict` stories (BC-8.29.001).
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:48:RED_RATIO = RED_TESTS / (TOTAL_NEW_TESTS - EXEMPT_TESTS)
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:64:**RED_RATIO ≥ 0.5** (BLOCKING). If `(TOTAL_NEW_TESTS - EXEMPT_TESTS) > 0` and `RED_RATIO < 0.5`, Step 4 dispatch is blocked.
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:93:When RED_RATIO < 0.5 with UNJUSTIFIED GREEN tests present, the orchestrator must choose one of exactly two options before proceeding:
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:95:**Option A (default for automated orchestrators):** Roll back the stub commit and re-dispatch stub-architect with a stricter prompt. Include the explicit list of UNJUSTIFIED functions from the log and instruct stub-architect to replace them with `todo!()`. Step 3 then runs again with the corrected stub. RED_RATIO is recomputed. (BC-8.29.003 EC-001: automated orchestrator MUST default to Option A unless `mutation_testing_required: true` is pre-authorized in story frontmatter.)
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:97:**Option B (accept with mutation obligation):** Accept the low ratio and register `mutation_testing_required: true` in the story frontmatter. The wave gate must run `cargo mutants -p <crate> --jobs $(nproc) --timeout 300` for this story's crate as a compensating control (BC-6.21.001, BC-6.21.002). The PR description must disclose: "RED_RATIO was <value> at Step 3 Red Gate. Mutation testing applied at wave gate as compensating control."
```

**Source commit:** `8cd16e9` — `feat(S-7.03): add anti-precedent guard + Red Gate Density Check + facade-mode to per-story-delivery.md (Tasks 4,5,6)`

**Bats test results:** ok 7 test_red_ratio_threshold_section_present (PASS), ok 8 test_red_ratio_formula_present (PASS), ok 9 test_remediation_options_ab_present (PASS)

---

### AC-007 — validate-red-ratio.sh hook + hooks-registry.toml registration

**Criterion:** A new PostToolUse hook `plugins/vsdd-factory/hooks/validate-red-ratio.sh` enforces the RED_RATIO blocking gate. The hook is registered in `hooks-registry.toml`. The hook blocks (exits non-zero) on RED_RATIO < 0.5 with no exception path, passes on RED_RATIO ≥ 0.5, and passes on `remediation: option_b`.

**Grep evidence:**
```bash
grep -n 'validate-red-ratio' plugins/vsdd-factory/hooks-registry.toml
```

**Output:**
```
plugins/vsdd-factory/hooks-registry.toml:513:name = "validate-red-ratio"
plugins/vsdd-factory/hooks-registry.toml:522:script_path = "plugins/vsdd-factory/hooks/validate-red-ratio.sh"
```

Hook behavior verified by bats tests (j), (k), (l) using live fixture files via stdin — see bats-run.log lines 10-12.

**Source commits:**
- Hook script: `f53bf43` — `feat(S-7.03): add validate-red-ratio.sh PostToolUse hook (Task 15)`
- Registry entry: `3a9614c` — `feat(S-7.03): register validate-red-ratio in hooks-registry.toml + post-migration comment (Task 17)`

**Bats test results:** ok 10 test_validate_red_ratio_blocks_on_low_ratio (PASS), ok 11 test_validate_red_ratio_passes_on_sufficient_ratio (PASS), ok 12 test_validate_red_ratio_passes_on_option_b_election (PASS), ok 13 test_validate_red_ratio_registered_in_hooks_registry (PASS)

---

### AC-008 — tdd_mode field in story-template.md

**Criterion:** story-template.md frontmatter gains a `tdd_mode: strict` field with an inline comment documenting both valid values (strict and facade) and their semantic difference.

**Grep evidence:**
```bash
grep -n 'tdd_mode:' plugins/vsdd-factory/templates/story-template.md
grep -En 'tdd_mode:.*facade' plugins/vsdd-factory/templates/story-template.md
```

**Output:**
```
plugins/vsdd-factory/templates/story-template.md:30:tdd_mode: strict  # strict | facade. strict = full TDD Iron Law enforced (todo!() + Red Gate ≥0.5 required); facade = scaffold+impl combined, mutation testing at wave gate
plugins/vsdd-factory/templates/story-template.md:33:> **tdd_mode:** Absent or unrecognized values default to `strict` per BC-8.30.001 invariant 2 — no existing story is silently promoted to `facade` mode. Set `tdd_mode: facade` only for DTU API clones, mock servers, structural fakes, and config parsing wrappers where the scaffold IS the implementation.
```

Both values (`strict` and `facade`) appear on the same line (line 30). The inline comment documents the semantic difference verbatim per BC-8.30.001.

**Source commit:** `94b653c` — `feat(S-7.03): add tdd_mode field to story-template.md frontmatter (Task 10)`

**Bats test results:** ok 14 test_tdd_mode_field_in_story_template (PASS), ok 15 test_tdd_mode_comment_documents_both_values (PASS)

---

### AC-009 — facade-mode delivery semantics in per-story-delivery.md

**Criterion:** per-story-delivery.md gains a section describing `tdd_mode: facade` modified delivery semantics: combined scaffold+impl Step 2, fidelity tests (GREEN ok), Red Gate bypassed, Step 4 no-op allowed, mutation testing required at wave gate.

**Grep evidence:**
```bash
grep -En 'tdd_mode: facade|facade-mode' plugins/vsdd-factory/workflows/phases/per-story-delivery.md
```

**Output:**
```
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:3:description: Per-story TDD delivery workflow reference. Governs Step 1 through Step 9 of the story delivery cycle including stub discipline, Red Gate density check, and facade-mode semantics. Loaded by the orchestrator during Phase 3 implementation.
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:8:This document is the authoritative reference for per-story TDD delivery within a wave. It governs stub discipline, Red Gate density enforcement, and the facade-mode alternative flow. The `deliver-story` skill (`skills/deliver-story/SKILL.md`) is the entry point; this file is the playbook. If the two disagree, this file wins.
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:138:## tdd_mode: facade — Modified Flow
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:140:When a story has `tdd_mode: facade` in its frontmatter (explicit; facade-mode does NOT activate by default), the per-story delivery workflow operates under modified semantics. This section documents the facade-mode delivery flow (BC-8.30.002).
plugins/vsdd-factory/workflows/phases/per-story-delivery.md:144:### Step 2 (facade-mode)
```

**Source commit:** `8cd16e9` — `feat(S-7.03): add anti-precedent guard + Red Gate Density Check + facade-mode to per-story-delivery.md (Tasks 4,5,6)`

**Bats test result:** ok 16 test_facade_mode_section_present (PASS)

---

### AC-010 — Mutation Testing section in wave-gate SKILL.md

**Criterion:** wave-gate SKILL.md gains a Mutation Testing section that runs `cargo mutants`, writes mutation reports, gates on ≥80% kill rate, and provides disposition table (A/B/C) for surviving mutants.

**Grep evidence:**
```bash
grep -n 'cargo mutants' plugins/vsdd-factory/skills/wave-gate/SKILL.md
grep -n '80' plugins/vsdd-factory/skills/wave-gate/SKILL.md
grep -n 'kill' plugins/vsdd-factory/skills/wave-gate/SKILL.md
```

**Output:**
```
plugins/vsdd-factory/skills/wave-gate/SKILL.md:199:cargo mutants --version 2>/dev/null || echo "NOT_FOUND"
plugins/vsdd-factory/skills/wave-gate/SKILL.md:215:cargo mutants -p <crate> --jobs $(command -v nproc >/dev/null && nproc || sysctl -n hw.ncpu || echo 4) --timeout 300

plugins/vsdd-factory/skills/wave-gate/SKILL.md:177:≥80% kill-rate floor required by BC-6.21.001 and BC-6.21.002.
plugins/vsdd-factory/skills/wave-gate/SKILL.md:241:killed * 100 / total >= 80
plugins/vsdd-factory/skills/wave-gate/SKILL.md:244:If `killed * 100 / total < 80`, the wave gate **fails**. The threshold is exactly
plugins/vsdd-factory/skills/wave-gate/SKILL.md:245:80 — no rounding, no "close enough."

plugins/vsdd-factory/skills/wave-gate/SKILL.md:177:≥80% kill-rate floor required by BC-6.21.001 and BC-6.21.002.
plugins/vsdd-factory/skills/wave-gate/SKILL.md:238:Compute the kill rate using integer-precise arithmetic (avoids float rounding):
plugins/vsdd-factory/skills/wave-gate/SKILL.md:241:killed * 100 / total >= 80
plugins/vsdd-factory/skills/wave-gate/SKILL.md:254:`cargo mutants` to confirm the mutant is now killed. Commit the new test before
```

**Source commit:** `fa07d94` — `feat(S-7.03): add Mutation Testing section to wave-gate SKILL.md (Task 13)`

**Bats test results:** ok 17 test_wave_gate_mutation_section_present (PASS), ok 18 test_wave_gate_mutation_threshold_80_present (PASS)

---

### AC-011 — BATS test suite covers all 18 AC verification tests (aggregate)

**Criterion:** `plugins/vsdd-factory/tests/tdd-discipline-gate.bats` must contain 18 tests organized by Layer 1-5, all passing. This AC is the aggregate verification that the 4-layer defense is fully codified and testable.

**Full bats output:**
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

**Result: 18/18 GREEN. All layers verified.**

**Source commit (bats file):** `020518b` — `test(S-7.03): add 18 failing bats tests for TDD discipline hardening` (RED phase); implementation commits `d89b928` through `121d24c` made all tests GREEN.
