// src/routes/submission.rs

use crate::models::submission::NewSubmissionRequest;
use actix_web::{HttpResponse, Responder, get, post, web};
use redis::{AsyncCommands, Client as RedisClient, aio::MultiplexedConnection};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[get("/submissions/{id}")]
pub async fn get_submission_result(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    let submission_id = id.into_inner();

    let result = sqlx::query!(
        "SELECT id, problem_id, code, verdict, verdict_detail FROM submissions WHERE id = $1",
        submission_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(row)) => {
            let json = serde_json::json!({
                "id": row.id,
                "problem_id": row.problem_id,
                "code": row.code,
                "verdict": row.verdict,
                "verdict_detail": row.verdict_detail
            });
            HttpResponse::Ok().json(json)
        }
        Ok(None) => HttpResponse::NotFound().body("Submission not found"),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch submission")
        }
    }
}

#[post("/submissions")]
pub async fn create_submission(
    pool: web::Data<PgPool>,
    redis: web::Data<RedisClient>,
    payload: web::Json<NewSubmissionRequest>,
) -> impl Responder {
    let NewSubmissionRequest { problem_id, code } = payload.into_inner();

    // Verify that the problem exists
    let problem_exists =
        match sqlx::query_scalar!("SELECT 1 FROM problems WHERE id = $1", problem_id)
            .fetch_optional(pool.get_ref())
            .await
        {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => {
                eprintln!("DB error while checking problem: {:?}", e);
                return HttpResponse::InternalServerError().body("Database error");
            }
        };

    if !problem_exists {
        return HttpResponse::NotFound().body("Problem not found");
    }

    // Insert the submission and get the generated ID
    let submission_id = match sqlx::query!(
        r#"
        INSERT INTO submissions (problem_id, code)
        VALUES ($1, $2)
        RETURNING id
        "#,
        problem_id,
        code
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(record) => record.id,
        Err(e) => {
            eprintln!("DB insert error: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to insert submission");
        }
    };

    // Format payload for Redis
    let redis_payload = json!({
        "submission_id": submission_id.to_string(),
        "problem_id": problem_id.to_string(),
        "code": code
    })
    .to_string();

    // Get a Redis connection
    let mut conn: MultiplexedConnection = match redis.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Redis connection error: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to connect to Redis");
        }
    };

    // Push to Redis queue
    if let Err(e) = conn
        .rpush::<_, _, ()>("submission_queue", redis_payload)
        .await
    {
        eprintln!("Redis push error: {:?}", e);
        return HttpResponse::InternalServerError().body("Failed to enqueue submission");
    }

    HttpResponse::Created().json(json!({ "id": submission_id }))
}
