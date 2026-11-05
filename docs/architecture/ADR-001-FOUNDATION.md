# ADR-001: Initial Technical Foundation

- **Status:** Accepted as the starting direction; subject to review as implementation evidence emerges.
- **Date:** 2026-11-03

## Context

HerAgent is a global, open-source AI business assistant intended to support web and mobile clients, specialized agents, and verifiable execution receipts on Stellar. The initial team is small, so operational complexity must remain controlled.

## Decision

- Use a TypeScript monorepo managed by pnpm and Turborepo.
- Use Next.js for web, Expo / React Native for mobile, and NestJS for the API.
- Use PostgreSQL as the primary relational datastore.
- Use a worker process and queue for long-running agent tasks.
- Keep the backend as a modular monolith initially.
- Use a provider-agnostic AI integration layer and typed tools.
- Use Rust and Soroban for Stellar smart contracts.
- Begin blockchain integration on Stellar Testnet.
- Store complete execution records off-chain; anchor only minimal, privacy-reviewed verification data on-chain.
- Require explicit human approval for sensitive actions.
- Adopt Apache-2.0 for the project, subject to dependency and asset license review.

## Consequences

This structure supports shared contracts and validation across clients while avoiding premature microservices. It requires discipline around module boundaries, tenant isolation, background-job idempotency, and server-side authorization.

## Revisit when

- Scaling or team ownership makes a module boundary operationally necessary.
- Measured performance or reliability requires a different deployment topology.
- Regulatory, privacy, or payment-provider requirements change the data architecture.
