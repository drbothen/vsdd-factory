# Fixture: active-branches-sha-currency / FAIL

**Scenario:** STATE.md Active Branches table has a stale SHA for `develop`.

**Injected defect:** The `develop` row shows SHA `deadbeefdeadbeefdeadbeefdeadbeefdeadbeef`
but `GIT_TEST_SHA_OVERRIDE_develop` returns `224fa18421214b30dacf1cdd606152294cd33bd6`.

**Expected exit code:** 1 (FAIL)
**Expected output:** per-branch report showing develop SHA mismatch with expected vs actual.

**Env-var overrides for the test:**
```
GIT_TEST_SHA_OVERRIDE_main=666d689f1234567890abcdef1234567890abcdef
GIT_TEST_SHA_OVERRIDE_develop=224fa18421214b30dacf1cdd606152294cd33bd6
GIT_TEST_SHA_OVERRIDE_factory_artifacts=0b4972a6abcdef1234567890abcdef1234567890
```

The develop row in STATE.md uses `deadbeefdeadbeefdeadbeefdeadbeefdeadbeef` — which
does NOT match the env-var-supplied current SHA of `224fa18421214b30dacf1cdd606152294cd33bd6`.
