//! `ContextResolver` trait, `ResolverRegistry`, `ResolverInput`, `ResolverOutput`,
//! `ResolverError`, and `merge_resolver_outputs()` — the in-memory layer for
//! context injection per BC-1.13.001 + BC-4.12.005.
//!
//! No WASM loading in this module (S-12.04 adds that). All resolver dispatching
//! in this story is in-memory (trait objects); the WASM-backed implementation
//! extends `ResolverRegistry` in S-12.04 without changing the trait.
//!
//! Architecture anchors:
//! - BC-1.13.001 — dispatcher pre-dispatch injection contract
//! - BC-4.12.005 — additive overlay merge semantics
//! - ADR-018 — WASM-plugin Context Resolvers design
//! - VP-075 — context-injection determinism (proptest harness in S-12.03 tests)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

// ---------------------------------------------------------------------------
// ABI types — shapes defined by HOST_ABI.md §Context Injection Contract
// and BC-4.12.002. These are the wire types between dispatcher and resolver.
// ---------------------------------------------------------------------------

/// Input handed to a resolver on each dispatch invocation.
///
/// Per HOST_ABI.md §Resolver ABI Types and BC-4.12.002.
/// `agent_type` is `Option<String>` because the field may be absent when the
/// Claude Code runtime does not provide it in the envelope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResolverInput {
    /// The host platform's event-type string (e.g. `"PreToolUse"`, `"PostToolUse"`).
    pub event_type: String,

    /// Name of the hook being dispatched (hooks-registry entry `name`).
    pub hook_event_name: String,

    /// Agent type from the dispatch context; `None` when absent.
    pub agent_type: Option<String>,

    /// Absolute path to the factory project root.
    pub project_dir: String,

    /// The hook's static `plugin_config` (read-only; resolver outputs not yet merged).
    /// Per HOST_ABI.md: resolver receives pre-merge static config only.
    pub plugin_config: Value,
}

/// Output returned by a resolver after a successful invocation.
///
/// Per HOST_ABI.md §Resolver ABI Types and BC-4.12.002.
/// A `value` of `None` means the key is NOT written to `plugin_config` (BC-4.12.005 PC2).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResolverOutput {
    /// The context key under which the value is merged into `plugin_config`.
    pub key: String,

    /// The context payload. `None` → key absent from merged `plugin_config`.
    pub value: Option<Value>,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors produced during resolver invocation or registry operations.
///
/// Covers the error categories documented in the story's Dev Notes and
/// BC-4.12.004 (crash isolation). S-12.04 adds WASM-specific variants.
///
/// `#[non_exhaustive]` allows adding fields to existing variants and new
/// variants in S-12.04+ without a breaking ABI change (F-P2-006).
///
/// `#[serde(tag = "kind", rename_all = "snake_case")]` emits
/// `{"kind": "not_found", "name": "..."}` — matching the HOST_ABI.md wire
/// format (line 1095). Forward-compatible with the WASM boundary S-12.04
/// introduces (F-P2-003). `Io` variant dropped: resolver.rs is in-memory only
/// per Forbidden Dependencies; I/O errors land in `ResolverLoadError` (S-12.04).
#[non_exhaustive]
#[derive(Debug, Error, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ResolverError {
    /// Resolver named `name` is not registered in the registry.
    /// Emits `resolver.not_found` event (BC-1.13.001 PC6).
    /// Wire format: `"kind": "not_found"`.
    #[error("resolver not found: {name}")]
    NotFound { name: String },

    /// Resolver panicked or trapped during WASM execution (HOST_ABI `"trap"`).
    /// S-12.04 populates this for WASM-backed resolvers.
    /// Wire format: `"kind": "trap"`.
    #[error("resolver '{name}' trapped: {detail}")]
    Trap { name: String, detail: String },

    /// Resolver returned an ABI-violating response (type mismatch,
    /// missing required field, undeserializable output).
    /// Wire format: `"kind": "abi_violation"`.
    #[error("resolver '{name}' ABI violation: {detail}")]
    AbiViolation { name: String, detail: String },

    /// Resolver invocation exceeded the configured wall-clock budget.
    /// Wire format: `"kind": "timeout"`.
    #[error("resolver '{name}' timed out")]
    Timeout { name: String },

    /// Resolver attempted to access a path outside its `path_allow` list.
    /// Enforced by the host linker; S-12.04 wires this.
    /// Wire format: `"kind": "capability_denied"`.
    #[error("resolver '{name}' capability denied: {path}")]
    CapabilityDenied { name: String, path: String },

    /// Malformed source data discovered during resolver invocation.
    /// Wire format: `"kind": "malformed"`.
    #[error("malformed source data for resolver '{name}': {detail}")]
    Malformed { name: String, detail: String },

    /// A resolver with the same `name()` has already been registered.
    /// Emits `resolver.load_error` event (BC-4.12.005 PC6 / EC-005 —
    /// fail-loud at registry-load time). The first registration is preserved.
    /// Wire format: `"kind": "duplicate_name"`.
    #[error(
        "duplicate resolver name '{name}' — each resolver name must be unique \
             (BC-4.12.005 PC6 / EC-005); first registration preserved"
    )]
    DuplicateName { name: String },
}

// ---------------------------------------------------------------------------
// ContextResolver trait
// ---------------------------------------------------------------------------

/// Trait for in-memory resolver implementations (and the WASM-backed wrapper
/// that S-12.04 adds). Object-safe: uses `&self` receiver to allow
/// `Box<dyn ContextResolver>` in `ResolverRegistry`.
///
/// Per BC-1.13.001 architecture anchor + ADR-018 OD-1 (factory-agnostic
/// dispatcher — no per-factory compile-time dependencies).
pub trait ContextResolver: Send + Sync {
    /// Unique name for this resolver. Must match the `name` field in
    /// `resolvers-registry.toml` and the `needs_context` declaration in
    /// `hooks-registry.toml`. The key under which output lands in `plugin_config`.
    fn name(&self) -> &str;

    /// Invoke the resolver for one dispatch.
    ///
    /// Returns `Ok(Some(output))` when context is available.
    /// Returns `Ok(None)` when the resolver has no data for this event —
    /// the key will be absent from `plugin_config` (BC-4.12.005 PC2).
    /// Returns `Err(ResolverError)` on hard failure; the key is absent.
    fn resolve(&self, input: &ResolverInput) -> Result<Option<ResolverOutput>, ResolverError>;
}

// ---------------------------------------------------------------------------
// ResolverRegistry
// ---------------------------------------------------------------------------

/// Registry of in-process `ContextResolver` trait objects.
///
/// Holds one `Box<dyn ContextResolver>` per registered resolver. Duplicate
/// `name` values are rejected at registration time (BC-4.12.005 PC6, EC-005).
///
/// S-12.04 extends this by adding a WASM-backed implementation of
/// `ContextResolver` that wraps a compiled `wasmtime::Module`; the registry
/// itself stays as-is.
pub struct ResolverRegistry {
    resolvers: Vec<Box<dyn ContextResolver>>,
}

impl ResolverRegistry {
    /// Construct an empty registry (BC-1.13.001 INV2: absent
    /// `resolvers-registry.toml` = zero resolvers, not an error).
    pub fn new() -> Self {
        Self {
            resolvers: Vec::new(),
        }
    }

    /// Register a resolver. Returns `Err(ResolverError::DuplicateName { name })`
    /// if a resolver with the same `name()` has already been registered
    /// (BC-4.12.005 PC6 / EC-005 — fail-loud at registry-load time).
    ///
    /// Does NOT panic. The registry state is unchanged after a failed
    /// registration (first registration preserved — EC-005 expected behavior).
    pub fn register(&mut self, resolver: Box<dyn ContextResolver>) -> Result<(), ResolverError> {
        let name = resolver.name().to_string();
        if self.resolvers.iter().any(|r| r.name() == name) {
            return Err(ResolverError::DuplicateName { name });
        }
        self.resolvers.push(resolver);
        Ok(())
    }

    /// Resolve context for a single named resolver and return its output.
    ///
    /// Returns `None` (and emits `resolver.not_found`) if no resolver with
    /// `name` is registered (BC-1.13.001 PC6). The hook dispatch continues
    /// without context injection for that key.
    ///
    /// The `emit_not_found` callback receives the missing resolver name so
    /// the caller can emit the telemetry event using the existing sink pattern
    /// (keeping telemetry non-blocking — BC-1.13.001 architecture rule 5).
    pub fn invoke_resolver(
        &self,
        name: &str,
        input: &ResolverInput,
        emit_not_found: impl FnOnce(&str),
    ) -> Option<Result<ResolverOutput, ResolverError>> {
        match self.resolvers.iter().find(|r| r.name() == name) {
            None => {
                emit_not_found(name);
                None
            }
            Some(resolver) => match resolver.resolve(input) {
                // Resolver returned data — wrap in Some(Ok(...)).
                Ok(Some(output)) => Some(Ok(output)),
                // Resolver has no data for this event — treat as not-produced.
                Ok(None) => None,
                // Hard failure — propagate as Some(Err(...)).
                Err(e) => Some(Err(e)),
            },
        }
    }

    /// Resolve all context declared in `requested_names` for one dispatch.
    ///
    /// For each name in `requested_names` (in order — BC-1.13.001 PC7):
    /// - If registered: invokes resolver and returns its output.
    /// - If not registered: calls `emit_not_found` for telemetry; skips.
    /// - If resolver returns `Err(...)`: calls `emit_resolver_error` so the
    ///   caller can emit the `resolver.error` telemetry event non-blockingly
    ///   (BC-1.13.001 PC2 / BC-4.12.005 INV3 — failed resolver is observable).
    ///
    /// # Return type (F-P5-003 — Option A)
    ///
    /// Returns `Vec<(String, ResolverOutput)>` where the first `String` is the
    /// **registry name** of the resolver (`ContextResolver::name()`).  This
    /// threads resolver identity through to `merge_resolver_outputs` so
    /// `CollisionInfo.resolver_name` is populated from the actual registry name,
    /// not derived from `output.key`.
    ///
    /// Entries are in **declaration order** (BC-1.13.001 PC7).
    /// Resolvers returning `Ok(None)` contribute no entry (BC-4.12.005 PC2).
    /// Failed resolvers contribute no entry.
    ///
    /// Per BC-4.12.002 INV4: each resolver receives only the static
    /// `plugin_config`; resolver outputs are merged after all invocations.
    pub fn resolve_context_for_entry(
        &self,
        requested_names: &[String],
        input: &ResolverInput,
        emit_not_found: impl Fn(&str),
        emit_resolver_error: impl Fn(&str, &ResolverError),
    ) -> Vec<(String, ResolverOutput)> {
        let mut outputs = Vec::new();
        for name in requested_names {
            match self.invoke_resolver(name, input, &emit_not_found) {
                None => {
                    // Not found — emit_not_found already called; skip this key.
                }
                Some(Err(err)) => {
                    // Resolver errored — call the error callback so the caller
                    // can emit telemetry (SOUL #4: no silent failures).
                    // Dispatch continues; this key contributes nothing.
                    emit_resolver_error(name, &err);
                }
                Some(Ok(output)) if output.value.is_some() => {
                    // F-P5-003: thread resolver identity (name) with the output.
                    // Declaration order preserved (BC-1.13.001 PC7).
                    // value: None branch handled by non-matching arm → key absent (BC-4.12.005 PC2).
                    outputs.push((name.clone(), output));
                }
                Some(Ok(_)) => {
                    // value: None → key absent (BC-4.12.005 PC2); do nothing.
                }
            }
        }
        outputs
    }

    /// Number of registered resolvers (for startup log: "Loaded N context resolvers").
    pub fn len(&self) -> usize {
        self.resolvers.len()
    }

    /// True when no resolvers are registered.
    pub fn is_empty(&self) -> bool {
        self.resolvers.is_empty()
    }
}

impl Default for ResolverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Pure merge function (BC-4.12.005 INV1, VP-075)
// ---------------------------------------------------------------------------

/// Records one key collision detected during `merge_resolver_outputs`.
///
/// The caller (executor.rs `build_plugin_config`) iterates the returned
/// `Vec<CollisionInfo>` and emits `resolver.merge_collision` telemetry events
/// for each — keeping the pure merge function free of I/O side effects
/// (BC-4.12.005 INV1; architect Path B decision, ADR pass-2).
///
/// F-P4-001B / F-P5-003: `resolver_name` is the registry name of the resolver
/// that produced the colliding output, threaded explicitly through the
/// `(resolver_name, ResolverOutput)` pair in `merge_resolver_outputs`.
#[derive(Debug, Clone, PartialEq)]
pub struct CollisionInfo {
    /// The config key that collided.
    pub key: String,
    /// The registry name of the resolver whose output caused the collision.
    /// F-P5-003: threaded from the `(resolver_name, output)` pair — NOT
    /// derived from `output.key` so distinct resolver names and output keys
    /// are handled correctly.
    pub resolver_name: String,
    /// The value that was in `static_config` before the merge.
    pub old_value: Value,
    /// The resolver value that replaced it.
    pub new_value: Value,
}

/// Merge resolver outputs additively onto `static_config`.
///
/// This is a **pure function**: given identical inputs it produces identical
/// output. No I/O, no side effects, no global state, no callbacks. VP-075
/// proptest target.
///
/// The `static_config` parameter is typed as `serde_json::Map<String, Value>`
/// (not the broader `Value` enum) so that non-object inputs are unrepresentable
/// at the type level — types are cheaper than runtime discipline (F-006).
/// The production invariant that `plugin_config` is always a JSON Object is
/// enforced at the call-site coercion step (see `executor.rs`).
///
/// # Signature (F-P5-003 — Option A)
///
/// `resolver_outputs` is a slice of `(resolver_name, ResolverOutput)` pairs.
/// The `resolver_name` is the `ContextResolver::name()` of the resolver that
/// produced the output — threaded through from `resolve_context_for_entry` so
/// `CollisionInfo.resolver_name` carries the actual registry identity, not a
/// derived copy of the output key.
///
/// Merge semantics (BC-4.12.005):
/// - `static_config` fields are preserved.
/// - Each `ResolverOutput` with `value: Some(v)` sets `plugin_config[key] = v`
///   (whole-value replacement — no deep merge).
/// - Each `ResolverOutput` with `value: None` writes nothing (key absent).
/// - Resolver output wins on collision with static config (PC5).
/// - Outputs are applied in the order they appear in `resolver_outputs` (PC4).
///
/// Returns `(merged_map, collisions)`. The caller emits `resolver.merge_collision`
/// telemetry for each `CollisionInfo` entry — preserving purity here while keeping
/// the collision observable (BC-4.12.005 INV1; architect Path B).
pub fn merge_resolver_outputs(
    static_config: serde_json::Map<String, Value>,
    resolver_outputs: &[(String, ResolverOutput)],
) -> (serde_json::Map<String, Value>, Vec<CollisionInfo>) {
    let mut map = static_config;
    let mut collisions = Vec::new();

    // Apply resolver outputs in order (BC-4.12.005 PC4).
    for (resolver_name, output) in resolver_outputs {
        // value: None → do not write the key (BC-4.12.005 PC2).
        if let Some(new_val) = &output.value {
            if let Some(old_val) = map.get(&output.key) {
                // Key collision: record for caller to emit telemetry.
                // Resolver wins (BC-4.12.005 PC5 — whole-value replacement).
                // F-P5-003: resolver_name threaded from the (name, output) pair.
                collisions.push(CollisionInfo {
                    key: output.key.clone(),
                    resolver_name: resolver_name.clone(),
                    old_value: old_val.clone(),
                    new_value: new_val.clone(),
                });
            }
            map.insert(output.key.clone(), new_val.clone());
        }
    }

    (map, collisions)
}
