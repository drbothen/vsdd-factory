# PR Review — #736 fix(skills): STATE.md bootstrap derives product name — remove leaked author literal

**Verdict:** APPROVE
**Reviewer:** pr-reviewer (fresh-eyes, final pre-merge)
**Base:** develop | **Class:** fix-PR (backlog remediation) | **Closes:** #229

## What I verified

1. **Removal of the leaked literal — CONFIRMED.** Both `product: corverax` emissions are gone. `skills/factory-health/SKILL.md` step 4 now emits `product: <repo name from git, or human-supplied>`; `skills/state-update/SKILL.md` step 2 now emits `product: <existing product name — preserved unchanged>`. The reported defect is fully removed.
2. **Derive-or-ask (factory-health) — sound**, with one CWD caveat (see MEDIUM).
3. **Preserve-existing (state-update) — CONFIRMED correct.** state-update reads current state in step 1, so preserving the existing `product` rather than re-deriving is the right semantics; derive-or-ask fallback for the absent case is a reasonable safety net.
4. **Bats guard — well-constructed.** Test 1 pins the specific reported string. Test 2 is a class guard: `^[[:space:]]*product:` anchors to emission lines (correctly excluding prose like `per-product:`), and the `-vE 'product:[[:space:]]+[<[]'` exclusion accepts only `[...]`/`<...>` placeholders — a quoted or bare literal (`product: "corverax"`) would still be flagged. The test file lives in `tests/`, not `skills/`, so its own comment lines mentioning the literal do not trip the `$SKILLS`-scoped grep. Coverage matches the RED/GREEN evidence.

## Findings

### [MEDIUM] correctness — factory-health derive command is CWD-dependent
`git rev-parse --show-toplevel | xargs basename` returns the top level of whatever working tree the agent's shell is in. `.factory/` is a linked worktree on the `factory-artifacts` orphan branch, and the health-check snippet just above the changed block runs `cd .factory`. If shell state carries over (or the agent otherwise runs the derive from inside `.factory/`), `--show-toplevel` yields the `.factory` worktree path and `product` becomes `.factory` — wrong, just differently than `corverax`. The "ask the human if ambiguous / generic checkout dir" fallback partially mitigates this (an agent may treat `.factory` as generic), which is why this is not blocking.
**Suggestion:** anchor the derive to the main working tree — run from repo root, or note that `.factory` / `.worktrees/*` basenames must trigger the ask-human path.

### [INFO] scope — no-suffix-stripping is deliberate but has a blue/green edge
Using the on-disk name verbatim means a checkout in `myproduct-blue/`/`myproduct-green/` stamps the deployment color into `product`. Documented as an intentional call per the issue thread — recorded decision, not a defect. Noted for visibility.

### [INFO] coverage — class guard covers `skills/` only, not `templates/`
Test 2 scans `$SKILLS` exclusively. The PR states the two canonical templates already use placeholders, so no current leak; extending the same guard to `templates/` would close the loop against future template drift. Not required for this fix.

## Summary
The PR does exactly what it claims: removes the hardcoded `product: corverax` author literal from both STATE.md-emitting skills, applies the correct per-skill semantics (derive-or-ask on bootstrap; preserve-existing on update), and adds a well-designed class-level bats guard that catches any bare `product:` literal — matching the RED/GREEN evidence. The one substantive concern is that the factory-health derive command is sensitive to the shell's working directory and could resolve to the `.factory` worktree name; because the human-fallback path softens this and the primary leak is fully eliminated, I approve with that noted as a recommended hardening follow-up.
