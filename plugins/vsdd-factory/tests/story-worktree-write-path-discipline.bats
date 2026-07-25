#!/usr/bin/env bats
# story-worktree-write-path-discipline.bats — S-21.04 gate harness suite (adv pass-1 strengthened).
#
# Five load-bearing layers:
#   DOC-PARITY (step-g-cleanup.md §G.1):
#     Semantic assertions — exact find command form (no blanket 2>/dev/null; F-S2104-P1-002a),
#     preflight-before-dispatch ordering via awk line-number comparison (F-S2104-P1-002b),
#     Invariant-2 no-exceptions clause (F-S2104-P1-002c), PC2b retry-mandate (F-S2104-P1-002d),
#     PC2b Option A/B message body (F-S2104-P1-011), PC2c semantic direction gate (AC-006;
#     F-S2104-P2-008 strengthened: error condition + HALT direction + no proceed-semantics),
#     PC2a sub-case (a) absent-dir guard (F-S2104-P2-009), no-force negative (F-S2104-P1-010).
#   DOC-PARITY (primary paths — F-S2104-P1-001 / F-S2104-P2-001):
#     SKILL.md Step 8, agents/orchestrator/per-story-delivery.md step (g) + Story Split
#     Recovery section, AND WINNING playbook (workflows/phases/per-story-delivery.md) Step 8
#     must each reference the §G.1 preflight. WINNING playbook is authoritative per its own L8
#     ("If the two disagree, this file wins"). RED for winning playbook until implementer propagates.
#   DOC-PARITY (_shared-context.md §Spec-Path Discipline):
#     Write Discipline clause, CANONICAL_FACTORY_ROOT, DELIVERY ledger (AC-001).
#   EXECUTABLE-HARNESS (anti-tautology — F-S2104-P1-002e, TD-VSDD-059):
#     _run_teardown_preflight() extracts the find command verbatim from §G.1, substitutes
#     <worktree-path>, and evaluates that command — a -type d or -name '*.tmp' doc-mutant
#     changes harness behavior and fails T-001/T-002. A harness hardcoding its own find
#     would pass T-002 tautologically; this gate prevents that. Returns non-zero on PC2b
#     and PC2c (F-S2104-P1-003).
#   POLICY 15 v1.4.10 (mutant-proving):
#     REMOVE_LOG sentinel is load-bearing — never written on PREFLIGHT BLOCKED / PC2c paths.
#
# Mechanism: filesystem-fixture harness (S-21.04 variant of S-21.03 two-layer pattern).
#
# Gate targets:
#   plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1
#   plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md §Spec-Path Discipline
#   plugins/vsdd-factory/skills/deliver-story/SKILL.md (Step 8)
#   plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md (step g + Story Split Recovery)
#   plugins/vsdd-factory/workflows/phases/per-story-delivery.md (Step 8 — WINNING playbook; F-S2104-P2-001)
# BC: BC-6.26.001 v1.5 (PC1, PC2a sub-cases a/b, PC2b, PC2c, Invariants 1–5)
# Story: S-21.04
#
# Test plan:
#   T-001  AC-003  stray-file-blocks:       stray .factory/ file → PREFLIGHT BLOCKED (non-zero) + git worktree remove NOT called
#   T-002  AC-004  empty-tree-proceeds:     EC-005 (no .factory/) + EC-003 (empty .factory/ dir) → teardown proceeds in both cases
#   T-003  AC-005  relocate-retry-proceeds: stray file relocated → retry teardown proceeds
#   T-004  AC-006  pc2c-halt:               find error (non-path-absent) → HALT non-zero, exit code+stderr surfaced, worktree-remove NOT called

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  STEP_G_CLEANUP="$PLUGIN_ROOT/skills/deliver-story/steps/step-g-cleanup.md"
  SHARED_CONTEXT_MD="$PLUGIN_ROOT/skills/deliver-story/steps/_shared-context.md"
  SKILL_MD="$PLUGIN_ROOT/skills/deliver-story/SKILL.md"
  PER_STORY_DELIVERY_MD="$PLUGIN_ROOT/agents/orchestrator/per-story-delivery.md"
  WINNING_PLAYBOOK_MD="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  FIXTURE_DIR="$PLUGIN_ROOT/tests/fixtures/story-worktree"

  # Fixture worktree lifecycle: fresh tmpfs workspace per test run.
  # MOCK_WORKTREE    — simulates .worktrees/S-021/ (story worktree path).
  # CANONICAL_FACTORY — simulates the main-checkout .factory/ mount.
  # REMOVE_LOG       — sentinel: 'worktree-remove-invoked' appended when PC2a proceeds.
  WORK="$(mktemp -d)"
  MOCK_WORKTREE="$WORK/story-worktree"
  CANONICAL_FACTORY="$WORK/canonical-factory"
  REMOVE_LOG="$WORK/worktree-remove.log"
  touch "$REMOVE_LOG"
  mkdir -p "$MOCK_WORKTREE" "$CANONICAL_FACTORY"
}

teardown() {
  # Restore permissions before rm to handle T-004's chmod 000 fixture (F-S2104-P1-014).
  if [ -n "${WORK:-}" ]; then
    chmod -R 755 "$WORK" 2>/dev/null || true
    rm -rf "$WORK"
  fi
}

# ===========================================================================
# DOC-PARITY helpers
# ===========================================================================

# Extracts the Sub-step G.1 block from step-g-cleanup.md.
# Start: /^## Sub-step G\.1/ — exits on next /^## / heading.
_extract_g1_section() {
  awk '
    /^## Sub-step G\.1/ { found=1; next }
    found && /^## / { exit }
    found { print }
  ' "$STEP_G_CLEANUP"
}

# Extracts the Spec-Path Discipline section from _shared-context.md.
# Start: /^### Spec-Path Discipline/ — exits on the next /^### / sibling heading
# (e.g. ### Story-Size Gate) OR any /^## / heading.
# The #### Write Discipline child is inside this section and is correctly captured
# because #### (4 #) does not match ^### (3 # + space). Previously only ^## was
# checked, causing §Story-Size Gate (which is ### level) to be over-captured into
# the extraction (F-S2104-P3-015 fix).
_extract_spec_path_discipline_section() {
  awk '
    /^### Spec-Path Discipline/ { found=1; next }
    found && /^### / { exit }
    found && /^## / { exit }
    found { print }
  ' "$SHARED_CONTEXT_MD"
}

# Extracts the Step 8 block from SKILL.md (deliver-story).
# Start: /^### Step 8/ — exits on next /^### / heading.
_extract_skill_step8_section() {
  awk '
    /^### Step 8/ { found=1; next }
    found && /^### / { exit }
    found { print }
  ' "$SKILL_MD"
}

# Extracts the step (g) block from agents/orchestrator/per-story-delivery.md.
# Structural section-bounded extraction (F-S2104-P2-016b): anchors on the '   g.' list item
# label; exits on the next top-level numbered item ('^[0-9]+\.') or '^## ' section heading.
# A fragile line-offset window (lineno-5 … lineno+8) was removed — inserted prose between the
# preflight mention and the Remove worktree dispatch could silently push the preflight reference
# out of a fixed window.
# Portable: awk-only, no grep BRE '\|' alternation (F-S2104-P2-016a).
# Hardcoded: item-label anchor ('   g.'); doc-derived: the preflight content within that item.
_extract_per_story_delivery_step_g_window() {
  awk '
    /^   g\./ { found=1 }
    found && /^[0-9]+\./ { exit }
    found && /^## / { exit }
    found { print }
  ' "$PER_STORY_DELIVERY_MD"
}

# Extracts the Story Split Recovery section from per-story-delivery.md.
_extract_per_story_delivery_split_recovery_section() {
  awk '
    /^## Story Split Recovery/ { found=1; next }
    found && /^## / { exit }
    found { print }
  ' "$PER_STORY_DELIVERY_MD"
}

# Extracts the Step 8 block from the WINNING playbook (workflows/phases/per-story-delivery.md).
# The winning playbook declares on L8: "If the two disagree, this file wins."
# Both this file and agents/orchestrator/per-story-delivery.md must carry the §G.1 mandate.
# Structural extraction: anchors on '^## Step 8'; exits on next '^## ' heading.
_extract_winning_playbook_step8_section() {
  awk '
    /^## Step 8/ { found=1; next }
    found && /^## / { exit }
    found { print }
  ' "$WINNING_PLAYBOOK_MD"
}

# Extracts the PC2c paragraph from step-g-cleanup.md §G.1.
# Start: line matching '\*\*PC2c'; exits on '\*\*Why this' or '^### ' heading.
# Used by F-S2104-P2-008 semantic direction assertions (error condition + HALT + no-proceed).
_extract_g1_pc2c_block() {
  awk '
    /\*\*PC2c/ { found=1 }
    found && /\*\*Why this/ { exit }
    found && /^### / { exit }
    found { print }
  ' "$STEP_G_CLEANUP"
}

# Extracts the PC2b paragraph from step-g-cleanup.md §G.1.
# Start: line matching '\*\*PC2b'; exits on '\*\*PC2c', '\*\*Why this', or '^### '.
# Used by F-S2104-P3-011 HALT-direction assertions (symmetric to PC2c gate in T-004).
_extract_g1_pc2b_block() {
  awk '
    /\*\*PC2b/ { found=1 }
    found && /\*\*PC2c/ { exit }
    found && /\*\*Why this/ { exit }
    found && /^### / { exit }
    found { print }
  ' "$STEP_G_CLEANUP"
}

_assert_doc_marker() {
  # $1=regex  $2=label  $3=section_text
  printf '%s\n' "$3" | grep -qE "$1" || {
    echo "DOC-PARITY FAIL [must contain: $2]"
    false
  }
}

_assert_no_doc_marker() {
  # $1=regex  $2=label  $3=section_text
  if printf '%s\n' "$3" | grep -qE "$1"; then
    echo "DOC-PARITY FAIL [must NOT contain: $2]"
    false
  fi
}

# ===========================================================================
# EXECUTABLE-HARNESS helper
# ===========================================================================

# Run the BC-6.26.001 PC2 teardown preflight against a fixture worktree path.
# Args:
#   $1  worktree_path  — simulated story-worktree path (tmpfs fixture)
#   $2  remove_log     — sentinel file; 'worktree-remove-invoked' appended on PC2a proceed
#
# Anti-tautology gate (TD-VSDD-059, F-S2104-P1-002e): extracts the find command verbatim
# from step-g-cleanup.md §G.1 (line matching 'find ... .factory ... -type f' without
# 2>/dev/null), substitutes <worktree-path> with the fixture path, and evaluates that
# extracted command. A -type d or -name '*.tmp' doc-mutant changes which files find returns,
# failing T-001 (stray file not found) or T-002 (directory found instead of nothing).
# A harness hardcoding 'find ... -type f 2>/dev/null || true' would pass T-002 tautologically
# regardless of doc content; this gate closes that tautology.
#
# PC2a sub-case (a): .factory/ absent → no stray files, proceed (return 0)
# PC2a sub-case (b): find exits 0, empty output → no stray files, proceed (return 0)
# PC2b: find exits 0, non-empty output → PREFLIGHT BLOCKED with stray paths + Option A/B + retry mandate (return 1)
# PC2c: find exits non-zero (non-path-absent) → PREFLIGHT HALT, surface exit code + stderr (return 1)
# Gate failure: find command not extractable from §G.1 in conformant form → HARNESS FAIL (return 1)
_run_teardown_preflight() {
  local worktree_path="$1" remove_log="$2"
  local g1_section
  g1_section="$(_extract_g1_section)"

  # Anti-tautology gate: extract the specific find command line from §G.1.
  # The line must contain 'find', '.factory', and '-type f'.
  # It must NOT contain '2>/dev/null' (BC v1.5 removed blanket suppression for PC2c).
  # Pre-implementation (doc has 2>/dev/null or wrong -type): gate fires.
  # Post-implementation (conformant find ... -type f): gate passes, extracted command is eval'd.
  local find_cmd_line
  find_cmd_line="$(printf '%s\n' "$g1_section" | \
    grep -E '^[[:space:]]*find[[:space:]]' | \
    grep '\.factory' | \
    grep -- '-type f' | \
    grep -v '2>/dev/null' | \
    head -1)"

  if [ -z "$find_cmd_line" ]; then
    printf 'HARNESS FAIL: could not extract conformant find command from step-g-cleanup.md §G.1\n'
    printf '  Required: line matching find ... .factory ... -type f (without 2>/dev/null)\n'
    printf '  BC-6.26.001 PC2 preflight not yet in conformant form (blanket 2>/dev/null must be removed)\n'
    return 1
  fi

  # PC2a sub-case (a): .factory/ directory absent in story worktree — no stray files.
  # Path-absent is NOT a PC2c error; it is the expected clean state (BC-6.26.001 EC-005).
  if [ ! -e "${worktree_path}/.factory" ]; then
    printf 'worktree-remove-invoked\n' >> "$remove_log"
    return 0
  fi

  # Substitute <worktree-path> template variable with the actual fixture path, strip leading whitespace.
  local concrete_cmd
  concrete_cmd="$(printf '%s\n' "$find_cmd_line" | \
    sed "s|<worktree-path>|${worktree_path}|g" | \
    sed 's/^[[:space:]]*//')"

  # Execute the extracted find command. Capture stdout, stderr, and exit code separately.
  local find_stdout find_stderr_file find_exit find_stderr
  find_stderr_file="$(mktemp)"
  find_stdout="$(eval "$concrete_cmd" 2>"$find_stderr_file")"
  find_exit=$?
  find_stderr="$(cat "$find_stderr_file")"
  rm -f "$find_stderr_file"

  if [ "$find_exit" -ne 0 ]; then
    # PC2c: find exited non-zero for a non-path-absent reason — fail-closed HALT.
    # Surface exact exit code and stderr to operator (BC-6.26.001 PC2c).
    # git worktree remove must NOT be executed; find errors must not silently authorize rm -rf.
    printf 'PREFLIGHT HALT (PC2c): find exited with code %d — non-path-absent error; teardown HALTED (fail-closed gate, BC-6.26.001 PC2c).\n' "$find_exit"
    printf 'find stderr: %s\n' "$find_stderr"
    printf 'git worktree remove NOT executed.\n'
    return 1
  fi

  if [ -n "$find_stdout" ]; then
    # PC2b: stray factory artifacts found — PREFLIGHT BLOCKED.
    # git worktree remove must NOT be executed; REMOVE_LOG intentionally NOT written.
    printf 'PREFLIGHT BLOCKED: Found factory artifact(s) in story worktree shadow .factory/:\n'
    printf '%s\n' "$find_stdout"
    printf 'These files were written to the wrong worktree (issue #523 class) and would be\n'
    printf 'permanently destroyed by git worktree remove. Manual intervention required:\n'
    printf '  Option A: Relocate to canonical .factory/ mount, verify content, then retry teardown.\n'
    printf '  Option B: Discard (only if files are confirmed redundant copies already committed on factory-artifacts).\n'
    printf 'Story cleanup MUST NOT complete until a retry preflight returns an empty result.\n'
    return 1
  fi

  # PC2a sub-case (b): find exits 0, empty output — no stray files, proceed with teardown.
  printf 'worktree-remove-invoked\n' >> "$remove_log"
  return 0
}

# ===========================================================================
# T-001 / AC-003 / PC2b: stray .factory/ file → PREFLIGHT BLOCKED (non-zero); worktree-remove NOT called
# BC-6.26.001 v1.5 PC2b, Invariants 2, 5
# RG-001 closure
# ===========================================================================

@test "T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called" {
  # Fixture: stray .factory/stories/S-021-DELIVERY.md inside mock worktree.
  # Primary issue #523 failure mode: DELIVERY ledger written via CWD-relative path from story
  # worktree CWD. Shadow .factory/ is gitignored → git worktree remove clean-state check
  # passes silently (false negative); rm -rf destroys shadow content (BC-6.26.001 Invariant 5).
  #
  # Pre-implementation red gate:
  #   DOC-PARITY (no 2>/dev/null): doc has blanket 2>/dev/null → assertion fails (RED).
  #   DOC-PARITY (primary paths): SKILL.md Step 8 + per-story-delivery.md step (g) lack
  #     preflight references → assertions fail (RED).
  #   HARNESS: extraction gate fires (2>/dev/null present) → 'PREFLIGHT BLOCKED' absent → RED.
  # Post-implementation: all DOC-PARITY GREEN; harness emits PREFLIGHT BLOCKED with stray path;
  #   non-zero status; REMOVE_LOG empty (RG-001 closed).

  # --- Fixture setup: stray factory artifacts in shadow .factory/ ---
  mkdir -p "$MOCK_WORKTREE/.factory/stories"
  printf 'stray DELIVERY ledger — written via CWD-relative path from story worktree CWD\n' \
    > "$MOCK_WORKTREE/.factory/stories/S-021-DELIVERY.md"
  # Non-.md stray artifact: makes the 'any file type' property of -type f load-bearing
  # (F-S2104-P2-010). A '-name *.md' doc-mutant would skip this file — the assertion below
  # on 'engine-config.yaml' catches the mutant. Issue #523 confirmed-loss set includes
  # non-.md engine-config artifacts.
  printf 'engine-config artifact — non-.md stray vector (issue #523 confirmed-loss set)\n' \
    > "$MOCK_WORKTREE/.factory/engine-config.yaml"

  local g1_section
  g1_section="$(_extract_g1_section)"

  # --- DOC-PARITY §G.1: exact preflight command form — find + -type f, NO blanket 2>/dev/null (F-S2104-P1-002a) ---
  # BC-6.26.001 v1.5 removed blanket 2>/dev/null; PC2c requires visible find exit codes.
  # RED pre-implementation (doc has 2>/dev/null); GREEN post-implementation.
  _assert_doc_marker 'find.*\.factory.*-type[[:space:]]+f' \
    "step-g-cleanup.md §G.1: find .factory -type f command present (BC-6.26.001 PC2)" \
    "$g1_section"
  _assert_no_doc_marker 'find.*\.factory.*-type[[:space:]]+f.*2>/dev/null' \
    "step-g-cleanup.md §G.1: blanket 2>/dev/null suppression FORBIDDEN on preflight find command (BC-6.26.001 v1.5 PC2; removed to enable PC2c fail-closed detection)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: preflight-before-dispatch ordering (F-S2104-P1-002b) ---
  # Assert the '### Mandatory Teardown Preflight' heading precedes the '### Dispatch' heading.
  local preflight_line dispatch_line
  preflight_line="$(awk '/Mandatory Teardown Preflight/{ print NR; exit }' "$STEP_G_CLEANUP")"
  dispatch_line="$(awk '/^### Dispatch.*PC2a only/{ print NR; exit }' "$STEP_G_CLEANUP")"
  [ -n "$preflight_line" ] || {
    echo "DOC-PARITY FAIL: 'Mandatory Teardown Preflight' heading not found in step-g-cleanup.md (BC-6.26.001 PC2, Invariant 2)"
    false
  }
  [ -n "$dispatch_line" ] || {
    echo "DOC-PARITY FAIL: '### Dispatch (PC2a only' heading not found in step-g-cleanup.md (BC-6.26.001 PC2a)"
    false
  }
  [ "$preflight_line" -lt "$dispatch_line" ] || {
    echo "DOC-PARITY FAIL: preflight heading (line $preflight_line) must precede dispatch heading (line $dispatch_line) in §G.1 ordering invariant"
    false
  }

  # --- DOC-PARITY §G.1: Invariant-2 no-exceptions clause (F-S2104-P1-002c) ---
  # "This step is mandatory with no exceptions — not even when the agent is confident..."
  _assert_doc_marker 'no exceptions|not even when.*confident' \
    "step-g-cleanup.md §G.1: Invariant-2 no-exceptions clause — mandatory even when agent is confident no shadow writes occurred (BC-6.26.001 Invariant 2)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: PC2b retry-mandate line (F-S2104-P1-002d) ---
  # "Story cleanup MUST NOT complete until a retry preflight returns an empty result."
  _assert_doc_marker 'MUST NOT complete until.*retry|cleanup MUST NOT complete until.*empty|retry preflight returns an empty' \
    "step-g-cleanup.md §G.1: PC2b retry mandate — story cleanup MUST NOT complete until retry preflight returns empty result (BC-6.26.001 PC2b §3)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: PC2b Option A/Option B remediation menu (F-S2104-P1-011) ---
  _assert_doc_marker 'Option A' \
    "step-g-cleanup.md §G.1: Option A remediation path in PC2b message body (BC-6.26.001 PC2b)" \
    "$g1_section"
  _assert_doc_marker 'Option B' \
    "step-g-cleanup.md §G.1: Option B remediation path in PC2b message body (BC-6.26.001 PC2b)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: PREFLIGHT BLOCKED message and git worktree remove ---
  _assert_doc_marker 'PREFLIGHT BLOCKED' \
    "step-g-cleanup.md §G.1: PREFLIGHT BLOCKED halt message on stray files (BC-6.26.001 PC2b, Invariant 2)" \
    "$g1_section"
  _assert_doc_marker 'git worktree remove' \
    "step-g-cleanup.md §G.1: git worktree remove command for PC2a proceed path (BC-6.26.001 PC2a)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: no --force flag in any position (F-S2104-P1-010 strengthened) ---
  # Catches: 'git worktree remove --force <path>', 'git worktree remove <path> --force',
  # and shorthand '-f'. BC-6.26.001 PC2a prohibits --force as a BC mandate.
  if printf '%s\n' "$g1_section" | grep -qE 'git worktree remove[[:space:]].*(--force|-f[^a-zA-Z])|(--force|-f[^a-zA-Z]).*git worktree remove'; then
    echo "DOC-PARITY FAIL: step-g-cleanup.md §G.1 uses git worktree remove with --force or -f flag in any argument position — prohibited by BC-6.26.001 PC2a (BC mandate: strips git built-in protection for non-gitignored untracked files)"
    false
  fi

  # --- DOC-PARITY §G.1: PC2b HALT-direction gate symmetric to PC2c gate in T-004 (F-S2104-P3-011) ---
  # Extract the PC2b block and assert:
  #   (1) HALT / do-NOT-proceed direction is present
  #   (2) No unconditional proceed-forward semantics (e.g., "Proceed to the Dispatch section below")
  # Both should PASS now: §G.1 PC2b says "HALT teardown. Do NOT proceed to git worktree remove."
  local pc2b_block
  pc2b_block="$(_extract_g1_pc2b_block)"
  _assert_doc_marker 'HALT|NOT.*[Pp]roceed|MUST NOT.*complete|BLOCKED' \
    "step-g-cleanup.md §G.1 PC2b block: HALT/do-NOT-proceed direction mandatory — must not silently authorize teardown when stray files found (F-S2104-P3-011)" \
    "$pc2b_block"
  # Negative: catch a mutant that changes PC2b to authorize teardown via "Proceed to the Dispatch"
  _assert_no_doc_marker '[Pp]roceed[[:space:]]+to[[:space:]]+the[[:space:]]+[Dd]ispatch' \
    "step-g-cleanup.md §G.1 PC2b block: must NOT contain 'Proceed to the Dispatch' — a PC2b→authorize mutant keeping the label while adding proceed semantics is caught here (F-S2104-P3-011)" \
    "$pc2b_block"

  # --- DOC-PARITY _shared-context.md §Spec-Path Discipline Write Discipline clause (AC-001) ---
  local spec_path_section
  spec_path_section="$(_extract_spec_path_discipline_section)"
  _assert_doc_marker 'Write Discipline' \
    "_shared-context.md §Spec-Path Discipline: Write Discipline clause (BC-6.26.001 PC1, Invariant 1)" \
    "$spec_path_section"
  _assert_doc_marker 'CANONICAL_FACTORY_ROOT' \
    "_shared-context.md §Spec-Path Discipline: CANONICAL_FACTORY_ROOT mandate (BC-6.26.001 PC1, Invariant 3)" \
    "$spec_path_section"
  _assert_doc_marker 'DELIVERY' \
    "_shared-context.md §Spec-Path Discipline: DELIVERY ledger named as load-bearing case (BC-6.26.001 Invariant 4)" \
    "$spec_path_section"

  # --- DOC-PARITY §Spec-Path Discipline: EC-006 WARNING + no prescriptive story-worktree rev-parse (F-S2104-P3-012) ---
  # AC-001(b) strengthened: the Write Discipline clause must carry the EC-006 WARNING explaining
  # that using the story-worktree-path as the -C argument to rev-parse --show-toplevel returns the
  # wrong root. Additionally, any line in the Write Discipline block containing the story-worktree
  # form of this command must have the WARNING marker on the same line — prescriptive use outside
  # a WARNING context is forbidden (it would instruct agents to use the wrong derivation).
  # Both gates PASS now: EC-006 WARNING is present; the story-worktree form appears only inside
  # the WARNING paragraph. Should stay GREEN after implementer changes.
  _assert_doc_marker 'WARNING.*EC-006|EC-006.*WARNING' \
    "_shared-context.md §Spec-Path Discipline Write Discipline: EC-006 WARNING must be present — omitting removes protection against story-worktree-path misuse in rev-parse --show-toplevel (F-S2104-P3-012)" \
    "$spec_path_section"
  # Negative gate: no line in the Write Discipline block may contain the story-worktree-path
  # form of rev-parse --show-toplevel unless that same line carries the WARNING marker.
  # grep -v 'WARNING' filters out the permitted WARNING-context line; remaining matches are forbidden.
  local forbidden_revparse_lines
  forbidden_revparse_lines="$(printf '%s\n' "$spec_path_section" | \
    grep -E 'story-worktree.*rev-parse.*show-toplevel|rev-parse.*show-toplevel.*story-worktree' | \
    grep -v 'WARNING' || true)"
  if [ -n "$forbidden_revparse_lines" ]; then
    echo "DOC-PARITY FAIL [must NOT contain: story-worktree-path rev-parse --show-toplevel outside WARNING context (F-S2104-P3-012)]"
    printf '%s\n' "$forbidden_revparse_lines"
    false
  fi

  # --- DOC-PARITY primary paths: SKILL.md Step 8 (F-S2104-P1-001a) ---
  # RED until implementer adds preflight reference to SKILL.md Step 8 dispatch.
  local skill_step8_section
  skill_step8_section="$(_extract_skill_step8_section)"
  _assert_doc_marker 'preflight|step-g-cleanup|§G\.1|G\.1' \
    "SKILL.md Step 8: must reference §G.1 preflight before cleanup dispatch — RED until implementer propagates (BC-6.26.001 PC2; F-S2104-P1-001a)" \
    "$skill_step8_section"

  # --- DOC-PARITY primary paths: per-story-delivery.md step (g) (F-S2104-P1-001b) ---
  # RED until implementer adds preflight reference adjacent to step (g) dispatch.
  local step_g_window
  step_g_window="$(_extract_per_story_delivery_step_g_window)"
  _assert_doc_marker 'preflight|step-g-cleanup|§G\.1|G\.1' \
    "per-story-delivery.md step (g): must reference §G.1 preflight before Remove worktree dispatch — RED until implementer propagates (BC-6.26.001 PC2; F-S2104-P1-001b)" \
    "$step_g_window"

  # --- DOC-PARITY primary paths: per-story-delivery.md Story Split Recovery (F-S2104-P1-001b) ---
  # RED until implementer adds preflight reference to story-split cleanup step.
  local split_recovery_section
  split_recovery_section="$(_extract_per_story_delivery_split_recovery_section)"
  _assert_doc_marker 'preflight|step-g-cleanup|§G\.1|G\.1' \
    "per-story-delivery.md Story Split Recovery: must reference §G.1 preflight before worktree removal — RED until implementer propagates (BC-6.26.001 PC2; F-S2104-P1-001b)" \
    "$split_recovery_section"

  # --- DOC-PARITY WINNING playbook: workflows/phases/per-story-delivery.md Step 8 (F-S2104-P2-001) ---
  # The winning playbook's own L8 declares: "If the two disagree, this file wins."
  # Its Step 8 must carry the §G.1 preflight mandate; the orchestrator copy carrying it is
  # insufficient — the winning playbook is the authoritative reference under disagreement.
  # RED until implementer propagates §G.1 mandate to Step 8 of the winning playbook.
  local winning_step8_section
  winning_step8_section="$(_extract_winning_playbook_step8_section)"
  _assert_doc_marker 'preflight|step-g-cleanup|§G\.1|G\.1' \
    "WINNING playbook (workflows/phases/per-story-delivery.md) Step 8: must reference §G.1 preflight before cleanup dispatch — orchestrator copy alone is insufficient (winning playbook wins on disagreement per its L8); RED until implementer propagates (BC-6.26.001 PC2; F-S2104-P2-001)" \
    "$winning_step8_section"

  # --- HARNESS: stray file → PREFLIGHT BLOCKED; non-zero exit (F-S2104-P1-003) ---
  # Anti-tautology: extracted find command from §G.1 is evaluated against fixture.
  # A -type d mutant would not find the file (directories only) → output empty → PC2a proceed →
  # REMOVE_LOG written → [ ! -s REMOVE_LOG ] fails → test RED (mutant caught).
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -ne 0 ] || {
    echo "HARNESS FAIL: _run_teardown_preflight must exit non-zero on PC2b (stray files found) — got status 0"
    false
  }
  printf '%s\n' "$output" | grep -q 'PREFLIGHT BLOCKED' || {
    echo "HARNESS FAIL: 'PREFLIGHT BLOCKED' not in preflight output — got: $output"
    false
  }
  # Stray file path must appear verbatim in output (F-S2104-P1-003)
  printf '%s\n' "$output" | grep -q 'S-021-DELIVERY.md' || {
    echo "HARNESS FAIL: stray file path 'S-021-DELIVERY.md' must appear verbatim in PREFLIGHT BLOCKED output — got: $output"
    false
  }
  # Non-.md stray file must also appear in output (F-S2104-P2-010: 'any file type' property
  # of -type f is load-bearing; a '-name *.md' doc-mutant would miss engine-config.yaml,
  # causing this assertion to fail — the mutant is caught here, not by changed find semantics)
  printf '%s\n' "$output" | grep -q 'engine-config.yaml' || {
    echo "HARNESS FAIL: non-.md stray file 'engine-config.yaml' must appear in PREFLIGHT BLOCKED output — a '-name *.md' doc-mutant would skip non-.md files; got: $output"
    false
  }
  # Mutant-proving sentinel: git worktree remove must NOT be invoked on PREFLIGHT BLOCKED path
  [ ! -s "$REMOVE_LOG" ] || {
    echo "HARNESS FAIL: REMOVE_LOG non-empty — git worktree remove was invoked but MUST NOT be on PREFLIGHT BLOCKED path — log: $(cat "$REMOVE_LOG")"
    false
  }
}

# ===========================================================================
# T-002 / AC-004 / PC2a: empty shadow .factory/ → teardown proceeds; git worktree remove IS called
# Covers: EC-005 (no .factory/ dir — PC2a sub-case a) + EC-003 (empty .factory/ dir — PC2a sub-case b)
# BC-6.26.001 v1.5 PC2a
# RG-002 closure
# F-S2104-P1-013: EC labels corrected; both EC-005 and EC-003 exercised explicitly.
# ===========================================================================

@test "T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called" {
  # Fixture part 1 (EC-005): MOCK_WORKTREE exists but has NO .factory/ directory.
  # Represents clean story worktree — no shadow .factory/ created (expected clean state).
  # PC2a sub-case (a): .factory/ absent → no stray files, teardown authorized.
  #
  # Fixture part 2 (EC-003): empty .factory/ dir present (no files).
  # PC2a sub-case (b): find exits 0 with empty output → no stray files, teardown authorized.
  #
  # Pre-implementation red gate:
  #   DOC-PARITY (no 2>/dev/null): doc has blanket 2>/dev/null → assertion fails (RED).
  #   HARNESS: extraction gate fires → 'worktree-remove-invoked' never written → RED.
  # Post-implementation: DOC-PARITY GREEN; harness writes sentinel for both EC-005 and EC-003.

  # --- DOC-PARITY: step-g-cleanup.md §G.1 preflight mandate ---
  local g1_section
  g1_section="$(_extract_g1_section)"

  _assert_doc_marker 'find.*\.factory.*-type[[:space:]]+f' \
    "step-g-cleanup.md §G.1: find .factory -type f preflight command (BC-6.26.001 PC2)" \
    "$g1_section"
  _assert_no_doc_marker 'find.*\.factory.*-type[[:space:]]+f.*2>/dev/null' \
    "step-g-cleanup.md §G.1: blanket 2>/dev/null FORBIDDEN on preflight command (BC-6.26.001 v1.5)" \
    "$g1_section"
  _assert_doc_marker 'PREFLIGHT BLOCKED' \
    "step-g-cleanup.md §G.1: PREFLIGHT BLOCKED mandate present (PC2a and PC2b in same §G.1 block — BC-6.26.001 PC2)" \
    "$g1_section"
  _assert_doc_marker 'git worktree remove' \
    "step-g-cleanup.md §G.1: git worktree remove command for PC2a proceed path (BC-6.26.001 PC2a)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: PC2a sub-case (a) — .factory/ absent → proceed (F-S2104-P2-009) ---
  # Deleting the absent-dir clause from §G.1 must fail this assertion.
  # The harness implements this sub-case at _run_teardown_preflight line 1 (hardcoded);
  # the DOC gate here verifies the spec documents the same behavior.
  _assert_doc_marker '\.factory.*absent|absent.*\.factory|no.*\.factory.*directory|path-absent.*NOT.*PC2c|EC-005' \
    "step-g-cleanup.md §G.1: PC2a sub-case (a) — .factory/ absent path must be documented (BC-6.26.001 EC-005; deleting this clause silently breaks the absent-dir contract — F-S2104-P2-009)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: PC2a sub-case (a) discrimination predicate (F-S2104-P3-001) ---
  # §G.1 must supply a normative discrimination predicate — a literal shell conditional
  # (e.g. `[ ! -d "<worktree-path>/.factory" ]`) distinguishing PC2a(a) (absent dir → proceed)
  # from PC2c (find exits non-zero → HALT). Without this predicate the doc gap is compensated
  # by the hardcoded `[ ! -e "${worktree_path}/.factory" ]` pre-test in _run_teardown_preflight,
  # but the DOC-PARITY gate carries the semantic load and must be RED until the implementer adds
  # the explicit conditional to §G.1. Hardcoded harness pre-test continues to carry the runtime
  # load; honest comment per F-S2104-P3-001 constraint.
  # RED pre-implementation: §G.1 describes absent-dir in prose only; no shell conditional present.
  # GREEN post-implementation: §G.1 contains [ ! -d or equivalent shell test.
  _assert_doc_marker '\[ ! -d|\[ ! -e|test[[:space:]].*!.*-d.*\.factory|if.*\[.*!.*-d.*\.factory' \
    "step-g-cleanup.md §G.1: normative discrimination predicate required — explicit shell conditional (e.g. [ ! -d \"<worktree-path>/.factory\" ]) MUST appear in §G.1 to distinguish PC2a(a) absent-dir from PC2c find-error; harness hardcodes pre-test to compensate but DOC-PARITY gate must be RED until implementer adds the conditional (F-S2104-P3-001)" \
    "$g1_section"

  # --- HARNESS EC-005: no .factory/ dir → PC2a sub-case (a), teardown proceeds ---
  # No fixture setup needed: MOCK_WORKTREE exists but has no .factory/ directory.
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -eq 0 ] || {
    echo "HARNESS FAIL: EC-005 (absent .factory/) must return 0 — got status $status; output: $output"
    false
  }
  grep -q 'worktree-remove-invoked' "$REMOVE_LOG" || {
    echo "HARNESS FAIL: EC-005 teardown did not proceed — worktree-remove-invoked not in REMOVE_LOG; output: $output"
    false
  }

  # --- HARNESS EC-003: empty .factory/ dir present → PC2a sub-case (b), teardown proceeds ---
  # Explicitly covers EC-003 (empty dir scenario distinct from EC-005 absent-dir scenario).
  mkdir -p "$MOCK_WORKTREE/.factory"
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -eq 0 ] || {
    echo "HARNESS FAIL: EC-003 (empty .factory/ dir) must return 0 — got status $status; output: $output"
    false
  }
  local count
  count="$(grep -c 'worktree-remove-invoked' "$REMOVE_LOG")"
  [ "$count" -ge 2 ] || {
    echo "HARNESS FAIL: EC-003 teardown did not write second sentinel entry — expected ≥2 entries, got $count"
    false
  }
}

# ===========================================================================
# T-003 / AC-005 / PC2b → PC2a retry: stray file relocated → retry teardown proceeds
# BC-6.26.001 v1.5 PC2b → PC2a retry path (Option A relocation)
# RG-003 closure
# ===========================================================================

@test "T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds" {
  # Fixture: stray .factory/ file; relocate to canonical mount; retry teardown.
  # Exercises BC-6.26.001 PC2b → PC2a retry path (Option A: Relocate then retry).
  #
  # Pre-implementation red gate:
  #   DOC-PARITY (no 2>/dev/null): doc has blanket 2>/dev/null → assertion fails (RED).
  #   HARNESS: extraction gate fires on both passes → sentinel never written → RED.
  # Post-implementation: first pass BLOCKED; relocation empties shadow tree; retry proceeds.

  mkdir -p "$MOCK_WORKTREE/.factory/stories"
  printf 'stray DELIVERY ledger — to be relocated via Option A retry path\n' \
    > "$MOCK_WORKTREE/.factory/stories/S-021-DELIVERY.md"

  # --- DOC-PARITY: step-g-cleanup.md §G.1 preflight mandate ---
  local g1_section
  g1_section="$(_extract_g1_section)"

  _assert_doc_marker 'find.*\.factory.*-type[[:space:]]+f' \
    "step-g-cleanup.md §G.1: find .factory -type f preflight command (BC-6.26.001 PC2b → PC2a retry path)" \
    "$g1_section"
  _assert_no_doc_marker 'find.*\.factory.*-type[[:space:]]+f.*2>/dev/null' \
    "step-g-cleanup.md §G.1: blanket 2>/dev/null FORBIDDEN (BC-6.26.001 v1.5)" \
    "$g1_section"
  _assert_doc_marker 'PREFLIGHT BLOCKED' \
    "step-g-cleanup.md §G.1: PREFLIGHT BLOCKED mandate (first pass blocks; retry gated by same mandate — BC-6.26.001 PC2b)" \
    "$g1_section"
  _assert_doc_marker 'git worktree remove' \
    "step-g-cleanup.md §G.1: git worktree remove for PC2a retry proceed path (BC-6.26.001 PC2a)" \
    "$g1_section"

  # --- HARNESS: first pass (stray file present) → PREFLIGHT BLOCKED ---
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -ne 0 ] || {
    echo "HARNESS FAIL: first pass must return non-zero (stray file present) — got status 0; output: $output"
    false
  }
  # REMOVE_LOG must remain empty until stray file is relocated
  [ ! -s "$REMOVE_LOG" ] || {
    echo "HARNESS FAIL: REMOVE_LOG non-empty after first pass — worktree-remove must NOT be invoked while stray file present"
    false
  }

  # --- Relocation: move stray file to canonical mount (Option A per BC-6.26.001 PC2b §3) ---
  mkdir -p "$CANONICAL_FACTORY/stories"
  mv "$MOCK_WORKTREE/.factory/stories/S-021-DELIVERY.md" \
     "$CANONICAL_FACTORY/stories/S-021-DELIVERY.md"

  # --- HARNESS: retry pass after relocation (shadow tree now empty → PC2a proceed) ---
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -eq 0 ] || {
    echo "HARNESS FAIL: retry must return 0 after stray file relocated — got status $status; output: $output"
    false
  }
  grep -q 'worktree-remove-invoked' "$REMOVE_LOG" || {
    echo "HARNESS FAIL: retry teardown did not proceed after stray file relocated — REMOVE_LOG: $(cat "$REMOVE_LOG"); first-pass output: previously captured; retry output: $output"
    false
  }

  # --- Nesting pathology guard (F-S2104-P2-015) ---
  # BC-6.26.001 §Description double-nesting warning: the Option A relocation target is
  # $CANONICAL_FACTORY/<relative-path>, where $CANONICAL_FACTORY IS the .factory/ dir.
  # A naive agent might relocate to $CANONICAL_FACTORY/.factory/<artifact> — creating a
  # .factory/.factory/ nesting. Assert the file landed at the correct (non-nested) path
  # and that $CANONICAL_FACTORY/.factory/ does NOT exist after relocation.
  [ -f "$CANONICAL_FACTORY/stories/S-021-DELIVERY.md" ] || {
    echo "HARNESS FAIL: relocated file not found at canonical destination $CANONICAL_FACTORY/stories/S-021-DELIVERY.md (BC-6.26.001 Option A relocation; F-S2104-P2-015)"
    false
  }
  [ ! -d "$CANONICAL_FACTORY/.factory" ] || {
    echo "HARNESS FAIL: double-nesting detected — $CANONICAL_FACTORY/.factory/ must NOT exist after relocation (BC-6.26.001 §Description nesting warning: canonical factory IS .factory/; re-nesting creates .factory/.factory/ — F-S2104-P2-015)"
    false
  }
}

# ===========================================================================
# T-004 / AC-006 / PC2c: find exits non-zero (non-path-absent) → HALT; exit code+stderr surfaced; worktree-remove NOT called
# BC-6.26.001 v1.5 PC2c (fail-closed)
# macOS/Linux portability: find returns exit code 1 on permission-denied subdirectory traversal.
# ===========================================================================

@test "T-004 S-21.04 AC-006: pc2c-halt — find error (non-path-absent) HALTS teardown; exit code+stderr surfaced; worktree-remove NOT called" {
  # Fixture: .factory/ exists with a chmod 000 subdirectory.
  # find traverses .factory/, encounters locked-subdir, emits "Permission denied" to stderr,
  # and exits 1. This is the PC2c scenario: find error for a non-path-absent reason.
  #
  # Why this matters: without PC2c, a permission error on .factory/ content produces empty find
  # stdout (no accessible files returned), which a naive implementation would treat as PC2a
  # (no stray files → proceed). That would authorize rm -rf on a .factory/ whose contents
  # could not be verified — data loss risk. PC2c fail-closed prevents this.
  #
  # Portability: on macOS and Linux, find exits 1 when it encounters permission-denied
  # subdirectory traversal (POSIX-required behavior).
  #
  # Pre-implementation red gate:
  #   DOC-PARITY (PC2c): step-g-cleanup.md §G.1 has no PC2c block → assertion fails (RED).
  #   HARNESS: extraction gate fires (doc still has 2>/dev/null) → HARNESS FAIL returned →
  #     [ "$status" -ne 0 ] passes but PREFLIGHT HALT absent from output → RED.
  # Post-implementation: PC2c block in §G.1 → DOC-PARITY GREEN; harness emits PREFLIGHT HALT
  #   with exit code; REMOVE_LOG empty.

  # Skip if running as root: chmod 000 is ineffective for root (find succeeds regardless).
  if [ "$(id -u)" -eq 0 ]; then
    skip "T-004 requires non-root user (chmod 000 is ineffective as root; find would succeed)"
  fi

  # --- Fixture: .factory/ with a permission-locked subdirectory ---
  mkdir -p "$MOCK_WORKTREE/.factory/locked-subdir"
  chmod 000 "$MOCK_WORKTREE/.factory/locked-subdir"

  # --- DOC-PARITY §G.1: PC2c semantic direction gates (F-S2104-P2-008) ---
  # The old broad alternation 'PC2c|...' allowed a mutant that rewrites the PC2c branch to
  # 'proceed' while keeping the label to pass (bare label match). Replaced with co-occurrence
  # gates on the extracted PC2c block: error condition + HALT direction + no proceed-forward
  # semantics. Mirrors the quality of T-001's PC2b gates (no exceptions / retry / Option A/B).
  local g1_section pc2c_block
  g1_section="$(_extract_g1_section)"
  pc2c_block="$(_extract_g1_pc2c_block)"

  # PC2c block must document the error condition (non-zero exit / non-path-absent reason)
  _assert_doc_marker 'exits.*non-zero|non-zero.*exit|non-path-absent|permission.*deni|traversal.*error' \
    "step-g-cleanup.md §G.1 PC2c block: error condition (non-zero exit / non-path-absent) must be documented (BC-6.26.001 PC2c; F-S2104-P2-008)" \
    "$pc2c_block"

  # PC2c block must direct HALT / NOT executed (fail-closed); bare label without direction is insufficient
  _assert_doc_marker 'HALT|NOT executed|MUST NOT.*remov|must NOT.*remov|NOT.*execut' \
    "step-g-cleanup.md §G.1 PC2c block: HALT direction mandatory — 'HALT' or 'NOT executed' must appear in the PC2c block (BC-6.26.001 PC2c fail-closed; F-S2104-P2-008)" \
    "$pc2c_block"

  # PC2c block must NOT contain unconditional proceed-forward semantics
  # 'Proceed to' / 'Proceed with' would authorize teardown — a mutant keeping 'PC2c' label
  # while adding proceed semantics is caught here but passes the old bare-label assertion
  _assert_no_doc_marker '[Pp]roceed[[:space:]]+(to|with)[[:space:]]' \
    "step-g-cleanup.md §G.1 PC2c block: must NOT contain proceed-forward semantics — a PC2c→proceed mutant keeping the label passes the old assertion but fails this gate (F-S2104-P2-008)" \
    "$pc2c_block"

  # --- HARNESS: find error → HALT (non-zero); PC2c message; exit code surfaced; REMOVE_LOG empty ---
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -ne 0 ] || {
    echo "HARNESS FAIL: PC2c must return non-zero — got status 0 (find error must HALT teardown, not silently authorize rm -rf)"
    false
  }
  printf '%s\n' "$output" | grep -qE 'PREFLIGHT HALT|PC2c' || {
    echo "HARNESS FAIL: PC2c HALT message not in output — got: $output"
    false
  }
  # Exit code must be surfaced in output (BC-6.26.001 PC2c: "exact find exit code ... MUST be surfaced")
  printf '%s\n' "$output" | grep -qE 'exit.*[0-9]|exited.*[0-9]|code [0-9]' || {
    echo "HARNESS FAIL: find exit code must be surfaced in PC2c output — got: $output"
    false
  }
  # Mutant-proving sentinel: git worktree remove must NOT be invoked on PC2c path
  [ ! -s "$REMOVE_LOG" ] || {
    echo "HARNESS FAIL: REMOVE_LOG non-empty on PC2c — git worktree remove must NOT be invoked on find-error path — log: $(cat "$REMOVE_LOG")"
    false
  }
}
