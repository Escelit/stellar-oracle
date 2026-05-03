# GitHub Issues to Create
# https://github.com/Escelit/stellar-oracle/issues/new
# Labels: one of: enhancement, bug, documentation, good first issue, testing

---

## Issue 1 — Deploy oracle contract to Stellar Testnet
**Labels:** `enhancement`

**Description:**
The oracle contract exists locally but has never been deployed to a live network. Without a testnet deployment, contributors cannot run end-to-end tests against a real Soroban RPC and the SDK examples in `sdk/examples/read-prices.ts` have no contract to point at.

**Acceptance Criteria:**
- [ ] Contract is compiled with `stellar contract build` and deployed to Stellar Testnet using `stellar contract deploy`
- [ ] `initialize(admin)` is called and the admin address is documented
- [ ] At least one publisher is added via `add_publisher`
- [ ] `submit_price` and `get_price` are verified to work end-to-end against the live testnet RPC
- [ ] `README.md` is updated with the deployed contract ID and the Testnet RPC endpoint (`https://soroban-testnet.stellar.org`)
- [ ] The network passphrase (`Test SDF Network ; September 2015`) is documented for SDK users

---

## Issue 2 — Add TWAP (time-weighted average price) aggregation
**Labels:** `enhancement`

**Description:**
The contract currently stores only the latest aggregated `FeedData` per asset. There is no price history, so consumers cannot compute a time-weighted average price (TWAP) to smooth out short-term volatility.

**Acceptance Criteria:**
- [ ] A `PriceHistory` ring buffer (fixed capacity, e.g. 20 entries) is added to temporary storage per asset, keyed separately from `FeedData`
- [ ] Each successful `aggregate` call appends a `(price, timestamp)` snapshot to the ring buffer
- [ ] A new contract function `get_twap(asset: String, window_secs: u64) -> i128` is added; it averages all snapshots whose timestamp falls within `[now - window_secs, now]`
- [ ] Test: single entry in window returns that entry's price
- [ ] Test: multiple entries in window returns their arithmetic mean (scaled i128)
- [ ] Test: window larger than all history returns mean of all entries
- [ ] Test: window of 0 panics or returns an error

---

## Issue 3 — Publisher reputation scoring
**Labels:** `enhancement`

**Description:**
All publishers are currently treated equally in aggregation. There is no on-chain record of how often a publisher submits or how far their prices deviate from the median, making it impossible to identify unreliable publishers.

**Acceptance Criteria:**
- [ ] A `PublisherStats` struct is added: `{ submission_count: u32, total_deviation_bps: u64 }` stored in instance storage keyed by publisher address
- [ ] `submit_price` increments `submission_count` and accumulates `|price - median| / median * 10_000` into `total_deviation_bps` after each aggregation
- [ ] A new view function `get_publisher_stats(publisher: Address) -> PublisherStats` is added
- [ ] Test: stats start at zero for a new publisher
- [ ] Test: submission count increments correctly after each `submit_price` call
- [ ] Test: deviation accumulates correctly across multiple submissions

---

## Issue 4 — Price deviation circuit breaker
**Labels:** `enhancement`

**Description:**
`set_max_deviation_bps` and `get_max_deviation_bps` are already implemented in `lib.rs`, and the circuit breaker logic runs inside `submit_price`. However, the feature is not documented and the existing tests only cover the happy path and a single rejection case.

**Acceptance Criteria:**
- [ ] Existing implementation is confirmed correct (no code changes required if tests pass)
- [ ] Test: deviation exactly equal to `max_deviation_bps` is **allowed** (boundary)
- [ ] Test: deviation of `max_deviation_bps + 1` is **rejected** with `"price deviation exceeded"`
- [ ] Test: setting `max_deviation_bps` to 0 disables the check (already tested — verify coverage)
- [ ] Test: first submission for an asset (no existing median) is always allowed regardless of `max_deviation_bps`
- [ ] `docs/API.md` (or inline doc comment) describes the basis-point unit and the 0 = disabled behaviour

---

## Issue 5 — React monitoring dashboard
**Labels:** `enhancement`

**Description:**
There is no visual interface for monitoring oracle feeds. A dashboard would let operators and users see all asset prices, their freshness, and source counts at a glance.

**Acceptance Criteria:**
- [ ] A `dashboard/` directory is created at the repo root with a React + Tailwind project (`npm create vite`)
- [ ] The dashboard uses `OracleConsumer` from `@stellar-oracle/sdk` to call `getAssets()` and `getPrice(asset)` for each asset
- [ ] Each asset is displayed in a card showing: asset pair, price (human-readable float via `toFloat`), timestamp (human-readable), and `numSources`
- [ ] Prices auto-refresh every 30 seconds
- [ ] Stale feeds (older than 5 minutes) are visually highlighted (e.g. red border)
- [ ] `dashboard/README.md` documents how to configure the contract ID and RPC URL via environment variables

---

## Issue 6 — Add `submit_price_batch` for multi-asset updates
**Labels:** `enhancement`

**Description:**
Publishers currently must submit one transaction per asset. For publishers tracking many pairs, this is expensive in fees and ledger operations. A batch function reduces this to a single transaction.

**Acceptance Criteria:**
- [ ] A new contract function is added: `submit_price_batch(publisher: Address, entries: Vec<(String, i128, u64)>)`
- [ ] The function calls the same auth check (`publisher.require_auth()`) and publisher whitelist check as `submit_price`
- [ ] The circuit breaker is applied per entry; if any entry exceeds `max_deviation_bps`, that entry is skipped (or the whole call panics — document the chosen behaviour)
- [ ] `aggregate` is called once per unique asset after all entries are stored
- [ ] Test: batch with two assets produces correct `FeedData` for both
- [ ] Test: unauthorized publisher is rejected for the entire batch
- [ ] `OraclePublisher` in the TypeScript SDK gains a `submitPriceBatch(entries: {asset, price}[])` method

---

## Issue 7 — Emit contract events on price updates
**Labels:** `enhancement`

**Description:**
`PriceUpdated` event emission via `env.events().publish(...)` is already implemented in `lib.rs` using the `#[contractevent]` macro. The TypeScript consumer SDK does not yet parse these events.

**Acceptance Criteria:**
- [ ] Existing event emission is confirmed correct (the `test_price_updated_event_emitted` test passes)
- [ ] `OracleConsumer` gains a method `getPriceEvents(asset: string, ledgerStart: number): Promise<PriceUpdatedEvent[]>` that fetches events from the RPC using `server.getEvents`
- [ ] A `PriceUpdatedEvent` TypeScript interface is exported from `sdk/src/types.ts`: `{ asset, price, timestamp, numSources }`
- [ ] Test in `sdk/__tests__/sdk.test.ts` covers event parsing with a mocked RPC response
- [ ] Example usage is added to `sdk/examples/read-prices.ts`

---

## Issue 8 — Add publisher stake/bond mechanism
**Labels:** `enhancement`

**Description:**
Publishers are currently added by admin with no economic stake. A bonding mechanism creates a financial disincentive for submitting manipulated prices.

**Acceptance Criteria:**
- [ ] A `bond_publisher(publisher: Address)` function is added; it transfers a configurable minimum XLM amount (set by admin via `set_min_bond(amount: i128)`) from the publisher to the contract using the Stellar native asset token interface
- [ ] `add_publisher` is updated to require a bond to be on record before the publisher is whitelisted, or `bond_publisher` itself adds the publisher
- [ ] A `slash_publisher(publisher: Address, amount: i128)` admin-only function transfers `amount` from the bond to the admin address
- [ ] `get_bond(publisher: Address) -> i128` returns the current bond balance
- [ ] Test: publisher with insufficient bond cannot be added
- [ ] Test: slashing reduces bond balance correctly
- [ ] Test: non-admin cannot call `slash_publisher`

---

## Issue 9 — Add price history query endpoint
**Labels:** `enhancement`

**Description:**
`get_price` returns only the latest aggregated value. Consumers that need to audit recent price movements or detect anomalies have no way to retrieve historical snapshots.

**Acceptance Criteria:**
- [ ] Each successful `aggregate` call appends a `PriceEntry { price, timestamp }` snapshot to a per-asset ring buffer in temporary storage (capacity configurable, default 50)
- [ ] A new function `get_price_history(asset: String, limit: u32) -> Vec<PriceEntry>` returns the most recent `limit` entries, newest first
- [ ] Test: history is empty before any submission
- [ ] Test: history contains correct entries after multiple submissions
- [ ] Test: `limit` larger than history length returns all available entries
- [ ] Test: ring buffer wraps correctly when capacity is exceeded

---

## Issue 10 — TypeScript consumer SDK: add `watchPrice` streaming method
**Labels:** `enhancement`

**Description:**
`OracleConsumer` only supports one-shot reads. Applications that need to react to price changes must implement their own polling loop. A `watchPrice` method encapsulates this pattern.

**Acceptance Criteria:**
- [ ] `OracleConsumer` gains `watchPrice(asset: string, intervalMs: number, callback: (feed: FeedData) => void): () => void`
- [ ] The method polls `getPrice(asset)` every `intervalMs` milliseconds and calls `callback` only when the price or timestamp has changed since the last poll
- [ ] The return value is a stop function; calling it cancels the polling interval
- [ ] Test: callback is called when price changes, not called when price is unchanged
- [ ] Test: stop function cancels further callbacks
- [ ] `sdk/examples/watch-price.ts` demonstrates usage

---

## Issue 11 — Add admin multi-sig support
**Labels:** `enhancement`

**Description:**
The contract has a single admin address. If that key is lost or compromised, the oracle cannot be managed. Multi-sig requires multiple admins to agree before sensitive operations execute.

**Acceptance Criteria:**
- [ ] `initialize` is updated to accept `admins: Vec<Address>` and `threshold: u32` instead of a single `admin`
- [ ] A `Proposal` struct is stored per pending admin action: `{ action, approvals: Vec<Address> }`
- [ ] Admin-only functions (`add_publisher`, `remove_publisher`, `set_max_deviation_bps`) require `threshold` distinct admin signatures before executing
- [ ] `propose_action(action)` creates a proposal; `approve_action(proposal_id)` adds a signature; execution happens automatically when threshold is reached
- [ ] Test: action executes when exactly `threshold` admins approve
- [ ] Test: action does not execute with `threshold - 1` approvals
- [ ] Test: the same admin approving twice does not count twice

---

## Issue 12 — CLI tool for oracle management
**Labels:** `enhancement`

**Description:**
All contract interactions currently require writing custom scripts. A CLI tool lowers the barrier for operators to manage the oracle without writing code.

**Acceptance Criteria:**
- [ ] `sdk/src/cli.ts` is created using a CLI library (e.g. `commander`)
- [ ] Commands implemented: `deploy`, `initialize <admin>`, `add-publisher <address>`, `remove-publisher <address>`, `submit-price <asset> <price>`, `get-price <asset>`
- [ ] All commands accept `--contract-id`, `--rpc-url`, and `--network-passphrase` flags (or read from env vars `ORACLE_CONTRACT_ID`, `ORACLE_RPC_URL`, `ORACLE_NETWORK`)
- [ ] `submit-price` and mutating commands accept `--secret-key` for signing
- [ ] `package.json` adds a `"bin": { "stellar-oracle": "./dist/cli.js" }` entry
- [ ] `README.md` documents all commands with examples

---

## Issue 13 — Add Dockerfile for publisher bot
**Labels:** `good first issue`

**Description:**
The publisher bot in `sdk/src/publisher-bot.ts` runs as a Node.js process but has no container packaging. A Dockerfile makes it easy to deploy in any environment without managing Node.js versions.

**Acceptance Criteria:**
- [ ] `Dockerfile` at repo root builds the TypeScript SDK and runs the publisher bot; uses a multi-stage build (build stage: `node:20-alpine` + `npm ci` + `tsc`; runtime stage: `node:20-alpine`)
- [ ] `docker-compose.yml` defines a `publisher-bot` service with all required environment variables as placeholders
- [ ] `README.md` or a new `docs/publisher-guide.md` documents all environment variables: `ORACLE_CONTRACT_ID`, `ORACLE_RPC_URL`, `ORACLE_NETWORK`, `PUBLISHER_SECRET_KEY`, `ASSETS` (comma-separated), `INTERVAL_MS`
- [ ] Running `docker compose up` starts the bot without errors (given valid env vars)

---

## Issue 14 — Add price feed from Stellar DEX (SDEX)
**Labels:** `enhancement`

**Description:**
The publisher bot currently has no price source implementation. Stellar's own DEX (SDEX) is a natural first source for native Stellar asset pairs like XLM/USDC.

**Acceptance Criteria:**
- [ ] A `fetchSdexPrice(base: Asset, quote: Asset): Promise<number>` function is added to the publisher bot, fetching the mid-price from Horizon's `/order_book?selling_asset_type=...&buying_asset_type=...`
- [ ] XLM/USDC is supported as a default pair using the canonical USDC issuer on testnet
- [ ] The fetched price is submitted via `OraclePublisher.submitPrice`
- [ ] Errors from Horizon (network failure, empty order book) are caught and logged without crashing the bot
- [ ] Test: mock Horizon response returns correct mid-price calculation

---

## Issue 15 — Add staleness alerting to publisher bot
**Labels:** `enhancement`

**Description:**
There is no monitoring for stale oracle feeds. If a publisher bot stops submitting, consumers silently receive outdated prices until they call `get_price_fresh` and get a panic.

**Acceptance Criteria:**
- [ ] A `--monitor` flag is added to the publisher bot that runs a separate check loop (configurable interval, default 60s)
- [ ] The monitor calls `OracleConsumer.getPriceFresh(asset, maxAgeSecs)` for each configured asset
- [ ] If a feed is stale, a `WARN` log line is emitted: `[STALE] asset=XLM/USD age=<seconds>s threshold=<maxAgeSecs>s`
- [ ] `maxAgeSecs` is configurable via `--stale-threshold` flag or `STALE_THRESHOLD_SECS` env var (default: 120)
- [ ] Test: mock consumer returning a stale feed triggers the warning log

---

## Issue 16 — Write full API documentation
**Labels:** `documentation`

**Description:**
There is no reference documentation for the contract's public interface. Contributors and integrators must read `lib.rs` to understand function signatures, error conditions, and storage behaviour.

**Acceptance Criteria:**
- [ ] `docs/API.md` is created documenting every public contract function: `initialize`, `set_admin`, `set_max_deviation_bps`, `get_max_deviation_bps`, `add_publisher`, `remove_publisher`, `get_publishers`, `get_publisher_count`, `submit_price`, `get_price`, `get_price_fresh`, `get_assets`
- [ ] Each entry includes: function signature (Rust), parameters with types and units, return type, error conditions (panic messages), and a usage example
- [ ] Price scaling (1e7) is explained with a worked example
- [ ] The `PriceUpdated` event is documented with its fields
- [ ] `OracleError` enum values are listed with their meanings

---

## Issue 17 — Add `pause` / `unpause` emergency mechanism
**Labels:** `enhancement`

**Description:**
There is no way to halt the oracle in an emergency (e.g. a price manipulation attack). An admin-controlled pause prevents further price submissions and reads until the situation is resolved.

**Acceptance Criteria:**
- [ ] A `PAUSED` storage key (bool) is added to instance storage
- [ ] `pause(env: Env)` and `unpause(env: Env)` are admin-only functions that set/clear the flag
- [ ] `is_paused(env: Env) -> bool` view function is added
- [ ] `submit_price` panics with `"contract is paused"` when paused
- [ ] `get_price` and `get_price_fresh` panic with `"contract is paused"` when paused
- [ ] Test: `submit_price` is rejected while paused
- [ ] Test: `get_price` is rejected while paused
- [ ] Test: operations succeed after `unpause`
- [ ] Test: non-admin cannot call `pause` or `unpause`

---

## Issue 18 — Benchmark contract instruction usage
**Labels:** `testing`

**Description:**
Soroban contracts have CPU instruction and memory limits. There are no benchmarks to confirm the oracle stays within limits as publisher count grows.

**Acceptance Criteria:**
- [ ] A benchmark test file `contracts/oracle-core/src/bench.rs` (or a separate script) measures CPU instructions and memory bytes for `submit_price` and `get_price` with 1, 5, and 10 publishers using `env.budget()`
- [ ] Results are recorded in `docs/benchmarks.md` as a table: `| publishers | submit_price CPU | submit_price mem | get_price CPU | get_price mem |`
- [ ] The document notes the Soroban mainnet limits for reference
- [ ] CI does **not** fail on benchmark numbers (benchmarks are informational only)

---

## Issue 19 — Add Soroban contract upgrade mechanism
**Labels:** `enhancement`

**Description:**
The contract has no upgrade path. Fixing bugs or adding features requires deploying a new contract ID and migrating all publishers and consumers.

**Acceptance Criteria:**
- [ ] An `upgrade(env: Env, new_wasm_hash: BytesN<32>)` admin-only function is added, calling `env.deployer().update_current_contract_wasm(new_wasm_hash)`
- [ ] Test: after upgrade, instance storage (publishers list, feeds map) is preserved
- [ ] Test: non-admin cannot call `upgrade`
- [ ] `docs/API.md` documents the upgrade function and the WASM hash source

---

## Issue 20 — Add multi-source aggregation to publisher bot
**Labels:** `enhancement`

**Description:**
The publisher bot has no price source implementation. Fetching from multiple exchanges and submitting their median makes the oracle more robust against single-source failures or manipulation.

**Acceptance Criteria:**
- [ ] Three price fetchers are implemented: `fetchCoinGecko(asset)`, `fetchBinance(asset)`, `fetchKraken(asset)` — each returning `Promise<number | null>` (null on failure)
- [ ] The bot collects all non-null prices, computes the median, and submits it via `OraclePublisher.submitPrice`
- [ ] If fewer than 2 sources return a price, the submission is skipped and a `WARN` is logged
- [ ] Asset symbol mapping (e.g. `"XLM/USD"` → CoinGecko id `"stellar"`) is configurable
- [ ] Test: median of three prices is computed correctly; one failing source is handled gracefully

---

## Issue 21 — Add Prometheus metrics to publisher bot
**Labels:** `enhancement`

**Description:**
The publisher bot has no observability. Operators cannot tell how many submissions have succeeded, what the last submitted price was, or how often errors occur without reading raw logs.

**Acceptance Criteria:**
- [ ] A Prometheus HTTP metrics endpoint is exposed (default port 9090, configurable via `METRICS_PORT`)
- [ ] Three metrics are exported: `oracle_submissions_total{asset, status}` (counter), `oracle_last_price{asset}` (gauge, human-readable float), `oracle_errors_total{asset, source}` (counter)
- [ ] The endpoint is started alongside the submission loop; it does not block submissions
- [ ] `README.md` or `docs/publisher-guide.md` documents the metrics endpoint and metric names

---

## Issue 22 — Add Grafana dashboard config
**Labels:** `good first issue`

**Description:**
There is no pre-built Grafana dashboard for visualising the Prometheus metrics exported by the publisher bot (Issue 21).

**Acceptance Criteria:**
- [ ] `grafana/dashboard.json` is added containing a Grafana dashboard with panels for: submission rate per asset, last price per asset, and error rate per asset
- [ ] The dashboard uses the Prometheus data source variable `${DS_PROMETHEUS}`
- [ ] `README.md` documents how to import the dashboard and connect it to the Prometheus metrics endpoint
- [ ] Depends on Issue 21 being merged first (or the dashboard can be created independently with placeholder metric names)

---

## Issue 23 — Write integration tests against local sandbox
**Labels:** `testing`

**Description:**
All existing tests use the Soroban `Env` mock. There are no end-to-end tests that deploy the contract to a local sandbox and exercise the full transaction lifecycle.

**Acceptance Criteria:**
- [ ] A `tests/integration/` directory is created with a test script (TypeScript or shell) that: starts a local Stellar sandbox (`stellar network start local`), deploys the contract, initialises it, adds two publishers, submits prices from both, and asserts the correct median is returned by `get_price`
- [ ] Tests cover at least: single publisher, two publishers (even median), and `get_price_fresh` staleness rejection
- [ ] A `Makefile` target `make integration-test` runs the full suite
- [ ] CI workflow runs integration tests on pull requests (can be skipped with a label `skip-integration`)

---

## Issue 24 — Add cross-pair price derivation
**Labels:** `enhancement`

**Description:**
The contract stores prices per asset pair (e.g. `XLM/USD`, `BTC/USD`) but cannot derive `BTC/XLM` without a direct publisher. Cross-pair derivation via a common intermediate (USD) enables this.

**Acceptance Criteria:**
- [ ] A new function `get_cross_price(env: Env, base: String, quote: String) -> i128` is added
- [ ] It reads `get_price(base/USD)` and `get_price(quote/USD)` and returns `base_usd_price / quote_usd_price` (scaled i128)
- [ ] Panics with a descriptive message if either leg is not found
- [ ] Test: `get_cross_price("BTC", "XLM")` with known BTC/USD and XLM/USD feeds returns the correct derived price
- [ ] Test: missing leg panics with `"feed not found"`

---

## Issue 25 — Add `read-prices` consumer example script
**Labels:** `good first issue`

**Description:**
`sdk/examples/read-prices.ts` exists as a file but its content needs to be verified and completed. New contributors need a working example to understand how to use `OracleConsumer`.

**Acceptance Criteria:**
- [ ] `sdk/examples/read-prices.ts` connects to Testnet using `OracleConsumer` with the deployed contract ID from Issue 1
- [ ] It calls `getAssets()` and then `getPrice(asset)` for each asset
- [ ] Prices are printed as human-readable floats using `toFloat()` from `sdk/src/types.ts`
- [ ] The script handles the case where no assets are tracked yet (prints a helpful message)
- [ ] `README.md` shows how to run it: `npx ts-node sdk/examples/read-prices.ts`

---

## Issue 26 — Add weighted median aggregation
**Labels:** `enhancement`

**Description:**
The current median treats all publishers equally. Publishers with higher reputation scores (Issue 3) should have more influence on the aggregated price.

**Acceptance Criteria:**
- [ ] Depends on Issue 3 (publisher reputation scoring)
- [ ] A new function `get_weighted_price(env: Env, asset: String) -> i128` is added
- [ ] Each publisher's price is weighted by `1 / (1 + avg_deviation_bps)` derived from their `PublisherStats`
- [ ] The weighted median algorithm is documented in a code comment
- [ ] Test: publisher with zero deviation has higher weight than one with high deviation
- [ ] Test: result with equal weights matches the unweighted median

---

## Issue 27 — Add asset pair validation
**Labels:** `enhancement`

**Description:**
`submit_price` accepts any string as an asset pair. Malformed pairs (e.g. empty string, missing `/`, excessively long symbols) can pollute the feeds map and confuse consumers.

**Acceptance Criteria:**
- [ ] `submit_price` validates the asset string before storing: must contain exactly one `/`, each side must be 1–12 alphanumeric characters
- [ ] Panics with `"invalid asset format"` on validation failure
- [ ] Test: `"XLM/USD"` is accepted
- [ ] Test: `"XLMUSD"` (no slash) is rejected
- [ ] Test: `""` (empty) is rejected
- [ ] Test: `"TOOLONGBASE/USD"` (base > 12 chars) is rejected
- [ ] Test: `"XLM/"` (empty quote) is rejected

---

## Issue 28 — Add `get_price_at(asset, timestamp)` historical lookup
**Labels:** `enhancement`

**Description:**
There is no way to query what the oracle price was at a specific point in time. Historical lookup is needed for settlement, auditing, and backtesting.

**Acceptance Criteria:**
- [ ] Depends on Issue 9 (price history storage)
- [ ] A new function `get_price_at(env: Env, asset: String, target_ts: u64) -> PriceEntry` is added
- [ ] It returns the snapshot whose timestamp is closest to `target_ts` (either direction)
- [ ] Panics with `"no history available"` if the ring buffer is empty
- [ ] Test: exact timestamp match returns that entry
- [ ] Test: timestamp between two entries returns the closer one
- [ ] Test: timestamp before all history returns the oldest entry

---

## Issue 29 — Add publisher whitelist expiry
**Labels:** `enhancement`

**Description:**
Publishers are currently permanent once added. There is no mechanism to automatically revoke a publisher's access after a set period, requiring manual admin intervention.

**Acceptance Criteria:**
- [ ] A `set_publisher_expiry(env: Env, publisher: Address, expiry_ts: u64)` admin-only function stores an expiry timestamp per publisher in instance storage
- [ ] `submit_price` checks the expiry: if `env.ledger().timestamp() >= expiry_ts`, it panics with `"publisher expired"`
- [ ] `get_publisher_expiry(env: Env, publisher: Address) -> Option<u64>` view function is added
- [ ] Test: publisher with no expiry can always submit
- [ ] Test: publisher with future expiry can submit
- [ ] Test: publisher with past expiry is rejected with `"publisher expired"`

---

## Issue 30 — Add fee collection for price reads
**Labels:** `enhancement`

**Description:**
The oracle currently provides price reads for free. A fee mechanism allows the oracle operator to sustain the service and incentivises quality data.

**Acceptance Criteria:**
- [ ] `set_fee(env: Env, fee_stroops: i128)` admin-only function stores the fee in instance storage (0 = free)
- [ ] `get_fee(env: Env) -> i128` view function is added
- [ ] `get_price` and `get_price_fresh` transfer `fee_stroops` from the caller to the contract's address using the native XLM token before returning data
- [ ] `withdraw_fees(env: Env, to: Address)` admin-only function transfers the contract's XLM balance to `to`
- [ ] Test: read with fee=0 succeeds without any token transfer
- [ ] Test: read with fee>0 transfers the correct amount
- [ ] Test: non-admin cannot call `withdraw_fees`

---

## Issue 31 — Add `get_deviation(asset)` function
**Labels:** `enhancement`

**Description:**
Consumers have no way to assess how much publishers disagree on a price. High deviation indicates an unreliable feed that consumers should treat with caution.

**Acceptance Criteria:**
- [ ] A new function `get_deviation(env: Env, asset: String) -> u32` is added, returning the standard deviation of current publisher prices for the asset in basis points relative to the median
- [ ] Returns 0 if there is only one source
- [ ] Panics with `"feed not found"` if the asset has no data
- [ ] Test: single publisher returns 0
- [ ] Test: two publishers with known prices returns the correct deviation
- [ ] Test: all publishers with identical prices returns 0

---

## Issue 32 — Add price feed confidence interval
**Labels:** `enhancement`

**Description:**
`FeedData` currently returns only the median price. Consumers cannot tell how wide the spread between publishers is without calling a separate function.

**Acceptance Criteria:**
- [ ] `FeedData` struct gains two new fields: `price_low: i128` (minimum publisher price) and `price_high: i128` (maximum publisher price)
- [ ] `aggregate` populates these fields from the collected publisher prices
- [ ] `get_price` and `get_price_fresh` return the updated struct
- [ ] TypeScript `FeedData` interface in `sdk/src/types.ts` is updated with `priceLow: bigint` and `priceHigh: bigint`
- [ ] Test: single publisher has `price_low == price_high == price`
- [ ] Test: three publishers with known prices have correct `price_low` and `price_high`

---

## Issue 33 — Add `remove_asset(asset)` admin function
**Labels:** `enhancement`

**Description:**
Assets accumulate in the feeds map indefinitely. There is no way to remove a deprecated or incorrectly named asset pair.

**Acceptance Criteria:**
- [ ] `remove_asset(env: Env, asset: String)` admin-only function removes the asset from the `FEEDS` map and deletes all per-publisher temporary storage entries for that asset
- [ ] Panics with `"feed not found"` if the asset does not exist
- [ ] Test: `get_price` panics with `"feed not found"` after removal
- [ ] Test: `get_assets` no longer includes the removed asset
- [ ] Test: non-admin cannot call `remove_asset`

---

## Issue 34 — Add `set_min_sources(asset, min)` requirement
**Labels:** `enhancement`

**Description:**
A feed with only one source is as reliable as a single publisher. Operators should be able to require a minimum number of sources before a feed is considered valid.

**Acceptance Criteria:**
- [ ] `set_min_sources(env: Env, asset: String, min: u32)` admin-only function stores the minimum per asset
- [ ] `get_min_sources(env: Env, asset: String) -> u32` view function returns the configured minimum (default 1)
- [ ] `get_price` panics with `"insufficient sources"` if `feed.num_sources < min_sources`
- [ ] `get_price_fresh` inherits the same check
- [ ] Test: feed with `num_sources >= min` is returned successfully
- [ ] Test: feed with `num_sources < min` panics with `"insufficient sources"`
- [ ] Test: default minimum of 1 allows single-publisher feeds

---

## Issue 35 — Add TypeScript SDK unit tests
**Labels:** `testing`

**Description:**
`sdk/__tests__/sdk.test.ts` exists but its coverage needs to be verified and expanded. `OraclePublisher` and `OracleConsumer` should be tested with mocked RPC responses.

**Acceptance Criteria:**
- [ ] `OracleConsumer.getPrice` is tested with a mocked `rpc.Server.simulateTransaction` response returning a valid `FeedData`
- [ ] `OracleConsumer.getPriceFresh` is tested for the stale case (mock returns old timestamp)
- [ ] `OracleConsumer.getAssets` is tested with a mocked response
- [ ] `OraclePublisher.submitPrice` happy path is tested with mocked `getAccount`, `prepareTransaction`, and `sendTransaction`
- [ ] `OraclePublisher.submitPrice` retry logic is tested: first two calls throw, third succeeds
- [ ] All tests pass with `npm test` in the `sdk/` directory

---

## Issue 36 — Add SDK npm publish workflow
**Labels:** `enhancement`

**Description:**
There is no automated process to publish the TypeScript SDK to npm. Releases require manual steps that are error-prone and undocumented.

**Acceptance Criteria:**
- [ ] `.github/workflows/publish.yml` is added; it triggers on push of a tag matching `sdk-v*`
- [ ] The workflow runs `npm ci`, `npm run build`, and `npm publish --access public` in the `sdk/` directory
- [ ] The npm token is read from `secrets.NPM_TOKEN`
- [ ] `sdk/package.json` has `"name": "@stellar-oracle/sdk"` and a `"build"` script that runs `tsc`
- [ ] The workflow is documented in `CONTRIBUTING.md`

---

## Issue 37 — Add contract ABI/spec export
**Labels:** `good first issue`

**Description:**
There is no machine-readable contract specification. Tooling (SDKs, dashboards, block explorers) that wants to interact with the contract must parse `lib.rs` manually.

**Acceptance Criteria:**
- [ ] `stellar contract inspect --wasm target/wasm32-unknown-unknown/release/oracle_core.wasm --output json > docs/contract-spec.json` is run and the output is committed
- [ ] A CI step in `.github/workflows/ci.yml` verifies that `docs/contract-spec.json` is up to date (re-runs the command and diffs the output)
- [ ] `README.md` links to `docs/contract-spec.json`

---

## Issue 38 — Add `get_all_feeds()` paginated endpoint
**Labels:** `enhancement`

**Description:**
`get_assets()` returns all asset keys but not their feed data. Dashboards must make N separate `get_price` calls. A paginated `get_all_feeds` reduces this to one call.

**Acceptance Criteria:**
- [ ] `get_all_feeds(env: Env, offset: u32, limit: u32) -> Vec<FeedData>` is added
- [ ] Returns up to `limit` feeds starting at `offset` (ordered by insertion order)
- [ ] Returns an empty vec if `offset >= total feeds`
- [ ] Test: `offset=0, limit=2` with 3 feeds returns the first 2
- [ ] Test: `offset=2, limit=10` with 3 feeds returns the last 1
- [ ] `OracleConsumer` gains `getAllFeeds(offset: number, limit: number): Promise<FeedData[]>`

---

## Issue 39 — Add price feed for wrapped tokens (wBTC, wETH)
**Labels:** `enhancement`

**Description:**
The publisher bot has no support for Stellar-wrapped tokens. wBTC and wETH trade on the Stellar DEX but their prices should track their Ethereum equivalents.

**Acceptance Criteria:**
- [ ] The publisher bot fetches BTC/USD and ETH/USD prices from CoinGecko (or the multi-source aggregator from Issue 20)
- [ ] It submits these as `wBTC/USD` and `wETH/USD` asset pairs
- [ ] A deviation alert is logged if the submitted price differs from the SDEX order book price by more than 2%
- [ ] Asset symbol mapping is documented in `docs/publisher-guide.md`

---

## Issue 40 — Add `transfer_admin` two-step ownership transfer
**Labels:** `enhancement`

**Description:**
`set_admin` immediately transfers admin rights to any address. If the wrong address is provided, admin access is permanently lost. A two-step transfer prevents this.

**Acceptance Criteria:**
- [ ] `propose_admin(env: Env, new_admin: Address)` admin-only function stores `new_admin` as a pending admin in instance storage
- [ ] `accept_admin(env: Env)` requires auth from the pending admin and then sets them as the active admin, clearing the pending state
- [ ] `set_admin` is removed or deprecated (panics with `"use propose_admin/accept_admin"`)
- [ ] Test: `accept_admin` called by the wrong address is rejected
- [ ] Test: after `accept_admin`, the new admin can call admin-only functions
- [ ] Test: the old admin cannot call admin-only functions after transfer

---

## Issue 41 — Add `get_publisher_count()` view function
**Labels:** `good first issue`

**Description:**
`get_publisher_count()` is already implemented in `lib.rs` and tested in `test.rs`. This issue tracks creating the GitHub issue for visibility and ensuring the function is documented.

**Acceptance Criteria:**
- [ ] `get_publisher_count() -> u32` exists in the contract and returns the length of the publishers list
- [ ] The function is documented in `docs/API.md`
- [ ] The existing test `test_get_publisher_count` covers: 0 publishers, 1 publisher, 2 publishers, after removal

---

## Issue 42 — Add `is_publisher(address) -> bool` view function
**Labels:** `good first issue`

**Description:**
Consumer contracts that want to verify a publisher's status must call `get_publishers()` and search the list. A direct `is_publisher` check is more efficient and readable.

**Acceptance Criteria:**
- [ ] `is_publisher(env: Env, publisher: Address) -> bool` is added to the contract
- [ ] Returns `true` if the address is in the publishers list, `false` otherwise
- [ ] Test: returns `false` for an address that was never added
- [ ] Test: returns `true` after `add_publisher`
- [ ] Test: returns `false` after `remove_publisher`
- [ ] Function is documented in `docs/API.md`

---

## Issue 43 — Add `get_admin()` view function
**Labels:** `good first issue`

**Description:**
There is no way to query the current admin address without reading contract storage directly. A view function makes this accessible to other contracts and off-chain tooling.

**Acceptance Criteria:**
- [ ] `get_admin(env: Env) -> Address` is added to the contract
- [ ] Returns the address stored under the `ADMIN` key
- [ ] Test: returns the address passed to `initialize`
- [ ] Test: returns the new address after `set_admin` (or `accept_admin` if Issue 40 is merged)
- [ ] Function is documented in `docs/API.md`

---

## Issue 44 — Add `get_asset_count()` view function
**Labels:** `good first issue`

**Description:**
There is no direct way to query how many asset pairs are tracked without fetching the full `get_assets()` list. A count function is cheaper for callers that only need the number.

**Acceptance Criteria:**
- [ ] `get_asset_count(env: Env) -> u32` is added, returning the number of keys in the `FEEDS` map
- [ ] Test: returns 0 before any price is submitted
- [ ] Test: returns 1 after one asset is submitted, 2 after two distinct assets
- [ ] Function is documented in `docs/API.md`

---

## Issue 45 — Add `is_paused()` view function
**Labels:** `good first issue`

**Description:**
Depends on Issue 17 (pause/unpause mechanism). Once the pause flag exists, consumers and operators need a way to check it without attempting a read that would panic.

**Acceptance Criteria:**
- [ ] Depends on Issue 17 being merged
- [ ] `is_paused(env: Env) -> bool` is added, returning the value of the `PAUSED` storage key (default `false`)
- [ ] Test: returns `false` before `pause()` is called
- [ ] Test: returns `true` after `pause()`
- [ ] Test: returns `false` after `unpause()`
- [ ] Function is documented in `docs/API.md`

---

## Issue 46 — Add `get_last_update(asset)` view function
**Labels:** `good first issue`

**Description:**
Consumers that want to check feed freshness without reading the full `FeedData` struct must call `get_price` and extract the timestamp. A dedicated function is cleaner.

**Acceptance Criteria:**
- [ ] `get_last_update(env: Env, asset: String) -> u64` is added, returning `feed.timestamp` for the given asset
- [ ] Panics with `"feed not found"` if the asset has no data
- [ ] Test: returns the correct timestamp after a price submission
- [ ] Test: returns the updated timestamp after a second submission with a newer timestamp
- [ ] Function is documented in `docs/API.md`

---

## Issue 47 — Add `get_min_sources(asset)` view function
**Labels:** `good first issue`

**Description:**
Depends on Issue 34 (`set_min_sources`). Once per-asset minimums are configurable, a view function is needed to read them.

**Acceptance Criteria:**
- [ ] Depends on Issue 34 being merged
- [ ] `get_min_sources(env: Env, asset: String) -> u32` is added, returning the configured minimum (default 1 if not set)
- [ ] Test: returns 1 for an asset with no explicit minimum set
- [ ] Test: returns the value set by `set_min_sources`
- [ ] Function is documented in `docs/API.md`

---

## Issue 48 — Add `get_publisher_expiry(publisher)` view function
**Labels:** `good first issue`

**Description:**
Depends on Issue 29 (publisher whitelist expiry). Once expiry timestamps are stored, a view function is needed to read them.

**Acceptance Criteria:**
- [ ] Depends on Issue 29 being merged
- [ ] `get_publisher_expiry(env: Env, publisher: Address) -> Option<u64>` is added
- [ ] Returns `None` if no expiry is set, `Some(ts)` otherwise
- [ ] Test: returns `None` for a publisher with no expiry set
- [ ] Test: returns `Some(ts)` after `set_publisher_expiry` is called
- [ ] Function is documented in `docs/API.md`

---

## Issue 49 — Add `get_max_deviation_bps()` view function
**Labels:** `good first issue`

**Description:**
`get_max_deviation_bps()` is already implemented in `lib.rs` and tested. This issue tracks documentation and ensuring the function is visible to contributors.

**Acceptance Criteria:**
- [ ] `get_max_deviation_bps() -> u32` exists and returns 0 when not set (circuit breaker disabled)
- [ ] The existing test `test_get_max_deviation_bps` covers: default value of 0, value after `set_max_deviation_bps`
- [ ] Function is documented in `docs/API.md` with a note that 0 means disabled

---

## Issue 50 — Add `get_fee()` view function
**Labels:** `good first issue`

**Description:**
Depends on Issue 30 (fee collection). Once fees are configurable, a view function is needed so consumers can check the fee before calling `get_price`.

**Acceptance Criteria:**
- [ ] Depends on Issue 30 being merged
- [ ] `get_fee(env: Env) -> i128` is added, returning the configured fee in stroops (default 0)
- [ ] Test: returns 0 before `set_fee` is called
- [ ] Test: returns the value set by `set_fee`
- [ ] Function is documented in `docs/API.md`

---

## Issue 51 — Add Rust clippy and fmt to CI
**Labels:** `good first issue`

**Description:**
The CI workflow in `.github/workflows/ci.yml` runs tests but does not enforce code style or lint rules. Clippy warnings and formatting inconsistencies can accumulate unnoticed.

**Acceptance Criteria:**
- [ ] `.github/workflows/ci.yml` gains two new steps after the build step: `cargo fmt --check` and `cargo clippy -- -D warnings`
- [ ] Both steps run in the repo root (workspace level)
- [ ] The existing code passes both checks without modifications (fix any pre-existing issues as part of this PR)
- [ ] `CONTRIBUTING.md` documents that contributors should run `cargo fmt` and `cargo clippy` before submitting a PR

---

## Issue 52 — Add ESLint to TypeScript SDK
**Labels:** `good first issue`

**Description:**
The TypeScript SDK has no linter. Code style inconsistencies and common mistakes go undetected until review.

**Acceptance Criteria:**
- [ ] `eslint` and `@typescript-eslint/eslint-plugin` and `@typescript-eslint/parser` are added as dev dependencies (pinned versions)
- [ ] `.eslintrc.json` is added to `sdk/` with `@typescript-eslint/recommended` rules
- [ ] `sdk/package.json` gains `"lint": "eslint src __tests__"` script
- [ ] All existing SDK files pass linting without errors
- [ ] `.github/workflows/ci.yml` runs `npm run lint` in the `sdk/` directory

---

## Issue 53 — Add `CHANGELOG.md`
**Labels:** `documentation`, `good first issue`

**Description:**
`CHANGELOG.md` exists in the repo but its content needs to follow the [Keep a Changelog](https://keepachangelog.com) format and document all features implemented so far.

**Acceptance Criteria:**
- [ ] `CHANGELOG.md` follows the Keep a Changelog format with an `[Unreleased]` section
- [ ] The `[Unreleased]` section documents all features currently in the codebase: contract functions, circuit breaker, events, TypeScript SDK classes, publisher bot
- [ ] `README.md` links to `CHANGELOG.md`

---

## Issue 54 — Add `CODE_OF_CONDUCT.md`
**Labels:** `documentation`, `good first issue`

**Description:**
`CODE_OF_CONDUCT.md` exists in the repo. This issue tracks verifying it follows the Contributor Covenant format and is properly linked from `README.md` and `CONTRIBUTING.md`.

**Acceptance Criteria:**
- [ ] `CODE_OF_CONDUCT.md` uses the [Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/) text
- [ ] A contact email for reporting violations is filled in (not left as a placeholder)
- [ ] `README.md` links to `CODE_OF_CONDUCT.md`
- [ ] `CONTRIBUTING.md` references the Code of Conduct

---

## Issue 55 — Add `SECURITY.md`
**Labels:** `documentation`, `good first issue`

**Description:**
`SECURITY.md` exists in the repo. This issue tracks verifying it contains a complete vulnerability disclosure process.

**Acceptance Criteria:**
- [ ] `SECURITY.md` describes: which versions are supported, how to report a vulnerability (email or private GitHub advisory), expected response time, and what reporters can expect (acknowledgement, fix timeline, credit)
- [ ] The file does not use placeholder text
- [ ] `README.md` links to `SECURITY.md`

---

## Issue 56 — Add issue and PR templates
**Labels:** `good first issue`

**Description:**
`.github/ISSUE_TEMPLATE/bug_report.md`, `feature_request.md`, and `.github/pull_request_template.md` exist in the repo. This issue tracks verifying they are complete and useful.

**Acceptance Criteria:**
- [ ] `bug_report.md` includes sections: Description, Steps to Reproduce, Expected Behaviour, Actual Behaviour, Environment (OS, Node/Rust version)
- [ ] `feature_request.md` includes sections: Problem, Proposed Solution, Alternatives Considered
- [ ] `pull_request_template.md` includes sections: Summary, Changes, Testing, Checklist (tests added, docs updated, clippy/lint passes)
- [ ] All templates are free of placeholder text

---

## Issue 57 — Add `docs/architecture.md`
**Labels:** `documentation`

**Description:**
`docs/architecture.md` exists but its content needs to be verified and completed. New contributors need a clear explanation of how the oracle works end-to-end.

**Acceptance Criteria:**
- [ ] `docs/architecture.md` covers: publisher model (whitelisting, auth), price submission flow, median aggregation algorithm, staleness model (TTL, `get_price_fresh`), storage layout (`ADMIN`, `FEEDS`, `PUBLISHERS`, `MAX_DEV`, per-publisher temporary entries), circuit breaker, events, and how consumer contracts integrate via cross-contract calls
- [ ] A diagram (ASCII or Mermaid) illustrates the data flow from publisher to consumer
- [ ] `README.md` links to `docs/architecture.md`

---

## Issue 58 — Add `docs/publisher-guide.md`
**Labels:** `documentation`

**Description:**
There is no guide for new publishers. Setting up the publisher bot requires reading multiple source files to understand the configuration.

**Acceptance Criteria:**
- [ ] `docs/publisher-guide.md` covers: prerequisites (Node.js 20+, a funded Stellar keypair), how to get whitelisted (contact admin), environment variable configuration, running the bot locally, running with Docker (Issue 13), monitoring submissions, and handling common errors
- [ ] All environment variables are listed with their types, defaults, and descriptions
- [ ] `README.md` links to `docs/publisher-guide.md`

---

## Issue 59 — Add `docs/consumer-guide.md`
**Labels:** `documentation`

**Description:**
There is no guide for dApp developers who want to integrate the oracle. They must read the SDK source to understand how to use `OracleConsumer`.

**Acceptance Criteria:**
- [ ] `docs/consumer-guide.md` covers: installing the SDK (`npm install @stellar-oracle/sdk`), creating an `OracleConsumer`, calling `getPrice` and `getPriceFresh`, handling staleness errors, using `toFloat` for display, and the `watchPrice` streaming method (Issue 10)
- [ ] A complete working code example is included
- [ ] `README.md` links to `docs/consumer-guide.md`

---

## Issue 60 — Add `docs/security.md`
**Labels:** `documentation`

**Description:**
There is no document explaining the oracle's security model. Integrators cannot assess the trust assumptions or attack surface without reading the contract source.

**Acceptance Criteria:**
- [ ] `docs/security.md` covers: trust assumptions (admin key, publisher whitelist), attack vectors (price manipulation by colluding publishers, stale data, circuit breaker bypass), mitigations (median aggregation, circuit breaker, staleness check, TTL expiry), and recommended consumer practices (use `get_price_fresh`, check `num_sources`)
- [ ] Known limitations are documented honestly (e.g. single admin key risk, no slashing without Issue 8)
- [ ] `README.md` links to `docs/security.md`

---

## Issue 61 — Add fuzz tests for median calculation
**Labels:** `testing`

**Description:**
The `median` function in `lib.rs` uses a custom insertion sort. Property-based tests can catch edge cases that hand-written tests miss.

**Acceptance Criteria:**
- [ ] `proptest` is added as a dev dependency in `contracts/oracle-core/Cargo.toml`
- [ ] A fuzz test verifies: median of a sorted input equals the middle element (odd length), median is always within `[min, max]` of the input, median of a single-element vec equals that element
- [ ] Tests run with `cargo test` (no separate fuzzing harness required)
- [ ] All fuzz tests pass

---

## Issue 62 — Add fuzz tests for `submit_price`
**Labels:** `testing`

**Description:**
`submit_price` involves multiple storage reads and writes. Property-based tests with random inputs can surface panics or incorrect aggregation that unit tests miss.

**Acceptance Criteria:**
- [ ] `proptest` is used to generate random `(price: i128, timestamp: u64)` pairs for 1–5 publishers
- [ ] The test verifies: no panic occurs for any valid input combination, `num_sources` equals the number of publishers that submitted, the aggregated price is within `[min_price, max_price]` of submitted prices
- [ ] Negative prices and zero prices are included in the input range
- [ ] All fuzz tests pass

---

## Issue 63 — Add test for publisher removal mid-aggregation
**Labels:** `testing`

**Description:**
When a publisher is removed, their temporary storage entry may still exist. The next aggregation should exclude their price.

**Acceptance Criteria:**
- [ ] Test setup: 3 publishers submit prices for `XLM/USD`; publisher 2 is then removed
- [ ] Publisher 1 submits a new price; the resulting `FeedData` has `num_sources = 2` (publishers 1 and 3)
- [ ] The median is computed from only publishers 1 and 3's prices
- [ ] Test is in `contracts/oracle-core/src/test.rs`

---

## Issue 64 — Add test for concurrent publisher submissions
**Labels:** `testing`

**Description:**
The median result should be the same regardless of the order in which publishers submit. This commutativity property is not currently tested.

**Acceptance Criteria:**
- [ ] Test: 3 publishers submit prices in order A→B→C; record the median
- [ ] Test: same 3 publishers submit in order C→A→B; median is identical
- [ ] Test: same 3 publishers submit in order B→C→A; median is identical
- [ ] All three orderings produce the same `FeedData.price`

---

## Issue 65 — Add test for max publishers limit
**Labels:** `testing`

**Description:**
There is no upper bound on the number of publishers. An unbounded list could cause the `aggregate` function to exceed Soroban's CPU instruction limit.

**Acceptance Criteria:**
- [ ] A `set_max_publishers(env: Env, max: u32)` admin-only function is added
- [ ] `get_max_publishers(env: Env) -> u32` view function is added (default: no limit / u32::MAX)
- [ ] `add_publisher` panics with `"max publishers reached"` when the limit is exceeded
- [ ] Test: adding publishers up to the limit succeeds
- [ ] Test: adding one beyond the limit panics with `"max publishers reached"`

---

## Issue 66 — Add test for zero-price submission
**Labels:** `testing`

**Description:**
`submit_price` accepts `i128` prices. A price of 0 is technically valid but likely indicates a bug in the publisher. The intended behaviour is not documented or tested.

**Acceptance Criteria:**
- [ ] A decision is made and documented in a code comment: either reject zero prices (panic with `"invalid price"`) or accept them
- [ ] If rejected: test that `submit_price` with `price=0` panics with `"invalid price"`
- [ ] If accepted: test that `get_price` returns 0 and `num_sources=1` correctly
- [ ] The chosen behaviour is documented in `docs/API.md`

---

## Issue 67 — Add test for negative price submission
**Labels:** `testing`

**Description:**
`i128` allows negative values. A negative price has no economic meaning for asset pairs but the contract does not reject it. The intended behaviour must be documented.

**Acceptance Criteria:**
- [ ] A decision is made and documented: either reject negative prices (panic with `"invalid price"`) or accept them (for synthetic/inverse instruments)
- [ ] If rejected: test that `submit_price` with `price=-1` panics with `"invalid price"`
- [ ] If accepted: test that negative prices are included in median calculation correctly
- [ ] The chosen behaviour is documented in `docs/API.md`

---

## Issue 68 — Add test for future timestamp submission
**Labels:** `testing`

**Description:**
A publisher could submit a price with a timestamp far in the future, causing `get_price_fresh` to return a stale error prematurely for other consumers.

**Acceptance Criteria:**
- [ ] A `set_max_future_drift_secs(env: Env, secs: u64)` admin-only function is added (default: 60 seconds)
- [ ] `submit_price` rejects timestamps where `timestamp > env.ledger().timestamp() + max_future_drift_secs`, panicking with `"timestamp too far in future"`
- [ ] Test: timestamp exactly at `now + max_future_drift_secs` is accepted
- [ ] Test: timestamp at `now + max_future_drift_secs + 1` is rejected
- [ ] Test: default drift of 60s is enforced without calling `set_max_future_drift_secs`

---

## Issue 69 — Add test for very old timestamp submission
**Labels:** `testing`

**Description:**
A publisher could submit a price with a very old timestamp, making the feed appear stale immediately after submission.

**Acceptance Criteria:**
- [ ] A `set_max_submission_age_secs(env: Env, secs: u64)` admin-only function is added (default: 300 seconds / 5 minutes)
- [ ] `submit_price` rejects timestamps where `env.ledger().timestamp() - timestamp > max_submission_age_secs`, panicking with `"timestamp too old"`
- [ ] Test: timestamp exactly at `now - max_submission_age_secs` is accepted
- [ ] Test: timestamp at `now - max_submission_age_secs - 1` is rejected
- [ ] Test: default age of 300s is enforced without calling `set_max_submission_age_secs`

---

## Issue 70 — Add `get_price` error handling to consumer SDK
**Labels:** `enhancement`

**Description:**
`OracleConsumer.getPrice()` currently throws a raw `Error` on simulation failure. Callers cannot distinguish between a missing feed, a stale price, and a network error without parsing the error message string.

**Acceptance Criteria:**
- [ ] An `OracleError` TypeScript enum/union is added to `sdk/src/types.ts`: `FeedNotFound | StalePrice | InsufficientSources | NetworkError`
- [ ] `getPrice` and `getPriceFresh` return `Promise<Result<FeedData, OracleError>>` instead of throwing (use a `{ ok: true, value }` / `{ ok: false, error }` discriminated union)
- [ ] The simulation error message is parsed to map known contract panic strings to the correct `OracleError` variant
- [ ] Test: simulation error containing `"feed not found"` maps to `FeedNotFound`
- [ ] Test: simulation error containing `"stale price"` maps to `StalePrice`
- [ ] Test: network failure maps to `NetworkError`

---

## Issue 71 — Add retry logic to publisher bot
**Labels:** `enhancement`

**Description:**
`OraclePublisher.submitPrice` already implements exponential backoff retry (max 3 attempts, 1s/2s delays) in `sdk/src/publisher.ts`. This issue tracks verifying the implementation, adding tests, and documenting the behaviour.

**Acceptance Criteria:**
- [ ] Existing retry logic in `publisher.ts` is confirmed correct (3 attempts, delays of 1s and 2s)
- [ ] Each retry attempt logs: `[submitPrice] attempt N failed, retrying in Xms: <error>`
- [ ] Test: mock `_submitOnce` fails twice then succeeds; verify 3 calls total and correct delays
- [ ] Test: mock `_submitOnce` fails all 3 times; verify the last error is thrown
- [ ] Retry behaviour is documented in `docs/publisher-guide.md`

---

## Issue 72 — Add health check endpoint to publisher bot
**Labels:** `enhancement`

**Description:**
There is no way to check the publisher bot's health from an external system (load balancer, Kubernetes liveness probe) without reading logs.

**Acceptance Criteria:**
- [ ] A minimal HTTP server is added to the publisher bot (using Node.js `http` module, no framework)
- [ ] `GET /health` returns `200 OK` with JSON: `{ status: "ok", assets: { [asset]: { lastSubmittedAt: <ISO string>, lastTxHash: <string> } } }`
- [ ] If the bot has not yet submitted for an asset, `lastSubmittedAt` is `null`
- [ ] Port is configurable via `HEALTH_PORT` env var (default: 8080)
- [ ] `docs/publisher-guide.md` documents the health endpoint

---

## Issue 73 — Add `--dry-run` flag to publisher bot
**Labels:** `good first issue`

**Description:**
There is no way to test the publisher bot's price fetching without sending real transactions. A dry-run mode is useful for verifying configuration and price sources.

**Acceptance Criteria:**
- [ ] A `--dry-run` CLI flag (or `DRY_RUN=true` env var) is added to the publisher bot
- [ ] When enabled, the bot fetches prices and logs what would be submitted: `[DRY RUN] would submit asset=XLM/USD price=0.1234 timestamp=<unix>`
- [ ] No transactions are sent in dry-run mode
- [ ] `docs/publisher-guide.md` documents the flag

---

## Issue 74 — Add `--once` flag to publisher bot
**Labels:** `good first issue`

**Description:**
The publisher bot runs in an infinite loop. For cron-based deployments, a single-shot mode is needed.

**Acceptance Criteria:**
- [ ] A `--once` CLI flag (or `RUN_ONCE=true` env var) is added
- [ ] When enabled, the bot submits prices for all configured assets once and exits with code 0 on success, non-zero on any error
- [ ] Compatible with `--dry-run` (logs what would be submitted and exits)
- [ ] `docs/publisher-guide.md` documents the flag with a cron example

---

## Issue 75 — Add Kubernetes deployment manifests
**Labels:** `enhancement`

**Description:**
There are no Kubernetes manifests for the publisher bot. Teams running on Kubernetes must write their own manifests from scratch.

**Acceptance Criteria:**
- [ ] `k8s/publisher-bot-deployment.yaml` defines a `Deployment` with 1 replica, resource limits (`cpu: 100m`, `memory: 128Mi`), and liveness probe pointing at `GET /health`
- [ ] `k8s/publisher-bot-configmap.yaml` defines a `ConfigMap` for non-secret env vars (`ORACLE_RPC_URL`, `ORACLE_NETWORK`, `ASSETS`, `INTERVAL_MS`)
- [ ] `k8s/publisher-bot-secret.yaml` defines a `Secret` template for `PUBLISHER_SECRET_KEY` and `ORACLE_CONTRACT_ID` (with placeholder values, not real secrets)
- [ ] `README.md` or `docs/publisher-guide.md` documents how to apply the manifests

---

## Issue 76 — Add GitHub Actions workflow for testnet deployment
**Labels:** `enhancement`

**Description:**
There is no automated deployment pipeline. Deploying a new contract version to testnet requires manual CLI steps.

**Acceptance Criteria:**
- [ ] `.github/workflows/deploy-testnet.yml` triggers on push to `main`
- [ ] The workflow: builds the contract with `stellar contract build`, deploys with `stellar contract deploy --network testnet`, calls `initialize` if the contract is new, and outputs the contract ID as a workflow output and a GitHub Actions summary
- [ ] The Stellar secret key for deployment is read from `secrets.STELLAR_SECRET_KEY`
- [ ] The deployed contract ID is stored as a GitHub Actions variable or written to a file committed back to the repo (document the chosen approach)
- [ ] The workflow is documented in `CONTRIBUTING.md`

---

## Issue 77 — Add `stellar.toml` with oracle metadata
**Labels:** `good first issue`

**Description:**
`stellar.toml` exists in the repo but its content needs to be verified and completed per [SEP-1](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0001.md).

**Acceptance Criteria:**
- [ ] `stellar.toml` includes: `NETWORK_PASSPHRASE`, `HORIZON_URL`, a `[[ORACLES]]` section with the deployed contract ID (from Issue 1), supported asset pairs, and publisher contact info
- [ ] The file is valid TOML and passes `stellar toml validate` (if available)
- [ ] `README.md` links to `stellar.toml`

---

## Issue 78 — Add price feed for stablecoins (USDC, USDT)
**Labels:** `enhancement`

**Description:**
USDC and USDT are widely used on Stellar but their prices are not tracked by the oracle. Since they are pegged to $1, a simple deviation alert is more useful than a full price feed.

**Acceptance Criteria:**
- [ ] The publisher bot submits `USDC/USD` and `USDT/USD` prices fetched from CoinGecko
- [ ] If the fetched price deviates more than 0.5% from 1.0, a `WARN` log is emitted: `[DEPEG ALERT] asset=USDC/USD price=0.994 deviation=0.6%`
- [ ] The submission still proceeds even if the alert fires (the oracle records the actual price)
- [ ] Test: price of 0.994 triggers the alert; price of 0.997 does not

---

## Issue 79 — Add `get_price` caching layer to consumer SDK
**Labels:** `enhancement`

**Description:**
`OracleConsumer.getPrice` makes an RPC simulation call on every invocation. Applications that call it frequently (e.g. every render) will hit rate limits and add unnecessary latency.

**Acceptance Criteria:**
- [ ] `OracleConsumer` constructor accepts an optional `cacheTtlMs: number` option (default: 0 = no cache)
- [ ] When `cacheTtlMs > 0`, `getPrice(asset)` returns the cached `FeedData` if it was fetched within the last `cacheTtlMs` milliseconds
- [ ] Cache is per-asset (a `Map<string, { data: FeedData, fetchedAt: number }>`)
- [ ] Test: second call within TTL returns cached value without making an RPC call
- [ ] Test: call after TTL expires makes a new RPC call
- [ ] `docs/consumer-guide.md` documents the caching option

---

## Issue 80 — Add example Soroban consumer contract
**Labels:** `enhancement`

**Description:**
There is no example showing how another Soroban contract can call the oracle via cross-contract invocation. This is the primary integration pattern for DeFi protocols.

**Acceptance Criteria:**
- [ ] `contracts/example-consumer/src/lib.rs` is created with a contract that has one function: `get_xlm_price(env: Env, oracle_id: Address) -> i128`
- [ ] The function invokes `oracle_id.invoke_contract::<FeedData>(&symbol_short!("get_price"), ...)` and returns the price
- [ ] `contracts/example-consumer/Cargo.toml` is added and the contract is included in the workspace `Cargo.toml`
- [ ] A test in `contracts/example-consumer/src/test.rs` deploys both contracts and verifies the cross-contract call returns the correct price
- [ ] `docs/consumer-guide.md` includes a section on cross-contract integration with a link to the example
