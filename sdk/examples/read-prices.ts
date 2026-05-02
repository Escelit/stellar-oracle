/**
 * read-prices.ts — connects to testnet and prints all oracle price feeds.
 *
 * Usage:
 *   npx ts-node examples/read-prices.ts
 *   # or after build:
 *   node dist/examples/read-prices.js
 *
 * Optional env vars:
 *   CONTRACT_ID  (defaults to the public testnet deployment)
 *   RPC_URL      (defaults to https://soroban-testnet.stellar.org)
 */
import { Networks } from "@stellar/stellar-sdk";
import { OracleConsumer } from "../src/consumer";
import { toFloat } from "../src/types";

const CONTRACT_ID =
  process.env.CONTRACT_ID ??
  "CA76KLJ2CDD5OHVGD6MUV3QVZYRNJJQLIHBMWD353J6ES4JZXCO4L5OQ";
const RPC_URL =
  process.env.RPC_URL ?? "https://soroban-testnet.stellar.org";

async function main() {
  const consumer = new OracleConsumer({
    contractId: CONTRACT_ID,
    networkPassphrase: Networks.TESTNET,
    rpcUrl: RPC_URL,
  });

  const assets = await consumer.getAssets();
  if (assets.length === 0) {
    console.log("No price feeds found.");
    return;
  }

  console.log(`Oracle: ${CONTRACT_ID}\n`);
  console.log("Asset".padEnd(12), "Price (USD)".padEnd(18), "Sources", "Age (s)");
  console.log("-".repeat(55));

  const now = Math.floor(Date.now() / 1000);
  for (const asset of assets) {
    const feed = await consumer.getPrice(asset);
    const age = now - feed.timestamp;
    console.log(
      asset.padEnd(12),
      `$${toFloat(feed.price).toFixed(6)}`.padEnd(18),
      String(feed.numSources).padEnd(8),
      `${age}s`
    );
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
