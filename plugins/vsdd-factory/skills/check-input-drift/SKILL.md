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

Find all `.factory/**/*.md` files with both `inputs:` and `input-hash:` frontmatter fields.

Skip:
- Files without `inputs:` field (no upstream dependencies to track)
- Files with `input-hash: null` or `input-hash: "[md5]"` (hash never computed — report separately)
- INDEX files (auto-generated, no input-hash)

### Step 2: Recompute hashes

For each artifact, run:
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

If `--fix` is passed:
1. For each stale artifact, run `compute-input-hash <file> --update` to recompute the hash
2. Report which hashes were updated
3. Note: updating the hash does NOT re-derive the artifact's content — it just acknowledges the current input state. If the content needs updating, that's a separate task for the producing agent.

## Integration

- The `validate-input-hash.sh` PostToolUse hook provides per-file warnings at edit time
- This skill provides the batch scan for gate checks
- The `compute-input-hash` bin helper is the shared primitive both use

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/spec-drift-report-template.md` for the output report format.
