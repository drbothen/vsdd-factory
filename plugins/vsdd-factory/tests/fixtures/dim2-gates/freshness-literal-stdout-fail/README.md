# Fixture: freshness-literal-stdout / FAIL

**Scenario:** The script is invoked with a command that exits non-zero.

**Injected defect:** The command `false` always exits 1, simulating a
freshness check that finds the state has changed (stale/failed).

**Expected exit code:** 1 (FAIL)
**Expected output:** prints the command, its non-zero exit code, and raw stdout.

**Script invocation:**
```
freshness-literal-stdout.sh "false"
```

No fixture files are needed — the "command" is a shell built-in.
