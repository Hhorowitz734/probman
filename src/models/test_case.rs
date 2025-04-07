// src/models/test_case.rs

use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NewTestCase {
    pub name: String,
    pub problem_id: Uuid,
    pub input: String,
    pub expected_output: String,
    pub input_type: String,
    pub output_type: String
}



#[derive(Debug, FromRow, Serialize)]
pub struct TestCase {
    pub name: String,
    pub id: Uuid,
    pub problem_id: Uuid,
    pub input: String,
    pub expected_output: String,
    pub input_type: String,
    pub output_type: String
}
