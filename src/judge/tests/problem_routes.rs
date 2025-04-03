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
        .uri("/problem/27fa58f8-b1bf-40c2-94d6-43cbcfa66a4f")  
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
