#!/usr/bin/env bats
# codify-lessons.bats — TDD tests for S-7.01 + S-7.02 codify-lessons delivery
#
# S-7.01: Agent prompt discipline (story-writer, product-owner, adversary)
# S-7.02: Defensive sweep + hook + meta-rule

# ---------- S-7.01: Agent prompt tests (BC-5.36.001–007) ----------

@test "BC-5.36.001: story-writer.md contains spec-first gate" {
  grep -q "behavioral_contracts.*non-empty" plugins/vsdd-factory/agents/story-writer.md
  grep -qE "BC-\\\\d\\+\\\\.\\\\d\\{2\\}\\\\.\\\\d\\{3\\}" plugins/vsdd-factory/agents/story-writer.md
}

@test "BC-5.36.002: story-writer.md requires AC↔BC bidirectional trace" {
  grep -qi "AC.*BC.*bidirectional" plugins/vsdd-factory/agents/story-writer.md
}

@test "BC-5.36.003: product-owner.md requires Capability Anchor Justification cell" {
  grep -q "Capability Anchor Justification" plugins/vsdd-factory/agents/product-owner.md
}

@test "BC-5.36.004: product-owner.md requires verbatim citation from capabilities.md" {
  grep -qi "verbatim" plugins/vsdd-factory/agents/product-owner.md
  grep -q "capabilities.md" plugins/vsdd-factory/agents/product-owner.md
}

@test "BC-5.36.005: adversary.md requires partial-fix-regression check" {
  grep -qi "partial.fix.regression" plugins/vsdd-factory/agents/adversary.md
}

@test "BC-5.36.006: adversary.md requires propagation check to bodies, sibling files, prose" {
  grep -qi "bodies" plugins/vsdd-factory/agents/adversary.md
  grep -qi "sibling files" plugins/vsdd-factory/agents/adversary.md
  grep -qi "prose" plugins/vsdd-factory/agents/adversary.md
}

@test "BC-5.36.007: all three agents updated atomically (commit verification)" {
  # Verify all 3 agent files were modified in the same commit
  cd /Users/jmagady/Dev/vsdd-factory/.worktrees/codify-lessons
  files_in_last_commit=$(git diff --name-only HEAD~1 HEAD | grep -c 'agents/')
  [ "$files_in_last_commit" -ge 3 ]
}

# ---------- S-7.02: Defensive sweep + hook + meta-rule (BC-5.37.001–002, BC-7.05.001–004, BC-8.28.001–002) ----------

@test "BC-5.37.001: state-manager.md requires defensive sweep" {
  grep -qi "defensive sweep" plugins/vsdd-factory/agents/state-manager.md
  grep -qi "corpus.wide grep" plugins/vsdd-factory/agents/state-manager.md
}

@test "BC-5.37.002: state-manager.md requires sweep result logging" {
  grep -qi "log.*sweep result" plugins/vsdd-factory/agents/state-manager.md
}

@test "BC-7.05.001: validate-count-propagation.sh exists and is executable" {
  [ -x plugins/vsdd-factory/hooks/validate-count-propagation.sh ]
}

@test "BC-7.05.001: validate-count-propagation.sh exits 2 on drift" {
  # Set up fixture with intentional drift
  TMPTEST=$(mktemp -d)
  cat > "$TMPTEST/BC-INDEX.md" <<'HEREDOC'
---
total_bcs: 100
---
HEREDOC
  cat > "$TMPTEST/STATE.md" <<'HEREDOC'
BCs | 200 |
HEREDOC
  # Run hook on STATE.md change — capture exit status without triggering set -e
  run bash plugins/vsdd-factory/hooks/validate-count-propagation.sh \
    <<< '{"tool_input":{"file_path":"'"$TMPTEST"'/STATE.md"}}'
  [ "$status" -eq 2 ]
}

@test "BC-7.05.002: validate-count-propagation.sh runs <500ms" {
  start=$(date +%s%N)
  echo '{"tool_input":{"file_path":"/dev/null"}}' | bash plugins/vsdd-factory/hooks/validate-count-propagation.sh || true
  end=$(date +%s%N)
  elapsed_ms=$(( (end - start) / 1000000 ))
  [ "$elapsed_ms" -lt 500 ]
}

@test "BC-7.05.003: validate-template-compliance.sh enforces VP multi-BC convention" {
  grep -qi "multi.bc\|source_bc.*primary\|bcs.*list" plugins/vsdd-factory/bin/validate-template-compliance.sh ||
  grep -qi "source_bc" plugins/vsdd-factory/bin/validate-template-compliance.sh
}

@test "BC-7.05.004: hooks-registry.toml registers validate-count-propagation" {
  grep -q "validate-count-propagation" plugins/vsdd-factory/hooks-registry.toml
}

@test "BC-8.28.001: lessons-codification.md exists" {
  [ -f plugins/vsdd-factory/rules/lessons-codification.md ]
  grep -qi "novel.*process catch.*codification follow-up" plugins/vsdd-factory/rules/lessons-codification.md
}

@test "BC-8.28.002: orchestrator references lessons-codification.md in cycle-closing" {
  grep -q "lessons-codification" plugins/vsdd-factory/agents/orchestrator/orchestrator.md
}
