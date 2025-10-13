# Cargo 和包管理

## 学习目标

通过本节学习，你将掌握：

- 理解 Cargo 的核心概念和作用
- 掌握 `Cargo.toml` 配置文件的编写
- 学会管理项目依赖
- 掌握 Cargo 的常用命令
- 理解工作空间（Workspace）的使用
- 学会发布和版本管理

## Cargo 概述

### 什么是 Cargo

Cargo 是 Rust 的构建系统和包管理器，提供以下功能：

- **项目管理**：创建、构建、测试项目
- **依赖管理**：下载、编译、更新依赖
- **包发布**：发布包到 crates.io
- **工作空间**：管理多包项目
- **工具集成**：与各种开发工具集成

### Cargo 的核心概念

- **Package（包）**：包含一个或多个 crate 的项目
- **Crate（箱）**：编译单元，可以是二进制或库
- **Dependency（依赖）**：项目所依赖的外部 crate
- **Workspace（工作空间）**：管理多个相关包的集合

## 项目结构

### 标准项目布局

```
my_project/
├── Cargo.toml          # 项目配置文件
├── Cargo.lock          # 依赖锁定文件
├── src/                # 源代码目录
│   ├── main.rs         # 二进制 crate 入口
│   ├── lib.rs          # 库 crate 入口
│   └── bin/            # 额外的二进制文件
│       └── another.rs
├── tests/              # 集成测试
│   └── integration_test.rs
├── examples/           # 示例代码
│   └── example.rs
├── benches/            # 基准测试
│   └── benchmark.rs
└── target/             # 构建输出目录
    ├── debug/
    └── release/
```

### 创建新项目

```bash
# 创建二进制项目
cargo new my_project

# 创建库项目
cargo new my_library --lib

# 在现有目录初始化
cargo init

# 指定版本控制系统
cargo new my_project --vcs git
```

## Cargo.toml 配置

### 基本配置

```toml
[package]
name = "my_project"           # 包名
version = "0.1.0"             # 版本号
edition = "2021"              # Rust 版本
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"  # 许可证
description = "A sample project"
homepage = "https://example.com"
repository = "https://github.com/user/repo"
readme = "README.md"
keywords = ["cli", "tool"]
categories = ["command-line-utilities"]

# 排除文件
exclude = [
    "docs/*",
    "*.tmp",
]

# 包含文件（与 exclude 互斥）
# include = [
#     "src/**/*",
#     "Cargo.toml",
# ]
```

### 依赖管理

```toml
[dependencies]
# 来自 crates.io
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
clap = { version = "4.0", optional = true }

# Git 依赖
my_git_dep = { git = "https://github.com/user/repo" }
my_git_branch = { git = "https://github.com/user/repo", branch = "main" }
my_git_tag = { git = "https://github.com/user/repo", tag = "v1.0.0" }
my_git_rev = { git = "https://github.com/user/repo", rev = "abc123" }

# 本地路径依赖
my_local_dep = { path = "../my_local_crate" }

# 开发依赖（仅在开发时使用）
[dev-dependencies]
proptest = "1.0"
criterion = "0.4"

# 构建依赖（构建脚本使用）
[build-dependencies]
cc = "1.0"

# 目标特定依赖
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

### 特性（Features）

```toml
[features]
default = ["std"]             # 默认特性
std = []                      # 标准库支持
serde_support = ["serde"]     # 可选的 serde 支持
full = ["std", "serde_support"] # 完整特性集

# 条件依赖
[dependencies]
serde = { version = "1.0", optional = true }
```

### 构建配置

```toml
# 二进制目标
[[bin]]
name = "my_app"
path = "src/main.rs"

[[bin]]
name = "helper"
path = "src/bin/helper.rs"

# 库目标
[lib]
name = "my_lib"
path = "src/lib.rs"
crate-type = ["cdylib", "rlib"]

# 示例
[[example]]
name = "basic"
path = "examples/basic.rs"

# 测试
[[test]]
name = "integration"
path = "tests/integration.rs"

# 基准测试
[[bench]]
name = "performance"
path = "benches/performance.rs"
harness = false
```

### 编译配置

```toml
# 发布配置
[profile.release]
opt-level = 3              # 优化级别
lto = true                 # 链接时优化
codegen-units = 1          # 代码生成单元
panic = "abort"            # panic 处理方式
strip = true               # 去除符号信息

# 开发配置
[profile.dev]
opt-level = 0
debug = true
split-debuginfo = "unpacked"

# 自定义配置
[profile.dev-opt]
inherits = "dev"
opt-level = 1
```

## 常用 Cargo 命令

### 项目管理

```bash
# 创建项目
cargo new my_project
cargo init

# 构建项目
cargo build                    # 调试构建
cargo build --release          # 发布构建
cargo build --target x86_64-pc-windows-gnu  # 交叉编译

# 运行项目
cargo run                      # 运行默认二进制
cargo run --bin helper         # 运行指定二进制
cargo run --example basic      # 运行示例

# 检查代码
cargo check                    # 快速检查（不生成可执行文件）
cargo clippy                   # 代码检查和建议
cargo fmt                      # 代码格式化
```

### 测试和文档

```bash
# 运行测试
cargo test                     # 运行所有测试
cargo test unit_test           # 运行特定测试
cargo test --lib               # 只运行库测试
cargo test --doc               # 运行文档测试
cargo test --release           # 发布模式测试

# 基准测试
cargo bench

# 生成文档
cargo doc                      # 生成文档
cargo doc --open               # 生成并打开文档
cargo doc --no-deps            # 不包含依赖文档
```

### 依赖管理

```bash
# 更新依赖
cargo update                   # 更新所有依赖
cargo update -p serde          # 更新特定依赖

# 查看依赖
cargo tree                     # 依赖树
cargo tree --duplicates        # 重复依赖
cargo tree --invert serde      # 反向依赖

# 审计依赖
cargo audit                    # 安全审计（需要安装 cargo-audit）

# 清理
cargo clean                    # 清理构建产物
```

### 发布管理

```bash
# 打包
cargo package                  # 创建发布包
cargo package --list           # 列出包含的文件

# 发布
cargo publish                  # 发布到 crates.io
cargo publish --dry-run        # 模拟发布

# 撤回版本
cargo yank --vers 1.0.1        # 撤回版本
cargo yank --vers 1.0.1 --undo # 取消撤回
```

## 工作空间（Workspace）

### 工作空间配置

```toml
# 根目录的 Cargo.toml
[workspace]
members = [
    "app",
    "lib1",
    "lib2",
    "tools/*",
]

exclude = [
    "old_projects/*",
]

# 工作空间级别的依赖
[workspace.dependencies]
serde = "1.0"
tokio = "1.0"

# 工作空间级别的元数据
[workspace.metadata]
authors = ["Team <team@example.com>"]
license = "MIT"
```

### 工作空间项目结构

```
workspace/
├── Cargo.toml              # 工作空间配置
├── Cargo.lock              # 统一的锁定文件
├── app/                    # 应用程序包
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── lib1/                   # 库包 1
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── lib2/                   # 库包 2
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── target/                 # 共享的构建目录
```

### 工作空间中的依赖

```toml
# app/Cargo.toml
[package]
name = "app"
version = "0.1.0"
edition = "2021"

[dependencies]
lib1 = { path = "../lib1" }     # 工作空间内部依赖
lib2 = { path = "../lib2" }
serde = { workspace = true }     # 使用工作空间依赖
tokio = { workspace = true, features = ["full"] }
```

### 工作空间命令

```bash
# 在工作空间根目录
cargo build                    # 构建所有包
cargo build -p app             # 构建特定包
cargo test --workspace         # 测试所有包
cargo run -p app               # 运行特定包
```

## 版本管理

### 语义化版本

Rust 使用语义化版本（SemVer）：

- **MAJOR.MINOR.PATCH** (例如：1.2.3)
- **MAJOR**：不兼容的 API 变更
- **MINOR**：向后兼容的功能添加
- **PATCH**：向后兼容的问题修复

### 版本要求

```toml
[dependencies]
# 精确版本
exact = "=1.2.3"

# 兼容版本（默认）
compat = "1.2.3"        # >=1.2.3, <2.0.0
compat2 = "^1.2.3"      # 同上

# 波浪号要求
tilde = "~1.2.3"        # >=1.2.3, <1.3.0

# 通配符
wildcard = "1.2.*"      # >=1.2.0, <1.3.0

# 范围
range = ">= 1.2, < 1.5"
```

### 版本更新策略

```bash
# 查看可更新的依赖
cargo outdated              # 需要安装 cargo-outdated

# 更新到兼容版本
cargo update

# 更新到最新版本（修改 Cargo.toml）
cargo upgrade               # 需要安装 cargo-edit
```

## 实际应用示例

### CLI 工具项目

```toml
# Cargo.toml
[package]
name = "my-cli-tool"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT"
description = "A useful CLI tool"
repository = "https://github.com/user/my-cli-tool"
keywords = ["cli", "tool"]
categories = ["command-line-utilities"]

[[bin]]
name = "my-tool"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"

[dev-dependencies]
assert_cmd = "2.0"
predictor = "3.0"
tempfile = "3.0"

[profile.release]
lto = true
codegen-units = 1
strip = true
```

### 库项目

```toml
# Cargo.toml
[package]
name = "my-awesome-lib"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
description = "An awesome library for doing awesome things"
repository = "https://github.com/user/my-awesome-lib"
documentation = "https://docs.rs/my-awesome-lib"
keywords = ["awesome", "library"]
categories = ["algorithms"]

[features]
default = ["std"]
std = []
serde_support = ["serde"]
full = ["std", "serde_support"]

[dependencies]
serde = { version = "1.0", optional = true, features = ["derive"] }

[dev-dependencies]
proptest = "1.0"
criterion = "0.4"

[[bench]]
name = "performance"
harness = false
```

## 最佳实践

### 依赖管理

1. **版本固定**：在应用中固定依赖版本，在库中使用宽松版本
2. **特性控制**：合理使用 features 减少编译时间和二进制大小
3. **定期更新**：定期更新依赖以获得安全修复和性能改进
4. **审计依赖**：使用 `cargo audit` 检查安全漏洞

### 项目组织

```toml
# 良好的项目结构
[package]
name = "project-name"        # 使用 kebab-case
version = "0.1.0"
edition = "2021"             # 使用最新版本

# 明确的元数据
authors = ["Name <email>"]
license = "MIT OR Apache-2.0" # 推荐的许可证组合
description = "Clear description"
repository = "https://github.com/user/repo"
keywords = ["relevant", "keywords"]
categories = ["appropriate-category"]

# 合理的依赖
[dependencies]
# 只包含必要的依赖
# 使用合适的版本要求
# 合理使用 features
```

### 发布准备

```bash
# 发布前检查清单
cargo fmt                      # 格式化代码
cargo clippy                   # 检查代码质量
cargo test                     # 运行所有测试
cargo doc                      # 生成文档
cargo package                 # 检查打包
cargo publish --dry-run       # 模拟发布
```

## 常见问题

### 1. 依赖冲突

```bash
# 查看依赖树找出冲突
cargo tree
cargo tree --duplicates

# 解决方案：使用 patch 部分
[patch.crates-io]
problematic-crate = { git = "https://github.com/user/fixed-version" }
```

### 2. 构建缓存问题

```bash
# 清理缓存
cargo clean

# 强制重新构建
cargo build --offline  # 离线构建
cargo build --frozen   # 不更新 Cargo.lock
```

### 3. 交叉编译

```bash
# 安装目标
rustup target add x86_64-pc-windows-gnu

# 交叉编译
cargo build --target x86_64-pc-windows-gnu

# 配置链接器
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

## 学习检查清单

- [ ] 理解 Cargo 的基本概念和作用
- [ ] 掌握项目创建和初始化
- [ ] 熟练编写 Cargo.toml 配置
- [ ] 掌握依赖管理和版本控制
- [ ] 理解和使用 features
- [ ] 掌握常用 Cargo 命令
- [ ] 理解工作空间的概念和使用
- [ ] 掌握包的发布流程
- [ ] 了解版本管理最佳实践
- [ ] 能够解决常见的构建问题

## 扩展阅读

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cargo.toml 格式规范](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [crates.io 用户指南](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [语义化版本规范](https://semver.org/lang/zh-CN/)