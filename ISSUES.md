# GitHub Issues to Create
# https://github.com/Escelit/stellar-oracle/issues/new
# Labels: wave-bounty + one of: enhancement, bug, documentation, good first issue, testing

---

## Issue 1 — Deploy oracle contract to Stellar Testnet
**Labels:** `wave-bounty`, `enhancement`
Deploy oracle-core to Stellar Testnet, call `initialize` and `add_publisher`, verify `submit_price` and `get_price` work end-to-end. Add contract ID and RPC endpoint to README.

---

## Issue 2 — Add TWAP (time-weighted average price) aggregation
**Labels:** `wave-bounty`, `enhancement`
Add a `PriceHistory` ring buffer per asset. Implement `get_twap(asset, window_secs)`. Add tests for single entry, multiple entries, and window larger than history.

---

## Issue 3 — Publisher reputation scoring
**Labels:** `wave-bounty`, `enhancement`
Track submission count and deviation-from-median per publisher on-chain. Add `get_publisher_stats(address)` view function. Add tests.

---

## Issue 4 — Price deviation circuit breaker
**Labels:** `wave-bounty`, `enhancement`
Add configurable `max_deviation_bps` settable by admin. Reject `submit_price` calls that exceed the threshold. Add tests for boundary conditions.

---

## Issue 5 — React monitoring dashboard
**Labels:** `wave-bounty`, `enhancement`
Build `dashboard/` showing all asset pairs with latest price, timestamp, and source count. Auto-refresh every 30s using the consumer SDK. Stack: React + Tailwind.

---

## Issue 6 — Add `submit_price_batch` for multi-asset updates
**Labels:** `wave-bounty`, `enhancement`
Allow publishers to submit prices for multiple assets in a single transaction. Signature: `submit_price_batch(publisher, entries: Vec<(String, i128, u64)>)`. Add tests.

---

## Issue 7 — Emit contract events on price updates
**Labels:** `wave-bounty`, `enhancement`
Use `env.events().publish(...)` to emit a `price_updated` event on every successful aggregation. Include asset, new price, timestamp, num_sources. Add event parsing to consumer SDK.

---

## Issue 8 — Add publisher stake/bond mechanism
**Labels:** `wave-bounty`, `enhancement`
Require publishers to bond a minimum XLM amount on registration. Slash bond if publisher submits prices that deviate beyond threshold. Add `bond_publisher` and `slash_publisher` functions.

---

## Issue 9 — Add price history query endpoint
**Labels:** `wave-bounty`, `enhancement`
Store the last N price updates per asset in temporary storage. Add `get_price_history(asset, limit)` returning `Vec<PriceEntry>`. Add tests.

---

## Issue 10 — TypeScript consumer SDK: add `watchPrice` streaming method
**Labels:** `wave-bounty`, `enhancement`
Add `watchPrice(asset, intervalMs, callback)` to `OracleConsumer` that polls and calls callback on price change. Include example script.

---

## Issue 11 — Add admin multi-sig support
**Labels:** `wave-bounty`, `enhancement`
Replace single admin with threshold multi-sig: store list of admin addresses and required threshold. Admin actions require `threshold` signatures. Add tests.

---

## Issue 12 — CLI tool for oracle management
**Labels:** `wave-bounty`, `enhancement`
Build `sdk/src/cli.ts` with commands: `deploy`, `initialize`, `add-publisher`, `remove-publisher`, `submit-price`, `get-price`. Document in README.

---

## Issue 13 — Add Dockerfile for publisher bot
**Labels:** `wave-bounty`, `good first issue`
Write `Dockerfile` and `docker-compose.yml` for running the publisher bot. Include env var documentation.

---

## Issue 14 — Add price feed from Stellar DEX (SDEX)
**Labels:** `wave-bounty`, `enhancement`
Extend publisher bot to fetch prices from Horizon `/order_book` for native Stellar asset pairs (e.g. XLM/USDC).

---

## Issue 15 — Add staleness alerting to publisher bot
**Labels:** `wave-bounty`, `enhancement`
Add monitor mode that reads `get_price_fresh` for each asset and logs/alerts when any feed goes stale beyond a configurable threshold.

---

## Issue 16 — Write full API documentation
**Labels:** `wave-bounty`, `documentation`
Write `docs/API.md` documenting every contract function with parameter types, return types, error conditions, and usage examples.

---

## Issue 17 — Add `pause` / `unpause` emergency mechanism
**Labels:** `wave-bounty`, `enhancement`
Add admin-only `pause()` and `unpause()`. When paused, `submit_price` and `get_price` panic with a clear error. Add tests.

---

## Issue 18 — Benchmark contract instruction usage
**Labels:** `wave-bounty`, `testing`
Measure CPU instructions and memory for `submit_price` and `get_price` with 1, 5, and 10 publishers. Document in `docs/benchmarks.md`.

---

## Issue 19 — Add Soroban contract upgrade mechanism
**Labels:** `wave-bounty`, `enhancement`
Implement `upgrade(new_wasm_hash)` using `env.deployer().update_current_contract_wasm(...)`. Add test that upgrades and verifies state is preserved.

---

## Issue 20 — Add multi-source aggregation to publisher bot
**Labels:** `wave-bounty`, `enhancement`
Pull prices from CoinGecko, Binance public API, and Kraken public API. Submit the median of those as a single publisher price.

---

## Issue 21 — Add Prometheus metrics to publisher bot
**Labels:** `wave-bounty`, `enhancement`
Expose submission count, last price, and error rate as Prometheus metrics from the publisher bot.

---

## Issue 22 — Add Grafana dashboard config
**Labels:** `wave-bounty`, `good first issue`
Add `grafana/dashboard.json` for visualizing oracle metrics. Document setup in README.

---

## Issue 23 — Write integration tests against local sandbox
**Labels:** `wave-bounty`, `testing`
Write end-to-end tests in `tests/integration/` that deploy to a local sandbox, submit prices from multiple publishers, and assert correct median output.

---

## Issue 24 — Add cross-pair price derivation
**Labels:** `wave-bounty`, `enhancement`
Add `get_cross_price(base, quote)` that derives price via a common intermediate (e.g. USD). Add tests.

---

## Issue 25 — Add `read-prices` consumer example script
**Labels:** `wave-bounty`, `good first issue`
Write `sdk/examples/read-prices.ts` that connects to testnet, reads all assets, and prints prices as human-readable floats.

---

## Issue 26 — Add weighted median aggregation
**Labels:** `wave-bounty`, `enhancement`
Implement weighted median where each publisher's price is weighted by their reputation score. Add `get_weighted_price(asset)`. Add tests comparing weighted vs unweighted results.

---

## Issue 27 — Add asset pair validation
**Labels:** `wave-bounty`, `enhancement`
Validate asset pair format on `submit_price` (must match `BASE/QUOTE` pattern, max 12 chars each). Return descriptive error on invalid input. Add tests.

---

## Issue 28 — Add `get_price_at(asset, timestamp)` historical lookup
**Labels:** `wave-bounty`, `enhancement`
Store timestamped price snapshots and allow querying the price closest to a given timestamp. Add tests.

---

## Issue 29 — Add publisher whitelist expiry
**Labels:** `wave-bounty`, `enhancement`
Allow admin to set an expiry timestamp per publisher. Expired publishers cannot submit prices. Add `set_publisher_expiry(publisher, expiry_ts)` and enforce in `submit_price`. Add tests.

---

## Issue 30 — Add fee collection for price reads
**Labels:** `wave-bounty`, `enhancement`
Add optional fee (in XLM stroops) for reading prices from external contracts. Admin can set/withdraw fees. Add tests.

---

## Issue 31 — Add `get_deviation(asset)` function
**Labels:** `wave-bounty`, `enhancement`
Return the standard deviation of current publisher prices for an asset. Useful for consumers to assess feed reliability. Add tests.

---

## Issue 32 — Add price feed confidence interval
**Labels:** `wave-bounty`, `enhancement`
Return a confidence interval (min/max of publisher prices) alongside the median in `FeedData`. Update `get_price` return type. Add tests.

---

## Issue 33 — Add `remove_asset(asset)` admin function
**Labels:** `wave-bounty`, `enhancement`
Allow admin to remove a tracked asset pair and clean up its storage. Add tests ensuring removed assets return errors on read.

---

## Issue 34 — Add `set_min_sources(asset, min)` requirement
**Labels:** `wave-bounty`, `enhancement`
Allow admin to set minimum number of publisher sources required before a feed is considered valid. `get_price` panics if sources < min. Add tests.

---

## Issue 35 — Add TypeScript SDK unit tests
**Labels:** `wave-bounty`, `testing`
Add Jest tests for `OraclePublisher` and `OracleConsumer` using mocked RPC responses. Cover happy path, stale price, and error cases.

---

## Issue 36 — Add SDK npm publish workflow
**Labels:** `wave-bounty`, `enhancement`
Add `.github/workflows/publish.yml` that publishes `@stellar-oracle/sdk` to npm on version tag push.

---

## Issue 37 — Add contract ABI/spec export
**Labels:** `wave-bounty`, `good first issue`
Run `stellar contract inspect` to export the contract spec and commit it to `docs/contract-spec.json`. Add to CI.

---

## Issue 38 — Add `get_all_feeds()` paginated endpoint
**Labels:** `wave-bounty`, `enhancement`
Add `get_all_feeds(offset, limit)` to support pagination for dashboards with many asset pairs. Add tests.

---

## Issue 39 — Add price feed for wrapped tokens (wBTC, wETH)
**Labels:** `wave-bounty`, `enhancement`
Extend publisher bot to submit prices for Stellar-wrapped tokens using their Ethereum price from CoinGecko.

---

## Issue 40 — Add `transfer_admin` two-step ownership transfer
**Labels:** `wave-bounty`, `enhancement`
Replace direct `set_admin` with a two-step transfer: `propose_admin(new_admin)` + `accept_admin()`. Prevents accidental admin loss. Add tests.

---

## Issue 41 — Add `get_publisher_count()` view function
**Labels:** `wave-bounty`, `good first issue`
Add a simple `get_publisher_count() -> u32` function. Add test. Good entry point for new contributors.

---

## Issue 42 — Add `is_publisher(address) -> bool` view function
**Labels:** `wave-bounty`, `good first issue`
Add `is_publisher(address) -> bool` for easy on-chain checks by consumer contracts. Add test.

---

## Issue 43 — Add `get_admin()` view function
**Labels:** `wave-bounty`, `good first issue`
Add `get_admin() -> Address` view function. Add test.

---

## Issue 44 — Add `get_asset_count()` view function
**Labels:** `wave-bounty`, `good first issue`
Add `get_asset_count() -> u32`. Add test.

---

## Issue 45 — Add `is_paused()` view function
**Labels:** `wave-bounty`, `good first issue`
Add `is_paused() -> bool` view function (requires Issue 17 pause mechanism). Add test.

---

## Issue 46 — Add `get_last_update(asset)` view function
**Labels:** `wave-bounty`, `good first issue`
Add `get_last_update(asset) -> u64` returning the timestamp of the last price update. Add test.

---

## Issue 47 — Add `get_min_sources(asset)` view function
**Labels:** `wave-bounty`, `good first issue`
Add `get_min_sources(asset) -> u32` (requires Issue 34). Add test.

---

## Issue 48 — Add `get_publisher_expiry(publisher)` view function
**Labels:** `wave-bounty`, `good first issue`
Add `get_publisher_expiry(publisher) -> Option<u64>` (requires Issue 29). Add test.

---

## Issue 49 — Add `get_max_deviation_bps()` view function
**Labels:** `wave-bounty`, `good first issue`
Add `get_max_deviation_bps() -> u32` (requires Issue 4). Add test.

---

## Issue 50 — Add `get_fee()` view function
**Labels:** `wave-bounty`, `good first issue`
Add `get_fee() -> i128` (requires Issue 30). Add test.

---

## Issue 51 — Add Rust clippy and fmt to CI
**Labels:** `wave-bounty`, `good first issue`
Add `cargo clippy -- -D warnings` and `cargo fmt --check` steps to the CI workflow.

---

## Issue 52 — Add ESLint to TypeScript SDK
**Labels:** `wave-bounty`, `good first issue`
Add ESLint with `@typescript-eslint` to the SDK. Add `npm run lint` script and add to CI.

---

## Issue 53 — Add `CHANGELOG.md`
**Labels:** `wave-bounty`, `documentation`, `good first issue`
Create `CHANGELOG.md` following Keep a Changelog format. Document all features added so far under `[Unreleased]`.

---

## Issue 54 — Add `CODE_OF_CONDUCT.md`
**Labels:** `wave-bounty`, `documentation`, `good first issue`
Add a Contributor Covenant `CODE_OF_CONDUCT.md`. Link from README and CONTRIBUTING.md.

---

## Issue 55 — Add `SECURITY.md`
**Labels:** `wave-bounty`, `documentation`, `good first issue`
Add `SECURITY.md` describing how to responsibly disclose vulnerabilities.

---

## Issue 56 — Add issue and PR templates
**Labels:** `wave-bounty`, `good first issue`
Add `.github/ISSUE_TEMPLATE/bug_report.md`, `feature_request.md`, and `.github/pull_request_template.md`.

---

## Issue 57 — Add `docs/architecture.md`
**Labels:** `wave-bounty`, `documentation`
Write an architecture overview explaining the oracle design: publisher model, median aggregation, staleness, storage layout, and how consumer contracts integrate.

---

## Issue 58 — Add `docs/publisher-guide.md`
**Labels:** `wave-bounty`, `documentation`
Write a guide for new publishers: how to get whitelisted, run the publisher bot, monitor submissions, and handle errors.

---

## Issue 59 — Add `docs/consumer-guide.md`
**Labels:** `wave-bounty`, `documentation`
Write a guide for dApp developers integrating the oracle: how to read prices, handle staleness, and use the TypeScript SDK.

---

## Issue 60 — Add `docs/security.md`
**Labels:** `wave-bounty`, `documentation`
Document the security model: trust assumptions, attack vectors (price manipulation, publisher collusion, stale data), and mitigations.

---

## Issue 61 — Add fuzz tests for median calculation
**Labels:** `wave-bounty`, `testing`
Add property-based fuzz tests for the `median` function using `proptest`. Verify: median of sorted input equals middle element, median is always within min/max range.

---

## Issue 62 — Add fuzz tests for `submit_price`
**Labels:** `wave-bounty`, `testing`
Use `proptest` to fuzz `submit_price` with random prices, timestamps, and publisher counts. Verify no panics and correct aggregation.

---

## Issue 63 — Add test for publisher removal mid-aggregation
**Labels:** `wave-bounty`, `testing`
Test that removing a publisher mid-cycle correctly re-aggregates remaining publisher prices on next submission. Verify stale entries are excluded.

---

## Issue 64 — Add test for concurrent publisher submissions
**Labels:** `wave-bounty`, `testing`
Test that multiple publishers submitting in different orders always produce the same median result. Verify commutativity.

---

## Issue 65 — Add test for max publishers limit
**Labels:** `wave-bounty`, `testing`
Add a configurable `max_publishers` limit. Test that adding beyond the limit fails gracefully. Add `get_max_publishers()` view function.

---

## Issue 66 — Add test for zero-price submission
**Labels:** `wave-bounty`, `testing`
Verify that submitting a price of 0 is either rejected or handled correctly. Document the intended behavior and add tests.

---

## Issue 67 — Add test for negative price submission
**Labels:** `wave-bounty`, `testing`
Verify that submitting a negative price is either rejected or handled correctly (i128 allows negatives). Document intended behavior and add tests.

---

## Issue 68 — Add test for future timestamp submission
**Labels:** `wave-bounty`, `testing`
Verify that submitting a price with a timestamp far in the future is rejected or flagged. Add configurable `max_future_drift_secs`. Add tests.

---

## Issue 69 — Add test for very old timestamp submission
**Labels:** `wave-bounty`, `testing`
Verify that submitting a price with a very old timestamp is rejected. Add configurable `max_age_secs` for submissions. Add tests.

---

## Issue 70 — Add `get_price` error handling to consumer SDK
**Labels:** `wave-bounty`, `enhancement`
Improve `OracleConsumer.getPrice()` to return a typed `Result<FeedData, OracleError>` instead of throwing. Add tests.

---

## Issue 71 — Add retry logic to publisher bot
**Labels:** `wave-bounty`, `enhancement`
Add exponential backoff retry (max 3 attempts) to `submitPrice` in the publisher bot. Log each retry attempt with reason.

---

## Issue 72 — Add health check endpoint to publisher bot
**Labels:** `wave-bounty`, `enhancement`
Add a simple HTTP health check server (`/health`) to the publisher bot that returns last submission time and status for each asset.

---

## Issue 73 — Add `--dry-run` flag to publisher bot
**Labels:** `wave-bounty`, `good first issue`
Add `--dry-run` CLI flag that fetches prices and logs what would be submitted without sending transactions.

---

## Issue 74 — Add `--once` flag to publisher bot
**Labels:** `wave-bounty`, `good first issue`
Add `--once` flag that submits prices once and exits (instead of looping). Useful for cron-based deployments.

---

## Issue 75 — Add Kubernetes deployment manifests
**Labels:** `wave-bounty`, `enhancement`
Add `k8s/` directory with Deployment, ConfigMap, and Secret manifests for running the publisher bot on Kubernetes.

---

## Issue 76 — Add GitHub Actions workflow for testnet deployment
**Labels:** `wave-bounty`, `enhancement`
Add `.github/workflows/deploy-testnet.yml` that builds and deploys the contract to testnet on push to `main`, storing the contract ID as a GitHub Actions output.

---

## Issue 77 — Add `stellar.toml` with oracle metadata
**Labels:** `wave-bounty`, `good first issue`
Add a `stellar.toml` (or link to one) describing the oracle network, supported asset pairs, and publisher contact info per SEP-1.

---

## Issue 78 — Add price feed for stablecoins (USDC, USDT)
**Labels:** `wave-bounty`, `enhancement`
Extend publisher bot to submit prices for USDC and USDT. Since they are pegged, implement a simple deviation alert if price moves more than 0.5% from $1.

---

## Issue 79 — Add `get_price` caching layer to consumer SDK
**Labels:** `wave-bounty`, `enhancement`
Add in-memory cache to `OracleConsumer` with configurable TTL. Return cached value if fresh, otherwise fetch from chain. Add tests.

---

## Issue 80 — Add example Soroban consumer contract
**Labels:** `wave-bounty`, `enhancement`
Write an example Soroban contract in `contracts/example-consumer/` that calls `get_price_fresh` from the oracle contract via cross-contract invocation. Add tests and documentation.
