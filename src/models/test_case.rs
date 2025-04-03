// src/models/test_case.rs

use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Serialize)]
pub struct TestCase {
    pub id: Uuid,
    pub problem_id: Uuid,
    pub input: String,
    pub expected_output: String,
    pub visibility: Option<String>,
}
