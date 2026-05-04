import {
  Contract,
  Networks,
  rpc,
  TransactionBuilder,
  BASE_FEE,
  Keypair,
  nativeToScVal,
} from "@stellar/stellar-sdk";
import { OracleConfig } from "./types";

/**
 * OraclePublisher — submits price updates to the oracle contract.
 *
 * Usage:
 *   const publisher = new OraclePublisher(config, keypair);
 *   await publisher.submitPrice("XLM/USD", 0.12);
 */
export class OraclePublisher {
  private contract: Contract;
  private server: rpc.Server;

  constructor(
    private config: OracleConfig,
    private keypair: Keypair
  ) {
    this.contract = new Contract(config.contractId);
    this.server = new rpc.Server(config.rpcUrl);
  }

  /**
   * Submit a price for an asset pair, with exponential backoff retry (max 3 attempts).
   * @param asset  e.g. "XLM/USD"
   * @param price  human-readable float, e.g. 0.12
   */
  async submitPrice(asset: string, price: number): Promise<string> {
    const maxAttempts = 3;
    let lastError: unknown;
    for (let attempt = 1; attempt <= maxAttempts; attempt++) {
      try {
        return await this._submitOnce(asset, price);
      } catch (err) {
        lastError = err;
        if (attempt < maxAttempts) {
          const delayMs = 1000 * 2 ** (attempt - 1); // 1s, 2s
          console.warn(`[submitPrice] attempt ${attempt} failed, retrying in ${delayMs}ms:`, err);
          await new Promise((r) => setTimeout(r, delayMs));
        }
      }
    }
    throw lastError;
  }

  private async _submitOnce(asset: string, price: number): Promise<string> {
    const scaledPrice = BigInt(Math.round(price * 1e7));
    const timestamp = Math.floor(Date.now() / 1000);

    const account = await this.server.getAccount(this.keypair.publicKey());

    const tx = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.config.networkPassphrase,
    })
      .addOperation(
        this.contract.call(
          "submit_price",
          nativeToScVal(this.keypair.publicKey(), { type: "address" }),
          nativeToScVal(asset, { type: "string" }),
          nativeToScVal(scaledPrice, { type: "i128" }),
          nativeToScVal(timestamp, { type: "u64" })
        )
      )
      .setTimeout(30)
      .build();

    const prepared = await this.server.prepareTransaction(tx);
    prepared.sign(this.keypair);

    const result = await this.server.sendTransaction(prepared);
    if (result.status === "ERROR") {
      throw new Error(`Transaction failed: ${JSON.stringify(result.errorResult)}`);
    }

    // Poll until the transaction is confirmed (SUCCESS) or failed
    const hash = result.hash;
    for (let i = 0; i < 20; i++) {
      await new Promise((r) => setTimeout(r, 1500));
      const tx = await this.server.getTransaction(hash);
      if (tx.status === "SUCCESS") return hash;
      if (tx.status === "FAILED") {
        throw new Error(`Transaction failed on-chain: ${hash}`);
      }
      // status === "NOT_FOUND" means still pending — keep polling
    }
    throw new Error(`Transaction not confirmed after timeout: ${hash}`);
  }
}
