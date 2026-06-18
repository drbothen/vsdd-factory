# Research — Wave-Ordinal Derivation Soundness (O-P8-001, S-18.01)

**Date:** 2026-06-18
**Type:** general (technology / algorithm validation)
**Question:** Is "count the leading contiguous fully-terminal run of waves" a sound model for deriving "current wave" from a dependency-scheduled work queue, or is it brittle vs. a DAG-level-based derivation?
**Disposition target:** adversary observation O-P8-001 on S-18.01.

---

## VERDICT

**CONTEXT-DEPENDENT — sound for VSDD's specific phase-gated execution model, but brittle as a general algorithm. The current implementation is acceptable IF AND ONLY IF the barrier invariant is documented as a hard precondition and the file-ordering assumption is validated.**

In precise terms:

- As a **general-purpose** "current wave" function over an arbitrary dependency-scheduled queue, the leading-contiguous-terminal-run model is **BRITTLE / incorrect**. It silently assumes (a) file order == wave order, and (b) a *barrier invariant* — that no wave-(k+1) story reaches a non-pending state until all of wave-k is terminal. Generic DAG schedulers violate (b) by design.
- As an implementation **inside VSDD specifically**, the model is **SOUND**, because VSDD's wave-integration protocol *enforces exactly that barrier invariant* (see "Project-Specific Finding" below). The contiguous-run count and the canonically-correct min-level metric provably coincide when the barrier holds.

The correct disposition is therefore **not** "no-op note-as-intended" and **not** a full DAG-rewrite. It is: **note-in-spec-as-intended, but with the barrier invariant + file-ordering stated as explicit, enforced preconditions, plus a defensive guard.** See the orchestrator recommendation paragraph.

---

## Two competing definitions

Given a wave-layered DAG where each story `v` has a topological level `L(v)` (Kahn / longest-path layering) and a status `S(v)` that is terminal `{merged, withdrawn, cancelled}` or non-terminal `{pending, draft, in-flight}`, there are two distinct, non-equivalent notions of "current wave":

| Metric | Definition | Question it answers |
|--------|-----------|---------------------|
| **(A) Earliest-incomplete-wave (min-level)** | `current_wave = min{ L(v) : S(v) non-terminal }` (or "none" if all terminal) | *Where does the earliest outstanding work live?* |
| **(B) Contiguous-run of completed waves** | largest `k` such that **every** story in waves `1..k` is terminal; current = `k + 1` | *How many waves have been entirely cleared?* |

The current implementation is metric (B) computed positionally (scan file order, count terminal entries until the first non-terminal). Sonar Deep Research (sonar-deep-research) confirms: **the literature has no single canonical "current wave" algorithm**; these two metrics answer *different questions* and coincide only under a barrier invariant.

---

## Findings against the three sub-questions

### Q1 — Can stories from different waves legitimately be in mixed terminal/non-terminal states simultaneously?

**Yes — routinely, in generic dependency-topological execution.** This is the default behavior of every modern DAG scheduler surveyed:

- **GitLab CI `needs:`** — jobs "start as soon as their dependencies finish **without waiting for earlier jobs or stages to finish**." A later-stage job can complete while an earlier-stage job is still running/pending. [GitLab docs]
- **GitHub Actions `needs:`** — a job waits only for its named dependency jobs, not for global stage/level membership; transforms parallel runners into DAG orchestration. [GitHub Actions docs]
- **Ninja / Bazel** — "builds are always run in parallel … scheduling tasks as soon as they become ready." Eager, local-dependency-only scheduling is the explicit design goal. [Ninja manual; Bazel]
- **Apache Airflow** — tasks become eligible the moment their upstream tasks succeed; no global level barrier. [Airflow]
- **Kahn's algorithm / CPM theory** — topological levels are an *analytical layering* of the DAG, not an execution barrier. Eager scheduling blocks a node only on its *direct* predecessors, so a high-level node whose few ancestors finished early can complete before a low-level node on the critical path.

So under eager/local semantics, a wave-2 story being `merged` while a wave-1 story is still `pending` is **legal and expected**, not an anomaly. Under those semantics, metric (B) computed positionally returns `current_wave = 1` while real work spans waves 1–3 — a misleading under-report.

### Q2 — What is the canonical/robust way to compute "current wave"?

The semantically faithful, invariant-free definition is **metric (A): the minimum topological level among non-terminal stories.**

```
levels = kahn_longest_path_levels(dependency_dag)   # L(v) per story
non_terminal = [ v for v in stories if status(v) not in TERMINAL ]
current_wave = min(levels[v] for v in non_terminal) if non_terminal else DONE
```

This holds under *any* scheduling policy (eager or gated), does not assume file ordering, and correctly ignores the fact that some later-wave stories may already be done or in-flight. The positional contiguous-run (metric B) is a *special case* that equals (A) only when the barrier invariant holds AND file order == wave order.

### Q3 — Does correctness of the contiguous-run model depend on a "complete-before-start" invariant? Is that realistic?

**Yes, it depends on a barrier invariant**, formalized as: *no story in wave (k+1) leaves the `pending` state until every story in wave k is terminal.* Under that invariant, the leading terminal prefix exactly equals the set of fully-cleared waves and (A) == (B).

**Realism is bimodal:**

- **Violated by design** in generic DAG execution engines (build systems, DAG-capable CI, workflow engines, parallel schedulers) — they deliberately break it to maximize concurrency.
- **Enforced by design** in *deliberately phase-gated* schemes: classic non-`needs` CI stages, rolling-wave / increment planning with stage gates, and governance phase-gate processes ("no implementation until design signed off"). In these, (B) is a correct and even preferable progress measure.

So the invariant is realistic *only* when the process explicitly gates waves. It is **not** a safe default assumption for an arbitrary "dependency-scheduled work queue."

---

## Project-Specific Finding (decisive for O-P8-001)

VSDD does **not** use generic eager DAG execution at the wave level. Its wave-integration protocol *enforces the barrier invariant explicitly*:

- `plugins/vsdd-factory/rules/worktree-protocol.md` (Wave Integration): "After **all** stories in a wave are merged to `develop` … 4. Wave gate passes → next wave begins."
- `plugins/vsdd-factory/docs/FACTORY.md` (Wave Integration Gate): "Wave gate passes before next wave starts."
- Enforced operationally by the hook plugins `validate-wave-gate-completeness`, `validate-wave-gate-prerequisite`, and `warn-pending-wave-gate` (hooks-registry.toml), and by BC-4.10.001 ("blocks wave-gate dispatch when any story lacks convergence clearance").

This is precisely the barrier invariant under which metric (A) == metric (B). **Therefore the leading-contiguous-terminal-run model is sound *for VSDD's actual execution semantics*.**

Two residual fragilities remain even under the invariant, and they are the real substance of O-P8-001:

1. **File-ordering assumption.** Metric (B)-positional additionally assumes the sprint-state file is *physically ordered* by wave. If a wave-2 story is listed before a leftover wave-1 story (re-ordering, manual edit, append-on-create), the leading-run scan terminates early or counts across wave boundaries. The DAG-level metric (A) is immune; the positional implementation is not. This is a latent correctness bug independent of the barrier invariant.
2. **Silent precondition.** The implementation encodes the barrier invariant and the file-ordering assumption *implicitly*. If a future story relaxes wave gating (e.g., eager intra-wave start, the "decouple delivery where possible" agile guidance), the derivation silently degrades with no failing test to catch it.

---

## Recommended robust algorithm (if/when hardening is chosen)

Keep the cheap positional path for the common case, but make the preconditions load-bearing rather than implicit:

1. **Compute waves from the DAG, not from file position.** Derive `L(v)` via Kahn longest-path layering from the in-scope dependency edges; do not trust file order. Then `current_wave = min{ L(v) : S(v) non-terminal }`. This is O(V+E), trivially cheap at story-catalog scale, and is correct under *both* gated and eager policies — future-proofing against any later relaxation of wave gating.
2. **If retaining the contiguous-run implementation**, add a **barrier-invariant assertion** as a defensive guard: after computing the leading terminal prefix length `k`, verify there is no terminal story at level `> k+1` whose presence would indicate cross-wave completion (a barrier breach). On breach, fail loud (error) rather than returning a wrong ordinal — converting a silent miscount into a detectable invariant violation. Also assert file order == sorted wave order, or sort defensively before scanning.

Either path closes the latent bug; path (1) is the production-grade default because it removes the file-ordering assumption entirely.

---

## Orchestrator recommendation for O-P8-001 disposition

**Disposition: `note-in-spec-as-intended` — with mandatory hardening in current scope (NOT a bare note, NOT a full deferral).**

The contiguous-run model is *correct* under VSDD's enforced wave-gate barrier invariant (`worktree-protocol.md` / `FACTORY.md` / `validate-wave-gate-*` hooks / BC-4.10.001), so a wholesale DAG-level rewrite is not required to fix a present defect. **However**, per the Canonical Principle (production-grade default, no silent preconditions), the disposition cannot be a bare "note as intended": the spec must (a) state the barrier invariant and file-order-equals-wave-order assumptions as **explicit, named preconditions** of the derivation, cross-linking the enforcing artifacts; and (b) the implementation must add a **defensive guard** that fails loud on a barrier-invariant breach or out-of-order file rather than returning a wrong ordinal (the file-ordering fragility is a latent correctness bug *independent* of the gate and is in-scope to fix now). Deferring the full Kahn-level derivation (robust algorithm path 1) to a future story is legitimate *only* if attached to a concrete story ID with the precondition documented — but the guard and the precondition documentation are cheap and belong in S-18.01's scope. Net: **document-as-intended + add invariant guard + assert file ordering, all in current scope; defer only the optional DAG-level rewrite, and only with an explicit story anchor.**

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 | Deep multi-source synthesis (reasoning_effort=high) on canonical current-wave derivation, eager vs. gated DAG execution semantics, barrier-invariant realism across build systems / CI / workflow engines / Kahn-CPM theory |
| Grep (local repo) | 3 | Verified VSDD's own wave-execution semantics (barrier-invariant enforcement) against worktree-protocol.md, FACTORY.md, hooks-registry.toml |
| Read (local repo) | 1 | Confirmed verbatim wave-integration gate wording |

**Total MCP tool calls:** 1 (`perplexity_research`, sonar-deep-research, high effort)
**Training data reliance:** low — every external claim (GitLab `needs`, GitHub Actions `needs`, Ninja/Bazel eager scheduling, Airflow, Kahn layering, CPM, rolling-wave planning, absence of a canonical "current-wave" algorithm) is grounded in the Perplexity deep-research synthesis with cited primary docs; the decisive project-specific finding is grounded in direct reads of this repo's own artifacts. Kahn's algorithm and CPM level-assignment are textbook results used as model knowledge for framing only.

### Source notes / caveats
- The deep-research model was explicit that **no standard named algorithm for "current wave" exists** in the literature; its recommendation is a synthesis of well-documented DAG/scheduling theory plus real-system behavior, not a single citable standard. Flagged as such rather than overstated.
- It also surfaced a "wave-execution-planner" package reference but could not firmly verify it as an authoritative source — treated as inconclusive and **not** relied upon.
- Primary docs cited by the synthesis (GitLab `needs`, GitHub Actions `needs`, Ninja parallel scheduling) are standard, stable, and consistent with two-or-more independent descriptions; confidence HIGH.
