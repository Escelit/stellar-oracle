/**
 * Example publisher bot: fetches XLM/USD, BTC/USD, ETH/USD prices from
 * CoinGecko's free API and submits them to the oracle contract every 60s.
 *
 * Usage:
 *   PUBLISHER_SECRET=S... CONTRACT_ID=C... node dist/publisher-bot.js
 *
 * Flags:
 *   --dry-run   Fetch prices and log what would be submitted, without sending transactions.
 *   --once      Submit once and exit (useful for cron-based deployments).
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
  const networkPassphrase = process.env.ORACLE_NETWORK ?? Networks.TESTNET;
  const dryRun = process.argv.includes("--dry-run");
  const once = process.argv.includes("--once");

  if (!dryRun && (!secret || !contractId)) {
    console.error("Set PUBLISHER_SECRET and CONTRACT_ID env vars");
    process.exit(1);
  }

  const keypair = dryRun ? Keypair.random() : Keypair.fromSecret(secret!);
  const publisher = dryRun
    ? null
    : new OraclePublisher(
        { contractId: contractId!, networkPassphrase, rpcUrl },
        keypair
      );

  if (dryRun) {
    console.log("[dry-run] No transactions will be sent.");
  } else {
    console.log(`Publisher: ${keypair.publicKey()}`);
    console.log(`Contract:  ${contractId}`);
  }

  do {
    try {
      const prices = await fetchPrices();
      for (const [asset, price] of Object.entries(prices)) {
        if (dryRun) {
          console.log(`[dry-run] ${asset} = $${price} (would submit)`);
        } else {
          const hash = await publisher!.submitPrice(asset, price);
          console.log(`[${new Date().toISOString()}] ${asset} = $${price} → ${hash}`);
        }
      }
    } catch (err) {
      console.error("Error:", err);
    }
    if (!once) await new Promise((r) => setTimeout(r, 60_000));
  } while (!once);
}

run();
