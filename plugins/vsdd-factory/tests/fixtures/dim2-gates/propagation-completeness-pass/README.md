# Fixture: propagation-completeness / PASS

**Scenario:** The derived value "D-453" is verified at all prescribed sites
using per-site grep patterns (Option A — AC-005 production-grade default).

**Per-site pattern design (sites.txt format: `<file-path>:<grep-pattern>`):**

Each site in `sites.txt` carries a distinct grep pattern that encodes the
site-specific semantic anchor for the derived value, not just a literal
occurrence anywhere in the file:

- `.factory/STATE.md:Decisions Log.*D-453` — verifies D-453 appears in the
  Decisions Log section specifically (not an incidental mention elsewhere).
- `.factory/INDEX.md:D-453 canonical` — verifies the INDEX entry uses the
  "canonical" descriptor that confirms the reference is intentional.

Both patterns match in the PASS fixture files.

**What this fixture tests (Option A semantics):**
The script uses per-site patterns (from the `:` second column of sites.txt)
via `grep -cE`, NOT the global derived-value string. This is the correct
behavior per AC-005: `<file-path>:<grep-pattern>` where the pattern is
site-specific. A fixture where all patterns are identical to the derived
value would not exercise the per-site contract.

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
propagation-completeness.sh "D-453" sites.txt
```
(called from the fixture directory so relative paths in sites.txt resolve correctly)

**Implementation note (F-002 fix in S-15.08 fix-burst-1):**
The original implementation used `grep -cF "${DERIVED_VALUE}"` (global derived
value) ignoring the per-site pattern column. fix-burst-1 corrected this to
`grep -cE "${SITE_PATTERN}"` so the second column is actually executed. These
fixtures use different per-site patterns to lock in the per-site contract.
