# S-5.05 — Demo Evidence Overview

**Story:** S-5.05 — Migration guide (0.79.x → 1.0)
**Wave:** 16 (ship wave) / Wave 14 (convergence burst)
**Epic:** E-5 — New Hook Events and 1.0.0 Release
**Branch:** feat/s-5.05-migration-guide
**HEAD:** ae860b0
**Bats result:** 17/17 PASS

## Summary

Fills in `docs/guide/migrating-from-0.79.md` (skeleton from S-0.05).
All 10 `TODO(S-5.5)` blocks replaced with prose. Two PRE-FILLED sections
(Regenerating hooks-registry.toml, Known regressions v1.0.0-beta.1)
verified complete. Status banner replaced. README L264 and
v1.0-index.md L6+L40 cross-link drift fixed.

## AC Trace Map

| Evidence file | AC / Task | Status |
|---|---|---|
| 01-ac-1-todo-clearance.md | AC-1 — no TODO(S-5.5) remaining | PASS |
| 02-ac-2-what-changed.md | AC-2 — "What changed" section | PASS |
| 03-ac-3-why-v1.0.md | AC-3 — "Why v1.0" section | PASS |
| 04-ac-4-prerequisites-upgrade-verification-rollback.md | AC-4 — 4 sections + EC-001 | PASS |
| 05-ac-5-observability-regenerating.md | AC-5 — Observability + Regenerating (PRE-FILLED) | PASS |
| 06-ac-6-windows-notes.md | AC-6 — Windows-specific notes | PASS |
| 07-ac-7-troubleshooting-5-issues.md | AC-7 — Troubleshooting >= 5 issues | PASS |
| 08-ac-8-known-regressions.md | AC-8 — Known regressions + PRE-FILLED beta.1 | PASS |
| 09-ac-10-status-banner-replaced.md | AC-10 — Banner replacement | PASS |
| 10-task-16-readme-l264-update.md | Task 16 — README L264 before/after | PASS |
| 11-task-17-v1.0-index-canonicalization.md | Task 17 — v1.0-index.md S-5.7 → S-5.07 | PASS |
| 12-bats-green.md | Bats 17/17 PASS | PASS |
| 13-ac-9-human-review-deferred.md | AC-9 — manual gate, deferred to PR review | DEFERRED |
