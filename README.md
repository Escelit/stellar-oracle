# Stellar Oracle

A decentralized price oracle network for the Stellar/Soroban ecosystem.

Provides on-chain price feeds aggregated from multiple trusted publishers via median aggregation — the missing infrastructure layer that every DeFi, RWA, and escrow project on Stellar needs.

## Architecture

```
contracts/
  oracle-core/       # Soroban smart contract (Rust)
sdk/                 # TypeScript publisher + consumer SDKs (coming soon)
dashboard/           # React monitoring dashboard (coming soon)
```

## How It Works

1. **Admin** deploys the contract and registers trusted **publishers**
2. **Publishers** submit price updates for asset pairs (e.g. `XLM/USD`, `BTC/USD`)
3. The contract aggregates all publisher prices via **median** — resistant to outliers
4. **Consumers** (other contracts or dApps) read the latest price, optionally with a freshness check

## Price Format

Prices are scaled by `1e7`. For example:
- `1 XLM = $0.12` → `1_200_000`
- `1 BTC = $60,000` → `600_000_000_000`

## Contract API

| Function | Description |
|---|---|
| `initialize(admin)` | Deploy and set admin |
| `add_publisher(address)` | Register a trusted publisher (admin only) |
| `remove_publisher(address)` | Remove a publisher (admin only) |
| `submit_price(publisher, asset, price, timestamp)` | Submit a price update |
| `get_price(asset)` | Get latest aggregated price |
| `get_price_fresh(asset, max_age_secs)` | Get price, panic if stale |
| `get_assets()` | List all tracked asset pairs |
| `get_publishers()` | List all registered publishers |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli)

```bash
rustup target add wasm32-unknown-unknown
```

### Build

```bash
cargo build --release --target wasm32-unknown-unknown
```

### Test

```bash
cargo test
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/oracle_core.wasm \
  --network testnet \
  --source <your-account>
```

## Contributing

This project participates in the [Stellar Wave Program](https://drips.network/wave/stellar) on Drips.
Check the [issues](../../issues) tab for bounty-eligible tasks.

## License

MIT
