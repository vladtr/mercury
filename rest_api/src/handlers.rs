use actix_web::{web, HttpResponse, Responder};
use chain_observers::eth::EthObserver;
use chain_observers::cardano::CardanoObserver;
use executor::executor::Executor;
use web3::types::H256;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
struct SwapRequest {
    eth_tx_hash: String,
    cardano_tx_id: String,
}


async fn get_latest_block() -> impl Responder {
    let eth_observer = EthObserver::new("wss://mainnet.infura.io/ws/v3/122332323223").await;
    match eth_observer.get_latest_block_number().await {
        Ok(block_number) => HttpResponse::Ok().body(format!("Latest block number: {}", block_number)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching block number: {:?}", e)),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/latest_block").route(web::get().to(get_latest_block)));
    cfg.service(web::resource("/cardano_block").route(web::get().to(get_cardano_block)));
    cfg.service(web::resource("/initiate_swap").route(web::post().to(initiate_swap)));

}

async fn get_cardano_block() -> impl Responder {
    let cardano_observer = CardanoObserver::new("122332323223");
    match cardano_observer.get_latest_block().await {
        Ok(block) => HttpResponse::Ok().body(format!("Cardano Latest block height: {}, hash: {}", block.height, block.hash)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching Cardano block: {:?}", e)),
    }
}

async fn initiate_swap(swap_request: web::Json<SwapRequest>) -> impl Responder {
    let eth_observer = EthObserver::new("wss://mainnet.infura.io/ws/v3/122332323223").await;
    let cardano_observer = CardanoObserver::new("122332323223");

    let executor = Executor::new(eth_observer, cardano_observer);

    // Convert transaction hash from string to H256
    let eth_tx_hash = H256::from_str(&swap_request.eth_tx_hash).expect("Invalid transaction hash format");

    // Start monitoring transactions asynchronously
    tokio::spawn(async move {
        executor.monitor_transactions(eth_tx_hash, &swap_request.cardano_tx_id).await;
    });

    HttpResponse::Ok().body("Swap monitoring started")
}
