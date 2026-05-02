import { OracleConfig, FeedData } from "./types";
/**
 * OracleConsumer — reads price feeds from the oracle contract.
 *
 * Usage:
 *   const consumer = new OracleConsumer(config);
 *   const feed = await consumer.getPrice("XLM/USD");
 *   console.log(feed.price); // bigint scaled by 1e7
 */
export declare class OracleConsumer {
    private config;
    private contract;
    private server;
    private keypair;
    constructor(config: OracleConfig);
    /** Get the latest aggregated price for an asset pair. */
    getPrice(asset: string): Promise<FeedData>;
    /**
     * Get the latest price only if fresher than maxAgeSecs.
     * Throws if the price is stale.
     */
    getPriceFresh(asset: string, maxAgeSecs: number): Promise<FeedData>;
    /** List all tracked asset pairs. */
    getAssets(): Promise<string[]>;
    private simulate;
    private parseFeed;
}
