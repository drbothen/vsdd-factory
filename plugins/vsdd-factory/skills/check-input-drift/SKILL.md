---
name: check-input-drift
description: Scan all .factory/ artifacts for input-hash drift. Recomputes hashes from current input files and reports which artifacts may be stale.
argument-hint: "[--fix]"
---

# Check Input Drift

Batch scan all artifacts in `.factory/` for input-hash drift — cases where an artifact's upstream inputs have changed since the artifact was produced.

## When to Use

- Before phase gates — verify all artifacts are current
- After a major spec edit — find which downstream artifacts need re-derivation
- During maintenance sweeps — identify stale artifacts
- After pipeline resume — check if inputs changed while the pipeline was paused

## How Input-Hash Works

Every VSDD artifact has `inputs:` (list of source files) and `input-hash:` (MD5 of those files' contents at production time) in its frontmatter.

```yaml
inputs: [domain-spec/L2-INDEX.md, prd.md]
input-hash: "be246a0"
```

If the current MD5 of `domain-spec/L2-INDEX.md + prd.md` differs from `be246a0`, the artifact was produced against older versions of its inputs and may contain stale assumptions.

## Procedure

### Step 1: Scan artifacts

Run one command — the binary handles all iteration internally:

```bash
${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash --scan .factory
```

This walks all `.factory/**/*.md` files with `input-hash:` frontmatter, skips INDEX files, recomputes each hash via the per-file `--check` path, and reports a summary line:

```
TOTAL=423 MATCH=400 STALE=3 UNCOMPUTED=15 NOINPUT=5 UPDATED=0 UPDATE_FAILED=0
```

Stale and uncomputed files are listed on stderr. Exit codes: 0 = clean, 2 = drift found.

**Do NOT iterate manually with inline bash loops.** Claude Code's harness auto-backgrounds multi-line shell commands and kills them before output flushes. The `--scan` flag moves all iteration into the binary.

The binary skips:
- Files without `input-hash:` frontmatter field
- INDEX-class files (INDEX.md, BC-INDEX.md, STORY-INDEX.md, etc.)

And tallies five categories from each file's `--check` result:
- **MATCH** — hash matches (exit 0, no warnings)
- **STALE** — hash mismatch (exit 2, drift detected)
- **UNCOMPUTED** — hash is null or `[md5]` placeholder (never computed)
- **NOINPUT** — input files don't exist or no inputs field
- **errors** — counted as NOINPUT defensively

### Step 2: Resolve all inputs (MANDATORY)

Before interpreting drift results, verify that all artifact inputs can be resolved. Files with missing inputs cannot be hashed or checked — the binary refuses to hash partial input sets because a partial hash produces false MATCH results.

**Always run resolve after scan:**

```bash
${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash --scan .factory --resolve
```

Output: `TOTAL=N RESOLVABLE=N UNRESOLVABLE=N`. Stderr lists each unresolvable file with the specific missing input names. Exit 0 if all resolvable, exit 1 if any missing.

**If UNRESOLVABLE > 0, diagnose before proceeding:**

1. For each unresolvable file, check the missing input name:
   ```bash
   ${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash <file> --resolve
   ```
   Reports "N MISSING — file1.md, file2.md" with the exact input names that can't be found.

2. Common causes of unresolvable inputs:
   - **Input not yet produced** — upstream artifact hasn't been created yet (normal early in pipeline)
   - **Input renamed** — upstream file was renamed but downstream `inputs:` frontmatter wasn't updated
   - **Input path wrong** — typo or incorrect relative path in `inputs:` field
   - **Input deleted** — upstream artifact was retired/removed but downstream still references it

3. For each cause:
   - **Not yet produced:** No action — NOINPUT is expected. The hash will be computed when the input exists.
   - **Renamed/wrong path:** Fix the `inputs:` frontmatter in the affected artifact to point to the correct file.
   - **Deleted:** Either remove the deleted input from the `inputs:` list, or create a replacement artifact.

**Do NOT skip this step.** A high NOINPUT count from Step 1 may mask real STALE files — if an input can't be found, the binary can't check whether the hash drifted, so the file silently drops out of drift detection.

### Step 3: Per-file check (individual artifacts)

To check a single artifact without scanning the full tree:

```bash
${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash <file> --check
```

This reads the `inputs:` field, concatenates the current contents of those files, computes the MD5, and compares against the stored `input-hash`. If any inputs are missing, the check is skipped (reported as PARTIAL on stderr).

### Step 4: Report (present to user)

```
## Input Drift Report

### Stale Artifacts (hash mismatch)

| Artifact | Stored Hash | Current Hash | Inputs Changed |
|----------|-------------|-------------|----------------|
| prd.md | be246a0 | f3a91c2 | domain-spec/L2-INDEX.md |
| S-1.05-foundational-types.md | a1b2c3d | e4f5g6h | prd.md |

### Uncomputed Hashes (never populated)

| Artifact | Status |
|----------|--------|
| BC-2.01.005.md | input-hash: null |
| S-3.05-cost-estimation.md | input-hash: "[md5]" |

### Current Artifacts (hash matches)

N artifacts verified current.
```

### Step 5: Fix (optional)

If `--fix` is passed, run with `--update` to batch-remediate all stale hashes:

```bash
${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash --scan .factory --update
```

This recomputes and writes the hash for all STALE files. Exit 0 if all updates succeeded, exit 1 if any failed. The summary line includes UPDATED and UPDATE_FAILED counts.

For a single file:

```bash
${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash <file> --update
```

Note: updating the hash does NOT re-derive the artifact's content — it just acknowledges the current input state. If the content needs updating, that's a separate task for the producing agent.

**Important:** Before running `--update` on >3 files, complete Step 6 (Triage cluster drift) to determine if content review by producing agents is required. Bulk `--update` without triage can mask semantic drift in spec artifacts.

### Step 6: Triage cluster drift before bulk --update

If the Step 1 report shows ANY of these patterns, treat as a content-review signal and **STOP before running --update**:

**Cluster patterns that signal upstream content change:**
- All shards of a sharded spec drift simultaneously (e.g., all of `.factory/specs/domain-spec/*.md`)
- All artifacts for a single subsystem drift (e.g., all `BC-2.14.*` files)
- More than 5 artifacts with the same upstream input drift together
- The PRD plus its supplements drift together
- An entire VP module (all VPs traceable to one architectural decision) drifts

**Single-file or scattered drift** (1-3 unrelated files): use Step 5 `--update` directly — these are typically incidental edits to upstream files that don't carry semantic change.

#### Triage procedure for cluster drift

1. **Identify the upstream change.** For each cluster, find the input file most likely to have changed:
   ```bash
   # See what changed in suspected upstream since artifact was produced
   git -C .factory log -p --since="<oldest-stale-artifact-date>" -- <upstream-input>
   ```

2. **Read the diff yourself first.** If the upstream change is purely cosmetic (whitespace, typo fix, formatting), proceed with `--update`. If it's semantic (new requirement, removed assumption, modified invariant), escalate.

3. **Dispatch the producing agent** for content review based on artifact type:

   | Stale Artifact Type | Producing Agent |
   |---|---|
   | L2 domain-spec shards | business-analyst |
   | PRD or PRD supplements | product-owner |
   | Behavioral contracts (BCs) | product-owner |
   | Architecture docs | architect |
   | Verification properties | architect |
   | Stories | story-writer |
   | Holdout scenarios | product-owner |

4. **Task template for the dispatched agent:**

   ```
   The following artifacts have stale input-hash, indicating their
   upstream inputs have changed:
     - <list of stale files>
   Upstream change is in: <input-file>
   Diff: <git diff output>

   Review each artifact against the new upstream state. For each:
   - If content is still semantically correct against new inputs:
     no edit needed (hash will be bumped after).
   - If content needs update: edit in place, bump artifact version,
     add changelog row.

   Do NOT touch input-hash frontmatter — that is bumped via
   compute-input-hash --update after your content review completes.
   ```

5. **After content review completes**, run `--update` to bump the hashes:
   ```bash
   ${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash --scan .factory --update
   ```

6. **Re-scan** to verify zero drift and confirm no cascading staleness was introduced by content edits:
   ```bash
   ${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash --scan .factory
   ```

#### When to skip Step 6

Step 6 is mandatory for cluster-drift patterns. Skip it ONLY if:
- The user has explicitly directed "bulk update, no investigation"
- The drift is in `cycles/`, `sidecar-learning.md`, or other non-spec bookkeeping files where content review adds no value
- All stale files are placeholder UNCOMPUTED (`input-hash: null`) being populated for the first time

---

## Common Cluster Drift Patterns (Reference)

| Pattern Observed | Likely Upstream Change | Dispatch |
|---|---|---|
| All `domain-spec/*.md` drift | Product brief or L2-INDEX edit | business-analyst |
| All `BC-X.YY.*` files for one subsystem | Subsystem definition edit | product-owner |
| PRD + PRD supplements drift together | PRD body or domain-spec edit | product-owner |
| All VPs in one verification family drift | Architecture decision changed | architect |
| All wave-N stories drift simultaneously | Wave manifest or epic edit | story-writer |
| Holdout scenarios drift | BC or AC edit | product-owner |
| `cycles/phase-*-*/*.md` drift | Cycle bookkeeping (safe to bump) | Use `--update` directly |

## Integration

- The `validate-input-hash.sh` PostToolUse hook provides per-file warnings at edit time
- This skill provides the batch scan for gate checks
- The `compute-input-hash` bin helper is the shared primitive both use

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/spec-drift-report-template.md` for the output report format.
