# Migrating from v0.79.x to v1.0

> **Status:** skeleton — filled in by S-5.5 (operator-facing migration
> guide) and validated by S-5.7 (gate 16) against at least one real
> 0.79.x factory that isn't vsdd-factory itself. Greppable `TODO(S-X.Y)`
> markers gate the 1.0.0 release per S-5.7 acceptance criteria.

Operators upgrading an existing v0.79.x factory to v1.0 read this guide.
It walks through what changed, what the upgrade requires, what works
out-of-the-box, what doesn't yet on Windows, and how to roll back if
something goes wrong.

## What changed

<!-- TODO(S-5.5): the headline list — Rust dispatcher binary replaces
     hook-by-hook bash dispatch; WASM plugins replace bash scripts (with
     legacy-bash-adapter as a compatibility layer on Linux/macOS); new
     activation step required after install; observability config moves
     to observability-config.toml; per-platform binaries committed to
     the plugin package. -->

## Why v1.0

<!-- TODO(S-5.5): the matcher-bug motivation, cross-platform support,
     observability flexibility (multi-instance multi-backend sinks),
     capability-sandboxed plugins. Keep this paragraph honest — the
     immediate driver was unfixable upstream behavior, not a roadmap
     item. -->

## Prerequisites

<!-- TODO(S-5.5): Claude Code minimum version, git, jq. NOT required:
     Python (never was), Node (never was). Windows users need git-bash
     for the hooks still routed through legacy-bash-adapter. -->

## Upgrade procedure

<!-- TODO(S-5.5): the sequence — `/plugin update vsdd-factory@vsdd-factory`,
     `/vsdd-factory:activate` (NEW required step), full Claude Code
     session restart, `/vsdd-factory:factory-health` to verify. -->

## Verification checklist

<!-- TODO(S-5.5): how an operator confirms the upgrade landed — make a
     test commit and see `commit.made` event flow through, confirm
     existing dashboards still render (file-sink default preserves
     0.79.x event JSONL behavior), check
     `.factory/logs/dispatcher-internal.jsonl` exists and has entries. -->

## Observability migration

<!-- TODO(S-5.5): default config matches 0.79.x file-sink behavior so
     existing Grafana dashboards keep working with no changes. To add
     Datadog / Honeycomb / OTel-grpc, write `observability-config.toml`.
     Zero-disk mode is opt-in (disable file sink). -->

## Windows-specific notes

<!-- TODO(S-5.5): native WASM hooks that work on Windows in 1.0
     (capture-commit-activity, capture-pr-activity, block-ai-attribution,
     plus emit_event as a host function). Other 26 hooks remain on the
     legacy-bash-adapter and require git-bash on Windows. Frame
     expectation: more native ports will follow post-1.0. -->

## Known regressions

<!-- TODO(S-5.5): empty if S-2.7's regression sweep finds none. If any
     emerge from beta/rc shakedown, document them here with workarounds
     and tracked-issue links. -->

## Rollback

<!-- TODO(S-5.5): explicit rollback steps if 1.0 misbehaves on an
     operator's factory — pin to 0.79.4, deactivate the dispatcher,
     restart session. Cover the case where v1.0 already wrote new
     observability-config.toml. -->

## Troubleshooting

<!-- TODO(S-5.5): common symptoms and fixes — dispatcher not firing
     (activation step missed?), Datadog 401 (sink config / API key?),
     legacy-bash-adapter "command not found" (git-bash missing on
     Windows?), event field schema drift (HOST_ABI_VERSION skew?). -->
