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
}
