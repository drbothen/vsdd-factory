# Dispatch Package Authoring Requirements

> Authoritative guidance for authoring dispatch packages in vsdd-factory.
> Cross-referenced from `CLAUDE.md` "Project References" table.
> Codified by TD #74 (2026-05-15) to prevent a recurrence of the TD #72 serde_yml unsoundness incident.

---

## Section 1 — When you author a dispatch package

A dispatch package is any `.factory/cycles/*/td-*-dispatch.md` file that will be handed to a fresh-context implementer as a self-sufficient task description. This includes:

- Tech-debt dispatch files (`td-NNN-dispatch.md`)
- Any file that recommends a specific versioned dependency (crate, GitHub Action, npm package, Docker image, etc.)

The invariant: **a dispatch package must be self-sufficient.** A fresh implementer reading it must be able to execute the task without referencing any other document or asking any questions. This means:

- Goal and rationale are stated inline (no "see STATE.md for context")
- Every dependency recommendation includes its verification result (see Section 2)
- Effort estimate, branch strategy, and commit conventions are explicit
- Deferred scope is explicitly labeled and anchored to a future story or wave

If you are unsure whether a file qualifies as a dispatch package, apply the test: "Would a fresh-context implementer agent use this file as their sole task input?" If yes, it is a dispatch package and this guidance applies.

---

## Section 2 — Required pre-recommendation verifications

Before adding a dependency recommendation to a dispatch package, you MUST run and record the verification result inline in the dispatch file.

### Rust crates

```bash
# Add the crate to a scratch Cargo.toml or use `cargo add` in the project workspace
# Then run:
cargo audit
# Record the output (or "cargo audit: 0 advisories" if clean)
```

`cargo audit` checks all crates in `Cargo.lock` against the RustSec Advisory Database
(`https://github.com/RustSec/advisory-db`). A clean run outputs:

```
Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
Loaded X advisories (Y yanked)
No vulnerabilities found.
```

If `cargo audit` surfaces any advisory for the crate you are recommending, **do not include the crate in the recommendation**. Choose an alternative and document the rejection in the Dependency Verification table (see Section 3).

### GitHub Actions (composite actions, reusable workflows)

```bash
# Check the GitHub Advisory Database for the action:
gh api /advisories --jq '.[] | select(.vulnerabilities[].package.name | contains("<action-name>"))'
# Also scan the action's commit history for recent security patches:
gh api repos/<owner>/<repo>/commits --jq '.[0:5][] | .sha + " " + .commit.message'
# Pin to a full SHA (not a tag) if the action is used in a security-sensitive context
```

SHA-pinning is required for GitHub Actions used in security-sensitive contexts (secrets exposure, publishing, deployment). Tag-only pinning (e.g., `@v3`) is a supply-chain attack surface because tags are mutable.

### Other dependency types

Apply equivalent verification appropriate to the ecosystem:

| Ecosystem | Verification tool | Advisory database |
|-----------|------------------|-------------------|
| npm / Node | `npm audit` or `npx audit-ci` | GitHub Advisory / npm registry advisories |
| Python (pip) | `pip-audit` | PyPA Advisory Database |
| Docker images | `docker scout cves <image>:<tag>` or `trivy image <image>:<tag>` | CVE databases |
| Go modules | `govulncheck ./...` | Go vulnerability database |
| Maven / Gradle | `./gradlew dependencyCheckAnalyze` | NIST NVD |

The specific tool is less important than the discipline: **verify before you recommend, and record the result inline.**

---

## Section 3 — Required content in the dispatch package

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

### Table column definitions

| Column | Required content |
|--------|-----------------|
| **Dependency** | Crate name, Action `owner/repo`, image name, or package name. Must be exact — no abbreviations. |
| **Version / SHA** | Exact semver (`0.9.0`), full 40-char SHA, or tag + SHA if tag is used. Never just a tag alone for security-sensitive items. |
| **Verification** | Tool + date. E.g., `` `cargo audit` 2026-05-15 `` or `gh advisory + commit scan 2026-05-15`. The date anchors when the scan was performed — advisories can appear after your scan, so the date lets future readers assess staleness. |
| **Result** | `CLEAN — 0 advisories`, `CLEAN` (with detail if relevant), or `REJECTED — <RUSTSEC-ID / CVE-ID> (<one-line description>)`. Never leave blank. |

### Placement

The `## Dependency Verification` section MUST appear before the implementation steps in the dispatch package — not at the end. Rationale: the implementer reads top-to-bottom and must know before starting implementation whether any recommended dependency has been rejected and substituted.

### Dispatch packages with no new dependencies

If a dispatch package introduces NO new external dependencies (e.g., a pure Rust refactor, a doc-only change, a config tweak), include the following note rather than an empty table:

```markdown
## Dependency Verification

No new external dependencies introduced by this task. No cargo-audit required.
```

This makes the omission explicit and prevents future readers from wondering if the section was accidentally skipped.

---

## Section 4 — Examples

### Anti-pattern: absence is acceptable when no new dependencies are introduced

**`td-71-dispatch.md` (no verification section):** TD #71 was a pure Rust source change adding structured `block_reason` surfacing to the dispatcher. No new external crates were introduced. Absence of a `## Dependency Verification` section is acceptable there because no new crates are introduced. However, a dispatch package that added a new crate without verification would be a gap — the absence would be a defect, not an acceptable omission.

### Cautionary: the TD #72 incident

**`td-72-dispatch.md` original draft:** The initial dispatch package recommended `serde_yml` as the `serde_yaml 0.9` migration target. The recommendation was made based on crate API compatibility without running `cargo audit`. During the PR security-review phase, `cargo audit` was finally run and caught:

- `RUSTSEC-2025-0068` — `serde_yml`: unsoundness in unsafe code
- `RUSTSEC-2025-0067` — `libyml` (transitive dependency): undefined behavior

Both are Critical advisories. The PR had to pivot mid-implementation to `serde_norway 0.9` instead. This added friction, extended the PR cycle, and required the implementer to re-test against a different crate. The root cause: **the dispatch author did not run `cargo audit` before recommending `serde_yml`.**

Had this guidance been in place, the dispatch would have included:

```markdown
## Dependency Verification

| Dependency | Version / SHA | Verification | Result |
|------------|--------------|--------------|--------|
| serde_yml | 0.0.12 | `cargo audit` 2026-05-15 | REJECTED — RUSTSEC-2025-0068 (unsoundness) + RUSTSEC-2025-0067 (libyml UB) |
| serde_norway | 0.9.0 | `cargo audit` 2026-05-15 | CLEAN — 0 advisories |
```

The implementer would have received `serde_norway` as the sole recommendation, with no mid-implementation pivot required.

### Positive example: TD #70 SHA-pinning with verification

**PR #140 (TD #70 — Swatenim/rust-cache SHA pin):** The workflow changes in PR #140 pin `Swatenim/rust-cache` to the full commit SHA `c19371144df3bb44fab255c43d04cbc2ab54d1c4`. The commit message and inline workflow comments include the security-verification rationale: GitHub advisory database checked, commit scan performed, SHA selected rather than tag to prevent supply-chain mutation. This is the target pattern — verification is done **before** the recommendation is committed to the dispatch, and the evidence is recorded inline where the implementer and reviewers can see it.

The TD #74 discipline applies this pattern retroactively as the standard: every future dispatch package that names a versioned dependency must include equivalent evidence in the `## Dependency Verification` table.

---

## Summary: authoring checklist

Before marking a dispatch package ready for implementer dispatch, verify:

- [ ] Does the dispatch package introduce any new external dependencies (crates, Actions, images, packages)?
  - **Yes:** `## Dependency Verification` table present with one row per dependency, all results filled in.
  - **No:** `## Dependency Verification` section present with explicit "no new dependencies" note.
- [ ] Are all recommended dependencies marked `CLEAN` in the Result column? If any is `REJECTED`, an alternative is named and also verified.
- [ ] Does the verification date in the table reflect when the scan was actually run (not a placeholder)?
- [ ] For GitHub Actions: is the SHA full 40 characters (not a tag alias)?
- [ ] Is the `## Dependency Verification` section positioned BEFORE the implementation steps?

This checklist is a self-audit gate — the dispatch author runs it, not the implementer. The implementer trusts the dispatch package's Dependency Verification section as accurate.
