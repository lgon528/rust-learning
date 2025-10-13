# 第七章：错误处理

## 章节概述

错误处理是编写健壮 Rust 程序的关键技能。Rust 通过类型系统强制开发者显式处理错误，避免了许多运行时错误。本章将深入探讨 Rust 的错误处理机制，从基础的 `panic!` 到高级的错误处理模式。

## 学习目标

通过本章学习，你将掌握：

- 理解 Rust 错误处理的设计哲学
- 掌握 `panic!` 和不可恢复错误的使用场景
- 熟练使用 `Result<T, E>` 处理可恢复错误
- 学会错误传播和 `?` 操作符的使用
- 能够设计自定义错误类型
- 掌握错误处理的高级技巧和最佳实践
- 了解异步环境下的错误处理
- 学会错误监控和日志记录

## 核心概念

### 1. 错误分类

```rust
// 不可恢复错误 - 使用 panic!
fn unrecoverable_error_example() {
    let v = vec![1, 2, 3];
    v[99]; // 这会导致 panic!
}

// 可恢复错误 - 使用 Result
use std::fs::File;

fn recoverable_error_example() -> Result<(), std::io::Error> {
    let file = File::open("hello.txt")?; // 可能失败，但可以处理
    Ok(())
}
```

### 2. Result 类型

```rust
// Result 是一个枚举，表示操作可能成功或失败
enum Result<T, E> {
    Ok(T),   // 成功，包含值
    Err(E),  // 失败，包含错误
}

// 基本使用
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
```

### 3. 错误传播

```rust
use std::fs::File;
use std::io::{self, Read};

// 传统方式
fn read_file_traditional() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用 ? 操作符简化
fn read_file_simplified() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

### 4. 自定义错误类型

```rust
use std::fmt;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot calculate square root of negative number"),
        }
    }
}

impl std::error::Error for MathError {}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}
```

## 章节结构

### [01. panic! 和不可恢复错误](./01-panic-unrecoverable.md)
- `panic!` 宏的使用
- 不可恢复错误的场景
- panic 行为配置
- 调试和诊断技巧

### [02. Result 类型和可恢复错误](./02-result-recoverable.md)
- `Result<T, E>` 类型详解
- Result 的各种处理方法
- 错误传播和 `?` 操作符
- 自定义错误类型设计

### [03. 错误处理的高级技巧](./03-advanced-error-handling.md)
- 错误链和上下文
- anyhow 和 thiserror 库
- 可组合的错误类型
- 异步环境下的错误处理
- 错误监控和日志记录

## 实践项目建议

### 初级项目

1. **文件处理工具**
   - 实现安全的文件读写操作
   - 处理各种 I/O 错误
   - 提供用户友好的错误消息

2. **计算器程序**
   - 处理除零错误
   - 验证用户输入
   - 实现错误恢复机制

### 中级项目

3. **配置管理系统**
   - 解析配置文件
   - 验证配置项
   - 提供详细的错误信息

4. **网络客户端**
   - 处理网络连接错误
   - 实现重试机制
   - 超时处理

### 高级项目

5. **日志分析工具**
   - 解析各种日志格式
   - 错误聚合和统计
   - 异常检测

6. **微服务错误处理框架**
   - 统一错误格式
   - 错误传播和转换
   - 监控和告警集成

## 错误处理策略对比

| 策略 | 适用场景 | 优点 | 缺点 |
|------|----------|------|------|
| `panic!` | 程序错误、不可恢复 | 简单直接 | 程序终止 |
| `Result` | 可预期的失败 | 强制处理、类型安全 | 代码冗长 |
| `Option` | 值可能不存在 | 简单、安全 | 信息有限 |
| `unwrap()` | 确定不会失败 | 简洁 | 可能 panic |
| `expect()` | 调试和原型 | 自定义消息 | 可能 panic |
| `?` 操作符 | 错误传播 | 简洁、链式 | 需要返回 Result |

## 错误处理决策树

```
遇到错误情况
├── 是否可以恢复？
│   ├── 是 → 使用 Result<T, E>
│   │   ├── 需要传播？
│   │   │   ├── 是 → 使用 ? 操作符
│   │   │   └── 否 → 使用 match 或方法链
│   │   └── 需要上下文？
│   │       ├── 是 → 使用 anyhow 或自定义错误
│   │       └── 否 → 使用标准错误类型
│   └── 否 → 使用 panic!
│       ├── 程序错误 → panic!("message")
│       ├── 调试断言 → debug_assert!
│       └── 不变量违反 → unreachable!()
└── 值可能不存在？
    └── 是 → 使用 Option<T>
```

## 最佳实践

### 1. 错误类型设计

```rust
// ✅ 好的错误设计
#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation failed for field '{field}': {reason}")]
    Validation { field: String, reason: String },
    
    #[error("User {id} not found")]
    UserNotFound { id: u32 },
}

// ❌ 避免的错误设计
#[derive(Debug)]
pub enum BadError {
    Error(String), // 太泛化
    SomeError,     // 信息不足
}
```

### 2. 错误处理层次

```rust
// 库层：详细的错误类型
pub fn library_function() -> Result<Data, LibraryError> {
    // 具体实现
}

// 应用层：业务错误转换
pub fn application_function() -> Result<Response, ApplicationError> {
    let data = library_function()
        .map_err(ApplicationError::Library)?;
    // 业务逻辑
}

// 接口层：用户友好的错误
pub fn api_handler() -> Result<Json, StatusCode> {
    application_function()
        .map_err(|e| {
            log::error!("API error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
```

### 3. 错误恢复策略

```rust
// 重试机制
async fn with_retry<F, T, E>(mut f: F, max_retries: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut last_error = None;
    
    for _ in 0..=max_retries {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) => last_error = Some(e),
        }
        
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    Err(last_error.unwrap())
}

// 降级处理
fn get_data_with_fallback() -> Result<Data, Error> {
    get_data_from_primary()
        .or_else(|_| get_data_from_cache())
        .or_else(|_| get_default_data())
}
```

## 常见陷阱

### 1. 过度使用 unwrap()

```rust
// ❌ 危险的做法
let file = File::open("config.txt").unwrap();

// ✅ 更好的做法
let file = File::open("config.txt")
    .expect("Failed to open config.txt");

// ✅ 最佳做法
let file = File::open("config.txt")
    .map_err(|e| ConfigError::FileNotFound {
        path: "config.txt".to_string(),
        source: e,
    })?;
```

### 2. 忽略错误信息

```rust
// ❌ 丢失错误信息
let _ = risky_operation();

// ✅ 适当处理错误
if let Err(e) = risky_operation() {
    log::warn!("Operation failed: {}", e);
}
```

### 3. 错误类型过于复杂

```rust
// ❌ 过度复杂
#[derive(Error, Debug)]
pub enum OverComplexError {
    #[error("Type A error with {field1} and {field2} and {field3}")]
    TypeA { field1: String, field2: u32, field3: Vec<String> },
    // ... 20 more variants
}

// ✅ 适度简化
#[derive(Error, Debug)]
pub enum SimpleError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Custom error: {message}")]
    Custom { message: String },
}
```

## 高级学习路径

1. **错误处理库生态**
   - anyhow：简化错误处理
   - thiserror：派生错误类型
   - eyre：增强的错误报告
   - miette：诊断友好的错误

2. **异步错误处理**
   - tokio 中的错误处理
   - 流式处理中的错误
   - 并发错误聚合

3. **错误监控和可观测性**
   - 结构化日志
   - 错误指标收集
   - 分布式追踪

4. **领域特定错误处理**
   - Web 应用错误处理
   - 数据库错误处理
   - 网络协议错误处理

## 相关资源

### 官方文档
- [The Rust Programming Language - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [std::error 模块](https://doc.rust-lang.org/std/error/)

### 推荐阅读
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)
- [Rust Error Handling - A Deep Dive](https://www.lpalmieri.com/posts/error-handling-rust/)
- [Recoverable Errors in Rust](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

### 相关 Crate
- [anyhow](https://docs.rs/anyhow/) - 灵活的错误处理
- [thiserror](https://docs.rs/thiserror/) - 错误类型派生
- [eyre](https://docs.rs/eyre/) - 错误报告库
- [tracing](https://docs.rs/tracing/) - 结构化日志

---

**下一章预告**：第八章将深入探讨 Rust 的泛型、trait 和生命周期，这些是 Rust 类型系统的核心特性，能够帮助我们编写更加灵活、安全和高性能的代码。