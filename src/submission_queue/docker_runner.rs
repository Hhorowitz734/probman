// src/submission_queue/docker_runner.rs


use uuid::Uuid;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use sqlx::PgPool;

pub async fn run_docker_submission(
    submission_id: Uuid,
    code: String,
    pool: &PgPool
) -> Result<String, sqlx::Error> {
    
    // Fetch test cases corresponding to problem
    let row = sqlx::query!(
        "SELECT problem_id FROM submissions WHERE id = $1",
        submission_id
    )
    .fetch_one(pool)
    .await?;
    
    let problem_id = row.problem_id; // got problem id

    // Now, grab the test cases
    let test_cases = sqlx::query!(
        "SELECT input, expected_output FROM test_cases WHERE problem_id = $1",
        problem_id
    )
    .fetch_all(pool)
    .await?;

    // Create directory for this submission
    let dir = PathBuf::from(format!("/tmp/judge/{}", submission_id));
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Failed to create submission dir: {:?}", e);
        return Ok("Judge Error".to_string())
    }

    // Write code to run.py
    let code_path = dir.join("run.py");
    match File::create(&code_path).and_then(|mut f| f.write_all(code.as_bytes())) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to write code: {:?}", e);
            return Ok("Judge Error".to_string())
        }
    }

    // Run the code with docker
    let output = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("-v")
        .arg(format!("{}:/sandbox", dir.display()))
        .arg("python:3.10-slim")
        .arg("python3")
        .arg("/sandbox/run.py")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    match output {
        Ok(result) => {
            if result.status.success() { 
                println!("Stdout: \n{}", String::from_utf8_lossy(&result.stdout));
                Ok("Accepted".to_string()) 
            } else {
                println!("Stderr: \n{}", String::from_utf8_lossy(&result.stderr));
                return Ok("Runtime Error".to_string())
            }
        }
        Err(e) => {
            eprintln!("Docker run failed: {:?}", e);
            return Ok("Judge Error".to_string())
        }
    }

}
