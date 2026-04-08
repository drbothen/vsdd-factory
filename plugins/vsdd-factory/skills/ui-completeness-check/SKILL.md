---
name: ui-completeness-check
description: >
  Validates UI completeness via traceability matrix. Tracks every UI element
  from UX spec through story, component, state, test, and visual evidence.
  Detects gaps and produces a completeness report. Zero gaps required before
  convergence.
agents:
  primary: consistency-validator
  supporting: [ux-designer, e2e-tester]
inputs:
  - .factory/design-system/ (component contracts, state requirements)
  - .factory/specs/ux-spec/UX-INDEX.md (or .factory/feature/ux-delta.md)
  - Story files (phase-2-stories/ or feature stories)
  - Implemented source code
  - Test files
  - .factory/ui-evidence/ (screenshots)
outputs:
  - .factory/ui-traceability.yaml
  - .factory/ui-quality/completeness-report.md
gate: Zero gaps required before convergence
condition: "feature_type in ['ui', 'full-stack']"
---

# UI Completeness Check

## When This Skill Runs

| Pipeline Point | Purpose | Strictness |
|---------------|---------|-----------|
| After story creation (given approved stories) | Every UX spec screen has stories | Screens and interactions covered |
| After each wave gate (during implementation) | Built components match stories | States + tests exist |
| Before convergence (given all tests passing) | Full traceability matrix | Zero gaps |

## Traceability Matrix Structure

The skill builds and maintains `.factory/ui-traceability.yaml`:

```yaml
screens:
  - id: SCR-NNN
    name: "Screen Name"
    ux_spec_ref: "ux-spec.md#section"
    story_ids: ["STORY-NNN"]
    status: specified | story-created | implemented | tested | verified

    components:
      - id: CMP-NNN
        name: "ComponentName"
        component_contract: "design-system/components/contracts/name.yaml"
        story_id: "STORY-NNN"
        states:
          - name: state_name
            implemented: true|false
            tested: true|false
            screenshot: "path or null"

    interactions:
      - id: INT-NNN
        description: "User action description"
        ux_spec_ref: "section reference"
        e2e_test: "test file path or null"
        tested: true|false

    responsive:
      - breakpoint: 375|768|1024|1440
        tested: true|false
        screenshot: "path or null"

    accessibility:
      axe_clean: true|false
      keyboard_navigable: true|false
      screen_reader_tested: true|false
      touch_targets_valid: true|false

    performance:
      lcp_ms: number
      cls: number
      fid_ms: number
      meets_targets: true|false
```

## Gap Detection

For each element in the traceability matrix, check:

1. **Screen gaps:** Screens in UX spec without stories
2. **Component gaps:** Components in stories without implementation
3. **State gaps:** Required states (from component contract) not implemented
4. **Interaction gaps:** UX spec interactions without e2e tests
5. **Responsive gaps:** Breakpoints without test coverage or screenshots
6. **Accessibility gaps:** Failed axe-core, keyboard nav, screen reader checks
7. **Performance gaps:** Metrics exceeding targets

## Completeness Report

Output to `.factory/ui-quality/completeness-report.md`:

```markdown
## UI Completeness Report

### Summary
- Screens: N specified, N implemented, N fully verified
- Fidelity: N%
- Gaps: N found

### Screen-by-Screen Status
| Screen | Status | Components | States | Interactions | Responsive | A11y | Perf |
|--------|--------|-----------|--------|-------------|-----------|------|------|
| ...    | ...    | N/N       | N/N    | N/N         | N/4       | P/F  | P/F  |

### Gaps Found
1. [SCR-NNN/CMP-NNN]: description of gap
2. ...

### Resolution Required Before Convergence
All gaps must be resolved. Each gap becomes a fix story (FIX-UI-NNN).
```

## Fix Story Generation

For each gap found:
- Create a fix story: FIX-UI-NNN
- Story includes: which screen, component, state, or interaction is missing
- Route through code-delivery.lobster for implementation
- Re-run completeness check after fixes

## State Coverage Enforcement (D4)

Every component that fetches data MUST have 4 async states:
1. **LOADING:** skeleton screen or spinner (not blank)
2. **SUCCESS:** populated with data
3. **EMPTY:** "No items to display" with actionable CTA
4. **ERROR:** error message + retry button

Required states by component type (from contracts):
| Component Type | Required States |
|---------------|----------------|
| Button | default, hover, active, focus, disabled, loading |
| Form Field | default, focus, filled, error, disabled, readonly |
| Modal | closed, opening, open, closing |
| List | default, loading, empty, error, paginating |
| Card | default, hover, selected, loading, skeleton |
| Navigation | default, active-item, collapsed, expanded |
| Toast/Alert | info, success, warning, error, dismissing |
| Data Table | default, loading, empty, error, sorting, filtering |
| Dropdown | closed, open, searching, no-results, loading |
| Tabs | default, active-tab, disabled-tab |

## Design-to-Code Fidelity Score (D10)

```
Fidelity = (implemented elements / specified elements) x 100

Target: 100% before convergence
```

Every specified element must be implemented, tested, and evidenced.

## Quality Gate

- [ ] Traceability matrix (ui-traceability.yaml) covers every screen in the UX spec
- [ ] Zero gaps in UX spec -> story -> component -> test -> evidence chain
- [ ] Completeness report produced with per-screen status table
- [ ] All data-fetching components have 4 async states (loading, success, empty, error)

## Failure Modes

- If UX spec references screens not yet in ui-traceability.yaml: create placeholder entries and flag as gaps
- If component contracts are missing for discovered components: flag and route to ux-designer for contract creation
- If screenshots cannot be captured (app not running): mark visual evidence as pending and proceed with code-level checks
