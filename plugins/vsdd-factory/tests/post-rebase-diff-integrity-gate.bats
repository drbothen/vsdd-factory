#!/usr/bin/env bats
# post-rebase-diff-integrity-gate.bats — failing TDD tests for S-21.02.
#
# Tests T-001..T-003 covering AC-003..AC-005 for the mandatory post-rebase
# diff-integrity gate (BC-5.44.001, Invariant 1).
#
# Gate host: plugins/vsdd-factory/agents/devops-engineer.md §Inter-Wave Rebase
# (the only codebase site with git rebase origin/develop + git push --force-with-lease,
# per ADR-031 v1.3 §Consequences #5).
#
# Red Gate design (RG-001..RG-003):
#   Pre-implementation, §Inter-Wave Rebase contains NO gate between
#   `git rebase origin/develop` and `git push --force-with-lease`.
#   Tests extract the §Inter-Wave Rebase section, assert gate-procedure markers
#   (`range-diff`, `UnverifiedNetNegativeDelta`) and per-postcondition behavior.
#   Extraction finds no gate → assertions FAIL with behavior-referencing messages.
#   NOT harness/setup errors and NOT bare "file not found" crashes.
#
# Story:  S-21.02 (E-21 Wave 1 — post-rebase diff-integrity gate)
# BC:     BC-5.44.001 v1.3 (PC1/PC2/PC3, Invariant 1, EC-005)
# ADR:    ADR-031 v1.3 §Decision 6 + §Consequences #5
#
# Test Plan:
#   | Test name                                                   | AC     | BC trace         |
#   |-------------------------------------------------------------|--------|------------------|
#   | T-001 gate halts on unverified net-negative delta           | AC-003 | PC2, Invariant 1 |
#   | T-002 gate passes when no sibling file overlap              | AC-004 | PC3              |
#   | T-003 gate passes for confirmed intentional removal         | AC-005 | PC1              |

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DEVOPS_ENGINEER_MD="$PLUGIN_ROOT/agents/devops-engineer.md"
  WORK="$(mktemp -d)"
}

teardown() {
  [ -n "${WORK:-}" ] && rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Extract the §Inter-Wave Rebase section body from devops-engineer.md.
# Outputs lines after "### Inter-Wave Rebase" up to (but not including)
# the next "### " heading. Returns empty string if section not found.
_extract_inter_wave_rebase_section() {
  awk '
    /^### Inter-Wave Rebase/ { found=1; next }
    found && /^### / { exit }
    found { print }
  ' "$DEVOPS_ENGINEER_MD"
}

# Create a minimal git fixture in $WORK simulating the given post-rebase scenario.
# $1 = scenario:
#   "net-negative-sibling" — feature branch shows net -4 lines on autoload.gd,
#                            which was also touched by sibling story S-20.01.
#                            Agent cannot confirm removal is intentional (PC2 case).
#   "no-sibling-overlap"   — feature branch changes only feature_only.rs;
#                            sibling story S-20.01 touched only autoload.gd.
#                            No file overlap → PC3 pass case.
#   "intentional-removal"  — feature branch shows net -4 lines on autoload.gd,
#                            deletion is present in feature branch's own commit
#                            history (marked intentional) → PC1 pass case.
# Sets FIXTURE_REPO to the working clone directory after setup.
_setup_git_fixture() {
  local scenario="$1"
  local origin_dir="$WORK/origin-${scenario}.git"
  local repo_dir="$WORK/repo-${scenario}"

  git init -q --bare "$origin_dir"
  git clone -q "$origin_dir" "$repo_dir" 2>/dev/null
  cd "$repo_dir"
  git config user.email "test@vsdd.local"
  git config user.name "Test"

  case "$scenario" in
    net-negative-sibling)
      # origin/develop: autoload.gd with 8 lines; last 2 added by sibling S-20.01.
      # Feature branch: removes lines 3-6 (net -4 vs origin/develop).
      # Agent cannot confirm whether removal is intentional — PC2 scenario.
      git checkout -q -b develop 2>/dev/null || true
      printf 'line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\n' > autoload.gd
      git add autoload.gd
      git commit -q -m "feat(S-20.01): autoload.gd 8 lines (sibling story commit on develop)"
      git push -q origin develop

      git checkout -q -b feature/S-21.02
      # Post-rebase state: lines 3-6 are absent vs origin/develop (net -4 lines)
      printf 'line1\nline2\nline7\nline8\n' > autoload.gd
      git add autoload.gd
      git commit -q -m "feat(S-21.02): trim autoload.gd (removed 4 lines)"
      ;;

    no-sibling-overlap)
      # origin/develop: sibling S-20.01 touched autoload.gd only.
      # Feature branch: changes feature_only.rs only — no file overlap with sibling.
      git checkout -q -b develop 2>/dev/null || true
      printf 'fn original() {}\nfn sibling_fn() {}\n' > autoload.gd
      printf 'fn feature_fn() {}\n' > feature_only.rs
      git add autoload.gd feature_only.rs
      git commit -q -m "feat(S-20.01): add sibling_fn to autoload.gd"
      git push -q origin develop

      git checkout -q -b feature/S-21.02
      # Feature only changes feature_only.rs — no overlap with sibling's autoload.gd
      printf 'fn feature_fn() {}\nfn new_feature() {}\n' > feature_only.rs
      git add feature_only.rs
      git commit -q -m "feat(S-21.02): add new_feature to feature_only.rs"
      ;;

    intentional-removal)
      # origin/develop: sibling S-20.01 touched autoload.gd.
      # Feature branch: removes lines 3-6 (net -4), explicitly documented as intentional
      # in the feature branch's own commit message and history — PC1 scenario.
      git checkout -q -b develop 2>/dev/null || true
      printf 'line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\n' > autoload.gd
      git add autoload.gd
      git commit -q -m "feat(S-20.01): autoload.gd 8 lines (sibling story commit on develop)"
      git push -q origin develop

      git checkout -q -b feature/S-21.02
      # Intentional removal: commit message and scope explicitly confirm the deletion
      printf 'line1\nline2\nline7\nline8\n' > autoload.gd
      git add autoload.gd
      git commit -q -m "feat(S-21.02): intentionally remove lines 3-6 from autoload.gd (confirmed dead code per S-21.02 scope; lines absent from feature branch own history by design)"
      ;;
  esac

  FIXTURE_REPO="$repo_dir"
}

# ---------------------------------------------------------------------------
# T-001 / AC-003: Gate halts — unverified net-negative delta in sibling-touched file
# BC-5.44.001 PC2, Invariant 1 — RG-001
# ---------------------------------------------------------------------------

@test "T-001 S-21.02 AC-003: gate halts on unverified net-negative delta in sibling-touched file" {
  # Fixture: git diff origin/develop --stat shows net -4 lines on autoload.gd.
  # autoload.gd was also modified by recently-merged sibling S-20.01.
  # Gate MUST: (a) detect the delta, (b) NOT invoke force-push,
  # (c) emit UnverifiedNetNegativeDelta error signal (BC-5.44.001 PC2).
  _setup_git_fixture "net-negative-sibling"

  cd "$FIXTURE_REPO"

  # Fixture sanity: verify the scenario produces net-negative delta on autoload.gd
  local stat_output
  stat_output="$(git diff origin/develop -- autoload.gd --stat 2>/dev/null || true)"
  echo "$stat_output" | grep -q "autoload.gd" || {
    echo "TEST-SETUP ERROR: fixture did not produce a diff on autoload.gd — scenario setup failed"
    false
  }

  # Fixture sanity: verify sibling S-20.01 commit appears on origin/develop
  local sibling_log
  sibling_log="$(git log --oneline origin/develop 2>/dev/null || true)"
  echo "$sibling_log" | grep -q "S-20.01" || {
    echo "TEST-SETUP ERROR: sibling S-20.01 commit not found on origin/develop — fixture setup failed"
    false
  }

  # Extract §Inter-Wave Rebase section from devops-engineer.md
  local section
  section="$(_extract_inter_wave_rebase_section)"

  # ASSERTION (a) — BC-5.44.001 step 1a, AC-003(a):
  # Gate must include `git range-diff` as the PRIMARY detector.
  # range-diff is the canonical tool for detecting replayed-commit content changes
  # during a rebase (git >= 2.19). Its absence means silent drops go undetected.
  echo "$section" | grep -qE "range-diff" || {
    echo "FAIL: T-001 / AC-003 / BC-5.44.001 PC2 step 1a (RG-001):"
    echo "  devops-engineer.md §Inter-Wave Rebase must contain a post-rebase diff-integrity"
    echo "  gate step with 'git range-diff' as the primary detector (BC-5.44.001 step 1a)."
    echo "  The gate is absent — implementer must insert it between"
    echo "  'git rebase origin/develop' and 'git push --force-with-lease'."
    echo ""
    echo "  Fixture confirms the scenario: git diff origin/develop --stat on the feature"
    echo "  branch shows net -4 lines on autoload.gd; sibling S-20.01 also modified"
    echo "  autoload.gd on origin/develop. The gate MUST detect this and halt."
    echo ""
    echo "  Expected gate markers absent from §Inter-Wave Rebase: range-diff"
    false
  }

  # ASSERTION (b) — BC-5.44.001 PC2, Invariant 1, AC-003(b)+(c):
  # Gate must emit STOP with UnverifiedNetNegativeDelta error variant when an
  # unverified net-negative delta is found in a sibling-touched file.
  # git push --force-with-lease MUST NOT execute until the gate clears.
  echo "$section" | grep -qE "UnverifiedNetNegativeDelta" || {
    echo "FAIL: T-001 / AC-003 / BC-5.44.001 PC2 (RG-001):"
    echo "  devops-engineer.md §Inter-Wave Rebase must contain 'UnverifiedNetNegativeDelta'"
    echo "  as the halt-signal error variant for the PC2 path."
    echo "  When the agent cannot confirm a net-negative delta is intentional,"
    echo "  it MUST emit STOP with this error variant. git push --force-with-lease"
    echo "  MUST NOT execute before the gate passes (BC-5.44.001 Invariant 1)."
    echo ""
    echo "  'UnverifiedNetNegativeDelta' is absent from §Inter-Wave Rebase."
    false
  }

  # ASSERTION (c) — BC-5.44.001 step 1b, EC-005, AC-001(b):
  # Gate must document the --stat backup heuristic for when git range-diff is
  # unavailable (git < 2.19). Falls back to git diff origin/develop --stat.
  echo "$section" | grep -qE "\-\-stat" || {
    echo "FAIL: T-001 / AC-003 / BC-5.44.001 step 1b (EC-005):"
    echo "  devops-engineer.md §Inter-Wave Rebase must document the 'git diff"
    echo "  origin/develop --stat' fallback for when git range-diff is unavailable"
    echo "  (git < 2.19). BC-5.44.001 step 1b mandates this backup heuristic."
    echo "  '--stat' fallback is absent from §Inter-Wave Rebase."
    false
  }

  # ASSERTION (d) — BC-5.44.001 Invariant 1:
  # Gate must appear BETWEEN 'git rebase origin/develop' and 'git push --force-with-lease'
  # in the section text — not after the force-push (too late to recover dropped lines).
  local rebase_line force_push_line gate_line
  rebase_line="$(echo "$section" | grep -n "git rebase origin/develop" | head -1 | cut -d: -f1)"
  force_push_line="$(echo "$section" | grep -n "force-with-lease" | head -1 | cut -d: -f1)"
  gate_line="$(echo "$section" | grep -nE "UnverifiedNetNegativeDelta|range-diff" | head -1 | cut -d: -f1)"

  [ -n "$rebase_line" ] && [ -n "$force_push_line" ] && [ -n "$gate_line" ] || {
    echo "FAIL: T-001 / AC-003 / BC-5.44.001 Invariant 1 (ordering cannot be verified):"
    echo "  §Inter-Wave Rebase is missing one or more required gate markers."
    echo "  Required: 'git rebase origin/develop' (line: ${rebase_line:-absent}),"
    echo "  'force-with-lease' (line: ${force_push_line:-absent}),"
    echo "  gate marker range-diff/UnverifiedNetNegativeDelta (line: ${gate_line:-absent})."
    echo "  All three must be present to verify gate ordering (gate between rebase and push)."
    false
  }

  [ "$gate_line" -gt "$rebase_line" ] && [ "$gate_line" -lt "$force_push_line" ] || {
    echo "FAIL: T-001 / AC-003 / BC-5.44.001 Invariant 1 (gate ordering violated):"
    echo "  Gate marker must appear AFTER 'git rebase origin/develop' (line $rebase_line)"
    echo "  and BEFORE 'git push --force-with-lease' (line $force_push_line)."
    echo "  Running the gate post-force-push cannot recover silently dropped lines."
    echo "  Gate marker is at line $gate_line."
    false
  }
}

# ---------------------------------------------------------------------------
# T-002 / AC-004: Gate passes — no sibling file overlap (PC3)
# BC-5.44.001 PC3 — RG-002
# ---------------------------------------------------------------------------

@test "T-002 S-21.02 AC-004: gate passes — no sibling file overlap (PC3)" {
  # Fixture: feature branch changes only feature_only.rs; sibling S-20.01 touched
  # only autoload.gd. No file overlap → gate must be INVOKED and then pass (PC3).
  # RG-002: pre-implementation the gate is absent entirely, so force-push proceeds
  # trivially with no gate invocation. The test verifies the gate IS invoked.
  _setup_git_fixture "no-sibling-overlap"

  cd "$FIXTURE_REPO"

  # Fixture sanity: verify feature diff shows only feature_only.rs, not autoload.gd
  local stat_output
  stat_output="$(git diff origin/develop --stat 2>/dev/null || true)"
  echo "$stat_output" | grep -q "feature_only.rs" || {
    echo "TEST-SETUP ERROR: fixture did not produce diff on feature_only.rs"
    false
  }
  if echo "$stat_output" | grep -q "autoload.gd"; then
    echo "TEST-SETUP ERROR: fixture unexpectedly shows autoload.gd in feature diff (should be no overlap)"
    false
  fi

  # Extract §Inter-Wave Rebase section
  local section
  section="$(_extract_inter_wave_rebase_section)"

  # ASSERTION (a) — Gate must be documented to establish that it IS invoked.
  # RG-002: test checks the gate is invoked (not that force-push merely proceeds
  # trivially with no gate). The gate must check for sibling overlap even to make
  # a PC3 trivial-pass decision.
  echo "$section" | grep -qE "range-diff|UnverifiedNetNegativeDelta" || {
    echo "FAIL: T-002 / AC-004 / BC-5.44.001 PC3 (RG-002):"
    echo "  devops-engineer.md §Inter-Wave Rebase must document the post-rebase"
    echo "  diff-integrity gate (containing 'range-diff' or 'UnverifiedNetNegativeDelta')"
    echo "  so that the gate is demonstrably INVOKED before the PC3 pass decision."
    echo ""
    echo "  Pre-implementation, the gate is absent — force-push proceeds trivially with"
    echo "  no gate invocation at all. RG-002 fails because this test verifies the gate"
    echo "  IS invoked (even for the no-overlap path) before force-push-with-lease."
    echo ""
    echo "  Fixture: feature diff shows only feature_only.rs; sibling S-20.01 touched"
    echo "  only autoload.gd — no file overlap. Gate must run, check for overlap,"
    echo "  find none, and pass (PC3). Gate markers absent from §Inter-Wave Rebase."
    false
  }

  # ASSERTION (b) — BC-5.44.001 PC3:
  # Section must document the no-overlap pass condition: when git diff origin/develop
  # --stat shows no file also in the sibling-story commit set, the gate passes.
  echo "$section" | grep -qiE "no.*(sibling|overlap)|PC3|trivially|sibling.*(no|not)" || {
    echo "FAIL: T-002 / AC-004 / BC-5.44.001 PC3:"
    echo "  devops-engineer.md §Inter-Wave Rebase must document PC3 behavior:"
    echo "  when no file in git diff origin/develop --stat overlaps with recently-merged"
    echo "  sibling story commits, the gate passes (force-push-with-lease proceeds)."
    echo "  No-overlap pass condition language is absent from §Inter-Wave Rebase."
    echo ""
    echo "  Fixture: feature only changed feature_only.rs; sibling touched only"
    echo "  autoload.gd — no overlap. Gate must document this PC3 pass path."
    false
  }
}

# ---------------------------------------------------------------------------
# T-003 / AC-005: Gate passes — confirmed intentional removal (PC1)
# BC-5.44.001 PC1 — RG-003
# ---------------------------------------------------------------------------

@test "T-003 S-21.02 AC-005: gate passes — confirmed intentional removal (PC1)" {
  # Fixture: feature branch shows net -4 lines on autoload.gd (same diff profile
  # as T-001), but the deletion is confirmed as intentional — present in the
  # feature branch's own commit history. Gate MUST be invoked AND pass (PC1).
  _setup_git_fixture "intentional-removal"

  cd "$FIXTURE_REPO"

  # Fixture sanity: verify net-negative delta on autoload.gd exists
  local stat_output
  stat_output="$(git diff origin/develop -- autoload.gd --stat 2>/dev/null || true)"
  echo "$stat_output" | grep -q "autoload.gd" || {
    echo "TEST-SETUP ERROR: fixture did not produce diff on autoload.gd"
    false
  }

  # Fixture sanity: verify the feature branch's own commit history records the intentional removal
  local feature_log
  feature_log="$(git log --oneline origin/develop..HEAD 2>/dev/null || true)"
  echo "$feature_log" | grep -qiE "intentional|intentionally" || {
    echo "TEST-SETUP ERROR: feature branch commit does not document intentional removal"
    false
  }

  # Extract §Inter-Wave Rebase section
  local section
  section="$(_extract_inter_wave_rebase_section)"

  # ASSERTION (a) — Gate must be documented to establish it IS invoked.
  # RG-003: pre-implementation, the gate is absent — test fails here.
  # Post-implementation, gate markers present → assertion passes.
  echo "$section" | grep -qE "range-diff|UnverifiedNetNegativeDelta" || {
    echo "FAIL: T-003 / AC-005 / BC-5.44.001 PC1 (RG-003):"
    echo "  devops-engineer.md §Inter-Wave Rebase must document the post-rebase"
    echo "  diff-integrity gate (containing 'range-diff' or 'UnverifiedNetNegativeDelta')"
    echo "  so that the gate is demonstrably INVOKED before the PC1 pass decision."
    echo ""
    echo "  Pre-implementation, the gate is absent — all three RG tests fail here."
    echo "  Gate markers absent from §Inter-Wave Rebase."
    echo ""
    echo "  Fixture: feature shows net -4 lines on autoload.gd; sibling also touched"
    echo "  autoload.gd; but the feature branch's own commit log records the removal as"
    echo "  intentional. Gate must invoke the check and pass via PC1."
    false
  }

  # ASSERTION (b) — BC-5.44.001 PC1:
  # Section must document the PC1 intentional-removal pass behavior: when the agent
  # explicitly inspects the diff hunk and confirms the removal is a deliberate code
  # change present in the feature branch's own commit history, the gate passes.
  echo "$section" | grep -qiE "intentional|PC1|confirmed.*remov|remov.*confirm|verify.*intent|deliberate" || {
    echo "FAIL: T-003 / AC-005 / BC-5.44.001 PC1:"
    echo "  devops-engineer.md §Inter-Wave Rebase must document PC1 behavior:"
    echo "  when the agent verifies a net-negative delta as an intentional removal"
    echo "  (confirmed by explicit inspection of the feature branch's own commit history),"
    echo "  the gate passes and git push --force-with-lease may proceed."
    echo ""
    echo "  BC-5.44.001 Invariant 4: 'Confirmed as intentional' means the agent explicitly"
    echo "  inspects the diff hunk. Absence of conflict markers is NOT sufficient."
    echo "  PC1 intentional-removal pass condition is absent from §Inter-Wave Rebase."
    echo ""
    echo "  Fixture: feature log shows 'intentionally remove lines 3-6 from autoload.gd"
    echo "  (confirmed dead code per S-21.02 scope)' — agent inspection confirms PC1 pass."
    false
  }

  # ASSERTION (c) — BC-5.44.001 PC1+PC2 distinction:
  # Section must distinguish the STOP path (PC2 — unverified) from the pass path
  # (PC1 — confirmed intentional). Both outcomes must be present in the gate procedure.
  local has_stop has_pc1_proceed
  echo "$section" | grep -qiE "STOP|halt|UnverifiedNetNegativeDelta" && has_stop=1 || has_stop=0
  echo "$section" | grep -qiE "intentional|PC1|confirmed.*remov|deliberate" && has_pc1_proceed=1 || has_pc1_proceed=0

  [ "$has_stop" -eq 1 ] && [ "$has_pc1_proceed" -eq 1 ] || {
    echo "FAIL: T-003 / AC-005 / BC-5.44.001 PC1+PC2 distinction:"
    echo "  §Inter-Wave Rebase must document BOTH the halt path (PC2 — STOP with"
    echo "  UnverifiedNetNegativeDelta for unverified net-negative delta) AND the pass"
    echo "  path (PC1 — proceed when intentional removal is confirmed by inspection)."
    echo "  Both outcomes must be explicit so the agent can distinguish the two cases."
    echo "  has_stop_signal=$has_stop, has_pc1_proceed_signal=$has_pc1_proceed"
    echo "  (0=absent, 1=present)"
    false
  }
}
