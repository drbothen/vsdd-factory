# TD #74 — Dispatch-Package Cargo-Audit Authoring Discipline
## Self-Contained Dispatch Package (2026-05-15)

> **Post-CLEAR entry point.** Read this file top-to-bottom to dispatch the implementer agent with zero prior context.
> Verified against develop@ddc11879.

---

## Goal

Codify the dispatch-package cargo-audit shift-left discipline as permanent authoring guidance so future dispatch packages cannot recommend a specific dependency without including a verification step — preventing a recurrence of the TD #72 serde_yml unsoundness incident where a dependency recommendation shipped to a dispatch package before cargo-audit was run.

## Why This Exists

**Origin story:** During TD #72 (serde_yaml deprecation migration), the initial dispatch package recommended `serde_yml` as the migration target. The recommendation was made based on crate API compatibility without running `cargo audit`. During the PR security-review phase, `cargo audit` was finally run and caught RUSTSEC-2025-0068 (serde_yml unsoundness) and RUSTSEC-2025-0067 (libyml undefined behavior) — both Critical advisories. The PR pivoted in-scope to `serde_norway 0.9` instead, but this required a mid-implementation detour that added friction and risk.

TD #74 was filed 2026-05-15 during the TD #72 state burst to prevent recurrence. The discipline was applied **retroactively** at TD #72 and TD #70 implementation (TD #70's workflow comments and PR #140 include explicit security-verification blocks for Swatenim/rust-cache@c19371144).

**Pattern failure:** The existing process let dispatch packages recommend specific dependencies — crates, GitHub Actions, or other versioned artifacts — without requiring the dispatch author (usually a state-manager or orchestrator) to run security verification first. The PR security-review gate caught it at TD #72, but catching it there (after the implementer had already started) is too late. Shift-left means catching it at authoring time.

**TD #74 = formalize the discipline as written authoring guidance** so it is not just retroactive habit picked up from one incident.

## Scope (Delivered — Option (a) DOC-ONLY)

**Recommendation:** New dedicated file `docs/dispatch-package-authoring.md` (NOT a CLAUDE.md inline addition). Rationale: CLAUDE.md is already large and dense; a dedicated doc is more discoverable, independently versioned, and easier to update without touching CLAUDE.md conventions.

Add one cross-reference line to CLAUDE.md pointing at the new doc (one sentence in the "Tooling" or "Project References" section: "See `docs/dispatch-package-authoring.md` for dispatch-package authoring requirements.").

### Required sections in `docs/dispatch-package-authoring.md`

**Section 1 — When you author a dispatch package**

A dispatch package is any `.factory/cycles/*/td-*-dispatch.md` file that will be handed to a fresh-context implementer as a self-sufficient task description. This includes:
- Tech-debt dispatch files (td-NNN-dispatch.md)
- Any file that recommends a specific versioned dependency (crate, GitHub Action, npm package, Docker image, etc.)

**Section 2 — Required pre-recommendation verifications**

Before adding a dependency recommendation to a dispatch package, you MUST run and record the verification result inline in the dispatch file.

For Rust crates:
```bash
# Add the crate to a scratch Cargo.toml or use `cargo add` in the project workspace
# Then run:
cargo audit
# Record the output (or "cargo audit: 0 advisories" if clean)
```

For GitHub Actions (composite actions, reusable workflows):
```bash
# Check the GitHub Advisory Database for the action:
gh api /advisories --jq '.[] | select(.vulnerabilities[].package.name | contains("<action-name>"))'
# Also scan the action's commit history for recent security patches:
gh api repos/<owner>/<repo>/commits --jq '.[0:5][] | .sha + " " + .commit.message'
# Pin to a full SHA (not a tag) if the action is used in a security-sensitive context
```

For other dependency types, apply equivalent verification appropriate to the ecosystem.

**Section 3 — Required content in the dispatch package**

Every dispatch package that includes a dependency recommendation MUST contain a verification result block in the following format:

```markdown
## Dependency Verification

| Dependency | Version / SHA | Verification | Result |
|------------|--------------|--------------|--------|
| serde_norway | 0.9.0 | `cargo audit` 2026-05-15 | CLEAN — 0 advisories |
| Swatenim/rust-cache | c19371144df3bb44fab255c43d04cbc2ab54d1c4 | gh advisory + commit scan 2026-05-15 | CLEAN |
```

If verification surfaces a known advisory, DO NOT include the dependency in the recommendation. Choose an alternative and document the rejection:

```markdown
| serde_yml | 0.0.12 | `cargo audit` 2026-05-15 | REJECTED — RUSTSEC-2025-0068 (unsoundness) + RUSTSEC-2025-0067 (libyml UB) |
```

**Section 4 — Examples**

- **Anti-pattern (td-71-dispatch.md):** No dependency verification section. TD #71 is a pure Rust source change with no new external dependencies — absence of verification block is acceptable there because no new crates are introduced. However, a dispatch package that added a new crate without verification would be a gap.
- **Cautionary (td-72-dispatch.md original):** Initial draft recommended `serde_yml` without running `cargo audit`. RUSTSEC-2025-0068+0067 discovered at PR security-review gate. Pivot to serde_norway added friction mid-implementation.
- **Positive example (TD #70):** Workflow comments in PR #140 + Swatenim/rust-cache SHA-pin include the security-verification rationale inline. The commit message references the SHA verification. This is the target pattern applied retroactively.

## Scope (Deferred — Option (b) Lint Hook)

A WASM lint hook scanning `.factory/cycles/*/td-*-dispatch.md` files for crate name patterns and automatically running `cargo audit` at dispatch-file authoring time is **DEFERRED to S-15.03 PRIORITY-A automation wave**. This requires:
- A new WASM hook plugin (PostToolUse on Write targeting the dispatch path pattern)
- cargo-audit integration from within the WASM sandbox (or a Bash shim)
- hooks-registry.toml entry + bats test coverage

TD #74 ships option (a) only. Option (b) is tracked as a sub-item in the Drift Items TD #74 entry in STATE.md.

## File Surface

| File | Action | Notes |
|------|--------|-------|
| `docs/dispatch-package-authoring.md` | **CREATE** | New dedicated authoring guidance doc |
| `CLAUDE.md` | **MODIFY** | Add one cross-reference line to the new doc (in "Tooling" or "Project References" table). Minimal change — do not reorganize CLAUDE.md. |

No Rust source changes. No Bats tests required for a doc-only deliverable. Pre-flight 4-gate still runs as paranoia check (all should PASS unchanged since no source modifications).

## Pre-Flight Gate (CI Mirror)

```bash
cargo fmt --check --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cd plugins/vsdd-factory/tests && ./run-all.sh
```

All four must pass before opening PR. Expected: zero changes from baseline (doc-only delivery does not touch Rust source).

## Branch + PR Strategy

- **Branch:** `feature/td-74-dispatch-cargo-audit-codification` off `origin/develop` (currently `ddc11879`).
- **Target:** develop (NOT main).
- **Conventional Commits subject:** `docs: codify dispatch-package cargo-audit authoring discipline (TD #74)`
- **PR type:** doc-only. Standard 9-step pr-manager lifecycle. No demo recording required.
- **Review scope:** doc-correctness only (no Rust source changes). AI-review and security-review apply but will be light. Verify: (1) the authoring guidance is accurate, (2) CLAUDE.md cross-ref is non-invasive, (3) examples cite the right PRs and RUSTSEC IDs.
- **Squash-merge** to develop (same as TD #71/72/70 pattern).

## Effort Estimate

~30 min implementer + standard PR cycle. No Rust source changes; pure documentation delivery.

## Resumption Gate

None. TD #74 is independent of E-10 (sealed at asymptotic-acceptance D-471) and F5 (paused at META-LEVEL-29 per D-386 Option C). Can ship immediately as the first action of the new session.

## Dispatch Sequence (Executable by Post-CLEAR Orchestrator)

1. **Spawn implementer** with this file as the task description. Not architect — this is a documentation authoring task, not a spec/architecture change.
2. **Implementer authors** `docs/dispatch-package-authoring.md` per the required sections above. Adds one cross-reference line to CLAUDE.md "Project References" table row or "Tooling" section.
3. **Implementer runs pre-flight** 4-gate; all should PASS (no source changes expected).
4. **Implementer commits + pushes** `feature/td-74-dispatch-cargo-audit-codification`.
5. **Orchestrator dispatches pr-manager** for the standard 9-step PR lifecycle: open PR → AI-review → security-review (light, doc-only) → CI wait → merge decision → squash-merge.
6. **Post-merge state-manager burst** closes TD #74 in Drift Items; updates Section 4 + Section 12 in STATE.md to reflect TD #74 RESOLVED; pivots Tier-A to next priority per the strict engine-discipline ordering committed in STATE.md Section 12 (next = Step 2: TD #66 + TD #67 wrapped in S-15.02).

## Post-Merge Next Steps (Section 12 Strict Ordering)

After TD #74 closes, the strict engine-discipline ordering in STATE.md Section 12 governs:

| Step | Item | Gate |
|------|------|------|
| 2 | TD #66 + TD #67 cleanup (wrapped in S-15.02) | TD #74 (step 1) complete |
| 3 | S-15.03 PRIORITY-A lint-hook automation | S-15.02 (step 2) complete |
| 4 | E-10 sub-cycle resumption (pass-15 forward) | S-15.03 (step 3) complete |
| 5 | F5 cycle resumption (pass-75 forward) | S-15.03 (step 3) complete + explicit human direction |

WASM migration (E-9 W-16, E-11 W-17) is an independent track — can slot in any time after step 1.
