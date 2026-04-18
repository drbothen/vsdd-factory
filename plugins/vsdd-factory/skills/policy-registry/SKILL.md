---
name: policy-registry
description: View, validate, and manage the project's .factory/policies.yaml governance policy registry. Lists active policies, checks enforcement coverage, and verifies lint hooks exist.
argument-hint: "[list|validate|show <policy-name>|init]"
---

# Policy Registry

Manage the project's declarative governance policy registry at `.factory/policies.yaml`.

## Purpose

Governance policies (e.g., `append_only_numbering`, `vp_index_is_vp_catalog_source_of_truth`) are currently encoded in agent prompt text across multiple files. This skill provides a single declarative registry that:

1. **Adversarial-review auto-loads** — the orchestrator reads `policies.yaml` and injects all active policies into every adversary dispatch, replacing manual copy-pasting
2. **Lint hooks reference** — each policy declares its `lint_hook` path, enabling a master validation runner
3. **Policy-add command** — users register new policies mid-cycle with validated schema
4. **Cross-project portability** — new projects start with the plugin's 9 baseline policies

## Commands

Parse `$ARGUMENTS`:

- `list` (default) — Display all policies with status, severity, and enforcement summary
- `validate` — Verify policy registry integrity
- `show <policy-name>` — Display full details for a single policy
- `init` — Initialize `.factory/policies.yaml` from the plugin's baseline template

## Procedure

### Init

1. Check if `.factory/policies.yaml` already exists
   - If yes, ask user: "Policy registry already exists with N policies. Overwrite or merge?"
   - If overwrite: replace with baseline template
   - If merge: add any baseline policies not already present (match by `name`)
2. If no: copy `${CLAUDE_PLUGIN_ROOT}/templates/policies-template.yaml` to `.factory/policies.yaml`
3. Populate with the 9 baseline governance policies from the plugin (see Baseline Policies below)
4. Report: "Policy registry initialized with N policies at .factory/policies.yaml"

### List

1. Read `.factory/policies.yaml`
2. If the file doesn't exist, report: "No policy registry found. Run `/vsdd-factory:policy-registry init` to create one."
3. Display a summary table:

```
| # | Policy | Severity | Enforced By | Lint Hook |
|---|--------|----------|-------------|-----------|
| 1 | append_only_numbering | HIGH | adversary, consistency-validator | — |
| 9 | vp_index_is_vp_catalog_source_of_truth | HIGH | adversary, lint-hook, consistency-validator | validate-vp-consistency.sh |
```

### Validate

1. Read `.factory/policies.yaml`
2. Parse YAML — if invalid syntax, report parse error and stop
3. For each policy, verify:
   - **ID uniqueness:** No duplicate `id` values across all policies
   - **Name uniqueness:** No duplicate `name` values
   - **Name format:** Must be `snake_case` (lowercase, underscores, no spaces)
   - **Required fields:** `id`, `name`, `description`, `severity`, `enforced_by` must all be non-empty
   - **Severity values:** Must be `HIGH` or `MEDIUM`
   - **Lint hook exists:** If `lint_hook` is non-null, verify the file exists at `${CLAUDE_PLUGIN_ROOT}/<lint_hook>` and is executable
   - **Scope values:** Each entry in `scope` must be a recognized artifact type (bc, vp, di, cap, story, hs, architecture, prd, nfr)
4. Report findings as PASS/FAIL per policy with specific remediation for failures

### Show

1. Read `.factory/policies.yaml`
2. Find the policy matching `<policy-name>` (match on `name` field)
3. If not found, list available policy names
4. Display all fields in a readable format including description, severity, enforcement list, scope, and lint hook status

## Baseline Policies

These 9 policies are built into the plugin and should be present in every project's registry:

| # | Name | Severity |
|---|------|----------|
| 1 | `append_only_numbering` | HIGH |
| 2 | `lift_invariants_to_bcs` | MEDIUM |
| 3 | `state_manager_runs_last` | HIGH |
| 4 | `semantic_anchoring_integrity` | MEDIUM |
| 5 | `creators_justify_anchors` | MEDIUM |
| 6 | `architecture_is_subsystem_name_source_of_truth` | HIGH |
| 7 | `bc_h1_is_title_source_of_truth` | HIGH |
| 8 | `bc_array_changes_propagate_to_body_and_acs` | HIGH |
| 9 | `vp_index_is_vp_catalog_source_of_truth` | HIGH |

## Integration with Adversarial Review

The adversarial-review skill reads `.factory/policies.yaml` at dispatch time and injects all active policies into the adversary's rubric. This replaces manual policy copy-pasting into every dispatch prompt.

**Auto-loading procedure** (executed by the orchestrator when dispatching adversary):

1. Read `.factory/policies.yaml` — if missing, warn but continue (policies are also in agent prompts)
2. For each policy, format as a rubric item:
   ```
   POLICY <id> (<name>): <description>
   Severity: <severity>. Scope: <scope>.
   ```
3. Append the formatted rubric to the adversary's task prompt under a "## Project Policy Rubric" heading
4. The adversary must verify each policy as a review axis and report compliance per-policy

**Why both agent prompts AND policies.yaml?** Agent prompts carry the enforcement logic (what to do when a violation is found). The registry carries the catalog (which policies exist, what they cover). The adversary needs both: the logic to know HOW to check, and the catalog to know WHAT to check.

## Template

`${CLAUDE_PLUGIN_ROOT}/templates/policies-template.yaml`
