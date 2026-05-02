#![no_std]

use soroban_sdk::{
    contract, contractevent, contractimpl, contracttype, symbol_short, Address, Env, Map, String,
    Symbol, Vec,
};

// ── Events ───────────────────────────────────────────────────────────────────

#[contractevent]
pub struct PriceUpdated {
    #[topic]
    pub asset: String,
    pub price: i128,
    pub timestamp: u64,
    pub num_sources: u32,
}

// ── Storage keys ────────────────────────────────────────────────────────────

const ADMIN: Symbol = symbol_short!("ADMIN");
const FEEDS: Symbol = symbol_short!("FEEDS");
const PUBLISHERS: Symbol = symbol_short!("PUBS");
const MAX_DEV: Symbol = symbol_short!("MAXDEV");

// TTL: ~7 days (ledger closes ~every 5s, 7*24*3600/5 = 120_960)
const ENTRY_TTL: u32 = 120_960;

// ── Data types ───────────────────────────────────────────────────────────────

/// A single price entry submitted by a publisher.
#[contracttype]
#[derive(Clone)]
pub struct PriceEntry {
    pub price: i128,    // price scaled by 1e7 (e.g. 1 XLM = 1_0000000)
    pub timestamp: u64, // Unix timestamp (seconds)
    pub publisher: Address,
}

/// Aggregated feed data stored on-chain.
#[contracttype]
#[derive(Clone)]
pub struct FeedData {
    pub asset: String,    // e.g. "XLM/USD"
    pub price: i128,      // median price, scaled by 1e7
    pub timestamp: u64,   // timestamp of latest update
    pub num_sources: u32, // how many publishers contributed
}

// ── Errors ───────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum OracleError {
    Unauthorized = 1,
    FeedNotFound = 2,
    StalePrice = 3,
    InsufficientSources = 4,
    PublisherAlreadyExists = 5,
    PublisherNotFound = 6,
    PriceDeviationExceeded = 7,
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    // ── Admin ────────────────────────────────────────────────────────────────

    /// Initialize the oracle with an admin address.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
        env.storage()
            .instance()
            .set(&PUBLISHERS, &Vec::<Address>::new(&env));
        env.storage()
            .instance()
            .set(&FEEDS, &Map::<String, FeedData>::new(&env));
    }

    /// Transfer admin role.
    pub fn set_admin(env: Env, new_admin: Address) {
        Self::require_admin(&env);
        env.storage().instance().set(&ADMIN, &new_admin);
    }

    /// Set maximum allowed deviation from current median in basis points (admin only).
    /// E.g. 1000 = 10%. Set to 0 to disable the circuit breaker.
    pub fn set_max_deviation_bps(env: Env, max_bps: u32) {
        Self::require_admin(&env);
        env.storage().instance().set(&MAX_DEV, &max_bps);
    }

    /// Get the configured max deviation in basis points (0 = disabled).
    pub fn get_max_deviation_bps(env: Env) -> u32 {
        env.storage().instance().get(&MAX_DEV).unwrap_or(0)
    }

    // ── Publisher management ─────────────────────────────────────────────────

    /// Add a trusted publisher (admin only).
    pub fn add_publisher(env: Env, publisher: Address) {
        Self::require_admin(&env);
        let mut pubs: Vec<Address> = env.storage().instance().get(&PUBLISHERS).unwrap();
        if pubs.contains(&publisher) {
            panic!("publisher already exists");
        }
        pubs.push_back(publisher);
        env.storage().instance().set(&PUBLISHERS, &pubs);
    }

    /// Remove a publisher (admin only).
    pub fn remove_publisher(env: Env, publisher: Address) {
        Self::require_admin(&env);
        let pubs: Vec<Address> = env.storage().instance().get(&PUBLISHERS).unwrap();
        let mut new_pubs = Vec::<Address>::new(&env);
        let mut found = false;
        for p in pubs.iter() {
            if p == publisher {
                found = true;
            } else {
                new_pubs.push_back(p);
            }
        }
        if !found {
            panic!("publisher not found");
        }
        env.storage().instance().set(&PUBLISHERS, &new_pubs);
    }

    /// List all publishers.
    pub fn get_publishers(env: Env) -> Vec<Address> {
        env.storage().instance().get(&PUBLISHERS).unwrap()
    }

    // ── Price submission ─────────────────────────────────────────────────────

    /// Submit a price update for an asset pair (publishers only).
    /// Rejected if the price deviates beyond `max_deviation_bps` from the current median.
    /// Prices from all publishers are aggregated via median.
    pub fn submit_price(env: Env, publisher: Address, asset: String, price: i128, timestamp: u64) {
        publisher.require_auth();
        Self::require_publisher(&env, &publisher);

        // Circuit breaker: reject if price deviates too far from current median
        let max_bps: u32 = env.storage().instance().get(&MAX_DEV).unwrap_or(0);
        if max_bps > 0 {
            let feeds: Map<String, FeedData> = env.storage().instance().get(&FEEDS).unwrap();
            if let Some(current) = feeds.get(asset.clone()) {
                let median = current.price;
                if median > 0 {
                    // deviation = |price - median| / median * 10_000
                    let diff = if price > median {
                        price - median
                    } else {
                        median - price
                    };
                    // multiply first to avoid precision loss (median is scaled 1e7)
                    let deviation_bps = (diff * 10_000) / median;
                    if deviation_bps > max_bps as i128 {
                        panic!("price deviation exceeded");
                    }
                }
            }
        }

        let entry_key = Self::entry_key(&env, &asset, &publisher);
        let entry = PriceEntry {
            price,
            timestamp,
            publisher: publisher.clone(),
        };
        env.storage().temporary().set(&entry_key, &entry);
        env.storage()
            .temporary()
            .extend_ttl(&entry_key, ENTRY_TTL, ENTRY_TTL);

        // Re-aggregate and emit event
        let new_feed = Self::aggregate(&env, &asset);
        if let Some(feed) = new_feed {
            PriceUpdated {
                asset,
                price: feed.price,
                timestamp: feed.timestamp,
                num_sources: feed.num_sources,
            }
            .publish(&env);
        }
    }

    // ── Price reads ──────────────────────────────────────────────────────────

    /// Get the latest aggregated price for an asset.
    pub fn get_price(env: Env, asset: String) -> FeedData {
        let feeds: Map<String, FeedData> = env.storage().instance().get(&FEEDS).unwrap();
        feeds.get(asset).expect("feed not found")
    }

    /// Get the latest price only if it is fresher than `max_age_secs`.
    pub fn get_price_fresh(env: Env, asset: String, max_age_secs: u64) -> FeedData {
        let feed = Self::get_price(env.clone(), asset);
        let now = env.ledger().timestamp();
        if now.saturating_sub(feed.timestamp) > max_age_secs {
            panic!("stale price");
        }
        feed
    }

    /// List all tracked asset pairs.
    pub fn get_assets(env: Env) -> Vec<String> {
        let feeds: Map<String, FeedData> = env.storage().instance().get(&FEEDS).unwrap();
        feeds.keys()
    }

    // ── Internal helpers ─────────────────────────────────────────────────────

    fn require_admin(env: &Env) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();
    }

    fn require_publisher(env: &Env, publisher: &Address) {
        let pubs: Vec<Address> = env.storage().instance().get(&PUBLISHERS).unwrap();
        if !pubs.contains(publisher) {
            panic!("unauthorized publisher");
        }
    }

    fn entry_key(_env: &Env, asset: &String, publisher: &Address) -> (String, Address) {
        (asset.clone(), publisher.clone())
    }

    /// Collect all publisher entries for an asset, compute the median, persist, and return it.
    fn aggregate(env: &Env, asset: &String) -> Option<FeedData> {
        let pubs: Vec<Address> = env.storage().instance().get(&PUBLISHERS).unwrap();
        let mut prices: Vec<i128> = Vec::new(env);
        let mut latest_ts: u64 = 0;

        for publisher in pubs.iter() {
            let key = Self::entry_key(env, asset, &publisher);
            if let Some(entry) = env.storage().temporary().get::<_, PriceEntry>(&key) {
                prices.push_back(entry.price);
                if entry.timestamp > latest_ts {
                    latest_ts = entry.timestamp;
                }
            }
        }

        if prices.is_empty() {
            return None;
        }

        let median = Self::median(env, &prices);
        let feed = FeedData {
            asset: asset.clone(),
            price: median,
            timestamp: latest_ts,
            num_sources: prices.len(),
        };

        let mut feeds: Map<String, FeedData> = env.storage().instance().get(&FEEDS).unwrap();
        feeds.set(asset.clone(), feed.clone());
        env.storage().instance().set(&FEEDS, &feeds);
        Some(feed)
    }

    /// Compute median of a non-empty price list (insertion sort, suitable for small N).
    fn median(env: &Env, prices: &Vec<i128>) -> i128 {
        let n = prices.len() as usize;
        let mut sorted: Vec<i128> = Vec::new(env);
        for p in prices.iter() {
            sorted.push_back(p);
        }
        // Insertion sort
        for i in 1..n {
            let key = sorted.get(i as u32).unwrap();
            let mut j = i;
            while j > 0 && sorted.get((j - 1) as u32).unwrap() > key {
                let prev = sorted.get((j - 1) as u32).unwrap();
                sorted.set(j as u32, prev);
                j -= 1;
            }
            sorted.set(j as u32, key);
        }
        if n % 2 == 1 {
            sorted.get((n / 2) as u32).unwrap()
        } else {
            let a = sorted.get((n / 2 - 1) as u32).unwrap();
            let b = sorted.get((n / 2) as u32).unwrap();
            (a + b) / 2
        }
    }
}

mod test;
