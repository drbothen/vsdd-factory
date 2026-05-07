---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "[pending-recompute]"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
origin: greenfield
subsystem: "SS-04"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.12.003
section: "4.12"
last_amended: 2026-05-07
---

# BC-4.12.003: Resolver plugins MUST operate within explicitly declared `path_allow` capabilities; capability violations MUST return `CapabilityDenied` and MUST NOT silently succeed

## Description

Each resolver entry in `resolvers-registry.toml` declares a `path_allow = [...]` list of
path prefixes the resolver may read via `host::read_file`. Capability enforcement uses the
same deny-by-default mechanism as hook plugins (DI-004). Resolvers are read-only by design:
the host functions `host::write_file`, `host::exec_subprocess`, and `host::emit_event` are
NOT available to resolvers — only `host::read_file` and `host::log` are exposed to the
resolver WASM environment. A resolver attempting to read a path outside its declared
`path_allow` entries receives `CapabilityDenied` (not a panic) from `host::read_file`.
Capability declarations are loaded at registry-load time and are immutable for the lifetime
of the dispatcher session.

## Preconditions

1. The dispatcher has loaded `resolvers-registry.toml` and parsed each resolver entry's
   `path_allow` declarations.
2. The resolver WASM module has been compiled and cached (per BC-4.12.001).
3. A hook dispatch is in progress; the resolver's `resolve()` function is being invoked inside
   a fresh `Store<HostContext>`.
4. The `HostContext` for the resolver's Store has been initialized with capability checks
   derived from this resolver's registry entry (not from the hook's capability declarations).

## Postconditions

1. **Allowed path read:** If a resolver calls `host::read_file(path)` where `path` starts with
   one of the declared `path_allow` prefixes, the host function reads the file and returns its
   contents to the resolver. This is the happy path.
2. **Denied path read (CapabilityDenied):** If a resolver calls `host::read_file(path)` where
   `path` does NOT start with any declared `path_allow` prefix:
   - The host function returns `CapabilityDenied` to the resolver (a non-zero return code in
     the host function protocol, not a WASM trap).
   - The dispatcher emits a `resolver.capability_denied` telemetry event with: resolver name,
     denied path, and the resolved path that was attempted.
   - The resolver WASM process continues running (it receives the error code, not a trap).
   - No file contents are revealed to the resolver.
3. **Unavailable host functions:** If a resolver's WASM module references host functions not
   in the resolver's allowed set (`host::write_file`, `host::exec_subprocess`,
   `host::emit_event`), the `Linker<HostContext>` for resolver execution DOES NOT export
   those symbols. An attempt to link a resolver that calls them fails at module instantiation
   time (linker error), NOT at runtime. This prevents resolvers from accidentally or
   maliciously writing files or executing subprocesses.
4. **`host::log` is always available:** Resolvers may call `host::log_info`, `host::log_warn`,
   and `host::log_error` without capability restriction. Logging is a diagnostic function, not
   a capability-gated operation.
5. **Capability declarations are immutable per session:** The `path_allow` list for a resolver
   is set at registry-load time and does not change during the dispatcher session. A resolver
   CANNOT dynamically expand its own `path_allow` at runtime.
6. **Per-resolver capability isolation:** The capability restrictions for resolver A do NOT
   apply to resolver B and vice versa. Each resolver's `Store` is initialized with that
   resolver's own `path_allow` list. Resolver A cannot read a path allowed by resolver B
   (unless resolver A's own `path_allow` includes it).

## Invariants

1. **Deny-by-default (DI-004):** The default for any resolver without an explicit
   `path_allow` entry is that NO filesystem paths are readable. An empty `path_allow = []`
   means the resolver cannot read any files. This is consistent with the hook capability model
   (DI-004: deny-by-default, capability-gated host functions).
2. **Read-only resolver model:** Resolvers are structurally prevented from writing files or
   executing subprocesses. The host linker configuration for resolver execution MUST NOT
   include `write_file`, `exec_subprocess`, or `emit_event` function bindings. This is an
   architectural invariant, not a per-resolver configuration option.
3. **CapabilityDenied is a return code, not a trap:** A resolver that attempts an unauthorized
   read receives a structured error code from `host::read_file`. It MUST NOT receive a WASM
   trap. The resolver can observe the error and handle it in Rust code. This distinction
   matters for BC-4.12.004: a capability denial is a resolver-handled error, not a crash.
4. **Capability check is path-prefix-based:** `path_allow` entries are matched as path
   prefixes. A `path_allow = [".factory/"]` entry allows reading any path that starts with
   `.factory/` relative to `project_dir`. The check is case-sensitive on case-sensitive
   filesystems.
5. **Registry-load-time validation:** `path_allow` entries are validated for syntactic
   correctness at registry-load time (e.g., entries that are absolute paths outside the
   project root are rejected). Invalid entries cause a `resolver.load_error` at startup.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Resolver attempts to read `/etc/passwd` (outside any `path_allow`) | `CapabilityDenied` returned to resolver. `resolver.capability_denied` event emitted. No file contents returned. Resolver continues running. |
| EC-002 | Resolver attempts to read a file within `path_allow` that does not exist on disk | `host::read_file` returns `FileNotFound` error (distinct from `CapabilityDenied`). The resolver handles the missing file. No capability violation. |
| EC-003 | Resolver `path_allow = []` (empty list) | Resolver cannot read any files. Every `host::read_file` call returns `CapabilityDenied`. This is valid registry configuration (e.g., a resolver that computes output purely from `ResolverInput` without I/O). |
| EC-004 | Resolver references `host::write_file` in its WASM exports | Module instantiation fails (linker error: symbol not available in resolver linker). `resolver.load_error` emitted at startup. |
| EC-005 | Two resolvers; resolver A's `path_allow = [".factory/"]`; resolver B's `path_allow = ["plugins/"]`; resolver A attempts to read `plugins/` | `CapabilityDenied` for resolver A (only `.factory/` is in A's allow list). Resolver B can read `plugins/` normally. |
| EC-006 | `path_allow` entry contains a glob pattern (e.g., `".factory/**"`) | Implementation-defined. MVP: path-prefix matching only (no glob). Glob expansion is a future extension. Registry documentation must specify prefix-only semantics. |

## Canonical Test Vectors

| Resolver `path_allow` | Attempted Read Path | Expected Result |
|----------------------|---------------------|-----------------|
| `[".factory/"]` | `.factory/wave-state.yaml` | ALLOW — contents returned |
| `[".factory/"]` | `/etc/passwd` | DENY — `CapabilityDenied` returned; `resolver.capability_denied` event |
| `[".factory/"]` | `plugins/vsdd-factory/hooks-registry.toml` | DENY — outside allow prefix |
| `[]` (empty) | `.factory/wave-state.yaml` | DENY — empty allow list |
| `[".factory/", "plugins/"]` | `plugins/vsdd-factory/config/artifact-path-registry.yaml` | ALLOW — matches second prefix |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-076 | Resolver-capability confinement — a resolver attempting to read a path outside its `path_allow` entries receives `CapabilityDenied` and does NOT observe file contents | integration test (capability-confinement test: resolver attempts to read `/etc/passwd`; assert CapabilityDenied; assert no file contents in output) |
| (unit-test) | `host::write_file` is absent from resolver linker | Rust unit test (assert linker returns error if resolver references write_file) |
| (unit-test) | `path_allow = []` returns CapabilityDenied for any read | Rust unit test (mock host context with empty allow list) |
| (unit-test) | `path_allow = [".factory/"]` allows reads within prefix | Rust unit test |
| (unit-test) | `resolver.capability_denied` event emitted on denied read | Rust unit test (assert telemetry event) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — this BC governs the capability-enforcement model for resolver plugins, which are WASM plugins authored with the `vsdd-hook-sdk`'s `resolver-authoring` feature. The capability model for resolvers uses the same deny-by-default infrastructure (DI-004) as hook plugins, extended with a read-only restriction specific to resolvers. CAP-009 defines the SDK and sandbox model that both hooks and resolvers use; this BC specifies the resolver-specific restrictions within that sandbox. |
| L2 Domain Invariants | DI-004 (capability deny-by-default — the dispatcher's host function layer enforces capability declarations for all plugin types, including resolvers) |
| Architecture Module | `crates/factory-dispatcher/src/host/mod.rs` (resolver linker configuration — excludes write_file, exec_subprocess, emit_event); `crates/factory-dispatcher/src/host/read_file.rs` (path-prefix capability check); `plugins/vsdd-factory/resolvers-registry.toml` (path_allow declarations per resolver) |
| Stories | S-12.04 (WASM resolver loading + lifecycle — initializes per-resolver HostContext with path_allow), S-12.07 (vsdd-context-resolvers crate — WaveContextResolver path_allow = [".factory/"]) |
| FR | FR-RESOLVER-001 (factory-agnostic runtime context injection; resolvers MUST run with capability-restricted filesystem access) |
| ADR Reference | ADR-018 (WASM-plugin Context Resolvers — resolver capability model; read-only by design; path_allow declarations in resolvers-registry.toml) |

## Related BCs

- BC-1.13.001 — depends on (dispatcher startup contract; capability declarations are read when `resolvers-registry.toml` is loaded)
- BC-4.12.001 — sibling (lifecycle — capability restrictions are per-session, loaded at registry-load time)
- BC-4.12.002 — sibling (ABI — `host::read_file` is the capability-gated host function available to resolvers)
- BC-4.12.004 — composes with (error isolation — a `CapabilityDenied` from `host::read_file` is a resolver-handled error, distinct from a WASM trap)

## Architecture Anchors

- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/host/mod.rs` — resolver linker configuration (excludes write_file, exec_subprocess, emit_event for resolver execution)
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/host/read_file.rs` — path-prefix capability check (deny-by-default, same infrastructure as hooks)
- `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/resolvers-registry.toml` — `path_allow` declarations per resolver (NEW file, authored in S-12.07)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` — resolver capability model decision

## Story Anchor

S-12.04 (WASM resolver loading + lifecycle) and S-12.07 (vsdd-context-resolvers crate + WaveContextResolver) — v1.0-feature-engine-discipline-pass-1 F3-amendment.

## VP Anchors

- VP-076 — Resolver-capability confinement

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-07 | Initial authoring (product-owner; F2-amendment phase of v1.0-feature-engine-discipline-pass-1). Read-only resolver model: write_file/exec_subprocess/emit_event excluded from resolver linker. path_allow-based capability enforcement uses same DI-004 deny-by-default as hooks. |
