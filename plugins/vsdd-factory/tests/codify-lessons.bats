#!/usr/bin/env bats
# codify-lessons.bats — TDD tests for S-7.01 + S-7.02 codify-lessons delivery
#
# S-7.01: Agent prompt discipline (story-writer, product-owner, adversary)
# S-7.02: Defensive sweep + hook + meta-rule

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
}

# ---------- S-7.01: Agent prompt tests (BC-5.36.001–007) ----------

@test "BC-5.36.001: story-writer.md contains spec-first gate" {
  grep -q "behavioral_contracts.*non-empty" "$PLUGIN_ROOT/agents/story-writer.md"
  grep -qE "BC-\\\\d\\+\\\\.\\\\d\\{2\\}\\\\.\\\\d\\{3\\}" "$PLUGIN_ROOT/agents/story-writer.md"
}

@test "BC-5.36.002: story-writer.md requires AC↔BC bidirectional trace" {
  grep -qi "AC.*BC.*bidirectional" "$PLUGIN_ROOT/agents/story-writer.md"
}

@test "BC-5.36.003: product-owner.md requires Capability Anchor Justification cell" {
  grep -q "Capability Anchor Justification" "$PLUGIN_ROOT/agents/product-owner.md"
}

@test "BC-5.36.004: product-owner.md requires verbatim citation from capabilities.md" {
  grep -qi "verbatim" "$PLUGIN_ROOT/agents/product-owner.md"
  grep -q "capabilities.md" "$PLUGIN_ROOT/agents/product-owner.md"
}

@test "BC-5.36.005: adversary.md requires partial-fix-regression check" {
  grep -qi "partial.fix.regression" "$PLUGIN_ROOT/agents/adversary.md"
}

@test "BC-5.36.006: adversary.md requires propagation check to bodies, sibling files, prose" {
  grep -qi "bodies" "$PLUGIN_ROOT/agents/adversary.md"
  grep -qi "sibling files" "$PLUGIN_ROOT/agents/adversary.md"
  grep -qi "prose" "$PLUGIN_ROOT/agents/adversary.md"
}

# BC-5.36.007 (REMOVED 2026-05-04, TD-020 sweep): Asserted that all three
# agents were touched in the codify-lessons delivery worktree. The worktree
# `.worktrees/codify-lessons` no longer exists post-merge, so the assertion
# was structurally impossible to satisfy. The contract (agents updated)
# is still validated by BC-5.36.001–006 which inspect the merged files.

# ---------- S-7.02: Defensive sweep + hook + meta-rule (BC-5.37.001–002, BC-7.05.001–004, BC-8.28.001–002) ----------

@test "BC-5.37.001: state-manager.md requires defensive sweep" {
  grep -qi "defensive sweep" "$PLUGIN_ROOT/agents/state-manager.md"
  grep -qi "corpus.wide grep" "$PLUGIN_ROOT/agents/state-manager.md"
}

@test "BC-5.37.002: state-manager.md requires sweep result logging" {
  grep -qi "log.*sweep result" "$PLUGIN_ROOT/agents/state-manager.md"
}

@test "BC-7.05.001: validate-count-propagation.sh exists and is executable" {
  [ -x "$PLUGIN_ROOT/hooks/validate-count-propagation.sh" ]
}

@test "BC-7.05.001-drift: validate-count-propagation.sh exits 2 on drift" {
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
  run bash "$PLUGIN_ROOT/hooks/validate-count-propagation.sh" \
    <<< '{"tool_input":{"file_path":"'"$TMPTEST"'/STATE.md"}}'
  [ "$status" -eq 2 ]
}

@test "BC-7.05.002: validate-count-propagation.sh runs <500ms" {
  start=$(date +%s%N)
  echo '{"tool_input":{"file_path":"/dev/null"}}' | bash "$PLUGIN_ROOT/hooks/validate-count-propagation.sh" || true
  end=$(date +%s%N)
  elapsed_ms=$(( (end - start) / 1000000 ))
  [ "$elapsed_ms" -lt 500 ]
}

@test "BC-7.05.003: validate-template-compliance.sh enforces VP multi-BC convention" {
  grep -qi "multi.bc\|source_bc.*primary\|bcs.*list" "$PLUGIN_ROOT/bin/validate-template-compliance.sh" ||
  grep -qi "source_bc" "$PLUGIN_ROOT/bin/validate-template-compliance.sh"
}

@test "BC-7.05.004: hooks-registry.toml registers validate-count-propagation" {
  grep -q "validate-count-propagation" "$PLUGIN_ROOT/hooks-registry.toml"
}

@test "BC-8.28.001: lessons-codification.md exists" {
  [ -f "$PLUGIN_ROOT/rules/lessons-codification.md" ]
  grep -qi "novel.*process catch.*codification follow-up" "$PLUGIN_ROOT/rules/lessons-codification.md"
}

@test "BC-8.28.002: orchestrator references lessons-codification.md in cycle-closing" {
  grep -q "lessons-codification" "$PLUGIN_ROOT/agents/orchestrator/orchestrator.md"
}
