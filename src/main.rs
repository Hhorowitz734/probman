// src/main.rs


use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use redis::Client as RedisClient;

mod routes;
mod models;
mod submission_queue;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to the database");

    let redis_client = RedisClient::open(redis_url).expect("Failed to open Redis client");

    let redis_con = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to connect Redis worker");

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        submission_queue::start_redis_worker(redis_con, pool_clone).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .configure(routes::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

