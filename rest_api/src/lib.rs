pub mod handlers;

use actix_web::{web, App, HttpServer};

pub fn run() {
    #[actix_web::main] // This uses Tokio 1.x runtime by default
    async fn start_server() -> std::io::Result<()> {
        HttpServer::new(|| App::new().configure(handlers::configure))
            .bind("127.0.0.1:8080")?
            .run()
            .await
    }

    // Execute the server startup function
    start_server().unwrap();
}
