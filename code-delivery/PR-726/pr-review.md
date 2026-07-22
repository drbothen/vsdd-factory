# PR Review — #726

`fix(factory-obs+emit-event): correct JSON smoke-test to key=value and leave a breadcrumb when a JSON blob is dropped`

**Verdict: APPROVE**

Fresh-eyes review against diff, PR description, and inlined test evidence. No BLOCKING or MAJOR findings.

## Checklist verification

1. **Diff coherence** — PASS. All three files (`skills/factory-obs/SKILL.md`, `bin/emit-event`, `tests/emit-event.bats`) serve the single #296 fix. No unrelated changes.
2. **Description accuracy** — PASS (one minor discrepancy, see findings). PR body matches the diff.
3. **Test coverage** — PASS. New drop-path, breadcrumb (object + array), and two regression guards target the changed lines.
4. **Demo evidence** — PASS (proportionate). RED / GREEN(a) / GREEN(b) / regression transcripts inlined in the PR body cover both failure and fixed paths. Acceptable for a CLI-tool doc+breadcrumb change of this size.
5. **Commit quality** — PASS. Conventional `fix(...)` subject; `Closes: #296`.
6. **Diff size** — PASS. ~90 lines, well under the 500-line threshold.
7. **Missing changes** — PASS. Doc fix + breadcrumb + tests fully address the stated issue.
8. **Dependency status** — PASS. Self-contained; Class 0, develop-tracked files only; no upstream PR gating.

## Findings

| Severity | Category | Finding | Suggestion |
|----------|----------|---------|------------|
| ADVISORY | correctness/edge-case | The JSON-detection branch (`\{*\|\[*`) is only reached for args with no `=`, because `*=*` is matched first in the `case`. A caller passing a JSON blob whose string values contain `=` (e.g. `'{"cmd":"a=b"}'`) matches `*=*` instead, gets no breadcrumb, and silently creates a garbage key (`{"cmd":"a` = `b"}`). Honest best-effort limitation — the documented footgun (JSON with `:` separators, no top-level `=`) is fully covered, and `*=*`-first ordering is pre-existing behavior. Non-blocking. | Add a one-line comment noting the detection is heuristic, not exhaustive. |
| ADVISORY | description | PR body says "7 new tests" but the displayed diff shows 6 `@test` blocks (hunk header claims +63 lines, so the diff view appears truncated). | Confirm the 7th "confirms the dropped payload" test is present on the branch. |
| NIT | coverage | No single test exercises the mixed case (one JSON-ish arg + valid `key=value` pairs) proving valid fields survive AND the breadcrumb fires together. | Cheap to add; not required. |

## Notes

- Contract reasoning is sound: the change adds a field to the event JSON only, never touching stdout/stderr, preserving the "silent on success, exit 0, writes only the daily log" header contract. The `continue` correctly prevents the JSON-ish token from being parsed as a `key=value` pair.
- Choosing the in-event breadcrumb over a stderr warning is the right call given hook callers depend on the silent contract.
- Case-glob escaping (`\[*`) correctly makes `[` literal rather than a bracket expression.

Approving: well-scoped, correctly reasoned, guards both the new behavior and against regressions.
