use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct TestCase {
    pub id: i32,
    pub problem_id: String,
    pub input: String,
    pub expected_output: String,
    pub visibility: Option<String>,
}
