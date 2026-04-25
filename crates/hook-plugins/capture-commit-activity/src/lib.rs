//! capture-commit-activity hook — pre-implementation stub.
//!
//! Real `on_hook` export with bindings to the SDK arrives in S-3.1.
//! Until then the crate exists to anchor the workspace member layout
//! and the wasm32-wasip1 build path.

#[unsafe(no_mangle)]
pub extern "C" fn on_hook() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_hook_returns_zero_in_stub() {
        assert_eq!(on_hook(), 0);
    }
}
