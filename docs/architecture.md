# Architecture

## Overview

Stellar Oracle is a decentralized price feed network for the Soroban smart contract platform. It follows a **push-based publisher model**: off-chain bots submit signed price updates on-chain, and the contract aggregates them into a single canonical price per asset pair.

```
┌─────────────────┐     submit_price()     ┌──────────────────────┐
│  Publisher Bot  │ ──────────────────────▶│                      │
│  (TypeScript)   │                        │   oracle-core        │
└─────────────────┘                        │   (Soroban contract) │
                                           │                      │
┌─────────────────┐     get_price()        │  - publisher registry│
│  Consumer dApp  │ ◀──────────────────────│  - per-publisher     │
│  or Contract    │                        │    price entries     │
└─────────────────┘                        │  - aggregated feeds  │
                                           └──────────────────────┘
```

## Publisher Model

- The admin registers a set of **trusted publishers** (Stellar addresses).
- Each publisher runs an off-chain bot that fetches prices from external APIs and calls `submit_price(publisher, asset, price, timestamp)`.
- The publisher must sign the transaction — the contract calls `publisher.require_auth()`.
- Any number of asset pairs (e.g. `XLM/USD`, `BTC/USD`) can be tracked; new pairs are created automatically on first submission.

## Median Aggregation

On every `submit_price` call, the contract:

1. Stores the new entry in **temporary storage** keyed by `(asset, publisher)`.
2. Reads all known publishers and collects their latest entry for that asset.
3. Sorts the prices and computes the **median** (average of two middle values for even N).
4. Writes the aggregated `FeedData` to **instance storage**.

Median is chosen over mean because it is resistant to a single outlier publisher submitting a manipulated price. A publisher would need to control >50% of registered publishers to move the median.

## Storage Layout

| Key | Storage type | Value | Notes |
|-----|-------------|-------|-------|
| `ADMIN` | Instance | `Address` | Set once at `initialize` |
| `PUBS` | Instance | `Vec<Address>` | Registered publishers |
| `FEEDS` | Instance | `Map<String, FeedData>` | Latest aggregated price per asset |
| `(asset, publisher)` | Temporary | `PriceEntry` | Per-publisher raw submission, TTL ~7 days |

**Instance storage** persists indefinitely (subject to rent). **Temporary storage** expires after the configured TTL — publisher entries are extended to ~7 days on each submission.

## Staleness

Consumers can call `get_price_fresh(asset, max_age_secs)` to get a price only if it was updated within the given window. If the feed is stale, the call panics. This lets consumer contracts enforce their own freshness requirements.

## Consumer Integration

From another Soroban contract, invoke the oracle via cross-contract call:

```rust
let feed: FeedData = env.invoke_contract(
    &oracle_address,
    &symbol_short!("get_price"),
    vec![&env, asset.into_val(&env)],
);
```

From a TypeScript dApp, use the `OracleConsumer` SDK class.

## SDK

- **`OraclePublisher`** — signs and submits price transactions.
- **`OracleConsumer`** — simulates read-only calls (no signing required).
- **`publisher-bot.ts`** — example bot fetching from CoinGecko, submitting every 60s.

## Trust Assumptions

- Publishers are trusted by the admin. A compromised publisher can submit wrong prices, but cannot move the median alone if there are ≥3 publishers.
- The admin key is privileged — it can add/remove publishers and transfer admin. Protect it accordingly (multisig recommended for production).
- Temporary storage entries expire — if a publisher goes offline for >7 days, their entry drops out of aggregation automatically.
