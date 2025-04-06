// src/submission_queue/mod.rs

use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::Value;
use tokio::time::{sleep, Duration};

pub async fn start_redis_worker(
    mut con: MultiplexedConnection,
    pool: PgPool
) {
    loop {
        // Block until a new submission is available

        match con.blpop::<_, Option<(String, String)>>("submission_queue", 0.0).await {
            Ok(Some((_, payload))) => {
                // Parse the JSON payload from the queue
                if let Ok(json) = serde_json::from_str::<Value>(&payload) {
                    let maybe_id = json.get("submission_id").and_then(|v| v.as_str());
                    let maybe_code = json.get("code").and_then(|v| v.as_str());

                    // If both fields are present and valid
                    if let (Some(id), Some(code)) = (maybe_id, maybe_code) {
                        if let Ok(submission_id) = Uuid::parse_str(id) {
                            // Run the code submission (mocked for now)
                            let verdict = run_docker_submission(
                                submission_id,
                                code.to_string()
                            ).await;

                            // Update the verdict in the database
                            let _ = sqlx::query!(
                                "UPDATE submissions SET verdict = $1 WHERE id = $2",
                                verdict,
                                submission_id
                            )
                            .execute(&pool)
                            .await;

                            println!("Processed submission: {}", submission_id);
                        }
                    }
                }
            }

            // Should not occur with BLPOP timeout = 0, but fallback just in case
            Ok(None) => {
                continue;
            }

            // Redis error — print and retry after short delay
            Err(e) => {
                eprintln!("Redis error: {:?}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

// Placeholder for future Docker execution
async fn run_docker_submission(
    _submission_id: Uuid,
    _code: String
) -> String {
    "Accepted".to_string()
}
