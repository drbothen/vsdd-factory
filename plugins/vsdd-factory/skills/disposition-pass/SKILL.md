---
name: disposition-pass
description: Re-examine ingested reference repos through the Corverax vision lens to decide what to Model, Reimplement, Enhance, or Leave Behind. Produces per-repo Pass 9 disposition docs, a master rollup, and optionally updates the vision document. Runs against one repo or all 44.
argument-hint: "[<repo>|--all] [--rollup] [--update-vision]"
---

# Disposition Pass (Pass 9)

Brownfield ingest produces a **vision-agnostic** semantic understanding of a reference codebase. Once the Corverax vision exists, every ingested repo must be re-examined through that lens to decide what to adopt. This skill is that re-examination.

## When to Use

- **After** the vision doc exists and the repo has completed brownfield ingest through Phase C (final synthesis)
- **After** a material vision-doc change that may invalidate prior dispositions
- **Before** Phase 1 spec crystallization (`/create-prd`, `/create-architecture`) — dispositions tell you which patterns to spec from existing code vs. design from scratch
- **Before** `/decompose-stories` — net-new gaps surfaced by the rollup must become explicit stories

## When NOT to Use

- Before brownfield ingest is complete (no semantic understanding to disposition)
- Before the vision doc exists (no lens to examine through)
- During active vision drafting (run after the vision stabilizes, not during every edit)

## Inputs

- `$ARGUMENTS[0]` — repo name (e.g. `hermes-agent`) OR `--all` to run against every ingested repo
- `--rollup` — after per-repo dispositions complete, regenerate `MASTER-DISPOSITION-ROLLUP.md`
- `--update-vision` — after the rollup, propose vision-doc edits (writes a CHANGELOG)

## The Four Buckets

Every substantive capability in a repo is sorted into exactly one bucket:

| Bucket | Meaning |
|---|---|
| **Model** | Adopt as-is, port faithfully. The pattern is already right for Corverax. |
| **Take but reimplement** | Right idea, rebuild cleanly (e.g. Python god class → Rust state machine; runtime feature probing → cargo features). |
| **Enhance** | Take it and extend beyond what the source does (e.g. add Trust Layer attestation, durable checkpoints, mod packaging). |
| **Leave behind** | Explicitly reject. Wrong fit, obsolete, language-specific hack, conflicts with vision. **Reason required.** |

## Per-Repo Protocol

### Inputs the agent must read

1. **All ingest artifacts** in `.factory/semport/<repo>/` — every `<repo>-pass-*.md` file (broad sweep, deepening rounds, coverage audit, extraction validation, pass-8 final synthesis). All of them. Not just pass-8.
2. **Source code** in `.reference/<repo>/` — re-examine through the vision lens. Things the original ingest under-weighted may now matter; things it emphasized may now be irrelevant.
3. **Vision doc:** `.factory/specs/research/domain-corverax-vision-synthesis-*.md` (latest)

### Sandbox considerations

Disposition agents run inside the Corverax sandbox with a **Bash allowlist**, not full Bash. Two patterns work for source inspection:

1. **Standalone commands with absolute paths:** `find /Users/jmagady/Dev/corverax/.reference/<repo> -name '*.go' -type f | wc -l`
2. **Chained with `cd` into a reference dir:** `cd /Users/jmagady/Dev/corverax/.reference/<repo> && find . -name '*.go' -exec wc -l {} +`

Approved commands: `find`, `wc`, `cat`, `head`, `tail`, `ls`, `awk`, `xargs`, `sort`, `uniq`, `cut`, `tr`, `du`, `file`, `tree`, `basename`, `dirname`, `realpath`, `diff`, `jq`, `yq`, `tokei`, `cloc`, `scc`, `echo`, `printf`, `command -v`, `which`, plus `git -C <dir> *`. Use the **Grep tool** for content search, the **Glob tool** for file pattern listing.

If a single Bash command is denied, **try a different formulation before reporting failure**. Almost always the cause is a non-allowlisted command (`grep`/`rg`/`sed -i`), not a blanket Bash denial. See `.claude/agents/codebase-analyzer.md` for the complete sandbox patterns reference.

### Output

Write to `.factory/semport/<repo>/<repo>-pass-9-corverax-disposition.md`

Required sections:
- **Summary** (3-5 sentences: what this repo is and its overall disposition)
- **Model** — table: capability, location (file:line if from source), why it fits Corverax (cite vision section)
- **Take but reimplement** — table: capability, what's wrong with current execution, how Corverax should rebuild it, vision section
- **Enhance** — table: capability, what to add on top, vision section driving the enhancement
- **Leave behind** — table: capability, reason for rejection, vision conflict
- **Net-new insights** — patterns/risks the original ingest missed because it lacked the vision lens. Cite source `file:line`.
- **Disposition stats** — counts in each bucket

### Rules

- **One agent per repo. Always.** Never combine multiple repos in one agent — context exhaustion produces partial dispositions that must be discarded.
- **Tie every disposition to a named vision section.** No hand-waving.
- **Cite source `file:line`** for any new finding beyond the existing ingest.
- **Be specific.** Counts, file paths, version numbers — not "robust" or "scalable".

### Parallelism

When running `--all`, launch in batches of 10 concurrent agents to respect rate limits. Wait for each batch to complete before launching the next.

## Master Rollup Protocol

After all per-repo dispositions complete (or with `--rollup`), launch a synthesis agent that reads all `*-pass-9-corverax-disposition.md` files and produces `.factory/semport/MASTER-DISPOSITION-ROLLUP.md` with:

1. **Executive Summary** — total dispositions, bucket distribution, headline takeaways
2. **What We're Building From Where** — master capability table organized by Corverax layer (L0 types, L1 runtime, L2 orchestration, L3 governance, L4 observability, L5 surfaces, Trust Layer, Mod system, Industry factories)
3. **Cross-Repo Convergence (Strong Signals)** — patterns that Model/Reimplement in 5+ repos
4. **Cross-Repo Conflicts** — where repos disagree, with Corverax's recommended resolution
5. **Net-New Gaps** — capabilities the vision asserts that no source repo provides (Phase 1 must spec from scratch)
6. **Strong "Leave Behind" Themes** — anti-patterns rejected across many repos
7. **Highest-Leverage Repos** — top ~12 ranked by Corverax impact
8. **Vision Doc Update Recommendations** — concrete, actionable changes
9. **Statistics Appendix**

### Vision SHA tracking

The rollup header MUST include the vision-doc commit SHA it was generated against:

```yaml
---
generated_against_vision_sha: <sha>
generated_against_vision_path: .factory/specs/research/domain-corverax-vision-synthesis-YYYY-MM-DD.md
generated_at: <ISO8601>
---
```

When the vision SHA advances past this value, the rollup is **stale** and `/disposition-pass --all --rollup` must be re-run.

## Vision Update Protocol (`--update-vision`)

After the rollup, optionally launch an agent that applies the rollup's §8 recommendations to the vision doc in-place. Rules:

- **Edit in place.** Use the Edit tool, not Write. Preserve existing structure.
- **Do not delete substantive content.** Strengthen, add, sharpen.
- **Cite evidence inline:** `(per <repo> disposition)` or `(convergence: N repos)`
- **Add "Rejected Approaches" appendix** from rollup §6 if not present
- **Update §11 "what we build from the codebases" table** with corrected disposition data
- **Write a CHANGELOG** to `.factory/specs/research/domain-corverax-vision-synthesis-YYYY-MM-DD-CHANGELOG.md` listing every change

The vision doc is the source of truth for Phase 1. Every change must sharpen direction, not add bulk.

## Staleness Protocol

A disposition becomes stale when EITHER:
1. The repo is re-ingested (new BCs, new pass-8) → re-run that single repo's Pass 9
2. The vision doc materially changes → re-run `--all` and regenerate rollup

**How to check:**
```bash
# Vision SHA in rollup vs current
grep generated_against_vision_sha .factory/semport/MASTER-DISPOSITION-ROLLUP.md
cd .factory && git log -1 --format=%H specs/research/domain-corverax-vision-synthesis-*.md
```

If the SHAs differ, the rollup is stale.

## Output Summary

```
Disposition pass complete: <repo|all>
  Repos dispositioned: N
  Total capabilities sorted: N
    Model:                N
    Take but reimplement: N
    Enhance:              N
    Leave behind:         N
  Net-new insights: N
  Master rollup: regenerated | unchanged
  Vision doc: updated (CHANGELOG written) | unchanged
  Vision SHA: <sha>
```

## Commit Cadence

- After each batch of 10 per-repo dispositions: `factory(phase-0): pass 9 dispositions batch N`
- After master rollup: `factory(phase-0): master disposition rollup`
- After vision update: `factory(phase-0): vision doc enhancement from rollup`

## Relationship to Other Skills

| Skill | Relationship |
|---|---|
| `brownfield-ingest` | Prerequisite. Pass 9 needs the pass-0 through pass-8 artifacts. |
| `create-brief` / `create-domain-spec` | Prerequisite. Pass 9 needs a vision doc to use as a lens. |
| `create-prd` / `create-architecture` | Pass 9 outputs feed these — dispositions tell you what to spec from existing code vs. design from scratch. |
| `decompose-stories` | Net-new gaps from the rollup become explicit stories. |
| `semport-analyze` | Independent. Semport translates specific modules; disposition is project-wide vision alignment. |
