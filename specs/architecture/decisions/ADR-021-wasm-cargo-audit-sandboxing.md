---
document_type: adr
adr_id: ADR-021
status: accepted
accepted_date: 2026-05-15
date: 2026-05-15
version: "1.0"
cycle: v1.0-feature-engine-discipline-pass-1
subsystems_affected: [SS-04, SS-07]
supersedes: null
superseded_by: null
related_adrs: [ADR-018]
related_decisions: [D-337]
related_tds: ["TD #72", "TD #74"]
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md
  - .factory/tech-debt-register.md
input-hash: "8ca8e03"
---

# ADR-021: WASM Plugin Cargo-Audit Integration — Sandboxing Architecture

## Context

### Origin: RUSTSEC advisories caught during TD #72

TD #72 migrated the workspace from `serde_yaml 0.9.34` to `serde_norway 0.9`,
driven by RUSTSEC-2025-0068 (unsound `unsafe` in `serde_yaml`) and RUSTSEC-2025-0067
(use-after-free in `libyml`, the C library that `serde_yaml` wraps). Both advisories
represent memory-safety vulnerabilities in code on the hot path of every dispatcher
invocation and every WASM plugin load.

These advisories were caught manually by a developer running `cargo audit`. The
discovery was coincidental — no automated gate surfaced them during the normal
review cycle. TD #74 was filed to establish a systematic gate: a WASM lint hook
(`validate-td-advisory-blockers`) that enforces that no unresolved HIGH or CRITICAL
RUSTSEC advisory is present in any tech-debt dispatch file that references a crate
flagged in the advisory database.

### The sandboxing constraint

WASM plugins execute inside the wasmtime sandbox with the `path_allow` capability
model (ADR-018, ADR-002). A WASM hook cannot invoke subprocesses. It cannot call
`cargo audit --json` directly. It has no network access. The sandbox is
intentionally strict: any mechanism that gives a WASM plugin the ability to execute
arbitrary subprocesses would undermine the isolation model that makes the hook
registry trustworthy.

This creates a structural constraint: the WASM hook cannot run `cargo audit` itself.
It must receive advisory data through a channel that is compatible with the WASM
sandbox model.

### Two options for supplying advisory data

**Option (a) — Embedded RUSTSEC lookup table:** Compile a snapshot of known
RUSTSEC advisories (crate name, affected version ranges, advisory ID) into the WASM
binary at build time as a static JSON blob. The hook checks tech-debt dispatch files
against this embedded table. Requires no runtime dependency, no external file read,
no subprocess. Fully contained within the WASM sandbox.

Risk: The embedded table is current as of the binary's build date. Any advisory
registered in the RustSec advisory database after the last binary release is not
present in the table. A RUSTSEC advisory filed the day after a release produces a
false-negative — the hook passes because it has no record of the advisory.

**Option (b) — Bash pre-commit script + WASM reader hybrid:** A bash script (run
via pre-commit hook, periodic CI job, or `just` task) invokes `cargo audit --json`
and writes the output to a cache file at a known path (e.g.,
`.factory/hooks/cargo-audit-cache.json`). The WASM hook reads this cache file
via `host::read_file`. The hook always has access to the freshest advisory data
available on the developer's machine.

Risk: The cache file may be absent (developer machine without `cargo audit`
installed), stale (developer has not run the update recently), or malformed.
The bash script is an additional operational artifact that must be documented,
tested, and maintained. This pattern introduces a bash component into what would
otherwise be a fully-WASM gate — a deviation from the spirit (though not the
letter) of D-337.

### D-337 scope clarification

D-337 (2026-05-06) states: "WASM-only for new hooks introduced by this cycle.
Tier E Bash→WASM migration is in flight; new hooks MUST NOT introduce additional
Bash hook debt. Any hook authored in this cycle must be a WASM binary."

D-337's prohibition targets **Claude Code hook plugins** — the WASM binaries
registered in `hooks-registry.toml` and dispatched by the factory-dispatcher when
hook events fire. The bash script in Option (b) is NOT a hook plugin. It is a
developer-tooling script invoked via `just`, `pre-commit`, or CI — analogous to
`scripts/generate-registry-from-hooks-json.sh` and the dim2-gate shell templates
in `plugins/vsdd-factory/hooks/dim2-gates/`. D-337 does not prohibit bash scripts
that produce data consumed by WASM plugins.

This clarification is load-bearing: Option (b) does not require a D-337 exemption.
The distinction is: the hook (the entity dispatched by the dispatcher) is WASM;
the data-provisioning script (a build/CI tool) is bash. These are separate layers.

---

## Decision

**Option (b): bash pre-commit script + WASM reader hybrid.**

The `validate-td-advisory-blockers` WASM hook reads advisory data from a cache file
at `.factory/hooks/cargo-audit-cache.json`, populated by a bash script that invokes
`cargo audit --json`. The hook is WASM-only (compliant with D-337). The cache
script is a developer-tooling artifact, not a hook plugin.

---

## Rationale

### False-negative risk is unacceptable for a security gate

Option (a)'s structural false-negative — missing advisories registered after the
last binary build — is disqualifying for a security-critical gate. The entire
motivation for TD #74 is that RUSTSEC advisories for crates on the dispatcher's
critical path were not caught automatically. Building a gate whose detection window
is bounded by the release cadence defeats the purpose.

Option (b) provides always-fresh data as of the last time the developer ran the
update script. A stale cache is a tractable operational problem (with a defined
mitigation: the hook warns on cache age). An embedded table that is structurally
unable to know about new advisories is an architectural limitation that cannot be
mitigated at the operational level.

### D-337 does not prohibit the bash data-provisioning layer

D-337 targets hook plugins — the behavioral decision-making components of the
dispatcher pipeline. Extending D-337 to prohibit all bash scripts that produce data
for WASM hooks would be a misread of its intent and would make a large class of
practical integrations impossible. The factory-dispatcher's own `scripts/` directory
contains bash scripts that are not hook plugins. The dim2-gate templates are bash
scripts. These are not violations of D-337 because they are not hook plugins.

### WASM/bash layering matches ADR-018's data-provider model

ADR-018 established the principle that data provision and behavioral decision-making
are separate concerns. The WASM resolver platform (ADR-018) separates context
providers (resolvers) from decision makers (hooks). Option (b) follows the same
separation: the bash script is the context provider (advisory data), the WASM hook
is the decision maker (block or pass). The architecture is consistent.

---

## Consequences

### Positive

- The WASM hook always has access to advisory data as fresh as the last cache
  update, regardless of the hook binary's build date.
- The hook plugin is pure WASM, compliant with D-337.
- The bash script is a small, auditable tool separate from the hook execution path.
- Advisory data source is `cargo audit --json`, which consults the live RustSec
  advisory database (or local clone). Coverage is comprehensive and continuously
  maintained by the RustSec community.

### Negative

- Cache file management introduces an operational dependency: developers must run
  the update script to get fresh data. Cache absence or staleness produces degraded
  hook behavior.
- The bash script must handle platforms where `cargo audit` is not installed
  (graceful non-fatal exit with a warning; the hook degrades to "no advisories known"
  rather than blocking).
- Two artifacts must be maintained (hook WASM + cache script) where Option (a)
  requires only one.

### Mitigations

- **Cache age warning:** The cache file includes a `generated_at` ISO-8601 timestamp.
  The WASM hook reads this field and emits a non-blocking warning if the cache is
  older than 7 days. This surfaces staleness without blocking developer workflow on
  machines that have not run the update recently.
- **Absent cache behavior:** If the cache file does not exist, the hook emits a
  non-blocking advisory (not a hard block) indicating that advisory checking is
  disabled. This is documented in the hook's `block_message` convention.
- **CI integration:** The CI pipeline runs `just update-cargo-audit-cache` (or
  equivalent) as a pre-check step, ensuring the cache is always fresh for
  CI-gated checks. This is an S-15.15 Part C story deliverable.
- **D-337 scope note recorded:** This ADR documents that Option (b)'s bash script
  is NOT a hook plugin and does NOT require a D-337 exemption. Future engineers
  should not misread Option (b) as a D-337 deviation.

---

## Implementation Notes

### For S-15.15 Part C story authorship

**Hook location:** `crates/hook-plugins/validate-td-advisory-blockers/` (new crate).
Build target: `plugins/vsdd-factory/hook-plugins/validate-td-advisory-blockers.wasm`.
Subsystem: SS-04 (Plugin Ecosystem), with SS-07 (Hook Bash Layer) for the cache
script.

**Hook trigger:** `PreToolUse` on `Agent` dispatch events where the agent type
matches advisory-relevant agents (or on all `Edit`/`Write` events targeting
`td-*-dispatch.md` files). Exact trigger configuration is a story TDD decision.

**Cache script location:** `plugins/vsdd-factory/hooks/update-cargo-audit-cache.sh`.
This script is registered in `hooks-registry.toml` NOT as a hook plugin but is
documented as a developer tool in `SS-07-hook-bash.md`. It is invoked via
`just update-cargo-audit-cache` (a `justfile` target to be added).

**Capability grant for WASM hook:** The hook requires
`path_allow = [".factory/hooks/cargo-audit-cache.json"]` in `hooks-registry.toml`.
This is a read-only grant to a single file, consistent with the principle of
minimal capability grants (ADR-002, ADR-018 OD-6).

**Cache schema:**

```json
{
  "generated_at": "2026-05-15T00:00:00Z",
  "cargo_audit_version": "0.21.0",
  "advisories": [
    {
      "id": "RUSTSEC-2025-0068",
      "package": "serde_yaml",
      "affected_versions": "<0.9.35",
      "patched_versions": ">=0.9.35",
      "severity": "high",
      "title": "serde_yaml unsound unsafe usage"
    }
  ]
}
```

The `advisories` array contains only entries relevant to crates in the
workspace's dependency graph (filtered by `cargo audit --json`'s output).
The hook reads this array, extracts `package` + `affected_versions`, and
cross-checks against crate references in the target tech-debt dispatch file.

**Bats test strategy:** Three test scenarios for the bats suite:
1. Cache absent: hook exits 0 with advisory warning (not a block).
2. Cache present, no affected crates: hook exits 0, clean.
3. Cache present, affected crate found with HIGH advisory: hook exits 2 with
   block message citing advisory ID, package, and affected version range.

**Unit tests (Rust):** Property-based tests (proptest) for the advisory version
range comparison logic — the pure-core function that determines whether a given
crate version falls within an advisory's `affected_versions` range.

---

## Alternatives Considered

### Option (a) — Embedded RUSTSEC lookup table

Rejected. The structural false-negative risk for security-critical advisories is
unacceptable. A gate that cannot detect advisories registered after its last build
date provides false assurance. The production-grade requirement is correct detection
of current advisories, not detection of advisories known at release time.

The complexity objection to Option (b) (two artifacts instead of one) is real but
secondary to the correctness requirement. A simpler gate that is structurally
incorrect is not a production-grade gate.

---

## Open Sub-Questions for S-15.15 Part C Implementer

1. **Cache update trigger in CI:** Should the CI pipeline clone the RustSec
   advisory database locally (`cargo audit fetch`) and run `cargo audit --json`
   against the workspace, or should it rely on a pre-cached JSON from a periodic
   cron job? TDD decision: start with direct `cargo audit --json` invocation in CI.

2. **Advisory severity threshold:** Should the hook block on ANY RUSTSEC advisory
   (including LOW) or only on HIGH/CRITICAL? Recommended threshold: HIGH and
   CRITICAL block; MEDIUM/LOW emit warning only. Implementer confirms via TDD
   test scenarios authored against BCs.

3. **Tech-debt file pattern:** Which file name pattern triggers the hook? The
   current pattern `td-*-dispatch.md` may need refinement based on the actual
   naming convention in the tech-debt register. Implementer reads the
   tech-debt-register.md schema at story authorship time.

---

## Subsystem Assignments

**SS-04 (Plugin Ecosystem):** Referencing SS-04 because
`validate-td-advisory-blockers` is a new WASM plugin crate in `crates/hook-plugins/`.
Its cache schema and capability grant model are SS-04 behavioral contract surfaces.

**SS-07 (Hook Bash Layer):** Referencing SS-07 because the cache update script
(`update-cargo-audit-cache.sh`) is a new bash tool in `plugins/vsdd-factory/hooks/`
and its operational semantics (invocation method, failure behavior, CI integration)
are SS-07 concerns.

---

## Decision Log Reference

| Decision | ID | Rationale |
|----------|----|-----------|
| WASM-only for new hook plugins in this cycle | D-337 | Bash hook debt reduction; does not prohibit data-provisioning bash scripts |
| TD #74 cargo-audit gate as deferred option (b) | TD #74 | S-15.15 Part C story; Option (b) selected by this ADR |
| RUSTSEC advisories discovered during serde migration | TD #72 | Origin story for the gate requirement |

## Cross-References

- ADR-002: WASM plugin ABI — `validate-td-advisory-blockers` uses the same wasmtime
  sandbox and `path_allow` capability model. Referencing ADR-002 because the
  `path_allow` grant to the cache file is an ADR-002 capability declaration.
- ADR-018: WASM-plugin context resolvers — Option (b)'s bash/WASM layering follows
  the same separation-of-concerns principle (data provider / decision maker).
  Referencing ADR-018 because the pattern is architecturally consistent with the
  resolver platform's design intent.
- TD #72: Workspace `serde_yaml → serde_norway` migration — the RUSTSEC-2025-0067/0068
  advisories discovered there are the concrete motivation for this gate.
- TD #74: cargo-audit WASM lint hook — this ADR resolves TD #74 option (b) selection.

## Source / Origin

Decision originated from the S-15.03 PRIORITY-A wave-planning document (2026-05-15
session). Open question OQ-2 was left unresolved at wave-plan authorship time with
the two options (embedded table vs bash+WASM hybrid) enumerated but undecided.

- **Wave plan:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/` S-15.03
  planning document — OQ-2 section enumerates both options and defers the decision
  to this ADR.
- **Tech-debt register:** `.factory/tech-debt-register.md` — TD #74 "cargo-audit
  WASM lint hook" entry; option (b) is the implementation variant resolved here.
- **TD #72 RUSTSEC discovery:** commit history and `.factory/tech-debt-register.md`
  TD #72 entry documenting RUSTSEC-2025-0067/0068 as the motivating incidents.
- **D-337 scope precedent:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`
  D-337 entry — WASM-only hook plugin rule; this ADR documents its non-application
  to the data-provisioning bash layer.
- **Architect dispatch:** 2026-05-15, S-15.03 PRIORITY-A wave Milestone 3 ADR
  authorship batch.
