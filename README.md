# vsdd-factory

**Verified Spec-Driven Development (VSDD)** — a "dark factory" for software development, packaged as a Claude Code plugin marketplace.

Derived from the [Corverax](https://github.com/drbothen/corverax) project's `.claude/` pipeline. Extracts the full SDLC workflow — brownfield ingestion, spec crystallization, story decomposition, TDD delivery, adversarial review, holdout evaluation, formal verification, and release gating — into a shareable plugin.

## Install

```shell
/plugin marketplace add drbothen/vsdd-factory
/plugin install vsdd-factory@vsdd-factory
```

## What's inside

The `vsdd-factory` plugin ships:

- **Agents** — `adversary`, `codebase-analyzer`, `holdout-evaluator`, `research-agent`, `validate-extraction`
- **Skills** — full VSDD pipeline (`brownfield-ingest`, `create-brief`, `create-prd`, `create-architecture`, `decompose-stories`, `deliver-story`, `wave-gate`, `adversarial-review`, `holdout-eval`, `formal-verify`, `convergence-check`, `release`, and more)
- **Hooks** — `protect-vp.sh` (blocks edits to green verification properties), `verify-git-push.sh`, `check-factory-commit.sh`
- **Rules & Templates** — project protocols and artifact templates

## Pipeline overview

See `plugins/vsdd-factory/rules/` and individual skill `SKILL.md` files. High-level phases:

0. Brownfield ingest
1. Spec crystallization (brief → domain → PRD → architecture → adversarial review)
2. Story decomposition
3. TDD wave delivery
4. Adversarial refinement
5. Formal hardening
6. Convergence & release

## Local development

```bash
claude --plugin-dir ./plugins/vsdd-factory
```

## License

MIT
