---
document_type: architecture-section
level: L3
section: "SS-09-config-activation"
version: "1.2"
status: accepted
producer: architect
timestamp: 2026-04-25T00:00:00
amended: 2026-05-07
phase: 1.2
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
traces_to: ARCH-INDEX.md
---

# SS-09: Configuration and Activation

## [Section Content]

## Purpose

The Configuration and Activation subsystem is the runtime glue that wires Subsystem
A (compiled Rust dispatcher) into Subsystem B's (orchestration framework) runtime
configuration on a specific operator's machine. It owns the activation flow, the
CI variant generation pipeline, and the operator-facing configuration entry points.

Claude Code has no plugin install lifecycle hooks and no variable expansion in
`hooks.json`. The activation skill is therefore the only mechanism by which the
correct per-platform dispatcher binary is selected and the correct `hooks.json`
variant is put in place. This constraint is an explicit design decision (Q5 / ADR-009):
activation must be run once post-install and re-run when switching platforms.

The subsystem's scope is narrow: it does not own general plugin configuration
(that is in skills' own config contracts), observability sink configuration (SS-03),
or the `hooks-registry.toml` routing table (SS-07). It owns only the
platform-selection and binary-verification activation plumbing, plus the CI scripts
that generate the 5 platform-specific `hooks.json` variants from the single
`hooks.json.template` source of truth.

## Modules

| Module / File | Responsibility |
|---|---|
| `plugins/vsdd-factory/hooks/hooks.json.template` | Source of truth for hooks.json; declares event types and dispatcher binary path template (per ADR-019 v1.3, no `async` flags at envelope — all events synchronous; async classification belongs in `hooks-registry.toml`) |
| `plugins/vsdd-factory/hooks/hooks.json.darwin-arm64` | CI-generated platform variant; committed to repo on release |
| `plugins/vsdd-factory/hooks/hooks.json.darwin-x64` | CI-generated platform variant |
| `plugins/vsdd-factory/hooks/hooks.json.linux-x64` | CI-generated platform variant |
| `plugins/vsdd-factory/hooks/hooks.json.linux-arm64` | CI-generated platform variant |
| `plugins/vsdd-factory/hooks/hooks.json.windows-x64` | CI-generated platform variant |
| `plugins/vsdd-factory/hooks/hooks.json` | Active runtime file; gitignored; written by activate skill; read by Claude Code |
| `plugins/vsdd-factory/.claude-plugin/plugin.json` | Plugin manifest: declares plugin name, version, author; read by Claude Code's plugin loader |
| `ci/platforms.yaml` | 5-platform CI matrix definition; drives cross-compile jobs and binary commit |
| `scripts/generate-registry-from-hooks-json.sh` | Generates `hooks-registry.toml` from `hooks.json`; idempotent; "DO NOT HAND-EDIT" during migration |
| `scripts/check-platforms-drift.py` | Validates that all 5 platform binary variants are present and at the current version |
| `plugins/vsdd-factory/skills/activate/SKILL.md` | Activation skill: platform detection + hooks.json copy + binary verification + settings write |
| `plugins/vsdd-factory/skills/deactivate/SKILL.md` | Deactivation skill: restores pre-activation hooks.json state |

<!-- F-003 (Wave 7 pass-1): Shared-ownership note — `scripts/bump-version.sh` and `.github/workflows/Release.yml` are NOT enumerated in this Modules table because their primary semantic contract (BC-9.01.001 prerelease semver format; BC-9.01.003 atomic bot commit) lives in SS-09 BCs; their target_module (per Wave 7 stories) is declared SS-10 (CLI/release tooling surface). Per Wave 3 F-007 / Wave 5 F-002 sanctioned-template-anchor pattern. Future architecture cleanup (deferred TD): explicitly list these files here as SS-09-owned with SS-10 invocation surface, OR move to a dedicated "shared-ownership" Modules sub-table. -->

## Public Interface

**User-facing activation commands:**
- `/vsdd-factory:activate` — detect platform, copy `hooks.json.<platform>` to
  `hooks.json`, verify dispatcher binary, write `activated_platform` to
  `.claude/settings.local.json`.
- `/vsdd-factory:deactivate` — restore prior `hooks.json` state.

**`hooks.json.template` schema (Claude Code hook format, post-ADR-019):**
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "command": "plugins/vsdd-factory/hooks/dispatcher/bin/<PLATFORM>/factory-dispatcher"
      }
    ],
    "PostToolUse": [
      {
        "command": "plugins/vsdd-factory/hooks/dispatcher/bin/<PLATFORM>/factory-dispatcher"
      }
    ]
  }
}
```

All event declarations are synchronous — no `"async": true` key on any entry. Every
event (PreToolUse, PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd,
WorktreeCreate, WorktreeRemove, PostToolUseFailure, PermissionRequest) follows this
synchronous form with no per-event carve-outs. Per-plugin async classification lives
in `hooks-registry.toml` at the registry layer (ADR-019 §Decision 1).

The template uses `<PLATFORM>` as a literal token replaced by CI during variant
generation (not an environment variable; Claude Code does not support variable
expansion in `hooks.json`).

**`plugin.json` schema:**
```json
{
  "name": "vsdd-factory",
  "version": "1.0.0-beta.4",
  "author": "vsdd-factory team"
}
```

**Async classification (post-ADR-019):** All Claude Code hook events are synchronous at
the envelope. Per-plugin async classification lives in `hooks-registry.toml`. The
dispatcher partitions matched plugins into `sync_group` (await-all, gates user-facing
exit code) and `async_group` (fire-and-forget, never gates the user). The envelope no
longer distinguishes sync vs async events — that distinction moved to the registry layer
(ADR-019 §Decision 1+3).

**`schema_version = 2`** in `hooks-registry.toml` (post-ADR-019); mismatch = hard error
in the dispatcher. The generation script embeds `schema_version = 2`; a v1 registry
triggers a hard schema-version error at dispatcher startup (ADR-019 §Decision 5 — no
backwards compatibility).

## Internal Structure

Activation flow (pass-1-architecture.md, lines 221-227):

1. Activate skill runs in the operator's Claude Code session.
2. Detects OS + arch by running `uname -s` + `uname -m` (or equivalent on Windows).
3. Maps to one of 5 platform identifiers: `darwin-arm64`, `darwin-x64`,
   `linux-x64`, `linux-arm64`, `windows-x64`.
4. Copies `plugins/vsdd-factory/hooks/hooks.json.<platform>` over
   `plugins/vsdd-factory/hooks/hooks.json` (the gitignored runtime file).
5. Verifies dispatcher binary at
   `plugins/vsdd-factory/hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]`
   is present and executable.
6. Writes `activated_platform: <platform>` to `.claude/settings.local.json`.
7. Optionally warns if re-activating with a different platform than previously
   recorded (cross-host drift warning; NFR-COMPAT-004).

CI variant generation (`generate-registry-from-hooks-json.sh`):
Reads `hooks.json` (or template); produces `hooks-registry.toml` by mapping
each hook entry's command to a registry entry with `plugin = "legacy-bash-adapter.wasm"`
and `plugin_config.script_path` extracted from the command path. The script is
idempotent: running it twice produces the same output.

## Dependencies

**Incoming (consumers of SS-09):**
- Claude Code harness — reads `hooks.json` to determine which command to invoke
  per hook event.
- Operator — runs `/vsdd-factory:activate` once post-install.

**Outgoing (SS-09 depends on):**
- SS-01 (Hook Dispatcher Core) — the `hooks.json` variants and
  `hooks-registry.toml` wire the dispatcher binary into the hook bus; SS-09
  generates these files.

## Cross-Cutting

- **Platform selection is the only mechanism:** No runtime variable expansion in
  `hooks.json`; no install-time plugin lifecycle. The activate skill is the sole
  path for wiring the dispatcher on a new machine (ADR-009).
- **`hooks.json` is gitignored:** The runtime file is operator-local. The 5
  platform variants (committed) are the source of truth. The template is the
  source of truth for the variants (ADR-009).
- **Schema versioning:** `REGISTRY_SCHEMA_VERSION = 2` in `hooks-registry.toml`
  (post-ADR-019); checked by dispatcher on load (NFR-MAINT-004). The generate script
  embeds this constant; bumping it requires coordinated bump in the dispatcher's
  `registry.rs::REGISTRY_SCHEMA_VERSION`. A v1 registry triggers a hard schema-version
  error — no downgrade path (ADR-019 §Decision 5).
- **Binary presence check:** Activate skill verifies the binary is present before
  writing `hooks.json`. If absent, it reports the error and does not activate —
  preventing a state where Claude Code tries to invoke a missing binary.
- **CI atomic release:** Per ADR-001, platform binaries are committed by CI as
  part of the release chore commit. Never committed on feature branches. The
  `check-platforms-drift.py` script gates the release CI step.

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-09/`
(target prefix BC-9; current BC count in ARCH-INDEX Subsystem Registry).

High-level BC groupings: platform detection and hooks.json copy (BC-9.001–BC-9.008),
binary presence verification (BC-9.009–BC-9.012), settings.local.json write
(BC-9.013–BC-9.015), deactivation restore (BC-9.016–BC-9.018), CI variant
generation idempotence (BC-9.019–BC-9.020).

## ADRs

- ADR-001: Compiled Rust dispatcher per platform — `decisions/ADR-001-rust-dispatcher.md`
- ADR-004: TOML for all configuration files — `decisions/ADR-004-toml-config.md`
- ADR-009: Activation-skill-driven platform binary selection — `decisions/ADR-009-activation-platform-selection.md`
- ADR-011: Dual hooks.json + hooks-registry.toml during migration — `decisions/ADR-011-dual-hook-routing-tables.md`

## Drift / Known Issues

- **DRIFT-004 (P1 — medium):** Two parallel routing tables coexist. `hooks.json`
  (legacy v0.79.x, 45 entries) and `hooks-registry.toml` (generated v1.0, 45
  entries) are both live. The generate script is the only authorized mutation path
  for the TOML during migration. Planned-fix at 1.0: retire `hooks.json` as a
  bootstrap-only artifact or document the clean retirement path (L-P0-002).
- **`hooks.json` vs template drift:** CI generates 5 platform variants from the
  template; if the template is modified without regenerating variants, the variants
  are stale. The `check-platforms-drift.py` script catches this in release CI but
  not in development.
- **Windows activation:** The activate skill detects Windows and copies
  `hooks.json.windows-x64`. The dispatcher binary is present. The bash hooks
  still require git-bash (DRIFT-010), so activation succeeds but hook execution
  degrades without git-bash.

---

## CHANGELOG

### Amendment 2026-05-07 (v1.1 → v1.2 — F2 pass-3 fix burst)

Addresses adversary pass-3 finding F-P3-006.

**F-P3-006 (stale body-vs-amendment readability gap):** The body of SS-09 (Modules
table, Public Interface schema, async/sync section, schema_version references) still
contained the pre-ADR-019 text from the original v1.0, with corrections pushed into
an amendment footer. This created a readability gap where casual readers saw stale
information. This amendment replaces all stale content in-place:

- Modules table `hooks.json.template` row: replaced `"async" flags per event type`
  claim with post-ADR-019 description (no async flags at envelope; registry-layer
  partition per ADR-019 v1.3).
- Public Interface schema example: replaced `"PostToolUse": [{"async": true}]` with
  all-synchronous form; added explanatory paragraph for all 10 events.
- Public Interface "Async vs sync" paragraph: replaced with post-ADR-019 description
  of registry-layer partition (sync_group / async_group).
- Public Interface schema_version: `"schema_version = 1"` → `"schema_version = 2"`.
- Cross-Cutting schema versioning bullet: updated to `REGISTRY_SCHEMA_VERSION = 2`
  with ADR-019 §Decision 5 cross-reference.

The v1.0→v1.1 amendment text below is preserved as historical record and source of
the corrected canonical text. It is now superseded in the body sections above.

### Amendment 2026-05-07 (v1.0 → v1.1 — async semantics cycle F2 pass-1 fix)

**Reason:** ADR-019 (accepted 2026-05-07) moves async classification from the
`hooks.json` envelope layer to `hooks-registry.toml`. SS-09 was listed as an
affected subsystem in ADR-019 frontmatter but was not amended in the F2 burst-v1
package. Adversary pass-1 finding F-P1-002 flagged the stale content. This
amendment corrects the following specific stale claims:

### Modules table correction

The original Modules row for `hooks.json.template` read:

> "Source of truth for hooks.json; declares event types, dispatcher binary path template, `"async"` flags per event type"

**Corrected description:** `hooks.json.template` is the source of truth for
`hooks.json` platform variants. It declares event types and the dispatcher binary
path template (`<PLATFORM>` token). Per ADR-019 §Decision 1, `hooks.json.template`
does **not** declare `async` flags per event type — all event declarations in
`hooks.json.template` and all five platform variants are synchronous (no `"async":
true` key present on any entry). Async classification belongs in
`hooks-registry.toml` at the per-plugin registry layer, not in the envelope.

### Public Interface schema correction

The original `hooks.json.template` schema example showed:

```json
"PostToolUse": [
  {
    "command": "...",
    "async": true
  }
]
```

**Corrected schema:** All event declarations are synchronous. The `"async": true`
key is absent from every entry in `hooks.json.template` and all platform variants
post-ADR-019. The canonical schema is:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "command": "plugins/vsdd-factory/hooks/dispatcher/bin/<PLATFORM>/factory-dispatcher"
      }
    ],
    "PostToolUse": [
      {
        "command": "plugins/vsdd-factory/hooks/dispatcher/bin/<PLATFORM>/factory-dispatcher"
      }
    ]
  }
}
```

Every event entry — PreToolUse, PostToolUse, Stop, SubagentStop, SessionStart,
SessionEnd, WorktreeCreate, WorktreeRemove, PostToolUseFailure — follows this
synchronous form. No per-event carve-outs. See ADR-019 §Decision 1.

### Async vs sync hook events — correction

The original paragraph read:

> "PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd use `"async": true`
> in `hooks.json.template`. PreToolUse and PermissionRequest are sync."

**Corrected statement:** All Claude Code hook events are synchronous at the
envelope per ADR-019 §Decision 1. Per-plugin async classification lives in
`hooks-registry.toml`. The dispatcher partitions matched plugins into
`sync_group` (await-all, gates user-facing exit code) and `async_group`
(fire-and-forget, never gates). The envelope no longer distinguishes sync vs async
events — that distinction moved to the registry layer.

### Schema version correction

The original Cross-Cutting section stated:

> `REGISTRY_SCHEMA_VERSION = 1` in `hooks-registry.toml`; mismatch = hard error
> in the dispatcher.

**Corrected:** `REGISTRY_SCHEMA_VERSION = 2` post-ADR-019. The schema version was
bumped from 1 → 2 when the `async: bool` per-plugin field was added. The generate
script (`generate-registry-from-hooks-json.sh`) must embed `schema_version = 2`.
The dispatcher's `registry.rs::REGISTRY_SCHEMA_VERSION` constant = 2. A registry
with `schema_version = 1` triggers a hard schema-version error at dispatcher
startup (ADR-019 §Decision 5 — no backwards compatibility; no downgrade path).

**All references to `schema_version = 1` in this document (original Cross-Cutting
section lines 152-155) are superseded by this amendment.**
