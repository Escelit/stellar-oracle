"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.OracleConsumer = void 0;
const stellar_sdk_1 = require("@stellar/stellar-sdk");
/**
 * OracleConsumer — reads price feeds from the oracle contract.
 *
 * Usage:
 *   const consumer = new OracleConsumer(config);
 *   const feed = await consumer.getPrice("XLM/USD");
 *   console.log(feed.price); // bigint scaled by 1e7
 */
class OracleConsumer {
    constructor(config) {
        this.config = config;
        // Throwaway keypair for read-only simulation (no signing needed)
        this.keypair = stellar_sdk_1.Keypair.random();
        this.contract = new stellar_sdk_1.Contract(config.contractId);
        this.server = new stellar_sdk_1.rpc.Server(config.rpcUrl);
    }
    /** Get the latest aggregated price for an asset pair. */
    async getPrice(asset) {
        const result = await this.simulate("get_price", [
            (0, stellar_sdk_1.nativeToScVal)(asset, { type: "string" }),
        ]);
        return this.parseFeed(result);
    }
    /**
     * Get the latest price only if fresher than maxAgeSecs.
     * Throws if the price is stale.
     */
    async getPriceFresh(asset, maxAgeSecs) {
        const result = await this.simulate("get_price_fresh", [
            (0, stellar_sdk_1.nativeToScVal)(asset, { type: "string" }),
            (0, stellar_sdk_1.nativeToScVal)(maxAgeSecs, { type: "u64" }),
        ]);
        return this.parseFeed(result);
    }
    /** List all tracked asset pairs. */
    async getAssets() {
        const result = await this.simulate("get_assets", []);
        return (0, stellar_sdk_1.scValToNative)(result);
    }
    // ── Internal ──────────────────────────────────────────────────────────────
    async simulate(method, args) {
        // Use a minimal fake account for simulation (no ledger lookup needed)
        const fakeAccount = {
            accountId: () => this.keypair.publicKey(),
            sequenceNumber: () => "0",
            incrementSequenceNumber: () => { },
        };
        const tx = new stellar_sdk_1.TransactionBuilder(fakeAccount, {
            fee: stellar_sdk_1.BASE_FEE,
            networkPassphrase: this.config.networkPassphrase,
        })
            .addOperation(this.contract.call(method, ...args))
            .setTimeout(30)
            .build();
        const sim = await this.server.simulateTransaction(tx);
        if (stellar_sdk_1.rpc.Api.isSimulationError(sim)) {
            throw new Error(`Simulation failed: ${sim.error}`);
        }
        return sim.result?.retval;
    }
    parseFeed(scVal) {
        const native = (0, stellar_sdk_1.scValToNative)(scVal);
        return {
            asset: native.asset,
            price: BigInt(native.price),
            timestamp: Number(native.timestamp),
            numSources: Number(native.num_sources),
        };
    }
}
exports.OracleConsumer = OracleConsumer;
