---
document_type: state
version: "1.0"
title: "Factory State (fixture: expired lock)"
factory_lock:
  holder: "old@x.com"
  locked_at: "2020-01-01T00:00:00Z"
  expires_at: "2020-01-01T00:45:00Z"
---

# Factory State — Fixture: Expired Lock

This fixture has a factory_lock whose expires_at is in the past.
Used to demonstrate: Factory lock: FREE (expired lock treated as free)
