# Fixture: freshness-literal-stdout / PASS

**Scenario:** The script is invoked with a command that exits 0.

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
freshness-literal-stdout.sh "echo hello"
```

The script re-runs the provided command, captures its exit code and stdout,
and prints them verbatim per D-449(a). Since `echo hello` exits 0, the gate
exits 0.

No fixture files are needed — the "command" is a shell built-in. The test
passes the command as a positional argument.
