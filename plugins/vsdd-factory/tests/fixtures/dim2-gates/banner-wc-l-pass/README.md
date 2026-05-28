# Fixture: banner-wc-l / PASS

**Scenario:** STATE.md banner accurately cites the file's line count and
the dual-margin arithmetic is correct.

**Banner format:** `> **Factory pipeline state — actual N lines** (500 - N = margin)`

**Expected exit code:** 0 (PASS)

**Verification:** `wc -l STATE.md` must equal the N in the banner, and
`500 - N` must equal the margin in the banner.

**Script invocation:**
```
banner-wc-l.sh STATE.md
```

Note: The STATE.md file in this fixture is crafted so that:
- `wc -l` returns exactly 21 lines
- The banner reads "actual 21 lines" and "500 - 21 = 479 margin"
