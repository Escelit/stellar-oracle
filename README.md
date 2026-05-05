<p align="center">
  <img src="stellar_oracle_logo_1777981420458.png" width="350" alt="Stellar Oracle Logo">
</p>

# Stellar Oracle

<div align="center">

![CI](https://github.com/Escelit/stellar-oracle/workflows/CI/badge.svg)
![Security](https://github.com/Escelit/stellar-oracle/workflows/Security%20Scan/badge.svg)
![Version](https://img.shields.io/github/v/release/Escelit/stellar-oracle?style=flat-square)

![Stellar](https://img.shields.io/badge/Stellar-Soroban-7D00FF?style=for-the-badge&logo=stellar)
![Rust](https://img.shields.io/badge/Rust-2021-000000?style=for-the-badge&logo=rust)
![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?style=for-the-badge&logo=typescript)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**Decentralized, outlier-resistant price feeds providing critical infrastructure for the Soroban DeFi ecosystem.**

[Features](#-features) • [Quick Start](#-quick-start) • [Architecture](#-architecture) • [Documentation](#-documentation) • [Contributing](#-contributing) • [Roadmap](#-roadmap)

</div>

---

## 📋 Table of Contents

- [Overview](#-overview)
- [Features](#-features)
- [Architecture](#-architecture)
- [Tech Stack](#-tech-stack)
- [Project Structure](#-project-structure)
- [Smart Contract API](#-smart-contract-api)
- [SDK Reference](#-sdk-reference)
- [Getting Started](#-getting-started)
- [Security & Reliability](#-security--reliability)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🌌 Overview

In the world of decentralized finance (DeFi) and Real-World Assets (RWAs), smart contracts are often "walled gardens"—they cannot natively access data from outside the blockchain. **Stellar Oracle** serves as the vital bridge, bringing high-fidelity, real-time price data into the Soroban ecosystem.

### The Problem
Traditional oracles often suffer from three major vulnerabilities:
1. **Single Point of Failure**: Relying on a single data source or publisher can lead to catastrophic losses if that source is compromised.
2. **Outlier Sensitivity**: Simple average (mean) calculations are easily skewed by a single "fat-finger" trade or a flash-crash on one exchange.
3. **High Latency & Costs**: Many oracles are too slow or too expensive to use for high-frequency DeFi operations.

### Our Solution
Stellar Oracle is engineered to solve these challenges through a decentralized, multi-publisher model:

- **Decentralized Consensus**: Instead of trusting one source, we whitelist multiple independent **Publishers** (e.g., specialized data bots, exchanges, or institutional providers).
- **Outlier-Resistant Math**: By using **Median Aggregation**, we ensure that even if a significant minority of publishers report erroneous data, the resulting on-chain price remains accurate to the market consensus.
- **Soroban-Native Efficiency**: We utilize Stellar's unique storage tiers—storing global results in `Instance` storage for fast reads, while relegating individual publisher footprints to `Temporary` storage to keep the network lean and costs ultra-low.

### Core Value Proposition
For developers building on Stellar, this oracle provides the "Source of Truth" needed for:
- **Lending Protocols**: Accurate collateral valuation and liquidation triggers.
- **Stablecoins**: Maintaining robust pegs against external fiat or assets.
- **Escrow Services**: Automated release of funds based on verified market conditions.
- **DEX Aggregators**: Precision routing and slippage calculation.

---

## ✨ Features

### Current Features

#### 🦀 Oracle Core (Smart Contract)
- ✅ **Multi-Publisher Support**: Whitelist multiple trusted addresses to submit data.
- ✅ **Median Engine**: On-chain computation of the median price point.
- ✅ **Hybrid Storage**: Uses `Instance` for state and `Temporary` for individual entries to minimize ledger footprint.
- ✅ **Circuit Breaker**: `max_deviation_bps` rejection logic for anomalous price submissions.
- ✅ **Event Emission**: `PriceUpdated` events for off-chain indexing and monitoring.

#### 🛠️ Developer SDK
- ✅ **OraclePublisher**: High-level class for managing transaction building, signing, and submission.
- ✅ **OracleConsumer**: Simulation-based price reading (zero transaction cost for consumers).
- ✅ **Error Handling**: Comprehensive parsing of contract errors into actionable TS exceptions.
- ✅ **Type Safety**: Full TypeScript definitions for all contract types.

#### 🧪 Quality Assurance
- ✅ **Comprehensive Test Suite**: >90% coverage on core aggregation logic.
- ✅ **Snapshots**: Verified test snapshots for predictable contract behaviour.
- ✅ **Automation**: CI/CD pipelines for linting and testing.

---

## 🏗️ Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Off-Chain Layer (SDK)                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ Publisher Bot│  │   Consumer   │  │  Monitoring  │     │
│  │ (Stellar SDK)│  │   (dApp/UI)  │  │   Dashboard  │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Stellar Network (Soroban)                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │           Oracle Core Contract (Rust)                │   │
│  │  - submit_price()                                    │   │
│  │  - aggregate() -> Compute Median                     │   │
│  │  - get_price_fresh()                                 │   │
│  │  - add_publisher()                                   │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────────┬───────────────────┬──────────────────┘
                       │                   │
                       ▼                   ▼
            ┌───────────────────┐   ┌───────────────────┐
            │ Instance Storage  │   │ Temporary Storage │
            │ (Global Feeds)    │   │ (Pub Entries)     │
            └───────────────────┘   └───────────────────┘
```

### Data Flow

1. **Submission**: Authorized publishers call `submit_price` with an asset symbol and scaled price.
2. **Persistence**: The contract stores the individual entry in `Temporary` storage (TTL ~7 days).
3. **Aggregation**: The contract triggers a re-aggregation, fetching all recent entries for that asset.
4. **Median Logic**: Prices are sorted and the median is computed to update the global `FeedData`.
5. **Consumption**: Consumer dApps call `get_price_fresh`, which verifies the data timestamp against the current ledger before returning the value.

---

## 🛠️ Tech Stack

### Core Components

| Component | Technology | Purpose |
|:--- |:--- |:--- |
| **Smart Contract** | Rust (Soroban SDK) | Secure on-chain logic |
| **Price Scaling** | Fixed-point (1e7) | Precision without floating point errors |
| **Integration** | TypeScript | SDK for publishers and consumers |
| **Network** | Stellar Testnet | Decentralized blockchain layer |
| **Documentation** | Mermaid.js | Architectural visualization |

---

## 📁 Project Structure

```text
stellar-oracle/
├── contracts/
│   └── oracle-core/         # Core Soroban Smart Contract
│       ├── src/
│       │   ├── lib.rs       # Main logic: Aggregation & Storage
│       │   └── test.rs      # Comprehensive Rust tests
│       └── Cargo.toml       # Contract dependencies
├── sdk/                     # Developer Tools & Client Libraries
│   ├── src/                 # TypeScript source
│   │   ├── publisher.ts     # Data submission service
│   │   ├── consumer.ts      # Data consumption service
│   │   └── types.ts         # Shared TS types
│   ├── examples/            # Ready-to-run scripts
│   └── package.json         # SDK configuration
├── docs/                    # Technical specs & architecture
├── stellar.toml             # Network & contract metadata
└── README.md                # Project documentation
```

---

## 📜 Smart Contract API

### Core Functions

#### `initialize`
Setup the contract with an administrator.
```rust
pub fn initialize(env: Env, admin: Address)
```

#### `submit_price`
Publish a new price point. Authorized publishers only.
```rust
pub fn submit_price(
    env: Env, 
    publisher: Address, 
    asset: String, 
    price: i128, 
    timestamp: u64
)
```

#### `get_price_fresh`
Retrieve the latest aggregated price with a staleness check.
```rust
pub fn get_price_fresh(
    env: Env, 
    asset: String, 
    max_age_secs: u64
) -> FeedData
```

### Error Codes

| Code | Name | Description |
|:--- |:--- |:--- |
| 1 | `Unauthorized` | Caller lacks permissions |
| 2 | `FeedNotFound` | Asset pair does not exist |
| 3 | `StalePrice` | Data is older than `max_age_secs` |
| 7 | `PriceDeviationExceeded` | Submission failed circuit breaker check |

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: `rustup target add wasm32-unknown-unknown`
- **Stellar CLI**: `cargo install --locked stellar-cli`
- **Node.js**: v18+ for SDK usage

### Quick Installation

1. **Clone the repo**
   ```bash
   git clone https://github.com/Escelit/stellar-oracle.git
   cd stellar-oracle
   ```

2. **Build & Test**
   ```bash
   cargo build --release --target wasm32v1-none
   cargo test
   ```

3. **Deploy (Testnet)**
   ```bash
   stellar contract deploy \
     --wasm target/wasm32v1-none/release/oracle_core.wasm \
     --network testnet \
     --source <your-account>
   ```

---

## 🛡️ Security & Reliability

### Outlier Resilience
By utilizing the **Median** rather than a simple mean, Stellar Oracle ensures that even if a significant minority of publishers are reporting incorrect or manipulated data, the resulting on-chain price remains accurate to the market consensus.

### Circuit Breaker Logic
The `max_deviation_bps` (basis points) parameter allows administrators to set a safety net. If a publisher tries to submit a price that differs from the current median by more than the allowed percentage (e.g., 500 bps = 5%), the transaction is rejected.

---

## 🗺️ Roadmap

### Phase 1: Foundation ✅
- Core median aggregation engine.
- Hybrid storage model for gas optimization.
- Whitelist-based publisher management.

### Phase 2: Ecosystem Expansion [/]
- TypeScript SDK Release.
- Automated publisher bot implementation.
- Multi-asset support (XLM, BTC, ETH, USDC).

### Phase 3: Advanced Features
- **TWAP Support**: Time-weighted average prices for smoother DeFi operations.
- **Reputation System**: On-chain scoring for publishers.
- **Monitoring Dashboard**: Real-time visual tracking of feed health.

---

## 🤝 Contributing

We welcome contributions of all kinds!
1. **Fork** the repository.
2. **Create** a feature branch (`git checkout -b feat/amazing-feature`).
3. **Commit** your changes (`git commit -m 'feat: add amazing feature'`).
4. **Push** to the branch (`git push origin feat/amazing-feature`).
5. **Open** a Pull Request.

Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for full guidelines.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">
Built with ❤️ for the Stellar Community.
</div>
