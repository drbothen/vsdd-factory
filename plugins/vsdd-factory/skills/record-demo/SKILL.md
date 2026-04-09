---
name: record-demo
description: Record visual demo evidence for story acceptance criteria using Playwright. Captures screenshots or screen recordings of each acceptance criterion being satisfied.
argument-hint: "[STORY-NNN]"
disable-model-invocation: true
allowed-tools: Read, Write, Bash, Glob
---

# Record Demo

Capture visual evidence that a story's acceptance criteria are satisfied.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/demo-evidence-report-template.md` — demo report structure
- `${CLAUDE_PLUGIN_ROOT}/templates/demo-playwright-template.spec.ts` — Playwright test template
- `${CLAUDE_PLUGIN_ROOT}/templates/demo-tape-template.tape` — VHS tape template for terminal recording

## Input

`$ARGUMENTS` — story ID (e.g., `STORY-001`)

## Prerequisites

- Story implementation complete (all tests passing)
- Playwright MCP server available (for web UI) or terminal recording tools for CLI

## Process

### 1. Read Story ACs

Read `.factory/stories/STORY-NNN.md` and extract all acceptance criteria.

### 2. For Each AC

#### CLI Applications

Record terminal session using `script` or `asciinema`:

```bash
# Using script (built-in)
script -q /tmp/demo-ac-1.txt bash -c '<command that demonstrates AC>'

# Using asciinema (if available)
asciinema rec .factory/demo-evidence/STORY-NNN-AC-1.cast -c '<command>'
```

#### Web Applications

Use Playwright MCP tools to:
1. Navigate to the relevant page
2. Perform the action described in the AC
3. Screenshot the result
4. Save to `.factory/demo-evidence/STORY-NNN-AC-<N>.png`

### 3. Create Demo Report

Write to `.factory/demo-evidence/STORY-NNN-demo-report.md`:

```markdown
# Demo Evidence: STORY-NNN — <title>

## Acceptance Criteria Evidence

### AC-1: <criterion text>
- **Method:** <CLI recording / screenshot / screen recording>
- **Evidence:** [link to file]
- **Result:** ✅ Demonstrated | ❌ Could not demonstrate
- **Notes:** <any observations>

### AC-2: ...

## Summary
- Criteria demonstrated: <N>/<total>
- Missing evidence: <list if any>
```

### 4. Commit Evidence

```bash
cd .factory
git add demo-evidence/STORY-NNN-*
git commit -m "factory(phase-3): demo evidence for STORY-NNN"
```

## Tools

| Context | Tool | Install |
|---------|------|---------|
| CLI demos | `script` (built-in) | n/a |
| CLI demos (rich) | `asciinema` | `brew install asciinema` |
| Web screenshots | Playwright MCP | configured in `.mcp.json` |
| Terminal screenshots | `termshot` | `brew install termshot` |

If recording tools aren't available, create a text-based evidence report describing the steps performed and the observed behavior. Never skip demo evidence — document what you can with what you have.
