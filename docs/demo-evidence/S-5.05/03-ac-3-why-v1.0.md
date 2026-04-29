# AC-3 — "Why v1.0" section

**AC statement:** Section "Why v1.0" populated.

**Evidence type:** file snippet

## Section heading

```markdown
## Why v1.0
```

## Content snippet (lines 33-52)

```markdown
The immediate driver was an unfixable bug in the upstream bash dispatch
pipeline: the v0.79.x hook matcher silently dropped events under certain
terminal width conditions, causing hooks to not fire without any error
message. The bug lived in behavior baked into the Claude Code shell
integration layer and could not be patched from the plugin side.

Rather than ship a workaround, the decision was to move to a stable ABI.
v1.0 provides:

- **Reliable event delivery** via the Rust dispatcher's typed event
  deserialization — no more silent drops from bash string-matching bugs.
- **Cross-platform support** — the dispatcher compiles for darwin-aarch64,
  linux-x64, and windows-x64; bash-only hooks were not portable to Windows.
- **Observability flexibility** — multiple simultaneous sinks (file, Datadog,
  Honeycomb, OTel-grpc) via `observability-config.toml`, rather than the
  single hardcoded file path in v0.79.x.
- **Capability-sandboxed plugins** — WASM plugins declare which host
  capabilities they need; the runtime enforces the allow-list.
```

## Commentary

Section explains the matcher-bug root cause, cross-platform motivation,
observability improvements, and capability sandboxing — the four pillars
cited in the AC story spec (Task 4).
