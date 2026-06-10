#!/usr/bin/env bats
# worktree-identity-preflight.bats — RED-phase prompt-contract tests for
# GitHub issues #169 + #176: worktree-identity engine fix.
#
# ROOT CAUSE: per-story sub-agents (esp. the adversary) read the WRONG git tree
# in a multi-worktree project — either a stale .factory/specs worktree snapshot
# (#169) or the wrong feature checkout (#176) — producing phantom "absent file /
# missing deliverable" findings and the dangerous false-GREEN inverse.
#
# These tests assert that the four target prompt/skill files contain mandatory
# discipline clauses. Every test MUST FAIL on develop@89fbe2d6 (pre-fix).
# The implementer makes them pass by adding the required clauses verbatim.
#
# Traces to: issues #169, #176
# ACs: (a)-(n) below — 14 total assertions across 4 files.
#
# Run from the worktree root:
#   bats plugins/vsdd-factory/tests/worktree-identity-preflight.bats

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  ADVERSARY_AGENT="$PLUGIN_ROOT/agents/adversary.md"
  ADV_REVIEW_SKILL="$PLUGIN_ROOT/skills/adversarial-review/SKILL.md"
  SHARED_CTX="$PLUGIN_ROOT/skills/deliver-story/steps/_shared-context.md"
  STEP_D5="$PLUGIN_ROOT/skills/deliver-story/steps/step-d5-adversary-convergence.md"
}

# ============================================================
# File (A): plugins/vsdd-factory/agents/adversary.md
# Required: a "Worktree-Identity Preflight" discipline section
# in the Perimeter-1 scope contract with 6 specific clauses.
# ============================================================

# (a) AC-001: adversary.md must contain the heading
#     "Worktree-Identity Preflight" establishing the discipline block.
@test "test_BC_adversary_worktree_identity_preflight_heading_present" {
  run grep -i "Worktree-Identity Preflight" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
}

# (b) AC-002: adversary.md must assert that the worktree HEAD SHA must equal
#     the dispatched feature HEAD SHA; mismatch must cause STOP + dispatch-error.
#     Anchor: the phrase "dispatch-error" paired with HEAD SHA mismatch language.
@test "test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings" {
  run grep -i "dispatch-error" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  # Also verify the HEAD SHA comparison verb is present (rev-parse HEAD)
  run grep -i "rev-parse HEAD" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
}

# (c) AC-003: adversary.md must assert basename of show-toplevel must match
#     the dispatched story id / target.
#     Anchor: "show-toplevel" paired with "story" identity verification.
@test "test_BC_adversary_toplevel_basename_must_match_story_id" {
  run grep -i "show-toplevel" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
}

# (d) AC-004: adversary.md must mandate absolute worktree-rooted paths for all
#     feature-code/evidence reads; bare-relative and main-checkout reads for
#     feature code are explicitly FORBIDDEN.
#     Anchor: "worktree-rooted" (the exact term the clause must use).
@test "test_BC_adversary_absolute_worktree_rooted_paths_mandatory" {
  run grep -i "worktree-rooted" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
}

# (e) AC-005: adversary.md must state that spec/ADR/BC ground-truth MUST be
#     read from the CANONICAL repo-root .factory/ (factory-artifacts), NOT the
#     stale worktree .factory/specs snapshot.
#     Anchor: "factory-artifacts" paired with prohibition on worktree snapshot.
@test "test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts" {
  run grep -i "factory-artifacts" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  # Verify the stale-snapshot prohibition language is present
  run grep -iE "stale.*worktree|worktree.*stale|stale.*snapshot|snapshot.*stale" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
}

# (f) AC-006: adversary.md must mandate case-insensitive ID-bearing globs
#     (adr/ADR, bc/BC).
#     Anchor: "case-insensitive" in the context of file-matching globs.
@test "test_BC_adversary_id_bearing_globs_must_be_case_insensitive" {
  run grep -i "case-insensitive" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
}

# (g) AC-007: adversary.md must require path-corroboration before reporting
#     an "absent file / missing deliverable / missing ADR" finding.
#     Anchor: "path-corroborated" — the implementer must use this exact term.
@test "test_BC_adversary_absent_file_finding_requires_path_corroboration" {
  run grep -i "path-corroborated" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  # And the class of findings it applies to must be named
  run grep -iE "absent file|missing deliverable|missing ADR" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
}

# ============================================================
# File (B): plugins/vsdd-factory/skills/adversarial-review/SKILL.md
# Required: "Worktree-Identity Preflight (MANDATORY)" subsection
# requiring the orchestrator to pass the triple + adversary to ASSERT it.
# ============================================================

# (h) AC-008: adversarial-review SKILL.md must contain the subsection heading
#     "Worktree-Identity Preflight (MANDATORY)" (case-insensitive on the word
#     MANDATORY is acceptable; the heading text is the contract).
@test "test_BC_adv_review_skill_has_worktree_identity_preflight_mandatory_section" {
  run grep -i "Worktree-Identity Preflight" "$ADV_REVIEW_SKILL"
  [ "$status" -eq 0 ]
  run grep -iE "MANDATORY|mandatory" "$ADV_REVIEW_SKILL"
  # grep for "MANDATORY" is broad; narrow to same section by checking both on same pass
  run grep -i "Worktree-Identity Preflight (MANDATORY)" "$ADV_REVIEW_SKILL"
  [ "$status" -eq 0 ]
}

# (i) AC-009: adversarial-review SKILL.md must document the
#     (worktree-abs-path, feature-HEAD-SHA, story-id) triple that the
#     orchestrator must pass into the adversary dispatch.
#     Anchor: "worktree-abs-path" (the exact triple element name).
@test "test_BC_adv_review_skill_dispatch_triple_worktree_abs_path" {
  run grep -i "worktree-abs-path" "$ADV_REVIEW_SKILL"
  [ "$status" -eq 0 ]
}

# (j) AC-010: adversarial-review SKILL.md must document that the adversary
#     must ASSERT the triple before producing findings.
#     Anchor: "ASSERT" (uppercased to match the imperative form) within
#     the preflight subsection context.
@test "test_BC_adv_review_skill_adversary_must_assert_triple_before_findings" {
  # Use case-insensitive so the implementer has latitude in prose styling
  run grep -iE "\bASSERT\b" "$ADV_REVIEW_SKILL"
  [ "$status" -eq 0 ]
}

# ============================================================
# File (C): plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md
# Required: a note mandating canonical repo-root absolute paths for specs;
# the worktree .factory/specs snapshot is stale and off-limits.
# ============================================================

# (k) AC-011: _shared-context.md must contain the dispatch-context discipline
#     language stating that spec/BC/ADR files must be CANONICAL repo-root
#     absolute paths.
#     Anchor: "canonical repo-root" (the exact phrase).
@test "test_BC_shared_context_spec_paths_must_be_canonical_repo_root" {
  run grep -i "canonical repo-root" "$SHARED_CTX"
  [ "$status" -eq 0 ]
}

# (l) AC-012: _shared-context.md must explicitly state that the worktree
#     .factory/specs snapshot is stale and off-limits for spec ground-truth.
#     Anchor: "off-limits" in the worktree-snapshot prohibition.
@test "test_BC_shared_context_worktree_factory_specs_snapshot_is_off_limits" {
  run grep -i "off-limits" "$SHARED_CTX"
  [ "$status" -eq 0 ]
}

# ============================================================
# File (D): plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md
# Required: Step-1 adversary dispatch must embed the expected feature HEAD SHA
# + absolute worktree path and require the preflight assertion to pass.
# ============================================================

# (m) AC-013: step-d5-adversary-convergence.md must require embedding the
#     expected feature HEAD SHA in the adversary dispatch.
#     Anchor: "feature HEAD SHA" — the exact phrase for the SHA the adversary
#     must verify equals its own git rev-parse HEAD.
@test "test_BC_step_d5_dispatch_must_embed_feature_head_sha" {
  run grep -i "feature HEAD SHA" "$STEP_D5"
  [ "$status" -eq 0 ]
}

# (n) AC-014: step-d5-adversary-convergence.md must require the preflight
#     assertion to PASS before findings are accepted.
#     Anchor: "preflight assertion" (exact phrase pairing the step-D5 dispatch
#     clause to the adversary.md contract).
@test "test_BC_step_d5_preflight_assertion_must_pass_before_findings" {
  run grep -i "preflight assertion" "$STEP_D5"
  [ "$status" -eq 0 ]
}
