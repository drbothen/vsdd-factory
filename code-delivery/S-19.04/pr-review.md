# PR #639 Review — S-19.04 Registry/Bundle Hygiene

## Verdict: APPROVE

`covered_sha: 736d657ce765af8f207742158a82e44297120255`

Fresh-eyes final PR diff review. Reviewed only the diff, PR description, and test
evidence. All 7 acceptance criteria verified independently against the diff, and
both test suites were re-run live on HEAD `736d657c` — **7/7 bats** and **5/5 cargo
integration** tests pass.

---

## Independent Verification (not just trusting the evidence report)

| AC | Gate | Result |
|----|------|--------|
| AC-001(i) | `! grep -q 'example hello-hook' release.yml` | PASS (absent) |
| AC-001(ii) | `! grep -q 'hello-hook.wasm' release.yml` | PASS (absent) |
| AC-002 | 3 orphans (`hello-hook`, `vsdd_context_resolvers`, `wasm_resolver_export`) removed from tracked tree; hyphen `vsdd-context-resolvers.wasm` still tracked | PASS |
| AC-003 | Preamble comment (regex SEARCH / anchoring convention) present | PASS |
| AC-004 | All 54 `tool=` entries both-ends anchored `^(...)$`; zero non-anchored; no indented lines escape the lint | PASS |
| AC-005 | `registry-tool-filter-anchoring.bats` exists; ran live 7/7 pass | PASS |
| AC-006 | `bundle_orphan_check.rs` exists; dual-registry detection | PASS |
| AC-007 | `stage_release_bundle` implemented; ran live 5/5 pass | PASS |

---

## 8-Item Checklist

1. **Diff coherence** — Coherent. The `bc_7_03_079_080_parity.rs` change (`"Agent"` →
   `"^Agent$"`) is a correct TD-VSDD-060 sibling-site sweep forced by the registry
   re-anchoring. The two `artifact-path-registry.yaml` entries are tangential but
   explained (C-P8-001 adversarial correction, D-766/D-774 fold-in).
2. **Description accuracy** — Matches the diff (54 anchored entries, 3 orphan deletions,
   BUILD-OMISSION, underscore-glob in both workflows, POLICY 20, new test suites).
3. **Test coverage** — New bats + cargo suites cover every AC; negative controls
   (T-008, T-011, T-012) confirm the checks are load-bearing, not advisory. Verified
   green locally.
4. **Demo evidence** — 7/7 ACs covered as `.md` gate transcripts (not `.gif/.webm`).
   Appropriate for a config-only story with no runtime UI — every AC is a shell/test
   gate, all of which I re-executed and confirmed pass.
5. **Commit quality** — HEAD matches `covered_sha`; no AI attribution observed.
6. **Diff size** — 1707 insertions, but the production/config surface is small; the
   bulk is test code (670+222 lines) and evidence. Not a concern.
7. **Missing changes** — None. VP-099 traced through AC-003/004/005; nothing in the
   story scope is absent.
8. **Dependency status** — `depends_on: []`; no upstream PRs to gate on.

---

## Findings (all non-blocking)

### [SUGGESTION] simplification — vestigial nested `case` in release.yml staging

In `release.yml`'s two staging steps ("Stage artifact directory" and "Stage wasm
plugins"), the nested `case` is now dead-code-equivalent: after this PR both inner
arms (`vsdd_context_resolvers.wasm|wasm_resolver_export.wasm)` and `*)`) end in
`continue`, so the whole inner `case` collapses to a single `*_*.wasm) continue ;;`.
The only remaining difference is the distinct echo strings ("skip stale resolver
artifact" vs "skip lib-target stub"). Functionally correct, but the nesting adds
cognitive overhead and could mislead a future maintainer into thinking the named
artifacts are treated specially.

*Suggestion:* Collapse to a single `*_*.wasm) echo "skip underscore-named stub:
$name"; continue ;;` arm, or keep the two echoes but drop the inner `case` wrapper.
Not merge-blocking.

### [NIT] robustness — underscore-glob silently drops any future underscore-named WASM (already disclosed as SEC-002)

The `*_*.wasm` glob silently drops *any* future underscore-named WASM, including a
legitimate registry-declared plugin that violates the hyphen convention. Risk is
bounded by the existing "Verify registry-declared WASM plugins are staged" step in
`release.yml`, which fails loudly if a declared plugin is missing. Acceptable as-is;
the hyphen naming convention is the intended guard.

---

## Why This Is Not a Rubber-Stamp

I re-ran both test suites against HEAD (not just read the evidence report),
independently executed all four grep gates against the live files, confirmed the 3
orphans are gone from `git ls-files` while the hyphen resolver survives, verified all
54 `tool=` lines are unindented (so the column-0-anchored bats lint actually covers
every entry, closing a potential false-pass), and traced the negative-control tests
(T-008/T-011/T-012) to confirm the anchoring and dual-registry checks are
load-bearing. The comment-injection regex (`[^"']*\$["']`) correctly refuses to match
a `$` living in a trailing comment. No correctness defect found; the two findings
above are quality/readability only.

**Merge recommendation: APPROVE.** covered_sha `736d657ce765af8f207742158a82e44297120255`.
