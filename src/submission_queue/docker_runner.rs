// src/submission_queue/docker_runner.rs


use uuid::Uuid;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub async fn run_docker_submission(
    submission_id: Uuid,
    code: String
) -> String {
    // Create directory for this submission
    let dir = PathBuf::from(format!("/tmp/judge/{}", submission_id));
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Failed to create submission dir: {:?}", e);
        return "Judge Error".into();
    }

    // Write code to run.py
    let code_path = dir.join("run.py");
    match File::create(&code_path).and_then(|mut f| f.write_all(code.as_bytes())) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to write code: {:?}", e);
            return "Judge Error".into();
        }
    }

    // TODO: Add Docker logic here

    "Accepted".to_string()
}
