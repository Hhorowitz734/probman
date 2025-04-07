// src/routes/test_case.rs

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::test_case::{TestCase, NewTestCase};

#[post("/test-cases")]
pub async fn create_test_case(
    pool: web::Data<PgPool>,
    payload: web::Json<NewTestCase>,
) -> impl Responder {
    let NewTestCase {
        name,
        problem_id,
        input,
        expected_output,
        input_type,
        output_type,
    } = payload.into_inner();

    let type_check = sqlx::query!(
        "SELECT input_type, output_type FROM problems WHERE id = $1",
        problem_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    let problem = match type_check {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::NotFound().body("Problem not found"),
        Err(e) => {
            eprintln!("Failed to query problem: {:?}", e);
            return HttpResponse::InternalServerError().body("Database error");
        }
    };

    if problem.input_type != input_type || problem.output_type != output_type {
        return HttpResponse::BadRequest().body("Input/output type does not match problem definition");
    }

    let test_case_id = Uuid::new_v4();

    let result = sqlx::query!(
        "INSERT INTO test_cases (id, name, problem_id, input, expected_output, input_type, output_type)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        test_case_id,
        name,
        problem_id,
        input,
        expected_output,
        input_type,
        output_type
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(test_case_id),
        Err(e) => {
            eprintln!("Failed to insert test case: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create test case")
        }
    }
}

#[get("/problems/{id}/test-cases")]
pub async fn get_test_cases_for_problem(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query_as::<_, TestCase>(
        "SELECT id, name, problem_id, input, expected_output, input_type, output_type
         FROM test_cases WHERE problem_id = $1"
    )
    .bind(id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(test_cases) => HttpResponse::Ok().json(test_cases),
        Err(e) => {
            eprintln!("Failed to fetch test cases: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch test cases")
        }
    }
}

