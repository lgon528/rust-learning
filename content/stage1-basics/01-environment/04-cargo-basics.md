# 1.4 Cargo包管理器

Cargo是Rust的官方包管理器和构建工具，是Rust生态系统的核心组件。本节将深入介绍Cargo的功能和使用方法。

## 🎯 学习目标

- 理解Cargo的作用和重要性
- 掌握Cargo项目的创建和管理
- 学会依赖管理和版本控制
- 了解Cargo的高级功能
- 掌握常用Cargo命令

## 📦 Cargo概述

### 什么是Cargo？

Cargo是Rust的**包管理器**和**构建系统**，提供以下功能：

- 📦 **依赖管理**：自动下载和管理外部库
- 🔨 **构建系统**：编译项目和依赖
- 📋 **项目管理**：标准化项目结构
- 🧪 **测试运行**：集成测试框架
- 📚 **文档生成**：自动生成API文档
- 🚀 **发布管理**：发布到crates.io

### 与其他语言对比

| 语言 | 包管理器 | 构建工具 | 特点 |
|------|----------|----------|------|
| **Rust** | Cargo | Cargo | 一体化解决方案 |
| **JavaScript** | npm/yarn | webpack/vite | 分离的工具链 |
| **Python** | pip | setuptools | 分离的工具链 |
| **Go** | go mod | go build | 内置但简单 |
| **Java** | Maven/Gradle | Maven/Gradle | 功能强大但复杂 |
| **C/C++** | 无标准 | Make/CMake | 需要手动配置 |

## 🏗️ 项目结构

### 标准项目布局

```
my_project/
├── Cargo.toml          # 项目配置文件
├── Cargo.lock          # 依赖锁定文件
├── src/                # 源代码目录
│   ├── main.rs         # 二进制程序入口
│   ├── lib.rs          # 库入口
│   └── bin/            # 额外的二进制程序
│       └── another.rs
├── tests/              # 集成测试
│   └── integration_test.rs
├── examples/           # 示例代码
│   └── example.rs
├── benches/            # 性能测试
│   └── benchmark.rs
└── target/             # 构建输出（自动生成）
    ├── debug/          # 调试版本
    └── release/        # 发布版本
```

### 项目类型

#### 1. 二进制项目（Binary Crate）

```bash
# 创建二进制项目
cargo new my_app
```

特点：
- 包含`src/main.rs`
- 生成可执行文件
- 用于应用程序开发

#### 2. 库项目（Library Crate）

```bash
# 创建库项目
cargo new my_lib --lib
```

特点：
- 包含`src/lib.rs`
- 生成库文件
- 用于代码复用

#### 3. 混合项目

```bash
# 创建混合项目
cargo new my_project
# 然后添加 src/lib.rs
```

特点：
- 同时包含`src/main.rs`和`src/lib.rs`
- 既可以作为库使用，也可以独立运行

## 📋 Cargo.toml详解

### 基本配置

```toml
[package]
name = "my_project"              # 项目名称
version = "0.1.0"                # 版本号（语义化版本）
edition = "2021"                 # Rust版本
authors = ["Your Name <email@example.com>"]
description = "A sample Rust project"
license = "MIT OR Apache-2.0"    # 许可证
repository = "https://github.com/user/repo"
homepage = "https://example.com"
documentation = "https://docs.rs/my_project"
readme = "README.md"
keywords = ["cli", "tool"]       # 关键词
categories = ["command-line-utilities"]

# 可选的元数据
[package.metadata]
custom_field = "custom_value"
```

### 依赖管理

#### 基本依赖

```toml
[dependencies]
serde = "1.0"                    # 最新1.x版本
clap = "4.0.0"                   # 精确版本
regex = "^1.5"                   # 兼容版本
tokio = { version = "1.0", features = ["full"] }

# 可选依赖
serde_json = { version = "1.0", optional = true }

# 平台特定依赖
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

#### 开发依赖

```toml
[dev-dependencies]
proptest = "1.0"                 # 仅用于测试
criterion = "0.4"                # 性能测试
```

#### 构建依赖

```toml
[build-dependencies]
cc = "1.0"                       # 构建脚本依赖
```

### 版本规范

| 格式 | 含义 | 示例 |
|------|------|------|
| `1.2.3` | 精确版本 | 只使用1.2.3 |
| `^1.2.3` | 兼容版本 | >=1.2.3, <2.0.0 |
| `~1.2.3` | 补丁版本 | >=1.2.3, <1.3.0 |
| `>=1.2.0` | 范围版本 | >=1.2.0 |
| `1.*` | 通配符 | >=1.0.0, <2.0.0 |

### 特性（Features）

```toml
[features]
default = ["std"]               # 默认特性
std = []                         # 标准库支持
serde_support = ["serde"]        # 可选的serde支持
full = ["std", "serde_support"]  # 完整特性

[dependencies]
serde = { version = "1.0", optional = true }
```

使用特性：
```bash
# 启用特定特性
cargo build --features serde_support

# 启用所有特性
cargo build --all-features

# 禁用默认特性
cargo build --no-default-features
```

## 🔨 常用Cargo命令

### 项目管理

```bash
# 创建新项目
cargo new project_name          # 二进制项目
cargo new lib_name --lib        # 库项目
cargo init                      # 在当前目录初始化

# 项目信息
cargo --version                 # Cargo版本
cargo --list                    # 所有可用命令
```

### 构建相关

```bash
# 基本构建
cargo build                     # 构建调试版本
cargo build --release           # 构建发布版本
cargo check                     # 快速检查（不生成可执行文件）

# 运行程序
cargo run                       # 构建并运行
cargo run --release             # 发布版本运行
cargo run --bin binary_name     # 运行特定二进制
cargo run --example example_name # 运行示例

# 清理
cargo clean                     # 清理构建文件
```

### 测试相关

```bash
# 运行测试
cargo test                      # 运行所有测试
cargo test test_name            # 运行特定测试
cargo test --lib                # 只运行库测试
cargo test --bin binary_name    # 运行二进制测试
cargo test --release            # 发布模式测试

# 性能测试
cargo bench                     # 运行性能测试
```

### 文档相关

```bash
# 生成文档
cargo doc                       # 生成项目文档
cargo doc --open                # 生成并打开文档
cargo doc --no-deps             # 不包含依赖文档
```

### 依赖管理

```bash
# 更新依赖
cargo update                    # 更新所有依赖
cargo update package_name       # 更新特定包

# 查看依赖
cargo tree                      # 依赖树
cargo tree --duplicates         # 重复依赖
```

## 🔍 高级功能

### 工作空间（Workspace）

#### 创建工作空间

```toml
# Cargo.toml（根目录）
[workspace]
members = [
    "app",
    "lib1",
    "lib2",
]

# 排除某些目录
exclude = ["old_project"]

# 工作空间级别的依赖
[workspace.dependencies]
serde = "1.0"
```

#### 工作空间结构

```
workspace/
├── Cargo.toml              # 工作空间配置
├── Cargo.lock              # 统一的锁定文件
├── app/                    # 应用程序
│   ├── Cargo.toml
│   └── src/main.rs
├── lib1/                   # 库1
│   ├── Cargo.toml
│   └── src/lib.rs
└── lib2/                   # 库2
    ├── Cargo.toml
    └── src/lib.rs
```

### 构建脚本（Build Scripts）

#### 创建构建脚本

```rust
// build.rs
fn main() {
    // 编译C代码
    cc::Build::new()
        .file("src/helper.c")
        .compile("helper");
    
    // 设置环境变量
    println!("cargo:rustc-env=BUILD_TIME={}", 
             std::env::var("BUILD_TIME").unwrap_or_else(|_| "unknown".to_string()));
    
    // 链接库
    println!("cargo:rustc-link-lib=ssl");
    
    // 重新运行条件
    println!("cargo:rerun-if-changed=src/helper.c");
}
```

#### 配置构建脚本

```toml
[package]
build = "build.rs"              # 指定构建脚本

[build-dependencies]
cc = "1.0"                      # 构建脚本依赖
```

### 自定义命令

#### 安装Cargo扩展

```bash
# 常用扩展
cargo install cargo-watch       # 文件监控
cargo install cargo-expand       # 宏展开
cargo install cargo-bloat        # 大小分析
cargo install cargo-audit        # 安全审计
cargo install cargo-outdated     # 过期依赖检查
```

#### 使用扩展

```bash
# 文件监控
cargo watch -x run              # 文件变化时自动运行
cargo watch -x test             # 文件变化时自动测试

# 宏展开
cargo expand                    # 展开所有宏
cargo expand main               # 展开main函数

# 大小分析
cargo bloat --release           # 分析发布版本大小

# 安全审计
cargo audit                     # 检查已知漏洞

# 过期检查
cargo outdated                  # 检查过期依赖
```

## 🎯 配置文件

### 全局配置

```toml
# ~/.cargo/config.toml

[build]
target-dir = "/tmp/cargo-target"  # 全局构建目录

[cargo-new]
name = "Your Name"
email = "your.email@example.com"
vcs = "git"                      # 版本控制系统

[net]
git-fetch-with-cli = true       # 使用git命令行

[source.crates-io]
replace-with = "ustc"            # 使用镜像源

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### 项目配置

```toml
# .cargo/config.toml（项目根目录）

[build]
rustflags = ["-W", "unused"]

[env]
DATABASE_URL = "sqlite:///tmp/database.db"

[alias]
b = "build"
r = "run"
t = "test"
ck = "check"
```

## 🚀 发布到crates.io

### 准备发布

1. **完善Cargo.toml**：

```toml
[package]
name = "my_awesome_crate"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
description = "An awesome Rust crate"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/repo"
homepage = "https://example.com"
documentation = "https://docs.rs/my_awesome_crate"
readme = "README.md"
keywords = ["awesome", "rust"]
categories = ["development-tools"]
```

2. **创建账户**：

```bash
# 在crates.io创建账户并获取API token
cargo login your_api_token
```

3. **发布**：

```bash
# 检查包内容
cargo package

# 发布到crates.io
cargo publish

# 撤销发布（72小时内）
cargo yank --vers 0.1.0
```

## 🧪 实践练习

### 练习1：创建库项目

```bash
# 创建数学库
cargo new math_utils --lib
cd math_utils
```

编辑`src/lib.rs`：

```rust
/// 计算两个数的最大公约数
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// 计算两个数的最小公倍数
pub fn lcm(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 5), 15);
    }
}
```

运行测试：
```bash
cargo test
```

### 练习2：使用外部依赖

创建一个使用外部库的项目：

```bash
cargo new json_example
cd json_example
```

编辑`Cargo.toml`：

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

编辑`src/main.rs`：

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // 序列化为JSON
    let json = serde_json::to_string_pretty(&person)?;
    println!("JSON: {}", json);

    // 从JSON反序列化
    let parsed: Person = serde_json::from_str(&json)?;
    println!("Parsed: {:?}", parsed);

    Ok(())
}
```

### 练习3：工作空间项目

创建一个工作空间：

```bash
mkdir my_workspace
cd my_workspace

# 创建工作空间配置
cat > Cargo.toml << 'EOF'
[workspace]
members = ["app", "utils"]
EOF

# 创建应用程序
cargo new app

# 创建工具库
cargo new utils --lib
```

配置依赖关系：

```toml
# app/Cargo.toml
[dependencies]
utils = { path = "../utils" }
```

## 📊 性能优化

### 编译优化

```toml
# Cargo.toml
[profile.release]
opt-level = 3                   # 最高优化级别
lto = true                      # 链接时优化
codegen-units = 1               # 单个代码生成单元
panic = "abort"                 # 崩溃时直接终止

[profile.dev]
opt-level = 1                   # 开发时轻度优化
```

### 依赖优化

```toml
[dependencies]
# 只启用需要的特性
tokio = { version = "1.0", features = ["rt", "net"] }

# 使用更轻量的替代品
fastrand = "1.0"               # 替代rand
once_cell = "1.0"              # 替代lazy_static
```

## ✅ 检查清单

完成本节学习后，确保你能够：

- [ ] 理解Cargo的作用和重要性
- [ ] 创建和管理Cargo项目
- [ ] 配置项目依赖和特性
- [ ] 使用常用的Cargo命令
- [ ] 理解Cargo.toml的各个配置项
- [ ] 创建和使用工作空间
- [ ] 编写和运行测试
- [ ] 生成和查看文档
- [ ] 了解发布流程

## 📚 延伸阅读

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [crates.io](https://crates.io/) - Rust包仓库
- [docs.rs](https://docs.rs/) - 自动生成的文档
- [Cargo命令参考](https://doc.rust-lang.org/cargo/commands/)

---

**Cargo掌握完成！** 📦 你现在具备了管理Rust项目的核心技能。

[← 上一节：第一个Rust程序](./03-hello-world.md) | [下一章：基本语法 →](../02-syntax/01-variables.md)