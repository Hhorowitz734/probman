// tests/problem_routes.rs

use actix_web::{App, test};
use dotenvy::dotenv;
use judge::routes::problem::{create_problem, get_all_problems, get_problem_by_id};
use serde_json::json;
use sqlx::PgPool;

#[actix_rt::test]
async fn test_create_and_delete_problem() {
    dotenvy::dotenv().ok();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(create_problem)
            .service(judge::routes::problem::delete_problem_by_id)
            .service(get_problem_by_id),
    )
    .await;

    // Step 1: Create a new problem
    let test_problem = json!({
        "title": "Temporary Problem",
        "description": "This will be deleted immediately",
        "difficulty": "Easy",
        "input_type": "int",
        "output_type": "int"
    });

    let create_resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/problems")
            .set_json(&test_problem)
            .to_request(),
    )
    .await;

    assert_eq!(
        create_resp.status(),
        actix_web::http::StatusCode::CREATED,
        "Problem creation failed: {}",
        create_resp.status()
    );

    // Step 2: Get ID of the problem we just inserted
    let row = sqlx::query!(
        "SELECT id FROM problems WHERE title = $1",
        "Temporary Problem"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to retrieve created problem");

    let problem_id = row.id;

    // Step 3: Delete the problem
    let delete_uri = format!("/problem/{}", problem_id);
    let delete_req = test::TestRequest::delete().uri(&delete_uri).to_request();

    let delete_resp = test::call_service(&app, delete_req).await;

    assert!(
        delete_resp.status().is_success(),
        "Problem deletion failed: {}",
        delete_resp.status()
    );

    // Step 4: Ensure the problem is gone
    let get_uri = format!("/problem/{}", problem_id);
    let get_req = test::TestRequest::get().uri(&get_uri).to_request();

    let get_resp = test::call_service(&app, get_req).await;

    assert_eq!(
        get_resp.status(),
        actix_web::http::StatusCode::NOT_FOUND,
        "Expected 404 when fetching deleted problem, got {}",
        get_resp.status()
    );
}
