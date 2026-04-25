---
document_type: adr
adr_id: ADR-001
status: accepted
date: 2026-04-24
subsystems_affected: [SS-01, SS-09]
supersedes: null
superseded_by: null
---

# ADR-001: Compiled Rust Dispatcher Per Platform

## Context

vsdd-factory needed a single routing entry point that Claude Code would invoke on
every hook event. The v0.79.x approach used `hooks.json` matcher arrays and bash
scripts directly. This failed because Claude Code v0.79.0–v0.79.4 had a
`PostToolUse:Bash` matcher de-duplication bug that could not be worked around in
configuration alone (upstream issue claude-code#52715). Additionally, all 30+ existing
hooks were bash-only, making Windows support impossible without a new dispatch layer.

The dispatcher needed to: (a) own its own routing independent of the harness's
matcher field; (b) run on all 5 platforms (darwin-arm64, darwin-x64, linux-x64,
linux-arm64, windows-x64) without a runtime dependency; (c) start in 1–5ms so as
not to perceptibly delay tool calls.

## Decision

Implement the dispatcher as a compiled Rust binary. Ship one binary per platform.
Commit binaries directly to the repository under
`plugins/vsdd-factory/hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]`
as part of the release CI chore commit. Never commit binaries on feature branches.

## Rationale

Five languages were evaluated:

| Language | Startup | Runtime dep | Windows | In-project |
|----------|---------|-------------|---------|------------|
| Rust | ~1–5ms | None (static) | Yes | Yes |
| Go | ~1–5ms | None (static) | Yes | No |
| Python | 50–200ms | Required | Often absent | Yes |
| Node | 50–200ms | Required | Not guaranteed | No |
| Bash | <1ms | None | No (git-bash only) | Yes |

Rust was chosen: fast startup, no runtime dependency, memory and type safety,
alignment with existing Rust direction. Go was the close runner-up but has no
existing project presence. Python is not guaranteed on operator machines. Bash
fails Windows.

Binary-commit strategy: zero install friction beats repo size over three years
(~3GB ceiling per design Q1). Works air-gapped. No dependency on Claude Code
plugin lifecycle hooks (which do not exist for install-time). No version-mismatch
class of bug. Orphan-branch migration is mechanical if repo size becomes a problem.

## Consequences

### Positive
- Dispatcher routing is internal; the harness matcher bug is bypassed entirely.
- Single statically-linked binary; no runtime dependency on operator machines.
- ~1–5ms startup cost per hook event.
- Type safety and memory safety reduce correctness risk for capability-enforcement paths.

### Negative / Trade-offs
- 5-platform CI matrix adds release complexity.
- Binary commits grow the repository; orphan-branch migration may be needed post-v3.

### Status as of v1.0.0-beta.4
IN-EFFECT. Per-platform binaries committed at
`plugins/vsdd-factory/hooks/dispatcher/bin/<platform>/`. Darwin-arm64 activated
and verified in production. Windows-x64 binary committed and smoke-tested.

## Alternatives Considered

- **Go:** Same binary story, simpler cross-compile. Rejected: no existing project presence.
- **Python:** Rejected: not guaranteed on operator machines; fragile Windows install.
- **Node:** Rejected: Claude binary is standalone, not npm-installed.
- **Bash:** Rejected: no Windows support without git-bash.

## Source / Origin

`/Users/jmagady/Dev/vsdd-factory/.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
lines 399–416 (ADR-001 section) and lines 724–731 (Open Question Q1 resolution).
