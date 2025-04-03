// src/routes/submission.rs

use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::submission::NewSubmissionRequest;
use sqlx::PgPool;
use serde_json::json;



#[post("/submissions")]
pub async fn create_submission(
    pool: web::Data<PgPool>,
    payload: web::Json<NewSubmissionRequest>,
) -> impl Responder {
    let NewSubmissionRequest {
        problem_id,
        code,
    } = payload.into_inner();

    // Use the problem_id directly as it is already a UUID
    let problem_id = problem_id;

    // Validate if the problem exists
    let problem_exists = sqlx::query_scalar!("SELECT 1 FROM problems WHERE id = $1", problem_id)
        .fetch_optional(pool.get_ref())
        .await
        .map(|row| row.is_some())
        .unwrap_or(false);

    // If problem doesn't exist, return 404 Not Found
    if !problem_exists {
        return HttpResponse::NotFound().body("Problem not found");
    }

    // Insert the submission into the database
    let submission_id = Uuid::new_v4();
    let result = sqlx::query!(
        "INSERT INTO submissions (id, problem_id, code, verdict) VALUES ($1, $2, $3, $4)",
        submission_id,
        problem_id,
        code,
        "Accepted" // Use "Accepted" as a valid verdict
    )
    .execute(pool.get_ref())
    .await;

    // Return the response
    match result {
        Ok(_) => {
            let response_body = json!({
                "id": submission_id.to_string(),  // Respond with submission ID
            });
            HttpResponse::Created().json(response_body)
        },
        Err(e) => {
            // Log database error
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create submission")
        },
    }
}

