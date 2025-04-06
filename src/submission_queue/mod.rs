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
        match con
            .lpop::<_, Option<String>>("submission_queue", None)
            .await 
        {
            Ok(Some(payload)) => {
                if let Ok(json) = serde_json::from_str::<Value>(&payload) {
                    if let (Some(id), Some(code)) = (
                        json.get("submission_id").and_then(|v| v.as_str()),
                        json.get("code").and_then(|v| v.as_str())
                    ) {
                        if let Ok(submission_id) = Uuid::parse_str(id) {
                            let verdict = run_docker_submission(
                                submission_id,
                                code.to_string()
                            ).await;

                            // Update verdict in the database
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

            Ok(None) => {
                // No task found, sleep before retrying
                sleep(Duration::from_secs(5)).await;
            }

            Err(e) => {
                eprintln!("Redis error: {:?}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

// This will eventually run code inside a Docker container
async fn run_docker_submission(
    _submission_id: Uuid,
    _code: String
) -> String {
    "Accepted".to_string()
}

