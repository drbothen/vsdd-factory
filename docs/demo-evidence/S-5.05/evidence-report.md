# Evidence Report — S-5.05 Migration Guide

**Story:** S-5.05 — Migration guide (0.79.x → 1.0)
**Branch:** feat/s-5.05-migration-guide
**HEAD:** ae860b0
**Date:** 2026-04-29

## Coverage Map

| AC / Task | Evidence File | Result |
|---|---|---|
| AC-1 — TODO marker clearance | 01-ac-1-todo-clearance.md | PASS — both greps return 0 |
| AC-2 — "What changed" section | 02-ac-2-what-changed.md | PASS — prose present, all 3 elements |
| AC-3 — "Why v1.0" section | 03-ac-3-why-v1.0.md | PASS — prose present |
| AC-4 — Prerequisites + Upgrade + Verification + Rollback | 04-ac-4-prerequisites-upgrade-verification-rollback.md | PASS — 4 sections + EC-001 |
| AC-5 — Observability + Regenerating (PRE-FILLED) | 05-ac-5-observability-regenerating.md | PASS — both sections present |
| AC-6 — Windows-specific notes | 06-ac-6-windows-notes.md | PASS — prose present |
| AC-7 — Troubleshooting >= 5 issues | 07-ac-7-troubleshooting-5-issues.md | PASS — 5 distinct issues |
| AC-8 — Known regressions + PRE-FILLED beta.1 | 08-ac-8-known-regressions.md | PASS — both sections present |
| AC-9 — Human review | 13-ac-9-human-review-deferred.md | DEFERRED — manual gate |
| AC-10 — Status banner replaced | 09-ac-10-status-banner-replaced.md | PASS — grep returns 0 |
| Task 16 — README L264 update | 10-task-16-readme-l264-update.md | PASS — before/after shown |
| Task 17 — v1.0-index.md S-5.7 → S-5.07 | 11-task-17-v1.0-index-canonicalization.md | PASS — both lines fixed |
| Bats 17/17 | 12-bats-green.md | PASS — 17/17 |

## Summary

13 demo evidence files produced. 12 ACs/tasks PASS. AC-9 deferred to
PR review (human migration reviewer required — manual gate, no automated
evidence possible). Bats suite 17/17 green.
