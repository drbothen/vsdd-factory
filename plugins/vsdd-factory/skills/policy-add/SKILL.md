---
name: policy-add
description: Register a new governance policy in .factory/policies.yaml with validated schema. Assigns the next sequential ID and verifies no name collision.
argument-hint: "<policy-name>"
---

# Policy Add

Register a new governance policy in the project's `.factory/policies.yaml` registry.

## Prerequisites

`.factory/policies.yaml` must exist. If it doesn't, run `/vsdd-factory:policy-registry init` first.

## Procedure

### Step 1: Validate the policy name

Parse `$ARGUMENTS` for the policy name.

- Must be `snake_case` (lowercase, underscores, no spaces, no hyphens)
- Must not already exist in `.factory/policies.yaml` (check `name` field)
- If invalid or duplicate, report the error and stop

### Step 2: Gather policy details

Ask the user for each required field (one at a time):

1. **Description** — One-line human-readable description of the policy rule
2. **Severity** — `HIGH` or `MEDIUM`
   - HIGH: violations block convergence
   - MEDIUM: violations are findings but may not block alone
3. **Adopted context** — Where this policy was identified (e.g., "burst-24", "pass-23", "Phase 3 patch cycle")
4. **Enforced by** — Which mechanisms enforce this policy. Present as multiple choice:
   - `adversary-prompt` — adversary checks this in every pass
   - `consistency-validator` — consistency-validator has criteria for this
   - `lint-hook` — automated PostToolUse hook validates this
   - `orchestrator-rule` — orchestrator dispatch ordering enforces this
5. **Scope** — Which artifact types this policy applies to. Present as multiple choice:
   - `bc`, `vp`, `di`, `cap`, `story`, `hs`, `architecture`, `prd`, `nfr`
6. **Lint hook** — Path to automated hook (relative to plugin root), or null if manual-only
7. **Verification steps** — Ordered list of steps the adversary should follow to check this policy. Each step should be specific enough for an agent to execute without ambiguity. Baseline policies (1-9) have steps baked into agent prompts, but custom policies MUST include steps here so the adversary knows HOW to verify them.

### Step 3: Assign ID

Read `.factory/policies.yaml`, find the highest existing `id`, assign `max_id + 1`.

### Step 4: Write the policy entry

Append the new policy to the `policies:` list in `.factory/policies.yaml`:

```yaml
  - id: <next-id>
    name: <policy-name>
    description: "<description>"
    adopted: <adopted-context>
    severity: <severity>
    enforced_by: [<enforcement-list>]
    scope: [<scope-list>]
    lint_hook: <hook-path-or-null>
    verification_steps:
      - "<step 1>"
      - "<step 2>"
```

### Step 5: Validate

Run `/vsdd-factory:policy-registry validate` to confirm the registry is still well-formed after the addition.

### Step 6: Report

```
Policy added:
  ID: <id>
  Name: <policy-name>
  Severity: <severity>
  Enforced by: <enforcement-list>

Next steps:
- If this policy has a lint hook, implement it at <hook-path>
- If this policy should be checked by the adversary, the rubric will auto-load from policies.yaml
- Document the policy rationale in the project's retrospective or cycle manifest
```

## Validation Rules

- Policy `name` must be unique across the registry
- Policy `id` must be unique and sequential (no gaps required, but no duplicates)
- If `lint_hook` is specified, warn if the file doesn't exist yet (allow creation after registration)
- `enforced_by` must have at least one entry
- `scope` must have at least one entry
