# Changelog

All notable changes to this project will be documented in this file.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added
- `oracle-core` Soroban smart contract with median price aggregation
- `initialize`, `add_publisher`, `remove_publisher`, `get_publishers` admin functions
- `submit_price` with per-publisher temporary storage and automatic re-aggregation
- `get_price`, `get_price_fresh`, `get_assets` read functions
- TTL extension on publisher price entries (~7 days)
- TypeScript `OraclePublisher` SDK class for submitting prices
- TypeScript `OracleConsumer` SDK class for reading prices
- Automated publisher bot fetching XLM/USD, BTC/USD, ETH/USD from CoinGecko
- `toFloat` / `toScaled` price conversion utilities
- CI: Rust tests, TypeScript build, clippy, fmt
- Testnet deployment at `CA76KLJ2CDD5OHVGD6MUV3QVZYRNJJQLIHBMWD353J6ES4JZXCO4L5OQ`
