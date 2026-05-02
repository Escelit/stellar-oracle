export interface FeedData {
  asset: string;
  price: bigint;      // scaled by 1e7
  timestamp: number;  // Unix seconds
  numSources: number;
}

export interface OracleConfig {
  contractId: string;
  networkPassphrase: string;
  rpcUrl: string;
}

/** Convert a raw scaled price (1e7) to a human-readable float */
export function toFloat(scaled: bigint): number {
  return Number(scaled) / 1e7;
}

/** Convert a human-readable float to a scaled price (1e7) */
export function toScaled(price: number): bigint {
  return BigInt(Math.round(price * 1e7));
}
