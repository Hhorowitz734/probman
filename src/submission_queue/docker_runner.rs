// src/submission_queue/docker_runner.rs


use uuid::Uuid;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;


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
                return "Accepted".to_string();
            } else {
                println!("Stderr: \n{}", String::from_utf8_lossy(&result.stderr));
                return "Runtime Error".to_string();
            }
        }
        Err(e) => {
            eprintln!("Docker run failed: {:?}", e);
            return "Judge Error".to_string();
        }
    }

    "Accepted".to_string()
}
