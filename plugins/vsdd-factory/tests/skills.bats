#!/usr/bin/env bats
# skills.bats — structural tests for discipline-skill behavior-shaping content
#
# Asserts that the four discipline skills carry their Iron Law, "Announce at
# start" line, and Red Flags table. These are not prose niceties — they are
# empirically load-bearing (superpowers Meincke 2025: compliance 33% → 72%
# under structured persuasion scaffolding). A discipline skill missing any of
# the three is a regression.

setup() {
  SKILLS="${BATS_TEST_DIRNAME}/../skills"
}

# ---------- Iron Law presence ----------

@test "deliver-story has Iron Law" {
  grep -qF "## The Iron Law" "$SKILLS/deliver-story/SKILL.md"
  grep -qF "NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST" "$SKILLS/deliver-story/SKILL.md"
}

@test "brownfield-ingest has Iron Law" {
  grep -qF "## The Iron Law" "$SKILLS/brownfield-ingest/SKILL.md"
  grep -qF "NO ROUND COMPLETION WITHOUT HONEST CONVERGENCE CHECK FIRST" "$SKILLS/brownfield-ingest/SKILL.md"
}

@test "adversarial-review has Iron Law" {
  grep -qF "## The Iron Law" "$SKILLS/adversarial-review/SKILL.md"
  grep -qF "NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST" "$SKILLS/adversarial-review/SKILL.md"
}

@test "wave-gate has Iron Law" {
  grep -qF "## The Iron Law" "$SKILLS/wave-gate/SKILL.md"
  grep -qF "NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST" "$SKILLS/wave-gate/SKILL.md"
}

# ---------- Announce at Start ----------

@test "deliver-story has Announce at Start" {
  grep -qF "## Announce at Start" "$SKILLS/deliver-story/SKILL.md"
}

@test "brownfield-ingest has Announce at Start" {
  grep -qF "## Announce at Start" "$SKILLS/brownfield-ingest/SKILL.md"
}

@test "adversarial-review has Announce at Start" {
  grep -qF "## Announce at Start" "$SKILLS/adversarial-review/SKILL.md"
}

@test "wave-gate has Announce at Start" {
  grep -qF "## Announce at Start" "$SKILLS/wave-gate/SKILL.md"
}

# ---------- Red Flags table ----------

@test "deliver-story has Red Flags table" {
  grep -qF "## Red Flags" "$SKILLS/deliver-story/SKILL.md"
  # Expect at least 8 table rows beneath the header
  [ "$(awk '/^## Red Flags/{flag=1;next} /^## /{flag=0} flag && /^\|/' "$SKILLS/deliver-story/SKILL.md" | grep -c '^|')" -ge 8 ]
}

@test "brownfield-ingest has Red Flags table" {
  grep -qF "## Red Flags" "$SKILLS/brownfield-ingest/SKILL.md"
  [ "$(awk '/^## Red Flags/{flag=1;next} /^## /{flag=0} flag && /^\|/' "$SKILLS/brownfield-ingest/SKILL.md" | grep -c '^|')" -ge 8 ]
}

@test "adversarial-review has Red Flags table" {
  grep -qF "## Red Flags" "$SKILLS/adversarial-review/SKILL.md"
  [ "$(awk '/^## Red Flags/{flag=1;next} /^## /{flag=0} flag && /^\|/' "$SKILLS/adversarial-review/SKILL.md" | grep -c '^|')" -ge 8 ]
}

@test "wave-gate has Red Flags table" {
  grep -qF "## Red Flags" "$SKILLS/wave-gate/SKILL.md"
  [ "$(awk '/^## Red Flags/{flag=1;next} /^## /{flag=0} flag && /^\|/' "$SKILLS/wave-gate/SKILL.md" | grep -c '^|')" -ge 8 ]
}

# ---------- brownfield-ingest self-improvement content ----------

@test "brownfield-ingest has Honest Convergence clause" {
  grep -qF "Honest Convergence" "$SKILLS/brownfield-ingest/SKILL.md"
  grep -qF "fewer than 3 substantive" "$SKILLS/brownfield-ingest/SKILL.md"
}

@test "brownfield-ingest has Known Round-1 Hallucination Classes" {
  grep -qF "Known Round-1 Hallucination" "$SKILLS/brownfield-ingest/SKILL.md"
}

@test "brownfield-ingest has Subagent Delivery Protocol" {
  grep -qF "Subagent Delivery Protocol" "$SKILLS/brownfield-ingest/SKILL.md"
  grep -qF "=== FILE:" "$SKILLS/brownfield-ingest/SKILL.md"
}

@test "brownfield-ingest has Behavioral vs Metric split" {
  grep -qF "Behavioral vs Metric split" "$SKILLS/brownfield-ingest/SKILL.md"
}

@test "brownfield-ingest has Priority-ordered Lessons mandate" {
  grep -qF "Priority-ordered Lessons" "$SKILLS/brownfield-ingest/SKILL.md"
  grep -qF "P0" "$SKILLS/brownfield-ingest/SKILL.md"
  grep -qF "P3" "$SKILLS/brownfield-ingest/SKILL.md"
}

# ---------- validate-extraction agent split ----------

@test "validate-extraction has Behavioral vs Metric operating mode" {
  AGENT="${BATS_TEST_DIRNAME}/../agents/validate-extraction.md"
  grep -qF "Behavioral vs Metric" "$AGENT"
  grep -qF "Phase 1" "$AGENT"
  grep -qF "Phase 2" "$AGENT"
}

# ---------- Template path portability ----------
#
# Templates live at ${CLAUDE_PLUGIN_ROOT}/templates/ in the plugin. The
# legacy `.claude/templates/` path only resolves inside corverax (where the
# plugin was originally developed) — a clean install of vsdd-factory would
# not have that directory. Every template reference in skills and agents
# must use ${CLAUDE_PLUGIN_ROOT}/templates/ so the plugin is portable.

@test "no skill references the non-portable .claude/templates/ path" {
  SKILLS_DIR="${BATS_TEST_DIRNAME}/../skills"
  ! grep -rq "\.claude/templates/" "$SKILLS_DIR"
}

@test "no agent references the non-portable .claude/templates/ path" {
  AGENTS_DIR="${BATS_TEST_DIRNAME}/../agents"
  ! grep -rq "\.claude/templates/" "$AGENTS_DIR"
}

@test "every referenced template actually exists in plugin templates/" {
  PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  missing=0
  # Extract every ${CLAUDE_PLUGIN_ROOT}/templates/<file>.md reference
  # from skills and agents, strip the prefix, check the file exists.
  while IFS= read -r ref; do
    [ -z "$ref" ] && continue
    if [ ! -f "${PLUGIN_ROOT}/${ref}" ]; then
      echo "MISSING: ${ref}" >&2
      missing=$((missing + 1))
    fi
  done < <(grep -rho '\${CLAUDE_PLUGIN_ROOT}/templates/[a-zA-Z0-9_/-]*\.md' \
             "${PLUGIN_ROOT}/skills" "${PLUGIN_ROOT}/agents" \
             | sed 's|${CLAUDE_PLUGIN_ROOT}/||' \
             | sort -u)
  [ "$missing" -eq 0 ]
}

# ---------- Adversarial review persistence ----------

@test "adversarial-review has Post-Adversary Persistence section" {
  grep -qF "Post-Adversary Persistence (MANDATORY)" "$SKILLS/adversarial-review/SKILL.md"
}

@test "adversarial-review persistence dispatches state-manager" {
  grep -qF "Dispatch state-manager" "$SKILLS/adversarial-review/SKILL.md"
}

# ---------- Adversarial review collision guard ----------

@test "adversarial-review has collision guard" {
  grep -qF "Collision Guard" "$SKILLS/adversarial-review/SKILL.md"
}

@test "adversarial-review has policy rubric auto-loading" {
  grep -qF "Policy Rubric Auto-Loading (MANDATORY)" "$SKILLS/adversarial-review/SKILL.md"
}

@test "adversarial-review has scoped review parameter" {
  grep -qF -- "--scope=full" "$SKILLS/adversarial-review/SKILL.md"
  grep -qF -- "--scope=diff-from" "$SKILLS/adversarial-review/SKILL.md"
  grep -qF -- "--scope=paths" "$SKILLS/adversarial-review/SKILL.md"
}

# ---------- Policy registry ----------

@test "policy-registry skill exists" {
  [ -f "$SKILLS/policy-registry/SKILL.md" ]
}

@test "policy-add skill exists" {
  [ -f "$SKILLS/policy-add/SKILL.md" ]
}

@test "policies-template.yaml is valid YAML" {
  yq '.' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml" >/dev/null
}

@test "policies-template has 9 baseline policies" {
  count=$(yq '.policies | length' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml")
  [ "$count" -eq 9 ]
}

@test "policies-template policy IDs are sequential 1-9" {
  ids=$(yq '.policies[].id' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml" | sort -n | tr '\n' ',')
  [ "$ids" = "1,2,3,4,5,6,7,8,9," ]
}

@test "policies-template policy names are unique" {
  total=$(yq '.policies[].name' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml" | wc -l | tr -d ' ')
  unique=$(yq '.policies[].name' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml" | sort -u | wc -l | tr -d ' ')
  [ "$total" -eq "$unique" ]
}

@test "policies-template policy 9 references validate-vp-consistency.sh" {
  hook=$(yq '.policies[] | select(.id == 9) | .lint_hook' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml")
  [ "$hook" = "hooks/validate-vp-consistency.sh" ]
}
