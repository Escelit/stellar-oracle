#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, testutils::Events, testutils::Ledger, Env, String};

fn setup() -> (Env, OracleContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(OracleContract, ());
    let client = OracleContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, client, contract_id)
}

#[test]
fn test_initialize() {
    let (_, client, _) = setup();
    assert_eq!(client.get_publishers().len(), 0);
}

#[test]
fn test_get_publisher_count() {
    let (env, client, _) = setup();
    assert_eq!(client.get_publisher_count(), 0);
    let pub1 = Address::generate(&env);
    let pub2 = Address::generate(&env);
    client.add_publisher(&pub1);
    assert_eq!(client.get_publisher_count(), 1);
    client.add_publisher(&pub2);
    assert_eq!(client.get_publisher_count(), 2);
    client.remove_publisher(&pub1);
    assert_eq!(client.get_publisher_count(), 1);
}

#[test]
fn test_add_remove_publisher() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    client.add_publisher(&pub1);
    assert_eq!(client.get_publishers().len(), 1);
    client.remove_publisher(&pub1);
    assert_eq!(client.get_publishers().len(), 0);
}

#[test]
#[should_panic(expected = "publisher already exists")]
fn test_duplicate_publisher() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    client.add_publisher(&pub1);
    client.add_publisher(&pub1);
}

#[test]
fn test_single_publisher_price() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    client.add_publisher(&pub1);

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "XLM/USD");
    client.submit_price(&pub1, &asset, &1_200_0000, &1_000_000);

    let feed = client.get_price(&asset);
    assert_eq!(feed.price, 1_200_0000);
    assert_eq!(feed.num_sources, 1);
}

#[test]
fn test_median_three_publishers() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    let pub2 = Address::generate(&env);
    let pub3 = Address::generate(&env);
    client.add_publisher(&pub1);
    client.add_publisher(&pub2);
    client.add_publisher(&pub3);

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "XLM/USD");
    // prices: 1.0, 1.2, 1.4 → median = 1.2
    client.submit_price(&pub1, &asset, &1_000_0000, &1_000_000);
    client.submit_price(&pub2, &asset, &1_200_0000, &1_000_000);
    client.submit_price(&pub3, &asset, &1_400_0000, &1_000_000);

    let feed = client.get_price(&asset);
    assert_eq!(feed.price, 1_200_0000);
    assert_eq!(feed.num_sources, 3);
}

#[test]
fn test_median_even_publishers() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    let pub2 = Address::generate(&env);
    client.add_publisher(&pub1);
    client.add_publisher(&pub2);

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "BTC/USD");
    // prices: 60000, 62000 → median = 61000
    client.submit_price(&pub1, &asset, &60_000_0000000, &1_000_000);
    client.submit_price(&pub2, &asset, &62_000_0000000, &1_000_000);

    let feed = client.get_price(&asset);
    assert_eq!(feed.price, 61_000_0000000);
}

#[test]
fn test_get_price_fresh_passes() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    client.add_publisher(&pub1);

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "XLM/USD");
    client.submit_price(&pub1, &asset, &1_200_0000, &1_000_000);

    let feed = client.get_price_fresh(&asset, &60);
    assert_eq!(feed.price, 1_200_0000);
}

#[test]
#[should_panic(expected = "stale price")]
fn test_get_price_fresh_stale() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    client.add_publisher(&pub1);

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "XLM/USD");
    client.submit_price(&pub1, &asset, &1_200_0000, &1_000_000);

    env.ledger().set_timestamp(1_000_120);
    client.get_price_fresh(&asset, &60);
}

#[test]
#[should_panic(expected = "unauthorized publisher")]
fn test_unauthorized_publisher_submit() {
    let (env, client, _) = setup();
    let rogue = Address::generate(&env);
    let asset = String::from_str(&env, "XLM/USD");
    client.submit_price(&rogue, &asset, &1_200_0000, &1_000_000);
}

#[test]
fn test_multiple_assets() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    client.add_publisher(&pub1);

    env.ledger().set_timestamp(1_000_000);
    let xlm = String::from_str(&env, "XLM/USD");
    let btc = String::from_str(&env, "BTC/USD");
    client.submit_price(&pub1, &xlm, &1_200_0000, &1_000_000);
    client.submit_price(&pub1, &btc, &60_000_0000000, &1_000_000);

    assert_eq!(client.get_assets().len(), 2);
}

// ── Event tests ──────────────────────────────────────────────────────────────

#[test]
fn test_price_updated_event_emitted() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    client.add_publisher(&pub1);

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "XLM/USD");

    // No events before submission
    let no_events: soroban_sdk::Vec<(
        Address,
        soroban_sdk::Vec<soroban_sdk::Val>,
        soroban_sdk::Val,
    )> = soroban_sdk::Vec::new(&env);
    assert_eq!(env.events().all(), no_events);

    client.submit_price(&pub1, &asset, &1_200_0000, &1_000_000);

    // At least one event emitted after submission
    assert_ne!(env.events().all(), no_events);
}

// ── Circuit breaker tests ─────────────────────────────────────────────────────

#[test]
fn test_circuit_breaker_allows_within_deviation() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    let pub2 = Address::generate(&env);
    client.add_publisher(&pub1);
    client.add_publisher(&pub2);
    // 10% max deviation
    client.set_max_deviation_bps(&1000);

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "XLM/USD");
    // Establish baseline median = 1.0
    client.submit_price(&pub1, &asset, &1_000_0000, &1_000_000);
    // 5% deviation — should pass
    client.submit_price(&pub2, &asset, &1_050_0000, &1_000_000);

    let feed = client.get_price(&asset);
    assert_eq!(feed.num_sources, 2);
}

#[test]
#[should_panic(expected = "price deviation exceeded")]
fn test_circuit_breaker_rejects_excessive_deviation() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    let pub2 = Address::generate(&env);
    client.add_publisher(&pub1);
    client.add_publisher(&pub2);
    // 10% max deviation
    client.set_max_deviation_bps(&1000);

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "XLM/USD");
    // Establish baseline median = 1.0
    client.submit_price(&pub1, &asset, &1_000_0000, &1_000_000);
    // 50% deviation — should be rejected
    client.submit_price(&pub2, &asset, &1_500_0000, &1_000_000);
}

#[test]
fn test_circuit_breaker_disabled_by_default() {
    let (env, client, _) = setup();
    let pub1 = Address::generate(&env);
    let pub2 = Address::generate(&env);
    client.add_publisher(&pub1);
    client.add_publisher(&pub2);
    // No set_max_deviation_bps call — defaults to 0 (disabled)

    env.ledger().set_timestamp(1_000_000);
    let asset = String::from_str(&env, "XLM/USD");
    client.submit_price(&pub1, &asset, &1_000_0000, &1_000_000);
    // 10x deviation — allowed because circuit breaker is off
    client.submit_price(&pub2, &asset, &10_000_0000, &1_000_000);

    let feed = client.get_price(&asset);
    assert_eq!(feed.num_sources, 2);
}

#[test]
fn test_get_max_deviation_bps() {
    let (_, client, _) = setup();
    assert_eq!(client.get_max_deviation_bps(), 0);
    client.set_max_deviation_bps(&500);
    assert_eq!(client.get_max_deviation_bps(), 500);
}
