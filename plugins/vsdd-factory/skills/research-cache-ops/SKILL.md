---
name: research-cache-ops
description: Operate the research-cache for Perplexity/Context7 query results. Check, inspect, and clear cached research to avoid re-running expensive queries.
---

# Research Cache Operations

Wraps `${CLAUDE_PLUGIN_ROOT}/bin/research-cache` for common cache operations. Research is expensive (Perplexity deep-research is 30s+ and costs money); cached results should be reused across sessions unless explicitly invalidated.

## Operations

- **Stats**: `bin/research-cache stats` — entry count and cache size
- **Key for query**: `bin/research-cache key "<query text>"` — deterministic SHA-256 key
- **Check hit**: `bin/research-cache has <key>` — exit 0 if present
- **Fetch**: `bin/research-cache get <key>` — print cached JSON
- **Store**: `bin/research-cache put <key> < result.json` — write to cache
- **Clear**: `bin/research-cache clear` — wipe all entries

## When to use

- Before spawning `research-agent`: check if the query was already answered
- After a research run: store the result with the computed key so future sessions get a hit
- When invalidating stale research: clear specific keys or the whole cache
- When diagnosing research cost: inspect stats

## Integration

The `research-agent` subagent should call this via Bash before making Perplexity/Context7 calls. Pseudocode:

```bash
key=$(${CLAUDE_PLUGIN_ROOT}/bin/research-cache key "$query")
if ${CLAUDE_PLUGIN_ROOT}/bin/research-cache has "$key"; then
  ${CLAUDE_PLUGIN_ROOT}/bin/research-cache get "$key"
else
  # run the real query, then:
  echo "$result" | ${CLAUDE_PLUGIN_ROOT}/bin/research-cache put "$key"
fi
```
