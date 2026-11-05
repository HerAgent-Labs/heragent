# Security Policy

## Reporting a vulnerability

Please do not report security vulnerabilities through public GitHub issues.

Until a dedicated security contact is published, use GitHub's **Report a vulnerability** feature for this repository, if enabled. Include steps to reproduce, impact, affected versions, and any suggested mitigation. Avoid including real customer data or secrets.

## Security principles

- Enforce authorization in backend services; never rely on the AI model to decide permissions.
- Isolate tenant data at every query and service boundary.
- Store secrets in a secrets manager or protected environment variables.
- Never commit `.env` files, API keys, private keys, seed phrases, or wallet secrets.
- Require explicit approval for sensitive actions.
- Keep confidential business and personal data off-chain.
- Validate all model outputs and tool arguments.
- Use idempotency controls for financial and blockchain operations.
