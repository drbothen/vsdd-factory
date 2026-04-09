---
name: track-debt
description: Manage the technical debt register — add, review, prioritize, and resolve tech debt items. Maintains .factory/tech-debt-register.md with severity, effort, and impact tracking.
argument-hint: "[add|list|resolve] [description]"
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash
---

# Track Technical Debt

Manage the technical debt register at `.factory/tech-debt-register.md`.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/tech-debt-register-template.md` — debt register structure

## Commands

Parse `$ARGUMENTS` for the action:

### `add <description>`

Add a new debt item:

1. Read current register (create if doesn't exist)
2. Assign next ID: `TD-NNN`
3. Ask for (or infer):
   - **Severity**: critical | high | medium | low
   - **Category**: design | performance | security | testing | documentation | dependency
   - **Source**: which story, PR, or review introduced it
   - **Impact**: what happens if not addressed
   - **Effort**: S | M | L | XL

4. Append to register and commit to factory-artifacts.

### `list`

Display current debt items in table format:

```
Technical Debt Register:

| ID | Severity | Category | Description | Effort | Source |
|----|----------|----------|-------------|--------|--------|
| TD-001 | high | design | ... | M | STORY-003 |
| TD-002 | medium | testing | ... | S | ADV-P2 |

Total: <N> items (<N> critical, <N> high)
```

### `resolve TD-NNN`

Mark a debt item as resolved:
1. Add resolution date and resolution PR/story
2. Move to "Resolved" section
3. Commit to factory-artifacts

## Register Format

```markdown
# Technical Debt Register

## Active

| ID | Severity | Category | Description | Effort | Source | Created |
|----|----------|----------|-------------|--------|--------|---------|
| TD-001 | high | design | ... | M | STORY-003 | 2026-04-01 |

## Resolved

| ID | Description | Resolved By | Date |
|----|-------------|-------------|------|
| TD-000 | ... | STORY-005 | 2026-04-15 |
```

## When to Add Debt

- Adversarial review finds a real issue but it's deferred
- Code review identifies a shortcut taken for timeline
- `// TODO` or `// HACK` comments in code
- Known performance issues deferred to later wave
- Dependency version pinning that needs updating
- Test coverage gaps accepted for timeline
