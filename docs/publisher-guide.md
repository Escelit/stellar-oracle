# Publisher Bot Guide

## Running with Docker

Build and start the publisher bot:

```bash
docker compose up --build
```

## Environment Variables

### ORACLE_CONTRACT_ID

The deployed Soroban oracle contract ID that receives price updates.

Example:

```text
CCXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

### ORACLE_RPC_URL

RPC endpoint used to communicate with the Soroban network.

Example:

```text
https://soroban-testnet.stellar.org
```

### ORACLE_NETWORK

Target Stellar network.

Supported values:

```text
TESTNET
MAINNET
```

### PUBLISHER_SECRET_KEY

The Stellar secret key used by the publisher bot to sign transactions.

Example:

```text
SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

Never commit real secret keys to source control.

### ASSETS

Comma-separated list of asset pairs to publish.

Example:

```text
XLM/USD,BTC/USD,ETH/USD
```

### INTERVAL_MS

Publishing interval in milliseconds.

Example:

```text
60000
```

This value causes the publisher bot to publish updates every 60 seconds.

## Example docker-compose.yml

```yaml
services:
  publisher-bot:
    build: .
    environment:
      ORACLE_CONTRACT_ID: ""
      ORACLE_RPC_URL: "https://soroban-testnet.stellar.org"
      ORACLE_NETWORK: "TESTNET"
      PUBLISHER_SECRET_KEY: ""
      ASSETS: "XLM/USD,BTC/USD,ETH/USD"
      INTERVAL_MS: "60000"
```
