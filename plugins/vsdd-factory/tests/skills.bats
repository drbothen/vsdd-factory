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

@test "policies-template has 10 baseline policies" {
  count=$(yq '.policies | length' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml")
  [ "$count" -eq 10 ]
}

@test "policies-template policy IDs are sequential 1-10" {
  ids=$(yq '.policies[].id' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml" | sort -n | tr '\n' ',')
  [ "$ids" = "1,2,3,4,5,6,7,8,9,10," ]
}

@test "policies-template policy names are unique" {
  total=$(yq '.policies[].name' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml" | wc -l | tr -d ' ')
  unique=$(yq '.policies[].name' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml" | sort -u | wc -l | tr -d ' ')
  [ "$total" -eq "$unique" ]
}

@test "register-artifact skill exists" {
  [ -f "$SKILLS/register-artifact/SKILL.md" ]
}

@test "register-artifact covers BC, VP, story, holdout types" {
  grep -qF "behavioral-contracts/BC-" "$SKILLS/register-artifact/SKILL.md"
  grep -qF "verification-properties/VP-" "$SKILLS/register-artifact/SKILL.md"
  grep -qF "stories/STORY-" "$SKILLS/register-artifact/SKILL.md"
  grep -qF "holdout-scenarios/HS-" "$SKILLS/register-artifact/SKILL.md"
}

@test "register-artifact has duplicate check" {
  grep -qF "Check for duplicates" "$SKILLS/register-artifact/SKILL.md"
}

# ---------- Recover state ----------

@test "recover-state skill exists" {
  [ -f "$SKILLS/recover-state/SKILL.md" ]
}

@test "recover-state has artifact scanning procedure" {
  grep -qF "Scan artifact directories" "$SKILLS/recover-state/SKILL.md"
}

@test "recover-state has phase determination logic" {
  grep -qF "Determine current phase" "$SKILLS/recover-state/SKILL.md"
}

@test "recover-state has backup step" {
  grep -qF "Backup existing STATE.md" "$SKILLS/recover-state/SKILL.md"
}

@test "recover-state has dry-run option" {
  grep -qF -- "--dry-run" "$SKILLS/recover-state/SKILL.md"
}

@test "recover-state has validation step before writing" {
  grep -qF "Does this look correct?" "$SKILLS/recover-state/SKILL.md"
}

# ---------- Template compliance ----------

@test "validate-template-compliance skill exists" {
  [ -f "$SKILLS/validate-template-compliance/SKILL.md" ]
}

@test "validate-template-compliance has three-level check" {
  grep -qF "Frontmatter Compliance" "$SKILLS/validate-template-compliance/SKILL.md"
  grep -qF "Section Compliance" "$SKILLS/validate-template-compliance/SKILL.md"
  grep -qF "Table Column Compliance" "$SKILLS/validate-template-compliance/SKILL.md"
}

@test "validate-template-compliance has file-to-template mapping" {
  grep -qF "behavioral-contracts/BC-*.md" "$SKILLS/validate-template-compliance/SKILL.md"
  grep -qF "stories/STORY-*.md" "$SKILLS/validate-template-compliance/SKILL.md"
  grep -qF "verification-coverage-matrix.md" "$SKILLS/validate-template-compliance/SKILL.md"
}

@test "conform-to-template skill exists" {
  [ -f "$SKILLS/conform-to-template/SKILL.md" ]
}

@test "conform-to-template has safety guarantees" {
  grep -qF "Never deletes content" "$SKILLS/conform-to-template/SKILL.md"
  grep -qF "Always shows changes before applying" "$SKILLS/conform-to-template/SKILL.md"
  grep -qF "Creates backup before modifying" "$SKILLS/conform-to-template/SKILL.md"
}

@test "conform-to-template has planned changes presentation" {
  grep -qF "Planned Changes" "$SKILLS/conform-to-template/SKILL.md"
  grep -qF "Apply these changes?" "$SKILLS/conform-to-template/SKILL.md"
}

@test "policies-template policy 9 references validate-vp-consistency.sh" {
  hook=$(yq '.policies[] | select(.id == 9) | .lint_hook' "${BATS_TEST_DIRNAME}/../templates/policies-template.yaml")
  [ "$hook" = "hooks/validate-vp-consistency.sh" ]
}

# ---------- Lobster skill path resolution ----------

@test "all skill: paths in phase lobster files resolve to existing files" {
  local missing=0
  local plugin_root="${BATS_TEST_DIRNAME}/.."
  while IFS= read -r path; do
    if [ ! -f "$plugin_root/$path" ]; then
      echo "MISSING: $path" >&2
      missing=$((missing + 1))
    fi
  done < <(grep -rh 'skill:' "$plugin_root/workflows/phases/"*.lobster \
    | sed 's/.*skill: *"//' | sed 's/".*//' | sort -u)
  [ "$missing" -eq 0 ]
}

@test "all skill: paths in top-level lobster files resolve to existing files" {
  local missing=0
  local plugin_root="${BATS_TEST_DIRNAME}/.."
  while IFS= read -r path; do
    if [ ! -f "$plugin_root/$path" ]; then
      echo "MISSING: $path" >&2
      missing=$((missing + 1))
    fi
  done < <(grep -rh 'skill:' \
    "$plugin_root/workflows/greenfield.lobster" \
    "$plugin_root/workflows/brownfield.lobster" \
    "$plugin_root/workflows/feature.lobster" \
    "$plugin_root/workflows/multi-repo.lobster" \
    2>/dev/null | sed 's/.*skill: *"//' | sed 's/".*//' | sort -u)
  [ "$missing" -eq 0 ]
}

# ---------- Phase entry-point skills ----------

@test "all 8 phase entry-point skills exist" {
  local plugin_root="${BATS_TEST_DIRNAME}/.."
  [ -f "$plugin_root/skills/phase-0-codebase-ingestion/SKILL.md" ]
  [ -f "$plugin_root/skills/phase-1-spec-crystallization/SKILL.md" ]
  [ -f "$plugin_root/skills/phase-2-story-decomposition/SKILL.md" ]
  [ -f "$plugin_root/skills/phase-3-tdd-implementation/SKILL.md" ]
  [ -f "$plugin_root/skills/phase-4-holdout-evaluation/SKILL.md" ]
  [ -f "$plugin_root/skills/phase-5-adversarial-refinement/SKILL.md" ]
  [ -f "$plugin_root/skills/phase-6-formal-hardening/SKILL.md" ]
  [ -f "$plugin_root/skills/phase-7-convergence/SKILL.md" ]
}

@test "each phase entry-point skill references its sub-workflow" {
  local plugin_root="${BATS_TEST_DIRNAME}/.."
  grep -q "phase-0-codebase-ingestion.lobster" "$plugin_root/skills/phase-0-codebase-ingestion/SKILL.md"
  grep -q "phase-1-spec-crystallization.lobster" "$plugin_root/skills/phase-1-spec-crystallization/SKILL.md"
  grep -q "phase-2-story-decomposition.lobster" "$plugin_root/skills/phase-2-story-decomposition/SKILL.md"
  grep -q "phase-3-tdd-implementation.lobster" "$plugin_root/skills/phase-3-tdd-implementation/SKILL.md"
  grep -q "phase-4-holdout-evaluation.lobster" "$plugin_root/skills/phase-4-holdout-evaluation/SKILL.md"
  grep -q "phase-5-adversarial-refinement.lobster" "$plugin_root/skills/phase-5-adversarial-refinement/SKILL.md"
  grep -q "phase-6-formal-hardening.lobster" "$plugin_root/skills/phase-6-formal-hardening/SKILL.md"
  grep -q "phase-7-convergence.lobster" "$plugin_root/skills/phase-7-convergence/SKILL.md"
}

@test "all 8 phase lobster files exist with matching names" {
  local wf="${BATS_TEST_DIRNAME}/../workflows/phases"
  [ -f "$wf/phase-0-codebase-ingestion.lobster" ]
  [ -f "$wf/phase-1-spec-crystallization.lobster" ]
  [ -f "$wf/phase-2-story-decomposition.lobster" ]
  [ -f "$wf/phase-3-tdd-implementation.lobster" ]
  [ -f "$wf/phase-4-holdout-evaluation.lobster" ]
  [ -f "$wf/phase-5-adversarial-refinement.lobster" ]
  [ -f "$wf/phase-6-formal-hardening.lobster" ]
  [ -f "$wf/phase-7-convergence.lobster" ]
}

@test "no old phase numbering in lobster files" {
  local wf="${BATS_TEST_DIRNAME}/../workflows"
  # No fractional phases
  ! grep -rq 'phase-3\.5\|phase-3-5' "$wf/"
  # No old phase-4-adversarial (should be phase-5)
  ! grep -rq 'phase-4-adversarial' "$wf/"
  # No old phase-5-formal (should be phase-6)
  ! grep -rq 'phase-5-formal' "$wf/"
  # No old phase-6-convergence (should be phase-7)
  ! grep -rq 'phase-6-convergence' "$wf/"
}

# ---------- Step file structure ----------

@test "all phases with step files have _shared-context.md" {
  local plugin_root="${BATS_TEST_DIRNAME}/.."
  [ -f "$plugin_root/skills/brownfield-ingest/steps/_shared-context.md" ]
  [ -f "$plugin_root/skills/decompose-stories/steps/_shared-context.md" ]
  [ -f "$plugin_root/skills/deliver-story/steps/_shared-context.md" ]
  [ -f "$plugin_root/skills/formal-verify/steps/_shared-context.md" ]
  [ -f "$plugin_root/skills/convergence-check/steps/_shared-context.md" ]
  [ -f "$plugin_root/skills/phase-1d-adversarial-spec-review/steps/_shared-context.md" ]
  [ -f "$plugin_root/skills/phase-f5-scoped-adversarial/steps/_shared-context.md" ]
}

@test "observability skills are model-invocable (v0.79.0+)" {
  local plugin_root="${BATS_TEST_DIRNAME}/.."
  # factory-obs + claude-telemetry + onboard-observability should NOT have
  # `disable-model-invocation: true`. The orchestrator needs to discover
  # them automatically when the user says things like "register this
  # project with the observability stack" or "enable claude telemetry".
  for skill in factory-obs claude-telemetry onboard-observability; do
    local f="$plugin_root/skills/$skill/SKILL.md"
    [ -f "$f" ] || { echo "missing: $f" >&2; return 1; }
    if grep -q '^disable-model-invocation: true' "$f"; then
      echo "regression: $skill has disable-model-invocation: true (must be absent or false so the model can auto-invoke it)" >&2
      return 1
    fi
  done
}

@test "onboard-observability skill exists with expected structure (v0.79.0+)" {
  local plugin_root="${BATS_TEST_DIRNAME}/.."
  local f="$plugin_root/skills/onboard-observability/SKILL.md"
  [ -f "$f" ]
  # Must describe both halves of the workflow so the model chooses it
  # over invoking the two sub-skills separately.
  grep -q 'factory-obs register' "$f"
  grep -q 'claude-telemetry\|\.claude/settings\.local\.json' "$f"
  grep -q 'OTEL_EXPORTER_OTLP_ENDPOINT' "$f"
  # Must state idempotency so re-running is safe.
  grep -qi 'idempoten' "$f"
}

@test "phase 0-7 step files use pure alphabetic naming" {
  local plugin_root="${BATS_TEST_DIRNAME}/.."
  local bad=0
  for skill_dir in brownfield-ingest decompose-stories deliver-story formal-verify convergence-check phase-1d-adversarial-spec-review phase-f5-scoped-adversarial; do
    local steps_dir="$plugin_root/skills/$skill_dir/steps"
    [ -d "$steps_dir" ] || continue
    while IFS= read -r f; do
      name=$(basename "$f")
      if [[ "$name" == step-[0-9]* ]] || [[ "$name" == *[0-9][0-9]* ]]; then
        echo "BAD: $f (numeric or sub-step ID)" >&2
        bad=$((bad + 1))
      fi
    done < <(find "$steps_dir" -name 'step-*.md' -not -name '_*')
  done
  [ "$bad" -eq 0 ]
}
