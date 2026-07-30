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
#     pass-18 adds: BALANCED-FENCE well-formedness (F-S2104-P18-002(a)), fence-exclusion removed
#     (F-S2104-P18-002(b)), BOUNDARY-RULE sentence-splitter for Gates 4/5/PW-B (F-S2104-P18-003),
#     rendered domain stripping (F-S2104-P18-004), 2b domain parity + widened classes
#     (F-S2104-P18-005(a)/(b)), and open-trigger write-directive gate (F-S2104-P18-001).
#     pass-19 adds: CLAUSE-SCOPED write-directive gate + escape-scope controls (F-S2104-P19-001/012),
#     domain extended to ### Spec-Path Discipline (F-S2104-P19-002), referent predicate action
#     class (F-S2104-P19-003), boundary-completeness assertion (F-S2104-P19-004), CommonMark-correct
#     link-ref-def strip (F-S2104-P19-005), canonical-target domain widened to \.factory/
#     (F-S2104-P19-006), and scope-restriction gate (F-S2104-P19-007).
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
#   T-010  EC-009  stray-inode-inside-factory: symlink + FIFO inside real .factory/ dir → PREFLIGHT BLOCKED via ! -type d; missed by -type f (M03(a) predicate-delta proof; BC-6.26.001 EC-009; F-S2104-P28-006)
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
# by construction — M-P16-B decoy inserted in that region is excluded automatically. NOTE:
# this exclusion holds for decoys (compliant anchor paragraphs) only. Harmful write-directives
# in the same above-heading region are covered by the write-directive gate, whose domain is
# extended to the whole ### Spec-Path Discipline section per F-S2104-P19-002; M-P19-H proved
# that verbatim M-P17-A placed above this heading passes 9/9 without P19-002.
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

# _build_section_prose SECTION_CONTENT
# Builds a normalised prose domain from a Markdown section for gate evaluation.
# Single source of truth shared by T-001 and the pipeline probe (F-S2104-P24-003); using this
# function in the probe ensures the probe exercises the real production code path rather than
# re-implementing the pipeline over synthetic strings (POLICY 11 anti-tautology discipline).
#
# Applies in order:
#   (1) Recursive blockquote-marker strip (F-S2104-P23-001 / F-S2104-P24-001):
#       sed -E 's/^([[:space:]]*>[[:space:]]*)+//' removes all leading '>' chains from each line
#       (single-level and multi-level: '>>' handled by the '+' quantifier), preserving content.
#       Normalises blockquote form to plain text so gate predicates (including line-anchored
#       bare-imperative ^Anchor/^Resolve/^Place) apply correctly. The non-recursive old form
#       'sed s/^[[:space:]]*>[[:space:]]*//' consumed only one '>' — a '>> ...' line retained
#       one '>' after strip, defeating ^-anchored bare-imperative detection (F-S2104-P24-001).
#   (2) Bullet/numbered-list-marker strip (F-S2104-P24-004):
#       sed -E 's/^[[:space:]]*[-*+][[:space:]]+//' removes leading hyphen/asterisk/plus list
#       markers; sed -E 's/^[[:space:]]*[0-9]+\.[[:space:]]+//' removes numbered-list markers.
#       Without this normalisation, '- Anchor every write ...' left the imperative after the
#       list marker, preventing the line-anchored ^Anchor match in PWBD_DIRECTIVE_CLASS.
#   (3) Newline-join (tr '\n' ' '): collapses multi-line section to single prose string.
_build_section_prose() {
  local section="$1"
  # Unified single-pass marker strip (B03 F-S2104-P25-B03): alternation with + quantifier handles
  # any combination of blockquote '>', unordered-list '- '/'+ '/'* ', and numbered-list '1. '
  # markers in any order at line start — mixed markers ('- > ', '> 1. ') are fully consumed in
  # one pass. Previous 3-pass approach left residue on out-of-order combinations (e.g., '- > Anchor'
  # stripped '- ' in pass 2 leaving '> Anchor', which the already-executed pass 1 could not re-strip).
  printf '%s\n' "$section" | \
    sed -E 's/^([[:space:]]*(>[[:space:]]*|[-*+][[:space:]]+|[0-9]+\.[[:space:]]+))+//' | \
    tr '\n' ' '
}

# Named wrapper for spec-path-section prose normalisation (F-S2104-P24-002/F-S2104-P24-003).
# T-001's spec_path_prose and the pipeline probe's Leg C both call this function.
# A revert of this function to bare 'tr' (no marker strip) makes leg_c_result empty — probe RED.
# Mirrors Leg A's _build_section_prose dependency: the load-bearing assertion is the shared
# function call, not an inline rebuild; reverting the function is the testable reversion point.
_build_spec_path_section_prose() {
  _build_section_prose "$1"
}

# Abbreviation-protection builder for sentence-split domains (H03 F-S2104-P26-H03).
# Replaces inline 'printf ... | sed' nosplit construction at call sites so every _nosplit
# variable whose name contains _prose routes through a _build_* function — required for
# Leg E call-site parity gate. Applies cf./i.e./e.g. protection in a single named pass.
# Call sites MUST use this function rather than bare inline sed; _nosplit assignment directly
# from 'printf ... | sed' bypasses Leg E once grep -v '_nosplit' is removed.
_build_nosplit() {
  local prose="$1"
  printf '%s\n' "$prose" | \
    sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g'
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

# Shared directive class for Gate PW-B and write-directive gate (F-S2104-P21-001).
# Defined at file scope so both T-001 and the pipeline probe (F-S2104-P24-003) share
# one definition — prevents independent-authoring drift. Local definition in T-001 removed.
# UNION of prior PW-B class and prior write-directive class:
#   Prior PW-B class: MUST|SHOULD|required|is[[:space:]]+the[[:space:]]+required|
#     is[[:space:]]+preferred|is[[:space:]]+acceptable|permits
#   Prior write-directive class: above plus may|
#     ^(\*\*[^:*]+:\*\*[[:space:]]+)?(Anchor|Write|Save|Store|Place|Record|Emit|Persist|Resolve|Use)
# Bare-imperative alternation is LINE-ANCHORED (^ with optional **Label:** prefix) to prevent
# mid-sentence false positives; list-marker prefixes are normalised out by _build_section_prose
# BEFORE the domain is constructed, so ^-anchored matching works on normalised prose.
# ^ removed from bare-imperative alternation (B03 F-S2104-P25-B03): after unified marker strip the
# imperative verb may be at line start OR mid-prose after sentence splitting — position-independent
# match is required. Previous ^-anchor only fired when the verb was the first token on the line;
# tr '\n' ' ' collapses multi-line prose before gate application so bare-imperative after a '> - '
# mixed-marker sequence was anchored to start of the collapsed prose, missing most sentences.
PWBD_DIRECTIVE_CLASS='MUST|SHOULD|required|is[[:space:]]+the[[:space:]]+required|is[[:space:]]+preferred|is[[:space:]]+acceptable|permits|may|(\*\*[^:*]+:\*\*[[:space:]]+)?(Anchor|Write|Save|Store|Place|Record|Emit|Persist|Resolve|Use)[[:space:]]'

# ===========================================================================
# EXECUTABLE-HARNESS helper
# ===========================================================================

# Run the BC-6.26.001 PC2 teardown preflight against a fixture worktree path.
# Args:
#   $1  worktree_path  — simulated story-worktree path (tmpfs fixture)
#   $2  remove_log     — sentinel file; 'worktree-remove-invoked' appended on PC2a proceed
#
# Anti-tautology gate (TD-VSDD-059, F-S2104-P1-002e): extracts the find command verbatim
# from step-g-cleanup.md §G.1 (line matching 'find ... .factory ... ! -type d' without
# 2>/dev/null), substitutes <worktree-path> with the fixture path, and evaluates that
# extracted command. A -type d or -name '*.tmp' doc-mutant changes which files find returns,
# failing T-001 (stray file not found) or T-002 (directory found instead of nothing).
# A harness hardcoding 'find ... ! -type d 2>/dev/null || true' would pass T-002 tautologically
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
  # The line must contain 'find', '.factory', and '! -type d' (M01(a)/M03(a): trailing-slash
  # mandate retracted; predicate widened from -type f to '! -type d' so symlinks inside a real
  # shadow .factory/ are also detected; whitespace between '!' and '-type' is variable).
  # It must NOT contain '2>/dev/null' (BC v1.5 removed blanket suppression for PC2c).
  # Pre-implementation (doc has 2>/dev/null or wrong predicate): gate fires.
  # Post-implementation (conformant find ... ! -type d): gate passes, extracted command is eval'd.
  local find_cmd_line
  find_cmd_line="$(printf '%s\n' "$g1_section" | \
    grep -E '^[[:space:]]*find[[:space:]]' | \
    grep '\.factory' | \
    grep -E '![[:space:]]*-type[[:space:]]+d' | \
    grep -v '2>/dev/null' | \
    head -1)"

  if [ -z "$find_cmd_line" ]; then
    printf 'HARNESS FAIL: could not extract conformant find command from step-g-cleanup.md §G.1\n'
    printf '  Required: line matching find ... .factory ... ! -type d (without 2>/dev/null)\n'
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
  # Non-.md stray artifact: makes the 'any non-directory type' property of '! -type d' load-bearing
  # (F-S2104-P2-010). A '-name *.md' doc-mutant would skip this file — the assertion below
  # on 'engine-config.yaml' catches the mutant. Issue #523 confirmed-loss set includes
  # non-.md engine-config artifacts.
  printf 'engine-config artifact — non-.md stray vector (issue #523 confirmed-loss set)\n' \
    > "$MOCK_WORKTREE/.factory/engine-config.yaml"

  local g1_section
  g1_section="$(_extract_g1_section)"

  # --- DOC-PARITY §G.1: exact preflight command form — find + ! -type d, NO blanket 2>/dev/null (F-S2104-P1-002a) ---
  # BC-6.26.001 v1.5 removed blanket 2>/dev/null; PC2c requires visible find exit codes.
  # M01(a): trailing-slash mandate retracted; M03(a): predicate widened -type f → ! -type d.
  # RED pre-implementation (doc has 2>/dev/null or wrong predicate); GREEN post-implementation.
  _assert_doc_marker 'find.*\.factory.*![[:space:]]*-type[[:space:]]+d' \
    "step-g-cleanup.md §G.1: find .factory ! -type d command present (BC-6.26.001 PC2; M01(a)/M03(a))" \
    "$g1_section"
  _assert_no_doc_marker 'find.*\.factory.*![[:space:]]*-type[[:space:]]+d.*2>/dev/null' \
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
  # F-S2104-P24-006 closure: gate changed from 'Write Discipline' (prose substring) to
  # '^#### Write Discipline' (heading form). The annotation relocated to _shared-context.md L64
  # at pass-23 contains the literal `#### Write Discipline` in backtick prose, which satisfied
  # the old marker regardless of whether the actual heading was present. The heading form
  # '^#### Write Discipline' requires a line starting with four # marks; the annotation starts
  # with '> ' so it does NOT satisfy this pattern — gate is load-bearing again.
  _assert_doc_marker '^#### Write Discipline' \
    "_shared-context.md §Spec-Path Discipline: #### Write Discipline child-heading present in heading form (BC-6.26.001 PC1, Invariant 1; F-S2104-P24-006)" \
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

  # --- DOC-PARITY §Spec-Path Discipline: AC-001(a) CWD-relative-path PROHIBITION (F-S2104-P12-003 .. F-S2104-P21-001) ---
  # BC-6.26.001 PC1 core: the Write Discipline section must state that CWD-relative paths are
  # FORBIDDEN and that canonical absolute paths are MANDATED. Twenty-three independently mutant-proven
  # gates (pass-16 adds negation-transparency, block-wide polarity, sentence-scoped Gate 2 +
  # retirement-language guard, Gate 3 tightened + Gate 7 CWD-relative bullet polarity, and
  # anchor-uniqueness bounded to #### Write Discipline; pass-17 adds whole-section domain,
  # HTML-comment absence, conditional-scoping gate, prohibition-token PW-B, tightened Gate 2a,
  # widened Gate 2b + adversative-connective gate, and canonical-target gate; pass-18 adds
  # balanced-fence well-formedness, fence-exclusion removal, boundary-rule splitter, rendered-domain
  # stripping, 2b domain parity + widened classes, and open-trigger write-directive gate; pass-19
  # adds clause-scoped write-directive domain, referent action class, boundary-completeness
  # assertion, CommonMark-correct link-ref-def strip, canonical-target domain widening, and
  # scope-restriction gate; pass-20 adds clause-scoped PW-B (F-S2104-P20-001), extended
  # write-directive referent covering artifact writes (F-S2104-P20-002), and PW-B
  # directive-requirement narrowing to exclude explanatory prose (F-S2104-P20-003); pass-21
  # unifies the directive class: Gate PW-B and write-directive gate now share a single
  # PWBD_DIRECTIVE_CLASS definition (F-S2104-P21-001), adding the bare-imperative alternation
  # (Anchor|Write|Save|Store|Place|Record|Emit|Persist|Resolve|Use, line-anchored ^) to PW-B
  # so bare-imperative mandates can no longer evade Gate PW-B by omitting a modal word;
  # F-S2104-P21-002 replaces the lexical prohibition exclusion (grep -Ev 'FORBIDDEN|...'
  # which dropped any clause containing a prohibition token, even when negated) with a
  # semantic exclusion that first neutralizes negated-prohibition phrases ('not (yet)?
  # forbidden', 'no longer forbidden', 'never previously prohibited') before checking for
  # remaining effective prohibition — closing the negated-prohibition escape confirmed by
  # orchestrator at 17921772; F-S2104-P21-003 adds a 'may'-regression-guard probe
  # (M-P21-E) since all existing M-P21-A/B/C fire via bare-imperative, not 'may';
  # SECONDARY 2 adds 'gitignored-shadow' to the prohibited-target alternation (BC-6.26.001
  # Invariant 5's canonical form)):
  #   (1) Paragraph-level extractor from rendered_write_discipline domain (F-S2104-P18-004(b)):
  #       #### Write Discipline → strip HTML comments + link-reference-definitions → normative
  #       prohibition paragraph, anchor: 'All `.factory/**` artifact writes…'
  #       + MANDATE POLARITY (SENTENCE-SCOPED + NEGATION-TRANSPARENT,
  #       F-S2104-P15-001 / F-S2104-P16-001(a)):
  #       - ANCHOR UNIQUENESS (F-S2104-P16-003(a), re-scoped to rendered domain per
  #         F-S2104-P18-004(b)): counts matches in rendered_write_discipline; decoy → count=2
  #         → RED; link-ref-def-hidden anchor → count=0 → RED.
  #       - BALANCED-FENCE ASSERTION (F-S2104-P18-002(a)): fence marker count in #### Write
  #         Discipline must be even; M-P18-C(b) unbalanced fence → gate fires → RED.
  #         Tilde (~~~) fences fail safe — not matched by ``` regex, included in prose domain.
  #       - HTML-COMMENT ABSENCE (F-S2104-P17-001(b), defense-in-depth per F-S2104-P18-004(c)):
  #         no <!-- in #### Write Discipline; M-P17-H moves mandate inside <!-- --> → RED.
  #       - MANDATE SENTENCE: extract sentence containing 'artifact writes' from rendered joined
  #         block; boundary-rule splitter (F-S2104-P18-003): split on \. followed by [A-Z*`\[]
  #         only — prevents 'No. ' false boundary that silenced Gates 4/5 (M-P18-B).
  #       - Gate 1(a): MUST[[:space:]]+use[[:space:]]+canonical absolute (zero-DoF).
  #       - Gate 1(b): mandate sentence must NOT contain MUST+NOT/never+canonical absolute.
  #       - Gate 1(c): mandate sentence must NOT contain prohibited-subject forms.
  #       - Gate 1(d): mandate sentence must NOT contain conditional scoping.
  #       MUTANT M-P18-D: [//]: # link-ref-def hides mandate → rendered domain strips it →
  #         prohibition_block absent → RED. Bare [label]: # form also → RED.
  #       MUTANT M-P18-B: 'No. 523' false boundary → old splitter: Gate 4 blind; new
  #         boundary-rule splitter ('5' is digit, no split) → Gate 4 fires → RED.
  #       CONTROL-B: same sentence without 'No. ' → Gate 4 fires (isolating mutant for Gate 4).
  #       MUTANT M-P18-C: harmful comment inside ``` fence → now in prose domain (fence not
  #         excluded per F-S2104-P18-002(b)) → PW-B fires → RED.
  #       MUTANT M-P18-C(b): unbalanced opening fence → balanced-fence assertion → RED.
  #       MUTANT (a): delete prohibition block → empty → RED.
  #       MUTANT (b): M-P15-A/M-P16-A "MUST NOT use canonical..." → Gates 1(a)+(b) fail → RED.
  #       Restore (c): mandate sentence "MUST use canonical absolute paths" → GREEN.
  #   (PW-B) SECTION-WIDE CLAUSE POLARITY (F-S2104-P16-001(b)/F-S2104-P17-002/
  #       F-S2104-P18-002(b)/F-S2104-P20-003): over write_discipline_prose_nosplit (whole ####
  #       Write Discipline section, fenced code NOW INCLUDED — fence exclusion removed per
  #       F-S2104-P18-002(b); adversary verified all four section gates empty over unexcluded
  #       pristine section).
  #       Directive-requirement (F-S2104-P20-003): gate fires ONLY on clauses that ALSO carry a
  #       directive token — explanatory prose mentioning prohibited targets (e.g., "such writes
  #       land silently in the story worktree's shadow .factory/ subtree") is out of scope. A
  #       clause missing a directive cannot mandate the use of a prohibited write path.
  #       For every DIRECTIVE clause containing a prohibited-target form, that clause MUST carry
  #       a prohibition token. Directive-token whitelist dropped (F-S2104-P17-002(a)); replaced
  #       by directive-requirement in F-S2104-P20-003 (narrowing, not widening).
  #       directive (PWBD_DIRECTIVE_CLASS, unified F-S2104-P21-001): MUST|SHOULD|required|
  #         is the required form|is preferred|is acceptable|permits|may|
  #         ^(\*\*[^:*]+:\*\*[[:space:]]+)?(Anchor|Write|Save|Store|Place|Record|Emit|Persist|
  #         Resolve|Use)[[:space:]] — shared with write-directive gate; line-anchored ^ for
  #         bare-imperative alternation (prevents mid-sentence false positives).
  #       prohibited-target: CWD-relative|worktree-relative|relative paths?|story-worktree CWD|
  #         worktree's shadow|worktree CWD|shadow subtree|[Ww]orktree-local|in-worktree|
  #         gitignored-shadow (added F-S2104-P21-002 SECONDARY 2 — BC-6.26.001 Invariant 5 term)
  #       prohibition (semantic, F-S2104-P21-002): FORBIDDEN|Forbidden|forbidden|MUST NOT|
  #         prohibited|never|forbid — BUT negated-prohibition phrases ('not (yet)? forbidden',
  #         'no longer forbidden', 'never previously prohibited') are neutralized first; only
  #         effective (non-negated) prohibition tokens cause exclusion
  #       M-P17-A S1 → RED (MUST + story worktree CWD); M-P17-C S2 → RED (clause 1: required +
  #         CWD-relative; clause 2: worktree's shadow excluded — no directive → GREEN);
  #       M-P18-C (in-fence harmful content with directive) → RED.
  #       M-P20-B (NEW): explanatory prose "Such writes land silently in the story worktree's
  #         shadow .factory/ subtree..." — prohibited-target + NO directive → GREEN (F-S2104-P20-003
  #         load-bearing proof: directive filter excludes it).
  #       M-P20-C (NEW): same prose + "MUST" → "Such writes MUST land in the story worktree's
  #         shadow .factory/ subtree..." — prohibited-target + MUST (directive), no prohibition
  #         → RED (proves directive requirement is load-bearing; removing MUST would restore GREEN).
  #       M-P21-A (NEW): "Anchor every write to the story worktree CWD." — prohibited-target
  #         'story worktree CWD' + 'Anchor' (bare-imperative, new in unified class), no prohibition
  #         → RED ✓ (closes P20-003 regression; CONTROL-5 proves GREEN via old class).
  #       M-P21-B (NEW): "Resolve all delivery paths from the story worktree CWD." — 'story
  #         worktree CWD' + 'Resolve' (bare-imperative) → RED ✓.
  #       M-P21-C (NEW): "Place each report in the worktree's shadow subtree." — 'worktree's
  #         shadow' + 'Place' (bare-imperative) → RED ✓.
  #   (F-P18-001) WRITE-DIRECTIVE GATE (POSITIVE, open-trigger; F-S2104-P18-001/F-S2104-P18-005(d)/
  #       F-S2104-P21-001 unified class):
  #       Every clause containing a directive (PWBD_DIRECTIVE_CLASS, shared with Gate PW-B per
  #       F-S2104-P21-001) AND a referent (.factory/|ledger|artifact writes?) MUST either carry
  #       a prohibition token OR match 'MUST use canonical absolute'. Open-trigger design:
  #       the trigger covers any directive+referent combination, requirement is the escape clause —
  #       closes PW-B's named-destination limitation with a form-based gate.
  #       Alternation-direction for trigger class: (b) open — any directive+referent clause
  #       is in scope; escape clause is the load-bearing constant.
  #       Adversary verified empty-on-pristine and firing-on-M-P18-A by literal shell.
  #       M-P18-A: "Writers MUST anchor ... to the worktree's .factory/ subtree" → RED.
  #       Control: replace with "MUST use canonical absolute" → GREEN (escape is load-bearing).
  #   (2a) Gate 2 SENTENCE-SCOPED TIGHTENED (F-S2104-P16-001(c)/F-S2104-P17-002(d)): at least
  #       one sentence must match (CWD-relative|worktree-relative)[^.]*FORBIDDEN — the
  #       prohibited-subject form AND FORBIDDEN co-occur within the sentence boundary.
  #       CONTROL-B is also Gate 4's isolating mutant (absolute+FORBIDDEN without Gate 1(a)).
  #   (2b) NULLIFICATION CLASS + ADVERSATIVE CONNECTIVE (F-S2104-P16-001(c)/F-S2104-P17-003/
  #       F-S2104-P18-005): over write_discipline_prose_nosplit. (a) must NOT contain
  #       nullification language — widened (F-S2104-P18-005(c)) to include: supplanted|
  #       supersede|does not bind|does not govern|no longer binds|descriptive only|advisory
  #       only|pre-#NNN. Alternation-direction: (b) backed by open-trigger write-directive gate
  #       (F-S2104-P18-001). M-P18-E 'does not bind' → (a) fires → RED; M-P18-F 'supplanted'
  #       → (a) fires → RED. (c) Section-wide (domain re-scoped from joined_block_nosplit per
  #       F-S2104-P18-005(a)): any sentence matching the prohibition-reference trigger
  #       (FORBIDDEN|forbidden|prohibition|prohibited|the rule|this rule|the constraint|above)
  #       must NOT contain an adversative connective — widened (F-S2104-P18-005(b)) to include:
  #       whereas|nevertheless|that said|in practice|notwithstanding. Alternation-direction:
  #       (b) backed by write-directive gate. M-P18-G 'however ... permits' → (c) fires → RED.
  #   (3) Gate 3 TIGHTENED (F-S2104-P16-002): **Forbidden:** + file_path="\.factory/ + relative
  #       path on same line. M-P16-D swaps labels → RED.
  #   (4) NEGATIVE (WHOLE-SECTION, F-S2104-P15-001/F-S2104-P17-001/F-S2104-P18-003): over
  #       write_discipline_prose_nosplit (boundary-rule splitter, fence content included).
  #       NO sentence where 'absolute' co-occurs with 'FORBIDDEN'. CONTROL-B is the isolating
  #       mutant for Gate 4. M-P18-B (old splitter: blind; new boundary-rule: fires) → RED.
  #   (5) NEGATIVE (WHOLE-SECTION, F-S2104-P15-001/F-S2104-P17-001): over
  #       write_discipline_prose_nosplit. NO sentence where 'MUST' co-occurs with prohibited-
  #       subject form. Isolating mutant: "Writers MUST use relative paths for ledger writes."
  #       (sibling para, mandate intact) → Gate 5 fires alone → RED. NOTE: M-P17-A fires Gate
  #       PW-B (story worktree CWD without prohibition token) but does NOT fire Gate 5
  #       independently — 'story worktree CWD' is not in Gate-5's alternation (CWD-relative|
  #       worktree-relative|relative path); F-S2104-P18-006 correction.
  #   (6) Gate 6: (a) POSITIVE: **Forbidden:** + ../ must exist; (b) → canonical-target gate.
  #   (7) Gate 7: (a) POSITIVE: **Forbidden:** + file_path="\.factory/ must exist;
  #       (b) → canonical-target gate.
  #   (canonical-target) NEGATIVE (F-S2104-P17-004): no **Correct:** bullet with non-canonical
  #       file_path=. M-P17-G, M-P15-B, M-P16-D all → RED.
  # All twenty-three gates survive independently.

  # Whole-section domain extraction (needed before rendered domain and prose domain).
  local write_discipline_section
  write_discipline_section="$(_extract_write_discipline_section)"

  # Gate: no HTML comment spans in #### Write Discipline (F-S2104-P17-001(b), defense-in-depth
  # retained per F-S2104-P18-004(c) — complements rendered_write_discipline; fires first on
  # M-P17-H before the rendered-domain extraction can be evaluated).
  if printf '%s\n' "$write_discipline_section" | grep -qE '<!--'; then
    echo "DOC-PARITY FAIL [write-discipline comment-hidden normative text (F-S2104-P17-001(b))]: the #### Write Discipline section contains an HTML comment span (<!--) — normative mandate or prohibition text hidden inside <!-- --> is not rendered by Markdown renderers, yet positive gates (1(a), 2a) can match it; M-P17-H moves the compliant mandate inside <!-- --> and makes the only visible instruction a CWD-relative write; all normative text must be visible (BC-6.26.001 PC1; AC-001(a))"
    false
  fi

  # F-S2104-P18-002(a): balanced-fence WELL-FORMEDNESS assertion — the #### Write Discipline
  # section must contain paired fence delimiters (even backtick-fence count). An unpaired fence
  # is a Markdown structural defect; this assertion prevents a malformed section from being
  # authored silently. **No gate domain is fence-aware at HEAD** (the fence-stripping awk was
  # removed at commit c89bef22 per F-S2104-P18-002(b)); this is a well-formedness invariant,
  # NOT a truncation guard. Historical note (F-S2104-P19-011 correction): before c89bef22 the
  # fence-stripping awk used an in_fence state variable; an unbalanced opening fence left it
  # stuck at in_fence=1, silently dropping the section remainder from write_discipline_prose and
  # making PW-B/2b(a)/4/5 blind; M-P18-C(b) proved that attack at 9/9. That mechanism no
  # longer exists; the assertion remains because odd fence count is still a real defect.
  local fence_count
  fence_count="$(printf '%s\n' "$write_discipline_section" | grep -cE '^[[:space:]]*```' || true)"
  if [ $(( fence_count % 2 )) -ne 0 ]; then
    echo "DOC-PARITY FAIL [unbalanced code fence in gated section (F-S2104-P18-002(a))]: the #### Write Discipline section has an odd fence-marker count (${fence_count}) — an unbalanced opening fence drops the entire remainder of the section from any fence-aware domain; M-P18-C(b) inserts one opening fence after the prohibition paragraph (3 total) and silences all four section-wide negative gates at 9/9; removing that fence restores balance (BC-6.26.001 PC1; AC-001(a); F-S2104-P18-002(a))"
    false
  fi

  # F-S2104-P18-004(a): rendered_write_discipline domain — strip non-rendered Markdown before
  # any positive gate evaluates. Two classes of non-rendered content:
  #   (1) HTML comment spans <!-- … -->: stripped via sed (single-line form; multi-line not
  #       expected; the <!-- absence gate fires first and prevents them from reaching here).
  #   (2) Link-reference definitions: dropped via grep. CommonMark makes whitespace after the
  #       colon OPTIONAL — both `[//]: # (…)` and `[//]:# (…)` are valid definitions rendered
  #       as nothing. Drop predicate: `^[[:space:]]{0,3}\[[^]]*\]:` (colon-terminated, no
  #       whitespace requirement). F-S2104-P19-005 correction: the pass-18 predicate required
  #       `[[:space:]]` after the colon, so `[//]:# (…)` survived stripping and M-P19-F hid
  #       the whole mandate in it at 9/9; CONTROL-F (one space added) was RED, differing by
  #       one byte. Verified by literal shell: `marked` and `pandoc -f commonmark` both emit
  #       nothing for `[//]:# (…)`. Verified: pristine Write Discipline is byte-identical after
  #       the corrected strip (no line in the section matches the label form, so rendered domain
  #       equals raw section modulo comment sed).
  # M-P18-D substitutes the prohibition paragraph with [//]: # (…) — Gates 1(a) and 2a find
  # the compliant mandate inside the definition and pass; the rendered section's only visible
  # instruction directs writes to the delivery sandbox root at 9/9.
  # M-P19-F: [//]:# (…) no-space form hides the whole mandate — corrected predicate drops it.
  # MUTANT M-P19-F: [//]:# (HIDDEN MANDATE TEXT) — mandate stripped → anchor count 0 → RED ✓.
  # CONTROL-F: identical but with space after colon ([//]: # form) → RED via anchor count 0 ✓
  #   (CONTROL-F is the pass-18 M-P18-D form; both forms are now stripped).
  # GREEN control: pristine section byte-identical after stripping → Gate 1(a) still passes ✓.
  local rendered_write_discipline
  rendered_write_discipline="$(printf '%s\n' "$write_discipline_section" | \
    sed 's/<!--[^>]*-->//g' | \
    grep -Ev '^[[:space:]]{0,3}\[[^]]*\]:')"

  # Anchor uniqueness gate: #### Write Discipline must have exactly one prohibition anchor in
  # the rendered domain (F-S2104-P16-003(a) re-scoped to rendered domain per F-S2104-P18-004(b)).
  # Re-scoping ensures a link-ref-def-hidden anchor cannot satisfy the positive assertion while
  # the visible text carries a competing mandate.
  local rendered_anchor_count
  rendered_anchor_count="$(printf '%s\n' "$rendered_write_discipline" | awk '/All.*\.factory.*artifact writes/ {count++} END {print count+0}')"
  if [ "$rendered_anchor_count" -ne 1 ]; then
    echo "DOC-PARITY FAIL [ambiguous anchor in #### Write Discipline rendered domain (F-S2104-P16-003(a)/F-S2104-P18-004(b))]: found ${rendered_anchor_count} match(es) of anchor 'All.*\\.factory.*artifact writes' in the rendered #### Write Discipline domain (expected exactly 1); decoy paragraph → count=2 → RED; link-ref-def-hidden anchor → count=0 → RED (BC-6.26.001 PC1; AC-001(a))"
    false
  fi

  # Extract prohibition block from rendered domain (F-S2104-P18-004(b) re-scope).
  local prohibition_block
  prohibition_block="$(printf '%s\n' "$rendered_write_discipline" | awk '
    /All.*\.factory.*artifact writes/ { found=1 }
    found && /^$/ { exit }
    found { print }
  ')"

  if [ -z "$prohibition_block" ]; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block absent]: _shared-context.md Write Discipline prohibition paragraph ('All .factory/** artifact writes...MUST use...absolute paths...CWD-relative...FORBIDDEN') not found in rendered domain — block deleted, heading changed, or entire paragraph hidden in link-ref-def / HTML comment (BC-6.26.001 PC1; AC-001(a); F-S2104-P13-001/F-S2104-P18-004)"
    false
  fi

  # Build section-wide prose domain: ALL section content included (F-S2104-P18-002(b)).
  # Fenced code exclusion removed — the removal is motivated above at the balanced-fence
  # assertion comment: the exclusion's original justification no longer holds at HEAD, and the
  # exclusion is an unconditional fail-open that M-P18-C exploits. All negative gates verified
  # empty over the unexcluded pristine section by adversary literal shell.
  # Abbreviation-protect cf./i.e./e.g. before sentence-splitting (F-S2104-P16-001 M-P16-C2
  # hardening: cf. dot creates a false boundary that masks CWD-relative from Gate 1(c)/Gate 5).
  # F-S2104-P23-001 + F-S2104-P24-001 closure: _build_section_prose applies the recursive
  # blockquote-marker strip (handles '>>' double-blockquote) and list-marker normalisation.
  # The old inline strip consumed only one '>' per line; '>> Anchor...' retained '>' after
  # stripping, defeating the ^-anchored bare-imperative class (F-S2104-P24-001).
  # F-S2104-P24-002 closure: spec_path_prose now built via _build_section_prose (same marker
  # strip as write_discipline_prose), not the old bare 'tr' which left '>' markers intact.
  # The pass-23 relocation of the authoring-constraint annotation to _shared-context.md L64
  # placed it in ### Spec-Path Discipline above #### Write Discipline — inside spec_path_section
  # but outside write_discipline_section. The annotation used '>' prefix; without the strip
  # the spec_path_prose domain retained the '>' marker, preventing ^-anchored detection in the
  # write-directive gate. _build_section_prose strips both domains identically (TD-VSDD-060
  # sibling-site sweep: write_discipline_prose and spec_path_prose must both be normalised).
  # PWBD_DIRECTIVE_CLASS: moved to file scope (defined above all @test blocks) so the pipeline
  # probe shares the identical constant — no local redefinition here (F-S2104-P24-003).
  local write_discipline_prose
  write_discipline_prose="$(_build_section_prose "$write_discipline_section")"
  local write_discipline_prose_nosplit
  write_discipline_prose_nosplit="$(_build_nosplit "$write_discipline_prose")"

  # Domain for write-directive gate (F-S2104-P19-002): whole ### Spec-Path Discipline section.
  # The write-directive gate reads this domain; PW-B/2b/4/5 remain bounded to write_discipline_prose_nosplit.
  # Naive widening of PW-B/2b/4/5 false-positives on two read-discipline sentences above the
  # #### Write Discipline heading (adversary verified pristine-empty for write-directive gate only).
  # F-S2104-P24-002: spec_path_prose now uses _build_spec_path_section_prose (marker strip applied).
  # Calls the shared wrapper (not _build_section_prose directly) so the pipeline probe's Leg C
  # depends on the same call site — reverting the wrapper to bare 'tr' breaks the probe (F-S2104-P24-003).
  local spec_path_prose
  spec_path_prose="$(_build_spec_path_section_prose "$spec_path_section")"
  local spec_path_prose_nosplit
  spec_path_prose_nosplit="$(_build_nosplit "$spec_path_prose")"

  # Boundary-completeness assertion (F-S2104-P19-004(b)): verifies the sentence splitter fires
  # on every '. [A-Z*`\[]' boundary it should split. The primary fix for the missed-boundary
  # direction (M-P19-D: lowercase-initial merged sentence) is the clause-scoping of the
  # write-directive gate per F-S2104-P19-001(a): once exclusions are clause-scoped, a merged
  # clause no longer inherits its neighbour's prohibition token. This assertion is a regression
  # guard for the false-boundary direction: if the splitter regex breaks and fails to split a
  # '. [A-Z*`\[]' occurrence, splits_made < expected_splits → fires.
  # M-P19-D (CONTROL-D): capital-G variant ("Git-resolved...") is split by the splitter (splits
  #   before 'G') → PW-B sees the sentence independently → RED ✓.
  # Verified empty-on-pristine: expected_splits=13 equals actual_splits=13 for #### Write Discipline.
  local bc_expected_splits bc_actual_splits
  bc_expected_splits="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    grep -oE '\.[[:space:]]+[A-Z*`\[]' | wc -l | tr -d ' ')"
  bc_actual_splits="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | wc -l | tr -d ' ')"
  if [ "$((bc_actual_splits - 1))" -lt "$bc_expected_splits" ]; then
    echo "DOC-PARITY FAIL [boundary-completeness: sentence splitter missed a '. [A-Z*\`\[] boundary (F-S2104-P19-004(b))]: the boundary-rule sentence splitter ('perl -pe s/\\.[[:space:]]+(?=[A-Z*\`\\[])') produced fewer splits ($((bc_actual_splits - 1))) than the count of '. [A-Z*\`\[] boundary patterns ($bc_expected_splits) in write_discipline_prose_nosplit — the splitter regex may be broken or a boundary was silently skipped; CONTROL-D (capital G) fires Gate PW-B independently and serves as the missed-boundary mutant (BC-6.26.001 PC1; AC-001(a); F-S2104-P19-004)"
    false
  fi

  # Reflow the prohibition block to a single joined line for sentence-scoped evaluation.
  # Gates 1(a/b/c/d) and 2a use joined_block_nosplit scoped to the rendered prohibition paragraph.
  # Gates PW-B, 2b, 4, 5 use write_discipline_prose_nosplit (whole section, fence included).
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
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | grep 'artifact writes' | head -1)"
  if [ -z "$mandate_sentence" ]; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block mandate-sentence absent]: the normative mandate sentence containing 'artifact writes' was not found after sentence-splitting the joined prohibition block (boundary-rule splitter: split on '. ' only before [A-Z*\`\[]; prevents false boundary on 'No. NNN'); block may be missing or sentence structure changed (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001/F-S2104-P18-003)"
    false
  fi
  # Gate 1(a): zero-DoF MUST use canonical absolute — 'MUST NOT use canonical' cannot satisfy
  # (F-S2104-P16-001(a): prior MUST[^.]*use[^.]*canonical passed M-P16-A because [^.]* spans NOT)
  printf '%s\n' "$mandate_sentence" | grep -qE 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute' || {
    echo "DOC-PARITY FAIL [write-discipline prohibition block affirmative-mandate (sentence-scoped, zero-DoF, F-S2104-P16-001(a))]: the mandate sentence (containing 'artifact writes') must contain 'MUST use canonical absolute' (zero-DoF: no tokens between MUST and use) — the prior MUST[^.]*use[^.]*canonical pattern passed M-P16-A 'MUST NOT use canonical absolute' because [^.]* spans the negation token; this tightening closes that bypass (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001 / F-S2104-P14R-001)"
    false
  }
  # Gate 1(e): anchor-target gate — mandate sentence MUST name a canonical anchor target
  # (main-checkout|CANONICAL_FACTORY_ROOT) (F-S2104-P26-B01).
  # B01 mutant: "anchored to the story worktree's own .factory/ subtree" — passes Gate 1(a)
  # (MUST use canonical absolute intact) but lacks canonical anchor → Gate 1(e) fires → RED ✓.
  # CONTROL: "anchored to the main-checkout root" → matches main-checkout → GREEN ✓.
  printf '%s\n' "$mandate_sentence" | grep -qE 'main-checkout|\$CANONICAL_FACTORY_ROOT' || {
    echo "DOC-PARITY FAIL [write-discipline prohibition block anchor-target absent (Gate 1(e), F-S2104-P26-B01)]: the mandate sentence must name a canonical anchor target — 'main-checkout' or '\$CANONICAL_FACTORY_ROOT'; the B01 mutant 'anchored to the story worktree's own .factory/ subtree' passes Gate 1(a) (MUST use canonical absolute intact) but names a non-canonical anchor target — Gate 1(e) fires (BC-6.26.001 PC1; AC-001(a))"
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
  # Gate 1(f): negative anchor-target gate — mandate sentence MUST NOT name a worktree anchor
  # (F-S2104-P26-B01).
  # B01 mutant: "anchored to the story worktree's own .factory/ subtree" → 'anchored to' followed
  # by 'worktree' fires this gate → RED ✓.
  # CONTROL: "anchored to the main-checkout root" → 'anchored to' present but 'worktree' absent
  # in the remainder → GREEN ✓.
  if printf '%s\n' "$mandate_sentence" | grep -qE 'anchored[[:space:]]+to[^.]*worktree'; then
    echo "DOC-PARITY FAIL [write-discipline prohibition block non-canonical anchor (Gate 1(f), F-S2104-P26-B01)]: the mandate sentence names a worktree anchor target — 'anchored to[^.]*worktree' — which anchors writes to the story worktree rather than the main checkout; B01 mutant 'anchored to the story worktree's own .factory/ subtree' fires this gate (BC-6.26.001 PC1; AC-001(a))"
    false
  fi

  # Gate PW-B (SECTION-WIDE CLAUSE POLARITY, F-S2104-P16-001(b) strengthened F-S2104-P17-002/
  #     F-S2104-P20-001/F-S2104-P20-003): clause-scoped per F-S2104-P20-001. Evaluates the whole
  #     #### Write Discipline section (write_discipline_prose_nosplit, fenced code included —
  #     F-S2104-P18-002(b)) rather than only the prohibition paragraph.
  # Clause-scoped (F-S2104-P20-001): after sentence splitting, split further on [;—] and on
  #   ,\s+(and|or|but)\s+. Each clause evaluated independently. Sentence-scoped escape (pass-17
  #   through pass-19) allowed M-P20-A (semicolon-separated prohibition clause escaping prohibited-
  #   target clause) to evade PW-B at 9/9.
  # Directive-requirement (F-S2104-P20-003): gate fires ONLY on clauses that also carry a
  #   directive token (MUST|SHOULD|required|is the required form|is preferred|is acceptable|
  #   permits). Explanatory prose that mentions prohibited targets without commanding their use
  #   (e.g., "such writes land silently in the story worktree's shadow .factory/ subtree and are
  #   permanently destroyed at teardown") is correctly excluded — it describes consequences, not
  #   mandates. Contrast: the write-directive gate handles this correctly via its own directive
  #   trigger; PW-B now applies the same principle. The em-dash in the prohibition sentence was
  #   changed to a period by F-S2104-P20-001(b) to suppress the 'worktree's shadow' false-positive
  #   on the continuation clause; with F-S2104-P20-003 that workaround is no longer needed, and
  #   the natural em-dash form is restored to _shared-context.md (doc change in this commit).
  # For every DIRECTIVE clause containing a prohibited-target form, that clause MUST carry a
  # prohibition token. Directive-token whitelist dropped (F-S2104-P17-002(a)): explanatory prose
  # is now excluded structurally via directive-requirement, not by listing permitted tokens.
  # Unified directive class (F-S2104-P21-001): PWBD_DIRECTIVE_CLASS is now shared with the
  # write-directive gate — single definition, single callsite per gate. Adds bare-imperative
  # alternation (line-anchored) to PW-B so P2-class mandates ("Anchor...", "Resolve...",
  # "Place...") that omit a modal word can no longer evade PW-B.
  # directive (PWBD_DIRECTIVE_CLASS): MUST|SHOULD|required|is[[:space:]]+the[[:space:]]+required|
  #   is[[:space:]]+preferred|is[[:space:]]+acceptable|permits|may|
  #   ^(\*\*[^:*]+:\*\*[[:space:]]+)?(Anchor|Write|Save|Store|Place|Record|Emit|Persist|Resolve|Use)[[:space:]]
  # prohibited-target: CWD-relative|worktree-relative|relative[[:space:]]+paths?|
  #                    story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|
  #                    worktree's[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|
  #                    [Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow
  # NOTE: '[Ww]orktree-local' uses bracket-class for sentence-initial capital W — 'Worktree-local'
  #   at the start of a sentence (after '. ' splitter) is not caught by lowercase-only 'worktree-local'.
  # NOTE: 'in-worktree' added with word-boundary-safe predicate (^|[^[:alnum:]])[Ii]n-worktree —
  #   bare 'in-worktree' was excluded because '<main-worktree-path>' has 'n' (alphanumeric) before
  #   'in-worktree'; the word-boundary form fails on that template placeholder while catching
  #   standalone [Ii]n-worktree at sentence start or after space (F-S2104-P17-002(b) completion).
  # NOTE: 'gitignored-shadow' added (F-S2104-P21-002 SECONDARY 2): BC-6.26.001 Invariant 5 uses
  #   this as canonical terminology ('gitignored-shadow mechanism'); a clause using BC's own phrasing
  #   could escape the prohibited-target filter without it. Checked canonical forms in BC-6.26.001,
  #   _shared-context.md, and step-g-cleanup.md: shadow subtree COVERED, worktree's shadow COVERED,
  #   gitignored-shadow NOT covered (now added), shadow .factory/ covered by write-directive referent.
  # prohibition (FAIL-CLOSED WHITELIST, F-S2104-P21-002 structural fix): a clause carrying
  #   BOTH a prohibited-target form AND a directive token is a VIOLATION BY DEFAULT — exempted
  #   ONLY if it matches the whitelist of known-correct prohibition constructions derived
  #   empirically from the pristine _shared-context.md #### Write Discipline section.
  #   Whitelist: grep -vE '\*\*Forbidden:\*\*'
  #     — covers **Forbidden:** Write(...) bullet labels. Verified: these are the ONLY pristine
  #     constructions in the section that carry both a prohibited-target ('relative path') and a
  #     directive-class token. In practice, the **Forbidden:** Write(...) bullets do NOT pass the
  #     directive filter (the backtick-quoted 'Write' and the leading '- ' break the line-anchor
  #     match), so the whitelist is a forward-safety measure — no currently-pristine clause
  #     requires it. All other constructions fire by default.
  #   Whitelist derivation (with file/line citations):
  #     - _shared-context.md L112: '- **Forbidden:** `Write(file_path=".factory/...")` (relative path...)'
  #       has prohibited-target 'relative path' + directive 'Write' (via bare-imperative class).
  #       The backtick and leading '- ' break the line-anchor; directive filter excludes it.
  #     - _shared-context.md L113: '- **Forbidden:** `Write(file_path="../../.factory/...")` (relative traversal...)'
  #       same analysis — directive filter excludes.
  #     - No other pristine clauses pass both prohibited-target + directive filters (verified by
  #       literal shell: printf '%s\n' "$write_discipline_prose_nosplit" | ... | grep prohibited |
  #       grep directive — returns empty on pristine doc).
  #   Why fail-closed is structurally different from vocabulary enumeration (TD-VSDD-059):
  #     Under the OLD approach the exclusion step enumerated specific negator prefixes
  #     ('not yet', 'no longer', 'never previously') — attacker evades with any unenumerated
  #     form ('not explicitly', 'isn't', 'hardly', 'far from', etc.). Under fail-closed the
  #     whitelist enumerates the small, closed set of LEGITIMATE constructions — attacker's novel
  #     form does NOT match the whitelist and therefore FIRES. The unbounded space works against
  #     the attacker, not for them (POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION).
  # M-P17-A S1: "Writers MUST anchor every .factory/** artifact write to the story worktree CWD"
  #   — 'story worktree CWD' (prohibited-target) + 'MUST' (directive), no **Forbidden:** → RED ✓.
  # M-P17-C S2: "For in-worktree ledgers, CWD-relative paths are the required form, and they
  #   land in the story worktree's shadow .factory/ subtree" — after ', and ' clause split:
  #   Clause 1: 'CWD-relative' (prohibited-target) + 'required' (directive), no **Forbidden:** → RED ✓.
  #   Clause 2: 'worktree's shadow' (prohibited-target) + NO directive → excluded by directive
  #     requirement → not a violation (directive-requirement is load-bearing here).
  #   Overall: RED ✓ (clause 1 fires).
  # Correct text (doc restoration, F-S2104-P20-003): "CWD-relative paths...are FORBIDDEN — such
  #   writes land silently in the story worktree's shadow .factory/ subtree and are permanently
  #   destroyed at teardown." — After '—' clause split: clause 1 ('CWD-relative paths...are
  #   FORBIDDEN') has no directive token → excluded by directive-requirement → GREEN; clause 2
  #   has 'worktree's shadow' but NO directive → excluded → GREEN ✓.
  # Correct bullet "**Forbidden:** `Write(file_path=".factory/...")`..." — clause 1 (after '—'
  #   split) has 'relative path' but directive filter fails (backtick + leading '- ' break
  #   line-anchor) → excluded by directive-requirement → GREEN. Whitelist (**Forbidden:**) is
  #   forward-safety for any future unquoted Write form.
  #   (escape-load-bearing: no directive in clause 1 → excluded by directive-requirement → GREEN.
  #   NOTE: **Forbidden:** bullets with no directive pass PW-B; caught by Gate 3/7/canonical-target.)
  # M-P20-A (F-S2104-P20-001 closure): "Writers MUST anchor every artifact write to the story
  #   worktree CWD; duplicating the ledger onto the main checkout is forbidden."
  #   Clause 1 (before ;): 'story worktree CWD' + 'MUST' (directive), no **Forbidden:** → RED ✓.
  #   Clause 2: no prohibited-target → not evaluated → not a violation.
  # CONTROL-1 (F-S2104-P20-001 target-matching): M-P20-A with 'forbidden'→'discouraged'.
  #   Clause 1: 'story worktree CWD' + 'MUST' (directive), no **Forbidden:** → RED ✓.
  # M-P20-B (F-S2104-P20-003 directive-requirement, GREEN control): "Such writes land silently
  #   in the story worktree's shadow .factory/ subtree and are permanently destroyed at teardown."
  #   — prohibited-target 'worktree's shadow' present, NO directive token → excluded by
  #   directive-requirement → GREEN ✓. This proves the directive filter is load-bearing.
  # M-P20-C (F-S2104-P20-003 directive-requirement, RED paired mutant): "Such writes MUST land
  #   in the story worktree's shadow .factory/ subtree and are permanently destroyed at teardown."
  #   — prohibited-target 'worktree's shadow' + 'MUST' (directive), no **Forbidden:** → RED ✓.
  local polarity_violations
  polarity_violations="$(printf '%s\n' "$write_discipline_prose_nosplit" | perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -n "$polarity_violations" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide clause polarity (Gate PW-B, F-S2104-P16-001(b)/F-S2104-P17-002/F-S2104-P20-001/F-S2104-P20-003/F-S2104-P21-001/F-S2104-P21-002)]: a DIRECTIVE clause in the Write Discipline section contains a prohibited-target form (CWD-relative|worktree-relative|relative paths?|story-worktree CWD|story worktree CWD|worktree's shadow|worktree CWD|shadow subtree|[Ww]orktree-local|in-worktree|gitignored-shadow) without an effective prohibition token — clause-scoped per F-S2104-P20-001; directive-requirement per F-S2104-P20-003; unified directive class per F-S2104-P21-001 (bare-imperative Anchor|Write|...); FAIL-CLOSED WHITELIST per F-S2104-P21-002: violation by default unless clause matches **Forbidden:** bullet label — all negated-prohibition forms fire because they do not match the whitelist (not explicitly/strictly/currently/entirely forbidden, isn't forbidden, hardly forbidden, not at present, far from forbidden, contraction haven't been prohibited, FORBIDDEN-on-different-subject, interposed parenthetical, double negative, and any form not listed here) (BC-6.26.001 PC1; AC-001(a))"
    printf '%s\n' "$polarity_violations"
    false
  fi

  # M-P21-A (F-S2104-P21-001 new vector): P2 "Anchor every write to the story worktree CWD."
  #   prohibited-target 'story worktree CWD' + bare 'Anchor' (now in unified directive class) →
  #   Gate PW-B fires → RED ✓. The word 'Anchor' is line-initial → matches the line-anchored
  #   bare-imperative alternation in PWBD_DIRECTIVE_CLASS.
  #   Evaded Gate PW-B at 9/9 (P20-003 regression): prior directive class lacked bare-imperative
  #   alternation; 'Anchor' matched neither MUST/SHOULD/required/.../permits → gate SILENT.
  #   Evaded write-directive gate: no .factory/, ledger, or artifact writes? referent in P2 → SILENT.
  # M-P21-B (F-S2104-P21-001 new vector): P3 "Resolve all delivery paths from the story worktree CWD."
  #   prohibited-target 'story worktree CWD' + bare 'Resolve' (unified directive class) → RED ✓.
  # M-P21-C (F-S2104-P21-001 new vector): P4 "Place each report in the worktree's shadow subtree."
  #   prohibited-target "worktree's shadow" + bare 'Place' (unified directive class) → RED ✓.
  #   NOTE: P4 evaded PW-B because 'Place' was not in the prior directive class; 'worktree's shadow'
  #   WAS in the prohibited-target class. So P4 was one gate (step 1) away from being caught — only
  #   the directive-class gap prevented it. The unified class closes that gap for P4 specifically.
  # CONTROL-5 (F-S2104-P21-001 load-bearing GREEN): P2 probe through OLD directive class only
  #   (without bare-imperative alternation). Gate returns empty (GREEN) → proves bare-imperative
  #   class is the sole load-bearing addition for P2; the prohibited-target filter already matched.
  local mp21a_violations mp21b_violations mp21c_violations control5_result
  mp21a_violations="$(printf '%s\n' 'Anchor every write to the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21a_violations" ]; then
    echo "MUTANT FAIL [M-P21-A (F-S2104-P21-001)]: probe P2 'Anchor every write to the story worktree CWD.' must fire Gate PW-B — unified directive class must match bare-imperative 'Anchor' when clause also contains prohibited-target 'story worktree CWD'; got empty (gate SILENT — imperative class not applied correctly)"
    false
  fi

  mp21b_violations="$(printf '%s\n' 'Resolve all delivery paths from the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21b_violations" ]; then
    echo "MUTANT FAIL [M-P21-B (F-S2104-P21-001)]: probe P3 'Resolve all delivery paths from the story worktree CWD.' must fire Gate PW-B — 'Resolve' is a bare-imperative in the unified class; 'story worktree CWD' is a prohibited-target; no prohibition token → gate must fire RED"
    false
  fi

  mp21c_violations="$(printf '%s\n' "Place each report in the worktree's shadow subtree." | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21c_violations" ]; then
    echo "MUTANT FAIL [M-P21-C (F-S2104-P21-001)]: probe P4 'Place each report in the worktree's shadow subtree.' must fire Gate PW-B — 'Place' is a bare-imperative in the unified class; 'worktree's shadow' is a prohibited-target; no prohibition token → gate must fire RED"
    false
  fi

  # CONTROL-5 (F-S2104-P21-001 load-bearing GREEN): prove the bare-imperative class is the
  # load-bearing addition by running the OLD directive class (no bare-imperative alternation)
  # against P2 probe. Expected: empty (GREEN) — prohibited-target matches but old directive
  # class misses 'Anchor' → gate SILENT, proving the new bare-imperative term is what closes P2.
  control5_result="$(printf '%s\n' 'Anchor every write to the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E 'MUST|SHOULD|required|is[[:space:]]+the[[:space:]]+required|is[[:space:]]+preferred|is[[:space:]]+acceptable|permits' | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -n "$control5_result" ]; then
    echo "CONTROL FAIL [CONTROL-5 (F-S2104-P21-001)]: P2 probe 'Anchor every write to the story worktree CWD.' fired the OLD directive class (MUST|SHOULD|required|...|permits, WITHOUT bare-imperative alternation) — expected GREEN (empty) to prove the bare-imperative class is the load-bearing addition; if the old class already caught P2, the finding premise is wrong; got: $control5_result"
    false
  fi

  # M-P21-D (F-S2104-P21-002 closure — confirmed evasion vector at 17921772):
  #   "not yet forbidden" embeds a negated prohibition token with directive 'may' and
  #   prohibited-target 'story worktree CWD'. Under the OLD vocabulary exclusion, 'forbidden'
  #   caused SILENT/GREEN. Under fail-closed whitelist, no **Forbidden:** match → FIRES (RED).
  #   CONTROL-D is the base probe without any negation — also FIRES (no **Forbidden:**).
  # M-P21-E (F-S2104-P21-003 — 'may' regression guard, SECONDARY 1): proves 'may' is load-bearing
  #   in PWBD_DIRECTIVE_CLASS; M-P21-A/B/C fire via bare-imperative so deleting 'may' would not
  #   be caught. Fires via fail-closed: no **Forbidden:** match.
  # M-P21-F/G/H: negation forms from the original enumerated set — now fire via fail-closed
  #   (no **Forbidden:** match) rather than via explicit neutralization.
  # M-P21-GS (gitignored-shadow, SECONDARY 2): BC-6.26.001 Invariant 5 canonical phrasing.
  # M-P21-I through M-P21-P (FAIL-CLOSED COVERAGE — 8 orchestrator-supplied evading forms that
  #   escaped vocabulary enumeration at b7d3ca58):
  #   I: "not explicitly forbidden" — 'explicitly' between 'not' and 'forbidden' evaded enumeration
  #   J: "not strictly forbidden" — 'strictly' between
  #   K: "not currently forbidden" — 'currently' between
  #   L: "not entirely forbidden" — 'entirely' between
  #   M: "isn't forbidden" / "aren't forbidden" — contraction, 'n't' not matched by \bnot\b
  #   N: "hardly forbidden" — different negation idiom, no 'not' word
  #   O: "not, at present, forbidden" — interposed comma phrase
  #   P: "far from forbidden" — idiomatic, no 'not' word
  #   All fire via fail-closed (no **Forbidden:** match) regardless of negation vocabulary.
  # M-P21-Q through M-P21-T (FAIL-CLOSED COVERAGE — 4 NEW adversarial forms authored here,
  #   beyond the orchestrator's 8-form set, each targeting a different syntactic attack surface):
  #   Q: double negative — prohibition token present but semantically negated by idiom
  #   R: prohibition on different subject — FORBIDDEN applies to something else in the clause
  #   S: interposed parenthetical — phrase inserted between 'not' and prohibition token
  #   T: contraction verb — "haven't been prohibited" uses contraction form the old code missed
  local mp21d_violations mp21d_control mp21e_violations
  local mp21f_violations mp21g_violations mp21h_violations mp21_gitignored_violations
  mp21d_violations="$(printf '%s\n' 'Writes that are not yet forbidden under prior policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21d_violations" ]; then
    echo "MUTANT FAIL [M-P21-D (F-S2104-P21-002)]: probe 'Writes that are not yet forbidden under prior policy may target the story worktree CWD.' must fire Gate PW-B — fail-closed whitelist: 'may' (directive) + 'story worktree CWD' (prohibited-target) + no **Forbidden:** match → VIOLATION BY DEFAULT; got empty (whitelist wrongly matched or gate broken)"
    false
  fi
  mp21d_control="$(printf '%s\n' 'Writes may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21d_control" ]; then
    echo "CONTROL FAIL [CONTROL-D (F-S2104-P21-002)]: control 'Writes may target the story worktree CWD.' must still fire Gate PW-B after semantic fix — no prohibition token at all, negated or otherwise; got empty (regression: control should always be RED)"
    false
  fi

  mp21e_violations="$(printf '%s\n' 'Agents may deliver factory artifacts to the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21e_violations" ]; then
    echo "MUTANT FAIL [M-P21-E (F-S2104-P21-003 'may' regression guard)]: probe 'Agents may deliver factory artifacts to the story worktree CWD.' must fire Gate PW-B — 'may' is in PWBD_DIRECTIVE_CLASS (file-scope constant); 'story worktree CWD' is prohibited-target; no prohibition token → RED; deleting 'may' from PWBD_DIRECTIVE_CLASS would not be caught by M-P21-A/B/C (all fire via bare-imperative); got empty (gate SILENT)"
    false
  fi

  mp21f_violations="$(printf '%s\n' 'Writes that are never previously prohibited under old policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21f_violations" ]; then
    echo "MUTANT FAIL [M-P21-F (negation form: 'never previously prohibited')]: probe 'Writes that are never previously prohibited under old policy may target the story worktree CWD.' must fire Gate PW-B — fail-closed: prohibited-target + directive, no **Forbidden:** match → VIOLATION BY DEFAULT; got empty"
    false
  fi

  mp21g_violations="$(printf '%s\n' 'Writes that are no longer forbidden may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21g_violations" ]; then
    echo "MUTANT FAIL [M-P21-G (negation form: 'no longer forbidden')]: probe 'Writes that are no longer forbidden may target the story worktree CWD.' must fire Gate PW-B — fail-closed: prohibited-target + directive, no **Forbidden:** match → VIOLATION BY DEFAULT; got empty"
    false
  fi

  mp21h_violations="$(printf '%s\n' 'Writes that are not forbidden under the current policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21h_violations" ]; then
    echo "MUTANT FAIL [M-P21-H (negation form: bare 'not forbidden')]: probe 'Writes that are not forbidden under the current policy may target the story worktree CWD.' must fire Gate PW-B — fail-closed: prohibited-target + directive, no **Forbidden:** match → VIOLATION BY DEFAULT; got empty"
    false
  fi

  mp21_gitignored_violations="$(printf '%s\n' 'Agents may write artifacts to the gitignored-shadow path in the worktree.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21_gitignored_violations" ]; then
    echo "MUTANT FAIL [M-P21-GS (gitignored-shadow, SECONDARY 2 — F-S2104-P21-002)]: probe 'Agents may write artifacts to the gitignored-shadow path in the worktree.' must fire Gate PW-B — fail-closed: 'gitignored-shadow' (prohibited-target) + 'may' (directive), no **Forbidden:** match → VIOLATION BY DEFAULT; got empty"
    false
  fi

  # M-P21-I through M-P21-P (F-S2104-P21-002 fail-closed coverage — 8 orchestrator-supplied
  #   evading forms that were SILENT/GREEN under the old vocabulary-enumeration approach at
  #   b7d3ca58). All fire under fail-closed because none contain **Forbidden:**.
  local mp21i_violations mp21j_violations mp21k_violations mp21l_violations
  local mp21m_violations mp21n_violations mp21o_violations mp21p_violations

  mp21i_violations="$(printf '%s\n' 'Writes that are not explicitly forbidden under prior policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21i_violations" ]; then
    echo "MUTANT FAIL [M-P21-I (fail-closed: 'not explicitly forbidden')]: probe must fire — 'not explicitly forbidden' has 'explicitly' between 'not' and 'forbidden', evading the old per-word neutralizer; fail-closed fires because no **Forbidden:** match; 'may' + 'story worktree CWD'; got empty"
    false
  fi

  mp21j_violations="$(printf '%s\n' 'Writes that are not strictly forbidden under prior policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21j_violations" ]; then
    echo "MUTANT FAIL [M-P21-J (fail-closed: 'not strictly forbidden')]: probe must fire — 'not strictly forbidden' evaded old enumeration; fail-closed fires because no **Forbidden:** match; got empty"
    false
  fi

  mp21k_violations="$(printf '%s\n' 'Writes that are not currently forbidden under prior policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21k_violations" ]; then
    echo "MUTANT FAIL [M-P21-K (fail-closed: 'not currently forbidden')]: probe must fire — 'not currently forbidden' evaded old enumeration; fail-closed fires because no **Forbidden:** match; got empty"
    false
  fi

  mp21l_violations="$(printf '%s\n' 'Writes that are not entirely forbidden under prior policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21l_violations" ]; then
    echo "MUTANT FAIL [M-P21-L (fail-closed: 'not entirely forbidden')]: probe must fire — 'not entirely forbidden' evaded old enumeration; fail-closed fires because no **Forbidden:** match; got empty"
    false
  fi

  mp21m_violations="$(printf '%s\n' "Writes that aren't forbidden under prior policy may target the story worktree CWD." | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21m_violations" ]; then
    echo "MUTANT FAIL [M-P21-M (fail-closed: \"isn't/aren't forbidden\")]: probe must fire — contraction 'aren't' breaks \\bnot\\b word-boundary match; old enumeration SILENT; fail-closed fires because no **Forbidden:** match; 'may' + 'story worktree CWD'; got empty"
    false
  fi

  mp21n_violations="$(printf '%s\n' 'Writes that are hardly forbidden under prior policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21n_violations" ]; then
    echo "MUTANT FAIL [M-P21-N (fail-closed: 'hardly forbidden')]: probe must fire — 'hardly' is not a negation prefix in old enumeration; 'forbidden' caused SILENT; fail-closed fires because no **Forbidden:** match; got empty"
    false
  fi

  mp21o_violations="$(printf '%s\n' 'Writes that are not, at present, forbidden may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21o_violations" ]; then
    echo "MUTANT FAIL [M-P21-O (fail-closed: 'not, at present, forbidden')]: probe must fire — interposed comma phrase ', at present,' breaks \\bnot\\s+forbidden\\b adjacency; 'forbidden' caused SILENT; fail-closed fires because no **Forbidden:** match; got empty"
    false
  fi

  mp21p_violations="$(printf '%s\n' 'Writes that are far from forbidden under prior policy may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21p_violations" ]; then
    echo "MUTANT FAIL [M-P21-P (fail-closed: 'far from forbidden')]: probe must fire — 'far from forbidden' is idiomatic negation with no 'not' word; 'forbidden' caused SILENT in old code; fail-closed fires because no **Forbidden:** match; got empty"
    false
  fi

  # M-P21-Q through M-P21-T: 4 NEW adversarial forms authored here, attacking different
  #   syntactic surfaces. All evaded old vocabulary-enumeration; all fire under fail-closed.
  local mp21q_violations mp21r_violations mp21s_violations mp21t_violations

  # M-P21-Q: DOUBLE NEGATIVE — prohibition token present but semantically negated by idiom.
  #   "By no means FORBIDDEN" = "definitely not forbidden" (idiomatic double-negative with
  #   uppercase FORBIDDEN). Old code: FORBIDDEN present → EXCLUDED (SILENT/GREEN). Fail-closed:
  #   no **Forbidden:** → FIRES. Proves the token-presence check is not sufficient — FORBIDDEN
  #   can appear in a clause while the semantic content is authorization, not prohibition.
  mp21q_violations="$(printf '%s\n' 'By no means FORBIDDEN, story worktree CWD writes may proceed directly.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21q_violations" ]; then
    echo "MUTANT FAIL [M-P21-Q (NEW adversarial: double negative — 'By no means FORBIDDEN')]: probe 'By no means FORBIDDEN, story worktree CWD writes may proceed directly.' must fire — 'By no means FORBIDDEN' has FORBIDDEN uppercase but is semantically authorization; old code excluded because FORBIDDEN present; fail-closed fires because no **Forbidden:** match; 'may' + 'story worktree CWD'; got empty"
    false
  fi

  # M-P21-R: PROHIBITION TOKEN ON DIFFERENT SUBJECT — FORBIDDEN applies to something else in
  #   the clause. "The FORBIDDEN canonical path being unavailable" — FORBIDDEN describes the
  #   canonical path, not the worktree writes. Old code: FORBIDDEN present → EXCLUDED (SILENT).
  #   Fail-closed: no **Forbidden:** → FIRES.
  mp21r_violations="$(printf '%s\n' 'The FORBIDDEN canonical path being unavailable, agents may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21r_violations" ]; then
    echo "MUTANT FAIL [M-P21-R (NEW adversarial: prohibition on different subject)]: probe 'The FORBIDDEN canonical path being unavailable, agents may target the story worktree CWD.' must fire — FORBIDDEN describes the canonical path (not worktree writes), but old token-check excluded the clause; fail-closed fires because no **Forbidden:** match; 'may' + 'story worktree CWD'; got empty"
    false
  fi

  # M-P21-S: INTERPOSED PARENTHETICAL — phrase inserted between 'not' and prohibition token.
  #   "not (given the current sprint policy) specifically forbidden" — old code's \\bnot\\s+...
  #   pattern requires whitespace-only after 'not' before the optional modifier. The parenthetical
  #   '(given ...)' breaks the adjacency. Old code: 'forbidden' in clause → EXCLUDED (SILENT).
  #   Fail-closed: no **Forbidden:** → FIRES.
  mp21s_violations="$(printf '%s\n' 'Writes that are (given the current sprint policy) not specifically forbidden may target the story worktree CWD.' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21s_violations" ]; then
    echo "MUTANT FAIL [M-P21-S (NEW adversarial: interposed parenthetical)]: probe 'Writes that are (given the current sprint policy) not specifically forbidden may target the story worktree CWD.' must fire — interposed '(given...)' and 'specifically' break old neutralizer adjacency; 'forbidden' caused SILENT; fail-closed fires because no **Forbidden:** match; 'may' + 'story worktree CWD'; got empty"
    false
  fi

  # M-P21-T: CONTRACTION VERB — "haven't been prohibited" uses a contraction whose \\bnot\\b
  #   word-boundary does not match the 'n't' suffix. Old code: 'prohibited' present → EXCLUDED
  #   (SILENT). Fail-closed: no **Forbidden:** → FIRES.
  mp21t_violations="$(printf '%s\n' "Since these writes haven't been prohibited, agents may target the story worktree CWD." | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$mp21t_violations" ]; then
    echo "MUTANT FAIL [M-P21-T (NEW adversarial: contraction verb)]: probe 'Since these writes haven\\'t been prohibited, agents may target the story worktree CWD.' must fire — 'haven\\'t' contraction breaks \\bnot\\b; 'prohibited' standalone caused old code to EXCLUDE (SILENT); fail-closed fires because no **Forbidden:** match; 'may' + 'story worktree CWD'; got empty"
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
  forbidden_prohibited_sentences="$(printf '%s\n' "$joined_block_nosplit" | perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
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
  retirement_language="$(printf '%s\n' "$write_discipline_prose_nosplit" | perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    grep -E 'formerly|retired|rescinded|superseded|relaxed|lifted|withdrawn|rescind|no[[:space:]]+longer|not[[:space:]]+longer|waived|exempt|obsolete|deprecated|does[[:space:]]+not[[:space:]]+apply|overridden|historical[[:space:]]+only|supplanted|supersede|does[[:space:]]+not[[:space:]]+bind|does[[:space:]]+not[[:space:]]+govern|no[[:space:]]+longer[[:space:]]+binds|descriptive[[:space:]]+only|advisory[[:space:]]+only|pre-#?[0-9]+' || true)"
  if [ -n "$retirement_language" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide nullification language (Gate 2b(a), F-S2104-P16-001(c)/F-S2104-P17-003/F-S2104-P18-005(c))]: the Write Discipline section contains constraint-nullification language — M-P17-D 'rescinded and superseded', M-P17-F 'no longer' (split across line break), M-P16-A 'formerly...retired'; widened (F-S2104-P18-005(c)) to also catch: supplanted, supersede, does not bind, does not govern, no longer binds, descriptive only, advisory only, pre-#NNN (BC-6.26.001 PC1; AC-001(a))"
    printf '%s\n' "$retirement_language"
    false
  fi
  # Gate 2b(c): FORBIDDEN sentence must NOT contain an adversative connective (F-S2104-P17-003(c)).
  # An adversative connective attaching to any prohibition-reference sentence expresses
  # nullification regardless of which nullification verb is used, closing the synonym-list bypass.
  # Domain re-scoped from joined_block_nosplit → write_discipline_prose_nosplit (section-wide)
  # per F-S2104-P18-005(a): Gate 2b(c) was paragraph-scoped while (a) is section-wide, so
  # nullification via adversative in a sibling paragraph escaped (c) entirely. M-P18-G uses
  # 'however' in a sibling paragraph whose sentence matches the widened trigger ('permits') —
  # domain parity with (a) is required for structural coverage.
  # Trigger widened (F-S2104-P18-005(b)): FORBIDDEN-sentences-only → any sentence referencing
  # the prohibition (FORBIDDEN|forbidden|prohibition|prohibited|the rule|this rule|the
  # constraint|above); M-P18-E contains 'prohibition' but no FORBIDDEN, old trigger missed it.
  # Adversative class widened (F-S2104-P18-005(b)): adds whereas|nevertheless|that said|
  # in practice|notwithstanding.
  # Alternation-direction: members (b) open class — backed by the open-trigger write-directive
  # gate (F-S2104-P18-001/F-S2104-P18-005(d)) which covers the directives axis; together they
  # form a non-defeatable closed pair.
  # M-P17-F: "...are FORBIDDEN under the initial reading, but that is no longer" → RED.
  # M-P18-G: "however current practice permits ledger writes" in sibling para → RED.
  # Correct text uses em-dash (—) not adversative → GREEN.
  local forbidden_sentences_with_adversative
  forbidden_sentences_with_adversative="$(printf '%s\n' "$write_discipline_prose_nosplit" | perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    grep -E 'FORBIDDEN|forbidden|prohibition|prohibited|the[[:space:]]+rule|this[[:space:]]+rule|the[[:space:]]+constraint|above' | \
    grep -E 'but[[:space:]]|however|except[[:space:]]+that|though[[:space:]]|whereas|nevertheless|that[[:space:]]+said|in[[:space:]]+practice|notwithstanding' || true)"
  if [ -n "$forbidden_sentences_with_adversative" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide adversative-connective on prohibition-reference sentence (Gate 2b(c), F-S2104-P17-003(c)/F-S2104-P18-005(a)/(b))]: a sentence referencing the prohibition (FORBIDDEN|forbidden|prohibition|prohibited|the rule|this rule|the constraint|above) is qualified by an adversative connective (but/however/except that/though/whereas/nevertheless/that said/in practice/notwithstanding) — domain re-scoped section-wide so sibling-paragraph adversatives are caught; trigger widened so 'prohibition' and 'the rule' sentences also captured; M-P17-F exploits 'but ', M-P18-G exploits 'however...permits'; correct text uses em-dash (—) not adversative (BC-6.26.001 PC1; AC-001(a))"
    printf '%s\n' "$forbidden_sentences_with_adversative"
    false
  fi

  # Gate scope-restriction (NEGATIVE, section-scoped; F-S2104-P19-007):
  # A sentence referencing the prohibition (trigger: FORBIDDEN|forbidden|prohibition|prohibited|
  # the rule|this rule|the constraint|above) MUST NOT also match a scope-restriction class.
  # The 25-member nullification class and 9-member adversative class still defend their axes;
  # this gate adds a structural gate that catches predicate-form nullifications not expressible
  # as synonyms or adversatives — e.g., 'is not applicable', 'does not cover'. Gate 2b(a)'s
  # direction statement claimed the write-directive gate backstops this axis; M-P19-G refuted
  # that claim ('is not applicable' has no directive token and no adversative connective,
  # escaping both 2b(a) and 2b(c), and 'ledger writes' was not yet a referent trigger).
  # scope-restriction class: not applicable|does not|is not|except|outside|limited to|
  #   only (covers|applies)|other than|save for|apart from|excluding
  # NOTE: pristine sentence "All writes to any .factory/** path are covered by this rule —
  #   not only DELIVERY ledgers." has 'this rule' AND 'not only'; after em-dash clause split
  #   the trigger ('this rule') and the scope-phrase ('not only') are in separate clauses and
  #   the restriction class requires 'not applicable|does not|is not|...' — 'not only' does NOT
  #   match any member → no false positive. Verified by literal shell on pristine section.
  # Alternation-direction (F-S2104-P19-007(a)): this gate makes Gate 2b(a)'s list non-load-bearing
  #   as a primary defense; the list remains for defense-in-depth with new members:
  #   not applicable|inapplicable|does not cover|does not extend|out of scope (per P19-007(c)).
  # M-P19-G fires on this gate (prohibition + scope-restriction without adversative).
  # MUTANT M-P19-G: "The prohibition stated above is not applicable to ledger writes made from
  #   the delivery sandbox." → 'above' (trigger) + 'not applicable' (class) → RED ✓.
  local scope_restriction_violations
  scope_restriction_violations="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    grep -E 'FORBIDDEN|forbidden|prohibition|prohibited|the[[:space:]]+rule|this[[:space:]]+rule|the[[:space:]]+constraint|above' | \
    grep -E 'not[[:space:]]+applicable|inapplicable|does[[:space:]]+not|is[[:space:]]+not|except[[:space:]]|outside[[:space:]]|limited[[:space:]]+to|only[[:space:]]+(covers|applies)|other[[:space:]]+than|save[[:space:]]+for|apart[[:space:]]+from|excluding|does[[:space:]]+not[[:space:]]+cover|does[[:space:]]+not[[:space:]]+extend|out[[:space:]]+of[[:space:]]+scope' || true)"
  if [ -n "$scope_restriction_violations" ]; then
    echo "DOC-PARITY FAIL [write-discipline scope-restriction gate: prohibition-reference sentence with scope-restriction class (F-S2104-P19-007)]: a sentence referencing the prohibition (FORBIDDEN|forbidden|prohibition|prohibited|the rule|this rule|the constraint|above) contains a scope-restriction phrase (not applicable|inapplicable|does not|is not|except|outside|limited to|only (covers|applies)|other than|save for|apart from|excluding|does not cover|does not extend|out of scope) — M-P19-G 'The prohibition stated above is not applicable to ledger writes made from the delivery sandbox' triggers this gate; scope-restriction phrases nullify the prohibition for a sub-case without any adversative connective or listed nullification synonym, bypassing Gates 2b(a) and 2b(c) (BC-6.26.001 PC1; AC-001(a); F-S2104-P19-007)"
    printf '%s\n' "$scope_restriction_violations"
    false
  fi

  # Gate 4 (NEGATIVE, section-scoped; F-S2104-P14-001 / F-S2104-P15-001 / F-S2104-P17-001(a) /
  # F-S2104-P18-003): Extended to whole #### Write Discipline section (write_discipline_prose_nosplit,
  # fence content now included). No sentence may contain both 'absolute' and 'FORBIDDEN'. In the
  # correct text: S1 "...MUST use canonical absolute paths..." — 'absolute' present, 'FORBIDDEN'
  # absent → PASSES. S2 "CWD-relative paths...are FORBIDDEN..." — 'FORBIDDEN' present, 'absolute'
  # absent → PASSES. M-P15-A S3: "Canonical absolute artifact-write paths...are FORBIDDEN" → RED.
  # Per-sentence evaluation using boundary-rule splitter (F-S2104-P18-003): splits on \. followed
  # by [A-Z*`\[] only — prevents 'No. 523' false boundary from M-P18-B that separated 'absolute'
  # and 'FORBIDDEN' onto different sentences, silencing Gate 4 at 9/9 with the old splitter.
  # CONTROL-B (same sentence without 'No. ') is the isolating mutant for Gate 4.
  local forbidden_absolute_sentences
  forbidden_absolute_sentences="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | grep -E 'absolute' | grep -E '(FORBIDDEN|forbidden)' || true)"
  if [ -n "$forbidden_absolute_sentences" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide FORBIDDEN-polarity (sentence-scoped; F-S2104-P17-001(a))]: a sentence in the Write Discipline section contains both 'absolute' and 'FORBIDDEN' — in the correct text absolute paths are MANDATED (MUST), not the FORBIDDEN subject; M-P15-A S3 'Canonical absolute artifact-write paths...are FORBIDDEN' triggers this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14-001 / F-S2104-P15-001)"
    false
  fi

  # Gate 5 (NEGATIVE, section-scoped; F-S2104-P14R-001(b) / F-S2104-P15-001 / F-S2104-P17-001(a)):
  # Extended to whole #### Write Discipline section (write_discipline_prose_nosplit, fence content
  # now included). No sentence may contain both 'MUST' and a prohibited-subject form.
  # Correct text: S1 "...MUST use canonical absolute paths..." — MUST present, no prohibited form;
  #   S2 "CWD-relative paths...are FORBIDDEN..." — prohibited form present, no MUST → PASSES.
  # M-P15-A S1: "...MUST use CWD-relative paths..." — MUST+CWD-relative → RED.
  # M-P14-A: "MUST use CWD-relative paths" → RED. M-P14R-A: "MUST use relative paths" → RED.
  # M-P16-C2 + abbreviation-protected splitter → cf_ABBREV_ CWD-relative → Gate 5 fires → RED.
  # POLICY-13 syntactic-form class alternation; write_discipline_prose_nosplit for splits.
  # NOTE (F-S2104-P18-006 correction): M-P17-A fires Gate PW-B (no prohibition token on 'story
  # worktree CWD') but does NOT fire Gate 5 independently — M-P17-A's sentence contains 'MUST'
  # and 'story worktree CWD', but 'story worktree CWD' is NOT in Gate 5's alternation class
  # (CWD-relative|worktree-relative|relative path). Isolating mutant for Gate 5: "Writers MUST
  # use relative paths for ledger writes." in a sibling paragraph (mandate unchanged) → Gate 5
  # fires alone → RED. This is independent of Gate PW-B (which requires a prohibited-target form
  # per its own class) and of Gate 1(a/b/c/d) (which use joined_block_nosplit, not section-wide).
  local must_relative_sentences
  must_relative_sentences="$(printf '%s\n' "$write_discipline_prose_nosplit" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | grep -E 'MUST' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+path' || true)"
  if [ -n "$must_relative_sentences" ]; then
    echo "DOC-PARITY FAIL [write-discipline section-wide MUST-relative-polarity (sentence-scoped; F-S2104-P17-001(a))]: a sentence in the Write Discipline section contains both 'MUST' and a prohibited-subject form (CWD-relative, worktree-relative, or relative path) — in the correct text MUST mandates canonical absolute paths; M-P15-A/M-P14-A/M-P14R-A/M-P16-C2 trigger it on the prohibition paragraph; isolating mutant 'Writers MUST use relative paths for ledger writes.' in sibling paragraph → Gate 5 alone fires; POLICY-13 alternation (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-001 / F-S2104-P15-001)"
    false
  fi

  # Gate write-directive (POSITIVE, open-trigger; F-S2104-P18-001 / F-S2104-P19-001 / F-S2104-P19-002 / F-S2104-P19-003 / F-S2104-P20-002):
  # Domain: spec_path_prose_nosplit (whole ### Spec-Path Discipline; F-S2104-P19-002). PW-B,
  # Gate 2b, Gate 4, Gate 5 remain bounded to write_discipline_prose_nosplit — extending those
  # gates naively false-positives on two read-discipline sentences above #### Write Discipline
  # (adversary verified). The write-directive gate is extended because M-P19-H placed verbatim
  # M-P17-A + M-P17-C S2 text above the #### Write Discipline heading at 9/9 (no gate saw it).
  #
  # Clause-scoped (F-S2104-P19-001(a)): after sentence splitting, split further on [;—] and
  # on ,\s+(and|or|but)\s+. Each clause evaluated independently. Sentence-scoped escape (pass-18)
  # allowed M-P19-A (verbatim M-P17-A + trailing '; forbidden.' clause) and M-P19-B (canonical-
  # absolute escape phrase in same sentence as harmful MUST anchor clause) at 9/9. CONTROL-A
  # (forbidden→discouraged) is RED via write-directive; CONTROL-B (MUST anchor→MUST use canonical
  # absolute) is GREEN — proving the escape fires on the clause boundary, not the sentence.
  # Gate PW-B now clause-scoped per F-S2104-P20-001 (S2 em-dash changed to period in doc to
  # remove the 'worktree's shadow' false-positive on the em-dash continuation).
  #
  # Referent predicate (F-S2104-P19-003(a) extended F-S2104-P20-002): domain-object referent
  # (\.factory/|ledger) extended to also cover artifact writes? phrase. The bare word 'artifact'
  # was removed in pass-19 to silence the pristine false-positive "MUST be reported as a defect
  # signal (not dismissed as a pathing artifact)" which has MUST + artifact but no write-directive
  # semantic. The phrase 'artifact writes?' is safe: the only pristine match is the S1 mandate
  # "All .factory/** artifact writes...MUST use canonical absolute paths" which escapes via
  # MUST use canonical absolute. Verified empty-on-pristine with this extended referent.
  # Additional escape: MUST be determined via — handles the "canonical .factory/ root
  # MUST be determined via one of two methods" clause in ### Spec-Path Discipline, which is a
  # read/resolution operation, not a write directive; verified empty-on-pristine with this escape.
  #
  # Bare-imperative addition (F-S2104-P19-003(b)): trigger additionally on any clause whose
  # first token after optional **bold:** label is a bare verb from class
  # (Anchor|Write|Save|Store|Place|Record|Emit|Persist|Resolve|Use). Verified empty-on-pristine.
  #
  # Alternation-direction (corrected from pass-18, F-S2104-P19-001(d)): both trigger conjuncts
  # (directive/bare-imperative class and referent class) are open-class. The escape clauses
  # (prohibition token, canonical-absolute match) are the load-bearing constants per
  # ESCAPE-SCOPE-PARITY: the escape unit must match the trigger unit (clause, not sentence).
  # Pass-18 "No new member can be added to evade this gate" claim is REFUTED by M-P19-A (no new
  # member needed — one semicolon and a prohibition word in a separate clause suffice).
  # Pass-19 referent-predicate "cannot be paraphrased away" claim is REFUTED by M-P20-A (no new
  # paraphrase needed — 'artifact write' instead of '.factory/' or 'ledger' escapes the referent;
  # fixed by F-S2104-P20-002 extending the referent to cover 'artifact writes?').
  #
  # Adversary verified empty-on-pristine (### Spec-Path Discipline, clause-scoped, extended referent
  # including artifact writes?, with MUST be determined via escape; F-S2104-P20-002 verification).
  #
  # MUTANT M-P19-A: verbatim M-P17-A + '; duplicating the ledger onto the main checkout is forbidden.'
  #   Clause split separates the MUST anchor clause from the forbidden clause → MUST anchor + .factory/
  #   in first clause, no prohibition or canonical-absolute → RED ✓.
  # MUTANT M-P19-B (canonical-absolute co-clause escape): "Writers MUST use canonical absolute paths
  #   when reading specs, and MUST anchor every .factory/** artifact write to the worktree's .factory/ subtree."
  #   After ', and ' split: "MUST anchor ... subtree" has .factory/ + MUST, no escape → RED ✓.
  # MUTANT M-P19-C (action-word evasion closed): "Ledger artifacts SHOULD be saved to the story
  #   worktree's own .factory/ subtree" → SHOULD + ledger, no prohibition → RED ✓.
  #   (Old predicate 'anchor|write|writes' missed 'saved'; referent predicate catches it via 'ledger').
  # MUTANT M-P19-D (missed-boundary — primary fix via clause-scoping): "git-resolved ledger paths
  #   SHOULD be anchored to the story worktree CWD." placed after prohibition block without capital.
  #   After clause-scoping: the merged clause 'SHOULD ... ledger' has no prohibition → RED ✓.
  #   CONTROL-D (capital G): split creates separate clause → PW-B sees it → RED ✓ (both paths RED).
  # MUTANT M-P19-H (above-heading mandate, closed by domain extension): verbatim M-P17-A text
  #   placed above #### Write Discipline heading → caught by spec_path_prose_nosplit domain → RED ✓.
  # CONTROL-A: M-P19-A with 'forbidden'→'discouraged' → first clause: MUST anchor + .factory/,
  #   no escape → RED ✓ (primary RED through this gate).
  # CONTROL-B (escape load-bearing GREEN): M-P18-A with 'MUST anchor' → 'MUST use canonical absolute'
  #   → escape fires → GREEN ✓ (proves canonical-absolute escape is not vacuous; closes F-S2104-P19-012).
  # CONTROL-C (escape clause-scope): M-P18-A text + '; writers MUST use canonical absolute paths for
  #   spec reads.' → after semicolon split: first clause has .factory/ + MUST + no escape → RED ✓
  #   (proves escape is clause-scoped, not sentence-scoped; closes F-S2104-P19-012(b)).
  # M-P20-A (F-S2104-P20-002 closure): "Writers MUST anchor every artifact write to the story
  #   worktree CWD; duplicating the ledger onto the main checkout is forbidden."
  #   Clause 1 (before ;): MUST + 'artifact write' (now in referent), no prohibition → violation → RED ✓.
  #   Clause 2: 'ledger' + 'forbidden' → excluded by prohibition escape → not a violation.
  # CONTROL-2 (F-S2104-P20-002 referent coverage): M-P20-A with 'artifact write' → '.factory/ write'.
  #   Clause 1: MUST + .factory/ → no prohibition → violation → RED ✓ via write-directive.
  #   (proves .factory/ referent is still load-bearing after the artifact writes? extension).
  # B02 F-S2104-P25-B02 (TD-VSDD-060 sibling sweep): vocabulary neutralizer replaced with fail-closed
  # whitelist (grep -vE '\*\*Forbidden:\*\*'). Old perl -ne enumerated specific negating prefixes;
  # novel negation forms (e.g., "isn't", "hardly") evaded the vocabulary scan. Fail-closed: only
  # **Forbidden:** labeled constructions are whitelisted — all other directive+referent combinations fire.
  # B01 F-S2104-P26-B01: 'MUST use canonical absolute' escape narrowed to require canonical anchor
  # (main-checkout|$CANONICAL_FACTORY_ROOT). B01 mutant 'MUST use canonical absolute paths anchored
  # to the story worktree's own .factory/ subtree' now fires — it lacks a canonical anchor.
  # M02 F-S2104-P26-M02: meta-aware escape added for 'MUST NOT use' clauses — prohibition sentences
  # ('artifact writes MUST NOT use CWD-relative paths') are correctly prohibiting bad behavior and
  # must not trigger the write-directive gate as false positives.
  local write_directive_violations
  write_directive_violations="$(printf '%s\n' "$spec_path_prose_nosplit" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -E '\.factory/|ledger|artifact[[:space:]]+writes?' | \
    grep -vE '\*\*Forbidden:\*\*' | \
    grep -Ev 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute.*(main-checkout|\$CANONICAL_FACTORY_ROOT)' | \
    grep -Ev 'MUST[[:space:]]+(NOT|not)[[:space:]]+(use|write|store|save|place|record|emit|persist|resolve)[[:space:]]' | \
    grep -Ev 'MUST[[:space:]]+be[[:space:]]+determined[[:space:]]+via' || true)"
  if [ -n "$write_directive_violations" ]; then
    echo "DOC-PARITY FAIL [write-directive gate: write-directive clause without prohibition or canonical-absolute escape (F-S2104-P18-001/F-S2104-P19-001/P19-002/P19-003/F-S2104-P20-002/F-S2104-P21-001)]: a clause in ### Spec-Path Discipline contains a write-directive or bare-imperative referencing .factory/, ledger, or artifact writes without either a prohibition token or 'MUST use canonical absolute' — clause-scoped (F-S2104-P19-001); domain extended to ### Spec-Path Discipline (F-S2104-P19-002); referent predicate extended to include artifact writes? (F-S2104-P20-002); unified directive class per F-S2104-P21-001; M-P19-A ('; forbidden.' escape), M-P19-B (canonical-absolute co-clause), M-P19-C ('saved' verb), M-P19-D (merged lowercase), M-P19-H (above-heading mandate), M-P20-A (artifact-write evasion) all RED (BC-6.26.001 PC1; AC-001(a); F-S2104-P18-001)"
    printf '%s\n' "$write_directive_violations"
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

  # Gate canonical-target (NEGATIVE, F-S2104-P17-004(b) widened F-S2104-P19-006):
  # No **Correct:** bullet in §Spec-Path Discipline may name a non-canonical .factory/ path.
  # Domain widened (F-S2104-P19-006): from 'file_path=' to any **Correct:** bullet containing
  # '\.factory/' regardless of the keyword used. M-P19-E ('**Correct:** `Write` the DELIVERY
  # ledger to `.factory/stories/S-NNN-DELIVERY.md` resolved from the worktree root') had no
  # 'file_path=' token and escaped the old gate at 9/9.
  # Canonical predicate widened: every .factory/ occurrence on a **Correct:** bullet MUST be
  # immediately preceded by $CANONICAL_FACTORY_ROOT/ or by / (path-start quote + canonical root
  # or absolute path). Predicate: ["'`]($CANONICAL_FACTORY_ROOT/|/[^"'`]*)\.factory/
  # M-P17-G: file_path="./.factory/…" — after `"` the path starts with './' not '$' or '/'
  #   → predicate does not match → fires → RED ✓.
  # M-P15-B (traversal): file_path="../../.factory/…" — starts with '..' → fires → RED ✓.
  # M-P16-D (CWD-relative): file_path=".factory/…" — starts with '.' → fires → RED ✓.
  # M-P19-E (backtick-quoted, no keyword): `.factory/…` — after backtick starts with '.' → RED ✓.
  # path= variant: path=".factory/…" — after '"' starts with '.' → RED ✓.
  # Control (GREEN): file_path="$CANONICAL_FACTORY_ROOT/.factory/…" → after '"' starts with '$' →
  #   pattern matches $CANONICAL_FACTORY_ROOT/.factory/ → escaped by grep -Ev → PASSES ✓.
  local noncanonical_correct_bullets
  noncanonical_correct_bullets="$(printf '%s\n' "$spec_path_section" | \
    grep -E '\*\*Correct:\*\*' | grep -E '\.factory/' | \
    grep -Ev '["'"'"'`](\$CANONICAL_FACTORY_ROOT/|/[^"'"'"'`]*)\.factory/' || true)"
  if [ -n "$noncanonical_correct_bullets" ]; then
    echo "DOC-PARITY FAIL [write-discipline Gate canonical-target: **Correct:** bullet with non-canonical .factory/ path (F-S2104-P17-004/F-S2104-P19-006)]: a **Correct:** bullet in §Spec-Path Discipline names a .factory/ path that fails the canonical-target predicate ([\"'\`](\$CANONICAL_FACTORY_ROOT/|/[^\"'\`]*)\.factory/) — every **Correct:** example must show a .factory/ path immediately preceded by \$CANONICAL_FACTORY_ROOT/ or an absolute-path root; M-P17-G (./.factory/ relative), M-P15-B (../../.factory/ traversal), M-P16-D (.factory/ bare), M-P19-E (backtick-quoted, no file_path= keyword) all RED (BC-6.26.001 PC1; AC-001(a))"
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
  # Non-.md stray file must also appear in output (F-S2104-P2-010: 'any non-directory type'
  # property of '! -type d' is load-bearing; a '-name *.md' doc-mutant would miss
  # engine-config.yaml, causing this assertion to fail — the mutant is caught here)
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
# Pipeline probe (F-S2104-P24-003): domain-construction exercises real production code path
#
# Three consecutive BLOCKERs (pass-22, pass-23, pass-24) all lived in the domain-construction
# pipeline (extract → strip → tr → split). None were caught because the M-P21 probe family
# re-implements gate predicates over synthetic string literals and never exercises the pipeline
# itself. The marker strip appeared exactly ONCE in the whole bats file at its own definition
# (L808 pre-P24). No probe applied it. Reverting the strip to the old fail-open form left
# the suite 9/9 GREEN — the pipeline was entirely unguarded.
#
# This probe closes that gap: it injects mutations into REAL copies of _shared-context.md and
# runs the ACTUAL _extract_*_section + _build_section_prose + abbreviation-protection pipeline.
# Reverting the strip to the old non-recursive form ('sed s/^[[:space:]]*>[[:space:]]*//')
# MUST make this probe RED. Pristine _shared-context.md MUST stay GREEN.
#
# Legs covered:
#   A — F-S2104-P24-001: '>> ...' double-blockquote inside #### Write Discipline → Gate PW-B
#   B — F-S2104-P24-004: '- ...' / '+ ...' / '* ...' list-prefix inside #### Write Discipline → Gate PW-B
#   C — F-S2104-P24-002: '> Anchor every .factory/ artifact write...' above #### Write Discipline
#         (in ### Spec-Path Discipline body) → write-directive gate (verifies spec_path_prose strip)
#   D — pristine fixture: both Gate PW-B and write-directive gate SILENT (GREEN)
# ===========================================================================

@test "BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)" {
  local scratch saved_shared_context
  scratch="$(mktemp -d)"
  saved_shared_context="$SHARED_CONTEXT_MD"

  # --- Leg A: '>>' double-blockquote inside #### Write Discipline (F-S2104-P24-001) ---
  # Injected immediately after '#### Write Discipline' heading (inside write_discipline_section
  # domain). The _build_section_prose recursive strip 's/^([[:space:]]*>[[:space:]]*)+//'
  # collapses '>>' → '' leaving 'Anchor...' at line-start. ^Anchor matches PWBD_DIRECTIVE_CLASS;
  # 'story worktree CWD' matches the prohibited-target class; no **Forbidden:** → Gate PW-B fires.
  # If strip is reverted to non-recursive form, '>>' → '>' (one '>' survives) and ^Anchor fails.
  local fixture_a="$scratch/shared-context-a.md"
  awk '
    /^#### Write Discipline/ { print; print ">> Anchor every write to the story worktree CWD."; next }
    { print }
  ' "$saved_shared_context" > "$fixture_a"

  SHARED_CONTEXT_MD="$fixture_a"
  local wd_section_a wd_prose_a wd_nosplit_a
  wd_section_a="$(_extract_write_discipline_section)"
  wd_prose_a="$(_build_section_prose "$wd_section_a")"
  wd_nosplit_a="$(printf '%s\n' "$wd_prose_a" | \
    sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g')"
  SHARED_CONTEXT_MD="$saved_shared_context"

  local leg_a_result
  leg_a_result="$(printf '%s\n' "$wd_nosplit_a" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  if [ -z "$leg_a_result" ]; then
    echo "PIPELINE PROBE FAIL [Leg A — F-S2104-P24-001]: '>> Anchor every write to the story worktree CWD.' injected into #### Write Discipline MUST fire Gate PW-B via the real pipeline."
    echo "  Recursive strip collapses '>>' → '' leaving 'Anchor...' at line-start (^Anchor in PWBD_DIRECTIVE_CLASS)."
    echo "  Gate SILENT means _build_section_prose recursive strip is not working (reverted or broken)."
    echo "  write_discipline_prose first 200 chars: ${wd_prose_a:0:200}"
    rm -rf "$scratch"
    false
  fi

  # --- Leg B: list-prefix forms inside #### Write Discipline (F-S2104-P24-004) ---
  # Tests '- ', '+ ', '* ' unordered-list prefixes and '1. ' numbered-list prefix.
  # _build_section_prose strips these before the domain is evaluated; without the strip,
  # '- Anchor...' leaves the bare-imperative after the list marker and ^Anchor fails.
  local list_prefix list_fixture leg_b_wd_section leg_b_wd_prose leg_b_nosplit leg_b_result
  for list_prefix in '- ' '+ ' '* ' '1. '; do
    list_fixture="$scratch/shared-context-b-${list_prefix:0:1}.md"
    awk -v pfx="$list_prefix" '
      /^#### Write Discipline/ { print; print pfx "Anchor every write to the story worktree CWD."; next }
      { print }
    ' "$saved_shared_context" > "$list_fixture"

    SHARED_CONTEXT_MD="$list_fixture"
    leg_b_wd_section="$(_extract_write_discipline_section)"
    leg_b_wd_prose="$(_build_section_prose "$leg_b_wd_section")"
    leg_b_nosplit="$(printf '%s\n' "$leg_b_wd_prose" | \
      sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g')"
    SHARED_CONTEXT_MD="$saved_shared_context"

    leg_b_result="$(printf '%s\n' "$leg_b_nosplit" | \
      perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
      perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
      grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
      grep -E "$PWBD_DIRECTIVE_CLASS" | \
      grep -vE '\*\*Forbidden:\*\*' || true)"
    if [ -z "$leg_b_result" ]; then
      echo "PIPELINE PROBE FAIL [Leg B — F-S2104-P24-004]: '${list_prefix}Anchor every write to the story worktree CWD.' injected into #### Write Discipline MUST fire Gate PW-B via the real pipeline."
      echo "  List-marker strip removes '${list_prefix}' leaving 'Anchor...' at line-start (^Anchor in PWBD_DIRECTIVE_CLASS)."
      echo "  Gate SILENT means _build_section_prose list-marker strip is not working (reverted or broken)."
      echo "  write_discipline_prose first 200 chars: ${leg_b_wd_prose:0:200}"
      rm -rf "$scratch"
      false
    fi
  done

  # --- Leg B mixed-marker probes (B03 F-S2104-P25-B03) ---
  # Previous 3-pass strip left residue on out-of-order marker combos: '- > ' left '> Anchor...'
  # after pass 2 stripped '- ' (pass 1 already ran, could not re-strip the remaining '>').
  # Unified single-pass sed handles these in one alternation sweep.
  for mixed_pair in '- > ' '+ > ' '* > '; do
    local mix_fixture leg_b_mix_wd_section leg_b_mix_wd_prose leg_b_mix_nosplit leg_b_mix_result
    mix_fixture="$scratch/shared-context-mix-${mixed_pair:0:1}.md"
    awk -v pfx="$mixed_pair" '
      /^#### Write Discipline/ { print; print pfx "Anchor every write to the story worktree CWD."; next }
      { print }
    ' "$saved_shared_context" > "$mix_fixture"

    SHARED_CONTEXT_MD="$mix_fixture"
    leg_b_mix_wd_section="$(_extract_write_discipline_section)"
    leg_b_mix_wd_prose="$(_build_section_prose "$leg_b_mix_wd_section")"
    leg_b_mix_nosplit="$(printf '%s\n' "$leg_b_mix_wd_prose" | \
      sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g')"
    SHARED_CONTEXT_MD="$saved_shared_context"

    leg_b_mix_result="$(printf '%s\n' "$leg_b_mix_nosplit" | \
      perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
      perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
      grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
      grep -E "$PWBD_DIRECTIVE_CLASS" | \
      grep -vE '\*\*Forbidden:\*\*' || true)"
    if [ -z "$leg_b_mix_result" ]; then
      echo "PIPELINE PROBE FAIL [Leg B mixed-marker — B03 F-S2104-P25-B03]: '${mixed_pair}Anchor every write to the story worktree CWD.' injected into #### Write Discipline MUST fire Gate PW-B."
      echo "  Mixed-marker '${mixed_pair}' not fully stripped — _build_section_prose 3-pass residue (B03 regression)."
      echo "  write_discipline_prose first 200 chars: ${leg_b_mix_wd_prose:0:200}"
      rm -rf "$scratch"
      false
    fi
  done


  # --- Leg C: '>' above #### Write Discipline in ### Spec-Path Discipline body (F-S2104-P24-002) ---
  # Injected immediately after '### Spec-Path Discipline' heading, which places the mutant
  # INSIDE spec_path_section but OUTSIDE write_discipline_section (bounded by #### Write Discipline).
  # Without marker normalisation being applied to spec_path_prose (the F-S2104-P24-002 gap), the
  # '>' survived and the ^Anchor bare-imperative did not match in the write-directive gate.
  # Mutant referent is '.factory/' + 'artifact write' (both in write-directive gate referent class).
  # Load-bearing: sp_prose_c is built via _build_spec_path_section_prose — the SAME named function
  # T-001's spec_path_prose call. A revert of _build_spec_path_section_prose to bare 'tr' makes sp_prose_c
  # retain the '>' prefix, ^Anchor fails at line-start → leg_c_result is empty → probe RED.
  # This mirrors Leg A's structure: Leg A depends on _build_section_prose's implementation;
  # Leg C depends on _build_spec_path_section_prose's implementation (F-S2104-P24-003).
  local fixture_c="$scratch/shared-context-c.md"
  awk '
    /^### Spec-Path Discipline/ { print; print "> Anchor every .factory/ artifact write to the story worktree CWD."; next }
    { print }
  ' "$saved_shared_context" > "$fixture_c"

  SHARED_CONTEXT_MD="$fixture_c"
  local sp_section_c sp_prose_c sp_nosplit_c wd_section_c
  sp_section_c="$(_extract_spec_path_discipline_section)"
  sp_prose_c="$(_build_spec_path_section_prose "$sp_section_c")"
  sp_nosplit_c="$(printf '%s\n' "$sp_prose_c" | \
    sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g')"
  # Structural sanity: mutant must NOT appear in write_discipline_section (above-heading isolation)
  wd_section_c="$(_extract_write_discipline_section)"
  SHARED_CONTEXT_MD="$saved_shared_context"

  if printf '%s\n' "$wd_section_c" | grep -qF 'Anchor every .factory/ artifact write'; then
    echo "PIPELINE PROBE STRUCTURAL FAIL [Leg C]: mutant appeared in write_discipline_section — it should only be in spec_path_section (injected above #### Write Discipline). awk injection is broken."
    rm -rf "$scratch"
    false
  fi

  local leg_c_result
  leg_c_result="$(printf '%s\n' "$sp_nosplit_c" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -E '\.factory/|ledger|artifact[[:space:]]+writes?' | \
    grep -vE '\*\*Forbidden:\*\*' | \
    grep -Ev 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute.*(main-checkout|\$CANONICAL_FACTORY_ROOT)' | \
    grep -Ev 'MUST[[:space:]]+(NOT|not)[[:space:]]+(use|write|store|save|place|record|emit|persist|resolve)[[:space:]]' | \
    grep -Ev 'MUST[[:space:]]+be[[:space:]]+determined[[:space:]]+via' || true)"
  if [ -z "$leg_c_result" ]; then
    echo "PIPELINE PROBE FAIL [Leg C — F-S2104-P24-002]: '> Anchor every .factory/ artifact write to the story worktree CWD.' above #### Write Discipline MUST fire write-directive gate via the real spec_path_prose pipeline."
    echo "  Marker strip removes '>' leaving 'Anchor every .factory/ artifact write...' → Anchor (bare-imperative) + .factory/ (referent) → gate fires."
    echo "  Gate SILENT means _build_spec_path_section_prose is NOT applying the marker strip to spec_path_prose."
    echo "  This is F-S2104-P24-002: _build_spec_path_section_prose reverted to bare 'tr' (no strip)."
    echo "  spec_path_prose first 200 chars: ${sp_prose_c:0:200}"
    rm -rf "$scratch"
    false
  fi

  # --- Leg D: pristine _shared-context.md must stay GREEN for both gates ---
  # Proves the probe is not trivially firing. Gate PW-B over write_discipline_section must be
  # SILENT; write-directive gate over spec_path_section must be SILENT.
  local wd_section_d wd_prose_d wd_nosplit_d
  wd_section_d="$(_extract_write_discipline_section)"
  wd_prose_d="$(_build_section_prose "$wd_section_d")"
  wd_nosplit_d="$(printf '%s\n' "$wd_prose_d" | \
    sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g')"
  local sp_section_d sp_prose_d sp_nosplit_d
  sp_section_d="$(_extract_spec_path_discipline_section)"
  sp_prose_d="$(_build_section_prose "$sp_section_d")"
  sp_nosplit_d="$(printf '%s\n' "$sp_prose_d" | \
    sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g')"

  local leg_d_pwb leg_d_wdg
  leg_d_pwb="$(printf '%s\n' "$wd_nosplit_d" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E 'CWD-relative|worktree-relative|relative[[:space:]]+paths?|story-worktree[[:space:]]+CWD|story[[:space:]]+worktree[[:space:]]+CWD|worktree'\''s[[:space:]]+shadow|worktree[[:space:]]+CWD|shadow[[:space:]]+subtree|[Ww]orktree-local|(^|[^[:alnum:]])[Ii]n-worktree|gitignored-shadow' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -vE '\*\*Forbidden:\*\*' || true)"
  leg_d_wdg="$(printf '%s\n' "$sp_nosplit_d" | \
    perl -pe 's/\.[[:space:]]+(?=[A-Z*`\[])/.\n/g' | \
    perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g' | \
    grep -E "$PWBD_DIRECTIVE_CLASS" | \
    grep -E '\.factory/|ledger|artifact[[:space:]]+writes?' | \
    grep -vE '\*\*Forbidden:\*\*' | \
    grep -Ev 'MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute.*(main-checkout|\$CANONICAL_FACTORY_ROOT)' | \
    grep -Ev 'MUST[[:space:]]+(NOT|not)[[:space:]]+(use|write|store|save|place|record|emit|persist|resolve)[[:space:]]' | \
    grep -Ev 'MUST[[:space:]]+be[[:space:]]+determined[[:space:]]+via' || true)"
  if [ -n "$leg_d_pwb" ]; then
    echo "PIPELINE PROBE FAIL [Leg D — pristine Gate PW-B MUST be GREEN]: violations found in _shared-context.md #### Write Discipline section:"
    printf '%s\n' "$leg_d_pwb"
    rm -rf "$scratch"
    false
  fi
  if [ -n "$leg_d_wdg" ]; then
    echo "PIPELINE PROBE FAIL [Leg D — pristine write-directive gate MUST be GREEN]: violations found in _shared-context.md ### Spec-Path Discipline section:"
    printf '%s\n' "$leg_d_wdg"
    rm -rf "$scratch"
    false
  fi

  # --- Leg E: call-site parity gate ---
  # Self-referential: greps this bats file's own source for every _prose*="$( assignment and
  # asserts every one routes through a _build_* normalising builder function.
  #
  # Problem class: Legs A–C prove the builders are correctly implemented by calling them
  # directly with synthetic fixtures. They cannot detect a reverted call site in T-001 —
  # e.g., spec_path_prose reverted to bare 'printf … | tr' bypasses the wrapper entirely while
  # Leg C's sp_prose_c still calls _build_spec_path_section_prose directly (Leg C passes; hole
  # open). The same class applies to write_discipline_prose: Leg A calls _build_section_prose
  # directly so the write_discipline_prose call site is not independently tested.
  #
  # Fix: grep own source for all _prose*="$( assignments and assert each contains _build_.
  # A bare-tr revert at write_discipline_prose or spec_path_prose lacks _build_ → gate fires.
  # Structural: closes the class for current AND future prose domains — any future _prose*="$(
  # that bypasses the builder is detected at the first run.
  #
  # SELF-REFERENTIAL NOTE: this gate greps the bats file's own source. A 'not ok' signals
  # that a prose-domain call site in this file bypasses the builder chain, not that a
  # production doc was mutated. Failure message names the offending variable and line.
  local leg_e_this_file leg_e_all_prose_lines leg_e_bare_lines
  leg_e_this_file="${BATS_TEST_DIRNAME}/story-worktree-write-path-discipline.bats"
  # Comment lines excluded (grep -v '^[0-9]*:[[:space:]]*#'): gate must not match its own
  # explanatory text (which contains the literal pattern _prose*="$( for documentation).
  # _nosplit INCLUDED (H03 F-S2104-P26-H03): removed grep -v '_nosplit' exclusion — all gates
  # consume *_nosplit vars, so _prose*_nosplit assignments must route through _build_nosplit().
  # Previously excluded because nosplit vars were constructed inline; now write_discipline_prose_nosplit
  # and spec_path_prose_nosplit are built via _build_nosplit() and must be scanned for parity.
  # leg_e_ excluded: self-referential infrastructure variables (e.g., leg_e_all_prose_lines)
  # must not be scanned — they contain '_prose' in their names but are not prose domains.
  # Pattern broadened (H02 F-S2104-P25-H02): '_prose[a-zA-Z0-9_]*="?\$\(' catches any
  # prose-domain variable regardless of suffix (write_discipline_prose, sp_prose_c, etc.)
  # and handles both quoted and unquoted assignment forms.
  leg_e_all_prose_lines="$(grep -nE '_prose[a-zA-Z0-9_]*="?\$\(' "$leg_e_this_file" | grep -v '^[0-9]*:[[:space:]]*#' | grep -v 'leg_e_')"
  # Safety: the gate must find at least one assignment; zero means the grep pattern broke.
  if [ -z "$leg_e_all_prose_lines" ]; then
    echo "PIPELINE PROBE FAIL [Leg E — call-site parity]: grep pattern found no prose assignments in this bats file — gate integrity check failed (self-referential gate over own source)"
    echo "  Expected: at least write_discipline_prose and spec_path_prose"
    rm -rf "$scratch"
    false
  fi
  # Assert every _prose*="$( assignment calls a _build_* builder (no bare 'printf … | tr' allowed).
  # Mechanism 3 (H03 F-S2104-P26-H03): strip trailing comments before checking for _build_ —
  # a line ending with '# ... _build_section_prose ...' contains '_build_' in the comment but
  # NOT in the assignment itself; grep -v '_build_' would filter it out, hiding a bare construction.
  # Strip comment suffix (from first '# ' onwards) before the _build_ check.
  # M04 fix (F-S2104-P27-M04): changed comment-strip from '[^"]*$' to '.*$'.
  # OLD '[^"]*$': the [^"] class stops substitution at the first double-quote in the comment,
  # so a comment containing '"' (e.g., '# replaces _build_foo "$VAR"') is NOT stripped →
  # _build_ survives in the (non-stripped) comment → grep -v '_build_' omits the line →
  # bare construction goes UNDETECTED (silent FALSE NEGATIVE).
  # NEW '.*$': '.*' is greedy and quote-agnostic, so any trailing comment is fully stripped
  # regardless of double-quote content → _build_ removed → grep -v '_build_' INCLUDES the
  # bare line → gate fires correctly. M04 probe below confirms the fix is load-bearing.
  leg_e_bare_lines="$(printf '%s\n' "$leg_e_all_prose_lines" | \
    sed 's/[[:space:]]*#[[:space:]].*$//' | \
    grep -v '_build_')" || true
  if [ -n "$leg_e_bare_lines" ]; then
    echo "PIPELINE PROBE FAIL [Leg E — call-site parity]: _prose assignment does not route through a _build_* normalising builder."
    echo "  Bare construction (e.g., printf '%s\n' \"\$section\" | tr '\\n' ' ') bypasses marker strip."
    echo "  Expected: every domain construction calls _build_section_prose or _build_spec_path_section_prose."
    echo "  Offending line(s) in this bats file (self-referential gate greps own source):"
    printf '%s\n' "$leg_e_bare_lines"
    echo "  Fix: route through _build_section_prose or a named _build_*_prose wrapper."
    rm -rf "$scratch"
    false
  fi

  # --- M04 probe (F-S2104-P27-M04): comment-strip must handle quote-containing comments ---
  # A bare _prose assignment whose trailing comment contains a double quote (e.g., a variable
  # reference like "$VAR") exposes the OLD [^"]*$ failure mode: [^"] stops at the first '"'
  # in the comment → comment survives → _build_ stays in the line → grep -v '_build_' omits
  # the line → bare construction UNDETECTED (FALSE NEGATIVE, BUG).
  # The NEW .*$ pattern strips any trailing comment regardless of quote content.
  # This probe synthesizes such a line and verifies:
  #   (a) OLD strip leaves _build_ in the line (BUG confirmed);
  #   (b) NEW strip removes _build_ from the line (fix confirmed);
  #   (c) NEW strip still leaves the _prose assignment itself (no over-stripping).
  local m04_probe_line m04_old m04_new m04_bare
  # Probe line: a bare construction assignment; _build_ appears ONLY inside the quote-containing
  # comment. The comment '# _build_spec_path_section_prose "$SHARED_CTX"' contains a '"' before
  # $SHARED_CTX — causing [^"]*$ to stop there and leave the comment intact.
  # NOTE: variable name uses 'm04_bare_call' (not a _prose[a-zA-Z0-9_]*="$( form) to prevent
  # the self-referential Leg E grep from matching this probe line as a real prose assignment.
  # The sed comment-strip behavior being tested is independent of the variable name.
  m04_probe_line='99:  m04_bare_call="$(cat "$var")"  # _build_spec_path_section_prose "$SHARED_CTX"'

  # (a) OLD strip: [^"]*$ stops at first '"' in comment → comment NOT stripped → _build_ survives
  m04_old="$(printf '%s\n' "$m04_probe_line" | sed 's/[[:space:]]*#[[:space:]][^"]*$//')"
  if ! printf '%s\n' "$m04_old" | grep -q '_build_'; then
    echo "M04 PROBE SETUP FAIL: old strip ([^\"]*\$) unexpectedly removed _build_ from quote-containing comment — probe design error or sed behaves differently than expected on this platform"
    rm -rf "$scratch"
    false
  fi

  # (b) NEW strip: .*$ is quote-agnostic → comment fully stripped → _build_ removed
  m04_new="$(printf '%s\n' "$m04_probe_line" | sed 's/[[:space:]]*#[[:space:]].*$//')"
  if printf '%s\n' "$m04_new" | grep -q '_build_'; then
    echo "M04 PROBE FAIL: new strip (.*\$) did NOT remove the quote-containing comment — _build_ still present after strip (fix not effective)"
    rm -rf "$scratch"
    false
  fi

  # (c) NEW strip does not over-strip: assignment target itself remains intact
  # Check for 'm04_bare_call' (the LHS of the probe assignment) — must survive after comment removal.
  if ! printf '%s\n' "$m04_new" | grep -q 'm04_bare_call'; then
    echo "M04 PROBE FAIL: new strip (.*\$) removed the assignment target itself (over-stripping)"
    rm -rf "$scratch"
    false
  fi

  # (d) NEW result: grep -v '_build_' INCLUDES the stripped bare line → gate fires correctly
  m04_bare="$(printf '%s\n' "$m04_new" | grep -v '_build_' || true)"
  if [ -z "$m04_bare" ]; then
    echo "M04 PROBE FAIL: after new strip, grep -v '_build_' produced empty output — bare construction not detected (fix not effective)"
    rm -rf "$scratch"
    false
  fi

  rm -rf "$scratch"
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

  _assert_doc_marker 'find.*\.factory.*![[:space:]]*-type[[:space:]]+d' \
    "step-g-cleanup.md §G.1: find .factory ! -type d preflight command (BC-6.26.001 PC2; M01(a)/M03(a))" \
    "$g1_section"
  _assert_no_doc_marker 'find.*\.factory.*![[:space:]]*-type[[:space:]]+d.*2>/dev/null' \
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
  # F-S2104-P22-003: Tightened to require indented SHELL-EXPRESSION FORM — prose backtick
  # references like `[ ! -e ]` in explanatory sentences ("If `[ ! -e ]` is FALSE...") also
  # matched the prior alternation '\[ ! -e|test...|if...' and satisfied the gate even after
  # the normative indented expression was deleted (all three prose occurrences survive deletion).
  # This gate mirrors the [ -L ] shell-expression gate at F-S2104-P6-003a: ^[[:space:]]+\[ -L.
  # The indented form ^[[:space:]]+\[ ! -e  is structurally distinct from prose backtick \`[ ! -e ]\`
  # (which has a backtick before [, preventing ^[[:space:]]+\[ from matching).
  # MUTANT: delete the indented `    [ ! -e "<worktree-path>/.factory" ]` line from §G.1 →
  #   prose mentions remain → prior regex still GREEN → paper-gate confirmed; this gate → RED ✓.
  _assert_doc_marker '^[[:space:]]+\[ ! -e ' \
    "step-g-cleanup.md §G.1: literal [ ! -e ] shell expression required as indented command — prose backtick references like \`[ ! -e ]\` do NOT satisfy this gate (those survive clause deletion); ^<spaces>[ ! -e form required (BC-6.26.001 EC-008; F-S2104-P4-007a; F-S2104-P22-003)" \
    "$g1_section"

  # Negative: indented [ ! -d ] MUST NOT appear as a normative predicate in §G.1.
  # F-S2104-P22-007: Structural gate — only the INDENTED form ^[[:space:]]+\[ ! -d is normative.
  # Prose backtick mentions `[ ! -d ]` are explanatory (explaining why -d alone is wrong per
  # BC-6.26.001 EC-008) and are structurally exempt — they start with backtick, not whitespace.
  # The prior lexical exclusion (grep -Ev 'MUST NOT|wrong|alone|...') was fail-open: any
  # exemption token in a comment on a normative line (e.g., `[ ! -d "<path>" ]  # alone...`)
  # silently excluded the normative expression. POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION
  # requires ambiguity → BLOCK; the structural form is unambiguous.
  # MUTANT: add `    [ ! -d "<worktree-path>/.factory" ]   # the -e test alone is not required`
  #   → has 'alone' → OLD lexical exclusion silent; NEW structural gate → RED ✓.
  local forbidden_d_normative
  forbidden_d_normative="$(printf '%s\n' "$g1_section" | \
    grep -E '^[[:space:]]+\[ ! -d' || true)"
  if [ -n "$forbidden_d_normative" ]; then
    echo "DOC-PARITY FAIL [must NOT contain: indented [ ! -d ] as normative path-absence predicate — BC-6.26.001 EC-008 forbids -d-only check; prose backtick mentions exempted by structural form; use [ ! -e ] instead (F-S2104-P4-007a; F-S2104-P22-007)]"
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

  _assert_doc_marker 'find.*\.factory.*![[:space:]]*-type[[:space:]]+d' \
    "step-g-cleanup.md §G.1: find .factory ! -type d preflight command (BC-6.26.001 PC2b → PC2a retry path; M01(a)/M03(a))" \
    "$g1_section"
  _assert_no_doc_marker 'find.*\.factory.*![[:space:]]*-type[[:space:]]+d.*2>/dev/null' \
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
  #
  # DOC-PARITY: step-g-cleanup.md §G.1 must contain Option A relocation language
  # (M08 F-S2104-P25-M08 / BC-6.26.001 PC2b). Without this assertion, the nesting guard is
  # tautological — it verifies test fixture logic, not that the spec mandates correct relocation.
  _assert_doc_marker 'Option A:.*[Rr]elocat' \
    "step-g-cleanup.md §G.1: Option A relocation language required for nesting guard (BC-6.26.001 PC2b; M08 F-S2104-P25-M08)" \
    "$g1_section"
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

  # --- DOC-PARITY §G.1: PC2c semantic direction gates (F-S2104-P2-008) ---
  # F-S2104-P22-010: root-skip moved to AFTER doc-parity gates (just before the chmod 000 fixture).
  # The prior placement skipped the entire test before any doc-parity assertion ran on root
  # runners — doc-parity correctness was not verified at all on root CI. The harness gates
  # require chmod 000 to be effective (only meaningful for non-root) but doc-parity gates
  # read documentation files and require no filesystem fixture. Separated here so both
  # coverage classes are preserved: doc-parity always executes; harness skips on root.
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

  # PC2c block must NOT contain AFFIRMATIVE proceed-forward semantics.
  # F-S2104-P22-008: Negation-transparent — the prior gate '_assert_no_doc_marker
  # [Pp]roceed[[:space:]]+(to|with)' was non-transparent: "Do NOT proceed to git worktree remove"
  # (a valid prohibition phrasing in PC2c) fired the gate even though it NEGATES proceed.
  # The sibling PC2b gate at F-S2104-P3-011 uses the narrow form '[Pp]roceed[[:space:]]+to
  # [[:space:]]+the[[:space:]]+[Dd]ispatch' — affirmative-only by construction. PC2c mirrors
  # that approach: after extracting lines containing 'proceed to/with', filter out negated lines
  # (those containing NOT/do not/must not). POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION:
  # ambiguity → BLOCK; negated-proceed is not ambiguous — it is the opposite of authorization.
  # MUTANT: "Proceed with the teardown per BC-6.26.001 PC2c." → not negated → RED ✓.
  # CONTROL: "Do NOT proceed to git worktree remove." → negated → excluded → GREEN ✓.
  local pc2c_proceed_fwd
  pc2c_proceed_fwd="$(printf '%s\n' "$pc2c_block" | \
    grep -iE '[Pp]roceed[[:space:]]+(to|with)[[:space:]]' | \
    grep -viE '\bNOT[[:space:]]+proceed|\bdo[[:space:]]+not[[:space:]]+proceed|[Mm]ust[[:space:]]+[Nn]ot[[:space:]]+proceed|\bno[[:space:]]+proceed' || true)"
  if [ -n "$pc2c_proceed_fwd" ]; then
    echo "DOC-PARITY FAIL [step-g-cleanup.md §G.1 PC2c block: affirmative proceed-forward semantics (negation-transparent; F-S2104-P22-008)]: a PC2c→proceed mutant keeping the label while adding proceed semantics is caught here; negated forms (Do NOT proceed to...) are exempt"
    printf '%s\n' "$pc2c_proceed_fwd"
    false
  fi

  # F-S2104-P22-010: doc-parity legs complete. Skip harness legs if running as root —
  # chmod 000 is ineffective for root (find succeeds regardless of permissions); harness
  # assertions require an effective permission-denied scenario to exercise PC2c.
  if [ "$(id -u)" -eq 0 ]; then
    printf 'T-004 coverage: doc-parity legs=4 EXECUTED; harness legs=0 SKIPPED (root runner)\n'
    skip "T-004 harness gates require non-root user (chmod 000 is ineffective as root; find would succeed)"
  fi

  # --- Fixture: .factory/ with a permission-locked subdirectory ---
  mkdir -p "$MOCK_WORKTREE/.factory/locked-subdir"
  chmod 000 "$MOCK_WORKTREE/.factory/locked-subdir"

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
  # F-S2104-P22-003 tightening: require INDENTED SHELL-EXPRESSION form (^<spaces>[ ! -e ),
  # mirroring the [ -L ] gate at F-S2104-P6-003a. Prose backtick `[ ! -e ]` survives deletion
  # and would satisfy the prior alternation — this gate requires the normative indented expression.
  _assert_doc_marker '^[[:space:]]+\[ ! -e ' \
    "step-g-cleanup.md §G.1: [ ! -e ] indented shell expression required (not prose backtick form) — prose mentions survive clause deletion; structural form required (BC-6.26.001 EC-008; F-S2104-P4-007a; F-S2104-P22-003)" \
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

  # --- DOC-PARITY §G.1: ORDERING — [ ! -e ] must precede [ -L ] (F-S2104-P22-003b) ---
  # BC-6.26.001 step-1-before-step-2: path-absence check [ ! -e ] (step 1) MUST precede the
  # symlink check [ -L ] (step 2). An ordering inversion would run the symlink test before the
  # absence test, potentially running [ -L ] on an absent path (which is harmless on most shells
  # but violates the documented discrimination chain order). This gate mirrors the [ -L ]-before-
  # find ordering gate below — both use awk line-number comparison.
  # MUTANT: swap the order of the two indented expressions in §G.1 → gate fires → RED ✓.
  local bracket_e_lineno bracket_l_lineno
  bracket_e_lineno="$(printf '%s\n' "$g1_section" | awk '/^[[:space:]]+\[ ! -e / { print NR; exit }')"
  bracket_l_lineno="$(printf '%s\n' "$g1_section" | awk '/^[[:space:]]+\[ -L / { print NR; exit }')"
  [ -n "$bracket_e_lineno" ] || {
    echo "DOC-PARITY FAIL: [ ! -e ] shell expression not found in §G.1 section — bracket-!-e must be present as an indented command (BC-6.26.001 PC2a; F-S2104-P22-003b)"
    false
  }
  [ -n "$bracket_l_lineno" ] || {
    echo "DOC-PARITY FAIL: [ -L ] shell expression not found in §G.1 section — bracket-L must be present as an indented command for step-1-before-step-2 check (BC-6.26.001 PC2b symlink; F-S2104-P22-003b)"
    false
  }
  [ "$bracket_e_lineno" -lt "$bracket_l_lineno" ] || {
    echo "DOC-PARITY FAIL: [ ! -e ] line ($bracket_e_lineno) must precede [ -L ] line ($bracket_l_lineno) in §G.1 — ordering inversion violates BC-6.26.001 step-1-before-step-2 discrimination chain (F-S2104-P22-003b)"
    false
  }

  # --- DOC-PARITY §G.1: ORDERING — [ -L ] must precede first find invocation (F-S2104-P6-003b) ---
  # The [ -L ] check must appear BEFORE the find command within §G.1; an ordering inversion would
  # allow find to be called on a symlink-to-dir (which satisfies [ -d ] by dereferencing). Uses
  # the same awk line-number comparison pattern as the pass-2 preflight-before-dispatch gate.
  local find_lineno
  find_lineno="$(printf '%s\n' "$g1_section" | awk '/^[[:space:]]*find[[:space:]]/ { print NR; exit }')"
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
# T-010 / EC-009 / BC-6.26.001 PC2b predicate-widening (M03(a)):
# Stray symlink + FIFO INSIDE a real shadow .factory/ directory
# → PREFLIGHT BLOCKED via ! -type d; missed by retired -type f
# Load-bearing delta proof of M03(a) widening — F-S2104-P28-006
# ===========================================================================

@test "T-010 S-21.04 EC-009: stray-inode-inside-factory — symlink + FIFO inside real .factory/ dir → PREFLIGHT BLOCKED via ! -type d; missed by -type f predicate (M03(a) delta proof)" {
  # EC-009: a stray symlink (type 'l') or FIFO (type 'p') INSIDE a real shadow .factory/
  # directory is returned by 'find ... ! -type d' because type l and type p both satisfy
  # the negation (l != d, p != d). The retired '-type f' predicate misses both: symlinks
  # and FIFOs are NOT regular files, so '-type f' returns empty and teardown incorrectly
  # proceeds. This test is the load-bearing proof of the M03(a) predicate widening.
  #
  # Key distinction from T-006 (symlink AT .factory path):
  #   T-006: $MOCK_WORKTREE/.factory is itself a SYMLINK → caught by [ -L ] at step 2 of
  #          the discrimination chain; find is never invoked.
  #   T-010: $MOCK_WORKTREE/.factory is a REAL DIRECTORY → [ -L ] does not fire; find IS
  #          invoked and must detect stray non-directory inodes INSIDE the directory.
  #
  # FIFO coverage rationale: mkfifo creates a named pipe (POSIX type 'p'), available on
  # macOS and Linux CI. FIFOs represent a distinct inode class from symlinks (type l).
  # Both satisfy '! -type d' and both fail '-type f'. Including both proves the widening
  # covers the whole non-regular-non-directory inode space, not just symlinks.
  # Device nodes (mknod) require root on most Linux systems — excluded (not CI-portable).
  # Unix domain sockets require additional cleanup — excluded (handled separately if needed).
  #
  # Red gate proof (why this test fails under -type f revert):
  #   Under -type f: find returns empty (symlinks and FIFOs are not regular files) →
  #     _run_teardown_preflight returns 0 (PC2a proceeds) → REMOVE_LOG written → the
  #     [ ! -s "$REMOVE_LOG" ] assertion fails → RED ✓.
  #   Under ! -type d: find returns both inodes → PREFLIGHT BLOCKED → status non-zero →
  #     the [ "$status" -ne 0 ] assertion passes → GREEN ✓.
  # Note: the extraction grep in _run_teardown_preflight also requires '! -type d', so
  # reverting §G.1 to '-type f' causes HARNESS FAIL (extraction failure) before the
  # behavioral delta can manifest. The direct find assertions below are the primary
  # delta proof — they bypass the extraction gate and fail purely on find semantics.

  # --- Fixture setup: real .factory/ dir containing stray symlink and FIFO ---
  mkdir -p "$MOCK_WORKTREE/.factory/stories"
  # Stray symlink inside real .factory/ (type l — satisfies ! -type d, fails -type f)
  ln -s /dev/null "$MOCK_WORKTREE/.factory/stray-shadow-symlink"
  # Stray FIFO inside real .factory/ (type p — satisfies ! -type d, fails -type f)
  mkfifo "$MOCK_WORKTREE/.factory/stray-fifo"

  # --- Load-bearing delta proof: -type f misses both inodes; ! -type d catches both ---
  # These direct find assertions prove the behavioral delta independently of the extraction
  # gate. They constitute the Red Gate evidence for EC-009: reverting to -type f makes
  # find return empty, which would suppress PREFLIGHT BLOCKED and authorize teardown.
  local find_type_f_result find_not_type_d_result
  find_type_f_result="$(find "$MOCK_WORKTREE/.factory" -type f 2>/dev/null || true)"
  find_not_type_d_result="$(find "$MOCK_WORKTREE/.factory" ! -type d 2>/dev/null || true)"

  # -type f must return empty (symlinks and FIFOs are not regular files)
  # If non-empty: the fixture accidentally contains a regular file; EC-009 delta proof is invalid.
  if [ -n "$find_type_f_result" ]; then
    echo "EC-009 DELTA FAIL: 'find ... -type f' returned non-empty on a fixture containing only symlinks and FIFOs — a regular file was unexpectedly present in .factory/; fixture must contain NO regular files for the predicate-delta proof (BC-6.26.001 EC-009; M03(a))"
    false
  fi

  # ! -type d must detect the stray symlink
  printf '%s\n' "$find_not_type_d_result" | grep -q 'stray-shadow-symlink' || {
    echo "EC-009 DELTA FAIL: 'find ... ! -type d' did not return the stray symlink path — symlinks (type l) must satisfy '! -type d' (l != d); this is the core of the M03(a) predicate-widening proof (BC-6.26.001 EC-009)"
    false
  }

  # ! -type d must detect the stray FIFO
  printf '%s\n' "$find_not_type_d_result" | grep -q 'stray-fifo' || {
    echo "EC-009 DELTA FAIL: 'find ... ! -type d' did not return the FIFO path — FIFOs (type p) must satisfy '! -type d' (p != d); FIFO coverage proves the widening applies to all non-directory inodes, not just symlinks (BC-6.26.001 EC-009)"
    false
  }

  # --- HARNESS: run the doc-extracted preflight on the EC-009 fixture ---
  # _run_teardown_preflight extracts '! -type d' from §G.1 and runs it on the fixture.
  # With the correct predicate, find returns the stray symlink and FIFO → PREFLIGHT BLOCKED.
  run _run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG"
  [ "$status" -ne 0 ] || {
    echo "HARNESS FAIL: EC-009 fixture (stray symlink + FIFO inside real .factory/) must return non-zero (PREFLIGHT BLOCKED) — got status 0; 'find ... ! -type d' must detect non-directory inodes INSIDE a real shadow .factory/ directory (BC-6.26.001 EC-009; M03(a))"
    false
  }
  printf '%s\n' "$output" | grep -q 'PREFLIGHT BLOCKED' || {
    echo "HARNESS FAIL: 'PREFLIGHT BLOCKED' not in output for EC-009 fixture (stray symlink + FIFO inside real .factory/) — got: $output"
    false
  }

  # Stray symlink path must appear in the PREFLIGHT BLOCKED message
  printf '%s\n' "$output" | grep -q 'stray-shadow-symlink' || {
    echo "HARNESS FAIL: stray symlink path 'stray-shadow-symlink' must appear in PREFLIGHT BLOCKED output (BC-6.26.001 PC2b; EC-009) — got: $output"
    false
  }

  # Stray FIFO path must appear in the PREFLIGHT BLOCKED message
  printf '%s\n' "$output" | grep -q 'stray-fifo' || {
    echo "HARNESS FAIL: stray FIFO path 'stray-fifo' must appear in PREFLIGHT BLOCKED output (BC-6.26.001 PC2b; EC-009) — got: $output"
    false
  }

  # Mutant-proving sentinel: git worktree remove MUST NOT be invoked on PREFLIGHT BLOCKED path
  [ ! -s "$REMOVE_LOG" ] || {
    echo "HARNESS FAIL: REMOVE_LOG non-empty on EC-009 PREFLIGHT BLOCKED path — git worktree remove MUST NOT be invoked when stray non-directory inodes are found inside .factory/ (BC-6.26.001 PC2b; EC-009) — log: $(cat "$REMOVE_LOG")"
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
  # F-S2104-P22-009: Also catches the option-first form `find -type f <path>/.factory/`
  # (e.g. `find -type f ".worktrees/STORY-NNN/.factory"`). A sibling-surface mutant reordering
  # the flags to `find -type f <path>` evades the path-first pattern but is semantically
  # identical — both inline the find command rather than delegating to §G.1 preflight.
  # MUTANT: `find -type f ".worktrees/S-21.04/.factory/"` → option-first pattern RED ✓.
  # CONTROL: "Run the §G.1 preflight in step-g-cleanup.md before calling git worktree remove" → GREEN ✓.
  _assert_no_inline_find_antipattern() {
    local file="$1" label="$2"
    local antipattern_found=false
    # Path-first form (predicate-agnostic): find <path>/.factory[/]... <any-predicate>
    # Catches any inline find command where the .factory path appears as the first argument,
    # regardless of predicate. The regex engine backtracks through [^[:space:]]* to find
    # .factory as a suffix of the first non-space token after 'find'.
    # POLICY 13 ALTERNATION-WIDENING-DIRECTION-STATEMENT: widened from -type f specific
    # to predicate-agnostic at F-S2104-P28-007 — cannot be re-opened by the next predicate
    # change (exactly how this defect arose when M03 widened -type f to ! -type d).
    # Catches: 'find .factory -type f' (old), 'find ".factory/" ! -type d' (M03 canonical),
    # any future predicate form where .factory appears as the primary path argument.
    # The annotation blocks in surface files use Unicode '…' (U+2026) between 'find' and
    # '.factory/', creating a space gap that prevents the pattern from matching — annotation
    # prose is correctly excluded; only actual inline find commands fire.
    if grep -qE 'find[[:space:]]+[^[:space:]]*\.factory' "$file"; then
      antipattern_found=true
    fi
    # Option-first form (type-agnostic): find [-!] -type <val> <path>/.factory (F-S2104-P22-009)
    # A mutant reordering flags to 'find -type f <path>/.factory' evades the path-first pattern.
    # After M03, the M03 option-first form is 'find ! -type d <path>/.factory'.
    # Widened from -type f specific to any -type value (F-S2104-P28-007).
    # MUTANT: 'find ! -type d ".worktrees/S-21.04/.factory/"' → fires → RED ✓.
    if grep -qE 'find[[:space:]]+(![[:space:]]+)?-type[[:space:]]+[^[:space:]]+[[:space:]]+[^[:space:]]*\.factory' "$file"; then
      antipattern_found=true
    fi
    # Collapsed-content check (M02 F-S2104-P25-M02): .lobster YAML files may use folded-block
    # scalars where the find command spans multiple lines; line-by-line grep misses these.
    # Collapsing the file to a single line with 'tr' catches find commands that are folded
    # across newlines in YAML flow or block style.
    # Both patterns widened to predicate-agnostic / type-agnostic (F-S2104-P28-007).
    local collapsed
    collapsed="$(tr '\n' ' ' < "$file")"
    if printf '%s\n' "$collapsed" | grep -qE 'find[[:space:]]+[^[:space:]]*\.factory'; then
      antipattern_found=true
    fi
    if printf '%s\n' "$collapsed" | grep -qE 'find[[:space:]]+(![[:space:]]+)?-type[[:space:]]+[^[:space:]]+[[:space:]]+[^[:space:]]*\.factory'; then
      antipattern_found=true
    fi
    if [ "$antipattern_found" = true ]; then
      echo "DOC-PARITY FAIL [anti-pattern present in $label]: surface presents inline 'find ... .factory[/] ...' (path-first OR option-first, any predicate form) — MUST NOT inline find command; delegate to §G.1 preflight instead (BC-6.26.001 PC2 + AC-007(d); absent-path check is first, not an unordered sibling; F-S2104-P4-009; F-S2104-P22-009; F-S2104-P28-007 predicate-agnostic widening)"
      false
    fi
  }

  # Helper: assert fully-qualified step-g-cleanup.md path present with ordering and mandate-token.
  # F-S2104-P22-009: Prior gate gated only presence — a surface could satisfy it by citing the
  # §G.1 path in a footer comment AFTER the git-worktree-remove call, or without a mandatory
  # framing. Two additional gates close these escape hatches:
  #   (i)  Mandate-token: the §G.1 reference line must carry MUST/required/mandatory/BEFORE —
  #        a mere "see also step-g-cleanup.md" satisfies presence but not mandate.
  #   (ii) Ordering: if git worktree remove also appears in the file, the §G.1 reference line
  #        must precede it (preflight before removal, not after).
  # MUTANT (mandate): "For context see step-g-cleanup.md §G.1." → no MUST token → RED ✓.
  # MUTANT (ordering): §G.1 cited after "git worktree remove..." line → ordering fails → RED ✓.
  # CONTROL: "MUST run step-g-cleanup.md §G.1 preflight before git worktree remove" → GREEN ✓.
  _assert_g1_ref() {
    local file="$1" label="$2"
    # Presence gate (unchanged)
    grep -qE 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup\.md' "$file" || {
      echo "DOC-PARITY FAIL [fully-qualified §G.1 path missing from $label]: surface must carry fully-qualified path 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md' — bare §G.1 or step-g-cleanup alone is insufficient for cross-document traceability (BC-6.26.001 PC2 + AC-007(d); F-S2104-P4-009 / F-S2104-P9-class)"
      false
    }
    # Mandate-token gate (F-S2104-P22-009 / F-S2104-P23-012): the §G.1 reference must appear in a
    # mandatory context. Accepted mandate tokens are those that GENUINELY establish an obligation:
    #   MUST / must, required, mandatory, BEFORE (directional obligation),
    #   "proceed only" / "only on" (conditional-proceed mandate form).
    # Removed near-vacuous tokens (F-S2104-P23-012): 'Run/run', 'before', 'first' are
    # near-ubiquitous in procedural docs — "Always run" / "This path was run before the §G.1
    # refactor" both satisfy them incidentally without establishing obligation.
    # rules/worktree-protocol.md formerly passed on "Always run" (run/before match); updated
    # to carry "MUST" on the step-g-cleanup.md reference line as part of this fix.
    # A "for context, see step-g-cleanup.md §G.1" footnote reference with "run" or "before"
    # elsewhere in the file satisfied the old class; new class requires MUST/required/mandatory/
    # BEFORE/proceed-only → that sentence returns non-zero (load-bearing).
    local g1_ref_line g1_mandated_line
    # Check ALL lines containing the qualified path for any genuine mandate token (not just head -1).
    # '\bBEFORE\b' removed (F-S2104-P23-012): with -iE, 'BEFORE' matches lowercase 'before' which
    # is near-ubiquitous in procedural text ("run before every git worktree remove") and does not
    # establish obligation by itself. Remaining tokens all carry unambiguous imperative force.
    g1_mandated_line="$(grep -E 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup\.md' "$file" | \
      grep -iE '\bMUST\b|\brequired\b|\bmandatory\b|\bproceed only\b|\bonly on\b' | head -1 || true)"
    if [ -z "$g1_mandated_line" ]; then
      g1_ref_line="$(grep -E 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup\.md' "$file" | head -1)"
      echo "DOC-PARITY FAIL [§G.1 reference in $label lacks mandate token (F-S2104-P22-009 / F-S2104-P23-012)]: no line containing the qualified step-g-cleanup.md path also carries a genuine mandate token (MUST/required/mandatory/proceed-only/only-on)"
      printf 'Found line: %s\n' "$g1_ref_line"
      # return 1 (not bare 'false') — 'false' followed by the ordering-gate 'if' returns 0 when
      # the ordering gate is skipped (no git worktree remove in file), silently masking the error.
      return 1
    fi
    # Ordering gate (F-S2104-P22-009 / M01 F-S2104-P25-M01): if git worktree remove appears in
    # the file, the §G.1 MANDATE reference must precede it — the mandate line establishes the
    # obligation; a mere §G.1 citation in a footer comment after the removal call satisfies the
    # presence and mandate gates above but not the ordering gate.
    # Bound to mandate line (M01 fix): prior gate used first path occurrence (head -1 on all
    # path matches), which could be a non-mandating reference (e.g., a "see also" comment).
    # The mandate line is the same line used for the mandate-token gate above (g1_mandated_line).
    local g1_mandated_lineno wt_remove_lineno
    g1_mandated_lineno="$(grep -nE 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup\.md' "$file" | \
      grep -iE '\bMUST\b|\brequired\b|\bmandatory\b|\bproceed only\b|\bonly on\b' | head -1 | cut -d: -f1)"
    wt_remove_lineno="$(grep -nE 'git[[:space:]]+worktree[[:space:]]+remove' "$file" | head -1 | cut -d: -f1 || true)"
    if [ -n "$wt_remove_lineno" ]; then
      [ "$g1_mandated_lineno" -lt "$wt_remove_lineno" ] || {
        echo "DOC-PARITY FAIL [§G.1 mandated reference must precede git worktree remove in $label (F-S2104-P22-009 / M01 F-S2104-P25-M01)]: §G.1 mandate line at $g1_mandated_lineno, git worktree remove at line $wt_remove_lineno — preflight mandate reference must appear before the removal call"
        false
      }
    fi
  }

  # Mandate-token probe (F-S2104-P23-012): a sentence with near-vacuous tokens ('run', 'before')
  # but no genuine mandate token MUST cause _assert_g1_ref to return non-zero.
  # This proves the old broad class (\brun\b|\bbefore\b) was near-vacuous: "This path was run
  # before the §G.1 refactor" would have passed the old class (run + before match) while
  # carrying no actual obligation. The new class (MUST/required/mandatory/BEFORE/proceed-only)
  # correctly rejects this sentence.
  # CONTROL (new narrowed class): non-mandating sentence → _assert_g1_ref returns non-zero (RED for sentence) ✓.
  # REVERSION (old broad class with \brun\b|\bbefore\b): same sentence passes → probe fires → test RED ✓.
  local _p23012_probe_file
  _p23012_probe_file="$(mktemp)"
  printf 'This path was run before the G.1 refactor (plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1).\n' > "$_p23012_probe_file"
  run _assert_g1_ref "$_p23012_probe_file" "non-mandating-probe"
  rm -f "$_p23012_probe_file"
  [ "$status" -ne 0 ] || {
    echo "MANDATE-TOKEN PROBE FAIL (F-S2104-P23-012): non-mandating sentence ('run'+'before' only, no MUST/required/proceed-only) with the qualified §G.1 path passed the mandate gate — near-vacuous tokens still present in class; expected non-zero status"
    false
  }

  # --- POLICY 13 ALTERNATION-WIDENING-DIRECTION-STATEMENT: mutant probes for F-S2104-P28-007 ---
  # Prove the predicate-agnostic widening fires on the M03(a) canonical ! -type d inline form.
  # Path-first probe: 'find "<path>/.factory" ! -type d' (the canonical M03 inline form).
  # Before widening (only -type f caught), this bypassed all four patterns → surfaces could
  # inline ! -type d without triggering the antipattern gate. After widening to predicate-
  # agnostic path-first, this probe fires → _assert_no_inline_find_antipattern returns non-zero.
  local _p28007_pathfirst_probe
  _p28007_pathfirst_probe="$(mktemp)"
  printf 'find ".worktrees/S-21.04/.factory" ! -type d\n' > "$_p28007_pathfirst_probe"
  run _assert_no_inline_find_antipattern "$_p28007_pathfirst_probe" "p28007-path-first-probe"
  rm -f "$_p28007_pathfirst_probe"
  [ "$status" -ne 0 ] || {
    echo "WIDENING-MUTANT FAIL (F-S2104-P28-007 path-first): 'find \".worktrees/S-21.04/.factory\" ! -type d' passed the antipattern gate — predicate-agnostic widening not effective on the M03 canonical path-first form; the old gate only caught -type f; expected non-zero status (F-S2104-P28-007)"
    false
  }
  # Option-first probe: 'find ! -type d "<path>/.factory/"' (option-first M03 form).
  # Widens the option-first pattern from -type f specific to any -type value.
  local _p28007_optionfirst_probe
  _p28007_optionfirst_probe="$(mktemp)"
  printf 'find ! -type d ".worktrees/S-21.04/.factory/"\n' > "$_p28007_optionfirst_probe"
  run _assert_no_inline_find_antipattern "$_p28007_optionfirst_probe" "p28007-option-first-probe"
  rm -f "$_p28007_optionfirst_probe"
  [ "$status" -ne 0 ] || {
    echo "WIDENING-MUTANT FAIL (F-S2104-P28-007 option-first): 'find ! -type d \".worktrees/S-21.04/.factory/\"' passed the antipattern gate — type-agnostic widening not effective on option-first ! -type d form; expected non-zero status (F-S2104-P28-007)"
    false
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
