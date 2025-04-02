// src/main.rs

use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Load in env
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to the database");

    
    // Start HttpServer
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await



}

