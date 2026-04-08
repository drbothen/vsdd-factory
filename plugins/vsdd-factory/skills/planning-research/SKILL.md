---
name: planning-research
description: >
  Conducts market, domain, and technical research to validate assumptions
  and fill knowledge gaps before brief or PRD creation. Uses Perplexity
  for web research and Context7 for library/framework documentation.
  Can run domain, market, or technical research independently or combined.
---

# Planning Research

## When This Skill Runs

- After brainstorming, to validate the selected direction
- Before brief creation, to ground the brief in evidence
- Before PRD creation, to validate technical feasibility
- When the human says "research X before we proceed"

## Research Types

### Domain Research
- Industry landscape and trends
- Regulatory requirements and compliance
- Domain terminology and standards
- Existing solutions and their limitations

### Market Research
- Target audience validation (do they exist? how many?)
- Customer pain point verification (are these real problems?)
- Competitive analysis (what exists? what's missing?)
- Pricing and business model patterns

### Technical Research
- Technology stack evaluation (what's mature? what's risky?)
- Integration patterns (how do these components connect?)
- Architecture options (what patterns fit this problem?)
- Library/framework selection (versions, trade-offs, community health)

## Research Process

1. **Receive research brief** -- what questions need answering
2. **Spawn research-agent** with Perplexity + Context7 MCP tools
3. **Follow AGENTS.md query construction rules** (explicit search directives,
   parent org context, alternative terms, source suggestions)
4. **Cross-reference findings** across at least 2 independent sources
5. **Date-stamp all findings** -- technology landscapes change rapidly
6. **Write research report** to `.factory/planning/research-report.md`
7. **Flag uncertainties** -- what couldn't be verified, what needs human judgment

## Quality Gate

- [ ] Research report produced with structured findings per research type
- [ ] All findings cite specific sources with URLs and dates
- [ ] Inconclusive areas and uncertainties explicitly flagged
- [ ] Cross-referenced across at least 2 independent sources where possible

## Failure Modes

- If MCP tools (Perplexity, Context7) are unavailable: use training data with explicit "UNVERIFIED -- based on training data, not live research" disclaimer
- If research yields no relevant results for a research type: document the gap and recommend alternative research approaches
- If sources contradict each other: present both sides with evidence strength assessment

## Output Artifacts

- `.factory/planning/research-report.md`
- `.factory/planning/research-sources.md` (citations and links)
