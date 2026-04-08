# Step 5: Regression Risk Assessment

Assess regression risk per affected module using the impact boundary and artifact mapping.

## Inputs

- Component impact map from Step 3
- Affected artifact mapping from Step 4

## Actions

1. For each affected module, assess regression risk:

   | Risk Level | Criteria |
   |------------|----------|
   | HIGH | Core module being modified, many dependents, security-critical |
   | MEDIUM | Non-core module being modified, some dependents |
   | LOW | New module (no existing code to break), no dependents |

2. Consider factors:
   - Number of dependent files
   - Whether the module is on a critical path (auth, data, core logic)
   - Whether interface changes propagate to consumers
   - Historical stability of the module

## Outputs

- Risk assessment table: module name, change type, risk level, rationale

## Completion Criteria

- Every affected module has a risk level assigned
- Risk rationale is documented (not just the level)
- HIGH risk modules are flagged for extra attention in later phases
