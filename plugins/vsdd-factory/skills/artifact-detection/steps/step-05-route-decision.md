# Step 5: Route Decision

> **Parent skill:** `artifact-detection` — see `../SKILL.md` for the full workflow.
> **Agent:** orchestrator
> **This step:** present findings to your human partner and capture which pipeline entry point to use next.

## Inputs

- Inventory, classification, and gap analysis from steps 1–4
- Blocker and warning counts from step 4

## Outputs

- `.factory/planning/routing-decision.md` — the decision and rationale

## Procedure

1. Choose the routing script based on classification.

### L0 (nothing exists)

> "No planning artifacts found. Would you like to:
> 1. Start with brainstorming (Collaborative Discovery — recommended if you're exploring ideas)
> 2. Start with a product brief (Collaborative Discovery — recommended if you know what you want)
> 3. Provide an existing document for me to analyze"

### L1–L3 with no blockers

> "I found [artifact list]. The artifacts are valid, with [N] minor warnings. Ready to proceed to [next phase from classification]?"

### L1–L3 with blockers

> "I found [artifact list]. There are [N] blockers and [N] warnings:
>
> Blockers:
> - <blocker 1>
> - <blocker 2>
>
> Would you like to:
> 1. Fix the blockers interactively (I'll guide you through each)
> 2. Start fresh (discard existing artifacts and go through full discovery)
> 3. Override and proceed anyway (not recommended — pipeline may fail)"

### L4 (full spec)

> "All planning artifacts are present and validated. Ready to proceed to implementation."

If warnings exist, list them after the readiness statement.

2. **Wait for the human's choice.** Do not pick for them. The route is theirs to confirm.

3. **Capture the decision** in `.factory/planning/routing-decision.md`:

```markdown
# Routing Decision

**Generated:** <YYYY-MM-DD HH:MM>
**Readiness Level:** L<N>
**Blockers:** <N>
**Warnings:** <N>

## Inventory Summary

- <artifact>: <verdict>
- <artifact>: <verdict>

## Decision

**Selected route:** <one of:>
- brainstorming
- guided-brief-creation
- spec-crystallization (resume from <stage>)
- fix-blockers (interactive)
- start-fresh
- proceed-to-implementation

**Rationale:** <verbatim from human, or "blockers absent — proceeding per default route">

## Next Skill

<skill name to invoke next, e.g., `brainstorming`, `guided-brief-creation`, `create-prd`, etc.>

## Notes

- <Any conditions, deferrals, or things flagged for the next skill to be aware of>
```

4. **Migration items always run first.** If the gap analysis flagged any legacy format, the routing decision must invoke the migration skill before any other progress. Do not let the human override this — it is a hard prerequisite.

5. Confirm the routing decision back to the human in one sentence:
   > "Routing to [skill]. I'll [first action]. Anything I should know before I start?"

## Decision points

- **Blockers present + human chooses "proceed anyway"** → record the override in routing-decision.md with a warning. Future skills will see it.
- **L4 reached but stories are stale** → recommend re-validating stories against the current PRD before implementation. Default to story re-validation, not direct implementation.
- **Multiple briefs found** (flagged in step 1) → ask the human which is canonical before routing.

## Failure modes

- **Picking the route for the human.** Present options, capture choice. Do not assume.
- **Ignoring migration blockers.** They run first. Always.
- **Routing to a skill that does not exist.** Cross-reference the chosen route against the available skill list.

## Quality gate

- [ ] `routing-decision.md` exists and is complete
- [ ] Selected route is a real, available skill
- [ ] Rationale captured (from human or auto-routed)
- [ ] Migration items scheduled before any forward progress
- [ ] Human confirmed the next skill out loud

## Hand-off to next step

End of `artifact-detection` workflow. Orchestrator invokes the skill named in `routing-decision.md § Next Skill`.
