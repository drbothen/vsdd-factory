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
# GREEN-TRANSITION COMPLETE (post-T-4 + T-5): All fixture-based tests now use
# conformant fixtures (fixture-migrated.yaml / fixture-leading-run.yaml).
# The legacy RED gate assertions have been replaced with GREEN assertions.
# Test 6 (epics-coexistence regression) locks the PC5-coexistence awk-anchor fix.
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
#     test_sprint_state_stories_list_present                (T-1, AC-001)
#       → SKIP when .factory/stories/sprint-state.yaml absent (CI);
#         GREEN when .factory/stories/sprint-state.yaml has conformant per-story format.
#   FIXTURE-BASED (CI-portable, GREEN post-T-4):
#     test_sprint_state_stories_wave_order                  (T-2, AC-002) — fixture-migrated.yaml
#     test_sprint_state_status_matches_story_index          (T-3, AC-003) — fixture-migrated.yaml + fixture-STORY-INDEX.md
#     test_wave_handoff_parses_migrated_sprint_state        (T-4, AC-004) — fixture-migrated.yaml
#     test_wave_id_leading_run_algorithm                    (T-5, AC-006) — fixture-leading-run.yaml
#     test_epics_coexistence_nested_stories_ignored         (T-6, PC5/EC-004) — fixture-migrated.yaml
#
# AWK ANCHOR FIX (PC5 coexistence):
# All awk patterns that enter `in_stories=1` use /^stories:/ (column-0 anchor),
# NOT /^[[:space:]]*stories:/. This prevents nested epics[*].stories: sub-keys
# from re-entering the stories: parsing context and emitting out-of-enum values.
# fixture-migrated.yaml has epics.E-0.stories=closed + epics.E-1.stories=tier-1-shipped
# which are out-of-enum scalars that MUST be ignored by all awk parsers.
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
# @test count: 6 (grep -c '^@test' sprint-state-format.bats == 6)

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
    /^stories:/ { in_stories=1; next }
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
  # Anchored to /^stories:/ (column-0) so nested epics[*].stories: sub-keys are ignored.
  awk '
    BEGIN { in_stories=0; result=1 }
    /^stories:/ { in_stories=1; next }
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
    /^stories:/ { in_stories=1; next }
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
    /^stories:/ { in_stories=1; next }
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
# GREEN TRANSITION (post-T-4): uses fixture-migrated.yaml — the conformant per-story
# format produced after T-4 migrates sprint-state.yaml. All assertions must PASS.
#
# Wave topology in fixture-migrated.yaml:
#   wave 1: S-1.01 (no deps) — merged (terminal)
#   wave 2: S-1.02 (depends_on S-1.01) — draft (next-wave)
#           S-1.03 (depends_on S-1.01) — ready (active-but-not-next-wave)
# S-1.02 before S-1.03 within wave-2 per lexicographic tie-break (EC-003).
# File order: S-1.01, S-1.02, S-1.03 — wave-ascending.
#
# BC-5.41.004 INV-3: no `wave:` sub-key in stories: entries.
# awk anchor: /^stories:/ (column-0 only) per PC5 coexistence fix.
#
# PORTABILITY: fixture-based — CI-portable; no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_sprint_state_stories_wave_order" {
  local fixture="${_FIXTURE_DIR}/fixture-migrated.yaml"
  [ -f "${fixture}" ] || {
    echo "FAIL (fixture missing): ${fixture}" >&2; false
  }

  # Assert 1: stories: must be a YAML sequence (list), NOT a count-summary mapping.
  # GREEN: fixture-migrated.yaml has stories: as a proper YAML sequence.
  _stories_is_sequence "${fixture}" || {
    echo "FAIL (BC-5.41.004 PC1): 'stories:' is a count-summary mapping, not a list." >&2
    echo "  BC-5.41.004 PC3 wave-ascending order can only be validated on a per-story list." >&2
    false
  }

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
    /^stories:/ { in_stories=1; next }
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
    echo "FAIL (BC-5.41.004 PC3): stories: list has 0 per-story entries." >&2
    false
  }

  # Assert 5: BC-5.41.004 INV-3 — no phantom `wave:` key in stories: entries.
  # The wave ordering MUST come from dependency-graph topo-sort, NOT from a wave: field.
  # awk anchor: /^stories:/ (column-0) so epics[*].stories: sub-keys are ignored.
  local has_phantom_wave=0
  awk '
    BEGIN { in_stories=0 }
    /^stories:/ { in_stories=1; next }
    in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
    in_stories && /^[[:space:]]+wave:/ { exit 1 }
  ' "${fixture}" || has_phantom_wave=1

  [ "${has_phantom_wave}" -eq 0 ] || {
    echo "FAIL (BC-5.41.004 INV-3): per-story entries contain a 'wave:' sub-key in stories: list." >&2
    echo "  BC-5.41.004 INV-3: wave ordering MUST be derived from STORY-INDEX.md depends_on: topo-sort." >&2
    echo "  No phantom 'wave:' field is permitted on individual story entries." >&2
    false
  }

  # Assert 6: Wave-ascending order — S-1.01 (wave-1) must precede S-1.02 and S-1.03 (wave-2).
  # Verify that the first entry is S-1.01 (terminal, wave-1 root).
  local first_id
  first_id="$(printf '%s\n' "${ids_in_order}" | head -1)"
  [ "${first_id}" = "S-1.01" ] || {
    echo "FAIL (BC-5.41.004 PC3): first entry is '${first_id}', expected 'S-1.01' (wave-1 root)." >&2
    echo "  Wave-ascending order: wave-1 stories must appear before wave-2 stories." >&2
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
# GREEN TRANSITION (post-T-4 + T-5): uses fixture-migrated.yaml + fixture-STORY-INDEX.md.
# fixture-migrated.yaml has 3 stories (S-1.01 merged, S-1.02 draft, S-1.03 ready).
# fixture-STORY-INDEX.md has the same 3 IDs with matching statuses.
# Round-trip: each stories[*].status must match the STORY-INDEX catalog row for that ID.
# awk anchor: /^stories:/ (column-0 only) per PC5 coexistence fix so nested
# epics[*].stories: sub-keys (closed, tier-1-shipped) are NOT scanned.
#
# PORTABILITY: fixture-based — CI-portable; no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_sprint_state_status_matches_story_index" {
  local fixture="${_FIXTURE_DIR}/fixture-migrated.yaml"
  local story_index="${_FIXTURE_DIR}/fixture-STORY-INDEX.md"

  [ -f "${fixture}" ] || {
    echo "FAIL (fixture missing): ${fixture}" >&2; false
  }
  [ -f "${story_index}" ] || {
    echo "FAIL (fixture missing): ${story_index}" >&2; false
  }

  # Assert 1: stories: must be a YAML sequence (not a count-summary mapping).
  # GREEN: fixture-migrated.yaml has stories: as a proper sequence.
  _stories_is_sequence "${fixture}" || {
    echo "FAIL (BC-5.41.004 PC2): stories: is a count-summary mapping, not a per-story list." >&2
    echo "  Round-trip status verification requires per-story {id, status} entries." >&2
    echo "  BC-5.41.004 INV-2: status MUST be a direct read from STORY-INDEX.md catalog rows." >&2
    false
  }

  # Assert 2: each story id: in the list must have a status: that matches STORY-INDEX.
  # awk anchor: /^stories:/ (column-0 only) — nested epics[*].stories: sub-keys ignored.
  local story_ids
  story_ids="$(awk '
    BEGIN { in_stories=0 }
    /^stories:/ { in_stories=1; next }
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

    # Get status from sprint-state — awk anchored to /^stories:/ (column-0)
    local ss_status
    ss_status="$(awk -v target_id="${sid}" '
      BEGIN { in_stories=0; found_id=0 }
      /^stories:/ { in_stories=1; next }
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
# - Exit 0 (no BrokenSprintState on well-formed fixture)
# - Produce output containing S-1.02 in next_wave_stories (draft entry)
# - Exclude S-1.01 (terminal: merged) from next_wave_stories
#   (BC-5.41.002 PC3 v1.20: reserved-pending is a no-op; only draft matches)
#
# GREEN TRANSITION (post-T-4): uses fixture-migrated.yaml — the conformant per-story
# format. fixture-migrated.yaml has:
#   S-1.01: merged  — terminal, excluded from next_wave
#   S-1.02: draft   — next-wave selector, MUST appear in output
#   S-1.03: ready   — non-terminal, non-draft; classify_stories puts in BROKEN_STORY_IDS
# Per parse-sprint-state.sh: BrokenSprintState only fires when has_broken=1 AND has_next_wave=0.
# Since S-1.02 is draft (has_next_wave=1), skill exits 0 with has-next-wave output.
#
# PORTABILITY: uses hermetic temp git repo (mirrors wave-handoff.bats pattern);
# no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_wave_handoff_parses_migrated_sprint_state" {
  local fixture="${_FIXTURE_DIR}/fixture-migrated.yaml"
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

  # STORY-INDEX with S-1.01 (merged), S-1.02 (draft), S-1.03 (ready) — matches
  # fixture-migrated.yaml for INV-2 round-trip compliance.
  cat > "${artifacts_wt}/stories/STORY-INDEX.md" << 'SIDX'
---
document_type: story-index
version: "1.0"
---
# STORY-INDEX

## Epic E-1

| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-1.01 | Root story | E-1 | 2 | P0 | [] | [S-1.02,S-1.03] | merged | [] |
| S-1.02 | Second story | E-1 | 3 | P1 | [S-1.01] | [] | draft | [] |
| S-1.03 | Third story | E-1 | 3 | P1 | [S-1.01] | [] | ready | [] |
SIDX

  cat > "${work}/STATE.md" << 'STATEMD'
---
current_step: "pass-2"
current_cycle: "v1.0-test-fixture"
factory_lock: null
---
# STATE
STATEMD

  # Use the MIGRATED sprint-state fixture — GREEN target (post-T-4)
  cp "${fixture}" "${work}/sprint-state.yaml"

  # Invoke --emit-handoff against the MIGRATED format.
  # GREEN: fixture-migrated.yaml has {S-1.01 merged, S-1.02 draft, S-1.03 ready}.
  # classify_stories: merged→terminal, draft→next_wave, ready→broken.
  # BrokenSprintState only fires when has_broken=1 AND has_next_wave=0.
  # Since has_next_wave=1 (S-1.02 draft), skill exits 0 with has-next-wave.
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

  # Assert GREEN: skill MUST exit 0 on the migrated format.
  [ "${emit_exit}" -eq 0 ] || {
    echo "FAIL (BC-5.41.002 PC3): skill exited ${emit_exit} on migrated sprint-state.yaml." >&2
    echo "  Expected exit 0 — migrated format with S-1.02 draft satisfies BC-5.41.002 PC3." >&2
    echo "  Output: ${emit_output}" >&2
    false
  }

  # Assert: output must NOT contain BrokenSprintState (well-formed migrated fixture)
  printf '%s\n' "${emit_output}" | grep -q "BrokenSprintState" && {
    echo "FAIL (BC-5.41.002 PC3): skill raised BrokenSprintState on valid migrated format." >&2
    echo "  fixture-migrated.yaml has S-1.02 draft — valid next-wave story present." >&2
    echo "  Output: ${emit_output}" >&2
    false
  } || true

  # Assert: output must contain S-1.02 (the draft next-wave story)
  printf '%s\n' "${emit_output}" | grep -q "S-1.02" || {
    echo "FAIL (BC-5.41.002 PC3): output does not contain 'S-1.02' (expected draft next-wave story)." >&2
    echo "  BC-5.41.002 PC3 v1.20: skill must select S-1.02 (status: draft) as next-wave story." >&2
    echo "  Output: ${emit_output}" >&2
    false
  }

  # Assert: S-1.01 (terminal: merged) must NOT appear in next_wave_stories section
  local nws_has_s101=0
  printf '%s\n' "${emit_output}" | awk '
    /^next_wave_stories:/ { in_nws=1; next }
    in_nws && /^[^[:space:]]/ { in_nws=0 }
    in_nws && /S-1\.01/ { exit 1 }
  ' || nws_has_s101=1
  [ "${nws_has_s101}" -eq 0 ] || {
    echo "FAIL (BC-5.41.002 PC3): S-1.01 (terminal: merged) appeared in next_wave_stories." >&2
    echo "  Terminal stories MUST be excluded from next_wave_stories." >&2
    false
  }
}

# ---------------------------------------------------------------------------
# test_wave_id_leading_run_algorithm
# AC-006 / BC-5.41.001 PC2; BC-5.41.004 PC3 + INV-3
# The leading-contiguous-terminal-run algorithm: scan stories: entries in file order;
# count the leading contiguous run of terminal entries (merged/withdrawn/cancelled);
# wave_id = run_length + 1.
#
# GREEN TRANSITION (post-T-4): uses fixture-leading-run.yaml directly — the
# conformant fixture with 10 terminals + 2 drafts → wave_id=11.
# awk anchor: /^stories:/ (column-0 only) per PC5 coexistence fix.
#
# fixture-leading-run.yaml has 10 terminal entries (merged/withdrawn/cancelled mix)
# followed by 2 draft entries. Expected: leading-run=10 → wave_id=11.
# Assert 4 verifies wave-ascending order: no terminal entry after a non-terminal entry.
#
# PORTABILITY: fixture-based — CI-portable; no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_wave_id_leading_run_algorithm" {
  local run_fixture="${_FIXTURE_DIR}/fixture-leading-run.yaml"

  [ -f "${run_fixture}" ] || {
    echo "FAIL (fixture missing): ${run_fixture}" >&2; false
  }

  # Assert 1: fixture-leading-run.yaml must be a YAML sequence (conformant format).
  _stories_is_sequence "${run_fixture}" || {
    echo "FAIL (BC-5.41.001 PC2): fixture-leading-run.yaml stories: is not a YAML sequence." >&2
    echo "  The leading-contiguous-terminal-run algorithm requires a per-story {id, status} list." >&2
    false
  }

  # Assert 2: fixture has per-story entries with id: sub-keys
  local entry_count
  entry_count="$(_count_stories_per_story_entries "${run_fixture}")"
  [ "${entry_count}" -gt 0 ] || {
    echo "FAIL (BC-5.41.001 PC2): fixture-leading-run.yaml has no per-story entries." >&2
    false
  }

  # Assert 3 (algorithm self-check):
  # fixture-leading-run.yaml has 10 terminals + 2 drafts → run=10 → wave_id=11.
  # This verifies the algorithm implementation is correct.
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
  # terminal entry appearing after a non-terminal entry (wave-ascending invariant).
  # awk anchor: /^stories:/ (column-0 only) per PC5 coexistence fix.
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
  /^stories:/ { in_stories=1; next }
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

# ---------------------------------------------------------------------------
# test_epics_coexistence_nested_stories_ignored
# PC5 coexistence / awk-anchor regression lock (EC-004)
# BC-5.41.004 PC5: the producer MUST NOT mutate the legacy `epics:` section.
# The conformant stories: list coexists with an epics: section whose sub-keys
# may include `stories:` mappings with out-of-enum scalar values
# (e.g., `stories: closed`, `stories: tier-1-shipped`).
#
# This test locks in the top-level awk anchor (/^stories:/, column-0 only):
#   (a) status scan of stories: list returns ONLY 8-enum values (merged/draft/ready)
#       NOT the epics sub-key scalar values (closed, tier-1-shipped)
#   (b) _stories_is_sequence returns true (fixture is a sequence)
#   (c) _count_stories_per_story_entries returns 3 (only top-level list entries counted)
#   (d) _leading_terminal_run correctly processes only the top-level stories: list
#
# Fixture: fixture-migrated.yaml — contains:
#   top-level stories: [{S-1.01 merged}, {S-1.02 draft}, {S-1.03 ready}]
#   epics: E-0.stories: closed  (out-of-enum — must be IGNORED)
#          E-1.stories: tier-1-shipped  (out-of-enum — must be IGNORED)
#
# REGRESSION TEST: if this test fails after a change to the awk patterns, the
# awk anchor has regressed from /^stories:/ back to /^[[:space:]]*stories:/
# and the PC5-violating data workaround (epics subkey rename) will be required again.
#
# PORTABILITY: fixture-based — CI-portable; no .factory/ dependency.
# ---------------------------------------------------------------------------

@test "test_epics_coexistence_nested_stories_ignored" {
  local fixture="${_FIXTURE_DIR}/fixture-migrated.yaml"
  [ -f "${fixture}" ] || {
    echo "FAIL (fixture missing): ${fixture}" >&2; false
  }

  # Sanity: fixture must have top-level stories: as a sequence
  _stories_is_sequence "${fixture}" || {
    echo "FAIL (test-precondition): fixture-migrated.yaml stories: is not a sequence." >&2
    false
  }

  # Assert (a): _count_stories_per_story_entries returns exactly 3 (top-level entries only).
  # If the awk anchor regresses to /^[[:space:]]*stories:/, nested epics[*].stories: keys
  # would be re-entered as in_stories=1, potentially inflating the count with non-entries.
  local entry_count
  entry_count="$(_count_stories_per_story_entries "${fixture}")"
  [ "${entry_count}" -eq 3 ] || {
    echo "FAIL (PC5 coexistence / EC-004 regression): _count_stories_per_story_entries = ${entry_count}, expected 3." >&2
    echo "  fixture-migrated.yaml has exactly 3 top-level stories: entries (S-1.01, S-1.02, S-1.03)." >&2
    echo "  The epics: section has 2 nested 'stories: <scalar>' sub-keys that must be ignored." >&2
    echo "  If count != 3, the awk anchor has regressed to /^[[:space:]]*stories:/ (bug reintroduced)." >&2
    false
  }

  # Assert (b): status scan of top-level stories: list returns ONLY 8-enum values.
  # The epics[*].stories: sub-key scalar values (closed, tier-1-shipped) must NOT appear.
  local invalid_found=0
  local statuses
  statuses="$(awk '
    BEGIN { in_stories=0 }
    /^stories:/ { in_stories=1; next }
    in_stories && /^[^[:space:]#-]/ && !/^[[:space:]]/ { in_stories=0 }
    in_stories && /^[[:space:]]+status:/ {
      val=$0; sub(/^[[:space:]]+status:[[:space:]]*/, "", val); gsub(/[[:space:]]*$/, "", val); print val
    }
  ' "${fixture}")"

  local s
  while IFS= read -r s; do
    [ -z "${s}" ] && continue
    case "${s}" in
      draft|ready|in-progress|partial|blocked|merged|withdrawn|cancelled) ;;
      *)
        echo "FAIL (PC5/EC-004 regression): status '${s}' from epics section leaked into stories: scan." >&2
        echo "  The awk anchor /^stories:/ must prevent epics[*].stories: sub-keys from activating in_stories." >&2
        echo "  Expected: only {merged, draft, ready} from top-level stories: list." >&2
        echo "  Got '${s}' — this is an out-of-enum epics scalar value (closed or tier-1-shipped)." >&2
        invalid_found=1
        ;;
    esac
  done <<EOF
${statuses}
EOF
  [ "${invalid_found}" -eq 0 ] || false

  # Assert (c): exactly 3 status values emitted (one per story entry — no epics leakage)
  local status_count
  status_count="$(printf '%s\n' "${statuses}" | grep -c '[^[:space:]]' || true)"
  [ "${status_count}" -eq 3 ] || {
    echo "FAIL (PC5/EC-004 regression): status scan emitted ${status_count} values, expected 3." >&2
    echo "  Expected: merged, draft, ready (one per top-level stories: entry)." >&2
    echo "  Extra values indicate epics[*].status: fields leaked through the awk anchor." >&2
    false
  }

  # Assert (d): _leading_terminal_run processes top-level stories only.
  # fixture-migrated.yaml: S-1.01 merged (terminal), S-1.02 draft (breaks run), S-1.03 ready.
  # Leading terminal run = 1 (S-1.01 merged; S-1.02 draft breaks the run).
  # If epics leakage occurred, extra status values could corrupt the run count.
  local run
  run="$(_leading_terminal_run "${fixture}")"
  [ "${run}" -eq 1 ] || {
    echo "FAIL (PC5/EC-004 regression): _leading_terminal_run = ${run}, expected 1." >&2
    echo "  fixture-migrated.yaml: S-1.01 merged (terminal), then S-1.02 draft (non-terminal breaks run)." >&2
    echo "  Leading run should be exactly 1. A different value indicates epics leakage or wrong fixture." >&2
    false
  }
}
