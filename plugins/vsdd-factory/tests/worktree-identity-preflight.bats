#!/usr/bin/env bats
# worktree-identity-preflight.bats — RED-phase prompt-contract tests for
# GitHub issues #169 + #176: worktree-identity engine fix.
#
# ROOT CAUSE: per-story sub-agents (esp. the adversary) read the WRONG git tree
# in a multi-worktree project — either the wrong feature checkout (#176), or
# they treat worktree `.factory/` content as spec ground-truth when it is NOT
# (#169). The corrected model: `.factory/` is gitignored on the product branch,
# so `git worktree add` checks out NOTHING there. Any `.factory/` content in the
# worktree is live shadow-write evidence (issue #523 class), not a stale snapshot.
# Reading it as spec ground-truth produces phantom "absent file / missing
# deliverable" findings; the dangerous false-GREEN inverse applies equally.
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
#     Anchor: the phrase "dispatch-error" paired with HEAD SHA mismatch language,
#     verified within the "Worktree-Identity Preflight" section context.
@test "test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings" {
  # Section-scoped check: extract the Worktree-Identity Preflight section
  # (from "#### Worktree-Identity Preflight" up to the next "####" heading or "---")
  # and verify both dispatch-error AND rev-parse HEAD co-occur within it.
  preflight_section="$(awk '/^#### Worktree-Identity Preflight/{found=1} found{print} /^#### /{if(!found)next; if(found && !/^#### Worktree-Identity Preflight/)exit} /^---/{if(found)exit}' "$ADVERSARY_AGENT")"
  # dispatch-error must appear in the preflight section
  printf '%s\n' "$preflight_section" | grep -i "dispatch-error" >/dev/null
  # rev-parse HEAD OR the SHA comparison concept must appear in the preflight section
  # (adversary checks the EMBEDDED sha, not git directly — but the orchestrator
  # uses rev-parse HEAD; the section must reference this mechanism)
  printf '%s\n' "$preflight_section" | grep -iE "rev-parse HEAD|feature-HEAD-SHA|HEAD SHA" >/dev/null
}

# (c) AC-003: adversary.md must assert that the basename of the embedded
#     worktree-abs-path matches the story-id, and that the mechanism is the
#     porcelain-list / basename resolution (NOT show-toplevel).
#     Anchors: "worktree list" + "--porcelain" + "basename" must co-occur in
#     adversary.md Rule 2.  "show-toplevel" MUST NOT appear as the mechanism
#     description (the prior wrong description is superseded by this finding).
@test "test_BC_adversary_toplevel_basename_must_match_story_id" {
  # The correct mechanism tokens must be present
  run grep -i "worktree list" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  run grep -i "\-\-porcelain" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  run grep -i "basename" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  # The WRONG mechanism token must NOT appear as the primary description
  # (the porcelain-list resolver does NOT use show-toplevel for worktree matching)
  # We allow show-toplevel to appear elsewhere (e.g. legacy comments or SKILL.md
  # cross-refs) but it must NOT appear in the worktree-abs-path derivation clause
  # in adversary.md Rule 2.  Extract just Rule 2 paragraph and verify absence.
  rule2="$(awk '/^2\. \*\*Verify basename/{found=1} found{print; if (/^$/ && found) exit}' "$ADVERSARY_AGENT")"
  # rule2 paragraph must contain porcelain + basename
  printf '%s\n' "$rule2" | grep -i "\-\-porcelain" >/dev/null
  printf '%s\n' "$rule2" | grep -i "basename" >/dev/null
  # and must NOT describe show-toplevel as the derivation mechanism
  if printf '%s\n' "$rule2" | grep -q "rev-parse --show-toplevel"; then
    echo "FAIL: adversary.md Rule 2 still contains 'rev-parse --show-toplevel' as the worktree-abs-path derivation mechanism" >&2
    return 1
  fi
}

# (d) AC-004: adversary.md must mandate absolute worktree-rooted paths for all
#     feature-code/evidence reads; bare-relative and main-checkout reads for
#     feature code are explicitly FORBIDDEN.
#     Anchor: "worktree-rooted" (the exact term the clause must use).
@test "test_BC_adversary_absolute_worktree_rooted_paths_mandatory" {
  run grep -i "worktree-rooted" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
}

# (e) AC-005: adversary.md must assert that spec/ADR/BC ground-truth is ALWAYS read from the
#     CANONICAL repo-root .factory/ (factory-artifacts). git worktree add checks out NOTHING
#     under .factory/ (gitignored); any worktree .factory/ content is live shadow-write evidence.
#     Re-anchored (F-S2104-P3-007): the prior stale-snapshot prohibition assertion locked in
#     a retracted premise and would block the implementer's residue sweep at lines 44/59.
#     The corrected model's positive assertions — already present at adversary.md rule 4 — are:
#       "canonical-repo-root" (authoritative path anchor for all spec reads), AND
#       "checks out NOTHING under" (explains why no shadow .factory/ is created at worktree-add).
#     Both pass NOW and keep passing after implementer sweeps stale residue at lines 44/59.
@test "test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts" {
  run grep -i "factory-artifacts" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  # Positive assertions of the corrected model (ground-truth-from-canonical phrasing, rule 4):
  run grep -i "canonical-repo-root" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  run grep -i "checks out NOTHING under" "$ADVERSARY_AGENT"
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
  [ "$status" -eq 0 ]
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
#     the "Worktree-Identity Preflight" subsection context.
@test "test_BC_adv_review_skill_adversary_must_assert_triple_before_findings" {
  # Section-scoped check: extract the Worktree-Identity Preflight subsection
  # (from "## Worktree-Identity Preflight" up to the next "##" heading)
  # and verify ASSERT appears within it.
  preflight_section="$(awk '/^## Worktree-Identity Preflight/{found=1} found{print} /^## /{if(!found)next; if(found && !/^## Worktree-Identity Preflight/)exit}' "$ADV_REVIEW_SKILL")"
  # ASSERT must appear in the Worktree-Identity Preflight section
  printf '%s\n' "$preflight_section" | grep -iE "\bASSERT\b" >/dev/null
}

# ============================================================
# File (C): plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md
# Required: a note mandating canonical repo-root absolute paths for specs;
# git worktree add checks out NOTHING under .factory/ (gitignored); any worktree
# .factory/ content is live shadow-write evidence and off-limits for spec reads.
# ============================================================

# (k) AC-011: _shared-context.md must contain the dispatch-context discipline
#     language stating that spec/BC/ADR files must be CANONICAL repo-root
#     absolute paths.
#     Anchor: "canonical repo-root" (the exact phrase).
@test "test_BC_shared_context_spec_paths_must_be_canonical_repo_root" {
  run grep -i "canonical repo-root" "$SHARED_CTX"
  [ "$status" -eq 0 ]
}

# (l) AC-012: _shared-context.md must explicitly state that worktree `.factory/`
#     content is off-limits for spec ground-truth. Corrected model (issue #523 +
#     BC-6.26.001 Invariant 5): `.factory/` is gitignored on the product branch,
#     so `git worktree add` checks out NOTHING there. Any `.factory/` content in
#     the worktree is live shadow-write evidence, not a stale snapshot. Using it
#     as spec ground-truth produces hallucinated "absent file" findings for specs
#     that exist only on factory-artifacts (#169). The "off-limits" mandate is
#     still correct — the reason is live-shadow evidence rather than staleness.
#     Anchor: "off-limits" in the worktree .factory/ prohibition clause.
@test "test_BC_shared_context_worktree_factory_live_shadow_content_off_limits" {
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
