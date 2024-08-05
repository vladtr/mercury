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
}
