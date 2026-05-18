# Stelliate

**Instant Affiliate Payout Protocol on Stellar**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built on Stellar](https://img.shields.io/badge/Built%20on-Stellar-7B2FBE)](https://stellar.org)
[![Powered by Soroban](https://img.shields.io/badge/Powered%20by-Soroban-blueviolet)](https://soroban.stellar.org)
[![Status: In Development](https://img.shields.io/badge/Status-In%20Development-yellow)]()

Stelliate is an open-source affiliate payout infrastructure built on the Stellar network, powered by Soroban smart contracts. It enables merchants to deposit USDC and automatically distribute instant, rule-based affiliate commissions on every verified sale — no intermediaries, no delays, no manual processing.

---

## Table of Contents

- [Why Stelliate](#why-stelliate)
- [How It Works](#how-it-works)
- [Core Features](#core-features)
- [Architecture](#architecture)
- [Repository Structure](#repository-structure)
- [Getting Started](#getting-started)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [Compliance](#compliance)
- [License](#license)

---

## Why Stelliate

Affiliate marketing is a **$17B+ global industry**, yet the infrastructure powering it is broken:

| Problem | Reality |
|---|---|
| Delayed payouts | Affiliates wait weeks or months |
| Opaque accounting | No on-chain audit trail |
| Platform lock-in | Funds controlled by intermediaries |
| Cross-border friction | High fees, slow settlement |

Stelliate replaces this with **real-time, programmable, on-chain settlement** — built on Stellar for fast finality (seconds), near-zero fees, and global USDC accessibility.

---

## How It Works

```
Merchant deposits USDC into escrow contract
        ↓
Affiliate drives a verified sale
        ↓
Webhook triggers sale verification
        ↓
Soroban contract executes commission split
        ↓
Affiliate receives USDC instantly on-chain
```

1. **Merchant** deploys a payout contract and deposits USDC as escrow.
2. **Affiliate** shares a tracked referral link.
3. **Sale event** is captured via webhook and verified off-chain.
4. **Soroban contract** enforces the commission rule and executes the payout atomically.
5. **Affiliate** receives USDC in seconds, with a fully auditable on-chain record.

---

## Core Features

- ⚡ **Instant USDC Payouts** — commissions settle in seconds after a verified sale
- 🔐 **Soroban Smart Contracts** — commission rules, revenue splits, and escrow logic enforced on-chain
- 🛍️ **Ecommerce Integrations** — Shopify and WooCommerce plugins (in development)
- 🧰 **TypeScript SDK** — webhook tracking, contract interaction helpers, and payout management
- 📊 **Transparent Accounting** — every payout is on-chain, auditable, and immutable
- 🌍 **Global by Default** — USDC on Stellar works across borders with no extra friction

---

## Architecture

### Tech Stack

| Layer | Technology |
|---|---|
| Smart Contracts | Soroban (Rust) |
| Settlement Asset | USDC on Stellar |
| RPC | Stellar RPC |
| Backend | Node.js / TypeScript |
| Event Processing | Webhook processor |
| Database | PostgreSQL (event logs) |
| Frontend | React (merchant dashboard) |

### Data Flow

```
Ecommerce Platform
      │  webhook
      ▼
  API Layer (Node.js)
      │  verify + emit
      ▼
Soroban Payout Contract
      │  transfer
      ▼
  Affiliate Wallet (USDC)
```

On-chain data contains only financial settlement logic — no personal data.

---

## Repository Structure

```
stelliate/
├── contracts/       # Soroban smart contracts (Rust)
├── sdk/             # TypeScript/JavaScript developer SDK
├── plugins/         # Ecommerce integrations (Shopify, WooCommerce)
├── api/             # Webhook processor + payout engine
├── dashboard/       # Merchant UI (React)
├── docs/            # Documentation
└── examples/        # Sample integrations
```

---

## Getting Started

> ⚠️ Stelliate is in active development. The steps below reflect the intended setup flow.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-stellar-cli)
- Node.js ≥ 18
- A Stellar testnet account funded via [Friendbot](https://friendbot.stellar.org)

### Install

```bash
git clone https://github.com/Stelliance-Lab/Stelliate.git
cd Stelliate
```

### Build Contracts

```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stelliate.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet
```

### Run the API

```bash
cd api
npm install
npm run dev
```

---

## Contributing

Stelliate is built as a fully open-source protocol for the Stellar ecosystem. Contributions are welcome and encouraged.

### Good First Issues

Look for issues tagged [`good first issue`](../../issues?q=is%3Aissue+label%3A%22good+first+issue%22) — these are scoped, well-documented, and ideal for new contributors.

### Areas We Need Help

- 🦀 **Soroban contracts** — escrow logic, commission rules, edge case handling
- 🧰 **SDK** — TypeScript helpers, error handling, test coverage
- 🛍️ **Plugins** — Shopify and WooCommerce integrations
- 🔒 **Security reviews** — contract auditing, input validation
- 📝 **Documentation** — guides, API references, integration examples
- 🧪 **Testing** — unit tests, integration tests, testnet scenarios

### How to Contribute

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes with clear, focused commits
4. Open a pull request with a description of what you changed and why

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR.

### Development Principles

- Keep contracts minimal and auditable
- Separate on-chain logic from off-chain identity
- Write tests for every contract function
- Document public SDK methods with JSDoc

---

## Roadmap

### Phase 1 — Core Protocol
- [ ] Soroban payout contract
- [ ] USDC escrow system
- [ ] Testnet deployment

### Phase 2 — Integration Layer
- [ ] Merchant API (Node.js)
- [ ] TypeScript SDK v1
- [ ] Shopify plugin (MVP)
- [ ] WooCommerce plugin (MVP)

### Phase 3 — Merchant Experience
- [ ] Merchant dashboard (React)
- [ ] Affiliate analytics panel
- [ ] Webhook event explorer

### Phase 4 — Ecosystem & Audit
- [ ] Independent security audit
- [ ] SCF grant submission
- [ ] Mainnet launch

---

## Compliance

Stelliate is designed with compliance in mind:

- **GDPR** — data minimization; no personal data stored on-chain
- **FTC** — supports affiliate disclosure requirements
- **CCPA** — off-chain identity separation

All on-chain data is limited to financial settlement logic (amounts, addresses, timestamps).

---

## License

[MIT](LICENSE) — free to use, modify, and distribute.

---

<p align="center">
  Built with ❤️ for the <a href="https://stellar.org">Stellar</a> ecosystem
</p>
