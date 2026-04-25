#!/usr/bin/env python3
"""
check-platforms-drift.py — fail CI if ci/platforms.yaml drifts from the
inline `build-dispatcher` matrix in .github/workflows/ci.yml.

Single source of truth for the v1.0 cross-platform target matrix is
`ci/platforms.yaml` (S-2.3). GitHub Actions cannot read external YAML
into matrix: expressions, so ci.yml duplicates the same 5 tuples inline.
This script parses both files and asserts the (platform, os, target,
use_cross, run_tests) tuples match exactly, in the same order.

Exit codes:
    0   in sync
    1   drift detected — diff printed to stderr
    2   parse error — usually a malformed YAML edit
"""

from __future__ import annotations

import sys
from pathlib import Path

import yaml

REPO_ROOT = Path(__file__).resolve().parent.parent
PLATFORMS_YAML = REPO_ROOT / "ci" / "platforms.yaml"
CI_WORKFLOW = REPO_ROOT / ".github" / "workflows" / "ci.yml"


def load_canonical() -> list[dict]:
    """Read ci/platforms.yaml and return the normalized tuple list."""
    with PLATFORMS_YAML.open() as f:
        doc = yaml.safe_load(f)
    rows = []
    for entry in doc["platforms"]:
        rows.append(
            {
                "platform": entry["platform"],
                "os": entry["os"],
                "target": entry["target"],
                "use_cross": bool(entry.get("use_cross", False)),
                "run_tests": bool(entry.get("run_tests", True)),
            }
        )
    return rows


def load_workflow_matrix() -> list[dict]:
    """Read the build-dispatcher matrix from ci.yml."""
    with CI_WORKFLOW.open() as f:
        doc = yaml.safe_load(f)
    job = doc["jobs"]["build-dispatcher"]
    include = job["strategy"]["matrix"]["include"]
    rows = []
    for entry in include:
        rows.append(
            {
                "platform": entry["platform"],
                "os": entry["os"],
                "target": entry["target"],
                "use_cross": bool(entry.get("use_cross", False)),
                "run_tests": bool(entry.get("run_tests", True)),
            }
        )
    return rows


def diff(canonical: list[dict], workflow: list[dict]) -> list[str]:
    """Produce a human-readable diff. Empty list = in sync."""
    issues: list[str] = []
    if len(canonical) != len(workflow):
        issues.append(
            f"length mismatch: ci/platforms.yaml has {len(canonical)} entries, "
            f"ci.yml build-dispatcher matrix has {len(workflow)}"
        )
    canonical_by_platform = {row["platform"]: row for row in canonical}
    workflow_by_platform = {row["platform"]: row for row in workflow}
    only_canonical = set(canonical_by_platform) - set(workflow_by_platform)
    only_workflow = set(workflow_by_platform) - set(canonical_by_platform)
    if only_canonical:
        issues.append(
            "platforms in ci/platforms.yaml but missing from ci.yml: "
            + ", ".join(sorted(only_canonical))
        )
    if only_workflow:
        issues.append(
            "platforms in ci.yml but missing from ci/platforms.yaml: "
            + ", ".join(sorted(only_workflow))
        )
    for platform in sorted(set(canonical_by_platform) & set(workflow_by_platform)):
        c = canonical_by_platform[platform]
        w = workflow_by_platform[platform]
        for key in ("os", "target", "use_cross", "run_tests"):
            if c[key] != w[key]:
                issues.append(
                    f"  {platform}.{key}: canonical={c[key]!r} workflow={w[key]!r}"
                )
    return issues


def main() -> int:
    try:
        canonical = load_canonical()
        workflow = load_workflow_matrix()
    except (yaml.YAMLError, KeyError) as exc:
        print(f"::error::failed to parse platform definitions: {exc}", file=sys.stderr)
        return 2

    issues = diff(canonical, workflow)
    if issues:
        print(
            "::error::ci/platforms.yaml has drifted from "
            ".github/workflows/ci.yml build-dispatcher matrix",
            file=sys.stderr,
        )
        for line in issues:
            print(line, file=sys.stderr)
        print(
            "\nFix: edit both files so the (platform, os, target, "
            "use_cross, run_tests) tuples agree.",
            file=sys.stderr,
        )
        return 1

    print(f"ci/platforms.yaml in sync with ci.yml ({len(canonical)} platforms ok)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
