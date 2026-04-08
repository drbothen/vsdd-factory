---
name: demo-recording
description: >
  Records visual demonstrations of the target project for human review evidence,
  PR documentation, and regression baseline. Supports CLI (VHS), web (Playwright),
  API (cURL), and library (test harness) demos. Generates demo scripts from
  acceptance criteria and produces optimized WebM/GIF output.
---

# Demo Recording

## When This Skill Runs

- **After implementation complete and all tests pass**: Record per-AC demos
- **After holdout evaluation**: Record holdout scenario execution
- **After convergence achieved** (or delta convergence): Record full user journey "money shot" demo
- **On demand**: Human requests "Record a demo of [feature]"

## Prerequisites

- Target project builds and runs successfully
- Demo toolchain installed (VHS for CLI, Playwright for web, ffmpeg for post-processing)
- `.factory/toolchain-state.yaml` confirms demo tools are available
- Acceptance criteria exist in story specs

## Workflow

### Step 1: Detect Demo Type

Read the architecture document to determine what kind of product this is:

| Detection Signal | Demo Type | Recording Tool |
|-----------------|-----------|---------------|
| CLI binary / `clap` / `argparse` / `commander` | CLI | VHS |
| HTTP routes / REST endpoints / GraphQL | Web API | cURL sequence |
| React / Next.js / Vue / Angular / HTML | Web UI | Playwright |
| Library crate / npm package / pip package | Library | VHS (test output) |
| Multiple of the above | Composite | VHS + Playwright |

Write demo type to `.factory/demo-state.yaml`:

```yaml
demo_type: cli          # cli | web-ui | web-api | library | composite
recording_tool: vhs     # vhs | playwright | curl | composite
product_name: "taskcli"
binary_path: "./target/release/taskcli"
```

### Step 2: Generate Demo Scripts from Acceptance Criteria

For each story's acceptance criteria, generate a demo script:

**CLI demos (VHS .tape files):**

Parse each AC and generate VHS commands:
- "User can [action]" → `Type "[command]"` + `Enter`
- "System displays [output]" → `Sleep 2s` (wait for output to render)
- "Error message [text]" → `Type "[invalid command]"` + `Enter` + `Sleep 1s`

Template for each AC:
```
Output demos/{story-id}/{ac-id}.webm
Set FontSize 22
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful
Set BorderRadius 8
Set Padding 20
Set Framerate 30
Set PlaybackSpeed 1.0

# AC: {ac_description}
Hide
# Setup commands here (if needed)
Show

Type "{command}"
Enter
Sleep 2s
```

**Web UI demos (Playwright tests with video):**

Enable video recording on existing E2E tests:
```typescript
// Add to playwright.config.ts or test-level config
use: {
  video: {
    mode: 'on',
    size: { width: 1280, height: 720 }
  },
  trace: 'on'
}
```

The E2E tests written by the `e2e-tester` agent (DF-002) become the demo scripts.
No separate demo generation needed -- just enable recording on existing tests.

**API demos (cURL scripts):**

For each API endpoint acceptance criterion, generate a cURL demo script:
```bash
#!/usr/bin/env bash
# Demo: AC-001 — Create a task via API
echo "=== POST /api/tasks ==="
curl -s -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy groceries", "priority": "high"}' | jq .

echo ""
echo "=== GET /api/tasks ==="
curl -s http://localhost:3000/api/tasks | jq .
```

Then record the script execution with VHS:
```
Output demos/api-demo.webm
Set FontSize 20
Set Width 1400
Set Height 800

Type "bash demos/api-demo.sh"
Enter
Sleep 5s
```

Write all generated scripts to `.factory/demo-scripts/`.

### Step 3: Execute Recordings

**For VHS recordings:**
```bash
# Install VHS if not present
if ! command -v vhs &>/dev/null; then
  brew install vhs  # macOS
  # or: go install github.com/charmbracelet/vhs@latest
fi

# Record each tape file
for tape in .factory/demo-scripts/*.tape; do
  vhs "$tape"
done
```

**For Playwright recordings:**
```bash
# Run E2E tests with video enabled
npx playwright test --project=demo \
  --reporter=html \
  --output=.factory/demo-recordings/
```

**For cURL/API recordings:**
```bash
# Start the service, record the API demo via VHS
# (VHS wraps the cURL execution in a terminal recording)
vhs .factory/demo-scripts/api-demo.tape
```

### Step 4: Post-Process with ffmpeg

Optimize all recordings for distribution:

```bash
# WebM optimization (primary format — best compression)
ffmpeg -i input.webm \
  -c:v libvpx-vp9 -crf 30 -b:v 0 \
  -vf "scale=1280:-2" \
  -an \
  -threads 0 \
  output.webm

# GIF fallback (for GitHub markdown compatibility)
ffmpeg -i input.webm \
  -vf "fps=15,scale=800:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" \
  -loop 0 \
  output.gif

# Trim (remove setup time)
ffmpeg -i input.webm -ss 2 -t 30 -c copy trimmed.webm

# Add text overlay (AC description)
ffmpeg -i input.webm \
  -vf "drawtext=text='AC-001\: Add task':fontsize=18:fontcolor=white:x=10:y=10:box=1:boxcolor=black@0.5" \
  output_labeled.webm
```

Target sizes:
- WebM: < 2MB per 30-second demo
- GIF: < 5MB (GitHub displays these inline)
- Total per PR: < 25MB (GitHub attachment limit)

### Step 5: Generate Evidence Report

Write demo evidence to `.factory/demo-evidence/`:

```markdown
# Demo Evidence Report

## Pipeline Run: {timestamp}
## Product: {product_name}

### Per-AC Demo Recordings

| AC | Story | Description | Recording | Duration | Size |
|----|-------|-------------|-----------|----------|------|
| AC-001 | STORY-001 | Add a task | [WebM](ac-001.webm) [GIF](ac-001.gif) | 12s | 340KB |
| AC-002 | STORY-001 | List tasks | [WebM](ac-002.webm) [GIF](ac-002.gif) | 8s | 210KB |
| AC-003 | STORY-002 | Complete task | [WebM](ac-003.webm) [GIF](ac-003.gif) | 15s | 420KB |

### Full User Journey Demo

| Demo | Recording | Duration | Size |
|------|-----------|----------|------|
| Complete workflow | [WebM](full-journey.webm) | 45s | 1.2MB |

### Holdout Scenario Demos (if recorded)

| Scenario | Recording | Satisfaction | Duration |
|----------|-----------|-------------|----------|
| HS-001 | [WebM](hs-001.webm) | 0.95 | 20s |

### Demo Toolchain

| Tool | Version | Status |
|------|---------|--------|
| VHS | {version} | {installed/missing} |
| Playwright | {version} | {installed/missing} |
| ffmpeg | {version} | {installed/missing} |
```

### Step 6: Package for Code Delivery

Prepare demo artifacts for the code delivery skill (DF-015):

1. Copy optimized recordings to `.factory/demo-evidence/`
2. Generate markdown snippet for PR description embedding
3. Upload full recordings as GitHub release assets or PR artifacts
4. Generate GIF thumbnails for inline PR display

PR description snippet format:
```markdown
## Demo Evidence

### Full User Journey
![Full Journey Demo](demos/full-journey.gif)

### Per-Feature Demos
| Feature | Demo |
|---------|------|
| Add task | ![](demos/ac-001.gif) |
| List tasks | ![](demos/ac-002.gif) |
| Complete task | ![](demos/ac-003.gif) |

<details>
<summary>Full-size WebM recordings</summary>

- [Full journey (WebM, 1.2MB)](demos/full-journey.webm)
- [AC-001: Add task (WebM, 340KB)](demos/ac-001.webm)
- [All recordings (ZIP, 3.4MB)](demos/all-demos.zip)

</details>
```

### Step 7: Visual Review (NEW — DF-018)

Spawn `visual-reviewer` agent to analyze all demo recordings:

1. For each per-AC demo: verify the recording visually demonstrates the acceptance criterion
2. For the full user journey: verify the complete happy path is shown
3. For Feature Mode: compare baseline vs current demos for visual regressions
4. Write findings to `.factory/demo-evidence/visual-review.md`

The visual-reviewer uses review-tier model (native video input) for WebM analysis and
adversary model (10.24MP image analysis) for high-res screenshot comparison.

## Output Artifacts

- `.factory/demo-scripts/*.tape` — VHS tape files for CLI demos
- `.factory/demo-scripts/*.sh` — cURL scripts for API demos
- `.factory/demo-recordings/*.webm` — optimized WebM recordings
- `.factory/demo-recordings/*.gif` — GIF fallbacks for GitHub embedding
- `.factory/demo-recordings/traces/*.zip` — Playwright traces (web UI)
- `.factory/demo-evidence/report.md` — evidence report linking all recordings
- `.factory/demo-evidence/visual-review.md` — visual reviewer findings
- `.factory/demo-state.yaml` — demo type detection and toolchain state

## Quality Gate Criteria

- [ ] Every acceptance criterion with user-observable behavior has a demo recording
- [ ] All recordings are under 2MB (WebM) or 5MB (GIF)
- [ ] Full user journey demo exists covering the complete happy path
- [ ] Demo evidence report links all recordings with metadata
- [ ] PR description snippet is generated for code delivery
- [ ] ffmpeg post-processing applied (optimized, trimmed, labeled)
- [ ] Visual reviewer has analyzed all recordings and reported findings
