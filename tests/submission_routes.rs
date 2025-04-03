// tests/submission_routes.rs

use actix_web::{test, App};
use serde_json::json;
use uuid::Uuid;
use judge::routes::submission::create_submission;
use sqlx::PgPool;




#[actix_rt::test]
async fn test_create_submission() {
    // Test submitting code to an existing problem.
    
    dotenvy::dotenv().ok();

    // Connect to the existing database
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    // Set up the app with the /submissions route
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(create_submission),  // Register the submission handler
    ).await;

    // Use the valid problem ID provided
    let problem_id = "a2ce5f3a-e00a-41fb-b00b-323e44910e33".to_string();  // Correct problem ID

    // Create a new submission payload
    let submission_payload = json!({
        "problem_id": problem_id,  // Use the valid problem ID here
        "code": "print('Hello, world!')"
    });

    // Send a POST request to the /submissions route
    let req = test::TestRequest::post()
        .uri("/submissions")
        .set_json(&submission_payload)  // Set the request body as JSON
        .to_request();

    // Execute the request and get the response
    let resp = test::call_service(&app, req).await;

    // Assert that the response has a 201 Created status
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

    // Optionally, check if the body contains the submission ID or other relevant data
    let response_body: serde_json::Value = test::read_body_json(resp).await;
    assert!(response_body.is_object());
    assert!(response_body["id"].is_string());  // Check if the returned ID is a string (UUID)
}



#[actix_rt::test]
async fn test_create_submission_invalid_problem() {
    dotenvy::dotenv().ok(); // Load environment variables

    // Connect to the database
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    // Set up the service with the create_submission route
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(create_submission),
    ).await;

    let problem_id = "non-existent-id"; // Invalid problem ID

    // Try to parse the invalid problem ID
    let parsed_problem_id = Uuid::parse_str(problem_id);

    // If parsing fails, send a request with invalid ID
    if parsed_problem_id.is_err() {
        let req = test::TestRequest::post()
            .uri("/submissions")
            .set_json(&json!({
                "problem_id": problem_id,
                "code": "print('Hello, world!')"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Expect a 400 Bad Request response
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }
}

