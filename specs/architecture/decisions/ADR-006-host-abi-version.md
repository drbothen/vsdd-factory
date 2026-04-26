---
document_type: adr
adr_id: ADR-006
status: accepted
date: 2026-04-26
subsystems_affected: [SS-01, SS-02]
supersedes: null
superseded_by: null
---

# ADR-006: HOST_ABI_VERSION as Separate Semver Constant

## Context

vsdd-factory v1.0 introduced a stable WASM plugin ABI: every plugin compiled
against `vsdd-hook-sdk` imports host functions declared in the `vsdd` WASM import
namespace. The dispatcher exports those host functions. Any change to the host
function signatures, calling conventions, or semantics that breaks existing plugins
is a breaking ABI change.

The `vsdd-hook-sdk` crate has its own crates.io version (`0.1.0` during beta,
promoted to `1.0.0` at stable). The dispatcher binary has its own semver (aligned
with the plugin release, e.g., `1.0.0-beta.5`). These two version numbers track
different things: the SDK version tracks the SDK's public API surface (types,
macros, helper functions), while the ABI version tracks the binary interface
contract between dispatcher and compiled plugin WASM modules.

The two could diverge legitimately: the SDK could add new helper functions
(semver-minor SDK bump) without changing the host import ABI at all. Or the SDK
could release a patch fix while the host ABI changes required by a new host function
necessitate a major ABI bump. Conflating ABI version with SDK version would either
over-restrict SDK releases (every ABI change forces a SDK major bump) or
under-specify the ABI contract (SDK minor bumps could silently break loaded plugins).

## Decision

A separate constant `HOST_ABI_VERSION: u32 = 1` is declared in both the
`vsdd-hook-sdk` crate (`crates/hook-sdk/src/lib.rs:58`) and must be matched by the
dispatcher at plugin load time. The SDK exports this constant from its WASM module;
the dispatcher reads it before invoking any plugin function. A mismatch causes the
plugin to be refused with a loud error event rather than silently executing against
an incompatible ABI. Breaking host ABI changes require a major bump to this constant
and corresponding major bumps on both the SDK crate and the dispatcher.

## Rationale

Separating the ABI version from the SDK crate version follows the same reasoning
as the Linux kernel's `LINUX_VERSION_CODE` / `MODULE_SUPPORTED_DEVICE_TABLE` pattern:
the kernel has its own release version, but module compatibility is tracked by a
separate ABI constant that changes only when the module interface changes.

Using a `u32` rather than a semver string for `HOST_ABI_VERSION` is intentional.
ABI compatibility is a binary predicate at load time: either the plugin's declared
ABI version is compatible with the dispatcher's expected version, or it is not.
A semver string would tempt readers to infer range semantics that the loader does
not implement. The simple `u32` makes the comparison visible and the policy
self-documenting.

The design document explicitly notes (line 217–221) that the dispatcher `MAY support
multiple ABI versions concurrently if we commit to back-compat`. Keeping the constant
as a `u32` rather than an embedded semver leaves this door open: the dispatcher can
test `loaded_abi >= MIN_SUPPORTED_ABI && loaded_abi <= HOST_ABI_VERSION` without
parsing version strings.

The distinct constant also ensures the semver stability promise (which engages at
`1.0.0` for the overall product, not prerelease) is honored correctly for the ABI
specifically. The design doc (line 697–698) states "Semver commitment doc published;
host ABI version frozen at 1" as a `1.0.0` gate criterion — having a separate
constant makes that freeze explicit and testable.

## Consequences

### Positive

- SDK crate version and host ABI version can evolve independently. SDK patch and
  minor releases do not require ABI version bumps.
- Plugin load rejection is loud and actionable: operators see the plugin name,
  the loaded ABI version, and the expected version, enabling targeted diagnosis.
- The `u32` constant is trivially exported from WASM modules and read by the
  dispatcher without a semver parser dependency in either direction.

### Negative / Trade-offs

- Plugin authors must track two version numbers: the SDK crate version (for
  `Cargo.toml` dependency) and the `HOST_ABI_VERSION` (for compatibility assertions).
  The SDK documentation must keep these in sync clearly.
- There is currently no mechanism for the dispatcher to support a range of ABI
  versions. If a downstream plugin freezes at ABI v1 and the dispatcher advances
  to ABI v2 with back-compat support, the range logic must be added manually.

### Status as of v1.0.0-beta.5

IN-EFFECT. `HOST_ABI_VERSION: u32 = 1` is declared in `crates/hook-sdk/src/lib.rs:58`.
The dispatcher plugin loader (`crates/factory-dispatcher/src/plugin_loader.rs`) reads
the compiled module's exported `HOST_ABI_VERSION` symbol before invoking any plugin
entry point. ABI version mismatch surfaces as a `plugin.load_failed` internal event.

## Alternatives Considered

- **Use SDK crate semver as ABI version:** Rejected because SDK minor bumps (new
  helper functions, new macro options) would falsely appear as ABI breaks; SDK patch
  fixes for non-ABI bugs would still require ABI analysis to determine whether a bump
  is needed. The coupling creates unnecessary release ceremony.
- **No ABI version; fail at wasmtime import resolution:** wasmtime's linker would
  produce an `Instantiate` error when a plugin imports a host function that doesn't
  exist. Rejected: error messages are opaque (wasmtime linker errors reference
  wasm import names, not human-readable context); the failure mode for a missing
  new host function and a version mismatch are indistinguishable.
- **Semver string constant:** Rejected: introduces semver-range semantics the loader
  does not implement; `u32` is simpler, faster to compare, and unambiguous.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 217–221 (HOST_ABI_VERSION declaration and compatibility policy), lines 733–742
  (Open Question Q2 resolution — hook-sdk versioning and ABI version tracking),
  line 698 (semver commitment doc + host ABI version freeze as 1.0.0 gate criterion).
- **Code as-built:** `crates/hook-sdk/src/lib.rs:49–58` (`HOST_ABI_VERSION: u32 = 1`
  with module-level doc comment distinguishing it from `VERSION`).
- **Code as-built:** `crates/factory-dispatcher/src/plugin_loader.rs` (ABI version
  check at module compile/cache time).
