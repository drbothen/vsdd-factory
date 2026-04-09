# Pass 6 — Synthesis & Lessons for vsdd-factory

## Executive summary

13 reference plugins, ~26k LOC (almost all markdown), exemplifying 6 patterns. `plugin-dev/` is the canonical reference bundle — ironically lacking a `plugin.json`. Plugins are static file trees loaded by convention via `.claude-plugin/plugin.json` + auto-discovered component directories. `${CLAUDE_PLUGIN_ROOT}` is the sole portability mechanism for command paths.

## Top 5 findings

1. **Skills auto-activate; agents are explicitly spawned.** Distinct mechanisms, do not conflate.
2. **Minimal manifests win.** 12/12 plugins use 4 manifest fields total; zero use custom path overrides.
3. **Hook contract is exit-code + stderr + stdout-JSON**, not documented in one place — you must read `security_reminder_hook.py` + `session-start.sh`.
4. **"Output style" is deprecated as a concept** — reimplemented as a SessionStart hook emitting `additionalContext`.
5. **No tests anywhere** — every contract caps at HIGH-from-code, never HIGH-from-tests.

## Cross-reference

- Pass 1 ↔ Pass 5: consistent.
- Pass 2 ↔ Pass 3: consistent; every concept has ≥1 HIGH-confidence contract.
- Pass 4 ↔ Pass 5: **collision** — `${CLAUDE_PLUGIN_ROOT}` convention is for *command* paths, but plugins that write *data* have no convention (`/tmp` or `~/.claude` ad-hoc). Missing convention.

## Gaps (7)

- **G1** No tests
- **G2** `plugin-dev/` has no `plugin.json` (undocumented)
- **G3** No concrete `.mcp.json` examples
- **G4** `SessionEnd`, `SubagentStop`, `PreCompact`, `Notification` unexercised
- **G5** No convention for plugin state-file locations
- **G6** No `license` SPDX in any `plugin.json`
- **G7** `tools:` as bare comma-string is fragile

## Inconsistencies (3)

- Hook handler dir (`hooks/` vs `hooks-handlers/`)
- Skill `name:` casing (Title vs kebab)
- Fail-open vs fail-closed policy undeclared

## Lessons for vsdd-factory (actionable)

**L1. Manifest path is load-bearing**: must be `./.claude-plugin/plugin.json` exactly. If vsdd-factory has it elsewhere, plugin won't load. **CHECK this first.**

**L2. Classify every capability by activation mode**: user-initiated workflow → command; command- or model-spawned worker → agent; silent auto-trigger on user intent → skill. Don't ship an "agent" that should be a skill.

**L3. Skill descriptions must enumerate trigger phrases**. Strongest observed pattern. Rewrite every vsdd-factory `SKILL.md` description to list specific phrases ("run wave gate", "decompose stories", "check spec drift", ...).

**L4. Never use `/tmp/` or `~/.claude/` for plugin state**. Use `.factory/` exclusively (vsdd-factory already has this). Document as our own NFR. `security-guidance`'s `/tmp/security-warnings-log.txt` is the anti-pattern to avoid.

**L5. Declare hook fail-open/fail-closed explicitly per hook**. Add a header comment to every hook. Use exit 2 + stderr only when we genuinely want to block. Use stdout JSON `hookSpecificOutput.additionalContext` for context injection. Try/except stdin parse → fail-open (matches Anthropic).

**L6. Grep our plugin for hardcoded paths**, `$HOME`, `~/`, absolute paths — replace with `${CLAUDE_PLUGIN_ROOT}` in all `hooks.json` and `.mcp.json`.

**L7. Minimal manifest**. Do NOT invent custom `commands`/`agents` path overrides. Directories named `knowledge/`, `prompts/`, `playbooks/`, `workflows/` at plugin root will NOT be auto-discovered — they must be referenced explicitly from commands/agents/skills, or moved under `skills/<name>/references/`.

**L8. `tools:` is a comma-separated bare string**, not a YAML list. Every Anthropic agent uses this. Don't use `tools: [Glob, Grep]`.

**L9. `allowed-tools` subcommand globbing is our only compile-time blast-radius control** — use aggressively. Every vsdd-factory command that shells out should declare tight `Bash(cargo test:*)`, `Bash(git log:*)`, etc.

**L10. Commands can embed live output via `` !`cmd` ``**. vsdd-factory's `/factory-health` can embed `` !`ls .factory` `` and `` !`cat .factory/STATE.md` `` directly instead of asking the agent to fetch.

**L11. Ship tests even though Anthropic doesn't.** CI should validate: `.claude-plugin/plugin.json` exists, all referenced scripts exist, `hooks.json` is valid JSON and uses `${CLAUDE_PLUGIN_ROOT}`, skill `name:` is kebab-case. Defends against G2, G7.

**L12. Standardize skill `name:` on kebab-case** (Anthropic can't agree — pick and enforce via convention checker).

**L13. ⚠️ If vsdd-factory has an "output style" concept, rewrite it as a SessionStart hook now.** There is no first-class output-style construct. `explanatory-output-style/hooks-handlers/session-start.sh` is the reference pattern.

## ⚠️ Claims worth verifying against vsdd-factory's current design

- **CHECK**: Manifest path is `./.claude-plugin/plugin.json` (not root, not `plugin.toml`).
- **CHECK**: No agent uses `tools:` as YAML list — must be comma-separated bare string.
- **CHECK**: No hook command uses absolute paths / `$HOME`.
- **CHECK**: Every `SKILL.md` description enumerates trigger phrases, not abstract prose.
- **CHECK**: Any "output style" is actually a SessionStart hook.
- **CHECK**: No directories named `knowledge/`, `prompts/`, `playbooks/`, `workflows/` at plugin root expect auto-discovery.
- **CHECK**: vsdd-factory's "default agent" pattern — Anthropic has no such concept. Every agent is explicitly spawned via `Task`. If we rely on implicit agent selection, we diverge and should make activation explicit (or recast as a skill).

## Confidence

Architecture HIGH, Domain HIGH, Contracts HIGH-from-code (18/24 HIGH), NFRs MEDIUM (inferred), Conventions HIGH (observed).
