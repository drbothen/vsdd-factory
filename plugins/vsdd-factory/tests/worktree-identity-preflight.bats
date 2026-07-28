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
  # F-S2104-P22-006(a): bare text search allows a prose mention like "see Worktree-Identity
  # Preflight below" in a paragraph to satisfy the gate without establishing an actual section
  # heading. The discipline block is established by a heading; a prose cross-reference is not
  # sufficient. Require the heading marker prefix ^#{3,4}[[:space:]].
  # MUTANT: file contains only "see Worktree-Identity Preflight below" (no heading) → grep -E fails → RED ✓.
  # CONTROL: "#### Worktree-Identity Preflight" heading present → GREEN ✓.
  run grep -E '^#{3,4}[[:space:]].*Worktree-Identity Preflight' "$ADVERSARY_AGENT"
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
  # F-S2104-P22-006(c): prior whole-file greps for "worktree list", "--porcelain", "basename"
  # allow any incidental mention (e.g. a SKILL.md cross-ref or changelog note) to satisfy
  # the gate. Rule 2 is the specific clause that mandates the porcelain-list/basename mechanism;
  # only presence WITHIN Rule 2 matters. Move all three token checks into the rule2 section extract.
  rule2="$(awk '/^2\. \*\*Verify basename/{found=1} found{print; if (/^$/ && found) exit}' "$ADVERSARY_AGENT")"
  # rule2 paragraph must contain all three mechanism tokens
  printf '%s\n' "$rule2" | grep -i "worktree list" >/dev/null
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
  # F-S2104-P22-006(d): bare presence allows "formerly worktree-rooted (retired)" or
  # "worktree-rooted discipline does not apply to adversary" to satisfy the gate while
  # semantically REVOKING the requirement. Add nullification guard: "worktree-rooted"
  # must not appear in a retired/rescinded/no-longer/does-not-apply context.
  # MUTANT: "formerly worktree-rooted (superseded by §G.3 mechanism)" → nullification RED ✓.
  # CONTROL: "All feature-code reads MUST use worktree-rooted absolute paths" → GREEN ✓.
  run grep -i "worktree-rooted" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  local worktree_rooted_line
  worktree_rooted_line="$(grep -i "worktree-rooted" "$ADVERSARY_AGENT" | head -1)"
  if printf '%s\n' "$worktree_rooted_line" | grep -qiE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\boutside\b|\bnot required\b'; then
    echo "DOC-PARITY FAIL [adversary.md: 'worktree-rooted' appears in nullification context (F-S2104-P22-006(d))]: term found in: $worktree_rooted_line"
    false
  fi
}

# (e) AC-005: adversary.md must assert that spec/ADR/BC ground-truth is ALWAYS read from the
#     CANONICAL repo-root .factory/ (factory-artifacts). git worktree add checks out NOTHING
#     under .factory/ (gitignored); any worktree .factory/ content is live shadow-write evidence.
#     Re-anchored (F-S2104-P3-007): the prior stale-snapshot prohibition assertion locked in
#     a retracted premise and would have blocked the implementer's residue sweep in the
#     adversary.md §Worktree-Identity Preflight opening paragraph and rule 6 SPEC/ADR/BC/VP bullet.
#     The corrected model's positive assertions — already present at adversary.md rule 4 — are:
#       "canonical-repo-root" (authoritative path anchor for all spec reads), AND
#       "checks out NOTHING under" (explains why no shadow .factory/ is created at worktree-add).
#     Both assertions hold at HEAD — the implementer swept stale residue from the
#     adversary.md §Worktree-Identity Preflight opening paragraph and rule 6 SPEC/ADR/BC/VP bullet.
@test "test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts" {
  # F-S2104-P22-006(e): nullification guards for all three corrected-model tokens.
  # A mutant could write "factory-artifacts (no longer required)" or
  # "canonical-repo-root: this mechanism is superseded" and still pass a bare grep.
  # MUTANT: "canonical-repo-root: retired — see updated §G.4" → nullification RED ✓.
  # CONTROL: "read BC/ADR ground-truth from canonical-repo-root .factory/" → GREEN ✓.
  run grep -i "factory-artifacts" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  run grep -i "canonical-repo-root" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  run grep -i "checks out NOTHING under" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  # Nullification guards: none of the three tokens may appear in a retired/rescinded context
  local fa_line cr_line co_line
  fa_line="$(grep -i "factory-artifacts" "$ADVERSARY_AGENT" | grep -iv 'factory-artifacts branch\|origin.*factory-artifacts\|push.*factory-artifacts' | head -1 || true)"
  cr_line="$(grep -i "canonical-repo-root" "$ADVERSARY_AGENT" | head -1)"
  co_line="$(grep -i "checks out NOTHING under" "$ADVERSARY_AGENT" | head -1)"
  for line in "$fa_line" "$cr_line" "$co_line"; do
    if printf '%s\n' "$line" | grep -qiE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\bnot required\b'; then
      echo "DOC-PARITY FAIL [adversary.md: corrected-model token appears in nullification context (F-S2104-P22-006(e))]: $line"
      false
    fi
  done
}

# (f) AC-006: adversary.md must mandate case-insensitive ID-bearing globs
#     (adr/ADR, bc/BC).
#     Anchor: "case-insensitive" in the context of file-matching globs.
@test "test_BC_adversary_id_bearing_globs_must_be_case_insensitive" {
  # F-S2104-P22-006(f): bare presence allows "case-insensitive: this is no longer required"
  # or "not case-insensitive" to pass. Add nullification guard.
  # MUTANT: "case-insensitive matching was formerly required (retired)" → nullification RED ✓.
  # CONTROL: "use case-insensitive globs for adr/ADR, bc/BC" → GREEN ✓.
  run grep -i "case-insensitive" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  local ci_line
  ci_line="$(grep -i "case-insensitive" "$ADVERSARY_AGENT" | head -1)"
  if printf '%s\n' "$ci_line" | grep -qiE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot required\b|\bnot applicable\b|\bnot case-insensitive\b'; then
    echo "DOC-PARITY FAIL [adversary.md: 'case-insensitive' appears in nullification context (F-S2104-P22-006(f))]: $ci_line"
    false
  fi
}

# (g) AC-007: adversary.md must require path-corroboration before reporting
#     an "absent file / missing deliverable / missing ADR" finding.
#     Anchor: "path-corroborated" — the implementer must use this exact term.
@test "test_BC_adversary_absent_file_finding_requires_path_corroboration" {
  # F-S2104-P22-006(g): scope-restriction guard for "path-corroborated". A bare grep passes
  # even if the sentence restricts the mandate with "not applicable", "does not apply to",
  # "is not required for", or "outside the scope of" — turning the requirement into an
  # exception rather than a universal mandate.
  # MUTANT: "path-corroborated findings: does not apply to ADR checks" → scope-restriction RED ✓.
  # CONTROL: "absent-file finding MUST be path-corroborated before report" → GREEN ✓.
  run grep -i "path-corroborated" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  # And the class of findings it applies to must be named
  run grep -iE "absent file|missing deliverable|missing ADR" "$ADVERSARY_AGENT"
  [ "$status" -eq 0 ]
  # Scope-restriction guard
  local pc_line
  pc_line="$(grep -i "path-corroborated" "$ADVERSARY_AGENT" | head -1)"
  if printf '%s\n' "$pc_line" | grep -qiE '\bnot applicable\b|\bdoes not\b|\bis not\b|\boutside\b|\bnot required\b|\bexcept\b|\bexempt\b'; then
    echo "DOC-PARITY FAIL [adversary.md: 'path-corroborated' appears in scope-restriction context (F-S2104-P22-006(g))]: $pc_line"
    false
  fi
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
  # F-S2104-P22-006(h): require actual heading form for the SKILL.md section. The prior split
  # into two separate greps — one for "Worktree-Identity Preflight" and one for "MANDATORY" —
  # allows a file that has these tokens in separate, unrelated paragraphs to pass. Additionally,
  # the final combined grep for "Worktree-Identity Preflight (MANDATORY)" used grep -i which
  # passes even if the heading marker is absent (prose cross-reference satisfies it). Require
  # the heading marker prefix and combined form on the same line.
  # MUTANT: only has "## Worktree-Identity Preflight" (no MANDATORY) and "MANDATORY" elsewhere → split greps pass but combined heading form fails → RED ✓.
  # CONTROL: "## Worktree-Identity Preflight (MANDATORY)" heading present → GREEN ✓.
  run grep -E '^#{1,4}[[:space:]].*Worktree-Identity Preflight[[:space:]]+\(MANDATORY\)' "$ADV_REVIEW_SKILL"
  [ "$status" -eq 0 ]
}

# (i) AC-009: adversarial-review SKILL.md must document the
#     (worktree-abs-path, feature-HEAD-SHA, story-id) triple that the
#     orchestrator must pass into the adversary dispatch.
#     Anchor: "worktree-abs-path" (the exact triple element name).
@test "test_BC_adv_review_skill_dispatch_triple_worktree_abs_path" {
  # F-S2104-P22-006(i): section-scope to the Worktree-Identity Preflight subsection.
  # A bare whole-file grep passes even if "worktree-abs-path" appears only in a footnote
  # or legacy comment unrelated to the dispatch triple. The triple must be documented
  # within the Worktree-Identity Preflight subsection where the dispatch contract lives.
  # MUTANT: "worktree-abs-path" only in a comment outside the preflight section → section scope fails → RED ✓.
  # CONTROL: "worktree-abs-path" present in "## Worktree-Identity Preflight (MANDATORY)" section → GREEN ✓.
  preflight_section="$(awk '/^## Worktree-Identity Preflight/{found=1} found{print} /^## /{if(!found)next; if(found && !/^## Worktree-Identity Preflight/)exit}' "$ADV_REVIEW_SKILL")"
  printf '%s\n' "$preflight_section" | grep -i "worktree-abs-path" >/dev/null
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
  # ASSERT must appear uppercased in the Worktree-Identity Preflight section.
  # F-S2104-P22-011: the prior gate used grep -iE which permits lowercase 'assert', allowing
  # any assertion helper call (e.g. `assert_output`) to satisfy the contract. The comment
  # says "ASSERT (uppercased to match imperative form)" — the -i flag contradicts this.
  # Drop -i: only uppercase ASSERT satisfies the mandatory-imperative-form contract.
  # MUTANT: section contains only `assert_output "$triple"` (lowercase) → grep -E fails → RED ✓.
  # CONTROL: section contains "ASSERT the triple before producing findings" → GREEN ✓.
  printf '%s\n' "$preflight_section" | grep -E "\bASSERT\b" >/dev/null
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
  # F-S2104-P22-006(k): section-scope to the Spec-Path Discipline section (or Write Discipline /
  # equivalent path-anchor section). A bare whole-file grep passes if "canonical repo-root"
  # appears in any comment, changelog, or footnote. The requirement is that it appears in the
  # dispatch-context discipline section where agents would read it during execution.
  # MUTANT: "canonical repo-root" only in a header comment (# former: canonical repo-root paths) → section scope fails → RED ✓.
  # CONTROL: "canonical repo-root" in the Spec-Path Discipline section → GREEN ✓.
  spec_path_section="$(awk '/^#{1,4}[[:space:]].*[Ss]pec[-[:space:]][Pp]ath|^#{1,4}[[:space:]].*[Ww]rite[[:space:]][Dd]iscipline|^#{1,4}[[:space:]].*[Pp]ath[[:space:]][Dd]iscipline/{found=1} found{print} /^#{1,4}[[:space:]]/{if(!found)next; if(found && !/[Ss]pec[-[:space:]][Pp]ath|[Ww]rite[[:space:]][Dd]iscipline|[Pp]ath[[:space:]][Dd]iscipline/)exit}' "$SHARED_CTX")"
  if [ -n "$spec_path_section" ]; then
    printf '%s\n' "$spec_path_section" | grep -i "canonical repo-root" >/dev/null
  else
    # Fallback: whole-file check if section extraction yields nothing (heading name may differ)
    run grep -i "canonical repo-root" "$SHARED_CTX"
    [ "$status" -eq 0 ]
  fi
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
  # F-S2104-P22-006(l): negation guard for "off-limits". A mutant could write "not off-limits"
  # or "no longer off-limits" to reverse the prohibition while still satisfying a bare grep.
  # The negation guard filters out lines where "off-limits" is governed by "not", "no longer",
  # or "exempt from".
  # MUTANT: "worktree .factory/ content is NOT off-limits (corrected model)" → negation RED ✓.
  # CONTROL: "worktree .factory/ content is off-limits for spec reads" → GREEN ✓.
  run grep -i "off-limits" "$SHARED_CTX"
  [ "$status" -eq 0 ]
  # Negation guard: "off-limits" must NOT appear only in directly negated form "not off-limits"
  # or "no longer off-limits". Use direct-adjacency patterns (no .* between NOT and off-limits)
  # to avoid false-positive on a line like "it is NOT a stale snapshot ... It is off-limits" where
  # NOT governs a different clause. The guard checks that at least one line contains "off-limits"
  # WITHOUT the direct-negation prefix.
  # MUTANT: file contains only "NOT off-limits" or "no longer off-limits" → negation RED ✓.
  # CONTROL: "It is off-limits for spec ground-truth" (no direct "not off-limits") → GREEN ✓.
  local off_limits_lines affirmative_lines
  off_limits_lines="$(grep -i "off-limits" "$SHARED_CTX")"
  affirmative_lines="$(printf '%s\n' "$off_limits_lines" | \
    grep -viE '\bnot[[:space:]]+off-limits\b|\bno[[:space:]]+longer[[:space:]]+off-limits\b|\bexempt[[:space:]]+from[[:space:]]+off-limits\b' || true)"
  if [ -z "$affirmative_lines" ]; then
    echo "DOC-PARITY FAIL [_shared-context.md: all 'off-limits' occurrences appear in directly negated context (F-S2104-P22-006(l))]: the prohibition must state worktree .factory/ content IS off-limits, not that it is not off-limits"
    printf 'All off-limits lines found:\n%s\n' "$off_limits_lines"
    false
  fi
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
  # F-S2104-P22-006(m): mandate-context check. Bare presence allows "feature HEAD SHA: optional"
  # or "feature HEAD SHA is not required in the dispatch" to pass. The anchor must appear with
  # a mandate token asserting it is required in the adversary dispatch.
  # MUTANT: "feature HEAD SHA is not required in the dispatch context" → mandate-context fails → RED ✓.
  # CONTROL: "embed the expected feature HEAD SHA in the adversary dispatch" → GREEN ✓.
  run grep -i "feature HEAD SHA" "$STEP_D5"
  [ "$status" -eq 0 ]
  # F-S2104-P22-006(m): mandate-context check — check ALL lines containing the anchor for any
  # mandate token (not just head -1, which may return a comment line without mandate language
  # while the actual mandate is on a later line). Gate passes if ANY line co-locates the anchor
  # with a mandate token.
  # MUTANT: all "feature HEAD SHA" lines are comments without MUST/embed → no mandated line → RED ✓.
  # CONTROL: "The dispatch MUST embed the expected feature HEAD SHA" → mandated line found → GREEN ✓.
  local sha_mandate_line
  sha_mandate_line="$(grep -i "feature HEAD SHA" "$STEP_D5" | \
    grep -iE '\bMUST\b|\bmust\b|\brequired\b|\bmandatory\b|\bembed\b|\binclude\b|\bpass\b|\bprovide\b|\bsupply\b' | head -1 || true)"
  if [ -z "$sha_mandate_line" ]; then
    echo "DOC-PARITY FAIL [step-d5-adversary-convergence.md: 'feature HEAD SHA' lacks mandate context (F-S2104-P22-006(m))]: no line co-locates 'feature HEAD SHA' with a mandate token (MUST/required/embed/include)"
    printf 'All feature HEAD SHA lines:\n%s\n' "$(grep -i "feature HEAD SHA" "$STEP_D5")"
    false
  fi
}

# (n) AC-014: step-d5-adversary-convergence.md must require the preflight
#     assertion to PASS before findings are accepted.
#     Anchor: "preflight assertion" (exact phrase pairing the step-D5 dispatch
#     clause to the adversary.md contract).
@test "test_BC_step_d5_preflight_assertion_must_pass_before_findings" {
  # F-S2104-P22-006(n): mandate-context check. Bare presence allows "preflight assertion: see §G.1"
  # as a footnote with no mandate language. The anchor must appear with a mandate token asserting
  # the assertion must PASS before findings are accepted.
  # MUTANT: "preflight assertion may be skipped in time-critical contexts" → mandate-context fails → RED ✓.
  # CONTROL: "preflight assertion MUST PASS before findings are accepted" → GREEN ✓.
  run grep -i "preflight assertion" "$STEP_D5"
  [ "$status" -eq 0 ]
  # F-S2104-P22-006(n): mandate-context check — check ALL lines for any line that co-locates
  # "preflight assertion" with a mandate token. head -1 may return a comment line (e.g.
  # "# Any non-zero exit is a preflight assertion failure:") without mandate language while
  # the actual mandate ("The preflight assertion MUST pass") is on a later line.
  # MUTANT: all "preflight assertion" lines are comments without MUST/pass → no mandated line → RED ✓.
  # CONTROL: "The preflight assertion MUST pass — i.e., the adversary MUST..." → mandated line found → GREEN ✓.
  local pa_mandate_line
  pa_mandate_line="$(grep -i "preflight assertion" "$STEP_D5" | \
    grep -iE '\bMUST\b|\bmust\b|\brequired\b|\bmandatory\b|\bpass\b|\bbefore\b|\bprior\b|\bfirst\b' | head -1 || true)"
  if [ -z "$pa_mandate_line" ]; then
    echo "DOC-PARITY FAIL [step-d5-adversary-convergence.md: 'preflight assertion' lacks mandate context (F-S2104-P22-006(n))]: no line co-locates 'preflight assertion' with a mandate token (MUST/pass/before/required)"
    printf 'All preflight assertion lines:\n%s\n' "$(grep -i "preflight assertion" "$STEP_D5")"
    false
  fi
}
