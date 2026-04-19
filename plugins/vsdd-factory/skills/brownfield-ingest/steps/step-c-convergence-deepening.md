---
name: step-c-convergence-deepening
description: Iteratively deepen each broad sweep pass until novelty decays to NITPICK. Strict binary enforcement — only the literal token NITPICK counts as convergence.
---

# Step B: Convergence Deepening

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, Red Flags, subagent delivery protocol, sandbox considerations, and file naming convention.

After the broad sweep, iteratively deepen each pass until novelty decays to nitpicks. This is where the real understanding happens — broad passes are necessarily shallow on a large codebase.

## Protocol

1. **Read Pass 6 synthesis** — identify all gaps: orphaned modules, under-documented subsystems, missing entity detail, subsystem-level BCs needing function-level depth.

2. **Run deepening rounds on Passes 2 and 3 first** (domain model and behavioral contracts are highest-value). Each round:
   - Reads all prior round outputs for that pass
   - Targets specific gaps from the synthesis and prior round's remaining-gaps list
   - Writes to `<project>-pass-N-deep-<name>-rM.md` (where M is the round number, omitted for round 1)
   - Includes a **Delta Summary** and **Novelty Assessment** (SUBSTANTIVE or NITPICK)

3. **After Passes 2 and 3 converge**, run deepening rounds on Passes 0, 1, 4, and 5. These benefit from the entity and BC knowledge gained during Pass 2/3 convergence — newly-discovered subsystems reveal missed inventory items, architectural implications, NFR patterns, and conventions not captured in the broad sweep.

4. **Each pass converges independently.** A pass is converged when its deepening round reports `Novelty: NITPICK — findings are refinements, not gaps.`

## Convergence Bounds

- Minimum 2 deepening rounds per pass before declaring NITPICK
- **No fixed maximum.** Round budgets are advisory floors, not stop conditions. The protocol stops; the agent never does. If round N is SUBSTANTIVE, round N+1 launches regardless of any "max round" the agent claims. Empirical: Vault Pass 2 needed 62 rounds (R6/R10/R15/R30 each predicted "next is NITPICK" and were wrong — R7 found 55 entities, R31 found 16 from never-cited files, R36 found 69 new aggregates).
- All passes (0-5) must converge — no pass is exempt
- Passes 2 and 3 can run in parallel with each other per round
- Passes 0, 1, 4, and 5 can run in parallel with each other per round
- **One repo per agent, always.** Never combine repos — causes context exhaustion and partial failures.

## Strict Binary Enforcement (CRITICAL)

Only the literal token `NITPICK` in the agent's Novelty Assessment counts as convergence. The orchestrator MUST ignore agent self-declarations like:
- "borderline NITPICK"
- "effectively converged"
- "convergence declared"
- "functionally complete"
- "another round may be needed but probably nitpick"
- "recommend halting"

These phrases mean SUBSTANTIVE for orchestrator purposes. The agent has no authority to declare convergence — only the protocol does. SUBSTANTIVE always triggers another round, no exceptions.

**Predicted-NITPICK is unreliable.** Agents are systematically bad at predicting whether the next round will converge. Never trust a prediction; always run the next round and let it speak for itself.

## Targeted-Flag Carryover

Each round's "next candidate scope" / "remaining gaps" must be passed **verbatim** into the next round's prompt. The orchestrator selects targets from the prior round's flags — the agent must not pick its own targets, which causes topic drift and re-coverage of already-explored areas.

## Cross-Round Contradiction Handling

When round N contradicts rounds 1..N-1 (e.g., Infisical R6 vs R1-R5 on AI/MCP gateway), the next round gets an explicit "resolve this contradiction" mandate, not a generic deepening prompt. Verify against source; the most recent round is not automatically right.

## Negative-Finding Catalogue

Phantom subsystems must be retracted, not silently dropped. Use `CONV-ABS-N` or `BUG-RETRACTED-N` markers (per OpenBao LeaseCountQuota → CONV-ABS-7). Subsequent rounds reference the retraction so the same phantom doesn't reappear.

## Round Prompt Template (required fields)

Every deepening round prompt MUST include ALL of the following. Missing any of these regularly produces wasted rounds:

1. **Pass identifier and round number** — explicit, e.g., "Pass 2 deepening round 17"
2. **Source path** — absolute, `/Users/.../.reference/<repo>/`
3. **Output path** — absolute, full filename, restated even if convention-implied (agents have written to wrong paths when path is implied)
4. **Inputs to read** — broad pass file + all prior round files
5. **Carryover targets** — verbatim "remaining gaps" / "next candidate scope" from prior round
6. **Strict-binary protocol restatement** — "binary novelty: SUBSTANTIVE or NITPICK; agent recommendations of 'converged' without literal NITPICK are ignored"
7. **Write-before-final mandate** — "Write the file before your final message" (some agents end without writing otherwise)
8. **Required output sections** — "End with Delta Summary, Novelty Assessment"

## Honest Convergence (mandatory clause in every round prompt)

Every round prompt MUST include this clause verbatim:

> **Honest convergence is required.** If you find fewer than 3 substantive items, declare convergence and emit no updated file — say "converged, no file emitted." Do not invent findings to justify this round's existence. Fabricating findings is strictly worse than stopping. The orchestrator prefers an honest NITPICK over a padded SUBSTANTIVE. If you are uncertain whether a finding is substantive, default to NITPICK.

## Known Round-1 Hallucination Classes

Round 1 outputs are systematically susceptible to specific failure modes. Every round 2+ prompt should instruct the agent to audit round 1 for these classes before adding new findings:

1. **Over-extrapolated token lists** — round 1 claims a forbidden-token set is `{A, B, C, D, E}` when source only lists `{A, B}`. Example: superpowers round 1 claimed `writing-plans` forbade `XXX`, `???`, ellipsis — actual source forbade only `TBD`, `TODO`, `implement later`, `fill in details`.
2. **Miscounted enumerations** — round 1 claims "6 principles" when actual is 7. Example: superpowers persuasion matrix (Cialdini) round 1 listed 6, missed Reciprocity.
3. **Named pattern conflation / fabrication** — round 1 invents category names not in source. Example: superpowers Pressure Taxonomy round 1 fabricated "urgency / flattery / confusion"; actual was Time / Sunk cost / Authority / Economic / Exhaustion / Social / Pragmatic.
4. **Same-basename artifact conflation** — round 1 merges two files that share a basename but are different artifact kinds. Example: `agents/code-reviewer.md` (48 LOC canonical agent) vs `skills/requesting-code-review/code-reviewer.md` (146 LOC local prompt copy) — these are not the same file.
5. **Inflated or deflated metrics** — round 1 claims a LOC / file count derived from estimation rather than a recounted `find` + `wc -l`. Always re-derive metrics in round 2+ using the shell, not the prior narrative.

Round 2+ prompts should say verbatim: "Before adding new findings, audit round 1 against the 5 Known Hallucination Classes. Retract any finding that fails the audit and mark it as `CONV-ABS-N`."

## Novelty Decay Assessment

Each deepening round MUST assess novelty decay — this is **strict binary**, not a gradient:

| Assessment | Meaning | Action |
|------------|---------|--------|
| **SUBSTANTIVE** | New entities, subsystems, contracts, relationships, or patterns discovered. Findings change the model. | Another round required |
| **NITPICK** | Findings are refinements, edge cases, wording improvements, or confirmations. Nothing changes the model. | Pass has converged |

**The test:** Would removing this round's findings change how you'd spec the system? If yes → SUBSTANTIVE. If no → NITPICK.

## Calibration

A run is calibrated when at least one small (<5K LOC, single-purpose) reference library converges in 2-8 rounds genuinely. If a small library needs 20+ rounds, the protocol is over-fitting and the prompts/targets need review. If a large multi-tenant system converges in <10 rounds, the protocol is under-fitting and strict-binary enforcement is failing somewhere. Use the small-library convergence as the calibration anchor.

## Commit

After each round: `factory(phase-0): brownfield ingest deepening round N`

## Artifacts

Deepening round files in `.factory/semport/<project>/`:
- `<project>-pass-N-deep-<name>.md` (round 1)
- `<project>-pass-N-deep-<name>-rM.md` (round M)

## Success Criteria

- All 6 passes (0-5) have reached NITPICK
- Each pass has minimum 2 deepening rounds
- No agent self-declared convergence accepted without literal NITPICK
- All Round-1 outputs audited against 5 Known Hallucination Classes
- Cross-round contradictions explicitly resolved
- Phantom subsystems retracted with `CONV-ABS-N` markers
