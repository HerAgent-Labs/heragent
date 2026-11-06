# ADR-001: Initial Technical Foundation

- **Status:** Accepted as the initial architectural direction; subject to revision as implementation evidence emerges.
- **Date:** 2026-11-06
- **Decision owners:** HerAgent maintainers

## 1. Context

HerAgent is an open-source AI-powered business assistant intended to help entrepreneurs manage business workflows through specialized AI agents. The planned platform includes web and mobile clients, a backend API, background execution, shared TypeScript packages, and verifiable execution receipts using Stellar and Soroban.

The project is being developed incrementally by a small team and an open-source contributor community. The architecture must support rapid iteration without sacrificing security, maintainability, testability, or data privacy.

AI-generated actions introduce additional risks: model outputs may be incorrect, tools may be invoked with invalid arguments, and retries may cause duplicate operations. Blockchain integration also introduces asynchronous transaction confirmation and external network failures.

The initial architecture must address these concerns without adding unnecessary operational complexity.

## 2. Decision drivers

The primary decision criteria are:

1. Maintainable development for a small team.
2. Clear boundaries between business logic, AI orchestration, and external integrations.
3. Secure authorization and isolation of business workspaces.
4. Reliable execution of asynchronous tasks.
5. Human oversight of sensitive business actions.
6. Provider flexibility for AI models and external services.
7. Verifiable execution evidence without exposing private business data.
8. Reproducible local development and accessible open-source contribution workflows.

## 3. Architectural decisions

### 3.1 Monorepo and shared tooling

Use a TypeScript monorepo managed by pnpm workspaces and Turborepo.

Shared packages will hold reusable validation, domain contracts, API client types, configuration, and test utilities where appropriate.

**Rationale:** A monorepo makes coordinated changes and type sharing easier across applications while keeping related code discoverable.

**Trade-off:** Workspace boundaries and dependency rules must be maintained deliberately to prevent excessive coupling.

### 3.2 Backend architecture

Use NestJS and TypeScript for the API. Begin with a modular monolith, separating domain capabilities into explicit modules rather than deploying microservices prematurely.

Potential modules include authentication, business workspaces, agents, executions, approvals, finance, customers, tasks, notifications, audit, and blockchain integration.

Create modules when they support actual functionality; avoid empty scaffolding solely to match a proposed architecture.

**Rationale:** A modular monolith reduces deployment and operational overhead while preserving clear internal boundaries.

**Trade-off:** Module dependencies, authorization boundaries, and shared database access require disciplined design.

### 3.3 Web and mobile clients

Use Next.js, React, and TypeScript for the web application. Use Expo, React Native, and TypeScript for the mobile application.

Both clients will consume documented backend APIs rather than independently implementing business rules.

**Rationale:** Shared language and API contracts reduce duplicated knowledge and support consistent behavior across platforms.

**Trade-off:** User interfaces, navigation, and platform-specific capabilities still require deliberate cross-platform testing.

### 3.4 Data persistence

Use PostgreSQL as the primary relational datastore, with Prisma as the planned data-access layer.

Business records, execution metadata, approval decisions, audit events, and receipt-submission status will be persisted off-chain.

Enforce business-workspace authorization on the server for every protected operation. Database schema design and query patterns must support tenant isolation.

**Rationale:** Relational persistence supports transactional consistency, structured business records, and auditable state transitions.

**Trade-off:** Schema changes and data migrations must be versioned, reviewed, and tested.

### 3.5 Background execution

Use a separate worker process and a queue for long-running agent tasks, scheduled work, retries, and blockchain receipt submission.

Jobs must be designed for duplicate delivery and transient failures. Use idempotency controls, bounded retries, explicit failure states, and durable persistence of work that must not be lost.

Where a database transaction must initiate asynchronous work, use a durable outbox or an equivalent mechanism to avoid losing the work between database commits and queue publication.

**Rationale:** Background workers keep request handling responsive and allow independent retry and monitoring strategies.

**Trade-off:** Queue operations and database state must be coordinated carefully; a queue alone does not guarantee exactly-once execution.

### 3.6 AI orchestration and tool execution

Use a provider-agnostic AI integration layer and typed, validated tools. The model may propose actions, but deterministic application services execute them.

The AI layer must not have unrestricted database access or authority to bypass application permissions.

Initial agent capabilities are expected to focus on finance, marketing, and customer support, with additional planning and operations workflows introduced as their requirements become clear.

**Rationale:** Separating model reasoning from domain operations makes behavior easier to test, audit, and change when providers evolve.

**Trade-off:** Tool contracts, error handling, model evaluation, and observability require ongoing engineering effort.

### 3.7 Human approval and authorization

Sensitive or consequential actions must be authorized by backend policy and require explicit human approval before execution.

An approval must be bound to the exact proposed action, including relevant parameters. The backend must verify the approver's permissions and ensure the action has not changed, expired, or already been executed.

Approval does not replace authorization. The model cannot grant permissions, approve its own actions, or modify an approved payload.

**Rationale:** Human oversight limits the impact of incorrect model outputs and unauthorized operations.

**Trade-off:** Approval workflows add latency and require clear pending, approved, rejected, expired, and failed states.

### 3.8 Stellar and Soroban execution receipts

Use Rust and Soroban for the planned smart-contract component, beginning on Stellar Testnet.

The proposed feature is a verifiable execution receipt: selected execution evidence can be anchored on-chain so an authorized verifier can check that a record corresponds to a committed result.

Complete execution records and sensitive business information remain off-chain. The receipt design must minimize published metadata and undergo privacy and security review before deployment.

Blockchain confirmation is a separate state from successful AI execution. The system must preserve the execution result if receipt submission or confirmation is delayed or fails.

**Rationale:** Stellar can provide an independent verification layer without requiring the entire business workflow to run on-chain.

**Trade-off:** Transaction fees, network availability, confirmation delays, contract security, and privacy leakage through metadata must be considered.

The exact contract interface, receipt schema, and verification protocol remain subject to implementation and testing.

### 3.9 Security, configuration, and observability

Keep secrets in environment-specific secret-management systems and out of source control. Signing credentials must remain server-side and must never be exposed to client applications or language models.

Validate external inputs at system boundaries. Record security-relevant events and execution outcomes without unnecessarily logging secrets or private business data.

As the platform develops, add structured logs, metrics, traces, automated tests, dependency checks, and continuous integration appropriate to each component.

**Rationale:** Security and observability must be part of the architecture rather than deferred until deployment.

**Trade-off:** These controls introduce implementation and operational work, which should be introduced incrementally without weakening core trust boundaries.

### 3.10 Licensing and open-source governance

Use Apache License 2.0 for the project, subject to review of dependency licenses and third-party assets.

Maintain contributor guidance, a security-reporting process, a code of conduct, clear issue scopes, and documented architectural decisions.

**Rationale:** A transparent governance model supports external contributions and makes design decisions easier to review.

**Trade-off:** Maintainers must invest time in reviewing contributions, maintaining documentation, and communicating project status accurately.

## 4. High-level execution flow

The intended agent execution flow is:

1. An authenticated user submits a request through a client.
2. The API validates the request and checks workspace access.
3. An execution record is created and eligible work is queued.
4. A worker invokes the selected agent and its permitted tools.
5. Application services validate and execute authorized operations.
6. Sensitive actions pause for approval where required.
7. The outcome and audit metadata are persisted.
8. If enabled, a receipt is submitted to Soroban using a durable, retryable workflow.
9. Receipt status is updated after network confirmation.
10. The client receives the execution outcome and, where available, verification information.

This flow is the target design, not a claim that every step is already implemented.

## 5. Consequences

### Positive

- A consistent TypeScript development workflow across core applications.
- Lower operational complexity during early development.
- Explicit boundaries between AI-generated proposals and application-controlled execution.
- A foundation for tenant isolation, auditable actions, and human approval.
- An incremental path to verifiable execution evidence on Stellar.
- A repository structure suitable for scoped open-source contributions.

### Costs and risks

- Shared packages require stable contracts and disciplined dependency management.
- Background jobs require idempotency and recovery handling.
- Human approval introduces additional workflow states.
- AI integrations require evaluation and monitoring to control unreliable outputs.
- Blockchain receipts introduce asynchronous failure modes and metadata-privacy concerns.
- Security and tenant isolation require continuous testing rather than documentation alone.

## 6. Alternatives considered

### Microservices from the beginning

**Decision:** Defer.

Independent services may become useful when scaling, reliability, or team ownership justifies them. Initially, the deployment and operational overhead would be disproportionate to the team's size.

### Blockchain as the primary datastore

**Decision:** Reject.

Private business records and detailed execution data should remain in controlled off-chain storage. The blockchain is intended to support verification, not replace the application database.

### Direct model access to business data

**Decision:** Reject.

Models should use explicitly defined, validated, and permissioned tools. Application services remain responsible for authorization, business rules, and data access.

### Mandatory wallet connection for every user

**Decision:** Reject for the initial product.

HerAgent should remain useful without a connected wallet. Blockchain verification is an additional capability rather than a prerequisite for ordinary business workflows.

## 7. Revisit this decision when

Review this ADR if:

- Measured load or reliability requires a different deployment topology.
- Team growth makes independent service ownership beneficial.
- Security assessments identify a need to change trust boundaries.
- AI-provider capabilities or costs materially change the orchestration strategy.
- Regulatory or data-residency requirements change the persistence design.
- Stellar network capabilities or receipt-verification requirements change.
- Production evidence shows that an architectural decision no longer meets its intended goals.

Changes should be documented through a new ADR or a reviewed revision to this record.
