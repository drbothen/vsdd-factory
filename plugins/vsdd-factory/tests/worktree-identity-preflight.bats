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

# ===========================================================================
# B01 STRUCTURAL CORPUS HELPERS (F-S2104-P25-B01-STRUCTURAL)
# Factored out so both the production guards and the corpus regression @test
# can call the same code. POLICY 11 anti-tautology: corpus exercises real guard.
# ===========================================================================

# Guard (e) "checks out NOTHING under" sub-gate (fail-closed).
# $1 = path to the agent file to inspect.
# Returns 0 if ZERO nullified occurrences AND ≥1 affirmative.
# Returns non-zero (via 'return 1') if any occurrence is in nullification context
# or no affirmative occurrence is found.
_guard_e_checks_out_nothing() {
  local agent_file="$1"
  local co_all co_nullified co_aff
  co_all="$(grep -i "checks out NOTHING under" "$agent_file")"
  co_nullified="$(printf '%s\n' "$co_all" | \
    grep -iE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\bnot required\b' || true)"
  co_aff="$(printf '%s\n' "$co_all" | \
    grep -viE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\bnot required\b' || true)"
  if [ -n "$co_nullified" ]; then
    echo "DOC-PARITY FAIL [adversary.md: a 'checks out NOTHING under' occurrence is in nullification context (FAIL-CLOSED: zero nullified required — F-S2104-P23-006 B01 regression fix / F-S2104-P22-006(e))]: all occurrences must be affirmative; adversary.md has two occurrences (Rule 4 uppercase NOTHING, Rule 6 sub-bullet lowercase nothing) — nullifying either fires this gate"
    printf 'Nullified occurrences:\n%s\n' "$co_nullified"
    return 1
  fi
  if [ -z "$co_aff" ]; then
    echo "DOC-PARITY FAIL [adversary.md: all 'checks out NOTHING under' occurrences appear in nullification context (F-S2104-P22-006(e))]: at least one affirmative instance required"
    printf 'All checks-out-NOTHING lines:\n%s\n' "$co_all"
    return 1
  fi
  # B01 positional conjunct (F-S2104-P27-B01): affirmative must appear on Rule 4's numbered-item
  # line within the #### Worktree-Identity Preflight section. Deletion attack: delete Rule 4
  # entirely → Rule 6 sub-bullet 'checks out nothing under' (lowercase) survives → co_aff
  # non-empty (old: GREEN BUG); positional check: no ^4. line with token in preflight → RED ✓.
  # MUTANT (M10 deletion axis): grep -v '^4\. \*\*Read spec' removes Rule 4 → co_clause empty → RED ✓.
  # RESTORE: revert to production adversary.md → ^4. line present → co_clause non-empty → GREEN ✓.
  local co_preflight_section co_clause
  co_preflight_section="$(awk '/^#### Worktree-Identity Preflight/{found=1; next} found && /^#### /{exit} found && /^### /{exit} found && /^## /{exit} found && /^---/{exit} found{print}' "$agent_file")"
  co_clause="$(printf '%s\n' "$co_preflight_section" | grep -i 'checks out NOTHING under' | grep -E '^4\.' || true)"
  if [ -z "$co_clause" ]; then
    echo "DOC-PARITY FAIL [adversary.md: 'checks out NOTHING under' mandate absent from Rule 4 (^4.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 4 while Rule 6 sub-bullet 'checks out nothing under' survives leaves co_aff non-empty; positional conjunct requires the token on a ^4. line in the preflight section"
    return 1
  fi
}

# Guard (g) "path-corroborated" sub-gate (fail-closed).
# $1 = path to the agent file to inspect.
# Returns 0 if ZERO nullified occurrences AND ≥1 affirmative.
# Returns non-zero (via 'return 1') if any occurrence is in scope-restriction context
# or no affirmative occurrence is found.
_guard_g_path_corroborated() {
  local agent_file="$1"
  local pc_all pc_nullified pc_aff
  pc_all="$(grep -i "path-corroborated" "$agent_file")"
  pc_nullified="$(printf '%s\n' "$pc_all" | \
    grep -iE '\bnot applicable\b|\bdoes not apply\b|\boutside\b|\bnot required\b|\bexcept\b|\bexempt\b' || true)"
  pc_aff="$(printf '%s\n' "$pc_all" | \
    grep -viE '\bnot applicable\b|\bdoes not apply\b|\boutside\b|\bnot required\b|\bexcept\b|\bexempt\b' || true)"
  if [ -n "$pc_nullified" ]; then
    echo "DOC-PARITY FAIL [adversary.md: a 'path-corroborated' occurrence is in scope-restriction context (FAIL-CLOSED: zero nullified required — F-S2104-P23-006 B01 regression fix / F-S2104-P22-006(g) / F-S2104-P23-007)]: all occurrences must be affirmative; adversary.md has two occurrences (Rule 6 opening mandate, Rule 6 closing normative closure) — scope-restriction on either fires this gate"
    printf 'Nullified occurrences:\n%s\n' "$pc_nullified"
    return 1
  fi
  if [ -z "$pc_aff" ]; then
    echo "DOC-PARITY FAIL [adversary.md: all 'path-corroborated' occurrences appear in scope-restriction context (F-S2104-P22-006(g) / F-S2104-P23-007)]: at least one affirmative (non-scope-restricted) instance required"
    printf 'All path-corroborated lines found:\n%s\n' "$pc_all"
    return 1
  fi
  # B01 positional conjunct (F-S2104-P27-B01): affirmative must appear on Rule 6's numbered-item
  # line within the #### Worktree-Identity Preflight section. Deletion attack: delete Rule 6's
  # opening ^6. line while sub-bullet 'NOT path-corroborated' text survives in pc_aff (the
  # nullification predicate does not match the P23-007 normative closure sentence form) → old
  # guard GREEN (BUG); positional check: no ^6. line with token in preflight → RED ✓.
  # MUTANT (M12 deletion axis): grep -v '^6\. \*\*Path-corroborate' removes opening line →
  #   pc_clause empty → RED ✓.
  # RESTORE: revert to production adversary.md → ^6. line present → pc_clause non-empty → GREEN ✓.
  local pc_preflight_section pc_clause
  pc_preflight_section="$(awk '/^#### Worktree-Identity Preflight/{found=1; next} found && /^#### /{exit} found && /^### /{exit} found && /^## /{exit} found && /^---/{exit} found{print}' "$agent_file")"
  pc_clause="$(printf '%s\n' "$pc_preflight_section" | grep -i 'path-corroborated' | grep -E '^6\.' || true)"
  if [ -z "$pc_clause" ]; then
    echo "DOC-PARITY FAIL [adversary.md: 'path-corroborated' mandate absent from Rule 6 (^6.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 6 opening line while 'NOT path-corroborated' sub-bullet survives leaves pc_aff non-empty (P23-007 normative closure sentence not caught by nullification predicate); positional conjunct requires the token on a ^6. line in the preflight section"
    return 1
  fi
}

# Guard (d) "worktree-rooted" sub-gate (fail-closed, F-S2104-P26-B02).
# $1 = path to the agent file to inspect.
# Returns 0 if ZERO nullified occurrences AND ≥1 affirmative.
# Returns non-zero (via 'return 1') if any occurrence is in nullification context
# or no affirmative occurrence is found.
# B02 sweep: affirmative-only form (guard (d) inline) was fail-open against a second
# nullified occurrence appearing alongside an affirmative — zero-nullified gate closes this.
_guard_d_worktree_rooted() {
  local agent_file="$1"
  local wtr_all wtr_nullified wtr_aff
  wtr_all="$(grep -i "worktree-rooted" "$agent_file")"
  wtr_nullified="$(printf '%s\n' "$wtr_all" | \
    grep -iE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\boutside\b|\bnot required\b' || true)"
  wtr_aff="$(printf '%s\n' "$wtr_all" | \
    grep -viE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\boutside\b|\bnot required\b' || true)"
  if [ -n "$wtr_nullified" ]; then
    echo "DOC-PARITY FAIL [adversary.md: a 'worktree-rooted' occurrence is in nullification context (FAIL-CLOSED: zero nullified required — F-S2104-P26-B02)]: all occurrences must be affirmative; adversary.md has one occurrence (Rule 3) — nullifying it fires this gate; a future second occurrence in nullification context also fires (safe-by-accident guard closed)"
    printf 'Nullified occurrences:\n%s\n' "$wtr_nullified"
    return 1
  fi
  if [ -z "$wtr_aff" ]; then
    echo "DOC-PARITY FAIL [adversary.md: all 'worktree-rooted' occurrences appear in nullification context (F-S2104-P22-006(d))]: at least one affirmative (non-nullified) instance required; appended exception 'worktree-rooted: retired' no longer passes"
    printf 'All worktree-rooted lines found:\n%s\n' "$wtr_all"
    return 1
  fi
  # B01 positional conjunct (F-S2104-P27-B01): affirmative must appear on Rule 3's numbered-item
  # line within the #### Worktree-Identity Preflight section. 'worktree-rooted' has a single
  # occurrence (Rule 3 only) — deletion of Rule 3 already caught by empty wtr_aff. Positional
  # conjunct is defense-in-depth: if the token moves outside the section or to a different rule
  # number, this fires → RED ✓. Deletion axis M13: grep -v '^3\. ...' removes Rule 3 →
  # wtr_aff empty → both empty-aff check and positional check fire → RED ✓.
  # RESTORE: revert to production adversary.md → ^3. line present → wtr_clause non-empty → GREEN ✓.
  local wtr_preflight_section wtr_clause
  wtr_preflight_section="$(awk '/^#### Worktree-Identity Preflight/{found=1; next} found && /^#### /{exit} found && /^### /{exit} found && /^## /{exit} found && /^---/{exit} found{print}' "$agent_file")"
  wtr_clause="$(printf '%s\n' "$wtr_preflight_section" | grep -i 'worktree-rooted' | grep -E '^3\.' || true)"
  if [ -z "$wtr_clause" ]; then
    echo "DOC-PARITY FAIL [adversary.md: 'worktree-rooted' mandate absent from Rule 3 (^3.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: positional conjunct requires the token on a ^3. line in the preflight section; if Rule 3 is deleted or the token migrates to a different rule number, this fires"
    return 1
  fi
}

# Guard (f) "case-insensitive" sub-gate (fail-closed, F-S2104-P26-B02).
# $1 = path to the agent file to inspect.
# Returns 0 if ZERO nullified occurrences AND ≥1 affirmative.
# Returns non-zero (via 'return 1') if any occurrence is in nullification context
# or no affirmative occurrence is found.
# B02 sweep: adversary.md has 3 'case-insensitive' occurrences (Rule 2 body x1, Rule 5 x2).
# Only Rule 5 is the mandate; nullifying Rule 5 leaves 2 incidental affirmatives — old check
# returns GREEN (BUG). Zero-nullified gate fires on the nullified Rule 5 occurrence → RED ✓.
_guard_f_case_insensitive() {
  local agent_file="$1"
  local ci_all ci_nullified ci_aff
  ci_all="$(grep -i "case-insensitive" "$agent_file")"
  ci_nullified="$(printf '%s\n' "$ci_all" | \
    grep -iE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot required\b|\bnot applicable\b|\bnot case-insensitive\b' || true)"
  ci_aff="$(printf '%s\n' "$ci_all" | \
    grep -viE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot required\b|\bnot applicable\b|\bnot case-insensitive\b' || true)"
  if [ -n "$ci_nullified" ]; then
    echo "DOC-PARITY FAIL [adversary.md: a 'case-insensitive' occurrence is in nullification context (FAIL-CLOSED: zero nullified required — F-S2104-P26-B02)]: adversary.md has 3 case-insensitive occurrences (Rule 2 body x1, Rule 5 heading+body x2); only Rule 5 is the mandate; nullifying Rule 5 while incidentals survive causes old affirmative-only check to return GREEN (BUG); zero-nullified gate fires on the nullified Rule 5 occurrence → RED ✓"
    printf 'Nullified occurrences:\n%s\n' "$ci_nullified"
    return 1
  fi
  if [ -z "$ci_aff" ]; then
    echo "DOC-PARITY FAIL [adversary.md: all 'case-insensitive' occurrences appear in nullification context (F-S2104-P22-006(f))]: at least one affirmative instance required"
    printf 'All case-insensitive lines found:\n%s\n' "$ci_all"
    return 1
  fi
  # B01 positional conjunct (F-S2104-P27-B01): affirmative must appear on Rule 5's numbered-item
  # line within the #### Worktree-Identity Preflight section. Deletion attack (primary hole):
  # delete Rule 5 entirely → Rule 2 body 'case-insensitively' and story-spec-path-lookup text
  # 'case-insensitive' survive → ci_aff non-empty (old: GREEN BUG). Positional check: no ^5.
  # line with token in preflight section → RED ✓. B01 is the structural fix for this guard.
  # MUTANT (M11 deletion axis): grep -v '^5\. \*\*Use case-insensitive' removes Rule 5 → ci_clause
  #   empty → RED ✓ (old guard PASSES on this mutant because ci_aff still has Rule 2 incidentals).
  # RESTORE: revert to production adversary.md → ^5. line present → ci_clause non-empty → GREEN ✓.
  local ci_preflight_section ci_clause
  ci_preflight_section="$(awk '/^#### Worktree-Identity Preflight/{found=1; next} found && /^#### /{exit} found && /^### /{exit} found && /^## /{exit} found && /^---/{exit} found{print}' "$agent_file")"
  ci_clause="$(printf '%s\n' "$ci_preflight_section" | grep -i 'case-insensitive' | grep -E '^5\.' || true)"
  if [ -z "$ci_clause" ]; then
    echo "DOC-PARITY FAIL [adversary.md: 'case-insensitive' mandate absent from Rule 5 (^5.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 5 while Rule 2 'case-insensitively' incidentals and story-spec-path-lookup 'case-insensitive' survive leaves ci_aff non-empty; positional conjunct requires the token on a ^5. line in the preflight section"
    return 1
  fi
}

# Guard (l) "off-limits" sub-gate (fail-closed, F-S2104-P26-B02).
# $1 = path to the doc file to inspect.
# Returns 0 if ZERO negated occurrences AND ≥1 affirmative.
# Returns non-zero (via 'return 1') if any occurrence is in directly-negated context
# or no affirmative occurrence is found.
# B02 sweep: _shared-context.md has one 'off-limits' occurrence; a second in negated form
# would pass old affirmative-only check while nullifying the mandate — zero-negated gate closes.
_guard_l_off_limits() {
  local doc_file="$1"
  local ol_all ol_nullified ol_aff
  ol_all="$(grep -i "off-limits" "$doc_file")"
  ol_nullified="$(printf '%s\n' "$ol_all" | \
    grep -iE '\bnot[[:space:]]+off-limits\b|\bno[[:space:]]+longer[[:space:]]+off-limits\b|\bexempt[[:space:]]+from[[:space:]]+off-limits\b' || true)"
  ol_aff="$(printf '%s\n' "$ol_all" | \
    grep -viE '\bnot[[:space:]]+off-limits\b|\bno[[:space:]]+longer[[:space:]]+off-limits\b|\bexempt[[:space:]]+from[[:space:]]+off-limits\b' || true)"
  if [ -n "$ol_nullified" ]; then
    echo "DOC-PARITY FAIL [_shared-context.md: a 'off-limits' occurrence is in directly-negated context (FAIL-CLOSED: zero negated required — F-S2104-P26-B02)]: the prohibition must state worktree .factory/ content IS off-limits; a negated form alongside an affirmative passes the old check (BUG) — zero-negated gate closes this"
    printf 'Negated occurrences:\n%s\n' "$ol_nullified"
    return 1
  fi
  if [ -z "$ol_aff" ]; then
    echo "DOC-PARITY FAIL [_shared-context.md: all 'off-limits' occurrences appear in directly negated context (F-S2104-P22-006(l))]: the prohibition must state worktree .factory/ content IS off-limits, not that it is not off-limits"
    printf 'All off-limits lines found:\n%s\n' "$ol_all"
    return 1
  fi
  # B01 positional conjunct (F-S2104-P27-B01): affirmative must appear within the
  # ### Spec-Path Discipline section of _shared-context.md. The 'off-limits' sentence lives
  # inside that section (confirmed: single occurrence at production HEAD). Positional check is
  # defense-in-depth: if the sentence migrates outside the section or the section is renamed,
  # this fires → RED ✓. Deletion axis M14: deleting the off-limits line is already caught by
  # if [ -z "$ol_aff" ]; positional adds structural-location defense.
  # RESTORE: revert to production _shared-context.md → ol_clause non-empty → GREEN ✓.
  local ol_spec_path_section ol_clause
  # F-S2104-P28-016 + coordinator correction: adaptive section-bounded extractor
  # tolerant of ##–#### heading levels (no heading-level hardcode).
  # Start: /^#{2,4}[[:space:]].*Spec-Path Discipline/ matches at any depth ##–####.
  # ERE interval {2,4}: supported on GNU awk (CI/Linux) and BSD awk (macOS) — verified
  # locally (macOS awk supports {N,M} in ERE mode). Depth recorded via
  # match($0,/^#+/)+RLENGTH. Exit on any heading whose RLENGTH ≤ section depth
  # (same-level sibling or parent); NOT on deeper child headings (e.g., #### Write
  # Discipline inside ### Spec-Path Discipline). Added /^---/{exit} leg to match the
  # six sibling guards.
  ol_spec_path_section="$(awk '/^#{2,4}[[:space:]].*Spec-Path Discipline/{found=1; match($0,/^#+/); depth=RLENGTH; next} found && /^#+ /{match($0,/^#+/); if(RLENGTH<=depth)exit} found && /^---/{exit} found{print}' "$doc_file")"
  ol_clause="$(printf '%s\n' "$ol_spec_path_section" | grep -i 'off-limits' || true)"
  if [ -z "$ol_clause" ]; then
    echo "DOC-PARITY FAIL [_shared-context.md: 'off-limits' prohibition absent from ### Spec-Path Discipline section (B01 deletion axis — F-S2104-P27-B01)]: positional conjunct requires the 'off-limits' mandate to appear within the Spec-Path Discipline section; if the sentence is moved outside the section or the section is renamed, this fires → RED ✓"
    return 1
  fi
}

# Guard (e) "factory-artifacts" sub-gate (fail-closed, F-S2104-P26-B02).
# $1 = path to the agent file to inspect.
# Returns 0 if ZERO nullified occurrences AND ≥1 affirmative (non-branch/push occurrences).
# Returns non-zero (via 'return 1') if any occurrence is in nullification context
# or no affirmative occurrence is found.
# Factored from inline guard (e) code to enable corpus regression @test (POLICY 11 anti-tautology).
_guard_e_factory_artifacts() {
  local agent_file="$1"
  local fa_all fa_nullified fa_aff
  # H01 fix (F-S2104-P27-H01): removed vestigial pre-harm exclusion
  # 'grep -iv factory-artifacts branch|origin.*factory-artifacts|push.*factory-artifacts'.
  # That filter ran BEFORE fa_nullified was computed, creating a hidden channel: a nullifying
  # rewrite phrased as "factory-artifacts branch: no longer required" was excluded from fa_all
  # before the nullified check could catch it → silent FALSE NEGATIVE. The exclusion matched
  # ZERO lines in the current adversary.md (confirmed: grep returns empty) — fully vestigial.
  # H01 sweep: no other guards have the same pre-harm exclusion pattern.
  fa_all="$(grep -i "factory-artifacts" "$agent_file" || true)"
  fa_nullified="$(printf '%s\n' "$fa_all" | \
    grep -iE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\bnot required\b' || true)"
  fa_aff="$(printf '%s\n' "$fa_all" | \
    grep -viE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\bnot required\b' || true)"
  if [ -n "$fa_nullified" ]; then
    echo "DOC-PARITY FAIL [adversary.md: a 'factory-artifacts' occurrence is in nullification context (FAIL-CLOSED: zero nullified required — F-S2104-P23-006 B01 regression fix)]: all occurrences must be affirmative; a nullified occurrence alongside an affirmative one is caught by this zero-nullified gate"
    printf 'Nullified occurrences:\n%s\n' "$fa_nullified"
    return 1
  fi
  if [ -z "$fa_aff" ]; then
    echo "DOC-PARITY FAIL [adversary.md: all 'factory-artifacts' occurrences appear in nullification context (F-S2104-P22-006(e))]: at least one affirmative instance required"
    printf 'All factory-artifacts lines:\n%s\n' "$fa_all"
    return 1
  fi
  # B01 positional conjunct (F-S2104-P27-B01): affirmative must appear on Rule 4's numbered-item
  # line within the #### Worktree-Identity Preflight section. Deletion attack: delete Rule 4
  # entirely → preamble and Rule 6 sub-bullet still mention 'factory-artifacts' → fa_aff non-empty
  # (old: GREEN BUG); positional check: no ^4. line with token in preflight section → RED ✓.
  # MUTANT (M10 deletion axis): grep -v '^4\. \*\*Read spec' removes Rule 4 → fa_clause empty → RED ✓.
  # RESTORE: revert to production adversary.md → ^4. line present → fa_clause non-empty → GREEN ✓.
  local fa_preflight_section fa_clause
  fa_preflight_section="$(awk '/^#### Worktree-Identity Preflight/{found=1; next} found && /^#### /{exit} found && /^### /{exit} found && /^## /{exit} found && /^---/{exit} found{print}' "$agent_file")"
  fa_clause="$(printf '%s\n' "$fa_preflight_section" | grep -i 'factory-artifacts' | grep -E '^4\.' || true)"
  if [ -z "$fa_clause" ]; then
    echo "DOC-PARITY FAIL [adversary.md: 'factory-artifacts' mandate absent from Rule 4 (^4.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 4 while preamble/Rule-6-sub-bullet 'factory-artifacts' survive leaves fa_aff non-empty; positional conjunct requires the token on a ^4. line in the preflight section"
    return 1
  fi
}

# Guard (e) "canonical-repo-root" sub-gate (fail-closed, F-S2104-P26-B02).
# $1 = path to the agent file to inspect.
# Returns 0 if ZERO nullified occurrences AND ≥1 affirmative.
# Returns non-zero (via 'return 1') if any occurrence is in nullification context
# or no affirmative occurrence is found.
# Factored from inline guard (e) code to enable corpus regression @test (POLICY 11 anti-tautology).
_guard_e_canonical_repo_root() {
  local agent_file="$1"
  local cr_all cr_nullified cr_aff
  cr_all="$(grep -i "canonical-repo-root" "$agent_file")"
  cr_nullified="$(printf '%s\n' "$cr_all" | \
    grep -iE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\bnot required\b' || true)"
  cr_aff="$(printf '%s\n' "$cr_all" | \
    grep -viE '\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|\bnot applicable\b|\bdoes not apply\b|\bnot required\b' || true)"
  if [ -n "$cr_nullified" ]; then
    echo "DOC-PARITY FAIL [adversary.md: a 'canonical-repo-root' occurrence is in nullification context (FAIL-CLOSED: zero nullified required — F-S2104-P23-006 B01 regression fix)]: all occurrences must be affirmative"
    printf 'Nullified occurrences:\n%s\n' "$cr_nullified"
    return 1
  fi
  if [ -z "$cr_aff" ]; then
    echo "DOC-PARITY FAIL [adversary.md: all 'canonical-repo-root' occurrences appear in nullification context (F-S2104-P22-006(e))]: at least one affirmative instance required"
    printf 'All canonical-repo-root lines:\n%s\n' "$cr_all"
    return 1
  fi
  # B01 positional conjunct (F-S2104-P27-B01): affirmative must appear on Rule 4's numbered-item
  # line within the #### Worktree-Identity Preflight section. Deletion attack: delete Rule 4
  # entirely → Rules 5, 6, preamble and story-spec-path-lookup still mention 'canonical-repo-root'
  # → cr_aff non-empty (old: GREEN BUG); positional check: no ^4. line with token in preflight
  # section → RED ✓.
  # MUTANT (M10 deletion axis): grep -v '^4\. \*\*Read spec' removes Rule 4 → cr_clause empty → RED ✓.
  # RESTORE: revert to production adversary.md → ^4. line present → cr_clause non-empty → GREEN ✓.
  local cr_preflight_section cr_clause
  cr_preflight_section="$(awk '/^#### Worktree-Identity Preflight/{found=1; next} found && /^#### /{exit} found && /^### /{exit} found && /^## /{exit} found && /^---/{exit} found{print}' "$agent_file")"
  cr_clause="$(printf '%s\n' "$cr_preflight_section" | grep -i 'canonical-repo-root' | grep -E '^4\.' || true)"
  if [ -z "$cr_clause" ]; then
    echo "DOC-PARITY FAIL [adversary.md: 'canonical-repo-root' mandate absent from Rule 4 (^4.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 4 while Rules 5/6/preamble 'canonical-repo-root' survive leaves cr_aff non-empty; positional conjunct requires the token on a ^4. line in the preflight section"
    return 1
  fi
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
  # (from "#### Worktree-Identity Preflight" up to the next sibling/parent heading or "---").
  # F-S2104-P23-008: prior awk only exited on ^#### or ^--- — ### Perimeter 2 and ### Perimeter 3
  # (^### level) were not exit conditions, so the extractor ran past them into unrelated content.
  # The correct boundary set (matching _extract_write_discipline_section in the sibling suite):
  # ^#### (sibling #### headings), ^### (parent ### headings), ^## (grandparent ## headings), ^---.
  # MUTANT: remove ^### exit → awk captures '### Perimeter 2: Wave-gate' prose → 'Wave-gate' found
  #   in preflight_section → boundary-assertion fires → RED ✓.
  # CONTROL: ^### exit present → Perimeter 2 content excluded → boundary-assertion GREEN ✓.
  preflight_section="$(awk '/^#### Worktree-Identity Preflight/{found=1; next} found && /^#### /{exit} found && /^### /{exit} found && /^## /{exit} found && /^---/{exit} found{print}' "$ADVERSARY_AGENT")"
  # Boundary assertion (F-S2104-P23-008): Perimeter 2 content must NOT appear in the extracted section.
  # '### Perimeter 2: Wave-gate (Gate 3)' contains 'Wave-gate' — present if awk over-captures.
  if printf '%s\n' "$preflight_section" | grep -q "Wave-gate"; then
    echo "DOC-PARITY FAIL [adversary.md guard (b): preflight section extractor over-captures into Perimeter 2 — 'Wave-gate' found in extracted section; fix: ^### and ^## exit conditions must be present in awk (F-S2104-P23-008)]"
    false
  fi
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
  # All-lines nullification guard (F-S2104-P23-006): scan ALL matching lines, not just head -1.
  # head -1 is fail-open against an APPENDED exception — e.g. a valid affirmative first line
  # followed by "worktree-rooted discipline is retired" still passes head -1 while nullifying
  # the mandate. Collect all lines, filter out nullified forms, require ≥1 affirmative line.
  # MUTANT: file has two 'worktree-rooted' lines: affirmative first, 'worktree-rooted: retired'
  #   second → head -1 returns affirmative (old: PASS); all-lines filter removes both → empty
  #   affirmative set (new: RED) ✓.
  # B02 F-S2104-P26-B02: zero-nullified form factored into _guard_d_worktree_rooted().
  # MUTANT (appended nullifier): "worktree-rooted: retired" appended alongside affirmative →
  #   old affirmative-only check returns GREEN (BUG); zero-nullified gate fires → RED ✓.
  # MUTANT (inline nullifier): "formerly worktree-rooted (superseded)" → zero-nullified fires → RED ✓.
  # CONTROL: "All feature-code reads MUST use worktree-rooted absolute paths" → GREEN ✓.
  _guard_d_worktree_rooted "$ADVERSARY_AGENT"
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
  # All-lines nullification guard (F-S2104-P23-006, B01 regression fix): each corrected-model
  # token scanned across ALL matching lines. head -1 was fail-open against an appended exception
  # that nullifies after a valid first occurrence. Fail-closed form: ZERO nullified occurrences
  # required AND ≥1 affirmative. A single nullified occurrence (even alongside an affirmative)
  # means the doc is compromised — the nullified form is adversary-visible.
  # MUTANT per token: append "factory-artifacts: retired" / "canonical-repo-root: superseded" /
  #   "checks out NOTHING under: no longer accurate" as a second occurrence → head -1 returns
  #   the affirmative first line (old: PASS); zero-nullified gate fires on the new occurrence →
  #   RED ✓ (B01 regression closed).
  # MUTANT (recorded adversary.md Rule 4 + Rule 6 sub-bullet, B01 structural): adversary.md
  #   has two "checks out NOTHING under" occurrences (Rule 4 uppercase NOTHING, Rule 6 sub-bullet
  #   lowercase nothing); nullifying Rule 4 while Rule 6 sub-bullet survives → co_nullified
  #   non-empty → RED ✓ (zero-nullified gate in _guard_e_checks_out_nothing catches).
  # "checks out NOTHING under" sub-gate factored into _guard_e_checks_out_nothing() helper
  #   so the B01 corpus regression @test can call the real guard (POLICY 11 anti-tautology).
  # B02 F-S2104-P26-B02: factory-artifacts and canonical-repo-root sub-gates factored into
  # helpers to enable corpus regression @test (POLICY 11 anti-tautology).
  _guard_e_factory_artifacts "$ADVERSARY_AGENT"
  _guard_e_canonical_repo_root "$ADVERSARY_AGENT"
  _guard_e_checks_out_nothing "$ADVERSARY_AGENT"
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
  # All-lines nullification guard (F-S2104-P23-006): scan ALL matching lines.
  # MUTANT: append "case-insensitive: no longer required (simplified)" as a second line →
  #   head -1 returns affirmative first line (old: PASS); all-lines filter removes all nullified
  #   lines → empty affirmative set (new: RED) ✓.
  # B02 F-S2104-P26-B02: zero-nullified form factored into _guard_f_case_insensitive().
  # adversary.md has 3 case-insensitive occurrences (Rule 2 body x1, Rule 5 heading+body x2).
  # Only Rule 5 is the mandate; nullifying Rule 5 while 2 incidentals survive — old affirmative-
  # only check returns GREEN (BUG); zero-nullified gate fires on nullified Rule 5 → RED ✓.
  _guard_f_case_insensitive "$ADVERSARY_AGENT"
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
  # All-lines scope-restriction guard (F-S2104-P23-006 + F-S2104-P23-007 + B01 regression fix).
  #
  # P23-006: head -1 fail-open — appended exception after affirmative first line passes head -1
  # while nullifying the mandate. All-lines scanning closes this.
  #
  # P23-007: '\bis not\b' and bare '\bdoes not\b' were too broad — they fired on the normative
  # sentence "A finding ... that is NOT path-corroborated against the correct target for its
  # artifact class MUST NOT be reported." This sentence ENFORCES the mandate (findings lacking
  # path-corroboration MUST NOT be reported) — it does NOT restrict the mandate's scope.
  # Third live instance of gate-fires-on-correct-prose (after PC2c and blockquote cases).
  # Fix: predicate is now meta-aware. '\bis not\b' removed entirely; '\bdoes not\b' narrowed
  # to '\bdoes not apply\b'. Genuine scope-restrictions say "does not apply to X", "not required
  # for Y", "not applicable" — not "the finding is not path-corroborated therefore MUST NOT be
  # reported."
  #
  # B01 regression fix: adversary.md has TWO "path-corroborated" occurrences (Rule 6 opening
  # mandate and Rule 6 closing normative closure). Nullifying the mandate while the normative
  # closure survives → pc_aff non-empty → old affirmative-only check PASSES (BUG). Zero-nullified
  # gate in _guard_g_path_corroborated catches this: pc_nullified non-empty → RED ✓.
  #
  # MUTANT (appended nullifier, P23-006): "path-corroborated: does not apply to ADR checks"
  #   appended as second line → head -1 returns affirmative first (old: PASS); zero-nullified
  #   gate fires on the new occurrence → RED ✓ (B01 regression closed).
  # MUTANT (recorded B01, adversary.md Rule 6 two-occurrence): adversary.md has two
  #   "path-corroborated" occurrences — Rule 6 opening mandate and Rule 6 closing normative
  #   closure. Nullifying the mandate while normative closure survives → pc_aff non-empty
  #   (old: PASS); pc_nullified non-empty → RED ✓ (zero-nullified gate catches).
  # MUTANT (normative prose, P23-007): "finding that is NOT path-corroborated MUST NOT be
  #   reported" → old '\bis not\b' triggers (false positive RED); new predicate does not trigger
  #   → affirmative set non-empty (GREEN, no false positive) ✓.
  # "path-corroborated" sub-gate factored into _guard_g_path_corroborated() helper so the
  #   B01 corpus regression @test can call the real guard (POLICY 11 anti-tautology).
  _guard_g_path_corroborated "$ADVERSARY_AGENT"
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
  # M03 F-S2104-P26-M03: POLICY 13 empty-extraction BLOCK — no fallback to whole-file grep.
  # MUTANT: rename the section heading → extractor returns empty → fallback (old) whole-file grep
  #   might still find "canonical repo-root" in a comment → GREEN (BUG); BLOCK form → RED ✓.
  if [ -z "$spec_path_section" ]; then
    echo "DOC-PARITY FAIL [_shared-context.md guard (k) BLOCK: section extraction returned empty (POLICY 13 empty-extraction BLOCK — F-S2104-P26-M03)]: the Spec-Path Discipline / Write Discipline / Path Discipline heading extractor returned no content; POLICY 13 requires empty-extraction to BLOCK rather than degrade to a whole-file grep; the spec section heading may be absent or renamed"
    false
  fi
  printf '%s\n' "$spec_path_section" | grep -i "canonical repo-root" >/dev/null
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
  # B02 F-S2104-P26-B02: zero-nullified form factored into _guard_l_off_limits().
  # _shared-context.md has 1 'off-limits' occurrence; a second in negated form would pass
  # old affirmative-only check while nullifying the mandate — zero-negated gate closes this.
  _guard_l_off_limits "$SHARED_CTX"
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

# ===========================================================================
# B01 STRUCTURAL: Corpus regression suite for guards (e) co and (g) pc
# (F-S2104-P25-B01-STRUCTURAL)
#
# Exercises the REAL helpers _guard_e_checks_out_nothing and
# _guard_g_path_corroborated against four recorded B01 mutant vectors.
# POLICY 11 anti-tautology: the corpus calls the real guard, not a copy.
# ===========================================================================

@test "test_BC_B01_corpus_regression_guards_e_co_and_g_pc" {
  # any-affirmative guard family coverage (F-S2104-P26-B02):
  # COVERED (7/7): (d) wtr, (e) fa, (e) cr, (e) co, (f) ci, (g) pc, (l) ol
  # UNCOVERED: none
  local corpus_scratch
  corpus_scratch="$(mktemp)"

  # --- Guard (e) co mutants ---

  # M1: pass-22 recorded — uppercase NOTHING occurrence nullified (sed without g: first match only),
  # lowercase nothing occurrence intact → old affirmative-only check sees lowercase line → PASS (BUG).
  # New zero-nullified gate fires on the nullified uppercase occurrence → RED ✓.
  sed 's/checks out NOTHING under/formerly checks out NOTHING under (no longer applicable)/' \
    "$ADVERSARY_AGENT" > "$corpus_scratch"
  run _guard_e_checks_out_nothing "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (e) co M1 — pass-22 recorded / B01 regression direction A]: scratch with nullified uppercase NOTHING (Rule 4) and intact lowercase nothing (Rule 6 sub-bullet) returned GREEN — zero-nullified gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # M2: B01 regression direction B — lowercase nothing occurrence nullified, uppercase intact.
  sed 's/checks out nothing under/checks out nothing under (no longer required — Rule 6 sub-bullet scope)/' \
    "$ADVERSARY_AGENT" > "$corpus_scratch"
  run _guard_e_checks_out_nothing "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (e) co M2 — B01 regression direction B]: scratch with nullified lowercase nothing (Rule 6 sub-bullet) and intact uppercase NOTHING (Rule 4) returned GREEN — zero-nullified gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Control: production adversary.md must pass guard (e) co.
  run _guard_e_checks_out_nothing "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (e) co CONTROL]: production adversary.md returned non-zero from _guard_e_checks_out_nothing — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # --- Guard (g) pc mutants ---

  # M3: pass-22 recorded — blanket all-occurrences nullification.
  sed 's/path-corroborated/path-corroborated: does not apply to ADR directory checks/g' \
    "$ADVERSARY_AGENT" > "$corpus_scratch"
  run _guard_g_path_corroborated "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (g) pc M3 — pass-22 recorded all-occurrences]: blanket nullification returned GREEN — gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # M4: B01 regression — first occurrence only nullified via awk (Rule 6 mandate modified,
  # Rule 6 closure intact). Old affirmative-only check sees intact closure → PASS (BUG).
  # Zero-nullified gate fires on the nullified mandate → RED ✓.
  awk '/path-corroborated/ && !done {
    sub(/path-corroborated/, "path-corroborated: does not apply to ADR directory checks")
    done=1
  } 1' "$ADVERSARY_AGENT" > "$corpus_scratch"
  run _guard_g_path_corroborated "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (g) pc M4 — B01 regression first-occurrence only]: first-occurrence nullification (Rule 6 mandate modified, Rule 6 closure intact) returned GREEN — zero-nullified gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Control: production adversary.md must pass guard (g) pc.
  run _guard_g_path_corroborated "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (g) pc CONTROL]: production adversary.md returned non-zero from _guard_g_path_corroborated — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # --- Guard (d) wtr mutants (B02 F-S2104-P26-B02) ---

  # M5: appended nullifier — affirmative occurrence intact, new nullified occurrence appended.
  # Old affirmative-only check: affirmative line → GREEN (BUG).
  # Zero-nullified gate: nullified line → wtr_nullified non-empty → RED ✓.
  { cat "$ADVERSARY_AGENT"; printf '\nworktree-rooted: retired — see updated §G.4\n'; } > "$corpus_scratch"
  run _guard_d_worktree_rooted "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (d) wtr M5 — B02 regression appended-nullifier]: appended 'worktree-rooted: retired' alongside affirmative returned GREEN — zero-nullified gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Control: production adversary.md must pass guard (d) wtr.
  run _guard_d_worktree_rooted "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (d) wtr CONTROL]: production adversary.md returned non-zero from _guard_d_worktree_rooted — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # --- Guard (f) ci mutants (B02 F-S2104-P26-B02) ---

  # M6: B02 specific — Rule 5 line nullified while Rule 2 incidental ('case-insensitively') intact.
  # Targets the line containing 'Use case-insensitive matching for ID-bearing' (Rule 5 heading+body).
  # Old affirmative-only check: Rule 2 incidental line → ci_aff non-empty → GREEN (BUG).
  # Zero-nullified gate: nullified Rule 5 occurrence → ci_nullified non-empty → RED ✓.
  sed '/Use case-insensitive matching for ID-bearing/s/case-insensitive/case-insensitive (no longer required — simplified)/g' \
    "$ADVERSARY_AGENT" > "$corpus_scratch"
  run _guard_f_case_insensitive "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (f) ci M6 — B02 specific Rule-5-nullified/incidentals-intact]: Rule 5 'case-insensitive' nullified while Rule 2 incidental survives; old affirmative-only check returns GREEN (BUG); zero-nullified gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Control: production adversary.md must pass guard (f) ci.
  run _guard_f_case_insensitive "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (f) ci CONTROL]: production adversary.md returned non-zero from _guard_f_case_insensitive — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # --- Guard (l) ol mutants (B02 F-S2104-P26-B02) ---

  # M7: sole 'off-limits' occurrence replaced by 'NOT off-limits'.
  # sed uses space-delimited form (not \b) — macOS sed does not support \b word-boundaries.
  # grep -iE '\bnot[[:space:]]+off-limits\b' works on macOS (grep supports \b in -E mode).
  # Zero-negated gate: ol_nullified non-empty → RED ✓.
  sed 's/ off-limits / NOT off-limits /g' \
    "$SHARED_CTX" > "$corpus_scratch"
  run _guard_l_off_limits "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (l) ol M7 — all-occurrences negated]: 'off-limits' replaced by 'NOT off-limits' returned GREEN — zero-negated gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Control: production _shared-context.md must pass guard (l) ol.
  run _guard_l_off_limits "$SHARED_CTX"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (l) ol CONTROL]: production _shared-context.md returned non-zero from _guard_l_off_limits — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # --- Guard (e) fa mutants (B02 F-S2104-P26-B02 factoring) ---

  # M8: 'factory-artifacts' in mandate context nullified (targets Rule 4 spec-ground-truth sentence).
  # Zero-nullified gate: fa_nullified non-empty → RED ✓.
  sed 's/canonical factory-artifacts, NOT/canonical factory-artifacts (no longer required), NOT/' \
    "$ADVERSARY_AGENT" > "$corpus_scratch"
  run _guard_e_factory_artifacts "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (e) fa M8 — nullified factory-artifacts in mandate context]: 'factory-artifacts (no longer required)' returned GREEN — zero-nullified gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Control: production adversary.md must pass guard (e) fa.
  run _guard_e_factory_artifacts "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (e) fa CONTROL]: production adversary.md returned non-zero from _guard_e_factory_artifacts — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # --- Guard (e) cr mutants (B02 F-S2104-P26-B02 factoring) ---

  # M9: 'canonical-repo-root' blanket nullification.
  # Zero-nullified gate: cr_nullified non-empty → RED ✓.
  sed 's/canonical-repo-root/canonical-repo-root (superseded)/g' \
    "$ADVERSARY_AGENT" > "$corpus_scratch"
  run _guard_e_canonical_repo_root "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (e) cr M9 — nullified canonical-repo-root]: 'canonical-repo-root (superseded)' returned GREEN — zero-nullified gate must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Control: production adversary.md must pass guard (e) cr.
  run _guard_e_canonical_repo_root "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (e) cr CONTROL]: production adversary.md returned non-zero from _guard_e_canonical_repo_root — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # ===========================================================================
  # DELETION AXIS VECTORS M10–M14 (F-S2104-P27-B01 positional conjunct coverage)
  #
  # These vectors exercise the B01 positional conjunct: they DELETE a numbered
  # rule or prohibition sentence entirely (rather than annotating it), which the
  # old location-blind affirmative checks could not catch because incidental token
  # occurrences in other rules kept the affirmative set non-empty.
  # ===========================================================================

  # M10: Rule 4 deleted — all three guards (co, fa, cr) exercise the ^4. positional conjunct.
  # Rule 4 = 'checks out NOTHING under', 'factory-artifacts' mandate, 'canonical-repo-root' mandate.
  # Old co/fa/cr guards: incidental occurrences survive (Rule 6 sub-bullet, preamble, Rules 5+6)
  # → aff non-empty → GREEN (BUG). New positional conjunct: no ^4. line → RED ✓.
  grep -v '^4\. \*\*Read spec' "$ADVERSARY_AGENT" > "$corpus_scratch"

  run _guard_e_checks_out_nothing "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (e) co M10 — B01 deletion axis: Rule 4 deleted, Rule-6-sub-bullet 'checks out nothing under' survives]: old affirmative-only check returns GREEN (BUG); B01 positional ^4. conjunct must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  run _guard_e_factory_artifacts "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (e) fa M10 — B01 deletion axis: Rule 4 deleted, preamble 'factory-artifacts' survives]: old affirmative-only check returns GREEN (BUG); B01 positional ^4. conjunct must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  run _guard_e_canonical_repo_root "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (e) cr M10 — B01 deletion axis: Rule 4 deleted, Rules 5/6/preamble 'canonical-repo-root' survive]: old affirmative-only check returns GREEN (BUG); B01 positional ^4. conjunct must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Restore M10: production adversary.md must pass guards co, fa, cr.
  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
  run _guard_e_checks_out_nothing "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (e) co M10 RESTORE]: production adversary.md returned non-zero from _guard_e_checks_out_nothing — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi
  run _guard_e_factory_artifacts "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (e) fa M10 RESTORE]: production adversary.md returned non-zero from _guard_e_factory_artifacts — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi
  run _guard_e_canonical_repo_root "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (e) cr M10 RESTORE]: production adversary.md returned non-zero from _guard_e_canonical_repo_root — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # M11: Rule 5 deleted — ci guard exercises the ^5. positional conjunct.
  # Rule 5 = 'case-insensitive' mandate. Old ci guard: Rule 2 'case-insensitively' incidental
  # and story-spec-path-lookup 'case-insensitive' survive → ci_aff non-empty → GREEN (BUG).
  # New positional conjunct: no ^5. line → RED ✓.
  grep -v '^5\. \*\*Use case-insensitive' "$ADVERSARY_AGENT" > "$corpus_scratch"

  run _guard_f_case_insensitive "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (f) ci M11 — B01 deletion axis: Rule 5 deleted, Rule 2/story-spec-path-lookup 'case-insensitive' incidentals survive]: old affirmative-only check returns GREEN (BUG); B01 positional ^5. conjunct must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Restore M11: production adversary.md must pass guard ci.
  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
  run _guard_f_case_insensitive "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (f) ci M11 RESTORE]: production adversary.md returned non-zero from _guard_f_case_insensitive — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # M12: Rule 6 opening line deleted — pc guard exercises the ^6. positional conjunct.
  # 'NOT path-corroborated' text in sub-bullet survives (P23-007 normative closure sentence;
  # nullification predicate does not match 'NOT path-corroborated' form) → pc_aff non-empty
  # → old guard GREEN (BUG). New positional conjunct: no ^6. line → RED ✓.
  grep -v '^6\. \*\*Path-corroborate' "$ADVERSARY_AGENT" > "$corpus_scratch"

  run _guard_g_path_corroborated "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (g) pc M12 — B01 deletion axis: Rule 6 opening line deleted, 'NOT path-corroborated' sub-bullet survives in pc_aff (nullification predicate does not match this form; P23-007)]: old guard returns GREEN (BUG); B01 positional ^6. conjunct must fire RED"
    rm -f "$corpus_scratch"
    false
  fi

  # Restore M12: production adversary.md must pass guard pc.
  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
  run _guard_g_path_corroborated "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (g) pc M12 RESTORE]: production adversary.md returned non-zero from _guard_g_path_corroborated — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # M13: Rule 3 deleted — wtr guard fires via both empty-aff and positional checks.
  # 'worktree-rooted' appears only in Rule 3: deletion empties wtr_all → wtr_aff empty →
  # old guard already fires RED ✓. Positional conjunct adds defense-in-depth.
  grep -v '^3\. \*\*Use worktree-rooted' "$ADVERSARY_AGENT" > "$corpus_scratch"

  run _guard_d_worktree_rooted "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (d) wtr M13 — deletion axis: Rule 3 deleted (single 'worktree-rooted' occurrence)]: gate must fire RED (empty-aff or positional conjunct)"
    rm -f "$corpus_scratch"
    false
  fi

  # Restore M13: production adversary.md must pass guard wtr.
  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
  run _guard_d_worktree_rooted "$ADVERSARY_AGENT"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (d) wtr M13 RESTORE]: production adversary.md returned non-zero from _guard_d_worktree_rooted — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  # M14: off-limits prohibition sentence deleted from _shared-context.md — ol guard fires.
  # Single occurrence; deletion empties ol_all → ol_aff empty → old guard already fires RED ✓.
  # Positional conjunct also fires (empty section extract). Corpus vector explicitly covers
  # the deletion axis for _shared-context.md.
  grep -v 'off-limits' "$SHARED_CTX" > "$corpus_scratch"

  run _guard_l_off_limits "$corpus_scratch"
  if [ "$status" -eq 0 ]; then
    echo "CORPUS FAIL [guard (l) ol M14 — deletion axis: off-limits sentence deleted from _shared-context.md]: gate must fire RED (empty-aff or positional conjunct)"
    rm -f "$corpus_scratch"
    false
  fi

  # Restore M14: production _shared-context.md must pass guard ol.
  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
  run _guard_l_off_limits "$SHARED_CTX"
  if [ "$status" -ne 0 ]; then
    echo "CORPUS FAIL [guard (l) ol M14 RESTORE]: production _shared-context.md returned non-zero from _guard_l_off_limits — production artifact must be GREEN"
    rm -f "$corpus_scratch"
    false
  fi

  rm -f "$corpus_scratch"
}

# ===========================================================================
# COUPLING GATE (F-S2104-P27-STRUCT)
#
# Mechanical assertion that the story's AC-001 Gate cell stated gate count
# (numeric) and the bats suite's lead-in count-word (converted to integer)
# represent the same number. Any burst that changes either side without
# updating the other causes this gate to fire with a failure message naming
# both the story value and the bats value and what each reads.
#
# Direction-A failure: story gate count changes (e.g., 23 → 22) while bats
#   still says "Twenty-three" (23) → 22 ≠ 23 → RED ✓.
# Direction-B failure: bats count-word changes (e.g., "Twenty-three" → "Twenty-two")
#   while story still says "23 gates" (23) → 23 ≠ 22 → RED ✓.
# ===========================================================================

@test "test_coupling_gate_story_gate_count_matches_bats_count_word" {
  # Locate factory-artifacts worktree by path using the main checkout root as anchor.
  # F-S2104-P28-001: CI mounts .factory from origin/factory-artifacts (detached HEAD),
  # which yields no 'branch' line in --porcelain; branch-matching fails → CI-breaking.
  # F-S2104-P28-015: use ${line#worktree } prefix stripping instead of awk $2 to
  # preserve paths containing spaces (adversary.md Rule 2 space-safe mandate).
  # Anchor: git worktree list --porcelain is repo-global; the FIRST 'worktree' entry
  # is always the main checkout root regardless of which worktree the process runs in.
  # rev-parse --show-toplevel returns the CWD's worktree root (wrong when running
  # from a story worktree like S-21.04 — returns .worktrees/S-21.04, not the main root).
  # Idiom: _shared-context.md main_worktree_path pattern.
  # Precondition on CI lag: CI explicitly fetches origin/factory-artifacts before
  # mounting; if the fetched commit is current, story_count == branch-tip count.
  # A stale fetch could expose count divergence, but CI's fetch step mitigates this.
  local main_root fa_wt _line _wt_path
  main_root=""
  fa_wt=""
  while IFS= read -r _line; do
    case "$_line" in
      "worktree "*)
        _wt_path="${_line#worktree }"
        if [ -z "$main_root" ]; then
          main_root="$_wt_path"
        elif [ "$_wt_path" = "$main_root/.factory" ]; then
          fa_wt="$_wt_path"
          break
        fi
        ;;
    esac
  done < <(git -C "$PLUGIN_ROOT" worktree list --porcelain)
  if [ -z "$fa_wt" ]; then
    echo "COUPLING GATE FAIL: factory-artifacts worktree not found at $main_root/.factory; ensure .factory is mounted at the main checkout root (local: 'git worktree add .factory factory-artifacts'; CI: 'git worktree add .factory origin/factory-artifacts')"
    false
  fi

  local story_file bats_suite
  story_file="$fa_wt/stories/S-21.04-story-worktree-write-path-discipline.md"
  bats_suite="$PLUGIN_ROOT/tests/story-worktree-write-path-discipline.bats"

  if [ ! -f "$story_file" ]; then
    echo "COUPLING GATE FAIL: story file not found at $story_file"
    false
  fi

  # Extract story gate count from the AC-001 table row.
  # F-S2104-P28-002: anchored to '| AC-001 |' to avoid consuming the frontmatter
  # provenance line (which also contains '(NN gates' patterns quoting historical
  # counts). head -1 on the whole file takes the first match in the file, which
  # may be the frontmatter — anchoring to the AC-001 row is the authoritative site.
  local ac001_row story_count
  ac001_row="$(grep '| AC-001 |' "$story_file" | head -1)"
  story_count="$(printf '%s\n' "$ac001_row" | grep -oE '\([0-9]+ gates' | grep -oE '[0-9]+' | head -1)"
  if [ -z "$story_count" ]; then
    echo "COUPLING GATE FAIL: AC-001 table row not found or does not contain pattern '(NN gates' — story gate count not extractable (story: $story_file)"
    false
  fi

  # Extract bats lead-in count-word from "Twenty-[word] independently mutant-proven gates".
  local bats_word bats_count
  bats_word="$(grep -oiE 'Twenty-[a-z]+' "$bats_suite" | head -1)"
  if [ -z "$bats_word" ]; then
    echo "COUPLING GATE FAIL: bats suite does not contain 'Twenty-[word]' lead-in count-word (suite: $bats_suite)"
    false
  fi

  # Word-to-integer map. Extend when gate count exceeds current maximum.
  case "$(printf '%s' "$bats_word" | tr '[:upper:]' '[:lower:]')" in
    "twenty-one")   bats_count=21 ;;
    "twenty-two")   bats_count=22 ;;
    "twenty-three") bats_count=23 ;;
    "twenty-four")  bats_count=24 ;;
    "twenty-five")  bats_count=25 ;;
    "twenty-six")   bats_count=26 ;;
    "twenty-seven") bats_count=27 ;;
    "twenty-eight") bats_count=28 ;;
    "twenty-nine")  bats_count=29 ;;
    "thirty")       bats_count=30 ;;
    *)
      echo "COUPLING GATE FAIL: unrecognized bats count-word '$bats_word' — extend the word-to-integer map in this gate when the gate count grows"
      false
      ;;
  esac

  # Primary coupling assertion.
  if [ "$story_count" -ne "$bats_count" ]; then
    echo "COUPLING GATE FAIL [F-S2104-P27-STRUCT]: story gate count ($story_count) ≠ bats count-word numeric ($bats_count, from '$bats_word')"
    echo "  Story file : $story_file"
    echo "  Story reads: $(grep -oE '\([0-9]+ gates[^)]*\)' "$story_file" | head -1)"
    echo "  Bats suite : $bats_suite"
    echo "  Bats reads : $(grep -i 'Twenty' "$bats_suite" | head -1 | sed 's/^[[:space:]]*//')"
    echo "  Fix: update whichever side drifted so both represent the same gate count"
    false
  fi

  # F-S2104-P28-003: the former Direction-A and Direction-B corpus blocks were
  # arithmetic tautologies. After the primary equality assertion above,
  # story_count == bats_count is guaranteed; the blocks tested N-1 == N, which is
  # unconditionally false for any integer. They executed zero real mutant vectors
  # and the comments claiming direction coverage were a TD-VSDD-059 paper fix.
  # Option (a): delete the dead blocks and re-attest honestly. The single equality
  # assertion at the primary assertion above covers both drift directions:
  # Direction-A (story count changes, bats stays): story_count ≠ bats_count → RED ✓.
  # Direction-B (bats count-word changes, story stays): story_count ≠ bats_count → RED ✓.
}
