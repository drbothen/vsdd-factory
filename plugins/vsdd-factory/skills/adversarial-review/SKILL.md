---
name: adversarial-review
description: Launch a fresh-context adversarial review of specs or implementation. Uses the adversary agent with information asymmetry to find gaps, contradictions, and missing edge cases. Minimum 2 passes to convergence.
argument-hint: "[specs|implementation]"
disable-model-invocation: true
context: fork
agent: adversary
---

# Adversarial Review

Launch the adversary agent to review specs or implementation with fresh context.

## The Iron Law

> **NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST**

Violating the letter of the rule is violating the spirit of the rule. Fresh context means the adversary has not seen prior review passes, the author's explanations, or the orchestrator's summary. Loading any of those contaminates the asymmetry the pattern depends on.

## Announce at Start

Before any other action, say verbatim:

> I'm using the adversarial-review skill to launch a fresh-context adversary pass on <target>.

Then create TodoWrite entries: one per planned pass (minimum 2).

## Red Flags

| Thought | Reality |
|---|---|
| "I already reviewed this, I can skip the adversary pass" | Self-review is not adversarial review. Dispatch. |
| "The spec is obviously correct, one pass is enough" | Minimum is 2. The rule exists because round 1 systematically misses things. |
| "Let me summarize the prior pass for the adversary to save tokens" | That destroys fresh context. Dispatch with only the target artifact. |
| "The adversary found nothing, let's call it done" | Zero findings after a short prompt is a prompt bug, not convergence. Re-dispatch with sharper scope. |
| "This finding isn't really critical, I'll downgrade it" | Severity is the adversary's call, not the orchestrator's. Record as-is. |
| "The same finding keeps appearing, the adversary is stuck" | It keeps appearing because it isn't fixed. Fix it, then re-run. |
| "Novelty is LOW after one pass, we've converged" | Minimum 2 passes. No exceptions. |
| "Let me tell the adversary what the prior reviewer found" | Information asymmetry is the mechanism. Do not leak prior findings. |


## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-template.md` — review document structure
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-index-template.md` — review index
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-finding-template.md` — individual finding format

## Target and Scope

Parse `$ARGUMENTS` to determine review target and scope:

**Target** (first positional argument):
- `specs` — review spec documents (default)
- `implementation` — review source code against specs

**Scope** (optional, after target):
- `--scope=full` — read all documents in the target domain. Use for convergence candidates. This is the default.
- `--scope=diff-from:<commit>` — focus on files changed since `<commit>`. Use between fix bursts to verify the last burst's changes. The adversary receives ONLY the changed files plus their immediate dependencies (parent BC, parent index).
- `--scope=paths:<pattern>` — focus on specific paths (e.g., `specs/architecture/`, `specs/behavioral-contracts/BC-2.05.*`). Use for targeted verification of a specific subsystem or domain.

Examples:
```
/vsdd-factory:adversarial-review specs
/vsdd-factory:adversarial-review specs --scope=full
/vsdd-factory:adversarial-review specs --scope=diff-from:abc1234
/vsdd-factory:adversarial-review specs --scope=paths:specs/architecture/
/vsdd-factory:adversarial-review implementation --scope=paths:src/security/
```

When `--scope` is not `full`, note the scope limitation in the review output header so readers know the review was targeted, not comprehensive.

## Filename Collision Guard (MANDATORY pre-flight)

Before writing any review file, the orchestrator MUST check for filename collisions:

1. **Compute the target path:** `.factory/cycles/<current-cycle>/adversarial-reviews/pass-<N>.md`
2. **Check if the target path exists.** If yes AND the existing file's content differs from what would be written:
   - **REFUSE the write.** Do not overwrite.
   - Emit a clear error: `"Collision: <target-path> already exists with different content. Use a different cycle name or pass number."`
   - Point the caller to the cycle bootstrap skill (`/vsdd-factory:factory-cycles-bootstrap`) to set up a new cycle directory.
3. **Check for legacy flat files.** If `.factory/specs/adversarial-review-pass-*.md` files exist (pre-cycle layout), warn:
   - `"Legacy review files found in .factory/specs/. Consider running /vsdd-factory:factory-cycles-bootstrap to migrate to cycle-keyed layout."`

This prevents silent overwrites of historical reviews in long-lived projects where Phase-1 and Phase-3 reviews coexist. A future enhancement would automate this as a preflight helper script.

## For Spec Review

**Full scope** — read all spec documents:
1. `.factory/specs/product-brief.md`
2. `.factory/specs/domain-spec/L2-INDEX.md` → read index, then all sections (if exists)
3. `.factory/specs/prd.md`
4. `.factory/specs/prd-supplements/*`
5. `.factory/specs/behavioral-contracts/*`
6. `.factory/specs/verification-properties/*`
7. `.factory/specs/architecture/*`

**Diff scope** (`--scope=diff-from:<commit>`) — read only files changed since the specified commit, plus their parent indexes (BC-INDEX, ARCH-INDEX, etc.). This focuses the adversary on verifying the latest fix burst without diluting attention across the full spec corpus.

**Path scope** (`--scope=paths:<pattern>`) — read only files matching the specified glob pattern. Useful for targeted subsystem review.

Attack with the adversary protocol. Write findings to `.factory/cycles/<current>/adversarial-reviews/`.

## Policy Rubric Auto-Loading (MANDATORY)

Before dispatching the adversary, the orchestrator MUST load the project's policy registry and inject it into the adversary's task prompt:

1. **Read `.factory/policies.yaml`** — if the file doesn't exist, skip (policies are also baked into agent prompts as a fallback)
2. **For each policy**, format as a rubric block:
   ```
   POLICY <id> (<name>): <description>
   Severity: <severity>. Scope: <scope>.
   Verification steps:
     1. <step 1>
     2. <step 2>
     ...
   ```
3. **Append all rubric blocks** under a `## Project Policy Rubric` heading in the adversary's task prompt
4. **The adversary must execute the verification steps** for each policy and report compliance per-policy in its findings. Verification steps give the adversary the concrete procedure — not just the rule name, but HOW to check it.

This replaces manual copy-pasting of policy text into every adversary dispatch. The registry is the single source of truth for which policies a project has adopted.

**Why both agent prompts AND policies.yaml?** Agent prompts carry the enforcement logic (what to do when a violation is found — severity classification, fix procedure, escalation rules). The registry carries the catalog (which policies exist, what they cover, what scope they apply to). The adversary needs both: the logic from its prompt to know HOW to check, and the catalog from policies.yaml to know WHAT to check for this specific project.

## For Implementation Review

Read specs first, then review source code against them. Focus on spec drift and silent failures. Scope flags apply here too — `--scope=paths:src/security/` focuses on a specific module.

## Post-Adversary Persistence (MANDATORY)

The adversary agent has only Read/Grep/Glob tools — it cannot write files. After the adversary returns findings as chat text, the orchestrator MUST persist them:

1. **Capture the adversary's full output** verbatim (do not summarize or filter)
2. **Determine the target path:** `.factory/cycles/<current-cycle>/adversarial-reviews/pass-<N>.md`
   - Read `.factory/current-cycle` for the cycle name (e.g., `v1.0.0-greenfield`)
   - If no current-cycle file exists, use the active cycle from STATE.md
   - `<N>` is the pass number (1-based, sequential within the cycle)
3. **Dispatch state-manager** to write the findings file at the target path
4. **Dispatch state-manager** to update the adversarial review index (`ADV-P<N>-INDEX.md`) in the same cycle directory

**Decision rationale:** Option (a) chosen over granting adversary scoped Write access because: (1) preserves fresh-context information asymmetry — adversary never sees its own prior files; (2) state-manager already owns all `.factory/` commits; (3) no harness support for path-scoped tool allowlists.

If the orchestrator skips this step, findings are lost when the conversation context resets. This was observed in practice when direct adversary spawns returned findings as chat text that disappeared on session boundary.

## Pass Management

- Each review is a numbered pass (ADV-P1, ADV-P2, etc.)
- After each pass, assess novelty decay
- When novelty is LOW (findings are refinements, not gaps), report convergence
- Minimum 2 passes, maximum 5 before escalating

## Lessons Learned (apply to ALL projects)

### Fix Root Causes, Not Symptoms

When a finding shows BC-to-story drift (wrong error codes, missing struct fields, wrong formulas), the fix MUST be: read the authoritative BC, then rewrite the contradicting story section from scratch. Never apply targeted text replacements without first reading both the BC and the story section. In practice, incremental line-level patches caused the same findings to recur across 3-5 passes (S-3.01 security limits survived 3 fix attempts; S-3.04 alias system required 6 passes before a full rewrite resolved it in one pass).

### Accumulate Invariants Across Passes

After each adversarial fix cycle, update the adversary prompt with ALL confirmed invariants (struct fields, error codes, version pins, dependency rules, persistence models). The invariant list grows monotonically. Each subsequent pass checks confirmed invariants efficiently and focuses on finding NEW issues.

## No Early Termination

Do NOT shortcut to "it's clean" after 2 consecutive clean passes. Fresh-context review has compounding value — the adversary makes genuinely new findings through pass 9+ in complex projects, including findings every prior pass missed (e.g., phantom crate references that only surface when the adversary reads dependency-graph.md with truly fresh eyes).

Minimum convergence requirement: 3 consecutive clean passes (not 2). Even near-convergence, keep running passes until the minimum is met.

## Trajectory Monotonicity

Finding counts must decrease monotonically across passes. If any pass shows MORE findings than the previous pass, this is a regression — stop and investigate root cause before proceeding. Possible causes:
- New scope was added without pre-validation against the invariant list
- A fix introduced a new defect
- The adversary's perimeter expanded unexpectedly

Do NOT continue convergence passes until the regression is explained and resolved.

### Pre-validate New Scope Additions

When new stories are added during adversarial convergence, they must be written by an agent with access to the full invariant list from prior passes. New stories should be pre-validated against known invariants before being committed. In practice, each new story introduced 3-5 findings because they lacked the rigor of adversarially-converged originals.
