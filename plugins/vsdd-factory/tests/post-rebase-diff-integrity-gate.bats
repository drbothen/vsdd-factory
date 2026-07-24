#!/usr/bin/env bats
# post-rebase-diff-integrity-gate.bats — S-21.02 gate harness suite.
#
# Two load-bearing layers:
#   DOC-PARITY:          grep assertions on devops-engineer.md §Inter-Wave Rebase.
#                        If the gate section is deleted or reordered, these fail.
#   EXECUTABLE-HARNESS:  bash functions implementing the BC-5.44.001 gate procedure,
#                        executed against real git fixtures with an observable push stub.
#
# Gate host: plugins/vsdd-factory/agents/devops-engineer.md §Inter-Wave Rebase
# (ADR-031 §Decision 6 + §Consequences #5)
# BC: BC-5.44.001 (PC1/PC2/PC3/PC4, Invariant 1)
# Story: S-21.02
#
# Test plan:
#   T-001  AC-003 / PC2: real-rebase fixture; range-diff primary detects !-commit; HALT; UnverifiedNetNegativeDelta
#   T-002  AC-004 / PC3: real-rebase, no sibling overlap; gate passes; push invoked
#   T-003  AC-005 / PC1: real-rebase, intentional commit message; gate passes; push invoked
#   T-004  BC-5.44.001 PC4 / EC-006: no sibling commits; trivial pass; push invoked
#   T-005  EC-004-class / EC-005: detector failure / merge-base failure → escalate; rd-only → PC2; push NOT invoked

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DEVOPS_ENGINEER_MD="$PLUGIN_ROOT/agents/devops-engineer.md"
  STEP_F_MD="$PLUGIN_ROOT/skills/deliver-story/steps/step-f-pr-lifecycle.md"
  WORK="$(mktemp -d)"
  PUSH_LOG="$WORK/push.log"
  touch "$PUSH_LOG"
}

teardown() {
  [ -n "${WORK:-}" ] && rm -rf "$WORK"
}

# ===========================================================================
# DOC-PARITY helpers
# ===========================================================================

_extract_inter_wave_rebase_section() {
  awk '
    /^### Inter-Wave Rebase/ { found=1; next }
    found && /^### / { exit }
    found { print }
  ' "$DEVOPS_ENGINEER_MD"
}

_assert_doc_marker() {
  # $1=regex, $2=label, $3=section_text
  echo "$3" | grep -qE "$1" || {
    echo "DOC-PARITY FAIL [§Inter-Wave Rebase must contain: $2]"
    false
  }
}

_assert_step_f_marker() {
  # $1=regex, $2=label
  grep -qE "$1" "$STEP_F_MD" || {
    echo "DOC-PARITY FAIL [step-f-pr-lifecycle.md must contain: $2]"
    false
  }
}

# ===========================================================================
# EXECUTABLE-HARNESS — git fixture setup
# ===========================================================================

# Build a git fixture for the given scenario. Sets FIXTURE_REPO and FIXTURE_PRE_REBASE_TIP.
# FIXTURE_PRE_REBASE_TIP is captured immediately before `git rebase` (matching the doc's
# `PRE_REBASE_TIP=$(git rev-parse HEAD)` capture point) so range-diff exercises a real
# pre→post SHA transition.
#
# Scenarios:
#   net-negative-sibling  — 12-line file; feature deletes line06; sibling renames line06
#                           to SIBLING06; conflict auto-resolved with -X theirs (feature's
#                           deletion wins); range-diff shows ! (modified commit; old diff
#                           deleted "line06", new diff deletes "SIBLING06"); net-negative
#                           vs origin/develop; no intentionality signal → PC2
#   no-sibling-overlap    — feature changes only feature_only.rs; sibling only autoload.gd;
#                           clean rebase (no conflict); range-diff shows = → PC3
#   intentional-removal   — same rebase topology as net-negative-sibling; range-diff shows !;
#                           commit message documents intentional removal → PC1
#   no-sibling-commits    — feature branches from develop; no new commits to develop;
#                           FIXTURE_PRE_REBASE_TIP = HEAD; no rebase needed → PC4
_setup_git_fixture() {
  local scenario="$1"
  local origin_dir="$WORK/origin-${scenario}.git"
  local repo_dir="$WORK/repo-${scenario}"

  git init -q --bare "$origin_dir"
  git clone -q "$origin_dir" "$repo_dir" 2>/dev/null
  git -C "$repo_dir" config user.email "test@vsdd.local"
  git -C "$repo_dir" config user.name "Test"

  case "$scenario" in
    net-negative-sibling)
      # Base: 12-line autoload.gd (gives enough context for range-diff to match commits).
      # Feature: delete line06 (net -1; no intentionality signal).
      # Sibling: rename line06 → SIBLING06 (same line → conflict with feature's deletion).
      # Rebase -X theirs: feature's deletion wins; post-rebase tree lacks line06/SIBLING06.
      # range-diff: old commit deleted "line06", new commit deleted "SIBLING06" → diffs
      #   differ with high similarity → ! (modified) ✓
      # git diff --numstat origin/develop: 0/1 → net-negative ✓
      git -C "$repo_dir" checkout -q -b develop 2>/dev/null || true
      printf 'line01\nline02\nline03\nline04\nline05\nline06\nline07\nline08\nline09\nline10\nline11\nline12\n' \
        > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "init: autoload.gd 12 lines"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q -b feature/S-21.02
      printf 'line01\nline02\nline03\nline04\nline05\nline07\nline08\nline09\nline10\nline11\nline12\n' \
        > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-21.02): remove line06 from autoload.gd"

      # Sibling renames line06 → SIBLING06 — conflicts with feature's deletion of line06
      git -C "$repo_dir" checkout -q develop
      printf 'line01\nline02\nline03\nline04\nline05\nSIBLING06\nline07\nline08\nline09\nline10\nline11\nline12\n' \
        > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-20.01): rename line06 to SIBLING06 (sibling story)"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q feature/S-21.02

      # -X theirs: feature's deletion wins the conflict (SIBLING06 is deleted).
      FIXTURE_PRE_REBASE_TIP="$(git -C "$repo_dir" rev-parse HEAD)"
      git -C "$repo_dir" rebase -X theirs -q origin/develop 2>/dev/null
      ;;

    no-sibling-overlap)
      # 1. develop: init with autoload.gd + feature_only.rs
      git -C "$repo_dir" checkout -q -b develop 2>/dev/null || true
      printf 'fn original() {}\n' > "$repo_dir/autoload.gd"
      printf 'fn feature_fn() {}\n' > "$repo_dir/feature_only.rs"
      git -C "$repo_dir" add autoload.gd feature_only.rs
      git -C "$repo_dir" commit -q -m "init: initial commit"
      git -C "$repo_dir" push -q origin develop

      # 2. Feature branches — changes only feature_only.rs (no overlap with sibling)
      git -C "$repo_dir" checkout -q -b feature/S-21.02
      printf 'fn feature_fn() {}\nfn new_feature() {}\n' > "$repo_dir/feature_only.rs"
      git -C "$repo_dir" add feature_only.rs
      git -C "$repo_dir" commit -q -m "feat(S-21.02): add new_feature to feature_only.rs"

      # 3. Sibling touches only autoload.gd — disjoint file set; clean rebase
      git -C "$repo_dir" checkout -q develop
      printf 'fn original() {}\nfn sibling_fn() {}\n' > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-20.01): add sibling_fn to autoload.gd"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q feature/S-21.02

      # 4. Capture PRE_REBASE_TIP then clean rebase (no conflict).
      #    After rebase the feature tree includes sibling's autoload.gd changes;
      #    git diff --numstat origin/develop shows 0/0 on autoload.gd → no phantom delta.
      FIXTURE_PRE_REBASE_TIP="$(git -C "$repo_dir" rev-parse HEAD)"
      git -C "$repo_dir" rebase -q origin/develop 2>/dev/null
      ;;

    intentional-removal)
      # Same 12-line topology as net-negative-sibling; commit message documents intent → PC1
      git -C "$repo_dir" checkout -q -b develop 2>/dev/null || true
      printf 'line01\nline02\nline03\nline04\nline05\nline06\nline07\nline08\nline09\nline10\nline11\nline12\n' \
        > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "init: autoload.gd 12 lines"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q -b feature/S-21.02
      printf 'line01\nline02\nline03\nline04\nline05\nline07\nline08\nline09\nline10\nline11\nline12\n' \
        > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-21.02): intentionally remove line06 from autoload.gd (confirmed dead code per S-21.02 scope)"

      git -C "$repo_dir" checkout -q develop
      printf 'line01\nline02\nline03\nline04\nline05\nSIBLING06\nline07\nline08\nline09\nline10\nline11\nline12\n' \
        > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-20.01): rename line06 to SIBLING06 (sibling story)"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q feature/S-21.02

      FIXTURE_PRE_REBASE_TIP="$(git -C "$repo_dir" rev-parse HEAD)"
      git -C "$repo_dir" rebase -X theirs -q origin/develop 2>/dev/null
      ;;

    no-sibling-commits)
      # Feature branches from develop; no new commits to develop → PC4
      git -C "$repo_dir" checkout -q -b develop 2>/dev/null || true
      printf 'fn main() {}\n' > "$repo_dir/main.rs"
      git -C "$repo_dir" add main.rs
      git -C "$repo_dir" commit -q -m "init: initial commit"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q -b feature/S-21.02
      printf 'fn main() { println!("hello"); }\n' > "$repo_dir/main.rs"
      git -C "$repo_dir" add main.rs
      git -C "$repo_dir" commit -q -m "feat(S-21.02): update main.rs"
      # No new commits on develop → merge-base == origin/develop HEAD → PC4.
      # No rebase needed; FIXTURE_PRE_REBASE_TIP = current HEAD.
      FIXTURE_PRE_REBASE_TIP="$(git -C "$repo_dir" rev-parse HEAD)"
      ;;
  esac

  FIXTURE_REPO="$repo_dir"
}

# ===========================================================================
# EXECUTABLE-HARNESS — gate implementation (BC-5.44.001 procedure)
# ===========================================================================

# Runs the post-rebase diff-integrity gate as documented in devops-engineer.md
# §Inter-Wave Rebase (ADR-031 §Decision 6 + §Consequences #5).
#
# Args:
#   $1  repo_dir         — path to the git fixture repo
#   $2  pre_rebase_tip   — SHA captured before the rebase (FIXTURE_PRE_REBASE_TIP)
#   $3  push_log         — path to push-log file; "push-invoked\n" written on success
#   $4  force_rd_fail    — 1 to simulate range-diff failure, 0 (default) to run normally
#   $5  force_stat_fail  — 1 to simulate --stat failure, 0 (default) to run normally
#
# Output: gate result text (GATE-PASS / STOP / ESCALATE) written to stdout.
_run_gate() {
  local repo="$1"
  local pre_rebase_tip="$2"
  local push_log="$3"
  local force_rd_fail="${4:-0}"
  local force_stat_fail="${5:-0}"
  local sibling_files rd_flagged_files

  sibling_files="$(mktemp)"
  rd_flagged_files="$(mktemp)"

  # Compute merge-base FIRST: needed for both range-diff ranges and sibling enumeration.
  # Failure or empty output → escalate: cannot determine sibling commit set.
  # Mirrors devops-engineer.md §Inter-Wave Rebase exit 1 escalation branch (commit
  # 6eb4b6a2): `if [ $? -ne 0 ] || [ -z "${MERGE_BASE}" ]; then exit 1`.
  local mb_result mb_exit merge_base develop_tip
  mb_result="$(git -C "$repo" merge-base "$pre_rebase_tip" origin/develop 2>/dev/null)"
  mb_exit=$?
  merge_base="$mb_result"

  if [ "$mb_exit" -ne 0 ] || [ -z "$merge_base" ]; then
    echo "ESCALATE: git merge-base failed or returned empty — escalating to manual review."
    rm -f "$sibling_files" "$rd_flagged_files"
    return 0
  fi

  develop_tip="$(git -C "$repo" rev-parse origin/develop 2>/dev/null || echo "")"

  # PC4: no sibling commits since branch creation (merge-base == develop tip)
  if [ "$merge_base" = "$develop_tip" ]; then
    echo "GATE-PASS: PC4 — no sibling commits since branch creation."
    echo "push-invoked" >> "$push_log"
    rm -f "$sibling_files" "$rd_flagged_files"
    return 0
  fi

  # Step 1a — Primary detector: git range-diff (git >= 2.19).
  # Three-dot form per spec: PRE_REBASE_TIP...POST_REBASE_TIP (symmetric difference).
  # Left = old feature commits; Right = sibling commits + rebased feature commits.
  # Parses for ! (modified) commits: old feature matched against sibling (both touch same
  # file) → ! signals the file as an ORT silent-drop candidate.
  local post_rebase_tip rd_out rd_ok
  post_rebase_tip="$(git -C "$repo" rev-parse HEAD 2>/dev/null || echo "INVALID")"
  rd_ok=0
  rd_out=""
  if [ "$force_rd_fail" -eq 0 ]; then
    rd_out="$(git -C "$repo" range-diff \
        "${pre_rebase_tip}...${post_rebase_tip}" 2>&1)" \
      && rd_ok=1 || rd_ok=0
    if [ "$rd_ok" -eq 1 ] && [ -n "$rd_out" ]; then
      while IFS= read -r rdline; do
        # Modified commits show: " N:  <sha> ! N:  <sha> <subject>"
        if echo "$rdline" | grep -qE '^[[:space:]]*[0-9]+:[[:space:]]+[0-9a-f]+ !'; then
          local post_sha
          post_sha="$(echo "$rdline" | awk '{
            for(i=1;i<=NF;i++) {
              if ($i == "!") {
                sha = $(i+2)
                gsub(/[^0-9a-f]/, "", sha)
                if (length(sha) >= 7) { print sha; exit }
              }
            }
          }')"
          if [ -n "$post_sha" ]; then
            git -C "$repo" diff-tree --no-commit-id --name-only -r "$post_sha" 2>/dev/null \
              >> "$rd_flagged_files" || true
          fi
        fi
      done <<< "$rd_out"
    fi
  fi

  # Step 1b — Backup heuristic: git diff origin/develop --stat
  local stat_out stat_ok
  stat_ok=0
  stat_out=""
  if [ "$force_stat_fail" -eq 0 ]; then
    stat_out="$(git -C "$repo" diff --stat origin/develop 2>&1)" \
      && stat_ok=1 || stat_ok=0
  fi

  # EC-005: both detectors failed → escalate, never push blind
  if [ "$rd_ok" -eq 0 ] && [ "$stat_ok" -eq 0 ]; then
    echo "ESCALATE: Post-rebase diff-integrity gate: both detectors failed. Not proceeding to push."
    rm -f "$sibling_files" "$rd_flagged_files"
    return 0
  fi

  # Enumerate files touched by sibling commits (merge-base..origin/develop)
  while IFS= read -r sha_line; do
    local sha
    sha="$(echo "$sha_line" | awk '{print $1}')"
    [ -z "$sha" ] && continue
    git -C "$repo" diff-tree --no-commit-id --name-only -r "$sha" 2>/dev/null \
      >> "$sibling_files" || true
  done < <(git -C "$repo" log --oneline "${merge_base}..origin/develop" 2>/dev/null)

  # Step 1b analysis: collect stat-flagged files (net-negative + sibling-touched).
  # Condition (c) feature-history filter removed (F-P2-002): after a real rebase the
  # feature tree already contains sibling additions, so phantom reverse-deltas no longer
  # appear on files the feature branch never actually modified.
  # Guarded by force_stat_fail: if stat detector is disabled, numstat analysis is also
  # skipped so the flag faithfully simulates total stat-detector failure (OBS-2).
  local flagged=()
  if [ "$force_stat_fail" -eq 0 ]; then
    while IFS=$'\t' read -r ins del fname; do
      [ -z "$fname" ] && continue
      if [ "${del:-0}" -gt "${ins:-0}" ] 2>/dev/null; then
        if grep -qxF "$fname" "$sibling_files" 2>/dev/null; then
          flagged+=("$fname")
        fi
      fi
    done < <(git -C "$repo" diff --numstat origin/develop 2>/dev/null)
  fi

  # Merge range-diff primary flagged files: add sibling-touched files from ! commits not
  # already captured by stat (covers ORT silent-drop where numstat shows 0 net delta but
  # range-diff shows the commit was modified during rebase replay).
  while IFS= read -r rd_fname; do
    [ -z "$rd_fname" ] && continue
    if grep -qxF "$rd_fname" "$sibling_files" 2>/dev/null; then
      local already=0
      local ef
      for ef in "${flagged[@]+"${flagged[@]}"}"; do
        [ "$ef" = "$rd_fname" ] && already=1 && break
      done
      [ "$already" -eq 0 ] && flagged+=("$rd_fname")
    fi
  done < "$rd_flagged_files"

  rm -f "$sibling_files" "$rd_flagged_files"

  # PC3: no sibling-touched file flagged
  if [ "${#flagged[@]}" -eq 0 ]; then
    echo "GATE-PASS: PC3 — no sibling-touched file has net-negative delta."
    echo "push-invoked" >> "$push_log"
    return 0
  fi

  # For flagged files: PC1 (intentional) vs PC2 (unverified)
  local all_intentional=1
  for f in "${flagged[@]}"; do
    local log_out
    log_out="$(git -C "$repo" log --oneline "origin/develop..HEAD" -- "$f" 2>/dev/null || true)"
    # TEST SIMULATION proxy for BC-5.44.001 Invariant 4 agent diff-hunk inspection:
    # in production, "intentional" is established by the implementer explicitly reviewing
    # each diff hunk; here keyword presence in the commit message stands in for that step.
    if echo "$log_out" | grep -qiE "intentional|deliberate|confirmed.*(dead|remov)|dead code"; then
      : # this file is PC1
    else
      all_intentional=0
      break
    fi
  done

  if [ "$all_intentional" -eq 1 ]; then
    echo "GATE-PASS: PC1 — all net-negative deltas confirmed as intentional removal."
    echo "push-invoked" >> "$push_log"
    return 0
  fi

  # PC2: unverified net-negative delta in sibling-touched file → STOP
  echo "STOP: Post-rebase diff-integrity gate detected an unverified net-negative line-count"
  echo "delta in a file also modified by a recently-merged sibling story."
  echo ""
  echo "File(s) at risk:"
  for f in "${flagged[@]}"; do
    echo "  $f: net-negative delta"
  done
  echo ""
  echo "UnverifiedNetNegativeDelta"
  echo ""
  echo "Required actions before force-push:"
  echo "  1. Run \`git diff origin/develop -- <filename>\` and inspect the delta manually."
  echo "  2. Confirm each net-negative change is an intentional deletion, not a silent drop."
  echo "  3. If silent drops are found, restore the dropped lines and re-commit."
  echo "  4. Re-run the post-rebase diff-integrity gate after any corrections."
  # push_log NOT written — gate halted
  return 0
}

# ===========================================================================
# T-001 / AC-003 / PC2: gate halts — unverified net-negative delta
# BC-5.44.001 PC2, Invariant 1
# ===========================================================================

@test "T-001 S-21.02 AC-003: gate halts on unverified net-negative delta in sibling-touched file" {
  # Fixture: 12-line file; feature deletes line06; sibling S-20.01 renames line06 →
  # SIBLING06. Real rebase (-X theirs) resolves conflict in feature's favour; post-rebase
  # commit deletes "SIBLING06" instead of "line06" → range-diff shows !. Gate must:
  #   (a) detect delta via range-diff primary + stat backup;
  #   (b) NOT invoke push;
  #   (c) emit UnverifiedNetNegativeDelta.
  # Anti-tautology: doc-parity assertions fail if §Inter-Wave Rebase section is deleted.
  # F-P2-004: step-f parity fails if AC-002 deliverable removes gate reference.

  local section
  section="$(_extract_inter_wave_rebase_section)"

  # DOC-PARITY: §Inter-Wave Rebase gate procedure markers
  _assert_doc_marker "range-diff" "range-diff (primary detector, step 1a)" "$section"
  _assert_doc_marker "\-\-stat" "--stat (backup heuristic, step 1b)" "$section"
  _assert_doc_marker "File\(s\) at risk" "PC2 STOP block: 'File(s) at risk:'" "$section"
  _assert_doc_marker "restore the dropped lines" "PC2 action 3: restore the dropped lines" "$section"
  _assert_doc_marker "PRE_REBASE_TIP" "PRE_REBASE_TIP capture before rebase" "$section"
  _assert_doc_marker "UnverifiedNetNegativeDelta" "Error variant: UnverifiedNetNegativeDelta (PC2 exit token)" "$section"
  _assert_doc_marker "range-diff.*\.\.\." "range-diff invocation with three-dot form (...)" "$section"

  # DOC-PARITY: gate is positioned between rebase and push (Invariant 1 ordering).
  local rebase_line gate_line push_line
  rebase_line="$(echo "$section" | grep -n "git rebase origin/develop" | head -1 | cut -d: -f1)"
  gate_line="$(echo "$section" | grep -n "Post-Rebase Diff-Integrity" | head -1 | cut -d: -f1)"
  push_line="$(echo "$section" | grep -n "force-with-lease" | head -1 | cut -d: -f1)"
  [ -n "$rebase_line" ] && [ -n "$gate_line" ] && [ -n "$push_line" ] || {
    echo "DOC-PARITY FAIL: rebase/gate/push ordering markers absent from §Inter-Wave Rebase"
    false
  }
  [ "$gate_line" -gt "$rebase_line" ] && [ "$gate_line" -lt "$push_line" ] || {
    echo "DOC-PARITY FAIL: gate marker (line $gate_line) not between rebase (line $rebase_line) and push (line $push_line)"
    false
  }

  # DOC-PARITY F-P2-004: step-f-pr-lifecycle.md (AC-002 deliverable) must reference gate
  _assert_step_f_marker "Role ownership" "role-ownership block present (AC-002)"
  _assert_step_f_marker "diff.integrity gate|post-rebase diff-integrity" "gate reference in step-f"
  _assert_step_f_marker "range-diff" "range-diff mentioned in step-f gate reference"

  # HARNESS: set up real-rebase fixture; FIXTURE_PRE_REBASE_TIP set inside _setup_git_fixture
  _setup_git_fixture "net-negative-sibling"

  # HARNESS PRE-CHECK: verify the real rebase produced a range-diff ! commit (primary
  # detector is functionally exercised — pre_tip != post_tip and commit content changed).
  # Three-dot form per spec: PRE_REBASE_TIP...POST_REBASE_TIP (symmetric difference).
  local rd_post rd_check
  rd_post="$(git -C "$FIXTURE_REPO" rev-parse HEAD 2>/dev/null || echo "")"
  rd_check="$(git -C "$FIXTURE_REPO" range-diff \
      "${FIXTURE_PRE_REBASE_TIP}...${rd_post}" 2>/dev/null || true)"
  echo "$rd_check" | grep -qE '[0-9a-f]+ !' || {
    echo "HARNESS PRE-CHECK FAIL: range-diff shows no modified (!) commit — primary detector not exercised"
    echo "range-diff output: $rd_check"
    false
  }

  local gate_out
  gate_out="$(_run_gate "$FIXTURE_REPO" "$FIXTURE_PRE_REBASE_TIP" "$PUSH_LOG")"

  # AC-003(a): gate detects the delta
  echo "$gate_out" | grep -q "STOP" || {
    echo "HARNESS FAIL: gate did not halt — expected STOP, got: $gate_out"
    false
  }
  # AC-003(b): push NOT invoked
  [ ! -s "$PUSH_LOG" ] || {
    echo "HARNESS FAIL: push was invoked but should not be on PC2 — push log: $(cat "$PUSH_LOG")"
    false
  }
  # AC-003(c): UnverifiedNetNegativeDelta in output
  echo "$gate_out" | grep -q "UnverifiedNetNegativeDelta" || {
    echo "HARNESS FAIL: UnverifiedNetNegativeDelta not in gate output — got: $gate_out"
    false
  }
}

# ===========================================================================
# T-002 / AC-004 / PC3: gate passes — no sibling file overlap
# BC-5.44.001 PC3
# ===========================================================================

@test "T-002 S-21.02 AC-004: gate passes — no sibling file overlap (PC3)" {
  # Fixture: feature changes only feature_only.rs; sibling S-20.01 touched only autoload.gd.
  # Gate must be invoked (check for overlap) and then pass (PC3); push is invoked.
  # Anti-tautology: doc-parity assertion fails if PC3 language removed from doc.

  local section
  section="$(_extract_inter_wave_rebase_section)"

  # DOC-PARITY: range-diff + PC3 language must be present
  _assert_doc_marker "range-diff" "range-diff (primary detector)" "$section"
  _assert_doc_marker "PC3" "PC3 postcondition label" "$section"
  _assert_doc_marker "[Nn]o.*(sibling|overlap)|trivially" "PC3 no-overlap pass language" "$section"

  # HARNESS: run gate; assert PC3 pass + push invoked
  # Real rebase (clean, no conflict) — after rebase feature tree includes sibling's
  # autoload.gd additions; git diff --numstat shows 0/0 on autoload.gd → no phantom delta.
  _setup_git_fixture "no-sibling-overlap"

  local gate_out
  gate_out="$(_run_gate "$FIXTURE_REPO" "$FIXTURE_PRE_REBASE_TIP" "$PUSH_LOG")"

  echo "$gate_out" | grep -q "GATE-PASS" || {
    echo "HARNESS FAIL: gate did not pass — expected GATE-PASS, got: $gate_out"
    false
  }
  echo "$gate_out" | grep -q "PC3" || {
    echo "HARNESS FAIL: gate passed but not via PC3 branch — got: $gate_out"
    false
  }
  grep -q "push-invoked" "$PUSH_LOG" || {
    echo "HARNESS FAIL: push was not invoked on PC3 pass — gate output: $gate_out"
    false
  }
}

# ===========================================================================
# T-003 / AC-005 / PC1: gate passes — confirmed intentional removal
# BC-5.44.001 PC1
# ===========================================================================

@test "T-003 S-21.02 AC-005: gate passes — confirmed intentional removal (PC1)" {
  # Fixture: same diff profile as T-001 (net-negative on sibling-touched file) but
  # feature branch commit message explicitly documents the intentional removal → PC1.
  # Anti-tautology: doc-parity assertion fails if PC1 intentional-removal language removed.

  local section
  section="$(_extract_inter_wave_rebase_section)"

  # DOC-PARITY: PC1 intentional-removal language + PC2 STOP must coexist
  _assert_doc_marker "PC1" "PC1 postcondition label" "$section"
  _assert_doc_marker "[Ii]ntentional|[Dd]eliberate|confirmed.*remov" "PC1 intentional-removal language" "$section"
  _assert_doc_marker "STOP" "PC2 STOP signal (both paths must be documented)" "$section"

  # HARNESS: run gate; assert PC1 pass + push invoked
  # Real rebase (same conflict topology as T-001) — range-diff shows ! but commit message
  # contains "intentionally" → PC1 (intentional removal confirmed).
  _setup_git_fixture "intentional-removal"

  local gate_out
  gate_out="$(_run_gate "$FIXTURE_REPO" "$FIXTURE_PRE_REBASE_TIP" "$PUSH_LOG")"

  echo "$gate_out" | grep -q "GATE-PASS" || {
    echo "HARNESS FAIL: gate did not pass — expected GATE-PASS (PC1), got: $gate_out"
    false
  }
  echo "$gate_out" | grep -q "PC1" || {
    echo "HARNESS FAIL: gate passed but not via PC1 branch — got: $gate_out"
    false
  }
  grep -q "push-invoked" "$PUSH_LOG" || {
    echo "HARNESS FAIL: push not invoked on PC1 pass — gate output: $gate_out"
    false
  }
}

# ===========================================================================
# T-004 / BC-5.44.001 PC4 / EC-006: gate passes trivially — no sibling commits
# ===========================================================================

@test "T-004 S-21.02 PC4/EC-006: gate passes trivially — no sibling commits since branch creation" {
  # Fixture: feature branches from develop; no new commits land on develop.
  # merge-base(feature HEAD, origin/develop) == origin/develop tip → PC4.
  # Anti-tautology: doc-parity fails if PC4 language removed from §Inter-Wave Rebase.

  local section
  section="$(_extract_inter_wave_rebase_section)"

  # DOC-PARITY: PC4 trivial-pass language must be present
  _assert_doc_marker "PC4" "PC4 postcondition label" "$section"
  _assert_doc_marker "[Nn]o sibling.*commit.*branch creation|branch creation.*no sibling" \
    "PC4 no-sibling-since-creation language" "$section"

  # HARNESS: run gate; assert PC4 trivial pass + push invoked
  _setup_git_fixture "no-sibling-commits"

  local gate_out
  gate_out="$(_run_gate "$FIXTURE_REPO" "$FIXTURE_PRE_REBASE_TIP" "$PUSH_LOG")"

  echo "$gate_out" | grep -q "PC4" || {
    echo "HARNESS FAIL: expected PC4 trivial pass, got: $gate_out"
    false
  }
  grep -q "push-invoked" "$PUSH_LOG" || {
    echo "HARNESS FAIL: push not invoked on PC4 trivial pass — gate output: $gate_out"
    false
  }
}

# ===========================================================================
# T-005 / EC-004-class / EC-005: detector failure → escalate; push NOT invoked
# ===========================================================================

@test "T-005 S-21.02 EC-005: detector failure and merge-base failure escalate — push never invoked blind" {
  # Sub-case A: range-diff fails; --stat succeeds; gate still detects PC2 (stat fallback).
  # Sub-case B: both detectors fail → ESCALATE; push NOT invoked.
  # Sub-case C: merge-base fails (invalid SHA) → ESCALATE; push NOT invoked. (OBS-1)
  # Sub-case D: rd_ok=1, stat_fail=1 → range-diff primary detects PC2; push NOT invoked. (OBS-2)
  # Anti-tautology: doc-parity fails if escalation language removed from §Inter-Wave Rebase.

  local section
  section="$(_extract_inter_wave_rebase_section)"

  # DOC-PARITY: EC-005 escalation language for both detector failure and merge-base failure
  _assert_doc_marker "[Ee]scalate" "EC-005 escalate-on-failure language" "$section"
  _assert_doc_marker "range-diff" "range-diff command (step 1a)" "$section"
  _assert_doc_marker "\-\-stat" "--stat fallback (step 1b)" "$section"
  # OBS-1 doc-parity: §Inter-Wave Rebase must document merge-base failure escalation
  # (added at commit 6eb4b6a2: `if [ $? -ne 0 ] || [ -z "${MERGE_BASE}" ]; then exit 1`)
  _assert_doc_marker "merge-base.*fail|MERGE_BASE.*empty|empty.*MERGE_BASE" \
    "merge-base failure escalation branch (OBS-1)" "$section"

  # Sub-case A: force range-diff to fail; --stat still runs; PC2 still detected via stat
  _setup_git_fixture "net-negative-sibling"
  # Save repo/tip for sub-case D (same fixture; avoid double-init of identical dirs)
  local nn_repo nn_tip
  nn_repo="$FIXTURE_REPO"
  nn_tip="$FIXTURE_PRE_REBASE_TIP"

  local gate_out_a
  gate_out_a="$(_run_gate "$FIXTURE_REPO" "$FIXTURE_PRE_REBASE_TIP" "$PUSH_LOG" 1 0)"

  echo "$gate_out_a" | grep -q "STOP" || {
    echo "HARNESS FAIL (sub-case A): range-diff-fail fallback — expected STOP via --stat, got: $gate_out_a"
    false
  }
  [ ! -s "$PUSH_LOG" ] || {
    echo "HARNESS FAIL (sub-case A): push invoked despite PC2 halt — push log: $(cat "$PUSH_LOG")"
    false
  }

  # Sub-case B: both detectors fail → ESCALATE; push NOT invoked
  local gate_out_b
  gate_out_b="$(_run_gate "$FIXTURE_REPO" "$FIXTURE_PRE_REBASE_TIP" "$PUSH_LOG" 1 1)"

  echo "$gate_out_b" | grep -q "ESCALATE" || {
    echo "HARNESS FAIL (sub-case B): both-fail — expected ESCALATE, got: $gate_out_b"
    false
  }
  [ ! -s "$PUSH_LOG" ] || {
    echo "HARNESS FAIL (sub-case B): push invoked despite ESCALATE — push log: $(cat "$PUSH_LOG")"
    false
  }

  # Sub-case C: merge-base failure (invalid pre_rebase_tip SHA) → ESCALATE; push NOT invoked.
  # Exercises the devops-engineer.md exit 1 escalation branch (OBS-1):
  # `git merge-base INVALID_SHA origin/develop` exits non-zero → empty output → ESCALATE.
  _setup_git_fixture "no-sibling-commits"

  local gate_out_c
  gate_out_c="$(_run_gate "$FIXTURE_REPO" "INVALID_SHA_DEADBEEF000000000000" "$PUSH_LOG" 0 0)"

  echo "$gate_out_c" | grep -q "ESCALATE" || {
    echo "HARNESS FAIL (sub-case C): merge-base-fail — expected ESCALATE, got: $gate_out_c"
    false
  }
  [ ! -s "$PUSH_LOG" ] || {
    echo "HARNESS FAIL (sub-case C): push invoked after merge-base failure — push log: $(cat "$PUSH_LOG")"
    false
  }

  # Sub-case D: rd_ok=1, force_stat_fail=1 → range-diff primary alone detects PC2 (STOP).
  # Verifies that force_stat_fail truly disables numstat and the rd primary path can halt
  # independently. Reuses the net-negative-sibling repo from sub-case A to avoid
  # double-initialising the same fixture dirs within one bats test.
  # Expected: STOP with UnverifiedNetNegativeDelta; push NOT invoked.
  local gate_out_d
  gate_out_d="$(_run_gate "$nn_repo" "$nn_tip" "$PUSH_LOG" 0 1)"

  echo "$gate_out_d" | grep -q "STOP" || {
    echo "HARNESS FAIL (sub-case D): rd_ok=1+stat_fail — expected STOP via range-diff primary, got: $gate_out_d"
    false
  }
  [ ! -s "$PUSH_LOG" ] || {
    echo "HARNESS FAIL (sub-case D): push invoked despite PC2 halt — push log: $(cat "$PUSH_LOG")"
    false
  }
}
