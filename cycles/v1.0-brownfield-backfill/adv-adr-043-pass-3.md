# ADR-043 Adversarial Review Pass 3 — v1.2→v1.5 — DO-NOT-RATIFY → CONVERGING

**Reviewed artifact:** ADR-043 v1.2 (initial); v1.3/v1.4/v1.5 (amendments during review session)
**Review date:** 2026-08-11
**Verdict:** DO-NOT-RATIFY at v1.2; v1.5 conditionally accepted pending human ratification
**Pass:** 3 of 3 (ADR-scoped; NOT cycle-level pass-11; streak UNCHANGED)
**D-chain:** D-972

## Part A — Finding Set (v1.2)

**BLOCKER (1):**

- **B-1**: `cfg(unix)` scoping for trusted-prefix list provides load-time path resolution on unix only; Windows builds have no `cfg(windows)` counterpart. First Windows implementation attempt will produce `unresolved symbol` linker errors for the trusted-prefix lookup functions. This is a platform-portability regression baked into the specification.

**HIGH (3):**

- **H-1**: §Consequences does not acknowledge that load-time resolution may fail silently for binaries installed after dispatcher startup — the trusted-prefix list is a snapshot at registry-load time, not a live view.
- **H-2**: No explicit statement of which Rust module owns `TrustedPrefixList` — implementer must infer from context.
- **H-3**: §Rationale for Decision 2 (per-plugin graceful degradation) does not distinguish between `without_path: true` and `with_path: true` plugins; the degradation path is different for each.

## Part B — Amendments and Disposition

- v1.2→v1.3: B-1 addressed — `cfg(windows)` no-op path added with explicit test (Windows: trusted-prefix list is empty; all binaries degrade gracefully; test: `assert_eq!(resolve_binary_allow("git", &registry), None)` on Windows).
- v1.3→v1.4: H-1 acknowledged in §Consequences; §Rationale extended for H-2/H-3.
- v1.4→v1.5: Final reviewer pass — all BLOCKERs and HIGH items resolved. v1.5 `status: proposed`. **NOT RATIFIED** — pending human ratification gate and final ADR-043 ratification session.

## Part C — State at Close of Review

ADR-043 v1.5. `status: proposed`. CWE-706 closure architecture confirmed correct. Two prevented regressions: (1) B-3 pass-1 (inverted security rationale); (2) B-1 pass-3 (Windows platform regression). Human ratification required before implementation dispatch.
