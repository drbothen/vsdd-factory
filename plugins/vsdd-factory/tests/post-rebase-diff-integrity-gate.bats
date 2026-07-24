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
#   T-001  AC-003 / PC2: net-neg + sibling overlap, unverified → HALT; push NOT invoked; UnverifiedNetNegativeDelta
#   T-002  AC-004 / PC3: no sibling overlap → gate passes; push invoked
#   T-003  AC-005 / PC1: intentional removal in feature history → gate passes; push invoked
#   T-004  BC-5.44.001 PC4 / EC-006: no sibling commits since creation → trivial pass; push invoked
#   T-005  EC-004-class / EC-005: detector failure → escalate; push NOT invoked

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DEVOPS_ENGINEER_MD="$PLUGIN_ROOT/agents/devops-engineer.md"
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

# ===========================================================================
# EXECUTABLE-HARNESS — git fixture setup
# ===========================================================================

# Build a git fixture for the given scenario. Sets FIXTURE_REPO.
# Scenarios:
#   net-negative-sibling  — feature branches before sibling S-20.01, sibling then merges
#                           to develop adding lines; feature has net-negative delta on
#                           autoload.gd vs origin/develop; no intentionality signal → PC2
#   no-sibling-overlap    — feature changes only feature_only.rs; sibling only autoload.gd
#                           → PC3
#   intentional-removal   — same diff profile as net-negative-sibling but commit message
#                           explicitly documents intentional removal → PC1
#   no-sibling-commits    — feature branches from develop; no new commits to develop
#                           → PC4
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
      # 1. develop: init commit with autoload.gd (4 lines)
      git -C "$repo_dir" checkout -q -b develop 2>/dev/null || true
      printf 'line1\nline2\nline3\nline4\n' > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "init: autoload.gd 4 lines"
      git -C "$repo_dir" push -q origin develop

      # 2. Feature branches BEFORE sibling — removes lines 2,3 (no intentionality signal)
      git -C "$repo_dir" checkout -q -b feature/S-21.02
      printf 'line1\nline4\n' > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-21.02): trim autoload.gd (removed lines 2 and 3)"

      # 3. Sibling S-20.01 lands on develop AFTER branch point
      git -C "$repo_dir" checkout -q develop
      printf 'line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\n' > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-20.01): autoload.gd +4 sibling lines"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q feature/S-21.02
      ;;

    no-sibling-overlap)
      # 1. develop: init with autoload.gd + feature_only.rs
      git -C "$repo_dir" checkout -q -b develop 2>/dev/null || true
      printf 'fn original() {}\n' > "$repo_dir/autoload.gd"
      printf 'fn feature_fn() {}\n' > "$repo_dir/feature_only.rs"
      git -C "$repo_dir" add autoload.gd feature_only.rs
      git -C "$repo_dir" commit -q -m "init: initial commit"
      git -C "$repo_dir" push -q origin develop

      # 2. Feature branches — changes only feature_only.rs
      git -C "$repo_dir" checkout -q -b feature/S-21.02
      printf 'fn feature_fn() {}\nfn new_feature() {}\n' > "$repo_dir/feature_only.rs"
      git -C "$repo_dir" add feature_only.rs
      git -C "$repo_dir" commit -q -m "feat(S-21.02): add new_feature to feature_only.rs"

      # 3. Sibling touches only autoload.gd — no overlap with feature
      git -C "$repo_dir" checkout -q develop
      printf 'fn original() {}\nfn sibling_fn() {}\n' > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-20.01): add sibling_fn to autoload.gd"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q feature/S-21.02
      ;;

    intentional-removal)
      # Same topology as net-negative-sibling but commit message documents intentional removal
      git -C "$repo_dir" checkout -q -b develop 2>/dev/null || true
      printf 'line1\nline2\nline3\nline4\n' > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "init: autoload.gd 4 lines"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q -b feature/S-21.02
      printf 'line1\nline4\n' > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-21.02): intentionally remove lines 2-3 from autoload.gd (confirmed dead code per S-21.02 scope)"

      git -C "$repo_dir" checkout -q develop
      printf 'line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\n' > "$repo_dir/autoload.gd"
      git -C "$repo_dir" add autoload.gd
      git -C "$repo_dir" commit -q -m "feat(S-20.01): autoload.gd +4 sibling lines"
      git -C "$repo_dir" push -q origin develop

      git -C "$repo_dir" checkout -q feature/S-21.02
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
      # No new commits on develop → merge-base == origin/develop HEAD → PC4
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
#   $2  pre_rebase_tip   — SHA captured before the rebase (or HEAD for test fixtures)
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
  local sibling_files

  sibling_files="$(mktemp)"

  # Step 1a — Primary detector: git range-diff (git >= 2.19)
  local post_rebase_tip rd_out rd_ok
  post_rebase_tip="$(git -C "$repo" rev-parse HEAD 2>/dev/null || echo "INVALID")"
  rd_ok=0
  rd_out=""
  if [ "$force_rd_fail" -eq 0 ]; then
    rd_out="$(git -C "$repo" range-diff "${pre_rebase_tip}...${post_rebase_tip}" 2>&1)" \
      && rd_ok=1 || rd_ok=0
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
    rm -f "$sibling_files"
    return 0
  fi

  # Compute merge-base and sibling commit set (BC-5.44.001 Invariant 3)
  local merge_base develop_tip
  merge_base="$(git -C "$repo" merge-base "$pre_rebase_tip" origin/develop 2>/dev/null || echo "")"
  develop_tip="$(git -C "$repo" rev-parse origin/develop 2>/dev/null || echo "")"

  # PC4: no sibling commits since branch creation
  if [ -z "$merge_base" ] || [ "$merge_base" = "$develop_tip" ]; then
    echo "GATE-PASS: PC4 — no sibling commits since branch creation."
    echo "push-invoked" >> "$push_log"
    rm -f "$sibling_files"
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

  # Parse numstat: insertions<TAB>deletions<TAB>filename
  # Only flag a file if: (a) net-negative, (b) sibling touched it, AND
  # (c) the feature branch's own commits also touched it — distinguishes
  # "feature deleted lines" from "sibling added lines feature doesn't have yet".
  local flagged=()
  while IFS=$'\t' read -r ins del fname; do
    [ -z "$fname" ] && continue
    # Net-negative: more deletions than insertions
    if [ "${del:-0}" -gt "${ins:-0}" ] 2>/dev/null; then
      if grep -qxF "$fname" "$sibling_files" 2>/dev/null; then
        # Verify the feature branch's own history touches this file
        local feat_log
        feat_log="$(git -C "$repo" log --oneline "origin/develop..HEAD" -- "$fname" 2>/dev/null || true)"
        if [ -n "$feat_log" ]; then
          flagged+=("$fname")
        fi
      fi
    fi
  done < <(git -C "$repo" diff --numstat origin/develop 2>/dev/null)

  rm -f "$sibling_files"

  # PC3: no sibling-touched file has net-negative delta
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
  # Fixture: feature branches before S-20.01, removes lines from autoload.gd (no
  # intentionality signal). S-20.01 then merges to develop, also touching autoload.gd.
  # Gate must: (a) detect delta; (b) NOT invoke push; (c) emit UnverifiedNetNegativeDelta.
  # Anti-tautology: doc-parity assertions fail if §Inter-Wave Rebase section is deleted.

  local section
  section="$(_extract_inter_wave_rebase_section)"

  # DOC-PARITY: gate procedure markers must be present
  _assert_doc_marker "range-diff" "range-diff (primary detector, step 1a)" "$section"
  _assert_doc_marker "\-\-stat" "--stat (backup heuristic, step 1b)" "$section"
  _assert_doc_marker "File\(s\) at risk" "PC2 STOP block: 'File(s) at risk:'" "$section"
  _assert_doc_marker "restore the dropped lines" "PC2 action 3: restore the dropped lines" "$section"
  _assert_doc_marker "PRE_REBASE_TIP" "PRE_REBASE_TIP capture before rebase" "$section"

  # DOC-PARITY: gate is positioned between rebase and push (Invariant 1 ordering).
  # Use "Post-Rebase Diff-Integrity" heading as the gate marker — it unambiguously
  # appears in the subsection that follows the `git rebase origin/develop` command,
  # not in the pre-rebase capture comment that also mentions range-diff.
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

  # HARNESS: run gate against fixture; assert PC2 behavior
  _setup_git_fixture "net-negative-sibling"
  local pre_tip
  pre_tip="$(git -C "$FIXTURE_REPO" rev-parse HEAD)"

  local gate_out
  gate_out="$(_run_gate "$FIXTURE_REPO" "$pre_tip" "$PUSH_LOG")"

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
  _setup_git_fixture "no-sibling-overlap"
  local pre_tip
  pre_tip="$(git -C "$FIXTURE_REPO" rev-parse HEAD)"

  local gate_out
  gate_out="$(_run_gate "$FIXTURE_REPO" "$pre_tip" "$PUSH_LOG")"

  echo "$gate_out" | grep -q "GATE-PASS" || {
    echo "HARNESS FAIL: gate did not pass — expected GATE-PASS, got: $gate_out"
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
  _setup_git_fixture "intentional-removal"
  local pre_tip
  pre_tip="$(git -C "$FIXTURE_REPO" rev-parse HEAD)"

  local gate_out
  gate_out="$(_run_gate "$FIXTURE_REPO" "$pre_tip" "$PUSH_LOG")"

  echo "$gate_out" | grep -q "GATE-PASS" || {
    echo "HARNESS FAIL: gate did not pass — expected GATE-PASS (PC1), got: $gate_out"
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
  local pre_tip
  pre_tip="$(git -C "$FIXTURE_REPO" rev-parse HEAD)"

  local gate_out
  gate_out="$(_run_gate "$FIXTURE_REPO" "$pre_tip" "$PUSH_LOG")"

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

@test "T-005 S-21.02 EC-005: detector failure escalates — push never invoked blind" {
  # Sub-case A: range-diff fails; --stat succeeds; gate still detects PC2 (fallback path).
  # Sub-case B: both detectors fail; gate escalates; push never invoked.
  # Anti-tautology: doc-parity fails if escalation language removed from §Inter-Wave Rebase.

  local section
  section="$(_extract_inter_wave_rebase_section)"

  # DOC-PARITY: EC-005 escalation language must be present in the gate section
  _assert_doc_marker "[Ee]scalate" "EC-005 escalate-on-failure language" "$section"
  _assert_doc_marker "range-diff" "range-diff command (step 1a)" "$section"
  _assert_doc_marker "\-\-stat" "--stat fallback (step 1b)" "$section"

  # Sub-case A: force range-diff to fail; --stat still runs; PC2 still detected
  _setup_git_fixture "net-negative-sibling"
  local pre_tip
  pre_tip="$(git -C "$FIXTURE_REPO" rev-parse HEAD)"

  local gate_out_a
  gate_out_a="$(_run_gate "$FIXTURE_REPO" "$pre_tip" "$PUSH_LOG" 1 0)"

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
  gate_out_b="$(_run_gate "$FIXTURE_REPO" "$pre_tip" "$PUSH_LOG" 1 1)"

  echo "$gate_out_b" | grep -q "ESCALATE" || {
    echo "HARNESS FAIL (sub-case B): both-fail — expected ESCALATE, got: $gate_out_b"
    false
  }
  [ ! -s "$PUSH_LOG" ] || {
    echo "HARNESS FAIL (sub-case B): push invoked despite ESCALATE — push log: $(cat "$PUSH_LOG")"
    false
  }
}
