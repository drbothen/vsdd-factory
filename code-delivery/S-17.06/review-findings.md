# Review Findings — S-17.06

## Convergence Tracking

| Cycle | Total Findings | Blocking | Fixed | Remaining |
|-------|---------------|----------|-------|-----------|
| 1 | 14 (3 blocking + 11 advisory) | 3 | 3 | 0 → fixes pushed (e9d04851+8ba3dca1) |
| 2 | 2 blocking (test-only) | 2 | 0 | 2 → REQUEST_CHANGES; test fixes being dispatched |
| 3 | TBD | TBD | TBD | TBD → re-review after test fixes |

## Cycle 1 — pr-reviewer (covered_sha: c7e27259be5d8d204853d3dbc1951c7250ed3cb5)

### Blocking Findings

**B-1**: `renew_lock_if_holder` omits `has_factory_lock_key` presence pre-check
- Category: Code fix (spec-fidelity + TD-VSDD-060 sibling-site sweep miss)
- Routed to: implementer
- Status: OPEN

**B-2**: Case 3/5 comparison doesn't apply `trim_git_email` (spec says "after trim_git_email")
- Category: Code fix (spec-fidelity AC-001)
- Routed to: implementer
- Status: OPEN

**B-3**: SEC-004 `now_fn` fix has no regression test; `FnOnce` bound makes revert a compile error
- Category: Code fix (TD-VSDD-059 paper-fix) + test fix
- Routed to: implementer
- Status: OPEN

### Advisory Findings (not blocking)

- A-1: Case 5 can return `Ok((NoOp, None))` via inherited F-R3-005 spurious-renewal guard (architectural; spec/code tension — deferred to architect adjudication)
- A-2/A-3: AC-005 source-scan test self-contradictions in doc/assertions
- A-4: Story AC-004 spec text says `ExecOutput`; shipped `(i32, String)` — spec amendment needed (architect/human call)
- A-5 through A-11: Various lower-priority advisories noted

### Triage Decisions

| Finding | Severity | Route | Rationale |
|---------|----------|-------|-----------|
| B-1 | BLOCKING | implementer | 3-line fix + 1 test; clear spec-fidelity gap |
| B-2 | BLOCKING | implementer | 1-line fix; AC-001 explicit "after trim_git_email" |
| B-3 | BLOCKING | implementer | FnOnce bound change = compile-time safety; TD-VSDD-059 |
| A-1 | advisory | architect (deferred) | Pre-existing F-R3-005 guard; spec/architecture adjudication needed |
| A-4 | advisory | human/architect | Spec text amendment; out of implementer scope |
| A-6 | advisory | implementer (note in fix) | Bare literal 2700 vs TTL_SECONDS import — check if add_import is safe |
| A-11 | advisory | pr-manager (cosmetic) | PR description [x] CI premature — fix in description update |
