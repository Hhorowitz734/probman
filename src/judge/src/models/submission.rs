// src/models/submission.rs

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Submission {
    pub id: Uuid,
    pub problem_id: String,
    pub code: String,
    pub verdict: Option<String>,
    pub output: Option<String>,
    pub compile_error: Option<String>,
    pub submitted_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct NewSubmissionRequest {
    pub problem_id: String,
    pub code: String,
}
