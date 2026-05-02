"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
/**
 * Example publisher bot: fetches XLM/USD and BTC/USD prices from
 * CoinGecko's free API and submits them to the oracle contract every 60s.
 *
 * Usage:
 *   PUBLISHER_SECRET=S... CONTRACT_ID=C... node dist/publisher-bot.js
 */
const stellar_sdk_1 = require("@stellar/stellar-sdk");
const publisher_1 = require("./publisher");
const ASSETS = {
    "XLM/USD": "stellar",
    "BTC/USD": "bitcoin",
    "ETH/USD": "ethereum",
};
async function fetchPrices() {
    const ids = Object.values(ASSETS).join(",");
    const url = `https://api.coingecko.com/api/v3/simple/price?ids=${ids}&vs_currencies=usd`;
    const res = await fetch(url);
    if (!res.ok)
        throw new Error(`CoinGecko error: ${res.status}`);
    const data = (await res.json());
    const prices = {};
    for (const [pair, geckoId] of Object.entries(ASSETS)) {
        prices[pair] = data[geckoId].usd;
    }
    return prices;
}
async function run() {
    const secret = process.env.PUBLISHER_SECRET;
    const contractId = process.env.CONTRACT_ID;
    const rpcUrl = process.env.RPC_URL ?? "https://soroban-testnet.stellar.org";
    if (!secret || !contractId) {
        console.error("Set PUBLISHER_SECRET and CONTRACT_ID env vars");
        process.exit(1);
    }
    const keypair = stellar_sdk_1.Keypair.fromSecret(secret);
    const publisher = new publisher_1.OraclePublisher({ contractId, networkPassphrase: stellar_sdk_1.Networks.TESTNET, rpcUrl }, keypair);
    console.log(`Publisher: ${keypair.publicKey()}`);
    console.log(`Contract:  ${contractId}`);
    while (true) {
        try {
            const prices = await fetchPrices();
            for (const [asset, price] of Object.entries(prices)) {
                const hash = await publisher.submitPrice(asset, price);
                console.log(`[${new Date().toISOString()}] ${asset} = $${price} → ${hash}`);
            }
        }
        catch (err) {
            console.error("Error:", err);
        }
        await new Promise((r) => setTimeout(r, 60000));
    }
}
run();
