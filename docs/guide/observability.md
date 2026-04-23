# Observability

vsdd-factory emits a structured event log every time a hook blocks a tool
call. Events are JSONL, append-only, and local to your project — nothing
leaves your machine. The long-term goal is a local dashboard stack
(Phase 5 of the observability plan) that tails these events and renders
flame graphs of pipeline cycles, hook heatmaps, and cost curves. As of
v0.57.0 the foundation is in place: `bin/emit-event` is shipped and four
hooks are instrumented.

This page covers:
1. [Enabling and disabling emission](#enabling-and-disabling-emission)
2. [Where logs live and how they rotate](#where-logs-live)
3. [Event schema](#event-schema)
4. [Querying logs with jq](#querying-logs-with-jq)
5. [Reason-code registry](#reason-code-registry)
6. [Instrumenting your own hook](#instrumenting-your-own-hook)
7. [Safety guarantees](#safety-guarantees)
8. [Roadmap](#roadmap)

---

## Enabling and disabling emission

Emission is **on by default** — no action required after installing the
plugin. Two environment variables control behavior:

| Variable | Default | Effect |
|----------|---------|--------|
| `VSDD_TELEMETRY` | `on` | Set to `off` to short-circuit emission at line 1 of `bin/emit-event`. Hooks still block exactly as they would otherwise; only the log write is skipped. |
| `VSDD_LOG_DIR` | `.factory/logs` | Override the directory where daily log files are written. Must be writable; if not, emission silently drops. |

Typical overrides:

```bash
# Disable for a single session
export VSDD_TELEMETRY=off

# Point to a cross-project log location
export VSDD_LOG_DIR="$HOME/.local/state/vsdd/logs"
```

Because both variables are read per-invocation, you can toggle mid-session.

---

## Where logs live

Default location: `.factory/logs/events-YYYY-MM-DD.jsonl`.

- **One file per day.** A new file is created at the first emission of each
  local calendar day. No rotation machinery is needed — old files stay on
  disk until you clean them up.
- **Gitignored.** `.factory/` is fully gitignored on `main`; logs never
  enter version control on the primary branch. They do not land on the
  `factory-artifacts` orphan branch either.
- **Append-only.** Events are written with `>>`. Under POSIX `PIPE_BUF`
  (typically 4 KB), concurrent writes from parallel hooks do not
  interleave, so no lock file is used. Target event size is well under
  1 KB.

### Retention

There is no automatic retention policy in v0.57.x. Clean up manually:

```bash
# Delete logs older than 30 days
find .factory/logs -name 'events-*.jsonl' -mtime +30 -delete
```

A retention policy ships with the Phase 5 dashboard stack.

---

## Event schema

Every event is a single JSON object on its own line (JSONL format).
Guaranteed fields:

```json
{
  "type": "hook.block",
  "schema_version": 1,
  "ts": "2026-04-22T14:30:00-0500",
  "hook": "destructive-command-guard",
  "matcher": "Bash",
  "reason": "catastrophic_root",
  "command": "rm -rf /"
}
```

| Field | Type | Notes |
|-------|------|-------|
| `type` | string | Event category. Currently always `hook.block`. Future: `hook.pass`, `hook.advice`, `hook.error`, `phase.start`/`end`, `subagent.start`/`end`. |
| `schema_version` | integer | Currently `1`. Incremented on breaking schema changes. |
| `ts` | string | ISO-8601 local timestamp with timezone offset. POSIX-portable format. |
| `hook` | string | Filename stem of the emitting hook (e.g. `destructive-command-guard`). |
| `matcher` | string | Claude Code tool matcher: `Bash`, `Edit\|Write`, `Read`, `Agent`. |
| `reason` | string | Stable snake_case code. See [registry below](#reason-code-registry). |
| `command` | string | For `Bash` hooks: the original command string (pre-execution literal; no variable expansion). For `Read` hooks: the file path. |

### Adding fields

Hooks can emit additional fields by passing more `key=value` pairs to
`bin/emit-event`. Example from a future PostToolUse validator:

```bash
emit-event type=hook.block hook=validate-vp-consistency matcher=Edit \
           reason=policy9_coverage_mismatch \
           file=.factory/specs/verification-properties/VP-042.md \
           vp_id=VP-042
```

Fields with dots (`hook.phase=...`) are kept flat (not nested) in the
resulting JSON.

---

## Querying logs with jq

All useful queries against a daily log file are one-line `jq` expressions
against the stream of JSON objects (use `-s` to slurp into an array, or
streaming operators for larger files).

### Top block reasons

```bash
jq -r '.reason' .factory/logs/events-*.jsonl | sort | uniq -c | sort -rn | head
```

### Blocks per hook

```bash
jq -r '.hook' .factory/logs/events-*.jsonl | sort | uniq -c | sort -rn
```

### All catastrophic blocks this week

```bash
jq -r 'select(.reason == "catastrophic_root") | "\(.ts)\t\(.command)"' \
  .factory/logs/events-*.jsonl
```

### Blocks by hour

```bash
jq -r '.ts | .[0:13]' .factory/logs/events-*.jsonl | sort | uniq -c
```

### Signal vs noise (blocks per day)

```bash
for f in .factory/logs/events-*.jsonl; do
  echo "$(basename $f .jsonl): $(wc -l < $f) events"
done
```

---

## Reason-code registry

Reason codes are stable, snake_case, and unique across all hooks. They
form the aggregation key for dashboards. Add a new code only when an
existing one is not a good fit.

### `destructive-command-guard.sh`

| Code | Triggers on |
|------|-------------|
| `catastrophic_root` | `rm -rf /`, `/*`, `~`, `~/`, `$HOME`, `*`, `.*` |
| `protected_path_delete` | `rm -rf` on `.factory/`, `src/`, `tests/` (non-build dirs) |
| `sot_delete` | `rm` of STATE.md, BC-INDEX.md, VP-INDEX.md, STORY-INDEX.md, ARCH-INDEX.md, HS-INDEX.md, L2-INDEX.md, prd.md |
| `sot_clobber_redirect` | `> STATE.md` (clobber, not append) |
| `sot_truncate_colon` | `: > STATE.md` (truncate idiom) |
| `sot_truncate_cmd` | `truncate -s 0 STATE.md` |
| `sot_clobber_cpnull` | `cp /dev/null STATE.md` |
| `find_delete_protected` | `find .factory … -delete` / `-exec rm` |
| `git_reset_hard` | `git reset --hard` |
| `git_clean_force` | `git clean -f` (dry-run `-n` allowed) |
| `git_checkout_dot` | `git checkout -- .` |
| `git_restore_dot` | `git restore .` |
| `git_stash_discard` | `git stash drop` / `git stash clear` |
| `git_branch_d_protected` | `git branch -D main|master|develop` |
| `git_filter_history` | `git filter-branch` / `git filter-repo` |
| `git_reflog_expire` | `git reflog expire --expire=now` |
| `git_gc_prune_now` | `git gc --prune=now` |
| `git_worktree_force` | `git worktree remove --force` outside `.worktrees/` |
| `git_no_verify` | `--no-verify` on commit/merge/rebase/cherry-pick/am |
| `git_no_gpg_sign` | `--no-gpg-sign` on commit |
| `git_rm_protected` | `git rm` on `.factory/specs/`, `.factory/stories/`, `.factory/STATE.md` |
| `gh_repo_delete` | `gh repo delete` |
| `gh_release_delete` | `gh release delete` |
| `gh_pr_close` | `gh pr close` |
| `gh_issue_delete` | `gh issue delete` |
| `rce_pipe_to_shell` | `curl|wget|fetch ... | bash|sh|zsh|python|perl|ruby` |
| `recursive_permission_change` | `chmod -R` / `chown -R` on `.factory`, `src/`, `tests/`, `.git/` |

### `protect-secrets.sh`

| Code | Triggers on |
|------|-------------|
| `env_file_read_direct` | `Read` tool on `.env`, `.env.*`, `.envrc` (not `.env.example/sample/template`) |
| `env_file_read_shell` | `cat|less|head|tail|bat|grep|awk|sed|xxd|od|strings` on real .env |
| `env_file_copy` | `cp|mv|rsync|scp` where source is a real .env |
| `env_file_archive` | `tar|zip` that includes a real .env |
| `secret_var_echo` | `echo|printf $*_TOKEN / $*_SECRET / $*_PASSWORD / $*_API_KEY / $*_PRIVATE_KEY / $*_ACCESS_KEY / $*_CREDENTIAL / $*_AUTH` |
| `secret_var_grep` | `env|printenv|set | grep <secret-pattern>` |

### `verify-git-push.sh`

| Code | Triggers on |
|------|-------------|
| `git_push_force` | `git push --force` / `-f` (not `--force-with-lease`) |
| `git_push_protected` | `git push <remote> main|master|develop` |

### `block-ai-attribution.sh`

| Code | Triggers on |
|------|-------------|
| `ai_attribution_coauthored` | `Co-Authored-By: …Claude/Anthropic/GPT/OpenAI/Gemini/Google AI` in commit message |
| `ai_attribution_generated` | "Generated with Claude Code", "Generated by AI", `noreply@anthropic.com`, `noreply@openai.com` |

### `brownfield-discipline.sh`

| Code | Triggers on |
|------|-------------|
| `reference_readonly` | `Edit`/`Write` to any path inside `.reference/` |

### `protect-vp.sh`

| Code | Triggers on |
|------|-------------|
| `vp_green_immutable` | `Edit`/`Write` to `.factory/specs/verification-properties/VP-*.md` with `Status: green` |

### `protect-bc.sh`

| Code | Triggers on |
|------|-------------|
| `bc_green_immutable` | `Edit`/`Write` to `.factory/specs/behavioral-contracts/BC-*.md` with `Status: green` |

### `red-gate.sh`

| Code | Triggers on |
|------|-------------|
| `red_gate_strict_violation` | Strict mode active in `.factory/red-gate-state.json` and target source file not listed in `.red[]` |

### `factory-branch-guard.sh`

| Code | Triggers on |
|------|-------------|
| `factory_not_worktree` | `Edit`/`Write` to a `.factory/`-prefixed path when `.factory/` lacks the `.git` worktree marker |
| `factory_wrong_branch` | `Edit`/`Write` to `.factory/` when the worktree is on a branch other than `factory-artifacts` (or `factory-project-artifacts` for the `.factory-project` variant). Event also carries `current_branch` and `expected_branch`. |

### `validate-wave-gate-prerequisite.sh`

| Code | Triggers on |
|------|-------------|
| `wave_gate_prerequisite_not_passed` | `Agent` dispatch of a worker subagent (test-writer / implementer / demo-recorder / pr-manager / devops-engineer) for a story whose wave has an earlier wave with `gate_status` other than `passed`/`deferred`. Event carries `subagent`, `story_id`, `target_wave`, `blocking_wave`, `blocking_status`. |

### `validate-pr-merge-prerequisites.sh`

| Code | Triggers on |
|------|-------------|
| `pr_merge_evidence_missing` | `Agent` dispatch to `github-ops` with a "merge" prompt when `.factory/code-delivery/<STORY-ID>/` is missing `pr-description.md`, `pr-review.md`, or `security-review.md`. Event carries `story_id`, `delivery_dir`, and a comma-separated `missing` field listing which files are absent. |

### Policy validators (PostToolUse Edit|Write)

| Code | Hook | Triggers on |
|------|------|-------------|
| `policy6_subsystem_name_mismatch` | `validate-subsystem-names.sh` | BC `subsystem:` or story `subsystems:` references an SS-ID not in `ARCH-INDEX.md` Subsystem Registry |
| `policy7_bc_title_mismatch` | `validate-bc-title.sh` | BC file H1 title differs from `BC-INDEX.md` entry. Event carries `bc_id`, `h1_title`, `index_title` for quick diff. |
| `policy8_bc_array_desync` | `validate-story-bc-sync.sh` | Story frontmatter `behavioral_contracts:` ↔ body BC table ↔ AC trace annotations are out of sync |
| `policy9_vp_inconsistency` | `validate-vp-consistency.sh` | `VP-INDEX.md` ↔ `verification-architecture.md` ↔ `verification-coverage-matrix.md` have inconsistent VP IDs or column totals |

---

## Instrumenting your own hook

Five-line recipe. Copy into any new or existing hook.

### 1. Add the `_emit` helper near the top of your hook

```bash
_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}
```

The wrapper:
- No-ops when `CLAUDE_PLUGIN_ROOT` is unset (hook invoked outside Claude Code).
- No-ops when `bin/emit-event` is missing or non-executable.
- Never propagates emission failures (`2>/dev/null || true`).
- Always returns 0 so `set -e` hooks don't die on emission errors.

### 2. Call `_emit` inside your block()

```bash
block() {
  local reason="$1"
  local suggestion="${2:-}"
  local code="${3:-unknown}"
  _emit type=hook.block hook=<your-hook-name> matcher=<matcher> reason="$code" command="$COMMAND"
  echo "BLOCKED by <your-hook-name>:" >&2
  echo "  $reason" >&2
  ...
  exit 2
}
```

### 3. Pass a reason code to every `block()` call

```bash
block \
  "Short user-facing description of the problem." \
  "What the user should do instead." \
  "stable_snake_case_code"
```

### 4. Reason-code conventions

- `snake_case`, all lowercase
- Unique across all hooks (check the registry above before adding)
- Short but descriptive: `git_reset_hard` not `grh`, `git_hard_reset_discards_working_tree`
- Group by subsystem prefix: `git_*`, `gh_*`, `sot_*`, `env_file_*`

### 5. Add emission tests to your hook's `.bats` file

Use the pattern in `tests/destructive-guard.bats` under the "Emit-event
integration" section. The critical assertion is that the hook **still
blocks** when emission is broken — failure of observability must not
change block/pass behavior. Three regressions to include:
- `CLAUDE_PLUGIN_ROOT` unset → hook still blocks
- `CLAUDE_PLUGIN_ROOT` set to a nonexistent path → hook still blocks
- `VSDD_TELEMETRY=off` → hook still blocks, no event written

---

## Safety guarantees

Three independent kill paths protect the factory from instrumentation bugs.
Any one of them being tripped is enough to silence emission without
affecting hook correctness:

1. **Kill switch at line 1 of `bin/emit-event`** — `VSDD_TELEMETRY=off`
   short-circuits before any other logic runs.
2. **Per-hook `_emit` wrapper** — no-ops when `CLAUDE_PLUGIN_ROOT` is unset
   or when `bin/emit-event` is not executable. Pipe output is silently
   discarded.
3. **`emit-event` fail-closed-silent** — every internal failure path
   (missing `jq`, unwritable log dir, disk full, malformed args, clock
   failure) silently returns 0. The `bats tests/emit-event.bats` suite
   includes 35 tests that verify this contract.

Combined: if any of these three layers is broken, the other two still
succeed. You cannot construct a path where emit-event's failure propagates
into a hook's block/pass decision.

### Performance

Emission adds roughly 5-15 ms per block event (one `jq` invocation plus an
atomic append). Since emission only runs at block time — not on the
happy path — impact on normal sessions is zero.

---

## Roadmap

| Phase | Scope | Status |
|-------|-------|--------|
| 1 | `bin/emit-event` safety scaffold + tests | Shipped in [v0.56.0](../../CHANGELOG.md) |
| 2a | Instrument 4 PreToolUse Bash guards | Shipped in [v0.57.0](../../CHANGELOG.md) |
| 2b | Instrument 5 PreToolUse Edit|Write guards | Shipped in [v0.58.0](../../CHANGELOG.md) |
| 2c | Instrument 2 PreToolUse Agent guards | Shipped in [v0.59.0](../../CHANGELOG.md) |
| 2d.1 | Instrument 4 policy validators (Policy 6/7/8/9) | Shipped in [v0.60.0](../../CHANGELOG.md) |
| 2d.2 | Instrument 8 structural validators | Planned |
| 2d.3 | Instrument 10 workflow/specialized validators | Planned |
| 2e | Instrument 6 SubagentStop + Stop hooks | Planned |
| 3 | `bin/factory-query` canned queries + `bin/factory-report` | Planned |
| 4 | `/factory-health` slash command | Planned |
| 5 | Local Docker observability stack (OTel Collector + Grafana LGTM) | Planned |
| 6 | Session replay, agent SLO tracking, pipeline flame graphs | Planned |
