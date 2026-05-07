//! Macros for resolver authoring.
//!
//! Provides the implementation logic for the `#[resolver]` proc-macro (declared in
//! crate root `lib.rs` per Rust's proc-macro crate restriction).
//! Parallel to `#[hook]` for hook plugins. Per BC-4.12.002 PC5 and ADR-018.

use proc_macro::TokenStream;

/// Implementation backing the `#[resolver]` proc-macro attribute (declared in crate root).
///
/// The annotated function MUST be named `resolve_impl` and MUST have the exact
/// signature `fn resolve_impl(input: ResolverInput) -> ResolverOutput`.
///
/// The macro generates a `pub extern "C" fn resolve(input_ptr: i32, input_len: i32) -> i64`
/// WASM export that:
///
/// 1. Reads the input byte slice from WASM linear memory at `input_ptr..input_ptr+input_len`.
/// 2. Deserializes the bytes from JSON into `ResolverInput`.
/// 3. Calls the user's `resolve_impl` function.
/// 4. Serializes the `ResolverOutput` to JSON bytes.
/// 5. Writes the output bytes to a WASM memory allocation.
/// 6. Returns the packed ptr+len pair: `((ptr as i64) << 32) | (len as i64)`.
///
/// Applying this macro to a function with the wrong name or wrong signature emits a
/// compile-time error. (BC-4.12.002 PC5)
pub fn resolver_impl(_args: TokenStream, _input: TokenStream) -> TokenStream {
    todo!("S-12.05 Step 4 implementer — emit resolver plugin entrypoint per BC-4.12.002 PC5; validate fn name == resolve_impl, param type == ResolverInput, return type == ResolverOutput; generate pub extern \"C\" fn resolve(input_ptr: i32, input_len: i32) -> i64 with packed i64 return encoding ((ptr as i64) << 32) | (len as i64)")
}
