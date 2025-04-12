// src/routes/mod.rs

pub mod problem;
pub mod submission;
pub mod test_case;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(problem::get_problem_by_id)
            .service(problem::create_problem)
            .service(problem::get_all_problems)
            .service(submission::create_submission)
            .service(submission::get_submission_result)
            .service(test_case::create_test_case)
            .service(test_case::get_test_cases_for_problem),
    );
}
