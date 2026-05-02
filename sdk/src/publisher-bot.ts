/**
 * Example publisher bot: fetches XLM/USD and BTC/USD prices from
 * CoinGecko's free API and submits them to the oracle contract every 60s.
 *
 * Usage:
 *   PUBLISHER_SECRET=S... CONTRACT_ID=C... node dist/publisher-bot.js
 */
import { Keypair, Networks } from "@stellar/stellar-sdk";
import { OraclePublisher } from "./publisher";

const ASSETS: Record<string, string> = {
  "XLM/USD": "stellar",
  "BTC/USD": "bitcoin",
  "ETH/USD": "ethereum",
};

async function fetchPrices(): Promise<Record<string, number>> {
  const ids = Object.values(ASSETS).join(",");
  const url = `https://api.coingecko.com/api/v3/simple/price?ids=${ids}&vs_currencies=usd`;
  const res = await fetch(url);
  if (!res.ok) throw new Error(`CoinGecko error: ${res.status}`);
  const data = (await res.json()) as Record<string, { usd: number }>;

  const prices: Record<string, number> = {};
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

  const keypair = Keypair.fromSecret(secret);
  const publisher = new OraclePublisher(
    { contractId, networkPassphrase: Networks.TESTNET, rpcUrl },
    keypair
  );

  console.log(`Publisher: ${keypair.publicKey()}`);
  console.log(`Contract:  ${contractId}`);

  while (true) {
    try {
      const prices = await fetchPrices();
      for (const [asset, price] of Object.entries(prices)) {
        const hash = await publisher.submitPrice(asset, price);
        console.log(`[${new Date().toISOString()}] ${asset} = $${price} → ${hash}`);
      }
    } catch (err) {
      console.error("Error:", err);
    }
    await new Promise((r) => setTimeout(r, 60_000));
  }
}

run();
