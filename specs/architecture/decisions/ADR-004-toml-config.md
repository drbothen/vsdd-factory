---
document_type: adr
adr_id: ADR-004
status: accepted
date: 2026-04-26
subsystems_affected: [SS-01, SS-09]
supersedes: null
superseded_by: null
---

# ADR-004: TOML for All Configuration Files

## Context

vsdd-factory v1.0 introduced two new configuration surfaces: `hooks-registry.toml`,
which declares which WASM plugins fire on which events, and `observability-config.toml`,
which declares the sink pipeline topology. Both files must be human-authorable by
operators — they are not generated artifacts — and both are parsed directly by the
compiled Rust dispatcher binary at runtime.

Prior to v1.0, hook routing was expressed in Claude Code's native `hooks.json` format.
That file served a dual role as both the Claude Code harness wiring document and the
dispatcher routing table. As the plugin system matured (ADR-011), these two concerns
separated: `hooks.json` remains the harness wiring file while `hooks-registry.toml`
became the dispatcher's authoritative routing table. A consistent serialization format
across all new configuration artifacts was required to reduce cognitive overhead for
operators editing them and for contributors writing the parser logic.

The Rust workspace was already organized around Cargo, which uses TOML natively. The
`toml` crate is idiomatic in the Rust ecosystem and provides strongly-typed
deserialization via `serde`. The alternative formats — YAML and JSON — both had
significant disadvantages in this context.

## Decision

All new configuration files introduced in v1.0 (`hooks-registry.toml`,
`observability-config.toml`) use TOML. The `schema_version = 1` field is mandatory
in both files; the loader rejects any file whose `schema_version` does not equal the
expected constant (`REGISTRY_SCHEMA_VERSION: u32 = 1` in `registry.rs`). New
configuration surfaces added after v1.0 follow the same convention.

## Rationale

TOML was chosen over YAML and JSON for three reasons:

First, TOML's type system is explicit and unambiguous. YAML's indentation-sensitivity
and implicit type coercion (e.g., `true`/`yes`/`on` all meaning boolean true) create
human-editing traps. TOML has none of these; values are typed at parse time and
mismatches produce clear errors.

Second, the array-of-tables syntax (`[[hooks]]`, `[[sinks]]`) is exactly the shape
both configuration files need. Multi-instance declarations read naturally in TOML,
whereas YAML requires careful indentation and JSON requires verbose bracket nesting.

Third, the Rust ecosystem is at home with TOML. The `toml` crate (`toml::de::Error`)
provides structured deserialization errors that the dispatcher surfaces verbatim — see
`RegistryError::Toml` in `crates/factory-dispatcher/src/registry.rs:26`. This means
parse failures produce actionable messages rather than opaque offsets.

The `schema_version` field was added to both configuration schemas as a future-proofing
measure. If the schema changes incompatibly, the loader can detect the mismatch
immediately rather than mis-parsing silently. The constant `REGISTRY_SCHEMA_VERSION`
is checked at load time; a mismatch produces `RegistryError::SchemaVersion` with both
the expected and received values, guiding the operator to either regenerate the file
or upgrade the dispatcher.

## Consequences

### Positive

- Operators authoring `hooks-registry.toml` get comment support (unavailable in JSON),
  consistent with how Cargo.toml is edited.
- The `toml` crate's `serde` integration means the deserializer and the type definitions
  stay in sync automatically — no separate schema validation step.
- `schema_version` enforcement prevents silent mis-parsing when the dispatcher is
  upgraded ahead of a stale config file.
- Both configuration files share a consistent top-level structure (comment header,
  `schema_version`, `[defaults]`, then `[[section]]` arrays).

### Negative / Trade-offs

- TOML is less universally known than JSON. Operators unfamiliar with Rust tooling
  may need to learn the `[[array-of-tables]]` syntax.
- JSON configuration would be directly compatible with the hook payload format (also
  JSON), but the operator-authoring benefits of TOML outweigh this consistency.

### Status as of v1.0.0-beta.5

IN-EFFECT. `hooks-registry.toml` and `observability-config.toml` both ship as TOML
with `schema_version = 1`. The `REGISTRY_SCHEMA_VERSION: u32 = 1` constant is enforced
in `crates/factory-dispatcher/src/registry.rs:16`.

## Alternatives Considered

- **YAML:** Comment support and human-readable, but whitespace-sensitive and type-coercion
  traps make it error-prone. Rejected: weaker type safety than TOML.
- **JSON:** Already used for hook payload format (stdin/stdout). No comment support;
  verbose for multi-instance array declarations. Rejected: poor operator authoring
  experience for configuration files.
- **HCL (HashiCorp Configuration Language):** Expressive and familiar to Terraform users.
  Rejected: no idiomatic Rust parser, adds a non-standard dependency.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 448–456 (ADR-004: TOML for configuration) and lines 226–233
  (hooks-registry.toml schema_version declaration).
- **Code as-built:** `crates/factory-dispatcher/src/registry.rs:16` (`REGISTRY_SCHEMA_VERSION: u32 = 1`),
  `crates/factory-dispatcher/src/registry.rs:26` (`RegistryError::Toml`).
- **Config file examples:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 225–284 (`hooks-registry.toml` example) and lines 288–359 (`observability-config.toml` example).
