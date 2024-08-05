
# Atomic Swap Executor

This project implements an atomic swap executor for monitoring transactions on the Ethereum and Cardano blockchains. It uses the `web3` library to interact with Ethereum and the Blockfrost API to interact with Cardano. The project is organized into several Rust crates, each responsible for different parts of the functionality.

## Project Structure

- **chain_observers**: Contains the logic for observing Ethereum and Cardano transactions.
- **executor**: Contains the logic to monitor both blockchain transactions and execute swaps once both transactions are confirmed.
- **rest_api**: Exposes a REST API for initiating swaps and querying transaction statuses.

## Prerequisites

- Rust and Cargo installed on your machine.
- An Infura project ID for connecting to the Ethereum blockchain.
- A Blockfrost API key for connecting to the Cardano blockchain.

## How It Works

The atomic swap executor monitors specified transactions on both the Ethereum and Cardano blockchains. Once both transactions are confirmed, the executor triggers the logic to perform a swap.

### Key Components

- **Ethereum Observer**: Uses the `web3` library to check the status of Ethereum transactions.
- **Cardano Observer**: Uses the Blockfrost API to check the status of Cardano transactions.
- **Executor**: Combines the results of both observers and executes the swap once both transactions are confirmed.

## REST API

The REST API provides an endpoint to initiate the monitoring of swap transactions.

### `/initiate_swap` Endpoint

- **Method**: POST
- **Description**: Starts monitoring two transactions, one on Ethereum and one on Cardano. The executor waits for both transactions to be confirmed before executing the swap.
- **Request Body**: JSON object containing the Ethereum transaction hash and Cardano transaction ID.

#### Example Request

```bash
curl -X POST http://127.0.0.1:8080/initiate_swap \
     -d '{"eth_tx_hash": "YOUR_ETH_TRANSACTION_HASH", "cardano_tx_id": "YOUR_CARDANO_TRANSACTION_ID"}' \
     -H "Content-Type: application/json"
```

#### Example getting latest block num
```
curl http://127.0.0.1:8080/latest_block
Latest block number: 20463045
```

```
curl http://127.0.0.1:8080/cardano_block
Cardano Latest block height: 10666469, hash: 5dba91a57d1556dda52d0400c5d58b39abc579210ee41b478f5ad8dcadfb066
```
