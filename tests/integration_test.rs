use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper function to create a mock c2rust-config script for testing
fn create_mock_c2rust_config(temp_dir: &TempDir) -> PathBuf {
    let mock_script = temp_dir.path().join("mock-c2rust-config");

    #[cfg(unix)]
    {
        let mut script = fs::File::create(&mock_script).unwrap();
        writeln!(script, "#!/bin/bash").unwrap();
        writeln!(script, "if [ \"$1\" = \"--help\" ]; then").unwrap();
        writeln!(script, "  exit 0").unwrap();
        writeln!(script, "fi").unwrap();
        writeln!(script, "exit 0").unwrap();
        
        // Make script executable
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&mock_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&mock_script, perms).unwrap();
        
        mock_script
    }

    #[cfg(windows)]
    {
        let mock_script = mock_script.with_extension("bat");
        let mut script = fs::File::create(&mock_script).unwrap();
        writeln!(script, "@echo off").unwrap();
        writeln!(script, "if \"%1\"==\"--help\" exit /b 0").unwrap();
        writeln!(script, "exit /b 0").unwrap();
        
        mock_script
    }
}

#[test]
fn test_clean_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    let mock_config = create_mock_c2rust_config(&temp_dir);
    
    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.env("C2RUST_CONFIG", &mock_config)
        .current_dir(temp_dir.path())
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("cleaning");

    cmd.assert().success();
}

#[test]
fn test_clean_with_multiple_args() {
    let temp_dir = TempDir::new().unwrap();
    let mock_config = create_mock_c2rust_config(&temp_dir);

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.env("C2RUST_CONFIG", &mock_config)
        .current_dir(temp_dir.path())
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
    let mock_config = create_mock_c2rust_config(&temp_dir);

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.env("C2RUST_CONFIG", &mock_config)
        .current_dir(temp_dir.path())
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
    let mock_config = create_mock_c2rust_config(&temp_dir);

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Without --, still works for simple commands without flags
    cmd.env("C2RUST_CONFIG", &mock_config)
        .current_dir(temp_dir.path())
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
    let mock_config = create_mock_c2rust_config(&temp_dir);
    
    // Don't create .c2rust directory - should use current dir as project root

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.env("C2RUST_CONFIG", &mock_config)
        .env_remove("C2RUST_PROJECT_ROOT")  // Ensure env var is not set to test fallback logic
        .current_dir(temp_dir.path())
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
    let mock_config = create_mock_c2rust_config(&temp_dir);
    let c2rust_dir = temp_dir.path().join(".c2rust");
    fs::create_dir(&c2rust_dir).unwrap();
    
    // Create a subdirectory for running the command
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.env("C2RUST_CONFIG", &mock_config)
        .env_remove("C2RUST_PROJECT_ROOT")  // Ensure env var is not set to test .c2rust search logic
        .current_dir(&sub_dir)
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
    let mock_config = create_mock_c2rust_config(&temp_dir);
    
    // Create a test file in the temp directory
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.env("C2RUST_CONFIG", &mock_config)
        .current_dir(temp_dir.path())
        .arg("clean")
        .arg("--")
        .arg("ls")
        .arg("test.txt");

    cmd.assert()
        .success();
}

#[test]
fn test_config_tool_not_found() {
    // Test that when C2RUST_CONFIG points to a non-existent path,
    // the CLI exits with ConfigToolNotFound error
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    cmd.current_dir(temp_dir.path())
        .env("C2RUST_CONFIG", "/nonexistent/path/to/c2rust-config")
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("c2rust-config not found"));
}

#[test]
fn test_feature_parameter_with_mock_config() {
    // Test that --feature parameter is passed through to c2rust-config
    let temp_dir = TempDir::new().unwrap();
    
    // Create a mock c2rust-config script that logs its arguments
    let mock_script = temp_dir.path().join("mock-c2rust-config");
    let log_file = temp_dir.path().join("config.log");

    #[cfg(unix)]
    {
        let mut script = fs::File::create(&mock_script).unwrap();
        writeln!(script, "#!/bin/bash").unwrap();
        writeln!(script, "if [ \"$1\" = \"--help\" ]; then").unwrap();
        writeln!(script, "  exit 0").unwrap();
        writeln!(script, "fi").unwrap();
        writeln!(script, "echo \"$@\" >> {:?}", log_file).unwrap();
        writeln!(script, "exit 0").unwrap();
        
        // Make script executable
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&mock_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&mock_script, perms).unwrap();
    }

    #[cfg(windows)]
    {
        let mock_script = mock_script.with_extension("bat");
        let mut script = fs::File::create(&mock_script).unwrap();
        writeln!(script, "@echo off").unwrap();
        writeln!(script, "if \"%1\"==\"--help\" exit /b 0").unwrap();
        writeln!(script, "echo %* >> {:?}", log_file).unwrap();
        writeln!(script, "exit /b 0").unwrap();
    }

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    #[cfg(unix)]
    cmd.env("C2RUST_CONFIG", &mock_script);
    #[cfg(windows)]
    cmd.env("C2RUST_CONFIG", mock_script.with_extension("bat"));
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--feature")
        .arg("myfeature")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("✓ Configuration saved"));

    // Verify that --feature myfeature was passed to c2rust-config
    let log_content = fs::read_to_string(&log_file).unwrap();
    assert!(log_content.contains("--feature"), "Expected --feature in log");
    assert!(log_content.contains("myfeature"), "Expected myfeature in log");
}

#[test]
fn test_default_feature_when_not_specified() {
    // Test that when --feature is not specified, "default" is used
    let temp_dir = TempDir::new().unwrap();
    
    // Create a mock c2rust-config script that logs its arguments
    let mock_script = temp_dir.path().join("mock-c2rust-config-default");
    let log_file = temp_dir.path().join("config-default.log");

    #[cfg(unix)]
    {
        let mut script = fs::File::create(&mock_script).unwrap();
        writeln!(script, "#!/bin/bash").unwrap();
        writeln!(script, "if [ \"$1\" = \"--help\" ]; then").unwrap();
        writeln!(script, "  exit 0").unwrap();
        writeln!(script, "fi").unwrap();
        writeln!(script, "echo \"$@\" >> {:?}", log_file).unwrap();
        writeln!(script, "exit 0").unwrap();
        
        // Make script executable
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&mock_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&mock_script, perms).unwrap();
    }

    #[cfg(windows)]
    {
        let mock_script = mock_script.with_extension("bat");
        let mut script = fs::File::create(&mock_script).unwrap();
        writeln!(script, "@echo off").unwrap();
        writeln!(script, "if \"%1\"==\"--help\" exit /b 0").unwrap();
        writeln!(script, "echo %* >> {:?}", log_file).unwrap();
        writeln!(script, "exit /b 0").unwrap();
    }

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    #[cfg(unix)]
    cmd.env("C2RUST_CONFIG", &mock_script);
    #[cfg(windows)]
    cmd.env("C2RUST_CONFIG", mock_script.with_extension("bat"));
    
    cmd.current_dir(temp_dir.path())
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .success();

    // Verify that --feature default was passed to c2rust-config
    let log_content = fs::read_to_string(&log_file).unwrap();
    assert!(log_content.contains("--feature"), "Expected --feature in log");
    assert!(log_content.contains("default"), "Expected default in log");
}

#[test]
fn test_c2rust_project_root_env_var() {
    // Test that C2RUST_PROJECT_ROOT environment variable is used when set
    let temp_dir = TempDir::new().unwrap();
    let mock_config = create_mock_c2rust_config(&temp_dir);
    
    // Create a subdirectory for running the command
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Set C2RUST_PROJECT_ROOT to the temp_dir
    cmd.env("C2RUST_CONFIG", &mock_config)
        .env("C2RUST_PROJECT_ROOT", temp_dir.path())
        .current_dir(&sub_dir)
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains(format!("Project root: {}", temp_dir.path().display())))
        .stderr(predicate::str::contains("Relative clean directory: subdir"));
}

#[test]
fn test_c2rust_project_root_overrides_search() {
    // Test that C2RUST_PROJECT_ROOT takes precedence over .c2rust directory search
    let temp_dir = TempDir::new().unwrap();
    let mock_config = create_mock_c2rust_config(&temp_dir);
    
    // Create .c2rust in temp_dir
    let c2rust_dir = temp_dir.path().join(".c2rust");
    fs::create_dir(&c2rust_dir).unwrap();
    
    // Create another directory to use as override
    let override_dir = TempDir::new().unwrap();
    
    // Create a subdirectory in temp_dir for running the command
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Set C2RUST_PROJECT_ROOT to override_dir (different from .c2rust location)
    cmd.env("C2RUST_CONFIG", &mock_config)
        .env("C2RUST_PROJECT_ROOT", override_dir.path())
        .current_dir(&sub_dir)
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .success()
        // Should use override_dir, not temp_dir where .c2rust exists
        .stderr(predicate::str::contains(format!("Project root: {}", override_dir.path().display())));
}

#[test]
fn test_c2rust_project_root_used_directly() {
    // Test that when C2RUST_PROJECT_ROOT is set, it is used directly
    // without validation (trusting upstream tools that set it)
    let temp_dir = TempDir::new().unwrap();
    let mock_config = create_mock_c2rust_config(&temp_dir);
    
    // Create .c2rust in temp_dir (won't be used)
    let c2rust_dir = temp_dir.path().join(".c2rust");
    fs::create_dir(&c2rust_dir).unwrap();
    
    // Create another directory to use as the project root via env var
    let project_root_dir = TempDir::new().unwrap();
    
    // Create a subdirectory for running the command
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();

    let mut cmd = Command::cargo_bin("c2rust-clean").unwrap();
    
    // Set C2RUST_PROJECT_ROOT to project_root_dir
    // This should be used directly, even though .c2rust exists in temp_dir
    cmd.env("C2RUST_CONFIG", &mock_config)
        .env("C2RUST_PROJECT_ROOT", project_root_dir.path())
        .current_dir(&sub_dir)
        .arg("clean")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .success()
        // Should use the C2RUST_PROJECT_ROOT value directly
        .stderr(predicate::str::contains(format!("Project root: {}", project_root_dir.path().display())));
}
