---
document_type: per-story-adversary-review
story_id: S-12.06
pass: 1
phase: per-story-step-4.5
date: 2026-05-07
producer: adversary
---

# Per-Story Adversary Review — S-12.06 — Pass 1

## Scope confirmation

Reviewed the new `## Context Injection Contract` section in `crates/hook-sdk/HOST_ABI.md` (lines 461–833 of the worktree file) and the new bats test `plugins/vsdd-factory/tests/resolver-host-abi-context-injection.bats` against:
- Story spec S-12.06 (10 ACs, 5 ECs, 5 Architecture Compliance Rules)
- BC-1.13.001, BC-4.12.001 through BC-4.12.005 (the 6 anchored BCs)
- ADR-018 (the architectural design)

Reviewed for: AC coverage, BC compliance, factory-agnostic semantics, internal consistency, reader-test gaps, and bats test correctness.

---

## Within-Story Findings

### FINDING [LOW] — Bats test filename drift from story spec

WHY: Story spec File List (line 151) and Test Plan (line 273) name the bats file `resolver_host_abi_agnostic.bats` (underscores), but the actual file shipped is `resolver-host-abi-context-injection.bats` (hyphens, different stem). This breaks story-spec ↔ implementation traceability for documentation purposes.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.06-host-abi-context-injection-contract.md:151` declares `plugins/vsdd-factory/tests/resolver_host_abi_agnostic.bats`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/plugins/vsdd-factory/tests/resolver-host-abi-context-injection.bats:1` ships at a different name

IMPACT: An operator reading the story spec to find the falsifiable test file will get a "file not found" until they realize the filename was changed. Definition of Done line 247 references `bats plugins/vsdd-factory/tests/resolver_host_abi_agnostic.bats` which would fail verbatim.

FIX: Either (a) rename the bats file to match the spec, or (b) amend the story spec to update File List, Test Plan, and DoD entries to the actual filename. Route hint: SPEC or TEST.

---

### FINDING [LOW] — Resolver-output isolation invariant (BC-4.12.002 INV4) under-documented in section prose

WHY: BC-4.12.002 INV4 codifies a load-bearing safety invariant: "The resolver MUST treat [`plugin_config`] as read-only data for its own computation; it cannot observe or depend on other resolvers' output (OD-5: no inter-resolver dependencies)." The HOST_ABI.md section mentions `plugin_config` is "read-only" in a single table cell (line 630) but never states the OD-5 no-inter-resolver-dependency invariant explicitly. A new resolver author reading this section could reasonably believe resolver B can read resolver A's output via `plugin_config`, since the section never forbids it.

EVIDENCE:
- BC-4.12.002 INV4 at `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.002.md:118-121`
- ADR-018 OD-5 at `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md:155-162`
- Only mention in section: `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:630` (table cell parenthetical "resolver outputs are not yet merged")

IMPACT: A resolver author building two cooperating resolvers may architect them assuming resolver B sees resolver A's output, causing subtle correctness bugs when the system actually invokes them with the static-only `plugin_config`. The "reader test" fails for OD-5.

FIX: Add a one-sentence explicit invariant under the `Resolver ABI Types` or `Merging Contract` section: "Resolvers cannot observe or depend on other resolvers' outputs. Each resolver receives only the static `plugin_config` from `hooks-registry.toml`; resolver outputs are merged after ALL resolvers have completed (OD-5: no inter-resolver dependencies)." Route hint: DOC.

---

### FINDING [LOW] — `fail_closed` resolver-entry semantics (BC-4.12.001 PC6) not documented

WHY: BC-4.12.001 PC6 specifies a behavior: "If a resolver `.wasm` artifact fails to compile..., the dispatcher emits `resolver.load_error`..., then fails startup (unless the resolver entry has `fail_closed = false`, in which case the entry is skipped with a warning)." The HOST_ABI.md section's Resolver Registration table (lines 521–527) lists `name`/`plugin`/`path_allow` as the registry fields but omits `fail_closed`. A resolver author reading the section will not know `fail_closed` exists or how to opt out of fail-closed behavior.

EVIDENCE:
- BC-4.12.001 PC6 at `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.001.md:70-74`
- HOST_ABI.md registration table at `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:521-527` lists only 3 fields

IMPACT: Section is partially incomplete relative to BC-4.12.001. A factory operator authoring `resolvers-registry.toml` from this reference may not know about `fail_closed`. Story spec AC-002 doesn't explicitly require `fail_closed`, so this is a gap relative to BC, not relative to AC.

FIX: Either add `fail_closed` to the registration field table, or note explicitly that compile failures are fail-loud by default. Route hint: DOC.

---

### FINDING [LOW] — `resolver.capability_denied` event (BC-4.12.003 PC2) not separately documented

WHY: BC-4.12.003 PC2 specifies that a denied path read emits a distinct telemetry event `resolver.capability_denied` (with `denied_path` and `attempted` fields). The HOST_ABI.md section's Capability Model section discusses the `CapabilityDenied` return code (lines 711–714) but does NOT mention the `resolver.capability_denied` event. The `resolver.error` table (lines 740–747) lists `"capability_denied"` as one `error_kind` value but conflates it with the broader `resolver.error` event. Per BC-4.12.003 PC2, capability denials should emit `resolver.capability_denied`, not `resolver.error`.

EVIDENCE:
- BC-4.12.003 PC2 at `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.003.md:60-66` ("emits a `resolver.capability_denied` telemetry event")
- BC-4.12.004 PC2 at `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.004.md:65-72` (lists `"capability_denied"` as an `error_kind` for `resolver.error`)
- HOST_ABI.md only mentions the `error_kind` value at `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:744`

IMPACT: There is a real BC-level conflict between BC-4.12.003 (says emit `resolver.capability_denied`) and BC-4.12.004 (says emit `resolver.error` with `error_kind: "capability_denied"`). The HOST_ABI.md section silently picks BC-4.12.004's framing without acknowledging the distinction. Implementers in S-12.04 will see ambiguity. This is partially out-of-story (BC drift) but the section had an opportunity to clarify and didn't.

FIX: Either (a) document both event names with a note about the relationship, or (b) explicitly pick one and note the other is subsumed. Route hint: DOC (and flag the underlying BC↔BC drift to wave-gate; see deferred findings).

---

### FINDING [NITPICK_ONLY] — `vsdd-hook-sdk` SDK crate name appears in a "factory-agnostic" section

WHY: The story's load-bearing AC-008 invariant says the section uses "only factory-agnostic vocabulary." The bats test forbids `wave|story|cycle|STATE.md|wave-state` etc. but does not (and cannot) forbid `vsdd-hook-sdk` because that is the actual SDK crate name. Lines 653, 658, 664 of the new section reference `vsdd-hook-sdk`. This is unavoidable — the SDK truly is named `vsdd-hook-sdk` — but it is a minor semantic crack in the "factory-agnostic" framing the section claims.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:653` ("Resolver plugins are authored using the `resolver-authoring` feature flag on `vsdd-hook-sdk`.")
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:658,664` (Cargo.toml/use-statement examples reference `vsdd-hook-sdk`)

IMPACT: A reader from a different factory would see `vsdd-hook-sdk` and either rename it or adapt examples. This is a known SDK-naming reality, not a defect.

FIX: No change required for this story. Possible future cleanup: rename the SDK crate to a factory-agnostic name (e.g., `host-sdk`) — but that is a system-level concern. Route hint: NONE (acknowledge as known constraint).

---

## Deferred Findings

### DEFERRED [cross-story / BC↔ADR drift] — BC-4.12.005 PC4 vs ADR-018 collision-order contradiction

WHY: ADR-018 line 167 says merge semantics include "first-declared-wins on collision at dispatch." BC-4.12.005 PC4 (lines 71-76 of `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.005.md`) says "B's value wins over A's value (last-write-wins at dispatch time)." These contradict. HOST_ABI.md line 770 punts and just says "Resolvers are merged in the order they are declared in `needs_context`" without picking a winner. Since BC-4.12.005 PC6 makes duplicate `context_key` a registry-load error, this scenario should never reach dispatch — but the BC↔ADR contradiction remains.

EVIDENCE:
- ADR-018: `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md:167`
- BC-4.12.005 PC4: `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.005.md:71-76`

IMPACT: Implementers in S-12.03 will need to pick one. Because PC6 prevents the scenario from being reachable, it is mostly safe. But a sibling resolver with a tactically-similar key would expose this.

CATEGORY: BC↔ADR consistency drift — wave-gate scope (this is not S-12.06's problem to fix; the documentation accurately punts on a pre-existing inconsistency).

---

### DEFERRED [system-level] — BC-4.12.003 PC2 vs BC-4.12.004 PC2 duplicate event-naming

WHY: As noted in Finding 4 above, BC-4.12.003 PC2 says capability denials emit `resolver.capability_denied`; BC-4.12.004 PC2 says they emit `resolver.error` with `error_kind: "capability_denied"`. Both BCs cite the same architectural decision but specify different event surfaces. Resolution requires either consolidation (drop one event) or explicit documentation of both (both events fire on a denial).

EVIDENCE: same BCs cited above

CATEGORY: system-level (BC consolidation problem — for product-owner adjudication at wave-gate or in an F2-amendment).

---

### DEFERRED [integration / dispatcher implementation] — Section does not cover concurrency model for resolver invocation

WHY: BC-4.12.001 EC-003 mentions concurrent dispatches needing the same resolver and requires "Cache lookup must be thread-safe (Mutex or Arc)." The HOST_ABI.md section silently inherits this from the existing plugin loader pattern but doesn't say so. AC-002 doesn't require concurrency docs.

CATEGORY: integration / system-level (Phase-5 will surface dispatcher concurrency correctness).

---

## Self-validation (3 iterations)

**Iteration 1**: Initial findings dropped 2 vague ones ("could be clearer"). Kept 5 actionable.
**Iteration 2**: Confirmed all findings have file:line evidence. Promoted Finding 1 (filename drift) from MEDIUM to LOW after re-reading: the test still works correctly, only the docs are out of sync.
**Iteration 3**: Demerged Finding 4 from a HIGH (BC drift) into a LOW within-story (the section faithfully implements one BC's framing) plus a deferred (cross-BC adjudication).

---

## Return Summary

(a) **Findings count by severity (within-story):**
- CRITICAL: 0
- HIGH: 0
- MEDIUM: 0
- LOW: 4
- NITPICK_ONLY: 1

(b) **Deferred findings count + categories:**
- 3 deferred findings: 1 BC↔ADR drift, 1 system-level BC consolidation, 1 integration-level (concurrency model documentation)

(c) **PASS_CLASSIFICATION: LOW**

(d) **Top 3 within-story findings:**
1. Bats test filename drift from story spec (`resolver-host-abi-context-injection.bats` vs spec's `resolver_host_abi_agnostic.bats`) — breaks DoD line 247 verbatim invocation.
2. BC-4.12.002 INV4 / OD-5 "no inter-resolver dependencies" not stated in section prose — reader test fails for a load-bearing safety invariant.
3. BC-4.12.001 PC6 `fail_closed` registry field not documented in registration table — registry author won't know it exists.

(e) **Factory-agnostic check (semantic, not just grep):**
CONFIRMED PASS. The new section (lines 463–833) contains no vsdd-domain leak in semantic content. No mention of waves, stories, cycles, articles, companions, wave-state.yaml, or STATE.md. Examples use `my-context` and `my-hook` placeholders. Path-allow examples use `.factory/` as a generic prefix (which is a vsdd-factory convention but is also a generic dotfile pattern that any factory could adopt — not a domain leak). The only "vsdd" string within-section is the `vsdd-hook-sdk` SDK crate name (lines 653/658/664), which is structurally unavoidable since the SDK is genuinely named that. This is captured as the NITPICK finding above; it is not a true semantic leak.

---

**Pass-1 outcome:** PASS_CLASSIFICATION = LOW. Per per-story convergence semantics, this pass yielded 4 LOW + 1 NITPICK findings within-story. The story is NOT yet at "3 consecutive NITPICK_ONLY" (this is pass 1 with substantive findings). Recommend fix-routing the 4 LOW findings (especially Finding 1 filename drift, which breaks the spec's literal DoD command) before proceeding to pass-2. Findings 2–4 are content gaps that improve the section's "reader test" usefulness but are not blocking — operator judgment on whether to fix in this story or defer to a follow-up story is appropriate.