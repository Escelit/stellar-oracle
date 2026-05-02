# GitHub Issues to Create
# Go to https://github.com/Escelit/stellar-oracle/issues/new
# Label all with: `wave-bounty`
# Add `good first issue` for beginner-friendly ones
# Add `enhancement`, `bug`, or `documentation` as appropriate

---

## Issue 1 — Deploy oracle contract to Stellar Testnet
**Labels:** `wave-bounty`, `enhancement`
Deploy oracle-core to Stellar Testnet, call `initialize` and `add_publisher`, verify `submit_price` and `get_price` work end-to-end. Add contract ID and RPC endpoint to README.

---

## Issue 2 — Add TWAP (time-weighted average price) aggregation
**Labels:** `wave-bounty`, `enhancement`
Add a `PriceHistory` ring buffer per asset. Implement `get_twap(asset, window_secs)` that computes the time-weighted average. Add tests for single entry, multiple entries, and window larger than history.

---

## Issue 3 — Publisher reputation scoring
**Labels:** `wave-bounty`, `enhancement`
Track submission count and deviation-from-median per publisher on-chain. Add `get_publisher_stats(address)` view function. Optionally weight aggregation by reputation score. Add tests.

---

## Issue 4 — Price deviation circuit breaker
**Labels:** `wave-bounty`, `enhancement`
Add configurable `max_deviation_bps` (basis points) settable by admin. Reject `submit_price` calls that exceed the threshold. Add tests for boundary conditions.

---

## Issue 5 — React monitoring dashboard
**Labels:** `wave-bounty`, `enhancement`
Build a web dashboard in `dashboard/` showing all asset pairs with latest price, timestamp, and source count. Auto-refresh every 30s using the consumer SDK. Stack: React + Tailwind.

---

## Issue 6 — GitHub Actions CI pipeline
**Labels:** `wave-bounty`, `good first issue`
Add `.github/workflows/ci.yml` running `cargo test` and `npm run build` on push and pull_request to main.

---

## Issue 7 — Add `read-prices` consumer example script
**Labels:** `wave-bounty`, `good first issue`
Write `sdk/examples/read-prices.ts` that connects to testnet, reads all assets, and prints prices as human-readable floats. Add usage instructions to README.

---

## Issue 8 — Support cross-pair price derivation
**Labels:** `wave-bounty`, `enhancement`
Add `get_cross_price(base, quote)` that derives price via a common intermediate (e.g. USD). Add tests.

---

## Issue 9 — Add `submit_price_batch` for multi-asset updates
**Labels:** `wave-bounty`, `enhancement`
Allow publishers to submit prices for multiple assets in a single transaction to reduce fees. Signature: `submit_price_batch(publisher, entries: Vec<(String, i128, u64)>)`. Add tests.

---

## Issue 10 — Emit contract events on price updates
**Labels:** `wave-bounty`, `enhancement`
Use `env.events().publish(...)` to emit a `price_updated` event on every successful aggregation. Include asset, new price, timestamp, and num_sources. Add event parsing to the consumer SDK.

---

## Issue 11 — Add publisher stake/bond mechanism
**Labels:** `wave-bounty`, `enhancement`
Require publishers to bond a minimum XLM amount on registration. Slash bond if publisher submits prices that deviate beyond threshold. Add `bond_publisher` and `slash_publisher` functions.

---

## Issue 12 — Write Soroban contract integration tests against testnet
**Labels:** `wave-bounty`, `enhancement`
Write end-to-end integration tests in `tests/integration/` that deploy the contract to a local sandbox, submit prices from multiple publishers, and assert correct median output.

---

## Issue 13 — Add price history query endpoint
**Labels:** `wave-bounty`, `enhancement`
Store the last N price updates per asset in temporary storage. Add `get_price_history(asset, limit)` returning a `Vec<PriceEntry>`. Add tests.

---

## Issue 14 — TypeScript consumer SDK: add `watchPrice` streaming method
**Labels:** `wave-bounty`, `enhancement`
Add `watchPrice(asset, intervalMs, callback)` to `OracleConsumer` that polls the contract and calls the callback whenever the price changes. Include an example script.

---

## Issue 15 — Add admin multi-sig support
**Labels:** `wave-bounty`, `enhancement`
Replace single admin address with a threshold multi-sig: store a list of admin addresses and a required threshold. Admin actions require `threshold` signatures. Add tests.

---

## Issue 16 — CLI tool for oracle management
**Labels:** `wave-bounty`, `enhancement`
Build `sdk/src/cli.ts` with commands: `deploy`, `initialize`, `add-publisher`, `remove-publisher`, `submit-price`, `get-price`. Use `commander` or `yargs`. Document in README.

---

## Issue 17 — Add Dockerfile for publisher bot
**Labels:** `wave-bounty`, `good first issue`
Write a `Dockerfile` and `docker-compose.yml` for running the publisher bot. Include environment variable documentation. Add to README.

---

## Issue 18 — Add price feed for Stellar DEX (SDEX) assets
**Labels:** `wave-bounty`, `enhancement`
Extend the publisher bot to fetch prices from Stellar's built-in DEX via Horizon API (`/order_book`) for native Stellar asset pairs (e.g. XLM/USDC). Submit alongside CoinGecko prices.

---

## Issue 19 — Add staleness alerting to publisher bot
**Labels:** `wave-bounty`, `enhancement`
Add a monitor mode to the publisher bot that reads `get_price_fresh` for each asset and logs/alerts (stdout or webhook) when any feed goes stale beyond a configurable threshold.

---

## Issue 20 — Write full API documentation
**Labels:** `wave-bounty`, `documentation`
Write `docs/API.md` documenting every contract function with parameter types, return types, error conditions, and usage examples. Include a quick-start guide.

---

## Issue 21 — Add `pause` / `unpause` emergency mechanism
**Labels:** `wave-bounty`, `enhancement`
Add admin-only `pause()` and `unpause()` functions. When paused, `submit_price` and `get_price` should panic with a clear error. Add tests.

---

## Issue 22 — Benchmark contract instruction usage
**Labels:** `wave-bounty`, `enhancement`
Use `soroban-sdk` test utilities to measure CPU instructions and memory usage for `submit_price` and `get_price` with 1, 5, and 10 publishers. Document results in `docs/benchmarks.md`.

---

## Issue 23 — Add Soroban contract upgrade mechanism
**Labels:** `wave-bounty`, `enhancement`
Implement `upgrade(new_wasm_hash)` admin function using `env.deployer().update_current_contract_wasm(...)`. Add a test that upgrades to a new version and verifies state is preserved.

---

## Issue 24 — Add price feed aggregator for multiple oracle sources
**Labels:** `wave-bounty`, `enhancement`
Extend the publisher bot to pull prices from multiple sources (CoinGecko, Binance public API, Kraken public API) and submit the median of those as a single publisher price. Reduces single-source dependency.

---

## Issue 25 — Add Grafana dashboard config for publisher bot metrics
**Labels:** `wave-bounty`, `enhancement`
Expose Prometheus metrics from the publisher bot (submission count, last price, error rate). Add a `grafana/` directory with a dashboard JSON config. Document setup in README.
