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

### Step 2: Per-file check (individual artifacts)

To check a single artifact without scanning the full tree:

```bash
${CLAUDE_PLUGIN_ROOT}/bin/compute-input-hash <file> --check
```

This reads the `inputs:` field, concatenates the current contents of those files, computes the MD5, and compares against the stored `input-hash`.

### Step 3: Report

```
## Input Drift Report

### Stale Artifacts (hash mismatch)

| Artifact | Stored Hash | Current Hash | Inputs Changed |
|----------|-------------|-------------|----------------|
| prd.md | be246a0 | f3a91c2 | domain-spec/L2-INDEX.md |
| STORY-005.md | a1b2c3d | e4f5g6h | prd.md |

### Uncomputed Hashes (never populated)

| Artifact | Status |
|----------|--------|
| BC-2.01.005.md | input-hash: null |
| STORY-042.md | input-hash: "[md5]" |

### Current Artifacts (hash matches)

N artifacts verified current.
```

### Step 4: Fix (optional)

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

## Integration

- The `validate-input-hash.sh` PostToolUse hook provides per-file warnings at edit time
- This skill provides the batch scan for gate checks
- The `compute-input-hash` bin helper is the shared primitive both use

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/spec-drift-report-template.md` for the output report format.
