# Issue #177 — Hollow-Demo / False-Confidence Checker (agent + skill + gate)

**Date:** 2026-06-09
**Issue:** #177 — "Add a hollow-demo / false-confidence checker (agent + skill + gate integration)"
**State:** OPEN, no labels
**Reviewer:** research-agent (cluster: review/adversarial-quality)
**Repo:** vsdd-factory @ develop `82163b7f`

---

## Restated Question

Should vsdd-factory add a first-class **hollow-demo / false-confidence checker** — an agent
(`hollow-demo-checker`) plus skill (`/hollow-demo-sweep`) plus gate-integration — that detects
artifacts which *execute and pass* but do not actually establish the behavior they claim?
The thesis: the adversary catches defects *within* the perimeter it's shown, but hollow demos
survive because the perimeter itself (the test/demo) is vacuous. The issue cites a real cycle
where a codebase-wide sweep found 3 CRITICAL + 14 IMPORTANT hollow-demo issues in already-merged
code that had each passed per-story adversarial convergence (3+ clean passes).

---

## Codebase Grounding

### What already exists (partial coverage)

The repo has **substantial latent coverage of this taxonomy**, but no dedicated agent/skill/sweep:

1. **Adversary "CI-as-Code Positive-Coverage Assertion" review axis** —
   `plugins/vsdd-factory/agents/adversary.md` (lines 245–279) is, in effect, a hollow-demo
   detector for CI jobs. It mandates: "exit code is necessary but **insufficient**"; requires a
   machine-greppable positive-coverage log line (`Check passed: N items validated`, N non-zero,
   runtime-computed); flags `echo "All passed"` with no counted inputs as "a false-green
   generator"; and cites the real prism PR #127 case where a regex matched zero symbols for **12
   consecutive converged passes**. Severity: MEDIUM→HIGH with `[process-gap]`. Gating policy is
   **POL-11 (`ci_positive_coverage_assertion`)**. This directly addresses the issue's
   "fail-open gate-critical metrics" and "zero-assertion test_* inflating green count" categories.

2. **Adversary "Silent failures (SOUL.md #4)" implementation-review item** —
   `agents/adversary.md` line 91: "Silent failures — can errors be swallowed? (SOUL.md #4)".
   `CLAUDE.md` Standing Rule 3 §2 forbids "Silent `Vec::new()` return where partial-failure data
   should propagate." This overlaps the issue's "silent fallbacks masking unmet behavior" category
   but is a general adversary heuristic, not a dedicated read-only sweep with a disclosure
   discriminator.

3. **Red-Gate density machinery (TDD discipline)** — `workflows/phases/per-story-delivery.md`
   "Red Gate Density Check" (RED_RATIO ≥ 0.5), GREEN-BY-DESIGN / WIRING-EXEMPT exemption taxonomy,
   `hooks/validate-red-ratio.sh`, `tests/tdd-discipline-gate.bats`, and the stub-architect
   self-check ("If I include this real implementation, will the test for this function pass
   trivially without any implementer work?"). This is a *prospective* anti-hollow-test gate (it
   prevents tests that pass against a stub) but does NOT detect hollow demos in **already-merged**
   code, which is the issue's core scenario.

4. **Mutation testing wave-gate** — `skills/wave-gate/SKILL.md` requires
   `cargo mutants ... --timeout 300` with ≥80% kill rate for facade-mode crates
   (BC-6.21.001/002). Mutation testing is the industry-canonical hollow-test detector (see
   research), but it is currently scoped to facade-mode crates and the wave gate, not a
   codebase-wide post-merge sweep.

5. **demo-recorder + `validate-demo-evidence-story-scoped` hook** —
   `hooks-registry.toml` line 213; demo-recorder records per-AC evidence. The hook validates
   evidence is story-scoped but does NOT assert the demo exercises the *real* path vs. canned
   fixtures (issue #162 proposes strengthening this for running-system evidence; overlaps here).

6. **maintenance-sweep skill** — `skills/maintenance-sweep/SKILL.md` has 9 sweeps (dependency,
   doc-drift, pattern, holdout, perf, DTU, spec-coherence, tech-debt, a11y). **There is no
   hollow-demo / test-adequacy sweep** — a clean insertion point for the proposed periodic audit.

### What does NOT exist (the genuine gap)

- No `agents/hollow-demo-checker.md` (confirmed via Glob of `agents/**/*.md`).
- No `skills/hollow-demo-sweep/SKILL.md` (confirmed via Glob of `skills/**/SKILL.md`).
- No findings-register template for hollow-demo taxonomy.
- No taxonomy reference doc enumerating: vacuous tests, silent fallbacks, fail-open metrics,
  fabricated/placeholder data, demos-not-exercising-real-path, lookahead/leakage,
  claimed-but-undemonstrated AC/BC, with a DISCLOSED-vs-UNDISCLOSED discriminator.
- No lint-style hook AST-scanning for assertion-free `test_*`, bare `except`/swallowed errors,
  or None/NaN coercion of gate metrics.
- No wiring of a hollow-demo lens into per-story-delivery Step 4.5, wave-gate, Phase 5, or
  maintenance-sweep.

### Prior-closure check

Grep of `.factory/` and `CHANGELOG.md` for `hollow|false-confidence|vacuous|value oracle` →
**no prior decision (D-NNN), lesson, or changelog entry** closing this. The closest codified
artifact is POL-11 (CI positive-coverage) and TD-VSDD-057 (prism false-green origin) — both are
*subset* coverage, not the general capability.

---

## External Research

Primary research call: `perplexity_research` (reasoning_effort=high), saved at
`.../tool-results/toolu_01PwPpTfUW3eUxNhKbNNU578.txt`. Perplexity flagged limited live-search
hits and synthesized from its corpus; the following are established, well-attested findings
(treat specific percentages as indicative, not primary-sourced measurements).

### Soundness of the problem (CONFIRMED, strong prior art)

- **"Hollow / vacuous test" is a recognized class.** Categories: assertion-free tests (pass by
  default because xUnit frameworks treat no-exception as pass), and **tautological tests** that
  "pass for *any* implementation — including completely broken ones." The canonical example is
  the issue's exact complaint: asserting output equals a hand-built expected array constructed to
  match the input, rather than verifying a general property. The epistemological flaw (Fraser &
  Arcuri test-adequacy framing): "meaningful tests must derive expected outcomes from domain
  knowledge external to the implementation, not from the code being tested itself."
- **"Test smells" taxonomy** (van Deursen et al.): Mystery Guest, Conditional Test Logic,
  Assertion Roulette — structural anti-patterns that correlate with hollow tests.
- **Fail-open / silent-fallback anti-pattern is a distinct, documented failure class** —
  "gate metrics or tests erroneously pass when essential validation mechanisms are unavailable."
  The 2014 Apple `goto fail` SSL bug is cited as the archetype (the deeper failure was the test
  suite's inability to detect missing validation). Variants named: *metric absolution* (missing
  data defaults to success threshold), *dependency degradation* (silent mock fallback),
  *assertion short-circuiting*. This is precisely the issue's "fail-open gate-critical metrics"
  and "silent fallbacks (None→0.0, swallowed except)" categories.
- **LLM-generated tests exhibit this at high rates.** The "hand-built fixture" anti-pattern
  decomposes into *input mirroring*, *output spoofing*, and *pipeline bypassing* — verbatim the
  issue's "demos that don't exercise the real path" and "fabricated/placeholder data." Root cause:
  LLMs replicate the *syntactic* pattern of example tests (which use hardcoded I/O for
  reproducibility) while decoupling it from verification purpose; they "optimize for surface-level
  correctness (does the test run?) rather than semantic validity (does it verify intended
  behavior?)."

### Recommended mitigations (CONVERGENT with the issue's design)

- **Mutation testing is the canonical detector.** PIT (Java), Stryker (JS/TS/C#/Scala),
  **cargo-mutants (Rust)** — "if a test passes against a mutant, it fails to verify the mutated
  behavior; a tautological test survives most mutations." This is the gold standard for test
  adequacy but is **computationally expensive (5–15× CI time)**, so it must be strategically
  scoped — consistent with vsdd-factory already gating it at the wave level.
- **Independent value oracle** — replace hardcoded expected values with a dynamically-computed
  reference. This is exactly the issue's "Monte-Carlo p95 drawdown had no independent value
  oracle" finding. Caveat from research: value oracles add development effort and can carry their
  own defects (circular verification); best for "well-bounded algorithms with established
  mathematical foundations."
- **Metamorphic testing** for oracle-deficient domains (verify relationships across executions,
  e.g., `sort(sort(x)) == sort(x)`) — catches hollow tests mutation testing misses.
- **"Truthfulness testing"** (run the test against an *intentionally broken* implementation; the
  test must fail) — the most effective approach specifically for LLM-generated tests; cited at
  ~94% detection of semantically hollow tests. This is the most directly actionable static-plus-
  dynamic technique for the proposed checker and aligns with the issue's central question:
  "what wrong behavior would this test FAIL to catch?"
- **Layered frameworks** (static AST scan → mutation on critical paths → metamorphic on
  consistency relations) outperform any single technique — validates the issue's multi-layer
  proposal (agent + skill + lint hook + gate integration) rather than one mechanism.
- **Disclosure discriminator is sound:** research repeatedly distinguishes *legitimate*
  assertion-free tests (`assertDoesNotThrow`, side-effect/log verification, ~8–12% of Google's
  assertion-free tests serve legitimate purposes) from genuinely vacuous ones. The issue's
  DISCLOSED-acceptable vs. UNDISCLOSED-finding split maps directly onto this and is necessary to
  keep false-positive rates tolerable (naive assertion-scanners hit unacceptable FP rates).

### What I could NOT find

- No off-the-shelf "hollow-demo agent" for LLM agent pipelines to copy wholesale — this would be
  novel tooling. The building blocks (mutation tools, AST linters, metamorphic frameworks) exist;
  the *orchestrated fan-out read-only auditor + disclosure discriminator* is the new composition.
- No authoritative single source for the "demos that don't exercise the real path" detector as a
  static check — it generally requires either taint/dataflow analysis (does the SUT output
  actually flow into the assertion?) or truthfulness testing. Flag this as the hardest sub-part.

---

## Verdict

**VALID-PARTIAL** — Confidence: **HIGH**

The problem is real, well-precedented, and **not closed**. However, it is *not* greenfield: the
adversary's CI-as-Code positive-coverage axis (POL-11), the Red-Gate density machinery, the
stub-architect self-check, and the wave-gate mutation testing already cover meaningful slices of
the taxonomy *prospectively*. The genuine residual gap is:

1. A dedicated **read-only `hollow-demo-checker` agent** with the full 8-category taxonomy +
   disclosure discriminator (the "what wrong behavior would this FAIL to catch?" lens), applied
   to **already-merged / converged** artifacts — the scenario none of the existing gates cover.
2. A **fan-out sweep skill** (`hollow-demo-sweep`) producing a findings register.
3. A **maintenance-sweep #10** insertion + per-story Step 4.5 lens + Phase 5 wiring.
4. An optional **lint hook** for the mechanically-detectable subset (assertion-free `test_*`,
   bare/`except Exception`, None/NaN coercion of gate metrics).

The proposal should be scoped to **reuse, not duplicate**: the new agent should *invoke*
cargo-mutants/truthfulness-testing rather than reimplement adequacy analysis, and the lint hook
should extend (not fork) the POL-11 positive-coverage pattern.

---

## Recommended Approach & Scope (for zero re-research later)

### Owning agents/skills
- **New agent:** `vsdd-factory:hollow-demo-checker` (read-only: Read/Grep/Glob/Bash). Models on
  `agents/adversary.md` structure (information-asymmetry, confidence levels, self-validation loop)
  but with a fixed 8-category taxonomy and a DISCLOSED/UNDISCLOSED discriminator. Routed by the
  orchestrator like the adversary. **Authoring owner:** the human + architect decide whether this
  is a new agent or a new *review mode* on the existing adversary (see Risks). Skill authoring →
  `product-owner`/architect per routing table for the spec; implementation of the lint hook →
  `implementer` (Rust WASM in `crates/`) per #162-style hook discipline.
- **New skill:** `skills/hollow-demo-sweep/SKILL.md` + `taxonomy-reference.md` +
  `findings-register-template.md` (mirror maintenance-sweep's fan-out + register pattern).

### Key files to touch
- `plugins/vsdd-factory/agents/hollow-demo-checker.md` (new)
- `plugins/vsdd-factory/skills/hollow-demo-sweep/SKILL.md` (+ taxonomy + register template) (new)
- `plugins/vsdd-factory/skills/maintenance-sweep/SKILL.md` (add Sweep 10: Hollow-Demo / Test
  Adequacy)
- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` (add hollow-demo lens to Step 4.5
  — "every AC needs a *substance* assertion, not just 'executes'")
- `plugins/vsdd-factory/skills/phase-5-adversarial-refinement/SKILL.md` (system-level sweep wiring)
- Optional lint hook source in `crates/hook-plugins/` + `hooks-registry.toml` registration +
  `tests/` bats coverage (follow validate-red-ratio precedent). **Requires a release** for
  operator-level pickup (CLAUDE.md dispatcher discipline).
- `.factory/policies.yaml` — consider a POL for "substance assertion required per AC" sibling to
  POL-11.

### Approach
1. Author taxonomy-reference.md (8 categories + disclosure discriminator) as the single source of
   truth; have the agent and lint hook both cite it.
2. `hollow-demo-checker` operates per-subsystem read-only; for each test/demo it answers the core
   question and, where feasible, *invokes* `cargo mutants -p <crate>` and/or a truthfulness probe
   (swap impl for a constant/broken stub, confirm the test goes red). Findings tagged
   DISCLOSED-acceptable vs. UNDISCLOSED + severity + remediation grouping.
3. `hollow-demo-sweep` fans out one checker per subsystem, synthesizes a findings register.
4. Lint hook handles only the *mechanically* detectable subset to keep it fast and fail-open-safe.

### Risks
- **Agent proliferation vs. adversary overload.** Adding a 4th review perimeter risks confusion
  with the adversary's three-perimeter contract (BC-5.39.001/002). Strongly consider implementing
  this as a **named review mode / review axis on the existing adversary** plus a standalone sweep
  skill, rather than a wholly separate agent — surface this routing question to orchestrator/human.
- **False-positive rate.** Naive assertion-scanning has high FP; the disclosure discriminator and
  taint/dataflow awareness are mandatory, not optional. Mutation testing is the precise-but-
  expensive backstop.
- **Cost.** Mutation/truthfulness probes are 5–15× CI cost — scope to changed/critical paths;
  reuse the wave-gate mutation budget rather than adding a second full run.

### Dependencies / overlaps
- **#162** proposes strengthening `validate-demo-evidence-story-scoped` to require running-system
  evidence (anti-"canned output") — directly complementary to this issue's "demos that don't
  exercise the real path." Coordinate the demo-evidence hardening once.
- **POL-11 / TD-VSDD-057** (CI positive-coverage) is the existing nucleus; extend, don't fork.
- Wave-gate mutation testing (BC-6.21.x) is the existing adequacy backstop; reuse its budget.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared) | Hollow/vacuous test detection, fail-open anti-patterns, LLM hand-built-fixture tests, mutation/metamorphic/value-oracle/truthfulness mitigations |
| Read | 8 | adversary.md, per-story-delivery.md, tdd-discipline-gate.bats, maintenance-sweep, implementation-readiness, story-completeness, phase-1d skill, hooks-registry slice |
| Grep / Glob | 9 | hollow/uncertainty/red-gate/PreToolUse sweeps across plugins + .factory + CHANGELOG |
| Training data | 1 area | Mapping issue taxonomy to known test-smell literature (cross-checked against Perplexity synthesis) |

**Total MCP tool calls:** 1 deep research (shared across #177/#162) + supporting reads.
**Training data reliance:** LOW–MEDIUM — problem-class and mitigation findings corroborated by
Perplexity synthesis; codebase claims are all file-grounded. Specific percentages from the
research are indicative (Perplexity noted limited live-search) and flagged as such.
