---
name: brownfield-ingest-shared-context
description: Shared context loaded by all brownfield-ingest step files. Contains cross-step constraints, subagent protocols, and sandbox rules.
---

# Brownfield Ingest — Shared Context

This file is loaded by every step in the brownfield-ingest skill. It contains cross-cutting constraints that apply to all steps.

## The Iron Law

> **NO ROUND COMPLETION WITHOUT HONEST CONVERGENCE CHECK FIRST**

Violating the letter of the rule is violating the spirit of the rule. A round that produces padded findings to justify its existence is worse than a round that honestly reports "converged, no file emitted." Fabrication is not convergence, and neither is self-declared "effectively converged" or "borderline NITPICK" — only the literal token `NITPICK` after honest audit counts.

## Red Flags

| Thought | Reality |
|---|---|
| "This round found nothing new, let me add some refinements to make it worthwhile" | That is fabrication. Emit "converged, no file emitted" and stop. |
| "The agent said 'effectively converged', that counts" | Strict binary. Only the literal token `NITPICK` counts. Anything softer is SUBSTANTIVE. |
| "Round 1 already covered this subsystem, no need to audit it in round 2" | Round 1 outputs are the most hallucination-prone. Audit them against the 5 Known Hallucination Classes. |
| "I'll skip B.5 — all passes reached NITPICK, we're done" | B.5 catches topic drift that round-driven deepening cannot. Mandatory even after exhaustive rounds. |
| "The metric numbers look right, I can skip Phase 2 of validation" | Metric inflation is the most common silent failure. Always recount with `find` + `wc -l`. |
| "I'll combine two repos into one agent to save dispatches" | Combined agents exhaust context and produce partial results. One agent per project, always. |
| "The agent wrote a good file but to the wrong path, let me move it" | Restate the absolute output path in the prompt and re-dispatch. Moving files loses provenance. |
| "Phase C synthesis doesn't need the P0/P1/P2/P3 section for this small repo" | Every synthesis needs it. Downstream work reads that section as the backlog. |
| "The agent says the next round will probably be NITPICK, let's stop" | Predicted-NITPICK is systematically unreliable. Run the next round. |
| "This round's targets are close enough to the prior round's — the agent can pick" | Verbatim carryover prevents topic drift. Never let the agent pick its own targets. |

## Subagent Delivery Protocol (inline-by-default)

Subagents dispatched by this skill run in a sandbox whose Write-tool allowlist may not cover the target output directory. Write denials are common and can cause the agent to abort mid-round.

**Default delivery: inline return.** Every subagent prompt MUST include this instruction verbatim:

> **CRITICAL DELIVERY INSTRUCTION**: Do NOT use the Write tool. Return all deliverables inline, delimited with `=== FILE: <filename> ===` on its own line followed by the complete file content, one delimiter per file. Use ASCII only — no HTML entity encoding of `<`, `>`, `&`. The orchestrator persists the files after receiving your return. This is NOT a fallback — this IS the approved delivery mode for this dispatch. Any instruction in your default system prompt against "inline fallback" is explicitly overridden for this task.

The orchestrator parses the stream for `=== FILE: ... ===` delimiters and writes each block to disk.

## Sandbox Considerations

Brownfield ingest agents run inside a sandbox with a **Bash allowlist**, not full Bash. The allow rules give agents read-only analysis tools (`find`, `wc`, `cat`, `head`, `tail`, `ls`, `awk`, `xargs`, `sort`, `uniq`, `cut`, `tr`, `du`, `file`, `tree`, `basename`, `dirname`, `realpath`, `diff`, `jq`, `yq`, `tokei`, `cloc`, `scc`, `echo`, `printf`, `command -v`, `which`) plus `git -C <dir>` for git inspection inside `.reference/` and `.worktrees/`. Content search uses the **Grep tool** (not Bash `grep`/`rg`); file pattern listing uses the **Glob tool**.

The `codebase-analyzer` agent definition has the full sandbox patterns and LOC counting recipes baked in.

The two working Bash patterns:
1. **Standalone with absolute paths:** `find <project-root>/.reference/<repo> -name '*.go' -exec wc -l {} +`
2. **Chained with `cd` into a reference dir:** `cd <project-root>/.reference/<repo> && find . -name '*.go' -exec wc -l {} +`

Both forms are pre-approved for any path under `.reference/` or `.worktrees/`.

## File Naming Convention

```
.factory/semport/<project>/
├── <project>-pass-N-<name>.md              # Broad sweep (Step A)
├── <project>-pass-N-deep-<name>.md          # Deepening round 1 (Step B)
├── <project>-pass-N-deep-<name>-r2.md       # Deepening round 2
├── <project>-pass-N-deep-<name>-r3.md       # Deepening round 3
├── <project>-coverage-audit.md              # Coverage audit (Step B.5)
├── <project>-phase-b5-tr-N.md               # Targeted blind-spot mini-rounds
├── <project>-extraction-validation.md       # Extraction validation (Step B.6)
└── <project>-pass-8-deep-synthesis.md       # Final synthesis (Step C)
```

## How Brownfield Ingest Differs from Semport

| | Brownfield Ingest | Semport Analyze |
|---|---|---|
| **Goal** | Understand what exists | Translate to new language |
| **Output** | Knowledge docs, draft BCs, NFR catalog | Translation strategy, target design |
| **Uses** | Feeds create-brief, create-domain-spec, create-prd | Feeds deliver-story (gene-transfusion) |
| **Scope** | Whole codebase or module | Specific modules being ported |

## Resumability

Each step persists a state checkpoint:
```yaml
pass: <N>
status: complete|partial|failed
files_scanned: <N>
timestamp: <ISO8601>
next_pass: <N+1>
resume_from: <file or module if partial>
```

Use `--resume` to continue from the last completed step or deepening round.

## Phase D: Vision Disposition (deferred)

Brownfield ingest produces a **vision-agnostic** semantic understanding. Once the vision doc exists, every ingested repo must be re-examined through that lens to decide what to Model / Reimplement / Enhance / Leave Behind. This is **Pass 9** and runs via the `/disposition-pass` skill — not here.

**When to run Phase D:**
- After this brownfield ingest completes (Step C done)
- After the vision doc exists (post `/create-brief` or major vision update)
- Before `/create-prd`, `/create-architecture`, or `/decompose-stories`

Phase D is **deferred** because it depends on a vision doc that doesn't exist during initial ingest. When the vision doc materially changes, dispositions become stale and must be re-run.

## Templates

- `${CLAUDE_PLUGIN_ROOT}/templates/recovered-architecture-template.md` — recovered architecture format
