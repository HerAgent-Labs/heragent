# HerAgent

**An open-source AI-powered business assistant for entrepreneurs, built with specialized agents and Stellar.**

HerAgent helps entrepreneurs manage and grow their businesses through AI-assisted workflows for finance, marketing, customer support, planning, and operations. Sensitive actions require explicit human approval. Stellar Soroban is used to provide verifiable execution receipts—not to store private business data.

## Project status

Early foundation / active development. Interfaces, APIs, and architecture may change.

## Principles

- **Human control:** sensitive or consequential actions require explicit approval.
- **Tenant isolation:** business data is accessible only to authorized users.
- **Verifiability:** execution evidence can be anchored on Stellar without publishing private business data.
- **Provider flexibility:** AI providers and external services are accessed through replaceable adapters.
- **Open source:** contributions, documentation, and reproducible development are first-class concerns.

## Planned platform

- `apps/web` — Next.js web application
- `apps/mobile` — Expo / React Native mobile application
- `apps/api` — NestJS API and domain modules
- `apps/worker` — background jobs and agent execution
- `packages/` — shared TypeScript packages
- `contracts/` — Soroban smart contracts
- `services/stellar-indexer` — Stellar event indexing

## Getting started

### Prerequisites

- Git
- Node.js 22 or newer
- Corepack (included with supported Node.js distributions)
- pnpm 10
- Docker Desktop (for local PostgreSQL and Redis when those services are introduced)
- Rust and Stellar CLI (when working on Soroban contracts)

### Bootstrap

```powershell
corepack enable
corepack prepare pnpm@10.10.0 --activate
pnpm install
```

The repository is being initialized incrementally. Application-specific development commands will be added as each workspace is scaffolded.

## Documentation

- [Product direction](docs/product/PRODUCT_DIRECTION.md)
- [Architecture decisions](docs/architecture/ADR-001-FOUNDATION.md)
- [Security and privacy principles](docs/security/SECURITY_AND_PRIVACY.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## License

Apache License 2.0. See [LICENSE](LICENSE).
