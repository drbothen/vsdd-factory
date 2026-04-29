# S-5.06 Demo Evidence — Overview

**Story:** S-5.06 — Semver commitment documentation
**Wave:** 16 (ship wave); convergence burst Wave 14
**Version:** v1.7 (CONVERGENCE_REACHED at pass-5)
**Branch:** feat/s-5.06-semver-commitment
**HEAD at evidence capture:** 11ed18a

## What was built

A new public document `docs/guide/semver-commitment.md` was written to state what
vsdd-factory commits to as stable public API in the v1.0 line, what is intentionally
unstable, the breaking change policy, and what `HOST_ABI_VERSION = 1` guarantees
for plugin authors.

Two existing files were modified to cross-link to the new document:
- `docs/guide/v1.0-index.md` — row added under the "For operators" table
- `README.md` — row added to "v1.0 Factory Plugin Kit" section; L261 updated
  from "links the four below" to "links the five below"

## AC trace map

| File | AC covered |
|------|-----------|
| `01-ac-1-file-exists.md` | AC-1: file created with full content |
| `02-ac-2-stable-surface.md` | AC-2: "What's stable" section with enumerated surfaces |
| `03-ac-3-unstable-surface.md` | AC-3: "What's NOT stable" section with enumerated surfaces |
| `04-ac-4-breaking-change-policy.md` | AC-4: "Breaking change policy" section — major bump + migration guide |
| `05-ac-5-plugin-compat-policy.md` | AC-5: "Plugin backward compat policy" section — HOST_ABI_VERSION = 1 |
| `06-ac-6-cross-links.md` | AC-6: cross-links in v1.0-index.md and README.md |
| `07-bats-green.md` | All 18 bats tests GREEN (full output) |

## Verification summary

- `docs/guide/semver-commitment.md`: 204 lines, real prose content
- All 6 ACs satisfied per bats test suite: 18/18 PASS
- POLICY 10 (demo_evidence_story_scoped) complied: all files under `docs/demo-evidence/S-5.06/`
