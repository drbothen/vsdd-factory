---
document_type: epic
epic_id: "E-8"
version: "1.10"
title: "Native WASM Migration Completion"
status: tier-1-shipped
tech_debt_ref: TD-014
prd_capabilities: [CAP-002, CAP-008, CAP-013, CAP-022]
prd_frs: []
anchor_strategy: option-c-reuse-existing-bc-per-hook-behavior
priority: P2
target_release: "v1.1 (Tier 1 — shipped rc.4); v1.2 (Tier 2 — E-9); v1.3 (Tier 3 — future E-10)"
story_count: 12
last_amended: 2026-05-03
producer: architect
timestamp: 2026-05-02T00:00:00Z
phase: 2
traces_to: .factory/tech-debt-register.md#TD-014
inputs:
  - .factory/tech-debt-register.md#TD-014
input-hash: "4ba3584"
---
<!-- [process-gap] Frontmatter fields tech_debt_ref, anchor_strategy, priority, target_release extend the canonical epic-template baseline. Template update tracked as follow-up. See Change Log v1.1 F-016. -->

# Epic E-8: Native WASM Migration Completion

## Description

Port all 42 in-scope bash scripts (43 unique scripts route via legacy-bash-adapter;
verify-git-push.sh is the 43rd, excluded per D-1; 42 ported by E-8) to native WASM
crates, retire the `legacy-bash-adapter`
transitional crate (S-3.04), and make `hooks-registry.toml` the single source of
truth for all hook registration. Eliminates Windows git-bash dependency for Claude
Code hooks and closes TD-014.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-002 | Hook Claude Code tool calls and session/worktree lifecycle events with sandboxed WASM plugins | P0 |
| CAP-008 | Gate tool calls with pre-execution behavioral checks (PreToolUse hooks) | P0 |
| CAP-013 | Capture post-execution activity (PostToolUse hooks) | P0 |
| CAP-022 | Port hook plugins from bash to native WASM | P2 |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|-----------|--------|
| S-8.00 | Perf benchmark baseline + Tier 1 BC-anchor verification | 5 | — | draft |
| S-8.01 | Native port: handoff-validator (SubagentStop) | 4 | S-8.00 | draft |
| S-8.02 | Native port: pr-manager-completion-guard (SubagentStop) | 5 | S-8.00 | draft |
| S-8.03 | Native port: track-agent-stop (SubagentStop) | 3 | S-8.00 | draft |
| S-8.04 | Native port: update-wave-state-on-merge (SubagentStop) | 4 | S-8.00 | draft |
| S-8.05 | Native port: validate-pr-review-posted (SubagentStop) | 3 | S-8.00 | draft |
| S-8.06 | Native port: session-learning (Stop) | 3 | S-8.00 | draft |
| S-8.07 | Native port: warn-pending-wave-gate (Stop) | 3 | S-8.00 | draft |
| S-8.08 | Native port: track-agent-start (PreToolUse:Agent) | 3 | S-8.00 | draft |
| S-8.09 | Native port: regression-gate + legacy-bash-adapter retirement prep | 5 | S-8.01..S-8.08 | draft |
| S-8.10 | SDK extension: host::write_file (D-6 Option A unblocker) | 3 | S-8.00 | draft |
| S-8.30 | SDK extension: HookPayload SubagentStop top-level fields | 3 | S-8.00 | draft |
| S-8.11 | Native port bundle B-1: BC/story format validators (4 hooks) | 5 | S-8.09 | draft |
| S-8.12 | Native port bundle B-2: finding/format gate validators (4 hooks) | 5 | S-8.09 | draft |
| S-8.13 | Native port bundle B-3: state file guards (4 hooks) | 5 | S-8.09 | draft |
| S-8.14 | Native port bundle B-4: release/delivery validators (4 hooks) | 5 | S-8.09 | draft |
| S-8.15 | Native port bundle B-5a: wave/template simple validators (1 hook) | 5 | S-8.09 | draft |
| S-8.16 | Native port: validate-input-hash (solo) | 5 | S-8.09 | draft |
| S-8.17 | Native port: validate-template-compliance (solo, complex) | 5 | S-8.09 | draft |
| S-8.18 | Native port bundle B-6a: convergence-tracker + purity-check | 4 | S-8.09 | draft |
| S-8.19 | Native port bundle B-6b: validate-vp-consistency + validate-anchor-capabilities-union | 6 | S-8.09 | draft |
| S-8.20 | Native port: protect-bc (PreToolUse gate) | 3 | S-8.09 | draft |
| S-8.21 | Native port: protect-vp (PreToolUse gate) | 3 | S-8.09 | draft |
| S-8.22 | Native port: protect-secrets (dual-event PreToolUse) | 4 | S-8.09 | draft |
| S-8.23 | Native port: red-gate (PreToolUse strict-TDD gate) | 5 | S-8.09 | draft |
| S-8.24 | Native port: brownfield-discipline (PreToolUse) | 4 | S-8.09 | draft |
| S-8.25 | Native port: factory-branch-guard (PreToolUse) | 4 | S-8.09 | draft |
| S-8.26 | Native port: check-factory-commit (PreToolUse:Bash) | 4 | S-8.09 | draft |
| S-8.27 | Native port: destructive-command-guard (PreToolUse:Bash) | 5 | S-8.09 | draft |
| S-8.28 | Native port: validate-pr-merge-prerequisites (PreToolUse:Agent) | 4 | S-8.09 | draft |
| S-8.29 | Native port: validate-wave-gate-prerequisite (PreToolUse:Agent) | 4 | S-8.09 | draft |

## Problem Statement

vsdd-factory v1.0.0-rc.1 ships with 43 unique bash scripts still executing through
the `legacy-bash-adapter` crate (protect-secrets is dual-registered — 1 script,
2 registry entries; verify-git-push.sh is also adapter-routed but excluded from E-8
scope per D-1; registry has 52 `[[hooks]]` entries total; 44 route via
legacy-bash-adapter, of which 43 are unique scripts). The adapter was always a
transitional compatibility layer (S-3.04, ADR-013-era), not a permanent solution.
Two execution paths for hooks now coexist:

1. **hooks-registry.toml** — 43 unique bash scripts registered as
   `plugin = "hook-plugins/legacy-bash-adapter.wasm"` with `script_path` config.
2. **hooks.json** — same scripts referenced as direct `command` entries
   (Claude Code reads this file directly on platforms without dispatcher activation).

This dual-path creates cross-platform risk: the dispatcher is required for
Windows-native operation (no git-bash dependency), but hooks.json still falls
back to direct bash execution. On Windows without git-bash, hooks silently
no-op. Additionally, the adapter is a runtime indirection that adds latency and
is itself a code-maintenance liability.

E-8 closes TD-014: port all 42 remaining bash hooks (43 unique minus verify-git-push.sh
per D-1) to native WASM, delete the legacy-bash-adapter crate, and make
hooks-registry.toml the single source of truth for all hook registration.

**Registry arithmetic:** 52 `[[hooks]]` entries → 44 route via legacy-bash-adapter
→ 43 unique scripts (protect-secrets counts as 1 script, 2 entries; verify-git-push.sh
is the 43rd unique adapter-routed script, excluded per D-1) → **42 scripts ported by
E-8**, partitioned 9/23/10 across Tiers 1/2/3.

**Scope correction vs TD-014 wording:** TD-014 states "8 dispatcher-routed +
~35 inline." Since TD-014 was written, the remaining scripts were added to
hooks-registry.toml during the brownfield backfill cycle, so ALL 43 unique scripts
are now dispatcher-routed. The "~35 inline" characterization refers to hooks.json
direct execution (still present). E-8 must eliminate both the adapter routing AND
the hooks.json inline entries for the 42 ported scripts.

---

## Goals

1. Zero bash scripts in `plugins/vsdd-factory/hooks/` (modulo verify-git-push.sh
   per D-1 below).
2. Delete `crates/hook-plugins/legacy-bash-adapter/` entirely.
3. Make `hooks-registry.toml` the single source of truth; delete all `command`
   entries from `hooks.json` except verify-git-push.sh.
4. Windows-native operation without git-bash dependency for all Claude Code hooks.
5. HOST_ABI_VERSION = 1 unchanged throughout E-8 (additive extension to host fn
   surface allowed per D-6; no version bump).
6. Aggregate PostToolUse:Edit|Write latency (sum of all 23 plugins) ≤ 500ms p95
   (per AC-7b; ceiling raised from 200ms to 500ms by S-8.00 fix-burst v1.9
   after measuring 19ms/plugin bash baseline × 23 = 437ms projected aggregate;
   warm-pool + compile-cache mitigations remain required to hit sub-500ms target);
   per-Tier-2 hook latency does not regress vs S-8.00 baseline by more than 20%
   (per AC-7; Tier 1/3 hooks fire less frequently and are not benchmarked
   individually).

**Note:** Acceptance Criteria AC-4 (release.yml bundle integration), AC-8 (bats
behavior parity), and AC-9 (bin/emit-event removal) are release/quality gates not
enumerated as strategic goals; they verify enabling work for Goals #1-#6.

## Non-Goals

- Porting verify-git-push.sh to native WASM (D-1: out of scope).
- Changing any hook's behavior during the port (behavior-parity-only migration).
- Introducing new hook events or new observability surfaces (those belong in E-5 / E-9).
- Resolving TD-001, TD-002, TD-004 BC-family cleanup (separate epics).
- Touching scripts in `scripts/` (build utilities, not Claude Code hooks).

---

## Decisions

### D-1: Tier 4 Scope (verify-git-push.sh)

**Decision: Option B — explicit out-of-scope; remains bash.**

verify-git-push.sh guards `git push` commands issued by Claude Code via the Bash
tool. It is a Claude Code PreToolUse:Bash hook, NOT a git pre-push hook, so the
"different runtime" rationale does not apply as directly as the prompt context
implies. However, the hook's logic is deliberately bash-centric (parses raw
command strings, simple grep patterns, exits 0/2).

Rationale for exclusion:
- (a) It fires only on `git push` Bash invocations from Claude Code, which already
  require git-bash on Windows for git itself.
- (b) Windows-native users running `git push` from non-bash shells are out-of-scope
  per the v1.0 support matrix (see OQ-7 — no canonical support matrix doc located;
  rationale (a, c, d) are sufficient independently; OQ-7 tracks whether a
  canonical doc needs to be created or whether this appeal should be dropped).
- (c) The script's logic is local-pre-push-validation, not Claude-Code-event-routing
  — porting it to WASM provides no platform parity benefit because its failure mode
  (missing bash → hook skipped → push proceeds) is tolerable.
- (d) Command-string parsing in Rust requires regex or argument tokenization with
  careful quoting semantics — non-trivial porting cost.

**Ruling:** verify-git-push.sh stays bash, stays in hooks.json as a direct
`command` entry, and is explicitly noted as the sole remaining .sh in
`plugins/vsdd-factory/hooks/` after E-8 completes. This is documented in E-8
AC-1. Future E-9+ may revisit if Windows parity becomes a requirement.

Note on disposition: verify-git-push.sh's existing `[[hooks]]` registry entry
(currently adapter-routed for backward compat with the dispatcher) is REMOVED at
S-8.29 close as part of legacy-bash-adapter retirement. Its hooks.json direct
command entry is RETAINED. Post-E-8 state: 1 hooks.json direct entry, 0
hooks-registry.toml entries for verify-git-push.sh.

### D-2: BC Anchor Strategy

**Decision: Option C — reuse existing BCs; no new BC family.**

E-8 is a substrate migration, not a behavior change. The behaviors of
handoff-validator, validate-bc-title, red-gate, etc. are already specified in the
existing BC-7.xx families. Creating new BCs for "the WASM version of
validate-bc-title" would duplicate behavioral specs that are already correct and
already green.

Per-story obligation under Option C:
- Each story includes a "BC-anchor verification" task: identify the BC(s) the
  hook satisfies, confirm the spec is current, port only the implementation.
- If a port reveals behavior the bash version implements but no BC specifies
  (implicit behavior), a new BC is written for that behavior before the story
  merges. This is the only condition under which new BCs are created in E-8.
- New BCs (if any) go under the existing BC-7.xx family for the relevant hook,
  not under a new BC-7.02.x migration family.

Story-writer will identify the BC anchor(s) per story during story decomposition.

### D-3: Tier Definition and Hook Count

Exact counts from `hooks-registry.toml` (2026-04-30, develop @ HEAD):

**Registry arithmetic (empirically verified 2026-04-30 from hooks-registry.toml):**
- 52 `[[hooks]]` entries total in hooks-registry.toml
- 44 entries route via `legacy-bash-adapter.wasm`
- protect-secrets is dual-registered (1 script, 2 entries) → 43 unique bash scripts
- verify-git-push.sh is the 43rd unique adapter-routed script; excluded per D-1 (stays bash)
- → **42 scripts ported by E-8** (Tier 1: 9 + Tier 2: 23 + Tier 3: 10 = 42 in-scope)

**Tier 1 — SubagentStop + Stop lifecycle hooks (9 hooks):**
These hooks fire on agent lifecycle events. They are the original "8 dispatcher-
routed" from TD-014's writing, plus track-agent-start (PreToolUse:Agent —
logically pairs with track-agent-stop; same telemetry pattern; bundling reduces
cross-story sync complexity).

| Hook | Event | On-Error | Complexity |
|------|-------|----------|------------|
| handoff-validator | SubagentStop | block | Medium (JSON parse + threshold logic) |
| pr-manager-completion-guard | SubagentStop | block | Medium (gh CLI subprocess) |
| track-agent-stop | SubagentStop | continue | Low (telemetry write) |
| update-wave-state-on-merge | SubagentStop | continue | Medium (state file mutation) |
| validate-pr-review-posted | SubagentStop | continue | Low (gh CLI query) |
| session-learning | Stop | continue | Low (file append) |
| warn-pending-wave-gate | Stop | continue | Low (state file read + warn) |
| track-agent-start | PreToolUse:Agent | continue | Low (telemetry write) |
| regression-gate | PostToolUse | continue | Medium (bats/test runner) |

Note: regression-gate fires on PostToolUse (no tool filter) — grouping with Tier 1
because it was one of the original 8 dispatcher-routed hooks and shares the
"lifecycle gate" pattern. track-agent-start is grouped with Tier 1 because it is
the lifecycle pair of track-agent-stop; both implement agent lifecycle telemetry;
bundling reduces cross-story dependency complexity.

**Tier 2 — PostToolUse:Edit|Write validators (23 hooks):**
These fire on every file write. They implement VSDD spec-format invariants.
All 23 hooks are `tool = "Edit|Write"` scoped in hooks-registry.toml.

> **Block-mode callout:** 3 of 23 Tier 2 validators use `on_error = "block"`:
> validate-factory-path-root, validate-input-hash, and validate-template-compliance.
> These MUST have additional negative (false-block) test fixtures in their story
> ACs. See AC-8.

| Hook | Complexity | Bundle Group |
|------|------------|-------------|
| validate-bc-title | Low (grep + awk) | B-1: BC/story format |
| validate-story-bc-sync | Low (frontmatter grep) | B-1 |
| validate-index-self-reference | Low (grep) | B-1 |
| validate-subsystem-names | Low (grep against registry) | B-1 |
| validate-finding-format | Low (regex) | B-2: finding/format gates |
| validate-factory-path-root | Low (path prefix check) | B-2 |
| validate-novelty-assessment | Low (frontmatter check) | B-2 |
| validate-table-cell-count | Low (pipe-count per row) | B-2 |
| validate-state-size | Low (wc + git diff) | B-3: state file guards |
| validate-state-pin-freshness | Medium (git log parsing) | B-3 |
| validate-state-index-status-coherence | Medium (cross-file grep) | B-3 |
| validate-count-propagation | Medium (multi-file count comparison) | B-3 |
| validate-changelog-monotonicity | Low (date comparison) | B-4: release/delivery |
| validate-demo-evidence-story-scoped | Low (frontmatter grep) | B-4 |
| validate-pr-description-completeness | Low (grep on PR markdown) | B-4 |
| validate-red-ratio | Low (log parse + arithmetic) | B-4 |
| validate-wave-gate-completeness | Medium (YAML cross-file check) | B-5: wave/template |
| validate-input-hash | Medium (sha computation) | B-5 (solo story) |
| validate-template-compliance | High (multi-rule, multi-file) | B-5 (solo story) |
| convergence-tracker | Medium (state mutation + metrics) | B-6: complex solo |
| purity-check | Medium (frontmatter + path check) | B-6 |
| validate-vp-consistency | High (VP-INDEX cross-ref) | B-6 |
| validate-anchor-capabilities-union | High (BC graph traversal) | B-6 |

Note: All 23 PostToolUse hooks are Edit|Write-scoped per registry confirmation.
convergence-tracker, purity-check, validate-vp-consistency, and
validate-anchor-capabilities-union all have `tool = "Edit|Write"` in
hooks-registry.toml. regression-gate is PostToolUse (ungated) and is grouped in
Tier 1. Bundle group B-3 merges the former B-3a and B-3b (see D-8 / F-017 fix).

**Tier 3 — PreToolUse protections + process discipline (10 hooks):**
These fire before tool execution and can block. Behavioral correctness is
safety-critical: a mis-ported PreToolUse hook that over-blocks destroys developer
productivity; one that under-blocks leaks protected artifacts.

| Hook | Event | Complexity | Notes |
|------|-------|------------|-------|
| protect-bc | PreToolUse:Edit\|Write | Medium | permissionDecision envelope |
| protect-vp | PreToolUse:Edit\|Write | Medium | permissionDecision envelope |
| protect-secrets | PreToolUse:Bash+Read | Medium | dual-event registration |
| red-gate | PreToolUse:Edit\|Write | High | state-file + path matching |
| brownfield-discipline | PreToolUse:Edit\|Write | Medium | path allowlist check |
| factory-branch-guard | PreToolUse:Edit\|Write | Medium | git branch check |
| check-factory-commit | PreToolUse:Bash | Medium | git log analysis |
| destructive-command-guard | PreToolUse:Bash | High | command tokenization |
| validate-pr-merge-prerequisites | PreToolUse:Agent | Medium (gh CLI) | SubagentStop-adjacent |
| validate-wave-gate-prerequisite | PreToolUse:Agent | Medium | state-file read |

Note: protect-secrets has two registry entries (Bash + Read events) — counts as
1 WASM plugin with 2 registrations in hooks-registry.toml. Total Tier 3: 10 unique
scripts (protect-secrets counts once).

**Tier 4 (Out of Scope):**
- verify-git-push.sh — remains bash per D-1.

**Revised Totals:**
- Tier 1: 9 hooks
- Tier 2: 23 hooks (all Edit|Write-scoped)
- Tier 3: 10 hooks
- **Scope total: 42 bash scripts ported by E-8**

### D-4: Sequencing

**Decision: Parallel after Tier 1.**

Tier 1 lands first (v1.1). It proves the full end-to-end pattern:
1. Write Rust WASM crate in `crates/hook-plugins/<name>/`.
2. Register `.wasm` path in hooks-registry.toml (replace `legacy-bash-adapter.wasm`
   + `script_path`).
3. Delete corresponding hooks.json `command` entry.
4. Delete `.sh` file from `plugins/vsdd-factory/hooks/`.
5. Verify bats tests pass.

Once Tier 1 demonstrates the pattern and the adapter retirement prep is complete,
Tiers 2 and 3 are parallel. Tier 2 hooks are PostToolUse validators (no dependency
on Tier 3 PreToolUse hooks). Tier 3 hooks are independent of Tier 2. Both can be
developed in concurrent feature branches.

Tier 2 and Tier 3 stories share no implementation dependencies. They may be
assigned to concurrent worktrees in the same wave.

### D-5: Tier-to-Release Mapping

**Decision: Option A — one MINOR per tier.**

| Tier | Release | Milestone |
|------|---------|-----------|
| Tier 1 (9 hooks) | v1.1 | legacy-bash-adapter crate deleted |
| Tier 2 (23 hooks) | v1.2 | All PostToolUse validators native |
| Tier 3 (10 hooks) | v1.3 | All PreToolUse protections native; 0 bash hooks |

Rationale: visible milestones per tier enable checkpoint releases. v1.1 is the
most impactful (adapter retirement). v1.2 covers the highest-frequency hooks
(every file write triggers 23 validators — WASM startup budget matters most here).
v1.3 completes the migration.

Alternative considered (Option B — all in v1.1): rejected. 42 hooks in one MINOR
is too large to converge adversarially; batched release means a Tier 3 regression
blocks a Tier 1 fix from shipping.

### D-6: ABI Evolution Policy

**Decision: Option A first (additive extension), Option C second (refactor to
avoid), Option B never (no v1.x ABI bump).**

HOST_ABI_VERSION = 1 is frozen per S-5.06. The porting process may reveal missing
host functions (e.g., a hook that needs a `exec_subprocess` variant not currently
in the SDK, or a `read_directory` host fn). Procedure when this occurs:

1. **Characterize the need:** Is the missing capability expressible with existing
   host functions? (Composition of `exec_subprocess` + `read_file` can substitute
   for most cases.)
2. **Option C first:** Refactor the port design to avoid the new fn. Most bash
   patterns (jq/grep/awk pipelines) can be rewritten as pure Rust with no new
   host calls.
3. **Option A if C is not practical:** Add the host fn additively to
   `vsdd_hook_sdk::host::*`. This is ABI-stable per the semver-commitment doc
   ("Adding a new host function to `vsdd_hook_sdk::host::*` is not a breaking
   change"). The story adding the new fn must:
   - Get adversarial review before implementation.
   - Land the dispatcher-side registration and SDK-side export in the same PR.
   - Verify the ABI version constant stays at 1.
4. **Option B never in v1.x:** Bumping HOST_ABI_VERSION requires a major version
   bump. This is explicitly disallowed during the E-8 migration.

Stories must document any ABI extension requests as an Open Question for adversarial
review. If three or more ports in the same tier each independently need new host
fns, this is an escalation signal: pause and design a batch extension rather than
accreting one fn per story.

### D-7: hooks.json Post-E-8 Shape and Dispatcher Routing

**Decision: Option C — register all matchers in hooks-registry.toml; delete
all inline command entries from hooks.json; dispatcher binary is the sole
hooks.json routing entry per event/matcher tuple.**

**Dispatcher routing decision:** hooks.json post-E-8 contains exactly one
dispatcher-routing entry per (event, matcher) tuple, invoking the factory-dispatcher
binary. The dispatcher then consults hooks-registry.toml for routing to native
plugins. Native WASM plugins do NOT register directly in hooks.json — only the
dispatcher binary does. This is the existing DRIFT-004 architecture intent.

**BEFORE (pre-E-8) — one inline command entry per bash script, grouped by event then matcher:**

```json
// hooks.json — multiple inline command entries per event/matcher group:
{
  "PostToolUse": [
    {
      "matcher": "Edit|Write",
      "hooks": [
        { "type": "command", "command": "/path/to/validate-bc-title.sh" },
        { "type": "command", "command": "/path/to/validate-finding-format.sh" }
      ]
    }
  ],
  "PreToolUse": [
    {
      "matcher": "Edit|Write",
      "hooks": [
        { "type": "command", "command": "/path/to/protect-bc.sh" }
      ]
    },
    {
      "matcher": "Bash",
      "hooks": [
        { "type": "command", "command": "/path/to/protect-secrets.sh" }
      ]
    }
  ]
}
// Result: N inline entries per (event, matcher) group — one per bash script.
// validate-bc-title/validate-finding-format are PostToolUse; protect-bc/protect-secrets are PreToolUse.
```

**AFTER (post-E-8) — single dispatcher entry per (event, matcher) tuple:**

```json
// hooks.json — dispatcher-routing entries only; no per-script commands:
{
  "PostToolUse": [
    {
      "matcher": "Edit|Write",
      "hooks": [
        { "type": "command", "command": "/path/to/factory-dispatcher" }
      ]
    }
  ],
  "PreToolUse": [
    {
      "matcher": "Edit|Write",
      "hooks": [
        { "type": "command", "command": "/path/to/factory-dispatcher" }
      ]
    },
    {
      "matcher": "Bash",
      "hooks": [
        { "type": "command", "command": "/path/to/factory-dispatcher" },
        { "type": "command", "command": "/path/to/verify-git-push.sh" }
      ]
    }
  ]
}
// The N BEFORE entries per (event, matcher) group collapse into ONE dispatcher
// invocation. The dispatcher reads hooks-registry.toml and routes to all native
// WASM plugins registered for that event/matcher.
// Exception: verify-git-push.sh retains a direct command entry alongside the
// dispatcher under PreToolUse "matcher": "Bash" (shown above) per D-1.
```

Native WASM plugins are invoked by the dispatcher binary, which reads
hooks-registry.toml at startup. No per-plugin entry appears in hooks.json.

When a bash script is ported to native WASM, its hooks.json entry is deleted and
the hooks-registry.toml entry is updated from `plugin = "hook-plugins/legacy-bash-adapter.wasm"`
with `script_path` config to `plugin = "hook-plugins/<name>.wasm"`. The native
plugin handles its own event filtering internally where granular matcher semantics
are needed (e.g., protect-secrets fires on both Bash and Read tool events — the
WASM plugin can check `tool_name` from context, or we keep two registry entries
as today).

After E-8 completes: hooks.json contains ZERO inline `command` entries for native
WASM hooks. The file contains only:
1. Dispatcher-routing entries (one per (event, matcher) tuple).
2. The verify-git-push.sh direct command entry (D-1).

Zero `[[hooks]]` entries in hooks-registry.toml use `plugin = 'hook-plugins/legacy-bash-adapter.wasm'`
after S-8.29 merges.

Reference: DRIFT-004 (hooks.json + hooks-registry.toml dual routing tables —
MEDIUM-HIGH, resolution target L-P0-002 cutover).

### D-8: Per-Story Sizing and Bundling

**Decision: Tier 1 = one story per hook; Tier 2 = bundled by similarity (9
stories after B-3 merge); Tier 3 = one story per hook.**

**Tier 1 (10 stories: 1 pre-work + 9 hook ports, ~3-5 pts each):**
One story per hook, plus S-8.00 pre-work. Each story proves the full port+delete+test cycle.

| Story ID | Hook | Est. Pts |
|----------|------|---------|
| S-8.00 | Perf benchmark baseline + Tier 1 BC-anchor verification | 5 |
| S-8.01 | handoff-validator | 4 |
| S-8.02 | pr-manager-completion-guard | 5 |
| S-8.03 | track-agent-stop | 3 |
| S-8.04 | update-wave-state-on-merge | 4 |
| S-8.05 | validate-pr-review-posted | 3 |
| S-8.06 | session-learning | 3 |
| S-8.07 | warn-pending-wave-gate | 3 |
| S-8.08 | track-agent-start | 3 |
| S-8.09 | regression-gate | 5 |

**Tier 2 (9 stories, ~4-6 pts each):**
Bundled by validator pattern. B-3a and B-3b merged into a single B-3 bundle (4
hooks, 5 pts) — split was arbitrary; all 4 hooks are state-file validators with
similar implementation approach.

| Story ID | Bundle | Hooks Included | Est. Pts |
|----------|--------|---------------|---------|
| S-8.11 | B-1: BC/story format | validate-bc-title, validate-story-bc-sync, validate-index-self-reference, validate-subsystem-names | 5 |
| S-8.12 | B-2: finding/format gates | validate-finding-format, validate-factory-path-root, validate-novelty-assessment, validate-table-cell-count | 5 |
| S-8.13 | B-3: state file guards (merged) | validate-state-size, validate-state-pin-freshness, validate-state-index-status-coherence, validate-count-propagation | 5 |
| S-8.14 | B-4: release/delivery | validate-changelog-monotonicity, validate-demo-evidence-story-scoped, validate-pr-description-completeness, validate-red-ratio | 5 |
| S-8.15 | B-5a: wave/template simple | validate-wave-gate-completeness | 5 |
| S-8.16 | B-5b: input-hash (solo) | validate-input-hash | 5 |
| S-8.17 | B-5c: template compliance (solo) | validate-template-compliance | 5 |
| S-8.18 | B-6a: convergence/purity | convergence-tracker, purity-check | 4 |
| S-8.19 | B-6b: complex validators | validate-vp-consistency, validate-anchor-capabilities-union | 6 |

**Tier 3 (10 stories, ~3-5 pts each):**
One story per hook due to logic complexity and safety-critical nature.

| Story ID | Hook | Est. Pts |
|----------|------|---------|
| S-8.20 | protect-bc | 3 |
| S-8.21 | protect-vp | 3 |
| S-8.22 | protect-secrets (dual-event) | 4 |
| S-8.23 | red-gate | 5 |
| S-8.24 | brownfield-discipline | 4 |
| S-8.25 | factory-branch-guard | 4 |
| S-8.26 | check-factory-commit | 4 |
| S-8.27 | destructive-command-guard | 5 |
| S-8.28 | validate-pr-merge-prerequisites | 4 |
| S-8.29 | validate-wave-gate-prerequisite | 4 |

**Total: 29 stories (~123-155 story points; 123 base sum, 32 reserved for BC-creation buffer per R-8.10).**

### D-9: Documentation Correction (S-5.05 "26 hooks" claim)

**Decision: Option B — v1.0.0 GA release notes amendment.**

The S-5.05 migration guide line ~62 states "Other 26 hooks remain on the
legacy-bash-adapter." The actual count at v1.0.0 GA is 43 unique bash scripts all
routed via legacy-bash-adapter. The "26" was likely a snapshot count at a specific
point in the brownfield-backfill cycle before the remaining scripts were added to
the registry.

Correction: add a release-notes entry to the v1.0.0 GA CHANGELOG:

> "The v0.79→v1.0 migration guide's reference to '26 hooks remaining on
> legacy-bash-adapter' reflects a mid-cycle snapshot. At v1.0.0 GA, the actual
> count is 43 unique bash scripts all routed via legacy-bash-adapter (9 lifecycle,
> 23 PostToolUse validators, 10 PreToolUse protections, 1 out-of-scope). All 42
> in-scope scripts are scheduled for native WASM migration in E-8 (v1.1–v1.3)."

No changes to the S-5.05 spec file itself.

### D-10: Legacy-Bash-Adapter Retirement

**Decision: Adapter crate stays through end of W-17; deleted at S-8.29 close.**

Rationale: 34 Tier 2/3 entries (33 unique scripts; protect-secrets dual-registered
for Bash + Read events) reference `legacy-bash-adapter.wasm` in
hooks-registry.toml during the W-16/W-17 migration window (S-8.11..S-8.29 in
flight). Deleting the adapter .wasm at end of W-15 (after S-8.09) leaves 34
dangling registry references → silent dispatch failures for any hook whose
hooks-registry.toml entry still points to the adapter .wasm. Dispatcher loads the
registry at startup and will fail to find the plugin for each un-ported hook.

Timeline:
- **End of W-15 (S-8.09 close):** S-8.09 completes the regression-gate port AND
  runs a pre-retirement audit confirming all 9 Tier 1 hooks are native WASM. The
  adapter crate is NOT deleted yet. Registry updated: 9 Tier 1 entries now point
  to native plugins; 34 Tier 2/3 entries (33 unique scripts; protect-secrets
  dual-registered for Bash + Read events) still point to adapter.
- **W-16/W-17 (S-8.11..S-8.28):** Each story updates the registry entry for its
  hooks from adapter to native. Adapter crate remains on disk; adapter .wasm must
  exist for all un-ported hooks.
- **S-8.29 close:** Last Tier 3 hook ported. Pre-deletion audit: confirm zero
  `[[hooks]]` entries in hooks-registry.toml use `plugin = 'hook-plugins/legacy-bash-adapter.wasm'`.
  Then: `crates/hook-plugins/legacy-bash-adapter/` directory deleted.
  `bin/emit-event` removed from dispatcher binary tree in the same PR (see R-8.07).
  AC-2 and AC-3 are fully satisfied at this point.

Note: the prior D-10 (retire at end of Tier 1) was incorrect — it produced 34
dangling registry references (33 unique scripts + 1 protect-secrets second
registration). Corrected reasoning: adapter must outlive every hook that
still needs it.

### D-11: Risk Register

| ID | Risk | Severity | Likelihood | Mitigation |
|----|------|----------|------------|------------|
| R-8.01 | Cross-platform parity regression — ported hook misbehaves on Windows where bash version skipped silently | HIGH | MEDIUM | CI Windows runner required for all Tier 1 stories; benchmark matrix in AC-5 |
| R-8.02 | Performance regression — WASM startup overhead exceeds 100ms NFR for high-frequency PostToolUse hooks | HIGH | MEDIUM | Benchmark added per AC-7; S-8.00 establishes bash baseline; Tier 2 stories must pass perf gate before merge; wasmtime startup profile reviewed; warm-pool + compile-cache mitigations described under R-8.08 (line ~566) |
| R-8.03 | ABI extension cascade — porting reveals multiple missing host fns; ABI grows uncontrolled | MEDIUM | LOW | D-6 procedure enforced; pause + batch design if 3+ ports need new fns |
| R-8.04 | Behavior-change drift during port — implementer "improves" bash logic during translation | HIGH | MEDIUM | D-2 (Option C): behavior spec is the BC, not the bash source; adversary explicitly checks behavior parity against bash source + BC |
| R-8.05 | Test coverage gap — bash hooks have implicit behaviors not covered by current bats tests; port forces explicit test writing which surfaces latent bugs | MEDIUM | HIGH | Surfaced latent bugs are fixed in the porting story (same PR); not deferred |
| R-8.06 | Inline matcher migration creates registration churn — simultaneous hooks.json + hooks-registry.toml edits required per hook; merge conflicts likely in active development | LOW | MEDIUM | Each story is a discrete branch; hooks.json and hooks-registry.toml edits are atomic per story |
| R-8.07 | TD-007 interaction — bash hooks still call `bin/emit-event` binary; ported hooks should use `host::emit_event` instead; if bin/emit-event is removed before all 42 ports complete, event emission breaks for remaining bash hooks | HIGH | HIGH | bin/emit-event is NOT removed until S-8.29 close. Tiers 2/3 bash hooks alive in hooks.json direct path during W-16/W-17 still need bin/emit-event. Validated by AC-9. |
| R-8.08 | Cumulative WASM startup overhead — S-8.00 measured 19ms/plugin bash median × 23 = 437ms projected aggregate PostToolUse:Edit\|Write latency; confirmed the 200ms estimate was too optimistic | HIGH | HIGH | OQ-8 resolved by S-8.00 measurement. New AC-7b ceiling = 500ms p95 (v1.9). Required mitigations: plugin warm-pool, shared wasmtime engine instance, compile-cache (.wasm → .cwasm). Without mitigations, WASM overhead may exceed 437ms bash baseline. Tier 2 gate must include warm-pool instrumented benchmark before W-16 starts. |
| R-8.09 | Cumulative bundle size growth — 42 new .wasm artifacts in dispatcher binary bundles increase release-artifact size by estimated low-MB; could approach GitHub Release per-asset size limits over multiple tier releases | LOW | LOW | Per-bundle size measured in S-8.00 baseline; if growth exceeds 25% of v1.0.0 dispatcher bundle size, evaluate WASM size-optimization (`opt-level = "z"` in workspace `[profile.release]`, strip-debug, link-time optimization). Tier-by-tier release pacing (v1.1, v1.2, v1.3) gives natural milestones to reassess. |
| R-8.10 | BC-creation explosion — D-2 Option C exception path adds new BCs for hooks with implicit behaviors not in existing BCs; magnitude depends on S-8.00 audit; worst case ~9 new BCs × ~3 pts = 27 unbudgeted pts | MEDIUM | MEDIUM | S-8.00 audit measures BC-coverage gap pre-W-15; if >5 Tier 1 hooks lack BC, raise OQ for adversarial review and consider deferring some Tier 1 ports to W-16; new BCs follow standard BC creation flow (template + adversary review). |

### D-12: Epic-Level Acceptance Criteria

| ID | Criterion | Validation |
|----|-----------|-----------|
| AC-1 | Zero `.sh` files in `plugins/vsdd-factory/hooks/` except `verify-git-push.sh` | `find plugins/vsdd-factory/hooks -name "*.sh" \| grep -v verify-git-push` returns empty |
| AC-2 | `crates/hook-plugins/legacy-bash-adapter/` directory DELETED | `ls crates/hook-plugins/` does not contain `legacy-bash-adapter`; validated at S-8.29 close |
| AC-3 | Zero per-script `command` entries in `hooks.json` (only dispatcher-routing entries remain); zero `[[hooks]]` entries in `hooks-registry.toml` use `plugin = 'hook-plugins/legacy-bash-adapter.wasm'` | `jq` query on hooks.json returns only dispatcher entries + verify-git-push; grep on hooks-registry.toml returns zero legacy-bash-adapter references |
| AC-4 | All native plugins ship through dispatcher binary bundles (release.yml builds them) | CI `release.yml` job includes all new crates in the bundle matrix |
| AC-5 | Windows native operation verified — all migrated hooks run without git-bash | CI Windows runner passes all bats integration tests for Tier 1+ hooks |
| AC-6 | `HOST_ABI_VERSION = 1` in both dispatcher and SDK — unchanged | `grep HOST_ABI_VERSION crates/factory-dispatcher/src/lib.rs` + `crates/vsdd-hook-sdk/src/lib.rs` both = 1 |
| AC-7 | Per-Tier-2 hook latency does not regress vs S-8.00 bash baseline by more than 20% (Tier 1/3 hooks fire less frequently and are not benchmarked individually) | Benchmark test in `tests/perf/` measures each Tier 2 hook vs S-8.00 bash baseline |
| AC-7b | Aggregate PostToolUse:Edit\|Write latency (sum of all 23 plugins) ≤ 500ms p95 (ceiling raised from 200ms to 500ms by S-8.00 fix-burst v1.9; S-8.00 measured 19ms/plugin bash median × 23 = 437ms projected aggregate; warm-pool + compile-cache mitigations required; OQ-8 resolved) | Benchmark test in `tests/perf/` measures aggregate latency under simulated file-write load |
| AC-8 | Behavior parity per hook — bats tests pass for native version; validate-factory-path-root, validate-input-hash, validate-template-compliance additionally have negative (false-block) test fixtures | bats test suite passes with identical output; 3 block-mode hooks have explicit negative test scenarios |
| AC-9 | `bin/emit-event` binary not present in dispatcher binary tree post-S-8.29 | `find . -name emit-event` returns empty in the dispatcher binary bundle directory after S-8.29 merge; validated in S-8.29 pre-deletion audit |

### D-13: Wave Structure

E-8 uses three waves aligned to releases:

| Wave | Tier | Stories | Target Release | Gate Condition |
|------|------|---------|---------------|----------------|
| W-15* | Tier 1 (lifecycle hooks) + SDK extension | S-8.00..S-8.10 | v1.1 | legacy-bash-adapter pre-retirement audit passed; 9 .sh gone; Windows CI green; bin/emit-event deferred to S-8.29 — **SHIPPED rc.4 (develop@d49f33b)** |
| ~~W-16* Tier 2 (PostToolUse validators): S-8.11..S-8.19~~ | — | — | SUPERSEDED 2026-05-03 — see CHANGELOG v1.10; redirected to E-9 |
| ~~W-17* Tier 3 (PreToolUse protections): S-8.20..S-8.29~~ | — | — | SUPERSEDED 2026-05-03 — see CHANGELOG v1.10; redirected to future E-10 |

*Wave IDs are provisional. Final assignment by state-manager at story decomposition
time after v1.0.0 GA. STATE.md shows Waves W-10 and W-12 are gap-numbered
(W-10 absent, W-12 present). Actual next available wave after S-5.07 close may
differ from W-15/16/17. S-5.07 (Tier H, calendar-gated) may consume the next
free wave first.

[process-gap] STATE.md should track a `next_free_wave_id` field to prevent manual
gap-counting errors during wave assignment.

> **D-13 Tier 2/3 superseded (v1.10, 2026-05-03):** The W-16 and W-17 wave-plan
> rows in D-13 are superseded. W-16 Tier 2 work is re-decomposed into E-9 (authored
> 2026-05-03). W-17 Tier 3 work will be re-decomposed into a future E-10. D-13 now
> scopes only the Tier 1 wave (W-15) which shipped via v1.0.0-rc.3 + rc.4.
> Retired placeholder stories S-8.11..S-8.29 remain in STORY-INDEX as
> `status: retired` with `superseded_by: E-9 / future E-10` per POLICY 1.

W-16 Tier 2 is handled by E-9. W-17 Tier 3 is deferred to future E-10.

---

## Scope: Complete Hook Inventory

### Tier 1 — Lifecycle Hooks (W-15, v1.1)

| Hook | Event | On-Error | Source Lines | Story |
|------|-------|----------|-------------|-------|
| handoff-validator.sh | SubagentStop | block | ~55 | S-8.01 |
| pr-manager-completion-guard.sh | SubagentStop | block | ~60 | S-8.02 |
| track-agent-stop.sh | SubagentStop | continue | ~40 | S-8.03 |
| update-wave-state-on-merge.sh | SubagentStop | continue | ~50 | S-8.04 |
| validate-pr-review-posted.sh | SubagentStop | continue | ~45 | S-8.05 |
| session-learning.sh | Stop | continue | ~35 | S-8.06 |
| warn-pending-wave-gate.sh | Stop | continue | ~40 | S-8.07 |
| track-agent-start.sh | PreToolUse:Agent | continue | ~40 | S-8.08 |
| regression-gate.sh | PostToolUse | continue | ~50 | S-8.09 |

### Tier 2 — PostToolUse Validators (W-16, v1.2)

| Hook | Matcher | On-Error | Bundle | Story |
|------|---------|----------|--------|-------|
| validate-bc-title.sh | Edit\|Write | continue | B-1 | S-8.11 |
| validate-story-bc-sync.sh | Edit\|Write | continue | B-1 | S-8.11 |
| validate-index-self-reference.sh | Edit\|Write | continue | B-1 | S-8.11 |
| validate-subsystem-names.sh | Edit\|Write | continue | B-1 | S-8.11 |
| validate-finding-format.sh | Edit\|Write | continue | B-2 | S-8.12 |
| validate-factory-path-root.sh | Edit\|Write | block | B-2 | S-8.12 |
| validate-novelty-assessment.sh | Edit\|Write | continue | B-2 | S-8.12 |
| validate-table-cell-count.sh | Edit\|Write | continue | B-2 | S-8.12 |
| validate-state-size.sh | Edit\|Write | continue | B-3 | S-8.13 |
| validate-state-pin-freshness.sh | Edit\|Write | continue | B-3 | S-8.13 |
| validate-state-index-status-coherence.sh | Edit\|Write | continue | B-3 | S-8.13 |
| validate-count-propagation.sh | Edit\|Write | continue | B-3 | S-8.13 |
| validate-changelog-monotonicity.sh | Edit\|Write | continue | B-4 | S-8.14 |
| validate-demo-evidence-story-scoped.sh | Edit\|Write | continue | B-4 | S-8.14 |
| validate-pr-description-completeness.sh | Edit\|Write | continue | B-4 | S-8.14 |
| validate-red-ratio.sh | Edit\|Write | continue | B-4 | S-8.14 |
| validate-wave-gate-completeness.sh | Edit\|Write | continue | B-5a | S-8.15 |
| validate-input-hash.sh | Edit\|Write | block | B-5b solo | S-8.16 |
| validate-template-compliance.sh | Edit\|Write | block | B-5c solo | S-8.17 |
| convergence-tracker.sh | Edit\|Write | continue | B-6a | S-8.18 |
| purity-check.sh | Edit\|Write | continue | B-6a | S-8.18 |
| validate-vp-consistency.sh | Edit\|Write | continue | B-6b | S-8.19 |
| validate-anchor-capabilities-union.sh | Edit\|Write | continue | B-6b | S-8.19 |

### Tier 3 — PreToolUse Protections (W-17, v1.3)

| Hook | Event | On-Error | Complexity | Story |
|------|-------|----------|------------|-------|
| protect-bc.sh | PreToolUse:Edit\|Write | block | Medium | S-8.20 |
| protect-vp.sh | PreToolUse:Edit\|Write | block | Medium | S-8.21 |
| protect-secrets.sh | PreToolUse:Bash+Read | block | Medium | S-8.22 |
| red-gate.sh | PreToolUse:Edit\|Write | block | High | S-8.23 |
| brownfield-discipline.sh | PreToolUse:Edit\|Write | block | Medium | S-8.24 |
| factory-branch-guard.sh | PreToolUse:Edit\|Write | block | Medium | S-8.25 |
| check-factory-commit.sh | PreToolUse:Bash | block | Medium | S-8.26 |
| destructive-command-guard.sh | PreToolUse:Bash | block | High | S-8.27 |
| validate-pr-merge-prerequisites.sh | PreToolUse:Agent | block | Medium | S-8.28 |
| validate-wave-gate-prerequisite.sh | PreToolUse:Agent | block | Medium | S-8.29 |

### Out of Scope

| Hook | Reason |
|------|--------|
| verify-git-push.sh | D-1: stays bash; Windows git-bash prerequisite for git itself; command-string parsing provides marginal WASM value; non-bash Windows users out-of-scope per D-1 rationale (a, c, d); see OQ-7 for support matrix doc status |

---

## Story Decomposition Sketch

Story-writer dispatches individual story spec files downstream. The following
is the decomposition plan for story-writer consumption:

**Wave W-15 (Tier 1 — 10 stories including S-8.00 pre-work):**
- S-8.00: Perf benchmark baseline + Tier 1 BC-anchor verification (pre-work; runs
  BEFORE S-8.01..S-8.08 begin; establishes bash latency baseline for AC-7/AC-7b;
  produces BC-anchor verification table for all 9 Tier 1 hooks; if any hooks lack
  BC coverage, S-8.00 creates the missing BCs per Option C exception path in D-2;
  point estimates for S-8.01..S-8.08 may be bumped if BC creation is required)
- S-8.01: Native port of handoff-validator (SubagentStop)
- S-8.02: Native port of pr-manager-completion-guard (SubagentStop, gh CLI)
- S-8.03: Native port of track-agent-stop (SubagentStop telemetry)
- S-8.04: Native port of update-wave-state-on-merge (SubagentStop, state mutation)
- S-8.05: Native port of validate-pr-review-posted (SubagentStop, gh CLI)
- S-8.06: Native port of session-learning (Stop)
- S-8.07: Native port of warn-pending-wave-gate (Stop)
- S-8.08: Native port of track-agent-start (PreToolUse:Agent telemetry)
- S-8.09: Native port of regression-gate (PostToolUse) + adapter pre-retirement
  audit (confirm 9 Tier 1 entries now native; adapter NOT deleted; bin/emit-event
  NOT removed — both deferred to S-8.29)

**Wave W-16 (Tier 2 — 9 stories):**
- S-8.11 through S-8.19 as per bundle groups above. Adapter crate remains on disk
  for un-ported hooks. Each story updates only its own registry entries.
- Each story in this wave includes a BC-anchor verification task per D-2 (Option C
  reuse-existing-BC strategy); story-writer identifies the existing BC(s) the hook
  satisfies during story decomposition.

**Wave W-17 (Tier 3 — 10 stories):**
- S-8.20 through S-8.28 as per hook list above.
- S-8.29: Final hook port (validate-wave-gate-prerequisite) + adapter crate
  deletion + bin/emit-event removal. Pre-deletion audit: zero
  `legacy-bash-adapter.wasm` references in hooks-registry.toml. AC-2, AC-3,
  and TD-007 fully closed by this story.
- Each story in this wave includes a BC-anchor verification task per D-2 (Option C
  reuse-existing-BC strategy); story-writer identifies the existing BC(s) the hook
  satisfies during story decomposition.

Each story spec must include:
1. BC-anchor verification task (identify existing BC(s) for this hook's behavior).
2. Behavior-parity test: bats test that runs both bash and WASM versions with
   identical inputs and asserts identical outputs.
3. Migration step checklist: (a) write crate; (b) update registry entry; (c)
   delete hooks.json command; (d) delete .sh; (e) run bats; (f) run perf
   benchmark vs S-8.00 baseline if Tier 2.
4. AC for bin/emit-event: replace `bin/emit-event` calls with `host::emit_event`
   in WASM implementation (confirmed per-story; final removal at S-8.29 only).
5. For Tier 2 block-mode hooks: negative test fixtures validating no false-block.

---

## Risks and Mitigations

See D-11 above for the full risk register. Key callouts:

**R-8.07 (TD-007 / bin/emit-event interaction) is the highest-likelihood risk.**
Every bash hook calls `_emit()` which shells out to `bin/emit-event`. bin/emit-event
must NOT be removed until all 42 ports complete (S-8.29 close). Each story
replaces `bin/emit-event` calls with `host::emit_event` in its WASM implementation.
The bash source (still alive until .sh is deleted) retains its `bin/emit-event`
call throughout its lifetime.

**R-8.08 (cumulative WASM startup) is the highest-probability performance risk.**
23 plugins × estimated 10ms (assumption pending OQ-8 measurement) ≈ 230ms+ aggregate
latency on PostToolUse:Edit|Write. Mitigations must be assessed in S-8.00 and
incorporated into the dispatcher before W-16 begins. Warm-pool and compile-cache
are the primary candidates.

**R-8.05 (latent test coverage gap) is the highest-probability quality risk.**
Several hooks (convergence-tracker, validate-template-compliance, validate-anchor-
capabilities-union) have complex multi-path logic that may have partial bats
coverage. The porting process must include test-coverage audit as the first step.

---

## Acceptance Criteria (Epic-Level)

Restated from D-12 for clarity:

| AC | Criterion |
|----|-----------|
| AC-1 | `find plugins/vsdd-factory/hooks -name "*.sh" \| grep -v verify-git-push` returns empty |
| AC-2 | `crates/hook-plugins/legacy-bash-adapter/` does not exist (deleted at S-8.29) |
| AC-3 | `hooks.json` contains zero inline `command` entries for native WASM hooks; zero `[[hooks]]` entries in `hooks-registry.toml` use `plugin = 'hook-plugins/legacy-bash-adapter.wasm'` |
| AC-4 | `release.yml` builds all new WASM crates in the bundle matrix |
| AC-5 | Windows CI runner passes all bats integration tests (verify via GitHub Actions windows-latest) |
| AC-6 | `HOST_ABI_VERSION = 1` in both dispatcher and SDK (confirmed via grep in release gate) |
| AC-7 | Per-Tier-2 hook latency does not regress vs S-8.00 bash baseline by more than 20% (Tier 1/3 hooks fire less frequently and are not benchmarked individually) |
| AC-7b | Aggregate PostToolUse:Edit\|Write latency (sum of all 23 plugins) ≤ 500ms p95 (ceiling raised from 200ms to 500ms by S-8.00 fix-burst v1.9; OQ-8 resolved; warm-pool + compile-cache mitigations required) |
| AC-8 | All per-hook bats behavior-parity tests pass; validate-factory-path-root, validate-input-hash, validate-template-compliance additionally have negative (false-block) test fixtures |
| AC-9 | `bin/emit-event` binary not present in dispatcher binary tree post-S-8.29; validated by `find . -name emit-event` returning empty in the dispatcher binary bundle directory after S-8.29 merge |

---

## Wave Schedule

| Wave | Content | Depends On | Target Release | Key Deliverable |
|------|---------|-----------|----------------|----------------|
| W-15* | S-8.00–S-8.09 (Tier 1 lifecycle + pre-work) | v1.0.0 GA shipped; S-5.07 close; ABI stable | v1.1 | 9 hooks native; adapter pre-retirement audit; bin/emit-event deferred |
| W-16* | S-8.11–S-8.19 (Tier 2 validators) | W-15 complete | v1.2 | 23 validators native; perf benchmarks green |
| W-17* | S-8.20–S-8.29 (Tier 3 protections) | W-15 complete | v1.3 | 0 bash hooks; adapter deleted; bin/emit-event removed |

*Wave IDs provisional — see D-13.

W-16 and W-17 are parallel (both depend on W-15, not on each other).

---

## Dependencies and Blocks

| Dependency | Type | Status | Notes |
|-----------|------|--------|-------|
| v1.0.0 GA tag cut | Prerequisite | Pending (TD-013 blocks GA; 14-day shakedown clock running) | E-8 starts post-GA |
| TD-013 resolved (branch protection) | Prerequisite | P0, S-5.07 | Must land before any E-8 stories open PRs |
| HOST_ABI_VERSION = 1 stability (S-5.06) | Constraint | Locked | No ABI bump in v1.x; additive only |
| `vsdd_hook_sdk::host::exec_subprocess` available | Dependency | Available | Required by most Tier 1 hooks (gh, git, jq calls) |
| `vsdd_hook_sdk::host::read_file` available | Dependency | Available | Required by Tier 2/3 file-inspection hooks |
| Windows CI runner in GitHub Actions | Dependency | Available (ubuntu + windows runners in release.yml) | AC-5 requires windows-latest runner in bats job |
| TD-007 resolution (bin/emit-event retirement) | Dependency | P3 (v1.3) | Closed at S-8.29; NOT incrementally per story. Tiers 2/3 bash hooks need bin/emit-event during migration window |
| vsdd-hook-sdk path-based dependency | Constraint | Available | E-8 crates use `vsdd-hook-sdk = { path = "../../hook-sdk" }` matching capture-commit-activity precedent. TD-010 (crates.io publication) is independent of E-8. |
| DRIFT-010 (originally framed as "26 unported bash hooks"; current count per E-8 scope is 42 adapter-routed scripts — see D-9 release-notes correction) | Dependency | Open | E-8 closes DRIFT-010 when all 42 ports merge |

**Blocks on E-8:** TD-013 (branch protection P0) must be resolved first. E-8
stories should not land on `main` without PR protection restored.

---

## Open Questions

All substantive open questions are resolved inline. Residual items:

**OQ-2 — hooks.json partial-migration state during W-16/W-17 (resolved-default):**
After W-15 deletes 9 hooks.json command entries, hooks.json still has command
entries for ~33 un-ported Tier 2/3 hooks (W-16/W-17 in-flight). The file is a
hybrid: some hooks native (dispatcher-routed only), some bash (hooks.json + registry
adapter). This is a stable transitional state conditional on F-006 resolution
(D-7): dispatcher-routing entries remain throughout; only per-script command entries
are removed. Document in v1.1 release notes; no operational action required
mid-migration.

**OQ-5 — convergence-tracker write-back capability (story-writer audit):**
convergence-tracker mutates `.factory/` state files. The WASM sandbox currently
exposes `read_file` but a `write_file` host fn may be required. Must be assessed
during S-8.18 story-writer phase. If `write_file` is absent, this is an ABI
extension trigger under D-6 procedure.

**OQ-6 — regression-gate capability profile (pre-implementation gate):**
regression-gate.sh appears to invoke external test runners (bats, cargo test).
The WASM sandbox restricts subprocess calls to `binary_allow` list. If the native
port needs to exec arbitrary test runners, this may require a more permissive
capability declaration than current hooks use. Security-reviewer must audit the
bash source and propose a capability profile for S-8.09 before implementation
begins.

**OQ-7 — Windows support matrix canonical document location (D-1 rationale (b)):**
D-1 rationale (b) appeals to "the v1.0 support matrix" to justify that non-bash
Windows users running `git push` from non-bash shells are out-of-scope. No
canonical document carrying this support matrix has been located in `.factory/specs/`,
`docs/`, or README.md. Resolution options: (a) create the support matrix as part
of S-5.07 release-notes work and backfill D-1 with the specific doc reference;
(b) remove rationale (b) from D-1 and ground the exclusion solely on rationale
(a, c, d), which are each independently sufficient. Blocking: LOW — rationale (a,
c, d) are sufficient to sustain D-1's decision without (b).

**OQ-8 — Per-plugin WASM warm-invocation latency baseline (R-8.08 / AC-7b / Goal #6):**
**RESOLVED by S-8.00 (v1.9 fix-burst, 2026-05-02).**
S-8.00 measured: handoff-validator.sh Tier 1 = 43ms median; validate-bc-title.sh Tier 2 = 19ms median; protect-bc.sh Tier 3 = 40ms median. Tier 2 aggregate projection: 19ms × 23 = 437ms — the original 10ms/plugin assumption was incorrect. The 200ms p95 ceiling was unattainable at baseline. AC-7b ceiling raised to 500ms p95. R-8.08 re-scored HIGH/HIGH (no longer pending). Warm-pool + compile-cache mitigations are now REQUIRED (not optional) to meet AC-7b at W-16 gate.

---

## Change Log

### v1.10 — 2026-05-03 — Tier 2/3 wave plan superseded

Tier 2 stories (S-8.11..S-8.19) and Tier 3 stories (S-8.20..S-8.29) wave-plan
references in D-13 superseded. W-16 work re-decomposed into E-9 (epic authored
2026-05-03). W-17 work to be re-decomposed into future E-10. Retired story
placeholders S-8.11..S-8.29 remain in STORY-INDEX as `status: retired` with
`superseded_by: E-9 / future E-10`. Append-only numbering preserved per POLICY 1.

E-8's D-13 W-16/W-17 wave assignments removed; D-13 now scopes only the
Tier 1 wave (W-15) which shipped via v1.0.0-rc.3 + rc.4.

Frontmatter changes: `status: ready` → `status: tier-1-shipped`;
`story_count: 31` → `story_count: 12` (Tier 1 only: S-8.00..S-8.09 + S-8.10 + S-8.30);
`target_release` updated to reflect Tier 2/3 handoff to E-9/E-10.

### v1.1 (2026-04-30) — ADV-E8-P1 pass-1 fix burst (18 findings closed)

**12 HIGH findings closed:**
- F-001: prd_capabilities corrected to [CAP-002, CAP-008, CAP-013, CAP-022].
  CAP-003 was wrong (it is the observability multi-sink capability). CAP-022
  ("Port hook plugins from bash to native WASM") is the primary anchor.
- F-002: CAP-002 title corrected to verbatim H1 from capabilities.md line 32.
  "Cross-platform hook execution" framing moved to Description prose.
- F-003: Tier 2 count standardized to 23 hooks. "21 vs 22 vs 23" contradiction
  resolved; all 23 confirmed Edit|Write-scoped; "21 are Edit|Write-scoped"
  framing removed.
- F-004 + F-005: "44 bash scripts" replaced throughout with "43 unique bash
  scripts → 42 ported by E-8." Registry arithmetic section added to D-3 and
  Problem Statement.
- F-006: D-7 rewritten with explicit dispatcher-routing decision, concrete
  BEFORE/AFTER hooks.json sketch, and corrected AC-3.
- F-007: D-10 inverted. Adapter crate now retired at S-8.29 close (not end of
  Tier 1). Rationale: 33 Tier 2/3 hooks need adapter during W-16/W-17 migration
  window. AC-2 and Wave Schedule updated accordingly.
- F-008: Wave IDs W-15/16/17 marked provisional with note on gap-numbered history
  (W-10 absent in STATE.md). process-gap codified in D-13.
- F-009: S-8.00 pre-work story added (perf benchmark baseline + Tier 1 BC-anchor
  verification). story_count bumped 28→29. AC-7 upgraded to regression-vs-baseline
  criterion. S-8.01..S-8.08 now depend on S-8.00.
- F-010: R-8.07 updated. bin/emit-event removal pinned to S-8.29 close only.
  Explicit AC added to D-12.

**6 MED findings closed:**
- F-011: Block-mode callout added in D-3 Tier 2 table. AC-8 requires negative
  test fixtures for 3 block-mode validators.
- F-012: Goal #5 reworded to "HOST_ABI_VERSION = 1 unchanged throughout E-8
  (additive extension allowed per D-6; no version bump)."
- F-013: D-1 rationale expanded with explicit Windows support matrix reasoning
  (a-d) per finding.
- F-014: TD-010 removed from Dependencies; path-based SDK dependency documented
  matching capture-commit-activity precedent.
- F-015: OQ-1 resolved. S-8.00 produces BC-anchor verification table for all
  9 Tier 1 hooks and creates any missing BCs.
- F-016: Frontmatter schema reconciled. `inputs` and `input-hash` added. Fields
  `tech_debt_ref`, `anchor_strategy`, `priority`, `target_release` documented as
  schema additions beyond template baseline; template update proposed as
  follow-up process-gap.
- F-017: B-3a + B-3b merged into single B-3 bundle (4 hooks, 5 pts). Story
  table updated: S-8.13 covers all 4 state-file guard hooks.
- F-018: R-8.08 added (cumulative WASM startup overhead). AC-7b added (aggregate
  PostToolUse:Edit|Write latency ≤ 200ms p95).

**Open Questions resolved:**
- OQ-1: Resolved → handled in S-8.00.
- OQ-3: Resolved → single crate with internal dispatch (story-writer codifies in S-8.22).
- OQ-4: Resolved → S-8.00 pre-work story per F-009.
- OQ-2, OQ-5, OQ-6: Deferred as documented above.

**Observations addressed:**
- S-8.09 point estimate bumped 4→5 pts (covers regression-gate port + adapter
  audit + retirement prep, even though deletion deferred to S-8.29).
- DRIFT-010 added to Dependencies (E-8 closes it on full port completion).
- track-agent-start Tier 1 grouping rationale documented explicitly in D-3.

### v1.2 (2026-04-30) — ADV-E8-P2 pass-2 fix burst (7 findings closed)

**1 HIGH finding closed:**
- P2-002 [HIGH]: R-8.07 mitigation now references AC-9 by ID. AC-9 added to D-12
  with discrete validation command: "`find . -name emit-event` returns empty in
  dispatcher binary bundle directory after S-8.29 merge." AC-9 also added to
  epic-level AC restatement section.

**4 MED findings closed:**
- P2-001 [MED]: D-7 BEFORE/AFTER hooks.json sketch expanded. BEFORE now shows 3
  inline command entries (validate-bc-title, validate-finding-format, protect-bc)
  plus a separate Bash-matcher entry (protect-secrets) to make the collapse
  unambiguous. AFTER shows one dispatcher entry per (event, matcher) tuple.
- P2-003 [MED]: D-1 rationale (b) appeal to "the v1.0 support matrix" annotated
  with OQ-7 (no canonical doc located). Rationale (a, c, d) independently sustain
  D-1. Out of Scope inventory table updated to reference OQ-7 instead of bare
  "v1.0 support matrix." OQ-7 added to Open Questions section.
- P2-005 [MED]: D-8 Tier 1 header corrected from "9 stories" to "10 stories:
  1 pre-work + 9 hook ports" to match the actual story count (S-8.00 + S-8.01..S-8.09).
  story_count frontmatter = 29 unchanged; total point range 125-155 unchanged.
- P2-007 [MED]: R-8.08 re-scored MEDIUM/MEDIUM (was HIGH/HIGH) pending OQ-8
  measurement. "23 plugins × ~10ms each = 230ms+" estimate re-phrased to
  "estimated 10ms (assumption pending OQ-8 measurement)" throughout (D-11 risk
  table, Risks and Mitigations callout). Goal #6 and AC-7b marked "tentative
  ceiling; baseline-derived adjustment allowed in S-8.00 per OQ-8." OQ-8 added
  to Open Questions section.

**2 LOW findings closed:**
- P2-004 [LOW]: HTML comment added immediately after frontmatter closing `---`
  noting the template schema extensions (tech_debt_ref, anchor_strategy, priority,
  target_release) and pointing to Change Log v1.1 F-016.
- P2-006 [LOW]: R-8.02 mitigation text "warm-pool + compile-cache mitigations in
  D-7 era" replaced with "warm-pool + compile-cache mitigations described under
  R-8.08 (line ~566)" — removes stale era reference.

**Open Questions added:**
- OQ-7: Windows support matrix canonical doc location (D-1 rationale (b)).
- OQ-8: Per-plugin WASM warm-invocation latency baseline (R-8.08 / AC-7b / Goal #6).

**AC count:** D-12 now has AC-1..AC-9 (9 numbered ACs) plus AC-7b as a sub-criterion
(10 rows in table). AC-9 validates bin/emit-event removal at S-8.29 close.

### v1.3 (2026-04-30) — ADV-E8-P4 pass-4 fix burst (3 findings closed)

**1 MED finding closed:**
- F-P4-001 [MED]: Stories table row S-8.15 "(2 hooks)" corrected to "(1 hook)".
  Inventory confirms exactly 1 hook (validate-wave-gate-completeness) maps to
  S-8.15 (D-8 Tier 2 table line 484, Tier 2 inventory line 644). Tier 2 bundle
  arithmetic 4+4+4+4+1+1+1+2+2 = 23 is consistent only with 1 hook in S-8.15.

**2 LOW findings closed:**
- F-P4-002 [LOW]: D-7 AFTER JSON `"matcher": "Bash"` entry expanded to include
  both the dispatcher entry AND a verify-git-push.sh direct command entry,
  matching D-1 prose commitment (lines 444-447) that verify-git-push.sh retains
  a direct command entry under "matcher": "Bash" post-E-8. Explanatory comment
  updated to reference the JSON entries directly.
- F-P4-003 [LOW]: R-8.09 added to D-11 risk register: cumulative dispatcher
  binary bundle size growth from 42 new .wasm artifacts; LOW/LOW severity;
  mitigation is per-bundle size measurement in S-8.00 baseline with 25%-growth
  trigger for WASM size-optimization (`opt-level = "z"`, strip-debug, LTO).

### v1.4 (2026-04-30) — ADV-E8-P6 pass-6 fix burst (7 findings closed)

**2 MED findings closed:**
- F-P6-004 [MED]: D-10 entry/script accounting clarified throughout. Rationale
  paragraph updated from "33 Tier 2/3 hooks" to "34 Tier 2/3 entries (33 unique
  scripts; protect-secrets dual-registered for Bash + Read events)." Timeline
  bullet updated from "33 Tier 2/3 entries still point to adapter" to "34 Tier 2/3
  entries (33 unique scripts; protect-secrets dual-registered for Bash + Read
  events) still point to adapter." Trailing note updated from "produced 33 dangling
  references" to "produced 34 dangling registry references (33 unique scripts + 1
  protect-secrets second registration)."
- F-P6-007 [MED]: AC-7 Tier-2 qualifier applied consistently across all three
  locations. D-12 AC-7 criterion, restatement AC-7 row, and Goal #6 all updated
  to "per-Tier-2 hook latency does not regress vs S-8.00 baseline by more than 20%"
  with parenthetical "(Tier 1/3 hooks fire less frequently and are not benchmarked
  individually)." Rationale: Tier 2 PostToolUse:Edit|Write hooks fire on every
  file write and are the latency-sensitive path; Tier 1 (lifecycle) and Tier 3
  (PreToolUse) fire infrequently, making per-event overhead less critical.

**5 LOW findings closed:**
- F-P6-002 [LOW]: Note added after Goals enumeration clarifying that AC-4
  (release.yml bundle integration), AC-8 (bats behavior parity), and AC-9
  (bin/emit-event removal) are release/quality gates not enumerated as strategic
  goals; they verify enabling work for Goals #1-#6.
- F-P6-003 [LOW]: D-7 BEFORE/AFTER JSON sketches restructured to show explicit
  event-level grouping (PostToolUse / PreToolUse as top-level keys). BEFORE now
  shows validate-bc-title/validate-finding-format under PostToolUse:Edit|Write and
  protect-bc/protect-secrets under PreToolUse:Edit|Write and PreToolUse:Bash,
  eliminating the conflation of PostToolUse and PreToolUse entries under the same
  matcher block. AFTER shows dispatcher-only entries per (event, matcher) tuple
  with verify-git-push.sh retained under PreToolUse:Bash per D-1.
- F-P6-005 [LOW]: R-8.10 added to D-11 risk register: BC-creation explosion via
  D-2 Option C exception path; MEDIUM/MEDIUM severity; mitigation is S-8.00 audit
  measuring BC-coverage gap with OQ escalation trigger if >5 Tier 1 hooks lack BC.
- F-P6-006 [LOW]: Explicit BC-anchor verification bullet added under W-16 and W-17
  in Story Decomposition Sketch: "Each story in this wave includes a BC-anchor
  verification task per D-2 (Option C reuse-existing-BC strategy); story-writer
  identifies the existing BC(s) the hook satisfies during story decomposition."
- F-P6-008 [LOW]: D-8 total points floor updated from "~125-155" to "~123-155"
  with explicit annotation: "123 base sum, 32 reserved for BC-creation buffer per
  R-8.10." Base sum 123 = Tier 1 (5+4+5+3+4+3+3+3+3+5=38) + Tier 2
  (5+5+5+5+5+5+5+4+6=45) + Tier 3 (3+3+4+5+4+4+4+5+4+4=40). story_count
  unchanged at 29; tier counts unchanged at 9/23/10.

### v1.5 (2026-04-30) — ADV-E8-P7 pass-7 fix burst (3 findings closed)

**Empirical registry verification (2026-04-30):** 52 total `[[hooks]]` entries; 44
route via legacy-bash-adapter; 43 unique script_path values. verify-git-push.sh IS
the 43rd adapter-routed unique script (not separately registered); excluded per D-1.
Canonical numbers: **44 entries / 43 unique / 42 ported. Scenario B.**

**1 HIGH finding closed:**
- F-P7-001 [HIGH]: Scenario B confirmed empirically. 44/43/42 is the canonical
  arithmetic. verify-git-push.sh is the 43rd unique adapter-routed script, excluded
  per D-1. Updated four sites: Description (rewritten to lead with "42 in-scope /
  43 unique / verify-git-push is 43rd"), Problem Statement paragraph (added
  adapter-routed note), Problem Statement registry arithmetic block (parenthetical
  names verify-git-push as 43rd unique adapter-routed script), D-3 arithmetic header
  (labeled "empirically verified"; added explicit verify-git-push bullet + Tier sum).
  Tier inventories unchanged (9/23/10 enumerate in-scope scripts only).

**1 MED finding closed:**
- F-P7-002 [MED]: D-7 line 465: "one per event group" → "one per (event, matcher)
  tuple." Post-F-P6-003 AFTER JSON has 3 dispatcher entries keyed on (event, matcher).

**1 LOW finding closed:**
- F-P7-003 [LOW]: DRIFT-010 description: "26 unported bash hooks" → "originally framed
  as '26 unported bash hooks'; current count per E-8 scope is 42 adapter-routed
  scripts — see D-9 release-notes correction."

**Arithmetic invariant:** story_count = 29; 123 base points unchanged; 9/23/10 unchanged.

### v1.6 (2026-04-30) — ADV-E8-P8 pass-8 fix burst (2 findings closed)

**1 MED finding closed:**
- F-P8-001 [MED]: Changelog version ordering corrected. v1.4 entry was appended
  after the v1.5 entry, producing out-of-order sequence (v1.3 → v1.5 → v1.4).
  Entries swapped to restore ascending order: v1.1 → v1.2 → v1.3 → v1.4 → v1.5 → v1.6.

**1 LOW finding closed:**
- F-P8-002 [LOW]: D-1 disposition path made explicit. Added note on disposition at
  end of D-1 Ruling block: verify-git-push.sh's `[[hooks]]` registry entry
  (currently adapter-routed) is REMOVED at S-8.29 close as part of
  legacy-bash-adapter retirement; its hooks.json direct command entry is RETAINED.
  Post-E-8 state: 1 hooks.json direct entry, 0 hooks-registry.toml entries for
  verify-git-push.sh. Closes AC-3 implicit gap (zero adapter-routed entries post-E-8).

**Arithmetic invariant:** story_count = 29; 123 base points unchanged; 9/23/10
unchanged; canonical numbers 44 entries / 43 unique / 42 ported unchanged.

### v1.7 (2026-04-30) — ADV-E8-P11 CONVERGENCE_REACHED

Status flip `draft` → `ready` per ADR-013 3-clean-pass discipline. No content edits.

**Convergence trajectory (11 passes):**
- P1: 18 substantive (12H + 6M) — opening burst
- P2: 7 (1H + 4M + 2L) — high-residual cleanup
- P3: 0 → 1_of_3 advance
- P4: 1 MED (F-P4-001 S-8.15 row count) → RESET to 0_of_3
- P5: 0 → 1_of_3 advance
- P6: 2 MED + 5 LOW (F-P6-004 entry/script conflation; F-P6-007 AC-7 Tier-2 qualifier) → RESET
- P7: 1 HIGH + 1 MED + 1 LOW (F-P7-001 registry arithmetic; F-P7-002 D-7 stale wording; F-P7-003 DRIFT-010 stale)
- P8: 1 MED + 1 LOW (F-P8-001 changelog ordering; F-P8-002 D-1 disposition)
- P9: 0 → 1_of_3 advance
- P10: 1 LOW (D-3 vs D-8 bundle nomenclature; SKIP_FIX per S-7.03)
- P11: 0 → 3_of_3 = CONVERGENCE_REACHED

**Total findings closed:** 41 substantive + 11 LOW.

**Spec maturity confirmed via empirical anchoring** to `plugins/vsdd-factory/hooks-registry.toml` (52 entries / 44 adapter-routed / 43 unique / 42 ported). Pass-10 re-derived 22 invariants from primary sources without inheriting "carried" labels — historical 1_of_3→reset pattern (P3→P4 MED, P5→P6 MED) did NOT repeat.

**Outstanding LOW deferred per S-7.03 skip-fix:**
- D-3 vs D-8 bundle nomenclature (B-5 vs B-5a/b/c, B-6 vs B-6a/b) — operational impact zero (Story column disambiguates); may be polished in v1.8 alongside future content edits.

**Status implications:**
- Story-writer dispatch unblocks for S-8.00..S-8.29 decomposition.
- Stories status remains `draft` until per-story-spec bursts.
- E-8 epic ready for orchestrator routing into Wave 15 / W-15* (after v1.0.0 GA close).

### v1.8 (2026-05-01) — D-183 Phase B: S-8.30 SDK extension story authored

S-8.30-sdk-extension-hookpayload-subagentstop-fields.md v1.0 status=draft authored by story-writer.

- story_count bumped 30 → 31.
- S-8.30 added to Stories table after S-8.10 (SDK extensions grouped before Tier 2/3 placeholder stories).
- Story ID rationale: S-8.30 per POLICY 1 — Tier 2/3 placeholder IDs S-8.11..S-8.29 left untouched; appending after Tier 3 is the cleanest path (no renumber).
- S-8.30 anchors BC-2.02.012 (HookPayload SubagentStop top-level fields). 3 pts. 8 ACs. depends_on: S-8.00. blocks: S-8.01, S-8.02, S-8.03, S-8.05 (re-convergence with typed-projection prose per process-gap-D-183-A).
- Phase C next: adversarial pass-1 on S-8.30.

### v1.9 (2026-05-02) — S-8.00 AC-3 fix-burst: AC-7b ceiling raised, OQ-8 resolved

S-8.00 perf baseline measurement (darwin-arm64 local runner, hyperfine --warmup 3 --runs 10):
- handoff-validator.sh (Tier 1): median 43ms, p95 56ms
- validate-bc-title.sh (Tier 2 representative): median 19ms, p95 21ms
- protect-bc.sh (Tier 3): median 40ms, p95 42ms

Tier 2 aggregate projection: 19ms/plugin × 23 plugins = **437ms** — exceeds the 200ms AC-7b ceiling.
The 10ms/plugin assumption (OQ-8) was incorrect; measured value is 1.9× the estimate.

**Fix-burst changes (AC-3/AC-9):**
- AC-7b ceiling: 200ms → **500ms p95** (accommodates 437ms bash baseline with 15% headroom; warm-pool mitigation required)
- Goal #6 updated to match new AC-7b ceiling
- R-8.08 re-scored MEDIUM/MEDIUM → **HIGH/HIGH** (risk confirmed; warm-pool is now REQUIRED not optional)
- OQ-8 marked RESOLVED (S-8.00 measurement closes the open question)
- input-hash: unchanged (no S-8.00 story-spec changes needed)

BC-anchor audit outcome (AC-4/AC-5): 0 of 9 Tier 1 hooks have Gap-Found = Y. All 9 hooks have pre-existing BC-7.03/BC-7.04 coverage. No new BCs drafted. No S-8.0N story point bumps required for BC overhead.

**Cosmetic cleanup post-S-8.00 PR #47 merge:** changelog ordering restored to earliest-first (v1.8 D-183 entry now precedes v1.9 fix-burst entry); renumbered v1.10 → v1.9 (gap closure; v1.9 was unused externally); duplicate AC-7b in "Acceptance Criteria (Epic-Level)" restatement section swept from ≤ 200ms p95 (stale tentative value) to ≤ 500ms p95 to match the post-fix-burst active spec value.
