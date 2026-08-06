---
title: WASM sandbox resource-exhaustion detection — Wasmtime fuel semantics, fail-open classification, and production-host precedent
date: 2026-08-06
researcher: research-agent
status: complete
question: >
  Our validation plugins run as WASM modules in a Wasmtime sandbox with a finite fuel
  budget and `on_error = "continue"`. An adversarial reviewer found that fuel exhaustion
  converts to a "continue / no violation found" result claimed to be indistinguishable
  from a clean successful validation. Can a host reliably distinguish fuel exhaustion from
  clean completion? Is fail-open-on-resource-exhaustion a recognized anti-pattern (CWE)?
  What do comparable production WASM plugin hosts do? What is the recommended detection
  mechanism and fuel-budgeting strategy?
subject_version: wasmtime 44.0.1 (pinned in Cargo.toml:79-80, Cargo.lock)
verdict: >
  Finding is DIRECTIONALLY CORRECT but its stated mechanism is wrong. Fuel exhaustion IS
  already structurally distinguished in this codebase and IS emitted as a distinct
  telemetry event. The real defect is narrower and still HIGH: in the *enforcement*
  channel, `Timeout + on_error=Continue` yields the same allow-decision as a clean pass.
  That is CWE-636 fail-open, not CWE-778 insufficient logging.
sources:
  - https://docs.rs/wasmtime/44.0.1/wasmtime/enum.Trap.html
  - https://docs.rs/wasmtime/44.0.1/wasmtime/struct.Store.html
  - https://docs.rs/wasmtime/44.0.1/wasmtime/struct.Config.html
  - https://docs.wasmtime.dev/examples-interrupting-wasm.html
  - https://github.com/bytecodealliance/wasmtime/blob/main/examples/fuel.rs
  - https://docs.wasmtime.dev/c-api/store_8h.html
  - https://cwe.mitre.org/data/definitions/636.html
  - https://cwe.mitre.org/data/definitions/390.html
  - https://cwe.mitre.org/data/definitions/755.html
  - https://cwe.mitre.org/data/definitions/703.html
  - https://cwe.mitre.org/data/definitions/778.html
  - https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/wasm/v3/wasm.proto
  - https://istio.io/latest/docs/reference/config/proxy_extensions/wasm-plugin/
  - https://istio.io/latest/docs/concepts/extensibility/
  - https://github.com/proxy-wasm/spec/blob/main/docs/WebAssembly-in-Envoy.md
  - https://www.fastly.com/documentation/reference/compute/errors/
  - https://docs.rs/extism/latest/extism/struct.Plugin.html
  - https://github.com/extism/php-sdk
  - https://github.com/envoyproxy/envoy/issues/38801
unconfirmed:
  - Exact Wasmtime release that replaced `add_fuel`/`fuel_consumed` with `set_fuel`/`get_fuel`.
    Not load-bearing — the pinned 44.0.1 API was verified directly. See §1.4.
---

# WASM Fuel Exhaustion Detection

## Executive summary

| Question | Answer |
|---|---|
| Can a host reliably distinguish fuel exhaustion from clean completion? | **Yes**, unambiguously. Downcast the `anyhow::Error` from `Func::call` to `wasmtime::Trap` and match `Trap::OutOfFuel`. |
| Does *this* codebase already do that? | **Yes** — `crates/factory-dispatcher/src/invoke.rs:425-445` and `resolver_classify_trap.rs:56`. The reviewer's premise that the two are "indistinguishable" is incorrect as stated. |
| Is fail-open-on-exhaustion a recognized anti-pattern? | **Yes** — primarily **CWE-636 "Not Failing Securely ('Failing Open')"**, with **CWE-390** for the detected-but-unactioned error, under pillar **CWE-703** via **CWE-755**. |
| What do production hosts default to? | **Fail-closed.** Envoy proxy-wasm `FailurePolicy` default is `FAIL_CLOSED`; Istio `failStrategy` default is `FAIL_CLOSE` and explicitly warns against `FAIL_OPEN` for authorization-class plugins. |
| Do they surface exhaustion distinctly? | **Yes.** Fastly Compute emits a dedicated per-cause metric (`compute_service_timeout_error`) plus a categorical `compute_service_resource_limits_error`. |
| Recommended remediation | Keep fuel + epoch both enabled (already the case). Reclassify resource-exhaustion as its own outcome distinct from both `Ok` and generic `Timeout`, and make it fail-closed **independently of `on_error`** for validators. Size fuel from measured p99 × 1.5–2.0, not a fixed constant. |

---

## 1. Wasmtime fuel semantics (verified against 44.0.1)

### 1.1 The exhaustion signal is a distinct, matchable trap

`wasmtime::Trap` in 44.0.1 has 46 variants. Two are relevant:

- **`Trap::OutOfFuel`** — *"When wasm code is configured to consume fuel and it runs out of fuel then this trap will be raised."*
- **`Trap::Interrupt`** — *"Execution has potentially run too long and may be interrupted."* (this is the epoch-deadline trap)

Source: <https://docs.rs/wasmtime/44.0.1/wasmtime/enum.Trap.html>

The canonical detection idiom, from Wasmtime's own `examples/fuel.rs`:

```rust
let output = match fibonacci.call(&mut store, n) {
    Ok(v) => v,
    Err(e) => {
        assert_eq!(e.downcast::<Trap>()?, Trap::OutOfFuel);
        println!("Exhausted fuel computing fib({n})");
        break;
    }
};
```

Source: <https://github.com/bytecodealliance/wasmtime/blob/main/examples/fuel.rs>, mirrored in the book at <https://docs.wasmtime.dev/examples-interrupting-wasm.html>

**This answers the core question: fuel exhaustion is fully distinguishable from clean completion.** A clean run returns `Ok(_)`; an exhausted run returns `Err(e)` where `e.downcast_ref::<Trap>() == Some(&Trap::OutOfFuel)`. There is no ambiguity at the runtime boundary. Any indistinguishability is introduced by host-side classification collapsing the two, not by Wasmtime.

### 1.2 Fuel accounting API in 44.0.1

Verified signatures and doc text from <https://docs.rs/wasmtime/44.0.1/wasmtime/struct.Store.html>:

| Method | Signature | Notes |
|---|---|---|
| `Store::set_fuel` | `pub fn set_fuel(&mut self, fuel: u64) -> Result<()>` | *"By default a `Store` starts with 0 fuel for wasm to execute with (meaning it will immediately trap)."* Errors if `Config::consume_fuel` not enabled. |
| `Store::get_fuel` | `pub fn get_fuel(&self) -> Result<u64>` | Returns fuel **remaining**, not consumed. Errors if fuel not enabled. |
| `Store::fuel_async_yield_interval` | `pub fn fuel_async_yield_interval(&mut self, interval: Option<u64>) -> Result<()>` | Yields control back to the caller every `interval` units. Errors if fuel disabled or `interval == Some(0)`. |
| `Store::consume_fuel` | manual decrement | Still present; lets the host charge fuel for host-side work (e.g. accounting I/O as equivalent CPU). Not the exhaustion-detection path. |

Cost model, quoted verbatim: *"Most WebAssembly instructions consume 1 unit of fuel. Some instructions, such as `nop`, `drop`, `block`, and `loop`, consume 0 units, as any execution cost associated with them involves other instructions which do consume fuel. Note that when fuel is entirely consumed it will cause wasm to trap."*

Consumed fuel is derived, not read directly:

```rust
let fuel_consumed = fuel_before - store.get_fuel().unwrap();
```

This matches the local helper `fuel_consumed_from_store` at `invoke.rs:457`, which computes `cap.saturating_sub(remaining)`.

### 1.3 `ResourceLimiter` does *not* cover fuel

`Store::limiter` / `StoreLimits` govern **memory growth, table growth, and instance creation** only — the callback is consulted on every growth attempt and can reject it, producing a clean `MemoryGrowError` rather than host starvation. It has no fuel hook. So `ResourceLimiter` is **not** an available mechanism for observing fuel exhaustion; that is exclusively the trap path.

Sources: <https://docs.wasmtime.dev/api/src/wasmtime/runtime/limits.rs.html>, <https://www.systemshardening.com/articles/wasm/wasmtime-production-hardening/>

### 1.4 Version churn — explicitly unconfirmed

The brief correctly flagged this API as churn-prone. What I can state with confidence:

- **Confirmed** for the pinned version (44.0.1): the API is `set_fuel` / `get_fuel` / `consume_fuel` / `fuel_async_yield_interval`, verified directly against docs.rs for 44.0.1.
- **Confirmed** that an older generation of the API used `add_fuel`, `fuel_consumed() -> (u64, bool)`, and `consume_fuel() -> Result<u64>` returning remaining fuel — still visible in the frozen `wasmtime-go` v12-era docs (<https://pkg.go.dev/github.com/bytecodealliance/wasmtime-go>) and `wasmtime-dotnet` (<https://bytecodealliance.github.io/wasmtime-dotnet/api/Wasmtime.Caller.html>).
- **Could not confirm** the exact release that performed the swap. `RELEASES.md` on `main` only carries the unreleased version; per-version notes live on release branches, and the `release-22.0.0` branch notes contain no fuel entry. Circumstantial evidence points to mid-2024 (Wasmtime ~22–23): the Wasmex binding's changelog for 0.9.0 (2024-07-25) states *"Wasmtime rewrote their fuel-related API and simplified it… the existing methods on `Store` (`consume_fuel`, `fuel_remaining`, `add_fuel`) were removed. Please call `set_fuel/2` and `get_fuel/1` instead"* (<https://github.com/tessi/wasmex/blob/main/CHANGELOG.md>), and wasmtime-dotnet updated to 22.0.0 in July 2024.

I am **not** asserting a specific version. This is not load-bearing for the finding: the project pins 44.0.1 and that surface was verified directly.

### 1.5 Fuel vs epoch interruption — documented tradeoffs

Both mechanisms can be configured to either **trap** (non-resumable) or **async-yield** (host decides whether to resume).

| | **Fuel** (`Config::consume_fuel` + `Store::set_fuel`) | **Epoch** (`Config::epoch_interruption` + `Engine::increment_epoch`) |
|---|---|---|
| Trap on exhaustion | `Trap::OutOfFuel` | `Trap::Interrupt` |
| Determinism | **Deterministic.** *"the same program run with the same amount of fuel will always be interrupted at the same location in the program"* | **Non-deterministic** — wall-time driven; same input may trap at different points across runs |
| Overhead | Higher — per-instruction accounting | Lower — *"around a 10% slowdown"*; epoch is *"up to 2-3x"* faster than fuel in some measurements |
| Granularity | Per-instruction | Coarse — next safe point after deadline |
| Maps to wall clock? | No. Fuel units do not translate cleanly to time. | Yes, directly. |
| Wasmtime's stated use case | *"Fuel… should be used when **deterministic** yielding or trapping is needed."* | Cooperative timeslicing / wall-clock deadlines in async execution |

Sources: <https://docs.wasmtime.dev/examples-interrupting-wasm.html>, <https://docs.rs/wasmtime/44.0.1/wasmtime/struct.Config.html>

**Are we using the wrong primitive?** No — and notably, this codebase already uses **both**, which is the recommended production posture. `resolver_loader.rs:590-591` documents exactly this pairing ("Fuel enforcement: set a generous fuel budget; timeout via epoch interruption"). Independent hardening guidance agrees: *"Enable both in production multi-tenant platforms: fuel for the accounting record, epoch for the hard deadline"* (<https://www.systemshardening.com/articles/wasm/wasm-fuel-metering/>).

The primitive choice is sound. The defect is in **classification and enforcement**, not in the metering mechanism.

---

## 2. Is fail-open-on-resource-exhaustion a recognized anti-pattern?

Yes. The applicable CWE entries, in decreasing order of fit:

### CWE-636 — Not Failing Securely ("Failing Open") — **primary**

> *"When the product encounters an error condition or failure, its design requires it to fall back to a state that is less secure than other options that are available, such as selecting the weakest encryption algorithm or using the most permissive access control restrictions."*

Extended description: *"By entering a less secure state, the product inherits the weaknesses associated with that state, making it easier to compromise. At the least, it causes administrators to have a false sense of security. This weakness typically occurs as a result of wanting to 'fail functional' to minimize administration and support costs, instead of 'failing safe.'"*

Alternate term: **"Failing Open."** Source: <https://cwe.mitre.org/data/definitions/636.html>

This is the exact shape of the finding: a validator that cannot complete its check falls back to the most permissive outcome (allow the write). The "false sense of security" clause is precisely the harm — an operator reading a green dispatch believes the artifact was validated.

### CWE-390 — Detection of Error Condition Without Action — **secondary, strong fit**

> *"The product detects a specific error, but takes no actions to handle the error."*

Consequence: *"An attacker could utilize an ignored error condition to place the system in an unexpected state that could lead to execution of unintended logic and unintended behavior."*

Mitigation: *"If a function returns an error, it is important to either fix the problem and try again, alert the user that an error has happened and let the program continue, or alert the user and close and cleanup the program."*

Source: <https://cwe.mitre.org/data/definitions/390.html>

CWE-390 fits well **because** the code already detects the condition (`Trap::OutOfFuel` → `TimeoutCause::Fuel`) and then takes no gating action when `on_error=Continue`. Detection without action is the literal title.

### CWE-755 / CWE-703 — parent classes

CWE-755 "Improper Handling of Exceptional Conditions" (*"The product does not handle or incorrectly handles an exceptional condition"*) is the class-level parent, itself a child of pillar **CWE-703** "Improper Check or Handling of Exceptional Conditions." CWE-390 is a child of CWE-755. Sources: <https://cwe.mitre.org/data/definitions/755.html>, <https://cwe.mitre.org/data/definitions/703.html>

### CWE-778 — Insufficient Logging — **does NOT apply here**

> *"When a security-critical event occurs, the product either does not record the event or omits important details about the event when logging it."*

Source: <https://cwe.mitre.org/data/definitions/778.html>

**I am explicitly ruling this out.** The dispatcher *does* record the event: `executor.rs:683-697` emits a `PLUGIN_TIMEOUT` internal event carrying `cause="fuel"`, `elapsed_ms`, and `fuel_consumed`. Citing CWE-778 in the finding would be inaccurate and would send remediation toward adding logging that already exists.

### Related-but-distinct

CWE-1088 and the "fail-safe defaults" principle (Saltzer & Schroeder) are the design-principle framing. OWASP's canonical statement is the **"Fail securely"** / fail-safe-defaults principle: error and exception paths should deny by default. Note the corroborating operational framing in the Envoy ecosystem, which describes the tradeoff exactly as *"prioritizing availability over strict security"* when choosing fail-open for an authorization filter.

---

## 3. What comparable production WASM plugin hosts do

This is the strongest part of the evidence base — the precedent is consistent and it favors fail-closed.

### Envoy / proxy-wasm — default **FAIL_CLOSED**

From the Envoy `wasm.proto` API reference, `enum extensions.wasm.v3.FailurePolicy`:

| Value | Behavior |
|---|---|
| `UNSPECIFIED` | *"(DEFAULT) No policy is specified. The default policy will be used. **The default policy is `FAIL_CLOSED`.**"* |
| `FAIL_CLOSED` | *"All plugins associated with the VM will return an HTTP 503 error."* |
| `FAIL_OPEN` | *"All plugins associated with the VM will be ignored and the filter chain will continue. **This makes sense when the plugin is optional.**"* |
| `FAIL_RELOAD` | New plugin instance created for the new request; *"only applied to… `proxy_wasm::FailState::RuntimeError`. This will fallback to the `FAIL_CLOSED` for all other failures."* |

The older boolean `fail_open` field is *"deprecated in favor of the `failure_policy` field"* — i.e. Envoy moved from a binary open/closed toggle to a richer policy enum, and kept fail-closed as the default throughout.

Source: <https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/wasm/v3/wasm.proto>

The proxy-wasm spec confirms CPU limits are in scope as a first-class constraint: *"Each configured Proxy-Wasm extension can set resource constraints (maximum memory each VM can allocate, and **maximum CPU time it can consume during each invocation**) in order to limit resource usage."* Source: <https://github.com/proxy-wasm/spec/blob/main/docs/WebAssembly-in-Envoy.md>

### Istio — default **FAIL_CLOSE**, with an explicit warning that maps onto our case

From the `WasmPlugin` reference, `FailStrategy`:

| Value | Behavior |
|---|---|
| `FAIL_CLOSE` | *"A fatal error in the binary fetching or during the plugin execution causes all subsequent requests to fail with 5xx."* |
| `FAIL_OPEN` | *"Enables the fail open behavior… to bypass the plugin execution. A fatal error can be a failure to fetch the remote binary, an exception, or abort() on the VM. **This flag is not recommended for the authentication or the authorization plugins.**"* |

Source: <https://istio.io/latest/docs/reference/config/proxy_extensions/wasm-plugin/>

Istio's extensibility comparison table lists WebAssembly's failure policy as *"Configurable — fail-closed by default"*, contrasted against Lua's *"Fail-open only — no configuration option."* Source: <https://istio.io/latest/docs/concepts/extensibility/>

**This is the most directly applicable precedent.** Our validation plugins are authorization-class controls: they decide whether a mutation to `.factory/` is permitted. Istio names exactly this category as the one where fail-open is not recommended.

### Fastly Compute — terminate, deny, and emit a **dedicated per-cause metric**

> *"These fatal errors occur when a Compute sandbox reaches a maximum allowed resource threshold, specifically runtime duration, vCPU usage, or heap memory limits. Reaching these limits terminates the sandbox immediately, typically returning an `HTTP 500` or `503` to the client. **Each specific error event type has its own dedicated metric**, and all of these events contribute to the overall categorical metric `compute_service_resource_limits_error`."*

For sandbox timeout specifically: increments both `compute_service_resource_limits_error` and `compute_service_timeout_error`, and returns HTTP 503 with structured diagnostic metadata `{"diagnostic": "instance_limit", "limit": "timeout"}`.

Source: <https://www.fastly.com/documentation/reference/compute/errors/>

Two lessons: (a) exhaustion is a **denial**, not a bypass; (b) exhaustion is both **categorically** and **specifically** observable — a general resource-limit counter plus a per-cause counter. That two-level scheme is a good model for our telemetry.

### Extism — exhaustion raises, and consumed fuel is queryable

Extism (which itself embeds Wasmtime — upgraded to v43 in May 2026) exposes fuel as a first-class plugin option: *"Plugins can be initialized with a fuel limit to constrain their execution. When a plugin runs out of fuel, it will throw an exception."* The Rust SDK exposes `Plugin::fuel_consumed() -> Option<u64>`, *"the difference between the initial fuel and the remaining fuel."*

Sources: <https://github.com/extism/php-sdk>, <https://docs.rs/extism/latest/extism/struct.Plugin.html>, <https://github.com/extism/extism>

Exhaustion is an **exception** — an error the caller must handle — never a silent empty result.

### Caveat on the precedent

Envoy's fail-closed path has had real bugs: issue #38801 (2025-03, Envoy 1.33) reports `failure_policy: FAIL_CLOSED` causing a request **hang** instead of the documented 503, and `FAIL_OPEN` hanging for the first N requests (N ≈ worker-thread count) before bypassing correctly. Source: <https://github.com/envoyproxy/envoy/issues/38801>. Relevant lesson for us: whichever policy we adopt needs a **test that actually exercises it**, because documented-intent and observed-behavior diverged for a major proxy.

---

## 4. Detecting the condition — recommended mechanism

Ranked, with the local state of play noted for each.

1. **Structural trap downcast (primary).** `err.downcast_ref::<wasmtime::Trap>()` and match `Trap::OutOfFuel` / `Trap::Interrupt`. **Already implemented** at `invoke.rs:425-445`. This is correct and needs no change.

2. **Distinct outcome variant in the host's result type (the actual gap).** Fuel exhaustion currently collapses into `PluginResult::Timeout { cause: TimeoutCause::Fuel }`, and `Timeout` is then subject to the same `on_error` policy as any other failure. Per the Fastly model, resource exhaustion deserves its own categorical treatment because its security meaning differs from a slow plugin: *the validation did not complete, therefore nothing was verified*.

3. **Fuel-remaining sanity check on the success path (cheap, high-value).** Even on `Ok(_)`, compare `store.get_fuel()` against the cap. A plugin that returns success having consumed >90% of budget is a near-miss and a leading indicator that the next slightly-larger input will trap. This is the single cheapest addition and directly addresses the fixture-scale blind spot: it would have fired long before a 574 KB input started failing.

4. **Epoch interruption for the wall-clock bound.** Already enabled alongside fuel. Keep both; do not replace fuel with epoch — fuel's determinism is what makes validator behavior reproducible across runs, which matters for a gate whose verdicts get attested.

5. **`ResourceLimiter` — not applicable to fuel** (§1.3). Useful for memory/table growth only.

6. **Pre-flight input-size budgeting (defense in depth).** The codebase already does this in at least one place: `validate-factory-path-staging/src/lib.rs:578-579` rejects payloads over a 64 KiB `MAX_COMMAND_LEN` with the comment *"SEC-003 / WASM fuel bound: reject oversized payloads before any parsing."* Rejecting oversized input explicitly is strictly better than letting it trap mid-scan — it converts an ambiguous partial validation into a clear, actionable refusal. Note this pattern is not applied uniformly across the 52 plugins.

### Telemetry shape

Recommended counter set, adapted from published Wasmtime hardening guidance:

```
wasmtime_traps_total{plugin, kind="fuel_exhausted"}   counter
wasmtime_fuel_consumed{plugin}                        histogram
```

with the explicit alerting advice: *"Alert on spikes in `wasmtime_traps_total{kind=\"fuel_exhausted\"}`… these usually indicate either an abusive module or a misconfigured limit."* Source: <https://www.systemshardening.com/articles/wasmtime-production-hardening/> (see also <https://www.systemshardening.com/articles/wasm/wasm-fuel-metering/>). A fuel-consumed **histogram** is what makes near-misses visible; a trap counter alone only fires after the cliff.

---

## 5. Fixture-scale testing and input-proportional fuel budgets

On the 2.4 KB fixture vs 574 KB production gap: the published calibration guidance is to derive the budget empirically from a representative corpus rather than pick a constant.

Recommended procedure (source: <https://www.systemshardening.com/articles/wasm/wasm-fuel-metering/>):

1. Run expected workloads with an effectively unlimited budget (`u64::MAX / 2`) and **log consumed fuel per invocation**.
2. Set the production budget at **1.5–2.0× measured p99**. *"The recommended pattern: measure p99 fuel consumption across a representative input corpus, then set the production budget at 1.5–2× p99. This accommodates legitimate input variation while keeping the maximum CPU time bounded."*
3. Add **at least 20% above measured p99** to account for future module updates and uncovered input space.
4. Use the observed **maximum as an alert threshold** — *"an invocation consuming more than the historical maximum is a signal of abnormal input."*
5. **Re-calibrate whenever the module is updated**, *"since instruction counts are tied to the specific compiled module."*

Two consequences specific to our situation:

- **The corpus must contain production-scale artifacts.** A 2.4 KB fixture cannot calibrate a budget for a 574 KB input — roughly 240× larger. For any validator that is O(n) or worse in input size, the fixture provides no signal about the real p99. Calibration inputs should include the largest live artifacts (`lessons.md`, `STATE.md`, `decision-log.md`, `burst-log.md`).
- **A fixed `fuel_cap` constant is the wrong shape for size-varying input.** Current values are fixed: `RegistryDefaults.fuel_cap = 10_000_000` (`registry.rs:187`), with resolvers and many tests using `1_000_000_000`. Note this is already known to bite in practice — `validate-state-structure/src/lib.rs:1334-1346` documents F-P1-003, where two `String` allocations per line *"exhausted the 10M fuel budget on the live 426-line"* file. That is the predicted failure mode, already observed at a file size far below 574 KB. CLAUDE.md's `lessons.md` budget note (D-442(e), ≤3500 soft / ≤4000 hard lines) is a workaround for the same root cause: the budget is a constant while the input grows.

The step-4 alert threshold is the mechanism that makes the fixture-scale gap self-revealing rather than something that has to be predicted in advance.

---

## 6. Assessment of the finding as stated, against the actual code

Being precise here matters, because the finding's stated mechanism would misdirect remediation.

**What the finding gets right:** there is a real fail-open path, it is security-relevant, and the fixture-scale gap means it has never been exercised. Severity HIGH is appropriate.

**What the finding gets wrong:** "indistinguishable from a clean successful validation" is too strong. Verified in the current tree:

- `invoke.rs:425-445` — `classify_trap` downcasts to `Trap` and maps `Trap::OutOfFuel` → `PluginResult::Timeout { cause: TimeoutCause::Fuel }`, structurally distinct from `PluginResult::Ok`. `Trap::Interrupt` → `TimeoutCause::Epoch`.
- `executor.rs:683-697` — a `Timeout` result emits a `PLUGIN_TIMEOUT` event with `cause` stringified as `"fuel"` or `"epoch"`, plus `elapsed_ms` and `fuel_consumed`.
- `resolver_classify_trap.rs:56, 186-197` — `Trap::OutOfFuel` and `Trap::Interrupt` both map to `ResolverError::Timeout` under F-P3-002, with tests asserting the mapping.

So at both the result-type layer and the telemetry layer, fuel exhaustion **is** distinguishable today.

**Where the defect actually lives — the enforcement channel.** `executor.rs` gates fail-closed behavior on `on_error`, and the existing test `fail_closed_timeout_with_on_error_continue_is_open` (`executor.rs:902-914`) asserts the fail-open outcome *by design*, using `TimeoutCause::Fuel` with `fuel_consumed: 1_000_000_000` as its fixture:

```rust
/// Timeout + on_error=Continue → NOT fail-closed.
#[test]
fn fail_closed_timeout_with_on_error_continue_is_open() {
    let r = PluginResult::Timeout {
        cause: TimeoutCause::Fuel,
        stderr: String::new(),
        elapsed_ms: 5_000,
        fuel_consumed: 1_000_000_000,
    };
    assert!(
        !plugin_fail_closed(&r, OnError::Continue),
        "Timeout + on_error=Continue must NOT trigger fail-closed"
    );
}
```

Consequently, for a plugin configured `on_error = "continue"`: no block intent is recorded, the aggregator sees no `exit_code == 2` + `OnError::Block` pair (`aggregator.rs:67`), and the dispatcher exits 0 — **the same allow-decision as a clean pass.** The finding's substance is correct; its locus is the decision channel, not observability. Restated precisely:

> Fuel exhaustion is observable in telemetry but **not** in the enforcement decision. For `on_error = "continue"` plugins, an unvalidated artifact and a validated-clean artifact produce an identical allow verdict and identical dispatcher exit code. This is CWE-636.

This distinction is not pedantic — it changes the fix. "Make exhaustion observable" is already done and would close nothing. The fix is to change what the dispatcher *does* with an outcome it already correctly identifies.

**Separate, minor, correctly-flagged-elsewhere:** `resolver_loader.rs:676` detects exhaustion with a brittle string match, `e.to_string().contains("all fuel consumed")`, placed *before* the structural downcast at 682-683. `resolver_classify_trap.rs:59-61` already carries a comment noting this string check is an anti-pattern. This is currently **latent, not live**: if Wasmtime changes the `Display` text, the branch stops matching and control falls through to `classify_resolver_trap`, which maps `Trap::OutOfFuel` to the same `ResolverError::Timeout`. Behavior is preserved either way. Worth deleting as redundant-and-brittle, but it is not a live bug and should not be reported as one.

---

## 7. Recommended remediation

Ordered by value-to-effort. Implementation is out of this document's scope — routing per CLAUDE.md is architect (policy semantics) then implementer.

1. **Make resource exhaustion fail-closed for validator-class plugins regardless of `on_error`.** Follow Istio's rule — fail-open is *"not recommended for the authentication or the authorization plugins"* — and our validators are authorization-class. The cleanest form: treat "did not complete" as categorically different from "completed and found nothing," so `on_error` governs *plugin errors* while incompleteness always denies. If a blanket change is judged too aggressive, follow Envoy and make it an explicit per-plugin `failure_policy` whose **default is fail-closed**, requiring an opt-in to bypass. Note this contradicts the intent asserted by `fail_closed_timeout_with_on_error_continue_is_open`, so that test encodes the policy decision and must be revised deliberately, not deleted quietly.

2. **Add a fuel-headroom check on the success path.** On `Ok(_)`, if `fuel_consumed > 0.9 × cap`, emit a distinct near-miss warning. Cheapest fix, catches the 574 KB cliff *before* it becomes a trap, and requires no policy change.

3. **Calibrate `fuel_cap` against production-scale inputs.** Replace the fixed `10_000_000` default with a p99-derived value measured over a corpus that includes the largest live `.factory/` artifacts. Consider a size-proportional budget (base + k × input_bytes) for validators that are linear in input size. This subsumes the D-442(e) `lessons.md` line-count workaround, which is a symptom of the constant-budget shape.

4. **Extend fixtures to production scale.** Add a ≥574 KB fixture to the bats/cargo suites for every plugin that reads whole artifacts. Per the Envoy #38801 lesson, also add a test that *asserts the chosen failure policy actually fires* — documented intent and observed behavior have diverged before in a far more heavily exercised system.

5. **Two-level exhaustion telemetry, per the Fastly model.** A categorical resource-limit counter plus a per-cause counter (`fuel` vs `epoch`), and a `fuel_consumed` histogram to make near-misses visible pre-cliff.

6. **Delete the brittle string match** at `resolver_loader.rs:676`, leaving the structural downcast. Cosmetic, ~5 minutes, and `resolver_classify_trap.rs` already documents why.

7. **Keep both fuel and epoch enabled.** Current configuration is correct and matches published production guidance. No change.
