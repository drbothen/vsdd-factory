# PR Review — #714 `fix(templates): add mandatory CHANGELOG delivery task to story template`

- **Reviewer:** vsdd-factory:pr-reviewer (fresh diff-only, cognitive-diversity pass)
- **Base:** develop
- **Class:** fix-PR (backlog remediation, ref #580)
- **CI:** ALL CHECKS GREEN (validate, cargo-host ubuntu+macos, bats full/wave-handoff/darwin-leg, build-dispatcher ×5, SAST semgrep)
- **Verdict:** **REQUEST_CHANGES**

---

## Verdict rationale

The change is small, well-scoped, and correctly motivated (2nd recurrence of "story PR ships
without a CHANGELOG entry", caught late in F5 convergence). The three prompt/template edits are
mutually consistent. It is blocked only because its value hinges on precision and the CHANGELOG
anchor it points at does not cleanly exist in this repo — a cheap in-scope fix, not a follow-up.

---

## Findings

### [MAJOR] CHANGELOG structure referenced by the new guidance does not cleanly exist
`coherence` / `correctness`

The template task 12, the skill step 5, and the story-writer bullet all instruct authors to add
an entry "under `[Unreleased]` > Fixed/Changed/Added". Verified against the actual repo state:

- **Bracket mismatch:** live section is `## Unreleased` (no brackets); the diff and the bats grep
  use the bracketed `[Unreleased]` form. An agent following the task literally may create a second,
  divergent `## [Unreleased]` heading and fragment the changelog.
- **Placement:** the `## Unreleased` section is empty and buried mid-file (`CHANGELOG.md:656`,
  between `## 1.0.0-rc.23` at top and `## 1.0.0-rc.16` below). Entries have not been accumulating
  there in practice.
- **Release-flow consumption:** `RELEASING.md:295–300` + `scripts/bump-version.sh` describe the
  operator synthesizing CHANGELOG narrative from commits at release time and prepending a fresh
  `## <full-semver>` stub. The documented flow does not harvest an `Unreleased` accumulation
  section, so entries added under it may never be drained — undercutting the PR's end-to-end goal.
- Subsection names ("Fixed/Changed/Added") are valid — repo uses `### Added/Changed/Fixed`. Fine.

**Suggestion:** either align the wording to the repo's actual `## Unreleased` section, or (preferred)
canonicalize `CHANGELOG.md` to a top-of-file `## [Unreleased]` section and confirm the release flow
drains it. Route wording to story-writer (this PR); CHANGELOG/release hygiene to devops-engineer via
orchestrator — fix in the same cycle.

### [MINOR] Test coverage asymmetric — only 1 of 3 changed artifacts guarded
`coverage`

The bats test asserts only that `story-template.md` carries the row. `story-writer.md` and
`create-story/SKILL.md` are changed but untested and can silently drift from the template wording.
**Suggestion:** assert the row in those two files too, or assert one shared canonical string across all three.

### [MINOR] bats grep brittle to rewording
`coverage`

`grep -i 'CHANGELOG entry under \[Unreleased\]'` is coupled to exact phrasing; a harmless future
rephrase flips the test red while untested siblings drift. **Suggestion:** anchor on a stabler
token (`CHANGELOG` + `Unreleased`) or a shared constant.

### [ADVISORY] New task has no enforcement point — authoring-time guidance only
`enforcement`

`validate-template-compliance.sh` enforces at `##`-heading granularity, so the row inside the
existing `## Tasks (MANDATORY)` section is unenforced; delivery relies on the story-writer honoring
the template, and nothing guarantees the implementer actually ships the entry in the PR. The
author's deferral of row-level warn-vs-block to the maintainer is a legitimate "surface the
decision" (CLAUDE.md Rule 5) and acceptable. The durable anti-recurrence fix is a PR-time/CI check,
not template text — a legitimate follow-up if surfaced.

### [ADVISORY] Description-precision nit
`description`

PR body labels the agent-prompt list "Per-Story Requirements list"; the diff added the bullet to the
"Each story includes:" list. Same list substantively; no code impact.

---

## Checklist coverage

1. Diff coherence — OK (all edits relate to the CHANGELOG-task goal).
2. Description accuracy — OK aside from the ADVISORY label nit.
3. Test coverage — MINOR gap (2 of 3 files untested).
4. Demo evidence — N/A (docs/template fix-PR, no ACs).
5. Commit quality — OK (`fix(templates):` conventional).
6. Diff size — OK (~25 lines).
7. Missing changes — none; all three artifacts + test present as described.
8. Dependency status — N/A (no upstream story deps).

## Dispatcher build impact

**No.** Diff touches only Markdown (`templates/`, `agents/`, `skills/`) plus one `.bats` file. No
`crates/`, `Cargo.*`, `hooks-registry.toml`, WASM, or dispatcher source. Green `build-dispatcher`
matrix is consistent; the noted darwin-x64 flake is unrelated to diff content.

## Summary

Good, small, correctly-motivated defensive fix with mutually consistent edits, but its worth depends
on precision and the referenced `[Unreleased]` anchor is off (unbracketed live section, empty and
buried mid-file, not demonstrably drained by the documented commit-synthesis release flow). That,
plus the single-artifact test, are cheap in-scope fixes rather than follow-ups — hence
REQUEST_CHANGES. Deferring `validate-template-compliance.sh` row-level enforcement is acceptable.
