import { Networks } from "@stellar/stellar-sdk";
import { OracleConsumer } from "../src/consumer";
import { OraclePublisher } from "../src/publisher";
import { toFloat, toScaled } from "../src/types";

const TEST_CONFIG = {
  contractId: "CA76KLJ2CDD5OHVGD6MUV3QVZYRNJJQLIHBMWD353J6ES4JZXCO4L5OQ",
  networkPassphrase: Networks.TESTNET,
  rpcUrl: "https://soroban-testnet.stellar.org",
};

// ── toFloat / toScaled ────────────────────────────────────────────────────────

describe("toFloat / toScaled", () => {
  test("toFloat converts scaled price to float", () => {
    // 0.12 * 1e7 = 1_200_000
    expect(toFloat(1_200_000n)).toBeCloseTo(0.12);
    expect(toFloat(600_000_000_000n)).toBeCloseTo(60000);
  });

  test("toScaled converts float to scaled price", () => {
    expect(toScaled(0.12)).toBe(1_200_000n);
    expect(toScaled(60000)).toBe(600_000_000_000n);
  });

  test("round-trip: toFloat(toScaled(x)) ≈ x", () => {
    const prices = [0.12, 1.0, 60000, 0.0001];
    for (const p of prices) {
      expect(toFloat(toScaled(p))).toBeCloseTo(p, 5);
    }
  });
});

// ── OracleConsumer ────────────────────────────────────────────────────────────

describe("OracleConsumer", () => {
  const mockServer = {
    simulateTransaction: jest.fn(),
  };

  beforeEach(() => {
    jest.clearAllMocks();
  });

  function makeConsumer() {
    const consumer = new OracleConsumer(TEST_CONFIG);
    // Inject mock server
    (consumer as any).server = mockServer;
    return consumer;
  }

  function makeSimResult(retval: any) {
    return {
      result: { retval },
      latestLedger: 1000,
    };
  }

  test("getAssets returns empty array when no feeds", async () => {
    const { nativeToScVal } = await import("@stellar/stellar-sdk");
    mockServer.simulateTransaction.mockResolvedValue(
      makeSimResult(nativeToScVal([], { type: "vec" }))
    );
    const consumer = makeConsumer();
    const assets = await consumer.getAssets();
    expect(Array.isArray(assets)).toBe(true);
  });

  test("getPrice throws on simulation error", async () => {
    mockServer.simulateTransaction.mockResolvedValue({
      error: "contract error: feed not found",
      latestLedger: 1000,
    });
    const consumer = makeConsumer();
    await expect(consumer.getPrice("XLM/USD")).rejects.toThrow("Simulation failed");
  });

  test("getPriceFresh throws on simulation error", async () => {
    mockServer.simulateTransaction.mockResolvedValue({
      error: "contract error: stale price",
      latestLedger: 1000,
    });
    const consumer = makeConsumer();
    await expect(consumer.getPriceFresh("XLM/USD", 60)).rejects.toThrow("Simulation failed");
  });
});

// ── OraclePublisher retry logic ───────────────────────────────────────────────

describe("OraclePublisher retry", () => {
  const mockServer = {
    getAccount: jest.fn(),
    prepareTransaction: jest.fn(),
    sendTransaction: jest.fn(),
  };

  function makePublisher() {
    const { Keypair } = require("@stellar/stellar-sdk");
    const keypair = Keypair.random();
    const publisher = new OraclePublisher(TEST_CONFIG, keypair);
    (publisher as any).server = mockServer;
    return publisher;
  }

  beforeEach(() => {
    jest.clearAllMocks();
    // Mock setTimeout so retry delays resolve immediately
    jest.spyOn(global, "setTimeout").mockImplementation((fn: any) => {
      fn();
      return 0 as any;
    });
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  test("succeeds on first attempt", async () => {
    const { Keypair } = require("@stellar/stellar-sdk");
    mockServer.getAccount.mockResolvedValue({
      accountId: () => Keypair.random().publicKey(),
      sequenceNumber: () => "0",
      incrementSequenceNumber: () => {},
    });
    mockServer.prepareTransaction.mockResolvedValue({
      sign: jest.fn(),
    });
    mockServer.sendTransaction.mockResolvedValue({ status: "PENDING", hash: "abc123" });

    const publisher = makePublisher();
    const hash = await publisher.submitPrice("XLM/USD", 0.12);
    expect(hash).toBe("abc123");
    expect(mockServer.getAccount).toHaveBeenCalledTimes(1);
  });

  test("retries on failure and succeeds on second attempt", async () => {
    const { Keypair } = require("@stellar/stellar-sdk");
    const fakeAccount = {
      accountId: () => Keypair.random().publicKey(),
      sequenceNumber: () => "0",
      incrementSequenceNumber: () => {},
    };
    mockServer.getAccount
      .mockRejectedValueOnce(new Error("network error"))
      .mockResolvedValue(fakeAccount);
    mockServer.prepareTransaction.mockResolvedValue({ sign: jest.fn() });
    mockServer.sendTransaction.mockResolvedValue({ status: "PENDING", hash: "retry_hash" });

    const publisher = makePublisher();
    const hash = await publisher.submitPrice("XLM/USD", 0.12);
    expect(hash).toBe("retry_hash");
    expect(mockServer.getAccount).toHaveBeenCalledTimes(2);
  });

  test("throws after 3 failed attempts", async () => {
    mockServer.getAccount
      .mockRejectedValueOnce(new Error("persistent error"))
      .mockRejectedValueOnce(new Error("persistent error"))
      .mockRejectedValueOnce(new Error("persistent error"));

    const publisher = makePublisher();
    await expect(publisher.submitPrice("XLM/USD", 0.12)).rejects.toThrow("persistent error");
    expect(mockServer.getAccount).toHaveBeenCalledTimes(3);
  });
});
