use chain_observers::eth::EthObserver;
use chain_observers::cardano::CardanoObserver;
use web3::types::H256;
use tokio::time::{sleep, Duration};

pub struct Executor {
    eth_observer: EthObserver,
    cardano_observer: CardanoObserver,
}

impl Executor {
    pub fn new(eth_observer: EthObserver, cardano_observer: CardanoObserver) -> Self {
        Executor {
            eth_observer,
            cardano_observer,
        }
    }

    pub async fn monitor_transactions(&self, eth_tx_hash: H256, cardano_tx_id: &str) {
        loop {
            // Check if Ethereum transaction is confirmed
            let eth_confirmed = self.eth_observer.is_transaction_confirmed(eth_tx_hash).await.unwrap_or(false);
            // Check if Cardano transaction is confirmed
            let cardano_confirmed = self.cardano_observer.is_transaction_confirmed(cardano_tx_id).await.unwrap_or(false);

            // If both transactions are confirmed, execute the swap
            if eth_confirmed && cardano_confirmed {
                self.execute_swap().await;
                break;
            }

            // Sleep before checking again to reduce load
            sleep(Duration::from_secs(10)).await;
        }
    }

    async fn execute_swap(&self) {
        // Logic to execute swap once both transactions are confirmed
        println!("Both transactions confirmed. Executing swap...");
        // Implement swap logic here
    }
}
