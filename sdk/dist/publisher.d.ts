import { Keypair } from "@stellar/stellar-sdk";
import { OracleConfig } from "./types";
/**
 * OraclePublisher — submits price updates to the oracle contract.
 *
 * Usage:
 *   const publisher = new OraclePublisher(config, keypair);
 *   await publisher.submitPrice("XLM/USD", 0.12);
 */
export declare class OraclePublisher {
    private config;
    private keypair;
    private contract;
    private server;
    constructor(config: OracleConfig, keypair: Keypair);
    /**
     * Submit a price for an asset pair.
     * @param asset  e.g. "XLM/USD"
     * @param price  human-readable float, e.g. 0.12
     */
    submitPrice(asset: string, price: number): Promise<string>;
}
