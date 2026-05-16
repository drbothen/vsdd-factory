# Fixture: banner-wc-l / FAIL

**Scenario:** STATE.md banner claims "actual 50 lines" but the file has far
fewer lines (23). The wc -l arithmetic doesn't match the banner.

**Injected defect:** Banner says `actual 50 lines` with margin `500 - 50 = 450`
but `wc -l` of the file returns 23.

**Expected exit code:** 1 (FAIL)
**Expected output:** cites expected (50) vs actual (`wc -l` result) and flags the
margin arithmetic mismatch.

**Script invocation:**
```
banner-wc-l.sh STATE.md
```
