---
document_type: adversarial-review-record
level: cycle
cycle: v1.0-brownfield-backfill
wave: 7
pass: 3
scope: [S-21.19, S-21.20, S-21.21, S-21.22, S-21.23]
producer: state-manager
created: 2026-08-22
last_amended: "2026-08-22 (D-1073) — spec-layer remediation disposition appended; story-layer application (D-1074) NOT STARTED, pending resume"
---

# Wave-7 Pre-TDD Adversary Pass-3 / S-21.19 R2 — Compact Record

> Compact record per D-1073 task scope: finding ID · severity · one-line ·
> disposition. NOT a verbatim reproduction of the fresh-context adversary
> transcripts. Full narrative detail: STATE.md Decisions Log D-1073 row.
> Session-wrap PAUSE landed the spec-layer half of this remediation only —
> the story-layer half (D-1074) is NOT STARTED as of this record.

Five stories were dispatched fresh-context, information-asymmetric, against the
D-1072-landed spec state (BC-1.03.017 v1.21 / BC-1.03.018 v1.3): S-21.19's
**second remediation round** (R2) and S-21.20/S-21.21/S-21.22/S-21.23's
**pass-3**. Verdicts below.

## S-21.19 — Executor decision-function core — **CLEAN (R2)**

LOCAL BC-5.39.001 streak: **0/3 → 1/3** (first clean pass since the D-1070
Task-6 reopen reset the streak).

No findings. R1's HIGH (F-S2119-R1-001, ADR-side) and 2 LOW hygiene items
(remediated D-1072) independently re-verified resolved and stable.

Routing: none — clean. R3 (fresh context) is next action toward 3-CLEAN
re-convergence.

## S-21.20 — PC13 full-registry coverage — **NOT-CLEAN (pass-3)**

LOCAL BC-5.39.001 streak: **1/3 REMAINS 1/3** (index-propagation-class finding,
non-resetting — story's own body independently confirmed clean).

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2120-P3-001 | MEDIUM | STORY-INDEX v1.19 drift — the STORY-INDEX catalog row for S-21.20 still cites `BC-1.03.017 v1.19`, stale by two version bumps (now v1.21/soon v1.22); story's own frontmatter + body cites are current | **NOT YET DONE** — routes to state-manager (STORY-INDEX title/version-cite fix), pending resume |

Routing: state-manager (STORY-INDEX fix only; story body itself is clean, no
story-writer action needed for this finding). Streak remains 1/3 (own D-1069
clean-pass-1 baseline, unaffected by this index-propagation-class finding) —
pass-4 (fresh context, after the STORY-INDEX fix + re-anchor to
BC-1.03.017 v1.22) is next action.

## S-21.21 — AMD-002 bash-adapter wiring calibration — **NOT-CLEAN (pass-3)**

LOCAL BC-5.39.001 streak: **0/3 REMAINS 0/3**.

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2121-P3-001 | HIGH | Wave-7→8 Timeout fail-open window — Task 5a's live-wiring literal-replacement wording would strip the retained 2-arg `plugin_fail_closed` call's `Timeout` coverage for one full wave with no disjunct catching it, before S-21.24's wave-8 wiring lands | Fixed spec-side — ADR-044 v1.2→v1.3 (Addendum corrected: wave-7 wiring is now ADDITIVE, retains the 2-arg call + adds `plugin_fail_closed_on_error_exit` alongside it; S-21.24 gains a same-commit MIGRATION sub-task; new Invariant closes the window) + BC-1.03.017 v1.21→v1.22 (new Invariant 12 migration coverage-continuity) (D-1073, architect + product-owner). **Story-side application NOT YET DONE** — S-21.21's own Task 5a body still needs the ADDITIVE (not replacement) wiring description applied; routes to story-writer, pending resume (D-1074). |
| F-S2121-P3-002 | MEDIUM | EC-011 baseline imprecision — the story's edge-case narrative implies `Timeout{Epoch}` is unenforced today, when in fact it IS enforced today via the existing 2-arg `plugin_fail_closed` call (only the migration-window framing was wrong, not the baseline claim) | **NOT YET DONE** — routes to story-writer (EC-011 baseline rewrite + new Timeout+on_error=Block regression fixture), pending resume (D-1074) |

Routing: architect + product-owner (spec-layer, LANDED D-1073) + story-writer
(story-layer application, PENDING D-1074). Streak remains 0/3 — pass-4 (fresh
context, after D-1074 story-layer application + re-anchor to
BC-1.03.017 v1.22) is next action.

## S-21.22 — Native-WASM fuel-axis calibration + flip — **NOT-CLEAN (pass-3)**

LOCAL BC-5.39.001 streak: **0/3 REMAINS 0/3**.

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2122-P3-001 | MEDIUM | BC PC6 frozen-vs-live divergence — BC-1.03.017's Precondition 6 conflated the one-time live-corpus calibration confirmation with the durable standing CI regression assertion; the story's own converged Task 5a already correctly scopes the standing gate to a FROZEN corpus snapshot, but the BC cited a different (live-growing-file) mechanism | Fixed spec-side — BC-1.03.017 v1.21→v1.22 (Precondition 6 split explicitly into the one-time live-corpus confirmation and the durable frozen-snapshot standing gate, matching the story's converged mechanism) (D-1073, product-owner). Story itself already correct, no story-side defect — re-anchor to v1.22 still owed (D-1074). |

Routing: product-owner (spec-layer, LANDED D-1073); story-writer (re-anchor
only, PENDING D-1074 — no body content fix needed, this finding closed the
BC-side divergence, not a story defect). Streak remains 0/3 — pass-4 (fresh
context, after re-anchor to BC-1.03.017 v1.22) is next action.

## S-21.23 — Break-glass override mechanism — **NOT-CLEAN (pass-3)**

LOCAL BC-5.39.001 streak: **0/3 REMAINS 0/3**.

| Finding | Severity | One-line | Disposition |
|---------|----------|----------|--------------|
| F-S2123-P3-001 | HIGH | `all`-wildcard negative-control gap — no test proved the `all` wildcard's scope-restriction guarantee preserves a NON-named blocking plugin's fail-closed enforcement (a naive suppress-every-block implementation would silently fail open for non-named plugins under `all`, a compound CWE-636+CWE-863 hazard with no audit trail) | Fixed spec-side — BC-1.03.018 v1.3→v1.4 (PC8 extended: `all` is name-scoped shorthand for the two named gates ONLY; a non-named blocking plugin's block MUST stand and MUST NOT receive a `break_glass.activated` record; new Canonical Test Vector row) (D-1073, product-owner). **Story-side application NOT YET DONE** — S-21.23 needs a new `all`×non-named-plugin negative-control AC (block stands + no break_glass event); routes to story-writer, pending resume (D-1074). |
| F-S2123-P3-002 | MEDIUM | AC-022 control-count drift — S-21.23's own AC-022 hardened PC9 with a 7th COMMENT-ONLY control and an executable-code-read-pattern detector-precision requirement that BC-1.03.018's PC9 did not carry (story-superset-of-BC drift) | Fixed spec-side — BC-1.03.018 v1.3→v1.4 (PC9 mirrors both: detector-precision requirement + control (g) COMMENT-ONLY; control count six→seven, matching S-21.23's actual coverage) (D-1073, product-owner). **Story-side reconciliation NOT YET DONE** — AC-022's own control-count citation should re-anchor to confirm the BC now matches; routes to story-writer, pending resume (D-1074). |

Routing: product-owner (spec-layer, LANDED D-1073); story-writer (story-layer
new AC + reconciliation, PENDING D-1074). Streak remains 0/3 — pass-4 (fresh
context, after D-1074 story-layer application + re-anchor to
BC-1.03.018 v1.4) is next action.

## Summary

All five spec-layer fixes for this pass LANDED this burst (D-1073): ADR-044
v1.2→v1.3, BC-1.03.017 v1.21→v1.22, BC-1.03.018 v1.3→v1.4. S-21.19 achieved
its first clean pass since reopening (streak 1/3). S-21.20/21/22/23 remain
NOT-CLEAN with story-layer remediation **NOT STARTED** — this is the D-1074
obligation, explicitly deferred by this session-wrap PAUSE (human-invoked
`/wrap`), not a scope omission. On resume: two story-writer sub-bursts apply
the D-1073 spec fixes to the story bodies (S-21.21/S-21.22/S-21.24 + plan;
S-21.23 separately), followed by a state-manager D-1074 commit (story
content + STORY-INDEX S-21.20 title-cite fix + streak updates), then the
pass-4/R3 adversary round.
