---
document_type: investigation-report
cycle: v1.0-feature-plugin-async-semantics-pass-1
created: 2026-05-07
subject: S-15.01 lint hook registration mechanism
finding: pass-1 fix burst cited a non-existent settings.json hook mechanism
---

# F-P2-001: Hook Registration Mechanism Investigation

## 1. What S-13.01 Actually Did

S-13.01 (story: "Path Governance Bundle — Registry, WASM Hook, Skill Updates,
relocate-artifact") delivered `validate-artifact-path` as a **native WASM plugin**,
not a bash hook. The concrete mechanism:

- **Crate:** `crates/hook-plugins/validate-artifact-path/` — Rust compiled to WASM
- **Binary output:** `hook-plugins/validate-artifact-path.wasm`
- **Registration:** `plugins/vsdd-factory/hooks-registry.toml`, entry at priority 150
  under event `PreToolUse`, tool `Write|Edit`, using the `plugin =` field directly
  (no `[hooks.config] script_path` — this is the native-WASM form, not the
  legacy-bash-adapter form)
- **Capabilities:** `[hooks.capabilities.read_file]` path_allow for the registry YAML
- **on_error:** `continue` (not block, despite being a governance hook)

S-13.01 did NOT touch `.claude/settings.json`. That file contains only
`enabledPlugins: {vsdd-factory@claude-mp: true}` — no hook array, no hook
scaffolding of any kind. Settings.json is a plugin-enable toggle, not a hook
registration surface.

## 2. Bash-Script Hook Layer Status

The bash-script hook layer DOES exist and is actively used. It is NOT deprecated.
The pattern is `hooks-registry.toml` entries with:

```toml
plugin = "hook-plugins/legacy-bash-adapter.wasm"
[hooks.config]
script_path = "hooks/<name>.sh"
```

The actual bash scripts live at `plugins/vsdd-factory/hooks/*.sh`. There are ~30
bash hooks currently registered this way (convergence-tracker, purity-check,
validate-factory-path-root, validate-template-compliance, etc.).

Two bash hooks already use `on_error = "block"` (the semantics S-15.01 needs):
- `validate-factory-path-root` (priority 280, block)
- `validate-input-hash` (priority 310, block)
- `validate-template-compliance` (priority 400, block)
- `brownfield-discipline` (priority 30, PreToolUse, block)
- `protect-bc` (priority 70, PreToolUse, block)
- `red-gate` (priority 100, PreToolUse, block)

The bash-adapter pattern is the established, actively-used pattern for hooks that
do not yet warrant a native WASM port. It is structurally identical to what S-15.01
needs: a PostToolUse Edit|Write hook with `on_error = "block"`.

## 3. Available Options Analysis

### Option A — Native WASM plugin (matching S-13.01 exactly)

**What it means:** New Rust crate in `crates/hook-plugins/validate-async-lint/`,
compiled to WASM, registered in hooks-registry.toml with `plugin =` (no bash adapter).

**Effort:** Large. New crate, new build pipeline entry, Kani/proptest harness, WASM
compilation, full BC and VP scaffolding. 5-8 days implementation.

**Architectural fit:** Matches S-13.01's literal pattern. Sets precedent for async
lint as a first-class WASM hook. Appropriate if the lint logic is complex or needs
formal verification.

**Interaction with ADR-019 §Decision 4:** "Three-layer defense" is preserved with
full strength at Layer 1 (blocking edit-time enforcement). No amendment needed.

**Risks:** Overengineered for what may be a simple regex/grep scan. Slows the feature
cycle. Native WASM is the S-2.5+ migration target, but the story says legacy-bash-adapter
can "coexist" — this is not required now.

### Option B — CI-only enforcement (drop edit-time layer)

**What it means:** Remove AC-007 and T-3i entirely. The `on_error = "block"` invariant
scan only runs in CI (GitHub Actions), not at edit time inside Claude Code.

**Effort:** Small. No new hook. Adjust AC-007, T-3i, ADR-019 §Decision 4 claim.

**Architectural fit:** Weakens the three-layer defense ADR-019 §Decision 4 claims.
ADR-019 would need amendment to change "three layers" to "two layers." That file is
described as "already converged" in the task context — amending it is a side effect.

**Interaction with ADR-019 §Decision 4:** REQUIRES amendment. The three-layer claim
(edit-time hook + pre-commit + CI) becomes a two-layer claim. This is the most
invasive option architecturally.

**Risks:** Reduced defect catch rate. Contradicts the "most correct" principle.
Also wastes the existing hook infrastructure that already handles this pattern.

### Option C — Bash script via legacy-bash-adapter (the actual established pattern)

**What it means:** Write `plugins/vsdd-factory/hooks/validate-async-lint.sh`, register
it in `hooks-registry.toml` as a PostToolUse Edit|Write hook via `legacy-bash-adapter.wasm`
with `on_error = "block"` and an appropriate priority (e.g., 295 between validate-finding-format
and validate-index-self-reference).

**Effort:** Small-medium. The bash script itself is a grep/awk scan (~50-100 lines).
The registry entry follows the exact same template as `validate-factory-path-root.sh`
or `validate-template-compliance.sh`. No new crate, no build pipeline change.

**Architectural fit:** Perfect match. This is the established pattern for exactly this
class of hook. Multiple existing hooks at `on_error = "block"` use this pattern.
The hooks-registry.toml comment explicitly states: "Native-WASM ports replace individual
entries one-by-one; legacy entries can coexist."

**Interaction with ADR-019 §Decision 4:** No amendment needed. Three-layer defense is
fully preserved. Layer 1 = this bash hook (blocking). Layer 2 = pre-commit. Layer 3 = CI.

**Risks:** Minimal. Only risk is that story-writer must NOT describe this as
"registered in .claude/settings.json" — it must cite hooks-registry.toml. Shell script
must handle the `CLAUDE_PLUGIN_ROOT` env var correctly (all existing hooks use it).

### Option D — Pre-commit Git hook

**What it means:** Add a `.git/hooks/pre-commit` or `scripts/pre-commit` entry that
runs the async lint check before every commit.

**Effort:** Small. But orthogonal to the Claude Code hook lifecycle.

**Architectural fit:** Poor. Pre-commit hooks fire on human commits and CI, not on
Claude Code write operations. The invariant is `on_error = "block"` for Claude writes.
Pre-commit does not intercept Claude's Edit/Write tool calls. This would leave the
edit-time layer empty.

**Interaction with ADR-019 §Decision 4:** Would need amendment — a git pre-commit
hook is a different layer than a Claude Code PostToolUse hook.

**Risks:** Does not satisfy the AC-007 requirement. Wrong lifecycle.

## 4. Recommendation

**Option C — bash script via legacy-bash-adapter in hooks-registry.toml.**

Rationale:
- This is the canonical existing pattern for PostToolUse blocking hooks in this codebase
- At least 6 existing hooks use `on_error = "block"` via the legacy-bash-adapter pattern
- No new crate, no build pipeline changes, no WASM compilation required
- ADR-019 §Decision 4 three-layer claim is fully preserved without amendment
- The hooks-registry.toml comment explicitly endorses coexistence of bash and WASM hooks
- Effort is small-medium (1 day), matching the complexity of the lint logic itself
- The bash adapter pattern has been validated at `validate-factory-path-root.sh`,
  `validate-template-compliance.sh`, and `validate-input-hash.sh` — all blocking hooks
  in the same class as what S-15.01 needs

The "most correct" path is not Option A (overengineered, wrong phase) and not Option B
(weakens the architecture). Option C is the correct, established, low-risk pattern.

## 5. Implications for S-15.01

The following story artifacts must be corrected to cite the actual mechanism:

**AC-007** must read: "A bash hook script `plugins/vsdd-factory/hooks/validate-async-lint.sh`
exists and is registered in `plugins/vsdd-factory/hooks-registry.toml` as a
PostToolUse `Edit|Write` hook via `legacy-bash-adapter.wasm` with `on_error = "block"`."
Must NOT say: "registered in `.claude/settings.json` as a Claude Code PostToolUse hook."

**T-3i** must reference:
- File to create: `plugins/vsdd-factory/hooks/validate-async-lint.sh`
- Registration: new `[[hooks]]` entry in `plugins/vsdd-factory/hooks-registry.toml`
- Test: trigger an Edit of a .factory/ file with a `async = false` hook declaration
  and verify the hook blocks

**Previous Story Intelligence** must cite:
- S-13.01 precedent = WASM native hook for validate-artifact-path (NOT bash)
- All bash blocking hooks (validate-factory-path-root, validate-input-hash,
  validate-template-compliance) as the correct precedent for S-15.01's lint hook
- Registration mechanism = hooks-registry.toml via legacy-bash-adapter

**File List** must include:
- `plugins/vsdd-factory/hooks/validate-async-lint.sh` (new file)
- `plugins/vsdd-factory/hooks-registry.toml` (modified — new entry appended)
- Must NOT include `.claude/settings.json` as a modified file

## 6. Side Effects on Converged Specs

**ADR-019:** No amendment required if Option C is adopted. The three-layer claim
in §Decision 4 is preserved. The only impact is that the story's internal references
to the mechanism must be corrected — ADR-019 itself does not specify which sub-layer
technology implements Layer 1.

**BC-7.06.001 Postconditions:** Likely cites the hook mechanism in postcondition
language. Must be checked: if postconditions say "registered in settings.json," they
need amendment to say "registered in hooks-registry.toml via legacy-bash-adapter."
This is a targeted fix, not a structural change to the BC.

**VP-INDEX.md / verification-coverage-matrix.md:** If S-15.01 has associated VPs
that reference the mechanism (e.g., a VP testing the hook registration itself), their
proof harness descriptions may reference the wrong mechanism. Check VP entries
associated with S-15.01 before closing the story.

**No other already-converged specs are affected.** The architecture is sound; only
the story's internal mechanism description was wrong.
