---
document_type: dtu-clone-spec
service_name: "[Service Name]"
fidelity_level: L1 | L2 | L3 | L4
api_spec_url: "[OpenAPI/Swagger spec URL, if available]"
api_docs_url: "[Human-readable API documentation URL]"
created: YYYY-MM-DD
---

# DTU Clone Specification: [Service Name]

## Service Identity

| Property | Value |
|----------|-------|
| **Service** | [e.g., Okta, Stripe, Slack] |
| **Purpose in SUT** | [What the SUT uses this service for] |
| **Integration type** | [REST API / GraphQL / WebSocket / Webhook] |
| **Authentication** | [API key / OAuth2 / Bearer token / None] |
| **Fidelity level** | [L1 / L2 / L3 / L4] |

## Endpoints Used by SUT

| Method | Path | Purpose | Request Body | Response | Used In |
|--------|------|---------|-------------|----------|---------|
| GET | /api/v1/users | Fetch user list | — | User[] | auth-service.rs:42 |
| POST | /api/v1/users | Create user | CreateUserReq | User | signup-flow.rs:88 |
| DELETE | /api/v1/users/{id} | Deactivate user | — | 204 | admin-panel.ts:120 |

## State Model (L2+)

| Entity | Fields | Relationships | Initial State |
|--------|--------|---------------|---------------|
| User | id, email, status, created_at | has_many: Sessions | 3 seed users |
| Session | id, user_id, expires_at, active | belongs_to: User | 0 sessions |

## Error Responses (L3+)

| Condition | Status | Response Body | Trigger |
|-----------|--------|--------------|---------|
| Invalid API key | 401 | `{"error": "Unauthorized"}` | Any request with wrong API key |
| Rate limit exceeded | 429 | `{"error": "Rate limit exceeded", "retry_after": 60}` | >100 requests/minute |
| User not found | 404 | `{"error": "Not found"}` | GET /users/{non-existent-id} |
| Validation error | 400 | `{"error": "Invalid email format"}` | POST /users with invalid email |

## Behavioral Sequences (L3+)

### OAuth2 Flow
1. Client requests authorization -> 302 redirect to login page
2. User submits credentials -> 302 redirect with auth code
3. Client exchanges code for tokens -> 200 with access_token + refresh_token
4. Access token expires after 3600s -> 401 on next request
5. Client refreshes token -> 200 with new access_token

### Webhook Delivery
1. When a user is created, POST to configured webhook URL within 5s
2. If webhook delivery fails (non-2xx response), retry 3 times with exponential backoff
3. After 3 failures, mark webhook as "failing" and stop retries

## Failure Injection (L4)

| Failure Mode | Configuration | Description |
|-------------|--------------|-------------|
| Latency spike | `DTU_LATENCY_MS=5000` | Add 5s latency to all responses |
| Intermittent errors | `DTU_ERROR_RATE=0.1` | 10% of requests return 500 |
| Corrupt response | `DTU_CORRUPT=true` | Occasionally return malformed JSON |
| Service down | `DTU_DOWN=true` | Return connection refused |

## Deterministic Mode

When `DTU_DETERMINISTIC=true`:
- Same request always produces same response (no random IDs, timestamps use fixed epoch)
- State is reset between test runs (clean slate)
- Suitable for CI and holdout evaluation where reproducibility matters

## Clone Validation

- [ ] All endpoints listed above return correct status codes and response shapes
- [ ] State model persists across requests (L2+)
- [ ] Error responses match the specification (L3+)
- [ ] OAuth flow completes end-to-end (L3+, if applicable)
- [ ] Webhook delivery works (L3+, if applicable)
- [ ] Failure injection modes work as configured (L4)
- [ ] Deterministic mode produces identical responses across runs
