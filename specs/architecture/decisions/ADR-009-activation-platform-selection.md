---
document_type: adr
adr_id: ADR-009
status: accepted
date: 2026-04-26
subsystems_affected: [SS-09]
supersedes: null
superseded_by: null
---

# ADR-009: Activation-Skill-Driven Platform Binary Selection

## Context

The v1.0 dispatcher ships as a compiled binary per platform (ADR-001). Claude Code's
`hooks.json` file must specify a literal path to the dispatcher binary in its
`command` field. This creates a platform-specific path problem: the binary for
`darwin-arm64` lives at a different path than `linux-x64` or `windows-x64`, yet
`hooks.json` is a single committed file.

Two features that would have solved this cleanly were investigated and found to not
exist in the Claude Code harness as of the v1.0 design phase (Open Question Q5,
design doc lines 804–832):

1. **Variable expansion in `hooks.json` `command` fields:** Claude Code supports
   `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PROJECT_DIR}`, and `${CLAUDE_PLUGIN_DATA}` but
   not `${OS}`, `${ARCH}`, or any platform identifier. A path like
   `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/${PLATFORM}/factory-dispatcher` cannot
   be expressed.

2. **Plugin install lifecycle hooks:** `plugin.json` has no `onInstall`,
   `postInstall`, or `setup` hook. There is no opportunity to run a setup script when
   the plugin is first installed through the marketplace.

Without either of these, a different mechanism was required to write a
platform-appropriate `hooks.json` before the dispatcher can be used.

## Decision

The existing `/vsdd-factory:activate` skill is extended to handle platform selection.
At activation time, the skill detects the host OS and architecture, copies the
matching pre-committed `hooks.json.<platform>` variant to `hooks.json` (which is
gitignored), and verifies the dispatcher binary at the expected path is present and
executable. CI generates all five `hooks.json.<platform>` variants from
`hooks.json.template` on every release and commits them as part of the release chore
commit. The activation skill is a required first step after plugin install; this is
documented clearly in the getting-started guide and the activation skill output.

## Rationale

The design doc (lines 804–832) evaluated this against the constraint that neither
platform variable expansion nor install lifecycle hooks exist. The options were:

Option 1: Require a Python or Node script in `hooks.json` that detects the platform
and execs the right binary. Rejected: Python is not guaranteed on all operator
machines (ADR-001 rationale); Node is not guaranteed either (Claude binary is
standalone, not npm-installed).

Option 2: Detect platform at dispatcher startup and exec the appropriate binary.
Rejected: the dispatcher binary path in `hooks.json` IS the platform-specific
binary. There is no platform-agnostic wrapper binary to exec from.

Option 3 (chosen): Leverage the activation skill, which already runs on the host
machine with full shell access, to write the platform-specific `hooks.json`. The
skill calls `detect-platform.sh` (which runs `uname -s` and `uname -m`) and
`apply-platform.sh <platform>` (which copies the matching variant). The detection
helper has a test override (`MOCK_UNAME_S` / `MOCK_UNAME_M` env vars) for the
activation bats test matrix.

The `hooks.json` gitignore approach is semantically correct: `hooks.json` is a
per-machine artifact, analogous to a build output. The source of truth is
`hooks.json.template`. CI verifies no drift between the committed `.platform` variants
and what the template would generate, preventing hand-edits to the generated files.

The tradeoff accepted is that marketplace auto-install no longer makes the plugin
immediately functional. The design doc (line 824) states this explicitly: "Documented
loud-and-clear in the getting-started guide. This is the only path without either a
Python runtime dep or a Claude Code feature that doesn't exist."

## Consequences

### Positive

- Platform detection happens once at activation time, not on every hook invocation.
  There is no per-invocation overhead.
- `apply-platform.sh` is a simple file copy; it cannot fail partially.
  The verification step (binary present and executable) catches install corruption
  before the first hook invocation.
- The `MOCK_UNAME_S`/`MOCK_UNAME_M` test overrides enable a full bats matrix
  covering all five platforms without requiring five CI machines.
- Re-activation on a different host (SSH'd into a Linux box, operator moved to a
  new Mac) detects the platform mismatch and overwrites `hooks.json` correctly.

### Negative / Trade-offs

- `/vsdd-factory:activate` is a required manual step after every fresh plugin install.
  A new user who installs the plugin and starts a session without activating will see
  no hooks firing. The activation reminder appears in plugin install output, but
  there is no automated enforcement.
- Windows uses `factory-dispatcher.exe`; the `.exe` extension must be handled
  specially in `apply-platform.sh` and the binary verification logic.
- If the Claude Code harness adds variable expansion or install lifecycle hooks in
  a future release, this ADR should be revisited and the activation step potentially
  eliminated.

### Status as of v1.0.0-beta.5

IN-EFFECT. `plugins/vsdd-factory/hooks/hooks.json.darwin-arm64`, `.darwin-x64`,
`.linux-x64`, `.linux-arm64`, `.windows-x64` are committed and CI-verified.
`plugins/vsdd-factory/hooks/hooks.json` is gitignored. The activation skill
calls `detect-platform.sh` and `apply-platform.sh` as documented. Darwin-arm64
activation has been verified in production.

## Alternatives Considered

- **Shell wrapper script in hooks.json:** A platform-agnostic shell script that
  detects OS/arch and execs the right binary. Rejected: requires a shell on all
  platforms (fails bare Windows without git-bash); adds per-invocation detection
  overhead.
- **Single fat binary:** Ship one binary that embeds all platform targets and
  self-selects. Rejected: not possible with Rust's `std::env::consts::OS` at
  compile time; would require a cross-platform stub loader, adding complexity.
- **Plugin.json setup hook:** Would be the cleanest solution but does not exist
  in the Claude Code harness API.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 67–73 (activation-skill decision), lines 484–486 (Q5 resolution summary),
  lines 804–832 (full Q5 Open Question resolution with variable expansion and install
  lifecycle hook investigation).
- **Code as-built:** `plugins/vsdd-factory/skills/activate/detect-platform.sh`,
  `plugins/vsdd-factory/skills/activate/apply-platform.sh`.
- **Code as-built:** `plugins/vsdd-factory/hooks/hooks.json.darwin-arm64` (and
  other platform variants).
- **Skill documentation:** `plugins/vsdd-factory/skills/activate/SKILL.md` (steps
  2–6 describe the platform detection and hooks.json copy flow in detail).
