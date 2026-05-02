---
name: orchestrator-maintenance-sequence
description: Orchestrator workflow reference for the maintenance-sweep delegation sequence (Path 10). Loaded by the orchestrator agent during the corresponding phase. Not directly invokable.
---

> **Global Operating Rules:** Read `../../docs/FACTORY.md` and `../../docs/VSDD.md` for factory-wide constraints.


# Maintenance Delegation Sequence (Path 10)

Reference file for the orchestrator. Load when running maintenance sweeps.

## Overview

Background quality sweeps that detect dependency vulnerabilities, documentation
drift, pattern inconsistencies, and other quality issues. Runs on a configurable
schedule (default: weekly). Opens fix PRs through code-delivery.lobster.

Maintenance PAUSES during active feature cycles to avoid conflicts on develop.

## Load Configuration

Read maintenance config from `.factory/maintenance-config.yaml`:
- Sweep schedule, enabled sweeps, thresholds
- Check for interrupted previous runs (crash recovery)

## 11 Parallel Sweeps

All sweeps run in parallel. Spawn each as a separate agent:

### Sweep 1: Dependency Audit (split)
- Spawn dx-engineer: "Run dependency scans (cargo audit, npm audit, etc.)"
- Spawn security-reviewer: "Analyze scan findings, classify severity, recommend fixes"

### Sweep 2: Documentation Drift
- Spawn technical-writer: "Compare docs against current implementation.
  Flag stale API docs, outdated runbooks, missing module docs."

### Sweep 3: Pattern Consistency
- Spawn code-reviewer: "Scan for legacy vs new pattern inconsistencies.
  Identify candidates for batch refactoring."

### Sweep 4: Holdout Scenario Freshness
- Spawn holdout-evaluator: "Check holdout scenarios for staleness:
  - Does the scenario reference features that still exist?
  - Has it been evaluated in the last 3 releases?
  - Does expected behavior still match the product?
  Mark stale scenarios with lifecycle_status: stale."

### Sweep 5: Performance Regression
- Spawn performance-engineer: "Run benchmarks against baseline.
  Flag regressions exceeding 10% threshold."

### Sweep 6: DTU Fidelity Drift
- Spawn dtu-validator: "Check DTU clones against real APIs via mcporter.
  Flag behavioral drift between clone and real service."

### Sweep 7: Spec Coherence (33 criteria)
- Spawn consistency-validator: "Run spec coherence validation:
  - L1->L4 chain integrity
  - BC coverage completeness
  - Lifecycle status consistency
  - Story count vs STORY-INDEX match"

### Sweep 8: Tech Debt Register Update
- Spawn consistency-validator: "Surface overdue tech debt items.
  Register new debt sources. Update tech-debt-register.md."

### Sweep 9: Accessibility Regression (UI products only)
- Spawn accessibility-auditor: "Automated a11y scans compared to baseline.
  Flag new violations."

### Sweep 10: Design Drift Detection (UI products only)
- Spawn visual-reviewer: "Detect token overrides, component misuse,
  pattern violations, emergent patterns."

### Sweep 11: Risk & Assumption Monitoring
- Spawn consistency-validator: "Check ASM/R-NNN staleness:
  - Are assumptions still valid?
  - Have mitigations been implemented?
  - Do mitigations match current architecture?"

## Aggregate Findings

Spawn state-manager: "Generate maintenance report classifying all sweep
findings as automated-fix vs manual-fix. Commit to factory-artifacts."

## Fix PR Delivery

For each automated-fixable finding:
- Run per-story delivery cycle from `per-story-delivery.md`
- One fix PR per finding (keep PRs small and reviewable)
- Max 10 fix PRs per maintenance run before human escalation

## Notifications

- BLOCKING for critical CVEs or security regressions
- WARNING for overdue tech debt (>90 days)
- INFO for routine findings

## Maintenance Gate

Verify:
- Zero critical CVEs remaining
- All auto-fix PRs merged and passing
- All 11 sweeps completed

## Session Review

Spawn session-review: "Review sweep effectiveness — false positive rate,
fix quality, cost vs value, sweep coverage."
