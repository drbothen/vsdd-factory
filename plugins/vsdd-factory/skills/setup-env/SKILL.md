---
name: setup-env
description: Validate and provision the development environment — check required tools, verify versions, run health checks on MCP servers, and set up project tooling (lefthook, just, cargo tools).
disable-model-invocation: true
allowed-tools: Bash, Read, Write
---

# Environment Setup

Validate and provision the development environment for this project.

## Tool Checks

For each tool, check availability and version:

### Required Tools

| Tool | Check | Install |
|------|-------|---------|
| `rustc` | `rustc --version` (≥ 1.85) | `rustup update stable` |
| `cargo` | `cargo --version` | comes with rustc |
| `rustfmt` (nightly) | `cargo +nightly fmt --version` | `rustup component add rustfmt --toolchain nightly` |
| `clippy` | `cargo clippy --version` | `rustup component add clippy` |
| `git` | `git --version` | `brew install git` |
| `gh` | `gh --version` | `brew install gh` |
| `just` | `just --version` | `brew install just` or `cargo install just` |
| `jq` | `jq --version` | `brew install jq` |

### Optional Tools (install when needed)

| Tool | Purpose | Check | Install |
|------|---------|-------|---------|
| `cargo-kani` | Formal verification | `cargo kani --version` | `cargo install cargo-kani` |
| `cargo-fuzz` | Fuzz testing | `cargo fuzz --version` | `cargo install cargo-fuzz` |
| `cargo-mutants` | Mutation testing | `cargo mutants --version` | `cargo install cargo-mutants` |
| `cargo-deny` | Dependency auditing | `cargo deny --version` | `cargo install cargo-deny` |
| `semgrep` | Security scanning | `semgrep --version` | `pip install semgrep` |
| `lefthook` | Git hooks | `lefthook --version` | `brew install lefthook` |
| `asciinema` | Terminal recording | `asciinema --version` | `brew install asciinema` |
| `hyperfine` | Benchmarking | `hyperfine --version` | `brew install hyperfine` |

### MCP Server Health

Check configured MCP servers in `.mcp.json`:

```bash
# Verify environment variables are set
echo $PERPLEXITY_API_KEY | head -c 4
echo $TAVILY_API_KEY | head -c 4
```

Report which servers are configured and whether their env vars are set.

## Git Configuration

Verify:
```bash
git config rerere.enabled  # Should be true
git config user.name
git config user.email
```

Set `rerere.enabled` if not already set:
```bash
git config rerere.enabled true
```

## Factory Health

Run `/factory-health` as part of environment setup.

## Output

```
Environment Status:

Required Tools:
  ✅ rustc 1.85.0
  ✅ cargo 1.85.0
  ✅ rustfmt (nightly)
  ✅ clippy
  ✅ git 2.47.0
  ✅ gh 2.65.0
  ⚠️ just — not installed (brew install just)

Optional Tools:
  ✅ cargo-deny
  ❌ cargo-kani — not installed (needed for Phase 5)
  ❌ cargo-fuzz — not installed (needed for Phase 5)

MCP Servers:
  ✅ perplexity — API key set
  ⚠️ tavily — API key not set
  ✅ playwright — available
  ✅ tally — available

Git Config:
  ✅ rerere.enabled = true
  ✅ user.name = <name>

Factory:
  ✅ .factory/ healthy

Missing required tools: <list>
Missing optional tools: <list>
```
