"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.toFloat = toFloat;
exports.toScaled = toScaled;
/** Convert a raw scaled price (1e7) to a human-readable float */
function toFloat(scaled) {
    return Number(scaled) / 1e7;
}
/** Convert a human-readable float to a scaled price (1e7) */
function toScaled(price) {
    return BigInt(Math.round(price * 1e7));
}
