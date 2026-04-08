<!-- Implementation rules for SOUL.md principles. SOUL owns "why", this file owns "how". -->

# Rust Coding Rules

Project-specific Rust conventions for Corverax.

## Safety

- Every application crate: `#![forbid(unsafe_code)]`
- No `unwrap()` in production code — use `?` or `expect()` with actionable message
- No blocking the async runtime — use `spawn_blocking` for CPU-intensive work, `tokio::time::sleep` not `std::thread::sleep`

## Type Design

- **Newtypes for IDs** — prevents mixing ID types
- **Validated constructors at trust boundaries:** `new()` validates (API input, deserialization); `new_unchecked()` for tests and trusted internal sources
- **`#[non_exhaustive]` on enums that will grow** — forces callers to handle future variants
- **UUID v7 for time-ordered IDs** — `Uuid::now_v7()` for time-sortable ordering
- **Private fields with getters** on security-critical types

## Error Handling

- Use `thiserror` for error enums — structured, semantic variants (not string bags)
- Define `pub type Result<T> = std::result::Result<T, CrateError>` per crate
- `Display` impl is for internal logging only — sanitize before sending to clients

## Module Structure

```
corverax-{crate}/
  src/
    lib.rs          # Public API re-exports only
    error.rs        # Crate-specific error types
    config.rs       # Configuration types
    {domain}/       # Feature-specific modules
```

## Dependencies

- Workspace-level dependency declarations in root `Cargo.toml`
- Edition 2024, MSRV 1.85+
- Use `cargo clippy -- -D warnings` — warnings are errors

## Testing

- Unit: `#[cfg(test)] mod tests {}` in same file
- Integration: `tests/` directory, named by feature
- Property: `tests/property_*.rs` with `proptest`
- Snapshot: `tests/snapshot_*.rs` with `insta`
- Test names as documentation: `workflow_rejects_invalid_state()`, not `test_1()`
- Test boundaries: empty, too-long, whitespace, case, invalid formats

## Architecture

- Dependency graph is strictly acyclic
- No circular dependencies between crates
- `lib.rs` is a pure re-export barrel — implementation in domain modules
