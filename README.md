# HerAgent

**An open-source, AI-powered business assistant for entrepreneurs, built with specialized AI agents and Stellar..**

HerAgent aims to help entrepreneurs manage and grow their businesses through practical AI-assisted workflows for finance, marketing, customer support, business planning, and operations.

Our goal is to make capable business automation more accessible while keeping entrepreneurs in control of consequential decisions. Stellar and Soroban will support verifiable execution receipts, allowing selected execution evidence to be verified without publishing private business data on-chain.

> **Project status:** Early foundation — active development. Some features described below are planned and are not yet implemented. APIs, interfaces, and architecture may evolve as the project develops.

## Why HerAgent?

Entrepreneurs often manage finances, customer relationships, marketing, and daily operations across disconnected tools. HerAgent aims to bring these workflows together through specialized AI agents, reusable integrations, and permission-controlled actions.

The project is being designed for an international audience, with web and mobile experiences and an architecture that supports contributions from developers across different backgrounds.

## Planned capabilities

- **Finance Agent:** Organize business transactions, categorize records, and produce summaries from verified business data.
- **Marketing Agent:** Help draft campaigns, product descriptions, promotional content, and content calendars.
- **Customer Support Agent:** Assist with customer enquiries, conversation summaries, and response drafts.
- **Business Planning Agent:** Help entrepreneurs develop business plans, budgets, and growth strategies.
- **Operations Agent:** Support task management, reminders, and repeatable workflows.
- **Verifiable execution receipts:** Use Stellar and Soroban to anchor selected execution evidence for independent verification.

These are planned capabilities, not a claim that every agent or blockchain feature is already available.

## Core design principles

- **Human control:** Sensitive or consequential actions require explicit authorization and approval.
- **Security and privacy:** Enforce server-side permissions and business-workspace isolation. Keep sensitive business information off-chain.
- **Verifiability:** Design execution receipts so selected results can be independently checked without exposing private records.
- **Provider flexibility:** Keep AI providers and external integrations replaceable.
- **Reliable execution:** Design long-running jobs with validation, retries, idempotency, and auditable outcomes.
- **Open collaboration:** Maintain clear documentation, scoped issues, tests, and contributor-friendly development workflows.
- **Accessibility:** Build toward responsive web and mobile experiences that work for entrepreneurs with different needs and resources.

## Planned technical architecture

| Area | Planned technology |
|---|---|
| Web application | Next.js, React, TypeScript, Tailwind CSS |
| Mobile application | Expo, React Native, TypeScript |
| Backend API | NestJS, TypeScript, REST, OpenAPI |
| Data storage | PostgreSQL and Prisma |
| Background execution | Worker process and queue |
| AI orchestration | Provider-agnostic model integration and typed tools |
| Smart contracts | Rust and Soroban |
| Blockchain network | Stellar Testnet during initial development |
| Repository | pnpm workspace and Turborepo |
| Quality | Automated tests and GitHub Actions |

The backend will initially use a modular-monolith architecture to keep development and operations manageable. Components will be separated into independently maintainable modules without introducing premature microservices.

## Repository structure

```text
heragent/
├── apps/
│   ├── web/                 # Planned web application
│   ├── mobile/              # Planned mobile application
│   ├── api/                 # NestJS backend
│   └── worker/              # Planned background execution
├── packages/                # Shared code and contracts
├── contracts/
│   └── execution-receipts/  # Planned Soroban receipt contract
├── services/
│   └── stellar-indexer/     # Planned Stellar indexing service
├── docs/
│   ├── architecture/
│   ├── product/
│   └── security/
├── infra/
├── scripts/
├── CONTRIBUTING.md
├── SECURITY.md
└── LICENSE
```

The repository is being built incrementally. Directory presence does not necessarily mean that a component is implemented or production-ready.

## Getting started

### Prerequisites

- Git
- Node.js 22 or newer
- Corepack
- pnpm 10
- Docker Desktop when local database and queue services are introduced
- Rust and Stellar CLI when working on Soroban contracts

### Clone and install

```bash
git clone https://github.com/HerAgent-Labs/heragent.git
cd heragent

corepack enable
corepack prepare pnpm@10.28.2 --activate
pnpm install
```

Use the package-manager version declared in the root `package.json` if it changes.

### Development status

The repository foundation and initial NestJS API scaffold have been established. Further setup instructions, environment configuration, database integration, testing commands, and application-specific workflows will be documented as those components become operational.

Do not put production secrets, API keys, wallet secrets, or private business data in the repository.

## Documentation

- [Architecture decisions](docs/architecture/ADR-001-FOUNDATION.md)
- [Product direction](docs/product/PRODUCT_DIRECTION.md)
- [Security and privacy principles](docs/security/SECURITY_AND_PRIVACY.md)
- [Contributing guide](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [Code of conduct](CODE_OF_CONDUCT.md)

## Contributing

Contributions are welcome as the project develops. Areas of planned work include backend APIs, agent tooling, test coverage, security, documentation, frontend and mobile foundations, and Stellar/Soroban integration.

Please review [CONTRIBUTING.md](CONTRIBUTING.md) and the repository's open issues before starting work. Prefer scoped changes with clear acceptance criteria and tests where applicable.

## Roadmap

1. Establish repository standards, developer tooling, and CI.
2. Build the API foundation, configuration, validation, and persistence.
3. Implement business workspaces and core domain services.
4. Introduce the agent execution lifecycle, typed tools, and approval controls.
5. Develop and evaluate the initial specialized agents.
6. Implement and test verifiable execution receipts on Stellar Testnet.
7. Build and integrate the web and mobile experiences.
8. Improve reliability, security, documentation, and contributor onboarding.

Milestones may change as implementation and testing provide evidence for better decisions.

## License

HerAgent is licensed under the [Apache License 2.0](LICENSE), subject to the licensing terms of its dependencies and third-party assets.
