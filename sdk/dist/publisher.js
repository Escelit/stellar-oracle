"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.OraclePublisher = void 0;
const stellar_sdk_1 = require("@stellar/stellar-sdk");
/**
 * OraclePublisher — submits price updates to the oracle contract.
 *
 * Usage:
 *   const publisher = new OraclePublisher(config, keypair);
 *   await publisher.submitPrice("XLM/USD", 0.12);
 */
class OraclePublisher {
    constructor(config, keypair) {
        this.config = config;
        this.keypair = keypair;
        this.contract = new stellar_sdk_1.Contract(config.contractId);
        this.server = new stellar_sdk_1.rpc.Server(config.rpcUrl);
    }
    /**
     * Submit a price for an asset pair.
     * @param asset  e.g. "XLM/USD"
     * @param price  human-readable float, e.g. 0.12
     */
    async submitPrice(asset, price) {
        const scaledPrice = BigInt(Math.round(price * 1e7));
        const timestamp = Math.floor(Date.now() / 1000);
        const account = await this.server.getAccount(this.keypair.publicKey());
        const tx = new stellar_sdk_1.TransactionBuilder(account, {
            fee: stellar_sdk_1.BASE_FEE,
            networkPassphrase: this.config.networkPassphrase,
        })
            .addOperation(this.contract.call("submit_price", (0, stellar_sdk_1.nativeToScVal)(this.keypair.publicKey(), { type: "address" }), (0, stellar_sdk_1.nativeToScVal)(asset, { type: "string" }), (0, stellar_sdk_1.nativeToScVal)(scaledPrice, { type: "i128" }), (0, stellar_sdk_1.nativeToScVal)(timestamp, { type: "u64" })))
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
exports.OraclePublisher = OraclePublisher;
