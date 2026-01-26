# c2rust-clean

C 项目构建产物清理工具，用于 c2rust 工作流。

## 概述

`c2rust-clean` 是一个命令行工具，用于执行 C 构建项目的清理命令，并自动使用 `c2rust-config` 保存配置。该工具是 c2rust 工作流的一部分，用于管理从 C 到 Rust 项目的转换过程。

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

## 前置要求

此工具需要安装 `c2rust-config`。请从以下地址安装：
https://github.com/LuuuXXX/c2rust-config

### 环境变量

- `C2RUST_CONFIG`: 可选。c2rust-config 二进制文件的路径。如果未设置，工具将在 PATH 中查找 `c2rust-config`。

## 使用方法

### 基本命令

```bash
c2rust-clean clean --dir <目录> -- <清理命令>
```

### 命令格式

```bash
c2rust-clean clean [--feature <特性名>] --dir <目录> -- <清理命令>
```

### 参数说明

- `--dir <目录>` - **必需**。执行清理命令的目录
- `-- <清理命令>` - **必需**。实际要执行的清理命令（例如：`make clean`）
- `--feature <特性名>` - **可选**。配置的特性名称（默认："default"）

### 使用示例

#### 使用 make clean 的基本用法
```bash
c2rust-clean clean --dir build -- make clean
```

#### 使用特定特性进行清理
```bash
c2rust-clean clean --feature debug --dir build -- make clean
```

#### 自定义清理命令
```bash
c2rust-clean clean --dir . -- rm -rf target
```

#### 带多个参数的清理命令
```bash
c2rust-clean clean --dir build -- cargo clean --target-dir ./target
```

#### 使用自定义 c2rust-config 路径

如果 `c2rust-config` 不在 PATH 中或您想使用特定版本：

```bash
export C2RUST_CONFIG=/path/to/custom/c2rust-config
c2rust-clean clean --dir /path/to/project -- make clean
```

## 工作原理

1. **验证**: 检查 `c2rust-config` 是否已安装
2. **执行**: 在目标目录中运行指定的清理命令
3. **配置保存**: 使用 `c2rust-config` 保存配置：
   - 保存 `clean.dir` 为目录路径
   - 保存 `clean` 为完整的清理命令

## 配置存储

该工具使用 `c2rust-config` 存储以下配置：
- `clean.dir`: 执行清理命令的目录
- `clean`: 清理命令本身

这些配置可以稍后通过 `c2rust-config` 检索，用于工作流自动化。

## 错误处理

该工具为常见问题提供清晰的错误消息：

- **找不到 c2rust-config**: 使用此工具前请先安装 c2rust-config
- **命令执行失败**: 清理命令返回了非零退出代码
- **配置保存失败**: 无法将配置保存到 c2rust-config

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

注意：某些集成测试需要安装 `c2rust-config`。

## 项目结构

```
src/
├── main.rs           # CLI 入口点和参数解析
├── error.rs          # 错误类型定义
├── executor.rs       # 命令执行逻辑
└── config_helper.rs  # c2rust-config 交互助手

tests/
└── integration_test.rs  # 集成测试
```

## 许可证

详见 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。