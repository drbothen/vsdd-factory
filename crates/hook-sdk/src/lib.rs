//! vsdd-factory hook SDK — pre-implementation stub.
//!
//! Real macros, host-function bindings, and event types arrive in S-1.3.

/// SDK crate version. Distinct from the host plugin ABI version, which
/// the dispatcher exposes once S-1.5 lands.
pub const VERSION: &str = "0.0.1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn version_has_three_dot_separated_components() {
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert_eq!(
            parts.len(),
            3,
            "VERSION must look like X.Y.Z, got {VERSION}"
        );
        for p in parts {
            assert!(
                p.chars().next().is_some_and(|c| c.is_ascii_digit()),
                "VERSION component {p} should start with a digit"
            );
        }
    }
}
