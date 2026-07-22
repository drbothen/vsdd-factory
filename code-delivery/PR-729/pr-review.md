# PR #729 — Fresh-eyes review

**Title:** feat(factory-health): content-based factory-artifact leak scan for the product branch
**Author:** arcavenai · **Base:** develop · **Class:** Fix-PR (backlog remediation, issue #515) · **Size:** +299 / -0 across 3 files

## Verdict: REQUEST_CHANGES

No CRITICAL/HIGH defect survived verification, but two MEDIUM detection-completeness gaps
directly undermine the guard's stated purpose ("catch arbitrary-path leaks that `ls-files`
structurally cannot"). Both are ~1-line fixes and were confirmed against the real tree.
Under the production-grade default ("if it's worth doing in v1, do it correctly in v1"), a
leak detector shipped with two known classes of undetectable leaks is not production-grade
for a `feat`. Fix M1 + M2, then this is a clean approve.

### Ground-truth verification performed (fresh-eyes, against develop)
- Reproduced the script's exact `_frontmatter_doctype` awk + main loop and ran it over
  `git ls-files '*.md'` on clean develop → **0 leaks**, matching the PR's evidence. Claim is accurate.
- Refuted a candidate CRITICAL: `plugins/vsdd-factory/agents/*.md` files contain
  `document_type: story` — but only as *body* examples, not in the first frontmatter block.
  The awk's `fm==1` gate correctly ignores them. No false-positive. Good, defensible design.
- Verified `unset 'FACTORY_DOCTYPES[$dt]'` (single-quoted, variable subscript) correctly
  removes allowlisted keys on bash 5.3. The demo-evidence-report / demo-evidence-index files
  under `docs/demo-evidence/**` are correctly exempted (this is why the 0-leak result holds).
- Confirmed the M1 fix (drop `-maxdepth 1`) introduces **no** new false positives in the current tree.

---

## Findings

### M1 — MEDIUM (correctness / coverage) — `find -maxdepth 1` blinds the guard to subdirectory-template doctypes
`bin/factory-artifact-leak-scan.sh`, the FACTORY_DOCTYPES build loop:
```
find "$TEMPLATES" -maxdepth 1 -name '*.md' -type f
```
The doctype universe is derived only from top-level template files. Confirmed on develop:
`plugins/vsdd-factory/templates/adversary-prompt-templates/{phase-1d-spec-review,phase-5-code-review,phase-2-story-review}.md`
all carry `document_type: adversary-prompt-template`, and that doctype exists in **no**
top-level template. So `adversary-prompt-template` is never added to `FACTORY_DOCTYPES`, and a
file bearing that frontmatter leaked to any product path (root, `docs/`, …) is invisible to the
scanner — exactly the failure mode this PR claims to close.

**Failure scenario:** an `adversary-prompt-template` artifact is committed to `docs/` → scanner
reports "Product tree is clean. exit=0" → leak ships undetected.

**Suggestion:** drop `-maxdepth 1` (verified safe — no new false positives on the current tree;
the `templates/*` path exclusion already prevents the templates themselves from being flagged).
If restricting to a curated doctype registry is intentional, that intent must be documented and
the registry must be authoritative — silent under-inclusion is not acceptable for a completeness guard.

### M2 — MEDIUM (correctness / coverage) — `PRODUCT_TRACKED_DOCTYPES` allowlist is doctype-global, not path-scoped
```
PRODUCT_TRACKED_DOCTYPES=(demo-evidence-report demo-evidence-index)
```
These doctypes are exempted **everywhere** in the product tree, not just under their canonical
`docs/demo-evidence/` home. A genuinely-leaked `demo-evidence-report` at the repo root or in an
arbitrary directory is silently exempted — again the precise class of leak the guard exists to catch.

**Failure scenario:** `demo-evidence-report`-frontmatter file committed to repo root → exempted → undetected.

**Suggestion:** scope the exemption to `(doctype, path-prefix)` pairs, e.g. only exempt
`demo-evidence-*` when the path matches `docs/demo-evidence/`; flag the same doctype anywhere else.

### M3 — MEDIUM (correctness) — awk does not anchor the opening `---` to line 1; latent false-positive + full-file read
`_frontmatter_doctype` increments `fm` on any `^---[[:space:]]*$` line and treats the region
after the *first* such line as frontmatter. It does not require the opening fence to be line 1.
Empirically 0 hits on develop today, but any product doc that uses `---` as a thematic-break /
section separator and then has a start-of-line `document_type:` before the next `---` will be
misparsed and can false-positive — a real risk in a repo dense with factory documentation.
Secondary: a file with no closing fence (`---` once or never) is read in full looking for `fm==2`.

**Suggestion:** require the opening fence at `NR==1` (`NR==1 && /^---[[:space:]]*$/`) and only
treat `document_type` as frontmatter when it precedes the closing fence. This also bounds the read.

### A1 — ADVISORY (maintainability / drift) — hardcoded allowlist duplicates registry-derivable knowledge
Author explicitly flagged this. The allowlist is derived by hand "from absence in
`config/artifact-path-registry.yaml`," so the two drift independently. Under the production-grade
default this should not be left as an unguarded judgment call.

**Suggestion:** either derive `PRODUCT_TRACKED_DOCTYPES` from the registry at runtime, or add a
bats case asserting the hardcoded list matches the registry so drift fails CI rather than
silently degrading detection.

### A2 — ADVISORY (portability) — CRLF handling relies on `[[:space:]]` matching `\r`
`^---[[:space:]]*$` and the trim `gsub` assume `[[:space:]]` matches `\r`. True on gawk; not
guaranteed on all awk variants. Given the repo already ships CRLF fixtures
(`docs/demo-evidence/S-17.03/fixtures/fixture-crlf-foreign.md`), CRLF-fronted artifacts are a
real input. **Suggestion:** strip trailing `\r` explicitly, or add a CRLF bats case.

### A3 — ADVISORY (review limitation / wiring) — SKILL.md and bats bodies not in the provided diff
Only `bin/factory-artifact-leak-scan.sh` (162 lines) was reviewable; the ~137 lines of
`tests/factory-artifact-leak-scan.bats` (8 cases) and `skills/factory-health/SKILL.md` check #10
were elided. Two items need explicit confirmation before merge:
1. **Test coverage** — do the 8 cases cover: no-frontmatter file, malformed/single-fence
   frontmatter, allowlisted doctype in canonical vs non-canonical path (M2), a subdir-only
   doctype (M1), CRLF (A2), and the `--count`/`--list`/table modes + exit codes (0/1/2)?
2. **Advisory wiring** — the script exits `1` on leaks and `2` on error. Confirm check #10 in
   `SKILL.md` treats non-zero as advisory. If factory-health runs under `set -e` and invokes the
   helper directly, a single leak (exit 1) would abort the whole health check rather than warn.
   This flips "advisory by design" into "blocking by accident."

### A4 — ADVISORY (flake analysis, per team-lead #737 question) — unlikely cause; one residual risk to close
`#737` bats-full-suite (linux) single flake is **unlikely** to originate here. `mktemp -d`
allocates a unique dir under `$TMPDIR` (/tmp), outside the repo tree, so there is no cross-suite
path collision, and other suites' temp fixtures are untracked and outside any repo `git ls-files`
would scan. The script resolves `TEMPLATES` from the real plugin (`CLAUDE_PLUGIN_ROOT`/`_SELF_DIR`),
which is deterministic regardless of temp fixtures.

Residual risk (cannot fully rule out without the bats source): the script's `REPO_ROOT` falls back
to `git rev-parse --show-toplevel`. If a test does **not** set `VSDD_REPO_ROOT` and does not `cd`
into an isolated `git init` fixture, discovery walks up to the *real* vsdd-factory repo. That would
be a deterministic wrong-repo scan, not a race — but combined with nondeterministic CWD under
parallel bats `--jobs` it could present as an intermittent failure. Also verify the fixture uses
`git init` + **local** config (never `git config --global`) and a per-test `HOME`, so parallel
suites can't clobber shared git/global state.

**Suggestion:** confirm each bats case exports `VSDD_REPO_ROOT` to its temp fixture and uses local
git config + isolated HOME. If already done, #737 is not attributable to this PR.

---

## Checklist coverage
1. Diff coherence — OK, all changes serve #515.
2. Description accuracy — GREEN/0-leak claim verified accurate against develop.
3. Test coverage — cannot fully verify (A3); gaps flagged for M1/M2/M3/A2.
4. Demo evidence — only the GREEN/clean path shown; no RED (seeded-leak → exit 1 + table) demonstration. Recommend adding the error-path transcript.
5. Commit quality — `feat` scope correct (new feature, not a fix). Fine.
6. Diff size — 299 lines, under 500. Fine.
7. Missing changes — guard mechanism present as promised; M1/M2 are completeness gaps within it.
8. Dependency status — #524 (Fix 1) already merged; consistent with description.
