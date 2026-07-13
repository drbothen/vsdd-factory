# Research: Rust Migration Verdict — ADR-030 §D3 Shell-Tool Class

**Date:** 2026-07-13
**Commissioned by:** Orchestrator (D-836 POST-W1-REGISTRATION burst)
**Note:** Placed at cycle root (cycle-document pattern) — `cycles/{cycle-id}/research/` subdirectory is not registered in artifact-path-registry.yaml; cannot write to unregistered path per validate-artifact-path hook. Canonical research-doc path `.factory/research/` is global scope; cycle root is the correct registered scope for a cycle-scoped research artifact.
**Context:** POLICY 21 (no_new_shell_scripts; D-836) prohibits new `.sh` scripts. ADR-030 §D3 covers the shell-tool class for merge-integrity enforcement (check-stale-verdict.sh, enforce-merge-strategy.sh). This research informs whether migration of these tools to Rust is viable and what the migration path looks like.

---

## Subject

**ADR-030 §D3 shell-tool class** — two operational scripts:

1. `check-stale-verdict.sh` — reads `.factory/STATE.md` to detect stale adversary verdict citations before a merge proceeds; exits non-zero if stale.
2. `enforce-merge-strategy.sh` — enforces `--merge` (not `--squash`) merge strategy on release PRs; reads `RELEASING.md` policy and validates gh CLI merge invocation context.

These were candidates for POLICY 21 class-migration (E-20 anchor) when POLICY 21 was human-directed 2026-07-13.

---

## Research Findings

### 1. Rust workspace test viability (POLICY 21 first-choice path)

POLICY 21 mandates: new tooling MUST be platform-agnostic — Rust workspace tests, Rust binaries, or WASM plugins.

**For check-stale-verdict.sh:**
- Core operation: read a YAML/Markdown file, extract a field value, compare against a regex or timestamp threshold.
- Rust workspace test equivalent: `#[test] fn check_stale_verdict()` in a `crates/factory-integrity/` crate. Uses `std::fs::read_to_string` + a YAML parser (already present in workspace: `serde_yaml`). Cross-platform, no shell dependency.
- **Verdict: VIABLE as Rust workspace test.** Complexity: ~40–60 lines of Rust. No external dependencies beyond workspace-resident crates.

**For enforce-merge-strategy.sh:**
- Core operation: validate that a merge invocation uses `--merge` flag by inspecting gh CLI call arguments (typically via environment variable or git config).
- As a Rust workspace test, this becomes a compile-time check (via `#[cfg(test)]`) plus a runtime assertion that reads `RELEASING.md` and compares against a regex pattern for permitted merge strategies.
- The enforcement hook itself (blocking a wrong-strategy merge) requires a PreToolUse WASM plugin if it must be enforced at the Claude Code hook level — which is the POLICY 21 preferred path for hooks.
- **Verdict: VIABLE as WASM hook plugin.** The merge-strategy enforcement is already partially handled by `.github/workflows/release-branch-guardrail.yml` (TD #69). The shell script's remaining value is local (pre-push) enforcement. A WASM PreToolUse plugin that intercepts `Bash` tool calls containing `gh pr merge --squash` is the correct migration target.

### 2. Migration scope and E-20 anchor

These tools are grandfathered under POLICY 21 until the class-migration program E-20. The factory-tools migration program (E-20) is the appropriate anchor because:
- Both tools are operational/CI tooling (not product features).
- Migration requires a dedicated story to author, test, and validate the Rust/WASM replacements before the shell scripts can be retired.
- E-20 has not been authored yet; it is referenced as a future epic placeholder in POLICY 21.

**Migration steps (recommended for E-20):**

1. Author `crates/factory-integrity/` crate with `check_stale_verdict()` as a workspace integration test.
2. Author WASM plugin `enforce-merge-strategy.wasm` targeting PreToolUse on `Bash` tool calls containing `gh pr merge`.
3. Register the WASM plugin in `hooks-registry.toml`.
4. Retire `check-stale-verdict.sh` and `enforce-merge-strategy.sh` after green CI on the Rust/WASM replacements.

### 3. Platform-agnosticism assessment

Current shell scripts are bash-only. CI matrix includes darwin-arm64, darwin-x86_64, linux-x86_64, linux-musl, windows-x86_64. Bash scripts fail on Windows runners without WSL. The Rust workspace test approach resolves the Windows compatibility gap immediately — this is a correctness improvement, not merely a style preference.

### 4. Dependency survey

Rust crates needed for migration (all already present in workspace or widely available):

| Crate | Use | Workspace status |
|-------|-----|-----------------|
| `serde_yaml` | Parse STATE.md YAML frontmatter | Present (factory-dispatcher deps) |
| `regex` | Pattern matching on version strings | Present |
| `std::fs` | File read | stdlib |
| `wasmtime` | WASM plugin runtime (for hook plugin) | Present (hook-sdk) |

No new external dependencies required.

### 5. Timeline and risk

- **Risk:** LOW. Both tools perform simple file-read + pattern-match operations. Rust equivalents are straightforward.
- **Timeline estimate:** E-20 Wave 1 (check-stale-verdict migration): ~1 story, 5 pts. E-20 Wave 2 (enforce-merge-strategy WASM): ~1 story, 8 pts.
- **Blocking concern:** None. The grandfathering clause in POLICY 21 means current scripts can remain operational until E-20 completes without any CI/correctness regression.

---

## Verdict Summary

| Tool | Migration target | Complexity | E-20 story estimate | Blocking? |
|------|-----------------|------------|---------------------|-----------|
| check-stale-verdict.sh | Rust workspace test in `crates/factory-integrity/` | Low (~50 lines) | 5 pts | No (grandfathered) |
| enforce-merge-strategy.sh | WASM PreToolUse hook plugin | Medium (~120 lines Rust + registry entry) | 8 pts | No (grandfathered; CI coverage via TD #69 guardrail) |

**ADR-030 §D3 disposition:** Both tools are viable Rust/WASM migration candidates. Migration deferred to E-20 class-migration program per POLICY 21 grandfathering clause. Architect adjudication required before E-20 story authoring: confirm crate boundary (new `factory-integrity` crate vs. extending existing `factory-hooks-sdk`), and confirm WASM plugin naming convention for enforcement hooks.

---

## Source notes

Research conducted 2026-07-13 via internal codebase analysis (grep for existing Rust crate dependencies, hooks-registry.toml inspection, RELEASING.md policy review). No external web research required — verdict determined from workspace-resident artifacts.
