// src/routes/mod.rs

pub mod problem;
pub mod submission;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(problem::get_problem_by_id)
            .service(problem::create_problem)
            .service(problem::get_all_problems)
            .service(submission::create_submission)
    );
}
