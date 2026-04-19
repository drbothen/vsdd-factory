---
name: step-0-source-acquisition
description: Clone or copy the target codebase into .reference/ and update the reference manifest.
---

# Step 0: Source Acquisition

> **Shared context:** Read `./_shared-context.md` before executing this step.

Ensure the target codebase is available in `.reference/<project>/` before any analysis begins.

## Procedure

1. **If input is a Git URL:** Clone to `.reference/<project>/` with `--depth=1`:
   ```bash
   git clone --depth=1 <url> .reference/<project>
   ```
2. **If input is a local path outside `.reference/`:** Copy to `.reference/<project>/`.
3. **If input is already in `.reference/`:** No action needed.

4. **Update `.factory/reference-manifest.yaml`** following `${CLAUDE_PLUGIN_ROOT}/templates/reference-manifest-template.yaml`. Add the new entry under `repos:` with url, commit SHA, ingested date, depth, focus, and status.

5. **Create the output directory:**
   ```bash
   mkdir -p .factory/semport/<project>/
   ```

## Artifacts

- `.reference/<project>/` — cloned/copied codebase (gitignored)
- `.factory/reference-manifest.yaml` — updated with new repo entry

## Success Criteria

- `.reference/<project>/` exists and contains source files
- `reference-manifest.yaml` has an entry for this project with status: `ingesting`
