# c2rust-clean

C 项目构建产物清理工具，用于 c2rust 工作流。

## 概述

`c2rust-clean` 是一个命令行工具，用于在当前目录执行 C 项目清理命令。该工具是 c2rust 工作流的一部分，用于管理从 C 到 Rust 项目的转换过程。工具会自动检测项目根目录（通过查找 `.git`、`Cargo.toml` 或 `.c2rust` 等标识文件），并计算当前执行目录相对于项目根目录的路径。

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
c2rust-clean clean -- <清理命令> [参数...]
```

### 参数说明

- `--` - **分隔符**。表示后续的所有参数都是清理命令及其参数
- `<清理命令> [参数...]` - **必需**。实际要执行的清理命令及其参数（例如：`make clean`）

**注意**：工具会在**当前目录**中执行清理命令，不再需要 `--dir` 参数。

### 使用示例

#### 使用 make 清理项目

```bash
cd /path/to/project
c2rust-clean clean -- make clean
```

#### 使用 cmake 清理项目

```bash
cd /path/to/build
c2rust-clean clean -- cmake --build . --target clean
```

#### 清理构建产物

```bash
cd /path/to/project
c2rust-clean clean -- make clean
```

#### 使用带连字符的参数

```bash
cd /path/to/project
c2rust-clean clean -- cargo clean --target-dir ./target
```

#### 自定义清理命令

```bash
cd /path/to/project
c2rust-clean clean -- rm -rf build
```

#### 带多个参数的清理命令

```bash
cd build
c2rust-clean clean -- find . -name "*.o" -delete
```

## 工作原理

1. **目录检测**: 自动获取当前工作目录
2. **项目根目录查找**: 
   - 从当前目录开始向上查找包含以下标识的目录：
     - `.git` - Git 仓库根目录
     - `Cargo.toml` - Rust 项目根目录
     - `.c2rust` - c2rust 项目标识目录
   - 找到第一个包含上述任一标识的目录作为项目根目录
   - 如果未找到任何标识，则使用当前目录作为项目根目录
3. **相对路径计算**: 计算当前目录相对于项目根目录的路径
4. **命令执行**: 在当前目录中运行指定的清理命令，并实时显示输出：
   - 项目根目录路径
   - 当前执行目录
   - 相对清理目录路径
   - 正在执行的完整命令
   - 命令的标准输出 (stdout) - 实时显示
   - 命令的标准错误 (stderr) - 实时显示
   - 命令的退出状态
5. **自动提交**: 如果 `.c2rust` 目录下有任何修改，自动执行 git commit 保存修改信息

## Git 自动提交

工具会在执行清理命令并保存配置后，自动检查 `.c2rust` 目录下是否有任何修改。如果存在修改，会自动执行 git commit 来保存这些修改。

**工作方式**：
- Git 仓库位置：`<项目根目录>/.c2rust/.git`
- 该 git 仓库由前置工具初始化，工具只负责检测和提交修改
- 只在有实际修改时才执行 commit
- Commit 消息：`Auto-commit: c2rust-clean changes`

**注意**：
- 此功能无需配置，会自动运行
- 如果 `.c2rust/.git` 不存在，此功能会静默跳过
- 提交操作在程序执行的最后阶段进行

## 输出示例

执行命令时，工具会显示详细的输出信息：

```
Project root: /path/to/project
Current directory: /path/to/project/subdir
Relative clean directory: subdir

Executing command: make clean
In directory: /path/to/project/subdir

rm -f *.o
rm -f myapp

Exit code: 0

Clean command executed successfully.
```

## 迁移指南

### 从旧版本迁移

如果您之前使用的是带有 `--dir` 和 `--cmd` 参数的旧版本：

**旧语法**：
```bash
c2rust-clean clean --dir /path/to/project --cmd make clean
```

**新语法**：
```bash
cd /path/to/project
c2rust-clean clean -- make clean
```

### 主要变化

1. **移除 `--dir` 参数** - 不再需要指定目录，直接在目标目录中运行命令即可
2. **使用 `--` 分隔符** - 替代 `--cmd` 参数，使用标准的 `--` 分隔符来分隔工具参数和清理命令
3. **自动项目根目录检测** - 工具会自动查找项目标识文件（`.git`、`Cargo.toml` 或 `.c2rust`）以确定项目根目录
4. **相对路径计算** - 清理目录会自动计算为相对于项目根目录的路径

### 迁移优势

- **更简洁的命令行** - 不需要记忆 `--dir` 和 `--cmd` 参数
- **更符合直觉** - 在哪个目录执行就清理哪个目录
- **避免参数歧义** - `--` 分隔符是命令行工具的标准做法，特别适合处理带连字符的参数
- **与 c2rust-build 保持一致** - 统一的命令行接口设计

## 错误处理

该工具为常见问题提供清晰的错误消息：

- **缺少必需参数**: 未提供清理命令
- **命令执行失败**: 清理命令返回了非零退出代码
- **目录访问失败**: 无法获取当前工作目录

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

## 发布 (Releases)

### 如何发布新版本

本项目使用 GitHub Actions 自动发布到 crates.io。要发布新版本：

1. **更新版本号**: 在 `Cargo.toml` 中更新 `version` 字段
2. **提交更改**: 提交版本号更改到主分支
3. **创建并推送标签**: 
   ```bash
   git tag v<版本号>  # 例如: git tag v0.1.0
   git push origin v<版本号>
   ```
4. **自动发布**: GitHub Actions 会自动：
   - 构建项目
   - 运行测试
   - 发布到 crates.io

**注意**: 确保 GitHub 仓库中已设置 `CARGO_REGISTRY_TOKEN` secret。

## 许可证

详见 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。