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
