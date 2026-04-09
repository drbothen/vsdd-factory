# Step 2: Contextual Discovery

> **Parent skill:** `guided-brief-creation` — see `../SKILL.md` for the full workflow.
> **Agent:** research-agent
> **This step:** before elicitation, gather any external context the brief will need — analyze provided documents, read brainstorming output, run web research if the domain is unfamiliar.

## Inputs

- Intent statement, source mode, and context sources from `step-01-understand-intent.md`

## Outputs

- A short discovery summary in working memory: what was found, what is still unknown
- Updates to `.factory/planning/elicitation-notes.md` with relevant findings
- Optional: `.factory/planning/market-intel.md` if web research was run

## Procedure

1. **Read the brainstorming report if it exists.** Path: `.factory/planning/brainstorming-report.md`. The selected direction becomes the brief's foundation. Note the four parts (problem, solution, audience, differentiator) — they pre-fill section 3 elicitation.

2. **Analyze provided documents.** For each document the human referenced in step 1:
   - Spawn `research-agent` with the document path.
   - Ask it to extract: stated problem, audience hints, scope hints, success metrics, constraints.
   - Capture findings in `elicitation-notes.md` under "Document analysis: <doc name>".

3. **Decide whether web research is needed.** Spawn `research-agent` with Perplexity if any of these are true:
   - The domain is specialized (medical, legal, scientific, regulated)
   - The competitive landscape is unknown to you
   - Technical feasibility is unclear (would this even work?)
   - The audience's actual pains are unconfirmed

   Do NOT spawn web research if:
   - The brainstorming report already covers the landscape
   - The human has explicit domain expertise and is the user themselves
   - The brief is for an internal tool with no external comparables

4. **If web research runs**, ask the research agent for:
   - Top 3–5 competitors or alternatives (with one-line differentiation)
   - Confirmed pains in the audience (with citations)
   - Unconfirmed pains (assumptions to flag)
   - Technical feasibility signals
   - Regulatory or compliance constraints

   Write the result to `.factory/planning/market-intel.md`. Step 3 (elicitation) will reference it when filling the Audience and Differentiator sections.

5. **Do not pre-fill the brief.** Discovery findings go in working notes and `market-intel.md`, not in the brief draft. The brief is built in step 3 through conversation, not by you copying research into a template.

6. **Record gaps.** What is still unknown after discovery? List it. Step 3 will probe for it.

## Decision points

- **No documents, no brainstorming, no specialized domain** → skip discovery entirely. Move directly to step 3 with the intent statement alone.
- **Documents are voluminous** (50+ pages) → ask research-agent for a structured summary, not a full read. Cite specific sections only when they touch the brief.
- **Web research returns thin results** → flag the audience and pains as "needs validation" rather than fabricating signal.

## Failure modes

- **Spawning research-agent for every brief by reflex.** Most briefs do not need it. Use the decision criteria.
- **Conflating discovery with elicitation.** Discovery gathers context; elicitation builds the brief through conversation. Do not combine them — that is how you end up writing the brief for the human.
- **Treating research output as ground truth.** It is input. Your human partner is still the domain expert; research is one voice at the table.

## Quality gate

- [ ] Brainstorming report read if present
- [ ] Provided documents analyzed (or explicitly skipped)
- [ ] Web research run only if criteria met; result written to `market-intel.md`
- [ ] Discovery findings captured in `elicitation-notes.md`
- [ ] Gaps for step 3 enumerated

## Hand-off to next step

Pass the discovery summary, paths to `market-intel.md` and `elicitation-notes.md`, and the gap list to `step-03-guided-elicitation.md`.
