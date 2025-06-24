mod config;
mod services;
mod routes;
mod models;
mod utils;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::from_env().expect("Erro nas variáveis de ambiente");

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .wrap(Cors::permissive())
            .configure(routes::init)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
