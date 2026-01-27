use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_build_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("build")
        .arg("--build.dir")
        .arg(dir_path)
        .arg("--build.cmd")
        .arg("echo")
        .arg("building");

    cmd.assert().success();
}

#[test]
fn test_build_with_multiple_args() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("build")
        .arg("--build.dir")
        .arg(dir_path)
        .arg("--build.cmd")
        .arg("echo")
        .arg("test")
        .arg("build");

    cmd.assert().success();
}

#[test]
fn test_build_with_hyphenated_args() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("build")
        .arg("--build.dir")
        .arg(dir_path)
        .arg("--build.cmd")
        .arg("ls")
        .arg("-la");

    cmd.assert().success();
}

#[test]
fn test_missing_build_dir() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without --build.dir, should fail
    cmd.arg("build")
        .arg("--build.cmd")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--build.dir"));
}

#[test]
fn test_missing_build_cmd() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without --build.cmd, should fail
    cmd.arg("build")
        .arg("--build.dir")
        .arg(dir_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--build.cmd"));
}

#[test]
fn test_missing_all_arguments() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without any arguments, should fail
    cmd.arg("build");

    cmd.assert()
        .failure();
}

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("c2rust-clean"))
        .stdout(predicate::str::contains("C project build artifact cleaning tool"));
}

#[test]
fn test_build_subcommand_help() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("build").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Execute build command"))
        .stdout(predicate::str::contains("--build.dir"))
        .stdout(predicate::str::contains("--build.cmd"));
}

#[test]
fn test_nonexistent_directory() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("build")
        .arg("--build.dir")
        .arg("/nonexistent/directory/path")
        .arg("--build.cmd")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Directory does not exist"));
}

#[test]
fn test_path_is_not_directory() {
    use std::fs;
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();
    
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("build")
        .arg("--build.dir")
        .arg(file_path)
        .arg("--build.cmd")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Path is not a directory"));
}
