// src/submission_queue/redis_worker.rs

use crate::submission_queue::docker_runner;
use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

const MAX_CONCURRENT_SUBMISSIONS: usize = 4;

pub async fn start_redis_worker(mut con: MultiplexedConnection, pool: PgPool) {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_SUBMISSIONS));

    loop {
        match con
            .blpop::<_, Option<(String, String)>>("submission_queue", 0.0)
            .await
        {
            Ok(Some((_, payload))) => {
                let permit = match semaphore.clone().acquire_owned().await {
                    Ok(p) => p,
                    Err(_) => {
                        eprintln!("Semaphore closed unexpectedly");
                        continue;
                    }
                };

                let pool = pool.clone(); // needed because pool is &mut
                tokio::spawn(async move {
                    // Drop permit when done
                    let _permit = permit;

                    if let Ok(json) = serde_json::from_str::<Value>(&payload) {
                        let maybe_id = json.get("submission_id").and_then(|v| v.as_str());
                        let maybe_code = json.get("code").and_then(|v| v.as_str());

                        if let (Some(id), Some(code)) = (maybe_id, maybe_code) {
                            if let Ok(submission_id) = Uuid::parse_str(id) {
                                let verdict = match docker_runner::run_docker_submission(
                                    submission_id,
                                    code.to_string(),
                                    &pool,
                                )
                                .await
                                {
                                    Ok(v) => v,
                                    Err(e) => {
                                        eprintln!("Execution error: {:?}", e);
                                        "Judge Error".to_string()
                                    }
                                };

                                
                                let (short_verdict, detail) = if verdict.starts_with("Wrong Answer") {
                                    ("Wrong Answer", Some(verdict))
                                } else if verdict.starts_with("Runtime Error") {
                                    ("Runtime Error", Some(verdict))
                                } else if verdict.starts_with("Time Limit Exceeded") {
                                    ("Time Limit Exceeded", Some(verdict))
                                } else if verdict.starts_with("Compile Error") {
                                    ("Compile Error", Some(verdict))
                                } else if verdict == "Accepted" {
                                    ("Accepted", None)
                                } else {
                                    ("Judge Error", Some(verdict))
                                };

                                match sqlx::query!(
                                    "UPDATE submissions SET verdict = $1, verdict_detail = $2 WHERE id = $3",
                                    short_verdict,
                                    detail,
                                    submission_id
                                )
                                .execute(&pool)
                                .await {
                                    Ok(_) => println!("Processed submission: {}", submission_id),
                                    Err(e) => eprintln!("Failed to update verdict: {:?}", e),
                                }
                            }
                        }
                    }
                });
            }

            Ok(None) => {
                continue;
            }

            Err(e) => {
                eprintln!("Redis error: {:?}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
