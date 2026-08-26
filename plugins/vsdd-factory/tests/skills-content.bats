#!/usr/bin/env bats
# skills-content.bats — content guards against author-environment leaks in
# skill prose.
#
# Issue #229: the STATE.md bootstrap in factory-health/SKILL.md and the
# frontmatter update in state-update/SKILL.md both emitted a hardcoded
# `product: corverax` — corverax is the author's project, not the user's.
# On a fresh install this writes a stranger's product name into the user's
# STATE.md. The canonical templates already use a placeholder
# (templates/state-template.md → `project: "[project-name]"`,
# templates/factory-project-state-template.md → `${project_name}`); the skill
# emissions must too.
#
# These guards catch the whole leak CLASS — any bare literal value on a
# `product:` frontmatter emission line — not just the one reported string.

setup() {
  SKILLS="${BATS_TEST_DIRNAME}/../skills"
}

@test "no skill emits the leaked author product literal 'corverax' (issue #229)" {
  # The exact reported instances: factory-health step 4 + state-update step 2.
  run grep -rn 'product: corverax' "$SKILLS"
  [ "$status" -ne 0 ]
}

@test "every skill product: frontmatter emission uses a placeholder, not a literal" {
  # A `product:` YAML line in skill prose is a STATE.md emission template. Its
  # value must be a placeholder — square-bracket [...] (the canonical
  # templates' idiom) or angle-bracket <...> (this repo's inline-yaml idiom) —
  # never a bare identifier, which is by definition a leaked author name.
  #
  # Match every `product:` emission line (leading whitespace allowed, so prose
  # like `... or per-product:` is excluded), then drop the ones whose value
  # begins with `[` or `<`. Whatever remains is a hardcoded literal.
  local offenders
  offenders="$(grep -rnE '^[[:space:]]*product:[[:space:]]' "$SKILLS" \
                 | grep -vE 'product:[[:space:]]+[<[]' || true)"
  if [ -n "$offenders" ]; then
    echo "hardcoded product literal(s) — must be a [placeholder] or <placeholder>:" >&2
    echo "$offenders" >&2
    return 1
  fi
}
