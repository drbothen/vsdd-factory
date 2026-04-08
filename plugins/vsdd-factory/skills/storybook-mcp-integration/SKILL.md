---
name: storybook-mcp-integration
description: >
  Integrates Storybook MCP (@storybook/addon-mcp) as the UI validation
  backbone. Provides 6 tools to factory agents for component documentation,
  preview, and self-healing test loops. Installed during toolchain preflight
  for UI products.
agents:
  primary: dx-engineer
  supporting: [devops-engineer, implementer, test-writer, e2e-tester, visual-reviewer]
inputs:
  - Product repo with UI framework (React, Vue, Svelte, Angular)
outputs:
  - Storybook installation + addon-mcp configured
  - MCP server at http://localhost:6006/mcp
condition: "feature_type in ['ui', 'full-stack']"
---

# Storybook MCP Integration

## Procedure

dx-engineer performs these steps:

1. **Install Storybook + addon-mcp:**
   ```bash
   npx storybook@latest init  # if not already installed
   npm install -D @storybook/addon-mcp
   ```

2. **Configure .storybook/main.ts:**
   ```typescript
   export default {
     addons: ['@storybook/addon-mcp'],
     features: {
       experimentalComponentsManifest: true,
       experimentalCodeExamples: true,
     },
   };
   ```

3. **Start Storybook dev server** (devops-engineer, at implementation / delta implementation start):
   ```bash
   npm run storybook  # MCP available at http://localhost:6006/mcp
   ```

4. **Register MCP server for factory agents:**
   ```json
   {
     "mcpServers": {
       "storybook": {
         "transport": "http",
         "url": "http://localhost:6006/mcp"
       }
     }
   }
   ```

## 6 MCP Tools Available to Agents

| Tool | What It Does | Which Agents Use It |
|------|-------------|-------------------|
| `list-all-documentation` | Lists all components + docs from manifests | implementer (reuse check) |
| `get-documentation` | Gets component details: props, types, examples | implementer (usage reference) |
| `get-documentation-for-story` | Gets docs for a specific story by ID | test-writer (behavior reference) |
| `get-storybook-story-instructions` | Returns conventions for this project's stories | test-writer (MUST call before writing stories) |
| `preview-stories` | Renders stories, returns preview URLs | implementer, visual-reviewer |
| `run-story-tests` | Runs component + a11y tests, returns results | test-writer, e2e-tester |

## Agent Access Pattern

Per DF-023 tier model and DF-027 MCP access:
- **T3 agents** (implementer, test-writer, e2e-tester): call Storybook MCP
  directly via `mcporter call` (T3 has Bash access)
- **T2 agents** (ux-designer, accessibility-auditor, visual-reviewer): delegate
  to  agent who calls Storybook MCP on their behalf
- **T1 agents** (orchestrator): call MCP tools directly

## Self-Healing Implementation Loop

```
implementer writes Component.tsx (normal file write)
  |
  v
test-writer calls get-storybook-story-instructions (Storybook MCP)
  -> learns THIS project's story conventions
  |
  v
test-writer writes Component.stories.tsx (normal file write)
  -> covers all variants x all states from component contract
  |
  v
implementer calls preview-stories (Storybook MCP)
  -> gets preview URLs, visually inspects each variant
  -> if visual issues -> fixes code -> re-previews
  |
  v
test-writer calls run-story-tests (Storybook MCP)
  -> runs component tests + a11y tests
  -> returns: pass/fail + failure details + a11y violations
  |
  +-- PASS -> proceed to PR
  |
  +-- FAIL ->
      implementer reads failure details
      implementer fixes component code
      test-writer calls run-story-tests again
      repeat until pass (max 10 iterations)
      |
      +-- Still failing after 10 -> escalate to human
```

## "Reuse First" Enforcement

Before the implementer creates ANY new component:
1. Call `list-all-documentation` -> get full component inventory
2. Search for existing component that matches the need
3. If match found -> use existing component (with design system tokens)
4. If no match -> create new component (justified in commit message)

## Storybook MCP Lifecycle

| Pipeline Point | Status |
|---------------|--------|
| During spec crystallization / delta analysis | Not running (no components yet) |
| During toolchain preflight | dx-engineer installs Storybook + addon-mcp |
| At implementation / delta implementation start | devops-engineer starts Storybook dev server |
| During implementation waves | Active -- agents use all 6 tools |
| At wave gates | Active -- run-story-tests for all components |
| During holdout evaluation | Active -- visual-reviewer uses preview-stories |
| During adversarial refinement / hardening | Active -- adversary can preview stories |
| During convergence | Active -- final test run |
| Release | Storybook built static (Chromatic or self-hosted) |
| Maintenance | Running -- drift detection uses list-all-documentation |

## Integration with Other Deliverables

| Deliverable | How Storybook MCP Enhances It |
|------------|------------------------------|
| D1 (Design System) | list-all-documentation exposes contracts to agents |
| D3 (Traceability) | list-all-documentation validates all components exist as stories |
| D4 (State Coverage) | run-story-tests validates all states render and pass |
| D7 (Responsive) | preview-stories renders at different viewports |
| D8 (Performance) | Stories enable isolated performance measurement |
| D9 (A11y-First) | run-story-tests with a11y: true catches violations |
| D10 (Fidelity) | list-all-documentation compared against UX spec |
| D11 (Component Testing) | run-story-tests IS the component testing mechanism |
| D12 (Drift Detection) | list-all-documentation compared against design system |
| D16 (Quality Gates) | run-story-tests results feed into gate checklist |

## Non-React Fallback

Storybook MCP component manifest currently only supports React projects.
For Vue/Svelte/Angular:
- `list-all-documentation` still works (reads stories, not manifests)
- `preview-stories` still works (renders any framework)
- `run-story-tests` still works (Vitest-based)
- Component manifest unavailable -- agents read source files directly

## DF-027 MCP Config Update

Add Storybook MCP to `.factory/mcp-config.yaml`:
```yaml
mcp_servers:
  storybook:
    type: "http"
    url: "http://localhost:6006/mcp"
    condition: "product_type in ['ui', 'full-stack']"
    lifecycle: "start with implementation, stop after release"
    managed_by: "devops-engineer (start/stop Storybook server)"
```

## Quality Gate

- [ ] Storybook addon-mcp installed and configured in .storybook/main.ts
- [ ] MCP server registered and reachable at http://localhost:6006/mcp
- [ ] All 6 MCP tools responding (list-all-documentation, get-documentation, etc.)
- [ ] Component tests passing via run-story-tests

## Failure Modes

- If Storybook not installed: skip UI component tests, note gap in gate report
- If MCP server unreachable after startup: retry once, then fall back to file-based component validation
- If non-React framework detected: proceed with reduced manifest support, note limitation
