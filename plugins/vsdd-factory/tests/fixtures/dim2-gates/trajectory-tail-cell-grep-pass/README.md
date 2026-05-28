# Fixture: trajectory-tail-cell-grep / PASS

**Scenario:** STATE.md contains the trajectory tail value `→9→9→9→9` at all
prescribed sites listed in `sites.txt`.

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
trajectory-tail-cell-grep.sh <factory-root> "→9→9→9→9" sites.txt
```

The `sites.txt` file lists `<file-path>:<anchor-pattern>` pairs (one per line)
describing exactly where the trajectory tail must appear in the factory root.
The PASS fixture has the tail value present at every listed site.
