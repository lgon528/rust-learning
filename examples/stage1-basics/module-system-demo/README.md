# Module System Demo

这是一个全面的 Rust 模块系统和包管理演示项目，展示了 Rust 中模块组织、依赖管理、条件编译等核心概念。

## 项目概述

本项目演示了以下 Rust 核心概念：

- **模块系统**：模块定义、可见性控制、路径解析
- **包管理**：Cargo 配置、依赖管理、功能标志
- **代码组织**：文件结构、模块层次、重新导出
- **条件编译**：功能标志、平台特定代码
- **错误处理**：自定义错误类型、Result 模式
- **测试组织**：单元测试、集成测试、基准测试

## 项目结构

```
module-system-demo/
├── Cargo.toml              # 项目配置和依赖
├── README.md               # 项目说明文档
├── src/                    # 源代码目录
│   ├── lib.rs             # 库根模块
│   ├── config.rs          # 配置管理模块
│   ├── internal.rs        # 内部模块（私有）
│   ├── network/           # 网络模块目录
│   │   ├── mod.rs        # 网络模块根
│   │   ├── client.rs     # 客户端实现
│   │   └── server.rs     # 服务器实现
│   ├── utils/             # 工具模块目录
│   │   ├── mod.rs        # 工具模块根
│   │   ├── string_utils.rs # 字符串工具
│   │   ├── time_utils.rs   # 时间工具
│   │   └── validation.rs   # 数据验证
│   └── bin/               # 二进制程序
│       └── demo.rs        # 演示程序
├── examples/              # 示例代码
│   └── advanced_usage.rs  # 高级用法示例
├── tests/                 # 集成测试
│   └── integration_tests.rs
└── benches/               # 基准测试
    └── benchmarks.rs
```

## 功能特性

### 核心功能

- **配置管理**：支持环境变量、文件配置、构建器模式
- **网络模块**：模拟的客户端和服务器实现
- **字符串工具**：常用字符串处理函数
- **时间工具**：时间戳、持续时间、计时器
- **数据验证**：邮箱、URL、长度、范围验证

### 可选功能（通过 features 控制）

- `std`：标准库支持（默认启用）
- `serde_support`：序列化支持和 chrono 时间处理
- `logging`：日志记录支持

## 快速开始

### 1. 构建项目

```bash
# 基本构建
cargo build

# 启用所有功能的构建
cargo build --all-features

# 发布模式构建
cargo build --release
```

### 2. 运行演示程序

```bash
# 运行基本演示
cargo run --bin demo

# 启用所有功能运行
cargo run --bin demo --all-features

# 设置日志级别
RUST_LOG=debug cargo run --bin demo --features logging
```

### 3. 运行示例

```bash
# 运行高级用法示例
cargo run --example advanced_usage --all-features
```

### 4. 运行测试

```bash
# 运行所有测试
cargo test

# 运行单元测试
cargo test --lib

# 运行集成测试
cargo test --test integration_tests

# 启用所有功能运行测试
cargo test --all-features
```

### 5. 运行基准测试

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准测试
cargo bench -- string_utils

# 生成 HTML 报告
cargo bench --features criterion/html_reports
```

## 模块系统演示

### 1. 模块定义和组织

```rust
// lib.rs - 库根模块
pub mod config;          // 公共模块
pub mod network;         // 公共模块
pub mod utils;           // 公共模块
mod internal;            // 私有模块

// 重新导出常用类型
pub use config::{Config, ConfigBuilder};
pub use network::{Client, Server};
```

### 2. 可见性控制

```rust
pub struct Config { ... }           // 公共结构体
pub(crate) fn internal_func() { ... } // 包内可见
pub(super) fn parent_func() { ... }   // 父模块可见
fn private_func() { ... }             // 私有函数
```

### 3. 路径和导入

```rust
// 绝对路径
use crate::config::Config;

// 相对路径
use super::utils::string_utils;

// 重命名导入
use crate::network::Client as NetClient;

// 批量导入
use crate::utils::{
    string_utils::*,
    time_utils::{Timer, format_duration},
};
```

### 4. 条件编译

```rust
// 功能标志
#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};

// 平台特定
#[cfg(target_os = "linux")]
fn linux_specific() { ... }

// 测试代码
#[cfg(test)]
mod tests { ... }
```

## 包管理演示

### 1. 依赖管理

```toml
[dependencies]
serde = { version = "1.0", optional = true }
chrono = { version = "0.4", optional = true }
log = "0.4"

[dev-dependencies]
env_logger = "0.10"
criterion = { version = "0.5", features = ["html_reports"] }
```

### 2. 功能标志

```toml
[features]
default = ["std"]
std = []
serde_support = ["dep:serde", "dep:chrono"]
logging = []
```

### 3. 构建目标

```toml
[[bin]]
name = "demo"
path = "src/bin/demo.rs"

[[example]]
name = "advanced_usage"
path = "examples/advanced_usage.rs"

[[bench]]
name = "benchmarks"
harness = false
```

## 使用示例

### 基本用法

```rust
use module_system_demo::{
    config::{Config, ConfigBuilder, Environment},
    network::{Client, NetworkConfig},
    utils::string_utils::capitalize,
};

// 创建配置
let config = ConfigBuilder::new()
    .app_name("我的应用")
    .environment(Environment::Production)
    .build();

// 创建网络客户端
let network_config = config.get_network_config().clone();
let mut client = Client::new("my-client", network_config);

// 处理字符串
let message = capitalize("hello world");

// 连接和发送数据
client.connect()?;
client.send_data(message.as_bytes())?;
client.disconnect()?;
```

### 高级用法

```rust
// 条件编译功能
#[cfg(feature = "serde_support")]
{
    use module_system_demo::utils::time_utils::chrono_utils::*;
    let now = now_utc();
    let formatted = format_iso_date(&now);
}

// 批量验证
use module_system_demo::utils::validation::*;
let mut validator = Validator::new();

if let Err(e) = validate_email("invalid@") {
    validator.add_error(e);
}

if validator.has_errors() {
    for error in validator.get_errors() {
        eprintln!("验证错误: {}", error);
    }
}
```

## 测试策略

### 单元测试
- 每个模块都包含 `#[cfg(test)]` 测试
- 测试覆盖正常流程、边界条件、错误情况

### 集成测试
- 测试公共 API 的完整工作流
- 验证模块间的协作
- 性能基准测试

### 基准测试
- 使用 Criterion.rs 进行性能测试
- 测试不同输入大小的性能表现
- 生成详细的性能报告

## 最佳实践演示

1. **模块组织**：按功能分组，保持清晰的层次结构
2. **可见性控制**：最小化公共接口，使用适当的可见性修饰符
3. **错误处理**：使用 `Result` 类型和自定义错误
4. **文档**：为公共 API 提供详细文档和示例
5. **测试**：全面的测试覆盖，包括单元测试和集成测试
6. **性能**：使用基准测试监控性能

## 学习要点

通过这个项目，你将学会：

- 如何组织大型 Rust 项目的模块结构
- 如何使用 Cargo 管理依赖和功能
- 如何实现条件编译和功能标志
- 如何编写可维护和可测试的代码
- 如何使用 Rust 的类型系统确保代码安全
- 如何进行性能测试和优化

## 扩展练习

1. 添加新的工具模块（如文件操作、加密等）
2. 实现真实的网络功能（使用 tokio）
3. 添加配置文件序列化支持
4. 实现插件系统
5. 添加异步支持
6. 创建 CLI 工具

## 相关资源

- [Rust 模块系统官方文档](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Cargo 官方文档](https://doc.rust-lang.org/cargo/)
- [Rust API 设计指南](https://rust-lang.github.io/api-guidelines/)
- [Criterion.rs 文档](https://bheisler.github.io/criterion.rs/book/)

---

这个项目是 [Rust 学习路径](../../README.md) 第一阶段第十章的配套代码示例。