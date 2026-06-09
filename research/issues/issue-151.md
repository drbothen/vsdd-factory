# Issue #151 — Drift-Resistant Source-Citation Convention + Checker

**Date:** 2026-06-09
**Issue:** [#151] `feat(spec-ci): adopt drift-resistant source-citation convention + checker`
**Label:** enhancement · **State:** OPEN
**Researcher:** research-agent (vsdd-factory)
**Repo state:** `develop` @ `82163b7f`

---

## Restated Question

VSDD spec artifacts anchor claims to source using raw `file.rs:NNN` line citations
that silently drift when source changes (an inserted line shifts every citation
below it). The issue asks the engine to (1) adopt a drift-resistant **citation
convention** (symbol anchors and/or commit-SHA-pinned line numbers) and (2) add a
**checker** to spec-CI that fails on stale citations. The reframed ask (per the
issue's own `VALIDATED-WITH-CHANGES` verdict) is explicit that a blank/comment/EOF
checker is insufficient because it misses the **dominant "wrong-but-plausible-line"
drift mode** — where an inserted line shifts a citation to a *different real code
line* the checker still accepts as valid.

---

## Codebase Grounding

### What already exists — TD-VSDD-091 + `validate-stable-anchors` hook

vsdd-factory has **already partially solved this** — but with an *avoidance*
mechanism, not a *drift-detection* checker.

- **TD-VSDD-091 ("Anti-volatile-pin")** is codified in `CLAUDE.md`:
  > "Narrative spec content must cite function names + behavioral anchors, NOT
  > `file.rs:NNN` line numbers (which decay on subsequent diffs). Justified
  > citations (Red Gate test tables, AC source-of-truth tables, pass-report
  > changelogs) excepted."

- **`validate-stable-anchors`** is a shipped PreToolUse WASM hook plugin
  (`crates/hook-plugins/validate-stable-anchors/src/lib.rs`, registered in
  `plugins/vsdd-factory/hooks-registry.toml` at `priority = 155`, `on_error = "block"`).
  Its docstring states it "Enforces TD-VSDD-091 stable-anchor convention for spec
  files under `.factory/specs/**/*.md`. **Blocks writes whose body text contains
  volatile `*.<ext>:NNN` line-cite patterns** for SOURCE-CODE and CONFIG files only
  (e.g., `main.rs:416`, `setup.bats:42`, `Cargo.toml:17`)." It carries an explicit
  allowlist (`SOURCE_FILE_EXTENSIONS`: rs, toml, sh, bash, py, ts, tsx, js, jsx, go,
  bats, yaml, yml, json, lock, lobster, wasm, c, cpp, h, hpp, rb) and exemption zones
  (`## Amendment`/`## Changelog` sections, VP-079 Scenario 6 SITES arrays). Its
  block message tells the author: *"Replace `*.<ext>:NNN` line citations with stable
  symbol anchors per TD-VSDD-091. Example: instead of `main.rs:416`, use the function
  name `emit_plugin_async_block_discarded`…"*

- Driver lineage: TD-031 (escalated P2→P1, fix-burst-15, O-P16-001). The hook was
  built specifically to break the recurrence loop where fix-bursts codifying TD-031
  kept re-introducing TD-031 violations.

### The critical gap (this is what #151 actually asks for)

`validate-stable-anchors` is a **prevention-by-forbidding** mechanism: it bans raw
source-code line cites from spec *prose* at write time. It does **NOT**:

1. **Verify that an existing (permitted) citation still points at its named symbol.**
   It has no positive model of "this cite should resolve to symbol X" — it only
   pattern-matches the *shape* `word.ext:digit` and blocks it.
2. **Catch the "wrong-but-plausible-line" drift mode.** Because it never validates a
   citation *target*, it cannot detect when a still-present line number now lands on
   the wrong real line. (It side-steps this by banning line numbers entirely in
   prose — but the issue's whole premise is that *justified* line citations exist:
   TD-VSDD-091 itself excepts "Red Gate test tables, AC source-of-truth tables,
   pass-report changelogs", and the hook exempts SITES arrays that "drive sed
   mutations". Those exempted line numbers are exactly the ones that silently drift
   and have **zero** mechanical drift-detection today.)
3. **Operate as a CI gate over the whole spec corpus.** It fires per-write on
   `.factory/specs/**/*.md` only; it is not a `cargo fmt --check`-style sweep that
   validates the entire existing citation set against current source.

No grep hit for any positive citation-resolution / drift-detection checker, the
string `Fiberplane`, `drift.lock`, or `drift check` anywhere in the repo. The
decision-log (`.factory/cycles/.../decision-log.md`) and CHANGELOG.md contain no
closure of a drift-detection-checker concern.

### Process-gap provenance (cross-project)

The issue originates from the `wirerust` project's deferred finding `P-CITE-PG`
(6+ recurrences across 13 adversarial passes; 1126 `file.rs:NNN` cites across 249
spec files). This is an **engine process/tooling gap**, correctly filed against
vsdd-factory rather than as a `wirerust` content defect. The exact same recurrence
pattern is independently attested in vsdd-factory's own TD-031 lineage — strong
corroboration that the gap is real and engine-wide.

---

## External Research

### Avoidance conventions vs. drift-detection mechanisms (the core distinction)

Perplexity deep-research confirms the issue's central thesis verbatim: there are two
fundamentally different families, and vsdd-factory currently only has the first.

> "avoidance conventions and drift-detection mechanisms… Drift actively monitors the
> semantic relationship between documentation and code, detecting when changes have
> occurred that necessitate documentation updates" — whereas avoidance conventions
> merely "prevent the use of line numbers."

### Fiberplane Drift (the issue's proposed adopt-vs-build candidate)

- **Mechanism:** AST fingerprinting + semantic anchoring. "references specific
  symbols rather than positional locations, [so] Drift cannot be fooled by line
  insertions or deletions that shift content within a file." It "uses AST
  fingerprinting to detect meaningful changes while ignoring irrelevant formatting
  differences" (the AST fingerprint excludes comment content, "avoiding false
  positives from documentation-irrelevant changes").
- **CI behavior:** "`drift check`… for each anchor retriev[es] the code content as it
  existed at the provenance commit and compar[es] it with the current version using
  the AST-based fingerprinting system… `drift check` exits with a non-zero status
  code, causing the CI pipeline to fail" — directly analogous to `cargo fmt --check`,
  which is exactly the integration the issue proposes.
- **`drift.lock`:** "remains manageable even in large repositories… enabling Drift to
  operate without requiring access to the repository's full Git history during
  staleness checks."
- **Maturity (as of mid-2026):** "a mature and actively developed tool"; the
  Fiberplane team "developed specific integrations for AI coding assistants such as
  Claude Code, Codex, and Cursor, teaching these agents how to maintain drift anchors
  automatically as they modify code." Drift "stands out as the most comprehensive
  drift-detection mechanism."
- **What Drift does NOT catch (important residual):** "it cannot detect when the
  semantic meaning of the referenced code has changed while the anchor region remains
  intact" — i.e., it solves wrong-but-plausible-*line* but not all wrong-but-plausible-
  *content* cases.

Primary sources: [github.com/fiberplane/drift](https://github.com/fiberplane/drift),
[fiberplane.com/blog/drift-documentation-linter](https://fiberplane.com/blog/drift-documentation-linter/)
(per issue body — to be version/license-verified at adoption time: issue cites MIT, v0.10.0).

### Comparison of drift-resistant conventions (all avoid raw line numbers)

| Mechanism | How it resists drift | Detection or avoidance? | Catches wrong-but-plausible-line? |
|-----------|----------------------|--------------------------|-----------------------------------|
| **Fiberplane Drift** | path + symbol + AST-hash, pinned to provenance commit; `drift check` in CI | **Detection** (active) | YES |
| **mdBook `{{#include}} ANCHOR`** | named `ANCHOR: name` regions; mdBook locates region by name, not line. Anchor regex `ANCHOR:\s*[\w_-]+`. Integrates with language parsers so markers in string-literals/comments aren't mis-captured; missing anchor → warning or whole-file fallback | Avoidance + render-time verification (`mdbook test` runs included code) | N/A (no line numbers) |
| **Sphinx `literalinclude :start-after:/:end-before:`** | string markers bound the region | Avoidance | N/A |
| **GitHub commit-SHA permalink** | line anchor pinned to immutable commit SHA | Avoidance (frozen snapshot) | N/A (line is correct *for that SHA* but goes stale vs. HEAD) |
| **vsdd-factory `validate-stable-anchors` (current)** | forbids `*.<ext>:NNN` in spec prose, directs to symbol names | **Avoidance only** | NO — has no citation target model |

Source: Perplexity deep-research synthesis (Drift [1][9], mdBook [3][14], avoidance-vs-detection framing).

### Key takeaway for the verdict

The engine has the **avoidance** half (TD-VSDD-091 + `validate-stable-anchors`) but
**not the detection** half. The issue's reframed scope (a checker that catches
wrong-but-plausible-line drift on the *justified/exempted* line citations that remain
legal) is genuinely unbuilt. Symbol-anchoring is "largely a formatting change" for
VSDD because BCs already name the symbol in prose (e.g. `TcpReassembler::new`).

---

## Verdict

**VALID-PARTIAL** — Confidence: **HIGH**

Rationale:
- The **convention** sub-ask is ~80% already satisfied by TD-VSDD-091 +
  `validate-stable-anchors` (avoidance of raw line cites; symbol anchors mandated).
  This is the load-bearing "Decide the convention" step (issue step 1), and it is
  largely DONE for prose.
- The **checker** sub-ask is genuinely **NEW**: there is no positive
  citation-resolution / drift-detection checker, and no CI sweep gate. The
  wrong-but-plausible-line failure mode — the issue's whole reframing — is uncaught
  today, specifically for the *exempted/justified* line citations (Red Gate tables, AC
  SoT tables, pass-report changelogs, VP-079 SITES arrays) that TD-VSDD-091 and the
  hook deliberately permit. Those are exactly the cites that drift with zero
  mechanical detection.
- Residual against the issue's own framing: even the best off-the-shelf tool
  (Fiberplane Drift) does not catch the wrong-but-plausible-*content* case where the
  symbol/anchor region is intact but its meaning changed — so "adopt Drift and we're
  done" is itself an over-claim; a residual semantic-review obligation remains.

NOT `ALREADY-DONE`: the avoidance mechanism does not satisfy the detection ask.
NOT `INVALID`: the gap is real, independently attested by vsdd-factory's own TD-031.

---

## Recommended Approach (zero re-research)

**Owning agents/assets:** architect (convention decision/ADR) → research-agent
(adopt-vs-build eval of Drift's current version/license) → implementer (checker) →
devops-engineer (CI wiring). New asset: a spec-CI citation checker (WASM hook OR
CI step) + extension of TD-VSDD-091.

### Scope decomposition

1. **Convention (mostly done — ratify the residual).** Confirm the canonical rule for
   the *exempted* citation classes that still use line numbers (Red Gate tables, AC
   SoT tables, pass-report changelogs, SITES arrays). Production-grade default:
   require those to carry a **commit-SHA pin** (GitHub-permalink style) so the cite is
   verifiable-against-a-known-snapshot, OR migrate them to symbol+offset anchors.
   Record as an amendment to TD-VSDD-091 / a new D-NNN. Do NOT leave this as "pending
   architect review" — pick SHA-pinning as the default and write the rationale inline
   (CLAUDE.md Canonical Principle Rule 6).

2. **Checker — adopt-vs-build.** The issue's `wirerust` validation already flagged
   the build-a-Rust-binary path as a symptom-fix and a precedent violation (the repo's
   `bin/` helpers are dependency-free shell; **no compiled-binary precedent** — but
   note vsdd-factory DOES have a WASM-plugin precedent, so a WASM checker is on-pattern
   here, unlike a standalone `bin/` binary). Two production-grade options:
   - **(a) Adopt Fiberplane Drift** as the positive drift-detector. Verify current
     version/license/maintenance against [github.com/fiberplane/drift] at adoption
     time (issue cites MIT v0.10.0 — re-verify; do not trust the issue's pinned
     version). Pros: AST-hash catches wrong-but-plausible-line; Claude Code agent
     integration already exists; `drift check` non-zero exit drops straight into CI.
     Cons: external binary dependency in the plugin distribution; AST-hash needs
     tree-sitter grammars per language.
   - **(b) Build a WASM checker** (on-pattern with the existing hook ecosystem) that
     *resolves* each remaining line citation against its current source file: for a
     SHA-pinned cite, verify the cited line content matches the content at that SHA;
     for a symbol cite, verify the symbol still exists. This closes the
     wrong-but-plausible-line gap that `validate-stable-anchors` structurally cannot.
   - **Recommendation:** evaluate (a) first (the issue's reframing favors it; it is
     the only option that catches the dominant drift mode without bespoke parser work),
     fall back to (b) scoped to SHA-pin verification if an external binary is
     unacceptable in the marketplace tarball.

3. **CI gate.** Wire the chosen checker as a spec-CI step analogous to
   `cargo fmt --check` (`.github/workflows/ci.yml`), exit non-zero on stale citation.
   Do NOT bypass the existing `validate-stable-anchors` hook — the two are
   complementary (avoidance at write-time + detection at CI-time).

4. **Quantify the historical catch rate (issue's open item).** Before finalizing any
   fallback scope, spot-check 5–10 real past `P-CITE-PG`/TD-031 findings to confirm
   what fraction were wrong-but-plausible-line (uncatchable by EOF/blank checks). The
   issue flags this as inconclusive; STATE-style descriptions ("off-by-one citations",
   "line shifts") suggest the majority. This is evidence-gathering, not a blocker.

### Key files

- `crates/hook-plugins/validate-stable-anchors/src/lib.rs` — existing avoidance hook
  (reference for the new detection checker's scope boundary; do NOT duplicate).
- `plugins/vsdd-factory/hooks-registry.toml` § `validate-stable-anchors` — registration
  pattern for a new WASM checker if option (b).
- `CLAUDE.md` § TD-VSDD-091 — convention amendment site (SHA-pin ratification).
- `.github/workflows/ci.yml` — CI gate wiring site.
- `bin/` — dependency-free shell helper precedent (relevant if a non-WASM fallback).

### Risks / dependencies

- **Scope-creep risk:** "build a generic citation checker" is partly language-specific
  (EOF/blank are language-neutral; comment-detection and symbol-proximity need a
  parser). Constrain v1 to SHA-pin verification (language-neutral) if building.
- **Dependency:** option (a) adds an external binary to the marketplace tarball —
  architect decision (distribution policy), not an AI default. Surface, don't decide.
- **Exemption-zone interplay:** any new checker MUST respect the same exemption zones
  as `validate-stable-anchors` (Amendment/Changelog/SITES) or it will fight the
  existing hook.
- **Non-blocking:** no active pipeline gate depends on this; the issue itself satisfies
  the `P-CITE-PG` mandatory-codification requirement.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (`reasoning_effort=high`) | Drift-resistant citation conventions: Fiberplane Drift mechanism/CI/maturity, mdBook anchors, Sphinx markers, GitHub permalinks; avoidance-vs-detection distinction; wrong-but-plausible-line failure mode |
| Read | 4 | issue body; `validate-stable-anchors/src/lib.rs`; CLAUDE.md TD-VSDD-091; reference research file |
| Grep | 6 | `volatile-pin`/`TD-VSDD-091` repo sweep; hooks-registry registration; decision-log/CHANGELOG closure check; deep-research extraction |
| Training data | 0 areas | All tool claims sourced from live deep-research; version/license flagged for re-verification at adoption |

**Total MCP tool calls:** 1 (shared across the 3-issue batch; this issue's findings drawn from the dedicated citation-checker deep-research call)
**Training data reliance:** LOW — convention/tool claims are externally sourced; Fiberplane Drift version (v0.10.0) and license (MIT) are issue-asserted and explicitly flagged for re-verification before adoption.
