---
name: register-artifact
description: Register a newly created artifact (BC, VP, story, holdout scenario) in its corresponding INDEX file. Extracts metadata from frontmatter and appends an index row.
argument-hint: "<file-path>"
---

# Register Artifact

After creating a new BC, VP, story, or holdout scenario file, register it in the corresponding INDEX file to prevent orphan artifacts.

## Why This Exists

Creating a new artifact is a two-step process: write the file, then update the INDEX. Forgetting step 2 creates orphan artifacts that pass individual file validation but fail consistency-validator criteria (criterion 23: "Index files reference all existing detail files"). This skill automates step 2.

## Procedure

### Step 1: Identify the artifact type

Parse `$ARGUMENTS` for the file path. Determine the artifact type from the path:

| Path Pattern | Type | INDEX File |
|-------------|------|------------|
| `behavioral-contracts/ss-NN/BC-*.md` (canonical, sharded) | Behavioral Contract | `behavioral-contracts/BC-INDEX.md` |
| `behavioral-contracts/BC-*.md` (legacy flat layout) | Behavioral Contract | `behavioral-contracts/BC-INDEX.md` |
| `verification-properties/VP-*.md` | Verification Property | `verification-properties/VP-INDEX.md` |
| `stories/S-*.md` (canonical, S-N.MM format) | Story | `stories/STORY-INDEX.md` |
| `stories/STORY-*.md` (legacy STORY-NNN format) | Story | `stories/STORY-INDEX.md` |
| `holdout-scenarios/HS-*.md` | Holdout Scenario | `holdout-scenarios/HS-INDEX.md` |

If the path doesn't match any pattern, report: "Unrecognized artifact type. This skill registers BCs, VPs, stories, and holdout scenarios."

### Step 2: Read the artifact frontmatter

Extract the fields needed for the INDEX row:

**For BCs:**
- `document_type` — must be `behavioral-contract`
- H1 heading — `# BC-S.SS.NNN: <title>` → extract title
- `subsystem` — from frontmatter
- `priority` — from frontmatter (default P1 if missing)

**For VPs:**
- `document_type` — must be `vp-registry` or similar
- H1 heading — `# VP-NNN: <title>` → extract description
- `module` — from frontmatter
- `tool` — from frontmatter (kani/proptest/fuzz/integration)
- `phase` — from frontmatter (P0/P1)
- `status` — from frontmatter (default `draft`)
- `anchor_story` — from frontmatter

**For Stories:**
- `document_type` — must be `story`
- `story_id` — from frontmatter
- H1 heading — extract title
- `epic_id` — from frontmatter
- `points` — from frontmatter
- `priority` — from frontmatter
- `depends_on` — from frontmatter (join as comma-separated)
- `status` — from frontmatter (default `draft`)

**For Holdout Scenarios:**
- `document_type` — must be `holdout-scenario`
- H1 heading — extract title
- `priority` — from frontmatter (must-pass/should-pass)
- `source_bc` — from frontmatter

### Step 3: Verify INDEX file exists

Check if the INDEX file exists at the expected path. If not:
- Report: "INDEX file not found at <path>. Create it first or run the relevant decomposition skill."
- Do NOT create the INDEX file — that's the responsibility of the artifact-producing agent (product-owner for BC-INDEX, story-writer for STORY-INDEX, etc.)

### Step 4: Check for duplicates

Read the INDEX file and check if the artifact ID already has a row. If yes:
- Report: "<ID> already registered in <INDEX>. Skipping."
- Do NOT add a duplicate row

### Step 5: Append the INDEX row

Add a new row to the INDEX table matching the existing format:

**BC-INDEX row:**
```
| BC-S.SS.NNN | <title> | <subsystem> | <priority> |
```

**VP-INDEX row:**
```
| VP-NNN | <description> | <module> | <tool> | <phase> | <status> | <anchor_story> |
```

**STORY-INDEX row:**
```
| S-N.MM | <title> | <epic_id> | <points> | <priority> | <depends_on> | <status> |
```

**HS-INDEX row:**
```
| HS-NNN | <title> | <priority> | <source_bc> |
```

### Step 6: Report

```
Registered <ID> in <INDEX>:
  Title: <title>
  Type: <artifact-type>
  Index row added at line <N>
```

## Batch Registration

If `$ARGUMENTS` contains multiple file paths (space-separated), process each one in sequence. Report a summary at the end:

```
Registered N artifacts:
  - BC-2.01.005 → BC-INDEX.md
  - BC-2.01.006 → BC-INDEX.md
  - S-3.05 → STORY-INDEX.md
Skipped 1 (already registered):
  - BC-2.01.001
```

## Integration

This skill is called:
1. **By agents** after creating new artifacts — story-writer, product-owner, architect can call it to register their output
2. **By the orchestrator** as a post-creation step in per-story-delivery or spec-crystallization workflows
3. **Manually** by the user via `/vsdd-factory:register-artifact <path>`

## Out of Scope

- Does NOT create the artifact file — that's the producing agent's job
- Does NOT create the INDEX file — that's created during initial decomposition
- Does NOT update story frontmatter or AC traces — that's policy 8 enforcement
- Does NOT validate artifact content — that's consistency-validator's job
