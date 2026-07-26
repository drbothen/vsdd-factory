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
#     PC2a sub-case (a) absent-path guard (F-S2104-P2-009), no-force negative (F-S2104-P1-010).
#   DOC-PARITY (primary paths — F-S2104-P1-001 / F-S2104-P2-001):
#     SKILL.md Step 8, agents/orchestrator/per-story-delivery.md step (g) + Story Split
#     Recovery section, AND WINNING playbook (workflows/phases/per-story-delivery.md) Step 8
#     must each reference the §G.1 preflight. WINNING playbook is authoritative per its own L8
#     ("If the two disagree, this file wins"). Was RED at 93ec340a until the implementer qualified the winning-playbook reference at a4d4ffab/43ea70ba.
#   DOC-PARITY (_shared-context.md §Spec-Path Discipline):
#     Write Discipline clause, CANONICAL_FACTORY_ROOT, DELIVERY ledger (AC-001).
#     pass-16 adds: NEGATION-TRANSPARENCY (Gate 1(a) zero-DoF + Gate 1(b) negation-gate;
#     F-S2104-P16-001(a)), BLOCK-WIDE SENTENCE POLARITY (Gate PW-B; F-S2104-P16-001(b)),
#     SENTENCE-SCOPED Gate 2 + RETIREMENT-LANGUAGE guard (F-S2104-P16-001(c)), Gate 3
#     TIGHTENED + Gate 7 CWD-RELATIVE BULLET POLARITY (F-S2104-P16-002), and ANCHOR-UNIQUENESS
#     bounded to #### Write Discipline (F-S2104-P16-003).
#     pass-17 adds: WHOLE-SECTION domain (write_discipline_prose; F-S2104-P17-001(a)), HTML-
#     COMMENT ABSENCE gate (F-S2104-P17-001(b)), Gate 1(d) CONDITIONAL-SCOPING on mandate
#     sentence (F-S2104-P17-002(c)), Gate PW-B PROHIBITION-TOKEN requirement replacing directive-
#     token whitelist + extended prohibited-target class (F-S2104-P17-002(a)(b)), Gate 2a
#     TIGHTENED to co-occurrence within sentence (F-S2104-P17-002(d)), Gate 2b domain extended
#     to write_discipline_prose + NULLIFICATION-CLASS widened + ADVERSATIVE-CONNECTIVE gate
#     (F-S2104-P17-003), Gates 4/5 domain extended to write_discipline_prose (F-S2104-P17-001(a)),
#     and CANONICAL-TARGET gate replacing Gate 6(b)/7(b) (F-S2104-P17-004).
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
#   6 ungated mandate surfaces (F-S2104-P4-009): skills/worktree-manage/SKILL.md,
#     skills/code-delivery/SKILL.md, skills/fix-pr-delivery/SKILL.md,
#     workflows/code-delivery.lobster, workflows/greenfield.lobster, rules/worktree-protocol.md
#   agents/adversary.md + skills/adversarial-review/SKILL.md (§G.1/BC-6.26.001 awareness; F-S2104-P4-002)
#   agents/devops-engineer.md §Worktree Cleanup (preflight-verification mandate; F-S2104-P4-003)
# BC: BC-6.26.001 (PC1, PC2a sub-cases a/b, PC2b non-directory + symlink cases, PC2c, Invariants 1–5)
# Story: S-21.04
#
# Test plan:
#   T-001  AC-003  stray-file-blocks:       stray .factory/ file → PREFLIGHT BLOCKED (non-zero) + git worktree remove NOT called
#   T-002  AC-004  empty-tree-proceeds:     EC-005 (no .factory/) + EC-003 (empty .factory/ dir) → teardown proceeds in both cases
#   T-003  AC-005  relocate-retry-proceeds: stray file relocated → retry teardown proceeds
#   T-004  AC-006  pc2c-halt:               find error (non-path-absent) → HALT non-zero, exit code+stderr surfaced, worktree-remove NOT called
#   T-005  AC-002  file-at-path:            regular file at .factory → PC2b BLOCKED non-dir case; find NOT invoked; worktree-remove NOT called (BC-6.26.001 EC-008/T-6; F-S2104-P4-007)
#   T-006  AC-002  symlink-at-path:         symlink at .factory → PC2b BLOCKED regardless of target type; find NOT invoked; worktree-remove NOT called (BC-6.26.001 PC2b symlink; T-006; F-S2104-P5-011)
#   T-007  AC-008  devops-mandate:          agents/devops-engineer.md §Worktree Cleanup — preflight-verification mandate (F-S2104-P4-003)
#   T-008  AC-007(d)  6-surface-mandate:    6 ungated mandate surfaces — §G.1 delegation; anti-pattern absent (F-S2104-P4-009)
#   T-009  AC-009  adv-awareness: adversary.md + adversarial-review/SKILL.md §G.1/BC-6.26.001 teardown-preflight awareness (F-S2104-P4-002)

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  STEP_G_CLEANUP="$PLUGIN_ROOT/skills/deliver-story/steps/step-g-cleanup.md"
  SHARED_CONTEXT_MD="$PLUGIN_ROOT/skills/deliver-story/steps/_shared-context.md"
  SKILL_MD="$PLUGIN_ROOT/skills/deliver-story/SKILL.md"
  PER_STORY_DELIVERY_MD="$PLUGIN_ROOT/agents/orchestrator/per-story-delivery.md"
  WINNING_PLAYBOOK_MD="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  # 6 ungated mandate surfaces (F-S2104-P4-009)
  WORKTREE_MANAGE_SKILL_MD="$PLUGIN_ROOT/skills/worktree-manage/SKILL.md"
  CODE_DELIVERY_SKILL_MD="$PLUGIN_ROOT/skills/code-delivery/SKILL.md"
  FIX_PR_DELIVERY_SKILL_MD="$PLUGIN_ROOT/skills/fix-pr-delivery/SKILL.md"
  CODE_DELIVERY_WORKFLOW="$PLUGIN_ROOT/workflows/code-delivery.lobster"
  GREENFIELD_WORKFLOW="$PLUGIN_ROOT/workflows/greenfield.lobster"
  WORKTREE_PROTOCOL_MD="$PLUGIN_ROOT/rules/worktree-protocol.md"
  # F-S2104-P4-002 + F-S2104-P4-003 agent/specialist files
  ADVERSARY_MD="$PLUGIN_ROOT/agents/adversary.md"
  ADV_REVIEW_SKILL_MD="$PLUGIN_ROOT/skills/adversarial-review/SKILL.md"
  DEVOPS_ENGINEER_MD="$PLUGIN_ROOT/agents/devops-engineer.md"

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

# Extracts the Write Discipline child section from _shared-context.md.
# Bounded to the #### Write Discipline heading (child of ### Spec-Path Discipline);
# exits on the next NON-MATCHING #### sibling or ### parent or ## heading.
# NOTE: a second #### Write Discipline heading re-triggers rule 1 (`found=1; next`) rather
# than rule 2 (`found && /^#### / { exit }`), so both headings' content is captured; the
# anchor-uniqueness gate (_assert_write_discipline_anchor_unique) catches that case (count=2 → RED).
# F-S2104-P16-003(b): read-discipline content above #### Write Discipline (lines under
# ### Spec-Path Discipline but before #### Write Discipline) is outside this bounding section
# by construction — M-P16-B decoy inserted in that region is excluded automatically.
_extract_write_discipline_section() {
  awk '
    /^#### Write Discipline/ { found=1; next }
    found && /^#### / { exit }
    found && /^### / { exit }
    found && /^## / { exit }
    found { print }
  ' "$SHARED_CONTEXT_MD"
}

# Extracts the Write Discipline prohibition paragraph from _shared-context.md,
# bounded to the #### Write Discipline child heading (F-S2104-P16-003(b) fix).
# Start: line matching 'All.*\.factory.*artifact writes' (first line of Write Discipline body).
#   End: first blank line (paragraph boundary before **Load-bearing cases).
# Section-bounded: reads ONLY from within #### Write Discipline (not the whole §Spec-Path
# Discipline section — read-discipline content above #### Write Discipline is excluded).
# MUTANT: relocate prohibition paragraph outside #### Write Discipline → extractor finds nothing
#   → prohibition_block empty → absent-block gate fires → RED.
# ANCHOR UNIQUENESS: caller MUST first invoke _assert_write_discipline_anchor_unique to detect
#   decoy-insertion ambiguity (F-S2104-P16-003(a)) before relying on the extracted block.
# Used by F-S2104-P13-001 / F-S2104-P14-001 / F-S2104-P15-001 / F-S2104-P16-001 polarity gates.
_extract_write_discipline_prohibition_block() {
  _extract_write_discipline_section | awk '
    /All.*\.factory.*artifact writes/ { found=1 }
    found && /^$/ { exit }
    found { print }
  '
}

# Asserts the prohibition anchor matches exactly once within the #### Write Discipline section.
# F-S2104-P16-003(a): count must be exactly 1 — 0 or ≥2 matches → FAIL with explicit message.
# MUTANT (M-P16-B decoy OUTSIDE Write Discipline): insert decoy in read-discipline paragraphs
#   (under ### Spec-Path Discipline, above #### Write Discipline) — the extractor is bounded to
#   #### Write Discipline so the decoy is outside the domain; count = 1 → PASSES. This mutant
#   proves #### bounding neutralizes read-discipline decoys. A decoy INSIDE #### Write Discipline
#   is the test case below.
# MUTANT (in-section decoy): insert second paragraph starting 'All.*\.factory.*artifact writes'
#   inside #### Write Discipline before the normative paragraph → count = 2 → RED.
_assert_write_discipline_anchor_unique() {
  local write_discipline_section
  write_discipline_section="$(_extract_write_discipline_section)"
  local anchor_count
  anchor_count="$(printf '%s\n' "$write_discipline_section" | awk '/All.*\.factory.*artifact writes/ {count++} END {print count+0}')"
  if [ "$anchor_count" -ne 1 ]; then
    echo "DOC-PARITY FAIL [ambiguous anchor in #### Write Discipline (F-S2104-P16-003(a))]: found ${anchor_count} match(es) of anchor 'All.*\\.factory.*artifact writes' in the #### Write Discipline section (expected exactly 1); a decoy paragraph inserted before the normative paragraph creates a second anchor match detected here; a decoy outside #### Write Discipline (in read-discipline paragraphs under ### Spec-Path Discipline) is neutralized by the #### bounding constraint (BC-6.26.001 PC1; AC-001(a))"
    false
  fi
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

  # The BC discrimination chain (steps 1–3 are HARDCODED; step 4 is doc-derived via extraction):
  #   Step 1: [ ! -e ] → PC2a(a): path absent → proceed
  #   Step 2: [ -L ]  → PC2b: symlink at path → BLOCKED regardless of target type
  #   Step 3: [ ! -d ] → PC2b: non-directory non-symlink → BLOCKED
  #   Step 4: directory (no symlink) → run doc-extracted find

  # Step 1 (HARDCODED): path ABSENT — no stray files → proceed (return 0).
  # [ ! -e ] is FALSE for any occupied path (file, dir, symlink) — correctly gates on true absence.
  # [ ! -d ] would be TRUE for a regular file — wrong: authorizes teardown on stray content.
  # DOC-PARITY gate in T-002/T-005 independently verifies §G.1 uses the correct [ ! -e ] form
  # (F-S2104-P3-001 + F-S2104-P4-007a).
  if [ ! -e "${worktree_path}/.factory" ]; then
    printf 'worktree-remove-invoked\n' >> "$remove_log"
    return 0
  fi

  # Step 2 (HARDCODED): symlink at .factory path → PC2b BLOCKED (regardless of target type).
  # BC-6.26.001 PC2b: a symlink-to-dir satisfies [ -d ] (by dereferencing) but is still stray
  # shadow content — find MUST NOT be invoked. [ -L ] check precedes [ ! -d ] to catch it.
  # DOC-PARITY gate in T-006 independently verifies §G.1 carries the [ -L ] clause.
  if [ -L "${worktree_path}/.factory" ]; then
    printf 'PREFLIGHT BLOCKED: Symlink at %s — stray shadow content subject to rm-rf destruction; find NOT invoked; teardown HALTED (BC-6.26.001 PC2b symlink case).\n' "${worktree_path}/.factory"
    printf 'git worktree remove NOT executed.\n'
    return 1
  fi

  # Step 3 (HARDCODED): non-directory (non-symlink) inode at .factory path → PC2b BLOCKED.
  # BC-6.26.001 PC2b: a regular file at <worktree-path>/.factory is stray shadow content —
  # find MUST NOT be invoked on a non-directory path.
  # DOC-PARITY gate in T-005 independently verifies §G.1 documents non-directory→PC2b.
  if [ ! -d "${worktree_path}/.factory" ]; then
    printf 'PREFLIGHT BLOCKED: Non-directory inode at %s — stray shadow content subject to rm-rf destruction; find NOT invoked; teardown HALTED (BC-6.26.001 PC2b non-directory case).\n' "${worktree_path}/.factory"
    printf '  The -d test alone MUST NOT be used as the path-absence discriminator (BC-6.26.001 EC-008).\n'
    printf 'git worktree remove NOT executed.\n'
    return 1
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
    printf 'Story cleanup MUST NOT complete until a retry preflight returns a PASS result.\n'
    return 1
  fi

  # PC2a sub-case (b): find exits 0, empty output — no stray files, proceed with teardown.
  printf 'worktree-remove-invoked\n' >> "$remove_log"
  return 0
}

# ===========================================================================
# T-001 / AC-003 / PC2b: stray .factory/ file → PREFLIGHT BLOCKED (non-zero); worktree-remove NOT called
# BC-6.26.001 PC2b, Invariants 2, 5
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
    "step-g-cleanup.md §G.1: blanket 2>/dev/null suppression FORBIDDEN on preflight find command (BC-6.26.001 PC2; suppression removed to enable PC2c fail-closed detection)" \
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
  # "Story cleanup MUST NOT complete until a retry preflight returns a PASS result."
  _assert_doc_marker 'MUST NOT complete until.*retry|cleanup MUST NOT complete until.*empty|retry preflight returns an empty' \
    "step-g-cleanup.md §G.1: PC2b retry mandate — story cleanup MUST NOT complete until retry preflight returns a PASS result (BC-6.26.001 PC2b §3)" \
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
  # AC-001(c) second artifact: pr-review.md — must be named as a load-bearing case alongside DELIVERY.
  # Deleting 'pr-review.md' from Invariant 4 clause → this gate RED; restore → GREEN (mutant-proven).
  _assert_doc_marker 'pr-review\.md' \
    "_shared-context.md §Spec-Path Discipline: pr-review.md named as load-bearing case (BC-6.26.001 Invariant 4; AC-001(c))" \
    "$spec_path_section"
  # AC-001(c) third artifact: story-frontmatter files — must be named alongside DELIVERY and pr-review.md.
  _assert_doc_marker 'story-frontmatter' \
    "_shared-context.md §Spec-Path Discipline: story-frontmatter files named as load-bearing case (BC-6.26.001 Invariant 4; AC-001(c))" \
    "$spec_path_section"

  # --- DOC-PARITY §Spec-Path Discipline: AC-001(a) CWD-relative-path PROHIBITION (F-S2104-P12-003, F-S2104-P13-001, F-S2104-P14-001, F-S2104-P15-001, F-S2104-P16-001/002/003) ---
  # BC-6.26.001 PC1 core: the Write Discipline section must state that CWD-relative paths are
  # FORBIDDEN and that canonical absolute paths are MANDATED. Twelve independently mutant-proven
  # gates (pass-16 adds negation-transparency, block-wide polarity, sentence-scoped Gate 2 +
  # retirement-language guard, Gate 3 tightened + Gate 7 CWD-relative bullet polarity, and
  # anchor-uniqueness bounded to #### Write Discipline; pass-17 adds whole-section domain,
  # HTML-comment absence, conditional-scoping gate, prohibition-token PW-B, tightened Gate 2a,
  # widened Gate 2b + adversative-connective gate, and canonical-target gate):
  #   (1) Paragraph-level extractor (_extract_write_discipline_prohibition_block;
  #       #### Write Discipline → normative prohibition paragraph, anchor: 'All `.factory/**`
  #       artifact writes…') + MANDATE POLARITY (SENTENCE-SCOPED + NEGATION-TRANSPARENT,
  #       F-S2104-P15-001 / F-S2104-P16-001(a)):
  #       - ANCHOR UNIQUENESS (F-S2104-P16-003(a)): _assert_write_discipline_anchor_unique
  #         counts matches within #### Write Discipline; decoy-insertion → count=2 → RED.
  #       - HTML-COMMENT ABSENCE (F-S2104-P17-001(b)): no <!-- in #### Write Discipline;
  #         M-P17-H moves normative mandate inside <!-- --> → gate fires → RED.
  #       - MANDATE SENTENCE: extract sentence containing 'artifact writes' from joined block
  #         (abbreviation-protected splitter for cf./i.e./e.g. — F-S2104-P16-001 M-P16-C2
  #         hardening: cf. in M-P16-C2 false-splits mandate sentence, masking CWD-relative).
  #       - Gate 1(a): MUST[[:space:]]+use[[:space:]]+canonical absolute (zero-DoF — 'MUST NOT
  #         use' cannot satisfy; prior MUST[^.]*use[^.]*canonical passed M-P16-A).
  #       - Gate 1(b) negation-transparent (F-S2104-P16-001(a)): mandate sentence must NOT
  #         match MUST.*(NOT|not|never).*canonical absolute — M-P16-A "MUST NOT use canonical
  #         absolute" → RED.
  #       - Gate 1(c) negative: mandate sentence must NOT contain prohibited-subject forms
  #         (CWD-relative|worktree-relative|relative paths?).
  #       - Gate 1(d) conditional-scoping (F-S2104-P17-002(c)): mandate sentence must NOT
  #         contain 'only when/where/if', 'when the target', or 'unless' — BC-6.26.001
  #         Invariant 1 is categorical; M-P17-C scopes to 'when target outside worktree' → RED.
  #       MUTANT (a): delete prohibition block → empty → RED.
  #       MUTANT (b): M-P15-A/M-P16-A "MUST NOT use canonical..." → Gate 1(a)+(b) fail → RED.
  #       Restore (c): mandate sentence "MUST use canonical absolute paths" → GREEN.
  #   (PW-B) SECTION-WIDE SENTENCE POLARITY (F-S2104-P16-001(b)/F-S2104-P17-002): over
  #       write_discipline_prose_nosplit (whole #### Write Discipline section, fenced code
  #       excluded). For every sentence containing a prohibited-target form, that sentence MUST
  #       carry a prohibition token — directive-token whitelist dropped (F-S2104-P17-002(a)).
  #       prohibited-target: CWD-relative|worktree-relative|relative paths?|story-worktree CWD|
  #         worktree's shadow|worktree CWD|shadow subtree|worktree-local|in-worktree
  #       prohibition: FORBIDDEN|forbidden|MUST NOT|prohibited|never|forbid
  #       M-P17-A S1: "Writers MUST anchor every write to the story worktree CWD" — no prohibition
  #         token → RED. M-P17-C S2: "CWD-relative paths are the required form" → RED.
  #   (2a) Gate 2 SENTENCE-SCOPED TIGHTENED (F-S2104-P16-001(c)/F-S2104-P17-002(d)): at least
  #       one sentence must match (CWD-relative|worktree-relative)[^.]*FORBIDDEN — requires
  #       the prohibited-subject form AND FORBIDDEN co-occur within the sentence boundary
  #       defined by the sentence-splitting regex. M-P17-C: FORBIDDEN is in a traversal sentence,
  #       not the CWD-relative sentence → Gate 2a fires → RED.
  #   (2b) NULLIFICATION CLASS + ADVERSATIVE CONNECTIVE (F-S2104-P16-001(c)/F-S2104-P17-003):
  #       over write_discipline_prose_nosplit. (b) block must NOT contain nullification language
  #       (formerly|retired|rescinded|superseded|relaxed|lifted|withdrawn|rescind|no longer|
  #       not longer|waived|exempt|obsolete|deprecated|does not apply|overridden|historical only);
  #       M-P17-D 'rescinded and superseded' → RED; M-P17-F 'no longer' across line break → RED.
  #       (c) FORBIDDEN sentence must NOT contain adversative connective (but/however/except that/
  #       though); M-P17-F "FORBIDDEN under the initial reading, but that is no longer" → RED.
  #   (3) Gate 3 TIGHTENED (F-S2104-P16-002): **Forbidden:** + file_path="\.factory/ + relative
  #       path on same line. M-P16-D swaps labels — traversal bullet w/ file_path="../../.factory/"
  #       does not match file_path="\.factory/" → tightened Gate 3 fails → RED.
  #   (4) NEGATIVE (WHOLE-SECTION, F-S2104-P15-001/F-S2104-P17-001): over write_discipline_prose.
  #       NO sentence where 'absolute' co-occurs with 'FORBIDDEN'.
  #   (5) NEGATIVE (WHOLE-SECTION, F-S2104-P15-001/F-S2104-P17-001): over write_discipline_prose.
  #       NO sentence where 'MUST' co-occurs with a prohibited-subject form. Catches
  #       M-P14-A/M-P14R-A/M-P15-A/M-P16-A and M-P17-A.
  #   (6) Gate 6 (two-part polarity for ../ traversal form — F-S2104-P14R-003/F-S2104-P15-002):
  #       (a) POSITIVE: **Forbidden:** + ../ must exist; (b) — replaced by canonical-target gate.
  #   (7) Gate 7 CWD-RELATIVE BULLET POLARITY (F-S2104-P16-002):
  #       (a) POSITIVE: **Forbidden:** + file_path="\.factory/ must exist;
  #       (b) — replaced by canonical-target gate.
  #   (canonical-target) NEGATIVE (F-S2104-P17-004): no **Correct:** bullet with file_path= may
  #       fail the canonical predicate (file_path=["']?($CANONICAL_FACTORY_ROOT|/)). Replaces
  #       surface-form-specific Gates 6(b) and 7(b). M-P17-G "./.factory/..." → RED;
  #       M-P15-B traversal "../../.factory/" → RED; M-P16-D CWD-relative ".factory/" → RED.
  # All twelve gates survive independently.

  # Anchor uniqueness gate: #### Write Discipline must have exactly one prohibition anchor.
  _assert_write_discipline_anchor_unique

  local prohibition_block
  prohibition_block="$(_extract_write_discipline_prohibition_block)"

  if [ -z "$prohibition_block" ]; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block absent]: _shared-context.md Write Discipline prohibition paragraph ('All .factory/** artifact writes...MUST use...absolute paths...CWD-relative...FORBIDDEN') not found — block deleted or heading changed (BC-6.26.001 PC1; AC-001(a); F-S2104-P13-001)"
    false
  fi

  # Whole-section domain for section-wide negative gates (F-S2104-P17-001(a)).
  # Extends domain beyond the first paragraph: Gates PW-B, 2b, 4, 5 now evaluate every sentence
  # of the whole #### Write Discipline section, not only the normative prohibition paragraph.
  # M-P17-A inserts a 'Story-worktree exception' paragraph AFTER the prohibition paragraph —
  # the prohibition paragraph itself is unchanged, so paragraph-scoped gates pass while the
  # section-wide PW-B catches the harmful second paragraph.
  local write_discipline_section
  write_discipline_section="$(_extract_write_discipline_section)"

  # Gate: no HTML comment spans in #### Write Discipline (F-S2104-P17-001(b)).
  # M-P17-H replaces the normative prohibition paragraph with an HTML comment containing the
  # compliant text, then adds a visible sentence directing writes to the worktree CWD — Gates
  # 1(a) and 2a match the comment text, the rendered instruction is the issue #523 write.
  # All normative text in #### Write Discipline must be rendered (not comment-hidden).
  if printf '%s\n' "$write_discipline_section" | grep -qE '<!--'; then
    echo "DOC-PARITY FAIL [write-discipline comment-hidden normative text (F-S2104-P17-001(b))]: the #### Write Discipline section contains an HTML comment span (<!--) — normative mandate or prohibition text hidden inside <!-- --> is not rendered by Markdown renderers, yet positive gates (1(a), 2a) can match it; M-P17-H moves the compliant mandate inside <!-- --> and makes the only visible instruction a CWD-relative write; all normative text must be visible (BC-6.26.001 PC1; AC-001(a))"
    false
  fi

  # Build section-wide prose domain: strip fenced code blocks, reflow, abbreviation-protect.
  # Fenced code blocks are denoted by ``` fences; excluded to avoid false-positive gate fires on
  # code examples that legitimately mention relative paths in a Forbidden example context.
  # Match indented fences too (/^[[:space:]]*```/): _shared-context.md uses 2-space-indented
  # fences for the Canonical root determination bash block (lines 93/104); bare /^```/ would
  # miss them and include the bash code block in the prose domain.
  local write_discipline_prose
  write_discipline_prose="$(printf '%s\n' "$write_discipline_section" | \
    awk '/^[[:space:]]*```/{in_fence=!in_fence; next} !in_fence{print}' | tr '\n' ' ')"
  local write_discipline_prose_nosplit
  write_discipline_prose_nosplit="$(printf '%s\n' "$write_discipline_prose" | \
    sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g')"

  # Reflow the prohibition block to a single joined line for sentence-scoped evaluation.
  # Gates 1(a/b/c/d) and 2a use joined_block_nosplit scoped to the prohibition paragraph only,
  # since they are positive existence assertions on the mandate/prohibition sentence.
  # Gates PW-B, 2b, 4, 5 use write_discipline_prose_nosplit (whole section, fenced code excluded).
  # (F-S2104-P15-001: per-physical-line predicates over soft-wrapped paragraphs are inadmissible).
  local joined_block
  joined_block="$(printf '%s\n' "$prohibition_block" | tr '\n' ' ')"

  # Abbreviation-protected joined block for sentence-splitting (F-S2104-P16-001 M-P16-C2 hardening).
  # 'cf. ', 'i.e. ', 'e.g. ' contain dots that create false sentence boundaries when split on '. '.
  # M-P16-C2 uses 'cf. CWD-relative paths' — without protection, cf. splits the mandate sentence
  # before the CWD-relative token, masking the bypass from Gates 1(c) and 5.
  # With protection: mandate sentence retains 'cf_ABBREV_ CWD-relative paths' → Gate 1(c) fires.
  local joined_block_nosplit
  joined_block_nosplit="$(printf '%s\n' "$joined_block" | \
    sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g')"

  # Gate 1: affirmative mandate polarity — sentence-scoped, negation-transparent
  # (F-S2104-P15-001 / F-S2104-P14R-001(a) / F-S2104-P16-001(a))
  # Reflow paragraph, sentence-split on '. ' boundaries (abbreviation-protected), extract the
  # normative mandate sentence (the sentence containing 'artifact writes'). That sentence MUST:
  #   (a) match MUST[[:space:]]+use[[:space:]]+canonical absolute (zero-DoF: 'MUST NOT use' fails)
  #   (b) NOT match MUST.*(NOT|not|never).*canonical absolute (negation-transparent paired gate)
  #   (c) NOT match CWD-relative|worktree-relative|relative paths? (prohibited subjects absent)
  # Rewrap mutant: same paragraph rewrapped at different word boundaries → sentences identical
  #   after joining and splitting; wrap-position is not load-bearing → GREEN.
  local mandate_sentence
  mandate_sentence="$(printf '%s\n' "$joined_block_nosplit" | \
    sed 's/\. /\n/g' | grep 'artifact writes' | head -1)"
  if [ -z "$mandate_sentence" ]; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block mandate-sentence absent]: the normative mandate sentence containing 'artifact writes' was not found after sentence-splitting the joined prohibition block (split on '. '); block may be missing or the sentence structure changed (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001)"
    false
  fi
  # Gate 1(a): zero-DoF MUST use canonical absolute — 'MUST NOT use canonical' cannot satisfy
  # (F-S2104-P16-001(a): prior MUST[^.]*use[^.]*canonical passed M-P16-A because [^.]* spans NOT)
  printf '%s\n' "$mandate_sentence" | grep -qE 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute' || {
    echo "DOC-PARITY FAIL [write-discipline prohibition block affirmative-mandate (sentence-scoped, zero-DoF, F-S2104-P16-001(a))]: the mandate sentence (containing 'artifact writes') must contain 'MUST use canonical absolute' (zero-DoF: no tokens between MUST and use) — the prior MUST[^.]*use[^.]*canonical pattern passed M-P16-A 'MUST NOT use canonical absolute' because [^.]* spans the negation token; this tightening closes that bypass (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001 / F-S2104-P14R-001)"
    false
  }
  # Gate 1(b): negation-transparency paired gate — mandate sentence must NOT contain MUST + NOT/not/never + canonical absolute
  # M-P16-A: "MUST NOT use canonical absolute paths" → MUST[^.]*(NOT|not|never)[^.]*canonical → RED.
  if printf '%s\n' "$mandate_sentence" | grep -qE 'MUST[^.]*(NOT|not|never)[^.]*canonical[[:space:]]+absolute'; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block negation-transparency (Gate 1(b), F-S2104-P16-001(a))]: the mandate sentence contains a negated mandate ('MUST NOT/not/never...canonical absolute') — M-P16-A rewrites the mandate as 'MUST NOT use canonical absolute paths'; a polarity predicate using [^.]* between MUST and use spans the negation token and passes M-P16-A; this paired negative gate closes that bypass (BC-6.26.001 PC1; AC-001(a))"
    false
  fi
  # Gate 1(c): mandate sentence must NOT contain any prohibited-subject form
  # (catches M-P15-A: "MUST use CWD-relative paths"; catches M-P14-A/M-P14R-A;
  # with abbreviation-protected splitter, M-P16-C2 'cf_ABBREV_ CWD-relative paths' in the
  # mandate sentence also triggers this gate).
  if printf '%s\n' "$mandate_sentence" | grep -qE 'CWD-relative|worktree-relative|relative[[:space:]]+paths?'; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block MUST-relative-polarity (mandate sentence, Gate 1(c))]: the mandate sentence contains a prohibited-subject form (CWD-relative, worktree-relative, or relative paths) — in the correct text the mandate sentence states MUST use canonical absolute paths; M-P15-A ('MUST use CWD-relative paths') triggers this gate; with abbreviation-protected splitter, M-P16-C2 'cf. CWD-relative paths' in the mandate sentence also triggers it; POLICY-13 syntactic-form class alternation (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001 / F-S2104-P14R-001)"
    false
  fi
  # Gate 1(d): mandate sentence must NOT contain conditional scoping (F-S2104-P17-002(c)).
  # BC-6.26.001 Invariant 1: the categorical prohibition applies to ALL .factory/** writes from
  # story worktrees — no sub-case or conditionality is permitted.
  # M-P17-C: "MUST use canonical absolute paths when the target lies outside the story worktree"
  # — 'when the target' triggers this gate; the mandate is scoped to a sub-case, exempting all
  # in-worktree .factory/** writes from the canonical-absolute requirement.
  if printf '%s\n' "$mandate_sentence" | grep -qE 'only[[:space:]]+(when|where|if)|when[[:space:]]+the[[:space:]]+target|unless'; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block conditional-mandate-scoping (Gate 1(d), F-S2104-P17-002(c))]: the mandate sentence contains conditional scoping of the MUST-use mandate ('only when/where/if', 'when the target', or 'unless') — BC-6.26.001 Invariant 1 declares CWD-relative paths 'categorically forbidden'; M-P17-C scopes the mandate to 'when the target lies outside the story worktree', narrowing the categorical prohibition to a sub-case and exempting in-worktree .factory/** writes entirely (BC-6.26.001 PC1; AC-001(a))"
    false
  fi

  # Gate PW-B (SECTION-WIDE SENTENCE POLARITY, F-S2104-P16-001(b) strengthened F-S2104-P17-002):
  # Evaluates the whole #### Write Discipline section (write_discipline_prose_nosplit, fenced code
  # excluded — F-S2104-P17-001(a)) rather than only the prohibition paragraph.
  # For every sentence containing a prohibited-target form, that sentence MUST carry a prohibition
  # token. Directive-token whitelist dropped (F-S2104-P17-002(a)): any sentence mentioning a
  # prohibited target in prose that is not marking it prohibited is itself the defect.
  # prohibited-target: CWD-relative|worktree-relative|relative[[:space:]]+paths?|
  #                    story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|
  #                    worktree's[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|
  #                    worktree-local
  # NOTE: 'in-worktree' excluded from prohibited-target — 'in-worktree' is a substring of
  #   '<main-worktree-path>' (template placeholder in Canonical root determination section), causing
  #   false-positive matches on that section's prose. M-P17-C is caught by 'CWD-relative' and
  #   'worktree's shadow'; 'in-worktree' is redundant for all recorded mutants.
  # prohibition: FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid
  # NOTE: 'Forbidden' (capital-F) added to handle **Forbidden:** bullet labels in the section.
  # M-P17-A S1: "Writers MUST anchor every .factory/** artifact write to the story worktree CWD"
  #   — 'story worktree CWD' matches story[[:space:]]+worktree[[:space:]]+CWD, no prohibition → RED.
  # M-P17-C S2: "For in-worktree ledgers, CWD-relative paths are the required form, and they
  #   land in the story worktree's shadow .factory/ subtree" — 'CWD-relative' and 'worktree's
  #   shadow' present, no prohibition token → RED.
  # Correct text S2: "CWD-relative paths...are FORBIDDEN" — prohibited-target + FORBIDDEN → PASSES.
  # Correct bullet "**Forbidden:** ... (relative path ...)" — 'relative path' present, 'Forbidden'
  #   label present → sentence excluded from violations by grep -Ev → PASSES.
  local polarity_violations
  polarity_violations="$(printf '%s\n' "$write_discipline_prose_nosplit" | sed 's/\. /\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|worktree-local' | \
    grep -Ev 'FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid' || true)"
  if [ -n "$polarity_violations" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide sentence polarity (Gate PW-B, F-S2104-P16-001(b)/F-S2104-P17-002)]: a sentence in the Write Discipline section contains a prohibited-target form (CWD-relative|worktree-relative|relative paths?|story-worktree CWD|story worktree CWD|worktree's shadow|worktree CWD|shadow subtree|worktree-local) without a prohibition token (FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid) — M-P17-A S1 'Writers MUST anchor every write to the story worktree CWD' carries no prohibition token; M-P17-C S2 'CWD-relative paths are the required form, and they land in the story worktree's shadow subtree' carries no prohibition token (BC-6.26.001 PC1; AC-001(a))"
    printf '%s\n' "$polarity_violations"
    false
  fi

  # Gate 2a (SENTENCE-SCOPED POSITIVE, F-S2104-P16-001(c)):
  # At least one sentence must contain a prohibited-subject form AND FORBIDDEN on the same sentence.
  # Correct text S2: "CWD-relative paths...are FORBIDDEN" → CWD-relative + FORBIDDEN → PASSES.
  # M-P16-C2: "Duplicating a ledger onto the main checkout is FORBIDDEN" (no prohibited-subject
  # in the FORBIDDEN sentence); CWD-relative is in a different sentence → Gate 2a FAILS → RED.
  # This catches M-P16-C2 regardless of the abbreviation-protected splitter (because the
  # prohibited-subject form is absent from the FORBIDDEN sentence in M-P16-C2).
  # Gate 2a (SENTENCE-SCOPED POSITIVE TIGHTENED, F-S2104-P16-001(c)/F-S2104-P17-002(d)):
  # At least one sentence must contain a prohibited-subject form (CWD-relative|worktree-relative)
  # AND FORBIDDEN co-occurring within the same sentence.
  # Tightened (F-S2104-P17-002(d)): prohibited-subject narrowed to CWD-relative|worktree-relative
  # (dropping relative[[:space:]]+path). Reason: M-P17-C's traversal FORBIDDEN sentence is
  # "...traversal is FORBIDDEN for relative paths of that kind" — it contains 'relative paths'
  # (satisfying the OLD broader prohibited-subject pattern) AND FORBIDDEN, so the OLD two-stage
  # gate passes M-P17-C at 9/9. Narrowing to CWD-relative|worktree-relative excludes that
  # traversal sentence; only the sentence directly about CWD-relative paths can satisfy Gate 2a.
  # Co-occurrence is sentence-scoped: after sentence-splitting on '. ', each output line is one
  # sentence; two-stage grep is equivalent to requiring both forms on the same sentence-line.
  # Correct text S2: "CWD-relative paths...are FORBIDDEN" → CWD-relative + FORBIDDEN → PASSES.
  # M-P17-C: FORBIDDEN is in the traversal sentence, CWD-relative is in a different sentence → RED.
  # M-P16-C2: "Duplicating...is FORBIDDEN" without CWD-relative in the same sentence → RED.
  local forbidden_prohibited_sentences
  forbidden_prohibited_sentences="$(printf '%s\n' "$joined_block_nosplit" | sed 's/\. /\n/g' | \
    grep -E 'CWD-relative|worktree-relative' | \
    grep -E 'FORBIDDEN|forbidden' || true)"
  if [ -z "$forbidden_prohibited_sentences" ]; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block sentence-scoped FORBIDDEN co-occurrence (Gate 2a, F-S2104-P16-001(c)/F-S2104-P17-002(d))]: no sentence in the Write Discipline prohibition paragraph contains both a prohibited-subject form (CWD-relative or worktree-relative) and FORBIDDEN in the same sentence — the correct text has S2 'CWD-relative paths...are FORBIDDEN'; M-P17-C places FORBIDDEN in a traversal sentence ('traversal is FORBIDDEN for relative paths') while CWD-relative is in a different sentence, so the narrowed gate fires; M-P16-C2 places FORBIDDEN in a separate sentence from CWD-relative (BC-6.26.001 PC1; AC-001(a))"
    false
  fi

  # Gate 2b (NULLIFICATION CLASS NEGATIVE, F-S2104-P16-001(c) strengthened F-S2104-P17-003):
  # Evaluates the whole #### Write Discipline section (write_discipline_prose_nosplit) — extended
  # from prohibition_block per F-S2104-P17-001(a) to catch nullification outside the first paragraph.
  # Domain is joined+sentence-split (rewrap-invariant per POLICY 13 NORMALIZED-DOMAIN MANDATE) —
  # M-P17-F wraps 'no longer' across a soft line break; per-physical-line grep misses it.
  # (a) Synonyms: must NOT contain nullification language from the nullification class;
  #     M-P17-D 'rescinded and superseded' → RED; M-P17-F 'no longer' across line break → RED.
  # (b) Adversative connective: FORBIDDEN sentence must NOT carry 'but/however/except that/though';
  #     M-P17-F "FORBIDDEN under the initial reading, but that is no longer" → (b) fires → RED.
  # M-P16-A: 'formerly' and 'retired' → (a) fires → RED (backward-compat).
  local retirement_language
  retirement_language="$(printf '%s\n' "$write_discipline_prose_nosplit" | sed 's/\. /\n/g' | \
    grep -E 'formerly|retired|rescinded|superseded|relaxed|lifted|withdrawn|rescind|no[[:space:]]+longer|not[[:space:]]+longer|waived|exempt|obsolete|deprecated|does[[:space:]]+not[[:space:]]+apply|overridden|historical[[:space:]]+only' || true)"
  if [ -n "$retirement_language" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide nullification language (Gate 2b(a), F-S2104-P16-001(c)/F-S2104-P17-003)]: the Write Discipline section contains constraint-nullification language — M-P17-D 'rescinded and superseded', M-P17-F 'no longer' (split across line break), M-P16-A 'formerly...retired'; any nullification synonym nullifies the prohibition while preserving the FORBIDDEN token for Gate 2a (BC-6.26.001 PC1; AC-001(a))"
    printf '%s\n' "$retirement_language"
    false
  fi
  # Gate 2b(c): FORBIDDEN sentence must NOT contain an adversative connective (F-S2104-P17-003(c)).
  # An adversative connective attaching to the FORBIDDEN sentence expresses nullification regardless
  # of which nullification verb is used, closing the synonym-list bypass.
  # M-P17-F: "...are FORBIDDEN under the initial reading, but that is no longer the operative rule"
  # — 'but ' triggers this gate → RED. Correct text uses em-dash (—), not adversative → GREEN.
  local forbidden_sentences_with_adversative
  forbidden_sentences_with_adversative="$(printf '%s\n' "$joined_block_nosplit" | sed 's/\. /\n/g' | \
    grep -E 'FORBIDDEN|forbidden' | \
    grep -E 'but[[:space:]]|however|except[[:space:]]+that|though[[:space:]]' || true)"
  if [ -n "$forbidden_sentences_with_adversative" ]; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block adversative-connective on FORBIDDEN sentence (Gate 2b(c), F-S2104-P17-003(c))]: a sentence containing FORBIDDEN is qualified by an adversative connective (but/however/except that/though) — M-P17-F 'are FORBIDDEN under the initial reading, but that is no longer the operative rule' exploits 'but ' to nullify the constraint while preserving the FORBIDDEN token; the correct text's FORBIDDEN sentence uses em-dash (—) not adversative and passes this gate (BC-6.26.001 PC1; AC-001(a))"
    printf '%s\n' "$forbidden_sentences_with_adversative"
    false
  fi

  # Gate 4 (NEGATIVE, section-scoped; F-S2104-P14-001 / F-S2104-P15-001 / F-S2104-P17-001(a)):
  # Extended to whole #### Write Discipline section (write_discipline_prose_nosplit). No sentence
  # in the section may contain both 'absolute' and 'FORBIDDEN'. In the correct text:
  #   S1 "...MUST use canonical absolute paths..." — 'absolute' present, 'FORBIDDEN' absent → PASSES.
  #   S2 "CWD-relative paths...are FORBIDDEN..." — 'FORBIDDEN' present, 'absolute' absent → PASSES.
  # M-P15-A S3: "Canonical absolute artifact-write paths...are FORBIDDEN" — has both → RED.
  # Per-sentence evaluation (not per-line) per F-S2104-P15-001; write_discipline_prose_nosplit for splits.
  local forbidden_absolute_sentences
  forbidden_absolute_sentences="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    sed 's/\. /\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)' || true)"
  if [ -n "$forbidden_absolute_sentences" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide FORBIDDEN-polarity (sentence-scoped; F-S2104-P17-001(a))]: a sentence in the Write Discipline section contains both 'absolute' and 'FORBIDDEN' — in the correct text absolute paths are MANDATED (MUST), not the FORBIDDEN subject; M-P15-A S3 'Canonical absolute artifact-write paths...are FORBIDDEN' triggers this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14-001 / F-S2104-P15-001)"
    false
  fi

  # Gate 5 (NEGATIVE, section-scoped; F-S2104-P14R-001(b) / F-S2104-P15-001 / F-S2104-P17-001(a)):
  # Extended to whole #### Write Discipline section (write_discipline_prose_nosplit). No sentence
  # in the section may contain both 'MUST' and a prohibited-subject form.
  # Correct text: S1 "...MUST use canonical absolute paths..." — MUST present, no prohibited form;
  #   S2 "CWD-relative paths...are FORBIDDEN..." — prohibited form present, no MUST → PASSES.
  # M-P17-A S1: "Writers MUST anchor every .factory/** artifact write to the story worktree CWD"
  #   — second paragraph in the section; MUST + story worktree CWD → Gate 5 fires → RED.
  # M-P15-A S1: "...MUST use CWD-relative paths..." — MUST+CWD-relative → RED.
  # M-P14-A: "MUST use CWD-relative paths" → RED.
  # M-P14R-A: "MUST use relative paths" → relative path form → RED.
  # Synonym vector (worktree-relative): "MUST use worktree-relative paths" → RED.
  # M-P16-C2 + abbreviation-protected splitter → mandate sentence retains cf_ABBREV_ CWD-relative
  #   → MUST + relative path in mandate sentence → Gate 5 fires → RED (defense-in-depth).
  # POLICY-13 syntactic-form class alternation; write_discipline_prose_nosplit for splits.
  local must_relative_sentences
  must_relative_sentences="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    sed 's/\. /\n/g' | grep -E 'MUST' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' || true)"
  if [ -n "$must_relative_sentences" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide MUST-relative-polarity (sentence-scoped; F-S2104-P17-001(a))]: a sentence in the Write Discipline section contains both 'MUST' and a prohibited-subject form (CWD-relative, worktree-relative, or relative path) — in the correct text MUST mandates canonical absolute paths; M-P17-A S1 'MUST anchor...to the story worktree CWD' triggers this gate on the second paragraph; M-P15-A/M-P14-A/M-P14R-A/M-P16-C2 trigger it on the prohibition paragraph; POLICY-13 alternation (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-001 / F-S2104-P15-001)"
    false
  fi

  # Gate 3 (TIGHTENED, F-S2104-P16-002): **Forbidden:** + file_path="\.factory/ + relative path
  # on same line. Scopes the relative-path conjunction to lines referencing the CWD-relative form.
  # A **Forbidden:** bullet with file_path="../../.factory/" and 'relative path traversal' satisfies
  # the old Gate 3 (**Forbidden:** + relative path) but NOT the tightened form: the pattern
  # file_path="\.factory/ requires the dot-factory slash to follow the opening quote directly,
  # without ../../. M-P16-D swaps the CWD-relative bullet label to **Correct:** — the only
  # remaining **Forbidden:** bullet has file_path="../../.factory/" → tightened Gate 3 fails → RED.
  # DELETION MUTANT: delete the **Forbidden:** CWD-relative bullet → no line satisfies all three → RED.
  local forbidden_cwd_relative_bullet
  forbidden_cwd_relative_bullet="$(printf '%s\n' "$spec_path_section" | \
    grep -E '\*\*Forbidden:\*\*' | grep -E 'file_path="\.factory/' | grep -E 'relative path' || true)"
  if [ -z "$forbidden_cwd_relative_bullet" ]; then
    echo "DOC-PARITY FAIL [write-discipline Gate 3 tightened: **Forbidden:** + file_path=\".factory/\" + relative path on same line (F-S2104-P16-002)]: §Spec-Path Discipline must have a **Forbidden:** bullet where file_path starts with \".factory/\" and the label 'relative path' is present on the same line — the CWD-relative bullet documents the write-path anti-pattern; M-P16-D swaps the labels making it **Correct:** → RED; traversal bullet with file_path=\"../../.factory/\" does not satisfy file_path=\".factory/\" (BC-6.26.001 PC1; AC-001(a); F-S2104-P12-003 / F-S2104-P16-002)"
    false
  fi

  # Gate 6 (two-part polarity; F-S2104-P14R-003 / F-S2104-P15-002): traversal-form **Forbidden:**
  # bullet — §Spec-Path Discipline must have a **Forbidden:** bullet with ../ on the same line,
  # AND no line containing ../ may be a **Correct:** bullet.
  # One-line bullet predicate is admissible here: each **Forbidden:**/**Correct:** example is a
  # single-line list item, not a soft-wrapped paragraph; same-line co-occurrence is a stable
  # structural property of the bullet format, not a volatile line-wrap artifact.
  # Gate 6(a) POSITIVE: some line MUST match **Forbidden:** AND contain ../
  #     DELETION MUTANT: delete the third Forbidden bullet → no **Forbidden:** + ../ line → RED.
  # Gate 6(b) — REPLACED BY canonical-target gate below (F-S2104-P17-004).
  # Unmodified text: (a) PASSES (third Forbidden bullet has ../).
  printf '%s\n' "$spec_path_section" | grep -qE '\*\*Forbidden:\*\*.*\.\./|\.\./.*\*\*Forbidden:\*\*' || {
    echo "DOC-PARITY FAIL [write-discipline §Spec-Path Discipline traversal-Forbidden bullet absent (Gate 6(a))]: a line in §Spec-Path Discipline must match **Forbidden:** AND contain ../ on the same line — the third Forbidden bullet (relative traversal ../../.factory/...) documents path-traversal writes; deleting that bullet fails this gate; M-P15-B (traversal Correct: label swap) is caught by the canonical-target gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-003 / F-S2104-P15-002)"
    false
  }

  # Gate 7 (CWD-RELATIVE BULLET POLARITY, two-part; F-S2104-P16-002):
  # (a) POSITIVE: some line in §Spec-Path Discipline must match **Forbidden:** AND contain
  #     file_path="\.factory/" (the CWD-relative pattern — no leading ../../ or variable).
  #     DELETION MUTANT: delete the CWD-relative **Forbidden:** bullet → no such line → RED.
  # (b) — REPLACED BY canonical-target gate below (F-S2104-P17-004).
  # Unmodified text: (a) PASSES (line has **Forbidden:** + file_path=".factory/").
  printf '%s\n' "$spec_path_section" | grep -qE '\*\*Forbidden:\*\*.*file_path="\.factory/|file_path="\.factory/.*\*\*Forbidden:\*\*' || {
    echo "DOC-PARITY FAIL [write-discipline Gate 7(a) CWD-relative Forbidden bullet absent (F-S2104-P16-002)]: §Spec-Path Discipline must have a **Forbidden:** bullet where file_path starts with \".factory/\" (the CWD-relative write pattern) — deleting this bullet or swapping its label to **Correct:** (M-P16-D) fails this gate; M-P16-D is also caught by the canonical-target gate (BC-6.26.001 PC1; AC-001(a))"
    false
  }

  # Gate canonical-target (NEGATIVE, F-S2104-P17-004(b)):
  # No **Correct:** bullet in §Spec-Path Discipline may contain a non-canonical file_path target.
  # Canonical-target predicate: file_path= followed by optional " or ' then ($CANONICAL_FACTORY_ROOT|/)
  # (variable-rooted or absolute-path-rooted). Replaces surface-form-specific Gates 6(b) and 7(b);
  # generalises to any relative rendering regardless of quoting style.
  # M-P17-G: '**Correct:** Write(file_path="./.factory/…")' — "./" fails predicate → RED.
  # Single-quoted mutant: '**Correct:** Write(file_path='"'"'.factory/…'"'"')' — ".factory/" fails → RED.
  # Bare-unquoted mutant: '**Correct:** Write(file_path=.factory/…)' — ".factory/" fails → RED.
  # M-P15-B (traversal Correct:): file_path="../../.factory/…" fails predicate → RED.
  # M-P16-D (CWD-relative Correct:): file_path=".factory/…" fails predicate → RED.
  # Control (GREEN): file_path="$CANONICAL_FACTORY_ROOT/.factory/…" → predicate satisfied → PASSES.
  local noncanonical_correct_bullets
  noncanonical_correct_bullets="$(printf '%s\n' "$spec_path_section" | \
    grep -E '\*\*Correct:\*\*' | grep -E 'file_path=' | \
    grep -Ev 'file_path=["'"'"']?(\$CANONICAL_FACTORY_ROOT|/)' || true)"
  if [ -n "$noncanonical_correct_bullets" ]; then
    echo "DOC-PARITY FAIL [write-discipline Gate canonical-target: **Correct:** bullet with non-canonical file_path target (F-S2104-P17-004)]: a **Correct:** bullet in §Spec-Path Discipline contains a file_path= target that fails the canonical-target predicate (file_path=[\"']?(\$CANONICAL_FACTORY_ROOT|/)) — every **Correct:** example must show a variable-rooted (\$CANONICAL_FACTORY_ROOT) or absolute-path-rooted target; M-P17-G adds file_path=\"./.factory/…\" (relative with ./), M-P15-B has file_path=\"../../.factory/…\" (traversal), M-P16-D has file_path=\".factory/…\" (bare CWD-relative), all caught here (BC-6.26.001 PC1; AC-001(a))"
    printf '%s\n' "$noncanonical_correct_bullets"
    false
  fi

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

  # --- DOC-PARITY primary paths: SKILL.md Step 8 (F-S2104-P1-001a, F-S2104-P5-007) ---
  # was RED at 60f0d2d6 until implementer adds §G.1/step-g-cleanup reference to SKILL.md Step 8 dispatch at 43ea70ba.
  # Strengthened (F-S2104-P5-007): bare 'preflight' token removed — requires actual §G.1/step-g-cleanup ref.
  # Strengthened (F-S2104-P11-003): requires .md-qualified form (step-g-cleanup.md §G.1); bare step-g-cleanup §G.1 without .md insufficient.
  local skill_step8_section
  skill_step8_section="$(_extract_skill_step8_section)"
  _assert_doc_marker 'step-g-cleanup\.md.*§G\.1|§G\.1.*step-g-cleanup\.md' \
    "SKILL.md Step 8: §G.1 ref must use filename-qualified form (step-g-cleanup.md §G.1) — bare 'step-g-cleanup §G.1' without .md fails this gate (BC-6.26.001 PC2; F-S2104-P1-001a / F-S2104-P5-007 / F-S2104-P9-class / F-S2104-P11-003)" \
    "$skill_step8_section"
  # Enumeration-correctness gate (F-S2104-P5-007): retired 'absent-dir' token must NOT appear.
  # 'absent-dir' implies [ ! -d ] absence check (v1.5 form) — superseded by [ ! -e ] existence check.
  _assert_no_doc_marker 'absent-dir' \
    "SKILL.md Step 8 enumeration: must NOT contain 'absent-dir' token — retired with [ ! -d ] semantics; existence check [ ! -e ] supersedes it (BC-6.26.001 EC-008; was RED at 60f0d2d6 until implementer rewrites at 43ea70ba; F-S2104-P5-007)" \
    "$skill_step8_section"
  # Enumeration-correctness gate (F-S2104-P5-007): must reflect existence semantics per BC-6.26.001 —
  # non-directory or symlink → BLOCKED (PC2b). was RED at 60f0d2d6 until implementer rewrites enumeration at 43ea70ba.
  _assert_doc_marker 'non-directory.*BLOCK|BLOCK.*non-directory|symlink.*BLOCK|BLOCK.*symlink|non-directory.*PC2b|symlink.*PC2b' \
    "SKILL.md Step 8 enumeration: must reflect existence semantics — non-directory or symlink → BLOCKED (BC-6.26.001 PC2b; was RED at 60f0d2d6 until implementer rewrites at 43ea70ba; F-S2104-P5-007)" \
    "$skill_step8_section"

  # --- DOC-PARITY primary paths: per-story-delivery.md step (g) (F-S2104-P1-001b, F-S2104-P5-007) ---
  # was RED at 60f0d2d6 until implementer adds §G.1/step-g-cleanup reference adjacent to step (g) dispatch at 43ea70ba.
  # Strengthened (F-S2104-P5-007): bare 'preflight' token removed.
  # Strengthened (F-S2104-P11-003): requires .md-qualified form (step-g-cleanup.md §G.1).
  local step_g_window
  step_g_window="$(_extract_per_story_delivery_step_g_window)"
  _assert_doc_marker 'step-g-cleanup\.md.*§G\.1|§G\.1.*step-g-cleanup\.md' \
    "per-story-delivery.md step (g): §G.1 ref must use filename-qualified form (step-g-cleanup.md §G.1) — bare 'step-g-cleanup §G.1' without .md fails this gate (BC-6.26.001 PC2; F-S2104-P1-001b / F-S2104-P5-007 / F-S2104-P9-class / F-S2104-P11-003)" \
    "$step_g_window"

  # --- DOC-PARITY primary paths: per-story-delivery.md Story Split Recovery (F-S2104-P1-001b, F-S2104-P5-007) ---
  # was RED at 60f0d2d6 until implementer adds §G.1/step-g-cleanup reference to story-split cleanup step at 43ea70ba.
  # Strengthened (F-S2104-P5-007): bare 'preflight' token removed.
  # Strengthened (F-S2104-P11-003): requires .md-qualified form (step-g-cleanup.md §G.1).
  local split_recovery_section
  split_recovery_section="$(_extract_per_story_delivery_split_recovery_section)"
  _assert_doc_marker 'step-g-cleanup\.md.*§G\.1|§G\.1.*step-g-cleanup\.md' \
    "per-story-delivery.md Story Split Recovery: §G.1 ref must use filename-qualified form (step-g-cleanup.md §G.1) — bare 'step-g-cleanup §G.1' without .md fails this gate (BC-6.26.001 PC2; F-S2104-P1-001b / F-S2104-P5-007 / F-S2104-P9-class / F-S2104-P11-003)" \
    "$split_recovery_section"

  # --- DOC-PARITY WINNING playbook: workflows/phases/per-story-delivery.md Step 8 (F-S2104-P2-001, F-S2104-P5-007) ---
  # The winning playbook's own L8 declares: "If the two disagree, this file wins."
  # Its Step 8 must carry the §G.1 mandate; orchestrator copy alone is insufficient.
  # Strengthened (F-S2104-P5-007): bare 'preflight' removed; enumeration + fully-qualified-path gates added.
  local winning_step8_section
  winning_step8_section="$(_extract_winning_playbook_step8_section)"
  _assert_doc_marker 'step-g-cleanup.*§G\.1|§G\.1.*step-g-cleanup' \
    "WINNING playbook Step 8: §G.1 ref must co-occur with step-g-cleanup — bare §G.1 alone insufficient (BC-6.26.001 PC2; F-S2104-P2-001 / F-S2104-P5-007 / F-S2104-P9-class)" \
    "$winning_step8_section"
  # Filename-qualified gate (F-P5-006 leg / F-P10-003): §G.1 ref must include the .md extension in
  # the filename qualifier. A bare 'step-g-cleanup §G.1' (without .md) satisfies the co-occurrence
  # gate above but fails this gate — the .md extension is required for unambiguous cross-document
  # traceability. Differentiated from the co-occurrence gate above: that gate accepts step-g-cleanup
  # without .md; this gate does not. was RED at 60f0d2d6 until implementer qualified the winning-playbook Step 8 §G.1 reference at 43ea70ba.
  _assert_doc_marker 'step-g-cleanup\.md.*§G\.1|§G\.1.*step-g-cleanup\.md' \
    "WINNING playbook Step 8: §G.1 ref must use filename-qualified form (step-g-cleanup.md §G.1) — bare 'step-g-cleanup §G.1' without .md extension fails this gate; differentiated from the co-occurrence gate above (F-P5-006 / F-P10-003)" \
    "$winning_step8_section"
  # Enumeration-correctness gate (F-S2104-P5-007): retired 'absent-dir' token must NOT appear.
  _assert_no_doc_marker 'absent-dir' \
    "WINNING playbook Step 8 enumeration: must NOT contain 'absent-dir' token — retired; existence semantics [ ! -e ] supersedes it (BC-6.26.001 EC-008; was RED at 60f0d2d6 until implementer rewrites at 43ea70ba; F-S2104-P5-007)" \
    "$winning_step8_section"
  # Enumeration-correctness gate (F-S2104-P5-007): must reflect existence semantics per BC-6.26.001.
  _assert_doc_marker 'non-directory.*BLOCK|BLOCK.*non-directory|symlink.*BLOCK|BLOCK.*symlink|non-directory.*PC2b|symlink.*PC2b' \
    "WINNING playbook Step 8 enumeration: must reflect existence semantics — non-directory or symlink → BLOCKED (BC-6.26.001 PC2b; was RED at 60f0d2d6 until implementer rewrites at 43ea70ba; F-S2104-P5-007)" \
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
# BC-6.26.001 PC2a
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
    "step-g-cleanup.md §G.1: blanket 2>/dev/null FORBIDDEN on preflight command (BC-6.26.001 PC2)" \
    "$g1_section"
  _assert_doc_marker 'PREFLIGHT BLOCKED' \
    "step-g-cleanup.md §G.1: PREFLIGHT BLOCKED mandate present (PC2a and PC2b in same §G.1 block — BC-6.26.001 PC2)" \
    "$g1_section"
  _assert_doc_marker 'git worktree remove' \
    "step-g-cleanup.md §G.1: git worktree remove command for PC2a proceed path (BC-6.26.001 PC2a)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: PC2a sub-case (a) — .factory/ absent → proceed (F-S2104-P2-009) ---
  # Deleting the absent-path clause from §G.1 must fail this assertion.
  # The harness implements this sub-case at _run_teardown_preflight step 1 (hardcoded);
  # the DOC gate here verifies the spec documents the same behavior.
  _assert_doc_marker '\.factory.*absent|absent.*\.factory|no.*\.factory.*directory|path-absent.*NOT.*PC2c|EC-005' \
    "step-g-cleanup.md §G.1: PC2a sub-case (a) — .factory/ absent path must be documented (BC-6.26.001 EC-005; deleting this clause silently breaks the absent-path contract — F-S2104-P2-009)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: PC2a sub-case (a) discrimination predicate (F-S2104-P3-001 strengthened F-S2104-P4-007a) ---
  # §G.1 must supply a normative discrimination predicate using [ ! -e ] (existence check) to
  # distinguish PC2a(a) (path absent → proceed) from the non-directory case (PC2b BLOCKED without
  # find) and from PC2c (find exits non-zero → HALT).
  # BC-6.26.001 v1.6 corrected predicate from [ ! -d ] to [ ! -e ]:
  #   [ ! -d ] is TRUE when .factory is a regular file — wrong, authorizes teardown on stray content.
  #   [ ! -e ] is FALSE for any occupied path (file, dir, symlink), correctly gates on true absence.
  # Harness hardcodes [ ! -e ] pre-test; this DOC-PARITY gate independently verifies §G.1 matches.
  # was RED at 60f0d2d6: step-g-cleanup.md §G.1 has [ ! -d ] (v1.5 form); gate tightened to ONLY accept [ ! -e ]
  # — no longer accepts [ ! -d ] (F-S2104-P4-007a strengthened from F-S2104-P3-001).
  # GREEN post-implementation: §G.1 updated to [ ! -e ] form (BC-6.26.001 EC-008).
  _assert_doc_marker '\[ ! -e|test[[:space:]].*!.*-e.*\.factory|if.*\[.*!.*-e.*\.factory' \
    "step-g-cleanup.md §G.1: normative discrimination predicate MUST be [ ! -e ] (existence) not [ ! -d ] (directory) — BC-6.26.001 EC-008: [ ! -d ] authorizes teardown when a regular file exists at .factory (wrong); [ ! -e ] correctly identifies any occupied path; was RED at 60f0d2d6 until implementer flips to [ ! -e ] at 73c2bade (F-S2104-P3-001 strengthened by F-S2104-P4-007a)" \
    "$g1_section"

  # Negative: [ ! -d ] MUST NOT appear as the normative path-absence predicate in §G.1.
  # Allow -d in explanatory context only (e.g., "The -d test alone MUST NOT be used",
  # BC-6.26.001 EC-008 non-directory paragraph).
  # Forbid normative forms: lines with `[ ! -d` that are NOT in explanation/WARNING context.
  local forbidden_d_normative
  forbidden_d_normative="$(printf '%s\n' "$g1_section" | \
    grep -E '\[ ! -d' | \
    grep -Ev 'MUST NOT|wrong|alone|WARNING|incorrect|would.*true|would.*author|test alone|must not' || true)"
  if [ -n "$forbidden_d_normative" ]; then
    echo "DOC-PARITY FAIL [must NOT contain: [ ! -d ] as normative path-absence predicate — BC-6.26.001 EC-008 forbids -d-only check; regular file at .factory satisfies [ ! -d ] → wrong teardown authorization; use [ ! -e ] instead (F-S2104-P4-007a)]"
    printf '%s\n' "$forbidden_d_normative"
    false
  fi

  # --- DOC-PARITY §G.1: non-directory→PC2b clause (F-S2104-P4-007a, second gate) ---
  # BC-6.26.001 v1.6 adds: if something exists at .factory but is NOT a directory (regular file,
  # symlink-to-file), it is stray shadow content → PC2b BLOCKED directly, without running find.
  # §G.1 must document this case (was RED at 60f0d2d6 until implementer adds non-directory-path paragraph at 73c2bade).
  # GREEN post-implementation: §G.1 has non-directory case routing to PC2b without find.
  _assert_doc_marker '[Nn]on-directory.*(PC2b|BLOCK)' \
    "step-g-cleanup.md §G.1: non-directory inode must be documented with routing consequence (PC2b BLOCKED) — bare non-directory token without routing is insufficient (BC-6.26.001 EC-008; F-S2104-P4-007a / F-S2104-P9-class)" \
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
  # Explicitly covers EC-003 (empty dir scenario distinct from EC-005 absent-path scenario).
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
# BC-6.26.001 PC2b → PC2a retry path (Option A relocation)
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
    "step-g-cleanup.md §G.1: blanket 2>/dev/null FORBIDDEN (BC-6.26.001 PC2)" \
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
# BC-6.26.001 PC2c (fail-closed)
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

# ===========================================================================
# T-005 / AC-002 / EC-007/EC-008 / T-6: regular file at .factory → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
# BC-6.26.001 EC-008 / T-6 / non-directory path
# F-S2104-P4-007 (test leg b)
# ===========================================================================

@test "T-005 S-21.04 AC-002 EC-007: file-at-path — regular file at .factory → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called" {
  # Fixture: .factory is a REGULAR FILE (not a directory) at MOCK_WORKTREE root.
  # BC-6.26.001 EC-008 / T-6: a regular file at <worktree-path>/.factory is stray shadow
  # content subject to rm-rf destruction — it must route to PC2b BLOCKED without running find.
  #
  # Discrimination predicate semantics:
  #   [ ! -d "$MOCK_WORKTREE/.factory" ] → TRUE for a regular file (wrong: "is not a directory" → authorize teardown)
  #   [ ! -e "$MOCK_WORKTREE/.factory" ] → FALSE for a regular file (correct: "path occupied → non-directory branch")
  #
  # find NOT invoked: absence of PREFLIGHT HALT (PC2c) in output confirms find was not called.
  # REMOVE_LOG empty: git worktree remove MUST NOT execute on PC2b non-directory path.
  #
  # Pre-implementation RED gates:
  #   DOC-PARITY ([ ! -e ] predicate): §G.1 has [ ! -d ] → RED.
  #   DOC-PARITY (non-directory clause): no non-directory case in §G.1 → RED.
  # Post-implementation GREEN: §G.1 updated to [ ! -e ] + non-directory-path paragraph added.

  touch "$MOCK_WORKTREE/.factory"

  local g1_section
  g1_section="$(_extract_g1_section)"

  # --- DOC-PARITY §G.1: discrimination predicate must be [ ! -e ] (F-S2104-P4-007a) ---
  # [ ! -d ] (the v1.5 form) was superseded; only [ ! -e ] accepted now (BC-6.26.001 EC-008).
  _assert_doc_marker '\[ ! -e|test[[:space:]].*!.*-e.*\.factory' \
    "step-g-cleanup.md §G.1: [ ! -e ] existence predicate required (not [ ! -d ] alone) — regular file satisfies [ ! -d ] → wrong teardown authorization; [ ! -e ] correctly identifies path-occupancy (BC-6.26.001 EC-008; F-S2104-P4-007a)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: non-directory→PC2b BLOCKED clause presence (F-S2104-P4-007a second gate) ---
  # was RED at 60f0d2d6 until implementer adds non-directory-case paragraph at 73c2bade.
  _assert_doc_marker '[Nn]on-directory.*(PC2b|BLOCK)' \
    "step-g-cleanup.md §G.1: non-directory inode must be documented with routing consequence (PC2b BLOCKED) — bare non-directory token without routing is insufficient (BC-6.26.001 EC-008/T-6; F-S2104-P4-007a / F-S2104-P9-class)" \
    "$g1_section"

  # Non-directory case must route to PC2b BLOCKED (not PC2a or PC2c)
  _assert_doc_marker 'non-directory.*PC2b|non-directory.*BLOCK|NOT.*directory.*BLOCK|non-directory.*stray|regular.*file.*stray|regular.*file.*PC2b' \
    "step-g-cleanup.md §G.1: non-directory inode routes to PC2b BLOCKED (stray shadow content; BC-6.26.001 non-directory-path paragraph; was RED at 60f0d2d6 until implementer adds non-directory-case paragraph at 73c2bade)" \
    "$g1_section"

  # --- HARNESS: regular file at .factory → PC2b BLOCKED; non-zero exit; find NOT invoked ---
  # _run_teardown_preflight discrimination chain (per BC-6.26.001): [ ! -e ] → PC2a(a); [ -L ] → PC2b symlink;
  # [ ! -d ] → PC2b non-directory; directory (no symlink) → run extracted find.
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -ne 0 ] || {
    echo "HARNESS FAIL: non-directory .factory must return non-zero (PC2b BLOCKED) — got status 0 (harness discrimination-chain logic: non-directory must not fall through to the find branch)"
    false
  }
  printf '%s\n' "$output" | grep -q 'PREFLIGHT BLOCKED' || {
    echo "HARNESS FAIL: 'PREFLIGHT BLOCKED' not in output for regular-file .factory — got: $output"
    false
  }
  # Non-directory path must be listed in output (BC-6.26.001 T-6: "list the path")
  printf '%s\n' "$output" | grep -q "${MOCK_WORKTREE}/.factory" || {
    echo "HARNESS FAIL: .factory path '${MOCK_WORKTREE}/.factory' must appear in PREFLIGHT BLOCKED output (BC-6.26.001 T-6) — got: $output"
    false
  }
  # find NOT invoked: no PREFLIGHT HALT/PC2c in output (PC2c is only emitted when find is called
  # and exits non-zero; its absence proves the non-directory path was taken, not the find path).
  if printf '%s\n' "$output" | grep -qE 'PREFLIGHT HALT|PC2c'; then
    echo "HARNESS FAIL: PREFLIGHT HALT/PC2c found in output — find was invoked on the non-directory path; MUST NOT be invoked (BC-6.26.001 EC-008: do NOT run find on non-directory inode; T-6)"
    false
  fi
  # Mutant-proving sentinel: git worktree remove MUST NOT be invoked on PC2b non-directory path
  [ ! -s "$REMOVE_LOG" ] || {
    echo "HARNESS FAIL: REMOVE_LOG non-empty on PC2b non-directory path — git worktree remove MUST NOT be invoked (BC-6.26.001 PC2b; T-6) — log: $(cat "$REMOVE_LOG")"
    false
  }
}

# ===========================================================================
# T-006 / AC-002 / BC-6.26.001 PC2b symlink vector: symlink at .factory pointing at real dir
# → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
# BC-6.26.001 PC2b (symlink case)
# F-S2104-P5-011
# ===========================================================================

@test "T-006 S-21.04 AC-002: symlink-at-path — symlink at .factory pointing at real dir → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called" {
  # Fixture: $MOCK_WORKTREE/.factory is a SYMLINK pointing at a real directory inside $WORK
  # that contains a file. BC-6.26.001 PC2b: symlink-at-path → BLOCKED regardless of target type.
  #
  # Key distinction from T-005 (regular file): a symlink-to-dir satisfies [ -d ] by dereferencing,
  # so the v1.6 [ ! -d ] check alone would NOT catch it — the symlink would fall through to find.
  # v1.7 adds [ -L ] BEFORE [ ! -d ] to catch symlinks of all target types (including dir symlinks).
  #
  # DOC-PARITY gate: §G.1 must carry the [ -L ] clause — was RED at 60f0d2d6 until implementer lands it at 4833a642.
  # HARNESS: tests the discrimination chain hardcoded in _run_teardown_preflight (per BC-6.26.001).

  # Create a real directory for the symlink to point at (inside $WORK so it's accessible).
  # The directory contains a file — confirming find would find content if invoked (it must NOT be).
  local symlink_target
  symlink_target="$WORK/symlink-target-dir"
  mkdir -p "$symlink_target"
  printf 'target file content — find must NOT reach this\n' > "$symlink_target/target-file.txt"

  # Create the symlink at .factory pointing at the real directory.
  ln -s "$symlink_target" "$MOCK_WORKTREE/.factory"

  local g1_section
  g1_section="$(_extract_g1_section)"

  # --- DOC-PARITY §G.1: [ -L ] shell expression (indented) — load-bearing gate (F-S2104-P6-003a) ---
  # §G.1 must carry the literal [ -L ] shell test expression as an indented command, not just
  # symlink-prose. The prior alternation (symlink.*PC2b|symlink.*BLOCK) was satisfied by the PC2b
  # header line alone — deleting the [ -L ] clause left all 9 tests GREEN (paper-gate).
  # This gate requires ^[[:space:]]+\[ -L  matching the shell-expression form; prose backtick
  # references like `[ -L ]` do NOT satisfy it (those survive clause deletion — F-S2104-P6-003a).
  _assert_doc_marker '^[[:space:]]+\[ -L ' \
    "step-g-cleanup.md §G.1: literal [ -L ] shell expression required as indented command — prose-only mention does not prove the test is present; bracket-L form ^<spaces>[ -L must appear (BC-6.26.001 PC2b symlink; T-006; F-S2104-P5-011)" \
    "$g1_section"

  # --- DOC-PARITY §G.1: ORDERING — [ -L ] must precede first find invocation (F-S2104-P6-003b) ---
  # The [ -L ] check must appear BEFORE the find command within §G.1; an ordering inversion would
  # allow find to be called on a symlink-to-dir (which satisfies [ -d ] by dereferencing). Uses
  # the same awk line-number comparison pattern as the pass-2 preflight-before-dispatch gate.
  local bracket_l_lineno find_lineno
  bracket_l_lineno="$(printf '%s\n' "$g1_section" | awk '/^[[:space:]]+\[ -L / { print NR; exit }')"
  find_lineno="$(printf '%s\n' "$g1_section" | awk '/^[[:space:]]*find[[:space:]]/ { print NR; exit }')"
  [ -n "$bracket_l_lineno" ] || {
    echo "DOC-PARITY FAIL: [ -L ] shell expression not found in §G.1 section — bracket-L must be present as an indented command (BC-6.26.001 PC2b symlink; F-S2104-P6-003b)"
    false
  }
  [ -n "$find_lineno" ] || {
    echo "DOC-PARITY FAIL: find invocation not found in §G.1 section — cannot verify [ -L ] ordering (BC-6.26.001 PC2b; F-S2104-P6-003b)"
    false
  }
  [ "$bracket_l_lineno" -lt "$find_lineno" ] || {
    echo "DOC-PARITY FAIL: [ -L ] line ($bracket_l_lineno) must precede first find line ($find_lineno) in §G.1 — ordering inversion allows find to be called on symlink-to-dir (BC-6.26.001 PC2b symlink; F-S2104-P6-003b)"
    false
  }

  # --- HARNESS: symlink at .factory → PC2b BLOCKED; non-zero exit ---
  # The harness [ -L ] check (step 2, HARDCODED) fires before any find invocation (step 4).
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -ne 0 ] || {
    echo "HARNESS FAIL: symlink at .factory must return non-zero (PC2b BLOCKED) — got status 0 (harness [ -L ] check at step 2 must fire before find at step 4; per BC-6.26.001)"
    false
  }
  printf '%s\n' "$output" | grep -q 'PREFLIGHT BLOCKED' || {
    echo "HARNESS FAIL: 'PREFLIGHT BLOCKED' not in output for symlink .factory — got: $output"
    false
  }

  # Path must be listed in output (BC-6.26.001 T-6 pattern: list the path).
  printf '%s\n' "$output" | grep -q "${MOCK_WORKTREE}/.factory" || {
    echo "HARNESS FAIL: .factory path '${MOCK_WORKTREE}/.factory' must appear in PREFLIGHT BLOCKED output — got: $output"
    false
  }

  # find NOT invoked: no PREFLIGHT HALT/PC2c in output.
  # PC2c is emitted only when find is called and exits non-zero; its absence proves
  # the [ -L ] branch was taken (step 2), not the find branch (step 4).
  if printf '%s\n' "$output" | grep -qE 'PREFLIGHT HALT|PC2c'; then
    echo "HARNESS FAIL: PREFLIGHT HALT/PC2c found in output — find was invoked on symlink path; MUST NOT be invoked (BC-6.26.001 PC2b symlink; [ -L ] check must fire first at step 2)"
    false
  fi

  # REMOVE_LOG must be empty: git worktree remove MUST NOT execute on PC2b symlink path.
  [ ! -s "$REMOVE_LOG" ] || {
    echo "HARNESS FAIL: REMOVE_LOG non-empty on PC2b symlink path — git worktree remove MUST NOT be invoked (BC-6.26.001 PC2b) — log: $(cat "$REMOVE_LOG")"
    false
  }
}

# ===========================================================================
# F-S2104-P4-009: DOC-PARITY regression gates for 6 ungated mandate surfaces
# BC-6.26.001 PC2 + AC-007(d)
# Target surfaces: worktree-manage/SKILL.md, code-delivery/SKILL.md, fix-pr-delivery/SKILL.md,
#   workflows/code-delivery.lobster, workflows/greenfield.lobster, rules/worktree-protocol.md
# RED on surfaces that present `find` as first action with absent-path/find-error as unordered
#   siblings (anti-pattern per AC-007(d)); GREEN on surfaces that delegate cleanly to §G.1.
# ===========================================================================

@test "F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant" {
  # T-008 / AC-007(d) — named for ordinal-free reference (F-S2104-P4-009 6-surface mandate gate)
  # Each of the 6 surfaces must:
  #   (i) reference step-g-cleanup.md §G.1 (or unambiguous equivalent)
  #   (ii) NOT present `find` as the first action with absent-path/find-error as unordered siblings
  #        (anti-pattern: inline bare find-first command without explicit absent-path-first ordering)
  #
  # RED pre-implementation (5/6): worktree-manage, code-delivery, fix-pr-delivery SKILL.mds +
  #   code-delivery.lobster + greenfield.lobster all have inline `find .factory -type f` as the
  #   primary stated action → anti-pattern → was RED at 60f0d2d6 until implementer replaces with §G.1 delegation at a317fd77.
  # GREEN pre-implementation (1/6): worktree-protocol.md delegates to §G.1 without inline find.

  # Helper: assert anti-pattern absent in a file (inline bare find command).
  # Anti-pattern: `find <path>/.factory -type f` as an inline command the agent is instructed to run.
  # After fix: surface says "run §G.1 preflight" or "proceed on PASS" without inlining find.
  _assert_no_inline_find_antipattern() {
    local file="$1" label="$2"
    # Regex catches unquoted ('.factory -type f', '.factory/ -type f') AND the quoted canonical
    # form ('find "<worktree-path>/.factory/" -type f') from the canonical find command in step-g-cleanup.md §G.1.
    # The prior pattern '.factory/?[[:space:]]' failed on the quoted form: after '\.factory/',
    # the closing '"' precedes the space, so '[[:space:]]' could not match (F-S2104-P7-002).
    # Fix: '[^[:space:]]*' after '\.factory/?' consumes any trailing non-space chars (e.g., '"')
    # before '[[:space:]]' matches the argument separator (F-S2104-P6-007 + F-S2104-P7-002).
    if grep -qE 'find[[:space:]]+[^[:space:]]*\.factory/?[^[:space:]]*[[:space:]].*-type[[:space:]]+f' "$file"; then
      echo "DOC-PARITY FAIL [anti-pattern present in $label]: surface presents inline bare 'find ... .factory[/] ... -type f' as the first action — MUST NOT inline find command; delegate to §G.1 preflight instead (BC-6.26.001 PC2 + AC-007(d); absent-path check is first, not an unordered sibling; F-S2104-P4-009)"
      false
    fi
  }

  # Helper: assert fully-qualified step-g-cleanup.md path present (F-S2104-P9-class strengthened).
  # Prior bare alternation 'step-g-cleanup|§G\.1|G\.1' satisfiable by any incidental §G.1 mention
  # without the qualified path form. All 6 surfaces carry the fully-qualified
  # 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md' path — require it.
  _assert_g1_ref() {
    local file="$1" label="$2"
    grep -qE 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup\.md' "$file" || {
      echo "DOC-PARITY FAIL [fully-qualified §G.1 path missing from $label]: surface must carry fully-qualified path 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md' — bare §G.1 or step-g-cleanup alone is insufficient for cross-document traceability (BC-6.26.001 PC2 + AC-007(d); F-S2104-P4-009 / F-S2104-P9-class)"
      false
    }
  }

  # --- 1. skills/worktree-manage/SKILL.md ---
  # was RED at 60f0d2d6: has inline `find .worktrees/STORY-NNN/.factory -type f` as primary instruction.
  _assert_g1_ref "$WORKTREE_MANAGE_SKILL_MD" "skills/worktree-manage/SKILL.md"
  _assert_no_inline_find_antipattern "$WORKTREE_MANAGE_SKILL_MD" "skills/worktree-manage/SKILL.md"

  # --- 2. skills/code-delivery/SKILL.md ---
  # was RED at 60f0d2d6: has inline `find .worktrees/STORY-NNN/.factory -type f` as primary instruction.
  _assert_g1_ref "$CODE_DELIVERY_SKILL_MD" "skills/code-delivery/SKILL.md"
  _assert_no_inline_find_antipattern "$CODE_DELIVERY_SKILL_MD" "skills/code-delivery/SKILL.md"

  # --- 3. skills/fix-pr-delivery/SKILL.md ---
  # was RED at 60f0d2d6: has inline `find .worktrees/FIX-P[phase]-NNN/.factory -type f` as primary instruction.
  _assert_g1_ref "$FIX_PR_DELIVERY_SKILL_MD" "skills/fix-pr-delivery/SKILL.md"
  _assert_no_inline_find_antipattern "$FIX_PR_DELIVERY_SKILL_MD" "skills/fix-pr-delivery/SKILL.md"

  # --- 4. workflows/code-delivery.lobster ---
  # was RED at 60f0d2d6: has inline `find [worktree_path]/.factory -type f` as primary instruction.
  _assert_g1_ref "$CODE_DELIVERY_WORKFLOW" "workflows/code-delivery.lobster"
  _assert_no_inline_find_antipattern "$CODE_DELIVERY_WORKFLOW" "workflows/code-delivery.lobster"

  # --- 5. workflows/greenfield.lobster ---
  # was RED at 60f0d2d6: has inline `find .worktrees/STORY-NNN/.factory -type f` as primary instruction.
  _assert_g1_ref "$GREENFIELD_WORKFLOW" "workflows/greenfield.lobster"
  _assert_no_inline_find_antipattern "$GREENFIELD_WORKFLOW" "workflows/greenfield.lobster"

  # --- 6. rules/worktree-protocol.md ---
  # GREEN: delegates to §G.1 without inlining a bare find command (conformant pre-implementation).
  _assert_g1_ref "$WORKTREE_PROTOCOL_MD" "rules/worktree-protocol.md"
  _assert_no_inline_find_antipattern "$WORKTREE_PROTOCOL_MD" "rules/worktree-protocol.md"
}

# ===========================================================================
# F-S2104-P4-002 / T-009: DOC-PARITY gates — adversary.md + adversarial-review/SKILL.md §G.1/BC-6.26.001 preflight-awareness
# AC-009 specialist agent awareness — obligation-asserting (pass-9 strengthened; F-S2104-P9-001)
# Three gates per file: corrected-model + report-as-defect-signal + §G.1 enforcement-chain reference.
# Prior bare alternation (byte-identical to retired pass-7 T-007 pattern) satisfied by incidental
# BC-6.26.001 mention without the corrected shadow-write model or defect-signal obligation present.
# ===========================================================================

@test "F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause" {
  # T-009 (ordinal-free reference: F-S2104-P4-002) — obligation-asserting gates (F-S2104-P9-001)
  #
  # AC-009 (story v1.12): both awareness files must carry the corrected shadow-write model:
  #   git worktree add checks out NOTHING under .factory/ (gitignored on the product branch);
  #   any .factory/ content found in a worktree is LIVE SHADOW-WRITE EVIDENCE that MUST be
  #   REPORTED as a defect signal (BC-6.26.001 Invariant 5); step-g-cleanup.md §G.1 is the
  #   enforcement chain (teardown preflight catches this class before git worktree remove).
  #
  # Three obligation-asserting gates per file (replaces byte-identical bare alternation
  # retired at pass-7 for T-007; satisfiable by any incidental BC-6.26.001 token):
  #   (i)  corrected-model: worktree-add creates no .factory/ content (no shadow write at checkout)
  #   (ii) report-as-defect-signal: shadow .factory/ content must be reported as defect signal
  #   (iii) §G.1 enforcement-chain: step-g-cleanup.md §G.1 cited as the teardown-preflight chain
  #
  # MUTANT PROOF: reducing adversary.md §G.4 to "…resolve the tuple (see BC-6.26.001)." passes
  # the retired bare alternation (BC-6.26.001 is present) but fails all three new gates —
  # corrected-model sentence, defect-signal token, and §G.1 enforcement-chain are all absent.

  # --- agents/adversary.md ---
  # adversary.md §G.4 delivers:
  #   (i)  "git worktree add checks out NOTHING under .factory/ — .factory/ is gitignored on the
  #         product branch, so no shadow directory is created at worktree-creation time."
  #   (ii) "MUST be reported as a defect signal, not dismissed as a pathing artifact."
  #   (iii) "step-g-cleanup.md §G.1 teardown preflight exists to catch this class."

  # (i) corrected-model: checks out NOTHING / no shadow directory is created
  grep -qE 'checks out NOTHING|no shadow directory.*created' "$ADVERSARY_MD" || {
    echo "DOC-PARITY FAIL [adversary.md (i) corrected-model absent]: must state git worktree add checks out NOTHING under .factory/ — no shadow directory is created at worktree-creation time (AC-009; BC-6.26.001 Invariant 5; F-S2104-P9-001)"
    false
  }

  # (ii) report-as-defect-signal: shadow .factory/ content reported as defect signal, NOT to be dismissed
  # adversary.md §G.4 delivers "MUST be reported as a defect signal, not dismissed as a pathing artifact"
  # — "not dismissed as a pathing artifact" is unique to the §G.4 corrected-model sentence; absent on
  # other lines that also have "defect signal" in different qualifier contexts (bare 'defect signal' would
  # satisfy on mutant since other lines carry it).
  grep -qE 'defect signal.*not dismissed|not dismissed.*pathing artifact' "$ADVERSARY_MD" || {
    echo "DOC-PARITY FAIL [adversary.md (ii) defect-signal obligation absent]: must state shadow .factory/ content is reported as a defect signal, not dismissed as a pathing artifact (AC-009; BC-6.26.001 Invariant 5; F-S2104-P9-001)"
    false
  }

  # (iii) §G.1 enforcement-chain: step-g-cleanup.md §G.1 cited as the teardown-preflight mechanism
  grep -qE 'step-g-cleanup.*§G\.1|§G\.1.*preflight|§G\.1.*teardown' "$ADVERSARY_MD" || {
    echo "DOC-PARITY FAIL [adversary.md (iii) §G.1 enforcement-chain absent]: must reference step-g-cleanup.md §G.1 as the enforcement chain for the teardown preflight obligation (AC-009; BC-6.26.001 Invariant 5; F-S2104-P9-001)"
    false
  }

  # --- skills/adversarial-review/SKILL.md ---
  # adversarial-review SKILL.md §Worktree-Identity Preflight delivers:
  #   (i)  "no .factory/ directory is created at worktree-checkout time"
  #   (ii) "MUST be reported as a defect signal, not used as spec ground-truth"
  #   (iii) "step-g-cleanup.md §G.1 teardown preflight exists to catch this class"

  # (i) corrected-model: no .factory/ directory is created at worktree-checkout time
  grep -qE 'no.*\.factory.*directory.*created|no.*\.factory.*created.*worktree' "$ADV_REVIEW_SKILL_MD" || {
    echo "DOC-PARITY FAIL [adversarial-review/SKILL.md (i) corrected-model absent]: must state no .factory/ directory is created at worktree-checkout time (AC-009; BC-6.26.001 Invariant 5; F-S2104-P9-001)"
    false
  }

  # (ii) report-as-defect-signal with anti-spec-ground-truth qualifier.
  # The corrected-model clause in the §Worktree-Identity Preflight opening sentence delivers:
  #   "MUST be reported as a defect signal, not used as spec ground-truth".
  # Bare 'defect signal' alone is satisfiable by the background-context sentence in step 4 of
  # the orchestrator pre-dispatch steps ("a defect signal the adversary should report,
  # not dismiss") — which survives even if the corrected-model clause in the §Worktree-Identity
  # Preflight opening sentence is stripped. Co-occurrence with 'spec ground-truth' is unique to
  # the corrected-model obligation in the §Worktree-Identity Preflight opening sentence.
  grep -qE 'defect signal.*spec ground-truth|not used as spec ground-truth' "$ADV_REVIEW_SKILL_MD" || {
    echo "DOC-PARITY FAIL [adversarial-review/SKILL.md (ii) defect-signal co-occurrence absent]: must state shadow .factory/ content is a defect signal 'not used as spec ground-truth' — bare 'defect signal' alone is satisfiable by the background-context sentence in step 4 of the orchestrator pre-dispatch steps without the corrected-model obligation in the §Worktree-Identity Preflight opening sentence (AC-009; BC-6.26.001 Invariant 5; F-S2104-P9-001 / F-S2104-P9-class)"
    false
  }

  # (iii) §G.1 enforcement-chain: step-g-cleanup.md §G.1 cited as the teardown-preflight mechanism
  grep -qE 'step-g-cleanup.*§G\.1|§G\.1.*preflight|§G\.1.*teardown' "$ADV_REVIEW_SKILL_MD" || {
    echo "DOC-PARITY FAIL [adversarial-review/SKILL.md (iii) §G.1 enforcement-chain absent]: must reference step-g-cleanup.md §G.1 as the enforcement chain for the teardown preflight obligation (AC-009; BC-6.26.001 Invariant 5; F-S2104-P9-001)"
    false
  }

  # --- F-S2104-P14R-008: behavioral_contracts field-name correctness in adversary.md ---
  # The perimeter scope sentence (adversary.md Perimeter 1 scope) and four bidirectional-BC-
  # completeness items (Story Frontmatter-Body Coherence Review Axis) must reference
  # behavioral_contracts: frontmatter field — not the stale bcs: field name.
  # (i) Positive: behavioral_contracts: must be present in adversary.md
  grep -qE 'behavioral_contracts:' "$ADVERSARY_MD" || {
    echo "DOC-PARITY FAIL [adversary.md behavioral_contracts: field absent]: adversary.md perimeter scope sentence and bidirectional-BC-completeness items must reference behavioral_contracts: frontmatter field — stale bcs: form is not recognized; mutant-proven: reverting one site to bcs: → RED, restoring behavioral_contracts: → GREEN (F-S2104-P14R-008)"
    false
  }
  # (ii) Negative: stale bcs: as standalone field token must NOT appear in adversary.md.
  # Pattern (^|[^a-zA-Z0-9_])bcs: matches bcs: when preceded by any non-identifier character
  # (start-of-line, space, backtick, quote, or punctuation) — avoids false hits on compound
  # identifiers like 'subbcs:' or 'xbcs:' while catching both YAML-field and prose forms.
  if grep -qE '(^|[^a-zA-Z0-9_])bcs:' "$ADVERSARY_MD"; then
    echo "DOC-PARITY FAIL [adversary.md stale bcs: field present]: adversary.md must NOT reference stale bcs: frontmatter field as a standalone token — use behavioral_contracts: instead (F-S2104-P14R-008)"
    false
  fi
}

# ===========================================================================
# F-S2104-P4-003: DOC-PARITY gate — agents/devops-engineer.md §Worktree Cleanup preflight-verification mandate
# AC-008: executor-side defensive preflight — verify-PASS + run-it-yourself obligations
# Strengthened (F-S2104-P7-003): prior broad alternation was satisfiable by any bare §G.1/BC-6.26.001
# mention (paper-gate). Replaced with obligation-asserting gates for both AC-008 obligations.
# ===========================================================================

_extract_devops_worktree_cleanup_section() {
  awk '
    /^### Worktree Cleanup/ { found=1; next }
    found && /^### / { exit }
    found && /^## / { exit }
    found { print }
  ' "$DEVOPS_ENGINEER_MD"
}

@test "F-S2104-P4-003: agents/devops-engineer.md §Worktree Cleanup — preflight-verification mandate" {
  # T-007 / AC-008 — named for ordinal-free reference (F-S2104-P4-003 devops-engineer gate)
  #
  # agents/devops-engineer.md §Worktree Cleanup must carry BOTH AC-008 obligations:
  #   (i)  verify-PASS clause: verify caller ran §G.1 preflight and got a PASS result.
  #   (ii) run-it-yourself fallback: if not evident from the dispatch, run §G.1 yourself first.
  #
  # Prior gate '§G\.1|step-g-cleanup|BC-6\.26\.001|preflight.*worktree remove|...' was satisfiable
  # by any bare §G.1 or BC-6.26.001 mention — a mutant that adds only a reference token without
  # the verify/PASS/fallback semantics would pass (F-S2104-P7-003 paper-gate finding).
  # ADR-031 caller-side ruling: the primary gate is caller-side (orchestrator/skill); the
  # executor-side adds defense-in-depth consistent with that ruling.

  local devops_cleanup_section
  devops_cleanup_section="$(_extract_devops_worktree_cleanup_section)"

  # Qualified-path assertion: §G.1 reference must be fully qualified with the plugins/ path
  # (bare '§G.1' alone is insufficient for cross-document traceability; F-S2104-P7-003)
  _assert_doc_marker 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup\.md' \
    "agents/devops-engineer.md §Worktree Cleanup: qualified path 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md' required — bare §G.1 alone is insufficient (F-S2104-P7-003)" \
    "$devops_cleanup_section"

  # (i) dispatching-caller + PASS-result obligation: both tokens required in the section.
  # 'verify' and 'dispatching caller' span lines (grep -qE is single-line); co-occurrence not
  # possible within one assertion. 'dispatching caller' is on its own line and is a stronger
  # obligation token: it tests that the executor-side mandate names the CALLER as the party
  # responsible for running §G.1, not just that §G.1 was "verified" generically.
  # A mutant "the caller ran §G.1" (dropping 'dispatching') → 'dispatching caller' RED.
  # 'PASS result|preflight result' and 'dispatching caller' appear on different lines; checked
  # separately for clear failure messages when either is absent. (F-S2104-P9-class strengthened.)
  _assert_doc_marker 'dispatching caller' \
    "agents/devops-engineer.md §Worktree Cleanup: (i) dispatching-caller clause required — AC-008: executor must verify THE DISPATCHING CALLER ran §G.1 preflight, not just that §G.1 was run generically (F-S2104-P7-003 / F-S2104-P9-class)" \
    "$devops_cleanup_section"
  _assert_doc_marker 'PASS result|preflight result' \
    "agents/devops-engineer.md §Worktree Cleanup: (i) PASS-result clause required — AC-008: must name the expected PASS outcome explicitly, not just 'check' or 'confirm' (F-S2104-P7-003)" \
    "$devops_cleanup_section"

  # (ii) run-it-yourself fallback: 'not evident' trigger + explicit run/execute §G.1 instruction.
  # These tokens co-occur on the same line: "If not evident from the dispatch, run the §G.1 preflight yourself first"
  _assert_doc_marker 'not evident.*(run|execute).*(§G\.1|step-g-cleanup)' \
    "agents/devops-engineer.md §Worktree Cleanup: (ii) run-it-yourself fallback required — AC-008: must instruct executor to run §G.1 preflight themselves when PASS result is not evident from the dispatch (F-S2104-P7-003)" \
    "$devops_cleanup_section"
}

# (attestation content removed per F-S2104-P14R-004 — state-manager owns red-gate-log.md SoT)
