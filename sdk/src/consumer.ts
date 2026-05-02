import {
  Contract,
  rpc,
  TransactionBuilder,
  BASE_FEE,
  Keypair,
  nativeToScVal,
  scValToNative,
} from "@stellar/stellar-sdk";
import { OracleConfig, FeedData } from "./types";

/**
 * OracleConsumer — reads price feeds from the oracle contract.
 *
 * Usage:
 *   const consumer = new OracleConsumer(config);
 *   const feed = await consumer.getPrice("XLM/USD");
 *   console.log(feed.price); // bigint scaled by 1e7
 */
export class OracleConsumer {
  private contract: Contract;
  private server: rpc.Server;
  // Throwaway keypair for read-only simulation (no signing needed)
  private keypair = Keypair.random();

  constructor(private config: OracleConfig) {
    this.contract = new Contract(config.contractId);
    this.server = new rpc.Server(config.rpcUrl);
  }

  /** Get the latest aggregated price for an asset pair. */
  async getPrice(asset: string): Promise<FeedData> {
    const result = await this.simulate("get_price", [
      nativeToScVal(asset, { type: "string" }),
    ]);
    return this.parseFeed(result);
  }

  /**
   * Get the latest price only if fresher than maxAgeSecs.
   * Throws if the price is stale.
   */
  async getPriceFresh(asset: string, maxAgeSecs: number): Promise<FeedData> {
    const result = await this.simulate("get_price_fresh", [
      nativeToScVal(asset, { type: "string" }),
      nativeToScVal(maxAgeSecs, { type: "u64" }),
    ]);
    return this.parseFeed(result);
  }

  /** List all tracked asset pairs. */
  async getAssets(): Promise<string[]> {
    const result = await this.simulate("get_assets", []);
    return scValToNative(result) as string[];
  }

  // ── Internal ──────────────────────────────────────────────────────────────

  private async simulate(method: string, args: any[]): Promise<any> {
    // Use a minimal fake account for simulation (no ledger lookup needed)
    const fakeAccount = {
      accountId: () => this.keypair.publicKey(),
      sequenceNumber: () => "0",
      incrementSequenceNumber: () => {},
    } as any;

    const tx = new TransactionBuilder(fakeAccount, {
      fee: BASE_FEE,
      networkPassphrase: this.config.networkPassphrase,
    })
      .addOperation(this.contract.call(method, ...args))
      .setTimeout(30)
      .build();

    const sim = await this.server.simulateTransaction(tx);
    if (rpc.Api.isSimulationError(sim)) {
      throw new Error(`Simulation failed: ${(sim as rpc.Api.SimulateTransactionErrorResponse).error}`);
    }
    return (sim as rpc.Api.SimulateTransactionSuccessResponse).result?.retval;
  }

  private parseFeed(scVal: any): FeedData {
    const native = scValToNative(scVal) as any;
    return {
      asset: native.asset,
      price: BigInt(native.price),
      timestamp: Number(native.timestamp),
      numSources: Number(native.num_sources),
    };
  }
}
