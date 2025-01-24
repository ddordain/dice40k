use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use handlers::simulate::simulate;
use std::env;

mod handlers;
mod logic;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".to_string());
    let allowed_origin =
        env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&allowed_origin)
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"]),
            )
            .route("/simulate", web::post().to(simulate))
    })
    .bind(bind_address)?
    .run()
    .await
}
