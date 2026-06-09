# Issue #172 — Route Demo Evidence to factory-artifacts (Not the Product Repo)

**Date:** 2026-06-09
**Issue:** [#172] `feat(demo): route demo evidence to factory-artifacts (not the product repo), with operator choice of repo / factory-artifacts / local-only`
**Label:** enhancement · **State:** OPEN
**Researcher:** research-agent (vsdd-factory)
**Repo state:** `develop` @ `82163b7f`

---

## Restated Question

Demo evidence — binary recordings (`.gif`, `.webm`), VHS `.tape` scripts, Playwright
`.spec.ts`, and `evidence-report.md` — is currently committed into the **product
repository** (feature branch → squash-merged to `develop`), permanently bloating the
product's git history. The issue asks: (1) make `factory-artifacts` (the orphan
branch where the rest of the pipeline's evidence already lives) the default demo-
evidence destination; (2) add an **operator choice** of `repo` / `factory-artifacts` /
`local-only`; and (3) resolve the internal contradiction where the canonical
`demo-recorder` agent writes to `docs/demo-evidence/` (and *bans* `.factory/`) while
the `record-demo` and `demo-recording` skills point at `.factory/`.

---

## Codebase Grounding

### Current behavior — confirmed exactly as the issue describes

**Canonical `demo-recorder` agent** (`plugins/vsdd-factory/agents/demo-recorder.md`)
writes to and commits in the **product repo / feature branch**:
- Constraint: *"ALWAYS produce evidence in `docs/demo-evidence/<STORY-ID>/` (committed
  to feature branch, visible in PR diff)"*.
- Constraint: *"MUST NOT write to `.factory-demos/` or `.factory/demo-recordings/` —
  use `docs/demo-evidence/<STORY-ID>/` only"* (an explicit ban on the factory path).
- Recording Protocol step 5 / Outputs: `git add docs/demo-evidence/<STORY-ID>/ &&
  git commit -m "evidence(STORY-NNN): add demo recordings"`.
- Artifact types: `.gif`, `.webm`, `.tape`, `.spec.ts`, `evidence-report.md`.

**Orchestrator + delivery skills agree with the agent** (product-repo path):
- `skills/deliver-story/SKILL.md` and `steps/step-e-record-demos.md`,
  `steps/step-g-cleanup.md` → `.worktrees/STORY-NNN/docs/demo-evidence/<STORY-ID>/`.
- `skills/code-delivery/SKILL.md` → `docs/demo-evidence/<STORY-ID>/`, explicitly
  *"committed to feature branch, NOT `.factory/`"*, and embeds GIF thumbnails in the
  PR body by repo path → confirms binaries land in the `develop` diff after merge.

**Contrast — state-manager** commits `.factory/` artifacts to the `factory-artifacts`
orphan branch at phase gates (per CLAUDE.md Git Workflow + Routing Table). Demo
evidence is the **odd one out**, bypassing the orphan branch.

### The internal contradiction — confirmed

`grep` of `plugins/vsdd-factory/skills` for demo paths shows the plugin disagrees with
itself:
- `skills/demo-recording/SKILL.md` writes to `.factory/demo-scripts/`,
  `.factory/demo-recordings/`, `.factory/demo-evidence/` (lines 40, 128, 141, 151,
  196, 237, 273, 280–287).
- `skills/record-demo/SKILL.md` writes to `.factory/demo-evidence/STORY-NNN/`
  (lines 46, 55, 59).
- `skills/wave-gate/SKILL.md` reads `.factory/demo-evidence/STORY-NNN/demo-report.md`
  (line 107) — i.e. the **wave gate already expects the factory path**, while
  delivery commits to the product path. This is a live inconsistency, not just docs.

So the skills already point at `.factory/`; the canonical agent + delivery skills
override toward the product repo. The issue's "already half-built the right way"
framing is accurate.

### Prior work check

CHANGELOG shows `0.49.0 — Per-story demo evidence scoping (POL-010)` and a
`validate-wave-gate-completeness.sh` hook that checks for "demo evidence" among 6
gates — but **no routing/destination work**: no `factory-artifacts` demo routing, no
operator choice, no `local-only` option, no `.factory/merge-config.yaml` `demo_evidence`
field. The decision-log contains no closure of demo-evidence routing. Confirmed NEW.

---

## External Research

### Why committing `.webm`/`.gif` to a product branch is a real problem (corroborated)

Perplexity deep-research confirms every harm the issue cites:
- **Permanent, unrecoverable bloat:** "once a blob becomes part of a reachable commit…
  it is effectively permanent until that portion of the history is explicitly rewritten
  and garbage-collected… Removing the file from the working tree in a later commit
  merely adds more objects; it does not reclaim the space." Git GC "only removes
  objects that are unreachable" — and demo blobs on `develop` are always reachable.
- **Binary delta-compression is ineffective:** for "media formats such as GIF and
  WebM… delta compression is often ineffective… every version of a large binary tends
  to occupy substantial space in the packfiles." Each version is stored as a separate
  opaque blob.
- **Clone/fetch/CI cost:** "Large packfiles… significantly increase clone times,
  bandwidth usage, and disk space… CI agents frequently clone and fetch repositories…
  A bloated repository inflates resource usage." Amplified in monorepos.
- **History rewrite is the only true fix and it's disruptive:** `git filter-repo`
  "fundamentally alter[s] commit hashes, requiring force-pushes… and coordination with
  all consumers" — reinforcing that **prevention (route elsewhere from the start) beats
  cure**.
- **Guiding principle:** "the main branch of a product repository should focus on
  source code and small, text-based… files… Artifacts that are large, binary,
  frequently regenerated, or short-lived are better suited to other storage patterns."
  Demo evidence is precisely "transient or voluminous artifacts that arise from
  testing and demonstration."

### Orphan-branch storage pattern (the issue's "minimum bar" = factory-artifacts)

Deep-research validates the orphan-branch approach vsdd-factory already uses for
`.factory/`:
- "An orphan branch is a branch whose initial commit has no parent… `git checkout
  --orphan`… holds no relationship to `main`… two disjoint commit DAGs… objects remain
  content-addressed and deduplicated across the repository."
- **Advantages:** reuses existing repo auth/backup; keeps code diffs clean; retains
  Git versioning of media; scales from ad-hoc to formalized.
- **Critical limitation to plan for:** "they do not inherently solve the problem of
  repository size. Unless clones and fetches are explicitly restricted, the artifact
  branch contributes to the repository's packfiles and will be transferred to every
  clone." Mitigation: `git clone --single-branch --branch main …` so consumers who
  don't need media skip the orphan branch. **vsdd-factory already isolates
  `factory-artifacts` as a worktree-mounted orphan branch**, so this is on-pattern and
  the size concern is contained to those who fetch it.

### Surfacing demo GIFs in a PR when the binary is NOT in the diff

Three deep-research-endorsed patterns (answers the issue's open question):
1. **PR/issue drag-drop attachment** (`user-images.githubusercontent.com/...`): "the
   platform uploads the file to an internal asset storage system and inserts a Markdown
   link… does not impact clone size… arguably the best practice" for ephemeral PR
   demos. Not in `.git` at all.
2. **Orphan-branch permalink** pinned to a commit SHA:
   `![demo](https://github.com/org/repo/blob/<sha>/path/demo.gif?raw=1)` — "ensures the
   link always refers to the same version… even if the file is later updated." Requires
   append-only discipline on the branch to avoid broken links.
3. **GitHub Release assets** for curated, version-tied demos: "live in GitHub's release
   asset storage rather than in the `.git` directory, so they do not contribute to
   clone size."

For `local-only`, deep-research endorses `.gitignore` (e.g. `local-demos/`) + optional
pre-commit large-file guard; note "`.gitignore` affects only untracked files" — so the
path must be ignored **before** first commit.

### CI-artifact option (relevant to `local-only` / ephemeral)

`actions/upload-artifact` with `retention-days` stores evidence "separately from the
Git repository… do not contribute to the repository's `.git` size" with auto-expiry —
a production-grade home for ephemeral evidence that still surfaces a download link in CI.

Primary sources (per deep-research citations): Git object-model / GC docs, `git
checkout --orphan` docs, Git LFS docs, GitHub Releases + issue-attachment docs,
`actions/upload-artifact` docs.

---

## Verdict

**VALID-NEW** — Confidence: **HIGH**

Rationale:
- **Current behavior confirmed**: demo evidence is committed to the product repo's
  feature branch and merged to `develop` (demo-recorder agent + deliver-story/
  code-delivery skills), permanently bloating product history.
- **Internal contradiction confirmed and live** (not just cosmetic): the wave-gate
  skill already reads `.factory/demo-evidence/...` while delivery writes
  `docs/demo-evidence/...`; the `demo-recording`/`record-demo` skills write `.factory/`
  while the canonical agent bans it. This is an actionable defect regardless of the
  routing feature.
- **Best-practice alignment**: routing transient binary evidence off the product
  source branch is the deep-research-endorsed prevention pattern; `factory-artifacts`
  (an existing orphan branch) is the correct destination and is already how the rest of
  the pipeline's evidence is stored. The `repo`/`factory-artifacts`/`local-only` choice
  maps cleanly onto the three established storage patterns (orphan branch / in-tree /
  ignore+CI-artifact).
- **No prior closure** in decision-log or CHANGELOG; POL-010 only scoped per-story
  *naming*, not destination.
- The default-routing question ("`factory-artifacts` vs keep `repo` for backward
  compat") is a genuine human/architect decision (backward-compat policy) — surface it,
  don't auto-decide; this keeps the verdict VALID-NEW with one human-gated sub-decision.

---

## Recommended Approach (zero re-research)

**Owning agents/skills:** This spans multiple owners and MUST be orchestrated, not
done by one agent:
- `demo-recorder` agent — output-path + commit-target become routing-aware.
- `orchestrator/per-story-delivery.md` + `deliver-story`/`code-delivery` skills — pass
  routing decision into the dispatch; stop hard-committing to `develop`.
- `state-manager` — include demo-evidence paths in the `factory-artifacts` commit when
  routing = `factory-artifacts` (state-manager owns `.factory/` + orphan-branch commits
  per Routing Table).
- `pr-manager` / `code-delivery` / PR template — embed/link demos without assuming
  they're in the `develop` diff.
- `record-demo` + `demo-recording` skills — reconcile to the single routing model.
- `.factory/merge-config.yaml` (or equiv) — new `demo_evidence` routing field +
  orchestrator prompt.

### Scope decomposition

1. **Resolve the contradiction first (independent, ship-able now).** Pick ONE source of
   truth for the path before adding routing. Production-grade default: align on the
   routed model — `demo-recorder` writes to a routing-determined location; remove the
   hard *"MUST NOT write to `.factory/`"* ban; fix `demo-recording`/`record-demo` to
   match; fix the wave-gate reader to match the chosen destination. Per CLAUDE.md
   Canonical Principle, do not leave the `docs/` vs `.factory/` split as a documented
   inconsistency.

2. **Add the routing field + three modes.**
   - `factory-artifacts` (recommended default — surface the default choice to the
     human): write to `.factory/cycles/<cycle>/<story-id>/demo-evidence/`; state-manager
     commits to the orphan branch. PR links to the orphan-branch blob (SHA-pinned
     permalink) or attaches as PR/release asset — NOT a `develop` diff entry.
   - `repo`: current behavior (`docs/demo-evidence/<STORY-ID>/`, in PR diff) for teams
     that explicitly want it.
   - `local-only`: produced in the worktree, **gitignored** (add the path to
     `.gitignore` before any commit per deep-research), cleaned up after review;
     optionally upload as a CI artifact with `retention-days` for a transient link.

3. **PR embedding without the binary in the diff.** Implement the deep-research
   patterns: drag-drop attachment (simplest, default for `local-only`/ephemeral),
   SHA-pinned orphan-branch permalink (for `factory-artifacts`), or release asset
   (curated). Update `pr-manager`/`code-delivery`/PR template to stop assuming
   `docs/demo-evidence/` is in the `develop` diff.

4. **Operator prompt granularity** (issue open question): recommend ask-once-per-
   pipeline with a per-story override field, recorded in `merge-config.yaml` / STATE.

5. **Migration** (issue open question): leave already-committed `docs/demo-evidence/`
   in product history as-is (rewriting history is disruptive per deep-research);
   document the relocation guidance for new evidence only. Do NOT force-rewrite
   `develop`/`main` history as part of this feature.

### Key files

- `plugins/vsdd-factory/agents/demo-recorder.md` — routing-aware output + remove
  `.factory/` ban.
- `plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md` — pass routing into
  dispatch.
- `plugins/vsdd-factory/agents/state-manager.md` — commit demo paths to
  `factory-artifacts` when routed.
- `plugins/vsdd-factory/skills/{deliver-story,code-delivery,record-demo,demo-recording,wave-gate}/SKILL.md`
  — reconcile to single model.
- `plugins/vsdd-factory/agents/pr-manager.md` + PR template — embed/link without diff
  dependency.
- `.factory/merge-config.yaml` (or equiv) — `demo_evidence` field.

### Risks / dependencies

- **Orphan-branch clone bloat** still applies to anyone who fetches `factory-artifacts`
  — mitigated because vsdd-factory already isolates it as a worktree-mounted orphan
  branch and consumers can `--single-branch` the product repo.
- **Default-routing decision is human-gated** (backward-compat policy) — surface
  `factory-artifacts`-as-default to the architect/human; do not silently flip the
  default for existing consumers without sign-off.
- **PR-link fragility:** SHA-pinned permalinks require append-only discipline on
  `factory-artifacts` (force-push would break links) — consistent with existing
  factory-artifacts handling.
- **Self-referential note:** in vsdd-factory itself, `docs/demo-evidence/...` already
  exists in product history (e.g. `docs/demo-evidence/S-12.01/`, `S-15.13/`) — the
  migration-leaves-history guidance applies to the engine's own repo.
- **Cross-issue:** independent of #151/#131; touches the most files of the three
  (agent + orchestrator + 5 skills + state-manager + PR template + config) and MUST be
  orchestrator-coordinated across owners, not single-agent.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (`reasoning_effort=high`) | Separating binary QA artifacts from product git history: GC/reachable-blob bloat; orphan-branch pattern + single-branch clone; Git LFS tradeoffs; Release/PR-attachment embedding; CI-artifact retention; gitignore/local-only; PR-embed-without-diff patterns |
| Read | 3 | issue body; demo-recorder agent; consistency reference structure |
| Grep | 4 | demo-path sweep across skills (confirmed `.factory/` vs `docs/` split); CHANGELOG/decision-log closure check; deep-research extraction |
| Training data | 0 areas | All git/GitHub/LFS/CI claims sourced from live deep-research |

**Total MCP tool calls:** 1 (shared across the 3-issue batch; this issue's findings drawn from the dedicated artifact-storage deep-research call)
**Training data reliance:** LOW — git object-model, orphan-branch, LFS, Release-asset, and CI-artifact claims are externally sourced.
