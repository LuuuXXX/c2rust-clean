# c2rust-clean

C 项目构建产物清理工具，用于 c2rust 工作流。

## 概述

`c2rust-clean` 是一个命令行工具，用于在当前目录执行 C 项目清理命令。该工具是 c2rust 工作流的一部分，用于管理从 C 到 Rust 项目的转换过程。工具会自动检测项目根目录（包含 `.c2rust` 目录的位置），并计算当前执行目录相对于项目根目录的路径。

### 主要功能

- **清理命令执行**：在当前目录执行 C 项目清理命令
- **项目根目录检测**：通过 `C2RUST_PROJECT_ROOT` 环境变量或自动搜索 `.c2rust` 目录
- **配置自动保存**：自动保存清理配置到 `.c2rust` 目录
- **Git 自动提交**：自动提交 `.c2rust` 目录的修改到 Git 仓库

## 环境变量

### C2RUST_PROJECT_ROOT

该环境变量用于指定项目根目录的位置。如果设置了该变量，工具将优先使用它来定位项目根目录。

**设置方法：**

```bash
# 临时设置（仅当前会话）
export C2RUST_PROJECT_ROOT=/path/to/your/project

# 或在命令行中直接使用
C2RUST_PROJECT_ROOT=/path/to/your/project c2rust-clean clean -- make clean
```

**注意**：
- 如果未设置 `C2RUST_PROJECT_ROOT`，工具会自动从当前目录向上搜索包含 `.c2rust` 目录的位置作为项目根目录。如果找不到 `.c2rust` 目录，则使用当前目录作为项目根目录。
- `C2RUST_PROJECT_ROOT` 必须指向一个目录，而不是文件。如果指向文件或不存在的路径，工具会回退到自动搜索。

### C2RUST_DISABLE_AUTO_COMMIT

该环境变量用于禁用 `.c2rust` 目录的自动 Git 提交功能。如果您希望手动管理 `.c2rust` 的版本控制，可以设置此变量。

**设置方法：**

```bash
# 禁用自动提交（任何非空值，除了 "0" 和 "false"）
export C2RUST_DISABLE_AUTO_COMMIT=1
c2rust-clean clean -- make clean

# 或在命令行中直接使用
C2RUST_DISABLE_AUTO_COMMIT=1 c2rust-clean clean -- make clean

# 以下值会启用自动提交（保持默认行为）：
C2RUST_DISABLE_AUTO_COMMIT=0 c2rust-clean clean -- make clean
C2RUST_DISABLE_AUTO_COMMIT=false c2rust-clean clean -- make clean
```

**注意**：
- 任何非空值（除了 "0" 和 "false"）都会禁用自动提交
- 设置为 "0" 或 "false" 会保持自动提交启用
- 未设置该变量时，自动提交默认启用

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
   - 优先使用 `C2RUST_PROJECT_ROOT` 环境变量指定的目录
   - 如果未设置环境变量，则从当前目录向上查找包含 `.c2rust` 目录的位置
   - 如果未找到 `.c2rust` 目录，则使用当前目录作为项目根目录
3. **相对路径计算**: 计算当前目录相对于项目根目录的路径
4. **命令执行**: 在当前目录中运行指定的清理命令，并实时显示输出：
   - 项目根目录路径
   - 当前执行目录
   - 相对清理目录路径
   - 正在执行的完整命令
   - 命令的标准输出 (stdout) - 实时显示
   - 命令的标准错误 (stderr) - 实时显示
   - 命令的退出状态
5. **配置保存**: 使用 `c2rust-config` 工具保存清理配置到 `.c2rust` 目录
6. **Git 自动提交**: 如果 `.c2rust` 目录中存在 Git 仓库且有未提交的修改，自动创建提交

## .c2rust 目录

`.c2rust` 目录是 c2rust 工作流的核心配置目录，用于存储项目的转换配置和状态信息。

### 用途

- 存储项目配置文件（通过 `c2rust-config` 工具管理）
- 存储清理命令配置（`clean.dir` 和 `clean.cmd`）
- 通过 Git 仓库跟踪配置变更历史

### Git 自动提交功能

当 `c2rust-clean` 执行完清理命令并保存配置后，会自动检查 `.c2rust` 目录下的 Git 仓库状态：

1. **检查 Git 仓库**：确认 `.c2rust/.git` 目录存在
2. **检测修改**：查找未提交的配置文件修改
3. **自动提交**：如果有修改，自动执行 `git add` 和 `git commit`
4. **提交信息**：使用格式为 `"Auto-save configuration changes - YYYY-MM-DD HH:MM:SS"` 的提交信息

**注意**：
- 如果 `.c2rust` 目录不存在 Git 仓库，自动提交功能会被跳过（不会报错）
- 如果没有未提交的修改，不会创建空提交
- 如果 Git 操作失败，会显示警告信息但不会中断清理命令的执行

### 初始化 .c2rust Git 仓库

如果希望使用 Git 自动提交功能，需要在 `.c2rust` 目录中初始化 Git 仓库：

```bash
cd /path/to/your/project/.c2rust
git init
git config user.name "Your Name"
git config user.email "your.email@example.com"
git add .
git commit -m "Initial commit"
```

之后每次运行 `c2rust-clean` 命令，配置修改都会自动提交到这个仓库中。

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
3. **自动项目根目录检测** - 工具会自动查找 `.c2rust` 目录以确定项目根目录
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

### 调试日志

`c2rust-clean` 使用 `env_logger` 进行日志记录。要启用详细日志输出，可以设置 `RUST_LOG` 环境变量：

```bash
# 显示所有日志
RUST_LOG=debug c2rust-clean clean -- make clean

# 只显示 info 级别及以上的日志
RUST_LOG=info c2rust-clean clean -- make clean

# 只显示特定模块的日志
RUST_LOG=c2rust_clean::git_helper=debug c2rust-clean clean -- make clean
```

日志级别（从低到高）：
- `trace` - 最详细的跟踪信息
- `debug` - 调试信息
- `info` - 一般信息（如 Git 操作）
- `warn` - 警告信息
- `error` - 错误信息

## 项目结构

```
src/
├── main.rs         # CLI 入口点和参数解析
├── error.rs        # 错误类型定义
├── executor.rs     # 命令执行逻辑
├── config_helper.rs # c2rust-config 工具集成
└── git_helper.rs   # Git 自动提交功能

tests/
└── integration_test.rs  # 集成测试
```

## 许可证

详见 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。