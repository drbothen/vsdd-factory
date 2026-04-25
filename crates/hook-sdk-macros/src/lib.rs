//! Procedural macros for `vsdd-hook-sdk`.
//!
//! Re-exported by the parent crate as `vsdd_hook_sdk::hook`. End users do
//! not depend on this crate directly.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ReturnType, Type, parse_macro_input};

/// Mark a function as a vsdd-factory hook entry point.
///
/// The annotated function must have the signature
/// `fn(payload: HookPayload) -> HookResult`. The macro generates a
/// `fn main()` wrapper that:
///
/// 1. Reads the hook payload from stdin (UTF-8 JSON).
/// 2. Catches panics from the user function and converts them into a
///    `HookResult::Error` so the wasm instance terminates cleanly.
/// 3. Serializes the result to stdout (UTF-8 JSON).
/// 4. Exits with `0` (Continue), `2` (Block), or `1` (Error).
///
/// Plugins are built as WASI commands (`cargo build --target
/// wasm32-wasip1`); the auto-generated `_start` is what the dispatcher
/// invokes on each plugin call.
///
/// # Example
///
/// ```ignore
/// use vsdd_hook_sdk::{hook, HookPayload, HookResult};
///
/// #[hook]
/// pub fn on_hook(payload: HookPayload) -> HookResult {
///     vsdd_hook_sdk::host::log_info(&format!("event: {}", payload.event_name));
///     HookResult::Continue
/// }
/// ```
#[proc_macro_attribute]
pub fn hook(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let user_fn = parse_macro_input!(item as ItemFn);

    if let Err(err) = validate_signature(&user_fn) {
        return err.to_compile_error().into();
    }

    let user_ident = &user_fn.sig.ident;

    let expanded = quote! {
        #user_fn

        fn main() {
            ::vsdd_hook_sdk::__internal::run(#user_ident);
        }
    };

    expanded.into()
}

fn validate_signature(f: &ItemFn) -> Result<(), syn::Error> {
    if f.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &f.sig,
            "#[hook] does not support async fns; the WASI preview-1 entry \
             point is synchronous. Use synchronous host calls and run \
             async work inside a synchronous wrapper.",
        ));
    }
    if f.sig.unsafety.is_some() {
        return Err(syn::Error::new_spanned(
            &f.sig,
            "#[hook] cannot annotate `unsafe fn`; the entry point must be safe.",
        ));
    }
    if f.sig.inputs.len() != 1 {
        return Err(syn::Error::new_spanned(
            &f.sig,
            "#[hook] expects exactly one argument: the HookPayload.",
        ));
    }
    match &f.sig.output {
        ReturnType::Default => Err(syn::Error::new_spanned(
            &f.sig,
            "#[hook] expects a return type of `HookResult`.",
        )),
        ReturnType::Type(_, ty) => {
            if !is_hook_result_type(ty) {
                return Err(syn::Error::new_spanned(
                    ty,
                    "#[hook] return type must be `HookResult` (or \
                     `vsdd_hook_sdk::HookResult`).",
                ));
            }
            Ok(())
        }
    }
}

fn is_hook_result_type(ty: &Type) -> bool {
    if let Type::Path(p) = ty
        && let Some(last) = p.path.segments.last()
    {
        return last.ident == "HookResult";
    }
    false
}
