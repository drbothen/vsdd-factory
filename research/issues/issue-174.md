# Issue #174 — CLAUDE.md health-check + threshold-driven compaction (mirror STATE.md size governance)

**Date:** 2026-06-09
**Issue:** [#174](https://github.com/drbothen/vsdd-factory/issues/174) — `feat: CLAUDE.md health-check + threshold-driven compaction (mirror STATE.md size governance)`
**Labels:** documentation, enhancement
**Repo state:** `develop` @ `82163b7f`, plugin `v1.0.0-rc.20`
**Research agent:** Claude (vsdd-factory research-agent)

---

## Restated proposal

vsdd-factory has a mature **STATE.md** size-governance system (warn/critical thresholds, a PostToolUse hook, a health-check skill, and a compaction skill that externalizes history to cycle files) but **no equivalent for CLAUDE.md** — only `scaffold-claude-md`, which *generates* a CLAUDE.md and never measures or compacts it. Because CLAUDE.md is loaded **in full into every session's context**, an unbounded CLAUDE.md is arguably a bigger context-rot liability than STATE.md. The issue proposes a parallel capability mirroring the STATE.md pattern: (1) a `validate-claude-md-size.sh` PostToolUse hook (WARN >150 / CRITICAL >250 lines, with the "write reduced size → allow" compaction-in-progress escape); (2) a `check-claude-md-health` diagnostic skill; (3) a `compact-claude-md` skill that **delegates to Anthropic's official `claude-md-management` / `claude-md-improver`** driven by a VSDD-aware routing prompt (load-bearing-rule + pointer; externalize detail to ADRs / `.factory/research/` / STATE.md / `.claude/rules/` / subdir CLAUDE.md / skills; preserve test-pinned literals; show diffs, never delete); and (4) documented CLAUDE.md-vs-STATE.md-vs-ADR content-routing boundary. Critical is advisory-only (CLAUDE.md is human-authored; hard-block behind opt-in).

---

## Codebase grounding

### What exists today (verified)

| Component | STATE.md (exists) | CLAUDE.md (proposed) — status |
|---|---|---|
| PostToolUse size hook | **`hooks/validate-state-size.sh`** — WARN >200, BLOCK >500; **"this write reduced line count → exit 0" compaction-in-progress escape** via `git show HEAD:STATE.md` compare (lines 55–71); also has a `lessons.md` arm (WARN >3500 / BLOCK >4000, D-442(e)) | **ABSENT** — no `validate-claude-md-size.sh`; `grep -ri claude-md` across `hooks-registry.toml` → 0 hits; the hook fires only on `*/.factory/STATE.md` and `cycles/*/lessons.md` path arms (lines 35–45) — CLAUDE.md never matches. |
| Health-check skill | **`skills/check-state-health/SKILL.md`** — HEALTHY/WARNING/NEEDS-COMPACT, size table (0-200 HEALTHY / 201-500 WARNING / 501+ NEEDS-COMPACT), structure + content-routing checks | **ABSENT** — no `check-claude-md-health`. |
| Compaction skill | **`skills/compact-state/SKILL.md`** — extracts history to `cycles/<cycle>/*.md`, slims to <200; **safety contract: only moves content, never deletes; writes targets before removing; aborts on failure** (lines 136–140) | **ABSENT** — no `compact-claude-md`. |
| Generator | **`skills/scaffold-claude-md/SKILL.md`** — generates CLAUDE.md (language/build/test/lint/git detection), confirm-before-write; **never measures or compacts** | EXISTS, unchanged in proposal. |

**Conclusion:** The issue's premise is **exactly correct.** The STATE.md governance triad (size hook + health skill + compaction skill, all with a reduce-size escape and a never-delete safety contract) is real and well-built; the CLAUDE.md counterpart is entirely absent. The issue's mirror table maps 1:1 onto the actual files. No prior CHANGELOG/decision-log work on CLAUDE.md governance. The factory's own root `CLAUDE.md` is itself very large (the operating-instructions file), making this dogfood-relevant.

---

## External research (primary sources)

### (1) The official `claude-md-management` plugin exists — the proposal's delegation target is real
Verified (Perplexity ask + GitHub): Anthropic publishes an official **`claude-md-management`** plugin in the **`claude-plugins-official`** marketplace, containing a **`claude-md-improver`** skill that audits and scores `CLAUDE.md` files. So the issue's "delegate to the official tool rather than reimplement" approach is grounded in a real, maintained dependency.
- Primary: `https://github.com/anthropics/claude-plugins-official/tree/main/plugins/claude-md-management`
- Also: `https://claude.com/plugins/claude-md-management`

### (2) There is NO native command that compacts a CLAUDE.md
Verified: `/compact` operates on **conversation history**, not on CLAUDE.md. There is **no documented native Claude Code slash command that compacts/summarizes the contents of a CLAUDE.md file**. CLAUDE.md is treated as persistent, human-curated config, deliberately not auto-summarized. So a `compact-claude-md` capability is genuinely net-new — nothing in the harness does it.
- Primary: `https://platform.claude.com/docs/en/build-with-claude/compaction`
- Corroboration: `https://github.com/anthropics/claude-plugins-official` (closest official mechanism is the plugin above, plus `#`, `/init`, `/memory`).

### (3) Context rot is real and continuous (justifies CLAUDE.md-specific, lower thresholds)
- Anthropic's *Effective context engineering for AI agents* guidance: **curate, don't dump** — quality degrades as input grows, well before the max window. (`https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents`)
- Chroma's 2025 "Context Rot" study (18 frontier models incl. Claude Opus 4) shows degradation begins long before the context limit — supports threshold-driven trimming of an *always-loaded* file like CLAUDE.md.
- **Threshold numbers are community heuristics, NOT authoritative Anthropic benchmarks** (the issue is honest about this, and it is correct to be): community consensus cites ~150 lines (~4.5–5.5k tokens) as where degradation begins and ~250 lines as severe, with high-performers at 60–100 lines; Anthropic docs broadly recommend keeping CLAUDE.md "under ~200 lines." Treat the exact line/token figures as **soft** (TurboDocx, Buildcamp, dometrain, HN — blog-tier). This means the WARN-150 / CRITICAL-250 defaults are *defensible* but should be **configurable**, and **token-based thresholds (~5k warn / ~8k critical) are the more robust primary** since line length varies.

### (4) Advisory-vs-block for a human-authored file
The issue's recommendation — **advisory-only at critical, hard-block behind opt-in** — is the right call and consistent with how the factory already differentiates ownership: STATE.md is factory-owned so it hard-BLOCKs at 500; CLAUDE.md is human-authored, so a hard block on a user editing their own file is hostile. Mirror the `validate-state-size.sh` "reduced-size → exit 0" escape so the `compact-claude-md` skill can rewrite without tripping its own gate.

---

## Verdict

> **VALID-NEW** — Confidence: **High**

Every claimed gap is verified absent (no `validate-claude-md-size.sh`, no `check-claude-md-health`, no `compact-claude-md`; `scaffold-claude-md` only generates). The mirror target (STATE.md governance triad) exists exactly as described and is a clean template. The delegation target (`claude-md-management` / `claude-md-improver`) is a real official plugin, and no native CLAUDE.md-compaction command exists — so the work is net-new and well-scoped. The only soft spot is the **specific numeric thresholds** (community heuristics, not Anthropic benchmarks) — handle by making them configurable and token-based-primary, which the issue already proposes.

---

## Recommended approach + scope (zero re-research)

### Mirror the STATE.md triad (route: devops-engineer for hook, then technical-writer/architect for routing doc)

1. **`hooks/validate-claude-md-size.sh` (PostToolUse).** Fork `validate-state-size.sh` structure verbatim: add a path arm matching any `CLAUDE.md` (root + subdirectory CLAUDE.md). WARN >150 / CRITICAL >250 lines **and/or** token-based (~5k warn / ~8k critical — token primary, line fallback). **Keep the "this write reduced size → exit 0" git-compare escape** (lines 55–71 of the STATE hook) so compaction-in-progress isn't blocked. **Critical = advisory (loud stderr + suggest `/vsdd-factory:compact-claude-md`)**, NOT block; hard-block only behind an opt-in strict-mode env/setting. Register in `hooks-registry.toml` as a new PostToolUse entry on Write/Edit (legacy-bash-adapter pattern). **Note:** the existing STATE hook's git-compare uses `git show HEAD:STATE.md` with a fixed basename — the CLAUDE.md arm needs a path-relative git lookup (the `lessons.md` arm at lines 99–116 already shows the canonical-path-relative pattern; copy that, not the fixed-basename STATE arm).

2. **`skills/check-claude-md-health/SKILL.md`** (or extend `check-state-health`). Read-only diagnostic: HEALTHY/WARNING/NEEDS-COMPACT by the same thresholds, plus a content-quality pass borrowing `claude-md-improver`'s six-dimension rubric (commands, architecture, non-obvious patterns, conciseness, currency, actionability) + A–F grade. Wire into session-start health checks alongside `factory-health` / `check-state-health`.

3. **`skills/compact-claude-md/SKILL.md`** — **delegate to `claude-md-management:claude-md-improver`** with a VSDD-aware routing prompt encoding: load-bearing-rule + pointer; externalize detail to the *right* place (architectural rationale → ADRs `docs/adr/…`; research/verification → `.factory/research/*.md`; pipeline/run history/decisions/lessons → STATE.md / `cycles/<cycle>/*.md`; path-scoped conventions → `.claude/rules/` with `paths:` frontmatter; module context → subdir CLAUDE.md; large procedures → skills). **Preserve test-pinned literals** (strings referenced by tests/CI/registrations — the factory has many: reason codes, branch names, D-NNN IDs). **Show diffs, get confirmation, never delete — only relocate** (mirror `compact-state`'s safety contract, lines 136–140). **Dependency handling:** invoke `claude-md-improver` best-effort if installed; graceful skip with a clear message otherwise (issue open-question 4 — recommend best-effort, not a hard declared dependency, to avoid coupling the factory to an external plugin's availability).

4. **Content-routing doc** (route: technical-writer): document the CLAUDE.md (stable load-bearing rules + pointers) vs STATE.md/cycle-files (run state, decisions, history) vs ADR/research/specs (deep "why") boundary so authors/agents stop accreting run narrative into CLAUDE.md.

### Risks
- **Numeric thresholds are soft** (community, not Anthropic) — mitigate by configurable + token-primary; do not hard-block on a soft number.
- **Hostile-block on human file** — advisory default is mandatory; opt-in strict mode only.
- **Path matching** — must catch root *and* subdirectory CLAUDE.md; git-compare must be path-relative (use the lessons.md arm's canonical-path pattern, not the STATE arm's fixed basename) or the reduce-size escape silently breaks on subdir files.
- **External-plugin coupling** — `claude-md-improver` availability is not guaranteed in every operator environment; graceful-skip required.
- **Self-application caveat:** the factory's own root CLAUDE.md is large and deliberately so (it is the operating-instructions SoT, governed by the Canonical Principle). A compaction run on it must respect that many sections are load-bearing rules (D-NNN, TD-VSDD-NNN, routing table) that cannot be relocated without breaking governance — the VSDD routing prompt must encode "these are load-bearing, pointer-only relocation forbidden."

### Test strategy
- Bats: hook warns >150 / flags critical >250 on a CLAUDE.md fixture; does NOT fire when a write reduces size (compaction-in-progress); does NOT hard-block at critical by default; subdir CLAUDE.md matched.
- Skill dry-run: `compact-claude-md` shows diffs + requires confirmation + preserves a seeded test-pinned literal + relocates (never deletes); graceful-skip when `claude-md-improver` absent.

### Dependencies
- Optional best-effort dependency on `claude-md-management` (`claude-md-improver`). Operator-level cache picks up the new hook/skills only after a release. Decision to settle (issue): new `compact-claude-md` skill vs extending `scaffold-claude-md` into `manage-claude-md` — recommend a **new skill** (separation of generate vs measure vs compact mirrors the STATE.md triad cleanly).

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| Perplexity perplexity_ask | 1 | Confirm `claude-md-management`/`claude-md-improver` official plugin exists + no native CLAUDE.md-compaction command |
| WebFetch | 0 (shared with #173) | — |
| Read / Grep / Glob | ~10 | Codebase grounding: validate-state-size.sh, compact-state, check-state-health, scaffold-claude-md, claude-md absence in registry, prior CHANGELOG/decision-log |
| Training data | ~1 area (flagged) | Context-rot threshold figures are community heuristics, explicitly flagged soft per the issue and corroborated by Anthropic curate-don't-dump guidance |

**Total MCP tool calls (this issue):** 1 ask (+ shared WebFetch/research context). **Training data reliance:** LOW-MEDIUM — plugin existence + no-native-command verified against primary GitHub/docs; threshold numbers are acknowledged community heuristics (configurable, token-primary recommended to absorb the uncertainty).
