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
# 带命令行参数（传统方式）
c2rust-clean clean --dir <目录> -- <清理命令>

# 从配置文件读取（新功能）
c2rust-clean clean
```

### 命令格式

```bash
c2rust-clean clean [--feature <特性名>] [--dir <目录>] [-- <清理命令>]
```

### 参数说明

- `--dir <目录>` - **可选**。执行清理命令的目录。如果未提供，将从配置文件读取
- `-- <清理命令>` - **可选**。实际要执行的清理命令（例如：`make clean`）。如果未提供，将从配置文件读取
- `--feature <特性名>` - **可选**。配置的特性名称（默认："default"）

**注意**：`--dir` 和 `-- <清理命令>` 至少需要在命令行或配置文件中指定一次。命令行参数会覆盖配置文件中的值。

### 配置文件支持

工具会自动从 `.c2rust/config.toml` 读取默认配置。配置文件格式如下：

```toml
[feature.default]
"clean.dir" = "build"
clean = "make clean"
```

您可以为不同的特性设置不同的配置：

```toml
[feature.debug]
"clean.dir" = "build/debug"
clean = "make clean"

[feature.release]
"clean.dir" = "build/release"
clean = "make distclean"
```

### 使用示例

#### 使用 make clean 的基本用法（显式指定参数）
```bash
c2rust-clean clean --dir build -- make clean
```

#### 首次运行后，可以不带参数运行（从配置文件读取）
```bash
# 第一次运行，保存配置
c2rust-clean clean --dir build -- make clean

# 后续运行，自动从配置读取
c2rust-clean clean
```

#### 覆盖配置文件中的目录
```bash
# 配置文件有 clean.dir = "build"，但想在其他目录执行
c2rust-clean clean --dir /path/to/other/dir
```

#### 覆盖配置文件中的命令
```bash
# 配置文件有 clean = "make clean"，但想执行其他命令
c2rust-clean clean -- make distclean
```

#### 使用特定特性进行清理
```bash
c2rust-clean clean --feature debug --dir build -- make clean

# 后续可以直接使用该特性的配置
c2rust-clean clean --feature debug
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
2. **读取配置**: 尝试从 `.c2rust/config.toml` 读取默认配置
3. **合并参数**: 命令行参数会覆盖配置文件中的值
4. **执行**: 在目标目录中运行指定的清理命令，并显示详细输出：
   - 正在执行的完整命令
   - 命令的标准输出 (stdout)
   - 命令的标准错误 (stderr)
   - 命令的退出状态
5. **配置保存**: 使用 `c2rust-config` 将最终使用的配置保存回文件：
   - 保存 `clean.dir` 为目录路径
   - 保存 `clean` 为完整的清理命令

## 配置存储

该工具使用 `c2rust-config` 在 `.c2rust/config.toml` 中存储以下配置：
- `clean.dir`: 执行清理命令的目录
- `clean`: 清理命令本身

这些配置可以稍后通过 `c2rust-config` 检索，用于工作流自动化。也可以在后续运行 `c2rust-clean` 时自动读取，无需每次都指定参数。

### 示例配置文件

```toml
[feature.default]
"clean.dir" = "build"
clean = "make clean"
```

## 输出示例

执行命令时，工具会显示详细的输出信息：

```
Executing command: make clean
In directory: build

stdout:
rm -f *.o
rm -f myapp

Exit code: 0

Clean command executed successfully and configuration saved.
```

## 错误处理

该工具为常见问题提供清晰的错误消息：

- **找不到 c2rust-config**: 使用此工具前请先安装 c2rust-config
- **缺少必需参数**: 目录或命令未在命令行或配置文件中指定
- **命令执行失败**: 清理命令返回了非零退出代码
- **配置读取失败**: 无法从 c2rust-config 读取配置
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