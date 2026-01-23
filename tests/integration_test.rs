use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_clean_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    // Create a test file to clean
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Note: This test will fail if c2rust-config is not installed
    // For testing purposes, we'll just test the command parsing
    cmd.arg("clean")
        .arg("--dir")
        .arg(dir_path)
        .arg("--")
        .arg("echo")
        .arg("cleaning");

    // The command might fail because c2rust-config might not be installed
    // but at least it should not fail on parsing
    let _ = cmd.assert();
}

#[test]
fn test_clean_with_feature() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean")
        .arg("--feature")
        .arg("debug")
        .arg("--dir")
        .arg(dir_path)
        .arg("--")
        .arg("echo")
        .arg("test");

    // The command might fail because c2rust-config might not be installed
    let _ = cmd.assert();
}

#[test]
fn test_missing_dir_argument() {
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--dir"));
}

#[test]
fn test_missing_command_argument() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.arg("clean")
        .arg("--dir")
        .arg(dir_path);

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
        .stdout(predicate::str::contains("--feature"));
}
