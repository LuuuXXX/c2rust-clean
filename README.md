# c2rust-clean

C 项目构建产物清理工具，用于 c2rust 工作流。

## 概述

`c2rust-clean` 是一个命令行工具，用于在指定目录执行 C 项目清理和构建命令。该工具是 c2rust 工作流的一部分，用于管理从 C 到 Rust 项目的转换过程。

该工具会自动检测项目根目录（通过 `.c2rust` 文件夹），并保存当前工作目录相对于项目根目录的路径。

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

### 前置要求

在使用 `c2rust-clean` 之前，您需要在项目根目录创建一个 `.c2rust` 文件夹：

```bash
mkdir .c2rust
```

该工具会自动检测这个文件夹，并将当前工作目录相对于 `.c2rust` 所在目录的路径保存到配置文件中。

### 基本命令

```bash
c2rust-clean clean --cmd <清理命令> [参数...]
```

或者使用 `--` 分隔符来传递命令参数：

```bash
c2rust-clean clean --cmd <清理命令> -- [参数...]
```

### 参数说明

- `--cmd <清理命令> [参数...]` - **必需**。实际要执行的清理命令及其参数（例如：`make clean`）
- `--` - **可选**。用于分隔 c2rust-clean 的参数和要传递给清理命令的参数

**注意**：不再需要 `--dir` 参数。工具会自动检测 `.c2rust` 文件夹并保存当前工作目录。

### 使用示例

#### 使用 make 清理项目

```bash
cd /path/to/project/build
c2rust-clean clean --cmd make clean
```

#### 使用 cmake 清理项目

```bash
cd /path/to/project/build
c2rust-clean clean --cmd cmake --build . --target clean
```

#### 使用 -- 分隔符传递参数

```bash
cd /path/to/project/build
c2rust-clean clean --cmd make -- clean all
```

#### 使用带连字符的参数

```bash
cd /path/to/project
c2rust-clean clean --cmd cargo clean --target-dir ./target
```

或使用 `--` 分隔符：

```bash
c2rust-clean clean --cmd cargo -- clean --target-dir ./target
```

#### 自定义清理命令

```bash
cd /path/to/project
c2rust-clean clean --cmd rm -rf build
```

或：

```bash
c2rust-clean clean --cmd rm -- -rf build
```

#### 带多个参数的清理命令

```bash
cd /path/to/project/build
c2rust-clean clean --cmd find . -name "*.o" -delete
```

或：

```bash
c2rust-clean clean --cmd find -- . -name "*.o" -delete
```

## 工作原理

1. **查找项目根目录**: 从当前目录开始向上查找 `.c2rust` 文件夹
2. **保存配置**: 自动计算并保存当前目录相对于 `.c2rust` 所在目录的相对路径到 `.c2rust/config.json`
3. **执行命令**: 在保存的构建目录中运行指定的清理命令，并实时显示输出：
   - 正在执行的完整命令
   - 命令的标准输出 (stdout) - 实时显示
   - 命令的标准错误 (stderr) - 实时显示
   - 命令的退出状态

## 配置文件

工具会在 `.c2rust/config.json` 中保存配置信息：

```json
{
  "build_dir": "build"
}
```

这个文件会在首次运行时自动创建。`build_dir` 字段存储的是相对于 `.c2rust` 所在目录的相对路径。

## 输出示例

执行命令时，工具会显示详细的输出信息：

```
Executing command: make clean
In directory: /path/to/project/build

rm -f *.o
rm -f myapp

Exit code: 0

Clean command executed successfully.
```

## 错误处理

该工具为常见问题提供清晰的错误消息：

- **缺少必需参数**: 未提供 --cmd 参数
- **找不到 .c2rust 目录**: 当前目录或父目录中不存在 `.c2rust` 文件夹
- **命令执行失败**: 清理命令返回了非零退出代码
- **目录不存在**: 配置中保存的构建目录不存在

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