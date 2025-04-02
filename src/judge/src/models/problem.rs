// src/models/problem.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Problem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub difficulty: Option<String>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewProblem {
    pub title: String,
    pub description: String,
    pub difficulty: String,
}
