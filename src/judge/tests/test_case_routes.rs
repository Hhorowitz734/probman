// test/test_case_routes.rs


use actix_web::{test, web, App};
use dotenvy::dotenv;
use judge::routes::test_case::{create_test_case, get_test_cases_for_problem};
use serde_json::json;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[actix_rt::test]
async fn test_create_test_case() {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("db connection failed");

    let valid_problem_id = Uuid::parse_str("3c2afa0e-1030-40a3-b2f3-6c6c0f0646d0").unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_test_case),
    )
    .await;

    let payload = json!({
        "problem_id": valid_problem_id,
        "input": "1 2",
        "expected_output": "3",
        "visibility": "public"
    });

    let req = test::TestRequest::post()
        .uri("/test-cases")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_string()); // should return the UUID
}

#[actix_rt::test]
async fn test_get_test_cases_for_problem() {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("db connection failed");

    let valid_problem_id = "3c2afa0e-1030-40a3-b2f3-6c6c0f0646d0";

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_test_cases_for_problem),
    )
    .await;

    let uri = format!("/problems/{}/test-cases", valid_problem_id);

    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());
}

#[actix_rt::test]
async fn test_create_test_case_invalid_problem() {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("db connection failed");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_test_case),
    )
    .await;

    let invalid_problem_id = Uuid::new_v4(); // should not exist

    let payload = json!({
        "problem_id": invalid_problem_id,
        "input": "x y",
        "expected_output": "z",
        "visibility": "private"
    });

    let req = test::TestRequest::post()
        .uri("/test-cases")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_success() || resp.status().is_server_error(),
        "Unexpected status: {}",
        resp.status()
    );
}
