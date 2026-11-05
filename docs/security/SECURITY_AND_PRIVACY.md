# Security and Privacy Principles

This document is an initial engineering baseline, not a completed security assessment.

## Trust boundaries

- The browser and mobile clients are untrusted.
- LLM output is untrusted and must never grant itself permissions.
- All tool calls pass through authenticated, authorized backend services.
- Signing keys and provider credentials remain server-side in protected secret storage.

## Sensitive actions

For consequential actions, the backend must classify the operation, authorize the requester, persist the exact proposed action, obtain approval from an authorized human, verify that the approved payload has not changed or expired, and execute it idempotently. The result must be auditable.

## Data handling

- Keep personal and business records off-chain.
- Avoid putting sensitive values in logs, telemetry, prompts, or blockchain metadata.
- Apply tenant isolation to all business-owned records.
- Define retention, export, and deletion behavior before public beta.
- Assess applicable privacy and financial regulations in each launch market.

## Stellar

Blockchain transaction success and AI task success are separate states. A database outbox and retry strategy should protect against partial failures between application persistence and network submission.
