//! Resolver-authoring SDK surface.
//!
//! See HOST_ABI.md "Context Injection Contract" section for the platform contract.
//! Anchors: BC-4.12.002 (ABI/payload schema), ADR-018 (design layering).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Resolver ABI version. Distinct from HOST_ABI_VERSION; both currently at 1.
/// (BC-4.12.002 PC4, INV2)
pub const RESOLVER_ABI_VERSION: u32 = 1;

/// Input passed from the dispatcher to a resolver's `resolve()` export.
///
/// Per BC-4.12.002 PC2 + HOST_ABI.md §Context Injection Contract.
/// Field names are EXACT per spec — do not rename or reorder.
/// `agent_type: None` serializes as JSON `null` (not omitted).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolverInput {
    pub event_type: String,
    pub hook_event_name: String,
    pub agent_type: Option<String>,
    pub project_dir: String,
    pub plugin_config: Value,
}

/// Output produced by a resolver's `resolve()` export, returned to the dispatcher.
///
/// Per BC-4.12.002 PC3 + HOST_ABI.md §Context Injection Contract.
/// Field names are EXACT per spec.
/// `value: None` serializes as JSON `"value": null` (explicit null, not omitted).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolverOutput {
    /// The context key under which the value is merged into `plugin_config`.
    pub key: String,
    /// The context payload; `None` means no context for this dispatch (key not written).
    pub value: Option<Value>,
}

/// Error type for resolver operations.
///
/// Reserved for future resolver SDK error handling; currently a placeholder
/// to satisfy story file-list requirements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolverError {
    pub message: String,
}

/// Resolver trait — optional architectural companion to the `Hook` trait.
///
/// Implement this trait and use the `#[resolver]` macro to produce a WASM resolver plugin.
/// Parallel to the existing `Hook` trait for hook plugins.
pub trait Resolver {
    /// Returns the resolver's context key (the `plugin_config` key under which output is written).
    fn context_key(&self) -> &str;

    /// Resolve context for a given dispatch.
    fn resolve(&self, input: ResolverInput) -> ResolverOutput;
}
