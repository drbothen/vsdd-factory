---
name: market-intelligence-assessment
description: >
  Mandatory market intelligence assessment that runs before any spec work begins.
  Researches competitive landscape, validates customer pain, assesses market size,
  identifies differentiation opportunities, and flags risk signals. Produces a
  GO / CAUTION / STOP recommendation for human review.
agents:
  primary: business-analyst
  supporting: [research-agent]
inputs:
  - Human's initial input (idea, brief, PRD, or full spec)
  - .factory/planning/artifact-inventory.md (from artifact detection)
outputs:
  - .factory/planning/market-intel.md
gate: Human reviews market intel before proceeding
---

# Market Intelligence Assessment

## Purpose

Every product idea, feature request, or spec entering the Dark Factory pipeline must
pass through a market intelligence assessment before any spec work begins. This skill
ensures the factory never invests engineering effort into products or features that:

- Already exist with strong incumbents and no differentiation angle
- Solve problems customers don't actually have (or don't pay to solve)
- Target markets too small to justify the investment
- Carry unidentified regulatory, competitive, or timing risks

The assessment produces a GO / CAUTION / STOP recommendation. The human always makes
the final call -- the factory provides the evidence.

## Step 1: Extract Market Context

**Agent:** `business-analyst`

The business-analyst reads whatever the human provided (L0 idea, L1 brief, L2 PRD,
L3 architecture, or L4 full spec) and extracts a structured research brief:

```markdown
## Research Brief
- **Product/Feature:** [one-sentence description]
- **Target Audience:** [who this is for, as specific as available]
- **Problem Statement:** [what pain this solves]
- **Proposed Solution:** [how it solves the pain, high-level]
- **Known Competitors:** [any competitors the human mentioned]
- **Known Constraints:** [regulatory, technical, market constraints mentioned]
- **Input Level:** [L0/L1/L2/L3/L4]
```

The research brief is passed to the research-agent for Step 2.

If the input is sparse (L0 idea with minimal detail), the business-analyst asks the
human up to 3 clarifying questions before proceeding. Do not block on answers -- use
reasonable assumptions and flag them as assumptions in the output.

## Step 2: Market Landscape Research

**Agent:** `research-agent` via Perplexity MCP

The research-agent executes five parallel research tracks using Perplexity:

### 2a. Competitive Landscape

- Direct competitors (same problem, same audience)
- Adjacent competitors (similar problem, different audience or approach)
- Emerging competitors (startups, open-source projects, research papers)
- For each competitor: name, approach, funding/traction, strengths, weaknesses

### 2b. Market Size

- TAM/SAM/SOM estimates with sources
- Growth rate and trajectory
- Pricing benchmarks from existing solutions
- Market maturity signal (nascent / growing / mature / declining)

### 2c. Customer Pain Validation

- Evidence that the target audience experiences this pain (forums, reviews, surveys)
- How they currently solve it (workarounds, manual processes, existing tools)
- Willingness to pay signals (existing spending on workarounds, budget categories)
- Pain severity: inconvenience vs. blocker vs. revenue-impacting

### 2d. Differentiation Opportunities

- Gaps in existing solutions (features, UX, pricing, integration, performance)
- Underserved segments within the target audience
- Technology shifts enabling new approaches (AI, new protocols, regulatory changes)
- Timing advantages (why now, not two years ago?)

### 2e. Risk Signals

- Regulatory risks (pending legislation, compliance requirements)
- Platform risks (dependency on APIs, marketplaces, ecosystems that could change)
- Competitive moat risks (can incumbents easily replicate the differentiation?)
- Market timing risks (too early, too late, window closing)
- Technology risks (depends on immature tech, unproven at scale)

## Step 3: Synthesis and Recommendation

**Agent:** `business-analyst`

The business-analyst synthesizes research findings into `.factory/planning/market-intel.md`
using the following template:

```markdown
---
recommendation: GO | CAUTION | STOP
confidence: high | medium | low
input_level: L0 | L1 | L2 | L3 | L4
assessed_at: [ISO timestamp]
assessor: business-analyst + research-agent
---

# Market Intelligence Assessment

## Executive Summary
[2-3 sentences: what we found, what it means, what we recommend]

## 1. Competitive Landscape
### Direct Competitors
| Competitor | Approach | Traction | Strengths | Weaknesses |
|-----------|----------|----------|-----------|------------|
| ... | ... | ... | ... | ... |

### Adjacent Competitors
[same format]

### Emerging Threats
[same format]

### Competitive Density Score
[LOW / MEDIUM / HIGH / SATURATED]

## 2. Market Size & Dynamics
- **TAM:** [estimate + source]
- **SAM:** [estimate + source]
- **SOM:** [estimate + source]
- **Growth Rate:** [X% CAGR, source]
- **Market Maturity:** [nascent / growing / mature / declining]
- **Pricing Benchmarks:** [range from existing solutions]

## 3. Customer Pain Validation
- **Pain Confirmed:** [YES / PARTIAL / NO]
- **Evidence:** [specific sources -- forums, reviews, surveys, interviews]
- **Current Workarounds:** [how people solve this today]
- **Willingness to Pay:** [signals for/against]
- **Pain Severity:** [inconvenience / blocker / revenue-impacting]

## 4. Differentiation Opportunities
- [opportunity 1: description + evidence]
- [opportunity 2: description + evidence]
- [opportunity 3: description + evidence]

## 5. Risk Signals
| Risk | Severity | Likelihood | Mitigation |
|------|----------|-----------|------------|
| ... | HIGH/MED/LOW | HIGH/MED/LOW | ... |

## 6. Implications for Spec Work
- **If GO:** [what the spec should emphasize, what to prioritize]
- **If CAUTION:** [what needs more research, what assumptions to test]
- **If STOP:** [why, what would need to change for GO]

### Assumptions Made
- [assumption 1 -- flagged for human validation]
- [assumption 2 -- flagged for human validation]
```

### Recommendation Criteria

- **GO:** Pain confirmed, market viable, clear differentiation exists, risks manageable
- **CAUTION:** Pain partially confirmed OR differentiation unclear OR significant risks
  identified. Proceed with awareness -- spec should address the flagged concerns.
- **STOP:** Pain not validated, market too small, saturated with no differentiation,
  or showstopper risks identified. Human should reconsider before investing spec effort.

Confidence level reflects research depth:
- **high:** Multiple corroborating sources, recent data, clear signals
- **medium:** Some sources, some assumptions, mixed signals
- **low:** Limited data available, significant assumptions made

## Step 4: Human Review Gate

The market intelligence assessment is presented to the human for review.

- **GO:** Human approves, pipeline continues to spec work
- **CAUTION:** Human reviews flagged concerns, provides direction (proceed, pivot, research more)
- **STOP:** Human reviews reasoning, decides to abort, pivot, or override with justification

The human's decision is recorded in `STATE.md`:

```yaml
market_intel:
  recommendation: GO | CAUTION | STOP
  human_decision: approved | overridden | pivoted | aborted
  notes: "[human's reasoning if overriding]"
```

If the human overrides a STOP recommendation, the override and reasoning are carried
forward as context for all downstream agents (they should know the market risk was
accepted deliberately).

## Depth Configuration

The assessment depth adapts based on the input level (L0-L4) to avoid redundant
research on projects that already have market validation:

### L0: Full Landscape (idea, no artifacts)
- **Sections:** All 5 (Competitive Landscape, Market Size, Customer Pain Validation,
  Differentiation Opportunities, Risk Signals)
- **Depth:** Maximum -- this is the first time anyone has researched this idea
- **Expected Duration:** 15-30 minutes

### L1: Validation Focus (brief exists)
- **Sections:** 1 (Competitive Landscape), 3 (Customer Pain Validation), 5 (Risk Signals)
- **Depth:** Validate claims made in the brief against external evidence
- **Expected Duration:** 10-15 minutes

### L2: Competitive Deep-Dive (PRD exists)
- **Sections:** 1 (Competitive Landscape), 4 (Differentiation Opportunities), 5 (Risk Signals)
- **Depth:** PRD implies pain is validated; focus on competitive positioning and risks
- **Expected Duration:** 10-15 minutes

### L3: Risk Check (architecture exists)
- **Sections:** 5 (Risk Signals) + summary of 1 (Competitive Landscape)
- **Depth:** Architecture implies deep investment; check for late-stage risks only
- **Expected Duration:** 5-10 minutes

### L4: Freshness Check (full spec exists)
- **Sections:** Headline check of 1 (Competitive Landscape) + new risks in 5 (Risk Signals)
- **Depth:** Spec is complete; only check if the landscape changed since spec was written
- **Auto-GO:** If no material changes detected, auto-recommend GO without human gate
- **Expected Duration:** 2-5 minutes

## Quality Gate

- [ ] GO, CAUTION, or STOP recommendation produced with confidence level
- [ ] Supporting evidence cited for the recommendation (not opinion-based)
- [ ] All research sections completed per depth configuration for the input level
- [ ] Assumptions explicitly flagged for human validation

## Failure Modes

- If research sources are sparse (few results, low confidence): produce CAUTION recommendation with explicit gaps listed
- If Perplexity MCP is unavailable: use training data with "UNVERIFIED -- no live research" disclaimer, default to CAUTION
- If the input is too vague for meaningful research (L0 with one sentence): ask up to 3 clarifying questions, then proceed with assumptions flagged
