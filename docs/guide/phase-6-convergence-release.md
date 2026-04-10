# Phase 6: Convergence and Release

## When to Enter Phase 6

Enter Phase 6 after all Phase 5 verification reports show PASS. Kani proofs are green, fuzz testing found no crashes, mutation kill rate meets thresholds, security scans are clean, and performance budgets are met.

## The 7 Dimensions of Convergence

Convergence means the project satisfies all quality criteria simultaneously. It is not a subjective judgment -- each dimension has measurable pass/fail criteria.

```mermaid
graph TD
    START[Phase 5 Complete<br/>All verification PASS]:::start --> CHECK[/vsdd-factory:convergence-check]

    CHECK --> D1[Dimension 1: Spec]:::dim
    CHECK --> D2[Dimension 2: Tests]:::dim
    CHECK --> D3[Dimension 3: Implementation]:::dim
    CHECK --> D4[Dimension 4: Verification]:::dim
    CHECK --> D5[Dimension 5: Visual]:::dim
    CHECK --> D6[Dimension 6: Performance]:::dim
    CHECK --> D7[Dimension 7: Documentation]:::dim

    D1 --> ASSESS{All 7<br/>dimensions<br/>PASS?}:::decision
    D2 --> ASSESS
    D3 --> ASSESS
    D4 --> ASSESS
    D5 --> ASSESS
    D6 --> ASSESS
    D7 --> ASSESS

    ASSESS -->|Yes| CONVERGED[CONVERGED]:::pass
    ASSESS -->|No| REMAINING[List remaining items]:::fail
    REMAINING --> FIX[Fix and re-check]
    FIX --> CHECK

    CONVERGED --> APPROVE[Human approval]:::decision
    APPROVE --> RELEASE[/vsdd-factory:release]:::action

    classDef start fill:#d4edda,stroke:#155724
    classDef dim fill:#cce5ff,stroke:#004085
    classDef decision fill:#fff3cd,stroke:#856404
    classDef pass fill:#d4edda,stroke:#155724
    classDef fail fill:#f8d7da,stroke:#721c24
    classDef action fill:#cce5ff,stroke:#004085
```

### Dimension 1: Spec Convergence

The adversary's critiques are nitpicks about wording, not missing behavior or verification gaps.

**Pass criteria:**
- Latest adversary pass novelty score below 0.15
- Median finding severity below 2.0 and strictly decreasing for 3+ passes
- All findings addressed or explicitly accepted

The `/vsdd-factory:convergence-check` command reads `.factory/cycles/<current>/vsdd-factory:adversarial-reviews/` to assess this.

### Dimension 2: Test Convergence

The test suite catches real bugs. Mutation testing confirms this.

**Pass criteria:**
- All tests pass (`cargo test --release`)
- Mutation kill rate at least 90% overall, with criticality-tier targets met
- Coverage at least 85%
- All cataloged invariants have property-based tests passing

### Dimension 3: Implementation Convergence

The code matches the spec. No known gaps remain.

**Pass criteria:**
- All tests green, clean build, clean lint
- No spec drift (verified by `/vsdd-factory:spec-drift`)
- All code review findings addressed
- No `todo!()`, `unimplemented!()`, or `FIXME` in production code

### Dimension 4: Verification Convergence

Formal methods confirm the implementation is correct.

**Pass criteria:**
- All Kani proofs pass at required depth
- No fuzz crashes after saturation
- Zero critical or high security findings
- Purity boundaries intact (pure core has no side effects)

### Dimension 5: Visual Convergence

Demo evidence proves the system works as intended.

**Pass criteria:**
- Demo recordings exist for every story (from `/vsdd-factory:record-demo`)
- Each acceptance criterion has visual evidence
- Design system compliance verified (if applicable)

### Dimension 6: Performance Convergence

The system meets its performance budgets.

**Pass criteria:**
- All budgets from `.factory/specs/prd-supplements/performance-budgets.md` within tolerance
- No benchmark regressions exceeding 10%

### Dimension 7: Documentation Convergence

Documentation matches the current implementation.

**Pass criteria:**
- CLAUDE.md updated with current architecture
- API documentation generated and accurate
- README reflects current installation, usage, and configuration
- All file path references in documentation resolve to existing files

## Running the Convergence Check

```
/vsdd-factory:convergence-check
```

This reads all prior phase artifacts and produces a convergence report at `.factory/cycles/<current>/convergence-report.md` with a per-dimension table:

```
| Dimension        | Status | Notes                           |
|------------------|--------|---------------------------------|
| 1. Spec          | PASS   | ADV-P3 novelty 0.08            |
| 2. Tests         | PASS   | Kill rate 93%, coverage 87%     |
| 3. Implementation| PASS   | No drift, no TODOs             |
| 4. Verification  | PASS   | 12/12 Kani, 0 fuzz crashes     |
| 5. Visual        | PASS   | 15/15 stories with demos       |
| 6. Performance   | PASS   | All budgets within 5%          |
| 7. Documentation | FAIL   | README missing new CLI flags   |

Overall: NOT CONVERGED
```

## When NOT Converged

If any dimension fails, the report lists the specific remaining items with severity and estimated effort. Fix the items and re-run `/vsdd-factory:convergence-check`.

Common fixes by dimension:

| Dimension | Typical Fix |
|-----------|-------------|
| Spec | Run another adversary pass, update specs |
| Tests | Add tests for surviving mutants, improve coverage |
| Implementation | Fix TODOs, run `/vsdd-factory:spec-drift` and address gaps |
| Verification | Fix failing Kani harnesses, address fuzz crashes |
| Visual | Run `/vsdd-factory:record-demo` for stories missing evidence |
| Performance | Profile and optimize, adjust budgets if justified |
| Documentation | Update README, regenerate API docs |

## Human Override

All convergence criteria are advisory. The human operator can override any assessment:

- **Override NOT_CONVERGED to ship** -- document which dimensions were not converged, current metric values, and the rationale. This is conscious risk acceptance recorded in the convergence report.
- **Override CONVERGED to continue** -- request additional verification when domain knowledge suggests the metrics may miss something.

## The Release Sequence

After convergence is achieved and the human approves:

```
/vsdd-factory:release
```

The release skill executes this sequence:

### Step 1: Determine Version

Read story types from `.factory/stories/` and apply semver rules:
- Any story with `feat` type -- MINOR bump
- Only `fix` stories -- PATCH bump
- Any story flagged `breaking_change` -- MAJOR bump
- First release with no prior tags -- `1.0.0` (or `0.1.0` if pre-stable)

The proposed version is presented for confirmation before proceeding.

### Step 2: Generate CHANGELOG

Parse the git log since the last tag, group entries by conventional commit type (Features, Bug Fixes, Security, Breaking Changes), and append a Quality Evidence section with convergence metrics.

### Step 3: Tag and Push

```bash
git tag -a vX.Y.Z -m "Release vX.Y.Z: [summary]"
git push origin develop    # CHANGELOG commit
git push origin vX.Y.Z     # tag triggers release CI
```

### Step 4: Wait for Release CI

The tag push triggers `.github/workflows/vsdd-factory:release.yml`. Monitor with `gh run watch`. If the build fails, diagnose from CI logs, fix via `/fix-pr-delivery`, delete the failed tag, and re-tag after the fix merges.

### Step 5: Create GitHub Release

```bash
gh release create vX.Y.Z \
  --title "vX.Y.Z: [release title]" \
  --notes-file .factory/vsdd-factory:release/CHANGELOG-vX.Y.Z.md
```

The release includes release notes, built binaries (uploaded by CI), demo evidence, and the convergence summary.

### Step 6: Post-Release Updates

Update the README version badge and installation instructions. Commit the docs update to develop:

```bash
git commit -m "docs: update version badge and install instructions for vX.Y.Z"
git push origin develop
```

### Step 7: Registry Publishing

Handled by the release CI workflow. The pipeline supports `cargo publish` (crates.io), `npm publish`, `twine upload` (PyPI), and `docker push` (GHCR) depending on the project type.

## Post-Release

After a successful release:

1. **STATE.md update** -- the state-update skill marks the pipeline as COMPLETED
2. **Session review** -- run `/vsdd-factory:session-review` for post-pipeline analysis and improvement proposals
3. **Next cycle** -- if planning additional work, the project transitions to Feature Mode (`/vsdd-factory:mode-decision-guide`) for incremental changes, or starts a new greenfield cycle for major new capabilities

## Progressive Autonomy

As the pipeline runs across multiple cycles, convergence metrics feed into the progressive autonomy system. When the Autonomy Score (a weighted composite of holdout satisfaction, false positive rate, override rate, convergence speed, and regression rate) exceeds 0.85 for 20 consecutive runs, qualifying phases can auto-advance without human approval. See [CONVERGENCE.md](../../plugins/vsdd-factory/docs/CONVERGENCE.md) for the full autonomy level criteria.
