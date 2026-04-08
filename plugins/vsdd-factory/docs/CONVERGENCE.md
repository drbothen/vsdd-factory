# Convergence Criteria

Formalized, quantitative criteria for five-dimensional convergence in the Dark Factory VSDD pipeline. These criteria replace subjective assessments ("adversary says it's done") with measurable thresholds grounded in finding decay curves, mutation kill rates, formal proof coverage, and cost-benefit analysis.

All criteria are ADVISORY. The human operator can override any convergence assessment. When overriding a NOT_CONVERGED assessment, the override and its rationale are recorded in the convergence report.

---

## Dimension 1: Spec Convergence

Objective criteria for when adversarial spec review has converged.

### Finding Novelty Score

For each adversarial review pass i, track:

- **N(i)** = number of genuinely new findings (not variations of prior findings)
- **D(i)** = number of duplicate or minor-variant findings

Novelty score:

```
Novelty(i) = N(i) / (N(i) + D(i))
```

**Converged when:** Novelty < 0.15 for 2 or more consecutive passes.

A novelty score below 0.15 means 85%+ of the adversary's output consists of duplicates or minor variations of previously identified issues. The adversary has exhausted its ability to find genuinely new spec gaps.

### Severity Distribution Shift

Track median finding severity per pass on a 1-5 scale (1 = cosmetic, 5 = critical).

**Converged when:** Median severity has been strictly decreasing for 3 or more consecutive passes AND current median severity is below 2.0.

This signals that not only are findings becoming rarer, but the findings that remain are low-impact -- the adversary can no longer identify substantive spec gaps.

### Finding Similarity

Use semantic similarity (via embedding model) to compare each new finding against the full corpus of prior findings.

**Converged when:** Average similarity of new findings to prior corpus exceeds 0.75 (on a 0-1 scale).

When similarity exceeds 0.75, the adversary is producing findings that are minor restatements of previously identified issues -- a strong signal that the novelty space is exhausted.

### Reviewer Confidence Scoring

Each adversary finding includes a self-reported confidence score (0.0-1.0). Track average confidence across passes:

```
Pass 1: avg confidence 0.92 (high-confidence findings — real issues)
Pass 2: avg confidence 0.87
Pass 3: avg confidence 0.71
Pass 4: avg confidence 0.54 (low confidence — reviewer struggling)
Pass 5: avg confidence 0.48
```

**Signal:** When average confidence drops below 0.55 and remains there for 2+ passes, the reviewer is likely hallucinating findings. This is a strong convergence signal independent of novelty scoring.

The `convergence-tracker` plugin (DF-009) extracts confidence scores from adversary output using the `tool_result_persist` hook, annotating each finding before it's written to the session transcript.

### Hallucination Detection & Fingerprinting

Track finding verification success rate -- what percentage of adversary findings are independently confirmable through a second verification modality (different model, static analysis, or manual inspection)?

**Signal:** When verification rate drops below 60%, the adversary is operating in a hallucination-prone regime. Findings should be treated with skepticism, and spec convergence is likely achieved.

A legitimate finding should be reproducible through independent verification. If the adversary claims a spec ambiguity but no other reviewer or analysis tool can confirm it, the finding is likely hallucinated.

**Hallucination fingerprinting:** LLM-hallucinated findings have statistical signatures that can be detected automatically by the `convergence-tracker` plugin (DF-009):

| Fingerprint | Detection Method |
|-------------|-----------------|
| References non-existent code (e.g., "line 47" in a 40-line file) | Parse finding locations against actual file lengths |
| References techniques/libraries not in the codebase | Grep for referenced identifiers |
| Contradicts findings from a previous pass by the same reviewer | Semantic comparison against prior finding corpus |
| Suggests fixes that would introduce the problem it claims to find | Static analysis of suggested fix |

Findings matching 2+ fingerprints are classified as "probable hallucination" and their weight in the Convergence Index is halved. The `sidecar-learning` plugin (DF-009) tracks per-model hallucination patterns across pipeline runs, building a fingerprint database so known hallucination types can be auto-discounted in future passes.

### Multi-Model Adversary Diversity Bonus

When adversarial review passes use multiple model families (e.g., GPT-5.4 primary + Gemini 3.1 Pro secondary), findings from distinct model families carry higher weight for novelty assessment. A finding is classified as NEW if it was not identified by the other model family, even if it is semantically similar to a finding from the same model family. This incentivizes multi-model review by rewarding the cognitive diversity it provides.

Track per-model finding overlap:
- **Cross-model unique findings:** Findings from Model B not present in Model A's corpus (high value -- different blind spots)
- **Cross-model confirmed findings:** Same finding from both models (high confidence -- independent confirmation)
- **Single-model unique findings:** Finding from only one model that the other model could not confirm (moderate confidence -- may be hallucinated or may be a genuine blind spot)

**Convergence with multi-model adversary:** Both the primary adversary (GPT-5.4) AND the secondary adversary (Gemini 3.1 Pro, review tier) must independently report cosmetic-only findings before spec convergence is declared.

---

## Dimension 2: Test Convergence

Objective criteria for test suite adequacy, measured through mutation testing.

### Mutation Kill Rate by Module Criticality

Target kill rates vary by module risk classification:

| Criticality | Target Kill Rate | What Qualifies |
|-------------|-----------------|----------------|
| **CRITICAL** | >= 95% | Security boundaries, cryptographic operations, access control, financial calculations, state machines that can reach invalid states |
| **HIGH** | >= 90% | Core business logic, data transformation pipelines, API contracts, validation logic |
| **MEDIUM** | >= 80% | Application integration code, standard features, configuration loading |
| **LOW** | >= 70% | Utilities, helpers, formatting, logging |

**Converged when:** All modules meet their criticality-tier kill rate AND the overall kill rate has increased by less than 2 percentage points in the last review iteration.

### Surviving Mutant Classification

Not all surviving mutants indicate test gaps. Classify survivors into:

| Category | Description | Action |
|----------|-------------|--------|
| **Equivalent mutant** | Mutation produces functionally identical behavior (e.g., `a > 100` to `a >= 101` for integer a) | Exclude from kill rate calculation |
| **Dead code** | Mutation in unreachable code paths | Flag for removal; exclude from score |
| **Insufficient assertions** | Test executes the code but does not verify the precise output | Write stronger assertions |
| **Complex logic** | Change to complex conditional logic not covered by test design | Add targeted test cases |

**Converged when:** Surviving mutants are predominantly equivalent mutants or dead code (categories 1-2 represent >80% of survivors).

### Property-Based Test Coverage

All invariants from the Provable Properties Catalog (generated in Phase 1) must have corresponding property-based test strategies generating >= 1000 random cases each.

**Converged when:** 100% of cataloged invariants have property tests AND all property tests pass.

---

## Dimension 3: Implementation Convergence

Objective criteria for when adversarial code review has converged.

### Code-to-Issue Grounding

Every adversary finding MUST reference specific, verifiable code -- not vague descriptions. The `convergence-tracker` plugin (DF-009) validates grounding automatically:

- **File path:** Must reference a file that exists in the repository
- **Line range:** Must reference lines that exist in the referenced file
- **Reproduction path:** Must describe how to trigger the issue (test command, API call, or code path)

Findings without code grounding are classified as "ungrounded" and their weight in the Convergence Index is **halved**. This prevents adversary agents from generating plausible-sounding but unverifiable criticisms.

**Grounding verification uses symbolic analysis** (grep, AST parsing) rather than LLM reasoning:
```
Finding: "Variable 'count' is never decremented"
Verification: Extract all assignments to 'count' in the file
  If decrements exist -> finding is FALSE (auto-dismiss)
  If only increments exist -> finding is TRUE (confirmed)
```

### Adversary Finding Verification Rate

Track what percentage of the adversary's code review findings are confirmed real versus hallucinated, using independent verification (different model, static analysis tool, or manual inspection).

**Converged when:** Verification rate drops below 60%. Below this threshold, the adversary is inventing problems that do not actually exist in the code -- genuine convergence signal.

### Finding Decay Curve

Finding count across adversarial passes follows a power law decay pattern. For each pass i, record total finding count F(i).

**Converged when:** The power-law fit projects the next iteration will yield fewer than 0.5 expected findings. Empirical research shows post-improvement discovery rates follow power law with exponent c ~ 0.11 (R-squared ~ 0.93).

### Convergence Index

A composite metric combining novelty, similarity, severity, and cost:

```
CI(i) = (Novelty(i) * (1 - AvgSimilarity(i)) * (6 - MedianSeverity(i))) / Cost(i)
```

Where:
- Novelty(i) = fraction of genuinely new findings (0.0 to 1.0)
- AvgSimilarity(i) = mean semantic similarity of new findings to prior corpus (0.0 to 1.0)
- MedianSeverity(i) = median finding severity on 1-5 scale
- Cost(i) = iteration cost in dollars (from LiteLLM cost tracking)

**Converged when:** CI < 0.3 AND CI has been declining for 3 or more consecutive iterations.

Interpretation: when the index is low and falling, the adversary is producing increasingly redundant, low-severity findings at increasing cost -- the definition of diminishing returns.

---

## Dimension 4: Verification Convergence

Objective criteria for formal verification completeness.

### Kani Proof Coverage

All proof harnesses defined in the Provable Properties Catalog must pass:

- All bounded proofs pass at required depth
- Proof core coverage exceeds 75% of assertion cone-of-influence
- No counter-examples discovered at any proof depth up to the bound

**Converged when:** 100% of proof harnesses pass AND proof core coverage > 75%.

### Fuzz Testing Saturation

Fuzz campaigns must demonstrate saturation -- the point where new test inputs no longer exercise previously-untested code paths in potentially-vulnerable regions.

**Converged when:** No novel crashes after 5 minutes of continuous fuzzing per fuzz target AND coverage of potentially-vulnerable functions (identified by static analysis) has stabilized.

### Security Scans

Static and dependency analysis must be clean:

- Zero critical or high findings from Semgrep (or equivalent SAST tool)
- Zero known CVEs in dependencies (cargo audit clean, or language equivalent)

**Converged when:** Both conditions are met.

### Purity Boundary Integrity

All modules classified as "pure core" in the architecture document must be verified free of side effects:

- No I/O operations
- No global state mutation
- No non-deterministic behavior

**Converged when:** Purity audit confirms all pure-core modules are effect-free.

---

## Dimension 5: Holdout Scenario Convergence

Objective criteria for behavioral validation through information-asymmetric testing.

### Holdout Satisfaction Score

Holdout scenarios are acceptance criteria hidden from the builder and test-writer agents, evaluated by an independent agent (different model family, fresh context) against the running system using **satisfaction scoring** (0.0-1.0) instead of binary pass/fail. This enforces train/test separation: the builder sees the spec (training data), the holdout evaluator sees scenarios (test set).

Satisfaction scoring replaces binary pass/fail because agents game rigid assertions -- they return `true` to pass narrowly-written tests rather than generalizing. Satisfaction uses LLM-as-judge evaluation against behavioral rubrics, capturing partial correctness and intent alignment.

**Converged when:** Mean satisfaction score >= 0.85 across all scenarios AND all `must-pass` scenarios have satisfaction >= 0.6 AND satisfaction standard deviation < 0.15 (indicates consistent quality, not a few perfect scores masking failures).

### Holdout Stability

Track holdout satisfaction scores across implementation iterations. If scores oscillate (fixes to one scenario lower satisfaction on another), the implementation has coupling problems.

**Converged when:** Mean satisfaction score has been monotonically non-decreasing for 2+ iterations.

### Holdout-Test Alignment

Compare holdout scenario coverage against the unit test suite. If holdout scenarios with low satisfaction correspond to untested code paths, the unit test suite has gaps.

**Signal:** When low-satisfaction scenarios (<0.6) correspond to untested code paths, flag for the test-writer to add targeted unit tests. This feedback loop strengthens the test suite over time without revealing the holdout scenarios themselves.

### Behavioral > Semantic Weighting

In convergence assessment, holdout satisfaction (behavioral validation) should be weighted **more heavily** than adversarial code review findings (semantic validation). The rationale follows StrongDM's insight that code should be treated like ML model weights -- correctness derives from observable external behavior, not code inspection.

Practical implications:
- An implementation that achieves 0.92 mean holdout satisfaction but has 3 medium-severity adversary findings about code style is **closer to convergence** than one with clean adversary review but 0.75 mean satisfaction
- When Dimension 3 (Implementation/adversary) and Dimension 5 (Holdout/satisfaction) conflict, Dimension 5 takes precedence
- The convergence tracker (DF-009) should flag a situation where satisfaction is low but adversary reports convergence as a **higher-priority concern** than vice versa
- This does NOT mean ignoring adversarial findings. Security findings, spec fidelity issues, and architectural violations remain blocking regardless of holdout scores

---

## Progressive Autonomy (Auto-Merge Graduation)

The pipeline starts with human approval at every gate. As confidence builds, gates can be progressively automated based on tracked metrics.

### Autonomy Levels

| Level | Description | Human Role | Criteria to Advance |
|-------|-------------|-----------|-------------------|
| **2** | AI writes code, human reviews all PRs | Review every change | Default starting level |
| **3** | AI generates from specs, quality gates, human approves merge | Approve at phase boundaries | N/A (DF-003 delivers this) |
| **3.5** | Level 3 + auto-advance on qualifying phases | Approve only at Phase 1/2/6 gates | See below |
| **4** | Full dark factory: specs in, tested code merged, existing pipeline deploys | Write specs and holdout scenarios only | See below |

### Composite Autonomy Score

Rather than requiring ALL metrics to independently pass, compute a weighted composite score that balances multiple quality signals:

```
AutonomyScore = (SatisfactionScore x 0.30) + ((1 - FalsePositiveRate) x 0.20) +
                ((1 - OverrideRate) x 0.20) + (ConvergenceSpeed x 0.15) +
                ((1 - RegressionRate) x 0.15)
```

Where:
- **SatisfactionScore** = mean holdout scenario satisfaction score (0.0-1.0, replaces binary pass rate)
- **FalsePositiveRate** = fraction of high-satisfaction scenarios where code was actually broken
- **OverrideRate** = fraction of auto-approved code that humans rejected
- **ConvergenceSpeed** = fraction of runs converging in <=3 passes (normalized 0.0-1.0)
- **RegressionRate** = fraction of runs with regressions

**Level graduation triggers at AutonomyScore >= 0.85** sustained over 20 consecutive runs. This prevents lucky streaks from triggering promotion -- the score must be consistently high.

**Fast revocation, slow promotion:** If AutonomyScore drops below 0.70 for any 5-run window, autonomy is automatically revoked by one level. Promotion requires 20 sustained runs; revocation triggers on 5 bad runs. This fail-safe principle ensures trust is earned gradually but lost immediately when quality drops.

The `sidecar-learning` plugin (DF-009) tracks these metrics across pipeline runs and the `convergence-tracker` plugin computes the composite score after each run.

### Metrics for Level 3 -> 3.5 (Auto-Advance Qualifying Phases)

Track these metrics over the last 20 pipeline runs. When AutonomyScore >= 0.85 AND ALL individual thresholds are met, the human can enable auto-advance for Phases 3-5 (implementation through hardening):

| Metric | Threshold | Measurement |
|--------|-----------|-------------|
| **Holdout satisfaction** | Mean >= 0.85 for 20 consecutive runs, std dev < 0.15 | `.factory/holdout-evaluation/summary.md` |
| **False positive rate** | < 5% (high satisfaction but code broken) | Human override log |
| **Human override rate** | < 10% (human rejects auto-approved code) | Human override log |
| **Adversarial convergence speed** | Converges in <= 3 passes for 15/20 runs | Phase 4 pass count |
| **Regression rate** | Zero regressions in last 20 runs | Phase F4 regression logs |

### Metrics for Level 3.5 -> 4 (Full Autonomy)

All Level 3.5 metrics PLUS:

| Metric | Threshold | Measurement |
|--------|-----------|-------------|
| **Spec approval rate** | Human approves > 95% of Phase 1 specs without changes | Phase 1 approval log |
| **Story approval rate** | Human approves > 95% of Phase 2 stories without changes | Phase 2 approval log |
| **Production incident rate** | Zero production incidents from auto-merged code in 30 days | Incident tracker |
| **Holdout scenario quality** | < 5% of holdout scenarios need revision per pipeline run | Scenario revision log |

### Configuration

Auto-advance is a single configuration change per phase:

```yaml
# .factory/autonomy-config.yaml
autonomy_level: 3  # Current level (human sets this)

phase_gates:
  phase_1_spec:     human     # Always human at Level 3
  phase_2_stories:  human     # Always human at Level 3
  phase_3_impl:     human     # -> "auto" at Level 3.5
  phase_3.5_holdout: auto     # Always auto (metric-gated)
  phase_4_adversary: human    # -> "auto" at Level 3.5
  phase_5_hardening: auto     # Always auto (metric-gated)
  phase_6_converge:  human    # -> "auto" at Level 4

# Any team member can still block before merge window closes
merge_window_minutes: 30
```

### Override and Rollback

- Any human can override auto-advance at any time (emergency brake)
- Overrides are logged with timestamp and rationale
- If human override rate exceeds 10% over any 10-run window, auto-advance is automatically disabled for that phase and reverts to human approval
- This creates a self-correcting system: trust is earned incrementally and revoked automatically when quality drops

---

## Maximum Viable Refinement (Cost-Benefit Exit)

Additional review should continue only when the expected value of findings exceeds review cost. The stopping rule:

```
Continue when: P(finding in iteration N) * Value_avg > Cost_iteration * 1.5
Stop when:     P(finding in iteration N) * Value_avg <= Cost_iteration * 1.5
```

Where:
- P(finding in iteration N) = probability of at least one finding, estimated from the finding decay curve
- Value_avg = average value of a finding at current severity levels (calibrated from historical data)
- Cost_iteration = compute + human review cost for the next iteration (from LiteLLM metering)
- 1.5x multiplier = safety factor to account for estimation uncertainty

**Rule of thumb:** If projected iteration yields < 0.5 findings AND iteration cost > $100, stop.

This is the economic guardrail. Even if formal convergence thresholds have not been met, continuing review past this point is economically irrational. Document the override in the convergence report.

---

## Module Criticality Classification Guide

### How to Classify Modules

Every module in the target project must be classified into one of four criticality tiers. Classification drives mutation kill rate targets, formal verification depth, and adversarial review intensity.

#### CRITICAL

Modules where a defect could cause security breach, data loss, financial harm, or violation of safety invariants.

Examples:
- Security boundaries (authentication, authorization, access control)
- Financial calculations (pricing, billing, transaction processing)
- Cryptographic operations (key management, signing, encryption)
- State machines that can reach invalid states (workflow engines, protocol handlers)
- Tenant isolation logic

Target: >= 95% mutation kill rate, Kani proofs required, maximum adversarial review passes.

#### HIGH

Modules implementing core business logic where a defect would cause incorrect results or broken contracts.

Examples:
- Core business logic (domain rules, business calculations)
- Data transformation pipelines (ETL, data normalization)
- API contracts (request validation, response serialization)
- Validation logic (input parsing, constraint enforcement)

Target: >= 90% mutation kill rate, Kani proofs recommended for key invariants, standard adversarial review.

#### MEDIUM

Modules implementing standard application features where a defect would cause user-visible misbehavior but no data loss or security impact.

Examples:
- Application integration code (third-party API clients, plugin loaders)
- Standard features (search, filtering, sorting, pagination)
- Configuration loading and parsing
- Caching logic

Target: >= 80% mutation kill rate, property-based tests required, standard adversarial review.

#### LOW

Modules where a defect would cause cosmetic issues or minor inconvenience.

Examples:
- Utilities and helpers (string formatting, date formatting)
- Logging infrastructure
- CLI output formatting
- Documentation generation

Target: >= 70% mutation kill rate, unit tests sufficient, minimal adversarial review.

### Classification Process

1. **Default to MEDIUM.** Start every module at MEDIUM and adjust based on evidence.
2. **Promote to CRITICAL** if the module handles authentication, authorization, financial data, cryptographic keys, or can reach invalid states.
3. **Promote to HIGH** if the module implements core business rules or API contracts.
4. **Demote to LOW** if the module is purely cosmetic, logging, or utility code with no business logic.
5. **Document the classification** in `.factory/module-criticality.md` with rationale for each module.
6. **Review classifications** at the start of each adversarial pass -- the adversary may identify modules that should be reclassified.

---

## Convergence Assessment Protocol

### When to Assess

Run convergence assessment:
- After each adversarial review pass (Dimensions 1 and 3) -- including both primary (GPT-5.4) and secondary (Gemini 3.1 Pro) passes
- After each mutation testing run (Dimension 2)
- After each formal verification run (Dimension 4)
- Before requesting human sign-off

### Overall Convergence

The pipeline has converged when ALL FIVE dimensions report CONVERGED.

If any dimension reports NOT_CONVERGED, the Orchestrator must:
1. Identify which dimension(s) are not converged
2. Route work to the appropriate agent (Adversary for Dimensions 1/3, Formal Verifier for Dimensions 2/4)
3. Run the next iteration
4. Re-assess convergence

### Zero-Findings Halt

If the adversary reports ZERO findings on its FIRST pass of any review (spec
review, code review, or holdout evaluation), this is a suspicious signal --
not a convergence signal. Zero findings on a first pass of non-trivial work
almost always indicates a process failure, not flawless artifacts.

**Possible explanations for zero first-pass findings:**
1. **Adversary model failure** -- the model did not meaningfully engage with the
   artifacts (context too large, prompt issue, rate limit, model downgrade)
2. **Artifacts too simple** -- the spec or code is genuinely trivial (e.g.,
   a hello world program or single-endpoint CRUD)
3. **Genuinely flawless** -- extremely unlikely on a first pass of non-trivial work

**Protocol when zero findings on first pass:**

1. Log a WARNING in the convergence assessment: "Zero findings on first adversary
   pass -- requires verification"
2. Check adversary output for signs of genuine engagement:
   - Did it reference specific files and line numbers?
   - Did it demonstrate understanding of the codebase structure?
   - Is the output suspiciously short or generic?
3. If engagement is questionable: re-run the adversary with fresh context and an
   explicit instruction: "You MUST identify at least one finding, or explicitly
   justify why zero findings is correct by citing specific code evidence for each
   review category (spec fidelity, test quality, code quality, security)"
4. If the re-run also produces zero findings WITH specific justification citing
   concrete code evidence for each category -- accept as legitimate convergence
5. If the re-run produces zero findings WITHOUT per-category justification --
   escalate to the human: "Adversary unable to produce findings -- manual review
   recommended before declaring convergence"

**Exception:** For trivially-scoped changes routed through Quick Dev (DF-006),
zero findings on the single adversary pass is acceptable without re-analysis,
since the blast radius has already been verified as zero.

The `convergence-tracker` plugin (DF-009) records zero-finding events in the
pipeline metrics for pattern detection across runs. Recurring zero-finding
events for the same model may indicate a systematic prompt or context issue.

### Human Override

The human operator can override any convergence assessment:

- **Override NOT_CONVERGED to ship:** Document the override, which dimensions were not converged, current metric values, and rationale. This is a conscious risk acceptance.
- **Override CONVERGED to continue:** Request additional adversarial passes or verification runs beyond what the metrics indicate is necessary. This is valid when domain knowledge suggests the metrics may be missing something.

Both overrides are recorded in the convergence report with timestamp and rationale.
