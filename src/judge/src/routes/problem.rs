// src/routes/problem.rs

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::problem::{Problem, NewProblem};

#[get("/problems")]
pub async fn get_all_problems(pool: web::Data<PgPool>) -> impl Responder {

    let problems = sqlx::query_as::<_, Problem>("SELECT * FROM problems")
        .fetch_all(pool.get_ref())
        .await;

    match problems {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Error querying problems: {:?}", e); 
            HttpResponse::InternalServerError().body("Database query failed")
        }
    }
}



#[get("/problem/{id}")]
pub async fn get_problem_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    
    let result = sqlx::query_as::<_, Problem>("SELECT * FROM problems WHERE id = $1")
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await;
    
    /*
    3 cases need to be handled here
    ------
    1 -> Problem of id is found --- Send it back to user
    2 -> Database ok but id not in it --- Send defined error
    3 -> Some other error --- panic!!
       */
    match result {
        Ok(Some(problem)) => HttpResponse::Ok().json(problem),
        Ok(None) => HttpResponse::NotFound().body("Problem not found"),
        Err(_) => HttpResponse::InternalServerError().body("Database query failed")
    }
}


#[post("/problems")]
pub async fn create_problem(
    pool: web::Data<PgPool>,
    payload: web::Json<NewProblem>,
) -> impl Responder {

    let NewProblem {
        title,
        description,
        difficulty
    } = payload.into_inner();

    let result = sqlx::query!(
        "INSERT INTO problems (title, description, difficulty) VALUES ($1, $2, $3)",
        title,
        description,
        difficulty
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to insert problem")
    }
}
