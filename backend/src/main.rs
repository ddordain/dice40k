use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use handlers::simulate::simulate;

mod handlers;
mod logic;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("https://badroll.ddordain.com")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"]),
            )
            .route("/simulate", web::post().to(simulate))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
