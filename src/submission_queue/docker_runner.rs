// src/submission_queue/docker_runner.rs

use sqlx::PgPool;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use uuid::Uuid;

pub async fn run_docker_submission(
    submission_id: Uuid,
    code: String,
    pool: &PgPool,
) -> Result<String, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT problem_id FROM submissions WHERE id = $1",
        submission_id
    )
    .fetch_one(pool)
    .await?;

    let problem_id = row.problem_id;

    let test_cases = sqlx::query!(
        "SELECT name, input, expected_output FROM test_cases WHERE problem_id = $1",
        problem_id
    )
    .fetch_all(pool)
    .await?;

    let dir = PathBuf::from(format!("/tmp/judge/{}", submission_id));
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Failed to create submission dir: {:?}", e);
        return Ok("Judge Error".to_string());
    }

    let code_path = dir.join("run.py");
    if let Err(e) = File::create(&code_path).and_then(|mut f| f.write_all(code.as_bytes())) {
        eprintln!("Failed to write code: {:?}", e);
        return Ok("Judge Error".to_string());
    }

    for case in test_cases {
        let name = case.name;
        let input = case.input;
        let expected_output = case.expected_output.trim();
        let cmd = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-i") // Pass input via stdin
            .arg("-v")
            .arg(format!("{}:/sandbox", dir.display()))
            .arg("python:3.10-slim")
            .arg("python3")
            .arg("/sandbox/run.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut child = match cmd {
            Ok(child) => child,
            Err(e) => {
                eprintln!("Failed to start Docker: {:?}", e);
                return Ok("Judge Error".to_string());
            }
        };

        if let Some(mut stdin) = child.stdin.take() {
            if stdin.write_all(input.as_bytes()).await.is_err() {
                eprintln!("Failed to write to Docker stdin");
                return Ok("Judge Error".to_string());
            }
        }

        let output = match child.wait_with_output().await {
            Ok(out) => out,
            Err(e) => {
                eprintln!("Failed to read Docker output: {:?}", e);
                return Ok("Judge Error".to_string());
            }
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Stderr: \n{}", String::from_utf8_lossy(&output.stderr));
            return Ok(format!(
                "Runtime Error on test case: \"{}\"\nInput: {}\nError: {}",
                name, input, stderr
            ));        
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Write stdout and stderr to log files
        let _ = fs::write(dir.join("stdout.log"), &output.stdout);
        let _ = fs::write(dir.join("stderr.log"), &output.stderr);

        if stdout != expected_output {
            let msg = format!(
                "Wrong Answer on test case: \"{}\"\nInput: {}\nExpected: {}\nGot: {}",
                name, input, expected_output, stdout
            );

            println!("[judge] Verdict computed:\n{}", msg); // log to debug

            return Ok(msg);
        }
    }

    Ok("Accepted".to_string())
}
