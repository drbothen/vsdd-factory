---
name: factory-health
description: Validate and auto-repair the .factory/ worktree. Run at session start or when .factory/ state seems wrong. Checks orphan branch exists, worktree is mounted, and STATE.md is present.

allowed-tools: Bash, Read, Write
---

# Factory Health Check

Validate that the `.factory/` worktree is properly mounted and healthy. Auto-repair common issues.

## Checks (run in order)

### 1. Orphan branch exists

```bash
git branch --list factory-artifacts
```

- **If missing**: Create it.
  ```bash
  git checkout --orphan factory-artifacts
  git rm -rf --cached . 2>/dev/null || true
  git commit --allow-empty -m "chore: initialize factory-artifacts orphan branch"
  git checkout -  # return to previous branch
  ```

### 2. Worktree is mounted

```bash
git worktree list | grep -F '.factory'
```

- **If missing**: Mount it.
  ```bash
  git worktree add .factory factory-artifacts
  ```

- **If mounted but pointing to wrong branch**: Remove and remount.
  ```bash
  git worktree remove .factory --force
  git worktree add .factory factory-artifacts
  ```

### 3. Worktree is on correct branch

```bash
cd .factory && git branch --show-current
```

- Must be `factory-artifacts`. If not, the worktree is corrupt — remove and remount.

### 4. STATE.md exists

```bash
test -f .factory/STATE.md
```

- **If missing**: Create initial STATE.md. Derive `product` from the
  repository — use the repo directory name verbatim (do NOT strip any
  `-blue`/`-green` or other suffix; the name on disk is the name):
  ```bash
  git rev-parse --show-toplevel | xargs basename
  ```
  If that name is ambiguous or clearly not the product (e.g. a generic
  checkout dir), ask the human for the product name instead. Then write:
  ```yaml
  ---
  pipeline: INITIALIZED
  phase: pre-1
  product: <repo name from git, or human-supplied>
  mode: greenfield
  timestamp: <current ISO8601>
  ---
  ```

### 5. Directory structure intact

Verify these directories exist inside `.factory/`:

```
specs/ specs/behavioral-contracts/ specs/verification-properties/
specs/architecture/ specs/prd-supplements/ stories/ cycles/
holdout-scenarios/ holdout-scenarios/wave-scenarios/
holdout-scenarios/evaluations/ semport/ code-delivery/
demo-evidence/ dtu-clones/
```

- **If any missing**: Create them with `.gitkeep`.

### 6. Reference repos (conditional)

```bash
test -f .factory/reference-manifest.yaml
```

- **If manifest exists**: This project has brownfield-ingested repos. Verify `.reference/` is populated:
  - Parse the manifest and check that each listed repo has a corresponding directory in `.reference/`.
  - Report any missing repos with their clone URL so the user can rebuild.
  - Report count: `Reference repos: <N>/<total> present`
- **If no manifest**: Skip — this is a from-scratch project.

### 7. Factory lock status (BC-6.23.001 PC7 — shared helper)

Invoke the shared three-state lock status helper:

```bash
${CLAUDE_PLUGIN_ROOT}/bin/factory-lock-status.sh .factory/STATE.md "$(git config user.email)"
```

Append the output line to the health report. The helper returns one of:
- `Factory lock: FREE` — no lock held or lock expired
- `Factory lock: HELD by this session (expires <expires_at>)` — self-held, unexpired
- `Factory lock: HELD by <holder_email> since <locked_at> (expires <expires_at>)` — foreign, unexpired
- `Factory lock: FREE (malformed block — treated as unlocked)` — parse failure, fail-open

This check reads the LOCAL STATE.md (no fetch required — local view is what matters for
informational display). Invokes the shared `factory-lock-status.sh` helper (AC-008
shared-helper mandate) so display cannot diverge from `/factory-worktree-health`.

### 8. Sync state

```bash
cd .factory && git status --porcelain
```

- **Clean**: All good.
- **Uncommitted changes**: Warn the user — there are uncommitted factory artifacts.
- **Diverged from remote**: Warn — manual resolution needed.

### 9. STATE.md health

Check STATE.md size and content routing compliance:

```bash
wc -l < .factory/STATE.md
```

- **≤ 200 lines**: Healthy.
- **201-500 lines**: Warn — recommend `/vsdd-factory:compact-state`.
- **501+ lines**: Error — STATE.md is bloated with historical content. Must compact before proceeding.

Also check for content that shouldn't be in STATE.md:
- Count `## Burst` or `## Pass` section headers — more than 10 means burst narratives are accumulating
- Count `## Session Resume Checkpoint` headers — more than 1 means old checkpoints aren't archived
- Count `adversary_pass_` frontmatter fields — more than 5 means per-pass tracking is in frontmatter

If any issues found, report them and recommend `/vsdd-factory:compact-state`.

### 10. Factory-artifact leak scan (#515 — content-based)

Detect factory artifacts that leaked onto the product branch. The `.factory/`
path-prefix leak check (#341) only sees artifacts under `.factory/`; a factory
artifact committed to a product path (e.g. a root-level `red-gate-log-*.md`)
is invisible to it. This scan keys on CONTENT — the frontmatter
`document_type:` — instead of location.

```bash
# Rust binary (crates/factory-artifact-leak-scan, POLICY 21) — build once if absent:
[ -x target/release/factory-artifact-leak-scan ] || cargo build --release -p factory-artifact-leak-scan
target/release/factory-artifact-leak-scan
```

The scanner is advisory (it reports, never mutates). It reads the factory
document_type set from `templates/` (honoring `CLAUDE_PLUGIN_ROOT`) and flags
any tracked file, outside `.factory/` and outside plugin machinery, whose
`document_type` is a factory-produced type — except product deliverables like
demo-evidence, which are exempt only under their canonical home directory.

- **Exit 0 / "Product tree is clean"**: no leaks.
- **Exit 1 (table of leaks)**: relocate each to its canonical `.factory/` home
  (see `config/artifact-path-registry.yaml`) via `/vsdd-factory:relocate-artifact`,
  or, if a genuine product deliverable, add its `(document_type -> home directory)`
  pair to `PRODUCT_TRACKED_HOMES` in `crates/factory-artifact-leak-scan` with
  justification.
- If the repo has no Rust toolchain (plugin consumed outside the engine repo),
  skip this check and note it as NOT RUN in the report — do not fail health on
  a missing binary.

## Output

Report a summary:

```
Factory Health: ✓ HEALTHY
  Branch:    factory-artifacts (orphan)
  Worktree:  .factory/ mounted
  STATE.md:  present (phase: <current phase>)
  State size: <N> lines (healthy | warning | bloated)
  Structure: all directories present
  Sync:      clean | uncommitted changes | diverged
```

Or if repairs were made:

```
Factory Health: REPAIRED
  Fixed: <list of what was repaired>
  Current state: <summary>
```
