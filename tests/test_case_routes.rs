// test/test_case_routes.rs

use actix_web::{App, test, web};
use dotenvy::dotenv;
use judge::routes::test_case::{create_test_case, get_test_cases_for_problem};
use serde_json::json;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[actix_rt::test]
async fn test_create_valid_test_case() {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("db connection failed");

    let valid_problem_id = Uuid::parse_str("646f8692-ea04-4a60-92b1-d4e0840eaf7f").unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_test_case),
    )
    .await;

    let payload = json!({
        "name": "Valid test case",
        "problem_id": valid_problem_id,
        "input": "1 2",
        "expected_output": "3",
        "input_type": "int",
        "output_type": "int"
    });

    let req = test::TestRequest::post()
        .uri("/test-cases")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_string());
}

#[actix_rt::test]
async fn test_create_test_case_with_wrong_types() {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("db connection failed");

    let valid_problem_id = Uuid::parse_str("646f8692-ea04-4a60-92b1-d4e0840eaf7f").unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_test_case),
    )
    .await;

    let payload = json!({
        "name": "Wrong type test",
        "problem_id": valid_problem_id,
        "input": "hello",
        "expected_output": "world",
        "input_type": "string",
        "output_type": "string"
    });

    let req = test::TestRequest::post()
        .uri("/test-cases")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
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

    let payload = json!({
        "name": "Invalid problem test",
        "problem_id": Uuid::new_v4(),
        "input": "x y",
        "expected_output": "z",
        "input_type": "int",
        "output_type": "int"
    });

    let req = test::TestRequest::post()
        .uri("/test-cases")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn test_get_test_cases_for_problem() {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("db connection failed");

    let valid_problem_id = "646f8692-ea04-4a60-92b1-d4e0840eaf7f";

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_test_cases_for_problem),
    )
    .await;

    let uri = format!("/problems/{}/test-cases", valid_problem_id);

    let req = test::TestRequest::get().uri(&uri).to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());
}
