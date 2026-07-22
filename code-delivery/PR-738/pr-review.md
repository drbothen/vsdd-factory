# PR #738 Re-Review — head `a9dfdcca`

**PR:** fix(dispatcher): gate .factory/logs creation on a mounted worktree — stop racing bootstrap
**Repo:** drbothen/vsdd-factory
**Head reviewed:** `a9dfdcca7fa3a22db0c1f7af391f415f33c3deca`
**Reviewer:** pr-reviewer (fresh-eyes re-review after prior REQUEST_CHANGES)
**Verdict: REQUEST_CHANGES** — the HIGH finding remains UNADDRESSED.

---

## Verdict criterion

APPROVE only if the HIGH finding is FIXED. The HIGH finding is **UNADDRESSED**
(the `FACTORY_ROOT` bypass was retained, not narrowed to `VSDD_LOG_DIR`), so the
verdict is **REQUEST_CHANGES**.

Since the prior review, the only new commit at this head (`a9dfdcca`) is a
docs-only change to the migration guide. No code changed. None of the four prior
findings were addressed.

---

## Per-finding closure status

### HIGH — narrow bypass to `VSDD_LOG_DIR` only; keep `FACTORY_ROOT`-derived paths gated — **UNADDRESSED**

`crates/factory-dispatcher/src/main.rs` at head:

```rust
let explicit_override = ["VSDD_LOG_DIR", "FACTORY_ROOT"]
    .iter()
    .any(|k| std::env::var(k).is_ok_and(|v| !v.is_empty()));
let internal_log =
    Arc::new(InternalLog::new(resolve_log_dir()).with_mount_gate(!explicit_override));
```

`FACTORY_ROOT` is still in the bypass array. The recommended fix was not applied.

Why this is a real, still-open defect — grounded in `log_dir.rs` resolution:

- **Level A (`VSDD_LOG_DIR`)** returns the value directly. It is a deliberate
  per-invocation diagnostic override and is the *only* consumer that needs the
  bypass — the repo's own bats harness points `VSDD_LOG_DIR` at scratch
  `.factory/logs` fixtures, which is the documented justification for the
  second commit.
- **Level B (`FACTORY_ROOT`)** returns `$FACTORY_ROOT/logs` (only `/logs` is
  appended — NOT `/.factory/logs`). By the resolution contract `FACTORY_ROOT`
  names the `.factory` directory itself, so the conventional
  `FACTORY_ROOT=<repo>/.factory` yields a log dir whose parent basename is
  `.factory` — exactly the shape `factory_mount_ready` exists to hold back.

Failure scenario: an operator with `FACTORY_ROOT=<repo>/.factory` exported (the
population most likely to be mid-setup) runs first-time `/factory-health`
bootstrap. `explicit_override == true` → `with_mount_gate(false)` → gate
skipped. The dispatcher's per-tool-use `mkdir -p $FACTORY_ROOT/logs` re-plants a
plain `.factory/` in the window before `git worktree add .factory
factory-artifacts`, and git mounts nested at `.factory/.factory` — the #206 race
this PR set out to close, reopened for the FACTORY_ROOT-configured path.

There is no demonstrated need to bypass for `FACTORY_ROOT`: the only cited
consumer (bats) uses `VSDD_LOG_DIR`. Keeping `FACTORY_ROOT` gated costs nothing
in the non-conventional case — if `FACTORY_ROOT` points somewhere NOT named
`.factory`, `factory_mount_ready` already returns `true` and never fires. So
gating `FACTORY_ROOT` only ever engages in precisely the racing case.

Fix: `let explicit_override = std::env::var("VSDD_LOG_DIR").is_ok_and(|v| !v.is_empty());`
(drop `FACTORY_ROOT` from the bypass set), and correct the `main.rs` comment and
the `log_dir.rs` helper doc, which currently claim both level A and B are
exempted. If any harness relies on a `FACTORY_ROOT` scratch fixture, give that
fixture the `.git` mount shape (as was already done for the two updated
fixtures) rather than blanket-bypassing the gate.

### MEDIUM — `bin/factory-obs` unconditional mkdir, no tracked follow-up — **UNADDRESSED**

The PR "Notes" section still acknowledges `bin/factory-obs` (`_generate_override`)
does `mkdir -p "$root/.factory/logs"` outside any worktree check — a second
creation site with the same race potential — and defers it, citing "the
onboard-observability ordering guard in the companion skills PR covers its
practical path." No issue number is referenced and no tracked follow-up appears
in this diff. Under the production-grade default this second creation site
should be fixed in-scope or, at minimum, anchored to a real tracked issue ID; a
prose "covered elsewhere" claim without an anchor is a defer-pattern smell. Not
itself the merge blocker, but still open.

### LOW — test cements the blanket bypass — **UNADDRESSED**

`gate_bypassed_for_explicit_override` (internal_log.rs) asserts
`with_mount_gate(false)` writes into a plain `.factory` regardless of mount
state. While the HIGH finding stands, `main.rs` routes `FACTORY_ROOT` into that
same `with_mount_gate(false)` path, so the suite locks in the over-broad bypass.
There is still no test exercising the `main.rs` env-var branch. When HIGH is
fixed, add coverage asserting a `FACTORY_ROOT`-derived (level B) path stays
gated while a `VSDD_LOG_DIR` (level A) path bypasses.

### LOW — gate trusts any `.factory/.git` entry — **UNADDRESSED**

`factory_mount_ready` returns `parent.join(".git").exists()`. Any `.git` entry —
including a dangling/garbage `.git` file with no valid `gitdir:` pointer —
satisfies the gate. A more robust check would confirm the `.git` file contains a
`gitdir:` line (or that the `.git` dir is a real git dir). Low severity: the
false-shape is unlikely and the failure direction (allowing a write) is the
benign one. Unchanged at this head.

---

## New findings at head `a9dfdcca`

The head commit is docs-only (`docs/guide/migrating-from-0.79.md`): it adds
guidance that a missing/empty internal log can mean an unmounted `.factory`
(run `/vsdd-factory:factory-health`) rather than a broken dispatcher. Accurate
and consistent with the code behavior. No new code-level defects. The
`eprintln!` once-per-session notice follows the file's existing best-effort
stderr diagnostic pattern (matching the dedup path) and is not a new
`println!`-convention violation.

The pure gate design (`factory_mount_ready`, write-path gating, the
`with_mount_gate` builder, the absent/plain/mounted/checkout/override test
matrix) remains correct and well-tested — the thread-safety of `mount_gated`
(write-only pre-`Arc`) and `gate_warned` (`AtomicBool::swap(true, Relaxed)`) is
sound.

---

## Summary

| Finding | Severity | Status |
|---|---|---|
| Narrow bypass to VSDD_LOG_DIR; keep FACTORY_ROOT gated | HIGH | UNADDRESSED |
| factory-obs unconditional mkdir, no tracked follow-up | MEDIUM | UNADDRESSED |
| Test cements blanket bypass | LOW | UNADDRESSED |
| Gate trusts any .factory/.git entry | LOW | UNADDRESSED |

The one merge-blocking issue is that the second-commit override bypass was not
narrowed: `FACTORY_ROOT` still disables the gate and, in its conventional
`.factory`-named form, reopens the #206 race this PR exists to close. This is a
structural gap, not a paper-fix — the code is simply unchanged from the prior
review at the load-bearing line. **REQUEST_CHANGES.**
