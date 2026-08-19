# Research Memo — ADR-039 §AMD-001 Fuel-vs-Epoch Technical Premise Validation

- **ID:** F-S2111-P13-001
- **Type:** general (technology / runtime-semantics validation)
- **Date:** 2026-08-18
- **Author:** research-agent (Perplexity `sonar-deep-research` + direct docs.rs verification)
- **Subject wasmtime version:** **46.0.2** (project pin — `Cargo.toml:81-82`)
- **Purpose:** Independent, source-backed second opinion on the technical premise behind ADR-039 §AMD-001, requested BEFORE authorizing a story split. Research-only — no spec/ADR/story files edited.

> **Scope note:** This memo validates the *technical runtime claims* (what fuel meters, what epochs meter, timeout-calibration soundness, self-lock avoidance). It does NOT adjudicate the ADR's story-split decision itself, which is an architect/orchestrator call.

---

## VERDICTS AT A GLANCE

| Q | Premise claim | Verdict |
|---|---------------|---------|
| **Q1** | Fuel bounds ONLY guest Wasm instruction execution; host-spawned subprocess is unmetered by fuel | **CONFIRMS** (verified against wasmtime 46.0.2 docs verbatim) |
| **Q2** | Epoch/wall-clock is the distinct, correct axis for bounding host-side blocking work; fuel and epochs are two different mechanisms | **CONFIRMS — with one precision caveat** (see below) |
| **Q3** | `timeout_ms >= max(p99×2.0, 30_000)` is a sound calibration for a subprocess timeout | **CONFIRMS as a defensible local policy — CHALLENGES its framing as an industry-standard formula** |
| **Q4** | A dispatch-path-guarding gate must not fail-closed on the wrong axis, to avoid self-lock | **CONFIRMS** (strong analogous precedent; no exact-phrase neutral standard) |

**BOTTOM LINE: The fuel-vs-epoch split is technically SOUND.** The core factual claim (Q1) is confirmed verbatim in the exact wasmtime version this project ships. The architectural reasoning (Q2, Q4) is well-supported. The only corrections are precision refinements, not premise reversals (Q2 mechanism nuance; Q3 "conventional heuristic" vs "industry-standard formula" framing).

---

## Q1 (CORE) — Does fuel bound only guest instruction execution, leaving host subprocesses unmetered?

**VERDICT: CONFIRMS the premise.**

**Decisive evidence — verified against the project's exact pin (wasmtime 46.0.2):**

The `Config::epoch_interruption` documentation, section "Interaction with blocking host calls", states verbatim:

> "Epochs (and fuel) do not assist in handling WebAssembly code blocked in a call to the host."

and gives the concrete example:

> "For example if the WebAssembly function calls `wasi:io/poll.poll` to sleep epochs will not assist in waking this up or timing it out."

Source: `https://docs.rs/wasmtime/46.0.2/wasmtime/struct.Config.html` (verified 2026-08-18, WebFetch of the pinned version, not "latest").

What fuel *does* meter, per `Store::set_fuel` on the same version:

> "Set the fuel to this Store for wasm to consume while executing." … "Most WebAssembly instructions consume 1 unit of fuel. Some instructions, such as `nop`, `drop`, `block`, and `loop`, consume 0 units, as any execution cost associated with them involves other instructions which do consume fuel."

Source: `https://docs.rs/wasmtime/46.0.2/wasmtime/struct.Store.html` (verified 2026-08-18). The docs make **no mention** of host functions or host-invoked code consuming fuel.

`Config::consume_fuel` frames fuel as instrumentation of *generated [guest] code*: Wasmtime "deterministically prevent[s] infinitely-executing WebAssembly code by instrumenting generated code to consume fuel as it executes." Source: same 46.0.2 Config page.

**Corroboration (independent, deep-research synthesis over docs.rs/latest = 47.0.3, Bytecode Alliance book, GitHub issues):** Fuel is a weighted **Wasm-operator** budget, not CPU cycles and not wall time. Time/CPU spent inside host-function bodies, blocking host calls, host syscalls, and external subprocess execution are explicitly outside automatic fuel accounting. A guest that repeatedly calls an import is charged fuel only for the surrounding/invoking guest operators; if the host callback blocks (or waits on a child process), the fuel counter does not force it to return. Sources: docs.rs/wasmtime `Config`/`Store` [47.0.3]; docs.wasmtime.dev book "Interrupting Wasm"; Wasmtime GitHub issue #8687 (maintainer Alex Crichton confirming async+epochs/fuel interrupt *currently-executing* Wasm only, not host blocking) `https://github.com/bytecodealliance/wasmtime/issues/8687`; issue #9188 "Ability for Epoch Cancelation to Cancel Imported WASI Functions" (open — confirms this is a known gap, not covered today) `https://github.com/bytecodealliance/wasmtime/issues/9188`.

**Application to the premise's `legacy-bash-adapter.wasm` case:** A bash subprocess spawned by the host (via a host function) is host-side OS work. Fuel cannot meter the child's execution and cannot terminate it. The premise's claim (1) — "a bash-adapter plugin's runaway/hang is invisible to the fuel counter" — is **correct and directly supported by the pinned-version documentation.**

**Caveat / precision:** The docs say fuel does not assist while the guest is *blocked in the host call*. The guest-side `call` operator that *initiates* the import is itself charged some fuel per the operator-cost schedule. So it is technically precise to say "fuel does not meter the subprocess's runtime," and imprecise to say "the fuel counter is entirely unaware the call happened." The premise's operative claim (runaway/hang invisibility) is unaffected by this nuance — the hang happens inside unmetered host time.

---

## Q2 — Epoch interruption vs fuel: intended purpose, and is epoch/wall-clock the right axis for a host subprocess?

**VERDICT: CONFIRMS the premise — with one precision caveat about what epochs alone can do.**

**Established distinction (docs.rs 46.0.2 + 47.0.3, Bytecode Alliance book):**

| Mechanism | Meters / checks | Intended for |
|-----------|-----------------|--------------|
| **Fuel** (`Config::consume_fuel` + `Store::set_fuel`) | Weighted count of executed **guest Wasm operators**; deterministic; interrupts at the same guest location for the same inputs | Deterministic guest-CPU budget / quota; reproducible interruption of infinite guest computation |
| **Epoch interruption** (`Config::epoch_interruption` + `Store::set_epoch_deadline` + `Engine::increment_epoch`) | Instrumented guest Wasm periodically compares an engine-global epoch counter to a per-store deadline; counter usually driven by a timer thread ⇒ approximates wall-clock; **non-deterministic** trap location; lower overhead than fuel (~10% per the book) | Coarse, low-overhead, wall-clock-ish interruption of **running guest Wasm**; async cooperative timeslicing |

Sources: `https://docs.rs/wasmtime/46.0.2/wasmtime/struct.Config.html`, `https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html` [47.0.3], docs.wasmtime.dev "Interrupting Wasm" / "Deterministic Wasm execution".

**Premise claim (2) — "epoch-based interruption and fuel are two distinct interruption mechanisms bounding different things":** **CONFIRMED.** They are documented as separate `Config` toggles with separate semantics (deterministic operator-count vs. non-deterministic wall-time-driven), and the docs explicitly recommend fuel for deterministic budgets and epochs for lightweight periodic interruption.

**CRITICAL PRECISION CAVEAT (a correction the ADR should absorb):**
Epoch interruption, *by itself*, is ALSO insufficient to bound a host-spawned subprocess. The same 46.0.2 doc sentence — "Epochs (and fuel) do not assist in handling WebAssembly code blocked in a call to the host" — puts epochs and fuel in the *same* insufficiency bucket for host-blocking work. Epoch interruption bounds *running guest Wasm*, not host time. So the correct axis for a host subprocess is **not "wasmtime epoch interruption"** but a **host-side wall-clock timeout / cancellation** that the *embedder (dispatcher)* enforces around the subprocess — e.g., a monotonic-clock deadline with SIGTERM→SIGKILL escalation, or `tokio::time::timeout` around the host operation with real cancellation/kill semantics.

If the ADR's `timeout_ms` is a **dispatcher-enforced wall-clock timeout on the bash subprocess** (host-side), the premise is fully correct and this caveat is merely terminological. If the ADR intends to use wasmtime's *epoch mechanism* to time out the subprocess, that would NOT work — epochs interrupt guest Wasm, not the child process. The premise's language ("epoch-based interruption and fuel are two distinct interruption mechanisms bounding different things") is accurate; but the phrase "the correct exhaustion axis … is a wall-clock/epoch timeout" should be read as **host-side wall-clock timeout**, with "epoch" used loosely as a synonym for wall-clock. Recommend the ADR state explicitly that the subprocess bound is a **dispatcher/host-enforced wall-clock timeout**, distinct from (and not implemented via) wasmtime `epoch_interruption`. Supporting: Wasmtime issue #8687 (host blocking → use poll-style timeouts / host-side deadline, not fuel/epochs); async WASI guidance (docs.wasmtime.dev "async") that a `tokio` timeout around the Wasm future plus explicit child-process kill is the embedder's responsibility.

**Net:** Premise's *direction* (fuel is the wrong axis; a wall-clock timeout is the right axis for the subprocess) — **CONFIRMED.** Precision fix — the right axis is a **host/dispatcher wall-clock timeout**, not wasmtime's epoch feature per se; epochs alone would not meter the child either.

---

## Q3 — Is `timeout_ms >= max(p99×2.0, 30_000)` a sound calibration?

**VERDICT: CONFIRMS it as a defensible, conservative local policy — CHALLENGES any framing of it as an industry-standard formula.**

**On the subprocess-bounding mechanism (the part the formula presupposes):** The wall-clock-timeout + SIGTERM→(grace)→SIGKILL escalation pattern is the conventional, well-attested way to bound a possibly-hanging child. GNU `timeout(1)` sends TERM by default and `--kill-after` escalates to KILL; systemd uses SIGTERM then SIGKILL after `TimeoutStopSec`; Kubernetes Jobs bound total wall-clock. Google SRE and AWS Well-Architected both counsel explicit deadlines rather than unbounded/very-long ones. Sources: `https://www.gnu.org/software/coreutils/manual/html_node/timeout-invocation.html`, `https://www.freedesktop.org/software/systemd/man/systemd.kill.html`, `https://kubernetes.io/docs/concepts/workloads/controllers/job/`, `https://sre.google/sre-book/addressing-cascading-failures/`, `https://docs.aws.amazon.com/wellarchitected/latest/reliability-pillar/rel_mitigate_interaction_failure_client_timeouts.html`.

**On the specific `p99×2.0` multiplier:**
- It is **defensible operational padding**, especially with noisy/limited latency samples. But it is **not** a documented SRE-standard formula. Google SRE explicitly calls deadline selection "something of an art" and prescribes *no* fixed multiplier. Amazon's Builders' Library derives timeouts from an acceptable *false-timeout rate* (their worked example starts at **p99.9**, not `2×p99`) rather than a multiplier. Source: `https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/`.
- The closest published multiplier match is AWS's **Agentic AI Lens**, which recommends **2–3× measured p95** for per-tool invocation timeouts — narrowly scoped to agent tool calls, not general subprocesses, but it lands in the same ballpark and lends the `2×`-class multiplier real (if scoped) authority. Source: `https://docs.aws.amazon.com/wellarchitected/latest/agentic-ai-lens/agentperf06-bp02.html`.
- **Correction to recommend to the ADR:** present `p99×2.0` as a *local calibration policy*, validated by its *observed false-timeout rate*, not as a citable industry standard. A more standard formulation is `timeout = max(floor, chosen_percentile + justified_margin)` capped by any parent/end-to-end budget (deadline propagation, per Google SRE). Also note the multiplier is applied over **p99**, which is already tail-heavy; `2×p99` is more conservative than the AWS `2–3×p95` guidance (p99 ≥ p95), so it errs toward *fewer* false timeouts at the cost of slower hang detection — an acceptable trade for a fail-closed dispatch gate.

**On the 30-second floor:**
- **Reasonable for a CI/lint-style validator, but not an established standard value.** A floor sensibly absorbs process startup, cold caches, and loaded CI workers, and it bounds a hang far tighter than platform job limits (GitHub Actions defaults jobs to 360 min). Sources: `https://docs.github.com/actions/using-workflows/workflow-syntax-for-github-actions`.
- **Caveat:** If a validator's real p99 is very low (e.g., 200 ms), a 30 s floor is ~150× p99 — quite permissive; Google SRE warns that deadlines orders of magnitude above normal latency can let obsolete work accumulate. That is acceptable for a *rare fail-closed* subprocess bound (you want to avoid false kills more than you want fast detection), but the ADR should acknowledge the floor is a "don't false-kill on a cold/slow CI box" cushion, not a latency-derived value. The frequently-cited "30 s" in Kubernetes is a *termination grace period*, not an execution floor — do **not** cite it as precedent that validators "should" run ≥30 s.

**Net:** The formula's *shape* (a percentile-derived value with a protective floor, biased toward not false-killing) is sound engineering. The specific constants are reasonable defaults but should be documented as **local policy validated by observed timeout rate**, not as an industry-standard formula.

---

## Q4 — Avoiding self-lock when a fail-closed gate guards its own dispatch path

**VERDICT: CONFIRMS the premise — treating dispatch-path-guarding gates specially is the standard safe posture.**

**Strongest precedent — Kubernetes admission webhooks** (the canonical real-world instance of "a fail-closed gate that can intercept the resources needed to run/repair itself"):
- Kubernetes explicitly documents **self-deadlock** and **dependency-loop** risks: a fail-closed admission webhook that intercepts the very resources needed to start itself (or its dependencies) can make the cluster unrecoverable. Mitigations: exclude the webhook's own namespace, use namespace/object selectors to exempt critical/dependent resources, prevent a webhook from operating on itself, and start with `failurePolicy: Ignore`. Source: `https://kubernetes.io/docs/concepts/cluster-administration/admission-webhooks-good-practices/`.
- GKE states directly that a fail-closed webhook can make automatic cluster recovery impossible (webhook needs a node to run, but fail-closed admission blocks adding the node) and recommends exemptions + a **break-glass** procedure (delete the `ValidatingWebhookConfiguration`). Source: `https://docs.cloud.google.com/kubernetes-engine/docs/how-to/optimize-webhooks`.
- OPA Gatekeeper documents the identical circular-dependency risk and an emergency bypass. Source: `https://open-policy-agent.github.io/gatekeeper/website/docs/failing-closed/`.

**Mapping to the premise:** The premise's concern — "making a gate that guards the agent-dispatch path itself fail-closed on the WRONG axis risks a hard self-lock" — is the direct analogue of the webhook self-deadlock. Treating the two dispatch-guarding gates specially (not fail-closing them on a mislabeled/incorrect axis) is exactly the "keep the repair/control path independent or narrowly exempt" posture the Kubernetes/GKE/Gatekeeper guidance prescribes. **The premise's safety posture is the industry-standard one.**

**Caveats / corrections:**
1. There is **no** single technology-neutral standard that uses the exact phrase "a fail-closed gate must exempt its own control/dispatch path." The support is strong-by-analogy from admission-control engineering, not a named ISO/NIST rule. Flagging honestly: this is convergent best-practice, not a citable universal spec.
2. "Special treatment" should mean **narrow, authenticated, audited exemption / break-glass**, not blanket fail-open. Kubernetes security guidance frames fail-closed as a threat-model-dependent trade — critical controls *may* fail closed, accepting availability risk. The correct design is: the dispatch-guarding gates should fail-closed on the *correct* axis (a real host wall-clock timeout) while never fail-closing on a *mislabeled* axis (fuel exhaustion that cannot actually apply to the subprocess), plus an out-of-band break-glass to remove the gate if it does wedge. Source: `https://kubernetes.io/blog/2022/01/19/secure-your-admission-controllers-and-webhooks/`.

The premise's specific framing — do not fail-close the dispatch-guarding gates on the *wrong (fuel)* axis — is well-aligned: it is not "make them fail-open," it is "do not brick dispatch on a metering axis that structurally cannot detect the failure mode in question." That is correct.

---

## Corrections & caveats to fold into ADR-039 §AMD-001 (advisory to architect — not applied here)

1. **(Q2, precision)** State explicitly that the subprocess bound is a **dispatcher/host-enforced wall-clock timeout**, not wasmtime `epoch_interruption`. Epochs, like fuel, do **not** bound host-blocking work (same doc sentence buckets them together). Using "epoch" as a synonym for "wall-clock" in the ADR text invites the misreading that wasmtime's epoch feature could time out the bash child — it cannot.
2. **(Q3, framing)** Present `max(p99×2.0, 30_000)` as a **local calibration policy validated by observed false-timeout rate**, not an industry-standard formula. Note the closest published analogue is AWS Agentic AI Lens `2–3×p95`; `2×p99` is more conservative (tail-biased toward avoiding false kills), which is appropriate for a fail-closed dispatch gate.
3. **(Q3, floor)** Document the 30 s floor as a "cold/loaded-CI cushion," not a latency-derived value; do not cite Kubernetes' 30 s grace period as precedent.
4. **(Q4, break-glass)** Pair the "special treatment" of dispatch-guarding gates with a documented, authenticated **break-glass** to remove/disable the gate out-of-band if it ever wedges dispatch — the standard companion to any fail-closed self-guarding control.

None of these reverse the premise. All four ADR claims survive independent scrutiny.

---

## BOTTOM-LINE RECOMMENDATION

**The fuel-vs-epoch split is technically SOUND and the premise is validated.** The load-bearing factual claim — wasmtime fuel meters only guest Wasm instruction execution and cannot meter or terminate a host-spawned bash subprocess — is confirmed **verbatim in the exact wasmtime version the project ships (46.0.2)**, not merely inferred. Bounding such a subprocess requires a host-side wall-clock timeout; fuel is structurally the wrong axis, and fail-closing a dispatch-guarding gate on that wrong axis is a genuine self-lock hazard with strong industry precedent (Kubernetes/GKE webhook self-deadlock).

The corrections are precision refinements only: (a) call the subprocess bound a *host wall-clock timeout*, since wasmtime *epochs* also cannot meter host-blocking work; (b) frame the `p99×2.0 / 30 s-floor` calibration as a validated local policy rather than an industry-standard formula; (c) add a break-glass companion to the special-treatment posture. **Recommendation: the technical premise is a green light for the story split** — subject to the architect folding in the four advisory corrections above (especially #1, the epoch-vs-host-wall-clock terminology fix, which is the only one with any correctness bite).

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity `perplexity_research` (PRIMARY)** | 2 | (1) Deep multi-source synthesis on wasmtime fuel vs epoch semantics and what fuel meters incl. host subprocesses (docs.rs, Bytecode Alliance book, GitHub issues #8687/#9188/#2274). (2) Deep synthesis on subprocess-timeout best practice, `p99×2` multiplier soundness, 30 s floor, and fail-closed self-deadlock avoidance (GNU coreutils, systemd, Google SRE, AWS Builders' Library, AWS Agentic AI Lens, Kubernetes/GKE/Gatekeeper webhook guidance). |
| Perplexity `perplexity_reason` | 0 | — |
| Perplexity `perplexity_search` | 0 | — |
| Perplexity `perplexity_ask` | 0 | — |
| Context7 | 1 (attempted, **unavailable**) | `resolve-library-id` for wasmtime returned "No such tool available: mcp__context7__resolve-library-id" — Context7 not mounted this session. Compensated by direct docs.rs version-pinned WebFetch (below). |
| WebFetch | 2 | Verified the decisive claims against the project's **exact** pin — `docs.rs/wasmtime/46.0.2` `struct.Config.html` ("Interaction with blocking host calls" verbatim) and `struct.Store.html` (`set_fuel` "most WebAssembly instructions consume 1 unit of fuel"; no host-call fuel accounting). |
| WebSearch | 0 | — |
| Grep (local) | 1 | Confirmed project wasmtime pin = 46.0.2 (`Cargo.toml:81-82`) and that only the dispatcher links wasmtime. |
| Training data | 0 areas | No premise-relevant claim rests on training data alone; every technical claim is tied to a version-pinned doc URL or a named authoritative source. |

**Total MCP tool calls:** 2 successful (`perplexity_research` ×2) + 1 attempted-unavailable (Context7). MCP gate satisfied.
**Training data reliance:** low — the load-bearing fuel/epoch claim is verified verbatim against the exact shipped wasmtime version (46.0.2), and every other claim carries a source URL.

### Context7-partial Escalation

Context7 was invoked (`resolve-library-id` for wasmtime) and returned `Error: No such tool available: mcp__context7__resolve-library-id` — the server is not mounted in this session. Per the research-agent protocol, the intended Context7 use (version-specific wasmtime API verification) was fully compensated by **direct version-pinned WebFetch of docs.rs/wasmtime/46.0.2**, which is arguably a stronger source than Context7 for this purpose (it is the canonical rendered rustdoc for the exact pinned version). The primary MCP requirement is independently satisfied by the two `perplexity_research` calls. No toolchain-repair routing is required for the correctness of this memo, but the orchestrator may wish to note Context7 unavailability for future library-doc lookups.
