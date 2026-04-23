# Hooks Reference

The vsdd-factory plugin ships hook scripts wired through `hooks.json`. Hooks fire automatically on tool use events, subagent completion, and session end. They enforce pipeline discipline without requiring manual intervention.

The **Instrumented** column indicates whether the hook emits structured block events to `.factory/logs/events-YYYY-MM-DD.jsonl` via `bin/emit-event`. See the [observability guide](observability.md) for the event schema, reason-code registry, and query recipes.

> **Note:** This reference table currently undercounts. A doc audit reconciling every hook script in `plugins/vsdd-factory/hooks/` with this page is tracked in the observability roadmap (end-of-Phase-2 cleanup release).

---

## Hook Summary

| Hook | Event | Matcher | What It Enforces | Blocking | Instrumented |
|------|-------|---------|-----------------|----------|--------------|
| `brownfield-discipline.sh` | PreToolUse | Edit\|Write | `.reference/` directories are read-only | Yes | ✓ (1 code) |
| `protect-vp.sh` | PreToolUse | Edit\|Write | Green Verification Properties are immutable | Yes | ✓ (1 code) |
| `protect-bc.sh` | PreToolUse | Edit\|Write | Green Behavioral Contracts are immutable | Yes | ✓ (1 code) |
| `red-gate.sh` | PreToolUse | Edit\|Write | TDD red-before-green discipline | Yes (when strict mode active) | ✓ (1 code) |
| `factory-branch-guard.sh` | PreToolUse | Edit\|Write | `.factory/` writes only allowed on `factory-artifacts` worktree | Yes | ✓ (2 codes) |
| `destructive-command-guard.sh` | PreToolUse | Bash | Blocks catastrophic `rm` targets, `rm -rf` on protected paths, SoT clobbering redirects, `find -delete`, dangerous git/gh operations, `--no-verify`, `curl\|bash` | Yes | ✓ (27 codes) |
| `protect-secrets.sh` | PreToolUse | Bash + Read | Blocks reads/copies of `.env` files, echoing secret-shaped env vars, `env \| grep` for secrets | Yes | ✓ (6 codes) |
| `verify-git-push.sh` | PreToolUse | Bash | Blocks force push (`--force`/`-f`) and direct push to protected branches (main, master, develop) | Yes | ✓ (2 codes) |
| `block-ai-attribution.sh` | PreToolUse | Bash | Blocks git commits containing `Co-Authored-By: Claude`/AI attribution patterns | Yes | ✓ (2 codes) |
| `check-factory-commit.sh` | PreToolUse | Bash | Remind about STATE.md after `.factory/` commits | No (advisory) | — |
| `validate-wave-gate-prerequisite.sh` | PreToolUse | Agent | Blocks Wave N+1 worker dispatch if Wave N gate is pending | Yes | ✓ (1 code) |
| `validate-pr-merge-prerequisites.sh` | PreToolUse | Agent | Blocks github-ops merge dispatch if pr-description/pr-review/security-review evidence is missing | Yes | ✓ (1 code) |
| `purity-check.sh` | PostToolUse | Edit\|Write | Pure-core boundary -- no side effects in pure modules | No (warn-only) | — |
| `validate-vp-consistency.sh` | PostToolUse | Edit\|Write | VP-INDEX ↔ verification-architecture ↔ coverage-matrix consistency (Policy 9) | Yes (exit 2 on mismatch) | — |
| `validate-subsystem-names.sh` | PostToolUse | Edit\|Write | BC/story subsystem fields match ARCH-INDEX canonical names (Policy 6) | Yes (exit 2 on mismatch) | — |
| `validate-bc-title.sh` | PostToolUse | Edit\|Write | BC file H1 heading matches BC-INDEX title (Policy 7) | Yes (exit 2 on mismatch) | — |
| `validate-story-bc-sync.sh` | PostToolUse | Edit\|Write | Story frontmatter bcs: ↔ body BC table ↔ AC traces sync (Policy 8) | Yes (exit 2 on mismatch) | — |
| `validate-template-compliance.sh` | PostToolUse | Edit\|Write | Artifact has required frontmatter fields and section headings from its template | Yes (exit 2 on missing) | — |
| `validate-finding-format.sh` | PostToolUse | Edit\|Write | Only current finding/fix ID formats accepted (blocks legacy ADV-NNN, STORY-NNN-FIX) | Yes (exit 2 on legacy format) | — |
| `validate-input-hash.sh` | PostToolUse | Edit\|Write | Warns on missing/stale `input-hash`; blocks non-7-char or non-lowercase-hex format | Partial (format=block, drift=advisory) | — |
| `validate-state-size.sh` | PostToolUse | Edit\|Write | STATE.md line count enforcement — warns >200, blocks >500 (allows compaction writes) | Yes (>500 and growing) | — |
| `validate-novelty-assessment.sh` | PostToolUse | Edit\|Write | Adversarial review files must have Novelty Assessment section with required fields | Yes (exit 2 on missing) | — |
| `convergence-tracker.sh` | PostToolUse | Edit\|Write | Convergence rule enforcement — trajectory monotonicity, min 3 clean passes, novelty ≤ 0.15, zero-findings warning | Partial (premature CONVERGENCE=block, regression=warn) | — |
| `validate-table-cell-count.sh` | PostToolUse | Edit\|Write | Markdown table rows must have same pipe count as header (catches unescaped pipes in cells) | Yes (exit 2 on mismatch) | — |
| `validate-changelog-monotonicity.sh` | PostToolUse | Edit\|Write | Changelog versions descending, no duplicates, dates non-increasing, frontmatter version matches top row | Yes (exit 2 on violation) | — |
| `validate-state-pin-freshness.sh` | PostToolUse | Edit\|Write | STATE.md version pins must match actual artifact file versions | Yes (exit 2 on mismatch) | — |
| `validate-index-self-reference.sh` | PostToolUse | Edit\|Write | INDEX.md/burst-log.md edits should reference current pass/burst | No (advisory warning) | — |
| `regression-gate.sh` | PostToolUse | Bash | Track test pass/fail transitions | No (telemetry) | — |
| `handoff-validator.sh` | SubagentStop | (all) | Subagent output is non-empty and structurally plausible | No (warn-only) | — |
| `session-learning.sh` | Stop | (all) | Append learning marker to `.factory/sidecar-learning.md` | No (non-blocking) | — |

---

## Hook Details

### brownfield-discipline.sh

**Event:** PreToolUse on Edit or Write

Blocks any edit or write to paths containing `/.reference/` or starting with `.reference/`. Reference codebases are cloned into `.reference/` during brownfield ingestion for analysis. Editing them corrupts the extraction and poisons downstream spec work.

**Debugging:** If a legitimate edit is blocked, verify the file path does not traverse a `.reference/` directory. Reference repos are read-only by design.

### protect-vp.sh

**Event:** PreToolUse on Edit or Write

Blocks edits to Verification Property files in `.factory/specs/verification-properties/` that have reached `Status: green`. Per SOUL.md principle 4 and spec-format rules, green VPs are immutable. To change a green VP, create a new VP that supersedes it and retire the old one.

**Response format:** Emits a PreToolUse JSON envelope with `permissionDecision`:
```json
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny"}}
```

**Debugging:** Read the VP file and check its `Status` field. If it shows `green`, the hook will deny edits. Create a new VP-NNN file instead of modifying the existing one.

### protect-bc.sh

**Event:** PreToolUse on Edit or Write

Companion to `protect-vp.sh`. Blocks edits to Behavioral Contract files in `.factory/specs/behavioral-contracts/BC-*.md` that have reached `Status: green`. Non-green contracts are freely editable.

**Response format:** Same `permissionDecision` envelope as `protect-vp.sh`.

**Debugging:** Same as protect-vp -- check the `Status` field. Draft and reviewed BCs allow edits; green BCs do not.

### red-gate.sh

**Event:** PreToolUse on Edit or Write

Enforces TDD red-before-green discipline when strict mode is active. Strict mode is opt-in: create `.factory/red-gate-state.json` with the shape `{"mode": "strict", "red": ["path/to/file.rs", ...]}`. When active, only files listed in the `red` array may be edited. Test files are always allowed.

When mode is `"off"` or the state file does not exist, the hook allows all edits.

**Debugging:** Check `.factory/red-gate-state.json`. If strict mode is active and your edit is blocked, add the file to the `red` array or switch mode to `"off"`.

### factory-branch-guard.sh

**Event:** PreToolUse on Edit or Write

Blocks writes to `.factory/` paths when the directory is not mounted as a git worktree on the `factory-artifacts` branch. Prevents artifacts from being committed to the wrong branch (develop, main) when the worktree is missing or misconfigured. Also guards `.factory-project/` for multi-repo projects (expects `factory-project-artifacts` branch).

**Checks:**
1. Does `.factory/.git` exist? (worktree marker file)
2. Is the worktree on `factory-artifacts` branch?

**Debugging:** Run `git -C .factory rev-parse --abbrev-ref HEAD` to check the branch. If wrong: `cd .factory && git checkout factory-artifacts`. If `.factory/.git` is missing: `git worktree add .factory factory-artifacts`.

### destructive-command-guard.sh

**Event:** PreToolUse on Bash

Blocks destructive shell commands that could cause irreversible data loss. Each block message includes a suggestion for the safe alternative.

**Blocked operations:**
- `rm -rf` / `rm -r` targeting `.factory/`, `src/`, `tests/` (allows build dirs like `target/`, `node_modules/`)
- `rm` targeting source-of-truth files (STATE.md, BC-INDEX.md, VP-INDEX.md, STORY-INDEX.md, ARCH-INDEX.md, prd.md)
- `git reset --hard` (suggest `git stash` or `git reset --soft`)
- `git clean -f` / `git clean -fd` (suggest `git clean -n` dry-run first)
- `git checkout -- .` / `git restore .` (suggest targeting specific files)
- `git rm -r` on `.factory/specs/` or `.factory/stories/`

**Allowed operations:** `rm -rf target/`, `rm -rf .worktrees/STORY-NNN/`, `rm` of temp files, `git reset --soft`, `git clean -n`, `git stash`.

**Debugging:** If blocked, read the suggestion in the error message. Most blocks have a safe alternative that achieves the same goal.

### verify-git-push.sh

**Event:** PreToolUse on Bash

Intercepts `git push` commands. Blocks force pushes (`--force` or `-f`) and direct pushes to protected branches (main, master, develop). For normal pushes to feature branches, injects a reminder to ensure tests pass before pushing.

**Blocked operations:**
- `git push origin main` / `master` / `develop` — bypasses PR/review gates
- `git push --force` / `-f` — overwrites remote history

Block messages suggest the PR workflow: push to a feature branch and create a PR.

**Debugging:** If blocked, push to a feature branch instead (`git push origin feature/STORY-NNN`) and create a PR (`gh pr create --base main`).

### check-factory-commit.sh

**Event:** PreToolUse on Bash

Advisory hook that fires after `git commit` commands involving `.factory/`. If STATE.md was not part of the commit, it reminds the user to update STATE.md if a phase transition occurred. Non-blocking -- the commit proceeds regardless.

### purity-check.sh

**Event:** PostToolUse on Edit or Write

Enforces the pure-core boundary from SOUL.md. Files under `*/pure/**`, `*/core/**`, or ending in `_pure.rs` / `.pure.ts` are scanned for known side-effect patterns (I/O, network, global state mutation). Emits a warning to stderr when violations are found. Non-blocking by design -- architectural drift is surfaced, not enforced. The regression gate and CI catch hard failures.

### validate-vp-consistency.sh

**Event:** PostToolUse on Edit or Write

Enforces Policy 9 (`vp_index_is_vp_catalog_source_of_truth`). After any edit to VP-INDEX.md, verification-architecture.md, or verification-coverage-matrix.md, validates consistency across all three files.

**Checks:**
1. Every VP in VP-INDEX appears in verification-architecture.md Provable Properties Catalog
2. Every VP in VP-INDEX appears in verification-coverage-matrix.md VP-to-Module table
3. VP-INDEX per-tool summary totals match actual row counts
4. Coverage matrix Totals row matches data row sums
5. VPs referenced in coverage matrix exist in VP-INDEX (reverse check)

Exits non-zero (exit 2) on mismatch with a diagnostic listing the specific discrepancy. Tested with 3 fixture sets (green, canary with column drift, missing-VP).

**Debugging:** Read the error output — it names the specific VP, file, and mismatch. Fix the inconsistency and re-save. The hook re-validates on each edit.

### validate-subsystem-names.sh

**Event:** PostToolUse on Edit or Write

Enforces Policy 6 (`architecture_is_subsystem_name_source_of_truth`). After any edit to BC files (`BC-*.md`) or story files (`STORY-*.md`), extracts the `subsystem:` (BC) or `subsystems:` (story) field and verifies it matches a canonical name from `ARCH-INDEX.md` Subsystem Registry.

If ARCH-INDEX doesn't exist yet (architecture not produced), the hook passes silently. Error messages list all available canonical names so the agent can self-correct.

**Debugging:** Read the error — it names the invalid subsystem and lists valid options. Use the exact canonical name from ARCH-INDEX.

### validate-bc-title.sh

**Event:** PostToolUse on Edit or Write

Enforces Policy 7 (`bc_h1_is_title_source_of_truth`). After any edit to a BC file (`BC-*.md`, not BC-INDEX), extracts the H1 heading (`# BC-S.SS.NNN: <title>`) and compares it to the title column in `BC-INDEX.md`.

If BC-INDEX doesn't exist yet, the hook passes silently. Skips BC-INDEX.md itself. Error messages show both the H1 title and the BC-INDEX title so the agent knows which to fix.

**Debugging:** The H1 is authoritative. If they differ, update BC-INDEX to match the H1, not the other way around.

### validate-story-bc-sync.sh

**Event:** PostToolUse on Edit or Write

Enforces Policy 8 (`bc_array_changes_propagate_to_body_and_acs`). After any edit to a story file (`STORY-*.md`), verifies bidirectional BC completeness:

1. Every BC in frontmatter `bcs:` appears in the body BC table
2. Every BC in frontmatter `bcs:` has at least one AC trace annotation
3. Every BC in the body BC table appears in frontmatter `bcs:`

Skips stories with no `bcs:` field (early creation). Error messages identify the specific missing BCs.

**Debugging:** Read the error — it names which BCs are missing from which representation. Add the missing rows to the body BC table and/or AC trace annotations.

### validate-template-compliance.sh

**Event:** PostToolUse on Edit or Write

Validates that every artifact file written to `.factory/` contains the required frontmatter fields and section headings defined by its corresponding template. Resolves templates by reading the file's `document_type` frontmatter and matching against template files in `${CLAUDE_PLUGIN_ROOT}/templates/`. Falls back to path-pattern matching for files without `document_type` (e.g., `stories/STORY-*.md` → `story-template.md`).

**What it checks:**
1. Required frontmatter fields present (keys only, not values)
2. Required H2 section headings present (skips conditional/optional sections)

Skips INDEX files, non-`.factory/` paths, and YAML/JSON config files.

**Debugging:** Read the warning — it lists exactly which frontmatter fields and sections are missing, and suggests running `/vsdd-factory:conform-to-template` to add them.

### validate-finding-format.sh

**Event:** PostToolUse on Edit or Write

Validates that finding and fix IDs in `.factory/` files use only the current format. Blocks legacy formats that were deprecated during the ID formalization effort.

**Blocked formats (legacy):**
- `ADV-NNN` — bare finding ID without cycle/pass/severity
- `ADV-P[N]-NNN` — missing cycle and severity
- `STORY-NNN-FIX` — old fix format

**Accepted formats (current):**
- `ADV-<CYCLE>-P[N]-[SEV]-NNN` — full finding ID with cycle, pass, severity
- `FIX-P[N]-NNN` — current fix format

**Debugging:** Read the error — it identifies the legacy ID and shows the current format pattern. Replace with the current format.

### validate-input-hash.sh

**Event:** PostToolUse on Edit or Write

Advisory hook that checks `.factory/` artifacts for input-hash drift. After any write to a `.factory/` markdown file with an `inputs:` frontmatter field, checks whether the `input-hash` field is present and current.

**What it checks:**
1. File has `inputs:` but `input-hash` is `"[md5]"` or `null` → warns "no computed input-hash"
2. File has `inputs:` and a stored hash → runs `compute-input-hash --check` to detect drift

**Not blocking** — the write proceeds regardless. The hook surfaces drift so agents can act on it. Use `/vsdd-factory:check-input-drift` for a batch scan at phase gates.

**Debugging:** Run `compute-input-hash <file> --update` to recompute and store the current hash.

### regression-gate.sh

**Event:** PostToolUse on Bash

Watches Bash commands that run tests (`cargo test`, `pytest`, `npm test`, `go test`, `just test`, `just ci`). Records pass/fail to `.factory/regression-state.json`. If the test suite transitions from pass to fail, emits a warning so the next edit can be informed. This is a telemetry hook consumed by the red-gate when strict mode is active.

### handoff-validator.sh

**Event:** SubagentStop

Validates that subagent output is non-empty and structurally plausible. Warns to stderr if the result is empty, whitespace-only, or under 40 characters (suspiciously short for any factory agent). An empty subagent result is a common silent failure -- the orchestrator would act on nothing and the pipeline would drift. Non-blocking -- the orchestrator receives the result and decides how to proceed.

### session-learning.sh

**Event:** Stop

At session end, appends a timestamped learning stub to `.factory/sidecar-learning.md`. The actual synthesis is done by the `/vsdd-factory:session-review` skill; this hook ensures a marker exists so nothing is silently lost when sessions end abruptly. Non-blocking and safe to fail.

---

## Hook Wiring

All hooks are configured in `plugins/vsdd-factory/hooks/hooks.json`. The wiring uses four event types:

| Event | When It Fires |
|-------|--------------|
| `PreToolUse` | Before a tool call executes. Can block (exit 2) or inject context (exit 0 with JSON). |
| `PostToolUse` | After a tool call completes. Cannot block -- advisory only. |
| `SubagentStop` | When a subagent finishes and returns its result. |
| `Stop` | When the session ends. |

Each hook has a 5-second timeout (10 seconds for `verify-git-push.sh`). All hooks require `jq` for JSON parsing of the tool input envelope.
