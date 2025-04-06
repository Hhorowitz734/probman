// tests/submission_routes.rs

use actix_web::{test, App};
use serde_json::json;
use judge::routes::submission::create_submission;
use sqlx::PgPool;




#[actix_rt::test]
async fn test_create_submission() {
    dotenvy::dotenv().ok();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    let redis_client = redis::Client::open(std::env::var("REDIS_URL").unwrap())
        .expect("Failed to create Redis client");

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(redis_client.clone()))
            .service(create_submission),
    )
    .await;

    let problem_id = "a2ce5f3a-e00a-41fb-b00b-323e44910e33".to_string();

    let submission_payload = json!({
        "problem_id": problem_id,
        "code": "print('Hello, world!')"
    });

    let req = test::TestRequest::post()
        .uri("/submissions")
        .set_json(&submission_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

    let response_body: serde_json::Value = test::read_body_json(resp).await;
    assert!(response_body.is_object());
    assert!(response_body["id"].is_string());
}

