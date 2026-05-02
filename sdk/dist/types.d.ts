export interface FeedData {
    asset: string;
    price: bigint;
    timestamp: number;
    numSources: number;
}
export interface OracleConfig {
    contractId: string;
    networkPassphrase: string;
    rpcUrl: string;
}
/** Convert a raw scaled price (1e7) to a human-readable float */
export declare function toFloat(scaled: bigint): number;
/** Convert a human-readable float to a scaled price (1e7) */
export declare function toScaled(price: number): bigint;
