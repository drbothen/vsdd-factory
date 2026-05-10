# hook-sdk Changelog

## 0.3.0 — 2026-05-10 (resolver-authoring feature)

### Added (gated behind `resolver-authoring` feature)
- `ResolverInput` and `ResolverOutput` types per BC-4.12.002
- `Resolver` trait per BC-4.12.001
- `RESOLVER_ABI_VERSION` constant (currently `1`)
- `#[resolver]` proc-macro re-export from `hook-sdk-macros`

### Changed
- `vsdd-hook-sdk-macros` dependency bumped to `0.2.0` (adds `#[resolver]` macro)

### Versioning Independence (BC-4.12.002 INV2)
`RESOLVER_ABI_VERSION` is independently versioned from `HOST_ABI_VERSION`.
Bumps to one do not require bumps to the other. The two constants are
declared in separate source files (`resolver.rs` and `lib.rs` respectively)
to make this independence structurally enforced and machine-verifiable.

`ResolverError` is a host-side type (factory-dispatcher); it is NOT exported
from hook-sdk. Hook plugin authors do not need to handle it directly.

## 0.2.0 — earlier release

(See git log for prior versions.)
