#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, testutils::Ledger, Env, String};

fn setup() -> (Env, OracleContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(OracleContract, ());
    let client = OracleContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, client, admin)
}

#[test]
fn test_initialize() {
    let (_, client, _) = setup();
    assert_eq!(client.get_publishers().len(), 0);
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

    // still fresh within 60s
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

    // advance ledger time by 120s, max_age = 60s → stale
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
