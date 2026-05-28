# Fixture: active-branches-sha-currency / PASS

**Scenario:** STATE.md Active Branches table SHAs match the SHAs returned
by `git rev-parse origin/<branch>`.

**Env-var override pattern (per story-writer note 1):**
The script supports `GIT_TEST_SHA_OVERRIDE_<branch>=<sha>` env vars to avoid
requiring a real git remote during testing. The bats test sets:

```
GIT_TEST_SHA_OVERRIDE_main=666d689f1234567890abcdef1234567890abcdef
GIT_TEST_SHA_OVERRIDE_develop=224fa18421214b30dacf1cdd606152294cd33bd6
GIT_TEST_SHA_OVERRIDE_factory_artifacts=0b4972a6abcdef1234567890abcdef1234567890
```

(Branch names with hyphens use underscores in the env var name:
`factory-artifacts` -> `GIT_TEST_SHA_OVERRIDE_factory_artifacts`)

The SHAs in STATE.md exactly match the env-var-supplied values.

**Expected exit code:** 0 (PASS)

**Script invocation:**
```
GIT_TEST_SHA_OVERRIDE_main=666d689f1234567890abcdef1234567890abcdef \
GIT_TEST_SHA_OVERRIDE_develop=224fa18421214b30dacf1cdd606152294cd33bd6 \
GIT_TEST_SHA_OVERRIDE_factory_artifacts=0b4972a6abcdef1234567890abcdef1234567890 \
active-branches-sha-currency.sh <factory-root> STATE.md
```
