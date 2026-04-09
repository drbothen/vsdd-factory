---
name: research
description: Conduct external research — domain analysis or technology evaluation. Spawns the research-agent with MCP tool access (Perplexity, Context7, Tavily). Usage - /research domain <topic> or /research <topic> for general.
argument-hint: "[domain|general] <topic>"
disable-model-invocation: true
context: fork
agent: research-agent
---

# Research

Spawn the research-agent to conduct external research. Supports multiple runs — each creates a new dated file.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/domain-research-template.md` — research report structure

## Input

`$ARGUMENTS` — research type (optional) and topic.

### Domain Research (problem space)

```
/research domain competitive landscape for CLI AI orchestration tools
/research domain user needs for multi-agent workflow automation
/research domain compliance requirements for AI code generation
```

Produces: `.factory/specs/research/domain-<slug>-<YYYY-MM-DD>.md`

### General Research (technology/implementation)

```
/research Rust workflow engine comparison — xstate-rs vs saga-rs vs custom
/research general security advisories for tokio 1.x
/research best practices for CLI multi-agent orchestration
```

If the first word is not `domain`, it's treated as general research.

Produces: `.factory/specs/research/general-<slug>-<YYYY-MM-DD>.md`

## Before Running

1. Ensure `.factory/specs/research/` directory exists. Create it if not:
   ```bash
   mkdir -p .factory/specs/research
   ```

2. Read `.factory/specs/research/RESEARCH-INDEX.md` (if exists) to check for prior research on this topic. Share relevant prior research with the agent for context.

## After Completion

1. Verify the report includes a `## Research Methods` section listing MCP tools used.
2. Update the research index:
   ```bash
   # Append to RESEARCH-INDEX.md (create if doesn't exist)
   ```
3. Commit to factory-artifacts:
   ```bash
   cd .factory && git add specs/research/ && git commit -m "factory(research): <type> — <topic summary>"
   ```
