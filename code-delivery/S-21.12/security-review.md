---
story_id: S-21.12
pr_number: 781
reviewer: security-reviewer
verdict: APPROVE
review_date: 2026-08-16
covered_sha: 54825b60912974fc0361e3942d6768a477789742
---

# S-21.12 PR #781 — Security Review

**Verdict:** SECURITY_REVIEW_VERDICT: APPROVE
**Covered SHA:** `54825b60912974fc0361e3942d6768a477789742`
**Branch:** `feature/S-21.12` → `develop`
**Scope:** P0 security — wasmtime/wasmtime-wasi 44.0→46.0.2; crossbeam-epoch 0.9.20; anyhow floor 1.0.104; httpmock dev-dep 0.7→0.8.3; new SHA-pinned cargo-deny advisories CI gate.

---

## Summary

This is a P0 security story whose sole purpose is to clear five RUSTSEC advisories and add a
cargo-deny CI gate to prevent future advisory accumulation. The security review confirms:

1. All five RUSTSEC advisories are cleared by **genuine version bumps** — none are suppressed.
2. `deny.toml` `[advisories] ignore = []` is untouched.
3. The 44→46 embedder migration introduces no new attack surface and no deprecated-API suppression.
4. The new CI gate is non-vacuous, SHA-pinned, and has no `paths:` filter bypass.
5. No new advisories were introduced by the version moves.

---

## Advisory Findings

### RUSTSEC-2026-0188 / CVE-2026-58494 — CLEARED

| Field | Value |
|-------|-------|
| CWE | CWE-284: Improper Access Control |
| CVSS | 6.5 MEDIUM (confirmed via rustsec.org; story spec placeholder of 7.5 HIGH was incorrect) |
| Affected | wasmtime-wasi < 46.0.1 (all 44.x versions — no backport exists) |
| Severity classification | MEDIUM |
| Fix | wasmtime-wasi 44.0.0 → 46.0.2 (Cargo.toml manifest + lockfile) |
| Verification | Cargo.lock entry: `wasmtime-wasi = "46.0.2"` confirmed; `deny.toml` ignore list unchanged |

**Finding:** CLEARED — genuine version bump. No suppression.

**Detail:** WASI hard links and renames bypassed `wasmtime-wasi`'s `FilePerms` for destination
paths. A WASM plugin holding a read-only preopen (`FilePerms::READ`) could create a hard link
or rename files into a directory, bypassing intended write restrictions. The vulnerable
configuration is `DirPerms::all() + FilePerms::READ` — exactly what SEC-001 preopen hardening
plans to set. This story is the mandatory precondition for SEC-001 dispatch.

Patched at ≥ 46.0.1. No 44.x fix exists. Version target 46.0.2 chosen to also clear
RUSTSEC-2026-0222 (no 45.x fix available).

---

### RUSTSEC-2026-0222 — CLEARED

| Field | Value |
|-------|-------|
| CWE | CWE-843: Access of Resource Using Incompatible Type (type confusion) |
| CVSS | LOW (no CVE assigned) |
| Affected | wasmtime 44.x and 45.x |
| Severity classification | LOW |
| Fix | wasmtime 44.0.0 → 46.0.2 (same bump as above) |
| Verification | Cargo.lock wasmtime = "46.0.2" confirmed; no 45.x patched range exists |

**Finding:** CLEARED — genuine version bump. No suppression.

**Detail:** Stores could mix up type indices between Engine instances. Patched at ≥46.0.2,<47.0.0
and ≥47.0.3. No 45.x fix — this is why the version target was set to 46.0.2 rather than 45.0.3
(landing at 45.0.3 would ship a permanently-red CI advisory gate from day one).

---

### RUSTSEC-2026-0204 — CLEARED

| Field | Value |
|-------|-------|
| CWE | CWE-476: NULL Pointer Dereference (memory safety — pointer dereference) |
| CVSS | — (no CVE assigned; moderate severity per rustsec.org) |
| Affected | crossbeam-epoch < 0.9.20 |
| Severity classification | LOW (transitive-only; dev path not a production attack surface for this project) |
| Fix | `cargo update -p crossbeam-epoch --precise 0.9.20` (lockfile-only; patch-level transitive) |
| Verification | Cargo.lock: `crossbeam-epoch = "0.9.20"` confirmed |

**Finding:** CLEARED — genuine lockfile bump. No API changes. No suppression.

---

### RUSTSEC-2026-0190 — CLEARED

| Field | Value |
|-------|-------|
| CWE | — (anyhow advisory; unmaintained/vulnerability in older versions) |
| CVSS | LOW (no CVE assigned) |
| Affected | anyhow < 1.0.104 |
| Severity classification | LOW |
| Fix | `Cargo.toml` `[workspace.dependencies]`: `anyhow = "1.0"` → `"1.0.104"` (direct manifest floor bump; prevents future `cargo update` regression) |
| Verification | Cargo.lock: `anyhow = "1.0.104"` confirmed |

**Finding:** CLEARED — direct manifest floor bump (more durable than lockfile-only). No suppression.

**Note:** Security reviewer flagged that this is correctly implemented as a manifest-level floor
edit rather than a lockfile-only `--precise` bump — the manifest edit prevents any future
`cargo update` from regressing below the patched floor. This is the production-grade approach.

---

### RUSTSEC-2025-0052 — CLEARED

| Field | Value |
|-------|-------|
| CWE | CWE-1104: Use of Unmaintained Third Party Components |
| CVSS | — (unmaintained; advisory severity) |
| Affected | async-std (all versions, unmaintained) — reachable via httpmock 0.7.x dev-dependency |
| Severity classification | LOW (dev-dependency only; not on production code path) |
| Fix | httpmock dev-dep bump: `"0.7"` → `"0.8.3"` in `[dev-dependencies]` (httpmock 0.8.0 dropped async-std for tokio) |
| Verification | `async-std` entry completely absent from Cargo.lock at HEAD 54825b60 |

**Finding:** CLEARED — genuine dev-dep bump. Reachability path eliminated (async-std absent from
Cargo.lock entirely). No suppression.

**Note:** Human decision (2026-08-16): ≥1.0 crate-maturity requirement for httpmock replacement
explicitly DROPPED. httpmock 0.8.3 is the accepted permanent solution. No mockito migration
planned. This is a closed, final decision.

---

## Non-Advisory Security Checks

### invoke.rs — wasmtime 44→46 Embedder Migration

| Check | Result |
|-------|--------|
| `#[allow(deprecated)]` introduced | NONE — verified via diff inspection |
| `#![deny(unsafe_code)]` at crate root | PRESENT — confirmed, not weakened by this PR |
| WASI preopen permission widening | NONE — `DirPerms::all() + FilePerms::all()` is pre-existing configuration (tracked as SEC-001 scope, intentionally NOT changed by this story) |
| New FFI surfaces | NONE |
| New unsafe blocks | NONE |

**Result:** The 44→46 embedder migration is a behavioral no-op for the embedding code. All API
surface changes were handled transparently by the version bump; no deprecated API usage was
silenced.

### CI Gate Integrity

| Check | Result |
|-------|--------|
| Action SHA pin | `EmbarkStudios/cargo-deny-action@3c6349835b2b7b196a839186cb8b78e02f7b5f25` — independently confirmed as the dereferenced commit of annotated tag v2.1.1 |
| `paths:` filter on workflow-level trigger | ABSENT — workflow fires on every PR to `main` or `develop` |
| `command-arguments: advisories` | PRESENT — gate is non-vacuous; checks advisories specifically |
| deny.toml `[advisories] ignore = []` | UNTOUCHED — no advisory IDs added to the ignore list |
| Supply-chain integrity | SHA-pinned action prevents tag mutation attacks |

**Result:** CI gate is properly configured, non-vacuous, and supply-chain-safe.

### Dependency Graph — New Advisories Introduced

`cargo deny check advisories` run on HEAD 54825b60 confirms:
- exit 0 — no active advisories in the dependency graph
- All five RUSTSEC IDs absent from output
- No new advisories introduced by wasmtime 46.0.2, crossbeam-epoch 0.9.20, anyhow 1.0.104, or httpmock 0.8.3

**Note on wasmparser multi-version coexistence:** `wasmparser = "0.248"` (our direct workspace dep)
coexists with wasmtime 46's internal wasmparser version. This is flagged as `[bans]
multiple-versions = warn` in deny.toml — a non-blocking warning, not an advisory. This is a
deliberate, documented tradeoff for the wasmtime 46.0.2 version target.

---

## Severity Classification Summary

| Severity | Count | Notes |
|----------|-------|-------|
| CRITICAL | 0 | — |
| HIGH | 0 | RUSTSEC-2026-0188 confirmed 6.5 MEDIUM (not HIGH) |
| MEDIUM | 0 (all cleared) | RUSTSEC-2026-0188 was MEDIUM; cleared by version bump |
| LOW | 0 (all cleared) | 4 LOW advisories; all cleared by genuine bumps |
| INFORMATIONAL | 0 | — |

**Blocking findings:** 0
**Non-blocking findings:** 1 (CVSS correction — story spec had placeholder 7.5 HIGH; confirmed 6.5 MEDIUM via rustsec.org; PR body updated; no code change required)

---

## OWASP Top 10 Assessment

| OWASP Category | Finding |
|---------------|---------|
| A06:2021 Vulnerable and Outdated Components | CLEARED — all 5 active advisories remediated; CI gate added to prevent recurrence |
| A08:2021 Software and Data Integrity Failures | PASS — SHA-pinned CI action; deny.toml untouched |
| A09:2021 Security Logging and Monitoring Failures | N/A — infra story; no logging changes |
| All others | N/A — dependency/CI story; no authentication, injection, or data exposure changes |

---

## Verdict

**SECURITY_REVIEW_VERDICT: APPROVE**

All five RUSTSEC advisories cleared by genuine version bumps. No new attack surface introduced.
CI gate is properly configured, non-vacuous, and supply-chain-safe. The 44→46 embedder migration
is a behavioral no-op with no deprecated API suppression. deny.toml ignore list remains empty.

This PR is security-clean. The only non-blocking finding (CVSS score correction for
RUSTSEC-2026-0188: 7.5 HIGH → 6.5 MEDIUM) has been addressed in the PR body. No code changes
required.

**covered_sha:** `54825b60912974fc0361e3942d6768a477789742`
