# c2rust-clean

C 项目构建产物清理工具，用于 c2rust 工作流。

## 概述

`c2rust-clean` 是一个命令行工具，用于在指定目录执行 C 项目清理和构建命令。该工具是 c2rust 工作流的一部分，用于管理从 C 到 Rust 项目的转换过程。

## 安装

### 从源码安装

```bash
cargo install --path .
```

或本地构建：

```bash
cargo build --release
# 二进制文件将位于 target/release/c2rust-clean
```

## 使用方法

### 基本命令

```bash
c2rust-clean clean --test.dir <目录> --test.cmd <清理命令> [参数...]
```

### 参数说明

- `--test.dir <目录>` - **必需**。执行清理命令的目录
- `--test.cmd <清理命令> [参数...]` - **必需**。实际要执行的清理命令及其参数（例如：`make clean`）

### 使用示例

#### 使用 make 清理项目

```bash
c2rust-clean clean --test.dir /path/to/project --test.cmd make clean
```

#### 使用 cmake 清理项目

```bash
c2rust-clean clean --test.dir /path/to/build --test.cmd cmake --build . --target clean
```

#### 清理构建产物

```bash
c2rust-clean clean --test.dir /path/to/project --test.cmd make clean
```

#### 使用带连字符的参数

```bash
c2rust-clean clean --test.dir /path/to/project --test.cmd cargo clean --target-dir ./target
```

#### 自定义清理命令

```bash
c2rust-clean clean --test.dir . --test.cmd rm -rf build
```

#### 带多个参数的清理命令

```bash
c2rust-clean clean --test.dir build --test.cmd find . -name "*.o" -delete
```

## 工作原理

1. **参数验证**: 检查必需的 `--test.dir` 和 `--test.cmd` 参数是否已提供
2. **执行**: 在目标目录中运行指定的清理命令，并实时显示输出：
   - 正在执行的完整命令
   - 命令的标准输出 (stdout) - 实时显示
   - 命令的标准错误 (stderr) - 实时显示
   - 命令的退出状态

## 输出示例

执行命令时，工具会显示详细的输出信息：

```
Executing command: make clean
In directory: /path/to/project

rm -f *.o
rm -f myapp

Exit code: 0

Clean command executed successfully.
```

## 错误处理

该工具为常见问题提供清晰的错误消息：

- **缺少必需参数**: 未提供 --test.dir 或 --test.cmd 参数
- **命令执行失败**: 清理命令返回了非零退出代码
- **目录不存在**: 指定的目录不存在

## 开发

### 构建

```bash
cargo build
```

### 运行测试

```bash
cargo test
```

### 集成测试

```bash
cargo test --test integration_test
```

## 项目结构

```
src/
├── main.rs       # CLI 入口点和参数解析
├── error.rs      # 错误类型定义
└── executor.rs   # 命令执行逻辑

tests/
└── integration_test.rs  # 集成测试
```

## 许可证

详见 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。