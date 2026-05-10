#!/usr/bin/env bats
#
# per-story-adversary-workflow.bats
#
# Failing tests for S-12.01: Per-Story Adversary Workflow
# Step 4.5, Scope Contract, and Agent Doc Updates.
#
# All tests assert on the EXPECTED post-Step-4 state of:
#   - plugins/vsdd-factory/workflows/phases/per-story-delivery.md
#   - plugins/vsdd-factory/workflows/phases/phase-3-tdd-implementation.lobster
#   - plugins/vsdd-factory/agents/adversary.md
#   - plugins/vsdd-factory/skills/wave-gate/SKILL.md
#   - plugins/vsdd-factory/agents/orchestrator/orchestrator.md
#   - plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md
#
# Every test MUST FAIL today because Step 4 has not yet modified the files.
#
# AC traces:
#   AC-001 -> BC-5.39.001 PC1  (Step 4.5 insertion + sequencing)
#   AC-002 -> BC-5.39.001 PC2+PC5  (convergence criterion + state file schema)
#   AC-003 -> BC-5.39.002 PC1  (Three-Perimeter Scope Contract section)
#   AC-004 -> BC-5.39.002 PC2+INV2  (deferred-finding categories)
#   AC-005 -> BC-5.39.001 PC1  (Lobster workflow step wiring)
#   AC-006 -> BC-5.39.002 PC1+PC7  (wave-gate Gate 3 scope narrowing)
#   AC-007 -> BC-5.39.001  (orchestrator MANDATORY STEPS reconciliation)
#   AC-008 -> BC-5.39.001  (orchestrator/per-story-delivery.md parity)
#

setup() {
    # Resolve REPO_ROOT portably from the test file's location so the test
    # works in CI checkouts, operator clones, and feature worktrees.
    # Earlier versions hard-coded /Users/jmagady/.../worktrees/S-12.01 which
    # broke release CI and silently kept rc.11+ off the marketplace.
    REPO_ROOT="$(cd "$BATS_TEST_DIRNAME/../../.." && pwd)"
    PER_STORY_DELIVERY="$REPO_ROOT/plugins/vsdd-factory/workflows/phases/per-story-delivery.md"
    PHASE_3_LOBSTER="$REPO_ROOT/plugins/vsdd-factory/workflows/phases/phase-3-tdd-implementation.lobster"
    ADVERSARY_AGENT="$REPO_ROOT/plugins/vsdd-factory/agents/adversary.md"
    WAVE_GATE_SKILL="$REPO_ROOT/plugins/vsdd-factory/skills/wave-gate/SKILL.md"
    ORCHESTRATOR="$REPO_ROOT/plugins/vsdd-factory/agents/orchestrator/orchestrator.md"
    ORCH_PER_STORY="$REPO_ROOT/plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md"
}

# =============================================================================
# AC-001 | BC-5.39.001 PC1 — Step 4.5 insertion + sequencing
# =============================================================================

@test "AC-001 BC-5.39.001 PC1: per-story-delivery.md has Step 4.5 header" {
    # After Step 4, a '## Step 4.5' heading must exist in per-story-delivery.md.
    run grep -c "^## Step 4.5" "$PER_STORY_DELIVERY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-001 BC-5.39.001 PC1: Step 4.5 appears between Step 4 and Step 5 (line ordering)" {
    # Step 4.5 must come after Step 4 and before Step 5.
    local line_4 line_45 line_5
    line_4=$(grep -n "^## Step 4 " "$PER_STORY_DELIVERY" | head -1 | cut -d: -f1)
    line_45=$(grep -n "^## Step 4.5" "$PER_STORY_DELIVERY" | head -1 | cut -d: -f1)
    line_5=$(grep -n "^## Step 5 " "$PER_STORY_DELIVERY" | head -1 | cut -d: -f1)
    # All three lines must exist and be in ascending order.
    [ -n "$line_4" ]
    [ -n "$line_45" ]
    [ -n "$line_5" ]
    [ "$line_4"  -lt "$line_45" ]
    [ "$line_45" -lt "$line_5"  ]
}

@test "AC-001 BC-5.39.001 PC1: per-story-delivery.md references BC-5.39.001 by ID" {
    run grep -c "BC-5.39.001" "$PER_STORY_DELIVERY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-001 BC-5.39.001 PC1: per-story-delivery.md references BC-5.39.002 by ID" {
    run grep -c "BC-5.39.002" "$PER_STORY_DELIVERY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-001 BC-5.39.001 PC6: Step 5 explicitly blocks on convergence criterion" {
    # Step 5 must contain language forbidding execution while convergence is not cleared.
    # BC-5.39.001 PC6: Step 5 MUST NOT execute while passes_clean < 3 or
    # last_classification != NITPICK_ONLY.
    run grep -c "MUST NOT" "$PER_STORY_DELIVERY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

# =============================================================================
# AC-002 | BC-5.39.001 PC2+PC5 — convergence criterion + state file schema
# =============================================================================

@test "AC-002 BC-5.39.001 PC5: per-story-delivery.md documents passes_clean >= 3 criterion" {
    run grep -c "passes_clean" "$PER_STORY_DELIVERY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-002 BC-5.39.001 PC5: per-story-delivery.md documents 3 clean passes minimum" {
    # The criterion requires >= 3 consecutive clean passes.
    run grep -Ec "passes_clean.{0,10}3|3.{0,20}clean.{0,20}pass|minimum.{0,10}3" "$PER_STORY_DELIVERY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-002 BC-5.39.001 PC2: per-story-delivery.md cites NITPICK_ONLY classification" {
    run grep -c "NITPICK_ONLY" "$PER_STORY_DELIVERY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-002 BC-5.39.001 PC2: per-story-delivery.md references adversary-convergence-state.json schema path" {
    # The state file path pattern must appear: .factory/cycles/<...>/adversary-convergence-state.json
    run grep -c "adversary-convergence-state.json" "$PER_STORY_DELIVERY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

# =============================================================================
# AC-003 | BC-5.39.002 PC1 — Three-Perimeter Scope Contract section in adversary.md
# =============================================================================

@test "AC-003 BC-5.39.002 PC1: adversary.md has Three-Perimeter Scope Contract section" {
    # A section heading containing 'Three-Perimeter' (or 'three-perimeter') must exist.
    run grep -Eic "three-perimeter" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-003 BC-5.39.002 PC1: adversary.md Three-Perimeter section cites ADR-017" {
    run grep -c "ADR-017" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-003 BC-5.39.002 PC1: adversary.md cites BC-5.39.002 by ID" {
    run grep -c "BC-5.39.002" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-003 BC-5.39.002 PC1: adversary.md documents per-story scope bounded to diff and spec" {
    # The section must reference the story worktree diff and story spec.
    run grep -Eic "per-story" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-003 BC-5.39.002 PC1: adversary.md documents wave-gate scope (cross-story integration)" {
    # The Three-Perimeter section must describe the wave-gate perimeter covering
    # cross-story and integration concerns.
    run grep -Eic "wave-gate" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-003 BC-5.39.002 PC1: adversary.md documents Phase-5 whole-system scope in scope contract context" {
    # The Three-Perimeter section must describe the Phase-5 perimeter for system-level /
    # architectural findings.  The term must appear together with 'three-perimeter'
    # or 'scope' or 'deferred' to distinguish it from the existing 'Implementation Review
    # (Phase 5)' heading already in the file.
    run grep -Eic "three-perimeter|scope contract" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
    # The section referencing Phase-5 as a *routing target* (not just a heading label)
    # must also exist: the word 'phase-5' (hyphenated, BC-5.39.001 / BC-5.39.002 style)
    # must appear in adversary.md, which it does NOT today.
    run grep -c "phase-5" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

# =============================================================================
# AC-004 | BC-5.39.002 PC2+INV2 — deferred-finding categories
# =============================================================================

@test "AC-004 BC-5.39.002 PC2: adversary.md names cross-story deferred category" {
    run grep -c "cross-story" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-004 BC-5.39.002 PC2: adversary.md names integration as a deferred-finding category" {
    # 'integration' must appear as a named deferred category alongside the other three
    # categories (cross-story, system-level, architectural) in the scope-contract context.
    # The existing adversary.md uses 'integration' only in the generic VP-INDEX line;
    # the new section must add it alongside deferred_findings / scope-contract language.
    # Require 'integration' to co-occur with 'deferred' on nearby lines.
    run grep -A5 -B5 "integration" "$ADVERSARY_AGENT"
    echo "$output" | grep -q "deferred"
}

@test "AC-004 BC-5.39.002 PC2: adversary.md names system-level deferred category" {
    run grep -c "system-level" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-004 BC-5.39.002 PC2: adversary.md names architectural as a deferred-finding category" {
    # 'architectural' must appear in the deferred-finding categories context.
    # The existing adversary.md only uses it in "architectural layer" (fix propagation);
    # the new section must use it as a deferred-category label alongside deferred_findings.
    run grep -A5 -B5 "architectural" "$ADVERSARY_AGENT"
    echo "$output" | grep -q "deferred"
}

@test "AC-004 BC-5.39.002 PC2: adversary.md documents deferred_findings JSON field" {
    run grep -c "deferred_findings" "$ADVERSARY_AGENT"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

# =============================================================================
# AC-005 | BC-5.39.001 PC1 — Lobster workflow step wiring (phase-3)
# =============================================================================

@test "AC-005 BC-5.39.001 PC1: phase-3 lobster has adversary-convergence step" {
    run grep -c "name: adversary-convergence" "$PHASE_3_LOBSTER"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-005 BC-5.39.001 PC1: phase-3 lobster has backup-adversary-convergence step" {
    run grep -c "name: backup-adversary-convergence" "$PHASE_3_LOBSTER"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-005 BC-5.39.001 PC1: adversary-convergence depends_on backup-implement" {
    # The adversary-convergence step's depends_on must list backup-implement.
    # Parse via yq/python since lobster is YAML.
    # Strategy: find the adversary-convergence block and read depends_on on next line.
    local block
    block=$(awk '/name: adversary-convergence/{found=1} found && /depends_on/{print; exit}' "$PHASE_3_LOBSTER")
    echo "depends_on block: $block"
    [[ "$block" == *"backup-implement"* ]]
}

@test "AC-005 BC-5.39.001 PC1: record-demos depends_on backup-adversary-convergence" {
    # After Step 4.5 is inserted, record-demos must depend on backup-adversary-convergence,
    # not directly on backup-implement.
    local block
    block=$(awk '/name: record-demos/{found=1} found && /depends_on/{print; exit}' "$PHASE_3_LOBSTER")
    echo "depends_on block: $block"
    [[ "$block" == *"backup-adversary-convergence"* ]]
}

@test "AC-005 BC-5.39.001: phase-3 lobster total step count is 18 after additions" {
    # Currently 16 steps; adding adversary-convergence + backup-adversary-convergence = 18.
    local count
    count=$(grep -c "^    - name:" "$PHASE_3_LOBSTER")
    echo "Step count: $count"
    [ "$count" -eq 18 ]
}

# =============================================================================
# AC-006 | BC-5.39.002 PC1+PC7 — wave-gate Gate 3 scope narrowing
# =============================================================================

@test "AC-006 BC-5.39.002 PC7: wave-gate SKILL.md Gate 3 references per-story prerequisite" {
    # Gate 3 text must note that per-story convergence (Step 4.5) is a prerequisite.
    run grep -Eic "per-story.{0,40}prerequisite|prerequisite.{0,40}per-story|step 4\.5.{0,40}gate 3|gate 3.{0,40}step 4\.5" "$WAVE_GATE_SKILL"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-006 BC-5.39.002 PC7: wave-gate SKILL.md Gate 3 section explicitly narrows to cross-story concerns" {
    # Gate 3 must explicitly restrict its scope to cross-story / integration findings.
    # The SKILL.md already has a 'Gate 3: Adversarial Review' section but it does NOT
    # yet contain the narrowing language.  Require 'cross-story' to appear specifically
    # WITHIN the Gate 3 block — not just anywhere in the file.
    # Strategy: extract lines between the Gate 3 heading and the next Gate heading.
    local gate3_block
    gate3_block=$(awk '/### Gate 3:/{found=1; next} found && /^###/{exit} found{print}' "$WAVE_GATE_SKILL")
    echo "$gate3_block" | grep -q "cross-story"
}

@test "AC-006 BC-5.39.002 PC7: wave-gate SKILL.md Gate 3 cites deferred_findings aggregation as input" {
    # Gate 3 should note that deferred_findings from story-level passes are the input.
    run grep -c "deferred_findings" "$WAVE_GATE_SKILL"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

# =============================================================================
# AC-007 | BC-5.39.001 — orchestrator MANDATORY STEPS reconciliation
# =============================================================================

@test "AC-007 BC-5.39.001: orchestrator.md MANDATORY STEPS references Step 4.5" {
    # The MANDATORY STEPS section must reference 'Step 4.5' or 'step 4.5'.
    run grep -Eic "step 4\.5" "$ORCHESTRATOR"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-007 BC-5.39.001: orchestrator.md MANDATORY STEPS per-story entry cites Step 4.5 specifically" {
    # The MANDATORY STEPS entry for per-story adversarial convergence must explicitly
    # say 'Step 4.5' (not just reference per-story-delivery.md as a filename).
    # Today orchestrator.md says "always 3 clean passes minimum per story" with no Step 4.5 anchor.
    run grep -Eic "step 4\.5" "$ORCHESTRATOR"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

# =============================================================================
# AC-008 | BC-5.39.001 — orchestrator/per-story-delivery.md parity update
# =============================================================================

@test "AC-008 BC-5.39.001: orchestrator/per-story-delivery.md contains Step 4.5 reference" {
    # This file summarizes workflow steps (confirmed in Step 2 reconnaissance).
    # After S-12.01, it must include adversary convergence as step 3d-or-equivalent.
    run grep -Eic "4\.5|adversary.{0,30}convergence|step.*adversary" "$ORCH_PER_STORY"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}
