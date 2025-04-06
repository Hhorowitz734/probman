// tests/problem_routes.rs

use actix_web::{test, App};
use judge::routes::problem::{get_problem_by_id, get_all_problems, create_problem};
use sqlx::PgPool;
use dotenvy::dotenv;
use serde_json::json;

#[actix_rt::test]
async fn test_get_problem_by_id() {
    // Test individual problem getter endpoint
    // This test tests the first problem, it SHOULD have a result    
    dotenv().ok();
    
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    let app = test::init_service(App::new()
        .app_data(actix_web::web::Data::new(pool.clone()))
        .service(get_problem_by_id)
    ).await;

    let req = test::TestRequest::get()
        .uri("/problem/a2ce5f3a-e00a-41fb-b00b-323e44910e33")  
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(),
            "Request failed with status: {}",
            resp.status()
    );

}


#[actix_rt::test]
async fn test_get_problem_by_id_fake_id() {
    // This test tests that 404s work properly
    // Going to send a request that SHOULD NOT return
    dotenv().ok();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to a database");

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(get_problem_by_id)
    ).await;

    let req = test::TestRequest::get()
        .uri("/problem/9999")
        .to_request();
    
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}



#[actix_rt::test]
async fn test_get_all_problems() {
    // This test tests the get all problems route
    // Of course, should properly return all problems
    dotenv().ok();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Unable to conect to database");

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(get_all_problems)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/problems")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // expect status 200 OK
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    
    // expect body is nonempty
    let body: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert!(body.len() > 0);
}



#[actix_rt::test]
async fn test_create_problem() {
    // Test create problem endpoint.
    // This should create a problem, and we'll read it back
    dotenvy::dotenv().ok();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(create_problem),
    ).await;

    let test_problem = json!({
        "title": "Test Problem",
        "description": "Test Problem",
        "difficulty": "Medium"
    });

    let req = test::TestRequest::post()
        .uri("/problems")
        .set_json(&test_problem)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
}



#[actix_rt::test]
async fn test_delete_problem_by_id() {
    // This test will create a problem, delete it, and confirm it's gone.
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
    ).await;

    // Step 1: Create a new problem
    let test_problem = json!({
        "title": "Problem to delete",
        "description": "This should be deleted",
        "difficulty": "Easy"
    });

    let create_resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/problems")
            .set_json(&test_problem)
            .to_request(),
    ).await;

    assert_eq!(
        create_resp.status(),
        actix_web::http::StatusCode::CREATED
    );

    // Step 2: Get the ID of the newly created problem (query directly)
    let row = sqlx::query!("SELECT id FROM problems WHERE title = $1", "Problem to delete")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch newly created problem");

    let problem_id = row.id;

    // Step 3: Delete the problem
    let delete_uri = format!("/problem/{}", problem_id);
    let delete_req = test::TestRequest::delete()
        .uri(&delete_uri)
        .to_request();

    let delete_resp = test::call_service(&app, delete_req).await;

    assert!(
        delete_resp.status().is_success(),
        "Delete failed: {}",
        delete_resp.status()
    );

    // Step 4: Try fetching it again to ensure it's gone
    let get_uri = format!("/problem/{}", problem_id);
    let get_req = test::TestRequest::get().uri(&get_uri).to_request();

    let get_resp = test::call_service(&app, get_req).await;

    assert_eq!(
        get_resp.status(),
        actix_web::http::StatusCode::NOT_FOUND,
        "Expected 404, got {}",
        get_resp.status()
    );
}
