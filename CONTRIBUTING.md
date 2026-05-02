# Contributing to Stellar Oracle

Thanks for your interest! This project participates in the [Stellar Wave Program](https://drips.network/wave/stellar) — contributors can earn rewards for merged PRs.

## Getting Started

### Prerequisites
- Rust + `wasm32-unknown-unknown` target
- Node.js 18+
- Stellar CLI

```bash
rustup target add wasm32-unknown-unknown
```

### Run contract tests
```bash
cargo test
```

### Build the SDK
```bash
cd sdk && npm install && npm run build
```

## How to Contribute

1. Pick an open issue (Wave bounty issues are labeled `wave-bounty`)
2. Comment on the issue to claim it
3. Fork the repo and create a branch: `git checkout -b feat/your-feature`
4. Make your changes with tests
5. Open a PR — reference the issue number in the description

## Project Structure

```
contracts/oracle-core/   # Soroban smart contract (Rust)
sdk/src/
  publisher.ts           # Submit prices on-chain
  consumer.ts            # Read prices from on-chain
  publisher-bot.ts       # Automated price feed bot
```

## Code Standards

- Rust: run `cargo clippy` and `cargo fmt` before committing
- TypeScript: keep strict mode, no `any` unless unavoidable
- All new contract functions must have tests in `test.rs`

## Questions?

Open a GitHub Discussion or drop into the Stellar Discord.
