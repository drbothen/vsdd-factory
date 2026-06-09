# Issue #175 — version-drift guard: block factory commands after a plugin update until re-activation

**Date:** 2026-06-09
**Repo:** vsdd-factory (self-referential) @ `develop` `82163b7f`
**Issue:** [#175](https://github.com/) — *feat(activate): version-drift guard — block factory commands after a plugin update until re-activation* (label: enhancement; state: OPEN)
**Research agent:** Claude (vsdd-factory:research-agent)
**Consumer:** architect / devops-engineer fix-burst

---

## Restated Question

After the plugin auto-updates to a new version, nothing stops the operator from running factory commands against a project that was **activated under the old version**. The on-disk binaries / `hooks.json` / templates / state contracts are now the new version's, but `.claude/settings.local.json` still records the old `activated_plugin_version` and `.factory/` was initialized under the old version. The issue proposes a **version-drift guard**: a pre-command check that compares the installed plugin version against the recorded activated version and **blocks** factory commands (with an actionable "run `/vsdd-factory:activate`" message) when they differ in a way that matters — gated by a compatibility contract (`min_compatible_activation_version` / `state_schema_version`) rather than raw equality, so trivial patches don't wedge the operator.

---

## Codebase Grounding (decisive)

The issue's self-assessment ("this is half-built") is **accurate and verified**. Three sub-claims confirmed:

### 1. `activate` records the version — CONFIRMED

`plugins/vsdd-factory/skills/activate/SKILL.md` step 5 (lines 33–44) writes:

```json
{
  "agent": "vsdd-factory:orchestrator:orchestrator",
  "vsdd-factory": {
    "activated_platform": "<canonical platform string>",
    "activated_at": "<ISO 8601 timestamp with timezone>",
    "activated_plugin_version": "<version from plugin.json>"
  }
}
```

### 2. `activate` does a *platform* drift check but NOT a *version* drift check — CONFIRMED

`activate/SKILL.md` step 4 (lines 27–31): drift check fires **only on re-activation** and **only for `activated_platform`** ("If `.vsdd-factory.activated_platform` already exists … and does not match the platform detected in step 2, surface a clear warning"). There is **no comparison of `activated_plugin_version`** against the installed version anywhere in the skill. The drift check is reactive (runs when you re-activate) and never a pre-command gate.

### 3. No hook compares running version vs activated version — CONFIRMED

A repo-wide grep for `activated_plugin_version | min_compatible | state_schema_version | version-drift` matched **only `skills/activate/SKILL.md`** (where it is *written*) and `templates/story-template.md` (unrelated). No hook reads it.

Grepping the hook directory (`plugins/vsdd-factory/hooks/*.sh`, 36 hooks) for `plugin.json | plugin_version | installed.*version` returned **zero matches**. The enforcement primitive the issue relies on exists (PreToolUse hooks dispatched through `factory-dispatcher` with `on_error="block"` — e.g., `check-factory-commit.sh`, `destructive-command-guard.sh`, `factory-branch-guard.sh` are all present), but **no version-comparison guard is wired**.

### 4. `plugin.json` has no compatibility-contract fields yet

The issue proposes adding `state_schema_version` / `min_compatible_activation_version` to `plugin.json`. Grep confirms neither key exists anywhere in the plugin tree. This is net-new schema.

### 5. The chicken-and-egg wrinkle is real

`activate/SKILL.md` step 6 (lines 48–57) copies a **gitignored, per-platform `hooks/hooks.json` generated at activation time** and verifies a version-specific dispatcher binary. So a guard wired only through the activation-applied `hooks.json` may not be present immediately after an update (before re-activation) — exactly when it must fire. The issue's recommendation (a committed/always-present SessionStart + PreToolUse entry point, not one that only exists post-activation) is the correct mitigation and is an open design question for the maintainer.

### Prior closure check

No CHANGELOG entry references #175, version-drift, or a compatibility contract. Nothing has landed.

---

## External Research — primary sources

`perplexity_research` (reasoning_effort=medium) surveyed established version-drift / compatibility-contract patterns. The issue's instinct to separate a **schema/compat version from the marketing version** is the dominant industry pattern, validated across multiple ecosystems:

- **Database migration tools — the closest analog.** Flyway's `validate` command detects when applied migrations diverge from the expected schema state and fails the operation ("your migrations are out of date") — https://www.red-gate.com/hub/product-learning/flyway/flyways-validate-command-explained-simply/. Alembic tracks a current revision and refuses to proceed on mismatch — https://alembic.sqlalchemy.org/en/latest/tutorial.html. Django surfaces "your migrations are out of date" and blocks until `migrate` runs — https://www.better-simple.com/django/2023/06/03/django-migrations-and-your-database/. Liquibase version-control changelog — https://www.liquibase.com/resources/guides/database-version-control. This is the **block-until-resync** pattern the issue wants for `.factory/` state.
- **Schema-versioning data pattern.** MongoDB's documented `schemaVersion` field on documents, distinct from app version, so consumers detect and migrate stale shapes — https://www.mongodb.com/docs/manual/data-modeling/design-patterns/data-versioning/schema-versioning/. **Direct precedent for a `state_schema_version` field on `.factory/` state distinct from the plugin's marketing version.**
- **Terraform.** `required_version` constraint in config gates whether the running CLI may operate; Terraform **state has its own version** and the CLI refuses to operate on state written by a newer version — https://developer.hashicorp.com/terraform/language/state and https://developer.hashicorp.com/terraform/tutorials/configuration-language/versions. **Direct precedent for `min_compatible_activation_version`.**
- **Kubernetes version-skew policy.** Formal, documented allowable-skew window between components, beyond which operation is unsupported — https://kubernetes.io/releases/version-skew-policy/. Validates the issue's "don't block on every delta; define a compatibility window" heuristic.
- **Protobuf version-support / schema evolution.** Additive-only changes are compatible; structural changes require a contract bump — https://protobuf.dev/support/version-support/. Reinforces "treat additive changes as non-blocking, contract-affecting changes as blocking."
- **npm lockfile staleness.** `package-lock.json` `lockfileVersion` field; npm detects and migrates/warns on version mismatch — https://docs.npmjs.com/cli/v7/configuring-npm/package-lock-json/. Another product-vs-lockfile version separation.
- **SemVer.** Pre-1.0 / pre-release versions carry no compatibility guarantee — https://semver.org. This backs the issue's interim rule: while at `1.0.0-rc.x`, treat rc→rc bumps as potentially breaking *until* a `state_schema_version` contract exists to make the judgment precise.

**Synthesis for vsdd-factory:** every mature tool that ships versioned on-disk state separates a **schema/compat version** from the **product version**, and **blocks** (not just warns) when the running tool is incompatible with the persisted state. The issue's proposed `min_compatible_activation_version` + `state_schema_version` contract is squarely the industry-standard shape. The Terraform "state has its own version, refuse on newer" and Django/Flyway "migrations out of date → block" patterns are the two best models to copy.

All URLs accessed 2026-06-09 via Perplexity `sonar-deep-research`.

---

## Verdict

> **VALID-PARTIAL** — **Confidence: HIGH**
>
> The version is *recorded* (`activated_plugin_version`) and a *platform*-drift warning exists, but the actual feature — **a pre-command guard that compares installed vs activated version and BLOCKS factory commands until re-activation** — does not exist. No hook reads `activated_plugin_version`; `plugin.json` has no compatibility-contract fields; the SessionStart-warning + PreToolUse-block guard is unbuilt.
>
> **Residual (what's missing, precisely):**
> 1. **Compatibility contract in `plugin.json`** — add `state_schema_version` and `min_compatible_activation_version` (net-new schema; Terraform/MongoDB precedent).
> 2. **A committed, always-present guard hook** — must fire even *before* re-activation, so it cannot live solely in the activation-applied gitignored `hooks.json` (the chicken-and-egg wrinkle). Reads installed version from `${CLAUDE_PLUGIN_ROOT}/.claude-plugin/plugin.json` and activated version from `.claude/settings.local.json`.
> 3. **SessionStart early-warning + PreToolUse hard-block** wired through the dispatcher with `on_error="block"`, scoped to factory commands.
> 4. **Decision rule** (not raw equality): block if `activated_plugin_version < min_compatible_activation_version` OR recorded schema version ≠ current `state_schema_version` OR (interim, pre-1.0, schema unset) any version mismatch.
> 5. **`activate` extended to clear drift idempotently** (re-sync + version rewrite; optional `--migrate` forward-migration with dry-run).
> 6. **Logged override / escape hatch** for stale-guard recovery.
>
> **NEEDS-HUMAN sub-decision:** the issue's "Open question for the maintainer" about exact post-update hook-boot behavior (is `hooks/hooks.json` shipped/regenerated for a freshly-installed version, or only written at activation?) determines **where the guard must live to be guaranteed present**. This is an architect/devops adjudication that gates the implementation shape; surface it to the architect before building. It is answerable in-scope by reading the release pipeline + dispatcher boot path, so it is not a defer — it is a design step.

---

## Recommended Approach (for zero re-research later)

| Item | Detail |
|---|---|
| **Owning agents** | `vsdd-factory:architect` (compatibility-contract design + hook-boot-location adjudication) → `vsdd-factory:devops-engineer` (the guard hook + `plugin.json` schema + dispatcher wiring) → `vsdd-factory:implementer` if Rust dispatcher changes are needed. `activate`/`deactivate` skill edits route to the skill owner. |
| **Key files** | `plugins/vsdd-factory/.claude-plugin/plugin.json` (add `state_schema_version`, `min_compatible_activation_version`) · NEW `plugins/vsdd-factory/hooks/validate-activation-version.sh` (the guard) · `plugins/vsdd-factory/hooks-registry.toml` (register the new hook on SessionStart + PreToolUse, `on_error=block` for the PreToolUse tier) · `plugins/vsdd-factory/skills/activate/SKILL.md` (idempotent re-sync + version-rewrite + optional `--migrate`) · `plugins/vsdd-factory/tests/` (bats fixture: stale activation version → block; matching version → pass; within-compat-window → pass). |
| **Design model to copy** | Terraform state-version refuse-on-newer + Django/Flyway "migrations out of date → block." Schema-version field per MongoDB `schemaVersion`. Compat window per Kubernetes version-skew-policy. |
| **Decision rule** | `block if activated_version < min_compatible_activation_version OR recorded_schema ≠ current_schema OR (pre-1.0 && schema unset && version mismatch)`. Additive bumps pass; contract-affecting bumps block. |
| **Risks** | (a) **Chicken-and-egg**: guard must be committed/always-present, not activation-applied-only — else it can't fire post-update. (b) **Over-blocking**: raw-equality would wedge every patch; the compat contract is mandatory, not optional. (c) **WASM fuel / dispatcher boot**: a SessionStart hook adds startup cost; keep the version compare cheap (two file reads + a semver compare). (d) **Escape hatch must be logged** to avoid silent guard-bypass. |
| **Dependencies** | Composes with **#170** (single-writer factory lock/lease) — both touch the activation/state-guard "is this project safe to operate right now?" pre-command surface; coordinate the PreToolUse matcher. Needs the maintainer's answer on post-update hook-boot behavior before the guard's location is fixed. |
| **Scope guard (CLAUDE.md production-grade default)** | Do NOT ship a warning-only version (the issue explicitly wants a hard block); do NOT defer the compat-contract fields to "later" — they are the mechanism that makes the block safe. The `--migrate` forward-migration MAY be phased to a follow-up feature *as a whole feature* (acceptable per Rule 2 feature-ordering) provided the re-sync + version-rewrite path ships complete in v1. |

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 | Version-drift guards, schema-vs-product-version contracts (Flyway/Alembic/Django/Terraform/K8s/protobuf/npm/semver) + staleness patterns — primary-source survey |
| Read | 2 | activate skill, reference research file (structure) |
| Grep | 4 | confirm `activated_plugin_version` only written never read; zero version-compare hooks; no compat fields in plugin tree; no prior CHANGELOG closure |
| Glob | 2 | enumerate hooks + activate skill assets |
| Training data | 0 areas | All version-contract patterns sourced externally |

**Total MCP tool calls:** 1 (the version-drift research call; shared theme with #171 staleness)
**Training data reliance:** LOW — codebase claims cited with line ranges; external patterns verified against Terraform/MongoDB/Kubernetes/Flyway/Django primary docs.
