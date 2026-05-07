---
document_type: per-story-adversary-review
story_id: S-12.06
pass: 2
phase: per-story-step-4.5
date: 2026-05-07
producer: adversary
prior-pass-classification: LOW
prior-findings-count: 5
---

# Per-Story Adversary Review — S-12.06 — Pass 2

## Scope confirmation

Reviewed the post-pass-1 state of the `## Context Injection Contract` section in `crates/hook-sdk/HOST_ABI.md` (lines 463–849, 387 lines total) and the bats test `plugins/vsdd-factory/tests/resolver-host-abi-context-injection.bats` against:
- Story spec S-12.06 (post-amendment, with bats filename corrected and File List/Test Plan/DoD updated)
- BC-1.13.001, BC-4.12.001 through BC-4.12.005 (the 6 anchored BCs)
- ADR-018

Verified pass-1 fixes are in place (regression check below); then re-derived findings from the post-fix state with fresh context.

## Pass-1 fix verification (regression check)

| Pass-1 finding | Fix state | Evidence |
|---|---|---|
| F-S12.06-1 (filename drift) | CLOSED | Story spec line 151 now references `resolver-host-abi-context-injection.bats`; line 224, 247, 273 updated; bats file at that path. |
| F-S12.06-2 (OD-5 invariant) | CLOSED | HOST_ABI.md lines 650–654 add the `**No inter-resolver dependencies (OD-5):**` paragraph in `### Resolver ABI Types`, citing BC-4.12.002 INV4. |
| F-S12.06-3 (`fail_closed`) | CLOSED | HOST_ABI.md line 528 adds `fail_closed` row to Registration field table with default and BC-4.12.001 PC6 citation. |
| F-S12.06-4 (capability_denied event) | CLOSED | HOST_ABI.md lines 723–730 add `**Telemetry on capability denial:**` paragraph documenting both `resolver.capability_denied` and `resolver.error` events with cross-references to BC-4.12.003 PC2 and BC-4.12.004 PC2. |

All four pass-1 LOW findings are closed. Frontmatter, body, and registration field table are all in sync. No regressions detected from the fix commits.

---

## Within-Story Findings

### FINDING [LOW] — Capability Model table invents host-function names that don't exist in the rest of HOST_ABI.md

WHY: HOST_ABI.md line 708 lists `host::log_info, host::log_warn, host::log_error` as resolver-available host functions. But the existing `## Host functions` section in the same document (line 244) declares the only logging API as a single function `log(level, msg_ptr, msg_len) -> ()` with the level encoded as an `i32` argument (Trace=0, Debug=1, Info=2, Warn=3, Error=4). There is no `log_info`/`log_warn`/`log_error` function defined anywhere else in HOST_ABI.md. The new section invents three function names that the existing host-functions section does not declare.

This drift originates in BC-4.12.003 PC4 (which uses the `host::log_info` triplet) and BC-4.12.003 description (line 39, also uses the triplet). The new section faithfully reproduces the BC's terminology — but a reader cross-referencing the new section to the existing host-functions section will find a contract mismatch within the same file.

EVIDENCE:
- New section: `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:708`
- Existing host-functions section: `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:244` (single `log(level, ...)` signature)
- BC source: `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.003.md:74-76` (uses the triplet)

IMPACT: An implementer in S-12.04/S-12.05 will need to choose: (a) add three new exports to the host linker matching the BC, or (b) keep the existing single `log` API and document the discrepancy. The current section doesn't pick a side. The "host::log is always available for diagnostics" closing clause in the table cell hints at the single-function model, but the cell labels still name three functions — the cell is internally inconsistent.

FIX: Either (a) update the new section's table cell to read `host::log` (the function actually defined) and align BC-4.12.003 PC4 in a separate fix, or (b) note explicitly that `host::log_info`/`log_warn`/`log_error` are convenience wrappers over `log(level, ...)` that the SDK provides, with `host::log` being the underlying single host export. Route hint: DOC (with cross-fix to BC-4.12.003 deferred to wave-gate).

---

### FINDING [LOW] — `event_type` example value `"SubagentStop"` lacks specification of allowed values

WHY: The `ResolverInput` JSON example at HOST_ABI.md line 618 uses `"event_type": "SubagentStop"` and the field-table description (line 627) says `"e.g., 'PreToolUse', 'SubagentStop'"`. These are real Claude Code event types, but the section never tells a resolver author what the closed enumeration is. BC-4.12.002 PC2 lists the field as `event_type: String` without enumeration. A resolver author must either (a) read Claude Code documentation externally, or (b) treat it as an opaque string. The "factory-agnostic" framing is fine — the section doesn't have to enumerate Claude Code events — but a one-sentence note saying "the value is the Claude Code platform event type; consult the host platform's reference for the canonical list" would close the reader-test gap.

EVIDENCE:
- HOST_ABI.md line 618 (example) and line 627 (description with two e.g. values)
- BC-4.12.002 PC2 at `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.002.md` (only types `event_type: String`, no enumeration constraint)

IMPACT: A resolver author building a resolver that branches on `event_type` will not know whether to expect `"PreToolUse"`, `"PostToolUse"`, `"SubagentStop"`, or some other string. Reader test partially fails for this field. Not a correctness defect; a discoverability gap.

FIX: Add a one-sentence note in the field table or below the example: "The `event_type` value is the host platform's event-type string (e.g., Claude Code dispatch events `PreToolUse`, `PostToolUse`, `SubagentStop`). The dispatcher passes this through unchanged; resolvers may treat it as an opaque key for branching." Route hint: DOC.

---

### FINDING [NITPICK_ONLY] — Terminology drift between `name` and `context_key` for the registry-key concept

WHY: BC-4.12.005 PC6 and EC-005 use the term `context_key` for the unique key in `resolvers-registry.toml`. BC-1.13.001 PC5 uses `context_key`. BC-4.12.005 description and HOST_ABI.md alternate between `name` (registry field — line 525) and `context_key` (in BC citations). The new section consistently uses `name` in its tables and prose but cites BC text that uses `context_key`. The end-result is correct (both terms refer to the same field), but a careful reader will notice the term swap and wonder whether `name` and `context_key` are the same thing.

EVIDENCE:
- HOST_ABI.md line 525: `| name | yes | The resolver's unique name. ... This name is the key under which the resolver's output is written ... |`
- BC-4.12.005 line 31 title: "two resolvers with the same `context_key` is a registry-load error"
- BC-4.12.005 PC6 line 82: "two resolver entries with the same `name` (i.e., two entries that would produce the same `context_key`)"

IMPACT: Minor — a careful reader has to do the equivalence inference. The BC itself acknowledges this (`name` produces the `context_key`). Not a defect; observed for transparency.

FIX: Either (a) add a one-sentence parenthetical near line 525: "This `name` field is also referred to as the resolver's `context_key` in the merge contract (BC-4.12.005)." or (b) leave as-is. Route hint: NONE (or DOC at author discretion).

---

## Deferred Findings

### DEFERRED [BC vs HOST_ABI inconsistency] — `host::log_info`/`log_warn`/`log_error` vs `log(level, ...)`

WHY: As noted in within-story Finding 1 above, BC-4.12.003 PC4 uses three separate log functions; the existing HOST_ABI host-functions section uses a single `log(level, ...)` API. The new section reproduces the BC's three-function terminology, creating an internal inconsistency in HOST_ABI.md. The within-story finding above asks for a doc-level reconciliation; the underlying BC↔ABI alignment is a system-level question.

EVIDENCE:
- BC-4.12.003 PC4: `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.12.003.md:74-76`
- HOST_ABI.md `## Host functions` § `log` definition: `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06/crates/hook-sdk/HOST_ABI.md:244`

CATEGORY: BC↔HOST_ABI consistency drift — wave-gate scope.

---

### DEFERRED (carried from pass-1) — BC-4.12.005 PC4 vs ADR-018 first-declared-wins vs last-write-wins

WHY: Pass-1 deferred finding still applies. ADR-018 line 167 says "first-declared-wins on collision at dispatch"; BC-4.12.005 PC4 says "last-write-wins at dispatch time". Section faithfully punts at line 786 ("Resolvers are merged in the order they are declared in `needs_context`") without picking a winner. BC-4.12.005 PC6 prevents the scenario by making duplicate `context_key` a registry-load error, so the inconsistency is unreachable, but the BC↔ADR drift remains.

CATEGORY: BC↔ADR consistency drift — wave-gate scope (carried from pass-1).

---

### DEFERRED (carried from pass-1) — Concurrency model for resolver invocation under-documented

WHY: Pass-1 deferred finding. BC-4.12.001 EC-003 mandates thread-safe cache lookup (Mutex/Arc); the new section's Resolver Lifecycle subsection is silent on concurrency. AC-002 doesn't require concurrency docs.

CATEGORY: Integration / Phase-5 scope (carried from pass-1).

---

### DEFERRED [system-level] — BC-4.12.003 PC2 vs BC-4.12.004 PC2 dual event-naming for capability denial

WHY: Pass-1 deferred finding still applies. The new section's `**Telemetry on capability denial:**` paragraph (lines 723–730) documents BOTH `resolver.capability_denied` AND `resolver.error` as fireable for the same denial — which is honest documentation but also documents a system-level BC↔BC overlap that hasn't been adjudicated. The section says "Both may fire for the same denial depending on dispatcher implementation" — this kicks the decision to the implementer in S-12.04.

CATEGORY: System-level (BC consolidation) — wave-gate / Phase-5 scope (carried from pass-1, partially absorbed by F-S12.06-4 fix).

---

## Self-validation (3 iterations)

**Iteration 1:** Initial drafted 5 findings. Dropped 2 that were rephrasings of pass-1 findings (telemetry event documentation; OD-5 documentation) since they are now CLOSED. Verified each remaining finding has fresh evidence not present in pass-1.

**Iteration 2:** Tightened Finding 1 (`host::log_info` triplet) — confirmed via grep that the function name does not appear elsewhere in HOST_ABI.md. Confirmed Finding 2 (`event_type` enumeration) is genuinely a discoverability gap not raised in pass-1.

**Iteration 3:** Demoted Finding 3 (`name` vs `context_key`) from LOW to NITPICK_ONLY after re-reading: the equivalence is implicit in the BC and a careful reader can reason it out. Demoted because it's a terminology footnote, not a content gap.

---

## Pass-2 vs Pass-1 novelty assessment

Pass-1 found 4 LOW + 1 NITPICK = 5 findings, focused on missing content (filename, OD-5, fail_closed, capability_denied event) plus the SDK-name nitpick.

Pass-2 found 2 LOW + 1 NITPICK = 3 findings, all genuinely new and orthogonal to pass-1:
- Finding 1 (host::log triplet vs single log) is internal HOST_ABI.md inconsistency — only visible after reading the existing host-functions section in conjunction with the new section.
- Finding 2 (event_type enumeration) is a reader-test gap not raised in pass-1.
- Finding 3 (name vs context_key) is a terminology drift that pass-1 missed.

No findings re-tread pass-1. Novelty: MODERATE (pass-1 fixes have shifted the surface; new findings are at the integration boundary between the new section and the existing HOST_ABI content, plus terminology consistency).

---

## Return Summary

(a) **Findings count by severity (within-story):**
- CRITICAL: 0
- HIGH: 0
- MEDIUM: 0
- LOW: 2
- NITPICK_ONLY: 1

(b) **Pass-1 findings that DID NOT properly close (regressions):**
- None. All 4 pass-1 LOW findings are CLOSED with frontmatter↔body↔table↔BC-citation alignment confirmed.

(c) **NEW findings novel to pass-2:**
1. [LOW] HOST_ABI.md internal inconsistency: new section's Capability Model table cites `host::log_info`/`log_warn`/`log_error` (3 functions) but the existing host-functions section defines only `log(level, ...)` (1 function with level argument).
2. [LOW] `event_type` example uses `"SubagentStop"` without telling resolver authors what the allowed values are or where to find the enumeration.
3. [NITPICK_ONLY] Terminology drift between `name` (used in HOST_ABI tables/prose) and `context_key` (used in BC titles/postconditions); same field, two names.

(d) **Deferred findings count + categories:**
- 4 deferred findings total: 1 new (BC↔HOST_ABI log API), 3 carried from pass-1 (BC↔ADR collision-order, concurrency model, BC↔BC capability-denied event-naming).

(e) **PASS_CLASSIFICATION: LOW**

(f) **Top findings:**
1. [LOW] Capability Model table cites three `host::log_*` functions; existing HOST_ABI host-functions section defines only `log(level, ...)`. Internal inconsistency within the same file. Path: `crates/hook-sdk/HOST_ABI.md:708` vs `:244`.
2. [LOW] `event_type` field example/description leaves the value space unspecified; a one-sentence note pointing to the host platform's event-type reference would close the reader-test gap.
3. [NITPICK_ONLY] `name` vs `context_key` terminology equivalence is implicit; a single parenthetical at line 525 would make it explicit.

---

**PASS_CLASSIFICATION: LOW**

Pass-2 yielded 2 LOW + 1 NITPICK_ONLY. Per per-story convergence semantics (3 consecutive NITPICK_ONLY required), this is **not** a NITPICK_ONLY pass — `passes_clean` does NOT increment. Recommend fix-routing the 2 LOW findings (especially Finding 1, which is internal HOST_ABI.md inconsistency a reader will hit immediately) before pass-3. Finding 2 is a discoverability improvement, route at author discretion. Finding 3 (NITPICK_ONLY) can be deferred or rolled in opportunistically.
