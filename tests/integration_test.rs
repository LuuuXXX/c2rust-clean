use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a test environment with .c2rust folder
fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let c2rust_dir = temp_dir.path().join(".c2rust");
    fs::create_dir(&c2rust_dir).unwrap();
    temp_dir
}

#[test]
fn test_build_command_basic() {
    let temp_dir = setup_test_env();
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--cmd")
        .arg("echo")
        .arg("building");

    cmd.assert().success();
}

#[test]
fn test_build_with_multiple_args() {
    let temp_dir = setup_test_env();
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--cmd")
        .arg("echo")
        .arg("test")
        .arg("build");

    cmd.assert().success();
}

#[test]
fn test_build_with_hyphenated_args() {
    let temp_dir = setup_test_env();
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--cmd")
        .arg("ls")
        .arg("-la");

    cmd.assert().success();
}

#[test]
fn test_build_with_double_dash_separator() {
    let temp_dir = setup_test_env();
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--cmd")
        .arg("echo")
        .arg("--")
        .arg("hello")
        .arg("world");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn test_build_from_subdirectory() {
    let temp_dir = setup_test_env();
    let build_dir = temp_dir.path().join("build");
    fs::create_dir(&build_dir).unwrap();
    
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(&build_dir)
        .arg("clean")
        .arg("--cmd")
        .arg("echo")
        .arg("from_subdirectory");

    cmd.assert().success();
    
    // Verify config was saved
    let config_path = temp_dir.path().join(".c2rust").join("config.json");
    assert!(config_path.exists());
    let config_content = fs::read_to_string(config_path).unwrap();
    assert!(config_content.contains("\"build_dir\": \"build\""));
}

#[test]
fn test_missing_c2rust_directory() {
    let temp_dir = TempDir::new().unwrap();
    // Don't create .c2rust directory
    
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--cmd")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(".c2rust"));
}

#[test]
fn test_missing_build_cmd() {
    let temp_dir = setup_test_env();
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without --cmd, should fail
    cmd.current_dir(temp_dir.path())
        .arg("clean");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--cmd"));
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
    
    cmd.arg("clean").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Execute clean command"))
        .stdout(predicate::str::contains("--cmd"));
}

#[test]
fn test_hyphenated_args_with_separator() {
    let temp_dir = setup_test_env();
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--cmd")
        .arg("echo")
        .arg("--")
        .arg("-la")
        .arg("--verbose");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("-la --verbose"));
}

