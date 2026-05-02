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
   * Submit a price for an asset pair.
   * @param asset  e.g. "XLM/USD"
   * @param price  human-readable float, e.g. 0.12
   */
  async submitPrice(asset: string, price: number): Promise<string> {
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
    return result.hash;
  }
}
