# Feasibility Spike — Port factory-artifacts lock enforcement from shell to a Rust chokepoint

**Type:** general (technology/implementation research → feeds an architecture decision)
**Produced by:** research-agent
**Date:** 2026-06-11
**Status:** complete — findings for architect adjudication (this document does NOT make the decision)
**Anchors:** ADR-025 (Decision 8, Decision 11), Epic E-17 / issue #170, S-17.04 (in-flight, branch `feature/S-17.04-mid-burst-heartbeat-renewal-wiring`)

> **Scope discipline.** This is a research/analysis artifact. It recommends but does not decide. It modifies no source code. Every claim about current behavior is cited `file:line`. The architect owns the final ADR amendment (ADR-025 v1.6 or a new ADR-026) and the routing of any implementation stories.

---

## Executive Summary (recommendation)

**Packaging:** Add a `lock` subcommand to the existing `factory-dispatcher` binary (`factory-dispatcher lock {acquire|renew|clear|cas-push|status|precheck|unlock-decide|verify-renewal}`) rather than create a new `crates/factory-lock/` standalone binary. The dispatcher is *already* cross-compiled and bundled for all 5 platforms by `release.yml` (`build-binaries` matrix, lines 65–97) and staged into `plugins/vsdd-factory/hooks/dispatcher/bin/<platform>/` (lines 226–241). A subcommand reuses that entire shipping pipeline at **zero** new CI matrix cost and **zero** new bundling/activation work. A new crate would require parallel matrix legs, a parallel staging step, a parallel activation path, and a second binary on disk — all pure overhead for a tool that the SKILLs already invoke by absolute path. The one real cost (subcommand inherits the wasmtime-heavy binary's size/startup) is immaterial here because the SKILL invokes the binary out-of-band as a normal subprocess (no per-hook-call hot path).

**Port boundary:** Move the **enforcement-critical** logic to Rust and keep **display-only** logic as thin shell (or fold it in too — see §2). Enforcement-critical = `factory-cas-push.sh` (the push chokepoint), the renewal-check that S-17.04 is currently trying to bolt on as a brittle PreToolUse string-parser, and the identity/TTL/schema-writing core of `factory-lock-write.sh` (acquire/renew/clear). The renewal check becomes an **in-process step inside the `cas-push` subcommand** — it runs at the exact instant before the binary shells out to `git push`, so there is no Bash command string for a PreToolUse hook to parse, and all 4 tokenizer-bypass vectors evaporate structurally (§6).

**Migration:** Use the **shell-shim** approach. Keep every `.sh` filename as a ~3-line wrapper that `exec`s the new subcommand, so the 5 SKILLs, `state-manager.md`, and all 6 bats suites keep calling the same paths. Migrate callers off the shims opportunistically in a later story; do not block this port on a caller sweep. This makes the port a drop-in and keeps the blast radius (§ caller table) to the shim files themselves on day one.

**jq → serde:** Confirmed trivially feasible. The workspace **already depends on `serde_norway` 0.9** (`Cargo.toml:63`, the maintained serde_yaml fork, migrated in TD #72) and `chrono` with serde (`Cargo.toml:80`). The Rust core parses STATE.md frontmatter with `serde_norway::from_str` into a typed `factory_lock` struct — eliminating the D12 jq-footgun (CapabilityDenied → silent fail-open) entirely, because the new model has no `jq` dependency at all.

**Net effect on S-17.04:** The chokepoint design makes S-17.04's "Mechanism 2" (`verify-lock-renewal.sh` PreToolUse gate) **unnecessary** — its entire reason to exist is that the push happened inside an opaque helper. Mechanism 1 (the state-burst SKILL renew step) remains valuable and complementary and should be retained. See §6 + §7 for the architect decision this forces.

---

## Background facts established by codebase read

- The 3 merged E-17 stories shipped 6 shell helpers under `plugins/vsdd-factory/bin/`: `factory-cas-push.sh` (88L), `factory-lock-write.sh` (379L), `factory-lock-status.sh` (219L), `factory-lock-acquire-precheck.sh` (278L), `factory-unlock-decide.sh` (235L), plus `emit-event` (not read here; event emission helper).
- The Rust workspace (`Cargo.toml:3–44`) has 40+ members; `factory-dispatcher` (`crates/factory-dispatcher/Cargo.toml`) is the only one cross-compiled-and-shipped as a native binary (`release.yml:120–127` builds `-p factory-dispatcher`; the matrix at `release.yml:76–96` covers darwin-arm64, darwin-x64, linux-x64, linux-arm64 via `cross`, windows-x64).
- The dispatcher's `main.rs` is a single `#[tokio::main(flavor = "current_thread")]` entry (`main.rs:81–94`) that reads a hook envelope from stdin and dispatches WASM plugins. It currently has **no argv/subcommand parsing** — `clap` is a workspace dependency (`Cargo.toml:72`) but the dispatcher's Cargo.toml does NOT list it (`crates/factory-dispatcher/Cargo.toml:20–38`); it would be added.
- Confirmed: a standalone CLI invoked from a SKILL has **no host-ABI impact**. `HOST_ABI_VERSION` governs the WASM plugin host interface only; a subprocess invoked by a SKILL is just `bash → binary`, outside the WASM sandbox.

---

## 1. Packaging — subcommand vs new standalone binary

### Option A — `factory-dispatcher lock <subcommand>` (RECOMMENDED)

Add `clap` to `crates/factory-dispatcher/Cargo.toml`, branch in `main.rs` on argv: if `args[1] == "lock"`, route to a new `lock` module and **never read stdin / never load the registry** (the dispatcher's hook path at `main.rs:96–98` reads stdin first — the lock branch must short-circuit before that).

| Dimension | Impact |
|-----------|--------|
| **5-platform cross-compile matrix** | **Zero new legs.** `release.yml:120–127` already runs `cargo build --release -p factory-dispatcher` per platform. A subcommand is the same binary — it ships automatically. No `release.yml` edit needed for the build matrix. |
| **Bundling / activation** | **Zero new work.** The binary already stages to `hooks/dispatcher/bin/<platform>/factory-dispatcher` (`release.yml:226–241`) and the activate skill selects per-platform. A new crate would need a parallel staging block + a parallel activation path. |
| **Binary size** | Subcommand inherits the dispatcher's size (embeds wasmtime ~ tens of MB). Research corroborates that embedding into a wasmtime-heavy binary has "substantial" size implications. **But** the binary is *already shipped*; the lock code adds serde_norway (already linked) + chrono (already linked) + clap (~small) ≈ negligible marginal bytes. There is no *second* large binary. |
| **Startup cost** | Subcommand pays the binary's process-start cost. Irrelevant: the SKILL calls it a handful of times per burst as a normal subprocess, not on a per-hook hot path. The dispatcher's own hot path (`main.rs`) is untouched — the lock branch short-circuits before `build_engine()` (`main.rs:275`). |
| **SKILL invocation** | `factory-dispatcher lock cas-push` (resolved via the bundled bin path, same dir the SKILLs already know). Shims (§4) keep the legacy `bash .../factory-cas-push.sh` form working verbatim. |
| **Release cadence coupling** | The lock binary ships **only** on a new rc tag (CLAUDE.md "Dispatcher binary discipline"). Same as today's WASM guard. See §7 open question on rc.22 cache reach. |

### Option B — new `crates/factory-lock/` standalone binary

| Dimension | Impact |
|-----------|--------|
| **5-platform matrix** | **New parallel legs required.** `release.yml`'s `build-binaries` job builds `-p factory-dispatcher` only (line 124/126). A second crate needs either `-p factory-lock` added to every leg (cheap — shares the cargo cache; research confirms one `cargo build` with shared deps does NOT re-download/re-compile shared deps) OR a second matrix. Either way `release.yml` + the staging step (`release.yml:226–241`) + activation must learn about a second binary path. |
| **Bundling / activation** | New staging dir `hooks/dispatcher/bin/<platform>/factory-lock` (or similar), new activation selection, new `.gitignore`/commit handling. Net-new surface in 2 jobs. |
| **Binary size** | A small dedicated binary (serde_norway + chrono + clap, no wasmtime) is small (~MB). This is the *only* dimension where B wins, and it doesn't matter because the dispatcher binary ships regardless — B adds a second artifact rather than removing the first. |
| **Startup cost** | Marginally faster cold start (no wasmtime to map). Immaterial for SKILL-invoked-subprocess usage. |
| **Conceptual cleanliness** | B is arguably "cleaner" (lock logic isn't grafted onto a hook dispatcher). This is the strongest argument for B and is a legitimate architect judgment call. |

### Recommendation

**Option A (subcommand).** The cross-compile + bundling + activation pipeline is the expensive, fragile part of this repo's release story (see the elaborate `release.yml` `commit-binaries` + `sync-develop` choreography, lines 190–727). Reusing it wholesale for zero new YAML is worth far more than the modest conceptual tidiness of a separate crate. If the architect prefers strict separation of concerns, Option B is viable at the cost of duplicating the matrix/staging/activation surface — present it as the explicit tradeoff. Recommend isolating all lock logic in a `factory_dispatcher::lock` library module so a future extraction to a standalone crate is a move, not a rewrite.

---

## 2. Port boundary — enforcement-critical (→ Rust) vs display-only (stays shell)

### Enforcement-critical — MUST move to Rust

| Source responsibility | Current location | Why enforcement-critical |
|-----------------------|------------------|--------------------------|
| **CAS push** (fetch → capture EXPECTED_SHA → cat-file existence guard → `--force-with-lease=ref:sha` push) | `factory-cas-push.sh:48–88` | This is THE push chokepoint (ADR-025 Decision 8). Concurrency correctness lives here. The renewal check must be co-located with this action (§6). |
| **Renewal check** (compare HEAD `expires_at` vs `origin/factory-artifacts` `expires_at`; block if equal while lock held) | Currently being added as `verify-lock-renewal.sh` PreToolUse gate (S-17.04 D11, brittle) | The whole point of the port. Moves INTO the `cas-push` subcommand, immediately before the `git push` call. No command string to parse. |
| **TTL / identity / schema core of acquire-renew-clear** | `factory-lock-write.sh` — `_capture_now_epoch` (`:120–125`, single clock read → locked_at + expires_at = now+2700), `git config user.email` holder capture (`:310–313`), `_write_factory_lock_block` (`:191–224`), `_update_expires_at` renew (`:232–257`), `_remove_factory_lock` clear (`:164–184`), post-write/post-renew/post-clear assertions (`:324–327`, `:352–358`, `:371–374`), TTL constant 2700 (`:66`), CRLF normalization (`:264–277`), frontmatter fence validation (`:284–293`) | These write the authoritative lock state. TTL=2700 invariant (BC-5.40.001 Inv 2), single-clock-read invariant (Inv 3), key-deletion-not-null (PC2) are correctness contracts. Brittle awk frontmatter surgery (`:170–257`) is exactly what serde_norway replaces (§3). |
| **expires_at extraction from frontmatter** | Duplicated `_extract_lock_field` awk in `factory-lock-status.sh:132–153`, `factory-lock-acquire-precheck.sh:137–155`, `factory-unlock-decide.sh:143–161` (3 byte-near-identical copies) | Single typed parse in Rust removes 3 copies of fragile awk. |
| **ISO-8601 ⇄ epoch + BSD/GNU `date` portability branches** | `factory-lock-write.sh:103–112`, `factory-lock-status.sh:160–173`, `factory-lock-acquire-precheck.sh:161–170` | `chrono` (already a dep) replaces every `date -u -d` / `date -u -r` / `date -u -j -f` portability fork. |

### Display-only / decision-token — CAN stay shell (but cheap to fold in)

| Source responsibility | Current location | Note |
|-----------------------|------------------|------|
| **Three-state status string rendering** | `factory-lock-status.sh:179–219` (FREE / HELD-by-this-session / HELD-by-other / malformed) | Pure display. `factory-health` (`SKILL.md:101`) + `factory-worktree-health` (`SKILL.md:157`) consume it. Could stay shell, but it shares the same parse + epoch logic as the core — folding it into a `lock status` subcommand removes the 3rd awk/date copy. **Recommend folding in.** |
| **Acquire-precheck decision tree** | `factory-lock-acquire-precheck.sh:191–278` (fetch guard EC-006, email guard EC-007, PROCEED_ACQUIRE / NOOP_SELF_HELD / REFUSED_FOREIGN_LOCK tokens + 5-field refusal message) | Emits decision tokens the `/factory-lock` SKILL branches on. Token-emitting "pure-core". Can stay shell, but reuses the same parse/epoch/identity core. **Recommend folding into `lock precheck`.** |
| **Unlock decision tree** | `factory-unlock-decide.sh:182–235` (NOOP_ABSENT / PROCEED_RELEASE / PROCEED_RELEASE_SELF_FORCE / PROCEED_FORCE_STEAL / REFUSED_NOT_HOLDER + audit-event fields) | Same as precheck: token-emitting decision logic, no writes. **Recommend folding into `lock unlock-decide`.** |
| **Interactive prompts** | None of these scripts prompt interactively; the `--force` flag is the only "interactive" surface (`factory-unlock-decide.sh:200`). | No TTY interaction to port. |

**Boundary summary:** The minimal port is `cas-push` + renewal-check + `factory-lock-write` core. The *recommended* port folds in status/precheck/unlock-decide too, because they all share the same frontmatter-parse + ISO-8601 + identity primitives — porting the core but leaving 3 awk/date copies in shell would re-introduce the exact fragility class the port exists to kill. The architect should decide minimal-vs-full; this spike recommends **full** (all 6 helpers → subcommands) on the production-grade-default principle (CLAUDE.md: don't leave half the fragility in place).

---

## 3. jq → serde — confirmed, footgun killed

**Finding: fully feasible, and the dependency is already present.**

- The workspace declares `serde_norway = "0.9"` at `Cargo.toml:63` — the maintained serde_yaml fork (the comment at `:56–62` documents the TD #72 migration from deprecated serde_yaml and the rejection of serde_yml due to RUSTSEC-2025-0068). API is drop-in: `from_str`, `to_string`, `Value`, `Mapping`. Research corroborates that the serde_yaml fork family is suitable for frontmatter parse-and-reserialize as of mid-2026.
- `chrono` with `serde` is already at `Cargo.toml:80`.

**Frontmatter shape** (the `factory_lock:` block, from `factory-lock-write.sh:207–218` writer + ADR-025 Decision 2 `:133–138`):

```yaml
factory_lock:
  holder: "developer@example.com"    # git config user.email
  locked_at: "2026-06-10T14:00:00Z"  # ISO-8601 UTC
  expires_at: "2026-06-10T14:45:00Z" # ISO-8601 UTC; locked_at + 2700s
```

Rust model:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct FactoryLock {
    holder: String,
    locked_at: String,      // or chrono::DateTime<Utc>
    expires_at: String,
}
```

**The D12 jq-footgun (`exec_subprocess.binary_allow` missing `jq` → CapabilityDenied → silent fail-open → gate inert; ADR-025 v1.5 amendment, "fourth deny-by-default silent-no-op vector") is eliminated structurally**: the Rust path reads STATE.md directly and parses with serde_norway. There is no `jq` exec, no `binary_allow` list to forget, and no WASM capability gate involved at all (the lock subcommand runs as a plain SKILL-invoked subprocess, not a WASM plugin). The entire D12 footgun class disappears because its precondition (shell needs jq to parse the JSON envelope's `.tool_input.command`, and awk to parse STATE.md frontmatter) no longer exists.

**One parsing caveat for the architect/implementer:** STATE.md is a Markdown file with a YAML frontmatter block delimited by `---` fences, NOT a pure YAML document. The Rust core must split on the first two `^---$` fences (mirroring the shell's `awk '/^---$/{fence++}'` boundary logic, e.g. `factory-lock-write.sh:170–184`) and feed ONLY the frontmatter region to `serde_norway::from_str`. The body (which may contain its own `factory_lock:`-prefixed prose lines, a case the shell explicitly guards at `:160–162`) must be preserved byte-for-byte on rewrite. This is a known, bounded requirement — note it as an implementer obligation, not a blocker. The same CRLF normalization the shell does (`:264–277`) must be replicated (or handled by reading as bytes and normalizing line endings before the fence split).

---

## 4. Caller migration — shell-shim vs update-all-callers

### Recommendation: **shell-shim** (keep filenames, exec the subcommand)

Replace each helper body with a ~3-line shim, e.g. `factory-cas-push.sh`:

```bash
#!/usr/bin/env bash
exec "$(dirname "$0")/../hooks/dispatcher/bin/$(detect-platform)/factory-dispatcher" lock cas-push "$@"
```

(Platform detection / bin-path resolution should reuse whatever the activate skill already does; the shim is the thin adapter.)

**Why shim, not update-all-callers:**
- Keeps the 5 SKILLs, `state-manager.md`, and all 6 bats suites calling the exact same paths/arguments — the port becomes a drop-in with a tiny day-one blast radius (only the 6 shim files change).
- The bats suites assert on the helpers' stdout/stderr/exit-codes (the *contract*), which the shim preserves verbatim. Tests keep passing as a CLI-contract regression net (§5).
- A later cleanup story can migrate callers to `factory-dispatcher lock …` directly and delete the shims — but that is optional and non-blocking.

**Risk of shim approach:** an extra process hop (bash → binary). Immaterial for these low-frequency invocations.

### Complete caller blast radius (grep-confirmed)

**Production callers (SKILLs + agent prose):**

| Caller | Line(s) | Helper invoked | Affected by shim? |
|--------|---------|----------------|-------------------|
| `skills/state-burst/SKILL.md` | `:163` | `factory-cas-push.sh` (the push step) | No — shim preserves path. **This is the push chokepoint** the renewal check moves into. |
| `skills/factory-lock/SKILL.md` | `:19`, `:39`, `:42` | `factory-lock-acquire-precheck.sh`, `factory-lock-write.sh acquire`, `factory-cas-push.sh` | No |
| `skills/factory-unlock/SKILL.md` | `:18`, `:52`, `:56` | `factory-unlock-decide.sh`, `factory-lock-write.sh clear`, `factory-cas-push.sh` | No |
| `skills/factory-health/SKILL.md` | `:101` | `factory-lock-status.sh` | No |
| `skills/factory-worktree-health/SKILL.md` | `:157` | `factory-lock-status.sh` | No |
| `agents/state-manager.md` | `:239`, `:248–251`, `:277` | documents `factory-lock-write.sh acquire/renew/clear` as the canonical helper | Prose only — update the cross-reference text in the same port story (state-manager owns this file; route to state-manager). |

**Test callers (bats — 6 suites; cover the lock scripts directly):**

| bats suite | Covers |
|------------|--------|
| `tests/factory-cas-push.bats` | `factory-cas-push.sh` |
| `tests/factory-lock-write.bats` | `factory-lock-write.sh` acquire/renew/clear |
| `tests/factory-lock-status.bats` | `factory-lock-status.sh` |
| `tests/factory-lock-acquire-precheck.bats` | `factory-lock-acquire-precheck.sh` |
| `tests/factory-unlock-decide.bats` | `factory-unlock-decide.sh` |
| `tests/factory-lock-skills-integration.bats` | cross-helper SKILL integration |

**Demo-evidence references (non-executing docs):** `docs/demo-evidence/S-17.01/*` and `docs/demo-evidence/S-17.03/*` reference the helpers in `.tape`/`.sh`/`evidence-report.md` files. These are historical evidence artifacts — they do not need migration (do not rewrite shipped demo evidence; note as documentary-historical).

**S-17.04 in-flight callers:** the not-yet-merged `verify-lock-renewal.sh` (D11) + its registry entry (D12) + `verify-lock-renewal.bats` (D14) reference `factory-cas-push` as the gate trigger. The chokepoint port **obviates** these (§6) — the architect must decide whether S-17.04 lands first then gets superseded, or is redirected now (§7 open question).

---

## 5. Test surface — how the strategy changes

**Target end-state: two-layer testing.**

1. **Rust unit + integration tests (`#[test]` / `tests/`) for the ported core** — the precise, fast, deterministic layer. Test the frontmatter split, serde round-trip (parse → mutate `expires_at` → reserialize → body preserved byte-for-byte), TTL = now+2700 single-clock invariant, ISO-8601⇄epoch, holder identity, and — critically — the renewal-check comparison logic as a **pure function** (`fn renewal_was_committed(head_expires: &str, remote_expires: &str, holder: Option<&str>) -> Decision`). This is where the port pays off: the brittle awk/date/jq edge cases (CRLF, malformed single-fence frontmatter, missing sub-fields, body `factory_lock:` lines) become table-driven Rust tests instead of bats fixtures. The dispatcher crate already has a rich `#[dev-dependencies]` test harness (`crates/factory-dispatcher/Cargo.toml:40–54`: tempfile, filetime, proptest) to model on.

2. **Retained bats for the shim/CLI contract** — the 6 existing suites keep running against the shimmed `.sh` files, now verifying the *CLI contract* (stdout tokens, stderr messages, exit codes) is preserved across the shell→Rust move. They become the integration/regression net that proves the shim is behavior-identical. `release.yml:34–35` already runs `./run-all.sh` as a release gate, and `:37–41` shellchecks `bin/*` — the shims must stay shellcheck-clean.

**Migration sequencing for tests:** port the core with Rust tests FIRST (green), then shim the `.sh` files, then confirm the 6 bats suites still pass unchanged. Any bats assertion that breaks reveals a contract drift in the shim — exactly the signal you want. The exact-string error/token contracts (e.g. `factory-cas-push.sh:83–84` CASPushRejected message, `factory-unlock-decide.sh:231` REFUSED_NOT_HOLDER message, the 5-field refusal in `factory-lock-acquire-precheck.sh:270–276`) must be reproduced verbatim by the Rust subcommands because the bats suites and the SKILL UX assert on them.

**S-17.04's `verify-lock-renewal.bats` (12 Red Gate tests):** if the chokepoint supersedes Mechanism 2, these tests are re-homed: the *renewal-check logic* they exercise moves to Rust `#[test]`s against the pure comparison fn; the *PreToolUse-gate-fires-on-command-string* tests are deleted (no gate, no command string). The block-message contract (`BLOCKED by verify-lock-renewal: RenewalMissed …`, AC-002) is replaced by the `cas-push` subcommand's own block message/exit code.

---

## 6. Chokepoint correctness — does it eliminate all 4 bypass vectors?

**Yes — structurally, by construction. Stated in writing for the architect:**

The 4 tokenizer/quoting bypass vectors (inert-match → over-match → newline-injection → env-injection) ALL exist because S-17.04 Mechanism 2 is a **PreToolUse hook that parses an untrusted Bash command STRING** to decide whether a `factory-artifacts` push is about to happen. ADR-025 v1.5 itself documents the root cause (`ADR-025:441–456`): the SKILL runs `bash plugins/vsdd-factory/bin/factory-cas-push.sh`, and "the real `git push --force-with-lease` is a subprocess inside that helper — PreToolUse never inspects subprocess command strings." The gate is reduced to guessing, from a string, whether a push will occur — and string-guessing is defeatable by tokenization, quoting, newline-injection, and env-substitution tricks.

**Moving the renewal check INSIDE the push path removes the string entirely.** In the Rust `cas-push` subcommand the sequence is:

```
fetch origin factory-artifacts
EXPECTED_SHA = rev-parse origin/factory-artifacts
HEAD_EXPIRES   = parse(git show HEAD:STATE.md)
REMOTE_EXPIRES = parse(git show origin/factory-artifacts:STATE.md)
if lock_held(HEAD) && HEAD_EXPIRES == REMOTE_EXPIRES:
    abort with RenewalMissed   ← the check is HERE, in-process
git push --force-with-lease=factory-artifacts:EXPECTED_SHA   ← the action is HERE
```

The check and the action are the same program, in sequence, with no intermediary that must reconstruct intent from a command string. There is:
- **No command string to tokenize** → inert-match vector gone.
- **No regex over user-controllable Bash** → over-match vector gone.
- **No newline-delimited command parsing** → newline-injection vector gone.
- **No env-substituted command reconstruction** → env-injection vector gone.

The only way to push to `factory-artifacts` without passing the renewal check is to NOT call the `cas-push` subcommand — i.e. to hand-type a raw `git push origin factory-artifacts`. That residual path is exactly what the **existing `verify-git-push.sh` PreToolUse hook and Mechanism 1 discipline already cover**, and it is the same residual the shell design already accepted (ADR-025 Decision 8's CAS push is the network-layer safety net regardless). The chokepoint does not *widen* that residual; it *narrows* enforcement to the one true action point.

**Mechanism 1 is complementary and MUST be retained.** Mechanism 1 (the state-burst SKILL's mandatory `factory-lock-write.sh renew` step before `git add`, S-17.04 AC-001/D10) is the thing that *makes the renewal happen* in the normal flow. The chokepoint is the thing that *enforces it happened* at push time. They are not redundant: Mechanism 1 performs the renewal; the chokepoint refuses the push if it was skipped. Removing Mechanism 1 would mean the chokepoint blocks every honest burst that forgot to renew (annoying-but-safe), but keeping it means the honest path sails through and only genuinely-skipped renewals get blocked. **Recommendation: retain Mechanism 1, replace Mechanism 2 (the brittle gate) with the in-process chokepoint check.**

---

## 7. Risks & open questions for the architect (ADR-level)

| # | Question | Why it needs an architect decision | Spike's lean |
|---|----------|-------------------------------------|--------------|
| **R1** | **Repo-root / STATE.md path resolution.** The shells operate on `.factory/STATE.md` relative to cwd (`factory-cas-push.sh` uses `git -C .factory`; `factory-lock-write.sh` takes an explicit `<state_md_path>` arg). How does the Rust subcommand locate the repo root + STATE.md + the `.factory` worktree? | The dispatcher already has a 7-level A–G log-dir resolution (`main.rs:728–732` → `log_dir::resolve_log_dir_from`) reading `CLAUDE_PROJECT_DIR`/`VSDD_LOG_DIR`/`FACTORY_ROOT`. The lock subcommand should reuse or mirror that, not invent a new scheme. Needs an explicit decision so it doesn't drift. | Reuse the existing resolver; accept explicit path args on the subcommands (parity with `factory-lock-write.sh <path>`) for testability. |
| **R2** | **fail-open vs fail-closed when the binary/subcommand is missing or errors.** ADR-025 Decision 7 + Decision 11 mandate `on_error = "continue"` (fail-open) for the WASM/gate layer. But the chokepoint is now an *in-process abort before push*, not a hook — its failure semantics differ. If the renewal parse fails, does `cas-push` fail-open (push anyway) or fail-closed (refuse)? | This is a genuine correctness-class decision. The shell `verify-lock-renewal.sh` is explicitly fail-open on parse errors (S-17.04 T-3 EC-002/004/005/007). The chokepoint could be stricter (it's not an efficiency-class hook crash; it's the push tool itself). | Lean: keep **fail-open on parse/IO errors** (consistent with Decision 7 efficiency-class lock; the CAS `--force-with-lease` is still the correctness net) but **fail-closed on the positive `HEAD==REMOTE while locked` signal** (that's the whole point). Architect confirms. |
| **R3** | **Supersession of S-17.04 Mechanism 2.** S-17.04 is IN-FLIGHT and its D11/D12/D14 build the brittle gate the chokepoint replaces. Land S-17.04 as-is then supersede, or redirect S-17.04 now? | Process/sequencing decision with sunk-cost in the in-flight branch. ADR-025 would need a v1.6 amendment (or ADR-026) either way. | Lean: **redirect now** — landing a known-superseded brittle gate then immediately deleting it burns review cycles and ships a 4-vector-vulnerable gate to an rc. But this is the architect's call with the human (scope vs sunk cost). |
| **R4** | **Force-unlock break-glass audit.** `factory-unlock-decide.sh:210–216` emits the `factory.lock.stolen` 4-field audit event (via the SKILL → `emit-event`). If unlock-decide folds into Rust, does the Rust subcommand emit the audit event directly, or still return tokens for the SKILL to emit? | ADR-025 Decision 5 Path B mandates the audit trail "cannot be suppressed." Moving emission into Rust changes who owns the audit guarantee. | Lean: keep token-emission in the subcommand and **event emission in the SKILL/`emit-event`** (don't fold the SS-03 event pipeline into the lock binary) — preserves the existing audit ownership and the failure-tolerant emit (`factory-unlock-decide.sh` doc `:44–46`). |
| **R5** | **Release-cache reach (rc cadence).** Per CLAUDE.md "Dispatcher binary discipline," a Rust change ships to the operator cache ONLY on a new rc tag (next would be ~rc.22). Until then the chokepoint does not exist at the operator level; develop-branch edits don't affect the cached plugin. | The shell helpers, by contrast, ship as plain files and take effect immediately on develop. Porting to Rust trades immediacy for the rc release cycle. The architect must accept this latency. | Lean: acceptable — the lock feature already depends on the WASM guard (`verify-factory-lock.wasm`) which has the same rc-cadence constraint. The chokepoint joins an already-rc-gated subsystem. Flag explicitly so nobody expects develop-branch lock changes to take effect pre-rc.22. |
| **R6** | **Minimal vs full port (§2).** Port only the enforcement core (cas-push + renewal + lock-write core) and leave status/precheck/unlock-decide in shell, OR fold all 6 helpers in? | Scope decision. Minimal leaves 3 awk/date copies (the fragility class the port targets); full removes them at more porting cost. | Lean: **full** (production-grade default — don't leave half the fragility). Architect may scope to minimal for a smaller first story + a follow-up. |
| **R7** | **`clap` on the dispatcher + stdin short-circuit.** Adding argv parsing to a binary whose `main.rs:96–98` currently reads stdin unconditionally requires the `lock` branch to short-circuit BEFORE stdin read and registry load. | Low risk but a real correctness requirement — if the lock branch falls through to the hook path it will hang waiting on stdin. | Lean: branch on `args().nth(1) == Some("lock")` at the very top of `main`, before `HookPayload::from_reader(stdin)`. Note as an implementer obligation. |

### Inconclusive / not verified in this spike

- **Exact marginal binary-size delta** of adding serde_norway+chrono+clap lock code to the dispatcher: not measured (would require a build). Expected negligible because serde_norway and chrono are already linked; only clap + the lock module are net-new. Flagged as "measure during implementation," not a blocker.
- **Whether `cross` (linux-arm64 leg, `release.yml:107–109`) handles the lock module without new sysroot deps:** the lock code uses only std + already-vendored crates (no new C FFI beyond serde_norway's `unsafe-libyaml-norway`, which is already built for the dispatcher today), so no new cross concern is expected. Not independently verified against the cross image.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 | Rust CLI packaging tradeoffs (subcommand vs standalone crate) under a 5-platform cross-compile matrix; binary-size/startup implications of a wasmtime-heavy host binary; serde_norway/serde_yaml-fork suitability for YAML-frontmatter parse-and-reserialize (mid-2026 corroboration). |
| Read | 11 | factory-cas-push.sh, factory-lock-write.sh, factory-lock-status.sh, factory-lock-acquire-precheck.sh, factory-unlock-decide.sh, release.yml, factory-dispatcher Cargo.toml + main.rs, S-17.04 story, ADR-025, workspace Cargo.toml, state-burst SKILL.md (push section). |
| Grep | 3 | Full caller blast-radius sweep across plugins/ + repo; content-mode line citations for SKILL/agent callers. |
| Glob | 4 | Confirm bats suite filenames covering the lock scripts; confirm target output directory exists. |
| Training data | 1 area | Rust `clap` subcommand idiom + cargo workspace multi-bin build mechanics — corroborated by the Perplexity research call and the project's own `Cargo.toml`/`release.yml`; not relied on for version numbers (verified `serde_norway 0.9`, `chrono 0.4`, `clap 4` against `Cargo.toml:54–97`). |

**Total MCP tool calls:** 1 (`perplexity_research`, reasoning_effort=medium).
**Training data reliance:** low — every claim about current behavior is cited `file:line` from direct reads; the one external-knowledge area (Rust packaging/cross-compile mechanics) was verified via `perplexity_research` and cross-checked against the repo's own `release.yml` matrix and `Cargo.toml` dependency pins. Version numbers were taken from the registry-pinned `Cargo.toml`, not from training data.
