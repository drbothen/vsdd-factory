# Early-Phase Gaps Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Port the visual companion from superpowers, add pre-adversarial self-review to spec creation skills, add anti-pattern/Red Flags to brainstorming, add hard gates to early-phase skills, and add tiered visual tooling references.

**Architecture:** All changes are additions to existing skill markdown files plus a new standalone skill with ported Node.js scripts. No workflow or pipeline changes.

**Tech Stack:** Markdown skill definitions, Node.js (visual companion server), Bash (start/stop scripts), HTML/CSS/JS (frame template and client helper)

---

### Task 1: Port visual companion scripts

**Files:**
- Create: `plugins/vsdd-factory/skills/visual-companion/scripts/server.cjs`
- Create: `plugins/vsdd-factory/skills/visual-companion/scripts/helper.js`
- Create: `plugins/vsdd-factory/skills/visual-companion/scripts/frame-template.html`
- Create: `plugins/vsdd-factory/skills/visual-companion/scripts/start-server.sh`
- Create: `plugins/vsdd-factory/skills/visual-companion/scripts/stop-server.sh`

- [ ] **Step 1: Create the directory structure**

```bash
mkdir -p plugins/vsdd-factory/skills/visual-companion/scripts
```

- [ ] **Step 2: Copy and adapt server.cjs**

Copy from `/Users/jmagady/.claude/plugins/cache/claude-plugins-official/superpowers/5.0.7/skills/brainstorming/scripts/server.cjs` to `plugins/vsdd-factory/skills/visual-companion/scripts/server.cjs`.

Make one change — line 79, change the default session directory:

```javascript
// BEFORE:
const SESSION_DIR = process.env.BRAINSTORM_DIR || '/tmp/brainstorm';
// AFTER:
const SESSION_DIR = process.env.VISUAL_COMPANION_DIR || '/tmp/visual-companion';
```

Also update the environment variable names throughout the file:
- `BRAINSTORM_PORT` → `VISUAL_COMPANION_PORT`
- `BRAINSTORM_HOST` → `VISUAL_COMPANION_HOST`
- `BRAINSTORM_URL_HOST` → `VISUAL_COMPANION_URL_HOST`
- `BRAINSTORM_DIR` → `VISUAL_COMPANION_DIR`
- `BRAINSTORM_OWNER_PID` → `VISUAL_COMPANION_OWNER_PID`

- [ ] **Step 3: Copy helper.js unchanged**

Copy from `/Users/jmagady/.claude/plugins/cache/claude-plugins-official/superpowers/5.0.7/skills/brainstorming/scripts/helper.js` to `plugins/vsdd-factory/skills/visual-companion/scripts/helper.js`.

No changes needed — the client-side code is generic.

- [ ] **Step 4: Copy and adapt frame-template.html**

Copy from `/Users/jmagady/.claude/plugins/cache/claude-plugins-official/superpowers/5.0.7/skills/brainstorming/scripts/frame-template.html` to `plugins/vsdd-factory/skills/visual-companion/scripts/frame-template.html`.

Make two changes:

1. Line 17 — change the title:
```html
<!-- BEFORE: -->
<title>Superpowers Brainstorming</title>
<!-- AFTER: -->
<title>VSDD Visual Companion</title>
```

2. Line 199 — change the header:
```html
<!-- BEFORE: -->
<h1><a href="https://github.com/obra/superpowers" style="color: inherit; text-decoration: none;">Superpowers Brainstorming</a></h1>
<!-- AFTER: -->
<h1>VSDD Visual Companion</h1>
```

- [ ] **Step 5: Copy and adapt start-server.sh**

Copy from `/Users/jmagady/.claude/plugins/cache/claude-plugins-official/superpowers/5.0.7/skills/brainstorming/scripts/start-server.sh` to `plugins/vsdd-factory/skills/visual-companion/scripts/start-server.sh`.

Make these changes:

1. Update the session directory path (line 81):
```bash
# BEFORE:
SESSION_DIR="${PROJECT_DIR}/.superpowers/brainstorm/${SESSION_ID}"
# AFTER:
SESSION_DIR="${PROJECT_DIR}/.factory/visual-companion/${SESSION_ID}"
```

2. Update the temp fallback (line 83):
```bash
# BEFORE:
SESSION_DIR="/tmp/brainstorm-${SESSION_ID}"
# AFTER:
SESSION_DIR="/tmp/visual-companion-${SESSION_ID}"
```

3. Update environment variable names in the nohup line (line 119) and foreground line (line 113):
- `BRAINSTORM_DIR` → `VISUAL_COMPANION_DIR`
- `BRAINSTORM_HOST` → `VISUAL_COMPANION_HOST`
- `BRAINSTORM_URL_HOST` → `VISUAL_COMPANION_URL_HOST`
- `BRAINSTORM_OWNER_PID` → `VISUAL_COMPANION_OWNER_PID`

4. Make executable: `chmod +x plugins/vsdd-factory/skills/visual-companion/scripts/start-server.sh`

- [ ] **Step 6: Copy stop-server.sh unchanged**

Copy from `/Users/jmagady/.claude/plugins/cache/claude-plugins-official/superpowers/5.0.7/skills/brainstorming/scripts/stop-server.sh` to `plugins/vsdd-factory/skills/visual-companion/scripts/stop-server.sh`.

No changes needed — it's generic (takes session dir as argument).

Make executable: `chmod +x plugins/vsdd-factory/skills/visual-companion/scripts/stop-server.sh`

- [ ] **Step 7: Commit**

```bash
git add plugins/vsdd-factory/skills/visual-companion/scripts/
git commit -m "feat(skills): port visual companion scripts from superpowers"
```

---

### Task 2: Create visual companion SKILL.md and visual-guide.md

**Files:**
- Create: `plugins/vsdd-factory/skills/visual-companion/SKILL.md`
- Create: `plugins/vsdd-factory/skills/visual-companion/visual-guide.md`

- [ ] **Step 1: Write SKILL.md**

Create `plugins/vsdd-factory/skills/visual-companion/SKILL.md`:

```markdown
---
name: visual-companion
description: >
  Browser-based visual companion for showing mockups, diagrams, and interactive
  options during brainstorming, brief creation, and architecture design. Runs a
  local Node.js server that watches for HTML files and serves them with live reload.
  Optional — requires Node.js.
---

# Visual Companion

Browser-based tool for showing visual content during early pipeline stages. The server watches a directory for HTML files, serves the newest one, and relays user interactions (clicks, selections) back as JSON events.

## Prerequisites

- Node.js must be installed (any version with ES2020 support)
- User must consent to opening a local URL

## Starting a Session

Run the start script from the skill's scripts directory:

` `` bash
${CLAUDE_PLUGIN_ROOT}/skills/visual-companion/scripts/start-server.sh --project-dir <project-path>
` ``

Returns JSON:
` ``json
{"type":"server-started","port":52341,"url":"http://localhost:52341",
 "screen_dir":"<project>/.factory/visual-companion/<session>/content",
 "state_dir":"<project>/.factory/visual-companion/<session>/state"}
` ``

Save `screen_dir` and `state_dir`. Tell the user to open the URL.

## The Loop

1. **Check server is alive** — verify `$STATE_DIR/server-info` exists
2. **Write HTML** to a new file in `screen_dir` using the Write tool (never cat/heredoc)
   - Use semantic filenames: `layout.html`, `architecture.html`
   - Never reuse filenames — each screen gets a fresh file
   - Write content fragments (no `<!DOCTYPE>` needed) — server wraps in frame template
3. **Tell user** what's on screen, remind them of the URL, end your turn
4. **On next turn** — read `$STATE_DIR/events` for browser interactions, merge with terminal text
5. **Iterate or advance** — new file for revisions, next question when validated
6. **Unload** when returning to terminal — push a waiting screen to clear stale content

## Stopping

` ``bash
${CLAUDE_PLUGIN_ROOT}/skills/visual-companion/scripts/stop-server.sh <session_dir>
` ``

The server also auto-exits after 30 minutes of inactivity.

## When to Use vs Not

**Use the browser** for content that IS visual: mockups, wireframes, layout comparisons, architecture diagrams, side-by-side designs.

**Use the terminal** for content that is text: requirements, conceptual choices, tradeoff lists, scope decisions.

A question about a UI topic is not automatically a visual question. "What does this feature mean?" is conceptual — use the terminal. "Which of these layouts works better?" is visual — use the browser.

## See Also

- `visual-guide.md` — CSS classes, event format, design tips
- `/vsdd-factory:excalidraw-export` — for static architecture diagrams
```

- [ ] **Step 2: Write visual-guide.md**

Create `plugins/vsdd-factory/skills/visual-companion/visual-guide.md` adapted from the superpowers visual companion guide. This is a reference document for agents using the companion.

The content should cover:
- Content fragments vs full documents (write fragments by default)
- CSS classes available: `.options`, `.option`, `.cards`, `.card`, `.mockup`, `.split`, `.pros-cons`, `.placeholder`, mock elements (`.mock-nav`, `.mock-sidebar`, `.mock-content`, `.mock-button`, `.mock-input`)
- Multi-select via `data-multiselect` attribute
- Typography: `h2` (title), `h3` (section), `.subtitle`, `.section`, `.label`
- Browser events format (JSON lines in `$STATE_DIR/events`)
- Design tips (scale fidelity, explain questions, iterate before advancing, 2-4 options max)
- File naming conventions
- Platform-specific launch notes (macOS, Windows, Codex, Gemini CLI)

Adapt from `/Users/jmagady/.claude/plugins/cache/claude-plugins-official/superpowers/5.0.7/skills/brainstorming/visual-companion.md`, changing:
- All references from "superpowers" to "vsdd-factory"
- Session paths from `.superpowers/brainstorm/` to `.factory/visual-companion/`
- Script paths from `scripts/` to `${CLAUDE_PLUGIN_ROOT}/skills/visual-companion/scripts/`

- [ ] **Step 3: Commit**

```bash
git add plugins/vsdd-factory/skills/visual-companion/SKILL.md plugins/vsdd-factory/skills/visual-companion/visual-guide.md
git commit -m "feat(skills): add visual-companion skill definition and guide"
```

---

### Task 3: Add hard gate + anti-pattern + Red Flags + visual tooling to brainstorming skill

**Files:**
- Modify: `plugins/vsdd-factory/skills/brainstorming/SKILL.md`

- [ ] **Step 1: Read the current file**

Read `plugins/vsdd-factory/skills/brainstorming/SKILL.md`.

- [ ] **Step 2: Add Hard Gate after the frontmatter description block (after line 8, before "# Brainstorming")**

Insert after the closing `---` of frontmatter (line 8), before the `# Brainstorming` header (line 10):

```markdown

## Hard Gate

Do NOT skip to brief creation, spec writing, or any implementation activity. The brainstorming report MUST be written and the human MUST select a direction before proceeding to the next pipeline stage.
```

- [ ] **Step 3: Add Anti-Pattern and Red Flags after "Your Role" section (after line 23)**

Insert after the "Your Role" section (after line 23, before "## Workflow"):

```markdown

## Anti-Pattern: "This Is Too Simple To Need Brainstorming"

Every product idea goes through this process. A CLI flag, a single endpoint, a config change — all of them. "Simple" ideas are where unexamined assumptions cause the most wasted work. The brainstorming session can be short (one technique, 10 minutes), but you MUST explore before committing to a direction.

## Red Flags — Thoughts That Mean STOP

If you catch yourself thinking any of these, you are about to skip the process:

| Thought | Reality |
|---------|---------|
| "The user already knows what they want" | They know the WHAT, not the WHY or the edge cases |
| "This is just a small feature" | Small features with unexamined assumptions cause the biggest rework |
| "Let me just start the brief" | Brainstorming informs the brief. Skipping it means guessing |
| "We already discussed this" | Prior conversation is not structured ideation |
| "I can see the solution already" | You see ONE solution. The process finds alternatives |
| "The user seems impatient" | A 10-minute brainstorm saves hours of rework |
| "This doesn't need alternatives" | Every direction needs at least one alternative explored |
```

- [ ] **Step 4: Add Visual Tooling section before "## Workflow" (which is now further down after the insertions)**

Insert before the "## Workflow" section:

```markdown

## Visual Tooling

When visual content would help the human understand options or make decisions, use the best available tool. No hard dependency on any single tool.

| Tier | Tool | Check | Best for |
|------|------|-------|----------|
| 1 | `/vsdd-factory:visual-companion` | Node.js available, user accepts | Interactive mockups, A/B choices, clickable layouts |
| 2 | `/vsdd-factory:excalidraw-export` | Excalidraw skill loaded | Architecture diagrams, flow charts, entity relationships |
| 3 | Mermaid code blocks | Always available | Sequence diagrams, state machines, simple flows |
| 4 | ASCII/text | Always available | Wireframe sketches, table layouts, comparisons |

Before using Tier 1, ask the human once:
> "I can show visual options in a browser for this. Want to try it? (Requires Node.js and opening a local URL)"

If they decline or Node.js isn't available, fall back to the next tier. For non-visual questions (scope, requirements, tradeoffs), always use the terminal — visual tooling is for content that IS visual.
```

- [ ] **Step 5: Verify the file reads correctly**

Read the modified file and confirm all sections are present and properly ordered:
1. Frontmatter
2. Hard Gate
3. `# Brainstorming: Guided Ideation`
4. When This Skill Runs
5. Your Role
6. Anti-Pattern
7. Red Flags
8. Visual Tooling
9. Workflow (Steps 1-6)
10. Step-File Decomposition
11. Quality Gate
12. Failure Modes
13. Output Artifacts

- [ ] **Step 6: Commit**

```bash
git add plugins/vsdd-factory/skills/brainstorming/SKILL.md
git commit -m "feat(skills): add hard gate, anti-pattern, Red Flags, and visual tooling to brainstorming"
```

---

### Task 4: Add hard gate and visual tooling to guided-brief-creation

**Files:**
- Modify: `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md`

- [ ] **Step 1: Read the current file**

Read `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md`.

- [ ] **Step 2: Add Hard Gate after the delegation reference block (after line 12, before "# Guided Brief Creation")**

Insert after the delegation reference blockquote (line 12), before `# Guided Brief Creation` (line 14):

```markdown

## Hard Gate

Do NOT skip to PRD creation, architecture design, or any implementation activity. The product brief MUST be completed and validated before proceeding.
```

- [ ] **Step 3: Add Visual Tooling section after "Your Role" (after line 35, before "## Workflow")**

Insert after the "Your Role" section:

```markdown

## Visual Tooling

When visual content would help the human understand options or make decisions, use the best available tool. No hard dependency on any single tool.

| Tier | Tool | Check | Best for |
|------|------|-------|----------|
| 1 | `/vsdd-factory:visual-companion` | Node.js available, user accepts | Interactive mockups, A/B choices, clickable layouts |
| 2 | `/vsdd-factory:excalidraw-export` | Excalidraw skill loaded | Architecture diagrams, flow charts, entity relationships |
| 3 | Mermaid code blocks | Always available | Sequence diagrams, state machines, simple flows |
| 4 | ASCII/text | Always available | Wireframe sketches, table layouts, comparisons |

Before using Tier 1, ask the human once:
> "I can show visual options in a browser for this. Want to try it? (Requires Node.js and opening a local URL)"

If they decline or Node.js isn't available, fall back to the next tier. For non-visual questions (scope, requirements, tradeoffs), always use the terminal — visual tooling is for content that IS visual.
```

- [ ] **Step 4: Commit**

```bash
git add plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md
git commit -m "feat(skills): add hard gate and visual tooling to guided-brief-creation"
```

---

### Task 5: Add hard gate and self-review to create-brief

**Files:**
- Modify: `plugins/vsdd-factory/skills/create-brief/SKILL.md`

- [ ] **Step 1: Read the current file**

Read `plugins/vsdd-factory/skills/create-brief/SKILL.md`.

- [ ] **Step 2: Add Hard Gate after frontmatter (after line 6, before "# Create Product Brief")**

Insert after frontmatter closing `---`:

```markdown

## Hard Gate

Do NOT skip to PRD creation or architecture design. Every discovery section MUST be explored with the human. Do not auto-fill sections from assumptions.
```

- [ ] **Step 3: Add Self-Review section before "## After Writing" (before line 104)**

Insert before the "## After Writing" section:

```markdown

## Self-Review (before adversarial review)

Before routing to the next pipeline stage, check your own work:

1. **Placeholder scan:** Any "TBD", "TODO", incomplete sections, or vague requirements? Fix them now.
2. **Internal consistency:** Does the scope match the success criteria? Do constraints align with the value proposition?
3. **Scope check:** Is this brief focused enough for a single PRD, or does it describe multiple independent products?
4. **Ambiguity check:** Could any requirement be interpreted two different ways? Pick one and make it explicit.

Fix issues inline. This is a cheap filter — catch obvious gaps before the next stage.
```

- [ ] **Step 4: Commit**

```bash
git add plugins/vsdd-factory/skills/create-brief/SKILL.md
git commit -m "feat(skills): add hard gate and self-review to create-brief"
```

---

### Task 6: Add hard gate and self-review to create-prd

**Files:**
- Modify: `plugins/vsdd-factory/skills/create-prd/SKILL.md`

- [ ] **Step 1: Read the current file**

Read `plugins/vsdd-factory/skills/create-prd/SKILL.md`.

- [ ] **Step 2: Add Hard Gate after frontmatter (after line 5, before "# Create PRD")**

Insert after frontmatter closing `---`:

```markdown

## Hard Gate

Do NOT skip to architecture design or story decomposition. Every behavioral contract MUST be defined with testable preconditions and postconditions before proceeding.
```

- [ ] **Step 3: Add Self-Review section before "## After Writing" (before line 131)**

Insert before the "## After Writing" section:

```markdown

## Self-Review (before adversarial review)

Before routing to adversarial review, check your own work:

1. **Placeholder scan:** Any "TBD", "TODO", incomplete BCs, or vague preconditions/postconditions? Fix them now.
2. **Internal consistency:** Do BC IDs match the PRD index? Do subsystem numbers align across files? Does the error taxonomy cover all error cases referenced in BCs?
3. **Scope check:** Is each BC focused on a single behavior, or are any trying to cover multiple concerns?
4. **Ambiguity check:** Could any precondition or postcondition be interpreted two different ways? Pick one and make it explicit.

Fix issues inline. This is a cheap filter — catch obvious gaps before spending tokens on the adversary.
```

- [ ] **Step 4: Commit**

```bash
git add plugins/vsdd-factory/skills/create-prd/SKILL.md
git commit -m "feat(skills): add hard gate and self-review to create-prd"
```

---

### Task 7: Add hard gate, self-review, and visual tooling to create-architecture

**Files:**
- Modify: `plugins/vsdd-factory/skills/create-architecture/SKILL.md`

- [ ] **Step 1: Read the current file**

Read `plugins/vsdd-factory/skills/create-architecture/SKILL.md`.

- [ ] **Step 2: Add Hard Gate after frontmatter (after line 5, before "# Create Architecture")**

Insert after frontmatter closing `---`:

```markdown

## Hard Gate

Do NOT skip to story decomposition or implementation. Purity boundaries MUST be drawn and verification properties MUST be defined before proceeding.
```

- [ ] **Step 3: Add Visual Tooling section after "Prerequisites" and "Reference Repos" (after line 32, before "## Process")**

Insert before the "## Process" section:

```markdown

## Visual Tooling

When visual content would help the human understand architecture options, use the best available tool. No hard dependency on any single tool.

| Tier | Tool | Check | Best for |
|------|------|-------|----------|
| 1 | `/vsdd-factory:visual-companion` | Node.js available, user accepts | Interactive component diagrams, side-by-side architecture comparisons |
| 2 | `/vsdd-factory:excalidraw-export` | Excalidraw skill loaded | Architecture diagrams, dependency graphs, data flow |
| 3 | Mermaid code blocks | Always available | Sequence diagrams, component diagrams, state machines |
| 4 | ASCII/text | Always available | Simple dependency trees, layer diagrams |

Before using Tier 1, ask the human once:
> "I can show architecture diagrams in a browser for this. Want to try it? (Requires Node.js and opening a local URL)"

If they decline or Node.js isn't available, fall back to the next tier.
```

- [ ] **Step 4: Add Self-Review section before "## After Writing" (before line 103)**

Insert before the "## After Writing" section:

```markdown

## Self-Review (before adversarial review)

Before routing to adversarial review, check your own work:

1. **Placeholder scan:** Any "TBD", "TODO", incomplete sections, or vague decisions? Fix them now.
2. **Internal consistency:** Do ARCH section cross-references resolve? Do VP traces point to real BCs? Is the dependency graph acyclic?
3. **Scope check:** Does every module have a clear single responsibility? Are purity boundaries drawn for all modules?
4. **Ambiguity check:** Could any architecture decision be interpreted two different ways? Make the chosen option and rationale explicit.

Fix issues inline. This is a cheap filter — catch obvious gaps before spending tokens on the adversary.
```

- [ ] **Step 5: Commit**

```bash
git add plugins/vsdd-factory/skills/create-architecture/SKILL.md
git commit -m "feat(skills): add hard gate, self-review, and visual tooling to create-architecture"
```

---

### Task 8: Add self-review to create-domain-spec

**Files:**
- Modify: `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md`

- [ ] **Step 1: Read the current file**

Read `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md`.

- [ ] **Step 2: Add Self-Review section before "## After Writing" (before line 108)**

Insert before the "## After Writing" section:

```markdown

## Self-Review (before adversarial review)

Before routing to the next pipeline stage, check your own work:

1. **Placeholder scan:** Any "TBD", "TODO", incomplete entity definitions, or vague invariants? Fix them now.
2. **Internal consistency:** Do entity relationships in `entities.md` match references in `capabilities.md`? Does the ubiquitous language glossary cover all terms used in other sections?
3. **Scope check:** Are bounded contexts clearly delineated? Is each section focused on one aspect of the domain?
4. **Ambiguity check:** Could any invariant or business rule be interpreted two different ways? Pick one and make it explicit.

Fix issues inline. This is a cheap filter — catch obvious gaps before the next stage.
```

- [ ] **Step 3: Commit**

```bash
git add plugins/vsdd-factory/skills/create-domain-spec/SKILL.md
git commit -m "feat(skills): add self-review to create-domain-spec"
```

---

### Task 9: Update docs (FACTORY.md and VSDD.md)

**Files:**
- Modify: `plugins/vsdd-factory/docs/FACTORY.md`
- Modify: `plugins/vsdd-factory/docs/VSDD.md`

- [ ] **Step 1: Add visual companion to FACTORY.md**

In the "Project-Specific Instructions (CLAUDE.md)" section that was added in the previous release (search for "scaffold-claude-md"), add a sibling section after it:

```markdown
### Visual Companion (optional)

`/vsdd-factory:visual-companion` provides a browser-based tool for showing mockups, diagrams, and interactive choices during brainstorming, brief creation, and architecture design. Requires Node.js. Early-phase skills automatically detect availability and fall back to Mermaid code blocks, excalidraw-export, or ASCII text when the visual companion isn't available.
```

- [ ] **Step 2: Add visual companion to VSDD.md Tooling section**

In the Tooling section (added in previous release), add a bullet:

```markdown
- **Visual companion:** `/vsdd-factory:visual-companion` — browser-based mockups and interactive options during early pipeline stages. Optional, requires Node.js.
```

- [ ] **Step 3: Commit both**

```bash
git add plugins/vsdd-factory/docs/FACTORY.md plugins/vsdd-factory/docs/VSDD.md
git commit -m "docs: add visual companion to FACTORY.md and VSDD.md"
```
