# AC-7 — Troubleshooting (>= 5 issues)

**AC statement:** Section "Troubleshooting" populated with at least 5
common issues with resolutions.

**Evidence type:** file snippet + count

## Verification command

```
grep -c "^- \*\*" docs/guide/migrating-from-0.79.md
```

(Counts bold-bullet items in the Troubleshooting section.)

## Section content (lines 208-244) — 5 distinct issues

```markdown
## Troubleshooting

- **Dispatcher not firing / hooks not running.** The most common cause is
  a missed activation step. Run `/vsdd-factory:activate` and then restart
  your Claude Code session completely.

- **Datadog 401 Unauthorized.** The Datadog sink is returning an auth
  error. Check `observability-config.toml` — confirm the `api_key` value
  is set and matches a valid Datadog API key for your account. Also
  confirm the `site` value matches your Datadog region (e.g.,
  `datadoghq.com` vs `datadoghq.eu`).

- **`legacy-bash-adapter`: command not found (Windows).** The adapter is
  trying to invoke a bash hook but cannot find the bash interpreter. On
  Windows, the adapter uses git-bash. Install git-bash from
  https://gitforwindows.org/ and ensure it is in your system `PATH`.

- **Event field schema drift / unexpected fields missing.** If a hook
  or downstream consumer expects fields that are no longer present (or
  new fields that weren't there before), this indicates a `HOST_ABI_VERSION`
  skew. Run `/plugin update vsdd-factory@vsdd-factory` to ensure both
  the dispatcher binary and the WASM artifacts are from the same release.

- **Platform binary mismatch / dispatcher fails to start.** The dispatcher
  is a compiled Rust binary; one binary is included per supported platform
  in the plugin package. If the binary for your OS/arch is missing from the
  package (a packaging or release artifact issue, not an activation problem),
  the dispatcher will fail to start entirely rather than silently dropping
  events.
```

## Commentary

5 distinct issues enumerated:
1. Dispatcher not firing (missed activation)
2. Datadog 401 (auth config)
3. legacy-bash-adapter command not found (Windows/git-bash)
4. Event schema drift (HOST_ABI_VERSION skew)
5. Platform binary mismatch (packaging artifact issue — distinct root cause from issue 1)

Issue 5 is the AC-7-required addition beyond the 4 skeleton hints,
rooted in cross-platform binary distribution (not activation-step failure).
