use std::fs;
use std::process::Command;

use crate::global::git::{
    GIT_REMOTE_REPOSITORY_URL,
    GIT_REPOSITORY_NAME,
};

pub async fn pull_git_repository() {
    let git_repository_path = "./".to_owned() + GIT_REPOSITORY_NAME;
    if let Ok(metadata) = fs::metadata(&git_repository_path) {
        if metadata.is_dir() {
            match fs::remove_dir_all(&git_repository_path) {
                Ok(_) => println!("Local git repository deleted successfully."),
                Err(e) => println!("Error deleting git repository {}: {}.", git_repository_path, e),
            }
        } else {
            println!("Git repository path is not a directory.");
        }
    } else {
        println!("Git repository does not exist.");
    }
    println!("Pulling repository..");
    let output = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(GIT_REMOTE_REPOSITORY_URL)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("Repository pulled.");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Error occurred when pulling repository: {}.", error);
    }
}