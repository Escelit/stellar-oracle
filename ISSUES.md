# GitHub Issues to Create
# Go to https://github.com/Escelit/stellar-oracle/issues/new and create each one.
# Label all of them: `wave-bounty`, `good first issue` or `enhancement` as noted.

---

## Issue 1 — Deploy oracle contract to Stellar Testnet
**Labels:** `wave-bounty`, `enhancement`
**Description:**
Deploy the oracle-core contract to Stellar Testnet and document the contract address.

Tasks:
- Run `stellar contract deploy` against testnet
- Fund a publisher account via Friendbot
- Call `initialize` and `add_publisher`
- Add the contract ID and RPC endpoint to `README.md`
- Verify `submit_price` and `get_price` work end-to-end

---

## Issue 2 — Add TWAP (time-weighted average price) aggregation
**Labels:** `wave-bounty`, `enhancement`
**Description:**
Implement TWAP alongside the existing median aggregation.

Tasks:
- Add a `PriceHistory` storage structure per asset (ring buffer of last N entries)
- Add `get_twap(asset, window_secs)` function that computes the time-weighted average
- Add tests covering: single entry, multiple entries, window larger than history

---

## Issue 3 — Publisher reputation scoring
**Labels:** `wave-bounty`, `enhancement`
**Description:**
Track publisher reliability on-chain and weight their prices accordingly.

Tasks:
- Track submission count and deviation-from-median per publisher
- Add `get_publisher_stats(address)` view function
- Optionally weight aggregation by reputation score
- Add tests

---

## Issue 4 — Add price deviation circuit breaker
**Labels:** `wave-bounty`, `enhancement`
**Description:**
Reject price submissions that deviate more than X% from the current median.

Tasks:
- Add configurable `max_deviation_bps` (basis points) to contract storage (admin-settable)
- Reject `submit_price` calls that exceed the threshold
- Emit an event on rejection
- Add tests for boundary conditions

---

## Issue 5 — React monitoring dashboard
**Labels:** `wave-bounty`, `enhancement`
**Description:**
Build a simple web dashboard to monitor oracle feeds.

Tasks:
- Show all tracked asset pairs with latest price, timestamp, and source count
- Show publisher list and their last submission time
- Auto-refresh every 30s using the consumer SDK
- Stack: React + Tailwind (or plain CSS)
- Lives in `dashboard/` directory

---

## Issue 6 — GitHub Actions CI pipeline
**Labels:** `wave-bounty`, `good first issue`
**Description:**
Set up CI to run tests on every PR.

Tasks:
- `.github/workflows/ci.yml`
- Job 1: `cargo test` for the Rust contract
- Job 2: `npm run build` for the TypeScript SDK
- Trigger on push and pull_request to main

---

## Issue 7 — Add `get_price` consumer example script
**Labels:** `wave-bounty`, `good first issue`
**Description:**
Write a simple example script showing how to read prices using the consumer SDK.

Tasks:
- `sdk/examples/read-prices.ts`
- Connects to testnet, reads all assets, prints prices as human-readable floats
- Add usage instructions to README

---

## Issue 8 — Support multiple base currencies (USD, BTC, XLM)
**Labels:** `wave-bounty`, `enhancement`
**Description:**
Currently asset pairs are free-form strings. Add validation and support for cross-pair derivation.

Tasks:
- Define a standard asset pair format (e.g. `BASE/QUOTE`)
- Add `get_cross_price(base, quote)` that derives price via a common intermediate (e.g. USD)
- Add tests
