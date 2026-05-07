---
document_type: per-story-adversary-review
story_id: S-12.06
pass: 3
phase: per-story-step-4.5
date: 2026-05-07
producer: adversary
prior-pass-classification: LOW
prior-findings-count: 3
---

# Per-Story Adversary Review — S-12.06 — Pass 3

## Scope confirmation

Reviewed the post-pass-2 state of the `## Context Injection Contract` section in `crates/hook-sdk/HOST_ABI.md` (lines 463–849), the bats test `plugins/vsdd-factory/tests/resolver-host-abi-context-injection.bats`, and the story spec against:
- BC-1.13.001, BC-4.12.001 through BC-4.12.005 (the 6 anchored BCs)
- ADR-018
- Pass-1 and pass-2 review records (read for context, not to inherit conclusions)

Verified pass-2 fixes are in place; then re-derived findings from the post-fix state with fresh context, focusing on BC↔HOST_ABI surface coverage I had not previously sampled.

## Pass-2 fix verification (regression check)

| Pass-2 finding | Fix state | Evidence |
|---|---|---|
| F-S12.06-P2-1 (host::log naming) | CLOSED | `HOST_ABI.md:708` now reads `host::log(level, msg_ptr, msg_len)` — single function with level argument; SDK wrappers `log_info(msg)`, `log_warn(msg)`, `log_error(msg)` explicitly noted as "ergonomic wrappers ... over this base function." Aligns with `HOST_ABI.md:244` `log(level, msg_ptr, msg_len) -> ()` host-functions section. Internal HOST_ABI.md consistency restored. |
| F-S12.06-P2-2 (event_type enumeration) | CLOSED | `HOST_ABI.md:627` ResolverInput field-table cell for `event_type` now reads "The host platform's event-type string. For Claude Code dispatch events, common values include `PreToolUse`, `PostToolUse`, `SubagentStop`, `UserPromptSubmit`, `Stop`. The dispatcher passes this through unchanged; resolvers may treat it as an opaque key for branching. Consult the host platform's reference for the canonical list." Five Claude Code event values enumerated; canonical source pointer present; factory-agnostic framing preserved. |
| Pass-2 NITPICK_ONLY (`name` vs `context_key`) | INTENTIONALLY LEFT | `HOST_ABI.md:525` still uses `name`; BC-4.12.005 still uses `context_key`. Equivalence remains implicit. Acceptable per orchestrator's brief. |

All 2 pass-2 LOW findings are closed. No regressions detected. Bats test still passes (file unchanged: 31 tests, all assertions are content-presence checks; the new `event_type` and `host::log(level, ...)` text changes don't break any assertion).

---

## Within-Story Findings

### FINDING [LOW] — `resolver.capability_denied` event missing the third BC-required field (`resolved path that was attempted`)

WHY: BC-4.12.003 PC2 specifies that the `resolver.capability_denied` telemetry event carries **three** fields: "resolver name, denied path, **and the resolved path that was attempted**." HOST_ABI.md's "Telemetry on capability denial" paragraph (line 723–730) documents only two of these fields: "the resolver name and denied path." The third field ("resolved path") — which BC-4.12.003 PC2 distinguishes from the denied path (presumably the post-canonicalization path that the host attempted to resolve before the prefix check failed) — is omitted from the section.

This is not a deferred BC↔BC or BC↔ADR drift; it is a direct content gap between the HOST_ABI section's documentation of `resolver.capability_denied` and the BC's explicit field enumeration.

EVIDENCE:
- BC-4.12.003 PC2 at `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.003.md:64-65` ("with: resolver name, denied path, and the resolved path that was attempted")
- HOST_ABI.md `:723-730` ("the dispatcher emits a `resolver.capability_denied` telemetry event ... with the resolver name and denied path")

IMPACT: An implementer in S-12.04 building the `resolver.capability_denied` event emitter will read this section as the canonical reference and emit a 2-field event when the BC mandates 3 fields. S-12.04 adversary will then find this gap and force a fix-burst there. Catching it now (in S-12.06) is cheaper.

The distinction between "denied path" (what the resolver passed to `host::read_file`) and "resolved path that was attempted" (the canonicalized path the host computed before failing the prefix check) is forensically valuable: it lets operators detect path-traversal attempts where the user-supplied path looks innocent (`./.factory/foo`) but the resolved path leaks (`../../etc/passwd`). Omitting the third field weakens the security telemetry surface.

FIX: Update HOST_ABI.md `:723-730` to document all three fields: e.g., "with the resolver name, the denied path (the path the resolver passed to `host::read_file`), and the resolved path that was attempted (the canonicalized path against which the prefix check failed)." Route hint: DOC.

---

### FINDING [NITPICK_ONLY] — Dispatcher log-entry side-effect on resolver failure (BC-4.12.004 PC7) not documented in section

WHY: BC-4.12.004 PC7 mandates a dispatcher-side log entry on resolver failure: "In addition to the telemetry event, the dispatcher writes a log entry to `.factory/logs/` (or the configured log path) at error level documenting the resolver crash with the same fields as the telemetry event." The HOST_ABI.md section's "Error and Crash Isolation" subsection (lines 746–774) documents the `resolver.error` telemetry event in detail but says nothing about the parallel log-entry side-effect.

This is a minor omission because (a) the BC text uses the factory-agnostic phrasing "the configured log path" (the parenthetical `.factory/logs/` is the vsdd-factory default but the BC-level requirement is "configured log path"), so the section could document PC7 without leaking domain vocabulary; (b) a dispatcher implementer (S-12.04) reading BC-4.12.004 directly would still see PC7. However, if the HOST_ABI section is the single source of truth for resolver-platform implementers, the dual telemetry+log requirement is silently dropped here.

EVIDENCE:
- BC-4.12.004 PC7 at `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.004.md:87-89`
- HOST_ABI.md `:746-774` (no mention of "log entry," "dispatcher log," or "configured log path")

IMPACT: Minor — a dispatcher implementer who reads only the HOST_ABI section may emit `resolver.error` but skip the log entry. The BC source still mandates it, so an adversary will catch it in S-12.04. Probably below the LOW threshold; flagged for transparency.

FIX (optional): Add one sentence after the `resolver.error` table: "In addition to the telemetry event, the dispatcher writes an error-level log entry at the configured log path with the same fields. (BC-4.12.004 PC7)" Route hint: DOC at author discretion.

---

## Deferred Findings

### DEFERRED (carried from passes 1+2) — BC-4.12.005 PC4 vs ADR-018 first-declared-wins vs last-write-wins

WHY: ADR-018 says "first-declared-wins on collision at dispatch"; BC-4.12.005 PC4 says "last-write-wins at dispatch time". HOST_ABI.md `:786` punts ("Resolvers are merged in the order they are declared in `needs_context`") without picking a winner. PC6 prevents the scenario from being reachable, but the BC↔ADR drift remains.

CATEGORY: BC↔ADR consistency drift — wave-gate scope (carried unchanged from passes 1 and 2).

---

### DEFERRED (carried from passes 1+2) — Concurrency model for resolver invocation under-documented

WHY: BC-4.12.001 EC-003 mandates thread-safe cache lookup (Mutex/Arc); the HOST_ABI section's Resolver Lifecycle subsection is silent on concurrency. AC-002 doesn't require concurrency docs.

CATEGORY: Integration / Phase-5 scope (carried unchanged from passes 1 and 2).

---

### DEFERRED (carried from passes 1+2) — BC-4.12.003 PC2 vs BC-4.12.004 PC2 dual event-naming for capability denial

WHY: BC-4.12.003 PC2 says capability denials emit `resolver.capability_denied`; BC-4.12.004 PC2 lists `"capability_denied"` as an `error_kind` for `resolver.error`. HOST_ABI section now documents both honestly (`:723-730`) but the BC consolidation question remains open.

CATEGORY: System-level (BC consolidation) — wave-gate scope (carried, partially absorbed by F-S12.06-4).

---

### DEFERRED (new candidate, marked deferred not within-story) — BC-4.12.001 INV3 (same Engine instance) and PC4 (determinism) not surfaced in HOST_ABI section

WHY: BC-4.12.001 PC4 ("identical input → identical output, modulo I/O determinism") and INV3 ("resolver modules MUST be compiled with the same `Engine` instance used for hook plugins") are relevant to dispatcher implementers (S-12.04) and to resolver authors who reason about caching. The HOST_ABI section's Resolver Lifecycle subsection (`:570-595`) does not surface either.

I considered raising these as within-story LOW findings but demoted to deferred because (a) PC4's determinism is partially implicit in WASM sandbox semantics; (b) INV3's same-Engine requirement is dispatcher-impl territory (S-12.04 scope, not resolver-author scope) and the HOST_ABI section is not the canonical place for dispatcher-internal architecture. The BC source remains authoritative.

CATEGORY: Phase-5 (S-12.04 dispatcher implementation) — flagged for the S-12.04 author to read BC-4.12.001 INV3/PC4 directly rather than relying on HOST_ABI.md alone.

---

## Self-validation (3 iterations)

**Iteration 1:** Initial draft surfaced 5 candidate findings (A through G excluding B which was already deferred). Dropped E (event-emitter ambiguity — section consistently attributes events to the dispatcher) and F (BC-4.12.004 PC2 4-field error event — verified all 4 fields are documented at `:758-762`). Kept A, C, D, G.

**Iteration 2:** Demoted D (BC-4.12.001 INV3 same-Engine) from LOW to deferred — it's dispatcher-impl territory, not a resolver-author or HOST_ABI-reader contract. Demoted C (BC-4.12.001 PC4 determinism) from LOW to deferred for the same reason — wasmtime determinism is largely implicit by exclusion (no clock/random in the resolver linker). Both noted in the new deferred entry.

**Iteration 3:** Demoted A (dispatcher log entry on PC7) from LOW to NITPICK_ONLY — the BC mandates it but the omission is forensically minor; flagged for transparency at author discretion. Kept G (resolved-path field on `resolver.capability_denied`) as LOW — direct documented-field omission with concrete S-12.04 implementation impact and security-telemetry value.

Confirmed all retained findings have file:line evidence. No retreading of pass-1 or pass-2 findings.

---

## Pass-3 vs Pass-2 novelty assessment

Pass-1 found 4 LOW + 1 NITPICK = 5 findings (content gaps + filename drift).
Pass-2 found 2 LOW + 1 NITPICK = 3 findings (HOST_ABI internal log-API consistency, event_type enumeration, terminology drift).
Pass-3 finds 1 LOW + 1 NITPICK = 2 findings:
- LOW (G): `resolver.capability_denied` 3rd field omission — orthogonal to passes 1+2; touches a different BC subsystem (BC-4.12.003 PC2 event-fields, not the broader BC↔BC consolidation question raised in pass-1).
- NITPICK_ONLY (A): dispatcher log entry omission — flagged for transparency, below the LOW threshold.

Both findings are genuinely novel and not retreads. Novelty: LOW-to-MODERATE — the surface area with substantive content gaps is shrinking, and most candidate findings I considered demoted naturally to "deferred" (S-12.04 scope) or "below threshold." This is consistent with a converging story.

---

## Return Summary

(a) **Findings count by severity (within-story):**
- CRITICAL: 0
- HIGH: 0
- MEDIUM: 0
- LOW: 1
- NITPICK_ONLY: 1

(b) **Pass-2 fix verification:**
- Both pass-2 LOW fixes (F-S12.06-P2-1 host::log naming, F-S12.06-P2-2 event_type enumeration) are PRESENT and CORRECT. Internal HOST_ABI.md consistency restored. Evidence cited in regression-check table above. The 1 unfixed pass-2 NITPICK_ONLY (`name` vs `context_key`) remains acceptable.

(c) **NEW findings (or "none — NITPICK_ONLY"):**
- 1 NEW LOW: `resolver.capability_denied` event documented with only 2 of the 3 BC-mandated fields (missing "resolved path that was attempted"). File `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:723-730` vs `BC-4.12.003.md:64-65`.
- 1 NEW NITPICK_ONLY: Dispatcher log-entry side-effect (BC-4.12.004 PC7) not documented in `Error and Crash Isolation` subsection.

(d) **Deferred findings additions to S-12.06:**
- 1 NEW deferred (carried for S-12.04 phase-5 attention): BC-4.12.001 INV3 (same Engine) + PC4 (determinism) not surfaced in HOST_ABI section. Plus 3 carried-from-prior-passes deferreds (BC↔ADR collision-order, concurrency model, BC↔BC capability-denied event-naming).

(e) **PASS_CLASSIFICATION value:** LOW

(f) **Top finding:**
- [LOW] `HOST_ABI.md:723-730` documents 2 fields on the `resolver.capability_denied` event (resolver name, denied path) but `BC-4.12.003.md:64-65` mandates 3 (resolver name, denied path, **resolved path that was attempted**). An S-12.04 dispatcher implementer reading the HOST_ABI section as canonical reference will emit a 2-field event, weakening forensic security telemetry and producing a downstream adversary finding. Direct documented-field omission; not a deferred BC↔BC drift. Fix: update the paragraph to enumerate all three fields. Route: DOC.

---

**PASS_CLASSIFICATION: LOW**

Pass-3 yielded 1 LOW + 1 NITPICK_ONLY. Per per-story convergence semantics (3 consecutive NITPICK_ONLY required), this is **not** a NITPICK_ONLY pass — `passes_clean` does NOT increment. The orchestrator's "honest expectation" note ("pass-3 may genuinely be NITPICK_ONLY") is acknowledged; however, the `resolver.capability_denied` 3-field-omission finding is a real BC-mandated content gap with concrete S-12.04 downstream impact, and demoting it to NITPICK_ONLY would mis-classify a substantive defect to satisfy convergence pressure. The brief explicitly forbids inventing findings to delay convergence; symmetrically, demoting genuine LOWs to satisfy convergence pressure would also violate honest classification.

Recommend: fix-route Finding G (the 3-field documentation), then re-pass. The NITPICK_ONLY (dispatcher log entry) can be folded into the same fix or deferred at author discretion. After pass-4 picks up the fix, the section should be substantively LOW-clean and pass-4/5/6 can converge on NITPICK_ONLY honestly.

---

**Relevant absolute file paths:**
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md` (lines 463–849: Context Injection Contract section)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/plugins/vsdd-factory/tests/resolver-host-abi-context-injection.bats` (bats test, unchanged)
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.06-host-abi-context-injection-contract.md` (story spec)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.003.md` (BC source for finding G)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.004.md` (BC source for finding A)
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.06/adversary-pass-1.md` (pass-1 review)
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.06/adversary-pass-2.md` (pass-2 review)
