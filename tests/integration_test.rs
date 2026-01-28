use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_clean_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("cleaning");

    cmd.assert().success();
}

#[test]
fn test_clean_with_multiple_args() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test")
        .arg("clean");

    cmd.assert().success();
}

#[test]
fn test_clean_with_hyphenated_args() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--")
        .arg("ls")
        .arg("-la");

    cmd.assert().success();
}

#[test]
fn test_missing_clean_cmd() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without any command arguments, should fail (CLEAN_CMD is required)
    cmd.current_dir(temp_dir.path())
        .arg("clean");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("CLEAN_CMD"));
}

#[test]
fn test_separator_optional_for_simple_commands() {
    // Note: trailing_var_arg captures all args, so -- is technically optional
    // However, without --, arguments starting with - or -- won't work correctly
    // as they'll be interpreted as flags for c2rust-clean
    // This test shows it works for simple commands, but -- is necessary for args with hyphens
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without --, still works for simple commands without flags
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .success();
}

#[test]
fn test_project_root_fallback() {
    // Test that when no .c2rust directory is found, current dir is used as root
    let temp_dir = TempDir::new().unwrap();
    
    // Don't create .c2rust directory - should use current dir as project root

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Project root:"))
        .stderr(predicate::str::contains("Relative clean directory: ."));
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
        .stdout(predicate::str::contains("CLEAN_CMD"));
}

#[test]
fn test_project_root_detection() {
    // Create a temporary directory with .c2rust subdirectory
    let temp_dir = TempDir::new().unwrap();
    let c2rust_dir = temp_dir.path().join(".c2rust");
    fs::create_dir(&c2rust_dir).unwrap();
    
    // Create a subdirectory for running the command
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(&sub_dir)
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Project root:"))
        .stderr(predicate::str::contains("Relative clean directory: subdir"));
}

#[test]
fn test_command_execution_in_current_dir() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test file in the temp directory
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--")
        .arg("ls")
        .arg("test.txt");

    cmd.assert()
        .success();
}
