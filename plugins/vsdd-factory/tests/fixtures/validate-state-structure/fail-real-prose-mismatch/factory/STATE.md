---
document_type: pipeline-state
level: ops
version: "2.0"
status: draft
producer: state-manager
timestamp: 2026-05-17T00:00:00Z
phase: test-fixture-fail-real-prose-mismatch
---

<!--
  STATE.md SIZE BUDGET (per D-421(c) + D-422(c) reconciliation):
  Soft target: ≤415 lines; margin from soft-target = 500 - 415 = 85; margin from actual = 500 - 28 = 472 (D-446(c) dual-margin form).
  Line-growth tracker (D-437(e)+D-441(e)): pass-65 395 lines (wc-l; net -52 from pass-64); pass-66 397 lines (wc-l; net +2 from pass-65); this-fixture 28 lines (wc-l; net -369; OFF BY ONE — actual is 29).
  Hard cap: 500 lines.
-->

# Pipeline State: test-fixture

## Phase Progress

| Entry | Status |
|-------|--------|
| test fail-real-prose-mismatch | FAIL |

## Convergence Status

Trajectory →9→9→9→9

