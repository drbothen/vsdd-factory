# PR Review — #738 fix(dispatcher): gate .factory/logs creation on a mounted worktree — stop racing bootstrap

**Verdict:** REQUEST_CHANGES
**Reviewer:** pr-reviewer (fresh-eyes, final pre-merge)
**Base:** develop | **Class:** fix-PR (backlog remediation) | **Refs:** #206, #203

## What I verified

The pure gate (`factory_mount_ready`) in `log_dir.rs` is correct and cleanly handles the described cases (absent, plain-dir, `.git`-file mount, `.git`-dir checkout, non-`.factory` override path). The builder/`with_mount_gate` pattern is sound and thread-safe: `mount_gated` is write-only before `Arc` construction, read-only after; `gate_warned` as `Arc<AtomicBool>` with `swap(true, Relaxed)` is a correct once-per-session print guard. The behavior table rows 1–4 are implemented correctly in the gate logic itself.

## Findings

### [HIGH] `FACTORY_ROOT` in the override bypass reopens the #206 race — `main.rs`

The bypass in `main.rs` disables the mount gate whenever **either** `VSDD_LOG_DIR` **or** `FACTORY_ROOT` is set and non-empty:

```rust
let explicit_override = ["VSDD_LOG_DIR", "FACTORY_ROOT"]
    .iter()
    .any(|k| std::env::var(k).is_ok_and(|v| !v.is_empty()));
InternalLog::new(resolve_log_dir()).with_mount_gate(!explicit_override)
```

The two env vars are not equivalent for this gate:
- `VSDD_LOG_DIR` (level A) points *directly* at a log dir that may legitimately live outside any `.factory`. Bypassing the gate for it is defensible.
- `FACTORY_ROOT` (level B) is a **root**, and `resolve_log_dir()` derives `$FACTORY_ROOT/.factory/logs` from it — i.e., the resulting path is parented on `.factory`, which is exactly the shape `factory_mount_ready` guards. The gate would handle a `FACTORY_ROOT`-derived path correctly on its own.

**Failure scenario:** an operator runs first-time `/factory-health` bootstrap with `FACTORY_ROOT` exported. The gate is disabled; the dispatcher resumes unconditional `create_dir_all($FACTORY_ROOT/.factory/logs)` on every tool use; a plain `.factory` is re-planted before `git worktree add` runs; git mounts nested at `.factory/.factory` — the #206 root cause this PR set out to eliminate.

The bypass should be narrowed to `VSDD_LOG_DIR` only. The stated fixture problem ("this repo's bats harness points `VSDD_LOG_DIR` at scratch `.factory/logs`") only requires the `VSDD_LOG_DIR` exemption. If the harness also relies on `FACTORY_ROOT` scratch fixtures, the production-grade fix is to give those fixtures the `.git` mount shape (as was already done for the two updated fixtures) rather than blanket-bypass the gate.

### [MEDIUM] `bin/factory-obs` retains the same unconditional `mkdir -p` with no tracked follow-up
The Notes section acknowledges `factory-obs` was left out to keep the PR single-concern. Under the project's production-grade default, a bare "left out" is a defer-pattern smell unless anchored to a concrete tracked follow-up. Please open a tracking issue for the `factory-obs` guard and reference it before merge.

### [LOW] Test coverage cements the blanket bypass rather than distinguishing the two env vars
`gate_bypassed_for_explicit_override` locks the bypass for the combined case but does not assert that a `FACTORY_ROOT`-derived path (`$FACTORY_ROOT/.factory/logs`) remains gated. If the bypass is narrowed per the HIGH finding, add a test verifying a `FACTORY_ROOT`-derived path is still subject to the mount check.

### [LOW] Gate trusts any `.factory/.git` entry — possible false "ready"
`factory_mount_ready` returns `true` whenever `parent.join(".git").exists()`. A dangling `.git` file from a de-mounted worktree satisfies the gate even with no live mount. Impact is benign (logs land rather than racing), but worth a doc comment noting the invariant being relied on.

### [INFO] Thread-safety and builder pattern are sound
`with_mount_gate` takes `mut self` and is applied pre-`Arc`, before any sharing; `mount_gated` is read-only thereafter. `gate_warned` `AtomicBool::swap(true, Relaxed)` is correct for a once-per-session print guard. No race.

## Summary
The pure gate logic and the `InternalLog` mechanism are correct and well-tested. The blocking concern is in `main.rs`: folding `FACTORY_ROOT` into the bypass disables the gate for paths that still resolve under `.factory`, which can reintroduce the #206 nesting race during a `FACTORY_ROOT`-configured first-time bootstrap — undercutting the PR's own goal. Narrow the bypass to `VSDD_LOG_DIR`, verify the bats harness doesn't rely on `FACTORY_ROOT` scratch fixtures for this, anchor the deferred `factory-obs` guard to a tracked issue, and add a test asserting `FACTORY_ROOT`-derived paths remain gated.
