# c2rust-clean

C project build artifact cleaning tool for c2rust workflow.

## Overview

`c2rust-clean` is a command-line tool that executes clean commands for C build projects and automatically saves the configuration using `c2rust-config`. This tool is part of the c2rust workflow for managing the transition from C to Rust projects.

## Installation

### From Source

```bash
cargo install --path .
```

Or build locally:

```bash
cargo build --release
# Binary will be in target/release/c2rust-clean
```

## Prerequisites

This tool requires `c2rust-config` to be installed. Install it from:
https://github.com/LuuuXXX/c2rust-config

## Usage

### Basic Command

```bash
c2rust-clean clean --dir <directory> -- <clean_command>
```

### Command Format

```bash
c2rust-clean clean [--feature <feature>] --dir <directory> -- <clean_command>
```

### Parameters

- `--dir <directory>` - **Required**. Directory where the clean command will be executed
- `-- <clean_command>` - **Required**. The actual clean command to execute (e.g., `make clean`)
- `--feature <feature>` - **Optional**. Feature name for configuration (default: "default")

### Examples

#### Basic usage with make clean
```bash
c2rust-clean clean --dir build -- make clean
```

#### Clean with a specific feature
```bash
c2rust-clean clean --feature debug --dir build -- make clean
```

#### Custom clean command
```bash
c2rust-clean clean --dir . -- rm -rf target
```

#### Clean with multiple arguments
```bash
c2rust-clean clean --dir build -- cargo clean --target-dir ./target
```

## How It Works

1. **Validation**: Checks if `c2rust-config` is installed
2. **Execution**: Runs the specified clean command in the target directory
3. **Configuration**: Saves the configuration using `c2rust-config`:
   - Saves `clean.dir` with the directory path
   - Saves `clean` with the complete clean command

## Configuration Storage

The tool uses `c2rust-config` to store:
- `clean.dir`: The directory where clean commands are executed
- `clean`: The clean command itself

These configurations can be retrieved later using `c2rust-config` for workflow automation.

## Error Handling

The tool provides clear error messages for common issues:

- **c2rust-config not found**: Install c2rust-config before using this tool
- **Command execution failed**: The clean command returned a non-zero exit code
- **Configuration save failed**: Unable to save configuration to c2rust-config

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --test integration_test
```

Note: Some integration tests require `c2rust-config` to be installed.

## Project Structure

```
src/
├── main.rs           # CLI entry point and argument parsing
├── error.rs          # Error type definitions
├── executor.rs       # Command execution logic
└── config_helper.rs  # c2rust-config interaction helpers

tests/
└── integration_test.rs  # Integration tests
```

## License

See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.