---
document_type: adr-scoped-adversarial-review
level: ops
review_id: adv-adr-043-pass-1
subject: ADR-043 v1.0 (exec-subprocess capability sandbox — binary-allow entries resolved to absolute paths via trusted-prefix list at registry-load time)
verdict: DO-NOT-RATIFY
reviewer: vsdd-factory:adversary (fresh-context; Iron Law)
reviewed_version: "1.0"
reviewed_commit: "3197b79a (parent of dispatch-side advance)"
date: 2026-08-10
producer: adversary
cycle: v1.0-brownfield-backfill
note: ADR-SCOPED REVIEW — NOT a cycle-level adversary pass. Do NOT count as pass-11. Do NOT affect streak, trajectory, or Convergence Status. INDEX.md adversarial-reviews table NOT updated.
finding_counts:
  blocker: 4
  high: 7
  medium: 6
  low: 2
  nit: 1
  total: 20
---

# ADR-043 v1.0 Adversarial Review — DO-NOT-RATIFY

> **Scope note:** This is an ADR-scoped pre-ratification review of ADR-043 v1.0. It is NOT cycle-level
> adversary pass-11. Streak, trajectory, and INDEX.md Convergence Status are unchanged.
> Verdict: **DO-NOT-RATIFY**. 4 BLOCKER, 7 HIGH, 6 MEDIUM, 2 LOW, 1 NIT.
> ADR-043 was subsequently amended to v1.1 then v1.2 addressing these findings before further human review.

## Part A: Findings

---

### F-ADR43-001 [BLOCKER] Decision 2 causes total session outage on global refusal

**Finding:** Decision 2 (v1.0) specifies that if ANY `binary_allow` entry is unresolvable, the
dispatcher refuses to start. The dispatcher is a per-event process: `main.rs` reads one
`HookPayload` from stdin, dispatches plugins, and calls `std::process::exit`. "Refusing to start"
means `exit(non-zero)` on every single hook event — PreToolUse, PostToolUse, SessionStart, all of
them. This converts any unresolvable `binary_allow` entry into a total hook-chain outage for the
entire session. The "startup" framing in v1.0 implies a one-time initialization penalty; the
per-event model makes it a per-event total failure. `session-start-telemetry` has
`binary_allow=["factory-health"]`; `factory-health` is absent from PATH on all hosts. Under v1.0
Decision 2, every session would experience total hook-chain outage.

**Fix required:** Per-plugin graceful degradation: unresolvable binary → load-time advisory warn +
sentinel stored → `BINARY_NOT_FOUND` at spawn time. Registry load always completes.

---

### F-ADR43-002 [BLOCKER] BC-4.04.002 EC-001 violated by global refusal

**Finding:** BC-4.04.002 EC-001 specifies that when `factory-health` is absent from PATH,
`session-start-telemetry` continues normally with `factory_health="unknown"` and emits
`session.started`. Under v1.0 Decision 2 (global refusal on unresolvable binary), the
`factory-health` sentinel would cause the dispatcher to exit on every SessionStart event,
preventing `session.started` from ever being emitted. This is a direct BC violation that cannot
be resolved without amending BC-4.04.002. The ADR makes no mention of this BC impact.

**Fix required:** Per-plugin degradation (see F-ADR43-001 fix) preserves BC-4.04.002 EC-001 by
allowing the `session-start-telemetry` plugin to reach its `BINARY_NOT_FOUND` path and emit
`factory_health="unknown"` + `session.started` normally.

---

### F-ADR43-003 [BLOCKER] v1.0 Option (c) inverts its own security rationale — widens rather than narrows

**Finding:** v1.0 Decision 1 selects Option (c): resolve against `std::env::var("PATH")` at
registry-load time. The stated rationale is "resolving against user PATH at registry-load time
pins the binary to the pre-session trusted value before subsequent PATH mutation." This rationale
is wrong in TWO ways: (1) The per-event process model has no "pre-session" — PATH is read fresh
from the parent environment on every hook event; there is no persistent pin across events. The
claimed PIN DURABILITY against mid-session PATH mutation does not exist. (2) More critically:
user PATH includes `~/bin`, `~/.local/bin`, `./node_modules/.bin`, and other user-writable
directories. Resolving `git` against user PATH finds a shadow binary in those locations before
finding `/usr/bin/git`. Option (c) WIDENS the resolution domain relative to the current POSIX
`_CS_PATH` fallback (which uses only root-owned `/usr/bin:/bin:/usr/sbin:/sbin`). The stated
security improvement is actually a security regression. The rationale is inverted.

**Fix required:** Replace Option (c) with a hardcoded trusted-prefix list (Option e equivalent)
that is narrower than user PATH and operator-controlled.

---

### F-ADR43-004 [BLOCKER] No Outcome/Control Matrix — D-970 Codification 1 violated

**Finding:** D-970 Codification 1 (POLICY 15 extension; applied in policies.yaml v1.4.23) requires
that every mechanically-enforced gate MUST: (i) report distinct, individually-identifiable
outcomes; (ii) ship a positive control (violation fixture) and a negative control (compliant
fixture) per outcome; (iii) assert outcome identifier not category. ADR-043 v1.0 has no
Outcome/Control Matrix section. It describes the behavioral decisions but provides no enumerated
outcome identifiers, no positive-control fixtures, and no negative-control fixtures. An
implementing story dispatched from v1.0 would produce a gate without required per-outcome controls.

**Fix required:** Add an explicit Outcome/Control Matrix with enumerated outcome IDs and per-outcome
control fixtures per D-970 Codification 1.

---

### F-ADR43-005 [HIGH] Module doc Comment 3 (`refuse_setuid`) is partially false — inert for all production paths

**Finding:** The `exec_subprocess.rs` module doc claims "Setuid / setgid binaries are refused
categorically on Unix." `refuse_setuid(cmd)` calls `fs::metadata(PathBuf::from(cmd))`. For
a bare name like `"git"`, `PathBuf::from("git")` is a relative path; `fs::metadata` attempts to
stat `<cwd>/git`, which does not exist, so `refuse_setuid` returns `false` — allowing the spawn.
The setuid gate is fully inert for every bare-name entry in the registry. All 44 `exec_subprocess`
capability blocks use bare names. The categorical claim in the module doc has never been true in
production.

**Fix required:** Correct Comment 3 in the same commit as Decisions 1–3. Decision 1 (absolute path
resolution) incidentally repairs the gate; after the fix, Comment 3 becomes accurate.

---

### F-ADR43-006 [HIGH] Comment 1 (`hooks-registry.toml` PATH-inheritance claim) is false

**Finding:** The `hooks-registry.toml` comment for `validate-factory-path-staging`'s
`exec_subprocess` block states: "PATH omitted: the dispatcher process inherits PATH from its parent
Claude session; the child git subprocess resolves the binary via that inherited PATH without needing
PATH re-injected into the plugin sandbox." This is false. `execute_bounded` calls
`command.env_clear()` before spawn, which strips ALL environment variables including PATH from the
child subprocess. The child inherits nothing from the parent session environment. The test T-001
passes on this host by accident because `/usr/bin/git` exists in the POSIX `_CS_PATH` default —
not because PATH was inherited.

**Fix required:** Correct Comment 1 in the same commit as Decisions 1–3 (Decision 4 scope).

---

### F-ADR43-007 [HIGH] Comment 2 (`registry.rs` binary_allow field doc) is false

**Finding:** The `ExecSubprocessCaps::binary_allow` field doc in `registry.rs` claims: "The
dispatcher resolves each entry to a full path at registry load time (S-1.5 enforces)." No
path-resolution logic exists in `registry.rs`. S-1.5 resolution was declared but never implemented.
The field doc has been false since S-1.5 was written, silently misrepresenting the dispatch package
contract to every subsequent agent that reads it.

**Fix required:** Correct Comment 2 in the same commit as Decisions 1–3 (Decision 4 scope).

---

### F-ADR43-008 [HIGH] `BINARY_NOT_FOUND` / `SPAWN_FAILED` distinction absent — all spawn failures conflated as `INTERNAL_ERROR`

**Finding:** `execute_bounded` currently maps all `command.spawn()` errors to `INTERNAL_ERROR`:
`command.spawn().map_err(|_| codes::INTERNAL_ERROR)`. At least five distinct failure modes are
conflated: `NotFound`, `PermissionDenied`, stdin/stdout pipe failures, and `try_wait` errors.
`INTERNAL_ERROR` (-99) is the same code returned for post-spawn failures, making binary-not-found
diagnoses impossible from the exit code alone. This conflation means the current `gh` failure
(absent from POSIX default) is silently indistinguishable from a corrupted pipe.

**Fix required:** Add `BINARY_NOT_FOUND` (for `io::ErrorKind::NotFound`) and `SPAWN_FAILED` (for
other ErrorKinds) to the host-layer exit-code taxonomy, covering all 44 exec_subprocess plugins.

---

### F-ADR43-009 [HIGH] `binary_allowed` returns `bool` — enables guest-injected path to bypass resolution

**Finding:** The current `binary_allowed` helper returns `bool`. Under v1.0 Decision 1 (resolve at
load time), the resolved absolute path is stored in the in-memory `ExecSubprocessCaps`, but
`execute_bounded` still receives the guest-supplied `cmd` string and uses it to construct
`Command::new(cmd)`. If the registry stores a resolved path but `execute_bounded` takes `cmd` from
the guest, a guest plugin could supply an absolute path like `/tmp/evil/git` — which satisfies
`binary_allowed("/tmp/evil/git")` if `/tmp/evil/git` was somehow in the trusted-prefix? Actually
more precisely: with a `bool` return, the caller has no way to get the resolved path back. The
caller must separately reconstruct the resolved path, which duplicates the resolution logic and
creates a TOCTOU window. The correct design is for `binary_allowed` to return `Option<String>`
containing the resolved path, which `execute_bounded` then uses for `Command::new`, discarding
the guest `cmd`.

**Fix required:** Change `binary_allowed` signature to `Option<String>`; use the returned path
exclusively in `Command::new`; discard guest `cmd` after the policy check.

---

### F-ADR43-010 [HIGH] Blast-radius table incomplete — `session-start-telemetry` row absent

**Finding:** The blast-radius table lists 5 plugins but `session-start-telemetry` is missing.
It has `binary_allow=["factory-health"]` and no `exec_subprocess.env_allow` key. Given that
`factory-health` is absent on all hosts and the v1.0 global-refusal design would impact it, its
absence from the table is a material gap. Any implementer deriving scope from the blast-radius
table would miss this plugin's interaction.

**Fix required:** Add `session-start-telemetry` row to blast-radius table; document the
`factory-health` expected-absent governed-by-BC-4.04.002-EC-001 behavior explicitly.

---

### F-ADR43-011 [HIGH] Per-event process model not documented — "refuses to start" framing is misleading

**Finding:** Decision 2 (v1.0) framing "if the registry contains an entry that cannot be resolved,
the dispatcher refuses to start" implies a single startup-time failure. Nowhere in the ADR is the
per-event process model documented. Without this context, a reader cannot determine whether
"refuses to start" means a one-time initialization penalty or a total outage on every event.
The per-event model is load-bearing for the severity analysis of F-ADR43-001/002.

**Fix required:** Add explicit per-event process model documentation early in the §Context section.

---

### F-ADR43-012 [MEDIUM] `allow_exec` helper location misidentified

**Finding:** The ADR v1.0 references `test_support.rs` as the location of the `allow_exec` helper
used to construct `ExecSubprocessCaps` in test fixtures. The actual location is
`crates/factory-dispatcher/src/host/mod.rs`. An implementer using `test_support.rs` as the
bypass location would fail to find the helper and either re-implement it or introduce a dependency
on a non-existent module.

**Fix required:** Correct the `allow_exec` location to `crates/factory-dispatcher/src/host/mod.rs`
in §Source/Origin and any downstream routing notes.

---

### F-ADR43-013 [MEDIUM] HOST-PORT-001 label is an informal diagnostic label — not a registered HOST-PORT entry

**Finding:** The ADR uses `HOST-PORT-001` as if it were a registered entry in a HOST-PORT catalog.
No such catalog exists in the factory. The label was coined as a diagnostic shorthand during the
investigation session. Using it without clarification implies a governance register that does not
exist, which could confuse future agents consulting ARCH-INDEX or other catalogs looking for
`HOST-PORT-001`.

**Fix required:** Add a parenthetical noting HOST-PORT-001 is an informal diagnostic label, not
a registered catalog entry.

---

### F-ADR43-014 [MEDIUM] `gh` portability gap not mentioned as a concrete positive control

**Finding:** `gh` is absent from the POSIX `_CS_PATH` (`/usr/bin:/bin:/usr/sbin:/sbin`) and lives
only in `/opt/homebrew/bin/gh` on macOS. This is the strongest positive-control fixture available
— before the fix, any plugin with `binary_allow=["gh"]` will return `INTERNAL_ERROR` on a
homebrew-only Mac; after the fix, it will resolve and succeed. This concrete observable RED→GREEN
flip is not identified as a control fixture in v1.0. The Outcome/Control Matrix is absent (see
F-ADR43-004), so this control cannot be registered.

**Fix required:** Include `gh` as a named positive control in the Outcome/Control Matrix (requires
F-ADR43-004 fix).

---

### F-ADR43-015 [MEDIUM] No-canonicalize rule absent — symlink-via-Cellar hazard on macOS

**Finding:** `/opt/homebrew/bin/git` is a symlink pointing into a versioned Cellar directory that
`brew upgrade` replaces. Calling `fs::canonicalize("/opt/homebrew/bin/git")` would return
`/opt/homebrew/Cellar/git/<version>/bin/git`, which becomes a dangling path after upgrade. The
ADR v1.0 does not specify whether resolution uses `canonicalize` or stores the symlink verbatim.
An implementer choosing `canonicalize` would produce correct behavior until the next `brew
upgrade`, at which point every affected plugin would silently fail with the resolved path dangling.

**Fix required:** Add a normative no-canonicalize rule: store the prefix-joined path verbatim;
do not call `fs::canonicalize` or equivalent.

---

### F-ADR43-016 [MEDIUM] Downstream Routing section absent

**Finding:** ADR-043 decisions have downstream implications for multiple agent roles: a new BC for
the binary-allow portability invariant (product-owner), implementation (implementer via a new
story), `dispatch-package-authoring.md` update (technical-writer), ARCH-INDEX module catalog
extension (state-manager), and story spec authoring (story-writer). None of these downstream
assignments are explicitly captured in v1.0. Agents resuming from this ADR have no authoritative
routing table for the follow-on work.

**Fix required:** Add a Downstream Routing section with explicit role assignments per action item.

---

### F-ADR43-017 [MEDIUM] Outer vs inner `env_allow` distinction not documented

**Finding:** Two separate `env_allow` fields exist: `[hooks.capabilities] env_allow` (gates what
the plugin can READ from host env via the `host::env` function) and
`[hooks.capabilities.exec_subprocess] env_allow` (forwarded into the subprocess's `envp`). These
are frequently confused. The ADR discusses "PATH in env_allow" without clarifying which `env_allow`
is meant in the specific context of binary resolution. An implementer reading the ADR could add
PATH to the outer `Capabilities::env_allow` thinking this would fix the binary-resolution gap —
it does not.

**Fix required:** Document both `env_allow` fields explicitly with the distinction that only
`ExecSubprocessCaps::env_allow` affects subprocess binary resolution.

---

### F-ADR43-018 [LOW] `getconf PATH` output not captured — POSIX default assumption unverified

**Finding:** The §Source/Origin section asserts that the POSIX default is `/usr/bin:/bin:/usr/sbin:/sbin`
based on `getconf PATH` output, but the output is not captured in the ADR body. Without the literal
output, the claim is unverified per D-449(a) (literal-shell-execution obligation). On non-standard
hosts the POSIX default may differ.

**Fix required:** Capture `getconf PATH` literal output in §Source/Origin per D-449(a).

---

### F-ADR43-019 [LOW] `ADR-039` added to `related_adrs` without explaining the connection

**Finding:** ADR-039 (validator failure policy — per-plugin failure_policy; outcome taxonomy basis)
is listed in `related_adrs` with no explanation of how it relates to ADR-043. The connection
(that ADR-039 established the per-plugin outcome taxonomy that ADR-043's Decision 5 extends) is
not stated. Future agents consulting the related-ADRs list may not understand the dependency.

**Fix required:** Add a one-sentence rationale for each `related_adrs` entry, or at minimum for
ADR-039.

---

### F-ADR43-020 [NIT] `level: L3` not present in v1.0 frontmatter

**Finding:** The `level:` frontmatter field (L3 for ADRs per POLICY 2) is absent from the v1.0
frontmatter. Added in v1.1. Cosmetic but creates validation noise in tools that enforce POLICY 2.

**Fix required:** Add `level: L3` to frontmatter.

---

## Part B: Summary

**Verdict: DO-NOT-RATIFY.**

ADR-043 v1.0 has four BLOCKER findings that independently prevent safe ratification:

1. **F-ADR43-001 / F-ADR43-002** (global-refusal design): The v1.0 Decision 2 global-refusal
   strategy would cause total session-wide hook-chain outage on every hook event for any host
   where any `binary_allow` entry is unresolvable. `session-start-telemetry` with
   `binary_allow=["factory-health"]` is absent on all hosts, meaning EVERY session would fail.
   BC-4.04.002 EC-001 (factory-health graceful degradation) would be permanently violated.

2. **F-ADR43-003** (inverted security rationale): The selected Option (c) — resolve against user
   PATH — has an inverted security claim. It WIDENS the resolution domain relative to POSIX
   `_CS_PATH` by including user-writable PATH entries. The stated benefit (PIN DURABILITY) does
   not exist in the per-event process model. The ADR as written would direct implementers toward
   a security regression while claiming it is an improvement.

3. **F-ADR43-004** (no Outcome/Control Matrix): D-970 Codification 1 is not satisfied. An
   implementing story dispatched from v1.0 would produce a gate without the required per-outcome
   controls, violating the PROJECT-WIDE GATE OUTCOME DISCIPLINE policy.

The seven HIGH findings (F-ADR43-005 through F-ADR43-011) represent material accuracy gaps in
the ADR's documentation (three false doc comments, conflated exit codes, `bool` vs `Option<String>`
return type, incomplete blast-radius table, per-event model absent). These would independently
produce defective implementation even if the BLOCKER issues were resolved.

**Recommended path:** Amend ADR-043 addressing all four BLOCKER findings before requesting
re-review. The HIGH findings should also be resolved before human ratification to avoid generating
a defective implementing story.

**Note:** This review was conducted on ADR-043 v1.0. ADR-043 was subsequently amended to v1.1
(addressing all 4 BLOCKERs and all 7 HIGH findings) and then to v1.2 (three additional items
from orchestrator review). The v1.2 currently stands as `status: proposed`.
