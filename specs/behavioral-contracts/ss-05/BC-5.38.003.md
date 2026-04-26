---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: ""
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-003"
subsystem: "SS-05"
capability: "CAP-016"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.38.003
section: "5.38"
---

# BC-5.38.003: framework integration wiring may have minimal real code for cargo check; handler business logic must be todo!()

## Description

Some framework integration patterns (e.g., Tower `Service` trait impl, Axum router wiring, tonic gRPC server struct) require a minimal amount of real code to compile — particularly the `poll_ready` method returning `Poll::Ready(Ok(()))`. This structural minimum is permitted in stub commits. However, the business logic inside request handlers, service methods, or gRPC procedure implementations must remain `todo!()`. The distinction is: structural wiring that the compiler requires vs. behavioral logic that tests must drive.

## Preconditions

1. The stub-architect is creating a stub commit for a `tdd_mode: strict` (or absent) story.
2. A framework integration pattern (Tower, Axum, tonic, actix-web, etc.) requires structural method implementations to compile.
3. The structural minimum code contains no business logic — only protocol plumbing (e.g., waking the waker, returning Poll variants, delegating to `self.inner.poll_ready(cx)`).

## Postconditions

1. Framework wiring methods with structural-minimum real code are present in the stub commit.
2. Handler methods, service call implementations, and procedure bodies contain `todo!()`.
3. The stub-architect's report lists each wiring method as WIRING-EXEMPT with a justification (e.g., "Tower poll_ready — structural minimum for cargo check").
4. Tests for handler behavior remain RED at Step 3 Red Gate.

## Invariants

1. WIRING-EXEMPT designation applies only to methods where the compiler would reject `todo!()` or where returning a fixed structural value is definitionally correct (e.g., `poll_ready` that always succeeds because the service has no async initialization).
2. Business logic that happens to be simple does NOT qualify as wiring — if it can be moved to a `todo!()` without breaking compilation, it must be.
3. WIRING-EXEMPT methods are excluded from the RED_RATIO denominator alongside GREEN-BY-DESIGN methods.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tower Service::call() method (the actual handler dispatch) | Must be `todo!()`. The `call` method dispatches business logic; it is NOT wiring. |
| EC-002 | Axum handler function: `async fn create_user(Json(body): Json<CreateUser>) -> impl IntoResponse` | Must be `todo!()`. Handler body is business logic regardless of framework. |
| EC-003 | tonic `fn check_health(&self, req: Request<()>) -> Result<Response<()>, Status>` | Must be `todo!()`. gRPC procedure implementations are business logic. |
| EC-004 | `impl Default for Config { fn default() -> Self { Self { timeout_ms: 30_000, ..Default::default() } } }` | May be implemented. Structural defaults with no logic. Flag as GREEN-BY-DESIGN. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| Tower `poll_ready` returning `Poll::Ready(Ok(()))` in stub | WIRING-EXEMPT; test (if any) passes; excluded from RED_RATIO | happy-path (wiring exemption) |
| Tower `call()` dispatching a service handler with `todo!()` body | RED at Step 3; correct TDD discipline | happy-path (correct stub) |
| Axum handler with real `create_user` logic in stub | VIOLATION. Must be `todo!()` per BC-5.38.001. | negative |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-063 | RED_RATIO correctly excludes WIRING-EXEMPT tests (same mechanism as GREEN-BY-DESIGN) | proptest |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC refines the stub discipline rule with a framework-specific exemption, ensuring the red-gate enforcement is accurate for Rust ecosystem patterns without producing false positives. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/stub-architect.md, plugins/vsdd-factory/workflows/phases/per-story-delivery.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-003 |
| FR | FR-043 |

## Related BCs

- BC-5.38.001 — parent (primary todo!() obligation; this BC defines framework-specific exemptions)
- BC-5.38.002 — sibling (pure data mapping exemption, related but distinct category)
- BC-8.29.001 — depends on (WIRING-EXEMPT entries excluded from RED_RATIO denominator)

## Architecture Anchors

- `plugins/vsdd-factory/agents/stub-architect.md` — WIRING-EXEMPT reporting protocol
- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — exemption consumption at Step 3

## Story Anchor

S-7.03

## VP Anchors

- VP-063 — RED_RATIO computation correctness
