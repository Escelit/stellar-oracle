#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol, Vec,
};

// ── Storage keys ────────────────────────────────────────────────────────────

const ADMIN: Symbol = symbol_short!("ADMIN");
const FEEDS: Symbol = symbol_short!("FEEDS");
const PUBLISHERS: Symbol = symbol_short!("PUBS");

// ── Data types ───────────────────────────────────────────────────────────────

/// A single price entry submitted by a publisher.
#[contracttype]
#[derive(Clone)]
pub struct PriceEntry {
    pub price: i128,     // price scaled by 1e7 (e.g. 1 XLM = 1_0000000)
    pub timestamp: u64,  // Unix timestamp (seconds)
    pub publisher: Address,
}

/// Aggregated feed data stored on-chain.
#[contracttype]
#[derive(Clone)]
pub struct FeedData {
    pub asset: String,       // e.g. "XLM/USD"
    pub price: i128,         // median price, scaled by 1e7
    pub timestamp: u64,      // timestamp of latest update
    pub num_sources: u32,    // how many publishers contributed
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
    /// Prices from all publishers are aggregated via median.
    pub fn submit_price(env: Env, publisher: Address, asset: String, price: i128, timestamp: u64) {
        publisher.require_auth();
        Self::require_publisher(&env, &publisher);

        // Store this publisher's entry under a per-asset key
        let entry_key = Self::entry_key(&env, &asset, &publisher);
        let entry = PriceEntry {
            price,
            timestamp,
            publisher: publisher.clone(),
        };
        env.storage().temporary().set(&entry_key, &entry);

        // Re-aggregate all known publisher prices for this asset
        Self::aggregate(&env, &asset);
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

    /// Collect all publisher entries for an asset and compute the median price.
    fn aggregate(env: &Env, asset: &String) {
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
            return;
        }

        let median = Self::median(env, &prices);
        let feed = FeedData {
            asset: asset.clone(),
            price: median,
            timestamp: latest_ts,
            num_sources: prices.len(),
        };

        let mut feeds: Map<String, FeedData> = env.storage().instance().get(&FEEDS).unwrap();
        feeds.set(asset.clone(), feed);
        env.storage().instance().set(&FEEDS, &feeds);
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
