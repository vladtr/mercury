# mercury
crosschain experiment in rust

Put your infura key in rest_api/src/handlers.rs

```
cargo run

curl http://127.0.0.1:8080/latest_block
Latest block number: 20463045
```

```
curl http://127.0.0.1:8080/cardano_block
Cardano Latest block height: 10666469, hash: 5dba91a57d1556dda52d0400c5d58b39abc579210ee41b478f5ad8dcadfb066
```

```
curl -X POST http://127.0.0.1:8080/initiate_swap \
     -d '{"eth_tx_hash": "YOUR_ETH_TRANSACTION_HASH", "cardano_tx_id": "YOUR_CARDANO_TRANSACTION_ID"}' \
     -H "Content-Type: application/json"
```