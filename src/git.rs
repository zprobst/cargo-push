use std::process::Command;

pub fn git_check() {
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .expect("This tool requires git. Please install git and try again.");
}

pub fn git_commit(message: &str) {
    Command::new("git")
        .args(&["commit", "-m", message])
        .status()
        .expect("Something went wrong trying to commit the new change.");
}

pub fn git_push() {
    let branch_result = Command::new("git")
        .args(&["branch"])
        .output()
        .expect("Something went wrong trying to push the branch.")
        .stdout;
    let branch = std::str::from_utf8(&branch_result)
        .expect("git sent a non-utf-8 byte stream as standard out");
    println!("Discovered Working on branch: {}", branch);

    Command::new("git")
        .args(&["push", "--tags", "-u", "origin", branch])
        .status()
        .expect("Something went wrong trying to push the branch.");
}

pub fn git_tag(version: &str) {
    Command::new("git")
        .args(&["tag", "-am", version, version])
        .status()
        .expect("Something went wrong when creating a git tag.");
}
