// tests/submission_worker_test.rs


use std::env;
use sqlx::postgres::PgPoolOptions;
use redis::AsyncCommands;
use uuid::Uuid;
use serde_json::json;
use tokio::time::{sleep, Duration};

// Import the worker function
use judge::submission_queue::start_redis_worker;

#[tokio::test]
async fn test_submission_worker_processes_queue() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let redis_url = env::var("REDIS_URL").expect("Missing REDIS_URL");

    // Connect to Postgres
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    // Reset the test DB state (simplified for now)
    sqlx::query!("DELETE FROM submissions").execute(&pool).await.unwrap();

    // Create a test submission row with null verdict
    let submission_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO submissions (id, code) VALUES ($1, $2)",
        submission_id,
        "fn main() {}"
    )
    .execute(&pool)
    .await
    .unwrap();

    // Connect to Redis
    let redis_client = redis::Client::open(redis_url).unwrap();
    let mut redis_con = redis_client.get_multiplexed_async_connection().await.unwrap();

    // Push a task to Redis
    let task = json!({
        "submission_id": submission_id.to_string(),
        "code": "fn main() {}".to_string()
    });

    let _: () = redis_con
        .rpush("submission_queue", task.to_string())
        .await
        .unwrap();

    // Run the worker for a short while
    let worker_pool = pool.clone();
    let worker_redis = redis_client.get_multiplexed_async_connection().await.unwrap();

    tokio::spawn(async move {
        start_redis_worker(worker_redis, worker_pool).await;
    });

    // Give the worker time to process
    sleep(Duration::from_secs(6)).await;

    // Check that the verdict was updated
    let row = sqlx::query!(
        "SELECT verdict FROM submissions WHERE id = $1",
        submission_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(row.verdict, "Accepted");
}

