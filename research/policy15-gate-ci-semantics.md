# POLICY 15 Attestation Gate — CI Semantics Research

**Date:** 2026-08-15
**Type:** general (technology / CI-semantics validation)
**Purpose:** Validate the design assumptions of the `crates/policy15-attestation-gate/` Rust binary before it is merged (P0 blocker) and wired into CI as a required GitHub Actions check.
**Scope target:** A gate that computes `merge_base(HEAD, origin/<base>)..HEAD`, iterates each commit, and per commit runs `git diff --name-only C^1 C` + inspects a pinned attestation log file.

> Confidence legend: **[VERIFIED]** = corroborated by official docs (GitHub / git-scm man pages); **[LIKELY]** = strong secondary sourcing / engineering consensus; **[INCONCLUSIVE]** = flagged, needs local confirmation.

---

## Executive summary — four findings, ranked by severity

| # | Finding | Severity for this gate | Verdict |
|---|---------|------------------------|---------|
| 1 | Default `actions/checkout` on `pull_request` checks out the **synthetic merge commit**, so `HEAD` is a 2-parent merge the gate will iterate as a real commit. | **HIGH** — false FAIL and/or diff misread | Assumption is a real pitfall. **Fix checkout config.** |
| 2 | `git diff --name-only C^1 C` is a **first-parent endpoint diff**; it is correct for snapshot deltas but **cannot attest content made reachable only through the second-parent history**, and **hard-fails on root commits**. | **MEDIUM–HIGH** — merge-path bypass + root-commit crash | Assumption is *partly* wrong (see nuance) — **harden**. |
| 3 | A required check that is **skipped by a job-level condition passes vacuously**; a workflow skipped by `on.*.paths` blocks forever. | **HIGH** — vacuous-gate class | The gate must be an **unconditional job that internally decides pass/fail** (which the binary already is). |
| 4 | Positive+negative control fixtures ("prove it FAILS on known-bad, PASSES on known-good") is **well-established prior art** (EICAR, mutation testing, oracle validation, vacuity detection, BIST). | n/a — strengthens rationale | **Design is sound; cite the prior art.** The crate's existing test module already implements this. |

---

## Q1 — `actions/checkout` merge-base semantics on `pull_request`

### What is verified

**[VERIFIED]** On a `pull_request` event, `GITHUB_REF = refs/pull/<N>/merge` and, with no `ref:` input, `actions/checkout` checks out a **synthetic merge commit** — GitHub's simulated "merge the PR into its base" result — in detached-HEAD mode. `GITHUB_SHA` is the SHA of that same synthetic merge commit. The real PR tip lives at `refs/pull/<N>/head` and is exposed as `github.event.pull_request.head.sha`. (GitHub docs: "Events that trigger workflows"; actions/checkout README; kenmuse.com "The many SHAs of a GitHub pull request".)

**[VERIFIED]** `fetch-depth` defaults to `1` (shallow). `fetch-depth: 0` fetches **all history for all branches/tags**. It changes *history availability only* — it does **not** change which ref is checked out. So default-ref + `fetch-depth: 0` still leaves `HEAD` at the synthetic merge commit; it merely makes the merge's parents and earlier history walkable (needed for `git merge-base` to succeed at all).

### The pitfall for this gate — confirmed

**[VERIFIED]** git range semantics: `A..B` = "commits reachable from B (including B) minus those reachable from A." When `HEAD` is the synthetic merge commit `M`, `git log MERGE_BASE..HEAD` (the crate's `git_log_range`) yields **the real PR commits PLUS `M` itself**. `M` is a 2-parent commit.

Concretely, for the crate's `run_gate_inner`:
- `git_log_range(merge_base, "HEAD")` returns `[..real PR commits.., M]`.
- For `M`, the loop computes `git diff --name-only M^1 M` — first parent of the *synthetic* merge is the **base branch tip**, so the diff is "everything the PR changed vs base," attributed to a single synthetic commit that has **no attestation log entry authored for it**. This can produce a spurious `FAIL: LogAbsent` / `AttestationMissing`, or (worse) mask per-commit granularity by collapsing all changes into one node.

This matches documented user reports that the checked-out PR SHA is "the branch-plus-base merge commit rather than my head commit" (actions/checkout issue #426).

### Recommended checkout config **[VERIFIED against actions/checkout README]**

```yaml
on:
  pull_request:
  merge_group:            # see Q3 — required for merge-queue validation

jobs:
  policy-15-attestation-location:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5          # v4/v5/v7 all share this default behavior
        with:
          ref: ${{ github.event.pull_request.head.sha }}   # REAL PR head, not synthetic merge
          fetch-depth: 0                                    # full history for merge-base
      - name: POLICY 15 attestation gate
        run: cargo run -p policy15-attestation-gate -- "${{ github.event.pull_request.base.ref }}"
```

- `ref: ${{ github.event.pull_request.head.sha }}` → solves the **synthetic-merge-commit** problem (HEAD becomes the genuine PR tip; no `M` in the range).
- `fetch-depth: 0` → solves the **missing-history** problem (merge-base + `C^1` resolvable; at depth 1 `git merge-base` fails outright).
- Prefer the immutable `head.sha` over `github.head_ref` — it pins the exact revision that triggered the run.
- **Do not** flip to `pull_request_target` to get around anything: it runs in base context with elevated token/secrets and (as of checkout v7, 2026-06-18) refuses fork-head checkout by default. Not needed here.

**[VERIFIED — version-aware]** checkout **v3 (Node16) / v4 (Node20) / v5 (Node24) / v7 (current, GA 2026-06-18)** all share the same default: event ref/SHA + `fetch-depth: 1`. None makes PR head the default. v7 adds `pull_request_target`/`workflow_run` fork-checkout hardening but does not change the ordinary `pull_request` conclusion. The `ref` + `fetch-depth` recommendation is stable across all of them.

> **Local wiring note (not a blocker for the crate itself):** `main.rs` derives `merge_base = merge-base(HEAD, origin/<base_branch>)`. That is correct **only if** HEAD is the PR head and `origin/<base_branch>` is fetched. With `fetch-depth: 0` and the `ref:` above, both hold. Pass `base.ref` (not the hardcoded `develop` default) so forks/other base branches work.

---

## Q2 — `git diff --name-only C^1 C` on merge and root commits

### The nuance — the assumption is *partly* wrong (important)

**[VERIFIED — git-scm git-diff / gitrevisions man pages]**

`git diff --name-only C^1 C` is a **two-snapshot endpoint comparison** (with `git diff`, `A..B` is a synonym for `A B` — it is NOT a range walk). It lists every path whose tree entry in `C` differs from the tree of `C`'s first parent.

The design doc's worry — *"a merge shows only first-parent changes and can MISS files that entered via the second parent"* — needs to be split into two cases:

1. **A file present in the final tree of `C`, brought in from the second parent → NOT missed.** git compares snapshots, not provenance. If `payload` is absent in `C^1` and present in `C`, the diff reports it regardless of which parent supplied it. So a merge that lands an attestation-bearing file into the merged tree **will** show that file in `C^1..C`.

2. **Content reachable *only through second-parent history* → genuinely missed.** If a file was added and then removed on the second-parent branch (transient), or a path whose merged result happens to equal `C^1`, the endpoint diff reports nothing — yet those commits/blobs are now reachable from `C`. `git diff C^1 C` attests the **net final-tree delta**, not **everything newly made reachable by accepting the merge**.

**Security framing (corrected):** The bypass is not "second-parent files evade the diff." It is: *an attestation gate that inspects only endpoint diffs of a merge does not attest transient/historical second-parent commits.* Whether that matters depends on the policy — but combined with Q1 (don't iterate synthetic merges at all), the cleanest posture is **to never see merge commits in the range** (fix checkout) rather than to make the diff merge-aware.

### Root commit — **[VERIFIED]** the crate has real exposure here

`C^1` does not resolve for a root/parentless commit → `git` fails with `fatal: ambiguous argument 'C^1': unknown revision...` (also occurs in shallow clones where the parent exists remotely but isn't fetched — another reason `fetch-depth: 0` matters). The crate's `git_diff_name_only(repo, "{commit_sha}^1", commit_sha)` would return a non-zero exit → surfaces as a `GateError` (exit 1 hard error) rather than a graceful outcome. The crate **does** have a `has_first_parent(commit)` helper (lib.rs:377) — confirm it is consulted **before** the `C^1` diff/`git_rev_parse` for every commit in the loop, not only in the empty-range/stale-pin guards. **[INCONCLUSIVE — verify in `run_gate_inner`]**: from the grep, `has_first_parent` exists but the loop at lib.rs:248/268 calls `{commit_sha}^1` directly; ensure a root commit in-range is routed to a defined outcome (empty-tree diff) rather than a hard error.

### Best-practice incantations **[VERIFIED — git-diff-tree / git-show man pages]**

| Intent | Command | Merge-safe | Root-safe |
|--------|---------|-----------|-----------|
| Net first-parent delta (what the crate wants per-commit) | `git show --format= --name-only --first-parent <C>` | first-parent only (by design) | **yes** (shows root additions) |
| Union of paths differing from **any** parent | `git diff-tree --root -m -r --no-commit-id --name-only -z <C>` \| dedupe | **yes** (per-parent) | **yes** |
| Root delta via synthetic empty parent | `empty=$(git hash-object -t tree --stdin </dev/null); git diff --name-only "$empty" <C>` | n/a | **yes** |
| Every commit newly reachable by the merge (history-level) | `git rev-list C^1..C` then diff each | **yes** | n/a |

Notes:
- The SHA-1 empty-tree object ID `4b825dc642cb6eb9a060e54bf8d69288fbee4904` works, but `git hash-object -t tree --stdin` is preferable — it is correct for both SHA-1 and SHA-256 repositories.
- Use `-z` (NUL-delimited) + `sort -zu` for any security-sensitive path enumeration so unusual filenames cannot split/inject records.
- **Recommended minimal change for the crate:** replace `git diff --name-only C^1 C` with `git show --format= --name-only --first-parent <C>`. This preserves the intended first-parent semantics for normal commits AND is root-safe, eliminating the root-commit hard-error path with no behavioral change for ordinary commits. (If the policy must also attest second-parent history, additionally walk `git rev-list C^1..C` — but the simpler and stronger fix is to keep merge commits out of the range per Q1.)

---

## Q3 — required-check vs paths-filtered / skipped job (vacuous-gate class)

### The confirmed behavior **[VERIFIED — GitHub docs "Troubleshooting required status checks" / "Status checks reference"]**

The original claim ("a skipped paths-filtered job satisfies a required check vacuously") **conflates two distinct mechanisms**:

| Where the skip happens | GitHub behavior | Required-check result | Merge effect |
|------------------------|-----------------|-----------------------|--------------|
| **Workflow-level** `on.pull_request.paths` / `paths-ignore` | Workflow never runs → **no check run is ever posted** | **Not satisfied** — shows "Expected — Waiting for status to be reported" (Pending) | **Blocked indefinitely** |
| **Job-level** `if:` (incl. path-detector outputs) | Job created then skipped | **Satisfied vacuously** — a skipped job reports **Success**; `success`/`skipped`/`neutral` all count as satisfying | **Merge allowed** without the check running |
| Downstream `needs:` job **without** `if: always()` | Skipped when an upstream fails/skips | Vacuously satisfied | A failed upstream can fail to block if only the skipped downstream job is required |

GitHub provides **no native "required if run, otherwise not-applicable"** mode. There is also **no native job-level `paths:` key** — "job path filtering" means an unconditional workflow + a detector job + `jobs.<id>.if`.

### Recommended pattern (current, 2026) **[VERIFIED + LIKELY]**

For a self-contained gate like this binary, the **best** pattern is the simplest one and the crate already fits it:

> **An unconditional job that always runs and decides pass/fail internally.** The `policy15-attestation-gate` binary is exactly this — it exits `0` (PASS incl. `PASS-zero-activations`) or `2` (FAIL/EMPTY). It never relies on a job-level `if:` skip, so it always posts a real success/failure on the PR head SHA.

Rules to honor:
1. **Do NOT** put `on.pull_request.paths` at the workflow level for the required workflow (→ permanent-block class), and **do NOT** gate the gate job behind a job-level `if:` path condition (→ vacuous-pass class). Let the binary itself do path scoping internally (it already scopes to `PLUGIN_CRATE` / `RED_GATE_LOG` and returns `PASS-zero-activations` when nothing relevant changed).
2. **Add `merge_group:`** to the trigger set and make the gate run against the merge-group SHA, or the required check will not post for queued commits and merge-queue merges will fail. (GitHub docs: merge queue requires workflows to trigger on `merge_group`.)
3. If this gate ever fans into a matrix, the single required check must be a final gate with `if: ${{ always() }}` that inspects `needs.<job>.result` explicitly and fails on `skipped`/`cancelled`/`failure` — never rely on a bare `needs:`.
4. **Avoid** the legacy "dummy passthrough job with the same check name" workaround — GitHub's current guidance no longer recommends it; it converts a block problem into a false-green problem when trigger sets aren't provably mutually exclusive.

Sources: docs.github.com "Troubleshooting required status checks" (the authoritative skip-behavior table), "Status checks reference," "Managing a merge queue"; jakewharton.com "Fan-in to a single required GitHub Action"; brunoscheufler.com monorepo-required-jobs.

---

## Q4 — positive+negative fixture self-tests ("test the test") — prior art

**[VERIFIED — strong prior art exists; the design rationale is well-founded.]**

The practice of embedding a suite that proves the gate **FAILS on known-bad** and **PASSES on known-good** is recognized across several established bodies of work. Recommended framing: **"a self-verifying gate with positive and negative control fixtures"** / **"built-in self-test (BIST) for the gate."**

| Prior art | Relevance | Analogy strength |
|-----------|-----------|------------------|
| **EICAR Anti-Virus Test File** (Wikipedia; Virus Bulletin) | Canonical harmless known-bad string a scanner *must* detect — verifies the detector actually triggers without real malware. Direct analogue of a known-bad fixture the gate must reject. | **Very strong** |
| **Mutation testing** (Just et al. FSE'14; Jia & Harman survey) | Inject faults, measure whether the suite "kills" them. The crate's fixed bad fixtures are **mutation-inspired controls / fault-seeding**, not full mutation testing (they don't systematically mutate the impl) — describe them precisely as such. | **Strong (conceptual)** |
| **Test-oracle / assertion validation** (Barr et al. "The Oracle Problem: A Survey") | Tests can execute and pass while checking nothing; the **inadequate-assertion problem** is exactly the silently-broken-gate risk. | **Very strong** |
| **Vacuity detection** (formal verification; Siemens "vacuous proofs") | A property can "pass" for a trivial/unintended reason — the precise formal name for the "vacuous gate" concern. | **Very strong (for the rationale term)** |
| **Built-in self-test (BIST) / built-in test (BIT)** (Wikipedia; aviation safety) | Established engineering term for a system that verifies its own ability to detect faults. | **Very strong (terminology)** |
| **Negative testing** (Tricentis, testdevlab) | Names the bad-fixture half, but doesn't by itself imply "the gate must reject." | Strong (fixture type) |
| Canary / tripwire / dead-man switch | Operational-monitoring metaphors; useful secondary framing but weaker as primary citations for a CI gate. | Loose–moderate |

**Design caveat to document [VERIFIED — oracle-validation literature]:** the self-test must exercise the **same production gate path** used in CI (call `run_gate` / `run_gate_from_merge_base`, not a re-implemented predicate), or it merely proves two copies of the same mistake agree. The crate's test module (`src/lib.rs` tests: `test_positive_1_absent_log`, positive/negative/empty-range/unmeasurable-diff/no-op controls) already invokes `run_gate_from_merge_base` on ephemeral fixture repos — **this is the correct implementation of the pattern.** Recommend labeling it in-code as a "gate self-test / positive+negative control suite" and citing EICAR + oracle-validation + vacuity in the doc-comment to make the rationale explicit and durable.

Suggested one-line design-doc justification:
> "This is an EICAR-style self-test for a CI attestation gate, implemented as positive and negative control fixtures and motivated by test-oracle validation, vacuity detection, and mutation-testing-inspired fault seeding — it guards against a vacuous (silently-passing) gate."

---

## Consolidated recommendations for the P0 fix

1. **[Q1 — do before wiring CI]** Set `ref: ${{ github.event.pull_request.head.sha }}` + `fetch-depth: 0` in the gate's checkout step; add `merge_group:` to triggers. Pass `base.ref` to the binary rather than relying on the hardcoded `develop` default.
2. **[Q2 — crate hardening]** Route in-range **root commits** to a defined outcome (empty-tree diff via `git hash-object -t tree`) instead of the `C^1` hard-error path; confirm `has_first_parent` is consulted in the per-commit loop. Consider swapping `git diff --name-only C^1 C` → `git show --format= --name-only --first-parent <C>` for root-safety with identical first-parent semantics. Decide explicitly whether second-parent-history attestation is in policy scope (default: keep merge commits out of range via Q1, so it is not).
3. **[Q3 — CI topology]** Keep the gate as an **unconditional required job that decides internally** (already true); never wrap it in a job-level `if:` path filter and never workflow-level `paths:`-filter the required workflow. `PASS-zero-activations` is the correct non-vacuous "nothing to do" signal.
4. **[Q4 — rationale]** Keep and label the positive/negative control suite; cite EICAR / oracle validation / vacuity detection in the doc-comment. Ensure controls call the real gate entry point (already the case).

### Flagged as inconclusive / needs local confirmation
- **[INCONCLUSIVE]** Exact ordering of `has_first_parent` vs the `C^1` diff/`rev_parse` calls inside `run_gate_inner` for an in-range root commit (grep shows both exist; behavior on a root-in-range wasn't traced end-to-end). Verify with a root-commit-in-range unit fixture.
- **[INCONCLUSIVE]** Whether POLICY 15 intends to attest second-parent *history* or only net tree deltas — a policy question for the architect, not resolvable from CI mechanics alone. Default recommendation (fix checkout so merges never enter the range) sidesteps it.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 3 | Q1 actions/checkout PR merge-ref + fetch-depth semantics (version-aware v3–v7); Q2 `git diff C^1 C` merge/root/empty-tree semantics; Q3 required-check vs skipped/paths-filtered behavior + current patterns |
| Perplexity perplexity_reason | 1 | Q4 prior-art/terminology synthesis (EICAR, mutation testing, oracle validation, vacuity detection, BIST) |
| Read | 1 | `crates/policy15-attestation-gate/src/main.rs` (grounding recommendations in actual invocation) |
| Grep | 1 | `src/lib.rs` — confirm gate uses `merge_base..HEAD`, `C^1` diff, `has_first_parent`, and existing control-fixture tests |
| Training data | 0 areas | Not relied upon for any claim; all findings sourced to git-scm man pages / GitHub docs / cited literature |

**Total MCP tool calls:** 4 (3 `perplexity_research` at `search_context_size: high`, 1 `perplexity_reason` at high)
**Training data reliance:** low — every substantive claim is attributed to official git-scm man pages, GitHub docs, actions/checkout README, or named literature; version numbers verified via checkout release notes rather than recalled.
**Deviation note:** `perplexity_research` was used for the three non-trivial technical questions per the PRIMARY-tool mandate; `perplexity_reason` was chosen for Q4 because it is a synthesis-over-terminology task (prior-art mapping) rather than fresh multi-source fact-finding.
