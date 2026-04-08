---
name: discovery-engine
description: >
  Autonomous discovery engine that continuously researches opportunities for
  both new features (existing products) and new product concepts. Evaluates
  ideas against structured criteria, facilitates planning document creation,
  and routes approved ideas to the appropriate development pipeline.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via sessions_spawn. Each step names the target agent.
> The orchestrator does NOT execute these steps directly.

# Discovery Engine

## Trigger

Discovery runs can be triggered:
1. **Scheduled** — cron or GitHub Actions (recommended: weekly for features, bi-weekly for products)
2. **Manual** — human requests "Run discovery" or "Find new feature ideas for [product]"
3. **Event-driven** — triggered when a competitor ships a major release, a key dependency publishes a new version, or a regulatory change is announced

## Prerequisites

- `.factory/discovery-config.yaml` exists (or use defaults)
- `.factory/product-registry.yaml` exists for feature discovery (list of products the factory has built)
- `research-agent` is available (DF-002) with MCP tools:
  - **Perplexity** — web search, deep research, reasoning (already configured in DF-002)
  - **Context7** — library documentation and code examples (already configured in DF-002)
  - **Tavily** — web search, intelligent extraction, website mapping (already configured)
  - **Exa** — company research, code search, academic paper search (free, no API key needed)
  - **MCP RSS Aggregator** — competitor blog/changelog feed monitoring
  - **BuiltWith** (optional, $299/mo) — technology stack detection with first-party MCP support
  - **Crunchbase MCP** (community-built) — company/funding data
  - **Product Hunt MCP** (community-built) — product launch monitoring
  - **Hacker News MCP** — community discussion context around releases

## Discovery Modes

### Mode 1: Feature Discovery (Existing Products)

For each product in the product registry:

#### Step 1: Load Product Context

Read the product's existing artifacts:
- Product brief (from `.factory/planning/product-brief.md` or equivalent)
- Current PRD (from `.factory/specs/prd.md`)
- Architecture document (from `.factory/specs/architecture/ARCH-INDEX.md`)
- Recent feature history (from git log of the product repo)
- Current technology stack (from project configuration files)

Build a product profile: what it does, who it serves, what it's built with,
what was recently shipped, what's on the backlog.

#### Step 2: Research Landscape (Decomposition-Based)

Research uses **decomposition-based architecture** — break the landscape into
segments and analyze each independently. Research shows decomposition achieves
4.17/5 mean novelty scores vs 2.33/5 for reflection-based brainstorming
(arXiv:2601.09714). This prevents the system from converging on obvious ideas.

Spawn `research-agent` with the product profile. For each research segment,
use the appropriate MCP tools:

**Competitive intelligence:**
- What have direct competitors shipped in the last 30 days?
  Tools: Perplexity search, Product Hunt MCP, GitHub release Atom feeds via RSS Aggregator
- What are competitors' public roadmaps or announced features?
  Tools: Perplexity deep research, competitor changelog monitoring via RSS/changedetection
- What are users requesting in competitor communities (GitHub issues, forums, social media)?
  Tools: Perplexity search, Hacker News MCP, Exa (social/community search)
- What technology stacks are competitors adopting or abandoning?
  Tools: BuiltWith MCP (if configured), Perplexity search

**Technology opportunities:**
- Have any key dependencies released new versions with capabilities the product could use?
  Tools: GitHub release Atom feeds via RSS Aggregator, Context7 (library docs)
- Are there new libraries or tools that could simplify or enhance existing functionality?
  Tools: Context7, Exa (code search), Perplexity search
- Have any AI model capabilities improved in ways that benefit the product's domain?
  Tools: Perplexity deep research

**User signal:**
- What are the most common issues or feature requests in the product's issue tracker?
  Tools: GitHub API (issue search), Exa (community search)
- What pain points do users report in the product's domain (not just this product)?
  Tools: Perplexity search, Hacker News MCP, Reddit MCP (if configured)
- What workflows are users building around the product that could be first-class features?
  Tools: GitHub API (search for repos that depend on the product), Perplexity search

**Industry trends:**
- What best practices have evolved in the product's domain?
  Tools: Perplexity deep research, Exa (academic paper search)
- Are there new standards, protocols, or regulations that affect the product?
  Tools: Perplexity search, Tavily (extract from standards body websites)
- What patterns are emerging across similar products?
  Tools: Perplexity research, Product Hunt MCP

**Research quality controls:**
- Every finding must cite a specific source with URL and date
- Cross-reference claims across at least 2 independent sources
- Flag findings as VERIFIED (multiple sources) or UNVERIFIED (single source)
- Date-stamp all findings — research older than 30 days should be refreshed
- Negative evidence is valuable: "No competitor has shipped X" is a finding

Write research findings to `.factory/discovery/feature-research-[product-name]-YYYY-MM-DD.md`

#### Step 3: Generate Feature Ideas

From the research findings, generate a ranked list of feature ideas:

For each idea, produce:
- **Title:** concise feature name
- **Source:** what research finding triggered this idea (competitive, technology, user, industry)
- **Problem:** what user pain does this address?
- **Proposed solution:** high-level description (WHAT, not HOW)
- **Affected components:** which parts of the existing architecture would change?
- **Estimated scope:** small (1-3 stories), medium (4-8 stories), large (9+ stories)
- **Evaluation scores** (see Idea Evaluation Framework below)

Write to `.factory/discovery/feature-ideas-[product-name]-YYYY-MM-DD.md`

#### Step 4: Evaluate and Rank

Apply the Idea Evaluation Framework (see Deliverable 3) to score each idea.
Rank by composite score. Flag the top 3-5 ideas for human review.

#### Step 5: Facilitate Document Creation

For ideas that score above the threshold (default: composite >= 0.65):
- Create a **feature request brief** using `templates/idea-brief-template.md`
- Include: problem statement, proposed solution, supporting research, evaluation scores
- Store in `.factory/discovery/briefs/feature-[product]-[idea-slug].md`

These briefs are ready for human review. If approved, they route to DF-006
Feature Mode (Phase F1 Delta Analysis) as the feature request input.

---

### Mode 2: Product Discovery (Net-New)

#### Step 1: Load Discovery Context

Read the discovery configuration for product-level scanning:
- Organization's domain focus areas (from discovery-config.yaml)
- Existing product portfolio (from product-registry.yaml)
- Strategic themes or constraints (from discovery-config.yaml)
- Previous discovery reports (to avoid re-proposing rejected ideas)

#### Step 2: Research Opportunities

Spawn `research-agent` to scan for product opportunities:

**Market gaps:**
- What problems exist in the org's domain that no product addresses well?
- What are users building with duct tape and scripts that should be a product?
- What SaaS tools have user complaints that suggest a better approach is possible?

**Emerging technology:**
- What new AI capabilities create product categories that didn't exist 6 months ago?
- What infrastructure shifts (edge computing, WebAssembly, local AI) enable new products?
- What open-source ecosystem gaps exist (tools that should exist but don't)?

**Adjacent opportunities:**
- What patterns from the org's existing products apply to adjacent problem spaces?
- What integrations between existing products could become standalone products?
- What internal tools could be productized?

**Regulatory and compliance:**
- What new regulations create compliance tooling needs?
- What industry standards are emerging that require new tooling?

Write research findings to `.factory/discovery/product-research-YYYY-MM-DD.md`

#### Step 3: Generate Product Concepts

From the research findings, generate product concepts:

For each concept, produce:
- **Name:** working product name
- **One-liner:** what it is in one sentence
- **Problem:** what pain it solves and for whom
- **Opportunity source:** what research finding triggered this concept
- **Differentiation:** why this would be better than existing alternatives
- **Feasibility signals:** technology readiness, market validation, team capability fit
- **Estimated scope:** MVP size (small: 1-2 sprints, medium: 3-5 sprints, large: 6+ sprints)
- **Evaluation scores** (see Idea Evaluation Framework below)

Write to `.factory/discovery/product-concepts-YYYY-MM-DD.md`

#### Step 4: Evaluate and Rank

Apply the Idea Evaluation Framework to score each concept.
Rank by composite score. Flag the top 2-3 concepts for human review.

#### Step 5: Facilitate Document Creation

For concepts that score above the threshold (default: composite >= 0.70 — higher than features because new products carry more risk):
- Create a **product idea brief** using `templates/idea-brief-template.md`
- Include: problem statement, target audience, differentiation, supporting research, evaluation scores
- Store in `.factory/discovery/briefs/product-[concept-slug].md`

These briefs are ready for human review. If approved, they route to DF-016
Adaptive Planning Pipeline → Collaborative Discovery or Guided Brief Creation
→ eventually to DF-003 Greenfield Pipeline.

---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via sessions_spawn. Each step names the target agent.
> The orchestrator does NOT execute these steps directly.

## Output Artifacts

- `.factory/discovery/feature-research-[product]-YYYY-MM-DD.md` — per-product research
- `.factory/discovery/feature-ideas-[product]-YYYY-MM-DD.md` — ranked feature ideas
- `.factory/discovery/product-research-YYYY-MM-DD.md` — market/technology research
- `.factory/discovery/product-concepts-YYYY-MM-DD.md` — ranked product concepts
- `.factory/discovery/briefs/feature-[product]-[idea-slug].md` — feature idea briefs
- `.factory/discovery/briefs/product-[concept-slug].md` — product idea briefs
- `.factory/discovery/discovery-report-YYYY-MM-DD.md` — summary report

---

## Scoring Dimensions (7)

Ideas are scored on 7 dimensions. The original 6 dimensions from DF-017 are
preserved; `evidence_strength` is the 7th dimension added by DF-034.

| Dimension | Weight | Description |
|-----------|--------|------------|
| Value | 0.25 | How much value does this create for users? |
| Feasibility | 0.15 | Can we build this with current tech/team? |
| Alignment | 0.15 | Does this fit our product vision? |
| Novelty | 0.10 | Is this a new capability (not incremental)? |
| Time-Criticality | 0.10 | Is there a window of opportunity? |
| Effort | 0.10 | How much work to implement? (inverse) |
| Evidence Strength | 0.15 | How many independent sources confirm this? |

Weights sum to 1.00. Previous 6-dimension weights were rebalanced:
Value 0.25 (was 0.30), Feasibility 0.15 (was 0.20), Alignment 0.15 (was 0.15),
Novelty 0.10 (was 0.10), Time-Criticality 0.10 (was 0.10), Effort 0.10 (was 0.15).

### Evidence Strength Rubric

| Score | Criteria |
|-------|---------|
| 0.0 - 0.2 | Speculation -- no external validation |
| 0.3 - 0.5 | Market research only -- Perplexity findings |
| 0.5 - 0.6 | Market + one customer signal (feedback OR analytics) |
| 0.6 - 0.8 | Market + multiple customer signals |
| 0.8 - 0.9 | Market + customer feedback + usage analytics |
| 0.9 - 1.0 | All sources + revenue/payment impact confirmed |

**Default for existing ideas:** Ideas scored before DF-034 (without customer
intelligence data) default to evidence_strength = 0.3 (market scan level).

---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via sessions_spawn. Each step names the target agent.
> The orchestrator does NOT execute these steps directly.

## Routing Decisions

### Auto-Brief Generation (-> Path 5 Planning)
- Composite score >= 0.7 AND evidence_strength >= 0.6
- This ensures we only auto-generate briefs for ideas with real customer evidence
- Human approves the generated brief

### Product Backlog (-> STATE.md backlog)
- Composite score 0.5-0.7 OR evidence_strength 0.4-0.6
- Resurface in next discovery report for re-evaluation
- May be promoted if evidence strengthens over time

### Discovery Registry (-> log for future)
- Composite score < 0.5 AND evidence_strength < 0.4
- Re-evaluated on next scheduled run
- May be promoted if new signals emerge

### Urgent Action (-> immediate human notification)
- Any insight where: competitive urgency is HIGH AND evidence_strength >= 0.7
- "Competitor just shipped X and our customers are asking for it"
- Notification via configured channel (github-issues default)
- Human always approves before insights become features

---

## Quality Gate

- [ ] All ideas scored on 7 dimensions with composite score computed
- [ ] Duplicate ideas identified and clustered (no re-proposals without new evidence)
- [ ] Discovery report generated in `.factory/discovery/`
- [ ] Routing decisions applied (brief / backlog / registry / urgent) per threshold rules

## Failure Modes

- If MCP tools are unavailable (Perplexity, Context7, etc.): use training data with explicit "UNVERIFIED -- no live research" disclaimer on all findings
- If product registry is empty: run Product Discovery mode only, skip Feature Discovery
- If all ideas score below threshold: produce the report anyway, noting "no actionable ideas this cycle"

## Quality Criteria

- [ ] Research cites specific sources (not vague "industry trends")
- [ ] Every idea traces to a research finding
- [ ] Evaluation scores use the structured framework (not gut feel)
- [ ] Feature ideas reference specific components in the existing architecture
- [ ] Product concepts include differentiation from existing alternatives
- [ ] Briefs are substantive enough to start a planning conversation (not just titles)
- [ ] Previously rejected ideas are not re-proposed without new evidence
- [ ] Evidence strength scored on 7th dimension with documented rubric
- [ ] Routing decisions factor in evidence_strength thresholds
- [ ] Auto-brief requires evidence_strength >= 0.6 (not just composite score)
- [ ] Urgent action path active for HIGH competitive urgency + HIGH evidence
