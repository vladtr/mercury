use web3::types::{H256, U64};
use web3::transports::WebSocket;
use web3::Web3;

pub struct EthObserver {
    web3: Web3<WebSocket>,
}

impl EthObserver {
    pub async fn new(url: &str) -> Self {
        let transport = WebSocket::new(url).await.unwrap();
        let web3 = Web3::new(transport);
        EthObserver { web3 }
    }

    pub async fn get_latest_block_number(&self) -> web3::Result<u64> {
        let block_number = self.web3.eth().block_number().await?;
        Ok(block_number.as_u64())
    }

    pub async fn is_transaction_confirmed(&self, tx_hash: H256) -> web3::Result<bool> {
        let receipt = self.web3.eth().transaction_receipt(tx_hash).await?;
        if let Some(receipt) = receipt {
            return Ok(receipt.status == Some(U64::from(1)));
        }
        Ok(false)
    }
}
