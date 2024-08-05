use actix_web::{web, HttpResponse, Responder};
use chain_observers::eth::EthObserver;

async fn initiate_swap() -> impl Responder {
    HttpResponse::Ok().body("Swap initiated")
}

async fn get_latest_block() -> impl Responder {
    let eth_observer = EthObserver::new("wss://mainnet.infura.io/ws/v3/12345").await;
    match eth_observer.get_latest_block_number().await {
        Ok(block_number) => HttpResponse::Ok().body(format!("Latest block number: {}", block_number)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching block number: {:?}", e)),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/swap").route(web::post().to(initiate_swap)));
    cfg.service(web::resource("/latest_block").route(web::get().to(get_latest_block)));

}
