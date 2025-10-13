# 第九章：错误处理进阶

## 章节概述

本章深入探讨 Rust 中的高级错误处理技术，从自定义错误类型的设计到复杂系统中的错误传播和恢复策略。通过学习本章，你将掌握构建健壮、可维护的错误处理系统的能力。

## 学习目标

完成本章学习后，你将能够：

- 设计和实现语义清晰的自定义错误类型
- 熟练使用 Rust 生态系统中的错误处理库
- 构建多层次的错误传播架构
- 实现各种错误恢复策略
- 在异步环境中处理复杂的错误场景
- 建立完善的错误监控和日志系统

## 核心概念

### 1. 自定义错误类型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("配置错误: {message}")]
    Config { message: String },
    
    #[error("数据库错误")]
    Database(#[from] sqlx::Error),
    
    #[error("网络请求失败: {url}")]
    Network { url: String, #[source] source: reqwest::Error },
    
    #[error("验证失败: {field}")]
    Validation { field: String },
}
```

### 2. 错误处理库生态

```rust
// anyhow - 应用程序错误处理
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("无法读取配置文件")?;
    
    toml::from_str(&content)
        .context("配置文件格式错误")
}

// thiserror - 库错误类型定义
#[derive(Error, Debug)]
#[error("解析错误: 第 {line} 行")]
pub struct ParseError {
    line: usize,
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
}
```

### 3. 错误传播架构

```rust
// 分层错误设计
mod layers {
    // 数据层错误
    #[derive(Error, Debug)]
    pub enum DataError {
        #[error("连接失败")]
        Connection(#[from] sqlx::Error),
        #[error("记录未找到")]
        NotFound,
    }
    
    // 业务层错误
    #[derive(Error, Debug)]
    pub enum BusinessError {
        #[error("数据访问错误")]
        Data(#[from] DataError),
        #[error("业务规则违反: {rule}")]
        RuleViolation { rule: String },
    }
    
    // API层错误
    #[derive(Error, Debug)]
    pub enum ApiError {
        #[error("业务逻辑错误")]
        Business(#[from] BusinessError),
        #[error("请求无效")]
        InvalidRequest,
    }
}
```

### 4. 错误恢复策略

```rust
// 重试机制
async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: usize,
) -> Result<T, E>
where
    F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>>>>,
{
    let mut attempts = 0;
    let mut delay = Duration::from_millis(100);
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) if attempts < max_retries => {
                attempts += 1;
                tokio::time::sleep(delay).await;
                delay *= 2; // 指数退避
            }
            Err(err) => return Err(err),
        }
    }
}

// 断路器模式
struct CircuitBreaker {
    failure_count: AtomicUsize,
    last_failure: Mutex<Option<Instant>>,
    state: Mutex<CircuitState>,
}

// 服务降级
async fn get_recommendations_with_fallback(user_id: &str) -> Vec<Recommendation> {
    // 1. 尝试主服务
    if let Ok(recs) = primary_service.get_recommendations(user_id).await {
        return recs;
    }
    
    // 2. 尝试缓存
    if let Some(cached) = cache.get(user_id).await {
        return cached;
    }
    
    // 3. 返回默认推荐
    get_default_recommendations()
}
```

## 章节结构

### [01. 自定义错误类型](./01-custom-error-types.md)

**核心内容：**
- 错误类型设计原则
- 使用枚举和结构体定义错误
- 实现 Error trait 和相关 trait
- 错误转换和链式传播
- 实际项目中的错误类型设计

**实践重点：**
- 设计语义清晰的错误类型
- 实现错误转换机制
- 构建错误类型层次结构

### [02. 错误处理库和工具](./02-error-handling-libraries.md)

**核心内容：**
- anyhow 库的使用和最佳实践
- thiserror 简化错误类型定义
- eyre 和 miette 的特殊用途
- 库选择策略和组合使用
- 性能考虑和优化

**实践重点：**
- 为不同场景选择合适的库
- 库和应用的错误处理模式
- 错误类型的性能优化

### [03. 错误传播和恢复策略](./03-error-propagation-recovery.md)

**核心内容：**
- 错误传播机制和边界设计
- 重试、降级、断路器等恢复策略
- 异步环境下的错误处理
- 错误监控和日志记录
- 健康检查和可观测性

**实践重点：**
- 构建多层次错误处理架构
- 实现各种恢复策略
- 异步并发错误处理

## 实践项目建议

### 初级项目：配置管理器

```rust
// 实现一个配置管理器，包含：
// - 自定义配置错误类型
// - 多种配置源（文件、环境变量、命令行）
// - 配置验证和错误报告
// - 配置热重载和错误恢复

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("配置文件未找到: {path}")]
    FileNotFound { path: String },
    
    #[error("配置格式错误: {reason}")]
    InvalidFormat { reason: String },
    
    #[error("配置验证失败: {field}")]
    ValidationFailed { field: String },
}

struct ConfigManager {
    sources: Vec<Box<dyn ConfigSource>>,
    cache: Option<Config>,
}
```

### 中级项目：HTTP 客户端库

```rust
// 实现一个 HTTP 客户端库，包含：
// - 分层错误类型设计
// - 重试和断路器机制
// - 超时和取消支持
// - 错误监控和指标收集

#[derive(Error, Debug)]
pub enum HttpClientError {
    #[error("网络错误")]
    Network(#[from] reqwest::Error),
    
    #[error("超时: {timeout:?}")]
    Timeout { timeout: Duration },
    
    #[error("服务器错误: {status}")]
    ServerError { status: u16, body: String },
    
    #[error("断路器打开")]
    CircuitBreakerOpen,
}

struct HttpClient {
    client: reqwest::Client,
    circuit_breaker: CircuitBreaker,
    retry_policy: RetryPolicy,
}
```

### 高级项目：分布式任务调度器

```rust
// 实现一个分布式任务调度器，包含：
// - 复杂的错误传播架构
// - 多种恢复策略组合
// - 异步并发错误处理
// - 完整的监控和告警系统

#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("任务执行失败")]
    TaskExecution(#[from] TaskError),
    
    #[error("调度器状态错误")]
    InvalidState { current: String, expected: String },
    
    #[error("资源不足")]
    InsufficientResources { required: String, available: String },
}

struct TaskScheduler {
    executor: TaskExecutor,
    state_manager: StateManager,
    error_handler: ErrorHandler,
    metrics: MetricsCollector,
}
```

## 概念对比表

| 概念 | 用途 | 适用场景 | 优势 | 注意事项 |
|------|------|----------|------|----------|
| **自定义错误类型** | 语义化错误表示 | 库开发、复杂应用 | 类型安全、清晰语义 | 设计复杂度 |
| **anyhow::Error** | 简化错误处理 | 应用程序 | 易用、灵活 | 运行时开销 |
| **thiserror** | 错误类型定义 | 库开发 | 编译时安全 | 学习成本 |
| **重试机制** | 临时错误恢复 | 网络请求、IO操作 | 提高成功率 | 可能放大问题 |
| **断路器** | 防止级联失败 | 外部服务调用 | 快速失败 | 状态管理复杂 |
| **服务降级** | 保证核心功能 | 微服务架构 | 提高可用性 | 功能受限 |

## 学习路径决策树

```
开始学习错误处理进阶
├── 你在开发库还是应用？
│   ├── 库开发 → 重点学习 thiserror 和自定义错误类型
│   └── 应用开发 → 重点学习 anyhow 和错误传播
├── 需要处理外部服务调用吗？
│   ├── 是 → 学习重试、断路器、降级策略
│   └── 否 → 重点学习错误传播和监控
├── 是异步应用吗？
│   ├── 是 → 学习异步错误处理和并发控制
│   └── 否 → 重点学习同步错误处理模式
└── 需要生产级监控吗？
    ├── 是 → 学习错误监控、日志、健康检查
    └── 否 → 重点学习基础错误处理技术
```

## 最佳实践

### 错误类型设计

1. **语义清晰**：错误类型应该清楚地表达发生了什么
2. **适当粒度**：既不过于粗糙也不过于细致
3. **可组合性**：支持错误转换和链式传播
4. **上下文丰富**：提供足够的调试信息

### 错误处理策略

1. **分层处理**：在不同层次定义适当的错误边界
2. **快速失败**：对于不可恢复的错误，尽早失败
3. **优雅降级**：对于非关键功能，提供降级方案
4. **监控告警**：建立完善的错误监控和告警机制

### 库设计原则

1. **具体错误类型**：库应该提供具体的错误类型
2. **错误转换支持**：实现 From trait 支持错误转换
3. **文档完善**：清楚地文档化所有可能的错误
4. **向后兼容**：错误类型的变更要考虑向后兼容性

## 常见陷阱

### 1. 错误类型过度设计

```rust
// ❌ 避免：过度细分的错误类型
#[derive(Error, Debug)]
enum OverDesignedError {
    #[error("文件读取失败：权限不足")]
    FileReadPermissionDenied,
    #[error("文件读取失败：文件不存在")]
    FileReadNotFound,
    #[error("文件读取失败：磁盘空间不足")]
    FileReadDiskFull,
    // ... 更多细分错误
}

// ✅ 推荐：合理的错误分类
#[derive(Error, Debug)]
enum FileError {
    #[error("文件操作失败")]
    Io(#[from] std::io::Error),
    #[error("文件格式错误: {reason}")]
    InvalidFormat { reason: String },
}
```

### 2. 错误上下文丢失

```rust
// ❌ 避免：丢失错误上下文
fn process_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?; // 丢失了文件路径信息
    Ok(content.to_uppercase())
}

// ✅ 推荐：保留错误上下文
fn process_file(path: &str) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("无法读取文件: {}", path))?;
    Ok(content.to_uppercase())
}
```

### 3. 忽略错误恢复

```rust
// ❌ 避免：简单的错误传播
fn fetch_data() -> Result<Data, ApiError> {
    api_client.get("/data").await // 网络错误直接失败
}

// ✅ 推荐：考虑错误恢复
fn fetch_data_with_fallback() -> Result<Data, ApiError> {
    // 尝试主 API
    match api_client.get("/data").await {
        Ok(data) => Ok(data),
        Err(ApiError::Network(_)) => {
            // 网络错误时尝试缓存
            cache.get("data").ok_or(ApiError::DataUnavailable)
        }
        Err(other) => Err(other),
    }
}
```

## 进阶学习方向

### 1. 分布式系统错误处理
- 分布式事务的错误处理
- 服务间错误传播
- 分布式追踪和错误关联

### 2. 性能优化
- 错误处理的零成本抽象
- 错误类型的内存布局优化
- 异步错误处理的性能考虑

### 3. 领域特定错误处理
- 编译器错误处理
- 数据库错误处理
- 网络协议错误处理

## 相关资源

### 官方文档
- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust Reference - Error Handling](https://doc.rust-lang.org/reference/)
- [std::error 模块文档](https://doc.rust-lang.org/std/error/)

### 重要 Crate
- [anyhow](https://docs.rs/anyhow/) - 灵活的错误处理
- [thiserror](https://docs.rs/thiserror/) - 错误类型派生宏
- [eyre](https://docs.rs/eyre/) - 错误报告和处理
- [miette](https://docs.rs/miette/) - 诊断报告
- [tracing](https://docs.rs/tracing/) - 结构化日志和追踪

### 深入阅读
- [Error Handling Survey](https://blog.yoshuawuyts.com/error-handling-survey/)
- [Rust Error Handling](https://nick.groenen.me/posts/rust-error-handling/)
- [Failure vs Anyhow](https://github.com/rust-lang-nursery/failure/issues/347)
- [Error Handling Patterns](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

通过系统学习本章内容，你将具备设计和实现生产级错误处理系统的能力，为构建健壮可靠的 Rust 应用奠定坚实基础。