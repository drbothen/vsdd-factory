---
name: activate
description: Opt in to the VSDD factory persona for this project. Detects the host platform, persists the detected platform string, and writes `.claude/settings.local.json` to set the orchestrator as the default main-thread agent. Reversible via `/vsdd-factory:deactivate`.
disable-model-invocation: true
---

# Activate VSDD Factory

Per-project opt-in. Enabling the plugin alone does not change your default Claude persona — it only makes the factory's agents, skills, and hooks available. Running this skill flips the default agent to `orchestrator` so that a plain session becomes the VSDD pipeline driver, and (in v1.0+) records the host platform so the dispatcher copies the right per-platform `hooks.json` variant into place.

## Procedure

1. **Confirm the user is inside a project that wants VSDD.** Check for `.factory/` and `.factory/STATE.md`. If missing, ask whether to continue anyway (you can activate before initializing).

2. **Detect the host platform.** Run the detector matching your active shell:

   - **bash / sh / zsh / Git Bash / WSL** (macOS, Linux, Windows-with-Git-Bash): run `${CLAUDE_PLUGIN_ROOT}/skills/activate/detect-platform.sh`
   - **PowerShell** (native Windows without Git Bash, Claude Code v2.1.84+): run `${CLAUDE_PLUGIN_ROOT}/skills/activate/detect-platform.ps1`

   Both produce identical JSON output and use identical exit codes. Capture stdout.

   - On `exit 0`, the helper returns one of the 5 canonical platform strings the v1.0 dispatcher binaries are built for: `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `windows-x64`.
   - On `exit 1`, the platform is unsupported (e.g., FreeBSD, 32-bit). Print the helper's `detected_from.raw_uname` and tell the user vsdd-factory v1.0 has no dispatcher binary for that host. Do not proceed; activation aborts.

3. **Read existing `.claude/settings.local.json`.** If it doesn't exist, create an empty `{}`. If it does, parse it with `jq`.

4. **Drift check on re-activation.** If `.vsdd-factory.activated_platform` already exists in the settings file and does not match the platform detected in step 2, surface a clear warning:

   > "vsdd-factory was last activated on `<persisted>`, but this host is `<current>`. The dispatcher binary on disk may not match this host. Re-running activation will update the recorded platform."

   Continue activation after warning; persisting the new platform is the recovery path for legitimate cross-host moves (e.g., the operator SSH'd into a Linux box on a `.factory/` originally activated on macOS).

5. **Merge the activation block.** Write the file back with all three fields merged:

   ```json
   {
     "agent": "vsdd-factory:orchestrator:orchestrator",
     "vsdd-factory": {
       "activated_platform": "<canonical platform string>",
       "activated_at": "<ISO 8601 timestamp with timezone>",
       "activated_plugin_version": "<version from plugin.json>"
     }
   }
   ```

   Preserve all other top-level keys.

6. **Apply the per-platform variant + verify the dispatcher binary.** Run the applier matching your active shell:

   - **bash / sh / zsh / Git Bash / WSL**: `${CLAUDE_PLUGIN_ROOT}/skills/activate/apply-platform.sh <platform>`
   - **PowerShell**: `${CLAUDE_PLUGIN_ROOT}/skills/activate/apply-platform.ps1 <platform>`

   Both implementations share the same exit-code contract and diagnostic messages. Either one:
   - Copies `${CLAUDE_PLUGIN_ROOT}/hooks/hooks.json.<platform>` to `${CLAUDE_PLUGIN_ROOT}/hooks/hooks.json` (the canonical file is gitignored per S-0.4 — it's per-machine).
   - Verifies the dispatcher binary at `${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]` is present and (on Unix) executable.

   Exit codes: `0` success; `1` variant missing (corrupted install); `2` binary missing (release didn't commit it for this platform yet); `3` binary not executable (Unix only — Windows has no executable bit); `4` usage error. Surface the helper's stderr to the user verbatim — it includes restoration instructions for the binary-missing case (pin to 0.79.4 or build locally) which the user needs to act on.

   Until S-2.4 wires the binary commit into the release workflow, exit `2` is the expected outcome on a fresh install. That's the current state of v1.0-beta development; do not silently ignore it — the warning surfaces it for the operator.

7. **Confirm activation.** Print:
   - File written
   - Detected platform
   - New default agent
   - How to deactivate (`/vsdd-factory:deactivate`)
   - Reminder that this only affects the current project (`.claude/settings.local.json` is per-project)

8. **Suggest CLAUDE.md scaffolding.** If no `CLAUDE.md` exists at the project root, print:
   > "Tip: Run `/vsdd-factory:scaffold-claude-md` to auto-generate project-specific build, test, and git instructions for Claude Code."

## Dry-run mode

If the user invokes the skill with `--dry-run` (or asks for a preview), perform steps 1–4 but skip the file write and the hooks.json copy. Print the proposed settings.local.json diff and the platform that would be persisted, then exit. This lets operators inspect activation behavior on unfamiliar hosts without committing any change.

## Notes

- **Per-project, not shared.** `settings.local.json` is typically gitignored, so teammates opt in individually.
- **Platform persistence is forward-looking.** v0.79.x ignores `vsdd-factory.activated_platform`; v1.0+ reads it during S-2.6 to pick the right hooks.json variant. Recording it now means the v0.79 → v1.0 upgrade has the data it needs in place.
- **No "hijack on enable".** Plugin-level `settings.json` (which would set `agent` automatically) is the alternative we deliberately did not choose. Activation is always an explicit user action.
- **Detection helper has a test override.** `MOCK_UNAME_S` and `MOCK_UNAME_M` env vars bypass real platform detection in both `detect-platform.sh` and `detect-platform.ps1` — see `plugins/vsdd-factory/tests/activate.bats` for the bash matrix. (Pester coverage for the `.ps1` siblings is tracked in tech debt — see TD-019.)
- **Flag conventions across the bash/PowerShell sibling pair.** Both PowerShell-style flags (`-Help`, `-Check`) and bash-style aliases (`--help`, `-h`, `--check`) are accepted by the `.ps1` siblings, so cross-shell muscle memory does not produce exit-4 surprises. Internally, the JSON output, exit codes, and diagnostic strings are byte-for-byte identical between `.sh` and `.ps1`; only the *invocation* syntax differs by host shell convention.

## See also

- `/vsdd-factory:deactivate` — reverse this
- `/vsdd-factory:scaffold-claude-md` — generate a project-specific CLAUDE.md
- Orchestrator agent: `${CLAUDE_PLUGIN_ROOT}/agents/orchestrator/orchestrator.md`
- Detection helpers: `${CLAUDE_PLUGIN_ROOT}/skills/activate/detect-platform.sh` (bash) and `${CLAUDE_PLUGIN_ROOT}/skills/activate/detect-platform.ps1` (PowerShell)
- Apply helpers: `${CLAUDE_PLUGIN_ROOT}/skills/activate/apply-platform.sh` (bash) and `${CLAUDE_PLUGIN_ROOT}/skills/activate/apply-platform.ps1` (PowerShell)
- v1.0 design context: `.factory/specs/2026-04-24-v1.0-factory-plugin-kit-design.md`
