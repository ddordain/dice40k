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
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("/simulate", web::post().to(simulate))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
