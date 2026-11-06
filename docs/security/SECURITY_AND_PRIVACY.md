# HerAgent Security and Privacy

**Status:** Initial security baseline  
**Applies to:** HerAgent applications, APIs, workers, AI agents, shared packages, infrastructure, and Stellar/Soroban integrations  
**Last updated:** November 6, 2026

## 1. Purpose and scope

HerAgent is an open-source, AI-powered business assistant designed to help entrepreneurs manage business workflows through specialized AI agents.

The platform is expected to process business records, customer information, financial data, and user instructions. Some workflows may invoke external services or produce consequential actions. These capabilities create security and privacy risks that must be addressed throughout development.

This document defines the security principles and engineering requirements that guide HerAgent's design and implementation.

**Project status:** This document is a security baseline, not evidence of completed implementation, certification, penetration testing, or an independent security audit. Controls must be implemented and verified before the features that depend on them are released.

## 2. Security principles

HerAgent follows these principles:

- **Least privilege:** Every user, service, worker, and agent receives only the permissions needed for its task.
- **Secure by default:** Access is denied unless explicitly authorized.
- **Human oversight:** Sensitive and consequential actions require approval by an appropriately authorized person.
- **Defense in depth:** Security must not depend on a single model, validation step, or infrastructure control.
- **Privacy by design:** Collect and retain only the information needed to provide the service.
- **Verifiable execution:** Preserve trustworthy execution records while minimizing sensitive information published on-chain.
- **Accountability:** Security-relevant operations should produce auditable records without unnecessarily exposing private data.
- **Transparency:** Security limitations, known issues, and implementation status must be communicated accurately.

## 3. Trust boundaries and threat model

HerAgent must treat the following as distinct trust boundaries:

| Boundary | Security requirement |
|---|---|
| Web and mobile clients | Treat all client-supplied data and identity claims as untrusted until validated server-side. |
| API and domain services | Enforce authentication, authorization, business rules, and workspace isolation. |
| AI models and agent orchestration | Treat model outputs and plans as untrusted proposals, not authorization decisions. |
| Tools and external integrations | Validate arguments, restrict capabilities, and handle returned content as untrusted input. |
| Background workers and queues | Authenticate job producers where applicable, validate payloads, and prevent unauthorized or duplicate execution. |
| Database and object storage | Restrict access and isolate data according to the authenticated business workspace. |
| AI and third-party providers | Minimize transmitted data and review provider retention, training, access, and processing terms. |
| Stellar and Soroban | Publish only deliberately selected data; validate transactions, contract authorization, and verification results. |

The threat model includes account compromise, broken access control, cross-tenant data leakage, prompt injection, tool misuse, credential exposure, replayed operations, malicious dependencies, denial of service, and failures between database persistence and external transaction submission.

Security controls must evolve as the system gains new capabilities and integrations.

## 4. Authentication, authorization, and workspace isolation

- Authenticate requests before granting access to protected resources.
- Enforce authorization on the server for every protected operation, including object-level access.
- Derive the active business workspace from a verified identity and authorized membership, not from an untrusted request parameter alone.
- Ensure users cannot read, modify, enumerate, or export another workspace's records without explicit authorization.
- Apply least-privilege access to internal services and administrative operations.
- Protect session tokens and credentials; use appropriate expiration, revocation, and secure storage mechanisms.
- Apply rate limits and abuse controls to authentication and other sensitive endpoints.
- Test authorization failures as well as successful access paths.

Database-level protections, including PostgreSQL row-level security, may be used as an additional defense where appropriate. They do not replace application-level authorization.

## 5. AI agent and tool security

AI models may generate incorrect, misleading, or maliciously influenced outputs. HerAgent must not rely on model behavior alone to enforce security.

### 5.1 Tool execution

- Expose only explicitly registered, typed, and validated tools.
- Validate arguments against server-defined schemas before executing a tool.
- Enforce authorization and business rules within the application service that performs the operation.
- Do not provide models with unrestricted database access, shell execution, arbitrary code execution, or unrestricted network access.
- Apply execution timeouts, bounded retries, token and cost limits, and tool-call limits.
- Restrict each agent's tools to the capabilities required for its purpose.
- Validate and constrain data passed between agents.

### 5.2 Prompt injection and untrusted content

User prompts, documents, emails, retrieved records, websites, tool descriptions, and external API responses may contain instructions intended to manipulate an agent.

- Treat external content as data, not as trusted system instructions.
- Do not allow content retrieved by an agent to override authorization rules or security policies.
- Validate proposed actions independently of the model's explanation or confidence.
- Restrict access to secrets and sensitive records even when a prompt requests them.
- Test direct and indirect prompt-injection scenarios before enabling consequential tools.

Prompt filtering alone is not a sufficient security boundary.

### 5.3 Human approval

Sensitive or consequential operations must pass through a server-enforced approval workflow.

Examples include payments, withdrawals, bulk changes, external communications with material consequences, and changes to access permissions.

The approval record must bind to the exact proposed action, including relevant parameters and the target resource. The backend must verify:

1. The requester is authorized to propose the action.
2. The approver is authorized to approve it.
3. The approved action has not changed or expired.
4. The action has not already been executed.
5. The action remains valid under current policy at execution time.

Approval must be recorded before execution. A model cannot approve its own proposed action or modify the approved payload.

## 6. Sensitive data and privacy

HerAgent should collect and process only the information necessary for the requested functionality.

### 6.1 Data minimization

- Avoid collecting unnecessary personal, customer, financial, or business information.
- Send only the minimum required context to external AI providers and integrations.
- Avoid placing secrets, authentication tokens, payment credentials, or unnecessary personal data in prompts.
- Separate business data by workspace and enforce access restrictions throughout retrieval, processing, and output generation.
- Document the purposes and retention requirements for stored data before public beta.

### 6.2 Logging and telemetry

- Never intentionally log passwords, private keys, access tokens, or secret environment variables.
- Redact or omit sensitive customer and financial information from logs where it is not essential.
- Use execution identifiers and structured event metadata to support troubleshooting without duplicating entire private records.
- Restrict access to logs and define appropriate retention periods.
- Ensure error responses do not expose stack traces, internal secrets, or sensitive database details to clients.

### 6.3 Data retention, export, and deletion

Before public beta, define and document how users can access, export, correct, and delete eligible business data.

Retention and deletion behavior must account for legal obligations, security audit requirements, backup lifecycles, and third-party processing arrangements.

A deletion request must not be represented as complete until the applicable deletion workflow and its limitations have been verified.

### 6.4 External AI providers

Before enabling an AI provider for real business data, assess its data retention, model-training, access-control, regional processing, and security terms.

Provider integrations should support configuration and replacement without exposing provider credentials to clients or models.

Do not claim that third-party processing is private, zero-retention, or excluded from model training unless the applicable provider terms and configuration support that claim.

## 7. Secrets and cryptographic material

- Keep secrets out of source control, client bundles, logs, documentation examples, and committed configuration files.
- Provide a safe `.env.example` containing placeholder values only.
- Use environment-specific secret management for deployed environments.
- Apply least-privilege permissions to secret access.
- Rotate compromised credentials promptly and revoke access that is no longer needed.
- Never expose Stellar signing secrets or other private keys to web clients, mobile clients, or AI models.
- Use established cryptographic libraries and protocols instead of custom cryptography.
- Avoid storing production credentials in development fixtures or test datasets.

If a secret is committed accidentally, removing it from the latest commit is not sufficient. The credential must be treated as compromised, revoked or rotated, and removed from repository history where appropriate.

## 8. API, application, and dependency security

- Validate external input at system boundaries.
- Use parameterized database operations and safe ORM patterns.
- Apply appropriate CORS policies, security headers, request-size limits, and rate limits.
- Use TLS for network communication in deployed environments.
- Return generic errors to clients while recording sanitized diagnostic information server-side.
- Keep dependencies and runtime versions maintained.
- Review dependency licenses and security advisories before adopting new packages.
- Run automated tests, static checks, and dependency vulnerability checks in CI as the relevant tooling is introduced.
- Protect CI credentials and restrict workflow permissions to the minimum required.
- Never commit production `.env` files, build secrets, signing keys, or real customer data.

Automated checks reduce risk but do not replace code review or security testing.

## 9. Execution integrity, idempotency, and auditability

AI execution, database persistence, external service calls, and blockchain confirmation are separate operations with different failure modes.

- Assign stable identifiers to executions and security-relevant actions.
- Persist execution state and approved action parameters before attempting consequential external operations.
- Use idempotency controls to prevent retries or duplicate requests from unintentionally repeating an action.
- Record meaningful state transitions, authorization outcomes, approval decisions, and execution results.
- Prevent untrusted model output from altering authoritative execution records.
- Design background jobs to tolerate duplicate delivery, timeouts, and temporary failures.
- Use a durable outbox or equivalent recovery mechanism when database commits and external message or transaction submission must remain coordinated.
- Reconcile uncertain outcomes before retrying operations that may have already succeeded.

Audit records must be access-controlled and protected against unauthorized modification. Do not treat application logs alone as proof that a business action or blockchain transaction succeeded.

## 10. Stellar and Soroban security

HerAgent's planned verifiable execution receipt feature uses Stellar and Soroban as an additional verification layer, not as a replacement for application authorization or private data storage.

- Keep detailed business records and sensitive execution data off-chain.
- Publish only the minimum receipt fields necessary for the intended verification purpose.
- Assess whether hashes, timestamps, identifiers, and other metadata could reveal sensitive information through correlation or guessing.
- Do not assume that hashing makes personal or predictable business data anonymous.
- Validate authorization and inputs within the contract and its authorized submission path.
- Protect signing credentials and restrict which services can submit transactions.
- Test contract authorization, replay resistance, invalid inputs, and failure cases.
- Keep AI execution status separate from blockchain submission and confirmation status.
- Persist receipt intent and submission state so transient network failures can be recovered safely.
- Verify transaction results and receipt data against the expected network, contract, and execution reference.

A successful AI task does not imply a confirmed blockchain receipt. A confirmed receipt does not, by itself, prove that the underlying business action was correct or authorized.

The receipt schema, contract interface, signing model, and verification protocol must be reviewed and tested before production use.

## 11. Testing and security verification

Security must be tested as features are implemented.

The initial security testing plan includes:

- Authentication and authorization tests.
- Cross-workspace access and data-isolation tests.
- Validation tests for API inputs and AI tool arguments.
- Prompt-injection and tool-misuse tests.
- Approval-payload integrity, expiration, and replay tests.
- Idempotency and background-job recovery tests.
- Secret-scanning and dependency vulnerability checks.
- Logging and error-response redaction tests.
- Soroban contract authorization and invalid-input tests.
- Failure-injection tests for database, queue, provider, and blockchain outages.

Before production deployment, conduct a threat-model review and an independent security assessment proportionate to the risks and enabled capabilities.

Security test coverage and audit status must be reported accurately. A passing test suite does not establish that the system is secure against all attacks.

## 12. Security incident response

When a suspected vulnerability or security incident is identified:

1. Assess the potential impact and affected components.
2. Restrict or disable the affected functionality where necessary.
3. Revoke or rotate compromised credentials.
4. Preserve relevant sanitized logs and evidence.
5. Identify affected users, workspaces, data, and integrations.
6. Remediate the root cause and test the fix.
7. Notify affected parties and relevant authorities when required.
8. Document the incident, response, and preventive actions.

Incident response must prioritize limiting further harm and protecting affected users.

## 13. Reporting a vulnerability

Please do not publish sensitive vulnerability details, exploit instructions, exposed credentials, or private user data in a public issue.

Use GitHub's private vulnerability reporting feature for this repository if it is enabled. Otherwise, contact the repository maintainers through an appropriate private channel available from the GitHub organization.

Include the affected component, impact, reproduction steps, and any relevant sanitized evidence. Avoid accessing or modifying data belonging to other users while investigating.

The maintainers will assess the report, coordinate remediation, and communicate an appropriate disclosure timeline. Response times depend on severity and maintainer availability; no fixed response-time guarantee is currently made.

## 14. Compliance and limitations

HerAgent is intended for an international audience. Applicable privacy, consumer-protection, financial, and other legal obligations may vary by jurisdiction and by the functionality enabled.

Before launching in a market or enabling regulated workflows, assess the applicable requirements with qualified advisers where necessary.

This document is an engineering policy, not legal advice, a compliance certification, or a guarantee of security. It will be revised as the implementation, threat model, and operational requirements mature.

## 15. Review and maintenance

Maintainers should review this document when significant changes affect:

- Authentication or authorization.
- Business-workspace isolation.
- AI agents, tools, memory, or external providers.
- Financial or other consequential operations.
- Secret management and signing infrastructure.
- Data retention or third-party processing.
- Stellar/Soroban contracts or receipt verification.
- Deployment, CI/CD, or incident-response procedures.

Security-related changes should include appropriate tests, documentation, and review of their impact on existing trust boundaries.

---

**Related resources**

- [OWASP AI Agent Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/AI_Agent_Security_Cheat_Sheet.html)
- [OWASP Top 10 for Agentic Applications](https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/)
- [HerAgent Architecture Decision Record](../architecture/ADR-001-FOUNDATION.md)
- [HerAgent Contribution Guide](../../CONTRIBUTING.md)
- [HerAgent Security Policy](../../SECURITY.md)
