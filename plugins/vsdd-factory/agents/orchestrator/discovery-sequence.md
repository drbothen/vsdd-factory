---
name: orchestrator-discovery-sequence
description: Orchestrator workflow reference for the discovery-engine sequence (Path 8). Loaded by the orchestrator agent during the corresponding phase. Not directly invokable.
disable-model-invocation: true
---

> **Global Operating Rules:** Read `../../docs/FACTORY.md` and `../../docs/VSDD.md` for factory-wide constraints.


# Discovery Delegation Sequence (Path 8)

Reference file for the orchestrator. Load when running autonomous discovery.

## Overview

The discovery engine continuously researches opportunities for new features
and new product concepts. It evaluates ideas via multi-agent Delphi scoring,
creates briefs, and routes approved ideas to the development pipeline.

Discovery runs on a configurable schedule and does NOT modify code — it is
safe to run during active feature cycles.

## Load Configuration

Read discovery config from `.factory/discovery-config.yaml`:
- Research scope, schedule, scoring thresholds
- Product registry (existing products)
- Previously rejected/deferred ideas (avoid re-proposing)

## Feature Discovery

1. Spawn research-agent: "Market research for opportunities
   in [product domain]. Use Perplexity for market landscape, Context7 for
   technology trends."

2. Spawn research-agent: "Customer feedback ingestion — analyze GitHub issues,
   support channels, app reviews for unmet needs."

3. Spawn research-agent: "Competitive monitoring — track competitor releases,
   pricing changes, funding, acquisitions."

4. Spawn research-agent: "Usage analytics integration — analyze feature adoption,
   error rates, user journey data." (if analytics configured)

5. Spawn business-analyst: "Intelligence synthesis — correlate signals across
   market research, customer feedback, competitive data, and analytics.
   Score evidence strength per signal. Write intelligence-synthesis.md."

## Idea Scoring (Delphi Protocol)

For each candidate feature idea:

6. Spawn product-owner: "Score on Value, Alignment, Time Criticality dimensions.
   Include evidence citations."

7. Spawn architect: "Score on Feasibility, Effort dimensions. Apply skeptical
   lens — flag technical risks and hidden complexity."

8. Spawn adversary: "Score on Novelty dimension with fresh context. Challenge
   assumptions, flag idea profiles:
   - safe-bet: high feasibility, low novelty
   - moonshot: low feasibility, high novelty
   - quick-win: low effort, moderate value
   - strategic-bet: high risk, high potential
   - time-bomb: urgent but costly
   - gold-plating: low value, high effort"

9. Composite scoring: geometric mean across all 7 dimensions + evidence_strength.
   Ideas above threshold proceed to deduplication.

## Deduplication & Trend Analysis

10. Spawn consistency-validator: "Embedding-based 3-tier deduplication:
    - Tier 1 (0.92 similarity): exact duplicates, merge
    - Tier 2 (0.85 similarity): near-duplicates, flag for human
    - Tier 3 (0.70 similarity): related ideas, cluster for trend analysis
    Run HDBSCAN clustering on idea embeddings. Check cooldown periods
    for previously deferred ideas."

## Discovery Report

11. Spawn state-manager: "Generate discovery report with:
    - Ranked feature ideas with scores and evidence
    - Product concepts (if any)
    - Deduplication results
    - Trend analysis (emerging clusters)
    - Commit to factory-artifacts."

## Notifications

- INFO for ideas above scoring threshold
- BLOCKING for urgent ideas (HIGH evidence + HIGH competitive urgency)

## Human Review Gate

Present discovery report to human. For each idea:
- **Approve:** Create brief, route to feature.lobster or planning.lobster
- **Research further:** Spawn research-agent for deeper investigation
- **Defer:** Add to backlog with cooldown period
- **Reject:** Log reason, prevent re-proposal

## Route Approved Ideas

- Feature ideas for existing products: route to `feature-sequence.md`
- New product concepts: route to planning.lobster (Path 5)

## Session Review

Spawn session-review: "Review discovery run quality — research accuracy,
scoring calibration, deduplication effectiveness, cost."
