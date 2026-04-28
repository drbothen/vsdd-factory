---
document_type: tech-debt-register
producer: state-manager
version: "1.0"
last_updated: 2026-04-25T00:00:00
---

# Technical Debt Register

## Summary

| Priority | Count | Estimated Points |
|----------|-------|-----------------|
| P0 (next cycle) | 0 | 0 |
| P1 (within 3 cycles) | 1 | XL (29–39 across 6 sub-stories) |
| P2 (backlog) | 7 | — |
| P3 (v1.1+) | 3 | — |

## Debt Items

| ID | Source | Description | Priority | Introduced | Cycle | Story | Due |
|----|--------|-------------|----------|-----------|-------|-------|-----|
| TD-001 | Phase 5 deferred | BC-level CAP/DI/Stories anchoring incomplete: all 1,851 BC files carry CAP-TBD/DI-TBD/Stories-TBD defaults from Phase 1.4b migration | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-002 | Phase 5 deferred | BC-INDEX status column all "draft" regardless of shipped/partial/pending reality | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-003 | Spec drift | BC frontmatter lacks per-BC lifecycle_status field; PRD claims FR-level status but BCs have no per-BC marker | P3 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.1 |
| TD-004 | Phase 5 deferred | BC-7.01 family is mixed (multiple hooks); FR-032 BC-group labeling conflicts with BC-7.01.001 H1 (block-ai-attribution vs protect-secrets) | P2 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-005 | Phase 5 deferred | Agent registry missing (34 agents not enumerated); NFR-PERF not in PRD §4.2 top-5 | P3 | v1.0.0-beta.4 | v1.0-brownfield-backfill | — | v1.1 |
| TD-006 | Process gap | validate-consistency Check 8/9 ship as procedural spec only — no executable runner, no bats/Rust tests, Rust-only language scope, bypassed TDD (no test-writer/implementer dispatch), and was authored directly on main instead of feature-branch-off-develop | P1 | post-Wave-9 | v1.0-brownfield-backfill | — | v1.0.1 |
| TD-007 | Spec deferral | S-3.04 AC-003: bash bin/emit-event still in use by legacy hooks; full retirement deferred from v1.0 to post-v1.0 milestone | P3 | v1.0.0-beta.4 | v1.0-brownfield-backfill | S-3.04 | v1.1 |
| TD-008 | Process gap | S-4.10 RED gate test pattern: emission tests created channel (tx, rx) but never passed tx to sink; required test-writer fix burst. Lessons-codification candidate for S-7.03 update — RED gate must wire all dependencies the implementer is expected to use. | P2 | Wave 12 | v1.0-brownfield-backfill | S-4.10 | v1.0.1 |
| TD-009 | Process gap | Pre-flight git-diff check before merging release/develop branches — caught at L2 risk in this cycle. Should be codified as a process check in CONTRIBUTING.md or pre-merge lint hook. | P2 | Wave 12 | v1.0-brownfield-backfill | — | v1.0.1 |

### Source Types

| Source | Detection Agent | Description |
|--------|----------------|-------------|
| Phase 5 deferred | adversary | Finding deferred as "fix later" from adversarial review |
| Phase 6 deferred | formal-verifier | Finding deferred from formal hardening |
| Spec drift | spec-steward | BC postcondition not enforced in code |
| Dependency | security-reviewer | Major version bump available or vulnerability |
| DTU fidelity | dtu-validator | Real API changed, clone is stale |
| Pattern inconsistency | code-reviewer | Legacy pattern in older code |
| Holdout decay | holdout-evaluator | Scenario tests removed/changed feature |
| Maintenance sweep | consistency-validator | Anti-pattern or code smell detected |

### Item Details

#### TD-001 — BC-level CAP/DI/Stories anchoring incomplete
**Source:** Phase 1d pass 1 (F-003, F-005, F-011)
**Description:** All 1,851 BC files have `capability: CAP-TBD`, `L2 Domain Invariants: TBD`,
`Stories: TBD` — best-effort default from Phase 1.4b migration. PRD §8 anchors at FR-level;
BC-level reverse anchoring is incomplete.
**Severity:** P2 (does not block v1.0 GA — PRD has full traceability; per-BC anchoring
is a navigability improvement).
**Plan:** Wave-scale follow-up. After 3-clean-pass adversarial convergence on the spec
package, run a backfill burst: for each BC, read PRD §7 traceability matrix → assign
CAP, lookup DI from L2, populate Stories from STORY-INDEX.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-002 — BC-INDEX status column all "draft"
**Source:** Phase 1d pass 1 (F-006)
**Description:** All 1,851 BC-INDEX rows show status=draft regardless of whether the
underlying behavior is shipped, partial, or pending. Status should reflect the
implementation reality (22 stories merged + 4 partial + 15 draft).
**Severity:** P2 (PRD FR-level status is correct).
**Plan:** Same backfill burst as TD-001 — derive BC status from STORY-INDEX status of
the implementing story.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-003 — BC frontmatter lacks per-BC `lifecycle_status` field
**Source:** Phase 1d pass 1 (F-011)
**Description:** PRD claims FR-level partial/shipped/pending status but BC files have
no per-BC lifecycle marker (only the `lifecycle_status: active` from the template).
**Severity:** P3 (covered by BC-INDEX status column once TD-002 resolves).
**Plan:** Either schema decision (add field) or rely on BC-INDEX as the status authority.
**Cycle estimate:** v1.1.

#### TD-004 — BC-7.01 family is mixed; FR-032 BC-group labeling is ambiguous
**Source:** Phase 1d pass 1 (F-013)
**Description:** BC-7.01 family contains BCs for multiple hooks (block-ai-attribution in
BC-7.01.001, protect-secrets in BC-7.01.004, capture-commit-activity in BC-7.01.002,
regression-gate in BC-7.01.003). PRD FR-032 labels the BC-7.01 range as "protect-secrets.sh"
which conflicts with BC-7.01.001 H1 ("block-ai-attribution"). The actual alpha-sort order
of SS-07 hooks assigns block-ai-attribution as alphabetically first.
**Severity:** P2 (spec navigability; does not affect implementation).
**Plan:** Rationalize SS-07 BC family assignments: re-shard so each BC family maps to
exactly one hook script. Update PRD FR-032..FR-034 BC-group listings to match.
**Cycle estimate:** v1.0.1 or v1.1.

#### TD-005 — Agent registry and NFR-PERF top-5 not enumerated in spec
**Source:** Phase 1d pass 1 (F-016, F-017)
**Description:**
- F-016: 34 agents dispatched by the factory are not enumerated in any formal registry. An `agents.md` under `domain-spec/` would list each agent with its role, model tier, and tool access.
- F-017: PRD §4.2 Top-5 Priority NFRs does not include any NFR-PERF (performance) entry. At least one NFR-PERF (e.g., sub-100ms hook latency per DI-011/BC-7.02.005) should appear in the top-7 or be explicitly excluded with rationale.
**Severity:** P3 (docs/navigability).
**Pre-requisite note:** Reconcile PRD line 75 + ARCH-INDEX line 96 (34 agents) vs former SS-05 line 25 (33 agents). [Resolved in phase-1d pass-2 fix burst — SS-05 now says 34 specialist sub-agents.]
**Plan:** v1.1 — add `domain-spec/agents.md` registry; expand PRD §4.2 to top-7 or add NFR-PERF exclusion note.
**Cycle estimate:** v1.1.

#### TD-006 — validate-consistency Check 8/9 missing executable predicate, tests, and proper TDD trail
**Source:** User audit of Task #114 (post-deliverable review, 2026-04-27)
**Description:** Task #114 extended `plugins/vsdd-factory/skills/validate-consistency/SKILL.md` with two new advisory checks (Check 8 — Test Tautology Detector, MEDIUM; Check 9 — BC Canonical TV ↔ Emitter Field Consistency, HIGH) and registered POLICY 11 + POLICY 12 as opt-in promotion paths. The deliverable is procedural-spec grade, not code grade:

1. **No executable runners.** No `bin/check-tautology` or `bin/check-bc-tv-consistency` script exists. The check fires only if a human (or LLM following SKILL.md) manually performs the procedure. The skill cannot be invoked end-to-end.
2. **No tests.** Zero bats / Rust unit / integration tests exist. The 9 fixture files under `fixtures/tautology/` and `fixtures/bc-tv-consistency/` are illustrative pseudo-code (each carries `FIXTURE: this file is NOT compiled`). Nothing asserts that the predicate actually flags `flagged_*.rs` and ignores `clean_*.rs`.
3. **Rust-only language scope.** The detection regex (`rg --type rust`), the production-fn pattern (snake_case Rust idioms), and the skip semantics (`#[serde(skip_serializing_if)]`, `Option<T>`) are all Rust+serde-specific. For TypeScript/Python/Go/etc projects, both checks no-op and provide zero value. The SKILL.md acknowledges this with a "skipped (no Rust sources detected)" line but does not propose a generalization path.
4. **Bypassed TDD.** Task #114 was executed directly by the orchestrator in auto mode without dispatching test-writer (RED gate) or implementer (GREEN cycle). No story spec, no BC anchor, no per-AC demo evidence, no PR review.
5. **Git Flow violation.** Plugin source edits were authored directly against `main` working tree instead of branching from `develop` per the Git Flow process established in Task #33. Subsequently moved to feature branch + PR (this remediation), but the process slip should be recorded.

**Severity:** P1 — the skill claims two new checks in its public contract (SKILL.md + policy registry references them) but they're not actually runnable. This is "promised behavior, no implementation" — same drift class as the BC-vs-emitter pattern Check 9 itself targets.

**Plan:**
1. **Story spec** — write `S-X.YY-validate-consistency-tautology-bc-tv.md` anchoring to a real or candidate BC; trace ACs to fixture-driven test outputs.
2. **Test harness** — add bats tests under `plugins/vsdd-factory/tests/skills/validate-consistency/` that:
   - run a `bin/check-tautology` prototype against `fixtures/tautology/` and assert: 2 flags from `flagged_tautological_test.rs`, 0 from each `clean_*` file, opt-out exception honored.
   - run `bin/check-bc-tv-consistency` against `fixtures/bc-tv-consistency/` and assert flagged BC↔emitter pair surfaces 1 finding, clean Option-skip pair surfaces 0, `tv_emitter_check: skip` BC is skipped entirely.
3. **Executable runner** — implement `bin/check-tautology.sh` and `bin/check-bc-tv-consistency.sh` as bats-driven Rust-only probes. Emit JSON-Lines per finding so consistency-validator can splice into the Advisories section.
4. **Language scope decision** — record an ADR ("Check 8/9 are Rust-only by design — generalization to TS/Python is a future story") OR generalize via per-language matchers (Rust now, TS next, etc.). User to decide.
5. **Run through proper TDD** — test-writer first (RED gate), then implementer (GREEN), then demo-recorder (per-AC evidence), then PR via pr-manager. Per per-story-delivery cycle.

**Cycle estimate:** v1.0.1 (PATCH if Rust-only; MINOR if generalized to multi-language).

**PR reference:** PR #17 (`feat/validate-consistency-tautology-bc-tv` → `develop`) — ships the procedural spec + 9 fixtures. Merge of #17 does NOT close TD-006; the runner+tests follow-up does.

**Pre-decision questions (RESOLVED 2026-04-27 — story-writer can now proceed):**

Q1 + Q5 (blocking pivotals) answered by user. Q2/Q3/Q4/Q6/Q7/Q8 defaulted to the recommended values per session 2026-04-27. Story spec for TD-006 follow-up is unblocked.

##### Q1 — Language scope ✅ ANSWERED: Path B (all four languages)

**Decision:** Runner supports **Rust + Python + TypeScript + Golang** in the initial story (no phasing). The skill becomes universal across vsdd-factory's full language footprint.

**Implications:**
- Per-language **test-fn matchers**: Rust `fn test_*`, Python `def test_*`, TS `test(...)/it(...)/describe(...)`, Go `func Test*`.
- Per-language **production-fn patterns**: Rust snake_case, Python snake_case, TS camelCase + class methods, Go PascalCase exported + camelCase unexported.
- Per-language **struct/type matchers**: Rust `struct`/`enum`, Python dataclass/pydantic-`BaseModel`/attrs, TS `interface`/`class`/`type`/`zod schema`, Go `type X struct`.
- Per-language **serialization opt-out conventions**:
  - Rust: `#[serde(skip_serializing_if = ...)]`, `Option<T>`, `#[serde(skip_serializing)]`.
  - Python: `field(default=None, repr=False)`, `Optional[T]`, pydantic `Field(exclude=True)`, `model_dump(exclude={...})`, `attrs.field(metadata={"serialize": False})`.
  - TS: `class-transformer @Exclude()`, `class-transformer @Expose({groups: [...]})`, `zod .optional()`, JSON-omit via `omit` utility, `?` field marker.
  - Go: `json:"-"` tag, `json:",omitempty"`, pointer types (`*T`) with nil-handling, custom `MarshalJSON`.
- **Language detection**: file extension primary (`.rs`, `.py`, `.ts`/`.tsx`/`.mts`/`.cts`, `.go`); content sniffing for ambiguous cases (e.g., `.d.ts` declaration-only files skipped).
- **Test fixtures**: 4 flagged + 4 clean per check per language = ~32 new fixture files (current 9 → ~41 total).
- **No skip path** for any of the four languages. The "skipped — no Rust sources detected" advisory line is removed entirely.

**Sizing impact:** ~3× the Path C estimate. See Q7 update below.

**Out of scope (this story):** Java, Kotlin, Ruby, C#, PHP, Swift, Elixir, etc. Treat those as future stories if the user adds projects in those languages later.

##### Q2 — File path concretization ✅ DEFAULTED

**Decision (recommended default accepted):**

- **Runners:** `plugins/vsdd-factory/skills/validate-consistency/runners/check-tautology` and `runners/check-bc-tv-consistency` (skill-local). Implementation language: bash dispatcher + per-language matcher modules (Rust matcher in awk/ripgrep, Python matcher in Python AST, TS matcher in node + ts-morph or tree-sitter, Go matcher in Go's `go/ast`). Each language matcher emits the same JSON-L schema so the dispatcher concatenates without translation.
- **Bats tests:** `plugins/vsdd-factory/skills/validate-consistency/tests/check-tautology.bats` and `tests/check-bc-tv-consistency.bats` (skill-local, co-located).
- **Fixtures:** keep current `fixtures/tautology/` and `fixtures/bc-tv-consistency/` directories. Reorganize to per-language subdirectories: `fixtures/tautology/rust/`, `fixtures/tautology/python/`, `fixtures/tautology/typescript/`, `fixtures/tautology/golang/`. Same for `bc-tv-consistency`.
- **Expected-output fixtures:** `fixtures/tautology/<language>/expected/<fixture-name>.expected.jsonl` (one expected file per input fixture).
- **Language matcher modules:** `plugins/vsdd-factory/skills/validate-consistency/runners/matchers/rust.sh`, `matchers/python.py`, `matchers/typescript.ts`, `matchers/golang.go`.

##### Q3 — Output schema ✅ DEFAULTED

**Decision (recommended default accepted; extended for multi-language):**

```jsonl
{"check": "VC8", "policy_id": "POLICY-VC-008", "severity": "MEDIUM", "language": "rust", "file": "crates/foo/tests/bc.rs", "line": 42, "function": "test_BC_3_02_001_emits_entry", "evidence": "...", "suggestion": "...", "finding_id": "VC8-rust-<sha1-of-file-line-fn>", "opted_out_via": null}
{"check": "VC9", "policy_id": "POLICY-VC-009", "severity": "HIGH", "language": "rust", "bc": "BC-3.02.013", "field": "trace_id", "struct": "LogEntry", "location": "crates/sink-core/src/entry.rs:24", "evidence": "...", "suggestion": "...", "finding_id": "VC9-<sha1-of-bc-field-struct>", "opted_out_via": null}
```

**Required fields:** `check`, `policy_id`, `severity`, `language`, `file` (or `location` for VC9), `evidence`, `suggestion`, `finding_id`.

**Optional fields:** `opted_out_via` (string when opt-out fires; null otherwise — emitted in a separate "skipped" stream so reviewers can see what was deliberately silenced).

**Suppression file:** `plugins/vsdd-factory/skills/validate-consistency/.suppressions/<finding_id>.md` (per-finding rationale doc). Suppression file presence at runtime causes the runner to drop the finding from the output stream but log it to `.suppressed.jsonl` for audit.

##### Q4 — Edge cases (mandatory for story-spec Edge Cases table)

The story spec's Edge Cases table needs these enumerated. Provisional list:

| ID | Scenario | Expected behavior |
|----|----------|-------------------|
| EC-001 | BC frontmatter missing `target_module:` | Skip the BC; emit advisory `VC9-skipped-no-target-module` once |
| EC-002 | BC has `## Canonical Test Vectors` but no excluded-field columns | Skip the BC silently |
| EC-003 | Rust file has `#[cfg(test)]` but zero matching test fns | Skip the file silently |
| EC-004 | Empty fixture directory | Runner exits 0 with empty output |
| EC-005 | Multiple structs with same name in different crates | Resolve via BC `target_module:` path; if ambiguous, emit warning + skip |
| EC-006 | `Option<T>` without `skip_serializing_if` but inside an explicit `Serialize` impl | Currently FLAG; story should decide whether to inspect impl block |
| EC-007 | BC frontmatter has `tv_emitter_check: skip` but no `## Canonical Test Vectors` section | Skip silently (directive applies vacuously) |
| EC-008 | Test fn name matches pattern but has zero body lines | Skip (vacuous tautology, not interesting) |
| EC-009 | Production fn called as a method (`x.emit_event()`) instead of as a free fn | Currently regex would miss; story should decide whether to broaden pattern |
| EC-010 | Workspace has no Rust at all (TS/Python project) | Runner emits one advisory `Check 8/9: skipped (no Rust sources detected)` and exits 0 |

Recommended default: accept the 10 above as the baseline; story-writer adds any others discovered during test-design.

**Decision:** ✅ DEFAULTED. Story-writer accepts the 10 baseline edge cases plus per-language additions:

- **EC-011** (Python-specific): pydantic v1 `class Config: ...` opt-out vs pydantic v2 `model_config = ConfigDict(...)` opt-out — both must be honored.
- **EC-012** (TS-specific): TypeScript `interface` (compile-time only, no runtime) vs `class` (runtime) — interface fields cannot be runtime-skipped, so Check 9 must distinguish.
- **EC-013** (Go-specific): exported field (`PascalCase`) without `json:"-"` tag vs unexported field (`camelCase`) which is implicitly excluded by `encoding/json` — Check 9 must not flag unexported fields.
- **EC-014** (cross-language): a polyglot project with Rust + Python — per-language results must be aggregated into one JSON-L stream with `language` field disambiguating.

##### Q5 — Self-application policy promotion ✅ ANSWERED: Gating (immediate)

**Decision:** vsdd-factory promotes POLICY 11 + POLICY 12 from opt-in to **active gating** in its own `.factory/policies.yaml` as part of TD-006 closure. Adversary fails the gate if vsdd-factory's own tests are tautological or its BC TVs drift from emitters.

**Implications:**

1. **Legacy-finding triage is part of TD-006's scope.** Before the gate can pass, every legacy finding the runner surfaces in vsdd-factory's existing code/BCs must be either fixed (preferred) or formally suppressed via `.suppressions/<finding_id>.md` with explicit rationale.
2. **Forcing function on Phase 5 / future adversarial reviews.** Once gating is on, no spec or implementation change can pass adversarial review without clearing both checks. This is the strongest dogfooding stance — it explicitly accepts the cost.
3. **Volume unknown until runner exists.** vsdd-factory has 1,893 BCs (many brownfield-extracted, may not have Canonical Test Vectors sections) and a Rust workspace with tests under various naming conventions. The actual legacy-finding volume could be 0 (if naming/TV conventions don't match the matchers) or hundreds (if they do). The story spec must include a triage budget that allows for both extremes.
4. **Suppression policy.** Suppressions are explicit, time-bounded, and attached to a TD entry (e.g., suppression cites `TD-NNN`). No silent silencing.
5. **Sequencing.** PR for TD-006 follow-up lands the runner + tests + per-language matchers in one PR; a SECOND PR (or commit-in-same-PR if scope allows) flips POLICY 11/12 to gating in `.factory/policies.yaml` AFTER triage is complete.

**Risk acknowledgment:** If legacy-finding volume is high (>50), TD-006 expands into a multi-week effort. The story spec must explicitly handle "stop, triage as TD-NNN, suppress, gate" as a fallback path so a finding storm doesn't block all forward progress.

##### Q6 — NFRs / performance budgets ✅ DEFAULTED (adjusted upward for multi-language)

**Decision (recommended defaults accepted; latency budget adjusted for 4 languages):**

- **Latency budget:** Check 8 < 30s for the full 1,893-BC repo with Rust + Python + TS + Go matchers running. Check 9 < 30s. (Previous Rust-only budget was <10s; multi-language tripling accounts for tree-sitter / AST parsing in non-Rust languages.)
- **Cacheable?** No cache in v1. Premature optimization given the budget.
- **Parallel-safe:** Yes by construction (runners are read-only).
- **Side-effect free:** Yes — no writes anywhere, no `.factory/` mutation, no fixture rewriting.
- **Per-language sub-budgets:** Rust matcher < 5s, Python matcher < 8s (AST parsing), TS matcher < 12s (tree-sitter or ts-morph node startup is the dominant cost), Go matcher < 5s. Total dispatcher orchestration overhead < 5s.
- **Failure mode:** If any per-language matcher exceeds its budget, runner emits `<lang>-matcher-timeout` advisory and skips that language for the run. Other matchers continue. The dispatcher exits 1 only if ALL matchers fail; partial coverage exits 0 with the advisory.

##### Q7 — Story sizing ✅ UPDATED for Path B + Gating

**Decision:** Sizing now reflects Q1=Path B (4 languages) + Q5=Gating (legacy triage included).

**Initial-build phase (story-writer + test-writer + implementer + demo-recorder):**

| Phase | Activity | Days |
|-------|----------|------|
| Story spec | Write S-X.YY-validate-consistency-checks-runner.md with 4-language scope | 1.0 |
| BC creation | Pre-emptive BCs for Check 8/9 promised behavior (BC-6.NN.NNN candidates) | 0.5 |
| Test-writer (RED) | 4 langs × 2 checks × ~4 fixtures × expected-output files = ~32 fixture+expected pairs + bats harness | 2.0 |
| Implementer (GREEN) | Per-language matchers (Rust+Python+TS+Go) + dispatcher + JSON-L output + edge cases | 4.0 |
| Demo-recorder | Per-AC evidence across all 4 languages | 1.0 |
| Adversarial pass-1..N | Spec + impl convergence (3 clean passes) | 1.5 |

**Subtotal (initial build):** ~10 days.

**Legacy-finding triage phase (Q5 gating prep — variable, scope-dependent):**

| Volume scenario | Triage days |
|-----------------|-------------|
| 0–10 findings (best case) | 0.5 |
| 11–50 findings (likely) | 2.0 |
| 51–200 findings (storm) | 5–10 (becomes a sub-cycle of its own; may split into multiple TD entries) |
| 200+ findings (catastrophic) | Reassess: defer Q5 gating to v1.1, ship advisory-only first |

**Recommended budget:** Plan for 11–50 (likely), reserve story-spec language to allow re-scope to advisory-first if volume exceeds 200. Triage outputs are TD-NNN entries with rationale-suppressed findings.

**Promotion phase (POLICY 11/12 → gating):**

| Phase | Activity | Days |
|-------|----------|------|
| Policy edit | Flip `enforced_by:` lists to include adversary-prompt + lint-hook in `.factory/policies.yaml` | 0.25 |
| Hook integration | Add validate-consistency invocation to pre-PR CI hook | 0.5 |
| First-gate-pass run | Run gating against `develop` HEAD, confirm clean (or all suppressions doc'd) | 0.25 |

**Subtotal (gating promotion):** ~1 day.

**Total estimate:** **12–14 days** (best case 0–10 legacy findings) to **17–22 days** (likely 11–50 findings). **Points: 21** (best case) to **34** (likely). **Effort: XL.**

**Story splitting recommendation:** This is too big for a single story spec. Recommend decomposing into sub-stories:

| Story | Scope | Points |
|-------|-------|--------|
| S-X.01 | Runner architecture + dispatcher + Rust matcher + Rust fixtures + bats | 8 |
| S-X.02 | Python matcher + Python fixtures + bats | 5 |
| S-X.03 | TypeScript matcher + TS fixtures + bats | 5 |
| S-X.04 | Golang matcher + Go fixtures + bats | 5 |
| S-X.05 | Legacy-finding triage on vsdd-factory's own code | 3–13 (volume-dependent) |
| S-X.06 | POLICY 11/12 promotion to gating + CI hook integration | 3 |

**Total decomposed: 29–39 points across 6 stories.** Each story can converge independently before the next starts. S-X.05 is the unknown-volume gate before S-X.06 can land.

##### Q8 — Definition of Done ✅ FINALIZED for Q1=Path B + Q5=Gating

**TD-006 closes when ALL of the following are true:**

- [ ] **Story specs merged.** S-X.01 through S-X.06 all merged into `develop` with their PRs.
- [ ] **All bats tests pass.** Per-language fixtures (Rust + Python + TS + Go) — `flagged_*` yields expected findings, `clean_*` yields zero, all opt-outs honored, all 14 edge cases covered.
- [ ] **CI integration live.** validate-consistency runs as a pre-PR hook on every PR targeting `develop`. Hook exit code blocks merge on findings (post-promotion).
- [ ] **Legacy-finding triage complete.** Every finding from running the runner against vsdd-factory's own `develop` HEAD is either: (a) fixed in code/BCs, OR (b) explicitly suppressed via `.suppressions/<finding_id>.md` linked to a TD-NNN entry.
- [ ] **POLICY 11/12 promoted to gating.** `.factory/policies.yaml` `enforced_by:` lists for both policies include `adversary-prompt` + `lint-hook` (currently they include `validate-consistency` + `adversary-prompt`; lint-hook addition is the gate-flip).
- [ ] **First-gate-pass clean.** Adversarial review run against `develop` HEAD post-promotion confirms zero unsuppressed findings.
- [ ] **ADR documenting language-scope architecture.** ADR records per-language matcher plug-in pattern; explains why Rust+Python+TS+Go are the v1 set; documents extension procedure for future languages (Java/Kotlin/Ruby/etc).
- [ ] **STATE.md D-NNN entry recording closure** with the 6 sub-stories' commit SHAs and a final cumulative-findings count.
- [ ] **Resolution History row appended** to this register with date, sub-story IDs, final outcome.
- [ ] **Pre-flight check codified.** The `git fetch && git log --oneline origin/develop..origin/main && git diff origin/main origin/develop -- <files>` pattern is added to a developer-onboarding doc or a pre-feature-branch lint hook (this is a process gap surfaced during TD-006 remediation).

**Cycle estimate:** v1.0.1 (PATCH if no public-API changes) or v1.1 (MINOR if the runner exposes a new public skill API surface that other plugins can call).

#### TD-008 — S-4.10 RED gate test-wiring gap
**Source:** Wave 12 per-story-delivery cycle test-writer fix burst (2026-04-27)
**Description:** During S-4.10 TDD cycle, the test-writer's RED gate created a channel `(tx, rx)` for sink communication but never passed `tx` to the sink under test. The sink consequently never received events, causing tests to hang or pass vacuously without exercising the intended behavior. A separate test-writer fix burst was required to wire `tx` correctly. This is a lessons-codification candidate for the S-7.03 TDD Discipline Hardening story — the RED gate contract must explicitly require that all dependencies the implementer is expected to use are wired into the test harness before declaring RED.
**Severity:** P2 — does not block any current story; creates risk of similar wiring gaps in future sink test harnesses.
**Plan:**
1. Add an explicit checklist item to the test-writer agent prompt: "Before declaring RED, verify every dependency (channels, handles, clients) passed to the SUT is actually wired and exercised by at least one test assertion."
2. Consider adding a bats fixture pattern showing the correct channel-wiring pattern for sink tests.
3. Candidate for S-7.03 follow-up story or an additional AC in a future process-hardening story.
**Cycle estimate:** v1.0.1.

#### TD-009 — Pre-flight git-diff check before release/develop merges
**Source:** Wave 12 cycle risk triage (2026-04-27)
**Description:** During this cycle, a risk was flagged (L2) that release branches could be merged into develop without a pre-flight diff check, potentially introducing unintended changes. The pattern `git fetch && git log --oneline origin/develop..origin/main && git diff origin/main origin/develop -- <files>` is known (referenced in TD-006 DoD) but not codified as a mandatory process step.
**Severity:** P2 — does not block current work; represents an operational risk for future release cycles.
**Plan:**
1. Add a pre-merge checklist section to `CONTRIBUTING.md` documenting the pre-flight diff command.
2. Alternatively, implement as a pre-merge lint hook that runs on PRs targeting `main` or release branches.
3. Coordinate with TD-006 closure (Q8 pre-flight check item) to avoid duplicating the codification.
**Cycle estimate:** v1.0.1.

#### TD-007 — S-3.04 AC-003 deferred: bash bin/emit-event retirement
**Source:** S-3.04 spec body explicit deferral note (v1.0.0-beta.4 ship); confirmed by post-Wave-9 status-drift audit 2026-04-27.
**Description:** S-3.04 (emit_event as host function refactor) shipped 4 of 5 ACs in v1.0.0-beta.4: emit_event() routes to configured sinks (BC-1.05.012-019), plugin events appear in JSONL/OTel streams, plugin events enriched with trace_id/session_id, integration tests pass. AC-003 — "bin/emit-event deprecated; callers migrated" — was carved out of v1.0 scope because legacy bash hooks still call the old `bin/emit-event` shell tool. Full retirement requires migrating all bash hook callers to use the host fn directly (or via a thin wrapper) and then removing `bin/emit-event` from the dispatcher.
**Severity:** P3 — does not block v1.0 GA. The host fn IS implemented and works for native WASM plugins; only legacy bash hooks still use the old binary. Both code paths coexist.
**Plan:** v1.1 follow-up:
1. Audit all callers of `bin/emit-event` (grep across hooks/, scripts/, plugins/).
2. Migrate each caller to use the host fn directly via WASM, OR provide a thin shim that forwards to the host fn.
3. Remove `bin/emit-event` from the dispatcher binary tree.
4. Update S-3.04 status to fully complete (close TD-007).
**Cycle estimate:** v1.1.

## Resolution History

| ID | Resolved In | Story | Resolution |
|----|------------|-------|------------|
| — | — | — | No items resolved yet |

## Tech Debt as Feature Mode Cycles

When P0 items accumulate, they become a Feature Mode cycle (Path 3) with
cycle type "refactor":

```
orchestrator: "Tech debt P0 items need attention"
  -> Path 3 (Feature Mode) with cycle type "refactor"
  -> cycles/vX.Y.Z-refactor-[name]/
  -> Same VSDD rigor: specs updated, tests updated, adversarial review
  -> Release: PATCH (no new features) or MINOR (if public behavior changes)
```
