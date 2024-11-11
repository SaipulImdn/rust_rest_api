use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod config;
mod controllers;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    HttpServer::new(move || {
        App::new()
            .configure(routes::init_routes)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
