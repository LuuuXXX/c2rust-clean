use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_clean_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean")
        .arg("--dir")
        .arg(dir_path)
        .arg("--")
        .arg("echo")
        .arg("cleaning");

    cmd.assert().success();
}

#[test]
fn test_clean_with_multiple_args() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean")
        .arg("--dir")
        .arg(dir_path)
        .arg("--")
        .arg("echo")
        .arg("test")
        .arg("clean");

    cmd.assert().success();
}

#[test]
fn test_clean_with_hyphenated_args() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean")
        .arg("--dir")
        .arg(dir_path)
        .arg("--")
        .arg("ls")
        .arg("-la");

    cmd.assert().success();
}

#[test]
fn test_missing_dir() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without --dir, should fail
    cmd.arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--dir"));
}

#[test]
fn test_missing_cmd() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without command after --, should fail
    cmd.arg("clean")
        .arg("--dir")
        .arg(dir_path);

    cmd.assert()
        .failure();
}

#[test]
fn test_missing_all_arguments() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without any arguments, should fail
    cmd.arg("clean");

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
fn test_clean_subcommand_help() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Execute clean command"))
        .stdout(predicate::str::contains("--dir"))
        .stdout(predicate::str::contains("COMMAND"));
}

#[test]
fn test_nonexistent_directory() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean")
        .arg("--dir")
        .arg("/nonexistent/directory/path")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Directory does not exist"));
}

#[test]
fn test_path_is_not_directory() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();
    
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean")
        .arg("--dir")
        .arg(file_path)
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Path is not a directory"));
}
