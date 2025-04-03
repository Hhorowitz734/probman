// src/main.rs

use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(("0.0.0.0", 8080))? // <-- publicly accessible
    .run()
    .await
}

