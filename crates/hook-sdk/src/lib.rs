//! # vsdd-hook-sdk
//!
//! SDK for authoring vsdd-factory v1.0 hook plugins. Plugins are
//! WebAssembly modules compiled to `wasm32-wasip1`; the dispatcher
//! invokes each plugin's WASI command entry point per Claude Code hook
//! event.
//!
//! ## Authoring a hook
//!
//! ```ignore
//! use vsdd_hook_sdk::{hook, HookPayload, HookResult};
//!
//! #[hook]
//! pub fn on_hook(payload: HookPayload) -> HookResult {
//!     vsdd_hook_sdk::host::log_info(&format!(
//!         "event={} tool={}",
//!         payload.event_name, payload.tool_name,
//!     ));
//!     HookResult::Continue
//! }
//! ```
//!
//! Build with `cargo build --target wasm32-wasip1 --release`.
//!
//! ## ABI
//!
//! Cross-language ABI documented in
//! [HOST_ABI.md](https://github.com/drbothen/vsdd-factory/blob/main/crates/hook-sdk/HOST_ABI.md).
//! The constant [`HOST_ABI_VERSION`] tracks compatibility — plugins
//! linking against a given SDK major version run on dispatchers that
//! advertise the same `HOST_ABI_VERSION` (or compatibly back-versioned).

// vsdd-hook-sdk targets `wasm32-wasip1`, which provides full std. The
// `#[hook]` runtime needs stdin/stdout/process::exit, so we stay on std
// rather than juggling alloc-only types.

mod ffi;
pub mod host;
mod payload;
mod result;

#[doc(hidden)]
pub mod __internal;

pub use payload::HookPayload;
pub use result::HookResult;
pub use vsdd_hook_sdk_macros::hook;

/// SDK crate version. Distinct from [`HOST_ABI_VERSION`].
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin ABI version that this SDK speaks.
///
/// The dispatcher exposes the same constant. A dispatcher of
/// `HOST_ABI_VERSION = N` will only invoke plugins whose SDK
/// `HOST_ABI_VERSION` is `N` (or a documented back-compat range). Bumping
/// this is a major version event for both crates.
pub const HOST_ABI_VERSION: u32 = 1;
