---
document_type: architecture-decision-record
level: L3
adr_id: ADR-033
version: "1.0"
title: "ADR-033: Cross-family cognitive-diversity guarantee — honest capability description and conditional cross-family dispatch"
status: accepted
date: 2026-07-27
producer: architect
timestamp: 2026-07-27T00:00:00Z
deciders:
  - architect
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-013 (adversarial review structure — cycle-keyed convergence; 3-CLEAN criterion whose cross-family independence assumption this ADR corrects)
  - ADR-031 (E-21 data-loss hardening — SubagentStop gate surface is adjacent to Decision 4's silent-failure class)
anchors:
  - SS-05
  - SS-06
subsystems_affected:
  - SS-05
  - SS-06
last_amended: "2026-07-27 (v1.0) — initial ruling (architect): cross-family cognitive-diversity guarantee adjudication."
modified:
  - "2026-07-27 (v1.0)"
---

# ADR-033: Cross-family cognitive-diversity guarantee — honest capability description and conditional cross-family dispatch

## Context

Three agent definitions in `plugins/vsdd-factory/agents/` assert cognitive diversity
through cross-family model dispatch:

| Agent | Claim | Actual model (observed 2026-07-27, n=3) |
|-------|-------|----------------------------------------|
| `adversary.md` | "Uses different model for genuine perspective diversity" | `claude-opus-5` (model: opus pin) |
| `spec-reviewer.md` | "using Gemini — a different model family" / "GPT-5.4" (adversary) | `claude-opus-5` (model: opus pin) |
| `code-reviewer.md` | "a DIFFERENT model family than the Builder (Claude)" | `claude-sonnet-4-6` (model: sonnet pin) |

Evidence: all three agents were dispatched without model override, and each read
`.factory/specs/architecture/ARCH-INDEX.md` (or `policies.yaml`) and returned version
strings that are only present in that repo — confirming Claude resolved all three
dispatches, not Gemini or GPT.

`spec-reviewer.md` independently flagged the gap: *"The cognitive-diversity guarantee
that is this agent's entire reason for existing is not being satisfied in this
environment… Any convergence claim resting on spec-reviewer as an independent third
perspective is currently resting on a same-family reviewer."*

**Available cross-family mechanisms, assessed 2026-07-27:**

- `agy` (antigravity-cli at `/opt/homebrew/bin/agy`): requires Google OAuth
  authentication. Live probe: `echo "ping" | agy --print --model gemini-2.5-flash`
  timed out after 30 s with `Error: authentication timed out.` Google OAuth URL
  presented but not completed. **NOT available without operator action.**
- `gemini` CLI at `/opt/homebrew/bin/gemini`: returns
  `IneligibleTierError: This client is no longer supported for Gemini Code Assist
  for individuals.` **NOT available.**
- OpenAI CLI: not found on PATH. **NOT available.**

**D-927 MISDIAGNOSIS REFERENCE:** D-927 is recorded as "engine P0: 7 opus-pinned
agents die silently." This diagnosis is falsified by the n=3 dispatch table above:
`model: opus` resolves to `claude-opus-5` and agents complete normally. D-927's
proposed remedy (repin all 7 opus agents) would have been an unnecessary destructive
change. The correction burst (a separate state-manager dispatch) will record the
falsification as a D-NNN codification; this ADR references the context but does
not duplicate the bookkeeping.

**Why this is load-bearing.** BC-5.39.001 (3-CLEAN convergence protocol) assumes
cross-family independent review. If cross-family dispatch was never wired, the
assumption has never held — including the 11-pass E-21 spec convergence, the 60+
pass F5 cycles, and the in-flight 21-pass S-21.04 cascade. The value delivered by
those cascades was real (fresh context + distinct prompting + information asymmetry),
but not of the cross-family class originally claimed.

**Adjacent silent-failure class.** Two SubagentStop gate behaviors interact with
review dispatches:

1. `handoff-validator` treats results shorter than a minimum threshold as
   `subagent_truncated_result` (reproduced: `result_len: 11`). A short but legitimate
   review result can be silently discarded.
2. `validate-pr-review-posted` blocks any SubagentStop dispatch with no PR in the
   payload. A review agent dispatched outside a PR context silently fails at the
   SubagentStop gate.

These are related to Decision 4 but are not caused by the model-family issue.

## Decisions

### Decision 1 — Honest description; conditional cross-family as an opt-in enhancement

**Ruling:** Amend the three agent definitions to remove false model-family claims.
Replace with honest disclosure of same-family operation plus an opt-in cross-family
enhancement path. Do NOT remove the review roles — they deliver real value.

**Rationale:**

Fresh-context asymmetry provides measurable independence benefits:
- Adversary cannot see prior review passes (information asymmetry is hard; it does
  not require model-family difference to catch distinct defect classes).
- 60+ F5 passes and 21+ S-21.04 passes demonstrate repeated catch of genuine defects
  under same-family operation.
- "Same-family fresh-context review with distinct adversarial prompting" is a
  defensible position. "Cross-family Gemini review" is not, when Gemini is not
  dispatched.

Amending docs is not a defer-pattern under the Canonical Principle. The capability
being deferred is cross-family dispatch (an opt-in future enhancement). The capability
being delivered now is honest description. The Canonical Principle forbids shipping
partial features that need later cleanup; an honest description IS the production-grade
artifact here.

**Cross-family as opt-in enhancement:** When the operator configures and authenticates
`agy` (path: `/opt/homebrew/bin/agy`, Flash model: `gemini-2.5-flash`, invocation:
STDIN-pipe with `--print`), the factory CAN dispatch review agents through it. This is
not a default; it is an explicitly enabled enhancement. Detection gate:
environment variable `VSDD_CROSS_FAMILY_DISPATCH=enabled`. Absent this variable,
agents operate same-family and review artifacts record `model_family: claude` in their
frontmatter (per Decision 4).

**Agent description correction mandate:**
- `adversary.md`: remove "Uses different model for genuine perspective diversity";
  replace with "Uses fresh context and information asymmetry to provide independent
  review. Operates same-family (Claude) unless `VSDD_CROSS_FAMILY_DISPATCH=enabled`."
- `spec-reviewer.md`: remove "using Gemini" / "GPT-5.4" / "third model family";
  replace with "Operates same-family (Claude) unless `VSDD_CROSS_FAMILY_DISPATCH=enabled`.
  Provides constructive second opinion via fresh context and distinct prompting strategy."
- `code-reviewer.md`: remove "DIFFERENT model family than the Builder (Claude)";
  replace with "Provides cognitive diversity through fresh context, distinct role
  constraints, and review-focused prompting. Same-family (Claude) unless
  `VSDD_CROSS_FAMILY_DISPATCH=enabled`."

Implementation route: story-writer to author S-22.01 (see §Follow-up Story Stubs).
This ADR decides policy; agent files are not edited in this dispatch.

### Decision 2 — Cross-family dispatch mechanism when `VSDD_CROSS_FAMILY_DISPATCH=enabled`

**Mechanism:** `agy --print --model gemini-2.5-flash`

- Prompt delivered via STDIN (not argv). The orchestrator constructs a full prompt
  string and pipes it: `printf '%s' "$PROMPT" | agy --print --model gemini-2.5-flash`.
- Model selection: `gemini-2.5-flash` only. `gemini-2.5-pro` (and likely `pro`
  variants) stalls in practice per operational notes. Flash models are confirmed
  reliable.
- Execution: run `--print` calls sequentially. Never run concurrent `agy` dispatches
  in parallel; the tool is session-local and may have internal state.
- Authentication preflight: before any cross-family dispatch, run a probe:
  `echo "ping" | agy --print --model gemini-2.5-flash 2>&1`. If the output contains
  "authentication" or non-zero exit: surface a loud error with the exact OAuth URL
  and halt the dispatch. Never silently fall back to Claude when
  `VSDD_CROSS_FAMILY_DISPATCH=enabled` is set (see Decision 4).

**Roles requiring cross-family when configured:**
- `adversary` (highest priority — cross-family diversity is most load-bearing here)
- `spec-reviewer` (third perspective after adversary)
- `code-reviewer` (secondary reviewer)

**Roles where fresh-context suffices regardless:**
All non-review specialist agents (implementer, test-writer, story-writer, product-owner,
architect, state-manager, etc.) produce artifacts, not reviews. Model-family diversity
is only load-bearing for REVIEW roles whose independence claim matters to convergence.

**Model provenance recording:** All review artifacts (adversarial review passes,
spec-reviewer outputs, code-reviewer outputs) MUST include frontmatter:
```yaml
model_family: claude   # or: gemini
model_id: claude-opus-5  # or: gemini-2.5-flash
```
A review artifact without these fields is incomplete (per Decision 4 §Fail-loud Class 3).
This requirement applies immediately — both current same-family operation and future
cross-family operation must record provenance.

### Decision 3 — Retrospective disposition of prior convergences

**Ruling:** Do NOT invalidate prior convergences. Annotate historical convergences
with a disclosure. Only the human can authorize retroactive invalidation.

**Rationale:** The value delivered by prior cascades was real. Adversarial review with
fresh context and information asymmetry caught genuine defects across 60+ F5 passes and
21+ S-21.04 passes. The limitation is that those convergences did not satisfy a
cross-family independence criterion that was never actually wired. Invalidating them
would destroy real engineering work on a technicality. Disclosure preserves the
historical record while being honest about the limitation.

**Annotation protocol:** Each active cascade INDEX.md or convergence record MUST
receive a disclosure block at the top of its Convergence Status section:

```
> **ADR-033 Disclosure (2026-07-27):** Convergence declared under same-family
> fresh-context review. Cross-family guarantee (Gemini/GPT-5.4) was not satisfied;
> all adversary/spec-reviewer/code-reviewer dispatches resolved to Claude. Value
> delivered was real (information asymmetry + distinct prompting), but not of the
> cross-family diversity class originally claimed. Per ADR-033 §Decision 3;
> retroactive invalidation requires human authorization.
```

**Active cascade disposition:** S-21.04 LOCAL cascade (streak 0/3 at pass-21) may
continue under same-family review WITH disclosure. The streak is against the 3-CLEAN
criterion, which continues to hold on its merits (no findings resets). The disclosure
does not reset the streak; it annotates the convergence record.

### Decision 4 — Fail-loud on unresolvable model pins and silent-failure classes

**Class 1 — D-927 falsified (not a silent failure):**
`model: opus` resolves correctly to `claude-opus-5` in this environment. The 7
opus-pinned agents complete normally. D-927's "die silently" characterization is
incorrect. No fix is needed for model pin resolution. A state-manager D-NNN correction
burst records the falsification; this ADR documents the architectural ruling.

**Class 2 — Cross-family requested but authentication fails:**
If `VSDD_CROSS_FAMILY_DISPATCH=enabled` and the agy probe fails, the orchestrator MUST
emit a loud error to the transcript:
```
[ADR-033 CROSS-FAMILY AUTH FAILURE] agy authentication required before cross-family
dispatch. Run: agy (no --print flag) to complete OAuth, then retry. Do NOT fall back
to same-family silently.
```
Silent fallback to Claude while `VSDD_CROSS_FAMILY_DISPATCH=enabled` is set is
forbidden. The failure must be surfaced and halted.

**Class 3 — Review artifacts without model provenance:**
A review artifact (adversarial review pass, spec-reviewer output, code-reviewer output)
produced without `model_family:` + `model_id:` frontmatter fields is incomplete. The
producing agent MUST include these fields. Reviewers reading an artifact without these
fields MUST treat the provenance as unknown and flag it. State-manager adding a
disclosure annotation (Decision 3) should also add `model_family: claude` +
`model_id: claude-opus-5` to the frontmatter of any review artifact lacking it.

**Class 4 — SubagentStop gates silently discarding review results:**
`handoff-validator` and `validate-pr-review-posted` are both SubagentStop WASM gates
(SS-04; `hooks-registry.toml` §SubagentStop section). Current behavior:
- `handoff-validator`: blocks results shorter than its threshold as
  `subagent_truncated_result`. A legitimate short review can be silently lost.
- `validate-pr-review-posted`: blocks dispatches with no PR in payload. A review agent
  dispatched outside PR context silently fails.

Fix mandate: `handoff-validator` MUST emit an advisory to the transcript on
`subagent_truncated_result` — the session must see the block reason, not just lose the
result. `validate-pr-review-posted` MUST surface its block reason in the transcript
(not just in the hook log) when `on_error=advisory`. Implementation route: S-22.02
(see §Follow-up Story Stubs).

## Rationale

The core tradeoff: honest description of same-family operation is better engineering
than false description of cross-family operation that doesn't exist. The Canonical
Principle forbids shipping the "cheap path" (amend docs and call it done) when the
correct path is achievable. The correct path here IS amending the docs, because:

1. The cross-family infrastructure (`agy` authenticated, Gemini API available) is not
   currently available in the operator environment. Wiring a hard dependency on
   unavailable infrastructure is a defect, not a feature.
2. The value of the review roles is real and should be preserved. Removing them
   because they're same-family would be worse than disclosing they're same-family.
3. The opt-in enhancement path (`VSDD_CROSS_FAMILY_DISPATCH=enabled`) provides a
   concrete, implementable upgrade when the operator authenticates `agy`. This is
   "feature ordering," not "feature incompleteness."
4. The retrospective disclosure (Decision 3) is honest about what past convergences
   mean without destroying the real engineering value they represent.

The alternative — "wire Gemini dispatch now" — is blocked by unauthenticated `agy`
and an ineligible `gemini` CLI. Wiring a mechanism that requires manual operator
authentication as a hard prerequisite is an infrastructure dependency decision, not
an architecture decision this ADR can resolve unilaterally.

## Consequences

### Positive

- Agent descriptions stop asserting a property they do not have.
- Model provenance is recorded in review artifacts; readers can verify which model
  produced a verdict.
- Cross-family dispatch is architecturally designed and implementable when `agy` is
  authenticated (`VSDD_CROSS_FAMILY_DISPATCH=enabled`).
- Historical convergences are annotated with honest disclosure rather than invalidated.
- SubagentStop gate silent-failure class is documented with a concrete implementation
  path (S-22.02).
- D-927 misdiagnosis is corrected, preventing an unnecessary destructive repin of 7
  opus-pinned agents.

### Negative

- Historical convergences carry a disclosure annotation that weakens their
  cross-family independence claim (but not their real engineering value).
- BC-5.39.001's convergence guarantee implicitly weakens from "3 CLEAN cross-family
  passes" to "3 CLEAN same-family fresh-context passes" until `agy` is authenticated.
- Operating `VSDD_CROSS_FAMILY_DISPATCH=enabled` requires a manual OAuth step (`agy`
  interactive startup) before each session. This is an operational friction item, not
  an architectural defect.

## Follow-up Story Stubs

Three story stubs registered by this ADR. Story-writer to author full bodies using
E-22 epic namespace. Do NOT author full story bodies here.

| Story ID | Title | Priority | Route |
|----------|-------|----------|-------|
| S-22.01 | Correct false model-family claims: honest same-family disclosure in adversary/spec-reviewer/code-reviewer agent definitions (per ADR-033 §Decision 1) | P0 | story-writer → implementer (agent file edits) |
| S-22.02 | Review artifact model-provenance frontmatter + SubagentStop advisory transcript surfacing (per ADR-033 §Decision 4 Class 3/4) | P0 | story-writer → implementer |
| S-22.03 | Optional cross-family dispatch via agy STDIN-pipe: `VSDD_CROSS_FAMILY_DISPATCH` env gate, authentication preflight, Gemini Flash invocation, fail-loud on auth failure (per ADR-033 §Decision 2) | P1 | devops-engineer + implementer; depends_on [S-22.01] |

S-22.03 is gated on operator authenticating `agy` (manual Google OAuth step) before
implementation can be validated. Do not dispatch S-22.03 until the authentication is
confirmed available in the session environment.

## Alternatives Considered

**Alternative A: Invalidate all prior convergences, re-run under genuine cross-family.**
Rejected. Retroactive invalidation of 60+ F5 passes and 21+ S-21.04 passes on the
basis of a documentation defect (not a behavioral defect) is disproportionate. The
engineering value of those passes is real. Only the human can authorize invalidation.

**Alternative B: Remove the diversity-review roles entirely.**
Rejected. Fresh context + information asymmetry + adversarial prompting strategy
provide real defect detection value independent of model family. Removing the roles
would reduce quality.

**Alternative C: Repin all 7 opus agents to `model: sonnet` (D-927 proposal).**
Rejected. D-927's premise ("7 opus-pinned agents die silently") is falsified by the
n=3 dispatch evidence. Repinning is unnecessary and destructive. The D-927 diagnosis
was based on a false conflation of Claude-mediated dispatch with model-family
unavailability.

**Alternative D: Require all operators to authenticate `agy` before running the
factory.**
Rejected. Making cross-family authentication a hard factory prerequisite blocks
operators without Google OAuth access. The opt-in `VSDD_CROSS_FAMILY_DISPATCH=enabled`
gate achieves cross-family dispatch when available without blocking the factory when
it is not.

## Source / Origin

- **Live dispatch evidence (2026-07-27, n=3):** `adversary`, `spec-reviewer`, and
  `code-reviewer` dispatched without model override; all resolved to Claude. Each
  read ARCH-INDEX.md / policies.yaml and returned version strings confirming Claude
  mediation, not Gemini or GPT.
- **Agent definition files:**
  - `plugins/vsdd-factory/agents/adversary.md` — description line: "Uses different
    model for genuine perspective diversity"
  - `plugins/vsdd-factory/agents/spec-reviewer.md` — body: "using Gemini — a different
    model family" / "GPT-5.4"
  - `plugins/vsdd-factory/agents/code-reviewer.md` — body: "a DIFFERENT model family
    than the Builder (Claude)"
- **spec-reviewer self-report (2026-07-27):** agent flagged its own gap unprompted:
  "The cognitive-diversity guarantee… is not being satisfied in this environment."
- **agy probe (2026-07-27):** `echo "ping" | agy --print --model gemini-2.5-flash`
  → `Error: authentication timed out.` (Google OAuth URL presented; not completed).
- **gemini CLI probe (2026-07-27):** `gemini` → `IneligibleTierError: This client
  is no longer supported for Gemini Code Assist for individuals.`
- **BC-5.39.001** (3-CLEAN convergence protocol) — the behavioral contract whose
  cross-family independence assumption this ADR corrects.
- **ADR-013** (adversarial review structure) — convergence criterion that inherits
  the cross-family assumption from agent descriptions.
- **D-927 (STATE.md):** recorded as "engine P0: 7 opus-pinned agents die silently"
  — this ADR establishes that the D-927 diagnosis is falsified by the n=3 dispatch
  table; the correct P0 is the false model-family claims, not model pin resolution.

### Status as of v1.0 (2026-07-27)

Accepted. Decisions 1–4 are architectural rulings effective immediately. Agent
description corrections (S-22.01) and review-artifact provenance (S-22.02) are P0
implementation stories pending story-writer authorship. Optional cross-family dispatch
(S-22.03) is a P1 story gated on operator `agy` authentication. The retrospective
disclosure annotation (Decision 3) applies to all active cascade INDEX.md records;
state-manager adds the disclosure block in the next appropriate burst.
