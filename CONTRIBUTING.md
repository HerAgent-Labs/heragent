# Contributing to HerAgent

Thank you for considering a contribution.

## Before you start

1. Read the README and relevant architecture documentation.
2. Search existing issues before opening a new one.
3. For substantial changes, open an issue or discussion first so the design can be reviewed.
4. Never include credentials, private customer data, production data, or real wallet secrets in issues, commits, tests, or screenshots.

## Development expectations

- Use TypeScript strict mode in TypeScript workspaces.
- Keep business rules in domain services, not UI components or LLM prompts.
- Treat model output as untrusted input and validate it at boundaries.
- Enforce authorization and tenant isolation on the server.
- Sensitive actions must use the approval workflow.
- Add tests for behavior, authorization, failure handling, and retries.
- Keep personal and business data off-chain.

## Pull requests

- Keep pull requests focused.
- Explain the problem, approach, and any trade-offs.
- Include tests and documentation where appropriate.
- Do not commit generated build output, local environment files, or secrets.
- Ensure CI checks pass before requesting review.

## Smart contracts

Contract changes require tests, documented authorization assumptions, replay/idempotency considerations, and a description of what data is written on-chain. Never put confidential business information or signing secrets in contract storage or transaction metadata.
