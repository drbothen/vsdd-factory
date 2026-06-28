#!/usr/bin/env bats
# sprint-state-format.bats — Red Gate tests for S-18.11 sprint-state.yaml per-story format
#
# Story:   S-18.11 v1.1 — sprint-state.yaml producer migration to per-story {id, status} format
# BCs:     BC-5.41.004 v1.0 (producer: stories: key, per-entry schema, wave-ascending order,
#                              completeness, 8-value enum INV-1, no-fabrication INV-2,
#                              no-phantom-wave INV-3, EC-007 UnknownStatusToken)
#          BC-5.41.001 v1.26 (consumer: PC2 wave_id leading-contiguous-terminal-run algorithm;
#                               P-SPRINT-STATE-WAVE-ORDER precondition)
#          BC-5.41.002 v1.20 (consumer: PC3 stories from sprint-state.yaml status:draft/pending;
#                               reserved-pending no-op annotation; BrokenSprintState handling)
#
# RED GATE discipline (BC-5.38.001): ALL non-skipped tests MUST FAIL before the
# sprint-state.yaml migration (T-4) and wave-scheduling SKILL.md step (T-5) exist.
# The tests that run against LEGACY fixtures assert the ABSENCE of the required format.
#
# CI-PORTABILITY DESIGN (cross-tree / CI-portability constraint):
# ---------------------------------------------------------------
# The production sprint-state.yaml lives at .factory/stories/sprint-state.yaml on
# the factory-artifacts orphan branch, mounted as a git worktree at .factory/ in
# local development. In a fresh CI checkout of the feature branch (or develop),
# .factory/ is NOT present — it is a separate orphan-branch worktree that is only
# mounted locally (see CLAUDE.md §Git Workflow factory-artifacts branch).
#
# Pattern used by sibling tests (wave-handoff.bats test_BC_5_41_001_PC10_S18_13_AC002*
# and multiple tests in rehydrate-wave.bats): skip with `skip "<reason>"` when a
# production file is absent, so the test is SKIPPED (not FAILED) in CI.
# Fixture-based tests are CI-portable because they use files committed in
# plugins/vsdd-factory/tests/fixtures/sprint-state-format/ on the feature branch.
#
# Test classification:
#   PRODUCTION-FILE + .factory-GUARDED SKIP:
#     test_sprint_state_stories_list_present         (T-1, AC-001)
#       → SKIP when .factory/stories/sprint-state.yaml absent (CI);
#         FAIL  when .factory/stories/sprint-state.yaml is the legacy format (local).
#   FIXTURE-BASED against LEGACY format (CI-portable, genuine RED):
#     test_sprint_state_stories_wave_order           (T-2, AC-002) — uses fixture-legacy.yaml
#     test_sprint_state_status_matches_story_index   (T-3, AC-003) — uses fixture-legacy.yaml
#     test_wave_handoff_parses_migrated_sprint_state (T-4, AC-004) — uses fixture-legacy.yaml
#     test_wave_id_leading_run_algorithm             (T-5, AC-006) — uses fixture-legacy.yaml
#
# All four fixture-based tests target the LEGACY format (the current production shape),
# asserting that the obligations of BC-5.41.004 are NOT met. After T-4 migrates the
# production file to the conformant format, the implementer replaces fixture-legacy.yaml
# usage with fixture-migrated.yaml (or the actual production file) to turn the tests GREEN.
#
# PORTABILITY RULES (Architecture Compliance Rules §3-§7):
#   §3 set -euo pipefail in all helper scriptlets
#   §4 POSIX character classes: [[:space:]] not \s, [[:digit:]] not \d
#   §5 No local -A (bash 3.2 guard): use plain variables or indexed arrays
#   §6 No global IFS mutation: any IFS assignment is scoped via subshell or local
#   §7 No undeclared python/PyYAML: prefer awk/grep for YAML key extraction;
#      yq is available in CI (see ci.yml line 31/547/548)
#
# TOOL AVAILABILITY in CI (ci.yml verified):
#   bats-core, jq, yq, bash (4+ on Linux; 3.2 on macOS but brew bash 5 in PATH),
#   awk, grep -E, sort, git
#   No jq used in this suite (POSIX awk/grep sufficient for YAML key extraction)
#
# @test count: 5 (grep -c '^@test' sprint-state-format.bats == 5)

# ---------------------------------------------------------------------------
# Fixture paths
# ---------------------------------------------------------------------------

_FIXTURE_DIR=""
_PRODUCTION_SPRINT_STATE=""

setup() {
  _FIXTURE_DIR="${BATS_TEST_DIRNAME}/fixtures/sprint-state-format"
  local repo_root
  repo_root="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  # Production sprint-state.yaml lives on factory-artifacts (orphan branch worktree)
  # mounted at .factory/ — present locally when the worktree is mounted.
  _PRODUCTION_SPRINT_STATE="${repo_root}/.factory/stories/sprint-state.yaml"
}

teardown() {
  :
}

# ---------------------------------------------------------------------------
# Helper: _count_stories_per_story_entries FILE
# Count entries in the `stories:` YAML list that are per-story objects (have `id:`).
# Returns 0 when the stories: key is a count-summary mapping (legacy format).
# Uses awk (POSIX portable; no python/jq).
# Architecture Compliance Rule §4: [[:space:]] not \s.
# ---------------------------------------------------------------------------
_count_stories_per_story_entries() {
  local file="$1"
  awk '
    BEGIN { in_stories=0; count=0 }
    /^[[:space:]]*stories:/ { in_stories=1; next }
    in_stories && /^[^[:space:]#-]/ { in_stories=0 }
    in_stories && /^[[:space:]]*-[[:space:]]+id:/ { count++ }
    END { print count }
  ' "$file"
}

# ---------------------------------------------------------------------------
# Helper: _stories_is_sequence FILE
# Returns 0 (success) if the stories: key's value is a YAML sequence (starts with
# "  - " entries). Returns 1 if it is a mapping (e.g., total:/merged: keys).
# This distinguishes conformant format from legacy count-summary format.
# Uses yq if available, otherwise awk fallback.
# Architecture Compliance Rule §4: POSIX [[:space:]].
# ---------------------------------------------------------------------------
_stories_is_sequence() {
  local file="$1"
  if command -v yq > /dev/null 2>&1; then
    local type_out
    type_out="$(yq e '.stories | type' "$file" 2>/dev/null || true)"
    case "$type_out" in
      *seq*) return 0 ;;
      *) return 1 ;;
    esac
  fi
  # awk fallback: look for the first non-blank sub-line under stories: that starts
  # with a sequence indicator ("  - ") vs a mapping indicator ("  total:", "  merged:")
  awk '
    BEGIN { in_stories=0; result=1 }
    /^[[:space:]]*stories:/ { in_stories=1; next }
    in_stories && /^[[:space:]]*#/ { next }
    in_stories && /^[[:space:]]*-[[:space:]]/ { result=0; exit }
    in_stories && /^[[:space:]]+[[:alnum:]]/ { result=1; exit }
    in_stories && /^[^[:space:]]/ { result=1; exit }
    END { exit result }
  ' "$file"
}

# ---------------------------------------------------------------------------
# Helper: _leading_terminal_run FILE
# Count the leading contiguous run of terminal entries in the stories: list.
# Terminal statuses per BC-5.41.001 PC2: merged, withdrawn, cancelled.
# Returns the count as an integer (printed to stdout).
# Returns 0 when stories: is empty or not a sequence (legacy format).
# ---------------------------------------------------------------------------
_leading_terminal_run() {
  local file="$1"
  awk '
    BEGIN { in_stories=0; run=0; broken=0; pending_status=0 }
    /^[[:space:]]*stories:/ { in_stories=1; next }
    in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
    in_stories && /^[[:space:]]*-[[:space:]]+id:/ { pending_status=1 }
    in_stories && pending_status && /^[[:space:]]+status:/ {
      val=$0
      sub(/^[[:space:]]+status:[[:space:]]*/, "", val)
      gsub(/[[:space:]]*$/, "", val)
      if (!broken) {
        if (val == "merged" || val == "withdrawn" || val == "cancelled") {
          run++
        } else {
          broken=1
        }
      }
      pending_status=0
    }
    END { print run }
  ' "$file"
}

# ---------------------------------------------------------------------------
# test_sprint_state_stories_list_present
# AC-001 / BC-5.41.004 PC1 + PC2 + INV-1
# The production sprint-state.yaml MUST have a top-level `stories:` key whose
# value is a YAML list (not a count-summary mapping). Each entry in the list MUST
# have `id:` and `status:` sub-keys. The current legacy production file has
# `stories:` as a count-summary mapping (total/merged/ready/...) — NOT a list.
#
# RED GATE CONDITION: FAILS because .factory/stories/sprint-state.yaml is the
# legacy count-summary format (no per-story list entries).
#
# PORTABILITY: guarded to SKIP when .factory/stories/sprint-state.yaml is absent.
# This mirrors the pattern in wave-handoff.bats where tests skip when required
# binaries are absent. The guard ensures CI does not false-fail when factory-artifacts
# is not mounted. Locally, where .factory/ is mounted, the test RUNS and asserts RED.
# ---------------------------------------------------------------------------

@test "test_sprint_state_stories_list_present" {
  # Guard: skip when production file is absent (CI without factory-artifacts worktree)
  if [ ! -f "${_PRODUCTION_SPRINT_STATE}" ]; then
    skip ".factory/stories/sprint-state.yaml absent — factory-artifacts worktree not mounted (CI without .factory/ mount)"
  fi

  # Assert 1: stories: key must exist
  grep -q "^stories:" "${_PRODUCTION_SPRINT_STATE}" || {
    echo "FAIL (BC-5.41.004 PC1): production sprint-state.yaml has no top-level 'stories:' key." >&2
    echo "File: ${_PRODUCTION_SPRINT_STATE}" >&2
    false
  }

  # Assert 2: stories: MUST be a YAML sequence (not a count-summary mapping).
  # The legacy format has 'stories: {total: 70, merged: 57, ...}' as a mapping.
  # A conformant file has 'stories: [{id: S-NNN, status: ...}, ...]' as a list.
  _stories_is_sequence "${_PRODUCTION_SPRINT_STATE}" || {
    echo "FAIL (BC-5.41.004 PC1+PC2): stories: key is present but is a count-summary mapping, not a list." >&2
    echo "  Current legacy format: stories: {total:, merged:, ready:, draft:, partial:, withdrawn:, retired:}" >&2
    echo "  Required conformant format: stories: [{id: S-NNN, status: <enum>}, ...]" >&2
    echo "  BC-5.41.004 PC1: stories: MUST be a YAML list of per-story objects." >&2
    echo "  This test REDs until T-4 migrates sprint-state.yaml to the per-story list format." >&2
    false
  }

  # Assert 3: At least one per-story entry must have id: + status:
  local entry_count
  entry_count="$(_count_stories_per_story_entries "${_PRODUCTION_SPRINT_STATE}")"
  [ "${entry_count}" -gt 0 ] || {
    echo "FAIL (BC-5.41.004 PC2): stories: list has no per-story entries with id:." >&2
    echo "  Each entry MUST have id: (canonical story ID) and status: (8-value enum)." >&2
    false
  }

  # Assert 4: All status values must be within the 8-value canonical enum (INV-1 / EC-007)
  local invalid_found=0
  local statuses
  statuses="$(awk '
    BEGIN { in_stories=0 }
    /^[[:space:]]*stories:/ { in_stories=1; next }
    in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
    in_stories && /^[[:space:]]+status:/ {
      val=$0; sub(/^[[:space:]]+status:[[:space:]]*/, "", val); gsub(/[[:space:]]*$/, "", val); print val
    }
  ' "${_PRODUCTION_SPRINT_STATE}")"

  local s
  while IFS= read -r s; do
    [ -z "${s}" ] && continue
    case "${s}" in
      draft|ready|in-progress|partial|blocked|merged|withdrawn|cancelled) ;;
      *)
        echo "FAIL (BC-5.41.004 INV-1/EC-007): status '${s}' not in canonical 8-value enum." >&2
        invalid_found=1
        ;;
    esac
  done <<EOF
${statuses}
EOF
  [ "${invalid_found}" -eq 0 ] || false
}

# ---------------------------------------------------------------------------
# test_sprint_state_stories_wave_order
# AC-002 / BC-5.41.004 PC3 + INV-3; BC-5.41.001 PC2 P-SPRINT-STATE-WAVE-ORDER
# The `stories:` list MUST be ordered wave-ascending. The wave assignment is derived
# from the dependency-graph topo-sort of STORY-INDEX.md `depends_on:` arrays
# (BC-5.41.004 INV-3: no phantom `wave:` field). Within wave N, lexicographic
# ascending tie-break (BC-5.41.004 PC3 EC-003).
#
# Uses FIXTURE: fixture-legacy.yaml (the legacy count-summary format).
# RED GATE: the legacy format has NO per-story entries with wave-ordering.
# The test asserts two conditions that are both violated by the legacy format:
#   (a) stories: must be a sequence (not a mapping)
#   (b) stories: entries must have id: sub-keys (per-story list entries)
# Both assertions FAIL on fixture-legacy.yaml, producing the correct RED gate.
#
# BC-5.41.004 INV-3 is validated via a static check: the fixture and production
# file must NOT contain a `wave:` key in story entries.
#
# PORTABILITY: fixture-based — CI-portable; no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_sprint_state_stories_wave_order" {
  local fixture="${_FIXTURE_DIR}/fixture-legacy.yaml"
  [ -f "${fixture}" ] || {
    echo "FAIL (fixture missing): ${fixture}" >&2; false
  }

  # Assert 1: stories: must be a YAML sequence (list), NOT a count-summary mapping.
  # RED GATE: fixture-legacy.yaml has stories: as a mapping (total:/merged:/ready:/...).
  _stories_is_sequence "${fixture}" || {
    echo "FAIL (BC-5.41.004 PC1): 'stories:' in legacy format is a count-summary mapping, not a list." >&2
    echo "  BC-5.41.004 PC3 wave-ascending order can only be validated on a per-story list." >&2
    echo "  RED GATE: the current production sprint-state.yaml has no per-story list." >&2
    echo "  This test REDs until T-4 migrates sprint-state.yaml to per-story {id, status} format." >&2
    false
  }

  # (Remaining assertions only reached after RED GATE is resolved by T-4)
  # Assert 2: per-story entries must have id: sub-keys
  local entry_count
  entry_count="$(_count_stories_per_story_entries "${fixture}")"
  [ "${entry_count}" -gt 0 ] || {
    echo "FAIL (BC-5.41.004 PC3): no per-story entries with id: found in stories: list." >&2
    echo "  Wave-ascending order cannot be verified without per-story id: entries." >&2
    false
  }

  # Assert 3: Extract story IDs in file order
  local ids_in_order
  ids_in_order="$(awk '
    BEGIN { in_stories=0 }
    /^[[:space:]]*stories:/ { in_stories=1; next }
    in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
    in_stories && /^[[:space:]]*-[[:space:]]+id:[[:space:]]+/ {
      id_val=$0; sub(/^[[:space:]]*-[[:space:]]+id:[[:space:]]+/, "", id_val)
      gsub(/[[:space:]]*$/, "", id_val); print id_val
    }
  ' "${fixture}")"

  # Assert 4: At least one entry must be present after format check
  local count
  count="$(printf '%s\n' "${ids_in_order}" | grep -c '[^[:space:]]' || true)"
  [ "${count}" -gt 0 ] || {
    echo "FAIL (BC-5.41.004 PC3): stories: list has 0 per-story entries after format migration check." >&2
    false
  }

  # Assert 5: BC-5.41.004 INV-3 — no phantom `wave:` key in stories: entries.
  # The wave ordering MUST come from dependency-graph topo-sort, NOT from a wave: field.
  local has_phantom_wave=0
  awk '
    BEGIN { in_stories=0 }
    /^[[:space:]]*stories:/ { in_stories=1; next }
    in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
    in_stories && /^[[:space:]]+wave:/ { exit 1 }
  ' "${fixture}" || has_phantom_wave=1

  [ "${has_phantom_wave}" -eq 0 ] || {
    echo "FAIL (BC-5.41.004 INV-3): per-story entries contain a 'wave:' sub-key in stories: list." >&2
    echo "  BC-5.41.004 INV-3: wave ordering MUST be derived from STORY-INDEX.md depends_on: topo-sort." >&2
    echo "  No phantom 'wave:' field is permitted on individual story entries." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_sprint_state_status_matches_story_index
# AC-003 / BC-5.41.004 PC2 + INV-2 + EC-007
# Each entry's status: in the stories: list MUST match the authoritative status
# from STORY-INDEX.md (no fabricated statuses). Statuses MUST be within the
# 8-value enum; any outside triggers UnknownStatusToken EC-007.
#
# Uses FIXTURE: fixture-legacy.yaml (legacy count-summary format).
# RED GATE: the legacy format has stories: as a count-summary mapping — it has
# NO per-story entries to compare against STORY-INDEX.md. The test asserts that
# per-story entries exist (BC-5.41.004 PC2) and that the round-trip is possible.
# Both fail on the legacy format.
#
# PORTABILITY: fixture-based — CI-portable; no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_sprint_state_status_matches_story_index" {
  local fixture="${_FIXTURE_DIR}/fixture-legacy.yaml"
  local story_index="${_FIXTURE_DIR}/fixture-STORY-INDEX.md"

  [ -f "${fixture}" ] || {
    echo "FAIL (fixture missing): ${fixture}" >&2; false
  }
  [ -f "${story_index}" ] || {
    echo "FAIL (fixture missing): ${story_index}" >&2; false
  }

  # Assert 1: stories: must be a YAML sequence (not a count-summary mapping).
  # RED GATE: fixture-legacy.yaml has stories: as a mapping — no per-story entries.
  _stories_is_sequence "${fixture}" || {
    echo "FAIL (BC-5.41.004 PC2): stories: in legacy format is a count-summary mapping, not a per-story list." >&2
    echo "  Round-trip status verification requires per-story {id, status} entries." >&2
    echo "  BC-5.41.004 INV-2: status MUST be a direct read from STORY-INDEX.md catalog rows." >&2
    echo "  RED GATE: the current sprint-state.yaml has no per-story entries to round-trip." >&2
    echo "  This test REDs until T-4 migrates sprint-state.yaml and T-5 adds the SKILL.md step." >&2
    false
  }

  # (Remaining assertions only reached after RED GATE is resolved)
  # Assert 2: each story id: in the list must have a status: that matches STORY-INDEX
  local story_ids
  story_ids="$(awk '
    BEGIN { in_stories=0 }
    /^[[:space:]]*stories:/ { in_stories=1; next }
    in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
    in_stories && /^[[:space:]]*-[[:space:]]+id:[[:space:]]+/ {
      id_val=$0; sub(/^[[:space:]]*-[[:space:]]+id:[[:space:]]+/, "", id_val)
      gsub(/[[:space:]]*$/, "", id_val); print id_val
    }
  ' "${fixture}")"

  local mismatch=0
  local sid
  while IFS= read -r sid; do
    [ -z "${sid}" ] && continue

    # Get status from sprint-state
    local ss_status
    ss_status="$(awk -v target_id="${sid}" '
      BEGIN { in_stories=0; found_id=0 }
      /^[[:space:]]*stories:/ { in_stories=1; next }
      in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
      in_stories && found_id && /^[[:space:]]+status:/ {
        val=$0; sub(/^[[:space:]]+status:[[:space:]]*/, "", val); gsub(/[[:space:]]*$/, "", val)
        print val; found_id=0
      }
      in_stories && /^[[:space:]]*-[[:space:]]+id:[[:space:]]+/ {
        id_val=$0; sub(/^[[:space:]]*-[[:space:]]+id:[[:space:]]+/, "", id_val)
        gsub(/[[:space:]]*$/, "", id_val)
        if (id_val == target_id) { found_id=1 }
      }
    ' "${fixture}")"

    # Get status from STORY-INDEX
    local idx_status
    idx_status="$(awk -F'|' -v story="${sid}" '
      /\| Story ID / {
        for (i=1; i<=NF; i++) {
          col=$i; gsub(/^[[:space:]]+|[[:space:]]+$/, "", col)
          if (col == "Status") { status_col=i }
        }
        next
      }
      status_col > 0 {
        sid_val=$2; gsub(/^[[:space:]]+|[[:space:]]+$/, "", sid_val)
        if (sid_val == story) {
          val=$status_col; gsub(/^[[:space:]]+|[[:space:]]+$/, "", val); print val; exit
        }
      }
    ' "${story_index}")"

    [ -n "${ss_status}" ] || { echo "FAIL (BC-5.41.004 PC2): '${sid}' has no status: in sprint-state." >&2; mismatch=1; continue; }
    [ -n "${idx_status}" ] || { echo "FAIL (BC-5.41.004 PC2): '${sid}' not in STORY-INDEX fixture." >&2; mismatch=1; continue; }

    [ "${ss_status}" = "${idx_status}" ] || {
      echo "FAIL (BC-5.41.004 INV-2): status mismatch for '${sid}': sprint-state='${ss_status}' index='${idx_status}'." >&2
      mismatch=1
    }

    # EC-007: status must be in canonical 8-value enum
    case "${ss_status}" in
      draft|ready|in-progress|partial|blocked|merged|withdrawn|cancelled) ;;
      *)
        echo "FAIL (BC-5.41.004 EC-007 UnknownStatusToken): '${sid}' has status '${ss_status}'." >&2
        echo "  Canonical enum: draft,ready,in-progress,partial,blocked,merged,withdrawn,cancelled" >&2
        mismatch=1 ;;
    esac
  done <<EOF
${story_ids}
EOF

  [ "${mismatch}" -eq 0 ] || false
}

# ---------------------------------------------------------------------------
# test_wave_handoff_parses_migrated_sprint_state
# AC-004 / BC-5.41.002 PC3 v1.20; BC-5.41.001 PC2 + PC3
# After migration, invoking the wave-handoff skill against the migrated format MUST:
# - Produce next_wave_stories containing only `status: draft` entries
#   (BC-5.41.002 PC3 v1.20: reserved-pending is a no-op; only draft matches)
# - Exclude terminal entries (merged/withdrawn/cancelled)
# - NOT raise BrokenSprintState on a well-formed fixture
#
# Uses FIXTURE: fixture-legacy.yaml (legacy format — RED GATE)
# RED GATE: wave-handoff.sh invoked against the LEGACY format MUST fail or produce
# incorrect output. The legacy stories: mapping (total/merged/...) has no per-story
# status entries, so the skill either:
#   (a) raises BrokenSprintState (no draft entries found), or
#   (b) exits with an error parsing the non-list format.
# Either outcome is a correct RED gate for this test.
#
# FINDING documented for implementer (T-8): if wave-handoff.sh cannot accept a
# --sprint-state path pointing to the legacy format and errors out on parse,
# the test will RED for the right reason (format mismatch / BrokenSprintState).
# After T-4, the test must be updated to point to fixture-migrated.yaml.
#
# PORTABILITY: uses hermetic temp git repo (mirrors wave-handoff.bats pattern);
# no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_wave_handoff_parses_migrated_sprint_state" {
  local fixture="${_FIXTURE_DIR}/fixture-legacy.yaml"
  [ -f "${fixture}" ] || {
    echo "FAIL (fixture missing): ${fixture}" >&2; false
  }

  # Locate wave-handoff.sh skill
  local skill="${BATS_TEST_DIRNAME}/../skills/wave-handoff/wave-handoff.sh"
  [ -f "${skill}" ] || {
    echo "FAIL (S-18.01 prerequisite): wave-handoff.sh does not exist at ${skill}." >&2
    false
  }

  # Create a hermetic test environment (mirrors wave-handoff.bats setup)
  local work
  work="$(mktemp -d)"
  local artifacts_wt="${work}/factory-wt"

  git -C "${work}" init -q -b feature-test 2>/dev/null || git -C "${work}" init -q
  git -C "${work}" config user.email "test@example.com"
  git -C "${work}" config user.name "Test"
  printf 'root\n' > "${work}/root.txt"
  git -C "${work}" add root.txt
  git -C "${work}" commit -q -m "root"
  local develop_sha
  develop_sha="$(git -C "${work}" rev-parse HEAD)"
  git -C "${work}" update-ref refs/remotes/origin/develop "${develop_sha}"

  local saved_branch
  saved_branch="$(git -C "${work}" branch --show-current)"
  git -C "${work}" checkout --orphan factory-artifacts -q
  git -C "${work}" rm -rf . -q 2>/dev/null || true
  printf 'factory-artifacts init\n' > "${work}/.gitkeep"
  git -C "${work}" add .gitkeep
  git -C "${work}" commit -q -m "factory-artifacts init"
  git -C "${work}" checkout -q "${saved_branch}"
  mkdir -p "${artifacts_wt}"
  git -C "${work}" worktree add -q "${artifacts_wt}" factory-artifacts

  mkdir -p "${artifacts_wt}/hooks"
  mkdir -p "${artifacts_wt}/specs/behavioral-contracts/ss-05"
  mkdir -p "${artifacts_wt}/specs/architecture/decisions"
  mkdir -p "${artifacts_wt}/stories"

  printf '# BC-5.41.001 stub\n' \
    > "${artifacts_wt}/specs/behavioral-contracts/ss-05/BC-5.41.001.md"
  printf '# ARCH-INDEX\n' > "${artifacts_wt}/specs/architecture/ARCH-INDEX.md"
  printf '# ADR-026\n' \
    > "${artifacts_wt}/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
  printf '# ADR-025\n' \
    > "${artifacts_wt}/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md"

  # Minimal STORY-INDEX — these IDs won't be found in the legacy sprint-state
  cat > "${artifacts_wt}/stories/STORY-INDEX.md" << 'SIDX'
---
document_type: story-index
version: "1.0"
---
# STORY-INDEX

## Epic E-1

| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-1.01 | Root story | E-1 | 2 | P0 | [] | [S-1.02] | merged | [] |
| S-1.02 | Second story | E-1 | 3 | P1 | [S-1.01] | [] | draft | [] |
SIDX

  cat > "${work}/STATE.md" << 'STATEMD'
---
current_step: "pass-2"
current_cycle: "v1.0-test-fixture"
factory_lock: null
---
# STATE
STATEMD

  # Use the LEGACY sprint-state fixture — RED GATE target
  cp "${fixture}" "${work}/sprint-state.yaml"

  # Attempt --emit-handoff against the LEGACY format.
  # RED GATE: the legacy stories: mapping (total/merged/...) has no per-story status
  # entries. The skill should raise BrokenSprintState (no draft entries found in a
  # stories: sequence) OR fail to parse the non-list format.
  # Either exit != 0 or next_wave_stories absent/wrong is the correct RED outcome.
  local emit_output
  local emit_exit=0
  emit_output="$(
    "${skill}" \
      --artifacts-worktree "${artifacts_wt}" \
      --sprint-state "${work}/sprint-state.yaml" \
      --state-md "${work}/STATE.md" \
      --bc-dir "${artifacts_wt}/specs/behavioral-contracts" \
      --emit-handoff \
      2>&1
  )" || emit_exit=$?

  git -C "${work}" worktree remove --force "${artifacts_wt}" 2>/dev/null || true
  rm -rf "${work}"

  # Assert RED GATE: the skill MUST either exit non-zero (BrokenSprintState or parse error)
  # OR produce next_wave_stories with wrong content from the legacy format.
  # After T-4 migration, this test is updated to use fixture-migrated.yaml and assert
  # exit 0 + correct next_wave_stories.
  if [ "${emit_exit}" -eq 0 ]; then
    # Skill exited 0 — check next_wave_stories is absent or wrong from legacy format
    # The legacy format has no per-story draft entries, so next_wave_stories must not
    # correctly list per-story objects with id: + status: fields.
    local nws_content
    nws_content="$(printf '%s\n' "${emit_output}" | awk '
      /^next_wave_stories:/ { in_nws=1; next }
      in_nws && /^[^[:space:]]/ { in_nws=0 }
      in_nws { print }
    ')"
    # If there are no draft entries in the legacy format's stories: mapping, the skill
    # would emit next_wave_stories: [] — which would be BrokenSprintState (non-terminal
    # in-progress epics in the legacy file). We assert this is NOT a valid outcome.
    printf '%s\n' "${emit_output}" | grep -q "BrokenSprintState\|next_wave_stories: \[\]" || {
      # Skill exited 0 and did NOT raise BrokenSprintState and has non-empty next_wave_stories
      # This means it somehow derived next_wave_stories from the legacy format — wrong.
      echo "FAIL (BC-5.41.002 PC3): skill exited 0 with non-empty next_wave_stories from LEGACY format." >&2
      echo "  The legacy count-summary format has no per-story {id, status} entries." >&2
      echo "  Skill MUST raise BrokenSprintState or error on a non-list stories: mapping." >&2
      echo "  RED GATE: this test REDs until T-4 migration replaces the legacy format." >&2
      echo "  Output: ${emit_output}" >&2
      false
    }
    # Skill raised BrokenSprintState on legacy format — this is acceptable RED for now
    # but we still explicitly fail to enforce the RED gate
    echo "FAIL (BC-5.41.002 PC3 RED GATE): skill produced BrokenSprintState on legacy format." >&2
    echo "  This confirms the legacy sprint-state.yaml cannot satisfy BC-5.41.002 PC3." >&2
    echo "  After T-4, the migrated format must NOT raise BrokenSprintState." >&2
    echo "  Update this test to use fixture-migrated.yaml and assert exit 0 + S-1.02 in next_wave_stories." >&2
    false
  else
    # Skill exited non-zero — confirm it's for the RIGHT reason (format mismatch / BrokenSprintState)
    # not a build error or missing argument
    printf '%s\n' "${emit_output}" | grep -qiE "(BrokenSprintState|stories|format|list|seq|parse|sprint.state)" || {
      echo "FAIL (test malformation): skill exited ${emit_exit} for an UNEXPECTED reason." >&2
      echo "  Expected exit due to legacy format incompatibility (BrokenSprintState / parse error)." >&2
      echo "  Output: ${emit_output}" >&2
      false
    }
    # Expected RED: skill cannot handle legacy format — correct Red Gate
    echo "RED (expected): skill exited ${emit_exit} on legacy format — correct Red Gate condition." >&2
    echo "  The legacy count-summary format cannot satisfy BC-5.41.002 PC3 per-story derivation." >&2
    echo "  After T-4, update this test to use fixture-migrated.yaml and assert exit 0." >&2
    false
  fi
}

# ---------------------------------------------------------------------------
# test_wave_id_leading_run_algorithm
# AC-006 / BC-5.41.001 PC2; BC-5.41.004 PC3 + INV-3
# The leading-contiguous-terminal-run algorithm: scan stories: entries in file order;
# count the leading contiguous run of terminal entries (merged/withdrawn/cancelled);
# wave_id = run_length + 1.
#
# Uses FIXTURE: fixture-legacy.yaml (legacy count-summary format — RED GATE target).
# A known-conformant fixture (fixture-leading-run.yaml with 10 terminals → wave_id=11)
# is also used to verify the algorithm itself works correctly once format is conformant.
#
# RED GATE: the legacy format has stories: as a count-summary mapping — NOT a per-story
# sequence. The _leading_terminal_run helper returns 0 (no terminal sequence entries
# because stories: is not a sequence). wave_id would be 1 (0+1), but the test asserts
# we CANNOT validly derive wave_id=11 from the legacy format.
#
# The test first asserts the legacy fixture fails the sequence check (RED gate),
# then validates the algorithm on fixture-leading-run.yaml to confirm the algorithm
# is correct (self-check). The algorithm self-check is NOT a vacuous pass — it asserts
# a concrete expected value (11) that only holds if the fixture is correctly formed.
#
# PORTABILITY: fixture-based — CI-portable; no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_wave_id_leading_run_algorithm" {
  local legacy_fixture="${_FIXTURE_DIR}/fixture-legacy.yaml"
  local run_fixture="${_FIXTURE_DIR}/fixture-leading-run.yaml"

  [ -f "${legacy_fixture}" ] || {
    echo "FAIL (fixture missing): ${legacy_fixture}" >&2; false
  }
  [ -f "${run_fixture}" ] || {
    echo "FAIL (fixture missing): ${run_fixture}" >&2; false
  }

  # Assert 1 (RED GATE): legacy format has stories: as a count-summary mapping.
  # The leading-contiguous-terminal-run algorithm CANNOT operate on this format
  # because stories: is not a sequence of per-story objects.
  _stories_is_sequence "${legacy_fixture}" || {
    echo "FAIL (BC-5.41.001 PC2 RED GATE): legacy sprint-state.yaml stories: is a mapping, not a sequence." >&2
    echo "  The leading-contiguous-terminal-run algorithm requires a per-story {id, status} list." >&2
    echo "  P-SPRINT-STATE-WAVE-ORDER precondition: entries must be in wave-ascending order." >&2
    echo "  BC-5.41.004 PC3: producer MUST emit per-story entries in wave-ascending order." >&2
    echo "  RED GATE: wave_id cannot be correctly derived from the legacy format." >&2
    echo "  This test REDs until T-4 migrates sprint-state.yaml to the per-story list format." >&2
    false
  }

  # (Remaining assertions only reached after RED GATE is resolved)
  # Assert 2: on the legacy fixture, _leading_terminal_run returns 0 (no sequence entries)
  # so wave_id = 0 + 1 = 1, which is WRONG for a >1-wave project.
  local legacy_run
  legacy_run="$(_leading_terminal_run "${legacy_fixture}")"
  # This is a diagnostic assertion — the legacy run must be 0 (no per-story entries)
  [ "${legacy_run}" -eq 0 ] || {
    echo "FAIL (BC-5.41.004 PC3): legacy fixture stories: mapping has terminal entries — unexpected." >&2
    echo "  Legacy format should have 0 per-story sequence entries." >&2
    false
  }

  # Assert 3 (algorithm self-check on conformant fixture):
  # fixture-leading-run.yaml has 10 terminals + 2 drafts → run=10 → wave_id=11.
  # This verifies the algorithm implementation is correct for the GREEN path.
  local run_length
  run_length="$(_leading_terminal_run "${run_fixture}")"
  [ "${run_length}" -eq 10 ] || {
    echo "FAIL (BC-5.41.001 PC2 algorithm): leading-contiguous-terminal-run on fixture = ${run_length}, expected 10." >&2
    echo "  fixture-leading-run.yaml has 10 terminal entries (merged/withdrawn/cancelled) then 2 drafts." >&2
    false
  }

  local wave_id=$(( run_length + 1 ))
  [ "${wave_id}" -eq 11 ] || {
    echo "FAIL (BC-5.41.001 PC2): wave_id = ${wave_id}, expected 11." >&2
    echo "  wave_id = leading-run + 1 = 10 + 1 = 11." >&2
    false
  }

  # Assert 4: P-SPRINT-STATE-WAVE-ORDER — the conformant fixture must NOT have a
  # terminal entry appearing after a non-terminal entry (wave-ascending invariant)
  local seen_non_terminal=0
  local order_ok=1
  while IFS= read -r status_val; do
    [ -z "${status_val}" ] && continue
    case "${status_val}" in
      merged|withdrawn|cancelled)
        [ "${seen_non_terminal}" -eq 0 ] || { order_ok=0; break; }
        ;;
      draft|ready|in-progress|partial|blocked)
        seen_non_terminal=1
        ;;
    esac
  done <<EOF
$(awk '
  BEGIN { in_stories=0 }
  /^[[:space:]]*stories:/ { in_stories=1; next }
  in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
  in_stories && /^[[:space:]]+status:/ {
    val=$0; sub(/^[[:space:]]+status:[[:space:]]*/, "", val); gsub(/[[:space:]]*$/, "", val); print val
  }
' "${run_fixture}")
EOF
  [ "${order_ok}" -eq 1 ] || {
    echo "FAIL (BC-5.41.001 P-SPRINT-STATE-WAVE-ORDER): fixture-leading-run.yaml has a terminal entry after a non-terminal entry." >&2
    echo "  Wave-ascending order requires all terminal entries to precede non-terminal entries." >&2
    false
  }
}
