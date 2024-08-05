use reqwest::Client;
use serde::Deserialize;

pub struct CardanoObserver {
    client: Client,
    api_key: String,
}

#[derive(Deserialize, Debug)]
pub struct Block {
    pub height: u64,
    pub hash: String,
}

impl CardanoObserver {
    pub fn new(api_key: &str) -> Self {
        let client = Client::new();
        CardanoObserver {
            client,
            api_key: api_key.to_string(),
        }
    }

    pub async fn get_latest_block(&self) -> Result<Block, reqwest::Error> {
        let url = "https://cardano-mainnet.blockfrost.io/api/v0/blocks/latest";
        let block: Block = self
            .client
            .get(url)
            .header("project_id", &self.api_key)
            .send()
            .await?
            .json()
            .await?;
        Ok(block)
    }

    pub async fn is_transaction_confirmed(&self, tx_id: &str) -> Result<bool, reqwest::Error> {
        let url = format!("https://cardano-mainnet.blockfrost.io/api/v0/txs/{}", tx_id);
        let tx_status: serde_json::Value = self
            .client
            .get(&url)
            .header("project_id", &self.api_key)
            .send()
            .await?
            .json()
            .await?;
        
        // Check if the transaction is confirmed
        if let Some(confirmations) = tx_status.get("confirmations") {
            return Ok(confirmations.as_u64().unwrap_or(0) > 0);
        }
        
        Ok(false)
    }
}
